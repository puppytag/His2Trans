#!/usr/bin/env python3
"""生成 Rust-native 重构审查任务，融合 unsafe、ABI 与死符号候选。"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.agentic_repair.abi_refactor_inventory import run_abi_refactor_inventory
from scripts.agentic_repair.unsafe_scope_gate import (
    IGNORED_GENERATED_DIRS,
    _rust_source_fingerprint,
    _top_file_scopes,
    run_unsafe_scope_gate,
)

RUST_NATIVE_REFACTOR_GATE = "rust_native_refactor"
RUST_NATIVE_REFACTOR_ALLOWED_DECISIONS = {
    "refactored",
    "deleted_dead_code",
    "kept_required",
    "kept_risky",
    "no_change_needed",
    "optimized",  # legacy alias kept for existing agents/tests.
}
UNSAFE_REDUCTION_DECISIONS = {"refactored", "deleted_dead_code", "optimized"}
UNSAFE_REDUCTION_SUMMARY_KEYS = (
    "unsafe_total_lines",
    "unsafe_total_ratio",
    "unsafe_context_lines",
    "unsafe_keyword_lines",
    "scope_count",
    "code_lines",
    "file_count",
    "files_with_unsafe_count",
)


def _write_json(path: Path, payload: Any) -> None:
    """写入稳定 JSON。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def _read_json_object(path: Path) -> dict[str, Any]:
    """读取 JSON object。"""
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    return payload if isinstance(payload, dict) else {}


def _number(value: Any, default: float = 0.0) -> float:
    """安全读取数字字段。"""
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def _compact_unsafe_summary(summary: dict[str, Any]) -> dict[str, Any]:
    """压缩 unsafe summary，只保留完成校验需要的稳定字段。"""
    return {key: summary.get(key, 0) for key in UNSAFE_REDUCTION_SUMMARY_KEYS}


def _unsafe_file_metrics(file_summary: dict[str, Any]) -> dict[str, Any]:
    """提取文件级 unsafe 指标。"""
    return {
        "unsafe_total_lines": file_summary.get("unsafe_total_lines", 0),
        "unsafe_total_ratio": file_summary.get("unsafe_total_ratio", 0),
        "unsafe_context_lines": file_summary.get("unsafe_context_lines", 0),
        "unsafe_keyword_lines": file_summary.get("unsafe_keyword_lines", 0),
        "scope_count": file_summary.get("scope_count", 0),
        "code_lines": file_summary.get("code_lines", 0),
    }


def _scope_metrics(scopes: list[dict[str, Any]]) -> dict[str, Any]:
    """提取文件内最大 unsafe 作用域和 unsafe extern 作用域指标。"""
    max_scope_lines = 0
    large_scope_count = 0
    unsafe_extern_total_lines = 0
    max_unsafe_extern_lines = 0
    for scope in scopes:
        lines = int(_number(scope.get("lines"), 0))
        kind = str(scope.get("kind") or "")
        max_scope_lines = max(max_scope_lines, lines)
        if lines >= 20:
            large_scope_count += 1
        if kind == "unsafe_extern":
            unsafe_extern_total_lines += lines
            max_unsafe_extern_lines = max(max_unsafe_extern_lines, lines)
    return {
        "max_scope_lines": max_scope_lines,
        "large_scope_count": large_scope_count,
        "unsafe_extern_total_lines": unsafe_extern_total_lines,
        "max_unsafe_extern_lines": max_unsafe_extern_lines,
    }


def _unsafe_reduction_metrics(file_summary: dict[str, Any], scopes: list[dict[str, Any]]) -> dict[str, Any]:
    """组合文件级 unsafe 行数和作用域指标。"""
    return {**_unsafe_file_metrics(file_summary), **_scope_metrics(scopes)}


def _previous_unsafe_reduction_goal(review_json: Path) -> dict[str, Any]:
    """读取已有 review JSON 的 unsafe 降低 baseline，用于 regenerate 后保持同一阶段基线。"""
    previous = _read_json_object(review_json)
    goal = previous.get("unsafe_reduction_goal") if isinstance(previous, dict) else {}
    return goal if isinstance(goal, dict) else {}


def _metric_reduced(baseline: dict[str, Any], current: dict[str, Any], key: str) -> bool:
    """判断某个 unsafe 指标是否相对 baseline 下降。"""
    return _number(current.get(key), 0) < _number(baseline.get(key), 0)


def _item_has_unsafe_reduction(item: dict[str, Any]) -> bool:
    """判断 review item 是否有可校验的 unsafe 降低。"""
    baseline = item.get("baseline_unsafe_metrics") if isinstance(item.get("baseline_unsafe_metrics"), dict) else {}
    current = item.get("current_unsafe_metrics") if isinstance(item.get("current_unsafe_metrics"), dict) else {}
    if _number(current.get("unsafe_extern_total_lines"), 0) > _number(baseline.get("unsafe_extern_total_lines"), 0):
        return False
    baseline_has_large_scope = _number(baseline.get("large_scope_count"), 0) > 0 or _number(baseline.get("max_scope_lines"), 0) >= 20
    if baseline_has_large_scope:
        return _metric_reduced(baseline, current, "large_scope_count") or _metric_reduced(baseline, current, "max_scope_lines")
    reduction_keys = ("unsafe_total_lines", "unsafe_context_lines", "scope_count", "unsafe_extern_total_lines", "max_unsafe_extern_lines")
    return any(_metric_reduced(baseline, current, key) for key in reduction_keys)


def _sample(items: Any, limit: int = 6) -> list[dict[str, Any]]:
    """限制候选证据数量，避免任务 JSON 过大。"""
    if not isinstance(items, list):
        return []
    return [item for item in items[:limit] if isinstance(item, dict)]


def _compact_abi_candidate(item: dict[str, Any]) -> dict[str, Any]:
    """压缩 ABI/native 候选函数信息。"""
    return {
        "name": item.get("name", ""),
        "rust_line": item.get("rust_line", 0),
        "decision": item.get("decision", ""),
        "confidence": item.get("confidence", ""),
        "is_pub": bool(item.get("is_pub")),
        "is_extern_c": bool(item.get("is_extern_c")),
        "no_mangle": bool(item.get("no_mangle")),
        "recommended_action": item.get("recommended_action", ""),
        "reasons": [str(value) for value in item.get("reasons", []) if str(value).strip()][:4]
        if isinstance(item.get("reasons"), list)
        else [],
    }


def _has_external_or_callback_evidence(item: dict[str, Any]) -> bool:
    """判断函数是否有必须保留符号/回调的证据。"""
    if item.get("no_mangle"):
        return True
    evidence = item.get("evidence") if isinstance(item.get("evidence"), dict) else {}
    evidence_keys = [
        "header_declarations",
        "c_test_calls",
        "dlsym_or_symbol_refs",
        "c_callback_or_vtable_refs",
        "callback_or_vtable_refs",
    ]
    return any(bool(evidence.get(key)) for key in evidence_keys)


def _dead_symbol_candidate(item: dict[str, Any]) -> dict[str, Any] | None:
    """按保守证据生成死符号候选，由 agent 最终确认，不自动删除。"""
    evidence = item.get("evidence") if isinstance(item.get("evidence"), dict) else {}
    rust_refs = int(evidence.get("rust_reference_count") or 0)
    if rust_refs != 0 or _has_external_or_callback_evidence(item):
        return None
    decision = str(item.get("decision") or "")
    if decision == "keep_c_abi":
        return None
    return {
        "name": item.get("name", ""),
        "rust_line": item.get("rust_line", 0),
        "is_pub": bool(item.get("is_pub")),
        "is_extern_c": bool(item.get("is_extern_c")),
        "abi_decision": decision,
        "reason": "no Rust references or external/callback ABI evidence detected by static scan; agent must verify before deletion",
    }


def _group_inventory_by_file(abi_report: dict[str, Any]) -> tuple[dict[str, list[dict[str, Any]]], dict[str, list[dict[str, Any]]]]:
    """按 Rust 文件聚合 ABI 重构候选和死符号候选。"""
    abi_by_file: dict[str, list[dict[str, Any]]] = {}
    dead_by_file: dict[str, list[dict[str, Any]]] = {}
    interesting = {"keep_c_abi", "extern_thunk_to_safe_core", "rust_internal_candidate", "review_required"}
    for item in abi_report.get("functions", []) if isinstance(abi_report.get("functions"), list) else []:
        if not isinstance(item, dict):
            continue
        rel = str(item.get("rust_file") or "").strip()
        if not rel:
            continue
        if str(item.get("decision") or "") in interesting:
            abi_by_file.setdefault(rel, []).append(_compact_abi_candidate(item))
        dead = _dead_symbol_candidate(item)
        if dead is not None:
            dead_by_file.setdefault(rel, []).append(dead)
    return abi_by_file, dead_by_file


def _placeholder_by_file(abi_report: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    """按文件聚合占位/兜底痕迹。"""
    result: dict[str, list[dict[str, Any]]] = {}
    for item in abi_report.get("placeholder_hits", []) if isinstance(abi_report.get("placeholder_hits"), list) else []:
        if not isinstance(item, dict):
            continue
        rel = str(item.get("file") or "").strip()
        if rel:
            result.setdefault(rel, []).append({"line": item.get("line", 0), "detail": item.get("detail", "")})
    return result


def _abs_file(crate_dir: Path, rel: str, file_summary: dict[str, Any] | None = None) -> str:
    """返回文件绝对路径。"""
    if file_summary and str(file_summary.get("abs_file") or "").strip():
        return str(file_summary.get("abs_file"))
    return str((crate_dir / rel).resolve())


def write_rust_native_refactor_task(
    crate_dir: Path,
    unsafe_json: Path,
    unsafe_md: Path,
    abi_json: Path,
    abi_md: Path,
    review_json: Path,
    *,
    source_roots: list[Path] | None = None,
    suite: str = "ohos",
) -> dict[str, Any]:
    """生成强制 Rust-native 重构审查任务 JSON。"""
    crate_dir = crate_dir.expanduser().resolve()
    unsafe_payload = run_unsafe_scope_gate(crate_dir, unsafe_json, unsafe_md)
    abi_payload = run_abi_refactor_inventory(crate_dir, abi_json, abi_md, source_roots=source_roots or [], suite=suite)
    unsafe_report = _read_json_object(Path(unsafe_json))
    abi_report = _read_json_object(Path(abi_json))
    suite = str(abi_report.get("suite") or suite or "ohos").strip().lower()
    if suite not in {"oss", "ohos"}:
        suite = "ohos"

    unsafe_by_file = {
        str(item.get("file") or ""): item
        for item in unsafe_report.get("by_file", [])
        if isinstance(item, dict) and str(item.get("file") or "").strip()
    }
    scopes = unsafe_report.get("scopes", []) if isinstance(unsafe_report.get("scopes"), list) else []
    scopes_by_file: dict[str, list[dict[str, Any]]] = {}
    for scope in scopes:
        if not isinstance(scope, dict):
            continue
        rel = str(scope.get("file") or "").strip()
        if rel:
            scopes_by_file.setdefault(rel, []).append(scope)
    abi_by_file, dead_by_file = _group_inventory_by_file(abi_report)
    placeholders = _placeholder_by_file(abi_report)
    unsafe_summary = unsafe_report.get("summary") if isinstance(unsafe_report.get("summary"), dict) else {}
    previous_goal = _previous_unsafe_reduction_goal(review_json)
    baseline_summary = previous_goal.get("baseline_unsafe_scope") if isinstance(previous_goal.get("baseline_unsafe_scope"), dict) else None
    baseline_by_file = previous_goal.get("baseline_by_file") if isinstance(previous_goal.get("baseline_by_file"), dict) else {}
    if baseline_summary is None:
        baseline_summary = _compact_unsafe_summary(unsafe_summary)
    if not baseline_by_file:
        baseline_by_file = {
            rel: _unsafe_reduction_metrics(file_summary, scopes_by_file.get(rel, []))
            for rel, file_summary in unsafe_by_file.items()
        }

    file_names = sorted(set(unsafe_by_file) | set(abi_by_file) | set(dead_by_file) | set(placeholders))
    items: list[dict[str, Any]] = []
    for rel in file_names:
        file_summary = unsafe_by_file.get(rel, {})
        current_metrics = _unsafe_reduction_metrics(file_summary, scopes_by_file.get(rel, []))
        baseline_metrics = baseline_by_file.get(rel) if isinstance(baseline_by_file.get(rel), dict) else current_metrics
        items.append(
            {
                "id": f"F{len(items) + 1:04d}",
                "kind": "rust_native_refactor_file_review",
                "file": rel,
                "abs_file": _abs_file(crate_dir, rel, file_summary),
                "scope_count": file_summary.get("scope_count", 0),
                "code_lines": file_summary.get("code_lines", 0),
                "unsafe_context_lines": file_summary.get("unsafe_context_lines", 0),
                "unsafe_keyword_lines": file_summary.get("unsafe_keyword_lines", 0),
                "unsafe_total_lines": file_summary.get("unsafe_total_lines", 0),
                "unsafe_total_ratio": file_summary.get("unsafe_total_ratio", 0),
                "baseline_unsafe_metrics": baseline_metrics,
                "current_unsafe_metrics": current_metrics,
                "unsafe_reduction_required": bool(
                    _number(baseline_metrics.get("unsafe_total_lines"), 0) > 0
                    or _number(baseline_metrics.get("unsafe_extern_total_lines"), 0) > 0
                    or _number(baseline_metrics.get("large_scope_count"), 0) > 0
                ),
                "top_scopes": _top_file_scopes(scopes, rel),
                "abi_candidates": _sample(abi_by_file.get(rel, []), 16),
                "dead_symbol_candidates": _sample(dead_by_file.get(rel, []), 16),
                "placeholder_hits": _sample(placeholders.get(rel, []), 12),
                "decision": "",
                "reason": "",
                "result": "",
            }
        )

    abi_summary = abi_report.get("summary") if isinstance(abi_report.get("summary"), dict) else {}
    current_summary = _compact_unsafe_summary(unsafe_summary)
    review_payload = {
        "schema_version": "c2r_rust_native_refactor_task_v1",
        "gate": RUST_NATIVE_REFACTOR_GATE,
        "suite": suite,
        "mode": "required_after_semantic_acceptance",
        "project_root": str(crate_dir),
        "source_fingerprint_sha256": _rust_source_fingerprint(crate_dir),
        "unsafe_scope_json_path": str(Path(unsafe_json).expanduser().resolve()),
        "unsafe_scope_markdown_path": str(Path(unsafe_md).expanduser().resolve()),
        "abi_inventory_json_path": str(Path(abi_json).expanduser().resolve()),
        "abi_inventory_markdown_path": str(Path(abi_md).expanduser().resolve()),
        "summary": {
            "item_count": len(items),
            "unsafe_scope": unsafe_summary,
            "abi_refactor": abi_summary,
            "dead_symbol_candidate_count": sum(len(value) for value in dead_by_file.values()),
        },
        "unsafe_reduction_goal": {
            "primary_goal": (
                "For standalone OSS/C project translation, drive raw unsafe to zero or near zero while preserving Rust-native project behavior."
                if suite == "oss"
                else "For an OHOS integration project, reduce unnecessary unsafe while preserving compile behavior, platform semantics, and required public ABI."
            ),
            "baseline_unsafe_scope": baseline_summary,
            "current_unsafe_scope": current_summary,
            "baseline_by_file": baseline_by_file,
            "must_reduce_when_possible": True,
            "completion_rule": "A refactored/deleted_dead_code/optimized item must reduce that file's unsafe_total_lines, max unsafe scope size, large unsafe scope count, or unsafe_extern scope lines versus the baseline. If no reduction is possible, use kept_required/kept_risky with specific evidence.",
        },
        "excluded_generated_dirs": sorted(IGNORED_GENERATED_DIRS),
        "allowed_decisions": sorted(RUST_NATIVE_REFACTOR_ALLOWED_DECISIONS),
        "instructions": [
            "逐个文件审查 items；每一项必须填写 decision、reason、result。",
            (
                "本阶段面向独立 OSS 小项目翻译，第一目标是把 raw unsafe 降到 0 或接近 0；C 测试入口和原 C 函数形状只是后续论文测试 harness 的语义参考，不是当前翻译产物必须保留的 ABI。"
                if suite == "oss"
                else "本阶段面向 OHOS 集成项目；在保持 public ABI、平台调用约定、公开布局和平台行为的前提下缩小不必要的 unsafe，不能把 C 内部实现形状当作默认约束。"
            ),
            "必须优先缩小大 unsafe block、拆出 safe core、删除确认无用符号，并避免 unsafe extern fn 把整个函数体计入 unsafe scope。",
            (
                "遇到 extern C raw-pointer 入口或 unsafe fn 时，优先改成安全 Rust API 或 safe core；只有真实外部 FFI、系统 API、callback/function pointer、资源生命周期等证据存在时，才保留最小 unsafe。"
                if suite == "oss"
                else "遇到 unsafe extern fn 或 unsafe fn 时，必须优先考虑 thin extern/unsafe ABI thunk + private safe/core helper；确实不能拆时写明 callback ABI、函数指针类型、调用方可观察行为等具体证据。"
            ),
            "不需要 unsafe 的控制流、局部变量、常量准备、普通条件判断、日志参数准备和普通计算必须尽量移出 unsafe block；不要因为文件内存在 raw pointer/FFI 就把整段逻辑都判定为 kept_required。",
            "refactored/deleted_dead_code/optimized 必须对应 unsafe_total_lines、最大 unsafe scope、large scope 数量或 unsafe_extern scope 行数相对 baseline 下降；只修 ABI 但 unsafe 不下降不能算完成。",
            (
                "约束是独立 Rust 项目的语义等价：保持算法结果、返回值、资源生命周期和 Rust 调用方可观察行为；不要求保留 C 外部入口、extern 调用约定或原 C 函数签名。"
                if suite == "oss"
                else "约束是语义等价：保持 public ABI、导出符号、extern 调用约定、公开 repr(C) 类型布局和测试/调用方可观察行为不变；private/internal 数据结构、状态表示、控制流和 helper API 可以重写为更安全的 Rust 形态。"
            ),
            "对 C 源中的 static/private helper、private struct、intrusive list、手写全局状态等，不要求一对一保留 C 内部实现；如果能用 Rust-owned registry、NonNull/newtype、封装后的窄 unsafe helper 或安全容器表达同等行为，并通过 compile/semantic gate，就应优先尝试。",
            (
                "必须 unsafe 只能来自真实外部 FFI/系统调用、callback ABI、无法安全表达的 raw pointer 交互或 FFI layout 操作；普通算法、C 测试入口、私有 helper 和内部结构不应保留 unsafe。"
                if suite == "oss"
                else "必须 unsafe 只能来自有证据的 public ABI/FFI 合约、raw pointer 交互、callback ABI、extern 调用或 FFI layout 操作；普通算法、私有状态维护、私有 helper 和测试不可观察的内部结构不应保留 unsafe。"
            ),
            "ABI/死符号清单只是候选证据，不能替代源码确认；删除符号前必须确认没有 Rust 调用、C ABI、回调/vtable、动态符号或测试可观察证据。",
            "纯 bindgen 生成 FFI/TU 目录不纳入统计和本审查任务。",
            (
                "不能用删除业务逻辑、默认返回或跳过项目可观察行为来降低 unsafe。"
                if suite == "oss"
                else "不能用删除业务逻辑、默认返回、改变 public ABI、改变外部调用约定、改变公开 repr(C) 类型布局或跳过 C 可观察行为来降低 unsafe。"
            ),
        ],
        "items": items,
    }
    _write_json(Path(review_json), review_payload)
    return {
        "review_json_path": str(Path(review_json).expanduser().resolve()),
        "scope_json_path": str(Path(unsafe_json).expanduser().resolve()),
        "scope_markdown_path": str(Path(unsafe_md).expanduser().resolve()),
        "abi_inventory_json_path": str(Path(abi_json).expanduser().resolve()),
        "abi_inventory_markdown_path": str(Path(abi_md).expanduser().resolve()),
        "summary": review_payload["summary"],
        "item_count": len(items),
        "unsafe_scope_payload": unsafe_payload,
        "abi_inventory_payload": abi_payload,
    }


def rust_native_refactor_status(review_json: Path, crate_dir: Path | None = None) -> dict[str, Any]:
    """返回 Rust-native 重构审查 JSON 的完成状态。"""
    path = Path(review_json).expanduser()
    status: dict[str, Any] = {
        "ok": False,
        "review_json_path": str(path),
        "item_count": 0,
        "missing_count": 0,
        "missing_items": [],
        "diagnostics": [],
        "fingerprint_match": None,
    }
    if not path.is_file():
        status["diagnostics"].append(f"review JSON 不存在: {path}")
        return status
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        status["diagnostics"].append(f"review JSON 读取或解析失败: {type(exc).__name__}: {exc}")
        return status
    items = payload.get("items")
    if not isinstance(items, list):
        status["diagnostics"].append("review JSON 缺少 items 列表")
        return status
    status["item_count"] = len(items)
    if crate_dir is not None:
        expected = str(payload.get("source_fingerprint_sha256", "")).strip()
        current = _rust_source_fingerprint(Path(crate_dir))
        status["source_fingerprint_sha256"] = expected
        status["current_source_fingerprint_sha256"] = current
        status["fingerprint_match"] = bool(expected and expected == current)
        if not status["fingerprint_match"]:
            status["diagnostics"].append("review JSON 源码指纹与当前 crate 不匹配，必须重新生成 Rust-native refactor review")

    missing_items: list[dict[str, Any]] = []
    refactor_without_reduction: list[dict[str, Any]] = []
    for index, item in enumerate(items):
        if not isinstance(item, dict):
            missing_items.append({"index": index, "id": "", "missing_fields": ["item"], "reason": "item 不是 JSON object"})
            continue
        missing_fields: list[str] = []
        decision = str(item.get("decision", "")).strip()
        if decision not in RUST_NATIVE_REFACTOR_ALLOWED_DECISIONS:
            missing_fields.append("decision")
        if not str(item.get("reason", "")).strip():
            missing_fields.append("reason")
        if not str(item.get("result", "")).strip():
            missing_fields.append("result")
        if decision in UNSAFE_REDUCTION_DECISIONS and bool(item.get("unsafe_reduction_required", False)) and not _item_has_unsafe_reduction(item):
            missing_fields.append("unsafe_reduction_evidence")
            refactor_without_reduction.append(
                {
                    "index": index,
                    "id": str(item.get("id", "")).strip(),
                    "file": item.get("file", ""),
                    "decision": decision,
                    "baseline_unsafe_metrics": item.get("baseline_unsafe_metrics", {}),
                    "current_unsafe_metrics": item.get("current_unsafe_metrics", {}),
                    "reason": "decision claims refactor/optimization but unsafe metrics did not decrease versus phase baseline",
                }
            )
        if missing_fields:
            missing_items.append(
                {
                    "index": index,
                    "id": str(item.get("id", "")).strip(),
                    "file": item.get("file", ""),
                    "scope_count": item.get("scope_count", 0),
                    "unsafe_total_lines": item.get("unsafe_total_lines", 0),
                    "abi_candidate_count": len(item.get("abi_candidates", [])) if isinstance(item.get("abi_candidates"), list) else 0,
                    "dead_symbol_candidate_count": len(item.get("dead_symbol_candidates", [])) if isinstance(item.get("dead_symbol_candidates"), list) else 0,
                    "baseline_unsafe_metrics": item.get("baseline_unsafe_metrics", {}),
                    "current_unsafe_metrics": item.get("current_unsafe_metrics", {}),
                    "current_decision": decision,
                    "missing_fields": missing_fields,
                }
            )
    status["missing_items"] = missing_items
    status["missing_count"] = len(missing_items)
    status["refactor_without_reduction"] = refactor_without_reduction
    status["unsafe_reduction_goal"] = payload.get("unsafe_reduction_goal", {})
    status["ok"] = not status["diagnostics"] and not missing_items
    return status


def rust_native_refactor_satisfied(review_json: Path, crate_dir: Path | None = None) -> bool:
    """检查 Rust-native 重构审查 JSON 是否已逐项填写。"""
    return bool(rust_native_refactor_status(review_json, crate_dir).get("ok"))


def _parse_args() -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description="Generate a mandatory Rust-native refactor review task for a Rust crate.")
    parser.add_argument("--crate-dir", required=True)
    parser.add_argument("--source-root", action="append", default=[])
    parser.add_argument("--unsafe-json", required=True)
    parser.add_argument("--unsafe-md", required=True)
    parser.add_argument("--abi-json", required=True)
    parser.add_argument("--abi-md", required=True)
    parser.add_argument("--review-json", required=True)
    parser.add_argument("--suite", default="ohos", choices=["ohos", "oss"])
    return parser.parse_args()


def main() -> int:
    """命令行入口。"""
    args = _parse_args()
    payload = write_rust_native_refactor_task(
        Path(args.crate_dir),
        Path(args.unsafe_json),
        Path(args.unsafe_md),
        Path(args.abi_json),
        Path(args.abi_md),
        Path(args.review_json),
        source_roots=[Path(item) for item in args.source_root],
        suite=args.suite,
    )
    print(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
