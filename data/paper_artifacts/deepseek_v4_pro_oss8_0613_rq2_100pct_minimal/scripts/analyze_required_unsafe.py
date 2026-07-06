#!/usr/bin/env python3
"""统计论文实验翻译产物中证据充分的必须 unsafe 代码行。"""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Sequence, Set, Tuple


DEFAULT_OHOS_C_PROJECTS = [
    "host__25c1898e1626",
    "appverify_lite__e5ebe91a98b9",
    "manager__c248934e0221",
    "shared__541f4e547bdb",
    "posix__1b7f59c68bbc",
    "common__89d5ecaafdff",
    "core__ef5242b7ab08",
    "shared__12e38ea922f7",
    "osal__0bc4f21396ad",
    "sapm__193cdeb43a97",
]

SKIP_DIRS = {"target", ".git", "__pycache__"}
FFI_MARKERS_RE = re.compile(
    r"extern\s*\"C\"|\*mut\b|\*const\b|Option\s*<|fn\s*\(|crate::types|::types::|libc::"
)
CONTRACT_REASONS = {"unsafe_abi_contract", "unsafe_extern_block_contract", "unsafe_fn_contract", "unsafe_impl_contract"}
STRUCTURAL_UNSAFE_LINE_RE = re.compile(r"^(?:unsafe\s*)?[{}\[\](),;]+$")


@dataclass(frozen=True)
class SourceRange:
    """表示一个 Rust 源码行区间。"""

    start_line: int
    end_line: int

    def contains(self, line_no: int) -> bool:
        """判断行号是否落在区间内。"""
        return self.start_line <= line_no <= self.end_line


@dataclass
class Finding:
    """记录一行被判定为必须 unsafe 的证据。"""

    file: str
    line: int
    reasons: Set[str]
    text: str

    def to_dict(self) -> Dict[str, Any]:
        """转换为稳定 JSON 结构。"""
        return {
            "file": self.file,
            "line": self.line,
            "reasons": sorted(self.reasons),
            "text": self.text.strip(),
        }


def _iter_rust_files(crate_dir: Path) -> List[Path]:
    """枚举 crate 内参与统计的 Rust 源文件。"""
    files: List[Path] = []
    for path in crate_dir.rglob("*.rs"):
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        files.append(path)
    return sorted(files)


def _mask_comments_and_strings(text: str) -> str:
    """屏蔽注释和字符串内容，保留换行和大致列宽用于括号匹配。"""
    out: List[str] = []
    i = 0
    state = "normal"
    block_depth = 0
    quote = ""
    while i < len(text):
        ch = text[i]
        nxt = text[i + 1] if i + 1 < len(text) else ""

        if state == "line_comment":
            out.append("\n" if ch == "\n" else " ")
            if ch == "\n":
                state = "normal"
            i += 1
            continue

        if state == "block_comment":
            if ch == "/" and nxt == "*":
                block_depth += 1
                out.extend("  ")
                i += 2
                continue
            if ch == "*" and nxt == "/":
                block_depth -= 1
                out.extend("  ")
                i += 2
                if block_depth <= 0:
                    state = "normal"
                continue
            out.append("\n" if ch == "\n" else " ")
            i += 1
            continue

        if state == "string":
            if ch == "\\" and quote != "`":
                out.extend("  " if i + 1 < len(text) else " ")
                i += 2
                continue
            out.append("\n" if ch == "\n" else " ")
            if ch == quote:
                state = "normal"
            i += 1
            continue

        if ch == "/" and nxt == "/":
            state = "line_comment"
            out.extend("  ")
            i += 2
            continue
        if ch == "/" and nxt == "*":
            state = "block_comment"
            block_depth = 1
            out.extend("  ")
            i += 2
            continue
        if ch in {'"', "'"}:
            # 生命周期标记如 'a 不按字符串处理。
            if ch == "'" and i + 1 < len(text) and re.match(r"[A-Za-z_]", text[i + 1]):
                out.append(ch)
                i += 1
                continue
            state = "string"
            quote = ch
            out.append(" ")
            i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def _strip_comments_keep_strings(text: str) -> str:
    """只移除注释，保留字符串，便于识别 extern \"C\" 等语法。"""
    out: List[str] = []
    i = 0
    state = "normal"
    block_depth = 0
    quote = ""
    while i < len(text):
        ch = text[i]
        nxt = text[i + 1] if i + 1 < len(text) else ""

        if state == "line_comment":
            out.append("\n" if ch == "\n" else " ")
            if ch == "\n":
                state = "normal"
            i += 1
            continue
        if state == "block_comment":
            if ch == "/" and nxt == "*":
                block_depth += 1
                out.extend("  ")
                i += 2
                continue
            if ch == "*" and nxt == "/":
                block_depth -= 1
                out.extend("  ")
                i += 2
                if block_depth <= 0:
                    state = "normal"
                continue
            out.append("\n" if ch == "\n" else " ")
            i += 1
            continue
        if state == "string":
            out.append(ch)
            if ch == "\\":
                if i + 1 < len(text):
                    out.append(text[i + 1])
                i += 2
                continue
            if ch == quote:
                state = "normal"
            i += 1
            continue

        if ch == "/" and nxt == "/":
            state = "line_comment"
            out.extend("  ")
            i += 2
            continue
        if ch == "/" and nxt == "*":
            state = "block_comment"
            block_depth = 1
            out.extend("  ")
            i += 2
            continue
        if ch in {'"', "'"}:
            if ch == "'" and i + 1 < len(text) and re.match(r"[A-Za-z_]", text[i + 1]):
                out.append(ch)
                i += 1
                continue
            state = "string"
            quote = ch
        out.append(ch)
        i += 1
    return "".join(out)


def _line_starts(text: str) -> List[int]:
    """生成每行起始偏移表。"""
    starts = [0]
    for match in re.finditer("\n", text):
        starts.append(match.end())
    return starts


def _offset_to_line(starts: Sequence[int], offset: int) -> int:
    """把字符偏移转换为 1-based 行号。"""
    lo, hi = 0, len(starts)
    while lo + 1 < hi:
        mid = (lo + hi) // 2
        if starts[mid] <= offset:
            lo = mid
        else:
            hi = mid
    return lo + 1


def _match_brace(masked: str, open_pos: int) -> Optional[int]:
    """从左花括号位置匹配对应右花括号。"""
    depth = 0
    for pos in range(open_pos, len(masked)):
        ch = masked[pos]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return pos
    return None


def _find_braced_ranges(text_for_regex: str, masked: str, pattern: re.Pattern[str]) -> List[SourceRange]:
    """查找 pattern 后最近花括号包裹的源码行区间。"""
    starts = _line_starts(masked)
    ranges: List[SourceRange] = []
    for match in pattern.finditer(text_for_regex):
        open_pos = masked.find("{", match.end() - 1)
        if open_pos < 0:
            continue
        semi_pos = masked.find(";", match.end() - 1)
        if semi_pos >= 0 and semi_pos < open_pos:
            continue
        close_pos = _match_brace(masked, open_pos)
        if close_pos is None:
            continue
        ranges.append(SourceRange(_offset_to_line(starts, open_pos), _offset_to_line(starts, close_pos)))
    return ranges


def _collect_unsafe_block_ranges(text: str) -> List[SourceRange]:
    """收集 unsafe block 的行区间。"""
    commentless = _strip_comments_keep_strings(text)
    masked = _mask_comments_and_strings(text)
    return _find_braced_ranges(commentless, masked, re.compile(r"\bunsafe\s*\{"))


def _collect_unsafe_fn_ranges(text: str) -> List[SourceRange]:
    """收集 unsafe fn body 的行区间。"""
    commentless = _strip_comments_keep_strings(text)
    masked = _mask_comments_and_strings(text)
    return _find_braced_ranges(
        commentless,
        masked,
        re.compile(
            r"(?:^|(?<=[\n;{}]))\s*"
            r"(?:pub(?:\s*\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?"
            r"unsafe\s+(?:extern\s*\"[^\"]*\"\s+)?fn\s+[A-Za-z_][A-Za-z0-9_]*\b"
        ),
    )


def _collect_unsafe_extern_block_ranges(text: str) -> List[SourceRange]:
    """收集 unsafe extern block 的行区间。"""
    commentless = _strip_comments_keep_strings(text)
    masked = _mask_comments_and_strings(text)
    return _find_braced_ranges(
        commentless,
        masked,
        re.compile(r"\bunsafe\s+extern\s*(?:\"[^\"]*\"\s*)?\{"),
    )


def _collect_unsafe_ranges(text: str) -> List[SourceRange]:
    """收集 unsafe block、unsafe fn body 和 unsafe extern block 的行区间。"""
    unsafe_blocks = _collect_unsafe_block_ranges(text)
    unsafe_fns = _collect_unsafe_fn_ranges(text)
    unsafe_extern_blocks = _collect_unsafe_extern_block_ranges(text)
    return unsafe_blocks + unsafe_fns + unsafe_extern_blocks


def _collect_extern_fn_names(text: str) -> Set[str]:
    """收集 extern \"C\" 声明块中的函数名。"""
    commentless = _strip_comments_keep_strings(text)
    masked = _mask_comments_and_strings(text)
    names: Set[str] = set()
    for match in re.finditer(r"\bextern\s*\"C\"\s*\{", commentless):
        open_pos = masked.find("{", match.start())
        if open_pos < 0:
            continue
        close_pos = _match_brace(masked, open_pos)
        if close_pos is None:
            continue
        block = commentless[open_pos:close_pos]
        for fn_match in re.finditer(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", block):
            names.add(fn_match.group(1))
    return names


def _collect_static_mut_names(text: str) -> Set[str]:
    """收集 static mut 全局变量名。"""
    commentless = _strip_comments_keep_strings(text)
    return set(re.findall(r"\bstatic\s+mut\s+([A-Za-z_][A-Za-z0-9_]*)\b", commentless))


def _collect_unsafe_fn_names(text: str) -> Set[str]:
    """收集本 crate 中调用时语言层面要求 unsafe 的函数名。"""
    commentless = _strip_comments_keep_strings(text)
    return set(
        re.findall(
            r"\bunsafe\s+(?:extern\s*\"C\"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
            commentless,
        )
    )


def _collect_unsafe_callable_names(text: str) -> Set[str]:
    """收集 unsafe extern C 函数指针变量/字段名。"""
    commentless = _strip_comments_keep_strings(text)
    names: Set[str] = set()
    for match in re.finditer(
        r"\b([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(?:(?:core|std|::core)::option::Option\s*<\s*)?"
        r"unsafe\s+extern\s*\"C\"\s+fn\b",
        commentless,
    ):
        names.add(match.group(1))
    return names


def _count_code_lines(text: str) -> int:
    """统计非空、非注释源码行数。"""
    commentless = _strip_comments_keep_strings(text)
    return sum(1 for line in commentless.splitlines() if line.strip())


def _count_total_lines(text: str) -> int:
    """统计 Rust 文件物理总行数。"""
    return len(text.splitlines())


def _is_metric_code_line(masked_line: str) -> bool:
    """判断一行是否能进入 unsafe 分子。"""
    stripped = masked_line.strip()
    if not stripped:
        return False
    if STRUCTURAL_UNSAFE_LINE_RE.fullmatch(stripped):
        return False
    return True


def _line_in_ranges(line_no: int, ranges: Sequence[SourceRange]) -> bool:
    """判断行号是否处于任意 unsafe 上下文。"""
    return any(rng.contains(line_no) for rng in ranges)


def _opening_ranges_for_line(line_no: int, ranges: Sequence[SourceRange]) -> List[SourceRange]:
    """找出当前行所在、且起始行早于当前行的 unsafe 区间。"""
    return [rng for rng in ranges if rng.start_line < line_no <= rng.end_line]


def _merge_finding(
    required: Dict[Tuple[str, int], Finding],
    rel: str,
    line_no: int,
    reasons: Set[str],
    lines: Sequence[str],
) -> None:
    """把一行 findings 合并进结果集。"""
    if line_no < 1 or line_no > len(lines):
        return
    key = (rel, line_no)
    finding = required.get(key)
    if finding is None:
        finding = Finding(rel, line_no, set(), lines[line_no - 1])
        required[key] = finding
    finding.reasons.update(reasons)


def _build_extern_call_re(extern_names: Set[str]) -> Optional[re.Pattern[str]]:
    """构建 extern C 函数调用检测正则。"""
    if not extern_names:
        return None
    names = "|".join(re.escape(name) for name in sorted(extern_names, key=len, reverse=True))
    return re.compile(
        rf"(?<!fn\s)(?<!::)\b(?:{names})\s*\(|\bcrate::compat::(?:{names})\s*\("
    )


def _build_unsafe_fn_call_re(unsafe_fn_names: Set[str]) -> Optional[re.Pattern[str]]:
    """构建本 crate unsafe fn 调用检测正则。"""
    if not unsafe_fn_names:
        return None
    names = "|".join(re.escape(name) for name in sorted(unsafe_fn_names, key=len, reverse=True))
    return re.compile(rf"(?<!fn\s)\b(?:{names})\s*\(|\bcrate::[A-Za-z_][A-Za-z0-9_:]*::(?:{names})\s*\(")


def _build_unsafe_callable_call_re(unsafe_callable_names: Set[str]) -> Optional[re.Pattern[str]]:
    """构建 unsafe extern C 函数指针调用检测正则。"""
    if not unsafe_callable_names:
        return None
    names = "|".join(re.escape(name) for name in sorted(unsafe_callable_names, key=len, reverse=True))
    return re.compile(rf"(?<!fn\s)\b(?:{names})\s*\(")


def _has_raw_pointer_deref(line: str) -> bool:
    """判断一行是否包含 raw pointer 解引用形态。"""
    if re.search(r"\(\s*\*[^)]+\)\s*(?:\.|\[)", line):
        return True
    if re.search(r"(?<![\w:])\*(?!\s*(?:mut|const)\b)\s*[A-Za-z_][A-Za-z0-9_:.]*", line):
        return True
    return False


def _detect_reasons(
    line: str,
    extern_call_re: Optional[re.Pattern[str]],
    unsafe_fn_call_re: Optional[re.Pattern[str]],
    unsafe_callable_call_re: Optional[re.Pattern[str]],
    static_mut_names: Set[str],
) -> Set[str]:
    """检测一行中的必须 unsafe 证据。"""
    reasons: Set[str] = set()
    stripped = line.strip()
    if not stripped:
        return reasons

    if extern_call_re and extern_call_re.search(line):
        reasons.add("ffi_call")
    if unsafe_fn_call_re and unsafe_fn_call_re.search(line):
        reasons.add("unsafe_fn_call")
    if unsafe_callable_call_re and unsafe_callable_call_re.search(line):
        reasons.add("unsafe_callback_call")
    if re.search(r"\blibc::[A-Za-z_][A-Za-z0-9_]*\s*\(", line):
        reasons.add("ffi_call")
    if _has_raw_pointer_deref(line):
        reasons.add("raw_pointer_deref")
    if re.search(r"\.(?:offset|add|sub)\s*\(", line):
        reasons.add("raw_pointer_arithmetic")
    if re.search(
        r"\b(?:core::|std::)?ptr::(?:read|write|copy|copy_nonoverlapping|read_volatile|write_volatile|"
        r"replace|drop_in_place)\s*\(",
        line,
    ):
        reasons.add("raw_pointer_operation")
    if re.search(r"\b(?:CStr|std::ffi::CStr|core::ffi::CStr)::from_ptr\s*\(", line):
        reasons.add("c_str_from_ptr")
    if re.search(r"\b(?:slice::|std::slice::|core::slice::)?from_raw_parts(?:_mut)?\s*\(", line):
        reasons.add("raw_slice_from_parts")
    if re.search(r"\b(?:Vec|Box|CString|std::ffi::CString)::from_raw\s*\(", line):
        reasons.add("raw_ownership_transfer")
    if ".assume_init" in line:
        reasons.add("ffi_layout_init")
    if re.search(r"\b(?:std::mem::|core::mem::)?transmute(?:\s*::|\s*\()", line):
        if FFI_MARKERS_RE.search(line):
            reasons.add("ffi_layout_transmute")
    if re.search(r"\bunsafe\s+impl\b", line):
        reasons.add("unsafe_impl_contract")
    if re.search(r"\bunsafe\s+extern\b.*\{", line):
        reasons.add("unsafe_extern_block_contract")
    if re.search(r"\bunsafe\s+extern\s*(?:\"[^\"]*\"\s*)?fn\b", line):
        reasons.add("unsafe_abi_contract")
    elif re.search(r"\bunsafe\s+fn\s+[A-Za-z_][A-Za-z0-9_]*\s*\([^)]*(?:\*mut\b|\*const\b|libc::|core::ffi|std::ffi|crate::types)", line):
        reasons.add("unsafe_fn_contract")

    for name in static_mut_names:
        if re.search(rf"\bstatic\s+mut\s+{re.escape(name)}\b", line):
            continue
        if re.search(rf"\b{re.escape(name)}\b", line):
            reasons.add("static_mut_access")
            break
    return reasons


def _expand_statement_lines(lines: Sequence[str], start_index: int, reasons: Set[str]) -> Set[int]:
    """返回包含必须 unsafe 证据的源码行。"""
    return {start_index + 1}


def _project_name_from_crate(crate_dir: Path) -> str:
    """从 crate 路径推断项目名。"""
    if crate_dir.name.startswith("translate_by_") and crate_dir.parent.name:
        return crate_dir.parent.name
    return crate_dir.name


def analyze_crate(crate_dir: Path, project: Optional[str] = None, details_limit: int = 500) -> Dict[str, Any]:
    """分析单个翻译 crate 的必须 unsafe 行。"""
    crate_dir = crate_dir.resolve()
    project_name = project or _project_name_from_crate(crate_dir)
    files = _iter_rust_files(crate_dir)

    extern_names: Set[str] = set()
    static_mut_names: Set[str] = set()
    unsafe_fn_names: Set[str] = set()
    unsafe_callable_names: Set[str] = set()
    file_texts: Dict[Path, str] = {}
    for path in files:
        text = path.read_text(encoding="utf-8", errors="ignore")
        file_texts[path] = text
        extern_names.update(_collect_extern_fn_names(text))
        static_mut_names.update(_collect_static_mut_names(text))
        unsafe_fn_names.update(_collect_unsafe_fn_names(text))
        unsafe_callable_names.update(_collect_unsafe_callable_names(text))

    extern_call_re = _build_extern_call_re(extern_names)
    unsafe_fn_call_re = _build_unsafe_fn_call_re(unsafe_fn_names)
    unsafe_callable_call_re = _build_unsafe_callable_call_re(unsafe_callable_names)
    required: Dict[Tuple[str, int], Finding] = {}
    reason_counts: Counter[str] = Counter()
    total_lines = 0
    code_lines = 0
    raw_unsafe_keyword_lines = 0
    raw_unsafe_lines = 0

    for path in files:
        text = file_texts[path]
        rel = str(path.relative_to(crate_dir))
        total_lines += _count_total_lines(text)
        code_lines += _count_code_lines(text)
        lines = text.splitlines()
        masked_lines = _mask_comments_and_strings(text).splitlines()
        unsafe_ranges = _collect_unsafe_ranges(text)
        raw_keyword_line_numbers: Set[int] = set()
        raw_unsafe_line_numbers: Set[int] = set()
        for idx, masked_line in enumerate(masked_lines):
            line_no = idx + 1
            if not _is_metric_code_line(masked_line):
                continue
            if re.search(r"\bunsafe\b", masked_line):
                raw_keyword_line_numbers.add(line_no)
                raw_unsafe_line_numbers.add(line_no)
            if _line_in_ranges(line_no, unsafe_ranges):
                raw_unsafe_line_numbers.add(line_no)
        raw_unsafe_keyword_lines += len(raw_keyword_line_numbers)
        raw_unsafe_lines += len(raw_unsafe_line_numbers)

        for idx, line in enumerate(lines):
            line_no = idx + 1
            detect_line = masked_lines[idx] if idx < len(masked_lines) else ""
            if not _is_metric_code_line(detect_line):
                continue
            reasons = _detect_reasons(
                detect_line,
                extern_call_re,
                unsafe_fn_call_re,
                unsafe_callable_call_re,
                static_mut_names,
            )
            if not reasons:
                continue
            in_unsafe_context = _line_in_ranges(line_no, unsafe_ranges)
            if not (reasons & CONTRACT_REASONS) and not in_unsafe_context:
                continue
            expanded = _expand_statement_lines(lines, idx, reasons)
            for expanded_line in expanded:
                if expanded_line < 1 or expanded_line > len(lines):
                    continue
                if not (reasons & CONTRACT_REASONS) and not _line_in_ranges(expanded_line, unsafe_ranges):
                    continue
                _merge_finding(required, rel, expanded_line, reasons, lines)

    for finding in required.values():
        for reason in finding.reasons:
            reason_counts[reason] += 1

    details = sorted(required.values(), key=lambda item: (item.file, item.line))
    required_lines = len(required)
    return {
        "project": project_name,
        "crate_dir": str(crate_dir),
        "rust_files": len(files),
        "total_lines": total_lines,
        "code_lines": code_lines,
        "raw_unsafe_keyword_lines": raw_unsafe_keyword_lines,
        "raw_unsafe_lines": raw_unsafe_lines,
        "raw_unsafe_ratio": (raw_unsafe_lines / total_lines) if total_lines else 0.0,
        "required_unsafe_lines": required_lines,
        "required_unsafe_ratio": (required_lines / total_lines) if total_lines else 0.0,
        "required_reasons": dict(sorted(reason_counts.items())),
        "extern_function_count": len(extern_names),
        "static_mut_count": len(static_mut_names),
        "unsafe_fn_count": len(unsafe_fn_names),
        "unsafe_callable_count": len(unsafe_callable_names),
        "findings_sample": [item.to_dict() for item in details[: max(0, details_limit)]],
        "findings_sample_truncated": len(details) > details_limit,
    }


def _resolve_run_dir(run_dir: Path) -> Path:
    """兼容 experiment_runs 下的运行目录和 raw/framework_output 目录。"""
    run_dir = run_dir.resolve()
    if (run_dir / "raw/framework_output/intermediate").is_dir():
        return run_dir / "raw/framework_output"
    if (run_dir / "intermediate").is_dir():
        return run_dir
    raise FileNotFoundError(f"无法识别运行目录: {run_dir}")


def find_project_crate_dir(run_dir: Path, project: str) -> Optional[Path]:
    """在运行目录中定位项目最终 Rust crate。"""
    framework_dir = _resolve_run_dir(run_dir)
    workspace = framework_dir / "intermediate" / project / "workspace"
    final_root = workspace / "final_projects" / project
    if final_root.is_dir():
        candidates = sorted(final_root.glob("translate_by_*"))
        for candidate in candidates:
            if (candidate / "Cargo.toml").exists() and (candidate / "src").is_dir():
                return candidate
    for candidate in workspace.glob("**/translate_by_*"):
        if (candidate / "Cargo.toml").exists() and (candidate / "src").is_dir():
            return candidate
    return None


def _parse_projects(value: Optional[str]) -> List[str]:
    """解析逗号分隔项目列表。"""
    if not value:
        return list(DEFAULT_OHOS_C_PROJECTS)
    return [item.strip() for item in value.split(",") if item.strip()]


def analyze_run(run_dir: Path, projects: Sequence[str], details_limit: int) -> Dict[str, Any]:
    """批量分析运行目录中的多个项目。"""
    results: Dict[str, Any] = {}
    missing: List[str] = []
    totals = Counter()
    reason_totals: Counter[str] = Counter()

    for project in projects:
        crate_dir = find_project_crate_dir(run_dir, project)
        if crate_dir is None:
            missing.append(project)
            results[project] = {"project": project, "error": "crate_not_found"}
            continue
        result = analyze_crate(crate_dir, project=project, details_limit=details_limit)
        results[project] = result
        totals["projects_analyzed"] += 1
        totals["total_lines"] += int(result["total_lines"])
        totals["code_lines"] += int(result["code_lines"])
        totals["raw_unsafe_keyword_lines"] += int(result["raw_unsafe_keyword_lines"])
        totals["raw_unsafe_lines"] += int(result["raw_unsafe_lines"])
        totals["required_unsafe_lines"] += int(result["required_unsafe_lines"])
        reason_totals.update(result.get("required_reasons") or {})

    micro_summary = {
        "projects_requested": len(projects),
        "projects_analyzed": totals["projects_analyzed"],
        "missing_projects": missing,
        "total_lines": totals["total_lines"],
        "code_lines": totals["code_lines"],
        "raw_unsafe_keyword_lines": totals["raw_unsafe_keyword_lines"],
        "raw_unsafe_lines": totals["raw_unsafe_lines"],
        "raw_unsafe_ratio": (
            totals["raw_unsafe_lines"] / totals["total_lines"] if totals["total_lines"] else 0.0
        ),
        "required_unsafe_lines": totals["required_unsafe_lines"],
        "required_unsafe_ratio": (
            totals["required_unsafe_lines"] / totals["total_lines"] if totals["total_lines"] else 0.0
        ),
        "required_reasons": dict(sorted(reason_totals.items())),
    }
    analyzed_results = [item for item in results.values() if not item.get("error")]
    summary = {
        **micro_summary,
        "metric_policy": "paper_macro_project_average",
        "raw_unsafe_ratio": _mean([float(item.get("raw_unsafe_ratio") or 0.0) for item in analyzed_results]),
        "required_unsafe_ratio": _mean(
            [float(item.get("required_unsafe_ratio") or 0.0) for item in analyzed_results]
        ),
        "micro_evidence": micro_summary,
    }
    return {"run_dir": str(run_dir.resolve()), "projects": results, "summary": summary}


def _format_percent(value: float) -> str:
    """格式化百分比。"""
    return f"{value * 100:.2f}%"


def _mean(values: Sequence[float]) -> float:
    """计算项目级宏平均。"""
    return (sum(values) / len(values)) if values else 0.0


def render_markdown(report: Dict[str, Any]) -> str:
    """生成论文实验可读 Markdown 表格。"""
    lines = [
        "| 项目 | total lines | raw unsafe lines | raw unsafe rate | required unsafe lines | required unsafe rate | top reasons |",
        "|---|---:|---:|---:|---:|---:|---|",
    ]
    projects = report.get("projects", {})
    for name, result in projects.items():
        if result.get("error"):
            lines.append(f"| `{name}` | - | - | - | - | - | {result['error']} |")
            continue
        reasons = result.get("required_reasons") or {}
        top = ", ".join(f"{k}:{v}" for k, v in sorted(reasons.items(), key=lambda kv: -kv[1])[:3])
        lines.append(
            f"| `{name}` | {result['total_lines']} | {result['raw_unsafe_lines']} | "
            f"{_format_percent(result['raw_unsafe_ratio'])} | {result['required_unsafe_lines']} | "
            f"{_format_percent(result['required_unsafe_ratio'])} | {top} |"
        )
    summary = report.get("summary") or {}
    if summary:
        lines.append(
            f"| **Paper macro average** | {summary.get('total_lines', 0)} | {summary.get('raw_unsafe_lines', 0)} | "
            f"{_format_percent(float(summary.get('raw_unsafe_ratio') or 0.0))} | "
            f"{summary.get('required_unsafe_lines', 0)} | "
            f"{_format_percent(float(summary.get('required_unsafe_ratio') or 0.0))} | - |"
        )
        lines.append("")
        lines.append("The final row is the paper metric policy: macro average over project-level rates.")
    return "\n".join(lines) + "\n"


def _default_output_for_run(run_dir: Path) -> Path:
    """生成默认输出路径。"""
    return Path("paper_experiments/results") / f"required_unsafe_{run_dir.resolve().name}.json"


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    parser = argparse.ArgumentParser(description="统计翻译 Rust crate 中证据充分的必须 unsafe 行。")
    parser.add_argument("--crate-dir", action="append", type=Path, help="单个 Rust crate 目录；可重复")
    parser.add_argument("--run-dir", type=Path, help="batch run 目录，例如 experiment_runs/deepseek-v4-pro-ohos11-full-0605")
    parser.add_argument("--projects", help="逗号分隔项目名；默认 10 个 OHOS C gtest 项目")
    parser.add_argument("--output", "-o", type=Path, help="JSON 输出路径")
    parser.add_argument("--markdown", type=Path, help="Markdown 表格输出路径")
    parser.add_argument("--details-limit", type=int, default=500, help="每项目最多保留多少条 findings 样例")
    args = parser.parse_args(argv)

    if not args.crate_dir and not args.run_dir:
        parser.error("必须提供 --crate-dir 或 --run-dir")

    if args.run_dir:
        projects = _parse_projects(args.projects)
        report = analyze_run(args.run_dir, projects, details_limit=args.details_limit)
        out_path = args.output or _default_output_for_run(args.run_dir)
    else:
        project_results: Dict[str, Any] = {}
        summary = Counter()
        reason_totals: Counter[str] = Counter()
        for crate_dir in args.crate_dir or []:
            result = analyze_crate(crate_dir, details_limit=args.details_limit)
            project_results[result["project"]] = result
            summary["projects_analyzed"] += 1
            summary["total_lines"] += int(result["total_lines"])
            summary["code_lines"] += int(result["code_lines"])
            summary["raw_unsafe_keyword_lines"] += int(result["raw_unsafe_keyword_lines"])
            summary["raw_unsafe_lines"] += int(result["raw_unsafe_lines"])
            summary["required_unsafe_lines"] += int(result["required_unsafe_lines"])
            reason_totals.update(result.get("required_reasons") or {})
        micro_summary = {
            "projects_analyzed": summary["projects_analyzed"],
            "total_lines": summary["total_lines"],
            "code_lines": summary["code_lines"],
            "raw_unsafe_keyword_lines": summary["raw_unsafe_keyword_lines"],
            "raw_unsafe_lines": summary["raw_unsafe_lines"],
            "raw_unsafe_ratio": (
                summary["raw_unsafe_lines"] / summary["total_lines"] if summary["total_lines"] else 0.0
            ),
            "required_unsafe_lines": summary["required_unsafe_lines"],
            "required_unsafe_ratio": (
                summary["required_unsafe_lines"] / summary["total_lines"] if summary["total_lines"] else 0.0
            ),
            "required_reasons": dict(sorted(reason_totals.items())),
        }
        report = {
            "projects": project_results,
            "summary": {
                **micro_summary,
                "metric_policy": "paper_macro_project_average",
                "raw_unsafe_ratio": _mean(
                    [float(item.get("raw_unsafe_ratio") or 0.0) for item in project_results.values()]
                ),
                "required_unsafe_ratio": _mean(
                    [float(item.get("required_unsafe_ratio") or 0.0) for item in project_results.values()]
                ),
                "micro_evidence": micro_summary,
            },
        }
        out_path = args.output or Path("paper_experiments/results/required_unsafe_crates.json")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    md_path = args.markdown or out_path.with_suffix(".md")
    md_path.write_text(render_markdown(report), encoding="utf-8")

    print(f"JSON: {out_path}")
    print(f"Markdown: {md_path}")
    summary_obj = report.get("summary") or {}
    print(
        "raw unsafe: " f"{_format_percent(float(summary_obj.get('raw_unsafe_ratio') or 0.0))}"
    )
    print(
        "required unsafe: " f"{_format_percent(float(summary_obj.get('required_unsafe_ratio') or 0.0))}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
