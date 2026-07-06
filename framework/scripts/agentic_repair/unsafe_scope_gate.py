#!/usr/bin/env python3
"""生成 Rust crate 的 unsafe 作用域索引，供后置修复 agent 最后审计。"""

from __future__ import annotations

import argparse
import bisect
import hashlib
import json
import re
from pathlib import Path
from typing import Any

UNSAFE_SCOPE_GATE = "unsafe_scope"
IGNORED_DIRS = {".git", ".cargo-home", "__pycache__", "target"}
IGNORED_GENERATED_DIRS = {".c2r_bindgen_extern", ".c2r_bindgen_fns", "__c2r_generated"}
REVIEW_TOP_SCOPE_LIMIT = 12
STRUCTURAL_UNSAFE_LINE_RE = re.compile(r"^(?:unsafe\s*)?[{}\[\](),;]+$")
RUST_FN_PATTERN = re.compile(
    r"\b(?:pub(?:\s*\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?"
    r"(?:extern\s+(?:\"[^\"]*\"\s+)?)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b"
)


def _write_json(path: Path, payload: Any) -> None:
    """写入稳定 JSON。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def _mask_span(chars: list[str], start: int, end: int) -> None:
    """屏蔽源码片段但保留换行和字符偏移。"""
    for index in range(start, min(end, len(chars))):
        if chars[index] != "\n":
            chars[index] = " "


def _raw_string_end(text: str, start: int) -> int | None:
    """识别 Rust raw string 结束位置。"""
    index = start
    if text.startswith("br", start):
        index += 2
    elif text.startswith("r", start):
        index += 1
    else:
        return None

    hashes = 0
    while index < len(text) and text[index] == "#":
        hashes += 1
        index += 1
    if index >= len(text) or text[index] != '"':
        return None
    terminator = '"' + ("#" * hashes)
    end = text.find(terminator, index + 1)
    if end < 0:
        return len(text)
    return end + len(terminator)


def _quoted_string_end(text: str, start: int) -> int:
    """识别普通 Rust 字符串结束位置。"""
    index = start + 1
    while index < len(text):
        if text[index] == "\\":
            index += 2
            continue
        if text[index] == '"':
            return index + 1
        index += 1
    return len(text)


def _mask_comments_and_strings(text: str) -> str:
    """去除注释和字符串对 unsafe/brace 识别的干扰。"""
    chars = list(text)
    index = 0
    while index < len(text):
        raw_end = _raw_string_end(text, index)
        if raw_end is not None:
            _mask_span(chars, index, raw_end)
            index = raw_end
            continue
        if text.startswith('b"', index):
            end = _quoted_string_end(text, index + 1)
            _mask_span(chars, index, end)
            index = end
            continue
        if text[index] == '"':
            end = _quoted_string_end(text, index)
            _mask_span(chars, index, end)
            index = end
            continue
        if text.startswith("//", index):
            end = text.find("\n", index)
            end = len(text) if end < 0 else end
            _mask_span(chars, index, end)
            index = end
            continue
        if text.startswith("/*", index):
            depth = 1
            cursor = index + 2
            while cursor < len(text) and depth:
                if text.startswith("/*", cursor):
                    depth += 1
                    cursor += 2
                elif text.startswith("*/", cursor):
                    depth -= 1
                    cursor += 2
                else:
                    cursor += 1
            _mask_span(chars, index, cursor)
            index = cursor
            continue
        index += 1
    return "".join(chars)


def _line_starts(text: str) -> list[int]:
    """生成行首偏移表。"""
    starts = [0]
    for match in re.finditer("\n", text):
        starts.append(match.end())
    return starts


def _line_no(starts: list[int], offset: int) -> int:
    """把字符偏移转换成 1-based 行号。"""
    return bisect.bisect_right(starts, offset)


def _find_matching_brace(masked: str, open_brace: int) -> int:
    """在已屏蔽源码中查找匹配右花括号。"""
    depth = 0
    for index in range(open_brace, len(masked)):
        char = masked[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
    return open_brace


def _next_body_brace(masked: str, start: int) -> int | None:
    """查找 unsafe item 的函数体或块体左花括号。"""
    brace = masked.find("{", start)
    semi = masked.find(";", start)
    if brace < 0:
        return None
    if semi >= 0 and semi < brace:
        return None
    return brace


def _nearest_function(masked_prefix: str) -> str:
    """查找 unsafe 作用域前最近的函数名。"""
    name = ""
    for match in RUST_FN_PATTERN.finditer(masked_prefix):
        name = match.group(1)
    return name


def _iter_rust_files(crate_dir: Path) -> list[Path]:
    """列出需要扫描的 Rust 源文件。"""
    files: list[Path] = []
    for path in sorted(crate_dir.rglob("*.rs")):
        rel = path.relative_to(crate_dir)
        if any(part in IGNORED_DIRS or part in IGNORED_GENERATED_DIRS for part in rel.parts):
            continue
        files.append(path)
    return files


def _rust_source_fingerprint(crate_dir: Path) -> str:
    """生成当前 Rust 源文件指纹，用于确认 unsafe review 没有过期。"""
    crate_dir = crate_dir.expanduser().resolve()
    entries: list[dict[str, str]] = []
    for path in _iter_rust_files(crate_dir):
        rel = path.relative_to(crate_dir).as_posix()
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        entries.append({"path": rel, "sha256": digest})
    raw = json.dumps(entries, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(raw).hexdigest()


def _count_code_lines(masked: str) -> int:
    """统计去注释/字符串后的非空 Rust 代码行。"""
    return sum(1 for line in masked.splitlines() if line.strip())


def _count_total_lines(source: str) -> int:
    """统计 Rust 文件物理总行数。"""
    return len(source.splitlines())


def _is_metric_code_line(masked_line: str) -> bool:
    """判断一行是否能进入 unsafe 率分子。"""
    stripped = masked_line.strip()
    if not stripped:
        return False
    if STRUCTURAL_UNSAFE_LINE_RE.fullmatch(stripped):
        return False
    return True


def _evidence_counts(masked_segment: str) -> dict[str, int]:
    """给 unsafe scope 生成轻量证据计数，供人工/agent 定位。"""
    patterns = {
        "raw_pointer_type": r"\*\s*(?:const|mut)\b",
        "raw_pointer_deref": r"(?<![\w:])\*\s*(?:[A-Za-z_][A-Za-z0-9_]*|\()",
        "ffi_or_libc": r"\b(?:crate::compat::|libc::|extern\b|CStr::|CString::)",
        "mutable_global": r"\bstatic\s+mut\b|addr_of_mut!|\bGLOBAL\b",
        "layout_or_raw_slice": r"\b(?:transmute|zeroed|MaybeUninit|from_raw_parts|from_ptr|cast::<)\b",
        "callback": r"\b(?:callback|Callback|fn_ptr)\b",
    }
    return {name: len(re.findall(pattern, masked_segment)) for name, pattern in patterns.items()}


def _compact_evidence(counts: dict[str, int]) -> str:
    """把证据计数压缩成 Markdown 表格字段。"""
    parts = [f"{key}={value}" for key, value in counts.items() if value]
    return ", ".join(parts) if parts else "-"


def _escape_md(value: Any) -> str:
    """转义 Markdown 表格单元格。"""
    text = str(value)
    return text.replace("|", "\\|").replace("\n", " ")


def _scope_candidates(masked: str) -> list[tuple[int, str, int]]:
    """提取 unsafe 关键字和对应块体左花括号。"""
    candidates: list[tuple[int, str, int]] = []
    item_patterns = [
        (
            "unsafe_fn",
            re.compile(
                r"(?:^|(?<=[\n;{}]))\s*"
                r"(?:pub(?:\s*\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?"
                r"unsafe\s+(?:extern\s+\"[^\"]*\"\s+)?fn\s+[A-Za-z_][A-Za-z0-9_]*\b"
            ),
        ),
        ("unsafe_extern", re.compile(r"\bunsafe\s+extern\s*(?:\"[^\"]*\"\s*)?\{")),
        ("unsafe_impl", re.compile(r"\bunsafe\s+impl\b")),
        ("unsafe_trait", re.compile(r"\bunsafe\s+trait\b")),
    ]
    for kind, pattern in item_patterns:
        for match in pattern.finditer(masked):
            if kind == "unsafe_extern":
                brace = match.end() - 1
            else:
                brace = _next_body_brace(masked, match.end())
            if brace is not None:
                candidates.append((match.start(), kind, brace))
    for match in re.finditer(r"\bunsafe\s*\{", masked):
        candidates.append((match.start(), "unsafe_block", match.end() - 1))
    candidates.sort(key=lambda item: (item[0], item[2], item[1]))
    return candidates


def _top_file_scopes(scopes: list[dict[str, Any]], file_name: str, limit: int = REVIEW_TOP_SCOPE_LIMIT) -> list[dict[str, Any]]:
    """为文件级 review 生成最大 unsafe scope 索引。"""
    file_scopes = [scope for scope in scopes if scope.get("file") == file_name]
    file_scopes.sort(key=lambda item: (int(item.get("scope_lines") or 0), str(item.get("id") or "")), reverse=True)
    top_scopes: list[dict[str, Any]] = []
    for scope in file_scopes[:limit]:
        top_scopes.append(
            {
                "id": scope.get("id", ""),
                "kind": scope.get("kind", ""),
                "function": scope.get("function", ""),
                "start_line": scope.get("start_line", 0),
                "end_line": scope.get("end_line", 0),
                "scope_lines": scope.get("scope_lines", 0),
                "evidence_counts": scope.get("evidence_counts", {}) if isinstance(scope.get("evidence_counts"), dict) else {},
            }
        )
    return top_scopes


def analyze_unsafe_scopes(crate_dir: Path) -> dict[str, Any]:
    """分析 Rust crate 中所有 unsafe 作用域并返回结构化结果。"""
    crate_dir = crate_dir.expanduser().resolve()
    if not crate_dir.is_dir():
        raise FileNotFoundError(f"crate directory not found: {crate_dir}")

    scopes: list[dict[str, Any]] = []
    files_summary: list[dict[str, Any]] = []
    total_lines_total = 0
    code_lines_total = 0
    unsafe_context_lines_global: set[tuple[str, int]] = set()
    unsafe_keyword_lines_global: set[tuple[str, int]] = set()

    for rust_file in _iter_rust_files(crate_dir):
        rel = rust_file.relative_to(crate_dir).as_posix()
        source = rust_file.read_text(encoding="utf-8", errors="replace")
        masked = _mask_comments_and_strings(source)
        masked_lines = masked.splitlines()
        starts = _line_starts(masked)
        file_total_lines = _count_total_lines(source)
        code_lines = _count_code_lines(masked)
        total_lines_total += file_total_lines
        code_lines_total += code_lines

        file_context_lines: set[int] = set()
        file_keyword_lines = {
            _line_no(starts, match.start())
            for match in re.finditer(r"\bunsafe\b", masked)
            if _is_metric_code_line(masked_lines[_line_no(starts, match.start()) - 1])
        }
        for line in file_keyword_lines:
            unsafe_keyword_lines_global.add((rel, line))

        for start, kind, open_brace in _scope_candidates(masked):
            end = _find_matching_brace(masked, open_brace)
            start_line = _line_no(starts, start)
            end_line = _line_no(starts, end)
            if end_line < start_line:
                end_line = start_line
            scope_line_numbers = set(range(start_line, end_line + 1))
            metric_scope_lines = {
                line for line in scope_line_numbers
                if 1 <= line <= len(masked_lines) and _is_metric_code_line(masked_lines[line - 1])
            }
            file_context_lines.update(metric_scope_lines)
            for line in metric_scope_lines:
                unsafe_context_lines_global.add((rel, line))
            keyword_lines = sorted(line for line in file_keyword_lines if start_line <= line <= end_line)
            segment = masked[start : end + 1]
            counts = _evidence_counts(segment)
            scopes.append(
                {
                    "id": f"U{len(scopes) + 1:04d}",
                    "kind": kind,
                    "file": rel,
                    "abs_file": str(rust_file.resolve()),
                    "function": _nearest_function(masked[:start]),
                    "start_line": start_line,
                    "end_line": end_line,
                    "scope_lines": len(scope_line_numbers),
                    "unsafe_keyword_lines": keyword_lines,
                    "evidence_counts": counts,
                }
            )

        if file_context_lines or file_keyword_lines:
            file_unsafe_total_lines = file_context_lines | file_keyword_lines
            files_summary.append(
                {
                    "file": rel,
                    "abs_file": str(rust_file.resolve()),
                    "scope_count": sum(1 for scope in scopes if scope["file"] == rel),
                    "total_lines": file_total_lines,
                    "code_lines": code_lines,
                    "unsafe_context_lines": len(file_context_lines),
                    "unsafe_keyword_lines": len(file_keyword_lines),
                    "unsafe_total_lines": len(file_unsafe_total_lines),
                    "unsafe_total_ratio": round(len(file_unsafe_total_lines) / file_total_lines, 6) if file_total_lines else 0.0,
                }
            )

    unsafe_total_lines = unsafe_context_lines_global | unsafe_keyword_lines_global
    summary = {
        "scope_count": len(scopes),
        "file_count": len(_iter_rust_files(crate_dir)),
        "files_with_unsafe_count": len(files_summary),
        "total_lines": total_lines_total,
        "code_lines": code_lines_total,
        "unsafe_context_lines": len(unsafe_context_lines_global),
        "unsafe_keyword_lines": len(unsafe_keyword_lines_global),
        "unsafe_total_lines": len(unsafe_total_lines),
        "unsafe_total_ratio": round(len(unsafe_total_lines) / total_lines_total, 6) if total_lines_total else 0.0,
    }
    return {
        "schema_version": "c2r_unsafe_scope_gate_v2",
        "gate": UNSAFE_SCOPE_GATE,
        "mode": "informational",
        "status": "available",
        "passed": True,
        "crate_dir": str(crate_dir),
        "excluded_generated_dirs": sorted(IGNORED_GENERATED_DIRS),
        "summary": summary,
        "by_file": files_summary,
        "scopes": scopes,
    }


def render_markdown(report: dict[str, Any]) -> str:
    """把 unsafe scope 报告渲染为给 agent 读取的精简 Markdown。"""
    summary = report.get("summary") if isinstance(report.get("summary"), dict) else {}
    lines = [
        "# Unsafe Scope Gate",
        "",
        "mode: informational; this gate does not block compile/semantic acceptance by itself.",
        "",
        "## Summary",
        f"- crate_dir: {report.get('crate_dir', '')}",
        f"- scope_count: {summary.get('scope_count', 0)}",
        f"- files_with_unsafe_count: {summary.get('files_with_unsafe_count', 0)} / {summary.get('file_count', 0)}",
        f"- unsafe_total_lines: {summary.get('unsafe_total_lines', 0)} / {summary.get('total_lines', 0)}",
        f"- unsafe_total_ratio: {summary.get('unsafe_total_ratio', 0)}",
        f"- excluded_generated_dirs: {', '.join(report.get('excluded_generated_dirs', [])) if isinstance(report.get('excluded_generated_dirs'), list) else ''}",
        "",
        "## Files",
        "| file | scopes | unsafe_total_lines | total_lines | ratio | abs_file |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for item in report.get("by_file", []) if isinstance(report.get("by_file"), list) else []:
        if not isinstance(item, dict):
            continue
        lines.append(
            "| "
            + " | ".join(
                [
                    _escape_md(item.get("file", "")),
                    str(item.get("scope_count", 0)),
                    str(item.get("unsafe_total_lines", 0)),
                    str(item.get("total_lines", 0)),
                    str(item.get("unsafe_total_ratio", 0)),
                    _escape_md(item.get("abs_file", "")),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Unsafe Scope Index",
            "| id | kind | location | function | lines | evidence | abs_file |",
            "|---|---|---|---|---:|---|---|",
        ]
    )
    for item in report.get("scopes", []) if isinstance(report.get("scopes"), list) else []:
        if not isinstance(item, dict):
            continue
        location = f"{item.get('file', '')}:{item.get('start_line', '')}-{item.get('end_line', '')}"
        lines.append(
            "| "
            + " | ".join(
                [
                    _escape_md(item.get("id", "")),
                    _escape_md(item.get("kind", "")),
                    _escape_md(location),
                    _escape_md(item.get("function", "") or "-"),
                    str(item.get("scope_lines", 0)),
                    _escape_md(_compact_evidence(item.get("evidence_counts", {}) if isinstance(item.get("evidence_counts"), dict) else {})),
                    _escape_md(item.get("abs_file", "")),
                ]
            )
            + " |"
        )
    lines.append("")
    return "\n".join(lines)


def run_unsafe_scope_gate(crate_dir: Path, output_json: Path, output_md: Path) -> dict[str, Any]:
    """运行 unsafe scope gate 并写出 JSON/Markdown。"""
    report = analyze_unsafe_scopes(crate_dir)
    output_json = output_json.expanduser().resolve()
    output_md = output_md.expanduser().resolve()
    _write_json(output_json, report)
    output_md.parent.mkdir(parents=True, exist_ok=True)
    output_md.write_text(render_markdown(report), encoding="utf-8")
    payload = {
        "gate": UNSAFE_SCOPE_GATE,
        "mode": "informational",
        "status": "available",
        "passed": True,
        "returncode": 0,
        "json_path": str(output_json),
        "markdown_path": str(output_md),
        "summary": report.get("summary", {}),
    }
    return payload


def write_unsafe_review_task(crate_dir: Path, output_json: Path, output_md: Path, review_json: Path) -> dict[str, Any]:
    """重新扫描 crate 并写出当前轮强制 unsafe 审查任务 JSON。"""
    payload = run_unsafe_scope_gate(crate_dir, output_json, output_md)
    report = json.loads(Path(output_json).read_text(encoding="utf-8"))
    items: list[dict[str, Any]] = []
    scopes = report.get("scopes", []) if isinstance(report.get("scopes"), list) else []
    for file_item in report.get("by_file", []) if isinstance(report.get("by_file"), list) else []:
        if not isinstance(file_item, dict):
            continue
        items.append(
            {
                "id": f"F{len(items) + 1:04d}",
                "kind": "unsafe_file_review",
                "file": file_item.get("file", ""),
                "abs_file": file_item.get("abs_file", ""),
                "scope_count": file_item.get("scope_count", 0),
                "total_lines": file_item.get("total_lines", 0),
                "code_lines": file_item.get("code_lines", 0),
                "unsafe_context_lines": file_item.get("unsafe_context_lines", 0),
                "unsafe_keyword_lines": file_item.get("unsafe_keyword_lines", 0),
                "unsafe_total_lines": file_item.get("unsafe_total_lines", 0),
                "unsafe_total_ratio": file_item.get("unsafe_total_ratio", 0),
                "top_scopes": _top_file_scopes(scopes, str(file_item.get("file", ""))),
                "decision": "",
                "reason": "",
                "result": "",
            }
        )
    review_payload = {
        "schema_version": "c2r_unsafe_review_task_v1",
        "mode": "required_after_semantic_acceptance",
        "project_root": str(Path(crate_dir).expanduser().resolve()),
        "source_fingerprint_sha256": _rust_source_fingerprint(Path(crate_dir)),
        "scope_json_path": str(Path(output_json).expanduser().resolve()),
        "scope_markdown_path": str(Path(output_md).expanduser().resolve()),
        "summary": report.get("summary") if isinstance(report.get("summary"), dict) else {},
        "excluded_generated_dirs": sorted(IGNORED_GENERATED_DIRS),
        "allowed_decisions": ["optimized", "kept_required", "kept_risky"],
        "instructions": [
            "逐个文件审查 items；每一项必须填写 decision、reason、result。",
            "每个 item 是一个可编辑 Rust 文件级 unsafe 审查任务；打开 abs_file 后整体移动不需要 unsafe 的控制流、局部变量和普通计算。",
            "能在保持语义、ABI、编译、导出符号、extern 调用约定、结构布局和测试可观察行为不变的前提下降低 unsafe，就必须优化。",
            "纯 bindgen 生成 FFI/TU 目录不纳入统计和本审查任务。",
            "不能用删除业务逻辑、默认返回、改变 ABI 或改变外部调用约定来降低 unsafe。",
        ],
        "items": items,
    }
    _write_json(Path(review_json), review_payload)
    return {
        "review_json_path": str(Path(review_json).expanduser().resolve()),
        "scope_json_path": str(Path(output_json).expanduser().resolve()),
        "scope_markdown_path": str(Path(output_md).expanduser().resolve()),
        "summary": review_payload["summary"],
        "item_count": len(items),
        "scope_payload": payload,
    }


def unsafe_review_status(review_json: Path, crate_dir: Path | None = None) -> dict[str, Any]:
    """返回 unsafe review JSON 的完成状态和缺漏明细。"""
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
            status["diagnostics"].append("review JSON 源码指纹与当前 crate 不匹配，必须重新生成 unsafe review")
    allowed = {"optimized", "kept_required", "kept_risky"}
    missing_items: list[dict[str, Any]] = []
    for index, item in enumerate(items):
        if not isinstance(item, dict):
            missing_items.append(
                {
                    "index": index,
                    "id": "",
                    "missing_fields": ["item"],
                    "reason": "item 不是 JSON object",
                }
            )
            continue
        missing_fields: list[str] = []
        decision = str(item.get("decision", "")).strip()
        if decision not in allowed:
            missing_fields.append("decision")
        if not str(item.get("reason", "")).strip():
            missing_fields.append("reason")
        if not str(item.get("result", "")).strip():
            missing_fields.append("result")
        if missing_fields:
            missing_items.append(
                {
                    "index": index,
                    "id": str(item.get("id", "")).strip(),
                    "file": item.get("file", ""),
                    "function": item.get("function", ""),
                    "start_line": item.get("start_line", 0),
                    "end_line": item.get("end_line", 0),
                    "scope_count": item.get("scope_count", 0),
                    "unsafe_total_lines": item.get("unsafe_total_lines", 0),
                    "current_decision": decision,
                    "missing_fields": missing_fields,
                }
            )
    status["missing_items"] = missing_items
    status["missing_count"] = len(missing_items)
    status["ok"] = not status["diagnostics"] and not missing_items
    return status


def unsafe_review_satisfied(review_json: Path, crate_dir: Path | None = None) -> bool:
    """检查 unsafe review JSON 是否已逐项填写。"""
    return bool(unsafe_review_status(review_json, crate_dir).get("ok"))


def _resolve_crate_dir(args: argparse.Namespace) -> Path:
    """按显式 crate-dir 或 run-dir/project/llm-name 推导 crate 路径。"""
    if args.crate_dir:
        return Path(args.crate_dir).expanduser().resolve()
    if args.run_dir and args.project and args.llm_name:
        run_dir = Path(args.run_dir).expanduser().resolve()
        return (
            run_dir
            / "raw"
            / "framework_output"
            / "intermediate"
            / args.project
            / "workspace"
            / "final_projects"
            / args.project
            / f"translate_by_{args.llm_name}"
        ).resolve()
    raise SystemExit("--crate-dir is required unless --run-dir, --project and --llm-name are provided")


def _parse_args() -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description="Generate an informational unsafe scope gate for a Rust crate.")
    parser.add_argument("--crate-dir", default="")
    parser.add_argument("--run-dir", default="")
    parser.add_argument("--project", default="")
    parser.add_argument("--llm-name", default="qwen3_coder")
    parser.add_argument("--output-json", required=True)
    parser.add_argument("--output-md", required=True)
    return parser.parse_args()


def main() -> int:
    """命令行入口。"""
    args = _parse_args()
    payload = run_unsafe_scope_gate(_resolve_crate_dir(args), Path(args.output_json), Path(args.output_md))
    print(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
