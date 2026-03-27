#!/usr/bin/env python3
"""
C2Rust 翻译结果分析脚本

分析 c2rust 翻译的 Rust 代码的编译状态、unsafe 代码比例、clippy 警告等

参考自 c2saferrust 的 analyze_compilation_rate.py

输出格式:
{
    "base_dir": "...",
    "timestamp": "...",
    "projects": {
        "project_name": {
            "source_files": ["src_qsort.rs"],
            "total_lines": 100,
            "code_lines": 80,
            "cargo_check_passed": true,
            "unsafe_analysis": {...},
            "clippy_results": {...}
        }
    },
    "summary": {
        "total_projects": 10,
        "projects_compiled": 9,
        "project_compile_rate": 0.9,
        ...
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
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from datetime import datetime


# ============================================================================
# 常量定义
# ============================================================================

# Keep consistent with other analysis scripts / paper tables.
ALL_PROJECTS = ["ht", "qsort", "quadtree", "buffer", "rgba", "urlparser", "genann", "avl", "zopfli"]

BASE_DIR = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/test_module/rust_tests")


# ============================================================================
# Cargo 操作
# ============================================================================

def run_cargo_check(project_dir: Path, timeout: int = 120) -> Tuple[bool, str]:
    """运行 cargo check 并返回结果"""
    if not project_dir.exists():
        return False, "Project directory does not exist"

    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        return False, "Cargo.toml not found"

    try:
        proc = subprocess.run(
            ["cargo", "check", "--offline"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
            env={**os.environ, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        )
        return proc.returncode == 0, proc.stderr
    except subprocess.TimeoutExpired:
        return False, f"Timeout after {timeout}s"
    except Exception as e:
        return False, str(e)


def run_cargo_clippy(project_dir: Path, timeout: int = 120) -> Dict[str, Any]:
    """运行 cargo clippy，并用 JSON 诊断精确统计 Clippy vs Rustc 的 warning/error。"""
    result = {
        "executed": False,
        # Clippy lints only (diagnostic code starts with "clippy::")
        "warning_count": 0,
        # Non-clippy warnings emitted during clippy run (plain rustc warnings)
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
        "warnings_by_type": {},
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
        proc = subprocess.run(
            ["cargo", "clippy", "--offline", "--message-format=json", "--", "-W", "clippy::all"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
            env={**os.environ, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        )
        result["executed"] = True
        # Keep raw output for debugging (JSON on stdout; plain text may be on stderr).
        output = proc.stdout + proc.stderr
        result["output"] = output[:200000]  # avoid huge json logs in result file

        clippy_warn = 0
        rustc_warn = 0
        err_cnt = 0
        warnings_by_type: Dict[str, int] = {}

        for line in proc.stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
            except Exception:
                continue
            if obj.get("reason") != "compiler-message":
                continue

            msg = obj.get("message") or {}
            level = msg.get("level")
            code_obj = msg.get("code") or {}
            code = code_obj.get("code") or ""

            if level == "warning":
                if code.startswith("clippy::"):
                    clippy_warn += 1
                    warnings_by_type[code] = warnings_by_type.get(code, 0) + 1
                else:
                    rustc_warn += 1
            elif level == "error":
                err_cnt += 1

        result["warning_count"] = clippy_warn
        result["rustc_warning_count"] = rustc_warn
        result["warning_count_total"] = clippy_warn + rustc_warn
        result["error_count"] = err_cnt
        result["warnings_by_type"] = warnings_by_type

        if proc.returncode != 0 and err_cnt > 0:
            # Still a "real run"; mark why it may be incomplete.
            result["error"] = "cargo clippy failed"

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except FileNotFoundError:
        result["error"] = "cargo clippy not found (install with: rustup component add clippy)"
    except Exception as e:
        result["error"] = str(e)

    return result


def run_cargo_test(project_dir: Path, timeout: int = 120) -> Dict[str, Any]:
    """运行 cargo test 并解析结果"""
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
        proc = subprocess.run(
            ["cargo", "test", "--offline", "--no-fail-fast"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
            env={**os.environ, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        )
        result["executed"] = True
        result["output"] = proc.stdout + proc.stderr
        output = proc.stdout + proc.stderr

        test_result_pattern = r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored'
        matches = re.findall(test_result_pattern, output)

        for match in matches:
            result["passed"] += int(match[0])
            result["failed"] += int(match[1])
            result["ignored"] += int(match[2])

        result["total"] = result["passed"] + result["failed"]
        if result["total"] > 0:
            result["pass_rate"] = result["passed"] / result["total"]

        if not matches:
            # Distinguish compile/build errors from runtime test failures.
            compilation_error_patterns = [
                r"error\[E\d+\]:",  # error[E0xxx]:
                r"could not compile",  # cargo/rustc error
                r"error: linking with",  # linker error
                r"undefined reference to",  # linker error (gcc/clang)
                r"undefined symbol:",  # linker error (lld)
                r"failed to run custom build command",  # build.rs
                r"error occurred in cc-rs",  # cc crate build failure
            ]
            is_compilation_error = any(re.search(p, output, re.IGNORECASE) for p in compilation_error_patterns)

            running_test_counts = [int(x) for x in re.findall(r'^running\s+(\d+)\s+tests\b', output, re.MULTILINE)]
            expected_total_tests = sum(running_test_counts) if running_test_counts else 0

            if not is_compilation_error and proc.returncode != 0:
                has_test_execution_markers = ("Running unittests" in output) or (expected_total_tests > 0)
                if not has_test_execution_markers:
                    is_compilation_error = True

            if is_compilation_error:
                result["error"] = "Compilation failed"
            elif "error: test failed" in output.lower() or "test failed" in output.lower():
                result["error"] = "Test failed"
            elif "no test target" in output.lower() or "0 tests" in output.lower():
                result["error"] = "No tests found"

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except Exception as e:
        result["error"] = str(e)

    return result


def run_c2rust_tests(project_name: str, c2rust_dir: Path, tests_base_dir: Path,
                     timeout: int = 180) -> Dict[str, Any]:
    """
    运行 c2rust 外部测试（直接调用 Rust 函数）

    参数:
        project_name: 项目名
        c2rust_dir: c2rust 翻译后的目录（包含 .rs 源文件）
        tests_base_dir: 测试代码目录路径（包含各项目的 c2rust/test.rs 文件）
        timeout: 超时时间（秒）
    """
    import shutil
    import tempfile

    result = {
        "executed": False,
        "test_file_found": False,
        "project_created": False,
        "compilation_succeeded": False,
        "tests_passed": 0,
        "tests_failed": 0,
        "tests_ignored": 0,
        "total_tests": 0,
        "pass_rate": 0.0,
        "error": None,
        "error_categories": {},
        "raw_errors": [],
        "output": "",
        "function_test_results": {}
    }

    if not c2rust_dir.exists():
        result["error"] = "c2rust directory does not exist"
        return result

    # 查找测试文件
    test_file = tests_base_dir / project_name / "c2rust" / "test.rs"

    if not test_file.exists():
        result["error"] = f"c2rust test file not found: {test_file}"
        return result

    result["test_file_found"] = True

    # 获取 c2rust 源文件（排除 test.rs）
    rs_files = [f for f in c2rust_dir.glob("*.rs") if f.name != "test.rs"]
    if not rs_files:
        result["error"] = "No source .rs files found in c2rust directory"
        return result

    # 创建临时项目目录
    temp_dir = tempfile.mkdtemp(prefix=f"c2rust_test_{project_name}_")

    try:
        temp_project = Path(temp_dir) / project_name
        src_dir = temp_project / "src"
        src_dir.mkdir(parents=True, exist_ok=True)

        # 创建 Cargo.toml
        cargo_toml_content = f'''[package]
name = "{project_name.replace("-", "_")}_c2rust_test"
version = "0.1.0"
edition = "2018"

[lib]
path = "src/lib.rs"

[dependencies]
libc = "0.2"
'''
        (temp_project / "Cargo.toml").write_text(cargo_toml_content)

        # 创建 rust-toolchain.toml (使用 nightly 工具链)
        rust_toolchain_content = '''[toolchain]
channel = "nightly-2022-08-08"
'''
        (temp_project / "rust-toolchain.toml").write_text(rust_toolchain_content)

        # 复制源文件并添加 allow 属性
        mod_names = []
        for rs_file in rs_files:
            mod_name = rs_file.stem
            mod_names.append(mod_name)

            src_content = rs_file.read_text(encoding='utf-8', errors='ignore')
            # 添加 allow 属性（如果没有）
            if not src_content.startswith('#![allow'):
                src_content = '''#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

''' + src_content
            (src_dir / rs_file.name).write_text(src_content, encoding='utf-8')

        # 创建 lib.rs
        lib_content = '''#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(c_variadic)]
#![feature(label_break_value)]
#![feature(extern_types)]
#![feature(linkage)]
#![register_tool(c2rust)]
#![feature(register_tool)]

'''
        # 添加模块声明
        for mod_name in mod_names:
            lib_content += f'pub mod {mod_name};\n'

        # 添加测试模块声明
        lib_content += '\n#[cfg(test)]\nmod test_c2rust;\n'

        (src_dir / "lib.rs").write_text(lib_content, encoding='utf-8')

        # 复制测试文件
        shutil.copy(test_file, src_dir / "test_c2rust.rs")

        result["project_created"] = True

        # 运行 cargo test
        proc = subprocess.run(
            ["cargo", "test", "--offline", "--lib", "test_c2rust", "--no-fail-fast", "--", "--test-threads=1"],
            cwd=temp_project,
            capture_output=True,
            text=True,
            timeout=timeout,
            env={**os.environ, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        )
        result["executed"] = True
        result["output"] = proc.stdout + proc.stderr
        output = result["output"]

        # 区分编译错误与测试失败
        running_test_counts = [int(x) for x in re.findall(r'^running\s+(\d+)\s+tests\b', output, re.MULTILINE)]
        expected_total_tests = sum(running_test_counts) if running_test_counts else 0

        compilation_error_patterns = [
            r"error\[E\d+\]:",
            r"could not compile",
            r"error: linking with",
            r"undefined reference to",
            r"undefined symbol:",
            r"failed to run custom build command",
            r"error occurred in cc-rs",
        ]
        is_compilation_error = any(re.search(p, output, re.IGNORECASE) for p in compilation_error_patterns)
        if not is_compilation_error and proc.returncode != 0:
            has_test_execution_markers = ("Running unittests" in output) or (expected_total_tests > 0)
            if not has_test_execution_markers:
                is_compilation_error = True

        if is_compilation_error:
            result["compilation_succeeded"] = False
            # 分类错误
            error_analysis = categorize_errors(output)
            result["error_categories"] = error_analysis.get("categories", {})
            # 提取原始错误信息
            error_lines = [line for line in output.split('\n') if line.startswith('error')]
            result["raw_errors"] = error_lines[:10]
            result["error"] = "Compilation failed"
        else:
            result["compilation_succeeded"] = True
            if proc.returncode != 0 and ("error: test failed" in output.lower() or "test failed" in output.lower()):
                result["error"] = "Test failed"

        # 解析测试结果
        test_result_pattern = r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored'
        matches = re.findall(test_result_pattern, output)

        for match in matches:
            result["tests_passed"] += int(match[1])
            result["tests_failed"] += int(match[2])
            result["tests_ignored"] += int(match[3])

        result["total_tests"] = result["tests_passed"] + result["tests_failed"]
        if result["total_tests"] > 0:
            result["pass_rate"] = result["tests_passed"] / result["total_tests"]

        # 解析每个测试的结果
        test_line_pattern = r'test test_c2rust::(\w+)\s+\.\.\.\s+(\w+)'
        test_matches = re.findall(test_line_pattern, output)
        for test_name, status in test_matches:
            result["function_test_results"][test_name] = status.lower()

    except subprocess.TimeoutExpired:
        result["error"] = f"Timeout after {timeout}s"
    except Exception as e:
        result["error"] = str(e)
        import traceback
        result["traceback"] = traceback.format_exc()
    finally:
        # 清理临时目录
        try:
            shutil.rmtree(temp_dir)
        except Exception:
            pass

    return result


# ============================================================================
# 错误分析
# ============================================================================

def categorize_errors(error_output: str) -> Dict[str, Any]:
    """分类错误类型"""
    if not error_output:
        return {"total_errors": 0, "categories": {}}

    categories = {
        "self_keyword": 0,
        "missing_macro": 0,
        "type_mismatch": 0,
        "missing_type": 0,
        "syntax_error": 0,
        "brace_mismatch": 0,
        "lifetime_error": 0,
        "borrow_error": 0,
        "undefined_function": 0,
        "other": 0
    }

    error_patterns = [
        (r'`self` parameter is only allowed', "self_keyword"),
        (r'`self` value is a keyword', "self_keyword"),
        (r'cannot find macro', "missing_macro"),
        (r'mismatched types', "type_mismatch"),
        (r'cannot find type', "missing_type"),
        (r'unexpected closing delimiter', "brace_mismatch"),
        (r'expected .+, found', "syntax_error"),
        (r'lifetime', "lifetime_error"),
        (r'cannot borrow', "borrow_error"),
        (r'cannot find function', "undefined_function"),
        (r'cannot find value', "undefined_function"),
    ]

    lines = error_output.split('\n')
    total_errors = 0

    for line in lines:
        if line.startswith('error'):
            total_errors += 1
            matched = False
            for pattern, category in error_patterns:
                if re.search(pattern, line):
                    categories[category] += 1
                    matched = True
                    break
            if not matched:
                categories["other"] += 1

    return {
        "total_errors": total_errors,
        "categories": {k: v for k, v in categories.items() if v > 0}
    }


# ============================================================================
# Unsafe 代码分析
# ============================================================================

def count_lines_in_braces(content: str, brace_start: int) -> int:
    """统计花括号内的代码行数"""
    n = len(content)
    if brace_start >= n or content[brace_start] != '{':
        return 0

    brace_count = 1
    j = brace_start + 1
    start_line = content[:brace_start].count('\n')
    in_string = False

    while j < n and brace_count > 0:
        c = content[j]

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

        if content[j:j+2] == '//':
            while j < n and content[j] != '\n':
                j += 1
            continue

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

        if s.startswith('b\"', i):
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

        if s.startswith('b\"', i):
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

        if content.startswith('b\"', i):
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


def count_unsafe_lines_precise(content: str) -> Tuple[int, int, int, int]:
    """精确统计 unsafe 代码的实际行数"""
    unsafe_block_lines = 0
    unsafe_fn_lines = 0
    unsafe_block_count = 0
    unsafe_fn_count = 0

    n = len(content)
    i = 0

    while i < n:
        if content[i] == '"':
            i += 1
            while i < n and content[i] != '"':
                if content[i] == '\\' and i + 1 < n:
                    i += 2
                else:
                    i += 1
            i += 1
            continue

        if content[i:i+2] == '//':
            while i < n and content[i] != '\n':
                i += 1
            continue

        if content[i:i+2] == '/*':
            i += 2
            while i < n - 1 and content[i:i+2] != '*/':
                i += 1
            i += 2
            continue

        if content[i:i+6] == 'unsafe' and (i == 0 or not content[i-1].isalnum() and content[i-1] != '_'):
            end_pos = i + 6
            if end_pos < n and (content[end_pos].isalnum() or content[end_pos] == '_'):
                i += 1
                continue

            j = end_pos
            while j < n and content[j] in ' \t\n\r':
                j += 1

            if j >= n:
                i += 1
                continue

            if content[j] == '{':
                unsafe_block_count += 1
                brace_start = j
                lines = count_lines_in_braces(content, brace_start)
                unsafe_block_lines += lines
                i = j + 1
            elif content[j:j+2] == 'fn' or content[j:j+6] == 'extern':
                # unsafe fn 或 unsafe extern "C" fn
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


def analyze_unsafe_code(rs_files: List[Path]) -> Dict[str, Any]:
    """分析 Rust 文件中的 unsafe 代码比例"""
    result = {
        "total_lines": 0,
        "code_lines": 0,  # 全局扫描得到的代码行（排除注释/字符串）
        "unsafe_blocks": 0,
        "unsafe_functions": 0,
        "unsafe_impls": 0,
        "unsafe_traits": 0,
        "total_unsafe_items": 0,
        "unsafe_lines": 0,
        "unsafe_ratio": 0.0,
        # 全局（关键字/作用域）统计：覆盖 unsafe extern 等场景
        "unsafe_keyword_occurrences": 0,
        "unsafe_keyword_lines": 0,
        "unsafe_keyword_ratio": 0.0,
        "unsafe_context_lines": 0,
        "unsafe_context_ratio": 0.0,
        "unsafe_total_lines": 0,
        "unsafe_total_ratio": 0.0,
        "files_analyzed": len(rs_files),
        "unsafe_density_per_kloc": 0.0,
        "error": None
    }

    if not rs_files:
        result["error"] = "No .rs files found"
        return result

    try:
        total_unsafe_block_lines = 0
        total_unsafe_fn_lines = 0

        for rs_file in rs_files:
            try:
                content = rs_file.read_text(encoding='utf-8', errors='ignore')
                result["total_lines"] += content.count('\n') + 1
                global_metrics = analyze_unsafe_global_in_content(content)
                result["code_lines"] += global_metrics.get("code_lines", 0)
                result["unsafe_keyword_occurrences"] += global_metrics.get("unsafe_keyword_occurrences", 0)
                result["unsafe_keyword_lines"] += global_metrics.get("unsafe_keyword_lines", 0)
                result["unsafe_context_lines"] += global_metrics.get("unsafe_context_lines", 0)
                result["unsafe_total_lines"] += global_metrics.get("unsafe_total_lines", 0)

                result["unsafe_blocks"] += len(re.findall(r'\bunsafe\s*\{', content))
                # 匹配 unsafe fn 和 unsafe extern "C" fn 等格式
                result["unsafe_functions"] += len(re.findall(r'\bunsafe\s+(?:extern\s*"C"\s*)?fn\b', content))
                result["unsafe_impls"] += len(re.findall(r'\bunsafe\s+impl\b', content))
                result["unsafe_traits"] += len(re.findall(r'\bunsafe\s+trait\b', content))

                block_lines, fn_lines, _, _ = count_unsafe_lines_precise(content)
                total_unsafe_block_lines += block_lines
                total_unsafe_fn_lines += fn_lines

            except Exception as e:
                print(f"Warning: Failed to analyze {rs_file}: {e}", file=sys.stderr)

        result["total_unsafe_items"] = (
            result["unsafe_blocks"] +
            result["unsafe_functions"] +
            result["unsafe_impls"] +
            result["unsafe_traits"]
        )

        result["unsafe_lines"] = total_unsafe_block_lines + total_unsafe_fn_lines

        if result["code_lines"] > 0:
            result["unsafe_ratio"] = result["unsafe_lines"] / result["code_lines"]
            result["unsafe_keyword_ratio"] = result["unsafe_keyword_lines"] / result["code_lines"]
            result["unsafe_context_ratio"] = result["unsafe_context_lines"] / result["code_lines"]
            result["unsafe_total_ratio"] = result["unsafe_total_lines"] / result["code_lines"]
            result["unsafe_density_per_kloc"] = result["total_unsafe_items"] / result["code_lines"] * 1000

    except Exception as e:
        result["error"] = str(e)

    return result


# ============================================================================
# 项目分析
# ============================================================================

def create_temp_cargo_project(project_name: str, rs_files: List[Path], temp_dir: Path) -> Optional[Path]:
    """创建临时 Cargo 项目用于编译检查（使用 nightly 工具链）"""
    import shutil

    project_dir = temp_dir / project_name
    src_dir = project_dir / "src"
    src_dir.mkdir(parents=True, exist_ok=True)

    # 创建 Cargo.toml (使用 edition = "2018" 与 c2rust 一致)
    cargo_toml_content = f'''[package]
name = "{project_name.replace("-", "_")}_c2rust"
version = "0.1.0"
edition = "2018"

[lib]
path = "src/lib.rs"

[dependencies]
libc = "0.2"
'''
    (project_dir / "Cargo.toml").write_text(cargo_toml_content)

    # 创建 rust-toolchain.toml (使用 nightly 工具链)
    rust_toolchain_content = '''[toolchain]
channel = "nightly-2022-08-08"
'''
    (project_dir / "rust-toolchain.toml").write_text(rust_toolchain_content)

    # 复制源文件
    for rs_file in rs_files:
        shutil.copy(rs_file, src_dir / rs_file.name)

    # 创建 lib.rs (添加 nightly 特性)
    lib_content = '''#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(c_variadic)]
#![feature(label_break_value)]
#![feature(extern_types)]
#![feature(linkage)]
#![register_tool(c2rust)]
#![feature(register_tool)]

'''
    # 添加模块声明
    for rs_file in rs_files:
        mod_name = rs_file.stem
        lib_content += f'mod {mod_name};\n'

        # 为每个源文件添加 allow 属性
        src_content = (src_dir / rs_file.name).read_text(encoding='utf-8')
        if not src_content.startswith('#![allow'):
            new_content = '''#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

''' + src_content
            (src_dir / rs_file.name).write_text(new_content, encoding='utf-8')

    (src_dir / "lib.rs").write_text(lib_content, encoding='utf-8')

    return project_dir


def analyze_project(project_name: str, c2rust_dir: Path, run_clippy: bool = False,
                   analyze_unsafe: bool = True, temp_base_dir: Optional[Path] = None,
                   run_c2rust_tests_flag: bool = False, c2rust_tests_dir: Optional[Path] = None) -> Dict[str, Any]:
    """分析单个项目的 c2rust 翻译结果"""
    result = {
        "project_name": project_name,
        "c2rust_dir": str(c2rust_dir),
        "source_files": [],
        "total_lines": 0,
        "code_lines": 0,
        "cargo_check_passed": False,
        "cargo_error": None,
        "error_analysis": {}
    }

    if not c2rust_dir.exists():
        result["error"] = "c2rust directory does not exist"
        return result

    # 查找翻译后的 .rs 文件（排除 test.rs）
    rs_files = [f for f in c2rust_dir.glob("*.rs") if f.name != "test.rs"]

    if not rs_files:
        result["error"] = "No source .rs files found"
        return result

    result["source_files"] = [f.name for f in rs_files]

    # 统计基本信息
    for rs_file in rs_files:
        try:
            content = rs_file.read_text(encoding='utf-8', errors='ignore')
            lines = content.split('\n')
            result["total_lines"] += len(lines)

            in_block_comment = False
            for line in lines:
                stripped = line.strip()
                if '/*' in stripped:
                    in_block_comment = True
                if '*/' in stripped:
                    in_block_comment = False
                    continue
                if in_block_comment:
                    continue
                if stripped and not stripped.startswith('//'):
                    result["code_lines"] += 1
        except Exception as e:
            print(f"Warning: Failed to read {rs_file}: {e}", file=sys.stderr)

    # Unsafe 代码分析
    if analyze_unsafe:
        result["unsafe_analysis"] = analyze_unsafe_code(rs_files)

    # 创建临时项目进行编译检查
    if temp_base_dir:
        import shutil
        temp_project_dir = None
        try:
            temp_project_dir = create_temp_cargo_project(project_name, rs_files, temp_base_dir)

            if temp_project_dir:
                # 运行 cargo check
                cargo_passed, cargo_error = run_cargo_check(temp_project_dir)
                result["cargo_check_passed"] = cargo_passed

                if not cargo_passed and cargo_error:
                    result["cargo_error"] = cargo_error[:2000]
                    result["error_analysis"] = categorize_errors(cargo_error)

                # 运行 clippy
                if run_clippy:
                    result["clippy_results"] = run_cargo_clippy(temp_project_dir)

        except Exception as e:
            result["error"] = str(e)
        finally:
            if temp_project_dir and temp_project_dir.exists():
                try:
                    shutil.rmtree(temp_project_dir)
                except Exception:
                    pass

    # 运行 c2rust 外部测试
    if run_c2rust_tests_flag and c2rust_tests_dir:
        result["c2rust_test_results"] = run_c2rust_tests(
            project_name, c2rust_dir, c2rust_tests_dir
        )
    elif run_c2rust_tests_flag:
        result["c2rust_test_results"] = {
            "executed": False,
            "error": "c2rust_tests_dir not specified",
            "tests_passed": 0, "tests_failed": 0, "total_tests": 0, "pass_rate": 0.0
        }

    return result


def analyze_all_projects(base_dir: Path, projects: List[str], run_clippy: bool = False,
                        analyze_unsafe: bool = True, run_c2rust_tests_flag: bool = False,
                        c2rust_tests_dir: Optional[Path] = None) -> Dict[str, Any]:
    """分析所有项目"""
    import tempfile

    result = {
        "base_dir": str(base_dir),
        "timestamp": datetime.now().isoformat(),
        "projects": {},
        "summary": {}
    }

    temp_dir = Path(tempfile.mkdtemp(prefix="c2rust_analysis_"))

    try:
        total_projects = 0
        projects_compiled = 0
        total_code_lines = 0
        total_unsafe_items = 0
        total_unsafe_lines_precise = 0
        total_unsafe_keyword_occurrences = 0
        total_unsafe_keyword_lines = 0
        total_unsafe_context_lines = 0
        total_unsafe_total_lines = 0
        total_clippy_warnings = 0
        total_rustc_warnings = 0
        total_warnings = 0
        total_clippy_errors = 0
        projects_with_clippy = 0

        # c2rust 测试汇总
        total_c2rust_tests_passed = 0
        total_c2rust_tests_failed = 0
        total_c2rust_tests = 0
        projects_with_c2rust_tests = 0

        error_categories_total: Dict[str, int] = {}

        for project_name in projects:
            c2rust_dir = base_dir / project_name / "c2rust"

            project_result = analyze_project(
                project_name,
                c2rust_dir,
                run_clippy=run_clippy,
                analyze_unsafe=analyze_unsafe,
                temp_base_dir=temp_dir,
                run_c2rust_tests_flag=run_c2rust_tests_flag,
                c2rust_tests_dir=c2rust_tests_dir
            )

            if project_result:
                result["projects"][project_name] = project_result
                total_projects += 1

                if project_result.get("cargo_check_passed"):
                    projects_compiled += 1

                total_code_lines += project_result.get("code_lines", 0)

                for cat, count in project_result.get("error_analysis", {}).get("categories", {}).items():
                    error_categories_total[cat] = error_categories_total.get(cat, 0) + count

                ua = project_result.get("unsafe_analysis", {})
                if ua and not ua.get("error"):
                    total_unsafe_items += ua.get("total_unsafe_items", 0)
                    total_unsafe_lines_precise += ua.get("unsafe_lines", 0)
                    total_unsafe_keyword_occurrences += ua.get("unsafe_keyword_occurrences", 0)
                    total_unsafe_keyword_lines += ua.get("unsafe_keyword_lines", 0)
                    total_unsafe_context_lines += ua.get("unsafe_context_lines", 0)
                    total_unsafe_total_lines += ua.get("unsafe_total_lines", 0)

                cr = project_result.get("clippy_results", {})
                if cr and cr.get("executed"):
                    projects_with_clippy += 1
                    total_clippy_warnings += cr.get("warning_count", 0)
                    total_rustc_warnings += cr.get("rustc_warning_count", 0)
                    total_warnings += cr.get("warning_count_total", cr.get("warning_count", 0) + cr.get("rustc_warning_count", 0))
                    total_clippy_errors += cr.get("error_count", 0)

                # 汇总 c2rust 测试结果
                ctr = project_result.get("c2rust_test_results", {})
                if ctr and ctr.get("executed"):
                    projects_with_c2rust_tests += 1
                    total_c2rust_tests_passed += ctr.get("tests_passed", 0)
                    total_c2rust_tests_failed += ctr.get("tests_failed", 0)
                    total_c2rust_tests += ctr.get("total_tests", 0)

        result["summary"] = {
            "total_projects": total_projects,
            "projects_compiled": projects_compiled,
            "project_compile_rate": projects_compiled / total_projects if total_projects > 0 else 0,
            "total_code_lines": total_code_lines,
            "error_categories": error_categories_total
        }

        if analyze_unsafe:
            result["summary"]["unsafe_summary"] = {
                "total_unsafe_items": total_unsafe_items,
                # 旧指标：只统计 unsafe 块/unsafe fn 体内的“实际行数”
                "total_unsafe_lines": total_unsafe_lines_precise,
                "unsafe_ratio": total_unsafe_lines_precise / total_code_lines if total_code_lines > 0 else 0,
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

        if run_clippy:
            result["summary"]["clippy_summary"] = {
                "projects_analyzed": projects_with_clippy,
                # Backward-compatible keys (older scripts treated this as "clippy warnings only").
                "total_warnings": total_clippy_warnings,
                "avg_warnings_per_project": total_clippy_warnings / projects_with_clippy if projects_with_clippy > 0 else 0,
                "total_clippy_warnings": total_clippy_warnings,
                "total_rustc_warnings": total_rustc_warnings,
                "total_warnings_including_rustc": total_warnings,
                "total_errors": total_clippy_errors,
                "avg_clippy_warnings_per_project": total_clippy_warnings / projects_with_clippy if projects_with_clippy > 0 else 0,
                "avg_total_warnings_per_project": total_warnings / projects_with_clippy if projects_with_clippy > 0 else 0,
            }

        if run_c2rust_tests_flag:
            result["summary"]["c2rust_test_summary"] = {
                "projects_with_tests": projects_with_c2rust_tests,
                "total_tests": total_c2rust_tests,
                "tests_passed": total_c2rust_tests_passed,
                "tests_failed": total_c2rust_tests_failed,
                "overall_test_pass_rate": total_c2rust_tests_passed / total_c2rust_tests if total_c2rust_tests > 0 else 0
            }

    finally:
        import shutil
        try:
            shutil.rmtree(temp_dir)
        except Exception:
            pass

    return result


# ============================================================================
# 函数级测试验证（使用 c2rust 作为基准）
# ============================================================================

def extract_functions_from_content(content: str) -> Dict[str, Dict[str, Any]]:
    """
    从 Rust 代码中提取所有函数定义

    返回: {函数名: {"signature": 签名, "body": 函数体, "full": 完整代码, "start": 起始位置, "end": 结束位置}}
    """
    functions = {}

    # 匹配函数定义（包括属性）
    # 模式: #[no_mangle] pub unsafe extern "C" fn name(...) -> RetType { ... }
    pattern = r'((?:#\[[\w_]+\]\s*)*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*)\{'

    for match in re.finditer(pattern, content):
        func_signature = match.group(1).strip()
        func_name = match.group(2)
        brace_start = match.end() - 1

        # 找到函数体的结束位置
        brace_count = 1
        j = brace_start + 1
        n = len(content)
        in_string = False

        while j < n and brace_count > 0:
            c = content[j]

            if in_string:
                if c == '\\' and j + 1 < n:
                    j += 2
                    continue
                if c == '"':
                    in_string = False
            else:
                if c == '"':
                    in_string = True
                elif c == '{':
                    brace_count += 1
                elif c == '}':
                    brace_count -= 1
            j += 1

        func_body = content[brace_start + 1:j - 1]
        full_func = content[match.start():j]

        functions[func_name] = {
            "signature": func_signature,
            "body": func_body,
            "full": full_func,
            "start": match.start(),
            "end": j
        }

    return functions


def replace_function_in_content(content: str, func_name: str, new_func_code: str) -> str:
    """
    在内容中替换指定函数的代码
    """
    functions = extract_functions_from_content(content)

    if func_name not in functions:
        return content

    func_info = functions[func_name]
    return content[:func_info["start"]] + new_func_code + content[func_info["end"]:]


def run_function_level_tests(
    project_name: str,
    c2rust_dir: Path,
    method_name: str,  # "c2saferrust", "evolc2rust", etc.
    method_rs_files: List[Path],
    tests_base_dir: Path,
    timeout: int = 180
) -> Dict[str, Any]:
    """
    函数级测试验证：
    1. 以 c2rust 翻译为基准
    2. 逐个将其他方法的函数替换进去
    3. 运行单元测试验证每个函数的正确性

    参数:
        project_name: 项目名
        c2rust_dir: c2rust 翻译目录
        method_name: 要测试的方法名
        method_rs_files: 该方法翻译的 .rs 文件列表
        tests_base_dir: 测试文件基础目录
        timeout: 超时时间
    """
    import shutil
    import tempfile

    result = {
        "method": method_name,
        "total_functions": 0,
        "matched_functions": 0,
        "tested_functions": 0,
        "passed_functions": 0,
        "function_pass_rate": 0.0,
        "functions_detail": {},
        "error": None
    }

    # 检查 c2rust 目录
    if not c2rust_dir.exists():
        result["error"] = f"c2rust directory not found: {c2rust_dir}"
        return result

    # 获取 c2rust 翻译文件
    c2rust_rs_files = list(c2rust_dir.glob("*.rs"))
    c2rust_rs_files = [f for f in c2rust_rs_files if f.name != "test.rs"]

    if not c2rust_rs_files:
        result["error"] = "No c2rust .rs files found"
        return result

    if not method_rs_files:
        result["error"] = "No method source files found"
        return result

    # 查找测试文件（使用 c2rust 的测试文件，因为我们用 c2rust 作为基准）
    test_file = tests_base_dir / project_name / "c2rust" / "test.rs"
    if not test_file.exists():
        test_file = tests_base_dir / project_name / "test.rs"

    if not test_file.exists():
        result["error"] = f"Test file not found for {project_name}"
        return result

    # 合并所有 c2rust 源文件内容
    c2rust_content = ""
    for rs_file in c2rust_rs_files:
        c2rust_content += rs_file.read_text(encoding='utf-8', errors='ignore') + "\n"

    # 提取 c2rust 中的函数
    c2rust_functions = extract_functions_from_content(c2rust_content)
    result["total_functions"] = len(c2rust_functions)

    # 合并所有目标方法的源文件内容
    method_content = ""
    for rs_file in method_rs_files:
        content = rs_file.read_text(encoding='utf-8', errors='ignore')
        # 移除内部属性
        content = re.sub(r'^#!\[allow\([^\]]+\)\]\s*\n?', '', content, flags=re.MULTILINE)
        content = re.sub(r'^#!\[feature\([^\]]+\)\]\s*\n?', '', content, flags=re.MULTILINE)
        content = re.sub(r'^#!\[register_tool\([^\]]+\)\]\s*\n?', '', content, flags=re.MULTILINE)
        method_content += content + "\n"

    # 提取目标方法中的函数
    method_functions = extract_functions_from_content(method_content)

    # 创建临时项目目录
    temp_dir = tempfile.mkdtemp(prefix=f"{method_name}_func_test_")

    try:
        temp_project = Path(temp_dir) / project_name
        src_dir = temp_project / "src"
        src_dir.mkdir(parents=True, exist_ok=True)

        # 创建 Cargo.toml
        cargo_toml = f'''[package]
name = "{project_name.replace("-", "_")}_func_test"
version = "0.1.0"
edition = "2018"

[lib]
path = "src/lib.rs"

[dependencies]
libc = "0.2"
'''
        (temp_project / "Cargo.toml").write_text(cargo_toml)

        # 创建 rust-toolchain.toml
        toolchain = '''[toolchain]
channel = "nightly-2022-08-08"
'''
        (temp_project / "rust-toolchain.toml").write_text(toolchain)

        # 准备测试文件内容
        test_content = test_file.read_text(encoding='utf-8')
        test_content = re.sub(r'use crate::', 'use super::', test_content)
        test_content = re.sub(r'use crate::(\w+)::', r'use super::\1::', test_content)
        # 移除 mod 声明（如 mod src_qsort）
        test_content = re.sub(r'^use super::\w+::', 'use super::', test_content, flags=re.MULTILINE)

        # 对每个函数进行测试
        for func_name in c2rust_functions:
            func_result = {
                "in_method": False,
                "tested": False,
                "compile_ok": False,
                "passed": False,
                "tests_passed": 0,
                "tests_failed": 0,
                "error": None
            }

            # 检查目标方法是否有这个函数
            if func_name not in method_functions:
                func_result["error"] = "Function not found in target method"
                result["functions_detail"][func_name] = func_result
                continue

            func_result["in_method"] = True
            result["matched_functions"] += 1

            # 创建测试项目：用目标方法的函数替换 c2rust 中的函数
            test_code = c2rust_content
            test_code = replace_function_in_content(
                test_code,
                func_name,
                method_functions[func_name]["full"]
            )

            # 添加必要的 allow 属性和 features
            lib_content = '''#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(c_variadic)]
#![feature(label_break_value)]
#![feature(extern_types)]
#![feature(linkage)]
#![register_tool(c2rust)]
#![feature(register_tool)]

''' + test_code + '''

#[cfg(test)]
mod test_funcs;
'''
            (src_dir / "lib.rs").write_text(lib_content, encoding='utf-8')
            (src_dir / "test_funcs.rs").write_text(test_content, encoding='utf-8')

            # 运行 cargo test
            try:
                proc = subprocess.run(
                    ["cargo", "test", "--no-fail-fast", "--", "--test-threads=1"],
                    cwd=temp_project,
                    capture_output=True,
                    text=True,
                    timeout=timeout,
                    env={**os.environ, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
                )

                func_result["tested"] = True
                result["tested_functions"] += 1

                output = proc.stdout + proc.stderr

                # 检查编译是否成功
                if "error[" in output or ("error:" in output and "could not compile" in output):
                    func_result["compile_ok"] = False
                    func_result["error"] = "Compilation failed"
                else:
                    func_result["compile_ok"] = True

                    # 解析测试结果
                    test_pattern = r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored'
                    matches = re.findall(test_pattern, output)

                    for match in matches:
                        func_result["tests_passed"] += int(match[0])
                        func_result["tests_failed"] += int(match[1])

                    # 判断是否通过（所有测试都通过）
                    if func_result["tests_failed"] == 0 and func_result["tests_passed"] > 0:
                        func_result["passed"] = True
                        result["passed_functions"] += 1

            except subprocess.TimeoutExpired:
                func_result["error"] = f"Timeout after {timeout}s"
            except Exception as e:
                func_result["error"] = str(e)

            result["functions_detail"][func_name] = func_result

        # 计算通过率
        if result["tested_functions"] > 0:
            result["function_pass_rate"] = result["passed_functions"] / result["tested_functions"]

    except Exception as e:
        result["error"] = str(e)
        import traceback
        result["traceback"] = traceback.format_exc()

    finally:
        # 清理临时目录
        try:
            shutil.rmtree(temp_dir)
        except Exception:
            pass

    return result


def run_method_function_tests(
    project_name: str,
    base_dir: Path,
    method_name: str,
    method_batch_dir: Optional[Path] = None,
    timeout: int = 180
) -> Dict[str, Any]:
    """
    对指定方法运行函数级测试

    参数:
        project_name: 项目名
        base_dir: rust_tests 基础目录
        method_name: 方法名 (c2saferrust, evolc2rust, etc.)
        method_batch_dir: 方法的批次目录（用于 c2saferrust）
        timeout: 超时时间
    """
    c2rust_dir = base_dir / project_name / "c2rust"

    # 根据方法名获取源文件
    method_rs_files = []

    if method_name == "c2saferrust" and method_batch_dir:
        # c2saferrust 的文件在批次目录中
        wip_dir = method_batch_dir / project_name / "input" / f"{project_name}_WIP"
        if wip_dir.exists():
            # 获取所有 .rs 文件，包括 c2rust-lib.rs
            method_rs_files = [f for f in wip_dir.glob("*.rs") if f.name not in ["build.rs", "test.rs"]]
    else:
        # 其他方法在 rust_tests 目录下
        method_dir = base_dir / project_name / method_name
        if method_dir.exists():
            method_rs_files = [f for f in method_dir.glob("*.rs") if f.name != "test.rs"]

    if not method_rs_files:
        return {
            "method": method_name,
            "error": f"No source files found for {method_name}",
            "total_functions": 0,
            "tested": False,
            "compile_ok": False,
            "tests_passed": 0,
            "tests_failed": 0,
            "tests_total": 0,
            "test_pass_rate": 0.0,
            "functions_detail": {}
        }

    return run_function_level_tests(
        project_name=project_name,
        c2rust_dir=c2rust_dir,
        method_name=method_name,
        method_rs_files=method_rs_files,
        tests_base_dir=base_dir,
        timeout=timeout
    )


def analyze_c2saferrust_batch(
    batch_dir: Path,
    base_dir: Path,
    projects: List[str],
    timeout: int = 180
) -> Dict[str, Any]:
    """
    分析整个 c2saferrust 批次的函数级测试结果
    """
    result = {
        "batch_dir": str(batch_dir),
        "timestamp": datetime.now().isoformat(),
        "method": "c2saferrust",
        "projects": {},
        "summary": {}
    }

    total_functions = 0
    matched_functions = 0
    tested_functions = 0
    passed_functions = 0

    for project_name in projects:
        print(f"  测试 {project_name}...", end=" ", flush=True)
        proj_result = run_method_function_tests(
            project_name=project_name,
            base_dir=base_dir,
            method_name="c2saferrust",
            method_batch_dir=batch_dir,
            timeout=timeout
        )

        result["projects"][project_name] = proj_result

        total_functions += proj_result.get("total_functions", 0)
        matched_functions += proj_result.get("matched_functions", 0)
        tested_functions += proj_result.get("tested_functions", 0)
        passed_functions += proj_result.get("passed_functions", 0)

        # 打印进度
        rate = proj_result.get("function_pass_rate", 0)
        tested = proj_result.get("tested_functions", 0)
        passed = proj_result.get("passed_functions", 0)
        print(f"{passed}/{tested} 通过 ({rate:.1%})")

    result["summary"] = {
        "total_projects": len(projects),
        "total_functions": total_functions,
        "matched_functions": matched_functions,
        "tested_functions": tested_functions,
        "passed_functions": passed_functions,
        "overall_pass_rate": passed_functions / tested_functions if tested_functions > 0 else 0.0
    }

    return result


# ============================================================================
# 主函数
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="分析 c2rust 翻译的 Rust 代码的编译率、unsafe 代码比例和 clippy 警告"
    )
    parser.add_argument(
        "--base-dir",
        type=Path,
        default=BASE_DIR,
        help=f"rust_tests 目录路径 (默认: {BASE_DIR})"
    )
    parser.add_argument(
        "--projects",
        type=str,
        default=",".join(ALL_PROJECTS),
        help=f"要分析的项目列表，逗号分隔 (默认: {','.join(ALL_PROJECTS)})"
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        help="输出 JSON 文件路径 (默认: 输出到 base_dir/c2rust_analysis.json)"
    )
    parser.add_argument(
        "--pretty",
        action="store_true",
        default=True,
        help="格式化 JSON 输出"
    )
    parser.add_argument(
        "--run-clippy",
        action="store_true",
        help="运行 cargo clippy 并统计警告数"
    )
    parser.add_argument(
        "--analyze-unsafe",
        action="store_true",
        default=True,
        help="分析 unsafe 代码比例 (默认开启)"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="运行所有分析 (等价于 --run-clippy --analyze-unsafe --run-c2rust-tests)"
    )
    parser.add_argument(
        "--run-c2rust-tests",
        action="store_true",
        help="运行 c2rust 外部测试（直接调用 Rust 函数）"
    )
    parser.add_argument(
        "--c2rust-tests-dir",
        type=Path,
        default=Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/test_module/rust_tests"),
        help="c2rust 外部测试代码目录（包含各项目的 c2rust/test.rs 文件）"
    )
    parser.add_argument(
        "--test-method",
        type=str,
        help="测试指定翻译方法的函数正确性 (c2saferrust, evolc2rust, etc.)"
    )
    parser.add_argument(
        "--method-batch-dir",
        type=Path,
        help="翻译方法的批次目录路径 (用于 c2saferrust)"
    )
    parser.add_argument(
        "--test-timeout",
        type=int,
        default=180,
        help="单个函数测试的超时时间（秒）"
    )

    args = parser.parse_args()

    run_clippy = args.run_clippy or args.all
    analyze_unsafe = args.analyze_unsafe or args.all
    run_c2rust_tests_flag = getattr(args, 'run_c2rust_tests', False) or args.all
    c2rust_tests_dir = args.c2rust_tests_dir if run_c2rust_tests_flag else None

    projects = [p.strip() for p in args.projects.split(",")]

    # 如果指定了 --test-method，运行函数级测试
    if args.test_method:
        print(f"函数级测试验证")
        print("=" * 70)
        print(f"翻译方法: {args.test_method}")
        print(f"基础目录: {args.base_dir}")
        if args.method_batch_dir:
            print(f"批次目录: {args.method_batch_dir}")
        print(f"项目列表: {', '.join(projects)}")
        print(f"测试超时: {args.test_timeout}s")
        print("=" * 70)

        if args.test_method == "c2saferrust":
            if not args.method_batch_dir:
                # 使用默认的最新批次
                runs_dir = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/c2saferrust/runs")
                batch_dirs = sorted([d for d in runs_dir.iterdir() if d.is_dir() and d.name.startswith("deepseek_batch_")])
                if batch_dirs:
                    args.method_batch_dir = batch_dirs[-1]
                    print(f"使用最新批次: {args.method_batch_dir}")
                else:
                    print("错误: 未找到 c2saferrust 批次目录", file=sys.stderr)
                    sys.exit(1)

            result = analyze_c2saferrust_batch(
                batch_dir=args.method_batch_dir,
                base_dir=args.base_dir,
                projects=projects,
                timeout=args.test_timeout
            )
        else:
            # 其他方法
            result = {
                "method": args.test_method,
                "base_dir": str(args.base_dir),
                "timestamp": datetime.now().isoformat(),
                "projects": {},
                "summary": {}
            }

            total_functions = 0
            total_tested = 0
            total_passed = 0

            for project_name in projects:
                proj_result = run_method_function_tests(
                    project_name=project_name,
                    base_dir=args.base_dir,
                    method_name=args.test_method,
                    method_batch_dir=args.method_batch_dir,
                    timeout=args.test_timeout
                )
                result["projects"][project_name] = proj_result
                total_functions += proj_result.get("total_functions", 0)
                total_tested += proj_result.get("tested_functions", 0)
                total_passed += proj_result.get("passed_functions", 0)

            result["summary"] = {
                "total_projects": len(projects),
                "total_functions": total_functions,
                "tested_functions": total_tested,
                "passed_functions": total_passed,
                "overall_pass_rate": total_passed / total_tested if total_tested > 0 else 0.0
            }

        # 输出结果
        output_path = args.output or (args.base_dir / f"{args.test_method}_func_test_results.json")
        indent = 2 if args.pretty else None
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(result, f, ensure_ascii=False, indent=indent)

        print(f"\n结果已保存到: {output_path}")

        # 打印摘要
        summary = result["summary"]
        print("\n" + "=" * 70)
        print(f"{args.test_method} 函数级测试摘要")
        print("=" * 70)
        print(f"项目数: {summary['total_projects']}")
        print(f"c2rust 总函数数: {summary['total_functions']}")
        print(f"匹配函数数: {summary.get('matched_functions', 0)}")
        print(f"测试函数数: {summary.get('tested_functions', 0)}")
        print(f"通过函数数: {summary.get('passed_functions', 0)}")
        print(f"函数通过率: {summary.get('overall_pass_rate', 0):.1%}")
        print("=" * 70)

        # 打印详细表格
        print("\n【函数级测试详情】:")
        print("-" * 100)
        print(f"{'项目':<12} {'c2rust函数':<12} {'匹配':<8} {'测试':<8} {'通过':<8} {'通过率':<12} {'错误':<30}")
        print("-" * 100)
        for proj_name in projects:
            proj_data = result["projects"].get(proj_name, {})
            total = proj_data.get("total_functions", 0)
            matched = proj_data.get("matched_functions", 0)
            tested = proj_data.get("tested_functions", 0)
            passed = proj_data.get("passed_functions", 0)
            rate = proj_data.get("function_pass_rate", 0)
            error = proj_data.get("error", "-")
            if error is None:
                error = "-"
            print(f"{proj_name:<12} {total:<12} {matched:<8} {tested:<8} {passed:<8} {rate:<12.1%} {str(error)[:30]:<30}")
        print("-" * 100)

        # 打印每个项目的函数详情
        for proj_name in projects:
            proj_data = result["projects"].get(proj_name, {})
            funcs_detail = proj_data.get("functions_detail", {})
            if funcs_detail:
                print(f"\n  {proj_name} 函数详情:")
                for func_name, func_info in funcs_detail.items():
                    if func_info.get("in_method"):
                        if func_info.get("compile_ok"):
                            status = "✓" if func_info.get("passed") else "✗"
                        else:
                            status = "⚠"  # 编译失败
                    else:
                        status = "-"  # 未匹配
                    print(f"    {status} {func_name}")

        sys.exit(0)

    print(f"C2Rust 翻译结果分析")
    print("=" * 70)
    print(f"基础目录: {args.base_dir}")
    print(f"项目列表: {', '.join(projects)}")
    if run_clippy:
        print("  - 运行 cargo clippy")
    if analyze_unsafe:
        print("  - 分析 unsafe 代码")
    if run_c2rust_tests_flag:
        print(f"  - 运行 c2rust 外部测试 (测试目录: {c2rust_tests_dir})")
    print("=" * 70)

    # 分析
    result = analyze_all_projects(
        args.base_dir,
        projects,
        run_clippy=run_clippy,
        analyze_unsafe=analyze_unsafe,
        run_c2rust_tests_flag=run_c2rust_tests_flag,
        c2rust_tests_dir=c2rust_tests_dir
    )

    # 输出
    output_path = args.output or (args.base_dir / "c2rust_analysis.json")

    indent = 2 if args.pretty else None
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(result, f, ensure_ascii=False, indent=indent)

    print(f"\n分析结果已保存到: {output_path}")

    # 打印摘要
    summary = result["summary"]
    print("\n" + "=" * 70)
    print("C2Rust 编译率分析摘要")
    print("=" * 70)
    print(f"分析时间: {result.get('timestamp', 'N/A')}")
    print(f"项目数: {summary['total_projects']}")
    print(f"项目编译通过: {summary['projects_compiled']} / {summary['total_projects']} ({summary['project_compile_rate']:.1%})")
    print(f"总代码行数: {summary['total_code_lines']}")

    if summary.get("error_categories"):
        print("\n错误类别分布:")
        cat_names = {
            "self_keyword": "self关键字冲突",
            "missing_macro": "缺少宏定义",
            "type_mismatch": "类型不匹配",
            "missing_type": "缺少类型定义",
            "syntax_error": "语法错误",
            "brace_mismatch": "括号不匹配",
            "lifetime_error": "生命周期错误",
            "borrow_error": "借用错误",
            "undefined_function": "未定义函数/变量",
            "other": "其他错误"
        }
        for cat, count in sorted(summary["error_categories"].items(), key=lambda x: -x[1]):
            cat_name = cat_names.get(cat, cat)
            print(f"  {cat_name}: {count}")

    if "unsafe_summary" in summary:
        us = summary["unsafe_summary"]
        print("\n" + "-" * 70)
        print("Unsafe 代码汇总:")
        print(f"  Unsafe 项总数: {us['total_unsafe_items']}")
        print(f"  Unsafe 总行数: {us.get('total_unsafe_total_lines', 0)}")
        print(f"  Unsafe 总率: {us.get('unsafe_total_ratio', 0.0):.2%}")
        print(f"    - 关键字行数: {us.get('total_unsafe_keyword_lines', 0)} ({us.get('unsafe_keyword_ratio', 0.0):.2%})")
        print(f"    - 作用域行数: {us.get('total_unsafe_context_lines', 0)} ({us.get('unsafe_context_ratio', 0.0):.2%})")
        print(f"  Unsafe 行数(旧): {us.get('total_unsafe_lines', 0)} (仅块/unsafe fn 体内)")
        print(f"  Unsafe 率(旧): {us.get('unsafe_ratio', 0.0):.2%}")
        print(f"  Unsafe 密度: {us['unsafe_density_per_kloc']:.2f} 项/千行代码")

    if "clippy_summary" in summary:
        cs = summary["clippy_summary"]
        print("\n" + "-" * 70)
        print("Clippy 警告汇总:")
        print(f"  分析的项目数: {cs['projects_analyzed']}")
        print(f"  Clippy warnings: {cs.get('total_clippy_warnings', cs.get('total_warnings', 0))}")
        print(f"  Rustc warnings:  {cs.get('total_rustc_warnings', 0)}")
        print(f"  总 warnings:     {cs.get('total_warnings_including_rustc', 0)}")
        print(f"  Errors:          {cs.get('total_errors', 0)}")
        print(f"  平均每项目 Clippy: {cs.get('avg_clippy_warnings_per_project', 0):.1f}")
        print(f"  平均每项目 总计:   {cs.get('avg_total_warnings_per_project', 0):.1f}")

    if "c2rust_test_summary" in summary:
        cts = summary["c2rust_test_summary"]
        print("\n" + "-" * 70)
        print("C2Rust 外部测试汇总:")
        print(f"  可执行测试的项目: {cts['projects_with_tests']}")
        print(f"  测试总数: {cts['total_tests']}")
        print(f"  通过: {cts['tests_passed']} / {cts['total_tests']}")
        print(f"  失败: {cts['tests_failed']}")
        print(f"  测试通过率: {cts['overall_test_pass_rate']:.1%}")

    print("=" * 70)

    # 表格1: 编译状态
    print("\n【表1】编译状态:")
    print("-" * 50)
    print(f"{'项目':<12} {'源文件数':<10} {'代码行数':<10} {'编译通过':<10}")
    print("-" * 50)
    for proj_name in projects:
        proj_data = result["projects"].get(proj_name, {})
        if proj_data:
            num_files = len(proj_data.get("source_files", []))
            code_lines = proj_data.get("code_lines", 0)
            passed = "Yes" if proj_data.get("cargo_check_passed") else "No"
            print(f"{proj_name:<12} {num_files:<10} {code_lines:<10} {passed:<10}")
        else:
            print(f"{proj_name:<12} {'-':<10} {'-':<10} {'-':<10}")
    print("-" * 50)

    # 表格2: Unsafe 代码分析
    if analyze_unsafe:
        print("\n【表2】Unsafe 代码分析:")
        print("-" * 95)
        print(f"{'项目':<12} {'代码行数':<10} {'Unsafe总行':<10} {'Unsafe块':<10} {'Unsafe函数':<12} {'Unsafe项总数':<14} {'Unsafe总率(%)':<12}")
        print("-" * 95)
        for proj_name in projects:
            proj_data = result["projects"].get(proj_name, {})
            ua = proj_data.get("unsafe_analysis", {})
            if ua and not ua.get("error"):
                code_lines = ua.get("code_lines", 0)
                unsafe_lines = ua.get("unsafe_total_lines", 0)
                unsafe_blocks = ua.get("unsafe_blocks", 0)
                unsafe_funcs = ua.get("unsafe_functions", 0)
                unsafe_total = ua.get("total_unsafe_items", 0)
                unsafe_ratio = ua.get("unsafe_total_ratio", 0) * 100
                print(f"{proj_name:<12} {code_lines:<10} {unsafe_lines:<10} {unsafe_blocks:<10} {unsafe_funcs:<12} {unsafe_total:<14} {unsafe_ratio:<12.2f}")
            else:
                print(f"{proj_name:<12} {'-':<10} {'-':<10} {'-':<10} {'-':<12} {'-':<14} {'-':<12}")
        print("-" * 95)

    # 表格3: Clippy 警告
    if run_clippy:
        print("\n【表3】Clippy 警告:")
        header3 = f"{'项目':<12} {'Clippy':<8} {'Rustc':<8} {'总计':<8} {'错误':<6}"
        print("-" * len(header3))
        print(header3)
        print("-" * len(header3))
        for proj_name in projects:
            proj_data = result["projects"].get(proj_name, {})
            cr = proj_data.get("clippy_results", {})
            if cr and cr.get("executed"):
                clippy_warn = cr.get("warning_count", 0)
                rustc_warn = cr.get("rustc_warning_count", 0)
                total_warn = cr.get("warning_count_total", clippy_warn + rustc_warn)
                err_cnt = cr.get("error_count", 0)
                print(f"{proj_name:<12} {clippy_warn:<8} {rustc_warn:<8} {total_warn:<8} {err_cnt:<6}")
            else:
                error = cr.get("error") if isinstance(cr, dict) else None
                if error:
                    print(f"{proj_name:<12} {'-':<8} {'-':<8} {'-':<8} {'-':<6}")
                else:
                    print(f"{proj_name:<12} {'-':<8} {'-':<8} {'-':<8} {'-':<6}")
        print("-" * len(header3))

    # 表格4: C2Rust 外部测试结果
    if run_c2rust_tests_flag:
        print("\n【表4】C2Rust 外部测试结果:")
        print("-" * 50)
        print(f"{'项目':<12} {'通过':<8} {'失败':<8} {'总数':<8} {'通过率':<10}")
        print("-" * 50)
        for proj_name in projects:
            proj_data = result["projects"].get(proj_name, {})
            ctr = proj_data.get("c2rust_test_results", {})
            if ctr and ctr.get("executed"):
                passed = ctr.get("tests_passed", 0)
                failed = ctr.get("tests_failed", 0)
                total = ctr.get("total_tests", 0)
                rate = ctr.get("pass_rate", 0)
                print(f"{proj_name:<12} {passed:<8} {failed:<8} {total:<8} {rate:<10.1%}")
            else:
                error = ctr.get("error", "未执行") if ctr else "未执行"
                print(f"{proj_name:<12} {'-':<8} {'-':<8} {'-':<8} {str(error)[:10]:<10}")
        print("-" * 50)

    # 表格5: 编译错误详情
    print("\n【表5】编译错误详情:")
    print("-" * 70)
    for proj_name in projects:
        proj_data = result["projects"].get(proj_name, {})
        if not proj_data.get("cargo_check_passed"):
            error_analysis = proj_data.get("error_analysis", {})
            total_errors = error_analysis.get("total_errors", 0)
            categories = error_analysis.get("categories", {})
            if total_errors > 0:
                print(f"\n{proj_name}: {total_errors} 个错误")
                for cat, count in sorted(categories.items(), key=lambda x: -x[1]):
                    cat_name = cat_names.get(cat, cat)
                    print(f"  - {cat_name}: {count}")
    print("-" * 70)


if __name__ == "__main__":
    main()
