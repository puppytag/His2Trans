#!/usr/bin/env python3
"""Post-generation repair and semantic-audit loop for translated Rust crates."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.agentic_repair.openai_action_runner import OpenAIActionRunner  # noqa: E402
from scripts.agentic_repair.rust_native_refactor_gate import (  # noqa: E402
    rust_native_refactor_satisfied,
    rust_native_refactor_status,
    write_rust_native_refactor_task,
)
from scripts.agentic_repair.unsafe_scope_gate import UNSAFE_SCOPE_GATE, run_unsafe_scope_gate  # noqa: E402
from canonical_result import build_post_repair_result, write_canonical_result  # noqa: E402

try:  # noqa: SIM105
    from rule_fix import try_rule_fix  # noqa: E402
except Exception:  # noqa: BLE001
    try_rule_fix = None

DEFAULT_OHOS_RUSTC = (
    REPO_ROOT
    / "SelfContained"
    / "ohos_full"
    / "OpenHarmony-v5.0.1-Release"
    / "OpenHarmony"
    / "prebuilts"
    / "rustc"
    / "linux-x86_64"
    / "current"
    / "bin"
    / "rustc"
)
DEFAULT_OHOS_ROOT = (
    REPO_ROOT
    / "SelfContained"
    / "ohos_full"
    / "OpenHarmony-v5.0.1-Release"
    / "OpenHarmony"
)

# 后置 repair 的 clippy gate 默认停用；论文后分析脚本仍可单独运行 clippy 指标。
CLIPPY_GATE_ENABLED = False
CHEAP_GATE_NAMES = ["cargo", "ohos_rustc"]
BODY_COMPLETENESS_GATE = "body_completeness"
BODY_COMPLETENESS_GATE_ENABLED = False
BODY_INCOMPLETE_PATTERNS: tuple[tuple[str, re.Pattern[str]], ...] = (
    ("unimplemented", re.compile(r"\bunimplemented!\s*\(")),
    ("todo", re.compile(r"\btodo!\s*\(")),
    ("not_implemented_panic", re.compile(r"\bpanic!\s*\(\s*(?:r[#]*|b)?[\"'].*(?:not implemented|unimplemented).*", re.IGNORECASE)),
)
RUST_FN_NAME_PATTERN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\b")
UNSAFE_REVIEW_COMPLETION_RETRY_LIMIT = 3
UNSAFE_REFACTOR_AUDIT_GATE = "unsafe_refactor_audit"
UNSAFE_REFACTOR_AUDIT_RETRY_LIMIT = 3
UNSAFE_REFACTOR_AUDIT_COVERAGE_CONTINUATION_LIMIT = 6
SEMANTIC_AUDIT_RETRY_LIMIT = 3
SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT = 30
UNSAFE_REFACTOR_REDUCIBLE_CLASSIFICATIONS = {
    "reducible",
    "partially_reducible",
    "risky_reducible",
}


def _normalize_suite(suite: str = "ohos") -> str:
    """规范化项目 suite，非法值保持历史 OHOS 行为。"""
    value = str(suite or "ohos").strip().lower()
    return value if value in {"ohos", "oss"} else "ohos"


def _cheap_gate_names_for_suite(suite: str = "ohos") -> list[str]:
    """OSS 只使用宿主 Cargo；OHOS 额外使用目标 rustc。"""
    return ["cargo"] if _normalize_suite(suite) == "oss" else list(CHEAP_GATE_NAMES)


def _not_configured_gate_payload(gate: str, reason: str) -> dict[str, Any]:
    """构造未配置 gate 的稳定结果。"""
    return {
        "gate": gate,
        "configured": False,
        "status": "not_configured",
        "passed": True,
        "returncode": None,
        "text_log_path": "",
        "reason": reason,
    }


def _utc_now() -> str:
    """Return an ISO UTC timestamp."""
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _write_json(path: Path, payload: Any) -> None:
    """Write stable JSON."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def _progress(message: str) -> None:
    """打印可刷新的后置 repair 进度。"""
    print(f"[post-repair] {_utc_now()} {message}", flush=True)


def _write_canonical_post_repair_result(
    *,
    output_dir: Path,
    result: dict[str, Any],
    workspace_dir: Path,
    rendered_root: Path,
    summary_path: Path,
) -> Path:
    """Write gate-aware canonical result for post repair."""
    canonical = build_post_repair_result(
        run_id=f"{result.get('project', '')}__{result.get('llm_name', '')}",
        post_repair_summary=result,
        work_dir=workspace_dir,
        final_project_dir=rendered_root,
        post_repair_summary_path=summary_path,
    )
    result_path = output_dir / "canonical_post_repair_result.json"
    write_canonical_result(result_path, canonical)
    return result_path


def _final_verify_gate_bundle(summary: dict[str, Any]) -> str:
    """读取最终验证 gate bundle 路径。"""
    final_summary = summary.get("final_verify_summary")
    if isinstance(final_summary, dict):
        return str(final_summary.get("final_gate_bundle") or "")
    return ""


def _file_fingerprint(path: Path) -> dict[str, Any]:
    """计算单个文件的稳定指纹。"""
    try:
        data = path.read_bytes()
    except OSError:
        return {"path": str(path), "exists": False}
    return {
        "path": str(path),
        "exists": True,
        "size": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
    }


def _source_fingerprint(rendered_root: Path) -> dict[str, Any]:
    """记录最终 Rust crate 源文件指纹，避免过期 gate 被误用。"""
    files: list[dict[str, Any]] = []
    for path in sorted(rendered_root.rglob("*")):
        if not path.is_file():
            continue
        try:
            rel = path.relative_to(rendered_root)
        except ValueError:
            continue
        if any(part in {"target", ".cargo-home", ".git"} for part in rel.parts):
            continue
        if path.suffix not in {".rs", ".toml", ".lock", ".c", ".h", ".cc", ".cpp"} and path.name not in {"build.rs"}:
            continue
        item = _file_fingerprint(path)
        item["relative_path"] = str(rel)
        files.append(item)
    digest_src = json.dumps(files, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return {
        "schema_version": "c2r_source_fingerprint_v1",
        "rendered_root": str(rendered_root.resolve()),
        "generated_at": _utc_now(),
        "file_count": len(files),
        "sha256": hashlib.sha256(digest_src).hexdigest(),
        "files": files,
    }


def _write_final_verify_artifacts(
    *,
    output_dir: Path,
    rendered_root: Path,
    manifest: Path,
    ohos_rustc: Path,
    ohos_rust_target: str,
    suite: str = "ohos",
    semantic_payload: dict[str, Any] | None = None,
    unsafe_refactor_payload: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """运行最终 gate，并写出唯一最终验证口径。"""
    final_dir = output_dir / "final_verify"
    final_dir.mkdir(parents=True, exist_ok=True)
    cargo = _run_cargo_check(manifest, final_dir / "cargo_check.json")
    clippy = _run_cargo_clippy_gate(manifest, final_dir / "cargo_clippy_check.json")
    suite = _normalize_suite(suite)
    ohos = _run_suite_rustc_check(suite, rendered_root, manifest, final_dir / "ohos_rustc_check.json", ohos_rustc, ohos_rust_target)
    cheap_passed = cargo.get("returncode") == 0 and (suite == "oss" or ohos.get("returncode") == 0)
    unsafe_scope = _run_unsafe_scope_gate(rendered_root, final_dir, "final_verify") if cheap_passed else None
    semantic = semantic_payload if isinstance(semantic_payload, dict) else _skipped_semantic_audit_payload(
        "final_verify",
        "semantic audit result unavailable for final verify",
        final_dir / "semantic_audit.json",
    )
    gate_bundle = _write_gate_bundle(
        output_dir / "final_gate_bundle.json",
        "final_verify",
        cargo,
        clippy,
        ohos,
        semantic,
        unsafe_scope=unsafe_scope,
        unsafe_refactor=unsafe_refactor_payload,
        suite=suite,
    )
    summary = _blocking_summary(gate_bundle)
    fingerprint = _source_fingerprint(rendered_root)
    fingerprint_path = output_dir / "source_fingerprint.json"
    _write_json(fingerprint_path, fingerprint)
    summary.update(
        {
            "schema_version": "c2r_final_verify_summary_v1",
            "final_verify_dir": str(final_dir.resolve()),
            "final_gate_bundle": str(gate_bundle.resolve()),
            "source_fingerprint": str(fingerprint_path.resolve()),
            "source_fingerprint_sha256": fingerprint.get("sha256", ""),
        }
    )
    _write_json(output_dir / "final_verify_summary.json", summary)
    return summary


def _write_terminal_rejected_result(
    *,
    output_dir: Path,
    workspace_dir: Path,
    rendered_root: Path,
    manifest: Path,
    project: str,
    llm_name: str,
    failure_kind: str,
    failure_message: str,
    ohos_rustc: Path,
    ohos_rust_target: str,
    suite: str = "ohos",
    rounds: list[dict[str, Any]] | None = None,
) -> int:
    """写出不会继续 repair 的 rejected 终态。"""
    output_dir.mkdir(parents=True, exist_ok=True)
    summary_path = output_dir / "post_repair_summary.json"
    semantic = _skipped_semantic_audit_payload(
        "final_verify",
        failure_message,
        output_dir / "final_verify" / "semantic_audit.json",
    )
    final_summary = _write_final_verify_artifacts(
        output_dir=output_dir,
        rendered_root=rendered_root,
        manifest=manifest,
        ohos_rustc=ohos_rustc,
        ohos_rust_target=ohos_rust_target,
        suite=suite,
        semantic_payload=semantic,
    )
    result: dict[str, Any] = {
        "schema_version": "c2r_post_repair_result_v1",
        "project": project,
        "llm_name": llm_name,
        "rendered_root": str(rendered_root),
        "output_dir": str(output_dir),
        "started_at": _utc_now(),
        "finished_at": _utc_now(),
        "rounds": rounds or [],
        "final_verify_summary": final_summary,
        "final_status": "rejected",
        "accepted_round": None,
        "failure_kind": failure_kind,
        "failure_message": failure_message,
    }
    _write_json(summary_path, result)
    _write_canonical_post_repair_result(
        output_dir=output_dir,
        result=result,
        workspace_dir=workspace_dir,
        rendered_root=rendered_root,
        summary_path=summary_path,
    )
    return 1


def _read_json(path: Path) -> Any:
    """Read JSON from path."""
    return json.loads(path.read_text(encoding="utf-8"))


def _read_json_object(path: Path) -> dict[str, Any]:
    """Read a JSON object, returning empty object on failure."""
    try:
        payload = _read_json(path)
    except (OSError, json.JSONDecodeError):
        return {}
    return payload if isinstance(payload, dict) else {}


SOURCE_EVIDENCE_SUFFIX_ORDER = (".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".hxx")
SOURCE_EVIDENCE_SUFFIXES = set(SOURCE_EVIDENCE_SUFFIX_ORDER)
SOURCE_HEADER_SUFFIXES = {".h", ".hh", ".hpp", ".hxx"}
SOURCE_SCAN_SKIP_DIRS = {
    ".cargo-home",
    ".git",
    "__pycache__",
    "build",
    "cmake-build-debug",
    "cmake-build-release",
    "out",
    "target",
}
SOURCE_EVIDENCE_LIST_LIMIT = 80
SOURCE_EVIDENCE_SCAN_LIMIT = 2000


def _normalized_source_file_text(value: Any) -> str:
    """规范化 manifest 中的 C/C++ 源文件文本。"""
    text = str(value or "").strip()
    if not text:
        return ""
    text = text.replace("\\", "/")
    while text.startswith("./"):
        text = text[2:]
    try:
        return str(PurePosixPath(text))
    except (TypeError, ValueError):
        return text


def _path_looks_like_test_or_example(path: Path) -> bool:
    """判断源码路径是否更像测试、示例或 benchmark 用法。"""
    parts = {part.lower() for part in path.parts}
    if parts & {"test", "tests", "unittest", "unittests", "example", "examples", "sample", "samples", "benchmark", "benchmarks"}:
        return True
    name = path.name.lower()
    return any(token in name for token in ("test", "unittest", "example", "sample", "bench"))


def _source_evidence_bucket(path: Path) -> str:
    """按源码角色分桶，优先区分测试/示例用法。"""
    if _path_looks_like_test_or_example(path):
        return "test_or_example_usage"
    if path.suffix.lower() in SOURCE_HEADER_SUFFIXES:
        return "public_headers"
    return "production_sources"


def _source_search_roots(source_project_root: str, copied_c_source: str) -> list[Path]:
    """收集可打开的原始源码搜索根目录。"""
    roots: list[Path] = []
    seen: set[str] = set()
    for raw in (source_project_root, copied_c_source):
        if not raw:
            continue
        path = Path(raw).expanduser()
        if not path.is_dir():
            continue
        resolved = path.resolve()
        key = str(resolved)
        if key in seen:
            continue
        seen.add(key)
        roots.append(resolved)
    return roots


def _relative_to_source_roots(path: Path, search_roots: list[Path]) -> str:
    """返回相对任一源码根的稳定路径。"""
    resolved = path.resolve()
    for root in search_roots:
        try:
            return str(resolved.relative_to(root)).replace("\\", "/")
        except ValueError:
            continue
    return resolved.name


def _scan_source_root_files(search_roots: list[Path]) -> tuple[list[Path], bool]:
    """轻量扫描源码根，跳过构建目录并限制最大文件数。"""
    files: list[Path] = []
    seen: set[str] = set()
    truncated = False
    for root in search_roots:
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = sorted(name for name in dirnames if name not in SOURCE_SCAN_SKIP_DIRS)
            for filename in sorted(filenames):
                path = Path(dirpath) / filename
                if path.suffix.lower() not in SOURCE_EVIDENCE_SUFFIXES:
                    continue
                resolved = path.resolve()
                key = str(resolved)
                if key in seen:
                    continue
                seen.add(key)
                files.append(resolved)
                if len(files) >= SOURCE_EVIDENCE_SCAN_LIMIT:
                    return files, True
    return files, truncated


def _source_index_keys(path: Path, search_roots: list[Path]) -> set[str]:
    """为源码路径构造 manifest 可匹配的索引键。"""
    rel = _relative_to_source_roots(path, search_roots)
    rel_no_suffix = str(PurePosixPath(rel).with_suffix(""))
    keys = {
        rel,
        rel_no_suffix,
        rel_no_suffix.replace("/", "_"),
        path.name,
        path.stem,
    }
    return {_normalized_source_file_text(key) for key in keys if key}


def _build_source_file_index(source_files: list[Path], search_roots: list[Path]) -> dict[str, list[Path]]:
    """构建源码文件反查表。"""
    index: dict[str, list[Path]] = {}
    for path in source_files:
        for key in _source_index_keys(path, search_roots):
            index.setdefault(key, []).append(path)
    return index


def _manifest_source_entries(functions_manifest: str) -> list[dict[str, Any]]:
    """从 functions_manifest 提取可能的原始源码键。"""
    if not functions_manifest:
        return []
    manifest_path = Path(functions_manifest)
    if not manifest_path.is_file():
        return []
    payload = _read_json_object(manifest_path)
    raw_entries = payload.get("functions")
    if not isinstance(raw_entries, list):
        raw_entries = payload.get("items")
    if not isinstance(raw_entries, list):
        return []
    entries: list[dict[str, Any]] = []
    for index, item in enumerate(raw_entries):
        if not isinstance(item, dict):
            continue
        source_keys: list[str] = []
        for key in ("source_path", "source_file_path", "source", "c_file", "cpp_file", "file", "source_file"):
            text = _normalized_source_file_text(item.get(key))
            if text and text not in source_keys:
                source_keys.append(text)
        uid_source = _normalized_source_file_text(str(item.get("uid") or "").split(":", 1)[0])
        if uid_source and uid_source not in source_keys:
            source_keys.append(uid_source)
        func_file = _normalized_source_file_text(item.get("func_file"))
        func_source = re.sub(r"_\d+$", "", func_file) if func_file else ""
        if func_source and func_source not in source_keys:
            source_keys.append(func_source)
        for source_key in source_keys:
            if not source_key:
                continue
            entries.append(
                {
                    "source_key": source_key,
                    "manifest_index": index,
                    "start_line": item.get("start_line"),
                    "end_line": item.get("end_line"),
                }
            )
    return entries


def _candidate_source_keys(source_key: str) -> list[str]:
    """为 src_buffer 这类 extractor 键生成候选源码键。"""
    key = _normalized_source_file_text(source_key)
    if not key:
        return []
    candidates = [key]
    no_suffix = str(PurePosixPath(key).with_suffix("")) if PurePosixPath(key).suffix else key
    if "_" in no_suffix:
        candidates.append(no_suffix.replace("_", "/", 1))
    if no_suffix.startswith("src_"):
        candidates.append(f"src/{no_suffix[len('src_'):]}")
    if no_suffix.startswith("include_"):
        candidates.append(f"include/{no_suffix[len('include_'):]}")
    expanded: list[str] = []
    for candidate in candidates:
        if candidate not in expanded:
            expanded.append(candidate)
        if PurePosixPath(candidate).suffix:
            continue
        for suffix in SOURCE_EVIDENCE_SUFFIX_ORDER:
            suffixed = f"{candidate}{suffix}"
            if suffixed not in expanded:
                expanded.append(suffixed)
    return [_normalized_source_file_text(candidate) for candidate in expanded]


def _pick_source_path(paths: list[Path]) -> Path | None:
    """从多个匹配中优先选择生产源码。"""
    if not paths:
        return None
    ranked = sorted(paths, key=lambda path: (_source_evidence_bucket(path) != "production_sources", path.suffix.lower() in SOURCE_HEADER_SUFFIXES))
    return ranked[0]


def _resolve_source_evidence_path(source_key: str, search_roots: list[Path], source_index: dict[str, list[Path]]) -> Path | None:
    """把 manifest 源码键解析为真实源码文件路径。"""
    for candidate in _candidate_source_keys(source_key):
        path = Path(candidate).expanduser()
        if path.is_absolute() and path.is_file():
            return path.resolve()
        for root in search_roots:
            direct = root / candidate
            if direct.is_file():
                return direct.resolve()
        indexed = _pick_source_path(source_index.get(candidate, []))
        if indexed is not None:
            return indexed
    return None


def _remember_source_evidence(
    buckets: dict[str, list[dict[str, Any]]],
    seen: set[str],
    *,
    path: Path,
    search_roots: list[Path],
    origin: str,
    source_key: str = "",
    start_line: Any = None,
    end_line: Any = None,
) -> None:
    """记录一个源码证据入口，保持绝对路径和相对路径。"""
    resolved = path.resolve()
    bucket = _source_evidence_bucket(resolved)
    dedupe_key = f"{bucket}:{resolved}"
    relative_path = _relative_to_source_roots(resolved, search_roots)
    relative_dedupe_key = f"{bucket}:rel:{relative_path}"
    if dedupe_key in seen or relative_dedupe_key in seen:
        return
    seen.add(dedupe_key)
    seen.add(relative_dedupe_key)
    item: dict[str, Any] = {
        "path": str(resolved),
        "relative_path": relative_path,
        "origin": origin,
    }
    if source_key:
        item["source_key"] = source_key
    if isinstance(start_line, int) and start_line > 0:
        item["start_line"] = start_line
    if isinstance(end_line, int) and end_line > 0:
        item["end_line"] = end_line
    if len(buckets[bucket]) < SOURCE_EVIDENCE_LIST_LIMIT:
        buckets[bucket].append(item)


def _build_resolved_source_evidence(source_project_root: str, copied_c_source: str, functions_manifest: str) -> dict[str, Any]:
    """构建 repair/semantic agent 首屏可用的原始源码证据索引。"""
    search_roots = _source_search_roots(source_project_root, copied_c_source)
    source_files, scan_truncated = _scan_source_root_files(search_roots)
    source_index = _build_source_file_index(source_files, search_roots)
    buckets: dict[str, list[dict[str, Any]]] = {
        "production_sources": [],
        "public_headers": [],
        "test_or_example_usage": [],
    }
    seen: set[str] = set()
    unresolved_sources: list[dict[str, Any]] = []
    manifest_entries = _manifest_source_entries(functions_manifest)
    for entry in manifest_entries:
        source_key = str(entry.get("source_key") or "")
        resolved = _resolve_source_evidence_path(source_key, search_roots, source_index)
        if resolved is None:
            if len(unresolved_sources) < SOURCE_EVIDENCE_LIST_LIMIT:
                unresolved_sources.append(
                    {
                        "source_key": source_key,
                        "manifest_index": entry.get("manifest_index"),
                    }
                )
            continue
        _remember_source_evidence(
            buckets,
            seen,
            path=resolved,
            search_roots=search_roots,
            origin="functions_manifest",
            source_key=source_key,
            start_line=entry.get("start_line"),
            end_line=entry.get("end_line"),
        )
    for path in source_files:
        bucket = _source_evidence_bucket(path)
        if bucket == "production_sources" and manifest_entries:
            continue
        _remember_source_evidence(
            buckets,
            seen,
            path=path,
            search_roots=search_roots,
            origin="source_root_scan",
        )
    return {
        "schema_version": "c2r_resolved_source_evidence_v1",
        "search_roots": [str(root) for root in search_roots],
        "production_sources": buckets["production_sources"],
        "public_headers": buckets["public_headers"],
        "test_or_example_usage": buckets["test_or_example_usage"],
        "unresolved_sources": unresolved_sources,
        "counts": {
            "search_root_count": len(search_roots),
            "scanned_source_file_count": len(source_files),
            "manifest_source_entry_count": len(manifest_entries),
            "production_sources": len(buckets["production_sources"]),
            "public_headers": len(buckets["public_headers"]),
            "test_or_example_usage": len(buckets["test_or_example_usage"]),
            "unresolved_sources": len(unresolved_sources),
            "scan_truncated": scan_truncated,
        },
    }


def _cargo_env(manifest_path: Path) -> dict[str, str]:
    """Build Cargo environment with writable CARGO_HOME."""
    env = dict(os.environ)
    if str(env.get("CARGO_HOME", "")).strip():
        return env
    default_cargo_home = Path.home() / ".cargo"
    if _cargo_home_has_registry(default_cargo_home):
        env["CARGO_HOME"] = str(default_cargo_home)
        env.setdefault("CARGO_NET_OFFLINE", "true")
        return env
    cargo_home = manifest_path.parent / ".cargo-home"
    cargo_home.mkdir(parents=True, exist_ok=True)
    env["CARGO_HOME"] = str(cargo_home)
    return env


def _cargo_home_has_registry(cargo_home: Path) -> bool:
    """检查 Cargo home 是否已有可离线使用的 registry 缓存。"""
    registry = cargo_home / "registry"
    cache = registry / "cache"
    src = registry / "src"
    return (
        any(cache.glob("*/*.crate")) if cache.is_dir() else False
    ) or (
        any(src.glob("*/*")) if src.is_dir() else False
    )


def _cargo_command_with_optional_offline(base: list[str], env: dict[str, str]) -> list[str]:
    """在复用本地 registry 缓存时为 cargo gate 加 offline 参数。"""
    if str(env.get("CARGO_NET_OFFLINE", "")).strip().lower() == "true" and "--offline" not in base:
        return [*base, "--offline"]
    return base


def _command_timeout_sec() -> float:
    """读取 cargo gate 默认超时秒数。"""
    try:
        return max(1.0, float(os.environ.get("C2R_CARGO_CHECK_TIMEOUT_SEC", "300") or "300"))
    except ValueError:
        return 300.0


def _run_command(command: list[str], *, cwd: Path | None, log_path: Path, env: dict[str, str] | None = None, timeout_sec: float | None = None) -> dict[str, Any]:
    """Run command and save JSON + text logs."""
    started = time.time()
    timeout = _command_timeout_sec() if timeout_sec is None else float(timeout_sec)
    try:
        completed = subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False, env=env, timeout=timeout)
        payload = {
            "command": command,
            "returncode": completed.returncode,
            "elapsed_sec": round(time.time() - started, 3),
            "stdout": completed.stdout,
            "stderr": completed.stderr,
            "timeout_sec": timeout,
        }
    except subprocess.TimeoutExpired as exc:
        payload = {
            "command": command,
            "returncode": 124,
            "elapsed_sec": round(time.time() - started, 3),
            "stdout": exc.stdout or "",
            "stderr": f"C2R_COMPILE_TIMEOUT: command timed out after {timeout:g}s",
            "timeout_sec": timeout,
        }
    except Exception as exc:  # noqa: BLE001
        payload = {
            "command": command,
            "returncode": 127,
            "elapsed_sec": round(time.time() - started, 3),
            "stdout": "",
            "stderr": f"{type(exc).__name__}: {exc}",
            "timeout_sec": timeout,
        }
    _write_json(log_path, payload)
    text_log = log_path.with_suffix(".log")
    text_log.write_text(
        "COMMAND: " + " ".join(command) + "\n\nSTDOUT:\n" + payload["stdout"] + "\nSTDERR:\n" + payload["stderr"],
        encoding="utf-8",
    )
    payload["text_log_path"] = str(text_log.resolve())
    return payload


def _run_cargo_check(manifest_path: Path, log_path: Path) -> dict[str, Any]:
    """Run cargo check."""
    env = _cargo_env(manifest_path)
    command = _cargo_command_with_optional_offline(
        ["cargo", "check", "--manifest-path", str(manifest_path), "--quiet"],
        env,
    )
    return _run_command(command, cwd=manifest_path.parent, log_path=log_path, env=env)


def _run_cargo_clippy(manifest_path: Path, log_path: Path) -> dict[str, Any]:
    """Run cargo clippy with warnings denied."""
    env = _cargo_env(manifest_path)
    command = _cargo_command_with_optional_offline(
        ["cargo", "clippy", "--manifest-path", str(manifest_path), "--quiet"],
        env,
    )
    return _run_command(
        [*command, "--", "-D", "warnings"],
        cwd=manifest_path.parent,
        log_path=log_path,
        env=env,
    )


def _disabled_cargo_clippy(log_path: Path) -> dict[str, Any]:
    """生成已停用的 clippy gate 记录，不执行 cargo clippy。"""
    payload = {
        "gate": "cargo_clippy",
        "mode": "off",
        "status": "not_configured",
        "passed": True,
        "returncode": 0,
        "elapsed_sec": 0.0,
        "stdout": "",
        "stderr": "cargo_clippy gate is disabled for post-repair.",
        "text_log_path": str(log_path.with_suffix(".log").resolve()),
    }
    _write_json(log_path, payload)
    log_path.with_suffix(".log").write_text(
        "cargo_clippy gate is disabled for post-repair.\n",
        encoding="utf-8",
    )
    return payload


def _run_cargo_clippy_gate(manifest_path: Path, log_path: Path) -> dict[str, Any]:
    """按当前 gate 配置运行或跳过 clippy。"""
    if not CLIPPY_GATE_ENABLED:
        return _disabled_cargo_clippy(log_path)
    return _run_cargo_clippy(manifest_path, log_path)


def _manifest_lib_relative_path(manifest_path: Path) -> str:
    """Read [lib].path from Cargo.toml, falling back to src/main.rs then src/lib.rs."""
    in_lib = False
    try:
        for line in manifest_path.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            if stripped.startswith("[") and stripped.endswith("]"):
                in_lib = stripped == "[lib]"
                continue
            if in_lib and stripped.startswith("path") and "=" in stripped:
                value = stripped.split("=", 1)[1].strip().strip('"').strip("'")
                if value:
                    return value
    except OSError:
        pass
    if (manifest_path.parent / "src" / "main.rs").is_file():
        return "src/main.rs"
    return "src/lib.rs"


def _crate_name_from_manifest(manifest_path: Path) -> str:
    """Read package name from Cargo.toml."""
    fallback = manifest_path.parent.name.replace("-", "_")
    try:
        for line in manifest_path.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if stripped.startswith("name") and "=" in stripped:
                value = stripped.split("=", 1)[1].strip().strip('"').strip("'")
                if value:
                    return value.replace("-", "_")
    except OSError:
        return fallback
    return fallback


def _run_ohos_rustc_check(rendered_root: Path, manifest_path: Path, log_path: Path, ohos_rustc: Path, ohos_rust_target: str) -> dict[str, Any]:
    """Run OHOS rustc target check through Cargo so dependencies/build.rs are honored."""
    command = [
        "cargo",
        "check",
        "--manifest-path",
        str(manifest_path),
        "--target",
        ohos_rust_target,
        "--quiet",
    ]
    if not ohos_rustc.is_file():
        payload = {
            "command": command,
            "returncode": 127,
            "elapsed_sec": 0.0,
            "stdout": "",
            "stderr": f"OHOS rustc 不存在：{ohos_rustc}\n",
            "ohos_rustc": str(ohos_rustc),
            "ohos_rust_target": ohos_rust_target,
        }
        _write_gate_log(log_path, payload)
        return payload
    wrapper_path = _write_ohos_rustc_wrapper(log_path.parent, ohos_rustc)
    env = _cargo_env(manifest_path)
    env["RUSTC"] = str(wrapper_path)
    env["RUSTC_BOOTSTRAP"] = "1"
    env.setdefault("RUST_BACKTRACE", "0")
    command = _cargo_command_with_optional_offline(command, env)
    payload = _run_command(command, cwd=rendered_root, log_path=log_path, env=env)
    payload["ohos_rustc"] = str(ohos_rustc)
    payload["ohos_rustc_wrapper"] = str(wrapper_path)
    payload["ohos_rust_target"] = ohos_rust_target
    _write_json(log_path, payload)
    return payload


def _run_suite_rustc_check(
    suite: str,
    rendered_root: Path,
    manifest_path: Path,
    log_path: Path,
    ohos_rustc: Path,
    ohos_rust_target: str,
) -> dict[str, Any]:
    """仅 OHOS suite 运行目标 rustc；OSS 不启动该进程。"""
    if _normalize_suite(suite) == "oss":
        return _not_configured_gate_payload("ohos_rustc", "OSS suite uses native cargo check only")
    return _run_ohos_rustc_check(rendered_root, manifest_path, log_path, ohos_rustc, ohos_rust_target)


def _gate_error_text(*payloads: dict[str, Any]) -> str:
    """从 gate payload 和 text log 中提取错误文本。"""
    parts: list[str] = []
    for payload in payloads:
        if not isinstance(payload, dict):
            continue
        for key in ("stderr", "stdout"):
            value = str(payload.get(key, "") or "")
            if value.strip():
                parts.append(value)
        text_log = str(payload.get("text_log_path", "") or "")
        if text_log:
            try:
                log_text = Path(text_log).read_text(encoding="utf-8", errors="ignore")
            except OSError:
                log_text = ""
            if log_text.strip():
                parts.append(log_text)
    return "\n".join(parts)


def _iter_rustc_error_chunks(error_text: str) -> list[str]:
    """把 rustc 输出拆成单个 diagnostic，避免后面的错误被第一行号遮蔽。"""
    if not error_text:
        return []
    chunks = [chunk.strip() for chunk in re.split(r"(?=error\[E\d+\])", error_text) if chunk.strip()]
    return chunks or [error_text]


def _error_chunk_rel_path(chunk: str) -> str:
    """从 rustc diagnostic 中提取相对源码路径。"""
    match = re.search(r"-->\s+([^\s:]+):\d+:\d+", chunk)
    if match:
        return match.group(1)
    match = re.search(r"\s(?:at\s+)?([^\s:]+\.rs):\d+:\d+", chunk)
    return match.group(1) if match else ""


def _append_dlist_compat_shims(rendered_root: Path, error_text: str) -> str:
    """当翻译误把 DLIST 宏当 compat 函数时，追加确定性链表 shim。"""
    if "DListForEachEntry" not in error_text and "DListForEachEntryNext" not in error_text:
        return ""
    compat_path = rendered_root / "src" / "compat.rs"
    if not compat_path.is_file():
        return ""
    content = compat_path.read_text(encoding="utf-8", errors="ignore")
    if "C2R_DLIST_SHIMS_BEGIN" in content:
        return ""
    shim = r'''

// === C2R_DLIST_SHIMS_BEGIN ===
#[allow(non_snake_case)]
pub unsafe fn DListForEachEntry(
    head: *const crate::types::DListHead,
    _entry_size: u64,
    field_offset: u64,
) -> *mut ::core::ffi::c_void {
    if head.is_null() {
        return ::core::ptr::null_mut();
    }
    let node = (*head).next as *mut u8;
    if node.is_null() {
        return ::core::ptr::null_mut();
    }
    node.offset(-(field_offset as isize)) as *mut ::core::ffi::c_void
}

#[allow(non_snake_case)]
pub unsafe fn DListForEachEntryNext(
    current: *const crate::types::DListHead,
    _entry_size: u64,
    field_offset: u64,
) -> *mut ::core::ffi::c_void {
    if current.is_null() {
        return ::core::ptr::null_mut();
    }
    let node = (*current).next as *mut u8;
    if node.is_null() {
        return ::core::ptr::null_mut();
    }
    node.offset(-(field_offset as isize)) as *mut ::core::ffi::c_void
}
// === C2R_DLIST_SHIMS_END ===
'''
    compat_path.write_text(content.rstrip() + shim + "\n", encoding="utf-8")
    return str(compat_path.resolve())


def _apply_post_repair_rule_fixes(rendered_root: Path, error_text: str, output_dir: Path, label: str) -> dict[str, Any]:
    """在 post-repair gate 后先跑确定性规则修复，减少 agent 循环损耗。"""
    record: dict[str, Any] = {
        "status": "skipped",
        "label": label,
        "changed_files": [],
        "fixers": [],
        "log_path": "",
    }
    enabled = str(os.environ.get("C2R_POST_REPAIR_RULE_FIX", "0") or "0").strip().lower()
    if enabled in {"0", "false", "off", "no"} or try_rule_fix is None or not error_text.strip():
        return record

    changed: set[str] = set()
    dlist_changed = _append_dlist_compat_shims(rendered_root, error_text)
    if dlist_changed:
        changed.add(dlist_changed)
        record["fixers"].append("DListCompatShim")

    for chunk in _iter_rustc_error_chunks(error_text):
        rel_path = _error_chunk_rel_path(chunk)
        if not rel_path:
            continue
        file_path = (rendered_root / rel_path).resolve()
        try:
            file_path.relative_to(rendered_root.resolve())
        except ValueError:
            continue
        if not file_path.is_file() or file_path.suffix != ".rs":
            continue

        original = file_path.read_text(encoding="utf-8", errors="ignore")
        current = original
        for _ in range(3):
            fixed, fixer_name = try_rule_fix(current, chunk)
            if not fixed or fixed == current:
                break
            current = fixed
            record["fixers"].append(fixer_name or "RuleFix")
        if current != original:
            file_path.write_text(current, encoding="utf-8")
            changed.add(str(file_path))

    if changed:
        record["status"] = "applied"
        record["changed_files"] = sorted(changed)
        log_path = output_dir / "rule_fix_logs" / f"{label}_post_repair_rule_fix.json"
        record["log_path"] = str(log_path.resolve())
        _write_json(log_path, record)
    return record


def _write_ohos_rustc_wrapper(output_dir: Path, ohos_rustc: Path) -> Path:
    """生成兼容 wrapper，过滤新版 Cargo 传给旧 OHOS rustc 的 check-cfg 参数。"""
    output_dir.mkdir(parents=True, exist_ok=True)
    wrapper_path = output_dir / "ohos_rustc_wrapper.sh"
    rustc_path = str(ohos_rustc)
    wrapper_path.write_text(
        "\n".join(
            [
                "#!/usr/bin/env bash",
                "args=()",
                "skip_next=0",
                'for arg in "$@"; do',
                '  if [ "$skip_next" = "1" ]; then',
                "    skip_next=0",
                "    continue",
                "  fi",
                '  case "$arg" in',
                "    --check-cfg)",
                "      skip_next=1",
                "      ;;",
                "    --check-cfg=*)",
                "      ;;",
                "    --allow=unexpected_cfgs)",
                "      ;;",
                "    *)",
                '      args+=("$arg")',
                "      ;;",
                "  esac",
                "done",
                f'exec "{rustc_path}" "${{args[@]}}"',
                "",
            ]
        ),
        encoding="utf-8",
    )
    wrapper_path.chmod(0o755)
    return wrapper_path


def _write_gate_log(log_path: Path, payload: dict[str, Any]) -> None:
    """Write a gate result to JSON and text log."""
    _write_json(log_path, payload)
    text_log = log_path.with_suffix(".log")
    text_log.write_text(
        "COMMAND: " + " ".join(str(x) for x in payload.get("command", [])) + "\n\nSTDOUT:\n" + str(payload.get("stdout", "")) + "\nSTDERR:\n" + str(payload.get("stderr", "")),
        encoding="utf-8",
    )
    payload["text_log_path"] = str(text_log.resolve())
    _write_json(log_path, payload)


def _rust_fn_name_from_signature(signature: str) -> str:
    """从 Rust 签名中提取函数名。"""
    match = RUST_FN_NAME_PATTERN.search(signature or "")
    return match.group(1) if match else ""


def _body_target_file_candidates(func_key: str, module: str) -> list[str]:
    """根据 func_key/module 生成可能的 Rust 源文件相对路径。"""
    names: list[str] = []
    for item in (module, str(func_key).rsplit("_", 1)[0] if "_" in str(func_key) else ""):
        item = str(item or "").strip()
        if item and item not in names:
            names.append(item)
    result: list[str] = []
    for name in names:
        for filename in (f"src/{name}.rs", f"src/src_{name}.rs"):
            if filename not in result:
                result.append(filename)
    return result


def _build_body_target_scope(workspace_dir: Path, project: str, *, output_path: Path | None = None) -> dict[str, Any]:
    """构造 body completeness 的源函数一对一目标集合。"""
    workspace = Path(workspace_dir)
    targets: list[dict[str, Any]] = []
    source = ""
    mapping_path = workspace / "source_skeletons" / project / "func_file_to_rust_sig.json"
    if mapping_path.is_file():
        mapping = _read_json_object(mapping_path)
        if isinstance(mapping, dict):
            source = str(mapping_path.resolve())
            for func_key, signature in sorted(mapping.items()):
                signature_text = str(signature or "").strip()
                func_key_text = str(func_key or "").strip()
                fn_name = _rust_fn_name_from_signature(signature_text)
                if not func_key_text or not signature_text or not fn_name:
                    continue
                module = func_key_text.rsplit("_", 1)[0] if "_" in func_key_text else func_key_text
                targets.append(
                    {
                        "func_key": func_key_text,
                        "fn_name": fn_name,
                        "module": module,
                        "rust_signature": signature_text,
                        "rust_file_candidates": _body_target_file_candidates(func_key_text, module),
                    }
                )

    if not targets:
        signature_dir = workspace / "signature_matches" / project
        if signature_dir.is_dir():
            source = str(signature_dir.resolve())
            for path in sorted(signature_dir.glob("*.txt")):
                signature_text = path.read_text(encoding="utf-8", errors="replace").strip()
                func_key_text = path.stem
                fn_name = _rust_fn_name_from_signature(signature_text)
                if not signature_text or not fn_name:
                    continue
                module = func_key_text.rsplit("_", 1)[0] if "_" in func_key_text else func_key_text
                targets.append(
                    {
                        "func_key": func_key_text,
                        "fn_name": fn_name,
                        "module": module,
                        "rust_signature": signature_text,
                        "rust_file_candidates": _body_target_file_candidates(func_key_text, module),
                    }
                )

    payload = {
        "schema_version": "c2r_body_target_scope_v1",
        "configured": bool(targets),
        "path": str(output_path.resolve()) if output_path is not None else "",
        "source": source,
        "project": project,
        "target_count": len(targets),
        "targets": targets,
    }
    if output_path is not None:
        _write_json(output_path, payload)
    return payload


def _read_body_target_scope(scope: dict[str, Any] | str | Path | None) -> dict[str, Any]:
    """读取 body target scope；缺失时返回空 scope。"""
    if isinstance(scope, dict):
        return scope
    if scope:
        path = Path(scope)
        if path.is_file():
            return _read_json_object(path)
    return {}


def _find_rust_function_block(content: str, fn_name: str) -> tuple[str, int] | None:
    """在 Rust 文件中提取目标函数块和起始行号。"""
    if not fn_name:
        return None
    pattern = re.compile(r"\bfn\s+" + re.escape(fn_name) + r"\b")
    for match in pattern.finditer(content):
        brace_pos = content.find("{", match.end())
        if brace_pos < 0:
            continue
        depth = 0
        for idx in range(brace_pos, len(content)):
            char = content[idx]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
                if depth == 0:
                    start_line = content.count("\n", 0, match.start()) + 1
                    return content[match.start() : idx + 1], start_line
    return None


def _first_incomplete_marker(block: str) -> tuple[str, str] | None:
    """返回函数块里的第一个占位实现标记。"""
    for line in block.splitlines():
        for kind, pattern in BODY_INCOMPLETE_PATTERNS:
            if pattern.search(line):
                return kind, line.strip()[:200]
    return None


def _scan_body_completeness(rendered_root: Path, *, max_findings: int = 200, body_target_scope: dict[str, Any] | str | Path | None = None) -> dict[str, Any]:
    """扫描 Rust 函数体残留占位实现，作为完整性硬 gate。"""
    root = Path(rendered_root)
    findings: list[dict[str, Any]] = []
    total = 0
    skip_dirs = {"target", ".git", ".cargo-home"}
    if not root.is_dir():
        return {
            "configured": True,
            "passed": False,
            "total_findings": 1,
            "findings": [{"path": str(root), "line": 0, "kind": "missing_root", "excerpt": "rendered root not found"}],
        }

    target_scope = _read_body_target_scope(body_target_scope)
    targets = target_scope.get("targets") if isinstance(target_scope.get("targets"), list) else []
    if targets:
        unmatched = 0
        for target in targets:
            if not isinstance(target, dict):
                continue
            fn_name = str(target.get("fn_name", "") or "").strip()
            candidates = target.get("rust_file_candidates")
            if not isinstance(candidates, list):
                candidates = []
            matched = False
            for rel_candidate in candidates:
                rel = str(rel_candidate or "").strip()
                if not rel:
                    continue
                path = root / rel
                if not path.is_file():
                    continue
                try:
                    content = path.read_text(encoding="utf-8", errors="replace")
                except OSError:
                    continue
                block_info = _find_rust_function_block(content, fn_name)
                if block_info is None:
                    continue
                matched = True
                block, start_line = block_info
                marker = _first_incomplete_marker(block)
                if marker is not None:
                    total += 1
                    if len(findings) < max_findings:
                        kind, excerpt = marker
                        findings.append(
                            {
                                "path": rel,
                                "line": start_line,
                                "kind": kind,
                                "excerpt": excerpt,
                                "func_key": str(target.get("func_key", "") or ""),
                                "fn_name": fn_name,
                            }
                        )
                break
            if not matched:
                unmatched += 1

        return {
            "configured": True,
            "passed": total == 0,
            "total_findings": total,
            "findings": findings,
            "truncated": total > len(findings),
            "scope": "source_target_functions",
            "target_count": len(targets),
            "unmatched_target_count": unmatched,
            "target_scope_path": str(target_scope.get("path", "") or ""),
            "target_scope_source": str(target_scope.get("source", "") or ""),
        }

    for path in sorted(root.rglob("*.rs")):
        rel = path.relative_to(root).as_posix()
        if any(part in skip_dirs for part in path.relative_to(root).parts):
            continue
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue
        for lineno, line in enumerate(lines, start=1):
            for kind, pattern in BODY_INCOMPLETE_PATTERNS:
                if not pattern.search(line):
                    continue
                total += 1
                if len(findings) < max_findings:
                    findings.append(
                        {
                            "path": rel,
                            "line": lineno,
                            "kind": kind,
                            "excerpt": line.strip()[:200],
                        }
                    )
                break

    return {
        "configured": True,
        "passed": total == 0,
        "total_findings": total,
        "findings": findings,
        "truncated": total > len(findings),
        "scope": "all_rs_no_target_scope",
    }


def _semantic_audit_passed(result: dict[str, Any] | None) -> bool:
    """Return true when semantic audit has no open blockers."""
    if not isinstance(result, dict):
        return True
    if str(result.get("mode", "")).strip() == "off":
        return True
    verdict = str(result.get("verdict", "")).strip()
    open_ids = result.get("open_blocker_ids")
    if not (isinstance(open_ids, list) and any(str(item).strip() for item in open_ids)):
        open_ids = result.get("blocking_ids")
    open_obligation_ids = result.get("open_obligation_ids", result.get("semantic_obligation_blocking_ids"))
    external_ids = result.get("blocked_external_ids")
    external_obligation_ids = result.get("blocked_external_obligation_ids")
    has_open = isinstance(open_ids, list) and any(str(item).strip() for item in open_ids)
    has_open_obligation = isinstance(open_obligation_ids, list) and any(str(item).strip() for item in open_obligation_ids)
    has_external = isinstance(external_ids, list) and any(str(item).strip() for item in external_ids)
    has_external_obligation = isinstance(external_obligation_ids, list) and any(str(item).strip() for item in external_obligation_ids)
    diagnostics = result.get("diagnostics")
    has_diagnostics = isinstance(diagnostics, list) and any(str(item).strip() for item in diagnostics)
    if has_open or has_open_obligation:
        return False
    if has_diagnostics:
        return False
    if verdict in {"accepted", "accepted_with_residual_risks"}:
        return True
    return verdict == "rejected" and (has_external or has_external_obligation)


def _unsafe_refactor_open_items(ledger: dict[str, Any]) -> list[dict[str, Any]]:
    """读取 unsafe auditor ledger 中仍需 repair 的可降低项。"""
    explicit = ledger.get("open_reducible_items")
    result: list[dict[str, Any]] = []
    seen: set[str] = set()
    if isinstance(explicit, list):
        for item in explicit:
            if not isinstance(item, dict):
                continue
            item_id = str(item.get("id", "")).strip()
            key = item_id or json.dumps(item, ensure_ascii=False, sort_keys=True)
            if key not in seen:
                result.append(dict(item))
                seen.add(key)
    items = ledger.get("items")
    if isinstance(items, list):
        for item in items:
            if not isinstance(item, dict):
                continue
            status = str(item.get("status", "")).strip()
            classification = str(item.get("classification", "")).strip()
            if status in {"open", "needs_repair"} and classification in UNSAFE_REFACTOR_REDUCIBLE_CLASSIFICATIONS:
                item_id = str(item.get("id", "")).strip()
                key = item_id or json.dumps(item, ensure_ascii=False, sort_keys=True)
                if key not in seen:
                    result.append(dict(item))
                    seen.add(key)
    return result


def _unsafe_refactor_open_item_diagnostics(open_items: list[dict[str, Any]]) -> list[str]:
    """校验 unsafe auditor 交给 repair agent 的 open item 是否可执行。"""
    diagnostics: list[str] = []
    required_text_fields = ("id", "file", "span", "problem", "repair_instruction")
    required_list_fields = ("must_preserve", "evidence")
    for index, item in enumerate(open_items, start=1):
        item_id = str(item.get("id", "")).strip() or f"<open_item_{index}>"
        for field in required_text_fields:
            if not str(item.get(field, "")).strip():
                diagnostics.append(f"unsafe refactor open item {item_id} missing required field: {field}")
        for field in required_list_fields:
            value = item.get(field)
            if not (isinstance(value, list) and any(str(entry).strip() for entry in value)):
                diagnostics.append(f"unsafe refactor open item {item_id} missing required non-empty list field: {field}")
    return diagnostics


def _unsafe_scope_review_plan(rendered_root: Path, output_dir: Path, round_label: str) -> tuple[dict[str, Any], Path, Path]:
    """生成本轮 unsafe scope 审计覆盖计划。"""
    scope_json = output_dir / f"{round_label}_unsafe_refactor_scope_inventory.json"
    scope_md = output_dir / f"{round_label}_unsafe_refactor_scope_inventory.md"
    run_unsafe_scope_gate(rendered_root, scope_json, scope_md)
    return _read_json_object(scope_json), scope_json, scope_md


def _unsafe_scope_key(scope: dict[str, Any]) -> str:
    """生成稳定 unsafe scope 覆盖 key。"""
    file_name = str(scope.get("file") or "").strip()
    scope_id = str(scope.get("id") or "").strip()
    function = str(scope.get("function") or "").strip()
    start = str(scope.get("start_line") or "").strip()
    end = str(scope.get("end_line") or "").strip()
    return "::".join([part for part in (file_name, function, scope_id, f"{start}-{end}") if part])


def _unsafe_scope_index(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    """读取 unsafe scope plan 中所有必审 scope。"""
    result: dict[str, dict[str, Any]] = {}
    scopes = plan.get("scopes")
    if not isinstance(scopes, list):
        return result
    for scope in scopes:
        if not isinstance(scope, dict):
            continue
        key = _unsafe_scope_key(scope)
        if key:
            result[key] = dict(scope)
    return result


def _unsafe_scope_tokens_for_item(item: dict[str, Any]) -> set[str]:
    """从 ledger item 提取已覆盖的 unsafe scope token。"""
    tokens: set[str] = set()
    for field in ("scope_id", "unsafe_scope_id"):
        value = str(item.get(field) or "").strip()
        if value:
            tokens.add(value)
    for field in ("scope_ids", "unsafe_scope_ids", "covered_scope_ids"):
        values = item.get(field)
        if isinstance(values, list):
            tokens.update(str(value).strip() for value in values if str(value).strip())
    evidence = item.get("evidence")
    if isinstance(evidence, list):
        for entry in evidence:
            tokens.update(re.findall(r"\bU\d{4}\b", str(entry)))
    item_id = str(item.get("id") or "")
    tokens.update(re.findall(r"\bU\d{4}\b", item_id))
    return tokens


def _unsafe_refactor_covered_scope_keys(ledger: dict[str, Any], plan: dict[str, Any]) -> set[str]:
    """从 unsafe refactor ledger 读取已覆盖的 scope key。"""
    scope_index = _unsafe_scope_index(plan)
    scopes_by_id: dict[str, list[str]] = {}
    for key, scope in scope_index.items():
        scope_id = str(scope.get("id") or "").strip()
        if scope_id:
            scopes_by_id.setdefault(scope_id, []).append(key)
    covered: set[str] = set()
    items: list[Any] = []
    for field in ("items", "open_reducible_items"):
        values = ledger.get(field)
        if isinstance(values, list):
            items.extend(values)
    for item in items:
        if not isinstance(item, dict):
            continue
        for token in _unsafe_scope_tokens_for_item(item):
            for key in scopes_by_id.get(token, []):
                covered.add(key)
        file_name = str(item.get("file") or item.get("file_path") or "").strip()
        span = str(item.get("span") or "").strip()
        function = str(item.get("function") or item.get("function_name") or "").strip()
        for key, scope in scope_index.items():
            if file_name and file_name != str(scope.get("file") or "").strip():
                continue
            if function and function != str(scope.get("function") or "").strip():
                continue
            expected_span = f"{scope.get('start_line')}-{scope.get('end_line')}"
            if span and span == expected_span:
                covered.add(key)
    return covered


def _unsafe_refactor_coverage_gaps(ledger: dict[str, Any], plan: dict[str, Any]) -> list[dict[str, Any]]:
    """计算 unsafe auditor ledger 未覆盖的 scope。"""
    scope_index = _unsafe_scope_index(plan)
    covered = _unsafe_refactor_covered_scope_keys(ledger, plan)
    return [scope_index[key] for key in scope_index if key not in covered]


def _unsafe_refactor_audit_passed(result: dict[str, Any] | None) -> bool:
    """Return true when unsafe refactor auditor has no open reducible items."""
    if not isinstance(result, dict):
        return True
    if str(result.get("mode", "")).strip() == "off":
        return True
    diagnostics = result.get("diagnostics")
    has_diagnostics = isinstance(diagnostics, list) and any(str(item).strip() for item in diagnostics)
    open_items = result.get("open_reducible_items")
    has_open = isinstance(open_items, list) and any(isinstance(item, dict) for item in open_items)
    return not has_diagnostics and not has_open and bool(result.get("accepted") or result.get("passed"))


def _run_unsafe_scope_gate(rendered_root: Path, output_dir: Path, label: str) -> dict[str, Any]:
    """运行非阻断 unsafe scope 信息 gate。"""
    json_path = output_dir / f"{label}_unsafe_scope_gate.json"
    md_path = output_dir / f"{label}_unsafe_scope_gate.md"
    summary_path = output_dir / f"{label}_unsafe_scope_gate_summary.json"
    try:
        payload = run_unsafe_scope_gate(rendered_root, json_path, md_path)
    except Exception as exc:  # noqa: BLE001
        payload = {
            "gate": UNSAFE_SCOPE_GATE,
            "mode": "informational",
            "status": "error",
            "passed": True,
            "returncode": 0,
            "json_path": str(json_path.resolve()),
            "markdown_path": str(md_path.resolve()),
            "summary_path": str(summary_path.resolve()),
            "summary": {},
            "error_type": type(exc).__name__,
            "error": str(exc),
        }
        _write_json(json_path, payload)
        md_path.parent.mkdir(parents=True, exist_ok=True)
        md_path.write_text(f"# Unsafe Scope Gate\n\nstatus: error\n\n{type(exc).__name__}: {exc}\n", encoding="utf-8")
    summary = payload.get("summary") if isinstance(payload.get("summary"), dict) else {}
    payload["summary_path"] = str(summary_path.resolve())
    _write_json(summary_path, summary)
    return payload


def _rust_native_refactor_source_roots(args: argparse.Namespace, workspace_dir: Path) -> list[Path]:
    """收集 Rust-native 重构候选扫描可用的 C/C++ 源码根目录。"""
    roots: list[Path] = []
    for candidate in (
        Path(args.source_project_root).expanduser() if str(getattr(args, "source_project_root", "") or "").strip() else None,
        workspace_dir / "c_source" / args.project,
    ):
        if candidate is None:
            continue
        resolved = candidate.resolve()
        if resolved.is_dir() and resolved not in roots:
            roots.append(resolved)
    return roots


def _write_unsafe_review_task(rendered_root: Path, output_dir: Path, label: str, *, source_roots: list[Path] | None = None, suite: str = "ohos") -> dict[str, Any]:
    """生成本轮强制 Rust-native 重构审查 JSON；函数名保留兼容旧调用点。"""
    scope_json = output_dir / f"{label}_unsafe_scope_gate.json"
    scope_md = output_dir / f"{label}_unsafe_scope_gate.md"
    review_path = output_dir / f"{label}_unsafe_review_task.json"
    abi_json = output_dir / f"{label}_abi_refactor_inventory.json"
    abi_md = output_dir / f"{label}_abi_refactor_inventory.md"
    native_review_path = output_dir / f"{label}_rust_native_refactor_task.json"
    payload = write_rust_native_refactor_task(
        rendered_root,
        scope_json,
        scope_md,
        abi_json,
        abi_md,
        native_review_path,
        source_roots=source_roots or [],
        suite=suite,
    )
    # 历史 action/tests 仍查找 *_unsafe_review_task.json；写一份同内容兼容别名。
    try:
        review_path.write_text(native_review_path.read_text(encoding="utf-8"), encoding="utf-8")
    except OSError:
        pass
    payload["review_json_path"] = str(review_path.resolve())
    payload["rust_native_review_json_path"] = str(native_review_path.resolve())
    return payload


def _unsafe_review_satisfied(review_path: str | Path | None) -> bool:
    """检查本轮 Rust-native 重构审查 JSON 是否已逐项填写。"""
    if not review_path:
        return False
    return rust_native_refactor_satisfied(Path(str(review_path)))


def _unsafe_review_satisfied_for_crate(review_path: str | Path | None, rendered_root: Path) -> bool:
    """检查 Rust-native 重构审查 JSON 已填写且对应当前 Rust 源码。"""
    if not review_path:
        return False
    return rust_native_refactor_satisfied(Path(str(review_path)), rendered_root)


def _unsafe_review_status_for_crate(review_path: str | Path | None, rendered_root: Path) -> dict[str, Any]:
    """返回 Rust-native 重构审查 JSON 相对当前 Rust 源码的完成状态。"""
    if not review_path:
        return {
            "ok": False,
            "review_json_path": "",
            "item_count": 0,
            "missing_count": 0,
            "missing_items": [],
            "diagnostics": ["unsafe review JSON path missing"],
            "fingerprint_match": None,
        }
    return rust_native_refactor_status(Path(str(review_path)), rendered_root)


def _write_unsafe_review_status(output_dir: Path, label: str, review_path: str | Path | None, rendered_root: Path) -> dict[str, Any]:
    """写出 Rust-native 重构 review 缺漏状态，供补齐轮 agent 精确读取。"""
    status = _unsafe_review_status_for_crate(review_path, rendered_root)
    status_path = output_dir / f"{label}_unsafe_review_status.json"
    status["status_json_path"] = str(status_path.resolve())
    _write_json(status_path, status)
    return status


def _format_unsafe_review_missing_preview(status: dict[str, Any], *, limit: int = 20) -> str:
    """生成紧凑缺漏预览；完整明细以 JSON 路径为准。"""
    lines: list[str] = []
    diagnostics = status.get("diagnostics") if isinstance(status.get("diagnostics"), list) else []
    for item in diagnostics[:limit]:
        lines.append(f"- diagnostic: {item}")
    missing_items = status.get("missing_items") if isinstance(status.get("missing_items"), list) else []
    for item in missing_items[:limit]:
        if not isinstance(item, dict):
            continue
        fields = ",".join(str(field) for field in item.get("missing_fields", []) if str(field).strip())
        if item.get("scope_count") or item.get("unsafe_total_lines"):
            lines.append(
                "- "
                f"{item.get('id') or '<unknown>'} "
                f"{item.get('file') or '<unknown>'} "
                f"scopes={item.get('scope_count', 0)} unsafe_lines={item.get('unsafe_total_lines', 0)} "
                f"abi_candidates={item.get('abi_candidate_count', 0)} dead_candidates={item.get('dead_symbol_candidate_count', 0)} "
                f"missing={fields or '<unknown>'}"
            )
        else:
            lines.append(
                "- "
                f"{item.get('id') or '<unknown>'} "
                f"{item.get('file') or '<unknown>'}:{item.get('start_line')}-{item.get('end_line')} "
                f"function={item.get('function') or '<unknown>'} missing={fields or '<unknown>'}"
            )
    total = int(status.get("missing_count") or 0) + len(diagnostics)
    if total > len(lines):
        lines.append(f"- ... 还有 {total - len(lines)} 项，读取 status JSON 查看完整明细。")
    return "\n".join(lines) if lines else "- <none>"


def _build_unsafe_review_completion_prompt(context_path: Path, status: dict[str, Any]) -> str:
    """构建 Rust-native 重构 review 缺项补齐任务提示词。"""
    review_json = str(status.get("review_json_path") or "")
    status_json = str(status.get("status_json_path") or "")
    missing_preview = _format_unsafe_review_missing_preview(status)
    return f"""# Rust-native Refactor Review Completion Required

上一轮 repair agent 已退出，但 mandatory Rust-native refactor review 还没有完成；不得进入下一轮 compile/semantic 前跳过此任务。

## Inputs
- agent_context_path: {context_path.resolve()}
- unsafe_review_status_json: {status_json}
- unsafe_review_task_json: {review_json}
- rust_native_refactor_status_json: {status_json}
- rust_native_refactor_task_json: {review_json}

## Current Missing Items Preview
{missing_preview}

## Required Action
1. 读取 agent context 和 rust_native_refactor_status_json。
2. 如果 status JSON 显示源码指纹不匹配，必须先调用 action `regenerate_rust_native_refactor_review`，再填写重新生成后的 rust_native_refactor_task_json。
3. 逐个文件补齐 task JSON 中所有 items 的 decision、reason、result；decision 只能是 refactored、deleted_dead_code、kept_required、kept_risky、no_change_needed。
4. 只有 review JSON 每项都完整后才能 finish status=done。

不要修改 semantic ledger/report；不要把缺漏 item 留空后 finish。
"""


def _context_has_active_unsafe_optimization(context_path: Path) -> bool:
    """判断当前 repair context 是否处在 Rust-native 重构任务轮。"""
    context = _read_json_object(context_path)
    native_info = context.get("rust_native_refactor") if isinstance(context, dict) else {}
    unsafe_info = context.get("unsafe_optimization") if isinstance(context, dict) else {}
    active_info = native_info if isinstance(native_info, dict) and native_info.get("active") else unsafe_info
    if not isinstance(active_info, dict) or not active_info.get("active"):
        return False
    review_path = str(active_info.get("review_task_json") or active_info.get("task_json") or "").strip()
    return bool(review_path and Path(review_path).expanduser().is_file())


def _complete_unsafe_review_after_agent_exit(
    *,
    repair_runner: OpenAIActionRunner,
    rendered_root: Path,
    repair_output_dir: Path,
    context_path: Path,
    review_path: str | Path | None,
    round_name: str,
) -> dict[str, Any]:
    """agent 退出后立即校验 Rust-native 重构 review，缺漏时反馈给 agent 补齐。"""
    attempts: list[dict[str, Any]] = []
    status = _write_unsafe_review_status(repair_output_dir, f"{round_name}_post_agent", review_path, rendered_root)
    initial_status = dict(status)
    for attempt_index in range(1, UNSAFE_REVIEW_COMPLETION_RETRY_LIMIT + 1):
        if status.get("ok"):
            break
        retry_prompt = _build_unsafe_review_completion_prompt(context_path, status)
        retry_name = f"{round_name}_unsafe_review_completion_{attempt_index:02d}"
        try:
            retry_run = repair_runner.run(task=retry_prompt, cwd=rendered_root, output_dir=repair_output_dir, name=retry_name)
            retry_payload = retry_run.to_dict()
        except Exception as exc:  # noqa: BLE001
            retry_payload = {
                "returncode": 127,
                "error_type": type(exc).__name__,
                "error": str(exc),
                "cwd": str(rendered_root),
                "output_dir": str(repair_output_dir),
                "command": ["openai-compatible-repair-agent", retry_name],
            }
            _write_json(repair_output_dir / f"{retry_name}.result.json", retry_payload)
        status = _write_unsafe_review_status(repair_output_dir, f"{retry_name}_post_agent", review_path, rendered_root)
        attempts.append({"attempt": attempt_index, "repair_agent": retry_payload, "unsafe_review_status": status})
    return {"satisfied": bool(status.get("ok")), "initial_status": initial_status, "final_status": status, "completion_attempts": attempts}


def _apply_unsafe_review_final_guard(
    *,
    final_summary: dict[str, Any],
    output_dir: Path,
    rendered_root: Path,
    review_path: str | Path | None,
) -> dict[str, Any]:
    """最终接受前校验 mandatory Rust-native 重构 review 是否完整。"""
    status = _write_unsafe_review_status(output_dir / "final_verify", "final_verify", review_path, rendered_root)
    if status.get("ok"):
        final_summary["unsafe_review_satisfied"] = True
        final_summary["rust_native_refactor_review_satisfied"] = True
        final_summary["unsafe_review_status_path"] = str(status.get("status_json_path") or "")
        final_summary["rust_native_refactor_status_path"] = str(status.get("status_json_path") or "")
        return final_summary
    guarded = dict(final_summary)
    blocking = guarded.get("blocking_gates") if isinstance(guarded.get("blocking_gates"), list) else []
    guarded["accepted_by_gates"] = False
    guarded["blocking_gates"] = [*blocking, "rust_native_refactor_review_completion"]
    guarded["unsafe_review_satisfied"] = False
    guarded["rust_native_refactor_review_satisfied"] = False
    guarded["unsafe_review_failure_kind"] = "unsafe_review_incomplete_after_repair_budget"
    guarded["rust_native_refactor_failure_kind"] = "rust_native_refactor_incomplete_after_repair_budget"
    guarded["unsafe_review_status_path"] = str(status.get("status_json_path") or "")
    guarded["rust_native_refactor_status_path"] = str(status.get("status_json_path") or "")
    guarded["unsafe_review_missing_count"] = int(status.get("missing_count") or 0)
    guarded["rust_native_refactor_missing_count"] = int(status.get("missing_count") or 0)
    guarded["unsafe_review_completion_retry_limit"] = UNSAFE_REVIEW_COMPLETION_RETRY_LIMIT
    diagnostics = guarded.get("diagnostics") if isinstance(guarded.get("diagnostics"), list) else []
    guarded["diagnostics"] = [*diagnostics, "mandatory Rust-native refactor review remains incomplete after available repair rounds"]
    _write_json(output_dir / "final_verify_summary.json", guarded)
    return guarded


def _write_gate_bundle(
    path: Path,
    label: str,
    cargo: dict[str, Any],
    clippy: dict[str, Any],
    ohos: dict[str, Any],
    semantic: dict[str, Any] | None = None,
    body_completeness: dict[str, Any] | None = None,
    unsafe_scope: dict[str, Any] | None = None,
    unsafe_refactor: dict[str, Any] | None = None,
    suite: str = "ohos",
) -> Path:
    """Write gate bundle for the repair agent."""
    suite = _normalize_suite(suite)
    cheap_gate_names = _cheap_gate_names_for_suite(suite)
    cargo_passed = cargo.get("returncode") == 0
    clippy_configured = "cargo_clippy" in cheap_gate_names
    clippy_passed = True if not clippy_configured else clippy.get("returncode") == 0
    clippy_status = "accepted" if clippy_configured and clippy_passed else ("rejected" if clippy_configured else "not_configured")
    ohos_configured = "ohos_rustc" in cheap_gate_names
    ohos_passed = True if not ohos_configured else ohos.get("returncode") == 0
    body_payload = body_completeness if isinstance(body_completeness, dict) else {"configured": False, "passed": True, "total_findings": 0, "findings": []}
    body_configured = BODY_COMPLETENESS_GATE_ENABLED and bool(body_payload.get("configured", True))
    body_passed = True if not body_configured else bool(body_payload.get("passed"))
    semantic_payload = semantic if isinstance(semantic, dict) else {}
    semantic_configured = str(semantic_payload.get("mode", "off")).strip() != "off"
    cheap_passed = cargo_passed and ohos_passed
    unsafe_scope_payload = unsafe_scope if cheap_passed and isinstance(unsafe_scope, dict) else {}
    unsafe_refactor_payload = unsafe_refactor if cheap_passed and isinstance(unsafe_refactor, dict) else {}
    unsafe_refactor_configured = str(unsafe_refactor_payload.get("mode", "off")).strip() != "off"
    semantic_passed = _semantic_audit_passed(semantic_payload) if semantic_configured else True
    unsafe_refactor_passed = _unsafe_refactor_audit_passed(unsafe_refactor_payload) if unsafe_refactor_configured else True
    full_gates = (
        [*cheap_gate_names]
        + ([BODY_COMPLETENESS_GATE] if body_configured else [])
        + (["semantic_audit"] if semantic_configured else [])
        + ([UNSAFE_REFACTOR_AUDIT_GATE] if unsafe_refactor_configured else [])
        + ([UNSAFE_SCOPE_GATE] if unsafe_scope_payload else [])
    )
    bundle = {
        "gate_model_version": "c2r_post_repair_gate_bundle_v1",
        "label": label,
        "suite": suite,
        "cheap_gates": cheap_gate_names,
        "full_gates": full_gates,
        "cargo": {"passed": cargo_passed, "returncode": cargo.get("returncode"), "text_log_path": cargo.get("text_log_path")},
        "cargo_clippy": {
            "passed": clippy_passed,
            "configured": clippy_configured,
            "status": clippy_status,
            "returncode": clippy.get("returncode"),
            "text_log_path": clippy.get("text_log_path"),
        },
        "ohos_rustc": {
            "passed": ohos_passed,
            "configured": ohos_configured,
            "status": "accepted" if ohos_configured and ohos_passed else ("rejected" if ohos_configured else "not_configured"),
            "returncode": ohos.get("returncode"),
            "text_log_path": ohos.get("text_log_path"),
        },
        "semantic_audit": semantic_payload,
        UNSAFE_REFACTOR_AUDIT_GATE: unsafe_refactor_payload,
        "cheap_gates_passed": cheap_passed,
        "accepted_by_gates": cheap_passed and body_passed and semantic_passed and unsafe_refactor_passed,
    }
    if unsafe_scope_payload:
        bundle[UNSAFE_SCOPE_GATE] = unsafe_scope_payload
    if body_configured:
        bundle[BODY_COMPLETENESS_GATE] = body_payload
    _write_json(path, bundle)
    return path


def _skipped_semantic_audit_payload(round_label: str, reason: str, log_path: Path) -> dict[str, Any]:
    """Write a skipped semantic audit payload for rounds blocked by cheap gates."""
    payload = {
        "gate": "semantic_audit",
        "mode": "required",
        "round": round_label,
        "status": "skipped",
        "passed": False,
        "accepted": False,
        "returncode": 0,
        "elapsed_sec": 0.0,
        "verdict": "skipped",
        "reason": reason,
        "ledger_path": "",
        "report_path": "",
        "text_log_path": str(log_path.with_suffix(".log").resolve()),
        "blocking_ids": [],
        "open_blocker_ids": [],
        "blocked_external_ids": [],
        "open_obligation_ids": [],
        "semantic_obligation_blocking_ids": [],
        "blocked_external_obligation_ids": [],
        "semantic_obligation_summary": {"total": 0, "by_status": {}},
        "semantic_obligation_count": 0,
        "diagnostics": [reason],
    }
    _write_json(log_path, payload)
    log_path.with_suffix(".log").write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return payload


def _blocking_summary(gate_bundle_path: Path) -> dict[str, Any]:
    """Summarize current blocking gates and important paths."""
    payload = _read_json_object(gate_bundle_path)
    cargo = payload.get("cargo") if isinstance(payload.get("cargo"), dict) else {}
    clippy = payload.get("cargo_clippy") if isinstance(payload.get("cargo_clippy"), dict) else {}
    ohos = payload.get("ohos_rustc") if isinstance(payload.get("ohos_rustc"), dict) else {}
    body = payload.get(BODY_COMPLETENESS_GATE) if isinstance(payload.get(BODY_COMPLETENESS_GATE), dict) else {}
    semantic = payload.get("semantic_audit") if isinstance(payload.get("semantic_audit"), dict) else {}
    unsafe_refactor = payload.get(UNSAFE_REFACTOR_AUDIT_GATE) if isinstance(payload.get(UNSAFE_REFACTOR_AUDIT_GATE), dict) else {}
    unsafe_scope = payload.get(UNSAFE_SCOPE_GATE) if isinstance(payload.get(UNSAFE_SCOPE_GATE), dict) else {}
    configured_cheap_gates = {str(item) for item in payload.get("cheap_gates", []) if str(item).strip()}
    clippy_configured = "cargo_clippy" in configured_cheap_gates
    blocking: list[str] = []
    pending: list[str] = []
    if not bool(cargo.get("passed")):
        blocking.append("cargo")
    if clippy_configured and not bool(clippy.get("passed")):
        blocking.append("cargo_clippy")
    ohos_configured = "ohos_rustc" in configured_cheap_gates
    if ohos_configured and not bool(ohos.get("passed")):
        blocking.append("ohos_rustc")
    body_configured = BODY_COMPLETENESS_GATE_ENABLED and BODY_COMPLETENESS_GATE in [str(x) for x in payload.get("full_gates", [])]
    if body_configured and not bool(body.get("passed")):
        blocking.append(BODY_COMPLETENESS_GATE)
    semantic_configured = "semantic_audit" in [str(x) for x in payload.get("full_gates", [])]
    if semantic_configured and str(semantic.get("status", "")).strip() == "skipped":
        pending.append("semantic_audit")
    elif semantic_configured and not _semantic_audit_passed(semantic):
        blocking.append("semantic_audit")
    unsafe_refactor_configured = UNSAFE_REFACTOR_AUDIT_GATE in [str(x) for x in payload.get("full_gates", [])]
    if unsafe_refactor_configured and str(unsafe_refactor.get("status", "")).strip() == "skipped":
        pending.append(UNSAFE_REFACTOR_AUDIT_GATE)
    elif unsafe_refactor_configured and not _unsafe_refactor_audit_passed(unsafe_refactor):
        blocking.append(UNSAFE_REFACTOR_AUDIT_GATE)
    open_ids = semantic.get("open_blocker_ids") if isinstance(semantic.get("open_blocker_ids"), list) else semantic.get("blocking_ids")
    external_ids = semantic.get("blocked_external_ids") if isinstance(semantic.get("blocked_external_ids"), list) else []
    open_obligation_ids = semantic.get("open_obligation_ids") if isinstance(semantic.get("open_obligation_ids"), list) else semantic.get("semantic_obligation_blocking_ids")
    external_obligation_ids = semantic.get("blocked_external_obligation_ids") if isinstance(semantic.get("blocked_external_obligation_ids"), list) else []
    obligation_summary = semantic.get("semantic_obligation_summary") if isinstance(semantic.get("semantic_obligation_summary"), dict) else {"total": 0, "by_status": {}}
    cargo_passed = bool(cargo.get("passed"))
    clippy_passed = True if not clippy_configured else bool(clippy.get("passed"))
    ohos_passed = True if not ohos_configured else bool(ohos.get("passed"))
    semantic_passed = True if not semantic_configured else _semantic_audit_passed(semantic)
    unsafe_refactor_passed = True if not unsafe_refactor_configured else _unsafe_refactor_audit_passed(unsafe_refactor)
    cheap_gates_passed = cargo_passed and ohos_passed
    accepted_by_gates = cheap_gates_passed and clippy_passed and semantic_passed and unsafe_refactor_passed
    gate_statuses = {
        "cargo": "accepted" if cargo_passed else "rejected",
        "cargo_clippy": "not_configured" if not clippy_configured else ("accepted" if clippy_passed else "rejected"),
        "ohos_rustc": "not_configured" if not ohos_configured else ("accepted" if ohos_passed else "rejected"),
        "semantic_audit": "not_configured" if not semantic_configured else ("accepted" if _semantic_audit_passed(semantic) else ("skipped" if str(semantic.get("status", "")).strip() == "skipped" else "rejected")),
        UNSAFE_REFACTOR_AUDIT_GATE: "not_configured" if not unsafe_refactor_configured else ("accepted" if _unsafe_refactor_audit_passed(unsafe_refactor) else ("skipped" if str(unsafe_refactor.get("status", "")).strip() == "skipped" else "rejected")),
    }
    if unsafe_scope and cheap_gates_passed:
        gate_statuses[UNSAFE_SCOPE_GATE] = str(unsafe_scope.get("status", "") or "available")
    summary = {
        "accepted_by_gates": accepted_by_gates,
        "cheap_gates_passed": cheap_gates_passed,
        "blocking_gates": blocking,
        "pending_gates": pending,
        "gate_statuses": gate_statuses,
        "cargo_log": cargo.get("text_log_path", ""),
        "cargo_clippy_log": clippy.get("text_log_path", ""),
        "ohos_rustc_log": ohos.get("text_log_path", ""),
        "semantic_audit_log": semantic.get("text_log_path", ""),
        "semantic_audit_status": semantic.get("status", ""),
        "semantic_blockers_ledger_path": semantic.get("ledger_path", ""),
        "semantic_audit_report_path": semantic.get("report_path", ""),
        "semantic_audit_verdict": semantic.get("verdict", ""),
        "semantic_audit_open_blocker_ids": [str(item) for item in (open_ids if isinstance(open_ids, list) else []) if str(item).strip()],
        "semantic_audit_blocked_external_ids": [str(item) for item in external_ids if str(item).strip()],
        "semantic_audit_diagnostics": semantic.get("diagnostics") if isinstance(semantic.get("diagnostics"), list) else [],
        "semantic_obligation_open_ids": [str(item) for item in (open_obligation_ids if isinstance(open_obligation_ids, list) else []) if str(item).strip()],
        "semantic_obligation_blocking_ids": [str(item) for item in (open_obligation_ids if isinstance(open_obligation_ids, list) else []) if str(item).strip()],
        "semantic_obligation_blocked_external_ids": [str(item) for item in external_obligation_ids if str(item).strip()],
        "semantic_obligation_summary": obligation_summary,
        "semantic_obligation_count": semantic.get("semantic_obligation_count", obligation_summary.get("total", 0)),
        "unsafe_refactor_audit_status": unsafe_refactor.get("status", ""),
        "unsafe_refactor_audit_verdict": unsafe_refactor.get("verdict", ""),
        "unsafe_refactor_ledger_path": unsafe_refactor.get("ledger_path", ""),
        "unsafe_refactor_report_path": unsafe_refactor.get("report_path", ""),
        "unsafe_refactor_open_item_count": int(unsafe_refactor.get("open_reducible_item_count", 0) or 0),
        "unsafe_refactor_open_reducible_items": unsafe_refactor.get("open_reducible_items") if isinstance(unsafe_refactor.get("open_reducible_items"), list) else [],
        "unsafe_refactor_feedback_paths": unsafe_refactor.get("feedback_paths") if isinstance(unsafe_refactor.get("feedback_paths"), list) else [],
    }
    if unsafe_scope and cheap_gates_passed:
        summary.update(
            {
                "unsafe_scope_gate_status": gate_statuses[UNSAFE_SCOPE_GATE],
                "unsafe_scope_gate_json": unsafe_scope.get("json_path", ""),
                "unsafe_scope_gate_markdown": unsafe_scope.get("markdown_path", ""),
                "unsafe_scope_gate_summary_path": unsafe_scope.get("summary_path", ""),
                "unsafe_scope_summary": unsafe_scope.get("summary") if isinstance(unsafe_scope.get("summary"), dict) else {},
            }
        )
    if body_configured:
        gate_statuses[BODY_COMPLETENESS_GATE] = "accepted" if bool(body.get("passed")) else "rejected"
        summary[BODY_COMPLETENESS_GATE] = {
            "configured": body_configured,
            "passed": bool(body.get("passed")),
            "total_findings": int(body.get("total_findings", 0) or 0),
            "findings": body.get("findings") if isinstance(body.get("findings"), list) else [],
            "truncated": bool(body.get("truncated")),
            "scope": body.get("scope", ""),
            "target_count": body.get("target_count", 0),
            "unmatched_target_count": body.get("unmatched_target_count", 0),
            "target_scope_path": body.get("target_scope_path", ""),
            "target_scope_source": body.get("target_scope_source", ""),
        }
    return summary


def _semantic_audit_result_executed(semantic: dict[str, Any] | None) -> bool:
    """Return true when semantic audit really ran instead of being skipped."""
    if not isinstance(semantic, dict):
        return False
    if str(semantic.get("mode", "")).strip() == "off":
        return False
    if str(semantic.get("status", "")).strip() == "skipped":
        return False
    diagnostics = semantic.get("diagnostics")
    if isinstance(diagnostics, list) and any(str(item).strip() for item in diagnostics):
        return False
    return str(semantic.get("verdict", "")).strip() in {"accepted", "rejected", "accepted_with_residual_risks"}


def _semantic_summary_has_context(summary: dict[str, Any]) -> bool:
    """Return true when a summary carries usable semantic audit context."""
    if not isinstance(summary, dict):
        return False
    if str(summary.get("semantic_audit_status", "")).strip() == "skipped":
        return False
    diagnostics = summary.get("semantic_audit_diagnostics")
    if isinstance(diagnostics, list) and any(str(item).strip() for item in diagnostics):
        return False
    list_keys = (
        "semantic_audit_open_blocker_ids",
        "semantic_audit_blocked_external_ids",
        "semantic_obligation_open_ids",
        "semantic_obligation_blocking_ids",
        "semantic_obligation_blocked_external_ids",
    )
    if any(isinstance(summary.get(key), list) and any(str(item).strip() for item in summary[key]) for key in list_keys):
        return True
    try:
        obligation_count = int(summary.get("semantic_obligation_count", 0) or 0)
    except (TypeError, ValueError):
        obligation_count = 0
    return bool(
        str(summary.get("semantic_blockers_ledger_path", "")).strip()
        or str(summary.get("semantic_audit_report_path", "")).strip()
        or obligation_count > 0
    )


def _snapshot_project(root: Path) -> dict[str, str]:
    """Hash Rust project files to detect read-only audit mutations."""
    snapshot: dict[str, str] = {}
    if not root.is_dir():
        return snapshot
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        rel = path.relative_to(root).as_posix()
        if rel.startswith("target/") or "/target/" in rel:
            continue
        try:
            snapshot[rel] = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError:
            continue
    return snapshot


def _project_mutations(before: dict[str, str], after: dict[str, str]) -> list[str]:
    """Return changed files between two snapshots."""
    return [key for key in sorted(set(before) | set(after)) if before.get(key) != after.get(key)]


def _semantic_audit_payload_from_ledger(
    *,
    round_label: str,
    ledger_path: Path,
    report_path: Path,
    log_path: Path,
    agent_payload: dict[str, Any],
    elapsed_sec: float,
    mutations: list[str],
) -> dict[str, Any]:
    """把 semantic ledger 转成 gate payload，并校验结构完整性。"""
    ledger = _read_json_object(ledger_path)
    open_ids = _semantic_blocker_ids_by_status(ledger, "open") if ledger else []
    external_ids = _semantic_blocker_ids_by_status(ledger, "blocked_external") if ledger else []
    open_obligations = _semantic_obligation_ids_by_status(ledger, {"blocking_mismatch"}) if ledger else []
    external_obligations = _semantic_obligation_ids_by_status(ledger, {"blocked_external"}) if ledger else []
    obligation_summary = _semantic_obligation_summary(ledger) if ledger else {"total": 0, "by_status": {}}
    verdict = str(ledger.get("verdict", "")).strip() if ledger else ""
    diagnostics: list[str] = []
    if _agent_returncode(agent_payload) != 0:
        diagnostics.append(f"semantic audit agent failed: returncode={agent_payload.get('returncode')}")
    if not ledger:
        diagnostics.append(f"semantic audit ledger missing or invalid: {ledger_path}")
    elif str(ledger.get("schema_version", "")).strip() != "rust_semantic_blockers_ledger_v1":
        diagnostics.append(f"semantic audit ledger schema invalid: {ledger_path}")
    if ledger and verdict not in {"accepted", "rejected", "accepted_with_residual_risks"}:
        diagnostics.append(f"semantic audit verdict invalid: {verdict or '<empty>'}")
    elif not ledger:
        diagnostics.append("semantic audit verdict invalid: <empty>")
    if ledger:
        diagnostics.extend(_semantic_coverage_summary_diagnostics(ledger))
    if mutations:
        diagnostics.append("semantic audit modified Rust project files: " + ", ".join(mutations[:12]))
    if ledger and not report_path.is_file():
        diagnostics.append(f"semantic audit report missing: {report_path}")
    accepted = not diagnostics and verdict in {"accepted", "accepted_with_residual_risks"} and not open_ids and not open_obligations
    return {
        "gate": "semantic_audit",
        "mode": "required",
        "round": round_label,
        "status": "accepted" if accepted else "rejected",
        "passed": accepted,
        "accepted": accepted,
        "returncode": 0 if accepted else 1,
        "elapsed_sec": elapsed_sec,
        "verdict": verdict or "rejected",
        "ledger_path": str(ledger_path.resolve()),
        "report_path": str(report_path.resolve()),
        "text_log_path": str(log_path.with_suffix(".log").resolve()),
        "blocking_ids": list(dict.fromkeys([*open_ids, *open_obligations])),
        "open_blocker_ids": open_ids,
        "blocked_external_ids": external_ids,
        "open_obligation_ids": open_obligations,
        "semantic_obligation_blocking_ids": open_obligations,
        "blocked_external_obligation_ids": external_obligations,
        "semantic_obligation_summary": obligation_summary,
        "semantic_obligation_count": int(obligation_summary.get("total", 0) or 0),
        "agent_result": agent_payload,
        "diagnostics": diagnostics,
    }


def _semantic_coverage_summary_diagnostics(ledger: dict[str, Any]) -> list[str]:
    """轻量校验 accepted semantic ledger 的 coverage_summary 格式。"""
    if not isinstance(ledger, dict) or not ledger:
        return []
    verdict = str(ledger.get("verdict", "") or "").strip()
    if verdict not in {"accepted", "accepted_with_residual_risks"}:
        return []
    coverage_summary = ledger.get("coverage_summary")
    if not isinstance(coverage_summary, dict):
        return ["semantic audit coverage_summary must be an object for accepted or residual verdict"]
    diagnostics: list[str] = []
    required_count_fields = (
        "rust_source_files_scanned",
        "translated_functions_scanned",
        "public_or_exported_items_scanned",
        "private_helpers_scanned",
        "state_or_layout_items_scanned",
        "reviewed_non_observable",
    )
    for field in required_count_fields:
        value = coverage_summary.get(field)
        if not isinstance(value, int) or isinstance(value, bool) or value < 0:
            diagnostics.append(f"semantic audit coverage_summary.{field} must be a non-negative integer")
    unmapped_items = coverage_summary.get("unmapped_items")
    if not isinstance(unmapped_items, list):
        diagnostics.append("semantic audit coverage_summary.unmapped_items must be a list")
    return diagnostics


def _semantic_seed_id(prefix: str, value: object) -> str:
    """生成稳定 semantic review seed id。"""
    raw = str(value or "").strip()
    sanitized = re.sub(r"[^A-Za-z0-9_.:/-]+", "_", raw).strip("_")
    return f"{prefix}::{sanitized or 'unknown'}"


def _dedupe_text(values: list[str]) -> list[str]:
    """按原顺序去重非空字符串。"""
    result: list[str] = []
    seen: set[str] = set()
    for value in values:
        text = str(value or "").strip()
        if not text or text in seen:
            continue
        seen.add(text)
        result.append(text)
    return result


def _read_agent_context(context_path: Path) -> dict[str, Any]:
    """读取 semantic audit 可用的 agent context。"""
    return _read_json_object(context_path) if context_path.is_file() else {}


def _rust_review_seed_cards(rendered_root: Path) -> list[dict[str, Any]]:
    """从最终 Rust crate 生成轻量源码覆盖 seed。"""
    cards: list[dict[str, Any]] = []
    if not rendered_root.is_dir():
        return cards
    for path in sorted(rendered_root.rglob("*.rs")):
        try:
            rel = path.relative_to(rendered_root).as_posix()
        except ValueError:
            continue
        if rel.startswith("target/") or "/target/" in rel:
            continue
        cards.append(
            {
                "seed_id": _semantic_seed_id("rust_file", rel),
                "kind": "rust_file",
                "path": str(path.resolve()),
                "relative_path": rel,
                "review_dimensions": [
                    "public_or_project_observable_behavior",
                    "state_resource_behavior",
                    "ffi_or_dependency_boundary",
                    "helper_effect_on_observable_paths",
                ],
            }
        )
    return cards


def _source_evidence_review_seed_cards(context: dict[str, Any]) -> list[dict[str, Any]]:
    """从 agent context 的原始源码证据生成 semantic review seed。"""
    info = context.get("information_paths") if isinstance(context.get("information_paths"), dict) else {}
    evidence = info.get("resolved_source_evidence") if isinstance(info.get("resolved_source_evidence"), dict) else {}
    cards: list[dict[str, Any]] = []
    for field in ("production_sources", "public_headers", "test_or_example_usage", "unresolved_sources"):
        values = evidence.get(field)
        if not isinstance(values, list):
            continue
        for index, item in enumerate(values):
            if not isinstance(item, dict):
                continue
            path_text = str(item.get("relative_path") or item.get("path") or item.get("source_key") or f"{field}_{index}").strip()
            if not path_text:
                continue
            cards.append(
                {
                    "seed_id": _semantic_seed_id(f"source::{field}", path_text),
                    "kind": field,
                    "path": str(item.get("path") or ""),
                    "relative_path": str(item.get("relative_path") or ""),
                    "source_key": str(item.get("source_key") or ""),
                    "origin": str(item.get("origin") or ""),
                    "review_dimensions": [
                        "callable_surface",
                        "return_or_error_behavior",
                        "state_or_resource_lifetime",
                        "layout_or_boundary_contract",
                    ],
                }
            )
    return cards


def _build_semantic_review_plan(*, rendered_root: Path, context_path: Path, output_dir: Path) -> tuple[dict[str, Any], Path]:
    """构建当前框架的轻量 semantic 覆盖计划。"""
    context = _read_agent_context(context_path)
    source_cards = _source_evidence_review_seed_cards(context)
    has_standard_context = str(context.get("schema_version") or "") == "c2r_post_repair_agent_context_v1"
    rust_cards = _rust_review_seed_cards(rendered_root) if source_cards or has_standard_context else []
    seeds = [*source_cards, *rust_cards]
    deduped: list[dict[str, Any]] = []
    seen: set[str] = set()
    for item in seeds:
        seed_id = str(item.get("seed_id") or "").strip()
        if not seed_id or seed_id in seen:
            continue
        seen.add(seed_id)
        deduped.append(item)
    plan = {
        "schema_version": "c2r_semantic_review_plan_v1",
        "policy": "coverage gaps trigger semantic audit continuation; remaining gaps reject this semantic gate after the continuation budget.",
        "seed_count": len(deduped),
        "seeds": deduped,
        "summary": {
            "source_seed_count": len(source_cards),
            "rust_seed_count": len(rust_cards),
        },
    }
    plan_path = output_dir / "semantic_review_plan.json"
    _write_json(plan_path, plan)
    return plan, plan_path


def _semantic_review_required_seed_ids(plan: dict[str, Any]) -> list[str]:
    """读取 semantic review plan 必须检查的 seed ids。"""
    seeds = plan.get("seeds")
    if not isinstance(seeds, list):
        return []
    return _dedupe_text([str(item.get("seed_id") or "") for item in seeds if isinstance(item, dict)])


def _semantic_review_covered_seed_ids(ledger: dict[str, Any]) -> list[str]:
    """从 semantic obligations 读取已覆盖 seed ids。"""
    result: list[str] = []
    for obligation in _semantic_obligations(ledger):
        seed_ids = obligation.get("seed_ids")
        if isinstance(seed_ids, list):
            result.extend(str(item) for item in seed_ids)
    return _dedupe_text(result)


def _semantic_review_coverage_gaps(ledger: dict[str, Any], plan: dict[str, Any]) -> list[str]:
    """计算 semantic review plan 的未覆盖 seed ids。"""
    required = _semantic_review_required_seed_ids(plan)
    covered = set(_semantic_review_covered_seed_ids(ledger))
    return [seed_id for seed_id in required if seed_id not in covered]


def _semantic_review_coverage_gap_diagnostic(coverage_gaps: list[str]) -> str:
    """生成 semantic coverage gap 阻断诊断。"""
    preview = ", ".join(coverage_gaps[:12])
    suffix = f"; showing first 12 of {len(coverage_gaps)}" if len(coverage_gaps) > 12 else ""
    return f"semantic audit coverage gaps remain after continuation: {preview}{suffix}"


def _semantic_blocker_ids_by_status(ledger: dict[str, Any], status: str) -> list[str]:
    """Read blocker ids by status."""
    blockers = ledger.get("blockers")
    if not isinstance(blockers, list):
        return []
    result: list[str] = []
    for item in blockers:
        if not isinstance(item, dict):
            continue
        if str(item.get("status", "")).strip() == status and str(item.get("id", "")).strip():
            result.append(str(item["id"]).strip())
    return result


def _semantic_obligations(ledger: dict[str, Any]) -> list[dict[str, Any]]:
    """Read semantic obligations."""
    obligations = ledger.get("semantic_obligations")
    if obligations is None and isinstance(ledger.get("obligation_matrix"), dict):
        obligations = ledger["obligation_matrix"].get("obligations")
    return [item for item in obligations if isinstance(item, dict)] if isinstance(obligations, list) else []


def _semantic_obligation_ids_by_status(ledger: dict[str, Any], statuses: set[str]) -> list[str]:
    """Read obligation ids by status."""
    result: list[str] = []
    for item in _semantic_obligations(ledger):
        status = str(item.get("status", item.get("verdict", ""))).strip()
        if status in statuses and str(item.get("id", "")).strip():
            result.append(str(item["id"]).strip())
    return list(dict.fromkeys(result))


def _semantic_obligation_summary(ledger: dict[str, Any]) -> dict[str, Any]:
    """Summarize obligations by status."""
    counts: dict[str, int] = {}
    obligations = _semantic_obligations(ledger)
    for item in obligations:
        status = str(item.get("status", item.get("verdict", "unreviewed"))).strip() or "unreviewed"
        counts[status] = counts.get(status, 0) + 1
    return {"total": len(obligations), "by_status": counts}


def _read_semantic_ledger_from_summary(summary: dict[str, Any]) -> dict[str, Any]:
    """Read semantic ledger referenced by a blocking summary."""
    ledger_path = str(summary.get("semantic_blockers_ledger_path", "") or "").strip()
    if not ledger_path:
        return {}
    return _read_json_object(Path(ledger_path))


def _root_cause_clusters_by_status(ledger: dict[str, Any], statuses: set[str]) -> list[dict[str, Any]]:
    """Read root-cause cluster objects by status."""
    clusters = ledger.get("root_cause_clusters")
    if not isinstance(clusters, list):
        return []
    result: list[dict[str, Any]] = []
    seen: set[str] = set()
    for item in clusters:
        if not isinstance(item, dict):
            continue
        status = str(item.get("status", "")).strip()
        item_id = str(item.get("id", "")).strip()
        if status in statuses and item_id and item_id not in seen:
            result.append(dict(item))
            seen.add(item_id)
    return result


def _semantic_obligation_items_by_status(ledger: dict[str, Any], statuses: set[str]) -> list[dict[str, Any]]:
    """Read semantic obligation objects by status."""
    result: list[dict[str, Any]] = []
    seen: set[str] = set()
    for item in _semantic_obligations(ledger):
        status = str(item.get("status", item.get("verdict", ""))).strip()
        item_id = str(item.get("id", "")).strip()
        if status in statuses and item_id and item_id not in seen:
            result.append(dict(item))
            seen.add(item_id)
    return result


def _synthetic_blocking_obligations(summary: dict[str, Any]) -> list[dict[str, Any]]:
    """Build minimal obligation records when old gate bundles only expose ids."""
    ids = summary.get("semantic_obligation_blocking_ids")
    if not isinstance(ids, list):
        ids = summary.get("semantic_obligation_open_ids")
    result: list[dict[str, Any]] = []
    for item in ids if isinstance(ids, list) else []:
        item_id = str(item).strip()
        if item_id:
            result.append({"id": item_id, "status": "blocking_mismatch"})
    return result


def _build_semantic_repair_bundle(summary: dict[str, Any]) -> dict[str, Any]:
    """Build the fixed semantic contract consumed by the repair agent."""
    ledger = _read_semantic_ledger_from_summary(summary)
    open_clusters = _root_cause_clusters_by_status(ledger, {"open"})
    blocked_external_clusters = _root_cause_clusters_by_status(ledger, {"blocked_external"})
    fixed_clusters = _root_cause_clusters_by_status(ledger, {"fixed"})
    blocking_obligations = _semantic_obligation_items_by_status(ledger, {"blocking_mismatch"})
    blocked_external_obligations = _semantic_obligation_items_by_status(ledger, {"blocked_external"})
    if not blocking_obligations:
        blocking_obligations = _synthetic_blocking_obligations(summary)
    active = _semantic_summary_has_context(summary)
    verdict = str(ledger.get("verdict", "") or summary.get("semantic_audit_verdict", "") or "").strip()
    if not verdict:
        verdict = "rejected" if active else "skipped"
    repair_root_cause_clusters = open_clusters
    repair_obligations = blocking_obligations
    return {
        "active": active,
        "semantic_locked": bool(active and not repair_root_cause_clusters),
        "verdict": verdict,
        "open_root_cause_clusters": open_clusters,
        "blocked_external_root_cause_clusters": blocked_external_clusters,
        "repair_root_cause_clusters": repair_root_cause_clusters,
        "fixed_root_cause_clusters": fixed_clusters,
        "blocking_obligations": blocking_obligations,
        "blocked_external_obligations": blocked_external_obligations,
        "repair_obligations": repair_obligations,
    }


def _current_blocking_gate(summary: dict[str, Any]) -> str:
    """Read the current gate that blocks progress."""
    blocking = summary.get("blocking_gates")
    if isinstance(blocking, list):
        for item in blocking:
            text = str(item).strip()
            if text:
                return text
    pending = summary.get("pending_gates")
    if isinstance(pending, list):
        for item in pending:
            text = str(item).strip()
            if text:
                return text
    return ""


def _build_repair_progress(current_gates: dict[str, Any], semantic_bundle: dict[str, Any]) -> dict[str, Any]:
    """Build per-round repair progress fields for the repair agent."""
    gate = _current_blocking_gate(current_gates)
    repair_clusters = semantic_bundle.get("repair_root_cause_clusters")
    repair_obligations = semantic_bundle.get("repair_obligations")
    has_repair_clusters = isinstance(repair_clusters, list) and bool(repair_clusters)
    semantic_locked = bool(semantic_bundle.get("semantic_locked"))
    if bool(semantic_bundle.get("active")) and has_repair_clusters and not semantic_locked:
        phase = "semantic_root_cause_repair"
    elif gate in {"cargo", "cargo_clippy", "ohos_rustc"}:
        phase = "cheap_gate_repair"
    elif gate == "semantic_audit" and has_repair_clusters:
        phase = "semantic_root_cause_repair"
    else:
        phase = "compile_or_full_gate_repair"

    focus: list[str] = []
    if gate:
        focus.append(f"gate:{gate}")
    if isinstance(repair_clusters, list):
        for item in repair_clusters[:8]:
            if isinstance(item, dict) and str(item.get("id", "")).strip():
                focus.append(f"root_cause:{str(item['id']).strip()}")
    if isinstance(repair_obligations, list):
        for item in repair_obligations[:8]:
            if isinstance(item, dict) and str(item.get("id", "")).strip():
                focus.append(f"obligation:{str(item['id']).strip()}")

    return {
        "current_blocking_gate": gate,
        "phase": phase,
        "last_round_changed_files": [],
        "last_round_failed_attempts": [],
        "recommended_focus": list(dict.fromkeys(focus)),
        "do_not_reaudit_semantics": True,
    }


def _build_semantic_audit_retry_prompt(context_path: Path, payload: dict[str, Any]) -> str:
    """构造 semantic audit 结构产物修正提示词。"""
    diagnostics = payload.get("diagnostics") if isinstance(payload.get("diagnostics"), list) else []
    diagnostic_lines = "\n".join(f"- {item}" for item in diagnostics if str(item).strip()) or "- <none>"
    return f"""# Semantic Audit Output Fix Required

当前 action loop 刚刚 finish，但 semantic audit 输出未通过结构完整性校验。继续当前任务，不要修改 Rust 源码，只修正 semantic_blockers_ledger.json 和 semantic_audit_report.md。

## Inputs
- agent_context_path: {context_path.resolve()}
- semantic_blockers_ledger_json: {payload.get('ledger_path', '')}
- semantic_audit_report: {payload.get('report_path', '')}

## Diagnostics
{diagnostic_lines}

## Required Action
1. 读取 agent context、现有 ledger/report 和 diagnostics。
2. 按原 semantic audit contract 补齐或修正 ledger/report。
3. 如果发现 high-confidence public observable mismatch，verdict=rejected，写 blocking_mismatch obligation 和 open root_cause_cluster。
4. 如果没有 open blocker，verdict=accepted 或 accepted_with_residual_risks，且不能留下 blocking_mismatch obligation 或 open root_cause_cluster。
5. verdict 为 accepted 或 accepted_with_residual_risks 时，必须补齐 coverage_summary 的所有计数字段和 unmapped_items 列表。
6. 完成前重新读取 semantic_blockers_ledger.json，确认文件非空、可解析 JSON、schema_version/verdict 合法，report 路径存在。
7. finish status=done。
    """


def _build_semantic_coverage_continuation_prompt(
    *,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    plan_path: Path,
    missing_seed_ids: list[str],
) -> str:
    """构造 semantic audit coverage gap 补审提示词。"""
    return f"""# Semantic Audit Coverage Continuation

当前 semantic audit 已完成一次，但 semantic_review_plan 仍有未覆盖 seed。继续当前 semantic audit，不修改 Rust 源码，只补审缺口并更新同一个 semantic_blockers_ledger.json 和 semantic_audit_report.md。

## Inputs
- agent_context_path: {context_path.resolve()}
- semantic_blockers_ledger_json: {ledger_path.resolve()}
- semantic_audit_report: {report_path.resolve()}
- semantic_review_plan: {plan_path.resolve()}

## Missing Seed IDs
{json.dumps(missing_seed_ids, ensure_ascii=False, indent=2)}

## Required Action
1. 读取 agent context、semantic_review_plan、现有 ledger/report。
2. 只围绕 Missing Seed IDs 补查；不要删除已有 blocker、root_cause_cluster 或 semantic_obligation。
3. 对每个能判定的 seed，在 semantic_obligations 中补 `seed_ids`，并写原始 C/C++ 证据与 Rust 证据。
4. 如果证据不足或本轮无法证明，不要硬造 blocker；写入带 `seed_ids` 的 uncertain obligation，并可补充 residual_risks。只写 residual_risks 不会清除 coverage gap。
5. 如果发现 high-confidence semantic mismatch，verdict=rejected，写 blocking_mismatch obligation 和 open root_cause_cluster。
6. 续审预算耗尽后仍未覆盖的 seed 会阻断 semantic gate；不要把未审 seed 留给后续流程。
7. 完成前重新读取 ledger，确认 JSON 合法且已有条目未被静默删除。
8. finish status=done。
"""


def _continue_runner(
    runner: OpenAIActionRunner,
    *,
    task: str,
    cwd: Path,
    output_dir: Path,
    name: str,
    base_name: str,
) -> Any:
    """优先复用已有 runner 会话；不支持时退回普通 run。"""
    continue_run = getattr(runner, "continue_run", None)
    if callable(continue_run):
        return continue_run(task=task, cwd=cwd, output_dir=output_dir, name=name, base_name=base_name, allowed_write_paths=())
    return runner.run(task=task, cwd=cwd, output_dir=output_dir, name=name)


def _continue_semantic_audit_coverage(
    *,
    runner: OpenAIActionRunner,
    rendered_root: Path,
    output_dir: Path,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    plan_path: Path,
    plan: dict[str, Any],
    continuation_runs: list[dict[str, Any]],
) -> list[str]:
    """补审 semantic coverage gaps；预算耗尽后返回仍会阻断的 residual gaps。"""
    ledger = _read_json_object(ledger_path)
    coverage_gaps = _semantic_review_coverage_gaps(ledger, plan) if ledger else []
    attempt = 1
    while coverage_gaps and attempt <= SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT:
        prompt = _build_semantic_coverage_continuation_prompt(
            context_path=context_path,
            ledger_path=ledger_path,
            report_path=report_path,
            plan_path=plan_path,
            missing_seed_ids=coverage_gaps,
        )
        name = f"semantic_audit_coverage_{attempt:02d}"
        try:
            run_result = _continue_runner(
                runner,
                task=prompt,
                cwd=rendered_root,
                output_dir=output_dir,
                name=name,
                base_name="semantic_audit",
            )
            continuation_runs.append(run_result.to_dict())
        except Exception as exc:  # noqa: BLE001
            continuation_runs.append({"returncode": 127, "name": name, "error_type": type(exc).__name__, "error": str(exc)})
            break
        ledger = _read_json_object(ledger_path)
        if not ledger:
            break
        coverage_gaps = _semantic_review_coverage_gaps(ledger, plan)
        if _agent_returncode(continuation_runs[-1]) != 0:
            break
        attempt += 1
    return coverage_gaps


def _agent_returncode(payload: dict[str, Any], default: int = 1) -> int:
    """Read an agent return code without treating 0 as missing."""
    try:
        return int(payload["returncode"])
    except (KeyError, TypeError, ValueError):
        return default


def _run_semantic_audit(
    *,
    args: argparse.Namespace,
    runner: OpenAIActionRunner,
    round_label: str,
    rendered_root: Path,
    manifest_path: Path,
    context_path: Path,
    log_path: Path,
) -> dict[str, Any]:
    """Run read-only semantic audit agent and read ledger."""
    output_dir = log_path.parent / log_path.stem
    output_dir.mkdir(parents=True, exist_ok=True)
    ledger_path = log_path.parent.parent / "semantic_blockers_ledger.json"
    report_path = output_dir / "semantic_audit_report.md"
    attempts: list[dict[str, Any]] = []
    payload: dict[str, Any] = {}
    cumulative_mutations: list[str] = []
    semantic_review_plan, semantic_review_plan_path = _build_semantic_review_plan(
        rendered_root=rendered_root,
        context_path=context_path,
        output_dir=output_dir,
    )
    semantic_review_continuations: list[dict[str, Any]] = []

    prompt = _build_semantic_audit_prompt(
        rendered_root=rendered_root,
        manifest_path=manifest_path,
        context_path=context_path,
        ledger_path=ledger_path,
        report_path=report_path,
        semantic_review_plan_path=semantic_review_plan_path,
        suite=str(getattr(args, "suite", "ohos") or "ohos"),
    )
    name = "semantic_audit"
    before = _snapshot_project(rendered_root)
    started = time.time()

    def build_payload(agent_payload: dict[str, Any]) -> dict[str, Any]:
        """Build and record one structural semantic audit attempt."""
        nonlocal payload
        after = _snapshot_project(rendered_root)
        mutations = _project_mutations(before, after)
        for item in mutations:
            if item not in cumulative_mutations:
                cumulative_mutations.append(item)
        payload = _semantic_audit_payload_from_ledger(
            round_label=round_label,
            ledger_path=ledger_path,
            report_path=report_path,
            log_path=log_path,
            agent_payload=agent_payload,
            elapsed_sec=round(time.time() - started, 3),
            mutations=cumulative_mutations,
        )
        attempt_index = len(attempts)
        attempts.append({"attempt": attempt_index, "name": name, "semantic_audit": dict(payload)})
        _progress(
            f"{round_label}: semantic audit finish attempt={attempt_index + 1}/{SEMANTIC_AUDIT_RETRY_LIMIT + 1} name={name} status={payload.get('status')} "
            f"verdict={payload.get('verdict')} open_blockers={len(payload.get('open_blocker_ids', []))} "
            f"open_obligations={len(payload.get('open_obligation_ids', []))} diagnostics={len(payload.get('diagnostics', []))} "
            f"elapsed={payload.get('elapsed_sec')}s ledger={payload.get('ledger_path')} report={payload.get('report_path')}"
        )
        return payload

    def finish_validator(state: dict[str, Any]) -> str | None:
        """Continue the same action loop when the audit artifact is structurally invalid."""
        finish_status = str(state.get("finish_status", "")).strip()
        agent_payload = {
            "returncode": 0 if finish_status in {"done", "complete", "completed", "submitted"} else 1,
            "finish_status": finish_status,
            "finish_message": str(state.get("finish_message", "")).strip(),
            "step": state.get("step"),
            "elapsed_sec": state.get("elapsed_sec"),
        }
        attempt_payload = build_payload(agent_payload)
        if not attempt_payload.get("diagnostics"):
            return None
        if len(attempts) <= SEMANTIC_AUDIT_RETRY_LIMIT:
            return _build_semantic_audit_retry_prompt(context_path, attempt_payload)
        return None

    _progress(f"{round_label}: semantic audit start retry_limit={SEMANTIC_AUDIT_RETRY_LIMIT} name={name}")
    try:
        run_result = runner.run(task=prompt, cwd=rendered_root, output_dir=output_dir, name=name, finish_validator=finish_validator)
        agent_payload = run_result.to_dict()
        if attempts:
            payload["agent_result"] = agent_payload
            attempts[-1]["semantic_audit"] = dict(payload)
        else:
            payload = build_payload(agent_payload)
    except Exception as exc:  # noqa: BLE001
        agent_payload = {"returncode": 127, "error_type": type(exc).__name__, "error": str(exc)}
        payload = build_payload(agent_payload)

    coverage_gaps = _continue_semantic_audit_coverage(
        runner=runner,
        rendered_root=rendered_root,
        output_dir=output_dir,
        context_path=context_path,
        ledger_path=ledger_path,
        report_path=report_path,
        plan_path=semantic_review_plan_path,
        plan=semantic_review_plan,
        continuation_runs=semantic_review_continuations,
    )
    if semantic_review_continuations:
        after = _snapshot_project(rendered_root)
        mutations = _project_mutations(before, after)
        for item in mutations:
            if item not in cumulative_mutations:
                cumulative_mutations.append(item)
        payload = _semantic_audit_payload_from_ledger(
            round_label=round_label,
            ledger_path=ledger_path,
            report_path=report_path,
            log_path=log_path,
            agent_payload=agent_payload,
            elapsed_sec=round(time.time() - started, 3),
            mutations=cumulative_mutations,
        )
    if coverage_gaps:
        diagnostics = payload.setdefault("diagnostics", [])
        diagnostics.append(_semantic_review_coverage_gap_diagnostic(coverage_gaps))
        payload["status"] = "rejected"
        payload["passed"] = False
        payload["accepted"] = False
        payload["returncode"] = 1
    payload["audit_attempts"] = attempts
    payload["retry_limit"] = SEMANTIC_AUDIT_RETRY_LIMIT
    payload["semantic_review_plan"] = str(semantic_review_plan_path.resolve())
    payload["semantic_review_plan_summary"] = semantic_review_plan.get("summary", {})
    payload["semantic_review_coverage_gaps"] = coverage_gaps
    payload["semantic_review_continuations"] = semantic_review_continuations
    _write_json(log_path, payload)
    log_path.with_suffix(".log").write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return payload


def _collect_unsafe_refactor_feedback_paths(output_dir: Path) -> list[str]:
    """收集 repair agent 写出的 unsafe auditor 反馈文件。"""
    paths: list[str] = []
    for path in sorted((output_dir / "agentic_logs").glob("repair_round_*/unsafe_refactor_feedback.jsonl")):
        if path.is_file():
            paths.append(str(path.resolve()))
    return paths


def _unsafe_refactor_audit_payload_from_ledger(
    *,
    round_label: str,
    ledger_path: Path,
    report_path: Path,
    log_path: Path,
    agent_payload: dict[str, Any],
    elapsed_sec: float,
    expected_fingerprint: str,
    mutations: list[str],
    feedback_paths: list[str],
    coverage_gaps: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    """把 unsafe auditor ledger 转成 gate payload，并只做结构完整性校验。"""
    ledger = _read_json_object(ledger_path)
    diagnostics: list[str] = []
    if _agent_returncode(agent_payload) != 0:
        diagnostics.append(f"unsafe auditor agent failed: returncode={agent_payload.get('returncode')}")
    if not ledger:
        diagnostics.append(f"unsafe refactor ledger missing or invalid: {ledger_path}")
    elif str(ledger.get("schema_version", "")).strip() != "c2r_unsafe_refactor_ledger_v1":
        diagnostics.append(f"unsafe refactor ledger schema invalid: {ledger_path}")
    verdict = str((ledger.get("verdict") or ledger.get("status") or "") if ledger else "").strip()
    if ledger and verdict not in {"accepted", "needs_repair", "rejected"}:
        diagnostics.append(f"unsafe refactor verdict invalid: {verdict or '<empty>'}")
    ledger_fingerprint = str(ledger.get("source_fingerprint_sha256", "") if ledger else "").strip()
    if ledger and ledger_fingerprint != expected_fingerprint:
        diagnostics.append(f"unsafe refactor ledger source fingerprint missing or stale: expected {expected_fingerprint}")
    open_items = _unsafe_refactor_open_items(ledger) if ledger else []
    diagnostics.extend(_unsafe_refactor_open_item_diagnostics(open_items))
    if ledger and verdict == "accepted" and open_items:
        diagnostics.append("unsafe refactor ledger is accepted but still has open reducible items")
    if ledger and verdict == "accepted" and not bool(ledger.get("done")):
        diagnostics.append("unsafe refactor ledger is accepted but done is not true")
    if ledger and verdict == "needs_repair" and not open_items:
        diagnostics.append("unsafe refactor ledger needs_repair but open_reducible_items is empty")
    if mutations:
        diagnostics.append("unsafe refactor auditor modified Rust project files: " + ", ".join(mutations[:12]))
    if ledger and not report_path.is_file():
        diagnostics.append(f"unsafe refactor report missing: {report_path}")
    coverage_gaps = coverage_gaps or []
    accepted = not diagnostics and verdict == "accepted" and bool(ledger.get("done")) and not open_items
    payload = {
        "gate": UNSAFE_REFACTOR_AUDIT_GATE,
        "mode": "required",
        "round": round_label,
        "status": "accepted" if accepted else ("needs_repair" if not diagnostics and verdict == "needs_repair" else "rejected"),
        "passed": accepted,
        "accepted": accepted,
        "returncode": 0 if accepted else 1,
        "elapsed_sec": elapsed_sec,
        "verdict": verdict or "rejected",
        "ledger_path": str(ledger_path.resolve()),
        "report_path": str(report_path.resolve()),
        "text_log_path": str(log_path.with_suffix(".log").resolve()),
        "open_reducible_items": open_items,
        "open_reducible_item_count": len(open_items),
        "hard_required_items": ledger.get("hard_required_items", []) if isinstance(ledger.get("hard_required_items"), list) else [],
        "feedback_paths": feedback_paths,
        "source_fingerprint_sha256": expected_fingerprint,
        "unsafe_refactor_coverage_gaps": coverage_gaps,
        "unsafe_refactor_coverage_gap_count": len(coverage_gaps),
        "unsafe_refactor_coverage_satisfied": not coverage_gaps,
        "agent_result": agent_payload,
        "diagnostics": diagnostics,
    }
    return payload


def _build_unsafe_refactor_audit_prompt(
    *,
    rendered_root: Path,
    manifest_path: Path,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    expected_fingerprint: str,
    feedback_paths: list[str],
    suite: str = "ohos",
    scope_inventory_path: Path | None = None,
    scope_inventory_markdown_path: Path | None = None,
) -> str:
    """Build read-only unsafe refactor auditor prompt."""
    feedback_lines = "\n".join(f"- {path}" for path in feedback_paths) if feedback_paths else "- <none>"
    suite = str(suite or "ohos").strip().lower()
    if suite not in {"ohos", "oss"}:
        suite = "ohos"
    if suite == "oss":
        objective = "审计当前 Rust crate 的 unsafe 是否还有可降低空间，并维护 unsafe refactor ledger/report。你只负责审计和给 repair agent 产出可执行任务，不修代码。当前项目是独立 OSS/C 小项目，翻译目标是 Rust-native 独立项目；目标是 raw unsafe = 0 或接近 0。原 C 头文件/测试入口用于理解源语义，不要求当前翻译产物保留 C 外部接口形状。"
        phase_b_context = "当前是独立 OSS/C 小项目翻译，不要求保留 C 外部接口、原 C 函数签名或内部结构一对一形状；普通算法、private/internal helper、private state、private struct、intrusive list、手写全局表可建议 Rust-native 重写，只要 Rust 项目可观察语义保持等价。"
        phase_b_abi = "真实外部 FFI/系统 API、callback/function pointer、资源生命周期或 FFI layout 仍须保留最小 unsafe；除此之外优先改成 safe Rust API 或 safe core。"
        phase_b_preserve = "不要要求删除业务逻辑、默认返回、跳过资源生命周期或改变项目可观察行为来降低 unsafe。"
        strict_required = "hard_required 只能覆盖真实外部 FFI/系统调用、callback ABI、无法安全表达的最小 raw pointer deref/offset 或 FFI layout 操作；C 头文件/源码测试引用本身不是 hard_required 证据。"
        strict_private = "private/internal 代码和 C 风格 public raw-pointer 入口中的 unsafe 默认应视为可疑候选；只有当你给出具体外部 FFI、raw pointer aliasing、callback order、resource lifetime 或外部副作用证据时，才能判为 hard_required 或 semantic_risk。"
        semantic_risk = "semantic_risk 只能用于有具体项目可观察语义风险证据的项；必须列出尝试过或评估过的替代方案，以及这些方案为什么会破坏算法结果、资源生命周期、回调顺序、别名约束或外部副作用。一次 repair 失败不能永久放行。"
        hard_rule = "hard_required 只能用于真实 FFI 调用点、系统 API、callback/function pointer ABI、raw pointer deref/offset、FFI layout 操作等确实无法用 safe Rust 表达的最小范围。"
        internal_rule = "不能因为翻译来自 C、C 头文件声明、C 测试调用、使用私有 raw pointer 结构、或原 C 项目用了 intrusive list/global state，就默认保留 unsafe；这些内部结构优先进入 Rust-native 重写候选。"
    else:
        objective = "审计 OHOS 集成项目当前 Rust crate 的 unsafe 是否还有可降低空间，并维护 unsafe refactor ledger/report。你只负责审计和给 repair agent 产出可执行任务，不修代码；任何建议都必须保持 public ABI、平台 FFI、布局和回调边界。"
        phase_b_context = "当前是 OHOS 集成项目；private/internal helper、private state、private struct、intrusive list、手写 singleton、手写全局表和普通算法可建议 Rust-native 重写，但必须保持 public ABI 和平台可观察语义。"
        phase_b_abi = "callback/extern ABI 必须优先考虑 thin ABI thunk + private safe/core helper；C static/private helper、private state、intrusive list、手写 singleton 可建议 Rust-owned registry、NonNull/newtype、窄 unsafe helper 或安全容器化重写。"
        phase_b_preserve = "不要要求删除业务逻辑、改变 public ABI、改变 extern 调用约定、改变公开 repr(C) 类型布局、跳过回调/锁/资源生命周期/外部副作用来降低 unsafe。"
        strict_required = "hard_required 只能覆盖最小 unsafe 表达式、最小 FFI 调用点、最小 raw pointer deref/offset、最小 ABI thunk 签名或最小 public repr(C) 访问范围；不能把函数、循环或大块 unsafe 整体判成 required。"
        strict_private = "private/internal 代码中的 unsafe 默认应视为可疑候选；只有当你给出具体 public ABI、FFI layout、raw pointer aliasing、callback order、resource lifetime 或外部副作用证据时，才能判为 hard_required 或 semantic_risk。"
        semantic_risk = "semantic_risk 只能用于有具体 public observable semantics 风险证据的项；必须列出尝试过或评估过的替代方案，以及这些方案为什么会破坏 ABI、锁/生命周期、回调顺序、别名约束或外部副作用。一次 repair 失败不能永久放行。"
        hard_rule = "hard_required 只能用于 ABI thunk 签名、FFI 调用点、raw pointer deref/offset、public repr(C) out-param/field access 等确实无法用 safe Rust 表达的最小范围。"
        internal_rule = "不能因为翻译来自 C、使用了私有 raw pointer 结构、或原 C 项目用了 intrusive list/global state，就默认保留 unsafe；这些内部结构优先进入 Rust-native 重写候选。"
    scope_inventory_line = f"unsafe_scope_inventory_json: {scope_inventory_path.resolve()}" if scope_inventory_path is not None else "unsafe_scope_inventory_json: <from agent context information_paths>"
    scope_inventory_md_line = f"unsafe_scope_inventory_markdown: {scope_inventory_markdown_path.resolve()}" if scope_inventory_markdown_path is not None else "unsafe_scope_inventory_markdown: <from agent context information_paths>"
    return f"""# Unsafe Refactor Audit Task

## Objective
{objective}

## Evidence Policy
- 不修改 Rust、C/C++、extracted facts、type-generation report、semantic ledger/report 或 gate 日志。
- 不读取、不运行、不引用 held-out Rust tests。
- 使用 agent context 中的绝对路径读取事实文档；必须按需读取 unsafe_scope_gate_json/markdown、abi_refactor_inventory_json/markdown、semantic_audit_report、src/*.rs、src/types.rs、src/compat.rs、copied_c_source 或 source_project_root。
- 如果 unsafe_refactor_ledger.json 已存在，必须先读取旧 ledger，复查旧 open/hard_required item 后再更新状态；不能因为本轮没重新想到就删除旧项。
- 如果 repair feedback 指出某个建议会改变 public observable semantics，必须读取反馈证据并复核：可以改成 hard_required、改成更窄的 partially_reducible，或说明反馈不成立并保留 open。
- 只产出 ledger 和 report；不要创建额外 trace/review/scratch 文档。

## Required Workflow
每轮必须按顺序完成三个阶段；旧项全 fixed 不能直接 accept。

Phase A: prior unsafe item re-evaluation
- 读取旧 ledger 和 feedback 后，逐项复核旧 open/hard_required/semantic_risk item。
- 每个旧项只给一个短结论：fixed / still_open / hard_required / semantic_risk / obsolete，并保留证据路径。

Phase B: fresh unsafe reduction discovery
- 重新读取当前 unsafe scope 索引和当前源码，覆盖每个剩余 unsafe scope，并优先深查 unsafe_total_lines 高、scope 行数大、unsafe extern/unsafe fn、raw pointer 遍历、FFI 调用混入普通控制流的区域。
- 对每个候选 scope 判断哪些表达式真的必须 unsafe，哪些 safe 语句可以移出、包进 safe helper、或通过 Rust-owned 状态重构降低 unsafe 范围。样例包括但不限于 safe 控制流、空判断、return、局部变量准备、常量/size 计算、普通条件判断；这些只是样例，你必须基于源码自行识别其他可安全外移或重写的内容。
- 对 unsafe scope 索引中的每个 Uxxxx 都必须在 ledger items 中留下覆盖记录；每条 item 写 `scope_ids`，列出本项覆盖的 Uxxxx。
- {phase_b_context}
- {phase_b_abi}
- {phase_b_preserve}

Phase C: repair ledger emission
- 如果还有可降低项，status/verdict=needs_repair，写 open_reducible_items，每项给 file/span/problem/repair_instruction/must_preserve/expected_effect。
- 如果没有可降低项，status/verdict=accepted，done=true；每个保留 unsafe 必须有 hard_required 证据，不能空泛写“raw pointer required”。

## Strict Unsafe Reduction Criteria
- {strict_required}
- {strict_private}
- 对混合了 safe/unsafe 的 scope，必须判断是否能拆成 safe 外层 + 窄 unsafe helper。若判定不能拆，必须按行或按表达式说明每个 safe-looking 片段为什么不可拆。
- {semantic_risk}
- partially_reducible/risky_reducible 仍应进入 open_reducible_items，只要 repair agent 有可执行的窄化步骤且能保留 public observable semantics。
- accepted 表示你已主动寻找新的 unsafe 降低空间并证明没有可执行剩余项，不只是旧 open items 清零。

## Ledger JSON Contract
{{
  "schema_version": "c2r_unsafe_refactor_ledger_v1",
  "status": "accepted | needs_repair | rejected",
  "verdict": "accepted | needs_repair | rejected",
  "done": false,
  "source_fingerprint_sha256": "<expected_source_fingerprint_sha256 from Dynamic Run Inputs>",
  "items": [
    {{
      "id": "UNSAFE::file::function::scope",
      "scope_ids": ["U0001"],
      "file": "absolute or rendered_root-relative Rust path",
      "span": "line-start-line-end",
      "function": "function name if known",
      "classification": "reducible | partially_reducible | risky_reducible | hard_required | semantic_risk | fixed | obsolete",
      "status": "open | fixed | hard_required | semantic_risk | obsolete",
      "problem": "why this unsafe remains too broad or why it is required",
      "repair_instruction": "specific Rust repair instruction; empty for hard_required",
      "must_preserve": ["ABI/semantic/resource facts"],
      "evidence": ["path:line evidence"],
      "expected_effect": "expected unsafe reduction, qualitative is ok"
    }}
  ],
  "open_reducible_items": [],
  "hard_required_items": [],
  "repair_feedback_consumed": [],
  "report_path": "<Markdown report path from Dynamic Run Inputs>",
  "summary": "one-line summary"
}}

## Rules
- open_reducible_items 只放 repair agent 当前应该修改的项；每项必须有 id/file/span/problem/repair_instruction/must_preserve/evidence。
- 每个 items 条目必须用 scope_ids 或 evidence 中的 Uxxxx 对应 unsafe_scope_gate_json 里的具体 scope；提交 accepted 结论时不能遗漏任何非生成目录 unsafe scope。
- {hard_rule}
- 不能把一个大 unsafe block 整体判 hard_required；必须说明哪些普通代码可移出，除非逐行证据证明不可拆。
- {internal_rule}
- accepted 必须满足：Phase A/B/C 完成、open_reducible_items 为空、done=true、每个剩余 unsafe scope 都有 hard_required 或 semantic_risk 解释。
- report 保持简短，优先列 open items 和 hard_required 摘要，不粘贴长源码。

## Dynamic Run Inputs
- agent_context_path: {context_path.resolve()}
- rendered_root: {rendered_root.resolve()}
- cargo_manifest: {manifest_path.resolve()}
- suite: {suite}
- {scope_inventory_line}
- {scope_inventory_md_line}
- expected_source_fingerprint_sha256: {expected_fingerprint}
- repair context markdown/facts、unsafe scope、ABI/native inventory、semantic report 路径都在 agent context 的 information_paths 中。

## Output Paths
- unsafe_refactor_ledger.json: {ledger_path.resolve()}
- Markdown report: {report_path.resolve()}

## Repair Feedback From Prior Rounds
{feedback_lines}

完成时使用 finish action，status=done。
"""


def _build_unsafe_refactor_audit_retry_prompt(context_path: Path, payload: dict[str, Any]) -> str:
    """Build retry prompt when unsafe auditor output is structurally invalid."""
    diagnostics = payload.get("diagnostics") if isinstance(payload.get("diagnostics"), list) else []
    diagnostic_lines = "\n".join(f"- {item}" for item in diagnostics if str(item).strip()) or "- <none>"
    return f"""# Unsafe Refactor Audit Output Fix Required

当前 action loop 刚刚 finish，但 unsafe auditor 输出未通过结构完整性校验。继续当前任务，不要修改 Rust 源码，只修正 unsafe_refactor_ledger.json 和 report。

## Inputs
- agent_context_path: {context_path.resolve()}
- unsafe_refactor_ledger_json: {payload.get('ledger_path', '')}
- unsafe_refactor_report: {payload.get('report_path', '')}
- expected_source_fingerprint_sha256: {payload.get('source_fingerprint_sha256', '')}

## Diagnostics
{diagnostic_lines}

## Required Action
1. 读取 agent context、现有 ledger/report 和 diagnostics。
2. 按原 unsafe audit contract 补齐或修正 ledger/report。
3. source_fingerprint_sha256 必须匹配 diagnostics 中要求的当前源码指纹。
4. 如果确实还有可优化项，status/verdict=needs_repair 并填写 open_reducible_items；如果没有，status/verdict=accepted 且 done=true。
5. finish status=done。
"""


def _build_unsafe_refactor_coverage_continuation_prompt(
    *,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    scope_plan_path: Path,
    expected_fingerprint: str,
    missing_scopes: list[dict[str, Any]],
) -> str:
    """构造 unsafe audit coverage gap 补审提示词。"""
    return f"""# Unsafe Refactor Audit Coverage Continuation

当前 unsafe refactor audit 已完成一次，但 unsafe scope inventory 仍有未覆盖 scope。继续当前 unsafe audit，不修改 Rust 源码，只补审缺口并更新同一个 unsafe_refactor_ledger.json 和 unsafe_refactor_report.md。

## Inputs
- agent_context_path: {context_path.resolve()}
- unsafe_refactor_ledger.json: {ledger_path.resolve()}
- Markdown report: {report_path.resolve()}
- unsafe_scope_inventory_json: {scope_plan_path.resolve()}
- expected_source_fingerprint_sha256: {expected_fingerprint}

## Missing Unsafe Scope IDs
{json.dumps(missing_scopes, ensure_ascii=False, indent=2)}

## Required Action
1. 读取 agent context、unsafe_scope_inventory_json、现有 ledger/report。
2. 只围绕 Missing Unsafe Scope IDs 补查；不要删除已有 item、open_reducible_items、hard_required 或 semantic_risk 结论。
3. 对每个 missing scope，在 ledger items 中补一条覆盖记录，必须包含 scope_ids、file、span、function、classification、status、problem、evidence。
4. 如果该 scope 仍可优化，classification=reducible/partially_reducible/risky_reducible，status=open，并把对应项放入 open_reducible_items。
5. 如果必须保留 unsafe，classification/status 写 hard_required 或 semantic_risk，并给出具体证据；不能空泛写 raw pointer required。
6. 完成前重新读取 ledger，确认 JSON 合法、source_fingerprint 未变、已有条目未被静默删除。
7. finish status=done。
"""


def _continue_unsafe_refactor_audit_coverage(
    *,
    runner: OpenAIActionRunner,
    rendered_root: Path,
    output_dir: Path,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    scope_plan_path: Path,
    scope_plan: dict[str, Any],
    expected_fingerprint: str,
    continuation_runs: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    """补审 unsafe scope coverage gaps；残余 gaps 只记录，不阻断 accepted。"""
    ledger = _read_json_object(ledger_path)
    coverage_gaps = _unsafe_refactor_coverage_gaps(ledger, scope_plan) if ledger else []
    attempt = 1
    while coverage_gaps and attempt <= UNSAFE_REFACTOR_AUDIT_COVERAGE_CONTINUATION_LIMIT:
        prompt = _build_unsafe_refactor_coverage_continuation_prompt(
            context_path=context_path,
            ledger_path=ledger_path,
            report_path=report_path,
            scope_plan_path=scope_plan_path,
            expected_fingerprint=expected_fingerprint,
            missing_scopes=coverage_gaps,
        )
        name = f"unsafe_refactor_audit_coverage_{attempt:02d}"
        try:
            run_result = _continue_runner(
                runner,
                task=prompt,
                cwd=rendered_root,
                output_dir=output_dir,
                name=name,
                base_name="unsafe_refactor_audit",
            )
            continuation_runs.append(run_result.to_dict())
        except Exception as exc:  # noqa: BLE001
            continuation_runs.append({"returncode": 127, "name": name, "error_type": type(exc).__name__, "error": str(exc)})
            break
        ledger = _read_json_object(ledger_path)
        if not ledger:
            break
        coverage_gaps = _unsafe_refactor_coverage_gaps(ledger, scope_plan)
        if _agent_returncode(continuation_runs[-1]) != 0:
            break
        attempt += 1
    return coverage_gaps


def _run_unsafe_refactor_audit(
    *,
    runner: OpenAIActionRunner,
    round_label: str,
    rendered_root: Path,
    manifest_path: Path,
    context_path: Path,
    log_path: Path,
    suite: str = "ohos",
) -> dict[str, Any]:
    """Run read-only unsafe refactor auditor and validate its ledger."""
    output_dir = log_path.parent / log_path.stem
    output_dir.mkdir(parents=True, exist_ok=True)
    run_root = log_path.parent.parent
    ledger_path = run_root / "unsafe_refactor_ledger.json"
    report_path = output_dir / "unsafe_refactor_report.md"
    feedback_paths = _collect_unsafe_refactor_feedback_paths(run_root)
    expected_fingerprint = str(_source_fingerprint(rendered_root).get("sha256", ""))
    attempts: list[dict[str, Any]] = []
    payload: dict[str, Any] = {}
    unsafe_scope_plan, unsafe_scope_plan_path, unsafe_scope_markdown_path = _unsafe_scope_review_plan(rendered_root, output_dir, round_label)
    unsafe_coverage_continuations: list[dict[str, Any]] = []

    prompt = _build_unsafe_refactor_audit_prompt(
        rendered_root=rendered_root,
        manifest_path=manifest_path,
        context_path=context_path,
        ledger_path=ledger_path,
        report_path=report_path,
        expected_fingerprint=expected_fingerprint,
        feedback_paths=feedback_paths,
        suite=suite,
        scope_inventory_path=unsafe_scope_plan_path,
        scope_inventory_markdown_path=unsafe_scope_markdown_path,
    )
    name = "unsafe_refactor_audit"
    before = _snapshot_project(rendered_root)
    started = time.time()

    def build_payload(agent_payload: dict[str, Any], *, include_coverage: bool = False) -> dict[str, Any]:
        """Build and record one structural unsafe audit attempt."""
        nonlocal payload
        after = _snapshot_project(rendered_root)
        mutations = _project_mutations(before, after)
        ledger = _read_json_object(ledger_path)
        coverage_gaps = _unsafe_refactor_coverage_gaps(ledger, unsafe_scope_plan) if include_coverage and ledger else []
        payload = _unsafe_refactor_audit_payload_from_ledger(
            round_label=round_label,
            ledger_path=ledger_path,
            report_path=report_path,
            log_path=log_path,
            agent_payload=agent_payload,
            elapsed_sec=round(time.time() - started, 3),
            expected_fingerprint=expected_fingerprint,
            mutations=mutations,
            feedback_paths=feedback_paths,
            coverage_gaps=coverage_gaps,
        )
        attempt_index = len(attempts)
        attempts.append({"attempt": attempt_index, "name": name, "unsafe_refactor_audit": dict(payload)})
        _progress(
            f"{round_label}: unsafe refactor audit finish attempt={attempt_index + 1}/{UNSAFE_REFACTOR_AUDIT_RETRY_LIMIT + 1} name={name} status={payload.get('status')} "
            f"verdict={payload.get('verdict')} open_items={payload.get('open_reducible_item_count', 0)} "
            f"diagnostics={len(payload.get('diagnostics', []))} elapsed={payload.get('elapsed_sec')}s "
            f"ledger={payload.get('ledger_path')} report={payload.get('report_path')}"
        )
        return payload

    def finish_validator(state: dict[str, Any]) -> str | None:
        """Continue the same action loop when the unsafe audit artifact is structurally invalid."""
        finish_status = str(state.get("finish_status", "")).strip()
        agent_payload = {
            "returncode": 0 if finish_status in {"done", "complete", "completed", "submitted"} else 1,
            "finish_status": finish_status,
            "finish_message": str(state.get("finish_message", "")).strip(),
            "step": state.get("step"),
            "elapsed_sec": state.get("elapsed_sec"),
        }
        attempt_payload = build_payload(agent_payload)
        structural_diagnostics = [
            str(item)
            for item in (attempt_payload.get("diagnostics") if isinstance(attempt_payload.get("diagnostics"), list) else [])
            if str(item).strip()
        ]
        if not structural_diagnostics:
            return None
        if len(attempts) <= UNSAFE_REFACTOR_AUDIT_RETRY_LIMIT:
            return _build_unsafe_refactor_audit_retry_prompt(context_path, attempt_payload)
        return None

    _progress(f"{round_label}: unsafe refactor audit start retry_limit={UNSAFE_REFACTOR_AUDIT_RETRY_LIMIT} name={name}")
    try:
        run_result = runner.run(task=prompt, cwd=rendered_root, output_dir=output_dir, name=name, finish_validator=finish_validator)
        agent_payload = run_result.to_dict()
        if attempts:
            payload["agent_result"] = agent_payload
            attempts[-1]["unsafe_refactor_audit"] = dict(payload)
        else:
            payload = build_payload(agent_payload)
    except Exception as exc:  # noqa: BLE001
        agent_payload = {"returncode": 127, "error_type": type(exc).__name__, "error": str(exc)}
        payload = build_payload(agent_payload)

    coverage_gaps = _continue_unsafe_refactor_audit_coverage(
        runner=runner,
        rendered_root=rendered_root,
        output_dir=output_dir,
        context_path=context_path,
        ledger_path=ledger_path,
        report_path=report_path,
        scope_plan_path=unsafe_scope_plan_path,
        scope_plan=unsafe_scope_plan,
        expected_fingerprint=expected_fingerprint,
        continuation_runs=unsafe_coverage_continuations,
    )
    if unsafe_coverage_continuations or coverage_gaps:
        payload = _unsafe_refactor_audit_payload_from_ledger(
            round_label=round_label,
            ledger_path=ledger_path,
            report_path=report_path,
            log_path=log_path,
            agent_payload=agent_payload,
            elapsed_sec=round(time.time() - started, 3),
            expected_fingerprint=expected_fingerprint,
            mutations=_project_mutations(before, _snapshot_project(rendered_root)),
            feedback_paths=feedback_paths,
            coverage_gaps=coverage_gaps,
        )
    payload["audit_attempts"] = attempts
    payload["retry_limit"] = UNSAFE_REFACTOR_AUDIT_RETRY_LIMIT
    payload["unsafe_refactor_scope_inventory"] = str(unsafe_scope_plan_path.resolve())
    payload["unsafe_refactor_scope_inventory_markdown"] = str(unsafe_scope_markdown_path.resolve())
    payload["unsafe_refactor_scope_inventory_summary"] = unsafe_scope_plan.get("summary", {})
    payload["unsafe_refactor_coverage_gaps"] = coverage_gaps
    payload["unsafe_refactor_coverage_gap_count"] = len(coverage_gaps)
    payload["unsafe_refactor_coverage_satisfied"] = not coverage_gaps
    payload["unsafe_refactor_coverage_continuations"] = unsafe_coverage_continuations
    payload["unsafe_refactor_coverage_continuation_limit"] = UNSAFE_REFACTOR_AUDIT_COVERAGE_CONTINUATION_LIMIT
    _write_json(log_path, payload)
    log_path.with_suffix(".log").write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return payload


def _collect_type_reports(workspace_dir: Path, project_name: str, llm_name: str) -> tuple[list[Path], list[dict[str, Any]]]:
    """Collect type-generation report paths and parsed payloads."""
    candidates = [
        workspace_dir / "skeletons" / project_name / "types_generation_report.json",
        workspace_dir / "final_projects" / project_name / f"translate_by_{llm_name}" / "types_generation_report.json",
        workspace_dir / "incremental_work" / project_name / f"translate_by_{llm_name}" / "types_generation_report.json",
    ]
    paths: list[Path] = []
    payloads: list[dict[str, Any]] = []
    seen: set[Path] = set()
    for path in candidates:
        resolved = path.resolve()
        if resolved in seen or not path.is_file():
            continue
        seen.add(resolved)
        paths.append(resolved)
        payloads.append(_read_json_object(path))
    return paths, payloads


def _extract_report_summary(report: dict[str, Any]) -> dict[str, Any]:
    """Extract compact facts from a types_generation_report."""
    attempts = report.get("attempts") if isinstance(report.get("attempts"), list) else []
    missing_files: list[str] = []
    attempt_errors: list[str] = []
    for item in attempts:
        if not isinstance(item, dict):
            continue
        for value in item.get("missing_files", []) if isinstance(item.get("missing_files"), list) else []:
            text = str(value).strip()
            if text and text not in missing_files:
                missing_files.append(text)
        error = str(item.get("error", "") or "").strip()
        if error:
            attempt_errors.append(error[:500])
    return {
        "mode": str(report.get("mode", "") or ""),
        "success": bool(report.get("success")),
        "compile_commands_loaded": bool(report.get("compile_commands_loaded")),
        "compile_commands_path": str(report.get("compile_commands_path", "") or ""),
        "ohos_root": str(report.get("ohos_root", "") or ""),
        "ohos_project_rel": str(report.get("ohos_project_rel", "") or ""),
        "missing_types_count": len(report.get("missing_types", []) if isinstance(report.get("missing_types"), list) else []),
        "source_scan_files_count": len(report.get("source_scan_files", []) if isinstance(report.get("source_scan_files"), list) else []),
        "missing_files": missing_files[:20],
        "attempt_error_count": len(attempt_errors),
        "attempt_errors_sample": attempt_errors[:3],
    }


def _scan_rust_markers(rendered_root: Path) -> dict[str, Any]:
    """Scan final Rust project for repair-relevant markers."""
    patterns = {
        "panic": r"\bpanic!\s*\(",
        "c2rust_fallback": r"__c2rust_fallback|C2Rust fallback",
        "failed_translation_comment": r"C2R_FAILED_TRANSLATION_BEGIN|C2R_LLM_FAILED_OUTPUT_BEGIN",
        "opaque_placeholder": r"Opaque placeholder|_opaque|_private",
        "safe_default": r"safe default|Default::default\(\)|return\s+0\s*;",
    }
    counts = {key: 0 for key in patterns}
    files: dict[str, list[str]] = {key: [] for key in patterns}
    absolute_files: dict[str, list[str]] = {key: [] for key in patterns}
    for path in rendered_root.rglob("*.rs"):
        if "target" in path.parts:
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        rel = path.relative_to(rendered_root).as_posix()
        for key, pattern in patterns.items():
            found = len(re.findall(pattern, text))
            if found:
                counts[key] += found
                if len(files[key]) < 20:
                    files[key].append(rel)
                    absolute_files[key].append(str(path.resolve()))
    return {"counts": counts, "files": files, "absolute_files": absolute_files}


def _read_jsonl_objects(path: Path) -> list[dict[str, Any]]:
    """Read JSONL objects, skipping invalid lines."""
    result: list[dict[str, Any]] = []
    if not path.is_file():
        return result
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return result
    for line in lines:
        text = line.strip()
        if not text:
            continue
        try:
            item = json.loads(text)
        except json.JSONDecodeError:
            continue
        if isinstance(item, dict):
            result.append(item)
    return result


def _existing_path_str(path: Path | str | None, *, expect: str = "any") -> str:
    """返回真实存在的绝对路径；不存在则返回空字符串。"""
    if path is None:
        return ""
    try:
        resolved = Path(path).expanduser().resolve()
    except (OSError, RuntimeError, TypeError, ValueError):
        return ""
    if expect == "file":
        return str(resolved) if resolved.is_file() else ""
    if expect == "dir":
        return str(resolved) if resolved.is_dir() else ""
    return str(resolved) if resolved.exists() else ""


def _collect_manual_fix_artifacts(workspace_dir: Path, project_name: str, llm_name: str) -> dict[str, Any]:
    """Collect path-oriented failed translation artifacts for post repair."""
    repair_history_root = (workspace_dir / "repair_history" / project_name / f"translate_by_{llm_name}").resolve()
    manual_fix_root = (repair_history_root / "_manual_fix").resolve()
    manifest_path = (manual_fix_root / "manifest.jsonl").resolve()
    llm_prompts_root = (workspace_dir / "llm_prompts" / project_name / f"translate_by_{llm_name}").resolve()

    entries = _read_jsonl_objects(manifest_path)
    if not entries and manual_fix_root.is_dir():
        for meta_path in sorted(manual_fix_root.glob("*/meta.json")):
            meta = _read_json_object(meta_path)
            if meta:
                entries.append(meta)

    compact_entries: list[dict[str, Any]] = []
    for item in entries:
        compact_entries.append(
            {
                "func_key": item.get("func_key", ""),
                "c_func": item.get("c_func", ""),
                "reason": item.get("reason", ""),
                "meta_path": item.get("meta_path", ""),
                "failed_translation_path": item.get("failed_translation_path") or item.get("rust_code_path", ""),
                "compile_error_path": item.get("compile_error_path", ""),
                "attempts_path": item.get("attempts_path", ""),
                "dependency_closure_manifest_path": item.get("dependency_closure_manifest_path", ""),
                "prompt_paths": item.get("prompt_paths", []),
                "repair_history_dir": item.get("repair_history_dir", ""),
                "rust_file_path": item.get("rust_file_path", ""),
                "source_comment_marker": item.get("source_comment_marker", ""),
            }
        )

    return {
        "repair_history_root": _existing_path_str(repair_history_root, expect="dir"),
        "manual_fix_root": _existing_path_str(manual_fix_root, expect="dir"),
        "manual_fix_manifest": _existing_path_str(manifest_path, expect="file"),
        "llm_prompts_root": _existing_path_str(llm_prompts_root, expect="dir"),
        "artifact_count": len(compact_entries),
        "artifacts": compact_entries,
    }


def _build_repair_context(
    *,
    args: argparse.Namespace,
    rendered_root: Path,
    manifest_path: Path,
    latest_gate_bundle_path: Path,
    round_index: int,
    output_dir: Path,
    semantic_reference_gate_bundle_path: Path | None = None,
    unsafe_review_task: dict[str, Any] | None = None,
) -> Path:
    """Build compact agent context and detailed sidecar documents."""
    workspace_dir = Path(args.workspace_dir).expanduser().resolve()
    context_root = output_dir / "repair_context"
    context_root.mkdir(parents=True, exist_ok=True)
    suite = _normalize_suite(getattr(args, "suite", "ohos"))
    type_report_paths, type_reports = _collect_type_reports(workspace_dir, args.project, args.llm_name)
    type_summaries = [_extract_report_summary(report) for report in type_reports]
    ohos_root = ""
    if suite == "ohos":
        ohos_root = str(DEFAULT_OHOS_ROOT.resolve()) if DEFAULT_OHOS_ROOT.exists() else ""
        for report_summary in type_summaries:
            candidate = str(report_summary.get("ohos_root", "") or "").strip()
            if candidate and Path(candidate).expanduser().exists():
                ohos_root = candidate
                break
    stub_reports = [summary for summary in type_summaries if summary.get("mode") == "stub"]
    markers = _scan_rust_markers(rendered_root)
    manual_fix = _collect_manual_fix_artifacts(workspace_dir, args.project, args.llm_name)
    source_project_root = _existing_path_str(args.source_project_root, expect="dir") if args.source_project_root else ""
    copied_c_source = _existing_path_str(workspace_dir / "c_source" / args.project, expect="dir")
    extracted_functions = _existing_path_str(workspace_dir / "extracted" / args.project / "functions", expect="dir")
    functions_manifest = _existing_path_str(workspace_dir / "extracted" / args.project / "functions_manifest.json", expect="file")
    resolved_source_evidence = _build_resolved_source_evidence(source_project_root, copied_c_source, functions_manifest)
    skeleton_root = _existing_path_str(workspace_dir / "skeletons" / args.project, expect="dir")
    manual_fix_summary = {
        "repair_history_root": manual_fix["repair_history_root"],
        "manual_fix_root": manual_fix["manual_fix_root"],
        "manual_fix_manifest": manual_fix["manual_fix_manifest"],
        "llm_prompts_root": manual_fix["llm_prompts_root"],
        "artifact_count": manual_fix.get("artifact_count", 0),
    }
    facts = {
        "schema_version": "c2r_repair_context_facts_v1",
        "project": args.project,
        "llm_name": args.llm_name,
        "suite": suite,
        "rendered_root": str(rendered_root),
        "cargo_manifest": str(manifest_path),
        "workspace_dir": str(workspace_dir),
        "ohos_root": ohos_root,
        "source_project_root": source_project_root,
        "type_report_paths": [str(path) for path in type_report_paths],
        "type_report_summaries": type_summaries,
        "stub_report_count": len(stub_reports),
        "rust_marker_scan": markers,
        "manual_fix_artifacts": manual_fix_summary,
        "source_paths": {
            "copied_c_source": copied_c_source,
            "extracted_functions": extracted_functions,
            "functions_manifest": functions_manifest,
            "skeleton": skeleton_root,
            "final_project": str(rendered_root),
        },
        "resolved_source_evidence": resolved_source_evidence,
        "policy": {
            "held_out_tests_forbidden": True,
            "forbidden_test_roots": [
                str((REPO_ROOT / "ComparisonMethod" / "test_module" / "rust_tests").resolve()),
            ],
            "stub_policy": "Do not invent struct layout or external ABI from stub reports; semantic audit owns root-cause classification.",
        },
    }
    facts_path = context_root / "repair_context_facts.json"
    _write_json(facts_path, facts)

    md_path = context_root / "repair_context.md"
    md_lines = [
        "# Repair Context",
        "",
        f"- project: {args.project}",
        f"- suite: {suite}",
        f"- rendered_root: {rendered_root}",
        f"- cargo_manifest: {manifest_path}",
        f"- latest_gate_bundle: {latest_gate_bundle_path}",
        f"- stub_report_count: {len(stub_reports)} / {len(type_summaries)}",
        f"- failed_translation_artifact_count: {manual_fix.get('artifact_count', 0)}",
        f"- manual_fix_manifest: {manual_fix.get('manual_fix_manifest', '')}",
        f"- full_facts_json: {facts_path.resolve()}",
        "",
        "## Type Reports",
    ]
    for path, summary in zip(type_report_paths, type_summaries):
        md_lines.append(f"- {path}: mode={summary.get('mode') or '<missing>'}, compile_commands_loaded={summary.get('compile_commands_loaded')}")
    md_lines.extend(["", "## Rust Marker Counts"])
    for key, value in markers.get("counts", {}).items():
        md_lines.append(f"- {key}: {value}")
    source_counts = resolved_source_evidence.get("counts", {}) if isinstance(resolved_source_evidence.get("counts"), dict) else {}
    md_lines.extend(
        [
            "",
            "## Resolved Source Evidence",
            f"- search_roots: {source_counts.get('search_root_count', 0)}",
            f"- production_sources: {source_counts.get('production_sources', 0)}",
            f"- public_headers: {source_counts.get('public_headers', 0)}",
            f"- test_or_example_usage: {source_counts.get('test_or_example_usage', 0)}",
            f"- unresolved_sources: {source_counts.get('unresolved_sources', 0)}",
        ]
    )
    for key in ("production_sources", "public_headers", "test_or_example_usage"):
        values = resolved_source_evidence.get(key)
        if not isinstance(values, list) or not values:
            continue
        md_lines.append(f"- first_{key}: {values[0].get('path', '') if isinstance(values[0], dict) else ''}")
    md_lines.extend(
        [
            "",
            "## Policy",
            "- Do not read or run held-out Rust tests during repair.",
            "- Read the full facts JSON and referenced report paths when needed.",
            "- If types are stubbed, do not invent fields/layout; stop with the relevant evidence path when the requested repair depends on missing type facts.",
            "",
        ]
    )
    md_path.write_text("\n".join(md_lines), encoding="utf-8")

    current_gates = _blocking_summary(latest_gate_bundle_path)
    semantic_reference_summary: dict[str, Any] = {}
    if semantic_reference_gate_bundle_path is not None and semantic_reference_gate_bundle_path.is_file():
        semantic_reference_summary = _blocking_summary(semantic_reference_gate_bundle_path)
    semantic_context_summary = current_gates
    semantic_context_from_reference = False
    if not _semantic_summary_has_context(current_gates) and _semantic_summary_has_context(semantic_reference_summary):
        semantic_context_summary = semantic_reference_summary
        semantic_context_from_reference = True
    semantic_defaults = {
        "semantic_audit_log": "",
        "semantic_audit_status": "",
        "semantic_blockers_ledger_path": "",
        "semantic_audit_report_path": "",
        "semantic_audit_verdict": "",
        "semantic_audit_open_blocker_ids": [],
        "semantic_audit_blocked_external_ids": [],
        "semantic_audit_diagnostics": [],
        "semantic_obligation_open_ids": [],
        "semantic_obligation_blocking_ids": [],
        "semantic_obligation_blocked_external_ids": [],
        "semantic_obligation_summary": {"total": 0, "by_status": {}},
        "semantic_obligation_count": 0,
    }
    for key, default in semantic_defaults.items():
        current_gates[key] = semantic_context_summary.get(key, default)
    unsafe_defaults = {
        "unsafe_refactor_audit_status": "",
        "unsafe_refactor_audit_verdict": "",
        "unsafe_refactor_ledger_path": "",
        "unsafe_refactor_report_path": "",
        "unsafe_refactor_open_item_count": 0,
        "unsafe_refactor_open_reducible_items": [],
        "unsafe_refactor_feedback_paths": [],
    }
    for key, default in unsafe_defaults.items():
        current_gates[key] = current_gates.get(key, default)
    semantic_repair_bundle = _build_semantic_repair_bundle(semantic_context_summary)
    semantic_repair_active = bool(
        semantic_context_from_reference
        and semantic_repair_bundle.get("repair_root_cause_clusters")
        and not semantic_repair_bundle.get("semantic_locked")
    )
    current_gates["semantic_repair_mode"] = {
        "active": semantic_repair_active,
        "reason": "cheap_gate_regression_after_semantic_audit" if semantic_repair_active else "",
        "semantic_reference_gate_bundle": str(semantic_reference_gate_bundle_path.resolve()) if semantic_reference_gate_bundle_path is not None else "",
        "latest_gate_bundle": str(latest_gate_bundle_path.resolve()),
    }
    repair_progress = _build_repair_progress(current_gates, semantic_repair_bundle)

    information_paths = {
        "repair_context_facts": str(facts_path.resolve()),
        "repair_context_markdown": str(md_path.resolve()),
        "latest_gate_bundle": str(latest_gate_bundle_path.resolve()),
        "semantic_reference_gate_bundle": str(semantic_reference_gate_bundle_path.resolve()) if semantic_reference_gate_bundle_path is not None else "",
        "semantic_blockers_ledger": str(current_gates.get("semantic_blockers_ledger_path") or "") if Path(str(current_gates.get("semantic_blockers_ledger_path") or "")).is_file() else "",
        "semantic_audit_report": str(current_gates.get("semantic_audit_report_path") or "") if Path(str(current_gates.get("semantic_audit_report_path") or "")).is_file() else "",
        "unsafe_refactor_ledger": str(current_gates.get("unsafe_refactor_ledger_path") or "") if Path(str(current_gates.get("unsafe_refactor_ledger_path") or "")).is_file() else "",
        "unsafe_refactor_report": str(current_gates.get("unsafe_refactor_report_path") or "") if Path(str(current_gates.get("unsafe_refactor_report_path") or "")).is_file() else "",
        "unsafe_refactor_feedback_jsonl": str((output_dir / "agentic_logs" / f"repair_round_{round_index + 1:02d}" / "unsafe_refactor_feedback.jsonl").resolve()),
        "unsafe_refactor_feedback_paths": _collect_unsafe_refactor_feedback_paths(output_dir),
        "rendered_root": str(rendered_root.resolve()),
        "cargo_manifest": str(manifest_path.resolve()),
        "ohos_root": ohos_root,
        "source_project_root": source_project_root,
        "copied_c_source": copied_c_source,
        "extracted_functions": extracted_functions,
        "functions_manifest": functions_manifest,
        "resolved_source_evidence": resolved_source_evidence,
        "type_report_paths": [str(path) for path in type_report_paths],
        "repair_history_root": manual_fix["repair_history_root"],
        "manual_fix_root": manual_fix["manual_fix_root"],
        "manual_fix_manifest": manual_fix["manual_fix_manifest"],
        "llm_prompts_root": manual_fix["llm_prompts_root"],
    }
    unsafe_review = unsafe_review_task if isinstance(unsafe_review_task, dict) else {}
    unsafe_review_json = str(unsafe_review.get("review_json_path") or "")
    rust_native_review_json = str(unsafe_review.get("rust_native_review_json_path") or unsafe_review_json)
    unsafe_scope_json = str(unsafe_review.get("scope_json_path") or "")
    unsafe_scope_markdown = str(unsafe_review.get("scope_markdown_path") or "")
    abi_inventory_json = str(unsafe_review.get("abi_inventory_json_path") or "")
    abi_inventory_markdown = str(unsafe_review.get("abi_inventory_markdown_path") or "")
    if Path(unsafe_review_json).is_file():
        information_paths["unsafe_review_task_json"] = unsafe_review_json
        information_paths["rust_native_refactor_task_json"] = unsafe_review_json
        if Path(rust_native_review_json).is_file():
            information_paths["rust_native_refactor_task_json_backup"] = rust_native_review_json
    if Path(unsafe_scope_json).is_file():
        information_paths["unsafe_scope_gate_json"] = unsafe_scope_json
    if Path(unsafe_scope_markdown).is_file():
        information_paths["unsafe_scope_gate_markdown"] = unsafe_scope_markdown
    if Path(abi_inventory_json).is_file():
        information_paths["abi_refactor_inventory_json"] = abi_inventory_json
    if Path(abi_inventory_markdown).is_file():
        information_paths["abi_refactor_inventory_markdown"] = abi_inventory_markdown

    repair_context_summary = {
        "stub_report_count": len(stub_reports),
        "type_report_count": len(type_summaries),
        "rust_marker_counts": markers.get("counts", {}),
        "failed_translation_artifact_count": manual_fix.get("artifact_count", 0),
        "held_out_tests_forbidden": True,
        "resolved_source_evidence_counts": resolved_source_evidence.get("counts", {}),
    }
    if isinstance(unsafe_review.get("summary"), dict):
        repair_context_summary["unsafe_scope_summary"] = unsafe_review["summary"]
        repair_context_summary["rust_native_refactor_summary"] = unsafe_review["summary"]
    repair_context_summary["unsafe_refactor_open_item_count"] = int(current_gates.get("unsafe_refactor_open_item_count", 0) or 0)

    verify_fast = [
        sys.executable,
        str((REPO_ROOT / "scripts" / "agentic_repair" / "post_repair_agent.py").resolve()),
        "check",
        "--rendered-root",
        str(rendered_root.resolve()),
        "--output-dir",
        str((output_dir / "agent_verify").resolve()),
        "--project",
        args.project,
        "--suite",
        suite,
    ]
    if suite == "ohos":
        verify_fast.extend(
            [
                "--ohos-rustc",
                str(Path(args.ohos_rustc).expanduser().resolve()),
                "--ohos-rust-target",
                args.ohos_rust_target,
            ]
        )

    agent_context = {
        "schema_version": "c2r_post_repair_agent_context_v1",
        "round_index": round_index,
        "project": args.project,
        "suite": str(getattr(args, "suite", "ohos") or "ohos"),
        "current_gates": current_gates,
        "semantic_repair_bundle": semantic_repair_bundle,
        "repair_progress": repair_progress,
        "rust_native_refactor": {
            "active": bool(unsafe_review_json),
            "task_json": unsafe_review_json,
            "review_task_json": unsafe_review_json,
            "rust_native_review_task_json": rust_native_review_json,
            "scope_json": unsafe_scope_json,
            "scope_markdown": unsafe_scope_markdown,
            "abi_inventory_json": abi_inventory_json,
            "abi_inventory_markdown": abi_inventory_markdown,
            "item_count": int(unsafe_review.get("item_count") or 0) if unsafe_review else 0,
        },
        "unsafe_refactor_audit": {
            "active": bool(current_gates.get("unsafe_refactor_ledger_path") or current_gates.get("unsafe_refactor_open_reducible_items")),
            "status": str(current_gates.get("unsafe_refactor_audit_status") or ""),
            "verdict": str(current_gates.get("unsafe_refactor_audit_verdict") or ""),
            "ledger_path": str(current_gates.get("unsafe_refactor_ledger_path") or ""),
            "report_path": str(current_gates.get("unsafe_refactor_report_path") or ""),
            "open_reducible_items": current_gates.get("unsafe_refactor_open_reducible_items") if isinstance(current_gates.get("unsafe_refactor_open_reducible_items"), list) else [],
            "open_reducible_item_count": int(current_gates.get("unsafe_refactor_open_item_count", 0) or 0),
            "repair_feedback_jsonl": information_paths["unsafe_refactor_feedback_jsonl"],
        },
        "unsafe_optimization": {
            "active": bool(unsafe_review_json),
            "review_task_json": unsafe_review_json,
            "scope_json": unsafe_scope_json,
            "scope_markdown": unsafe_scope_markdown,
            "abi_inventory_json": abi_inventory_json,
            "abi_inventory_markdown": abi_inventory_markdown,
            "item_count": int(unsafe_review.get("item_count") or 0) if unsafe_review else 0,
        },
        "information_paths": information_paths,
        "repair_context_summary": repair_context_summary,
        "verify_commands": {
            "fast": verify_fast
        },
        "write_scope": {
            "preferred_edit_targets": [
                "src/*.rs",
                "src/types.rs",
                "src/compat.rs",
                "build.rs",
                "native/*",
            ],
            "notes": "可以按需要修改这些文件来修复语义或依赖问题，但不是强制必须全部修改。",
        },
        "write_policy": {
            "allowed": [
                str(rendered_root.resolve()),
                str((output_dir / "agentic_logs").resolve()),
            ],
            "forbidden": [
                "original C/C++ source tree",
                "requirements/source extraction facts",
                "held-out Rust tests under ComparisonMethod/test_module/rust_tests",
                "semantic_blockers_ledger.json and semantic_audit_report.md except by semantic audit agent",
                str((output_dir / "semantic_blockers_ledger.json").resolve()),
                str((output_dir / "semantic_audit_logs").resolve()),
            ],
        },
    }
    context_path = context_root / f"agent_context_round_{round_index:02d}.json"
    _write_json(context_path, agent_context)
    return context_path


def _build_repair_prompt(context_path: Path) -> str:
    """Build compact repair prompt with path-oriented context."""
    context = _read_json_object(context_path)
    suite = str(context.get("suite") or "ohos").strip().lower()
    if suite not in {"ohos", "oss"}:
        suite = "ohos"
    info = context.get("information_paths") if isinstance(context.get("information_paths"), dict) else {}
    summary = context.get("repair_context_summary") if isinstance(context.get("repair_context_summary"), dict) else {}
    semantic_bundle = context.get("semantic_repair_bundle") if isinstance(context.get("semantic_repair_bundle"), dict) else {}
    rust_native_refactor = context.get("rust_native_refactor") if isinstance(context.get("rust_native_refactor"), dict) else {}
    unsafe_refactor_audit = context.get("unsafe_refactor_audit") if isinstance(context.get("unsafe_refactor_audit"), dict) else {}
    write_scope = context.get("write_scope") if isinstance(context.get("write_scope"), dict) else {}
    failed_count = int(summary.get("failed_translation_artifact_count") or 0)
    manual_fix_manifest = str(info.get("manual_fix_manifest", "") or "")
    repair_history_root = str(info.get("repair_history_root", "") or "")
    llm_prompts_root = str(info.get("llm_prompts_root", "") or "")
    rendered_root = str(info.get("rendered_root", "") or "")
    cargo_manifest = str(info.get("cargo_manifest", "") or "")
    ohos_root = str(info.get("ohos_root", "") or "")
    source_project_root = str(info.get("source_project_root", "") or "")
    copied_c_source = str(info.get("copied_c_source", "") or "")
    resolved_source_evidence = info.get("resolved_source_evidence") if isinstance(info.get("resolved_source_evidence"), dict) else {}
    source_counts = resolved_source_evidence.get("counts") if isinstance(resolved_source_evidence.get("counts"), dict) else {}
    source_evidence_line = (
        f"- resolved_source_evidence: information_paths.resolved_source_evidence / repair_context_facts.resolved_source_evidence "
        f"(production_sources={source_counts.get('production_sources', 0)}, public_headers={source_counts.get('public_headers', 0)}, "
        f"test_or_example_usage={source_counts.get('test_or_example_usage', 0)}, unresolved_sources={source_counts.get('unresolved_sources', 0)})"
    )
    acceptance_gate_text = "cargo check" if suite == "oss" else "cargo check、OHOS rustc"
    ohos_input_line = "" if suite == "oss" else f"- ohos_root: {ohos_root}"
    unsafe_equivalence_policy = (
        "- Unsafe refactor 阶段允许实现与 C 源不一一同构；独立 Rust-native 项目只需保持项目可观察行为、返回值、错误处理、资源生命周期、回调时机和真实外部 FFI 行为等价。"
        if suite == "oss"
        else "- 当前是 OHOS 集成项目；允许重构 internal implementation，但必须保持 public ABI、公开 repr(C) 类型布局、返回码/out-param、资源生命周期、锁语义、回调时机和平台可观察副作用等价。"
    )
    unsafe_review_json = str(info.get("rust_native_refactor_task_json") or info.get("unsafe_review_task_json") or "")
    unsafe_scope_json = str(info.get("unsafe_scope_gate_json", "") or "")
    unsafe_scope_markdown = str(info.get("unsafe_scope_gate_markdown", "") or "")
    abi_inventory_json = str(info.get("abi_refactor_inventory_json", "") or "")
    abi_inventory_markdown = str(info.get("abi_refactor_inventory_markdown", "") or "")
    unsafe_refactor_ledger = str(info.get("unsafe_refactor_ledger", "") or "")
    unsafe_refactor_report = str(info.get("unsafe_refactor_report", "") or "")
    unsafe_refactor_feedback_jsonl = str(info.get("unsafe_refactor_feedback_jsonl", "") or "")
    write_targets = write_scope.get("preferred_edit_targets", []) if isinstance(write_scope.get("preferred_edit_targets"), list) else []
    repair_cluster_lines: list[str] = []
    for item in semantic_bundle.get("repair_root_cause_clusters", []) if isinstance(semantic_bundle.get("repair_root_cause_clusters"), list) else []:
        if not isinstance(item, dict):
            continue
        repair_cluster_lines.append(
            f"- {str(item.get('id', '')).strip()} | status={str(item.get('status', '')).strip()} | summary={str(item.get('summary', '')).strip()} | strategy={str(item.get('repair_strategy', '')).strip()}"
        )
    if not repair_cluster_lines:
        repair_cluster_lines.append("- <none>")
    failed_paths = ""
    if failed_count:
        failed_paths = f"""
失败翻译 artifact 路径：
- manual_fix_manifest: {manual_fix_manifest}
- repair_history_root: {repair_history_root}
- llm_prompts_root: {llm_prompts_root}
- 读取 manifest 每条记录中的非空路径字段；常见字段包括 meta_path、failed_translation_path、compile_error_path、attempts_path、dependency_closure_manifest_path、prompt_paths、rust_file_path。
"""
    unsafe_scope_section = ""
    if unsafe_refactor_ledger or unsafe_review_json:
        open_count = int(unsafe_refactor_audit.get("open_reducible_item_count", 0) or 0)
        if suite == "oss":
            unsafe_project_context = (
                "- 当前项目是独立 OSS/C 小项目，翻译目标是 Rust-native 独立项目；原 C 源和源码测试用于理解项目语义，当前产物按 Rust 调用方可观察行为修复。\n"
                "- 目标是 raw unsafe=0 或接近 0；普通算法、private/internal helper、private state、private struct、手写全局表和普通控制流都应优先重写为安全 Rust 形态。\n"
                "- 不需要 unsafe 的控制流、局部变量、常量准备、普通条件判断和普通计算应移出 unsafe block；遇到 raw-pointer 入口时优先实现 safe Rust API 或 safe core。\n"
                "- 必须 unsafe 只能来自真实外部 FFI、系统 API、callback/function pointer、资源生命周期、无法安全表达的 raw pointer 交互或 FFI layout 操作。\n"
                "- 不能用删除业务逻辑、默认返回或跳过项目可观察行为来降低 unsafe。"
            )
        else:
            unsafe_project_context = (
                "- 当前是 OHOS 集成项目；在保持 public ABI、extern 调用约定、公开 repr(C) 布局和平台行为的前提下，可重构 private/internal 实现并缩小不必要的 unsafe。\n"
                "- 不需要 unsafe 的控制流、局部变量、常量准备、普通条件判断和普通计算应移出 unsafe block；遇到 callback ABI 时优先实现 thin extern/unsafe ABI thunk + private safe/core helper。\n"
                "- 必须 unsafe 只能来自有证据的 public ABI/FFI 合约、raw pointer 交互、callback ABI、extern 调用或 FFI layout 操作；不能因为原始 C 使用 private raw pointer、intrusive list 或 global state 就默认保留 unsafe。\n"
                "- 禁止通过删除业务逻辑、默认返回、改变 public ABI、改变外部调用约定、改变公开 repr(C) 类型布局或跳过 C 可观察行为来降低 unsafe。"
            )
        unsafe_scope_section = f"""
## Unsafe Refactor Repair Inputs
- unsafe auditor 已接管 unsafe 可优化点判断；repair agent 不维护 unsafe ledger/report，也不再填写 Rust-native review decision/reason/result。
- unsafe_refactor_ledger: {unsafe_refactor_ledger}
- unsafe_refactor_report: {unsafe_refactor_report}
- unsafe_refactor_feedback_jsonl: {unsafe_refactor_feedback_jsonl}
- 当前 open_reducible_item_count: {open_count}
- 可参考 unsafe scope 原始 JSON：{unsafe_scope_json}
- 可参考 unsafe Markdown 索引：{unsafe_scope_markdown}
- 可参考 ABI/native 重构候选 JSON：{abi_inventory_json}
- 可参考 ABI/native Markdown 索引：{abi_inventory_markdown}
- 如果 unsafe_refactor_ledger 中有 open_reducible_items，必须逐项读取并实现其中 repair_instruction，保持 must_preserve 中列出的 ABI、返回码、锁、资源生命周期、回调时机和外部可观察副作用。
- 如果你基于源码和证据确认某个 unsafe auditor 建议会改变 public observable semantics，不要硬改；向 unsafe_refactor_feedback_jsonl 追加一行 JSON，字段包含 item_id、status="semantic_risk"、reason、evidence、repair_agent_round，然后继续处理其它不影响语义的 item。
- 如果所有 open item 都被确认存在 semantic_risk 或不在当前 Rust 写范围内，写反馈后 finish status=blocked，并在 message 中写明反馈路径。
{unsafe_project_context}
- 修改后的当前 crate 会在下一轮重新经过 compile gate、semantic audit 和 unsafe auditor，只有 semantic/unsafe auditor 同时认可才最终通过。
"""
    editable_lines = "\n".join(f"- {item}" for item in write_targets) if write_targets else "- <none>"
    cluster_lines = "\n".join(repair_cluster_lines)
    return f"""# Post Repair Task

## Objective
修复当前已经生成的 Rust crate，使当前 gate bundle 中的阻塞项被解决。你不是重新运行 C/C++2Rust 翻译器，也不是重新生成 skeleton/types/bindgen 产物。

{unsafe_scope_section}

## Acceptance Gates
- cheap gates 必须通过：{acceptance_gate_text}。cargo clippy 当前不作为后置 repair gate。
- repair agent 不是 semantic audit agent；不得重新发现、重分类或关闭语义不一致。
- 只消费 agent context 中 semantic_repair_bundle、repair_progress 以及其引用的 ledger/report。
- 当前任务是在 rendered_root 下修 Rust crate；不要假设可以回到 Stage1/Stage2 重新提取、重新翻译或重跑 bindgen，除非 agent context/verify_commands 显式提供该入口。
- 如果 semantic_repair_bundle.repair_root_cause_clusters 非空，必须按 repair_root_cause_clusters 全量修复，不允许只修一个 obligation 就结束。
- blocked_external_root_cause_clusters / blocked_external_obligations 是可选参考输入，不是必修阻断；可以基于给定路径自行判断能否在当前 Rust crate 内修，修不了时写明证据路径和原因后结束，不要为了消除标签而猜测 ABI 或布局。
- 如果 semantic_repair_bundle.repair_root_cause_clusters 为空或 semantic_repair_bundle.semantic_locked=true，不做语义泛化审计，只修 compile/full gate/replacement artifact 阻塞。
- 如果 unsafe_refactor_audit.open_reducible_items 非空，完成其它阻塞修复后必须按 unsafe auditor ledger 修复这些项或写 semantic_risk 反馈。

## Inputs
- agent_context_path: {context_path.resolve()}
- rendered_root: {rendered_root}
- cargo_manifest: {cargo_manifest}
{ohos_input_line}
- source_project_root: {source_project_root}
- copied_c_source: {copied_c_source}
{source_evidence_line}
{failed_paths}

## Primary Semantic Repair Targets
{cluster_lines}

## Preferred Editable Files
{editable_lines}

## Evidence Policy
- 先读取 agent context 的 current_gates、semantic_repair_bundle、repair_progress、repair_context_summary 和 information_paths.repair_context_markdown。
- 需要细节时，读取 information_paths.repair_context_facts 以及 type_report_paths 中的原始报告。
- 如果 repair_context_summary.failed_translation_artifact_count > 0，读取 information_paths.manual_fix_manifest。
- manual_fix_manifest 每条记录里的 meta_path、failed_translation_path、compile_error_path、attempts_path、dependency_closure_manifest_path、prompt_paths、rust_file_path 是可选字段；只读取非空且存在的路径，缺失字段表示该轮没有生成对应 artifact。
- dependency_closure_manifest_path 里 facts 是高置信依赖事实；hints/gaps 只能作为排查线索，不能当成 ABI 或语义真值。
- 失败译文只以注释形式保留在 rust_file_path 的 source_comment_marker 附近；编译错误、尝试版本和真实翻译/修复提示词不要从源码注释猜，按 artifact 路径读取。
- prompt_paths 是历史 LLM 输入证据，只用于理解信息缺口和约束错位；不要把旧提示词里不存在于当前 crate 的类型、常量、extern 或模块当成事实。
- stub_report_count > 0 时，不要猜测 C struct layout、ABI 或外部符号签名；若指定 root cause 的实现依赖缺失类型事实，带证据路径 finish status=blocked，不写 semantic ledger/report。
- 需要 C/C++ 证据时，先打开 information_paths.resolved_source_evidence 中的 production_sources、public_headers、test_or_example_usage 对应原始源码/header 绝对路径。
- extracted_functions 和 functions_manifest 只能作为索引或片段线索，不能替代源码证据；不要只根据 extracted/functions 片段判断翻译准确性。
- 如果某个指定 root cause 只能用 extracted/functions 片段定位，必须先说明 resolved_source_evidence 中原始源码不可用或已核验的原因。
- 按需打开 C/C++ 证据时，只用于实现 semantic audit agent 已指定的 root cause cluster；不得重新审计整个模块。
- C/C++ 证据只能来自 Inputs 或 agent context 中列出的 source_project_root、copied_c_source、extracted_functions、functions_manifest、dependency_closure_manifest_path、prompt_paths 等真实路径。
{unsafe_equivalence_policy}
- 不读取、不运行、不引用 held-out Rust tests。
- 不回写原始 C/C++、extracted facts、skeleton/type-generation report。
- 不修改 semantic_blockers_ledger.json 或 semantic_audit_report.md；这些文件只由 semantic audit agent 维护。
- 不要为了记录分析过程创建额外的 trace/review/scratch 文档；只有修复本身需要新增源码、shim 或 build 配置文件时才写新文件。

## Current Crate Repair Policy
- 优先使用当前 Rust crate 已有文件和模块：src/*.rs、src/types.rs、src/compat.rs、src/globals.rs、build.rs、native/*。
- 不要发明 crate::types 中不存在的常量/类型，也不要发明 compat.rs 中不存在的 extern 签名；缺少这些事实时，先读取 manifest/type report/compile log 证明来源，仍无事实则按 blocker 汇报。
- 可以修改 src/types.rs 来修正当前 crate 内有证据支持的语法、常量、类型别名或 extern 声明问题；禁止无 ABI 证据猜测 C++ std::string/std::vector/opaque struct 的内部布局。
- 不要新增 crates.io/registry 依赖来绕过编译错误；Cargo.toml 只允许修正当前 crate 已有依赖、feature、build-dependencies 或本地 path/build/native 配置。
- failed_translation_path 是候选译文，不是必须照抄的真值；结合 compile_error_path、attempts_path、dependency_closure facts 和当前 Rust API 修最小必要代码。
- 当阻塞来自 cargo dependency/registry/workspace/timeout 日志时，优先修本地 Cargo/workspace 配置或保持当前函数基线可编译，不要把它当作函数体语义错误。

## Required Steps
1. 读取 agent context，确认 blocking gates、semantic_repair_bundle、repair_progress、允许写入范围和 verify_commands.fast。
2. 读取当前阻塞 gate 的日志；如果存在失败翻译 artifact，读取 manifest 中对应路径。
3. 如果 repair_progress.phase=semantic_root_cause_repair 且存在 repair_root_cause_clusters，按 cluster 列表全量修改当前 Rust 项目内的最小必要代码。
4. 调用 run_verify；它只运行 cheap gates，不运行 held-out tests。
5. 如果当前 cheap gate 失败，先让 run_verify 回绿；但 repair_progress.do_not_reaudit_semantics=true 时仍必须只沿 semantic_repair_bundle 指定的 cluster 推进。
6. 如果 unsafe_refactor_audit.open_reducible_items 非空，按 unsafe_refactor_ledger 修改源码；若确认某项会破坏语义，追加 unsafe_refactor_feedback_jsonl 后不要硬改该项。

## Output Contract
- 完成时使用 finish action，status=done。
- finish.message 保持 1-2 句，只写当前结果或剩余 blocker。
- 不要额外写 trace/review/scratch 留痕文件。
- 如果问题不在当前 Rust 写范围内，写明 blocker id / gate / 证据路径 / 不可修原因，然后 finish status=blocked。
"""


def _build_semantic_audit_prompt(
    *,
    rendered_root: Path,
    manifest_path: Path,
    context_path: Path,
    ledger_path: Path,
    report_path: Path,
    semantic_review_plan_path: Path | None = None,
    suite: str = "ohos",
) -> str:
    """Build read-only semantic audit prompt."""
    suite = _normalize_suite(suite)
    if suite == "oss":
        objective = "对比独立 Rust-native 项目与原始 C/C++ 行为证据，发现 high-confidence semantic mismatch，并维护 obligation、root cause、ledger/report。你只负责审计和报告，不修代码。"
        inventory_rule = "先建立 coverage inventory：列出每个翻译模块、所有 Rust 函数/extern/static/const/类型布局/全局状态和可达调用路径；然后逐项映射到 agent context 中的原始 C/C++ 源、预处理 truth、functions_manifest、types report、manual-fix evidence。"
        contract_rule = "对 inventory 中每个项目可观察行为都必须给出等价判断；不能漏掉状态机、allocator/free、init/reset/update/clean 生命周期、buffer 长度/capacity、copy loop、索引边界、错误码/out-param、常量/static 初始化、条件编译、回调/函数表、依赖调用/FFI/shim。"
        unmapped_rule = "如果某个 Rust 翻译结果找不到对应 C/C++ 证据，不能静默跳过：若它会影响项目行为，标为 open/blocked_external；若确认为无行为影响，写入 coverage_summary 的 reviewed_non_observable。"
        residual_rule = "Residual downgrade guard：residual_risks 只能用于完成指定源码/anchor/usage 搜索后仍无法建立项目可观察行为、或无法证明 Rust 当前行为存在差异的区域；不能用于已证实的 mismatch。"
        blocker_rule = "如果存在任一确定证据，禁止写 residual，必须写 blocking_mismatch 或 blocked_external：原测试、示例、consumer usage 或 semantic_review_plan 明确依赖该项目行为；Rust 当前实现返回值、错误码、out/ref 参数、panic/unwrap、状态副作用、资源生命周期、callback/真实外部 FFI 行为与 C/C++ 证据冲突；缺失项在当前 Rust 写范围内可修复。"
        blocker_summary = "project-observable mismatch"
        open_rule = "只把 high-confidence、项目可观察的 semantic mismatch 标为 open。"
        obligation_id_example = "SEM::Project::Behavior::dimension"
        obligation_subject_example = "ProjectBehavior"
        obligation_contract_example = "one project-observable behavior"
    else:
        objective = "对比最终 Rust 项目与原始 C/C++ 证据，发现 high-confidence public observable semantic mismatch，并维护 obligation、root cause、ledger/report。你只负责审计和报告，不修代码。"
        inventory_rule = "先建立 coverage inventory：列出每个翻译模块、所有 Rust 函数/extern/static/const/类型布局/全局状态、公开入口和被公开入口调用的私有 helper；然后逐项映射到 agent context 中的原始 C/C++ 源、预处理 truth、functions_manifest、types report、manual-fix evidence。"
        contract_rule = "对 inventory 中每个可观察 contract 都必须给出等价判断；优先覆盖 public API，但不能漏掉会影响 public API 的私有 helper、状态机、allocator/free、init/reset/update/clean 生命周期、buffer 长度/capacity、copy loop、索引边界、错误码/out-param、常量/static 初始化、条件编译、回调/函数表、依赖调用/FFI/shim。"
        unmapped_rule = "如果某个 Rust 翻译结果找不到对应 C/C++ 证据，不能静默跳过：若它会影响外部行为，标为 open/blocked_external；若确认为无外部影响，写入 coverage_summary 的 reviewed_non_observable。"
        residual_rule = "Residual downgrade guard：residual_risks 只能用于完成指定源码/anchor/usage 搜索后仍无法建立外部可观察 contract、或无法证明 Rust 当前行为存在差异的区域；不能用于已证实的 public-observable mismatch。"
        blocker_rule = "如果存在任一确定证据，禁止写 residual，必须写 blocking_mismatch 或 blocked_external：C/C++ header/public/exported API 声明存在但 Rust 没有等价 public entry 或可达实现；原测试、示例、consumer usage、semantic_review_plan、ABI anchor 或 DLD anchor 明确调用或依赖该行为；Rust 当前实现返回值、错误码、out/ref 参数、panic/unwrap、状态副作用、资源生命周期、callback/FFI 行为与 C/C++ 证据冲突；缺失项在当前 Rust 写范围内可修复。"
        blocker_summary = "externally observable mismatch"
        open_rule = "只把 high-confidence、public observable mismatch 标为 open。"
        obligation_id_example = "EXT::Project::PublicApi::dimension"
        obligation_subject_example = "PublicApi"
        obligation_contract_example = "one public-observable contract"
    semantic_review_plan_line = f"semantic_review_plan: {semantic_review_plan_path.resolve()}" if semantic_review_plan_path is not None else "semantic_review_plan: <not generated>"
    return f"""# Semantic Audit Task

## Objective
{objective}

## Inputs
- agent_context_path: {context_path.resolve()}
- rendered_root: {rendered_root.resolve()}
- cargo_manifest: {manifest_path.resolve()}
- {semantic_review_plan_line}
- repair context markdown 和 facts JSON 路径在 agent context 的 information_paths 中。

## Output Paths
- semantic_blockers_ledger.json: {ledger_path.resolve()}
- Markdown report: {report_path.resolve()}

## Evidence Policy
- 不读取、不运行、不引用 held-out Rust tests；论文测试不能泄漏到修复阶段。
- 不修改 Rust、C/C++、extracted facts、type-generation report 或日志。
- 使用 agent context 中的绝对路径读取事实文档；stub/type 细节只从文档路径读取。
- 优先从 agent context 的 information_paths.resolved_source_evidence 或 repair_context_facts.resolved_source_evidence 打开 production_sources、public_headers、test_or_example_usage 中的原始源码/header 绝对路径。
- functions_manifest、extracted/functions、type reports、requirements/DLD、semantic_context_index、review_plan 和旧 ledger/report 只能作为索引、导航或待复核摘要，不能替代源码证据。
- semantic_review_plan 是本轮 coverage seed 索引；必须优先读取，围绕其中 seeds 建立或补齐 semantic_obligations，并尽量在每条 obligation 写入对应 `seed_ids`。
- semantic_obligations 中的 status=`equivalent` / status=`redesigned_equivalent` / proved/已证明项不能只基于中间产物摘要，必须有原始 C/C++ 源码/header 或明确 usage 证据支撑。
- cpp_trace、cpp_evidence、searched_paths 优先写原始 `.c/.cc/.cpp/.h/.hpp` 或 public header 的 `path:line`；只有原始源码不可用时才写 extracted/functions 片段，并说明原因。
- 如果 semantic_blockers_ledger.json 已存在，必须先读取旧 ledger，复查旧 blocker / obligation 后再更新状态；不能因为本轮没有重新想到就删除旧条目。
- 只产出 ledger 和 report；不要创建额外 trace/review/scratch 文档。

## Required Workflow
每轮都必须按顺序完成两个阶段；Phase A 全 fixed 不能直接 accept。

Phase A: prior blocker re-evaluation
- 读取旧 ledger/report 后，逐个复核旧 blocker/root cause/obligation。
- 每个旧项只给一个短结论：fixed / still_open / blocked_external / residual，并保留证据路径。

Phase B: exhaustive translated-result semantic discovery
- 完成 Phase A 后，必须重新做一轮独立的新问题发现；不能只抽样 3-5 个 anchor。
- 读取 semantic_review_plan 中的 seeds；能判定的 seed 必须通过 semantic_obligations[].seed_ids 标记覆盖，证据不足的 seed 写入 residual_risks。
- 穷尽扫描 rendered_root 下最终 Rust 翻译结果：至少覆盖 `src/**/*.rs`、`build.rs` 中会影响构建/链接/运行语义的项，并跳过 `target/`、临时测试文件、agent 日志和非源码产物。
- {inventory_rule}
- {contract_rule}
- {unmapped_rule}
- 如果发现 high-confidence mismatch，按共同根因归并 root_cause_clusters；不要人为限制为 3 个，必须覆盖本轮发现的全部 open/blocked_external root cause。
- {residual_rule}
- {blocker_rule}
- 每条 residual 必须写明 searched_paths 或 evidence class、缺少哪类证据、为什么不满足 blocker 条件；不能把已有证据的缺失或不等价降级成 residual。
- 如果没有新 blocker，report 必须写明 coverage inventory 的数量摘要和已扫描范围，而不是只列少量 anchor。
- uncertain 只能进 residual_risks，不阻断；不要为了满足流程硬造 blocker。

ledger JSON contract：
{{
  "schema_version": "rust_semantic_blockers_ledger_v1",
  "verdict": "accepted | rejected | accepted_with_residual_risks",
  "semantic_obligations": [
    {{
      "id": "{obligation_id_example}",
      "seed_ids": ["seed ids from semantic_review_plan when applicable"],
      "api": "{obligation_subject_example}",
      "dimension": "success_path | error_code | enum_value | out_param | resource_lifetime | function_table | dependency_call | other",
      "status": "equivalent | redesigned_equivalent | blocking_mismatch | uncertain | blocked_external",
      "cpp_trace": ["path:line evidence"],
      "rust_trace": ["path:line evidence"],
      "observable_contract": "{obligation_contract_example}",
      "root_cause_cluster_id": "RC::Project::cause",
      "repair_target": "specific Rust repair target, empty when not blocking"
    }}
  ],
  "root_cause_clusters": [
    {{
      "id": "RC::Project::cause",
      "status": "open | fixed | blocked_external | residual",
      "affected_obligations": ["{obligation_id_example}"],
      "summary": "shared root cause",
      "cpp_evidence": "path:line",
      "rust_evidence": "path:line",
      "repair_kind": "rust_only | needs_type_regen | blocked_by_stub_types | needs_external_binding",
      "repair_strategy": "what the repair agent should do"
    }}
  ],
  "blockers": [
    {{
      "id": "SEM::Symbol::category",
      "status": "open | fixed | blocked_external | residual",
      "summary": "{blocker_summary}",
      "cpp_evidence": "path:line",
      "rust_evidence": "path:line",
      "repair_target": "specific Rust repair target or external condition"
    }}
  ],
  "coverage_summary": {{
    "rust_source_files_scanned": 0,
    "translated_functions_scanned": 0,
    "public_or_exported_items_scanned": 0,
    "private_helpers_scanned": 0,
    "state_or_layout_items_scanned": 0,
    "reviewed_non_observable": 0,
    "unmapped_items": []
  }},
  "residual_risks": [],
  "report_path": "{report_path.resolve()}",
  "summary": "one-line summary"
}}

规则：
- {open_rule}
- root_cause_clusters 是 repair agent 的唯一语义修复输入；每个 blocking_mismatch obligation 必须有 root_cause_cluster_id，并在 root_cause_clusters 中有对应 open / blocked_external / residual cluster。
- open root cause cluster 必须写清 repair_kind 和 repair_strategy，让 repair agent 只按指定 cluster 修 Rust 代码。
- stub 类型导致无法证明或无法修的问题，标为 blocked_external 或 root cause repair_kind=blocked_by_stub_types，不能要求 repair agent 猜字段布局。
- 只有 Phase A 已复核、Phase B 已按 coverage inventory 完成全量扫描，且没有 open root cause / blocking_mismatch obligation 时，verdict=accepted 或 accepted_with_residual_risks；blocked_external 必须保留证据和理由，但不阻断 accepted_with_residual_risks。
- report 保持简短：不要写长篇审查日志、不要列大表格、不要重复粘贴源码、不要写逐文件扫描流水账；必须包含 coverage_summary 摘要、blocker 摘要和必要证据路径。
- finish 前必须重新读取 semantic_blockers_ledger.json，确认文件非空、可解析 JSON、schema_version/verdict 合法、report_path 指向的 Markdown report 已写入；如果存在 blocking_mismatch obligation，必须有对应 open root_cause_cluster。

完成时使用 finish action，status=done。
"""


def _find_rendered_root(workspace_dir: Path, project: str, llm_name: str) -> Path:
    """Find final Rust project root."""
    candidates = [
        workspace_dir / "final_projects" / project / f"translate_by_{llm_name}",
        workspace_dir / "incremental_work" / project / f"translate_by_{llm_name}",
        workspace_dir / "skeletons" / project,
    ]
    for candidate in candidates:
        if (candidate / "Cargo.toml").is_file():
            return candidate.resolve()
    raise FileNotFoundError(f"未找到最终 Rust 项目 Cargo.toml，检查路径：{', '.join(str(x) for x in candidates)}")


def run_check(args: argparse.Namespace) -> int:
    """CLI subcommand: run cheap gates only."""
    rendered_root = Path(args.rendered_root).expanduser().resolve()
    manifest = rendered_root / "Cargo.toml"
    out = Path(args.output_dir).expanduser().resolve()
    out.mkdir(parents=True, exist_ok=True)
    cargo = _run_cargo_check(manifest, out / "cargo_check.json")
    clippy = _run_cargo_clippy_gate(manifest, out / "cargo_clippy_check.json")
    suite = _normalize_suite(getattr(args, "suite", "ohos"))
    ohos = _run_suite_rustc_check(suite, rendered_root, manifest, out / "ohos_rustc_check.json", Path(args.ohos_rustc).expanduser().resolve(), args.ohos_rust_target)
    bundle = _write_gate_bundle(out / "gate_bundle.json", "check", cargo, clippy, ohos, suite=suite)
    summary = _blocking_summary(bundle)
    _write_json(out / "summary.json", summary)
    print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if summary.get("accepted_by_gates") else 1


def run_agentic(args: argparse.Namespace) -> int:
    """CLI subcommand: run post-generation repair loop."""
    args.suite = _normalize_suite(getattr(args, "suite", "ohos"))
    workspace_dir = Path(args.workspace_dir).expanduser().resolve()
    output_dir = Path(args.output_dir).expanduser().resolve()
    _progress(f"run start project={args.project} llm={args.llm_name} workspace={workspace_dir} output={output_dir}")
    try:
        rendered_root = Path(args.rendered_root).expanduser().resolve() if args.rendered_root else _find_rendered_root(workspace_dir, args.project, args.llm_name)
    except FileNotFoundError as exc:
        rendered_root = (
            Path(args.rendered_root).expanduser().resolve()
            if args.rendered_root
            else (workspace_dir / "final_projects" / args.project / f"translate_by_{args.llm_name}").resolve()
        )
        manifest = rendered_root / "Cargo.toml"
        _progress(f"run rejected missing rendered crate: {exc}")
        return _write_terminal_rejected_result(
            output_dir=output_dir,
            workspace_dir=workspace_dir,
            rendered_root=rendered_root,
            manifest=manifest,
            project=args.project,
            llm_name=args.llm_name,
            failure_kind="missing_rendered_crate",
            failure_message=str(exc),
            ohos_rustc=Path(args.ohos_rustc).expanduser().resolve(),
            ohos_rust_target=args.ohos_rust_target,
            suite=args.suite,
        )
    manifest = rendered_root / "Cargo.toml"
    if not manifest.is_file():
        _progress(f"run rejected missing Cargo.toml: {manifest}")
        return _write_terminal_rejected_result(
            output_dir=output_dir,
            workspace_dir=workspace_dir,
            rendered_root=rendered_root,
            manifest=manifest,
            project=args.project,
            llm_name=args.llm_name,
            failure_kind="missing_rendered_crate",
            failure_message=f"Cargo.toml not found: {manifest}",
            ohos_rustc=Path(args.ohos_rustc).expanduser().resolve(),
            ohos_rust_target=args.ohos_rust_target,
            suite=args.suite,
        )
    output_dir.mkdir(parents=True, exist_ok=True)
    gates_dir = output_dir / "gate_logs"
    semantic_dir = output_dir / "semantic_audit_logs"
    unsafe_audit_dir = output_dir / "unsafe_refactor_audit_logs"
    agent_dir = output_dir / "agentic_logs"
    summary_path = output_dir / "post_repair_summary.json"
    gates_dir.mkdir(parents=True, exist_ok=True)
    semantic_dir.mkdir(parents=True, exist_ok=True)
    unsafe_audit_dir.mkdir(parents=True, exist_ok=True)
    agent_dir.mkdir(parents=True, exist_ok=True)

    audit_runner = OpenAIActionRunner(step_limit=args.semantic_audit_step_limit, timeout_sec=args.agent_timeout_sec)
    repair_runner = OpenAIActionRunner(step_limit=args.agent_step_limit, timeout_sec=args.agent_timeout_sec, log_parent_write_depth=0)
    result: dict[str, Any] = {
        "schema_version": "c2r_post_repair_result_v1",
        "project": args.project,
        "llm_name": args.llm_name,
        "rendered_root": str(rendered_root),
        "output_dir": str(output_dir),
        "started_at": _utc_now(),
        "rounds": [],
    }

    latest_gate_bundle = output_dir / "initial_gate_bundle.json"
    semantic_reference_gate_bundle: Path | None = None
    latest_semantic_payload: dict[str, Any] | None = None
    latest_unsafe_refactor_payload: dict[str, Any] | None = None
    latest_unsafe_review_task: dict[str, Any] | None = None
    unsafe_phase_started = False
    unsafe_review_path_for_current_candidate: Path | None = None
    rust_native_source_roots = _rust_native_refactor_source_roots(args, workspace_dir)
    for round_index in range(0, args.max_rounds + 1):
        label = "initial" if round_index == 0 else f"repair_round_{round_index:02d}"
        next_repair_output_dir = agent_dir / f"repair_round_{round_index + 1:02d}"
        round_started = time.time()
        _progress(f"{label}: round start index={round_index}/{args.max_rounds} rendered_root={rendered_root}")
        _progress(f"{label}: cheap gates start manifest={manifest}")
        cargo = _run_cargo_check(manifest, gates_dir / f"{label}_cargo_check.json")
        clippy = _run_cargo_clippy_gate(manifest, gates_dir / f"{label}_cargo_clippy_check.json")
        ohos = _run_suite_rustc_check(args.suite, rendered_root, manifest, gates_dir / f"{label}_ohos_rustc_check.json", Path(args.ohos_rustc).expanduser().resolve(), args.ohos_rust_target)
        rule_fix_record: dict[str, Any] = {}
        ohos_failed = args.suite == "ohos" and ohos.get("returncode") != 0
        if cargo.get("returncode") != 0 or ohos_failed:
            rule_fix_record = _apply_post_repair_rule_fixes(
                rendered_root,
                _gate_error_text(cargo, ohos),
                output_dir,
                label,
            )
            if rule_fix_record.get("status") == "applied":
                cargo = _run_cargo_check(manifest, gates_dir / f"{label}_cargo_check_after_rule_fix.json")
                clippy = _run_cargo_clippy_gate(manifest, gates_dir / f"{label}_cargo_clippy_check_after_rule_fix.json")
                ohos = _run_suite_rustc_check(args.suite, rendered_root, manifest, gates_dir / f"{label}_ohos_rustc_check_after_rule_fix.json", Path(args.ohos_rustc).expanduser().resolve(), args.ohos_rust_target)
                _progress(f"{label}: rule fix applied changed_files={len(rule_fix_record.get('changed_files', []))}")
        cargo_passed = cargo.get("returncode") == 0
        ohos_passed = True if args.suite == "oss" else ohos.get("returncode") == 0
        cheap_passed = cargo_passed and ohos_passed
        _progress(
            f"{label}: cheap gates end cargo={cargo.get('returncode')} ohos={ohos.get('returncode')} "
            f"clippy={clippy.get('returncode')} cheap_passed={cheap_passed} cargo_log={cargo.get('text_log_path')} ohos_log={ohos.get('text_log_path')}"
        )
        semantic_for_bundle = _skipped_semantic_audit_payload(
            label,
            (
                "cheap gates passed; semantic audit pending for current round"
                if cheap_passed
                else ("cheap gates failed; semantic audit requires cargo check first" if args.suite == "oss" else "cheap gates failed; semantic audit requires cargo check and OHOS rustc first")
            ),
            semantic_dir / f"{label}_semantic_audit.json",
        )
        latest_gate_bundle = _write_gate_bundle(gates_dir / f"{label}_gate_bundle.json", label, cargo, clippy, ohos, semantic_for_bundle, suite=args.suite)
        context_path = _build_repair_context(
            args=args,
            rendered_root=rendered_root,
            manifest_path=manifest,
            latest_gate_bundle_path=latest_gate_bundle,
            round_index=round_index,
            output_dir=output_dir,
            semantic_reference_gate_bundle_path=semantic_reference_gate_bundle,
            unsafe_review_task=latest_unsafe_review_task if unsafe_phase_started else None,
        )
        summary = _blocking_summary(latest_gate_bundle)
        round_record: dict[str, Any] = {"round_index": round_index, "label": label, "gate_bundle": str(latest_gate_bundle), "agent_context": str(context_path), "blocking_summary": summary}
        if rule_fix_record:
            round_record["rule_fix"] = rule_fix_record
        result["rounds"].append(round_record)
        _write_json(summary_path, result)
        _progress(
            f"{label}: gate summary accepted={summary.get('accepted_by_gates')} "
            f"blocking={summary.get('blocking_gates')} pending={summary.get('pending_gates')} context={context_path}"
        )

        if summary.get("cheap_gates_passed"):
            latest_semantic = _run_semantic_audit(args=args, runner=audit_runner, round_label=label, rendered_root=rendered_root, manifest_path=manifest, context_path=context_path, log_path=semantic_dir / f"{label}_semantic_audit.json")
            latest_semantic_payload = latest_semantic
            latest_gate_bundle = _write_gate_bundle(gates_dir / f"{label}_gate_bundle.json", label, cargo, clippy, ohos, latest_semantic, suite=args.suite)
            if _semantic_audit_result_executed(latest_semantic):
                semantic_reference_gate_bundle = latest_gate_bundle
            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered_root,
                manifest_path=manifest,
                latest_gate_bundle_path=latest_gate_bundle,
                round_index=round_index,
                output_dir=output_dir,
                semantic_reference_gate_bundle_path=semantic_reference_gate_bundle,
            )
            summary = _blocking_summary(latest_gate_bundle)
            round_record.update({"semantic_audit": latest_semantic, "gate_bundle": str(latest_gate_bundle), "agent_context": str(context_path), "blocking_summary": summary})
            semantic_passed = _semantic_audit_passed(latest_semantic)
            _progress(
                f"{label}: semantic gate status={latest_semantic.get('status')} verdict={latest_semantic.get('verdict')} "
                f"passed={semantic_passed} blocking={summary.get('blocking_gates')} context={context_path}"
            )
            if semantic_passed or unsafe_phase_started:
                unsafe_phase_started = True
                unsafe_review_task = _write_unsafe_review_task(
                    rendered_root,
                    next_repair_output_dir,
                    f"repair_round_{round_index + 1:02d}",
                    source_roots=rust_native_source_roots,
                    suite=str(getattr(args, "suite", "ohos") or "ohos"),
                )
                latest_unsafe_review_task = unsafe_review_task
                unsafe_review_path_for_current_candidate = Path(str(unsafe_review_task.get("review_json_path", "")))
                round_record["unsafe_optimization_required"] = {
                    "reason": "Rust-native refactor phase active; semantic repair must keep reviewing refactor items",
                    "review_json_path": str(unsafe_review_path_for_current_candidate),
                    "item_count": unsafe_review_task.get("item_count", 0),
                }
                round_record["rust_native_refactor_required"] = dict(round_record["unsafe_optimization_required"])
                _progress(
                    f"{label}: unsafe phase active item_count={unsafe_review_task.get('item_count', 0)} "
                    f"review_json={unsafe_review_path_for_current_candidate}"
                )
                context_path = _build_repair_context(
                    args=args,
                    rendered_root=rendered_root,
                    manifest_path=manifest,
                    latest_gate_bundle_path=latest_gate_bundle,
                    round_index=round_index,
                    output_dir=output_dir,
                    semantic_reference_gate_bundle_path=semantic_reference_gate_bundle,
                    unsafe_review_task=unsafe_review_task,
                )
                latest_unsafe_refactor = _run_unsafe_refactor_audit(
                    runner=audit_runner,
                    round_label=label,
                    rendered_root=rendered_root,
                    manifest_path=manifest,
                    context_path=context_path,
                    log_path=unsafe_audit_dir / f"{label}_unsafe_refactor_audit.json",
                    suite=str(getattr(args, "suite", "ohos") or "ohos"),
                )
                latest_unsafe_refactor_payload = latest_unsafe_refactor
                latest_gate_bundle = _write_gate_bundle(
                    gates_dir / f"{label}_gate_bundle.json",
                    label,
                    cargo,
                    clippy,
                    ohos,
                    latest_semantic,
                    unsafe_refactor=latest_unsafe_refactor,
                    suite=args.suite,
                )
                context_path = _build_repair_context(
                    args=args,
                    rendered_root=rendered_root,
                    manifest_path=manifest,
                    latest_gate_bundle_path=latest_gate_bundle,
                    round_index=round_index,
                    output_dir=output_dir,
                    semantic_reference_gate_bundle_path=semantic_reference_gate_bundle,
                    unsafe_review_task=unsafe_review_task,
                )
                summary = _blocking_summary(latest_gate_bundle)
                round_record.update(
                    {
                        "unsafe_refactor_audit": latest_unsafe_refactor,
                        "gate_bundle": str(latest_gate_bundle),
                        "agent_context": str(context_path),
                        "blocking_summary": summary,
                    }
                )
                if not _unsafe_refactor_audit_passed(latest_unsafe_refactor):
                    round_record["unsafe_optimization_required"] = {
                        "reason": "unsafe auditor has open reducible items or invalid ledger",
                        "review_json_path": str(unsafe_review_path_for_current_candidate),
                        "ledger_path": str(latest_unsafe_refactor.get("ledger_path", "")),
                        "report_path": str(latest_unsafe_refactor.get("report_path", "")),
                        "open_reducible_item_count": latest_unsafe_refactor.get("open_reducible_item_count", 0),
                    }
                    round_record["rust_native_refactor_required"] = dict(round_record["unsafe_optimization_required"])
                _write_json(summary_path, result)
                _progress(
                    f"{label}: combined gate summary accepted={summary.get('accepted_by_gates')} "
                    f"blocking={summary.get('blocking_gates')} unsafe_status={latest_unsafe_refactor.get('status')} "
                    f"open_items={latest_unsafe_refactor.get('open_reducible_item_count', 0)}"
                )
            if summary.get("accepted_by_gates"):
                _progress(f"{label}: final verify start")
                final_summary = _write_final_verify_artifacts(
                    output_dir=output_dir,
                    rendered_root=rendered_root,
                    manifest=manifest,
                    ohos_rustc=Path(args.ohos_rustc).expanduser().resolve(),
                    ohos_rust_target=args.ohos_rust_target,
                    suite=args.suite,
                    semantic_payload=latest_semantic,
                    unsafe_refactor_payload=latest_unsafe_refactor_payload if unsafe_phase_started else None,
                )
                _progress(
                    f"{label}: final verify end accepted={final_summary.get('accepted_by_gates')} "
                    f"blocking={final_summary.get('blocking_gates')} gate_bundle={final_summary.get('final_gate_bundle')}"
                )
                result["final_verify_summary"] = final_summary
                round_record["final_verify_summary"] = final_summary
                if final_summary.get("accepted_by_gates"):
                    result.update({"final_status": "accepted", "accepted_round": round_index, "finished_at": _utc_now()})
                    _write_json(summary_path, result)
                    _write_canonical_post_repair_result(
                        output_dir=output_dir,
                        result=result,
                        workspace_dir=workspace_dir,
                        rendered_root=rendered_root,
                        summary_path=summary_path,
                    )
                    _progress(f"{label}: run accepted elapsed={round(time.time() - round_started, 3)}s summary={summary_path}")
                    return 0
                latest_gate_bundle = Path(str(final_summary.get("final_gate_bundle") or latest_gate_bundle))
                context_path = _build_repair_context(
                    args=args,
                    rendered_root=rendered_root,
                    manifest_path=manifest,
                    latest_gate_bundle_path=latest_gate_bundle,
                    round_index=round_index,
                    output_dir=output_dir,
                    semantic_reference_gate_bundle_path=semantic_reference_gate_bundle,
                    unsafe_review_task=latest_unsafe_review_task if unsafe_phase_started else None,
                )
                summary = _blocking_summary(latest_gate_bundle)
                round_record.update({"gate_bundle": str(latest_gate_bundle), "agent_context": str(context_path), "blocking_summary": summary})
                _write_json(summary_path, result)
        else:
            unsafe_review_path_for_current_candidate = None
        if round_index >= args.max_rounds:
            _progress(f"{label}: repair budget exhausted before next repair round")
            break

        repair_prompt = _build_repair_prompt(context_path)
        repair_output_dir = next_repair_output_dir
        repair_name = f"agentic_repair_round_{round_index + 1:02d}"
        repair_started = time.time()
        _progress(f"{label}: repair agent start name={repair_name} task_context={context_path} output={repair_output_dir}")
        try:
            agent_run = repair_runner.run(task=repair_prompt, cwd=rendered_root, output_dir=repair_output_dir, name=repair_name)
            agent_payload = agent_run.to_dict()
        except Exception as exc:  # noqa: BLE001
            repair_output_dir.mkdir(parents=True, exist_ok=True)
            agent_payload = {
                "returncode": 127,
                "error_type": type(exc).__name__,
                "error": str(exc),
                "cwd": str(rendered_root),
                "output_dir": str(repair_output_dir),
                "command": ["openai-compatible-repair-agent", repair_name],
            }
            _write_json(repair_output_dir / f"{repair_name}.result.json", agent_payload)
            round_record["repair_agent"] = agent_payload
            round_record["repair_agent_exception"] = agent_payload
            _write_json(summary_path, result)
            _progress(f"{label}: repair agent exception name={repair_name} error={type(exc).__name__}: {exc}")
            break
        round_record["repair_agent"] = agent_payload
        _write_json(summary_path, result)
        _progress(
            f"{label}: repair agent end name={repair_name} returncode={agent_payload.get('returncode')} "
            f"elapsed={round(time.time() - repair_started, 3)}s result={agent_payload.get('result_path')} diff={agent_payload.get('workspace_diff_path')}"
        )

    final_summary = result.get("final_verify_summary") if isinstance(result.get("final_verify_summary"), dict) else None
    if final_summary is None:
        _progress("final verify start after repair loop")
        final_summary = _write_final_verify_artifacts(
            output_dir=output_dir,
            rendered_root=rendered_root,
            manifest=manifest,
            ohos_rustc=Path(args.ohos_rustc).expanduser().resolve(),
            ohos_rust_target=args.ohos_rust_target,
            suite=args.suite,
            semantic_payload=latest_semantic_payload,
            unsafe_refactor_payload=latest_unsafe_refactor_payload if unsafe_phase_started else None,
        )
        result["final_verify_summary"] = final_summary
        _progress(
            f"final verify end accepted={final_summary.get('accepted_by_gates')} "
            f"blocking={final_summary.get('blocking_gates')} gate_bundle={final_summary.get('final_gate_bundle')}"
        )
    if unsafe_phase_started and not _unsafe_refactor_audit_passed(latest_unsafe_refactor_payload):
        guarded = dict(final_summary)
        blocking = guarded.get("blocking_gates") if isinstance(guarded.get("blocking_gates"), list) else []
        if UNSAFE_REFACTOR_AUDIT_GATE not in blocking:
            blocking = [*blocking, UNSAFE_REFACTOR_AUDIT_GATE]
        guarded["accepted_by_gates"] = False
        guarded["blocking_gates"] = blocking
        guarded["unsafe_refactor_audit_satisfied"] = False
        guarded["unsafe_refactor_failure_kind"] = "unsafe_refactor_audit_not_accepted_after_repair_budget"
        guarded["unsafe_refactor_ledger_path"] = str((latest_unsafe_refactor_payload or {}).get("ledger_path", ""))
        guarded["unsafe_refactor_open_reducible_item_count"] = int((latest_unsafe_refactor_payload or {}).get("open_reducible_item_count", 0) or 0)
        diagnostics = guarded.get("diagnostics") if isinstance(guarded.get("diagnostics"), list) else []
        guarded["diagnostics"] = [*diagnostics, "unsafe refactor auditor did not accept after available repair rounds"]
        _write_json(output_dir / "final_verify_summary.json", guarded)
        final_summary = guarded
        result["final_verify_summary"] = final_summary
    result.update({"final_status": "rejected", "accepted_round": None, "finished_at": _utc_now()})
    _write_json(summary_path, result)
    _write_canonical_post_repair_result(
        output_dir=output_dir,
        result=result,
        workspace_dir=workspace_dir,
        rendered_root=rendered_root,
        summary_path=summary_path,
    )
    _progress(f"run rejected summary={summary_path} final_blocking={final_summary.get('blocking_gates') if isinstance(final_summary, dict) else []}")
    return 1


def _parse_args() -> argparse.Namespace:
    """Parse CLI args."""
    parser = argparse.ArgumentParser(description="Run post-generation Rust repair and semantic audit loop.")
    sub = parser.add_subparsers(dest="command", required=True)

    check = sub.add_parser("check", help="Run cheap gates only.")
    check.add_argument("--rendered-root", required=True)
    check.add_argument("--output-dir", required=True)
    check.add_argument("--project", default="")
    check.add_argument("--workspace-dir", default="")
    check.add_argument("--body-target-scope", default="")
    check.add_argument("--suite", default="ohos", choices=["ohos", "oss"])
    check.add_argument("--ohos-rustc", default=os.environ.get("OHOS_RUSTC", str(DEFAULT_OHOS_RUSTC)))
    check.add_argument("--ohos-rust-target", default=os.environ.get("OHOS_RUST_TARGET", "x86_64-unknown-linux-ohos"))

    run = sub.add_parser("run", help="Run repair loop.")
    run.add_argument("--project", required=True)
    run.add_argument("--llm-name", required=True)
    run.add_argument("--workspace-dir", required=True)
    run.add_argument("--output-dir", required=True)
    run.add_argument("--source-project-root", default="")
    run.add_argument("--rendered-root", default="")
    run.add_argument("--suite", default="ohos", choices=["ohos", "oss"])
    run.add_argument("--max-rounds", type=int, default=int(os.environ.get("C2R_POST_REPAIR_MAX_ROUNDS", "50")))
    run.add_argument("--agent-step-limit", type=int, default=int(os.environ.get("C2R_POST_REPAIR_AGENT_STEP_LIMIT", "160")))
    run.add_argument("--semantic-audit-step-limit", type=int, default=int(os.environ.get("C2R_SEMANTIC_AUDIT_STEP_LIMIT", "320")))
    run.add_argument("--agent-timeout-sec", type=float, default=float(os.environ.get("C2R_POST_REPAIR_AGENT_TIMEOUT_SEC", "3600")))
    run.add_argument("--ohos-rustc", default=os.environ.get("OHOS_RUSTC", str(DEFAULT_OHOS_RUSTC)))
    run.add_argument("--ohos-rust-target", default=os.environ.get("OHOS_RUST_TARGET", "x86_64-unknown-linux-ohos"))
    run.add_argument("--allow-external-blockers", action="store_true")
    return parser.parse_args()


def main() -> int:
    """CLI entrypoint."""
    args = _parse_args()
    if args.command == "check":
        return run_check(args)
    if args.command == "run":
        return run_agentic(args)
    raise AssertionError(args.command)


if __name__ == "__main__":
    raise SystemExit(main())
