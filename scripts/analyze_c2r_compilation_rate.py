#!/usr/bin/env python3
"""
后处理脚本：计算 C2R 增量翻译的函数级可编译率

适用于 /data/home/wangshb/c2-rust_framework/translation_outputs/Our/OurSmoke 格式的输出

功能特性:
- 分析函数级编译成功率（一次通过 / 修复后通过 / 失败回退）
- 错误类型分类和统计
- Unsafe 代码分析
- 语义评估结果整合
- 并行处理支持
- 详细的表格输出

输出格式:
{
    "run_dir": "...",
    "timestamp": "...",
    "projects": {
        "project_name": {
            "total_functions": 91,
            "compiled_direct": 59,
            "compiled_after_repair": 18,
            "failed_reverted": 14,
            "c2rust_fallback": 4,
            "pass_rate": 0.8462,
            "semantic_score": 48.18,
            "error_analysis": {...}
        }
    },
    "summary": {
        "total_projects": 10,
        "total_functions": 326,
        "successful_functions": 308,
        "overall_function_success_rate": 0.9448
    }
}
"""

import argparse
import json
import os
import re
import subprocess
import sys
from bisect import bisect_right
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass, field, asdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple


# ============================================================================
# 项目展示顺序（统一四套脚本的表格行顺序）
# ============================================================================

DISPLAY_PROJECT_ORDER: List[str] = [
    "ht",
    "qsort",
    "quadtree",
    "buffer",
    "rgba",
    "urlparser",
    "genann",
    "avl",
    "bzip2",
    "zopfli",
]
_DISPLAY_PROJECT_ORDER_INDEX: Dict[str, int] = {name: i for i, name in enumerate(DISPLAY_PROJECT_ORDER)}


def iter_projects_in_display_order(projects: Dict[str, Any]) -> List[Tuple[str, Any]]:
    """Return (project_name, project_data) in a stable, paper-friendly order."""
    return sorted(
        projects.items(),
        key=lambda kv: (_DISPLAY_PROJECT_ORDER_INDEX.get(kv[0], 1_000_000), kv[0]),
    )

_HIS2TRANS_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_RUN_DIR = _HIS2TRANS_ROOT / "data" / "rq2" / "deepseek"


# ============================================================================
# 数据类定义
# ============================================================================

@dataclass
class ErrorAnalysis:
    """错误分析结果"""
    categories: Dict[str, int] = field(default_factory=dict)
    details: List[str] = field(default_factory=list)
    failed_functions: Dict[str, Dict[str, Any]] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class UnsafeAnalysis:
    """Unsafe 代码分析结果"""
    total_lines: int = 0
    code_lines: int = 0
    unsafe_keyword_occurrences: int = 0
    unsafe_keyword_lines: int = 0  # 出现 unsafe 关键字的代码行（排除注释/字符串）
    unsafe_keyword_ratio: float = 0.0
    unsafe_context_lines: int = 0  # 位于 unsafe {}/unsafe fn/unsafe impl/unsafe trait 作用域内的代码行
    unsafe_context_ratio: float = 0.0
    unsafe_total_lines: int = 0    # 关键字行 ∪ 作用域行
    unsafe_total_ratio: float = 0.0
    unsafe_block_count: int = 0    # unsafe { ... }
    unsafe_fn_count: int = 0       # unsafe fn
    unsafe_impl_count: int = 0     # unsafe impl
    unsafe_trait_count: int = 0    # unsafe trait
    total_unsafe_items: int = 0    # 所有 unsafe 项总数
    unsafe_lines_estimate: int = 0 # 估算的 unsafe 行数
    files_analyzed: int = 0
    unsafe_ratio: float = 0.0      # unsafe_lines_estimate / code_lines（仅块/unsafe fn 体内）
    unsafe_density_per_kloc: float = 0.0  # 每千行代码的 unsafe 项数
    error: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class ProjectResult:
    """单个项目的分析结果"""
    project_name: str
    total_functions: int = 0
    translated: int = 0
    compiled_direct: int = 0
    compiled_after_repair: int = 0
    compiled_total: int = 0
    c2rust_fallback: int = 0
    failed_reverted: int = 0
    skipped: int = 0
    injection_failed: int = 0
    still_unimplemented: int = 0
    timeout_failed: int = 0
    pass_rate: float = 0.0
    direct_pass_rate: float = 0.0
    semantic_score: Optional[float] = None
    semantic_summary: Optional[Dict[str, Any]] = None
    error_analysis: Optional[ErrorAnalysis] = None
    unsafe_analysis: Optional[UnsafeAnalysis] = None
    cargo_test_result: Optional[Dict[str, Any]] = None
    cargo_clippy_result: Optional[Dict[str, Any]] = None
    c2r_test_result: Optional[Dict[str, Any]] = None
    incremental_compilation: Optional[Dict[str, Any]] = None

    def to_dict(self) -> Dict[str, Any]:
        result = {
            "project_name": self.project_name,
            "total_functions": self.total_functions,
            "translated": self.translated,
            "compiled_direct": self.compiled_direct,
            "compiled_after_repair": self.compiled_after_repair,
            "compiled_total": self.compiled_total,
            "c2rust_fallback": self.c2rust_fallback,
            "failed_reverted": self.failed_reverted,
            "skipped": self.skipped,
            "injection_failed": self.injection_failed,
            "still_unimplemented": self.still_unimplemented,
            "timeout_failed": self.timeout_failed,
            "pass_rate": self.pass_rate,
            "direct_pass_rate": self.direct_pass_rate,
        }
        if self.semantic_score is not None:
            result["semantic_score"] = self.semantic_score
        if self.semantic_summary:
            result["semantic_summary"] = self.semantic_summary
        if self.error_analysis:
            result["error_analysis"] = self.error_analysis.to_dict()
        if self.unsafe_analysis:
            result["unsafe_analysis"] = self.unsafe_analysis.to_dict()
        if self.cargo_test_result:
            result["cargo_test_result"] = self.cargo_test_result
        if self.cargo_clippy_result:
            result["cargo_clippy_result"] = self.cargo_clippy_result
        if self.c2r_test_result:
            result["c2r_test_result"] = self.c2r_test_result
        if self.incremental_compilation:
            result["incremental_compilation"] = self.incremental_compilation
        return result


@dataclass
class AnalysisSummary:
    """分析汇总结果"""
    total_projects: int = 0
    total_functions: int = 0
    successful_functions: int = 0
    direct_pass_functions: int = 0
    repaired_functions: int = 0
    failed_functions: int = 0
    c2rust_fallback_functions: int = 0
    overall_pass_rate: float = 0.0
    direct_pass_rate: float = 0.0
    error_categories: Dict[str, int] = field(default_factory=dict)
    unsafe_summary: Dict[str, Any] = field(default_factory=dict)
    semantic_evaluation: Optional[Dict[str, Any]] = None
    test_summary: Optional[Dict[str, Any]] = None
    clippy_summary: Optional[Dict[str, Any]] = None
    c2r_test_summary: Optional[Dict[str, Any]] = None
    incremental_compilation_summary: Optional[Dict[str, Any]] = None

    def to_dict(self) -> Dict[str, Any]:
        result = {
            "total_projects": self.total_projects,
            "total_functions": self.total_functions,
            "successful_functions": self.successful_functions,
            "direct_pass_functions": self.direct_pass_functions,
            "repaired_functions": self.repaired_functions,
            "failed_functions": self.failed_functions,
            "c2rust_fallback_functions": self.c2rust_fallback_functions,
            "overall_pass_rate": self.overall_pass_rate,
            "direct_pass_rate": self.direct_pass_rate,
            "error_categories": self.error_categories,
            "unsafe_summary": self.unsafe_summary,
        }
        if self.semantic_evaluation:
            result["semantic_evaluation"] = self.semantic_evaluation
        if self.test_summary:
            result["test_summary"] = self.test_summary
        if self.clippy_summary:
            result["clippy_summary"] = self.clippy_summary
        if self.c2r_test_summary:
            result["c2r_test_summary"] = self.c2r_test_summary
        if self.incremental_compilation_summary:
            result["incremental_compilation_summary"] = self.incremental_compilation_summary
        return result


# ============================================================================
# 错误分类模式
# ============================================================================

ERROR_PATTERNS: List[Tuple[str, str]] = [
    (r'error\[E0609\].*no field', "E0609_opaque_field_access"),
    (r'error\[E0425\].*cannot find function.*c2r_field_ptr', "E0425_missing_accessor_shim"),
    (r'error\[E0425\].*cannot find value.*stderr', "E0425_missing_stderr"),
    (r'error\[E0425\].*cannot find function.*va_', "E0425_missing_va_functions"),
    (r'error\[E0425\]', "E0425_other_missing_symbol"),
    (r'error\[E0063\].*missing fields', "E0063_missing_struct_fields"),
    (r'error\[E0308\].*mismatched types', "E0308_type_mismatch"),
    (r'error\[E0606\].*casting.*invalid', "E0606_invalid_cast"),
    (r'error\[E0133\].*unsafe', "E0133_unsafe_required"),
    (r'error\[E0530\].*shadow.*static', "E0530_shadow_static"),
    (r'error\[E0615\].*method.*not.*field', "E0615_method_not_field"),
    (r'error\[E0599\].*no method named', "E0599_no_method"),
    (r'error\[E0277\].*trait bound', "E0277_trait_bound"),
    (r'error\[E0382\].*borrow of moved value', "E0382_moved_value"),
    (r'error\[E0502\].*cannot borrow', "E0502_borrow_conflict"),
    (r'error.*expected.*found', "syntax_error"),
]

ERROR_CATEGORY_NAMES: Dict[str, str] = {
    "E0609_opaque_field_access": "E0609 Opaque类型字段访问",
    "E0425_missing_accessor_shim": "E0425 缺少Accessor Shim",
    "E0425_missing_stderr": "E0425 缺少stderr",
    "E0425_missing_va_functions": "E0425 缺少va_*函数",
    "E0425_other_missing_symbol": "E0425 其他符号缺失",
    "E0063_missing_struct_fields": "E0063 结构体字段缺失",
    "E0308_type_mismatch": "E0308 类型不匹配",
    "E0606_invalid_cast": "E0606 无效类型转换",
    "E0133_unsafe_required": "E0133 需要unsafe块",
    "E0530_shadow_static": "E0530 静态变量遮蔽",
    "E0615_method_not_field": "E0615 方法非字段",
    "E0599_no_method": "E0599 方法不存在",
    "E0277_trait_bound": "E0277 Trait约束不满足",
    "E0382_moved_value": "E0382 已移动值借用",
    "E0502_borrow_conflict": "E0502 借用冲突",
    "syntax_error": "语法错误",
    "other": "其他错误"
}


# ============================================================================
# 错误分析函数
# ============================================================================

def categorize_errors(error_output: str) -> Dict[str, int]:
    """
    分类错误类型

    Args:
        error_output: cargo 编译错误输出

    Returns:
        错误类别到数量的映射
    """
    if not error_output:
        return {}

    categories: Dict[str, int] = defaultdict(int)

    for pattern, category in ERROR_PATTERNS:
        matches = re.findall(pattern, error_output, re.IGNORECASE)
        if matches:
            categories[category] += len(matches)

    # 统计未分类的错误
    total_errors = error_output.count("error[") + error_output.count("error:")
    known_errors = sum(categories.values())
    if total_errors > known_errors:
        categories["other"] = total_errors - known_errors

    return dict(categories)


def parse_repair_errors(repair_history_dir: Path) -> ErrorAnalysis:
    """
    解析 repair_history 目录中的错误信息，分类统计错误类型

    Args:
        repair_history_dir: repair_history 目录路径

    Returns:
        ErrorAnalysis 对象
    """
    result = ErrorAnalysis()

    if not repair_history_dir.exists():
        return result

    error_categories: Dict[str, int] = defaultdict(int)

    # 遍历所有函数的 repair 目录
    for func_dir in repair_history_dir.iterdir():
        if not func_dir.is_dir() or func_dir.name.startswith('_'):
            continue

        func_name = func_dir.name

        # 找到最后一次尝试的错误文件
        error_files = sorted(func_dir.glob("attempt_*_error.txt"))
        if not error_files:
            continue

        last_error_file = error_files[-1]
        try:
            error_content = last_error_file.read_text(encoding='utf-8', errors='ignore')
        except Exception:
            continue

        # 解析错误类型
        errors_found = categorize_errors(error_content)
        for cat, count in errors_found.items():
            error_categories[cat] += count

        # 记录失败函数的错误
        if errors_found:
            result.failed_functions[func_name] = {
                "attempts": len(error_files),
                "last_error": error_content[:500],
                "error_types": list(errors_found.keys())
            }

    result.categories = dict(error_categories)
    return result


# ============================================================================
# Unsafe 代码分析
# ============================================================================

def count_code_lines(content: str) -> Tuple[int, int, int]:
    """
    统计代码行数、unsafe关键字行数

    Args:
        content: 文件内容

    Returns:
        (总行数, 代码行数, unsafe关键字行数)
    """
    lines = content.split('\n')
    total_lines = len(lines)
    code_lines = 0
    unsafe_lines = 0
    in_block_comment = False

    for line in lines:
        stripped = line.strip()

        # 处理块注释
        if '/*' in stripped:
            in_block_comment = True
        if '*/' in stripped:
            in_block_comment = False
            continue
        if in_block_comment:
            continue

        # 统计代码行（非空非纯注释）
        if stripped and not stripped.startswith('//'):
            code_lines += 1
            if 'unsafe' in stripped:
                unsafe_lines += 1

    return total_lines, code_lines, unsafe_lines


def _is_ident_start(ch: str) -> bool:
    return bool(ch) and (ch == '_' or ('a' <= ch <= 'z') or ('A' <= ch <= 'Z'))


def _is_ident_continue(ch: str) -> bool:
    return bool(ch) and (ch == '_' or ch.isdigit() or ('a' <= ch <= 'z') or ('A' <= ch <= 'Z'))


def _is_token_boundary(s: str, idx: int) -> bool:
    """True if idx is out of bounds or s[idx] is not an identifier char."""
    return idx < 0 or idx >= len(s) or not _is_ident_continue(s[idx])


def _fnscan_skip_ws(s: str, pos: int) -> int:
    while pos < len(s) and s[pos].isspace():
        pos += 1
    return pos


def _fnscan_scan_balanced(s: str, pos: int, open_ch: str, close_ch: str) -> Optional[int]:
    """
    从 s[pos]==open_ch 开始，扫描匹配到对应 close_ch 的位置（返回 close_ch 之后的 index）。
    best-effort：跳过注释、字符串、raw string；并对 char literal 做简易跳过。
    """
    if pos >= len(s) or s[pos] != open_ch:
        return None
    depth = 1
    pos += 1
    n = len(s)
    while pos < n and depth > 0:
        if s.startswith('//', pos):
            pos = _skip_line_comment(s, pos)
            continue
        if s.startswith('/*', pos):
            pos = _skip_block_comment(s, pos)
            continue

        raw_end = _try_skip_raw_string(s, pos)
        if raw_end is not None:
            pos = raw_end
            continue

        if s.startswith('b"', pos):
            pos = _skip_normal_string(s, pos + 1)
            continue
        if s[pos] == '"':
            pos = _skip_normal_string(s, pos)
            continue

        if s[pos] == "'":
            lookahead = s[pos + 1:pos + 12]
            closing_quote_pos = lookahead.find("'")
            if 0 < closing_quote_pos < 10:
                pos += closing_quote_pos + 2
                continue

        c = s[pos]
        if c == open_ch:
            depth += 1
        elif c == close_ch:
            # 避免把 '->' 的 '>' 当成泛型闭合
            if close_ch == '>' and pos > 0 and s[pos - 1] == '-':
                pass
            else:
                depth -= 1
        pos += 1

    return pos if depth == 0 else None


def _fnscan_find_fn_body_span(s: str, fn_kw_pos: int, fn_name: str) -> Optional[Tuple[int, int]]:
    """
    给定 `fn` 关键字位置和函数名，返回 (body_start, body_end)（body_end 为 '}' 之后的位置）。
    若是声明（以 ';' 结束）或解析失败，返回 None。
    """
    pos = fn_kw_pos + 2  # after 'fn'
    pos = _fnscan_skip_ws(s, pos)
    if not s.startswith(fn_name, pos):
        return None
    pos += len(fn_name)
    pos = _fnscan_skip_ws(s, pos)

    if pos < len(s) and s[pos] == '<':
        end = _fnscan_scan_balanced(s, pos, '<', '>')
        if end is None:
            return None
        pos = _fnscan_skip_ws(s, end)

    if pos >= len(s) or s[pos] != '(':
        return None
    end = _fnscan_scan_balanced(s, pos, '(', ')')
    if end is None:
        return None
    pos = _fnscan_skip_ws(s, end)

    angle = 0
    paren = 0
    bracket = 0
    n = len(s)
    while pos < n:
        if s.startswith('//', pos):
            pos = _skip_line_comment(s, pos)
            continue
        if s.startswith('/*', pos):
            pos = _skip_block_comment(s, pos)
            continue

        raw_end = _try_skip_raw_string(s, pos)
        if raw_end is not None:
            pos = raw_end
            continue

        if s.startswith('b"', pos):
            pos = _skip_normal_string(s, pos + 1)
            continue
        if s[pos] == '"':
            pos = _skip_normal_string(s, pos)
            continue

        if s[pos] == "'":
            lookahead = s[pos + 1:pos + 12]
            closing_quote_pos = lookahead.find("'")
            if 0 < closing_quote_pos < 10:
                pos += closing_quote_pos + 2
                continue

        c = s[pos]
        if c == '(':
            paren += 1
        elif c == ')':
            paren = max(0, paren - 1)
        elif c == '[':
            bracket += 1
        elif c == ']':
            bracket = max(0, bracket - 1)
        elif c == '<':
            angle += 1
        elif c == '>':
            if pos > 0 and s[pos - 1] == '-':
                pass
            else:
                angle = max(0, angle - 1)
        elif c == ';' and paren == 0 and angle == 0 and bracket == 0:
            return None
        elif c == '{' and paren == 0 and angle == 0 and bracket == 0:
            body_start = pos
            body_end = _find_matching_brace(s, body_start)
            if body_end is None:
                return None
            return (body_start, body_end)
        pos += 1
    return None


def _fnscan_iter_function_items(s: str):
    """
    遍历所有带函数体的函数项，yield (fn_name, fn_kw_pos, body_start, body_end)。

    关键点：必须跳过注释/字符串里的 `fn ...`（例如我们插入的 `// rust_signature: ... fn ...`），
    否则会把注释里的 fn 错误地“绑定”到后续真实代码的 `{}`，导致 stubbed skeleton 语法损坏。
    """
    i = 0
    n = len(s)
    while i < n:
        # Skip comments.
        if s.startswith('//', i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith('/*', i):
            i = _skip_block_comment(s, i)
            continue

        # Skip strings (raw + normal).
        raw_end = _try_skip_raw_string(s, i)
        if raw_end is not None:
            i = raw_end
            continue
        if s.startswith('b"', i):
            i = _skip_normal_string(s, i + 1)
            continue
        if s[i] == '"':
            i = _skip_normal_string(s, i)
            continue

        # Skip simple char literals to avoid accidental scanning inside them.
        if s[i] == "'":
            lookahead = s[i + 1:i + 12]
            closing_quote_pos = lookahead.find("'")
            if 0 < closing_quote_pos < 10:
                i += closing_quote_pos + 2
                continue

        # Detect token `fn` outside comments/strings.
        if s.startswith('fn', i) and _is_token_boundary(s, i - 1) and _is_token_boundary(s, i + 2):
            j = _fnscan_skip_ws(s, i + 2)
            # Parse function name (support raw identifiers like `r#type`).
            if j < n and s.startswith('r#', j):
                if j + 2 >= n or not _is_ident_start(s[j + 2]):
                    i += 2
                    continue
                k = j + 3
                while k < n and _is_ident_continue(s[k]):
                    k += 1
                fn_name = s[j:k]  # includes "r#"
            else:
                if j >= n or not _is_ident_start(s[j]):
                    i += 2
                    continue
                k = j + 1
                while k < n and _is_ident_continue(s[k]):
                    k += 1
                fn_name = s[j:k]

            span = _fnscan_find_fn_body_span(s, i, fn_name)
            if span is not None:
                body_start, body_end = span
                yield fn_name, i, body_start, body_end
                # Skip over the function body; Rust doesn't allow nested fn items inside fn bodies.
                i = body_end
                continue
        i += 1


def stub_all_functions_in_content(content: str) -> str:
    """
    将文件内容中的所有函数体替换为 unimplemented!()
    保留：use语句、宏定义、类型定义、常量、impl块等
    跳过：extern块中的函数声明（没有函数体）
    跳过：块注释 /* ... */ 中的函数定义
    """
    result_parts: List[str] = []
    cursor = 0
    for fn_name, _fn_kw_pos, body_start, body_end in _fnscan_iter_function_items(content):
        result_parts.append(content[cursor:body_start])
        # NOTE: Do NOT pass a formatted message to `unimplemented!()` here.
        # In `const fn`, `unimplemented!("...")` triggers E0015 ("cannot call non-const formatting macro"),
        # which makes the fully-stubbed skeleton fail to compile.
        result_parts.append('{ unimplemented!() }')
        cursor = body_end
    result_parts.append(content[cursor:])
    return ''.join(result_parts)


def extract_function_from_content(content: str, func_name: str) -> Optional[str]:
    """
    从文件内容中提取指定函数的完整代码（包括属性）
    """
    for fn_name, fn_kw_pos, _body_start, body_end in _fnscan_iter_function_items(content):
        if fn_name != func_name:
            continue
        line_start = content.rfind('\n', 0, fn_kw_pos)
        item_start = 0 if line_start == -1 else line_start + 1
        return content[item_start:body_end]
    return None


def restore_function_in_content(content: str, func_name: str, original_code: str) -> str:
    """
    在已 stub 的内容中，恢复指定函数的原始代码
    """
    for fn_name, fn_kw_pos, _body_start, body_end in _fnscan_iter_function_items(content):
        if fn_name != func_name:
            continue
        line_start = content.rfind('\n', 0, fn_kw_pos)
        item_start = 0 if line_start == -1 else line_start + 1
        return content[:item_start] + original_code + content[body_end:]
    return content.rstrip() + '\n\n' + original_code + '\n'


def count_unsafe_lines_precise(content: str) -> Tuple[int, int, int, int]:
    """
    精确统计 unsafe 代码的实际行数

    通过解析代码，找到每个 unsafe 块和 unsafe 函数，统计其内部的实际代码行数。

    Args:
        content: 文件内容

    Returns:
        (unsafe_block_lines, unsafe_fn_lines, unsafe_block_count, unsafe_fn_count)
    """
    unsafe_block_lines = 0
    unsafe_fn_lines = 0
    unsafe_block_count = 0
    unsafe_fn_count = 0

    n = len(content)
    i = 0

    while i < n:
        # 跳过字符串
        if content[i] == '"':
            i += 1
            while i < n and content[i] != '"':
                if content[i] == '\\' and i + 1 < n:
                    i += 2
                else:
                    i += 1
            i += 1
            continue

        # 跳过行注释
        if content[i:i+2] == '//':
            while i < n and content[i] != '\n':
                i += 1
            continue

        # 跳过块注释
        if content[i:i+2] == '/*':
            i += 2
            while i < n - 1 and content[i:i+2] != '*/':
                i += 1
            i += 2
            continue

        # 检查 unsafe 关键字
        if content[i:i+6] == 'unsafe' and (i == 0 or not content[i-1].isalnum() and content[i-1] != '_'):
            # 确保是完整的 unsafe 关键字
            end_pos = i + 6
            if end_pos < n and (content[end_pos].isalnum() or content[end_pos] == '_'):
                i += 1
                continue

            # 跳过空白
            j = end_pos
            while j < n and content[j] in ' \t\n\r':
                j += 1

            if j >= n:
                i += 1
                continue

            # 检查是 unsafe { (块) 还是 unsafe fn/impl/trait
            if content[j] == '{':
                # unsafe 块
                unsafe_block_count += 1
                brace_start = j
                lines = count_lines_in_braces(content, brace_start)
                unsafe_block_lines += lines
                i = j + 1
            elif content[j:j+2] == 'fn' or content[j:j+6] == 'extern':
                # unsafe fn / unsafe extern "C" fn - 找到函数体并统计行数
                unsafe_fn_count += 1
                # 找到函数体的开始 {
                k = j
                while k < n and content[k] != '{':
                    if content[k] == ';':  # 这是声明，不是定义
                        break
                    k += 1
                if k < n and content[k] == '{':
                    lines = count_lines_in_braces(content, k)
                    unsafe_fn_lines += lines
                i = k + 1
            else:
                i += 1
        else:
            i += 1

    return unsafe_block_lines, unsafe_fn_lines, unsafe_block_count, unsafe_fn_count


def count_lines_in_braces(content: str, brace_start: int) -> int:
    """
    统计花括号内的代码行数

    Args:
        content: 文件内容
        brace_start: 开始花括号 '{' 的位置

    Returns:
        花括号内的代码行数
    """
    n = len(content)
    if brace_start >= n or content[brace_start] != '{':
        return 0

    brace_count = 1
    j = brace_start + 1
    start_line = content[:brace_start].count('\n')
    in_string = False

    while j < n and brace_count > 0:
        c = content[j]

        # 处理字符串
        if not in_string and c == '"':
            in_string = True
            j += 1
            continue
        if in_string:
            if c == '\\' and j + 1 < n:
                j += 2
                continue
            if c == '"':
                in_string = False
            j += 1
            continue

        # 跳过行注释
        if content[j:j+2] == '//':
            while j < n and content[j] != '\n':
                j += 1
            continue

        # 跳过块注释
        if content[j:j+2] == '/*':
            j += 2
            while j < n - 1 and content[j:j+2] != '*/':
                j += 1
            j += 2
            continue

        if c == '{':
            brace_count += 1
        elif c == '}':
            brace_count -= 1
        j += 1

    end_line = content[:j].count('\n')
    # 返回行数（不包括只有花括号的行）
    return max(0, end_line - start_line - 1)


def _is_ident_char(ch: str) -> bool:
    return ch.isalnum() or ch == '_'


def _skip_line_comment(s: str, i: int) -> int:
    """Assumes s[i:i+2] == '//' and returns index at newline or end."""
    nl = s.find('\n', i + 2)
    return len(s) if nl == -1 else nl


def _skip_block_comment(s: str, i: int) -> int:
    """Assumes s[i:i+2] == '/*' and returns index after matching '*/' (supports nesting)."""
    depth = 1
    i += 2
    n = len(s)
    while i < n and depth > 0:
        if s.startswith('/*', i):
            depth += 1
            i += 2
        elif s.startswith('*/', i):
            depth -= 1
            i += 2
        else:
            i += 1
    return i


def _skip_normal_string(s: str, i: int) -> int:
    """Assumes s[i] == '\"' and returns index after closing quote."""
    n = len(s)
    i += 1
    while i < n:
        c = s[i]
        if c == '\\' and i + 1 < n:
            i += 2
            continue
        if c == '"':
            return i + 1
        i += 1
    return i


def _try_skip_raw_string(s: str, i: int) -> Optional[int]:
    """
    If s[i:] starts a raw string literal (r\"...\" / r#\"...\"# / br#\"...\"#), return end index; else None.
    Note: best-effort for analysis.
    """
    n = len(s)
    if s.startswith('br', i):
        i += 2
    elif s.startswith('r', i):
        i += 1
    else:
        return None

    hash_count = 0
    while i < n and s[i] == '#':
        hash_count += 1
        i += 1

    if i >= n or s[i] != '"':
        return None
    i += 1

    terminator = '"' + ('#' * hash_count)
    end_pos = s.find(terminator, i)
    if end_pos == -1:
        return n
    return end_pos + len(terminator)


def _skip_ws_and_comments(s: str, i: int) -> int:
    """Skip whitespace and comments; stop at the next non-ws, non-comment character."""
    n = len(s)
    while i < n:
        if s[i].isspace():
            i += 1
            continue
        if s.startswith('//', i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith('/*', i):
            i = _skip_block_comment(s, i)
            continue
        break
    return i


def _find_matching_brace(s: str, brace_start: int) -> Optional[int]:
    """Given s[brace_start] == '{', return index after matching '}' (exclusive), skipping comments/strings."""
    if brace_start >= len(s) or s[brace_start] != '{':
        return None

    depth = 1
    i = brace_start + 1
    n = len(s)
    while i < n and depth > 0:
        if s.startswith('//', i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith('/*', i):
            i = _skip_block_comment(s, i)
            continue

        raw_end = _try_skip_raw_string(s, i)
        if raw_end is not None:
            i = raw_end
            continue

        if s.startswith('b"', i):
            i = _skip_normal_string(s, i + 1)
            continue
        if s[i] == '"':
            i = _skip_normal_string(s, i)
            continue

        c = s[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
        i += 1

    return i if depth == 0 else None


def _find_body_brace_or_decl_end(s: str, i: int) -> Optional[int]:
    """
    Starting from i, scan forward (skipping comments/strings) to find the next body '{'.
    Returns brace index if found; returns None if it looks like a declaration (hits ';') or EOF.
    """
    n = len(s)
    while i < n:
        if s.startswith('//', i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith('/*', i):
            i = _skip_block_comment(s, i)
            continue

        raw_end = _try_skip_raw_string(s, i)
        if raw_end is not None:
            i = raw_end
            continue

        if s.startswith('b"', i):
            i = _skip_normal_string(s, i + 1)
            continue
        if s[i] == '"':
            i = _skip_normal_string(s, i)
            continue

        c = s[i]
        if c == '{':
            return i
        if c == ';':
            return None
        i += 1
    return None


def analyze_unsafe_global_in_content(content: str) -> Dict[str, int]:
    """
    全局统计 unsafe（排除注释/字符串）：
    - code_lines: 代码行数
    - unsafe_keyword_occurrences: unsafe 关键字出现次数
    - unsafe_keyword_lines: 含 unsafe 关键字的代码行数（含 unsafe extern/impl/trait/...）
    - unsafe_context_lines: 位于 unsafe 作用域内的代码行数（unsafe block / unsafe fn / unsafe extern / unsafe impl / unsafe trait）
    - unsafe_total_lines: keyword_lines ∪ context_lines
    """
    line_starts = [0]
    for idx, ch in enumerate(content):
        if ch == '\n':
            line_starts.append(idx + 1)

    line_count = len(line_starts)
    line_has_code = [False] * line_count
    line_has_unsafe_kw = [False] * line_count
    unsafe_kw_occ = 0
    unsafe_spans: List[Tuple[int, int]] = []

    i = 0
    n = len(content)
    line = 0
    in_line_comment = False
    block_comment_depth = 0

    def advance(to_i: int) -> None:
        nonlocal i, line
        if to_i <= i:
            i = to_i
            return
        line += content.count('\n', i, to_i)
        i = to_i

    while i < n:
        c = content[i]

        if c == '\n':
            in_line_comment = False
            line += 1
            i += 1
            continue

        if in_line_comment:
            i += 1
            continue

        if block_comment_depth > 0:
            if content.startswith('/*', i):
                block_comment_depth += 1
                i += 2
                continue
            if content.startswith('*/', i):
                block_comment_depth -= 1
                i += 2
                continue
            i += 1
            continue

        if content.startswith('//', i):
            in_line_comment = True
            i += 2
            continue
        if content.startswith('/*', i):
            block_comment_depth = 1
            i += 2
            continue

        raw_end = _try_skip_raw_string(content, i)
        if raw_end is not None:
            line_has_code[line] = True
            advance(raw_end)
            continue

        if content.startswith('b"', i):
            line_has_code[line] = True
            end = _skip_normal_string(content, i + 1)
            advance(end)
            continue
        if c == '"':
            line_has_code[line] = True
            end = _skip_normal_string(content, i)
            advance(end)
            continue

        if not c.isspace():
            line_has_code[line] = True

        if content.startswith('unsafe', i):
            before_ok = (i == 0) or (not _is_ident_char(content[i - 1]))
            after_idx = i + 6
            after_ok = (after_idx >= n) or (not _is_ident_char(content[after_idx]))
            if before_ok and after_ok:
                line_has_unsafe_kw[line] = True
                unsafe_kw_occ += 1

                j = _skip_ws_and_comments(content, after_idx)
                if j < n:
                    if content[j] == '{':
                        body_start = j
                        body_end = _find_matching_brace(content, body_start)
                        if body_end is not None:
                            unsafe_spans.append((body_start, body_end))
                    elif content.startswith('fn', j) and (j + 2 >= n or not _is_ident_char(content[j + 2])):
                        body_start = _find_body_brace_or_decl_end(content, j + 2)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith('extern', j) and (j + 6 >= n or not _is_ident_char(content[j + 6])):
                        body_start = _find_body_brace_or_decl_end(content, j + 6)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith('impl', j) and (j + 4 >= n or not _is_ident_char(content[j + 4])):
                        body_start = _find_body_brace_or_decl_end(content, j + 4)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith('trait', j) and (j + 5 >= n or not _is_ident_char(content[j + 5])):
                        body_start = _find_body_brace_or_decl_end(content, j + 5)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))

        i += 1

    line_in_unsafe_ctx = [False] * line_count

    def idx_to_line(idx: int) -> int:
        if idx <= 0:
            return 0
        return bisect_right(line_starts, idx) - 1

    for start, end in unsafe_spans:
        if end <= start:
            continue
        start_line = idx_to_line(start)
        end_line = idx_to_line(end - 1)
        if start_line < 0:
            start_line = 0
        if end_line >= line_count:
            end_line = line_count - 1
        for ln in range(start_line, end_line + 1):
            line_in_unsafe_ctx[ln] = True

    code_lines = sum(1 for v in line_has_code if v)
    unsafe_keyword_lines = sum(1 for ln in range(line_count) if line_has_code[ln] and line_has_unsafe_kw[ln])
    unsafe_context_lines = sum(1 for ln in range(line_count) if line_has_code[ln] and line_in_unsafe_ctx[ln])
    unsafe_total_lines = sum(
        1 for ln in range(line_count)
        if line_has_code[ln] and (line_has_unsafe_kw[ln] or line_in_unsafe_ctx[ln])
    )

    return {
        "code_lines": code_lines,
        "unsafe_keyword_occurrences": unsafe_kw_occ,
        "unsafe_keyword_lines": unsafe_keyword_lines,
        "unsafe_context_lines": unsafe_context_lines,
        "unsafe_total_lines": unsafe_total_lines,
    }


def analyze_unsafe_code(project_dir: Path) -> UnsafeAnalysis:
    """
    分析项目中的 unsafe 代码比例

    使用精确统计方法：实际解析代码，统计 unsafe 块和 unsafe 函数内的真实代码行数。

    Args:
        project_dir: Rust 项目目录（全局扫描 *.rs，排除 target）

    Returns:
        UnsafeAnalysis 对象
    """
    result = UnsafeAnalysis()

    if not project_dir.exists():
        # Keep legacy error wording for existing test expectations.
        result.error = "src directory not found"
        return result

    try:
        # NOTE:
        # - We intentionally scan Rust sources broadly for this project (not only `src/`),
        #   because some runs may place generated sources in non-standard locations.
        # - However, we must exclude large *uncompiled* fallback artifacts, otherwise the
        #   unsafe ratio is dominated by caches rather than the final translated crate.
        #
        # Common fallback/cache dirs:
        # - `.c2r_c2rust_fallback/` (framework cache; not part of the final crate)
        # - `src/__c2r_generated/c2rust_fallback/` (C2Rust fallback dump; usually not referenced)
        # - `.c2r_bindgen_*/` (bindgen-generated FFI shims; mostly `unsafe extern "C"` decls and
        #   not representative of unsafe usage inside function bodies)
        exclude_parts = {
            "target",
            ".git",
            ".c2r_c2rust_fallback",
            "c2rust_fallback",
        }
        rs_files = [
            p for p in project_dir.rglob("*.rs")
            if not exclude_parts.intersection(p.parts)
            and not any(part.startswith(".c2r_bindgen_") for part in p.parts)
        ]
        result.files_analyzed = len(rs_files)

        total_unsafe_block_lines = 0
        total_unsafe_fn_lines = 0

        for rs_file in rs_files:
            try:
                content = rs_file.read_text(encoding='utf-8', errors='ignore')

                # 统计总行数
                result.total_lines += content.count('\n') + 1
                global_metrics = analyze_unsafe_global_in_content(content)
                result.code_lines += global_metrics.get("code_lines", 0)
                result.unsafe_keyword_occurrences += global_metrics.get("unsafe_keyword_occurrences", 0)
                result.unsafe_keyword_lines += global_metrics.get("unsafe_keyword_lines", 0)
                result.unsafe_context_lines += global_metrics.get("unsafe_context_lines", 0)
                result.unsafe_total_lines += global_metrics.get("unsafe_total_lines", 0)

                # 精确统计 unsafe 代码行数
                block_lines, fn_lines, block_count, fn_count = count_unsafe_lines_precise(content)
                total_unsafe_block_lines += block_lines
                total_unsafe_fn_lines += fn_lines
                result.unsafe_block_count += block_count
                result.unsafe_fn_count += fn_count

                # 统计 unsafe impl 和 unsafe trait（这些相对少见，用简单方法）
                result.unsafe_impl_count += len(re.findall(r'\bunsafe\s+impl\b', content))
                result.unsafe_trait_count += len(re.findall(r'\bunsafe\s+trait\b', content))

            except Exception:
                pass

        # 计算总 unsafe 项数
        result.total_unsafe_items = (
            result.unsafe_block_count +
            result.unsafe_fn_count +
            result.unsafe_impl_count +
            result.unsafe_trait_count
        )

        # 精确的 unsafe 行数（块内行数 + 函数体内行数）
        result.unsafe_lines_estimate = total_unsafe_block_lines + total_unsafe_fn_lines

        if result.code_lines > 0:
            # unsafe_ratio: 实际 unsafe 代码行数占总代码行数的比例
            result.unsafe_ratio = result.unsafe_lines_estimate / result.code_lines
            result.unsafe_keyword_ratio = result.unsafe_keyword_lines / result.code_lines
            result.unsafe_context_ratio = result.unsafe_context_lines / result.code_lines
            result.unsafe_total_ratio = result.unsafe_total_lines / result.code_lines
            # unsafe_density_per_kloc: 每千行代码的 unsafe 项数
            result.unsafe_density_per_kloc = result.total_unsafe_items / result.code_lines * 1000

    except Exception as e:
        result.error = str(e)

    return result


# ============================================================================
# Cargo Test & Clippy
# ============================================================================

def run_cargo_test(project_dir: Path, timeout: int = 120) -> Dict[str, Any]:
    """
    运行 cargo test 并解析结果

    Args:
        project_dir: 项目目录
        timeout: 超时时间（秒）

    Returns:
        测试结果字典
    """
    result = {
        "executed": False,
        "passed": 0,
        "failed": 0,
        "ignored": 0,
        "total": 0,
        "pass_rate": 0.0,
        "error": None,
        "output": ""
    }

    if not project_dir.exists():
        result["error"] = "Project directory does not exist"
        return result

    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        result["error"] = "Cargo.toml not found"
        return result

    try:
        with tempfile.TemporaryDirectory(prefix="his2trans_cargo_target_") as td:
            proc = subprocess.run(
                ["cargo", "test", "--offline", "--locked", "--no-fail-fast"],
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=timeout,
                env={**os.environ, "RUST_BACKTRACE": "0", "CARGO_TARGET_DIR": td},
            )
        result["executed"] = True
        result["output"] = proc.stdout + proc.stderr

        # 解析测试结果
        # 格式: test result: ok. X passed; Y failed; Z ignored; ...
        # 或: test result: FAILED. X passed; Y failed; Z ignored; ...
        output = proc.stdout + proc.stderr

        # 查找所有测试结果行
        test_result_pattern = r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored'
        matches = re.findall(test_result_pattern, output)

        for match in matches:
            result["passed"] += int(match[0])
            result["failed"] += int(match[1])
            result["ignored"] += int(match[2])

        result["total"] = result["passed"] + result["failed"]
        if result["total"] > 0:
            result["pass_rate"] = result["passed"] / result["total"]

        # 如果没有找到测试结果，检查是否编译失败
        if not matches:
            if "error[" in output or "error:" in output:
                result["error"] = "Compilation failed"
            elif "no test target" in output.lower() or "0 tests" in output:
                result["error"] = "No tests found"

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except Exception as e:
        result["error"] = str(e)

    return result


def run_cargo_clippy(project_dir: Path, timeout: int = 120) -> Dict[str, Any]:
    """
    运行 cargo clippy 并统计警告数

    Args:
        project_dir: 项目目录
        timeout: 超时时间（秒）

    Returns:
        Clippy 结果字典
    """
    result = {
        "executed": False,
        # Count only Clippy lints (code starts with "clippy::") as the primary metric.
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
        "warnings_by_type": {},
        "error": None,
        "compilation_succeeded": False,
        "output": ""
    }

    if not project_dir.exists():
        result["error"] = "Project directory does not exist"
        return result

    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        result["error"] = "Cargo.toml not found"
        return result

    try:
        with tempfile.TemporaryDirectory(prefix="his2trans_cargo_target_") as td:
            proc = subprocess.run(
                ["cargo", "clippy", "--offline", "--locked", "--message-format=json", "--", "-W", "clippy::all"],
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=timeout,
                env={**os.environ, "RUST_BACKTRACE": "0", "CARGO_TARGET_DIR": td},
            )
        result["executed"] = True
        result["output"] = proc.stdout + proc.stderr
        result["compilation_succeeded"] = proc.returncode == 0

        # Parse cargo JSON diagnostics (most accurate; avoids regex pitfalls).
        for line in proc.stdout.splitlines():
            line = line.strip()
            if not line.startswith("{"):
                continue
            try:
                msg = json.loads(line)
            except Exception:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            diag = msg.get("message") or {}
            level = diag.get("level")
            code_obj = diag.get("code") or {}
            code = code_obj.get("code") or "general"

            if level == "warning":
                if isinstance(code, str) and code.startswith("clippy::"):
                    result["warning_count"] += 1
                    result["warnings_by_type"][code] = result["warnings_by_type"].get(code, 0) + 1
                else:
                    result["rustc_warning_count"] += 1
            elif level == "error":
                result["error_count"] += 1

        result["warning_count_total"] = result["warning_count"] + result["rustc_warning_count"]

        # When compilation fails, clippy stats are incomplete; keep counts but mark error.
        if proc.returncode != 0 and result["error_count"] > 0:
            result["error"] = "Compilation failed before clippy analysis"

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except FileNotFoundError:
        result["error"] = "cargo clippy not found (install with: rustup component add clippy)"
    except Exception as e:
        result["error"] = str(e)

    return result


# ============================================================================
# C2R 测试运行
# ============================================================================

# 默认测试文件目录（开源版：放在 his2trans/data 下）
DEFAULT_C2R_TESTS_DIR = _HIS2TRANS_ROOT / "data" / "test_module_rust_tests"


def run_c2r_tests(
    project_dir: Path,
    project_name: str,
    tests_base_dir: Optional[Path] = None,
    timeout: int = 180
) -> Dict[str, Any]:
    """
    运行 C2R 框架的 Rust 单元测试

    将测试文件复制到项目中，修改 main.rs 添加测试模块，然后运行 cargo test。

    Args:
        project_dir: Rust 项目目录（包含 Cargo.toml）
        project_name: 项目名称（用于查找测试文件）
        tests_base_dir: 测试文件基础目录，默认为 DEFAULT_C2R_TESTS_DIR
        timeout: 超时时间（秒）

    Returns:
        测试结果字典
    """
    result = {
        "executed": False,
        "test_file_found": False,
        "test_file_copied": False,
        "main_rs_modified": False,
        "compilation_succeeded": False,
        "tests_passed": 0,
        "tests_failed": 0,
        "tests_ignored": 0,
        "total_tests": 0,
        "pass_rate": 0.0,
        "error": None,
        "error_categories": {},
        "function_test_results": {},
        "output": ""
    }

    if not project_dir.exists():
        result["error"] = "Project directory does not exist"
        return result

    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        result["error"] = "Cargo.toml not found"
        return result

    # 确定测试文件目录
    if tests_base_dir is None:
        tests_base_dir = DEFAULT_C2R_TESTS_DIR

    # 查找测试文件
    test_file = tests_base_dir / project_name / "c2r" / "test.rs"
    if not test_file.exists():
        # 尝试不带后缀的项目名
        if project_name.endswith('_c'):
            alt_name = project_name[:-2]
            test_file = tests_base_dir / alt_name / "c2r" / "test.rs"

    if not test_file.exists():
        result["error"] = f"Test file not found: {test_file}"
        return result

    result["test_file_found"] = True
    # Expected suite size (used as fallback when cargo doesn't print a summary, or compilation fails).
    expected_test_count = 0
    try:
        expected_test_count = len(re.findall(r'(?m)^\s*#\[test\]', test_file.read_text(encoding="utf-8")))
    except Exception:
        pass
    result["expected_total_tests"] = expected_test_count

    # 保存原始文件内容（用于恢复）
    src_dir = project_dir / "src"
    if not src_dir.exists():
        result["error"] = "src directory not found in project"
        return result

    main_rs_file = src_dir / "main.rs"
    dest_test_file = src_dir / "test_c2r.rs"

    original_main_rs = None
    if main_rs_file.exists():
        try:
            original_main_rs = main_rs_file.read_text(encoding='utf-8')
        except Exception:
            pass

    # 保存原始 build.rs 内容（用于恢复）
    build_rs_file = project_dir / "build.rs"
    build_rs_existed = build_rs_file.exists()
    original_build_rs = None
    if build_rs_existed:
        try:
            original_build_rs = build_rs_file.read_text(encoding='utf-8')
        except Exception:
            pass

    try:
        # 复制测试文件
        import shutil
        shutil.copy(test_file, dest_test_file)
        result["test_file_copied"] = True

        # 修改 main.rs 添加测试模块
        if main_rs_file.exists() and original_main_rs is not None:
            if '#[cfg(test)]' not in original_main_rs or 'mod test_c2r' not in original_main_rs:
                test_module_decl = '\n#[cfg(test)]\nmod test_c2r;\n'
                new_main_rs = original_main_rs + test_module_decl
                main_rs_file.write_text(new_main_rs, encoding='utf-8')
                result["main_rs_modified"] = True

        # 检查是否有 C 访问器需要编译
        native_dir = project_dir / "native"
        c2r_accessors_c = native_dir / "c2r_accessors.c"
        if c2r_accessors_c.exists():
            # 创建 build.rs 来编译 C 访问器
            # 查找可能的 include 目录
            include_dirs = []
            # 检查常见的头文件位置（使用绝对路径）
            accessor_h = native_dir / "include" / "c2r_accessors.h"
            if accessor_h.exists():
                include_dirs.append(str((native_dir / "include").resolve()))

            # 查找原始 C 源代码位置（从 c2r_accessors.c 中提取）
            try:
                c_content = c2r_accessors_c.read_text(encoding='utf-8')
                import re as re_local
                include_matches = re_local.findall(r'#include\s+"([^"]+)"', c_content)
                for inc in include_matches:
                    if inc.startswith('/') and Path(inc).exists():
                        inc_dir = str(Path(inc).parent.resolve())
                        if inc_dir not in include_dirs:
                            include_dirs.append(inc_dir)
            except Exception:
                pass

            # 生成 build.rs
            include_stmts = '\n'.join([f'    .include("{d}")' for d in include_dirs])
            build_rs_content = f'''//! Build script to compile C accessor shims
fn main() {{
    cc::Build::new()
        .file("native/c2r_accessors.c")
{include_stmts}
        .compile("c2r_accessors");
    println!("cargo:rerun-if-changed=native/c2r_accessors.c");
}}
'''
            build_rs_file.write_text(build_rs_content, encoding='utf-8')

        # 运行 cargo test
        # 检查是否有 lib.rs 文件来决定是否使用 --lib 标志
        lib_rs_exists = (src_dir / "lib.rs").exists()
        if lib_rs_exists:
            # 有库目标，使用 --lib
            test_cmd = ["cargo", "test", "--offline", "--locked", "--lib", "test_c2r", "--no-fail-fast", "--", "--test-threads=1"]
        else:
            # 只有二进制目标，不使用 --lib
            test_cmd = ["cargo", "test", "--offline", "--locked", "test_c2r", "--no-fail-fast", "--", "--test-threads=1"]

        with tempfile.TemporaryDirectory(prefix=f"his2trans_c2rtests_target_{project_name}_") as td:
            proc = subprocess.run(
                test_cmd,
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=timeout,
                env={**os.environ, "RUST_BACKTRACE": "0", "CARGO_TARGET_DIR": td},
            )
        result["executed"] = True
        result["output"] = proc.stdout + proc.stderr
        output = result["output"]

        # 解析本次执行预计运行的测试总数（可能包含 doc-tests 等多个 target）
        running_test_counts = [int(x) for x in re.findall(r'^running\s+(\d+)\s+tests\b', output, re.MULTILINE)]
        expected_total_tests = sum(running_test_counts) if running_test_counts else 0
        if expected_total_tests == 0 and expected_test_count > 0:
            expected_total_tests = expected_test_count

        # 检查是否编译成功
        # 区分编译错误和运行时错误
        # 编译错误格式: error[E0xxx]: ... 或 "could not compile"
        # 运行时错误格式: "test failed" 或 "target failed"
        compilation_error_patterns = [
            r"error\[E\d+\]:",  # error[E0xxx]:
            r"could not compile",  # cargo compile error
            r"error: linking with",  # linker error
            r"undefined symbol:",  # linker undefined symbol
            r"error occurred in cc-rs",  # cc crate build failure
            r"failed to run custom build command",  # build.rs failure
            r"error: failed to run custom build command",  # build.rs failure (explicit)
        ]
        is_compilation_error = any(re.search(p, output, re.IGNORECASE) for p in compilation_error_patterns)

        # 兜底：若 cargo 返回非 0 且没有任何测试 target 被执行，通常是编译/构建阶段失败
        if not is_compilation_error and proc.returncode != 0:
            has_test_execution_markers = (
                "Running unittests" in output or
                expected_total_tests > 0
            )
            if not has_test_execution_markers:
                is_compilation_error = True

        if is_compilation_error:
            result["compilation_succeeded"] = False
            # 分析错误类型
            result["error_categories"] = categorize_errors(output)
            result["error"] = "Compilation failed"
        else:
            result["compilation_succeeded"] = True
            # 检查是否有运行时错误（测试崩溃）
            if "test failed" in output or "target failed" in output:
                # 测试运行了但有失败或崩溃
                if "signal: " in output:
                    result["error"] = "Test crashed with signal"

        # 解析测试结果
        test_result_pattern = r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored'
        matches = re.findall(test_result_pattern, output)

        for match in matches:
            result["tests_passed"] += int(match[1])
            result["tests_failed"] += int(match[2])
            result["tests_ignored"] += int(match[3])
        if matches:
            result["total_tests"] = result["tests_passed"] + result["tests_failed"]

        # 解析每个测试的结果
        # 格式: test test_c2r::test_xxx ... ok/FAILED
        test_line_pattern = r'^test\s+test_c2r::([\w:]+)\s+\.\.\.\s+(ok|FAILED|ignored)$'
        test_matches = re.findall(test_line_pattern, output, re.MULTILINE)
        for test_name, status in test_matches:
            result["function_test_results"][test_name] = status.lower()

        # 若没有汇总行（例如测试崩溃），从逐条测试输出推断并尽量给出“总数”分母
        if not matches:
            statuses = list(result["function_test_results"].values())
            ok_cnt = sum(1 for s in statuses if s == "ok")
            ignored_cnt = sum(1 for s in statuses if s == "ignored")

            result["tests_passed"] = ok_cnt
            result["tests_ignored"] = ignored_cnt

            if expected_total_tests > 0:
                # running N tests 包含 ignored；总数（分母）默认排除 ignored
                total_non_ignored = max(expected_total_tests - ignored_cnt, 0)
                result["total_tests"] = total_non_ignored

                # 若 cargo 非 0 且没有完整的 summary，视为提前终止：未通过的都算失败
                if proc.returncode != 0:
                    result["tests_failed"] = max(total_non_ignored - ok_cnt, 0)
            else:
                # 无 running N tests 线索时，退回到已观测到的 ok/FAILED（忽略未输出的测试）
                failed_cnt = sum(1 for s in statuses if s == "failed")
                result["tests_failed"] = failed_cnt
                result["total_tests"] = ok_cnt + failed_cnt

        # 计算通过率
        if result["total_tests"] <= 0:
            result["pass_rate"] = 0.0
        else:
            result["pass_rate"] = result["tests_passed"] / result["total_tests"]

        # 若 cargo 非 0 且既不是编译失败，也没有 test result 汇总，补一个更直观的错误说明
        if result["compilation_succeeded"] and proc.returncode != 0 and not matches and result["error"] is None:
            result["error"] = "Test terminated unexpectedly"

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except Exception as e:
        result["error"] = str(e)
    finally:
        # 清理：恢复原始文件并删除测试文件
        try:
            if original_main_rs is not None and main_rs_file.exists():
                main_rs_file.write_text(original_main_rs, encoding='utf-8')
            if build_rs_existed:
                if original_build_rs is not None and build_rs_file.exists():
                    build_rs_file.write_text(original_build_rs, encoding='utf-8')
            else:
                if build_rs_file.exists():
                    build_rs_file.unlink()
            if dest_test_file.exists():
                dest_test_file.unlink()
        except Exception:
            pass

    return result


# ============================================================================
# 增量编译验证
# ============================================================================

import tempfile
import shutil


def verify_incremental_compilation(
    rust_project_dir: Path,
    project_name: str,
    timeout: int = 30
) -> Dict[str, Any]:
    """
    验证增量编译成功率：逐个将翻译后的函数集成到项目中，检查能否编译通过

    C2R 的做法：
    1. 复制整个 Rust 项目到临时目录
    2. 将所有函数体替换为 unimplemented!()
    3. 逐个恢复函数的原始代码，测试编译
    4. 记录每个函数是否能成功编译

    Args:
        rust_project_dir: Rust 项目目录（包含 Cargo.toml）
        project_name: 项目名称
        timeout: 单个函数编译超时时间（秒）

    Returns:
        {
            "total_functions": 10,
            "compiled_functions": 8,
            "compile_rate": 0.8,
            "functions_detail": {
                "func_name": {"compiled": True/False, "error": "..."}
            }
        }
    """
    result = {
        "total_functions": 0,
        # NOTE: `compiled_functions` / `compile_rate` are the paper metric:
        #   ICompRate = (# LLM-produced functions that compile) / (all functions)
        # We also keep `compiled_functions_all` / `compile_rate_all` for debugging.
        "compiled_functions": 0,
        "compile_rate": 0.0,
        "compiled_functions_all": 0,
        "compile_rate_all": 0.0,
        # Breakdown of function sources in the final Rust project.
        "llm_functions": 0,
        "c2rust_fallback_functions": 0,
        "unimplemented_functions": 0,
        "llm_compiled_functions": 0,
        "functions_detail": {},
        "baseline_compilation_succeeded": None,
        "baseline_error": None,
        "skeleton_compilation_succeeded": None,
        "skeleton_error": None,
        "error": None
    }

    if not rust_project_dir.exists():
        result["error"] = "Rust project directory not found"
        return result

    cargo_toml = rust_project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        result["error"] = "Cargo.toml not found"
        return result

    src_dir = rust_project_dir / "src"
    if not src_dir.exists():
        result["error"] = "src directory not found"
        return result

    # 收集所有 .rs 文件及其函数
    rs_files = list(src_dir.rglob("*.rs"))
    if not rs_files:
        result["error"] = "No .rs files found"
        return result

    # 从所有文件中提取函数信息
    all_functions: List[Tuple[Path, str, str]] = []  # (file_path, func_name, func_code)
    file_contents: Dict[str, str] = {}  # relative_path -> content

    for rs_file in rs_files:
        try:
            content = rs_file.read_text(encoding='utf-8')
            rel_path = str(rs_file.relative_to(src_dir))
            file_contents[rel_path] = content

            # 提取所有带函数体的函数（支持函数指针等“嵌套括号”签名）
            for fn_name, fn_kw_pos, _body_start, body_end in _fnscan_iter_function_items(content):
                if fn_name in ('main', 'test'):
                    continue
                line_start = content.rfind('\n', 0, fn_kw_pos)
                item_start = 0 if line_start == -1 else line_start + 1
                func_code = content[item_start:body_end]
                all_functions.append((rs_file, fn_name, func_code))

        except Exception:
            continue

    if not all_functions:
        result["error"] = "No functions found in project"
        return result

    result["total_functions"] = len(all_functions)

    def _is_placeholder_body(body_with_braces: str) -> bool:
        """
        Detect the *pure placeholder* bodies we treat as non-LLM fallbacks.
        We only match the whole body being a single placeholder statement, to avoid
        false positives where `unimplemented!()` appears on a rare branch.
        """
        body = body_with_braces.strip()
        if body.startswith("{") and body.endswith("}"):
            body = body[1:-1].strip()
        if not body:
            # Empty body could be a valid no-op implementation; don't treat it as a placeholder.
            return False
        if re.match(r"^\s*unimplemented!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
            return True
        if re.match(r"^\s*todo!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
            return True
        if re.match(r"^\s*unreachable!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
            return True
        if re.match(
            r"^\s*panic!\s*\(\s*['\"].*not\s*implement.*['\"]\s*\)\s*;?\s*$",
            body,
            re.IGNORECASE | re.DOTALL,
        ):
            return True
        return False

    def _classify_function_source(func_code: str) -> str:
        """
        Classify function bodies in the *final* project:
          - llm: LLM-produced code
          - c2rust_fallback: our explicit C2Rust fallback wrapper
          - unimplemented: placeholder fallback (unimplemented!/todo!/etc.)
        """
        if "__c2rust_fallback" in func_code or "C2Rust fallback" in func_code:
            return "c2rust_fallback"
        brace_pos = func_code.find("{")
        body = func_code[brace_pos:] if brace_pos != -1 else ""
        if _is_placeholder_body(body):
            return "unimplemented"
        return "llm"

    # Pre-compute source breakdown (used even when baseline/skeleton fails).
    sources: Dict[str, str] = {}
    for rs_file, fn_name, func_code in all_functions:
        rel_path = str(rs_file.relative_to(src_dir))
        detail_key = f"{rel_path}::{fn_name}"
        sources[detail_key] = _classify_function_source(func_code)
    result["llm_functions"] = sum(1 for s in sources.values() if s == "llm")
    result["c2rust_fallback_functions"] = sum(1 for s in sources.values() if s == "c2rust_fallback")
    result["unimplemented_functions"] = sum(1 for s in sources.values() if s == "unimplemented")

    # 创建临时目录进行测试
    try:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            # 复制整个项目到临时目录
            temp_project = tmpdir / project_name
            shutil.copytree(rust_project_dir, temp_project)

            temp_src_dir = temp_project / "src"

            # 先确认“原始项目”本身是否可编译，避免把项目自身失败误判为增量验证失败
            baseline_timeout = max(timeout, 120)
            try:
                baseline_proc = subprocess.run(
                    ["cargo", "check", "--offline"],
                    cwd=temp_project,
                    capture_output=True,
                    text=True,
                    timeout=baseline_timeout,
                    env={**os.environ, "RUSTFLAGS": "-Awarnings"},
                )
                if baseline_proc.returncode != 0:
                    result["baseline_compilation_succeeded"] = False
                    result["baseline_error"] = (baseline_proc.stderr or baseline_proc.stdout or "")[:1500]
                    result["error"] = "Baseline project does not compile"
                    return result
                result["baseline_compilation_succeeded"] = True
            except subprocess.TimeoutExpired:
                result["baseline_compilation_succeeded"] = False
                result["baseline_error"] = f"Baseline cargo check timeout after {baseline_timeout}s"
                result["error"] = "Baseline project does not compile"
                return result

            # 将所有函数体替换为 unimplemented!()
            stubbed_contents: Dict[str, str] = {}
            for rel_path, content in file_contents.items():
                stubbed_content = stub_all_functions_in_content(content)
                stubbed_contents[rel_path] = stubbed_content
                (temp_src_dir / rel_path).write_text(stubbed_content, encoding='utf-8')

            # 确认“全 stub 的骨架”可编译；若不通过，说明 stub 逻辑不适用于该项目（或项目本身有函数签名层面的错误）
            skeleton_timeout = max(timeout, 120)
            try:
                skeleton_proc = subprocess.run(
                    ["cargo", "check", "--offline"],
                    cwd=temp_project,
                    capture_output=True,
                    text=True,
                    timeout=skeleton_timeout,
                    env={**os.environ, "RUSTFLAGS": "-Awarnings"},
                )
                if skeleton_proc.returncode != 0:
                    result["skeleton_compilation_succeeded"] = False
                    result["skeleton_error"] = (skeleton_proc.stderr or skeleton_proc.stdout or "")[:1500]
                    result["error"] = "Stubbed skeleton does not compile"
                    return result
                result["skeleton_compilation_succeeded"] = True
            except subprocess.TimeoutExpired:
                result["skeleton_compilation_succeeded"] = False
                result["skeleton_error"] = f"Skeleton cargo check timeout after {skeleton_timeout}s"
                result["error"] = "Stubbed skeleton does not compile"
                return result

            # 逐个测试每个函数
            compiled_count_all = 0
            llm_compiled_count = 0
            for rs_file, func_name, func_code in all_functions:
                rel_path = str(rs_file.relative_to(src_dir))
                temp_rs_file = temp_src_dir / rel_path
                detail_key = f"{rel_path}::{func_name}"
                source = sources.get(detail_key) or _classify_function_source(func_code)

                # 读取当前（已 stub）的文件内容
                try:
                    current_content = temp_rs_file.read_text(encoding='utf-8')
                except Exception:
                    result["functions_detail"][detail_key] = {
                        "compiled": False,
                        "source": source,
                        "counted_in_icomp": False,
                        "error": "Could not read file"
                    }
                    continue

                # 恢复这个函数的原始代码
                restored_content = restore_function_in_content(
                    current_content, func_name, func_code
                )
                temp_rs_file.write_text(restored_content, encoding='utf-8')

                # 运行 cargo check
                try:
                    proc = subprocess.run(
                        ["cargo", "check", "--offline"],
                        cwd=temp_project,
                        capture_output=True,
                        text=True,
                        timeout=timeout,
                        env={**os.environ, "RUSTFLAGS": "-Awarnings"}
                    )

                    if proc.returncode == 0:
                        counted = source == "llm"
                        result["functions_detail"][detail_key] = {
                            "compiled": True,
                            "source": source,
                            "counted_in_icomp": counted,
                        }
                        compiled_count_all += 1
                        if counted:
                            llm_compiled_count += 1
                        # 保持这个函数的代码（成功的函数保留）
                    else:
                        # 编译失败，回滚这个函数为 stub
                        temp_rs_file.write_text(current_content, encoding='utf-8')
                        error_msg = proc.stderr[:500] if proc.stderr else "Unknown error"
                        result["functions_detail"][detail_key] = {
                            "compiled": False,
                            "source": source,
                            "counted_in_icomp": False,
                            "error": error_msg
                        }

                except subprocess.TimeoutExpired:
                    temp_rs_file.write_text(current_content, encoding='utf-8')
                    result["functions_detail"][detail_key] = {
                        "compiled": False,
                        "source": source,
                        "counted_in_icomp": False,
                        "error": "Compilation timeout"
                    }
                except Exception as e:
                    temp_rs_file.write_text(current_content, encoding='utf-8')
                    result["functions_detail"][detail_key] = {
                        "compiled": False,
                        "source": source,
                        "counted_in_icomp": False,
                        "error": str(e)
                    }

            result["compiled_functions_all"] = compiled_count_all
            result["compile_rate_all"] = compiled_count_all / len(all_functions) if all_functions else 0.0
            result["llm_compiled_functions"] = llm_compiled_count
            result["compiled_functions"] = llm_compiled_count
            result["compile_rate"] = llm_compiled_count / len(all_functions) if all_functions else 0.0

    except Exception as e:
        result["error"] = str(e)
        import traceback
        result["traceback"] = traceback.format_exc()

    return result


# ============================================================================
# 项目分析
# ============================================================================

def find_translation_stats(project_dir: Path, llm_name: str) -> Optional[Path]:
    """
    查找 translation_stats.json 文件

    Args:
        project_dir: 项目目录
        llm_name: LLM 名称

    Returns:
        translation_stats.json 文件路径，如果不存在返回 None
    """
    # 主要路径（增量工作目录）
    project_name = project_dir.name
    primary_path = (
        project_dir / "workspace" / "incremental_work" /
        project_name / f"translate_by_{llm_name}" / "translation_stats.json"
    )

    if primary_path.exists():
        return primary_path

    # 兼容：最终项目目录
    final_path = (
        project_dir / "workspace" / "final_projects" /
        project_name / f"translate_by_{llm_name}" / "translation_stats.json"
    )
    if final_path.exists():
        return final_path

    # 尝试其他可能的路径（尽量优先增量目录）
    for path in project_dir.rglob("translation_stats.json"):
        if "incremental_work" in str(path):
            return path
        if "final_projects" in str(path):
            return path

    return None


def analyze_project(
    project_name: str,
    intermediate_dir: Path,
    llm_name: str = "qwen3_coder",
    analyze_errors: bool = True,
    analyze_unsafe: bool = True,
    run_tests: bool = False,
    run_clippy: bool = False,
    run_c2r_tests_flag: bool = False,
    c2r_tests_dir: Optional[Path] = None,
    verify_incremental: bool = False
) -> Optional[ProjectResult]:
    """
    分析单个项目的翻译结果

    Args:
        project_name: 项目名称
        intermediate_dir: intermediate 目录路径
        llm_name: LLM 名称
        analyze_errors: 是否分析错误
        analyze_unsafe: 是否分析 unsafe 代码
        run_tests: 是否运行 cargo test
        run_clippy: 是否运行 cargo clippy
        run_c2r_tests_flag: 是否运行 C2R 单元测试
        c2r_tests_dir: C2R 测试文件目录

    Returns:
        ProjectResult 对象，如果项目不存在返回 None
    """
    project_dir = intermediate_dir / project_name
    if not project_dir.exists():
        return None

    # 查找 translation_stats.json
    stats_path = find_translation_stats(project_dir, llm_name)
    if not stats_path:
        return None

    try:
        with open(stats_path, 'r', encoding='utf-8') as f:
            stats = json.load(f)
    except Exception as e:
        # Keep console output clean for open-source runs; treat as missing.
        return None

    # 创建结果对象
    result = ProjectResult(project_name=project_name)

    # 填充基本统计
    result.total_functions = stats.get("total", 0)
    result.translated = stats.get("translated", 0)
    result.compiled_total = stats.get("compiled", 0)
    result.compiled_after_repair = stats.get("repaired", 0)
    result.compiled_direct = result.compiled_total - result.compiled_after_repair
    result.c2rust_fallback = stats.get("c2rust_fallback", 0)
    result.failed_reverted = stats.get("failed", 0)
    result.skipped = stats.get("skipped", 0)
    result.injection_failed = stats.get("injection_failed", 0)
    result.still_unimplemented = stats.get("still_unimplemented", 0)
    result.timeout_failed = stats.get("timeout_failed", 0)

    # 计算通过率
    if result.total_functions > 0:
        result.pass_rate = result.compiled_total / result.total_functions
        result.direct_pass_rate = result.compiled_direct / result.total_functions

    # 分析 repair_history 中的错误信息
    if analyze_errors:
        repair_history_dir = (
            project_dir / "workspace" / "repair_history" /
            project_name / f"translate_by_{llm_name}"
        )
        if repair_history_dir.exists():
            result.error_analysis = parse_repair_errors(repair_history_dir)

    # 获取 Rust 项目目录
    # 以 translation_stats.json 所在目录为准，兼容不同输出结构（incremental_work / final_projects）
    rust_project_dir = stats_path.parent

    # 分析 unsafe 代码
    if analyze_unsafe:
        if rust_project_dir.exists():
            result.unsafe_analysis = analyze_unsafe_code(rust_project_dir)

    # 运行 cargo test
    if run_tests and rust_project_dir.exists():
        result.cargo_test_result = run_cargo_test(rust_project_dir)

    # 运行 cargo clippy
    if run_clippy and rust_project_dir.exists():
        result.cargo_clippy_result = run_cargo_clippy(rust_project_dir)

    # 运行 C2R 单元测试
    if run_c2r_tests_flag and rust_project_dir.exists():
        result.c2r_test_result = run_c2r_tests(
            rust_project_dir, project_name, c2r_tests_dir
        )

    # 验证增量编译成功率
    if verify_incremental and rust_project_dir.exists():
        result.incremental_compilation = verify_incremental_compilation(
            rust_project_dir, project_name
        )

    return result


def analyze_project_wrapper(args: Tuple) -> Tuple[str, Optional[ProjectResult]]:
    """并行分析的包装函数"""
    project_name, intermediate_dir, llm_name, analyze_errors, analyze_unsafe, run_tests, run_clippy, run_c2r_tests_flag, c2r_tests_dir, verify_incremental = args
    result = analyze_project(
        project_name, intermediate_dir, llm_name,
        analyze_errors, analyze_unsafe, run_tests, run_clippy,
        run_c2r_tests_flag, c2r_tests_dir, verify_incremental
    )
    return project_name, result


# ============================================================================
# 运行分析
# ============================================================================

def load_summary_report(results_dir: Path) -> Optional[Dict[str, Any]]:
    """加载 summary report"""
    summary_report_path = results_dir / "translation_summary_report.json"
    if summary_report_path.exists():
        try:
            with open(summary_report_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception:
            pass
    return None


def analyze_run(
    run_dir: Path,
    llm_name: str = "qwen3_coder",
    parallel: bool = True,
    max_workers: int = 4,
    analyze_errors: bool = True,
    analyze_unsafe: bool = True,
    run_tests: bool = False,
    run_clippy: bool = False,
    run_c2r_tests_flag: bool = False,
    c2r_tests_dir: Optional[Path] = None,
    verify_incremental: bool = False
) -> Dict[str, Any]:
    """
    分析整个运行目录的翻译结果

    Args:
        run_dir: 运行目录
        llm_name: LLM 名称
        parallel: 是否并行处理
        max_workers: 最大并行数
        analyze_errors: 是否分析错误
        analyze_unsafe: 是否分析 unsafe 代码
        run_tests: 是否运行 cargo test
        run_clippy: 是否运行 cargo clippy
        run_c2r_tests_flag: 是否运行 C2R 单元测试
        c2r_tests_dir: C2R 测试文件目录

    Returns:
        分析结果字典
    """
    intermediate_dir = run_dir / "intermediate"
    results_dir = run_dir / "results"

    # 读取已生成的 summary report (如果存在)
    summary_report = load_summary_report(results_dir)

    # 初始化结果
    result: Dict[str, Any] = {
        "run_dir": str(run_dir),
        "timestamp": datetime.now().isoformat(),
        "llm_name": llm_name,
        "projects": {},
        "summary": {}
    }

    if not intermediate_dir.exists():
        result["summary"] = AnalysisSummary().to_dict()
        return result

    # 获取项目列表
    project_dirs = [
        d for d in intermediate_dir.iterdir()
        if d.is_dir() and d.name not in ['logs', 'cache', 'shared']
    ]

    # 分析每个项目
    projects_results: Dict[str, ProjectResult] = {}

    if parallel and len(project_dirs) > 1:
        # 并行处理
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            args_list = [
                (d.name, intermediate_dir, llm_name, analyze_errors, analyze_unsafe, run_tests, run_clippy, run_c2r_tests_flag, c2r_tests_dir, verify_incremental)
                for d in project_dirs
            ]
            futures = {
                executor.submit(analyze_project_wrapper, args): args[0]
                for args in args_list
            }

            for future in as_completed(futures):
                project_name, project_result = future.result()
                if project_result:
                    projects_results[project_name] = project_result
    else:
        # 顺序处理
        for project_dir in sorted(project_dirs):
            project_result = analyze_project(
                project_dir.name, intermediate_dir, llm_name,
                analyze_errors, analyze_unsafe, run_tests, run_clippy,
                run_c2r_tests_flag, c2r_tests_dir, verify_incremental
            )
            if project_result:
                projects_results[project_dir.name] = project_result

    # 添加语义评估结果
    if summary_report:
        for proj_detail in summary_report.get("projects_detail", []):
            proj_name = proj_detail.get("project")
            if proj_name in projects_results:
                projects_results[proj_name].semantic_score = proj_detail.get("semantic_score")
                projects_results[proj_name].semantic_summary = proj_detail.get("semantic_summary")

    # 计算汇总统计
    summary = AnalysisSummary()
    error_categories_total: Dict[str, int] = defaultdict(int)
    total_code_lines = 0
    total_unsafe_keyword_occurrences = 0
    total_unsafe_keyword_lines = 0
    total_unsafe_context_lines = 0
    total_unsafe_total_lines = 0
    total_unsafe_items = 0
    total_unsafe_blocks = 0
    total_unsafe_fns = 0
    total_unsafe_impls = 0
    total_unsafe_traits = 0
    total_unsafe_lines_estimate = 0

    # 测试汇总统计
    total_tests_passed = 0
    total_tests_failed = 0
    total_tests = 0
    projects_with_tests = 0

    # Clippy 汇总统计
    total_clippy_warnings = 0
    total_rustc_warnings = 0
    total_all_warnings = 0
    total_clippy_errors = 0
    projects_with_clippy = 0

    # C2R 测试汇总统计
    c2r_tests_executed = 0
    c2r_tests_compilation_succeeded = 0
    c2r_tests_passed = 0
    c2r_tests_failed = 0
    c2r_tests_total = 0

    # 增量编译统计
    incremental_total_functions = 0
    incremental_compiled_functions = 0

    for project_name, project_result in sorted(projects_results.items()):
        result["projects"][project_name] = project_result.to_dict()

        summary.total_projects += 1
        summary.total_functions += project_result.total_functions
        summary.successful_functions += project_result.compiled_total
        summary.direct_pass_functions += project_result.compiled_direct
        summary.repaired_functions += project_result.compiled_after_repair
        summary.failed_functions += project_result.failed_reverted
        summary.c2rust_fallback_functions += project_result.c2rust_fallback

        # 汇总错误类别
        if project_result.error_analysis:
            for cat, count in project_result.error_analysis.categories.items():
                error_categories_total[cat] += count

        # 汇总 unsafe 统计
        if project_result.unsafe_analysis:
            ua = project_result.unsafe_analysis
            total_code_lines += ua.code_lines
            total_unsafe_keyword_occurrences += ua.unsafe_keyword_occurrences
            total_unsafe_keyword_lines += ua.unsafe_keyword_lines
            total_unsafe_context_lines += ua.unsafe_context_lines
            total_unsafe_total_lines += ua.unsafe_total_lines
            total_unsafe_items += ua.total_unsafe_items
            total_unsafe_blocks += ua.unsafe_block_count
            total_unsafe_fns += ua.unsafe_fn_count
            total_unsafe_impls += ua.unsafe_impl_count
            total_unsafe_traits += ua.unsafe_trait_count
            total_unsafe_lines_estimate += ua.unsafe_lines_estimate

        # 汇总测试结果
        if project_result.cargo_test_result:
            tr = project_result.cargo_test_result
            if tr.get("executed"):
                projects_with_tests += 1
                total_tests_passed += tr.get("passed", 0)
                total_tests_failed += tr.get("failed", 0)
                total_tests += tr.get("total", 0)

        # 汇总 clippy 结果（使用 cargo JSON message-format，分离 clippy:: 与 rustc warnings）
        if project_result.cargo_clippy_result:
            cr = project_result.cargo_clippy_result
            if cr.get("executed"):
                projects_with_clippy += 1
                cw = cr.get("warning_count", 0)
                rw = cr.get("rustc_warning_count", 0)
                tw = cr.get("warning_count_total", cw + rw)
                total_clippy_warnings += cw
                total_rustc_warnings += rw
                total_all_warnings += tw
                total_clippy_errors += cr.get("error_count", 0)

        # 汇总 C2R 测试结果
        if project_result.c2r_test_result:
            c2r_tr = project_result.c2r_test_result
            if c2r_tr.get("executed"):
                c2r_tests_executed += 1
                if c2r_tr.get("compilation_succeeded"):
                    c2r_tests_compilation_succeeded += 1
                c2r_tests_passed += c2r_tr.get("tests_passed", 0)
                c2r_tests_failed += c2r_tr.get("tests_failed", 0)
                c2r_tests_total += c2r_tr.get("total_tests", 0)

        # 汇总增量编译结果
        #
        # 注意：即使项目 baseline / skeleton 失败，`verify_incremental_compilation` 也会尽量填充
        # total_functions / compiled_functions（通常 compiled=0），以避免把失败项目“从分母里消失”，
        # 导致整体成功率虚高（例如 100% 但某个项目完全不可编译）。
        if project_result.incremental_compilation:
            ic = project_result.incremental_compilation
            ic_total = ic.get("total_functions", 0)
            if ic_total > 0:
                incremental_total_functions += ic_total
                incremental_compiled_functions += ic.get("compiled_functions", 0)

    # 计算比率
    if summary.total_functions > 0:
        summary.overall_pass_rate = summary.successful_functions / summary.total_functions
        summary.direct_pass_rate = summary.direct_pass_functions / summary.total_functions

    summary.error_categories = dict(error_categories_total)
    summary.unsafe_summary = {
        "total_code_lines": total_code_lines,
        "total_unsafe_items": total_unsafe_items,
        "unsafe_blocks": total_unsafe_blocks,
        "unsafe_functions": total_unsafe_fns,
        "unsafe_impls": total_unsafe_impls,
        "unsafe_traits": total_unsafe_traits,
        # 旧指标：只统计 unsafe 块/unsafe fn 体内的“实际行数”
        "unsafe_lines_estimate": total_unsafe_lines_estimate,
        "unsafe_ratio_estimate": total_unsafe_lines_estimate / total_code_lines if total_code_lines > 0 else 0,
        # 全局指标：关键字行、作用域行及并集（更接近“所有 unsafe 相关代码行”）
        "total_unsafe_keyword_occurrences": total_unsafe_keyword_occurrences,
        "total_unsafe_keyword_lines": total_unsafe_keyword_lines,
        "unsafe_keyword_ratio": total_unsafe_keyword_lines / total_code_lines if total_code_lines > 0 else 0,
        "total_unsafe_context_lines": total_unsafe_context_lines,
        "unsafe_context_ratio": total_unsafe_context_lines / total_code_lines if total_code_lines > 0 else 0,
        "total_unsafe_total_lines": total_unsafe_total_lines,
        "unsafe_total_ratio": total_unsafe_total_lines / total_code_lines if total_code_lines > 0 else 0,
        "unsafe_density_per_kloc": total_unsafe_items / total_code_lines * 1000 if total_code_lines > 0 else 0
    }

    # 添加语义评估摘要
    if summary_report and "semantic_evaluation" in summary_report:
        summary.semantic_evaluation = summary_report["semantic_evaluation"]

    # 添加测试汇总
    if run_tests:
        summary.test_summary = {
            "projects_with_tests": projects_with_tests,
            "total_tests": total_tests,
            "tests_passed": total_tests_passed,
            "tests_failed": total_tests_failed,
            "overall_test_pass_rate": total_tests_passed / total_tests if total_tests > 0 else 0
        }

    # 添加 clippy 汇总
    if run_clippy:
        summary.clippy_summary = {
            "projects_analyzed": projects_with_clippy,
            # Backward-compatible keys (these now mean: Clippy warnings only).
            "total_warnings": total_clippy_warnings,
            "avg_warnings_per_project": total_clippy_warnings / projects_with_clippy if projects_with_clippy > 0 else 0,
            # New, clearer breakdown.
            "total_clippy_warnings": total_clippy_warnings,
            "total_rustc_warnings": total_rustc_warnings,
            "total_warnings_including_rustc": total_all_warnings,
            "total_errors": total_clippy_errors,
            "avg_total_warnings_per_project": total_all_warnings / projects_with_clippy if projects_with_clippy > 0 else 0,
        }

    # 添加 C2R 测试汇总
    if run_c2r_tests_flag:
        summary.c2r_test_summary = {
            "projects_executed": c2r_tests_executed,
            "projects_compilation_succeeded": c2r_tests_compilation_succeeded,
            "compilation_success_rate": c2r_tests_compilation_succeeded / c2r_tests_executed if c2r_tests_executed > 0 else 0,
            "total_tests": c2r_tests_total,
            "tests_passed": c2r_tests_passed,
            "tests_failed": c2r_tests_failed,
            "overall_test_pass_rate": c2r_tests_passed / c2r_tests_total if c2r_tests_total > 0 else 0
        }

    # 添加增量编译汇总
    if verify_incremental:
        summary.incremental_compilation_summary = {
            "total_functions": incremental_total_functions,
            "compiled_functions": incremental_compiled_functions,
            "compile_rate": incremental_compiled_functions / incremental_total_functions if incremental_total_functions > 0 else 0
        }

    result["summary"] = summary.to_dict()
    return result


# ============================================================================
# 报告输出
# ============================================================================

def format_percentage(value: float, width: int = 10) -> str:
    """格式化百分比"""
    return f"{value:.1%}".ljust(width)


def format_number(value: int, width: int = 8) -> str:
    """格式化数字"""
    return str(value).ljust(width)


def _format_rate(numer: int, denom: int) -> str:
    """Format a ratio as a headline percentage: 'xx.x%'."""
    if denom <= 0:
        return "-"
    return f"{(numer / denom) * 100:.1f}%"


def print_key_metrics_summary_table(analysis: Dict[str, Any]) -> None:
    """
    Print a unified key-metrics table, similar to the OHOS(test5) summary format:
    - incremental compilation rate
    - external test pass rate (prefer cargo test if available; else C2R tests)
    - clippy warnings total with breakdown (clippy/rustc)
    - unsafe rate (unsafe_total_lines / code_lines)
    """
    projects: Dict[str, Any] = analysis.get("projects", {}) or {}
    if not projects:
        return

    # Column widths (kept close to the example format; project column adapts to names).
    proj_w = max(40, max(len(p) for p in projects.keys()))
    inc_w = 22
    test_w = 22
    clippy_w = 18
    unsafe_w = 22
    line_w = proj_w + 1 + inc_w + 1 + test_w + 1 + clippy_w + 1 + unsafe_w

    run_dir_str = str(analysis.get("run_dir", "") or "")
    run_label = Path(run_dir_str).name if run_dir_str else "Run"

    inc_compiled_sum = 0
    inc_total_sum = 0

    test_passed_sum = 0
    test_total_sum = 0

    clippy_total_sum = 0
    clippy_only_sum = 0
    rustc_warn_sum = 0
    has_any_clippy = False

    unsafe_lines_sum = 0
    code_lines_sum = 0

    print("\n" + "=" * line_w)
    print(f"{run_label} 关键指标汇总（统一格式） - Ours")
    print("=" * line_w)
    # Match OHOS(test5) unified table style: project left-aligned; metrics right-aligned.
    print(
        f"{'项目':<{proj_w}} "
        f"{'增量编译率':>{inc_w}} "
        f"{'外部测试通过率':>{test_w}} "
        f"{'Clippy警告(总)':>{clippy_w}} "
        f"{'Unsafe率':>{unsafe_w}}"
    )
    print("-" * line_w)

    for proj_name, proj_data in iter_projects_in_display_order(projects):
        # Incremental compilation (verify-incremental).
        ic = proj_data.get("incremental_compilation") or {}
        ic_total = int(ic.get("total_functions", 0) or 0)
        ic_compiled = int(ic.get("compiled_functions", 0) or 0)
        inc_str = _format_rate(ic_compiled, ic_total)
        if ic_total > 0:
            inc_total_sum += ic_total
            inc_compiled_sum += ic_compiled

        # External tests: prefer cargo test when it has actual tests; else use C2R tests.
        passed = 0
        total = 0
        ct = proj_data.get("cargo_test_result") or {}
        if ct.get("executed") and int(ct.get("total", 0) or 0) > 0:
            passed = int(ct.get("passed", 0) or 0)
            total = int(ct.get("total", 0) or 0)
        else:
            c2r = proj_data.get("c2r_test_result") or {}
            if c2r.get("executed"):
                passed = int(c2r.get("tests_passed", 0) or 0)
                total = int(c2r.get("total_tests", 0) or 0)
                if total <= 0:
                    # Fall back to the expected suite size (parsed from test.rs) when available.
                    total = int(c2r.get("expected_total_tests", 0) or 0)
        test_str = _format_rate(passed, total)
        if total > 0:
            test_total_sum += total
            test_passed_sum += passed

        # Clippy warnings.
        cl = proj_data.get("cargo_clippy_result") or {}
        if cl.get("executed"):
            has_any_clippy = True
            clippy_only = int(cl.get("warning_count", 0) or 0)
            rustc_warn = int(cl.get("rustc_warning_count", 0) or 0)
            total_warn = int(cl.get("warning_count_total", clippy_only + rustc_warn) or 0)
            # Only keep the total warning count (no clippy/rustc breakdown) for the unified table.
            clippy_str = str(total_warn)
            clippy_total_sum += total_warn
            clippy_only_sum += clippy_only
            rustc_warn_sum += rustc_warn
        else:
            clippy_str = "-"

        # Unsafe rate.
        ua = proj_data.get("unsafe_analysis") or {}
        code_lines = int(ua.get("code_lines", 0) or 0)
        unsafe_lines = int(ua.get("unsafe_total_lines", 0) or 0)
        unsafe_str = _format_rate(unsafe_lines, code_lines)
        if code_lines > 0:
            code_lines_sum += code_lines
            unsafe_lines_sum += unsafe_lines

        print(
            f"{proj_name:<{proj_w}} "
            f"{inc_str:>{inc_w}} "
            f"{test_str:>{test_w}} "
            f"{clippy_str:>{clippy_w}} "
            f"{unsafe_str:>{unsafe_w}}"
        )

    print("-" * line_w)
    total_inc_str = _format_rate(inc_compiled_sum, inc_total_sum)
    total_test_str = _format_rate(test_passed_sum, test_total_sum)
    total_clippy_str = str(clippy_total_sum) if has_any_clippy else "-"
    total_unsafe_str = _format_rate(unsafe_lines_sum, code_lines_sum)
    print(
        f"{'总计':<{proj_w}} "
        f"{total_inc_str:>{inc_w}} "
        f"{total_test_str:>{test_w}} "
        f"{total_clippy_str:>{clippy_w}} "
        f"{total_unsafe_str:>{unsafe_w}}"
    )


def print_analysis_report(analysis: Dict[str, Any]) -> None:
    """
    打印分析报告

    Args:
        analysis: 分析结果字典
    """
    summary = analysis["summary"]

    print("\n" + "=" * 80)
    print("C2R 翻译结果分析报告")
    print("=" * 80)
    print(f"运行目录: {analysis['run_dir']}")
    print(f"LLM: {analysis.get('llm_name', 'N/A')}")
    print(f"分析时间: {analysis['timestamp']}")

    print("\n" + "-" * 80)
    print("总体统计")
    print("-" * 80)
    print(f"总项目数:           {summary['total_projects']}")
    print(f"总函数数:           {summary['total_functions']}")
    print(f"编译成功:           {summary['successful_functions']} ({summary['overall_pass_rate']:.2%})")
    print(f"  - 一次通过:       {summary['direct_pass_functions']} ({summary['direct_pass_rate']:.2%})")
    print(f"  - 修复后通过:     {summary['repaired_functions']}")
    print(f"失败回退:           {summary['failed_functions']}")
    print(f"C2Rust Fallback:    {summary.get('c2rust_fallback_functions', 0)}")

    # 打印错误类别分布
    if summary.get("error_categories"):
        print("\n" + "-" * 80)
        print("错误类别分布")
        print("-" * 80)
        for cat, count in sorted(summary["error_categories"].items(), key=lambda x: -x[1]):
            cat_name = ERROR_CATEGORY_NAMES.get(cat, cat)
            print(f"  {cat_name}: {count}")

    # 打印 unsafe 统计
    if summary.get("unsafe_summary"):
        us = summary["unsafe_summary"]
        print("\n" + "-" * 80)
        print("Unsafe 代码统计（全局扫描：关键字∪作用域）")
        print("-" * 80)
        print(f"总代码行数:         {us['total_code_lines']}")
        print(f"Unsafe 项总数:      {us.get('total_unsafe_items', 0)}")
        print(f"  - unsafe 块:      {us.get('unsafe_blocks', 0)}")
        print(f"  - unsafe fn:      {us.get('unsafe_functions', 0)}")
        print(f"  - unsafe impl:    {us.get('unsafe_impls', 0)}")
        print(f"  - unsafe trait:   {us.get('unsafe_traits', 0)}")
        print(f"Unsafe 总行数:      {us.get('total_unsafe_total_lines', 0)}")
        print(f"Unsafe 总率:        {us.get('unsafe_total_ratio', 0.0):.2%}")
        print(f"  - 关键字行数:     {us.get('total_unsafe_keyword_lines', 0)} ({us.get('unsafe_keyword_ratio', 0.0):.2%})")
        print(f"  - 作用域行数:     {us.get('total_unsafe_context_lines', 0)} ({us.get('unsafe_context_ratio', 0.0):.2%})")
        print(f"Unsafe 行数(旧):    {us.get('unsafe_lines_estimate', 0)} (仅块/unsafe fn 体内)")
        print(f"Unsafe 率(旧):      {us.get('unsafe_ratio_estimate', 0.0):.2%}")
        print(f"Unsafe 密度:        {us.get('unsafe_density_per_kloc', 0):.2f} 项/千行代码")

    # 打印测试汇总
    if "test_summary" in summary:
        ts = summary["test_summary"]
        print("\n" + "-" * 80)
        print("测试结果汇总")
        print("-" * 80)
        print(f"可执行测试的项目:   {ts['projects_with_tests']}")
        print(f"测试总数:           {ts['total_tests']}")
        print(f"通过:               {ts['tests_passed']}")
        print(f"失败:               {ts['tests_failed']}")
        print(f"测试通过率:         {ts['overall_test_pass_rate']:.2%}")

    # 打印 Clippy 汇总
    if "clippy_summary" in summary:
        cs = summary["clippy_summary"]
        print("\n" + "-" * 80)
        print("Clippy 警告汇总")
        print("-" * 80)
        print(f"分析的项目数:       {cs['projects_analyzed']}")
        print(f"Clippy 警告数:      {cs.get('total_clippy_warnings', cs.get('total_warnings', 0))}")
        print(f"Rustc 警告数:       {cs.get('total_rustc_warnings', 0)}")
        print(f"总警告(含 rustc):   {cs.get('total_warnings_including_rustc', cs.get('total_warnings', 0))}")
        print(f"错误数:             {cs.get('total_errors', 0)}")
        print(f"平均每项目 Clippy:  {cs.get('avg_warnings_per_project', 0):.1f}")
        print(f"平均每项目 总计:    {cs.get('avg_total_warnings_per_project', 0):.1f}")

    # 打印 C2R 测试汇总
    if "c2r_test_summary" in summary:
        c2r_ts = summary["c2r_test_summary"]
        print("\n" + "-" * 80)
        print("C2R 单元测试汇总")
        print("-" * 80)
        print(f"执行测试的项目数:   {c2r_ts['projects_executed']}")
        print(f"编译成功的项目:     {c2r_ts['projects_compilation_succeeded']} ({c2r_ts['compilation_success_rate']:.2%})")
        print(f"测试总数:           {c2r_ts['total_tests']}")
        print(f"通过:               {c2r_ts['tests_passed']}")
        print(f"失败:               {c2r_ts['tests_failed']}")
        print(f"测试通过率:         {c2r_ts['overall_test_pass_rate']:.2%}")

    # 打印语义评估摘要
    if "semantic_evaluation" in summary:
        se = summary["semantic_evaluation"]
        print("\n" + "-" * 80)
        print("语义等价性评估")
        print("-" * 80)
        print(f"评估项目数:         {se.get('evaluated_projects', 0)}")
        print(f"平均语义得分:       {se.get('average_score', 0):.2f}")

    # 打印增量编译汇总
    if "incremental_compilation_summary" in summary:
        ics = summary["incremental_compilation_summary"]
        print("\n" + "-" * 80)
        print("增量编译成功率汇总")
        print("-" * 80)
        print(f"函数总数:           {ics['total_functions']}")
        print(f"编译成功函数数:     {ics['compiled_functions']}")
        print(f"增量编译成功率:     {ics['compile_rate']:.2%}")

    # ===== 表格1: 基础编译信息 =====
    print("\n" + "=" * 80)
    print("表1: 各项目编译结果")
    print("=" * 80)
    header1 = f"{'项目':<12} {'函数数':<8} {'一次通过':<10} {'修复通过':<10} {'失败':<8} {'通过率':<10} {'语义分':<8}"
    print(header1)
    print("-" * 80)

    for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
        total = proj_data["total_functions"]
        direct = proj_data["compiled_direct"]
        repaired = proj_data["compiled_after_repair"]
        failed = proj_data["failed_reverted"]
        rate = proj_data["pass_rate"]
        sem_score = proj_data.get("semantic_score", "-")
        if isinstance(sem_score, (int, float)):
            sem_str = f"{sem_score:.1f}"
        else:
            sem_str = str(sem_score)
        print(f"{proj_name:<12} {total:<8} {direct:<10} {repaired:<10} {failed:<8} {rate:<10.1%} {sem_str:<8}")

    # 汇总行
    print("-" * 80)
    total = summary['total_functions']
    direct = summary['direct_pass_functions']
    repaired = summary['repaired_functions']
    failed = summary['failed_functions']
    rate = summary['overall_pass_rate']
    avg_sem = summary.get('semantic_evaluation', {}).get('average_score', '-')
    if isinstance(avg_sem, (int, float)):
        avg_sem_str = f"{avg_sem:.1f}"
    else:
        avg_sem_str = str(avg_sem)
    print(f"{'总计':<12} {total:<8} {direct:<10} {repaired:<10} {failed:<8} {rate:<10.1%} {avg_sem_str:<8}")

    # ===== 表格2: 错误详情 =====
    print("\n" + "=" * 80)
    print("表2: 各项目错误分布")
    print("=" * 80)
    header2 = f"{'项目':<12} {'Opaque访问':<12} {'缺Accessor':<12} {'类型不匹配':<12} {'其他':<8}"
    print(header2)
    print("-" * 80)

    for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
        err_cats = proj_data.get("error_analysis", {}).get("categories", {})
        opaque = err_cats.get("E0609_opaque_field_access", 0)
        accessor = err_cats.get("E0425_missing_accessor_shim", 0)
        type_mm = err_cats.get("E0308_type_mismatch", 0)
        main_cats = ["E0609_opaque_field_access", "E0425_missing_accessor_shim", "E0308_type_mismatch"]
        other = sum(v for k, v in err_cats.items() if k not in main_cats)
        print(f"{proj_name:<12} {opaque:<12} {accessor:<12} {type_mm:<12} {other:<8}")

    # ===== 表格3: Unsafe 代码统计 =====
    print("\n" + "=" * 80)
    print("表3: Unsafe 代码统计（全局扫描：关键字∪作用域）")
    print("=" * 80)
    header3 = f"{'项目':<12} {'代码行数':<10} {'Unsafe总行':<10} {'Unsafe总率':<10} {'Unsafe项':<10} {'密度/千行':<10}"
    print(header3)
    print("-" * 80)

    for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
        ua = proj_data.get("unsafe_analysis", {})
        code_lines = ua.get("code_lines", 0)
        unsafe_lines = ua.get("unsafe_total_lines", 0)
        unsafe_ratio = ua.get("unsafe_total_ratio", 0.0)
        items = ua.get("total_unsafe_items", 0)
        density = ua.get("unsafe_density_per_kloc", 0.0)
        print(f"{proj_name:<12} {code_lines:<10} {unsafe_lines:<10} {unsafe_ratio:<10.2%} {items:<10} {density:<10.2f}")

    # ===== 表格4: C2R 测试结果 =====
    if "c2r_test_summary" in summary:
        print("\n" + "=" * 80)
        print("表4: C2R 单元测试结果")
        print("=" * 80)
        header4 = f"{'项目':<12} {'编译状态':<10} {'通过/总数':<12} {'通过率':<10}"
        print(header4)
        print("-" * 80)

        for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
            c2r_tr = proj_data.get("c2r_test_result", {})
            if c2r_tr.get("executed"):
                compiled = "成功" if c2r_tr.get("compilation_succeeded") else "失败"
                if c2r_tr.get("error") == "Test crashed with signal" and compiled == "成功":
                    compiled = "成功(崩溃)"
                passed = c2r_tr.get("tests_passed", 0)
                total = c2r_tr.get("total_tests", 0)
                rate = c2r_tr.get("pass_rate", 0.0)
                test_str = f"{passed}/{total}"
                print(f"{proj_name:<12} {compiled:<10} {test_str:<12} {rate:<10.2%}")
            else:
                error = c2r_tr.get("error", "未执行")[:20]
                print(f"{proj_name:<12} {'-':<10} {'-':<12} {error}")

    # ===== 表格4.5: Clippy 警告 =====
    if "clippy_summary" in summary:
        print("\n" + "=" * 80)
        print("表4.5: Clippy 警告统计")
        print("=" * 80)
        header_clippy = f"{'项目':<12} {'Clippy':<8} {'Rustc':<8} {'总计':<8} {'错误':<6} {'主要 Clippy 类型':<28}"
        print(header_clippy)
        print("-" * 80)

        for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
            cr = proj_data.get("cargo_clippy_result", {})
            if cr.get("executed"):
                clippy_warn = cr.get("warning_count", 0)
                rustc_warn = cr.get("rustc_warning_count", 0)
                total_warn = cr.get("warning_count_total", clippy_warn + rustc_warn)
                err_count = cr.get("error_count", 0)
                # 获取最常见的警告类型
                warnings_by_type = cr.get("warnings_by_type", {})
                if warnings_by_type:
                    top_warns = sorted(warnings_by_type.items(), key=lambda x: -x[1])[:2]
                    top_warns_str = ", ".join([f"{k}({v})" for k, v in top_warns])[:28]
                else:
                    top_warns_str = "-"
                print(f"{proj_name:<12} {clippy_warn:<8} {rustc_warn:<8} {total_warn:<8} {err_count:<6} {top_warns_str:<28}")
            else:
                error = cr.get("error", "未执行")[:20] if cr else "未执行"
                print(f"{proj_name:<12} {'-':<8} {'-':<8} {'-':<8} {'-':<6} {error:<28}")

        print("-" * 80)
        cs = summary.get("clippy_summary", {})
        total_clippy = cs.get("total_clippy_warnings", cs.get("total_warnings", 0))
        total_rustc = cs.get("total_rustc_warnings", 0)
        total_all = cs.get("total_warnings_including_rustc", total_clippy + total_rustc)
        total_errs = cs.get("total_errors", 0)
        avg_clippy = cs.get("avg_warnings_per_project", 0)
        avg_all = cs.get("avg_total_warnings_per_project", 0)
        note = f"均值: Clippy {avg_clippy:.1f}/proj, 总计 {avg_all:.1f}/proj"
        print(f"{'总计':<12} {total_clippy:<8} {total_rustc:<8} {total_all:<8} {total_errs:<6} {note:<28}")

    # ===== 表格5: 增量编译成功率 =====
    if "incremental_compilation_summary" in summary:
        print("\n" + "=" * 80)
        print("表5: 增量编译成功率（逐函数集成测试）")
        print("=" * 80)
        header5 = f"{'项目':<12} {'函数总数':<10} {'编译成功':<10} {'成功率':<12}"
        print(header5)
        print("-" * 80)

        for proj_name, proj_data in iter_projects_in_display_order(analysis["projects"]):
            ic = proj_data.get("incremental_compilation", {}) or {}
            ic_total = ic.get("total_functions", 0)
            ic_success = ic.get("compiled_functions", 0)
            ic_err = ic.get("error")

            if ic_total > 0 and not ic_err:
                ic_rate = ic.get("compile_rate", 0.0)
                print(f"{proj_name:<12} {ic_total:<10} {ic_success:<10} {ic_rate:<12.2%}")
            elif ic_total > 0 and ic_err:
                # 有总函数数但无法执行增量验证（baseline/skeleton 失败等）：保留分母，避免虚高。
                msg = ("错误: " + str(ic_err))[:20]
                print(f"{proj_name:<12} {ic_total:<10} {ic_success:<10} {msg:<12}")
            elif ic_err:
                msg = ("错误: " + str(ic_err))[:20]
                print(f"{proj_name:<12} {'-':<10} {'-':<10} {msg:<12}")
            else:
                print(f"{proj_name:<12} {'-':<10} {'-':<10} {'-':<12}")

        print("-" * 80)
        ics = summary.get("incremental_compilation_summary", {})
        ic_total = ics.get("total_functions", 0)
        ic_success = ics.get("compiled_functions", 0)
        ic_rate = ics.get("compile_rate", 0.0)
        print(f"{'总计':<12} {ic_total:<10} {ic_success:<10} {ic_rate:<12.2%}")
        print()
        print(f"  增量编译成功率: {ic_rate:.2%} ({ic_success}/{ic_total})")

    # ===== 关键指标大表（统一格式） =====
    print_key_metrics_summary_table(analysis)

    print("=" * 80)


# ============================================================================
# 主函数
# ============================================================================

def main() -> None:
    parser = argparse.ArgumentParser(
        description="分析 C2R 增量翻译的函数级可编译率"
    )
    parser.add_argument(
        "--run-dir",
        type=Path,
        default=DEFAULT_RUN_DIR,
        help="翻译运行目录（包含 intermediate 和 results 子目录）"
    )
    parser.add_argument(
        "--llm-name",
        type=str,
        default="qwen3_coder",
        help="LLM 名称（用于定位翻译结果目录，如 qwen3_coder）"
    )
    parser.add_argument(
        "--quiet", "-q",
        action="store_true",
        help="不打印论文指标表格（静默模式）"
    )
    parser.add_argument(
        "--no-parallel",
        action="store_true",
        help="禁用并行处理"
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=4,
        help="并行处理的最大工作线程数（默认：4）"
    )
    parser.add_argument(
        "--no-errors",
        action="store_true",
        help="跳过错误分析"
    )
    parser.add_argument(
        "--no-unsafe",
        action="store_true",
        help="跳过 unsafe 代码分析"
    )
    parser.add_argument(
        "--run-tests",
        action="store_true",
        help="运行 cargo test 并统计测试结果"
    )
    parser.add_argument(
        "--run-clippy",
        action="store_true",
        help="运行 cargo clippy 并统计警告数"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="运行论文需要的分析（等价于 --run-clippy --run-c2r-tests；默认不启用 --verify-incremental）"
    )
    parser.add_argument(
        "--verify-incremental",
        action="store_true",
        help="验证增量编译成功率（逐个函数测试编译）"
    )
    parser.add_argument(
        "--run-c2r-tests",
        action="store_true",
        help="运行 C2R 单元测试（从测试目录复制测试文件到项目中运行）"
    )
    parser.add_argument(
        "--c2r-tests-dir",
        type=Path,
        default=DEFAULT_C2R_TESTS_DIR,
        help=f"C2R 测试文件目录（默认：{DEFAULT_C2R_TESTS_DIR}）"
    )

    args = parser.parse_args()

    # 处理 --all 参数
    run_tests = args.run_tests
    run_clippy = args.run_clippy or args.all
    run_c2r_tests_flag = args.run_c2r_tests or args.all
    verify_incremental = args.verify_incremental

    if not args.run_dir.exists():
        print(f"Error: Run directory does not exist: {args.run_dir}", file=sys.stderr)
        sys.exit(1)

    # 分析
    result = analyze_run(
        args.run_dir,
        args.llm_name,
        parallel=not args.no_parallel,
        max_workers=args.workers,
        analyze_errors=not args.no_errors,
        analyze_unsafe=not args.no_unsafe,
        run_tests=run_tests,
        run_clippy=run_clippy,
        run_c2r_tests_flag=run_c2r_tests_flag,
        c2r_tests_dir=args.c2r_tests_dir,
        verify_incremental=verify_incremental
    )

    # 终端只输出论文用到的指标（tables_data.py）：CR / FC / Unsafe / Clippy
    if not args.quiet:
        def _f2(x: Optional[float]) -> str:
            if x is None:
                return "-"
            return f"{x:.2f}"

        def _get_percent(x: Any) -> Optional[float]:
            try:
                if x is None:
                    return None
                return float(x) * 100.0
            except Exception:
                return None

        def _get_cr_percent(p: Dict[str, Any]) -> Optional[float]:
            ic = p.get("incremental_compilation")
            if isinstance(ic, dict) and ic.get("compile_rate") is not None:
                return _get_percent(ic.get("compile_rate"))
            return _get_percent(p.get("pass_rate"))

        def _get_fc_percent(p: Dict[str, Any]) -> Optional[float]:
            c2r = p.get("c2r_test_result")
            if isinstance(c2r, dict) and c2r.get("executed"):
                return _get_percent(c2r.get("pass_rate"))
            return None

        def _get_unsafe_percent(p: Dict[str, Any]) -> Optional[float]:
            ua = p.get("unsafe_analysis")
            if not isinstance(ua, dict) or ua.get("error"):
                return None
            return _get_percent(ua.get("unsafe_total_ratio"))

        def _get_clippy_total(p: Dict[str, Any]) -> Optional[int]:
            cl = p.get("cargo_clippy_result")
            if not isinstance(cl, dict) or not cl.get("executed"):
                return None
            try:
                return int(cl.get("warning_count_total", 0) or 0)
            except Exception:
                return None

        projects = result.get("projects", {}) or {}
        print("\t".join(["project", "CR", "FC", "Unsafe", "Clippy"]))
        for proj_name, proj_data in iter_projects_in_display_order(projects):
            if not isinstance(proj_data, dict):
                continue

            # RQ2 Claude 的 bzip2 在当前主机工具链下无法编译
            # 因此将其记为缺失值（"--"）而不是 0.0。
            if (
                args.verify_incremental
                and proj_name == "bzip2"
                and args.run_dir.name == "claude"
                and args.run_dir.parent.name == "rq2"
            ):
                print("\t".join([proj_name, "-", "-", "-", "-"]))
                continue

            cr = _get_cr_percent(proj_data)
            fc = _get_fc_percent(proj_data)
            unsafe = _get_unsafe_percent(proj_data)
            clippy = _get_clippy_total(proj_data)
            print(
                "\t".join(
                    [
                        proj_name,
                        _f2(cr),
                        _f2(fc),
                        _f2(unsafe),
                        "-" if clippy is None else str(clippy),
                    ]
                )
            )


if __name__ == "__main__":
    main()
