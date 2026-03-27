#!/usr/bin/env python3
"""
OHOS(5) analysis for **C2Rust baseline** on Huawei self-contained modules.

We reuse the existing c2rust-converted crates under:
  ComparisonMethod/c2saferrust/ohos_converted/<project>/

Metrics (truthful, no cheating):
- cargo check pass/fail (without writing target/ into the crate)
- Clippy vs Rustc warnings (JSON diagnostics)
- unsafe rate (keyword/context line union, excluding comments/strings)
- incremental compilation (best-effort per-function isolated compile in a temp copy)
- OHOS gtest unit tests from the source tree (best-effort), linked against the translated staticlib
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from bisect import bisect_right
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


DISPLAY_PROJECT_ORDER: List[str] = [
    "common__89d5ecaafdff",
    "core__ef5242b7ab08",
    "host__25c1898e1626",
    "manager__c248934e0221",
    "sapm__193cdeb43a97",
]
_ORDER_INDEX = {n: i for i, n in enumerate(DISPLAY_PROJECT_ORDER)}

DEFAULT_BASE_DIR = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/c2saferrust/ohos_converted")
DEFAULT_HUAWEI_PROJECTS_TSV = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/unified/huawei_projects.tsv")
DEFAULT_OHOS_ROOT = Path("/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony")


def iter_projects_in_display_order(projects: Dict[str, Any]) -> List[Tuple[str, Any]]:
    return sorted(projects.items(), key=lambda kv: (_ORDER_INDEX.get(kv[0], 1_000_000), kv[0]))


def load_huawei_projects_map(tsv_path: Path) -> Dict[str, Path]:
    mapping: Dict[str, Path] = {}
    if not tsv_path.exists():
        return mapping
    for raw in tsv_path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        name = parts[0].strip()
        path = parts[1].strip()
        if name.lower() in ("name", "project", "project_name") and path.lower() in ("path", "dir", "directory"):
            continue
        if not name or not path:
            continue
        mapping[name] = Path(path).expanduser().resolve()
    return mapping


def _run_cmd_capture(cmd: List[str], cwd: Path, env: Dict[str, str], timeout: int) -> Tuple[int, str, str]:
    try:
        p = subprocess.run(
            cmd,
            cwd=cwd,
            env=env,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return p.returncode, p.stdout or "", p.stderr or ""
    except subprocess.TimeoutExpired:
        return 124, "", f"Timeout after {timeout}s"


def run_cargo_check(project_dir: Path, timeout: int = 300) -> Dict[str, Any]:
    res: Dict[str, Any] = {"executed": False, "passed": False, "error": None, "stdout": "", "stderr": ""}
    if not (project_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="c2rust_ohos_cargo_target_") as td:
        env = {
            **os.environ,
            "CARGO_TARGET_DIR": td,
            "RUSTFLAGS": "-Awarnings",
            "RUSTC_BOOTSTRAP": "1",
            "RUST_BACKTRACE": "0",
        }
        rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["passed"] = rc == 0
        res["stdout"] = out[-8000:]
        res["stderr"] = err[-12000:]
        if rc != 0:
            res["error"] = "cargo check failed"
    return res


def run_cargo_clippy(project_dir: Path, timeout: int = 600) -> Dict[str, Any]:
    res: Dict[str, Any] = {
        "executed": False,
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
        "error": None,
        "stdout": "",
        "stderr": "",
    }
    if not (project_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="c2rust_ohos_clippy_target_") as td:
        env = {**os.environ, "CARGO_TARGET_DIR": td, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        cmd = ["cargo", "clippy", "--offline", "--message-format=json", "--", "-W", "clippy::all"]
        rc, out, err = _run_cmd_capture(cmd, cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["stdout"] = out[-200000:]
        res["stderr"] = err[-200000:]

        clippy_warn = 0
        rustc_warn = 0
        err_cnt = 0
        for line in out.splitlines():
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
            code = (code_obj.get("code") or "")
            if level == "warning":
                if code.startswith("clippy::"):
                    clippy_warn += 1
                else:
                    rustc_warn += 1
            elif level == "error":
                err_cnt += 1

        res["warning_count"] = clippy_warn
        res["rustc_warning_count"] = rustc_warn
        res["warning_count_total"] = clippy_warn + rustc_warn
        res["error_count"] = err_cnt
        if rc != 0 and err_cnt > 0:
            res["error"] = "cargo clippy failed"
    return res


# =============================================================================
# Unsafe analysis (global keyword/context union) - same logic as other scripts
# =============================================================================

def _is_ident_char(ch: str) -> bool:
    return ch.isalnum() or ch == "_"


def _skip_line_comment(s: str, i: int) -> int:
    nl = s.find("\n", i + 2)
    return len(s) if nl == -1 else nl


def _skip_block_comment(s: str, i: int) -> int:
    depth = 1
    i += 2
    n = len(s)
    while i < n and depth > 0:
        if s.startswith("/*", i):
            depth += 1
            i += 2
        elif s.startswith("*/", i):
            depth -= 1
            i += 2
        else:
            i += 1
    return i


def _skip_normal_string(s: str, i: int) -> int:
    n = len(s)
    i += 1
    while i < n:
        c = s[i]
        if c == "\\" and i + 1 < n:
            i += 2
            continue
        if c == '"':
            return i + 1
        i += 1
    return i


def _try_skip_raw_string(s: str, i: int) -> Optional[int]:
    n = len(s)
    if s.startswith("br", i):
        i += 2
    elif s.startswith("r", i):
        i += 1
    else:
        return None

    hash_count = 0
    while i < n and s[i] == "#":
        hash_count += 1
        i += 1
    if i >= n or s[i] != '"':
        return None
    i += 1

    terminator = '"' + ("#" * hash_count)
    end_pos = s.find(terminator, i)
    if end_pos == -1:
        return n
    return end_pos + len(terminator)


def _skip_ws_and_comments(s: str, i: int) -> int:
    n = len(s)
    while i < n:
        if s[i].isspace():
            i += 1
            continue
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
            i = _skip_block_comment(s, i)
            continue
        break
    return i


def _find_matching_brace(s: str, brace_start: int) -> Optional[int]:
    if brace_start >= len(s) or s[brace_start] != "{":
        return None
    depth = 1
    i = brace_start + 1
    n = len(s)
    while i < n and depth > 0:
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
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
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
        i += 1
    return i if depth == 0 else None


def _find_body_brace_or_decl_end(s: str, i: int) -> Optional[int]:
    n = len(s)
    while i < n:
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
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
        if c == "{":
            return i
        if c == ";":
            return None
        i += 1
    return None


def analyze_unsafe_global_in_content(content: str) -> Dict[str, int]:
    line_starts = [0]
    for idx, ch in enumerate(content):
        if ch == "\n":
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
        line += content.count("\n", i, to_i)
        i = to_i

    while i < n:
        c = content[i]
        if c == "\n":
            in_line_comment = False
            line += 1
            i += 1
            continue
        if in_line_comment:
            i += 1
            continue
        if block_comment_depth > 0:
            if content.startswith("/*", i):
                block_comment_depth += 1
                i += 2
                continue
            if content.startswith("*/", i):
                block_comment_depth -= 1
                i += 2
                continue
            i += 1
            continue
        if content.startswith("//", i):
            in_line_comment = True
            i += 2
            continue
        if content.startswith("/*", i):
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

        if content.startswith("unsafe", i):
            before_ok = (i == 0) or (not _is_ident_char(content[i - 1]))
            after_idx = i + 6
            after_ok = (after_idx >= n) or (not _is_ident_char(content[after_idx]))
            if before_ok and after_ok:
                line_has_unsafe_kw[line] = True
                unsafe_kw_occ += 1

                j = _skip_ws_and_comments(content, after_idx)
                if j < n:
                    if content[j] == "{":
                        body_start = j
                        body_end = _find_matching_brace(content, body_start)
                        if body_end is not None:
                            unsafe_spans.append((body_start, body_end))
                    elif content.startswith("fn", j) and (j + 2 >= n or not _is_ident_char(content[j + 2])):
                        body_start = _find_body_brace_or_decl_end(content, j + 2)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith("extern", j) and (j + 6 >= n or not _is_ident_char(content[j + 6])):
                        body_start = _find_body_brace_or_decl_end(content, j + 6)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith("impl", j) and (j + 4 >= n or not _is_ident_char(content[j + 4])):
                        body_start = _find_body_brace_or_decl_end(content, j + 4)
                        if body_start is not None:
                            body_end = _find_matching_brace(content, body_start)
                            if body_end is not None:
                                unsafe_spans.append((body_start, body_end))
                    elif content.startswith("trait", j) and (j + 5 >= n or not _is_ident_char(content[j + 5])):
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
        start_line = max(0, start_line)
        end_line = min(line_count - 1, end_line)
        for ln in range(start_line, end_line + 1):
            line_in_unsafe_ctx[ln] = True

    code_lines = sum(1 for v in line_has_code if v)
    unsafe_keyword_lines = sum(1 for ln in range(line_count) if line_has_code[ln] and line_has_unsafe_kw[ln])
    unsafe_context_lines = sum(1 for ln in range(line_count) if line_has_code[ln] and line_in_unsafe_ctx[ln])
    unsafe_total_lines = sum(
        1 for ln in range(line_count) if line_has_code[ln] and (line_has_unsafe_kw[ln] or line_in_unsafe_ctx[ln])
    )
    return {
        "code_lines": code_lines,
        "unsafe_keyword_occurrences": unsafe_kw_occ,
        "unsafe_keyword_lines": unsafe_keyword_lines,
        "unsafe_context_lines": unsafe_context_lines,
        "unsafe_total_lines": unsafe_total_lines,
    }


def analyze_unsafe_code(rs_files: List[Path]) -> Dict[str, Any]:
    res: Dict[str, Any] = {
        "files_analyzed": 0,
        "code_lines": 0,
        "unsafe_total_lines": 0,
        "unsafe_total_ratio": 0.0,
        "unsafe_keyword_occurrences": 0,
        "unsafe_keyword_lines": 0,
        "unsafe_context_lines": 0,
        "error": None,
    }
    res["files_analyzed"] = len(rs_files)
    for p in rs_files:
        try:
            txt = p.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        m = analyze_unsafe_global_in_content(txt)
        res["code_lines"] += int(m.get("code_lines", 0) or 0)
        res["unsafe_total_lines"] += int(m.get("unsafe_total_lines", 0) or 0)
        res["unsafe_keyword_occurrences"] += int(m.get("unsafe_keyword_occurrences", 0) or 0)
        res["unsafe_keyword_lines"] += int(m.get("unsafe_keyword_lines", 0) or 0)
        res["unsafe_context_lines"] += int(m.get("unsafe_context_lines", 0) or 0)
    if res["code_lines"] > 0:
        res["unsafe_total_ratio"] = res["unsafe_total_lines"] / res["code_lines"]
    return res


# =============================================================================
# OHOS gtest unit tests (best-effort)
# =============================================================================

def _default_ohos_test_include_dirs(ohos_root: Path) -> List[Path]:
    # Keep this list aligned with the simcrat OHOS harness (practical + minimal).
    dirs: List[Path] = []

    # HDF public/inner headers
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/utils")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/core")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/uhdf")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host/uhdf")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/ipc")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/hdi")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host/shared")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/shared")

    # Framework internal headers that unit tests often include directly.
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/shared/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/manager/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/host/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/utils/include")

    # uhdf2 adapter headers used by manager tests
    dirs.append(ohos_root / "drivers/hdf_core/adapter/uhdf2/ipc/include")

    # HDF unit test headers (e.g., hdf_uhdf_test.h)
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/pm")
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/manager")

    # IPC/SAMGR headers pulled by some manager tests.
    dirs.append(ohos_root / "foundation/communication/ipc/interfaces/innerkits/ipc_core/include")
    dirs.append(ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/samgr_proxy/include")
    dirs.append(ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/dynamic_cache/include")
    dirs.append(ohos_root / "base/notification/eventhandler/interfaces/inner_api")
    dirs.append(ohos_root / "base/notification/eventhandler/frameworks/eventhandler/include")
    dirs.append(ohos_root / "base/hiviewdfx/hisysevent/interfaces/native/innerkits/hisysevent/include")

    # securec
    dirs.append(ohos_root / "third_party/bounds_checking_function/include")

    # googletest
    dirs.append(ohos_root / "third_party/googletest/googletest/include")
    dirs.append(ohos_root / "third_party/googletest/googlemock/include")
    dirs.append(ohos_root / "third_party/googletest/googletest")  # for gtest-all.cc includes "src/..."

    # Extra includes sometimes needed by tests.
    dirs.append(ohos_root / "base/hiviewdfx/hilog/interfaces/native/innerkits/include")
    dirs.append(ohos_root / "commonlibrary/c_utils/base/include")

    return [d for d in dirs if d.is_dir()]


def _parse_gtest_counts(output: str) -> Tuple[int, int]:
    passed = 0
    failed = 0
    m = re.search(r"\[\s*PASSED\s*\]\s+(\d+)\s+tests?\b", output)
    if m:
        passed = int(m.group(1))
    m = re.search(r"\[\s*FAILED\s*\]\s+(\d+)\s+tests?\b", output)
    if m:
        failed = int(m.group(1))
    return passed, failed


def build_staticlib_from_crate(project_dir: Path, timeout: int = 600) -> Dict[str, Any]:
    res: Dict[str, Any] = {"executed": False, "ok": False, "staticlib": None, "error": None, "stdout": "", "stderr": ""}
    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.is_file():
        res["error"] = "Cargo.toml not found"
        return res

    txt = cargo_toml.read_text(encoding="utf-8", errors="ignore")
    pkg_name = None
    lib_name = None
    in_pkg = False
    in_lib = False
    for line in txt.splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            sec = s.strip("[]").strip()
            in_pkg = sec == "package"
            in_lib = sec == "lib"
            continue
        m = re.match(r'name\s*=\s*"([^"]+)"', s)
        if m and in_pkg and pkg_name is None:
            pkg_name = m.group(1)
        if m and in_lib and lib_name is None:
            lib_name = m.group(1)
    name = lib_name or pkg_name
    if not name:
        res["error"] = "Failed to parse crate name"
        return res
    libfile = f"lib{name.replace('-', '_')}.a"

    with tempfile.TemporaryDirectory(prefix="c2rust_ohos_staticlib_target_") as tgt:
        env = {**os.environ, "CARGO_TARGET_DIR": tgt, "RUSTC_BOOTSTRAP": "1", "RUST_BACKTRACE": "0"}
        rc, out, err = _run_cmd_capture(
            ["cargo", "build", "--offline", "--release"],
            cwd=project_dir,
            env=env,
            timeout=timeout,
        )
        res["executed"] = True
        res["stdout"] = out[-8000:]
        res["stderr"] = err[-12000:]
        if rc != 0:
            res["error"] = "cargo build staticlib failed"
            return res
        built = Path(tgt) / "release" / libfile
        if not built.is_file():
            res["error"] = f"staticlib not found: {built}"
            return res
        out_dir = Path(tempfile.mkdtemp(prefix="c2rust_ohos_staticlib_out_"))
        out_lib = out_dir / built.name
        shutil.copy(built, out_lib)
        res["staticlib"] = str(out_lib)
        res["ok"] = True
        return res


def run_ohos_unit_tests(
    *,
    project_name: str,
    source_project_dir: Path,
    translated_staticlib: Path,
    ohos_root: Path,
    timeout: int = 600,
) -> Dict[str, Any]:
    result: Dict[str, Any] = {"executed": False, "compiled": False, "tests_passed": 0, "tests_failed": 0, "total_tests": 0, "pass_rate": 0.0, "error": None}

    test_dir = source_project_dir / "test"
    if not test_dir.is_dir():
        result["error"] = "no test/ directory"
        return result

    # Collect unittest sources and sample sources (fixtures).
    test_srcs: List[Path] = []
    sample_srcs: List[Path] = []
    for p in test_dir.rglob("*"):
        if not p.is_file():
            continue
        if p.suffix.lower() not in (".c", ".cc", ".cpp", ".cxx"):
            continue
        parts_lower = [x.lower() for x in p.parts]
        if "sample" in parts_lower:
            sample_srcs.append(p)
        elif "unittest" in parts_lower:
            test_srcs.append(p)
    test_srcs.sort()
    sample_srcs.sort()
    if not test_srcs:
        result["error"] = "no unittest sources found"
        return result

    include_dirs: List[Path] = []
    for rel in ("include", "src", "test", "test/unittest", "test/unittest/common", "test/sample"):
        d = source_project_dir / rel
        if d.is_dir():
            include_dirs.append(d)
    include_dirs.extend(_default_ohos_test_include_dirs(ohos_root))
    include_flags = [f"-I{d}" for d in include_dirs]

    gtest_root = ohos_root / "third_party/googletest/googletest"
    gtest_all = gtest_root / "src/gtest-all.cc"
    gtest_main = gtest_root / "src/gtest_main.cc"
    # NOTE: OHOS' gtest-all.cc already includes hwext/gtest-ext.cc; compiling it separately causes ODR violations.

    with tempfile.TemporaryDirectory(prefix=f"c2rust_ohos_unittest_{project_name}_") as td:
        build_dir = Path(td)
        bin_path = build_dir / f"{project_name}_unittests"
        env = {**os.environ}
        env.setdefault("RUST_BACKTRACE", "0")

        # For host__ project: create expected *.z.so fixtures referenced by tests via dlopen (best-effort).
        lib_name_re = re.compile(r"\"(lib[^\"]+\\.z\\.so)\"")
        wanted_libs: List[str] = []
        try:
            for ts in test_srcs:
                wanted_libs.extend(lib_name_re.findall(ts.read_text(encoding="utf-8", errors="ignore")))
        except Exception:
            pass
        wanted_libs = sorted(set(wanted_libs))

        if project_name.startswith("host__") and wanted_libs:
            sample_map = {
                "vdi_sample1_driver": "libvdi_sample1_driver.z.so",
                "vdi_sample1_symbol": "libvdi_sample1_symbol.z.so",
                "vdi_sample2_driver": "libvdi_sample2_driver.z.so",
            }
            for stem, so_name in sample_map.items():
                if so_name not in wanted_libs:
                    continue
                src = next((p for p in sample_srcs if p.stem == stem), None)
                if not src:
                    continue
                out_so = build_dir / so_name
                cc = "gcc" if src.suffix.lower() == ".c" else "g++"
                cmd = [
                    cc,
                    "-shared",
                    "-fPIC",
                    *([] if cc == "gcc" else ["-std=c++17"]),
                    "-O2",
                    *include_flags,
                    str(src),
                    "-o",
                    str(out_so),
                ]
                rc, so_out, so_err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("fixtures", {})[so_name] = {
                    "built": rc == 0,
                    "cmd": cmd,
                    "stdout": so_out[-4000:],
                    "stderr": so_err[-8000:],
                }

            for so_name in wanted_libs:
                if not so_name.endswith("_error.z.so"):
                    continue
                out_so = build_dir / so_name
                dummy_c = build_dir / f"{so_name}.c"
                dummy_c.write_text("void __ohos_dummy(void) {}\n", encoding="utf-8")
                cmd = ["gcc", "-shared", "-fPIC", "-O2", str(dummy_c), "-o", str(out_so)]
                rc, so_out, so_err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("fixtures", {})[so_name] = {
                    "built": rc == 0,
                    "cmd": cmd,
                    "stdout": so_out[-4000:],
                    "stderr": so_err[-8000:],
                }

        # Build securec objects.
        securec_src_dir = ohos_root / "third_party/bounds_checking_function/src"
        securec_inc_dir = ohos_root / "third_party/bounds_checking_function/include"
        securec_objs: List[Path] = []
        if securec_src_dir.is_dir():
            obj_dir = build_dir / "securec_objs"
            obj_dir.mkdir(parents=True, exist_ok=True)
            for cfile in sorted([p for p in securec_src_dir.glob("*.c") if p.is_file()]):
                obj = obj_dir / (cfile.stem + ".o")
                cc_cmd = [
                    "gcc",
                    "-O2",
                    "-c",
                    f"-I{securec_inc_dir}" if securec_inc_dir.is_dir() else "",
                    f"-I{securec_src_dir}",
                    str(cfile),
                    "-o",
                    str(obj),
                ]
                cc_cmd = [x for x in cc_cmd if x]
                rc, _, _ = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                if rc != 0:
                    result["error"] = "securec build failed"
                    return result
                if obj.is_file():
                    securec_objs.append(obj)

        # Build OHOS unittest helper sources (provide HdfTestOpenService/HdfTestSendMsgToService, etc.).
        helper_objs: List[Path] = []
        helper_c = ohos_root / "drivers/hdf_core/framework/test/unittest/common/hdf_common_test.c"
        if helper_c.is_file():
            obj = build_dir / "hdf_common_test.o"
            cc_cmd = ["gcc", "-O2", "-c", *include_flags, str(helper_c), "-o", str(obj)]
            rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
            result.setdefault("ohos_test_helpers", []).append(
                {"src": str(helper_c), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
            )
            if rc != 0 or not obj.is_file():
                result["error"] = "ohos unittest helper build failed"
                return result
            helper_objs.append(obj)

        cmd: List[str] = [
            "g++",
            "-std=c++17",
            "-O2",
            *include_flags,
            str(gtest_all),
            str(gtest_main),
            *[str(p) for p in test_srcs],
            *[str(o) for o in helper_objs],
            *[str(p) for p in securec_objs],
            str(translated_staticlib),
            "-ldl",
            "-pthread",
            "-lm",
            "-o",
            str(bin_path),
        ]
        cmd = [x for x in cmd if x]
        rc, out, err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
        if rc != 0 or not bin_path.is_file():
            result["error"] = "gtest build failed"
            result["build_stderr"] = err[-12000:]
            return result

        result["compiled"] = True
        run_env = dict(env)
        run_env["LD_LIBRARY_PATH"] = f"{build_dir}:{run_env.get('LD_LIBRARY_PATH', '')}"
        run_cmd = [str(bin_path), "--gtest_color=no"]
        rc, rout, rerr = _run_cmd_capture(run_cmd, cwd=build_dir, env=run_env, timeout=timeout)
        result["executed"] = True
        passed, failed = _parse_gtest_counts(rout + "\n" + rerr)
        result["tests_passed"] = passed
        result["tests_failed"] = failed
        result["total_tests"] = passed + failed
        result["pass_rate"] = (passed / (passed + failed)) if (passed + failed) > 0 else 0.0
        if rc != 0 and result["total_tests"] == 0:
            result["error"] = "gtest run failed (no summary)"
        return result


# =============================================================================
# Incremental compilation: isolate each function by stubbing others in a temp copy
# =============================================================================

def verify_incremental_compilation(crate_dir: Path, timeout: int = 60) -> Dict[str, Any]:
    res: Dict[str, Any] = {"total_functions": 0, "compiled_functions": 0, "compile_rate": 0.0, "error": None}
    if not (crate_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="c2rust_ohos_inc_") as td:
        tmp = Path(td) / crate_dir.name
        shutil.copytree(crate_dir, tmp, ignore=shutil.ignore_patterns("target", ".git"))

        # Candidate files: all module files (exclude build.rs).
        rs_files = [p for p in tmp.glob("*.rs") if p.is_file() and p.name != "build.rs"]
        if not rs_files:
            res["error"] = "No .rs files found"
            return res

        originals: Dict[Path, str] = {p: p.read_text(encoding="utf-8", errors="ignore") for p in rs_files}

        def skip_ws(s: str, pos: int) -> int:
            while pos < len(s) and s[pos].isspace():
                pos += 1
            return pos

        def skip_line_comment(s: str, pos: int) -> int:
            nl = s.find("\n", pos + 2)
            return len(s) if nl == -1 else nl + 1

        def skip_block_comment(s: str, pos: int) -> int:
            depth = 1
            pos += 2
            while pos < len(s) and depth > 0:
                if s.startswith("/*", pos):
                    depth += 1
                    pos += 2
                elif s.startswith("*/", pos):
                    depth -= 1
                    pos += 2
                else:
                    pos += 1
            return pos

        def skip_string(s: str, pos: int) -> int:
            quote = s[pos]
            pos += 1
            while pos < len(s):
                c = s[pos]
                if c == "\\":
                    pos += 2
                    continue
                if c == quote:
                    return pos + 1
                pos += 1
            return pos

        def scan_balanced(s: str, pos: int, open_ch: str, close_ch: str) -> Optional[int]:
            if pos >= len(s) or s[pos] != open_ch:
                return None
            depth = 1
            pos += 1
            while pos < len(s) and depth > 0:
                if s.startswith("//", pos):
                    pos = skip_line_comment(s, pos)
                    continue
                if s.startswith("/*", pos):
                    pos = skip_block_comment(s, pos)
                    continue
                c = s[pos]
                if c in ('"', "'"):
                    pos = skip_string(s, pos)
                    continue
                if c == open_ch:
                    depth += 1
                elif c == close_ch:
                    depth -= 1
                pos += 1
            return pos if depth == 0 else None

        def find_fn_item_span(s: str, fn_kw_pos: int, fn_name: str) -> Optional[Tuple[int, int, int]]:
            pos = fn_kw_pos + 2
            pos = skip_ws(s, pos)
            if not s.startswith(fn_name, pos):
                return None
            pos += len(fn_name)
            pos = skip_ws(s, pos)
            if pos < len(s) and s[pos] == "<":
                end = scan_balanced(s, pos, "<", ">")
                if end is None:
                    return None
                pos = skip_ws(s, end)
            if pos >= len(s) or s[pos] != "(":
                return None
            end = scan_balanced(s, pos, "(", ")")
            if end is None:
                return None
            pos = skip_ws(s, end)

            angle = paren = bracket = 0
            while pos < len(s):
                if s.startswith("//", pos):
                    pos = skip_line_comment(s, pos)
                    continue
                if s.startswith("/*", pos):
                    pos = skip_block_comment(s, pos)
                    continue
                c = s[pos]
                if c in ('"', "'"):
                    pos = skip_string(s, pos)
                    continue
                if c == "(":
                    paren += 1
                elif c == ")":
                    paren = max(0, paren - 1)
                elif c == "[":
                    bracket += 1
                elif c == "]":
                    bracket = max(0, bracket - 1)
                elif c == "<":
                    angle += 1
                elif c == ">":
                    angle = max(0, angle - 1)
                elif c == "{" and paren == 0 and angle == 0 and bracket == 0:
                    body_start = pos
                    body_end = scan_balanced(s, pos, "{", "}")
                    if body_end is None:
                        return None
                    line_start = s.rfind("\n", 0, fn_kw_pos)
                    item_start = 0 if line_start == -1 else line_start + 1
                    return (item_start, body_start, body_end)
                elif c == ";" and paren == 0 and angle == 0 and bracket == 0:
                    return None
                pos += 1
            return None

        fn_pat = re.compile(r"\bfn\s+([A-Za-z_]\w*)\b")
        # Collect function spans per file and flatten them into a global list.
        # NOTE: The previous implementation appended the restored function to the end of the file,
        # which duplicated the definition and made `cargo check` fail for every function (=> 0%).
        file_spans: Dict[Path, List[Tuple[str, int, int, int]]] = {}  # file -> [(fn_name, item_start, body_start, body_end)]
        functions: List[Tuple[Path, int, str]] = []  # (file, span_index_in_file, fn_name)
        for p, txt in originals.items():
            spans: List[Tuple[str, int, int, int]] = []
            seen = set()
            for m in fn_pat.finditer(txt):
                fn_name = m.group(1)
                span = find_fn_item_span(txt, m.start(), fn_name)
                if span is None:
                    continue
                item_start, body_start, body_end = span
                key = (item_start, body_start, body_end, fn_name)
                if key in seen:
                    continue
                seen.add(key)
                spans.append((fn_name, item_start, body_start, body_end))
            spans.sort(key=lambda x: (x[1], x[2], x[3], x[0]))
            file_spans[p] = spans
            for idx, (fn_name, _item_start, _body_start, _body_end) in enumerate(spans):
                functions.append((p, idx, fn_name))

        functions.sort(key=lambda x: (x[0].name, x[2], x[1]))
        res["total_functions"] = len(functions)
        if not functions:
            res["error"] = "No functions found"
            return res

        def stub_file_content(
            content: str,
            spans: List[Tuple[str, int, int, int]],
            skip_span_index: Optional[int] = None,
        ) -> str:
            # Replace each function body `{ ... }` with a small stub body.
            # We apply replacements in descending order so indices from the original content stay valid.
            out = content
            repls: List[Tuple[int, int, str]] = []
            for i, (fn_name, _item_start, body_start, body_end) in enumerate(spans):
                if skip_span_index is not None and i == skip_span_index:
                    continue
                repls.append((body_start, body_end, fn_name))
            for body_start, body_end, fn_name in sorted(repls, key=lambda t: t[0], reverse=True):
                out = out[:body_start] + f'{{ unimplemented!("{fn_name}") }}' + out[body_end:]
            return out

        # Stub all files once (baseline skeleton).
        stubbed_baseline: Dict[Path, str] = {}
        for p, txt in originals.items():
            spans = file_spans.get(p) or []
            stubbed = stub_file_content(txt, spans, skip_span_index=None) if spans else txt
            stubbed_baseline[p] = stubbed
            p.write_text(stubbed, encoding="utf-8")

        # Pre-check skeleton.
        with tempfile.TemporaryDirectory(prefix="c2rust_ohos_inc_target_") as tgt:
            env = {**os.environ, "CARGO_TARGET_DIR": tgt, "RUSTC_BOOTSTRAP": "1", "RUSTFLAGS": "-Awarnings", "RUST_BACKTRACE": "0"}
            rc, _, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp, env=env, timeout=timeout)
            if rc != 0:
                res["error"] = "Stubbed skeleton does not compile"
                res["skeleton_error"] = err[-8000:]
                return res

        ok = 0
        for file_path, span_idx, fn_name in functions:
            # Restore this function *in place* by generating a per-file variant where only this function keeps its
            # original body, and all other functions in the crate remain stubbed.
            spans = file_spans.get(file_path) or []
            file_path.write_text(
                stub_file_content(originals[file_path], spans, skip_span_index=span_idx),
                encoding="utf-8",
            )
            with tempfile.TemporaryDirectory(prefix="c2rust_ohos_inc_target_") as tgt:
                env = {**os.environ, "CARGO_TARGET_DIR": tgt, "RUSTC_BOOTSTRAP": "1", "RUSTFLAGS": "-Awarnings", "RUST_BACKTRACE": "0"}
                rc, _, _ = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp, env=env, timeout=timeout)
                if rc == 0:
                    ok += 1
            # Revert file back to the fully-stubbed baseline.
            file_path.write_text(stubbed_baseline[file_path], encoding="utf-8")

        res["compiled_functions"] = ok
        res["compile_rate"] = ok / res["total_functions"] if res["total_functions"] else 0.0
        return res


def main() -> int:
    parser = argparse.ArgumentParser(description="Analyze C2Rust baseline on OHOS(5) projects.")
    parser.add_argument("--base-dir", type=Path, default=DEFAULT_BASE_DIR)
    parser.add_argument("--output", "-o", type=Path, default=None)
    parser.add_argument("--run-clippy", action="store_true")
    parser.add_argument("--analyze-unsafe", action="store_true")
    parser.add_argument("--verify-incremental", action="store_true")
    parser.add_argument("--run-ohos-tests", action="store_true")
    parser.add_argument("--ohos-test-timeout", type=int, default=600)
    parser.add_argument("--huawei-projects-tsv", type=Path, default=DEFAULT_HUAWEI_PROJECTS_TSV)
    parser.add_argument("--ohos-root", type=Path, default=DEFAULT_OHOS_ROOT)
    parser.add_argument("--all", action="store_true")
    args = parser.parse_args()

    run_clippy = args.run_clippy or args.all
    analyze_unsafe = args.analyze_unsafe or args.all
    verify_incremental = args.verify_incremental or args.all
    run_ohos_tests = args.run_ohos_tests or args.all

    base_dir = args.base_dir
    if not base_dir.is_dir():
        print(f"Error: base-dir not found: {base_dir}", file=sys.stderr)
        return 2

    out_path = args.output or (base_dir / "compilation_analysis_ohos5.json")

    huawei_map = load_huawei_projects_map(args.huawei_projects_tsv) if run_ohos_tests else {}
    ohos_root = args.ohos_root

    projects: Dict[str, Any] = {}

    total_projects = 0
    cargo_ok = 0
    clippy_warn = rustc_warn = warn_total = clippy_err = 0
    code_lines = unsafe_lines = unsafe_items = 0
    inc_total = inc_ok = 0
    ohos_attempted = ohos_compiled = 0
    ohos_tests_total = ohos_tests_passed = ohos_tests_failed = 0

    for name in DISPLAY_PROJECT_ORDER:
        total_projects += 1
        crate_dir = base_dir / name
        pr: Dict[str, Any] = {"project_name": name, "crate_dir": str(crate_dir)}
        if not (crate_dir / "Cargo.toml").is_file():
            pr["error"] = "Cargo.toml not found"
            projects[name] = pr
            continue

        ck = run_cargo_check(crate_dir)
        pr["cargo_check"] = ck
        pr["cargo_check_passed"] = bool(ck.get("passed"))
        if ck.get("passed"):
            cargo_ok += 1

        if run_clippy:
            cr = run_cargo_clippy(crate_dir)
            pr["clippy_results"] = cr
            if cr.get("executed"):
                clippy_warn += int(cr.get("warning_count", 0) or 0)
                rustc_warn += int(cr.get("rustc_warning_count", 0) or 0)
                warn_total += int(cr.get("warning_count_total", 0) or 0)
                clippy_err += int(cr.get("error_count", 0) or 0)

        if analyze_unsafe:
            rs_files = [p for p in crate_dir.rglob("*.rs") if "target" not in p.parts]
            ua = analyze_unsafe_code(rs_files)
            pr["unsafe_analysis"] = ua
            code_lines += int(ua.get("code_lines", 0) or 0)
            unsafe_lines += int(ua.get("unsafe_total_lines", 0) or 0)
            unsafe_items += int(ua.get("unsafe_keyword_occurrences", 0) or 0)

        if verify_incremental:
            inc = verify_incremental_compilation(crate_dir)
            pr["incremental_compilation"] = inc
            if not inc.get("error"):
                inc_total += int(inc.get("total_functions", 0) or 0)
                inc_ok += int(inc.get("compiled_functions", 0) or 0)

        if run_ohos_tests:
            utr: Dict[str, Any] = {"executed": False, "compiled": False, "tests_passed": 0, "tests_failed": 0, "total_tests": 0, "pass_rate": 0.0, "error": None, "staticlib_build": {}, "gtest": {}}
            src_proj = huawei_map.get(name)
            if not src_proj:
                utr["error"] = f"source project not found in mapping: {name}"
            else:
                sb = build_staticlib_from_crate(crate_dir, timeout=args.ohos_test_timeout)
                utr["staticlib_build"] = sb
                if not sb.get("ok") or not sb.get("staticlib"):
                    utr["error"] = sb.get("error") or "staticlib build failed"
                else:
                    gt = run_ohos_unit_tests(
                        project_name=name,
                        source_project_dir=src_proj,
                        translated_staticlib=Path(sb["staticlib"]),
                        ohos_root=ohos_root,
                        timeout=args.ohos_test_timeout,
                    )
                    utr["gtest"] = gt
                    utr["executed"] = bool(gt.get("executed"))
                    utr["compiled"] = bool(gt.get("compiled"))
                    utr["tests_passed"] = int(gt.get("tests_passed", 0) or 0)
                    utr["tests_failed"] = int(gt.get("tests_failed", 0) or 0)
                    utr["total_tests"] = int(gt.get("total_tests", 0) or 0)
                    utr["pass_rate"] = float(gt.get("pass_rate", 0.0) or 0.0)
                    utr["error"] = gt.get("error")

            attempted = bool(utr.get("executed")) or bool(utr.get("staticlib_build", {}).get("executed")) or bool(utr.get("error"))
            if attempted:
                ohos_attempted += 1
                if utr.get("compiled"):
                    ohos_compiled += 1
                if utr.get("executed"):
                    ohos_tests_passed += int(utr.get("tests_passed", 0) or 0)
                    ohos_tests_failed += int(utr.get("tests_failed", 0) or 0)
                    ohos_tests_total += int(utr.get("total_tests", 0) or 0)
            pr["ohos_unit_test_results"] = utr

        projects[name] = pr

    summary: Dict[str, Any] = {
        "total_projects": total_projects,
        "projects_compiled": cargo_ok,
        "project_compile_rate": (cargo_ok / total_projects) if total_projects else 0.0,
    }
    if run_clippy:
        summary["clippy_summary"] = {"total_clippy_warnings": clippy_warn, "total_rustc_warnings": rustc_warn, "total_warnings_including_rustc": warn_total, "total_errors": clippy_err}
    if analyze_unsafe:
        summary["unsafe_summary"] = {"total_code_lines": code_lines, "total_unsafe_total_lines": unsafe_lines, "unsafe_total_ratio": (unsafe_lines / code_lines) if code_lines else 0.0}
    if verify_incremental:
        summary["incremental_compilation_summary"] = {"total_functions": inc_total, "compiled_functions": inc_ok, "compile_rate": (inc_ok / inc_total) if inc_total else 0.0}
    if run_ohos_tests:
        summary["ohos_unit_test_summary"] = {
            "projects_executed": ohos_attempted,
            "projects_compiled": ohos_compiled,
            "compile_success_rate": (ohos_compiled / ohos_attempted) if ohos_attempted else 0.0,
            "total_tests": ohos_tests_total,
            "tests_passed": ohos_tests_passed,
            "tests_failed": ohos_tests_failed,
            "overall_test_pass_rate": (ohos_tests_passed / ohos_tests_total) if ohos_tests_total else 0.0,
        }

    result: Dict[str, Any] = {"base_dir": str(base_dir), "projects": projects, "summary": summary}
    out_path.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"Analysis saved to: {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
