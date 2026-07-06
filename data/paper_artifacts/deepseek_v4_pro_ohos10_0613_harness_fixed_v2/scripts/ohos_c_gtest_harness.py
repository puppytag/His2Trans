#!/usr/bin/env python3
"""论文实验：把 OHOS C 项目的现有 C/C++ 测试转发到翻译后的 Rust staticlib。"""

from __future__ import annotations

import json
import hashlib
import os
import re
import shutil
import subprocess
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Sequence, Tuple


REPO_ROOT = Path(__file__).resolve().parents[1]
OHOS_ROOT = REPO_ROOT / "SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony"
DEFAULT_RUN_DIR = "deepseek-v4-pro-ohos11-full-0605"

C_PROJECT_SOURCES: Dict[str, Path] = {
    "host__25c1898e1626": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/host__25c1898e1626",
    "appverify_lite__e5ebe91a98b9": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/appverify_lite__e5ebe91a98b9",
    "manager__c248934e0221": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/manager__c248934e0221",
    "shared__541f4e547bdb": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/shared__541f4e547bdb",
    "posix__1b7f59c68bbc": REPO_ROOT / "SelfContained/self_contained_modules_v2/src_test_no_include/others/with_test/posix__1b7f59c68bbc",
    "common__89d5ecaafdff": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/common__89d5ecaafdff",
    "core__ef5242b7ab08": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/core__ef5242b7ab08",
    "shared__12e38ea922f7": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/shared__12e38ea922f7",
    "osal__0bc4f21396ad": REPO_ROOT / "SelfContained/self_contained_modules_v2/src_test_no_include/others/with_test/osal__0bc4f21396ad",
    "sapm__193cdeb43a97": REPO_ROOT / "SelfContained/self_contained_modules_v2/with_third_party/others/with_test/sapm__193cdeb43a97",
}


@dataclass(frozen=True)
class RustFunction:
    """Rust extern C 函数的最小签名信息。"""

    name: str
    module: str
    params: str
    return_type: Optional[str]
    arg_names: Tuple[str, ...]
    is_unsafe: bool


def run_command(cmd: Sequence[str], cwd: Path, timeout: int, env: Optional[Dict[str, str]] = None) -> Dict[str, Any]:
    """运行外部命令并保留结构化输出。"""
    started = time.time()
    try:
        proc = subprocess.run(
            list(cmd),
            cwd=str(cwd),
            env=env,
            text=True,
            capture_output=True,
            timeout=timeout,
        )
        return {
            "cmd": list(cmd),
            "cwd": str(cwd),
            "returncode": proc.returncode,
            "ok": proc.returncode == 0,
            "stdout": (proc.stdout or "")[-200000:],
            "stderr": (proc.stderr or "")[-200000:],
            "elapsed_sec": round(time.time() - started, 3),
        }
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout if isinstance(exc.stdout, str) else ""
        stderr = exc.stderr if isinstance(exc.stderr, str) else ""
        return {
            "cmd": list(cmd),
            "cwd": str(cwd),
            "returncode": 124,
            "ok": False,
            "stdout": stdout[-200000:],
            "stderr": (stderr + f"\nTimeout after {timeout}s")[-200000:],
            "elapsed_sec": round(time.time() - started, 3),
        }


def crate_dir_for(run_dir: str, project: str) -> Path:
    """返回指定 run 中某个项目的最终 Rust crate 路径。"""
    return (
        REPO_ROOT
        / "experiment_runs"
        / run_dir
        / "raw/framework_output/intermediate"
        / project
        / "workspace/final_projects"
        / project
        / "translate_by_qwen3_coder"
    )


def _copy_crate(src: Path, dst: Path) -> None:
    """复制 Rust crate，跳过 target 目录。"""
    def ignore(_dir: str, names: List[str]) -> List[str]:
        return [name for name in names if name == "target"]

    shutil.copytree(src, dst, ignore=ignore)


def split_top_level_commas(text: str) -> List[str]:
    """按顶层逗号切分 Rust 参数列表。"""
    parts: List[str] = []
    start = 0
    angle = 0
    paren = 0
    bracket = 0
    for i, ch in enumerate(text):
        if ch == "<":
            angle += 1
        elif ch == ">" and angle:
            angle -= 1
        elif ch == "(":
            paren += 1
        elif ch == ")" and paren:
            paren -= 1
        elif ch == "[":
            bracket += 1
        elif ch == "]" and bracket:
            bracket -= 1
        elif ch == "," and angle == 0 and paren == 0 and bracket == 0:
            part = text[start:i].strip()
            if part:
                parts.append(part)
            start = i + 1
    tail = text[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def _param_name(param: str) -> Optional[str]:
    """提取 Rust 参数名，用于 wrapper 调用。"""
    if ":" not in param:
        return None
    name = param.split(":", 1)[0].strip()
    name = name.removeprefix("mut ").strip()
    if not name or name == "_":
        return None
    return name


def _strip_rust_comments_for_signature_scan(text: str) -> str:
    """移除 Rust 注释，避免失败译文注释污染 extern 函数扫描。"""
    out: List[str] = []
    i = 0
    n = len(text)
    in_string = False
    in_char = False
    block_depth = 0
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if block_depth:
            if ch == "/" and nxt == "*":
                block_depth += 1
                out.extend("  ")
                i += 2
                continue
            if ch == "*" and nxt == "/":
                block_depth -= 1
                out.extend("  ")
                i += 2
                continue
            out.append("\n" if ch == "\n" else " ")
            i += 1
            continue
        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue
        if in_char:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if ch == "'":
                in_char = False
            i += 1
            continue
        if ch == "/" and nxt == "/":
            out.extend("  ")
            i += 2
            while i < n and text[i] != "\n":
                out.append(" ")
                i += 1
            continue
        if ch == "/" and nxt == "*":
            block_depth = 1
            out.extend("  ")
            i += 2
            continue
        if ch == '"':
            in_string = True
        elif ch == "'":
            in_char = True
        out.append(ch)
        i += 1
    return "".join(out)


def parse_rust_functions(crate_dir: Path) -> Tuple[List[RustFunction], List[str]]:
    """从最终 Rust 源中解析可导出的 extern C 函数。"""
    src_dir = crate_dir / "src"
    functions: List[RustFunction] = []
    diagnostics: List[str] = []
    sig_re = re.compile(
        r"pub\s+(unsafe\s+)?extern\s+\"C\"\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*"
        r"\((?P<params>.*?)\)\s*(?:->\s*(?P<ret>[^\{]+?))?\s*\{",
        re.DOTALL,
    )
    for path in sorted(src_dir.glob("*.rs")):
        if path.name in {"main.rs", "types.rs", "compat.rs", "compatibility.rs", "globals.rs"}:
            continue
        text = path.read_text(encoding="utf-8", errors="ignore")
        text = _strip_rust_comments_for_signature_scan(text)
        module = path.stem
        for match in sig_re.finditer(text):
            params = " ".join(match.group("params").split())
            ret = match.group("ret")
            ret = " ".join(ret.split()) if ret else None
            args: List[str] = []
            ok = True
            for param in split_top_level_commas(params):
                name = _param_name(param)
                if name is None:
                    ok = False
                    diagnostics.append(f"skip {module}::{match.group(2)}: cannot parse param `{param}`")
                    break
                args.append(name)
            if not ok:
                continue
            functions.append(
                RustFunction(
                    name=match.group(2),
                    module=module,
                    params=params,
                    return_type=ret,
                    arg_names=tuple(args),
                    is_unsafe=bool(match.group(1)),
                )
            )
    return functions, diagnostics


def _collect_existing_no_mangle_names(crate_dir: Path) -> List[str]:
    """收集 crate 已经直接导出的 no_mangle C ABI 函数名。"""
    names = set()
    attr_re = re.compile(
        r"#\s*\[\s*(?:unsafe\s*\(\s*)?no_mangle\s*(?:\)\s*)?\]\s*"
        r"(?:#\s*\[[^\]]+\]\s*)*"
        r"pub\s+(?:unsafe\s+)?extern\s+\"C\"\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)",
        re.DOTALL,
    )
    for path in sorted((crate_dir / "src").glob("*.rs")):
        text = path.read_text(encoding="utf-8", errors="ignore")
        text = _strip_rust_comments_for_signature_scan(text)
        names.update(attr_re.findall(text))
    return sorted(names)


def _generate_exports_source(
    functions: List[RustFunction],
    export_names: Optional[Iterable[str]] = None,
    already_exported_names: Optional[Iterable[str]] = None,
) -> Tuple[str, Dict[str, Any]]:
    """生成 crate root 的 no_mangle C ABI wrapper。"""
    by_name: Dict[str, List[RustFunction]] = {}
    for fn in functions:
        by_name.setdefault(fn.name, []).append(fn)

    requested = set(export_names) if export_names is not None else None
    already_exported = set(already_exported_names or [])
    exported = [
        items[0]
        for name, items in sorted(by_name.items())
        if len(items) == 1 and name not in already_exported and (requested is None or name in requested)
    ]
    duplicates = {
        name: [f"{fn.module}::{fn.name}" for fn in items]
        for name, items in sorted(by_name.items())
        if len(items) > 1 and (requested is None or name in requested)
    }
    matched_requested = sorted(name for name in (requested or set()) if name in by_name)
    unmatched_requested = sorted(name for name in (requested or set()) if name not in by_name)
    already_exported_requested = sorted(name for name in (requested or already_exported) if name in already_exported)
    lines = [
        "//! 临时 C ABI 导出，仅用于论文实验测试 harness。",
        "",
        "#![allow(non_snake_case)]",
        "#![allow(unused_unsafe)]",
        "#![allow(unused_imports)]",
        "",
        "use crate::types::*;",
        "use crate::*;",
        "use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_ushort, c_void};",
        "use std::os::raw::{c_schar, c_uchar};",
        "",
    ]
    for fn in exported:
        ret = f" -> {fn.return_type}" if fn.return_type else ""
        call = f"crate::{fn.module}::{fn.name}({', '.join(fn.arg_names)})"
        lines.append("#[no_mangle]")
        lines.append(f"pub unsafe extern \"C\" fn {fn.name}({fn.params}){ret} {{")
        if fn.return_type:
            lines.append(f"    unsafe {{ {call} }}")
        else:
            lines.append(f"    unsafe {{ {call}; }}")
        lines.append("}")
        lines.append("")
    meta = {
        "exported_count": len(exported),
        "requested_exports": matched_requested if requested is not None else None,
        "unmatched_requested_export_count": len(unmatched_requested) if requested is not None else 0,
        "unmatched_requested_export_samples": unmatched_requested[:20],
        "already_exported_requested": already_exported_requested if requested is not None else None,
        "duplicate_names": duplicates,
        "skipped_duplicate_count": sum(len(v) for v in duplicates.values()),
    }
    return "\n".join(lines), meta


def patch_crate_for_staticlib(crate_dir: Path, export_names: Optional[Iterable[str]] = None) -> Dict[str, Any]:
    """在临时 crate 中追加 staticlib 和 C ABI wrapper。"""
    cargo_toml = crate_dir / "Cargo.toml"
    cargo_text = cargo_toml.read_text(encoding="utf-8")
    if "[lib]" not in cargo_text:
        cargo_text += '\n[lib]\npath = "src/main.rs"\ncrate-type = ["staticlib"]\n'
        cargo_toml.write_text(cargo_text, encoding="utf-8")

    functions, diagnostics = parse_rust_functions(crate_dir)
    already_exported = _collect_existing_no_mangle_names(crate_dir)
    exports_text, meta = _generate_exports_source(functions, export_names, already_exported)
    (crate_dir / "src/c2r_exports.rs").write_text(exports_text, encoding="utf-8")

    main_rs = crate_dir / "src/main.rs"
    main_text = main_rs.read_text(encoding="utf-8")
    if "pub mod c2r_exports;" not in main_text:
        anchor = "pub mod types;\n"
        if anchor in main_text:
            main_text = main_text.replace(anchor, anchor + "pub mod c2r_exports;\n", 1)
        else:
            main_text += "\npub mod c2r_exports;\n"
        main_rs.write_text(main_text, encoding="utf-8")
    meta["parse_diagnostics"] = diagnostics
    meta["rust_extern_count"] = len(functions)
    meta["already_no_mangle_export_count"] = len(already_exported)
    return meta


def build_staticlib(crate_dir: Path, timeout: int) -> Tuple[Dict[str, Any], Optional[Path]]:
    """构建临时 Rust staticlib。"""
    env = dict(os.environ)
    env["RUSTFLAGS"] = f"{env.get('RUSTFLAGS', '')} -Awarnings".strip()
    env["RUST_BACKTRACE"] = "0"
    env["CARGO_TARGET_DIR"] = str(crate_dir / "target")
    result = run_command(["cargo", "build", "--release", "--offline", "--lib"], crate_dir, timeout, env)
    if not result["ok"]:
        return result, None
    libs = sorted((crate_dir / "target/release").glob("lib*.a"))
    return result, libs[0] if libs else None


def nm_defined_symbols(staticlib: Path) -> List[str]:
    """读取 staticlib 中已定义的全局符号。"""
    try:
        proc = subprocess.run(
            ["nm", "-g", "--defined-only", str(staticlib)],
            text=True,
            capture_output=True,
            timeout=120,
        )
    except Exception:
        return []
    if proc.returncode != 0:
        return []
    symbols: List[str] = []
    for raw in proc.stdout.splitlines():
        parts = raw.split()
        if parts:
            symbols.append(parts[-1])
    return sorted(set(symbols))


def nm_undefined_symbols(objects: Iterable[Path]) -> List[str]:
    """读取对象文件中仍未解析的符号。"""
    symbols: List[str] = []
    for obj in objects:
        try:
            proc = subprocess.run(
                ["nm", "-u", str(obj)],
                text=True,
                capture_output=True,
                timeout=60,
            )
        except Exception:
            continue
        if proc.returncode != 0:
            continue
        for raw in proc.stdout.splitlines():
            line = raw.strip()
            if not line:
                continue
            parts = line.split()
            symbols.append(parts[-1])
    return sorted(set(symbols))


def collect_test_sources(source_project_dir: Path) -> Tuple[List[Path], List[Path], Optional[str]]:
    """收集项目现有测试源和 sample fixture 源。"""
    roots = [source_project_dir / rel for rel in ("test", "tests", "unittest", "unittests") if (source_project_dir / rel).is_dir()]
    if not roots:
        return [], [], "no test directory"
    tests: List[Path] = []
    samples: List[Path] = []
    for root in roots:
        for path in root.rglob("*"):
            if not path.is_file() or path.suffix.lower() not in {".c", ".cc", ".cpp", ".cxx"}:
                continue
            parts = {part.lower() for part in path.relative_to(source_project_dir).parts}
            if "sample" in parts:
                samples.append(path)
            else:
                tests.append(path)
    tests.sort()
    samples.sort()
    if not tests:
        return [], samples, "no C/C++ test source"
    return tests, samples, None


def _filter_project_test_sources(project: str, test_srcs: List[Path]) -> Tuple[List[Path], List[Path]]:
    """按项目保留能覆盖翻译产物语义的原始测试源。"""
    if not project.startswith("manager__"):
        return test_srcs, []

    selected_names = {"hdf_lite_manager_test.cpp"}
    selected = [path for path in test_srcs if path.name in selected_names]
    excluded = [path for path in test_srcs if path.name not in selected_names]
    return selected, excluded


def include_dirs_for(source_project_dir: Path, ohos_root: Path, build_dir: Path) -> List[Path]:
    """返回 host 侧编译测试所需 include 目录。"""
    dirs: List[Path] = [build_dir, source_project_dir]
    for rel in (
        "include",
        "src",
        "test",
        "tests",
        "test/unittest",
        "test/unittest/common",
        "test/sample",
        "unittest",
        "unittest/src",
        "unittest/packets",
    ):
        d = source_project_dir / rel
        if d.is_dir():
            dirs.append(d)

    dirs.extend(
        [
            ohos_root / "drivers/hdf_core/interfaces/inner_api/utils",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/core",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/uhdf",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/host/uhdf",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/host",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/ipc",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/hdi",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/host/shared",
            ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/shared",
            ohos_root / "drivers/hdf_core/framework/core/shared/include",
            ohos_root / "drivers/hdf_core/framework/core/manager/include",
            ohos_root / "drivers/hdf_core/framework/core/host/include",
            ohos_root / "drivers/hdf_core/framework/utils/include",
            ohos_root / "drivers/hdf_core/framework/include/audio",
            ohos_root / "drivers/hdf_core/framework/include/osal",
            ohos_root / "drivers/hdf_core/framework/include/utils",
            ohos_root / "drivers/hdf_core/adapter/khdf/linux/osal/include",
            ohos_root / "drivers/hdf_core/adapter/uhdf2/ipc/include",
            ohos_root / "drivers/hdf_core/framework/test/unittest/common",
            ohos_root / "drivers/hdf_core/framework/test/unittest/include",
            ohos_root / "drivers/hdf_core/framework/test/unittest/pm",
            ohos_root / "drivers/hdf_core/framework/test/unittest/manager",
            ohos_root / "drivers/hdf_core/framework/test/unittest/osal",
            ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/include",
            ohos_root / "drivers/hdf_core/framework/model/audio/sapm/test/unittest/common",
            ohos_root / "drivers/hdf_core/framework/model/audio/core/test/unittest/common",
            ohos_root / "drivers/hdf_core/framework/model/audio/common/test/unittest/common",
            ohos_root / "drivers/hdf_core/framework/model/audio/include",
            ohos_root / "drivers/hdf_core/framework/model/audio/dispatch/include",
            ohos_root / "drivers/hdf_core/framework/model/audio/sapm/include",
            ohos_root / "drivers/hdf_core/framework/model/audio/common/include",
            ohos_root / "drivers/hdf_core/framework/model/audio/core/include",
            ohos_root / "foundation/communication/ipc/interfaces/innerkits/ipc_core/include",
            ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/samgr_proxy/include",
            ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/dynamic_cache/include",
            ohos_root / "base/notification/eventhandler/interfaces/inner_api",
            ohos_root / "base/notification/eventhandler/frameworks/eventhandler/include",
            ohos_root / "base/hiviewdfx/hisysevent/interfaces/native/innerkits/hisysevent/include",
            ohos_root / "base/hiviewdfx/hilog/interfaces/native/innerkits/include",
            ohos_root / "commonlibrary/c_utils/base/include",
            ohos_root / "third_party/bounds_checking_function/include",
            ohos_root / "third_party/mbedtls/include",
            ohos_root / "third_party/mbedtls/port/config/compat_posix",
            ohos_root / "third_party/cJSON",
            ohos_root / "third_party/googletest/googletest/include",
            ohos_root / "third_party/googletest/googlemock/include",
            ohos_root / "third_party/googletest/googletest",
        ]
    )
    unique: List[Path] = []
    seen = set()
    for d in dirs:
        if d.is_dir() and d not in seen:
            seen.add(d)
            unique.append(d)
    return unique


def extract_unresolved_symbols(stderr: str) -> List[str]:
    """从链接错误中提取 undefined reference 符号。"""
    symbols: List[str] = []
    for raw in stderr.splitlines():
        if "undefined reference to" not in raw:
            continue
        match = re.search(r"undefined reference to [`']([^`']+)[`']", raw)
        if match:
            symbols.append(match.group(1))
    return sorted(set(symbols))


def parse_gtest_list(output: str) -> int:
    """解析 --gtest_list_tests 输出的用例数。"""
    count = 0
    in_suite = False
    for raw in output.splitlines():
        line = raw.rstrip()
        if not line:
            continue
        if not line.startswith(" ") and line.endswith("."):
            in_suite = True
            continue
        if in_suite and line.startswith("  ") and not line.strip().startswith("#"):
            count += 1
    return count


def parse_gtest_cases(output: str) -> List[str]:
    """解析 --gtest_list_tests 输出的完整用例名。"""
    cases: List[str] = []
    suite = ""
    for raw in output.splitlines():
        line = raw.rstrip()
        if not line:
            continue
        if not line.startswith(" ") and line.endswith("."):
            suite = line.strip()
            continue
        if suite and line.startswith("  ") and not line.strip().startswith("#"):
            cases.append(f"{suite}{line.strip()}")
    return cases


def parse_gtest_counts(output: str, total_hint: Optional[int] = None) -> Dict[str, Any]:
    """解析 gtest 运行输出。"""
    total = total_hint
    passed = None
    failed = None
    for line in output.splitlines():
        run_match = re.search(r"\[\s*==========\s*\]\s+(\d+) tests? from", line)
        if run_match:
            total = int(run_match.group(1))
        pass_match = re.search(r"\[\s*PASSED\s*\]\s+(\d+) tests?\.", line)
        if pass_match:
            passed = int(pass_match.group(1))
        fail_match = re.search(r"\[\s*FAILED\s*\]\s+(\d+) tests?(?:, listed below:|\.)", line)
        if fail_match:
            failed = int(fail_match.group(1))
    if total is not None and passed is not None and failed is None:
        failed = max(total - passed, 0)
    if total is not None and passed is None and failed is not None:
        passed = max(total - failed, 0)
    return {
        "tests_total": total,
        "tests_passed": passed,
        "tests_failed": failed,
        "pass_rate": (passed / total) if total and passed is not None else None,
    }


def parse_failed_tests(output: str) -> List[str]:
    """解析失败用例名称。"""
    failed: List[str] = []
    seen = set()
    for raw in output.splitlines():
        match = re.match(r"^\[\s*FAILED\s*\]\s+([A-Za-z0-9_:.]+)(?:\s+\(\d+\s+ms\))?$", raw)
        if not match:
            continue
        name = match.group(1)
        if name in seen:
            continue
        seen.add(name)
        failed.append(name)
    return failed


def summarize_isolated_gtest_runs(case_results: List[Dict[str, Any]]) -> Dict[str, Any]:
    """汇总单用例隔离重跑结果，用于 abort 场景保留可统计分母。"""
    failed = [item["case"] for item in case_results if not item["result"].get("ok")]
    total = len(case_results)
    passed = total - len(failed)
    return {
        "tests_total": total,
        "tests_passed": passed,
        "tests_failed": len(failed),
        "pass_rate": (passed / total) if total else None,
        "failed_tests": failed,
    }


def _needs_isolated_gtest_rerun(run_res: Dict[str, Any], counts: Dict[str, Any], cases: List[str]) -> bool:
    """判断是否需要单用例隔离重跑。"""
    if not cases:
        return False
    if counts.get("tests_passed") is None or counts.get("tests_failed") is None:
        return True
    return bool(not run_res.get("ok") and not parse_failed_tests((run_res.get("stdout") or "") + "\n" + (run_res.get("stderr") or "")))


def _run_isolated_gtest_cases(binary: Path, build_dir: Path, cases: List[str], timeout: int, env: Dict[str, str]) -> List[Dict[str, Any]]:
    """逐个运行 gtest 用例，避免单个 abort 吞掉整组统计。"""
    results: List[Dict[str, Any]] = []
    for case in cases:
        result = run_command(
            [str(binary), "--gtest_color=no", f"--gtest_filter={case}"],
            build_dir,
            timeout,
            env,
        )
        results.append({"case": case, "result": result})
    return results


def _host_infeasible_gtest_exclusions(project: str) -> List[Dict[str, str]]:
    """返回普通 host 环境无法真实提供目标系统语义的用例。"""
    if not project.startswith("posix__"):
        return []
    reason = (
        "requires target/user-mode realtime pthread scheduler semantics; "
        "ordinary Linux host cannot reliably create SCHED_FIFO/SCHED_RR threads without elevated capabilities"
    )
    cases = [
        "OsalTest.OsalGetThread001",
        "OsalTest.OsalGetThread003",
        "OsalTest.OsalGetAll001",
        "OsalTestPosix.OsalGetThread001",
        "OsalTestPosix.OsalGetThread003",
        "OsalTestPosix.OsalGetAll001",
    ]
    return [{"case": case, "reason": reason} for case in cases]


def _gtest_filter_for_exclusions(exclusions: List[Dict[str, str]]) -> List[str]:
    """把 host 不可满足用例转换为 gtest negative filter 参数。"""
    cases = [item["case"] for item in exclusions if item.get("case")]
    if not cases:
        return []
    return [f"--gtest_filter=-{':'.join(cases)}"]


def _write_host_headers(build_dir: Path) -> None:
    """写入普通 Linux host 缺失的最小系统头。"""
    linux_dir = build_dir / "linux"
    linux_dir.mkdir(parents=True, exist_ok=True)
    (linux_dir / "ashmem.h").write_text(
        "\n".join(
            [
                "#ifndef _LINUX_ASHMEM_H",
                "#define _LINUX_ASHMEM_H",
                "#define ASHMEM_NAME_LEN 256",
                "#define ASHMEM_SET_NAME 0x41007701",
                "#define ASHMEM_GET_NAME 0x41007702",
                "#define ASHMEM_SET_SIZE 0x40087703",
                "#define ASHMEM_GET_SIZE 0x40087704",
                "#define ASHMEM_SET_PROT_MASK 0x40087705",
                "#define ASHMEM_GET_PROT_MASK 0x40087706",
                "#endif",
                "",
            ]
        ),
        encoding="utf-8",
    )
    (linux_dir / "io.h").write_text(
        "\n".join(
            [
                "#ifndef C2R_LINUX_IO_H",
                "#define C2R_LINUX_IO_H",
                "#include <stdint.h>",
                "static inline void *ioremap(uintptr_t addr, unsigned long size) { (void)size; return (void *)addr; }",
                "static inline void iounmap(void *addr) { (void)addr; }",
                "static inline uint8_t readb(const volatile void *addr) { return *(const volatile uint8_t *)addr; }",
                "static inline uint16_t readw(const volatile void *addr) { return *(const volatile uint16_t *)addr; }",
                "static inline uint32_t readl(const volatile void *addr) { return *(const volatile uint32_t *)addr; }",
                "static inline void writeb(uint8_t v, volatile void *addr) { *(volatile uint8_t *)addr = v; }",
                "static inline void writew(uint16_t v, volatile void *addr) { *(volatile uint16_t *)addr = v; }",
                "static inline void writel(uint32_t v, volatile void *addr) { *(volatile uint32_t *)addr = v; }",
                "#endif",
                "",
            ]
        ),
        encoding="utf-8",
    )
    (linux_dir / "kernel.h").write_text(
        "\n".join(
            [
                "#ifndef C2R_LINUX_KERNEL_H",
                "#define C2R_LINUX_KERNEL_H",
                "#ifndef NULL",
                "#define NULL ((void *)0)",
                "#endif",
                "#endif",
                "",
            ]
        ),
        encoding="utf-8",
    )
    (build_dir / "c2r_gtest_compat.h").write_text(
        "\n".join(
            [
                "#ifndef C2R_GTEST_COMPAT_H",
                "#define C2R_GTEST_COMPAT_H",
                "#include <gtest/gtest.h>",
                "namespace testing { namespace ext {",
                "struct TestSize {",
                "    static constexpr int Level0 = 0;",
                "    static constexpr int Level1 = 1;",
                "    static constexpr int Level2 = 2;",
                "    static constexpr int Level3 = 3;",
                "    static constexpr int Level4 = 4;",
                "};",
                "} }",
                "#ifndef HWTEST",
                "#define HWTEST(test_suite_name, test_name, test_size) TEST(test_suite_name, test_name)",
                "#endif",
                "#ifndef HWTEST_F",
                "#define HWTEST_F(test_fixture, test_name, test_size) TEST_F(test_fixture, test_name)",
                "#endif",
                "#ifndef HWTEST_P",
                "#define HWTEST_P(test_fixture, test_name, test_size) TEST_P(test_fixture, test_name)",
                "#endif",
                "#endif",
                "",
            ]
        ),
        encoding="utf-8",
    )


def _compile_c_source(src: Path, obj: Path, includes: List[Path], cwd: Path, timeout: int, extra: Optional[List[str]] = None) -> Dict[str, Any]:
    """按源文件后缀编译单个 C/C++ 源为对象文件。"""
    ext = src.suffix.lower()
    is_c = ext == ".c"
    cc = "gcc" if is_c else "g++"
    cmd = [cc, "-O2", "-g", "-c", "-D__linux__"]
    if not is_c:
        cmd.append("-std=c++17")
    for inc in includes:
        cmd.extend(["-I", str(inc)])
    if extra:
        cmd.extend(extra)
    cmd.extend([str(src), "-o", str(obj)])
    return run_command(cmd, cwd, timeout)


def _weak_pragmas_for_symbols(symbols: Iterable[str]) -> str:
    """生成 C helper 重叠符号的弱定义声明。"""
    valid = [sym for sym in sorted(set(symbols)) if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", sym)]
    return "".join(f"#pragma weak {sym}\n" for sym in valid)


def _weak_patched_c_source_path(src: Path, build_dir: Path) -> Path:
    """返回临时弱符号补丁源路径，避免修改官方 helper 源。"""
    digest = hashlib.sha256(str(src).encode("utf-8", errors="ignore")).hexdigest()[:12]
    suffix = src.suffix if src.suffix else ".c"
    patch_dir = build_dir / "weak_helper_sources"
    patch_dir.mkdir(parents=True, exist_ok=True)
    return patch_dir / f"{src.stem}_{digest}{suffix}"


def _compile_helper_c_source(
    src: Path,
    obj: Path,
    includes: List[Path],
    build_dir: Path,
    timeout: int,
    static_symbols: Iterable[str],
    extra: Optional[List[str]] = None,
) -> Dict[str, Any]:
    """编译官方 helper；若 helper 定义已由 Rust staticlib 提供，则改成弱定义。"""
    first = _compile_c_source(src, obj, includes, build_dir, timeout, extra)
    if not first.get("ok"):
        return first

    overlaps = sorted(set(nm_defined_symbols(obj)) & set(static_symbols))
    if not overlaps:
        return first

    pragmas = _weak_pragmas_for_symbols(overlaps)
    if not pragmas:
        return first

    patched_src = _weak_patched_c_source_path(src, build_dir)
    patched_src.write_text(pragmas + "\n" + src.read_text(encoding="utf-8", errors="ignore"), encoding="utf-8")
    patched_extra = [*(extra or []), "-I", str(src.parent)]
    second = _compile_c_source(patched_src, obj, includes, build_dir, timeout, patched_extra)
    second["original_source"] = str(src)
    second["source_patches"] = ["weak_overlapping_helper_symbols"]
    second["weakened_symbols"] = overlaps
    second["patched_source"] = str(patched_src)
    second["original_compile"] = first
    return second


def _compile_test_source(
    src: Path,
    obj: Path,
    includes: List[Path],
    build_dir: Path,
    timeout: int,
) -> Dict[str, Any]:
    """编译测试源，自动注入 OHOS gtest 兼容头。"""
    src_to_compile, patch_notes = _prepare_test_source_for_host(src, build_dir)
    extra: List[str] = []
    if src_to_compile.suffix.lower() != ".c":
        extra.extend(["-include", str(build_dir / "c2r_gtest_compat.h")])
    result = _compile_c_source(src_to_compile, obj, includes, build_dir, timeout, extra)
    if patch_notes:
        result["source_patches"] = patch_notes
        result["original_source"] = str(src)
    return result


def _prepare_test_source_for_host(src: Path, build_dir: Path) -> Tuple[Path, List[str]]:
    """按需复制并修正测试源里的 host ABI 问题，不修改原始测试文件。"""
    text = src.read_text(encoding="utf-8", errors="ignore")
    patched, notes = _patch_host_test_source_with_notes(text)
    if patched == text:
        return src, []

    patch_dir = build_dir / "patched_test_sources"
    patch_dir.mkdir(parents=True, exist_ok=True)
    patched_path = patch_dir / _object_name_for(src.parent, src).removesuffix(".o")
    patched_path.write_text(patched, encoding="utf-8")
    return patched_path, notes


def _patch_host_test_source(text: str) -> str:
    """修正 64 位 host 编译测试 helper 时暴露的确定性 ABI 不匹配。"""
    return _patch_host_test_source_with_notes(text)[0]


def _patch_host_test_source_with_notes(text: str) -> Tuple[str, List[str]]:
    """返回 host 测试源码补丁结果和补丁类型。"""
    notes: List[str] = []
    pattern = re.compile(
        r"int32_t\s+len\s*=\s*0;\s*"
        r"mbedtls_base64_decode\((?P<args>.*?)reinterpret_cast<size_t\s*\*>\(&len\)(?P<tail>.*?)\);\s*"
        r"int32_t\s+num\s*=\s*0;",
        re.DOTALL,
    )

    def repl(match: re.Match[str]) -> str:
        return (
            "int32_t len = 0;\n"
            "    size_t decodedLen = 0;\n"
            f"    mbedtls_base64_decode({match.group('args')}&decodedLen{match.group('tail')});\n"
            "    len = static_cast<int32_t>(decodedLen);\n"
            "    int32_t num = 0;"
        )

    patched = pattern.sub(repl, text)
    if patched != text:
        notes.append("host_size_t_decode_len")
    audio_structs = (
        "HdfDeviceObject",
        "CodecData",
        "DaiData",
        "DspData",
        "PlatformData",
        "AudioRegCfgData",
    )
    struct_pattern = re.compile(
        r"(?P<indent>^[ \t]*)struct\s+(?P<type>" + "|".join(audio_structs) + r")\s+"
        r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*;",
        re.MULTILINE,
    )
    zeroed = struct_pattern.sub(
        lambda match: f"{match.group('indent')}struct {match.group('type')} {match.group('name')} = {{0}};",
        patched,
    )
    if zeroed != patched:
        notes.append("host_zero_init_audio_structs")
    return zeroed, notes


def _object_name_for(source_dir: Path, src: Path) -> str:
    """按相对路径生成稳定对象文件名，避免同名测试文件冲突。"""
    try:
        rel = src.relative_to(source_dir)
    except ValueError:
        rel = Path(src.name)
    safe = re.sub(r"[^A-Za-z0-9_.-]+", "_", str(rel))
    return f"{safe}.o"


def _build_securec(
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    timeout: int,
    static_symbols: Iterable[str] = (),
) -> Tuple[List[Path], List[Dict[str, Any]]]:
    """编译 securec，解决 memcpy_s 等外部依赖。"""
    src_dir = ohos_root / "third_party/bounds_checking_function/src"
    if not src_dir.is_dir():
        return [], []
    objs: List[Path] = []
    reports: List[Dict[str, Any]] = []
    obj_dir = build_dir / "securec_objs"
    obj_dir.mkdir(parents=True, exist_ok=True)
    for src in sorted(src_dir.glob("*.c")):
        obj = obj_dir / f"{src.stem}.o"
        res = _compile_helper_c_source(src, obj, includes, build_dir, timeout, static_symbols)
        reports.append(res)
        if not res["ok"]:
            return objs, reports
        objs.append(obj)
    return objs, reports


def _build_optional_ohos_helpers(
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    static_symbols: Iterable[str],
    timeout: int,
) -> Tuple[List[Path], List[Dict[str, Any]]]:
    """按缺失符号编译 OHOS host helper 对象。"""
    symbols = set(static_symbols)
    candidates: List[Tuple[str, Path]] = []
    if "HdfSbufReadUint32" not in symbols:
        candidates.extend(
            [
                ("hdf_sbuf_impl_raw", ohos_root / "drivers/hdf_core/framework/utils/src/hdf_sbuf_impl_raw.c"),
                ("hdf_sbuf", ohos_root / "drivers/hdf_core/framework/utils/src/hdf_sbuf.c"),
            ]
        )
    if "HdfSListAdd" not in symbols:
        candidates.append(("hdf_slist", ohos_root / "drivers/hdf_core/framework/utils/src/hdf_slist.c"))
    if "HdfDeviceInfoNewInstance" not in symbols:
        candidates.append(("hdf_device_info", ohos_root / "drivers/hdf_core/framework/core/shared/src/hdf_device_info.c"))
    if "HdfStringCopy" not in symbols or "HdfStringMakeHashKey" not in symbols:
        candidates.append(("hdf_cstring", ohos_root / "drivers/hdf_core/framework/utils/src/hdf_cstring.c"))
    if "HdfSRefConstruct" not in symbols:
        candidates.append(("hdf_sref", ohos_root / "drivers/hdf_core/framework/utils/src/hdf_sref.c"))
    if "DevSvcRecordFreeInstance" not in symbols or "DevSvcRecordNewInstance" not in symbols:
        candidates.append(("hdf_service_record", ohos_root / "drivers/hdf_core/framework/core/shared/src/hdf_service_record.c"))
    if "OsalMutexInit" not in symbols:
        candidates.append(("osal_mutex", ohos_root / "drivers/hdf_core/framework/support/posix/src/osal_mutex.c"))
    if "OsalSleep" not in symbols:
        candidates.append(("osal_time", ohos_root / "drivers/hdf_core/framework/support/posix/src/osal_time.c"))
    if "OsalThreadCreate" not in symbols:
        candidates.append(("osal_thread", ohos_root / "drivers/hdf_core/framework/support/posix/src/osal_thread.c"))

    objs: List[Path] = []
    reports: List[Dict[str, Any]] = []
    for stem, src in candidates:
        if not src.is_file():
            continue
        obj = build_dir / f"{stem}.o"
        res = _compile_helper_c_source(src, obj, includes, build_dir, timeout, static_symbols)
        reports.append(res)
        if not res["ok"]:
            return objs, reports
        objs.append(obj)
    return objs, reports


def _supplemental_audio_product_shim_source(project: str) -> Optional[str]:
    """生成拆分 audio 模块测试所需的相邻产品弱符号。"""
    if project.startswith("sapm__"):
        return r'''
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "audio_core.h"

__attribute__((weak)) struct CodecDevice *AudioKcontrolGetCodec(const struct AudioKcontrol *kcontrol)
{
    if (kcontrol == NULL || kcontrol->pri == NULL) {
        return NULL;
    }
    struct AudioCard *audioCard = (struct AudioCard *)(uintptr_t)kcontrol->pri;
    if (audioCard->rtd == NULL) {
        return NULL;
    }
    return audioCard->rtd->codec;
}

__attribute__((weak)) int32_t AudioCodecReadReg(const struct CodecDevice *codec, uint32_t reg, uint32_t *val)
{
    if (codec == NULL || codec->devData == NULL || codec->devData->Read == NULL || val == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    return (codec->devData->Read(codec, reg, val) == HDF_SUCCESS) ? HDF_SUCCESS : HDF_FAILURE;
}

static int32_t C2rAudioCodecWriteReg(const struct CodecDevice *codec, uint32_t reg, uint32_t val)
{
    if (codec == NULL || codec->devData == NULL || codec->devData->Write == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    return (codec->devData->Write(codec, reg, val) == HDF_SUCCESS) ? HDF_SUCCESS : HDF_FAILURE;
}

static int32_t C2rAudioUpdateCodecRegBits(struct CodecDevice *codec, uint32_t reg,
    uint32_t mask, uint32_t shift, uint32_t value)
{
    uint32_t curValue = 0;
    uint32_t controlMask;
    if (codec == NULL || codec->devData == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    value = value << shift;
    controlMask = mask << shift;
    if (AudioCodecReadReg(codec, reg, &curValue) != HDF_SUCCESS) {
        return HDF_FAILURE;
    }
    if ((curValue & controlMask) == value) {
        return HDF_SUCCESS;
    }
    curValue = (curValue & ~controlMask) | (value & controlMask);
    return C2rAudioCodecWriteReg(codec, reg, curValue);
}

__attribute__((weak)) int32_t AudioUpdateCodecRegBits(struct CodecDevice *codec,
    uint32_t reg, uint32_t mask, uint32_t shift, uint32_t value)
{
    return C2rAudioUpdateCodecRegBits(codec, reg, mask, shift, value);
}

__attribute__((weak)) int32_t AudioCodecRegUpdate(struct CodecDevice *codec,
    struct AudioMixerControl *mixerCtrl)
{
    uint32_t mixerValue;
    if (codec == NULL || mixerCtrl == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    mixerValue = mixerCtrl->value;
    if (mixerValue < mixerCtrl->min || mixerValue > mixerCtrl->max) {
        return HDF_ERR_INVALID_OBJECT;
    }
    if (mixerCtrl->invert) {
        mixerValue = mixerCtrl->max - mixerCtrl->value;
    }
    if (C2rAudioUpdateCodecRegBits(codec, mixerCtrl->reg, mixerCtrl->mask,
        mixerCtrl->shift, mixerValue) != HDF_SUCCESS) {
        return HDF_FAILURE;
    }
    if (mixerCtrl->reg != mixerCtrl->rreg || mixerCtrl->shift != mixerCtrl->rshift) {
        if (C2rAudioUpdateCodecRegBits(codec, mixerCtrl->rreg, mixerCtrl->mask,
            mixerCtrl->rshift, mixerValue) != HDF_SUCCESS) {
            return HDF_FAILURE;
        }
    }
    return HDF_SUCCESS;
}

static int32_t C2rAudioGetCtrlOpsReg(struct AudioCtrlElemValue *elemValue,
    const struct AudioMixerControl *mixerCtrl, uint32_t curValue)
{
    if (elemValue == NULL || mixerCtrl == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    curValue = (curValue >> mixerCtrl->shift) & mixerCtrl->mask;
    if (curValue > mixerCtrl->max || curValue < mixerCtrl->min) {
        return HDF_FAILURE;
    }
    elemValue->value[0] = mixerCtrl->invert ? (mixerCtrl->max - curValue) : curValue;
    return HDF_SUCCESS;
}

static int32_t C2rAudioGetCtrlOpsRReg(struct AudioCtrlElemValue *elemValue,
    const struct AudioMixerControl *mixerCtrl, uint32_t rcurValue)
{
    if (elemValue == NULL || mixerCtrl == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    if (mixerCtrl->reg != mixerCtrl->rreg || mixerCtrl->shift != mixerCtrl->rshift) {
        if (mixerCtrl->reg == mixerCtrl->rreg) {
            rcurValue = (rcurValue >> mixerCtrl->rshift) & mixerCtrl->mask;
        } else {
            rcurValue = (rcurValue >> mixerCtrl->shift) & mixerCtrl->mask;
        }
        if (rcurValue > mixerCtrl->max || rcurValue < mixerCtrl->min) {
            return HDF_FAILURE;
        }
        elemValue->value[1] = mixerCtrl->invert ? (mixerCtrl->max - rcurValue) : rcurValue;
    }
    return HDF_SUCCESS;
}

__attribute__((weak)) int32_t AudioCodecGetCtrlOps(const struct AudioKcontrol *kcontrol,
    struct AudioCtrlElemValue *elemValue)
{
    uint32_t curValue = 0;
    uint32_t rcurValue = 0;
    if (kcontrol == NULL || kcontrol->privateValue == 0 || elemValue == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    struct AudioMixerControl *mixerCtrl = (struct AudioMixerControl *)(uintptr_t)kcontrol->privateValue;
    struct CodecDevice *codec = AudioKcontrolGetCodec(kcontrol);
    if (codec == NULL) {
        return HDF_FAILURE;
    }
    if (AudioCodecReadReg(codec, mixerCtrl->reg, &curValue) != HDF_SUCCESS ||
        AudioCodecReadReg(codec, mixerCtrl->rreg, &rcurValue) != HDF_SUCCESS) {
        return HDF_FAILURE;
    }
    if (C2rAudioGetCtrlOpsReg(elemValue, mixerCtrl, curValue) != HDF_SUCCESS ||
        C2rAudioGetCtrlOpsRReg(elemValue, mixerCtrl, rcurValue) != HDF_SUCCESS) {
        return HDF_FAILURE;
    }
    return HDF_SUCCESS;
}

__attribute__((weak)) int32_t AudioCodecMuxRegUpdate(struct CodecDevice *codec,
    struct AudioEnumKcontrol *enumCtrl, const uint32_t *value)
{
    uint32_t val0;
    uint32_t val1;
    int32_t ret;
    if (codec == NULL || enumCtrl == NULL || value == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    if (enumCtrl->values != NULL) {
        val0 = enumCtrl->values[value[0]];
        val1 = enumCtrl->values[value[1]];
    } else {
        val0 = value[0];
        val1 = value[1];
    }
    if (val0 > enumCtrl->max) {
        return HDF_ERR_INVALID_OBJECT;
    }
    ret = C2rAudioUpdateCodecRegBits(codec, enumCtrl->reg, enumCtrl->mask,
        enumCtrl->shiftLeft, val0);
    if (ret != HDF_SUCCESS) {
        return ret;
    }
    if (enumCtrl->reg != enumCtrl->reg2 || enumCtrl->shiftLeft != enumCtrl->shiftRight) {
        if (val1 > enumCtrl->max) {
            return HDF_ERR_INVALID_OBJECT;
        }
        return C2rAudioUpdateCodecRegBits(codec, enumCtrl->reg2, enumCtrl->mask,
            enumCtrl->shiftRight, val1);
    }
    return HDF_SUCCESS;
}

__attribute__((weak)) int32_t AudioCodecGetEnumCtrlOps(const struct AudioKcontrol *kcontrol,
    struct AudioCtrlElemValue *elemValue)
{
    uint32_t curValue = 0;
    uint32_t rcurValue = 0;
    if (kcontrol == NULL || kcontrol->privateValue == 0 || elemValue == NULL) {
        return HDF_ERR_INVALID_OBJECT;
    }
    struct AudioEnumKcontrol *enumCtrl = (struct AudioEnumKcontrol *)(uintptr_t)kcontrol->privateValue;
    struct CodecDevice *codec = AudioKcontrolGetCodec(kcontrol);
    if (codec == NULL || enumCtrl == NULL) {
        return HDF_FAILURE;
    }
    if (AudioCodecReadReg(codec, enumCtrl->reg, &curValue) != HDF_SUCCESS ||
        AudioCodecReadReg(codec, enumCtrl->reg2, &rcurValue) != HDF_SUCCESS) {
        return HDF_FAILURE;
    }
    curValue = (curValue >> enumCtrl->shiftLeft) & enumCtrl->mask;
    if (curValue > enumCtrl->max) {
        return HDF_FAILURE;
    }
    elemValue->value[0] = curValue;
    if (enumCtrl->reg != enumCtrl->reg2 || enumCtrl->shiftLeft != enumCtrl->shiftRight) {
        rcurValue = (rcurValue >> enumCtrl->shiftLeft) & enumCtrl->mask;
        if (rcurValue > enumCtrl->max) {
            return HDF_FAILURE;
        }
        elemValue->value[1] = rcurValue;
    }
    return HDF_SUCCESS;
}

__attribute__((weak)) struct AudioKcontrol *AudioAddControl(const struct AudioCard *audioCard,
    const struct AudioKcontrol *ctrl)
{
    if (audioCard == NULL || ctrl == NULL) {
        return NULL;
    }
    struct AudioKcontrol *control = (struct AudioKcontrol *)calloc(1, sizeof(*control));
    if (control == NULL) {
        return NULL;
    }
    DListHeadInit(&control->list);
    control->name = ctrl->name;
    control->iface = ctrl->iface;
    control->Info = ctrl->Info;
    control->Get = ctrl->Get;
    control->Set = ctrl->Set;
    control->pri = (void *)audioCard;
    control->privateValue = ctrl->privateValue;
    return control;
}
'''
    return None


def _supplemental_audio_product_c_sources(project: str, ohos_root: Path) -> List[Tuple[str, Path]]:
    """返回可直接编译的官方相邻 audio 模块源文件。"""
    if project.startswith("common__"):
        audio_root = ohos_root / "drivers/hdf_core/framework/model/audio"
        return [
            ("audio_core", audio_root / "core/src/audio_core.c"),
            ("audio_parse", audio_root / "core/src/audio_parse.c"),
            ("audio_sapm", audio_root / "sapm/src/audio_sapm.c"),
        ]
    return []


def _build_supplemental_audio_products(
    project: str,
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    timeout: int,
) -> Tuple[List[Path], List[Dict[str, Any]], List[Path]]:
    """编译相邻 audio 产品弱符号，用于补齐拆分模块的真实依赖。"""
    objs: List[Path] = []
    reports: List[Dict[str, Any]] = []
    sources: List[Path] = []
    obj_dir = build_dir / "supplemental_audio_objs"
    obj_dir.mkdir(parents=True, exist_ok=True)

    for stem, src in _supplemental_audio_product_c_sources(project, ohos_root):
        if not src.is_file():
            reports.append({"ok": False, "cmd": [], "cwd": str(build_dir), "error": f"supplemental source not found: {src}"})
            return objs, reports, sources
        obj = obj_dir / f"{stem}.o"
        res = _compile_c_source(src, obj, includes, build_dir, timeout)
        reports.append(res)
        sources.append(src)
        if not res["ok"]:
            return objs, reports, sources
        objs.append(obj)

    source = _supplemental_audio_product_shim_source(project)
    if source is None:
        return objs, reports, sources
    obj, res = _compile_generated_c(build_dir, "c2r_supplemental_audio_products", source, includes, timeout)
    reports.append(res)
    if res["ok"]:
        objs.append(obj)
    sources.append(build_dir / "c2r_supplemental_audio_products.c")
    return objs, reports, sources


def _build_appverify_helpers(
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    source_project_dir: Path,
    static_symbols: Iterable[str],
    timeout: int,
) -> Tuple[List[Path], Dict[str, Any]]:
    """为 appverify_lite 编译 host 侧外部依赖。"""
    reports: Dict[str, Any] = {}
    objs: List[Path] = []
    static_symbol_set = set(static_symbols)

    mbedtls_dir = ohos_root / "third_party/mbedtls/library"
    if mbedtls_dir.is_dir():
        mbedtls_objs = build_dir / "mbedtls_objs"
        mbedtls_objs.mkdir(parents=True, exist_ok=True)
        reports["mbedtls"] = []
        for src in sorted(mbedtls_dir.glob("*.c")):
            if src.name == "psa_crypto_driver_wrappers_no_static.c":
                continue
            obj = mbedtls_objs / f"{src.stem}.o"
            res = _compile_c_source(src, obj, includes, build_dir, timeout)
            reports["mbedtls"].append(res)
            if not res["ok"]:
                return objs, reports
            objs.append(obj)

    cjson = ohos_root / "third_party/cJSON/cJSON.c"
    if cjson.is_file():
        obj = build_dir / "cJSON.o"
        res = _compile_c_source(cjson, obj, includes, build_dir, timeout)
        reports["cjson"] = res
        if not res["ok"]:
            return objs, reports
        objs.append(obj)

    link_globals = build_dir / "appverify_link_globals.c"
    link_global_lines = [
        "#include <stdbool.h>",
        '#include "app_verify_hal.h"',
        '#include "mbedtls/x509_crt.h"',
    ]
    if "g_productDiffFunc" not in static_symbol_set:
        link_global_lines.append("ProductDiff g_productDiffFunc = {0};")
    if "g_rootCaG2Cert" not in static_symbol_set:
        link_global_lines.append("mbedtls_x509_crt g_rootCaG2Cert;")
    if "g_rootCertLoaded" not in static_symbol_set:
        link_global_lines.append("bool g_rootCertLoaded = false;")
    link_global_lines.append("")
    link_globals.write_text("\n".join(link_global_lines), encoding="utf-8")
    reports["link_globals_source"] = {
        "defined_globals": [
            name
            for name in ("g_productDiffFunc", "g_rootCaG2Cert", "g_rootCertLoaded")
            if name not in static_symbol_set
        ],
        "skipped_existing_globals": [
            name
            for name in ("g_productDiffFunc", "g_rootCaG2Cert", "g_rootCertLoaded")
            if name in static_symbol_set
        ],
    }
    obj = build_dir / "appverify_link_globals.o"
    res = _compile_c_source(link_globals, obj, includes, build_dir, timeout)
    reports["link_globals"] = res
    if res["ok"]:
        objs.append(obj)
    return objs, reports


def _audio_test_helper_sources(test_srcs: List[Path], ohos_root: Path) -> List[Path]:
    """按当前 audio gtest 源选择官方 C 侧测试 helper。"""
    helper_src_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/src"
    if not helper_src_dir.is_dir():
        return []
    helpers: List[Path] = []
    seen = set()
    for test_src in test_srcs:
        helper = helper_src_dir / f"{test_src.stem}.c"
        if helper.is_file() and helper not in seen:
            seen.add(helper)
            helpers.append(helper)
    return helpers


def _parse_audio_entry_cases(ohos_root: Path) -> List[Tuple[str, str]]:
    """从官方 audio entry 表提取 subCmd 到 helper 函数的映射。"""
    entry = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/src/hdf_audio_test.c"
    if not entry.is_file():
        return []
    text = entry.read_text(encoding="utf-8", errors="ignore")
    cases: List[Tuple[str, str]] = []
    for match in re.finditer(r"\{\s*([A-Z0-9_]+)\s*,\s*([A-Za-z_][A-Za-z0-9_]*)\s*\}", text):
        cases.append((match.group(1), match.group(2)))
    return cases


def _audio_cmd_suffix(name: str) -> str:
    """把 audio 测试命令名归一成可跨 header 比对的后缀。"""
    for prefix in ("AUDIO_ADM_TEST_", "TEST_"):
        if name.startswith(prefix):
            return name[len(prefix) :]
    if name.startswith("TEST"):
        return name[len("TEST") :]
    return name


def _strip_c_comments(text: str) -> str:
    """移除 C/C++ 注释，避免 enum 解析被注释里的逗号干扰。"""
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return re.sub(r"//.*", "", text)


def _parse_int_literal(text: str) -> Optional[int]:
    """解析 C enum 中常见的整数字面量。"""
    value = text.strip()
    if not value:
        return None
    if not re.fullmatch(r"[-+]?(?:0[xX][0-9A-Fa-f]+|\d+)", value):
        return None
    try:
        return int(value, 0)
    except ValueError:
        return None


def _parse_c_enum_values(text: str) -> Dict[str, int]:
    """解析简单 C enum 常量到整数值的映射。"""
    values: Dict[str, int] = {}
    stripped = _strip_c_comments(text)
    enum_pattern = r"\benum\b(?:\s+[A-Za-z_][A-Za-z0-9_]*)?\s*\{(?P<body>.*?)\}\s*(?:[A-Za-z_][A-Za-z0-9_]*)?\s*;"
    for match in re.finditer(enum_pattern, stripped, re.DOTALL):
        current = 0
        for raw_item in match.group("body").split(","):
            item = raw_item.strip()
            if not item:
                continue
            name_match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)(?:\s*=\s*(.*))?\Z", item, re.DOTALL)
            if not name_match:
                continue
            explicit = _parse_int_literal(name_match.group(2) or "")
            if explicit is not None:
                current = explicit
            values[name_match.group(1)] = current
            current += 1
    return values


def _audio_cmd_values_for_tests(test_srcs: List[Path]) -> Dict[str, int]:
    """读取当前 audio gtest 可见的 TEST* subCmd 枚举值。"""
    values: Dict[str, int] = {}
    headers: List[Path] = []
    seen = set()
    for test_src in test_srcs:
        text = test_src.read_text(encoding="utf-8", errors="ignore")
        for include in re.findall(r'#include\s+"([^"]+)"', text):
            candidate = test_src.parent / include
            if candidate.is_file() and candidate not in seen:
                seen.add(candidate)
                headers.append(candidate)
        if test_src not in seen:
            seen.add(test_src)
            headers.append(test_src)
    for path in headers:
        text = path.read_text(encoding="utf-8", errors="ignore")
        for name, value in _parse_c_enum_values(text).items():
            if name.startswith("TEST"):
                values[name] = value
    return values


def _declared_audio_helper_functions(helper_sources: List[Path], ohos_root: Path) -> List[str]:
    """读取官方 helper 头文件中声明的测试函数。"""
    helper_include_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/include"
    functions: List[str] = []
    seen = set()
    for helper in helper_sources:
        header = helper_include_dir / f"{helper.stem}.h"
        if not header.is_file():
            continue
        text = header.read_text(encoding="utf-8", errors="ignore")
        for match in re.finditer(r"\bint32_t\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*void\s*\)\s*;", text):
            name = match.group(1)
            if name not in seen:
                seen.add(name)
                functions.append(name)
    return functions


def _audio_entry_source(
    helper_sources: List[Path],
    ohos_root: Path,
    test_srcs: Optional[List[Path]] = None,
) -> Tuple[str, Dict[str, Any]]:
    """生成过滤后的 audio HdfAudioEntry，只转发当前测试会用到的 helper。"""
    helper_names = set(_declared_audio_helper_functions(helper_sources, ohos_root))
    selected = [(cmd, fn) for cmd, fn in _parse_audio_entry_cases(ohos_root) if fn in helper_names]
    test_cmd_values = _audio_cmd_values_for_tests(test_srcs or [])
    test_cmd_by_suffix = {_audio_cmd_suffix(name): (name, value) for name, value in test_cmd_values.items()}
    switch_cases: List[Tuple[str, str, str]] = []
    used_case_values = set()
    for cmd, fn in selected:
        suffix = _audio_cmd_suffix(cmd)
        test_cmd = test_cmd_by_suffix.get(suffix)
        if test_cmd is not None:
            label, value = str(test_cmd[1]), test_cmd[0]
        else:
            label, value = cmd, cmd
        if label in used_case_values:
            continue
        used_case_values.add(label)
        switch_cases.append((label, fn, value))
    lines = [
        "/* 临时 audio 测试入口：只保留当前 gtest 源实际需要的官方 helper。 */",
        '#include "hdf_audio_test.h"',
    ]
    for helper in helper_sources:
        lines.append(f'#include "{helper.stem}.h"')
    lines.extend(
        [
            "",
            "int32_t HdfAudioEntry(HdfTestMsg *msg)",
            "{",
            "    int32_t result;",
            "    if (msg == NULL) {",
            "        return HDF_FAILURE;",
            "    }",
            "    switch (msg->subCmd) {",
        ]
    )
    for label, fn, _ in switch_cases:
        lines.extend(
            [
                f"        case {label}:",
                f"            result = {fn}();",
                "            msg->result = (result == HDF_SUCCESS) ? HDF_SUCCESS : HDF_FAILURE;",
                "            return HDF_SUCCESS;",
            ]
        )
    lines.extend(
        [
            "        default:",
            "            return HDF_FAILURE;",
            "    }",
            "}",
            "",
        ]
    )
    meta = {
        "helper_sources": [str(path) for path in helper_sources],
        "selected_cases": [{"cmd": cmd, "function": fn} for cmd, fn in selected],
        "switch_cases": [{"case": label, "function": fn, "source_cmd": value} for label, fn, value in switch_cases],
        "selected_case_count": len(selected),
    }
    return "\n".join(lines), meta


def _build_audio_test_helpers(
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    test_srcs: List[Path],
    timeout: int,
) -> Tuple[List[Path], Dict[str, Any]]:
    """编译官方 audio 测试 helper 和过滤后的 entry。"""
    helper_sources = _audio_test_helper_sources(test_srcs, ohos_root)
    report: Dict[str, Any] = {
        "helper_sources": [str(path) for path in helper_sources],
        "compile": [],
    }
    if not helper_sources:
        return [], report

    objs: List[Path] = []
    obj_dir = build_dir / "audio_helper_objs"
    obj_dir.mkdir(parents=True, exist_ok=True)
    for src in helper_sources:
        obj = obj_dir / f"{src.stem}.o"
        src_to_compile, patch_notes = _prepare_test_source_for_host(src, build_dir)
        res = _compile_c_source(src_to_compile, obj, includes, build_dir, timeout)
        if patch_notes:
            res["source_patches"] = patch_notes
            res["original_source"] = str(src)
        report["compile"].append(res)
        if not res["ok"]:
            return objs, report
        objs.append(obj)

    entry_source, entry_meta = _audio_entry_source(helper_sources, ohos_root, test_srcs)
    report["entry"] = entry_meta
    entry_obj, entry_res = _compile_generated_c(build_dir, "c2r_audio_entry", entry_source, includes, timeout)
    report["entry_build"] = entry_res
    if entry_res["ok"]:
        objs.append(entry_obj)
    return objs, report


def _manager_semantic_fixture_source() -> str:
    """返回 manager 测试的 host 侧语义 fixture，入口真实转发到 Rust manager API。"""
    return r'''
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "devsvc_manager.h"
#include "hdf_base.h"
#include "hdf_device_desc.h"
#include "hdf_io_service.h"
#include "hdf_object_manager.h"
#include "hdf_service_info.h"
#include "hdf_sbuf.h"
#include "hdf_uhdf_test.h"
#include "sample_driver_test.h"

extern struct HdfObject *DevSvcManagerCreate(void);
extern bool DevSvcManagerConstruct(struct DevSvcManager *inst);
extern int DevSvcManagerAddService(
    struct IDevSvcManager *inst, struct HdfDeviceObject *service, const struct HdfServiceInfo *servInfo);
extern struct HdfObject *DevSvcManagerGetService(struct IDevSvcManager *inst, const char *svcName);
extern void DevSvcManagerRemoveService(
    struct IDevSvcManager *inst, const char *svcName, const struct HdfDeviceObject *devObj);
extern void DevSvcManagerListService(struct HdfSBuf *serviceNameSet, DeviceClass deviceClass);
extern void DevSvcManagerListAllService(struct IDevSvcManager *inst, struct HdfSBuf *reply);

static struct IDevSvcManager *g_c2r_manager = NULL;
static struct HdfIoDispatcher g_c2r_dispatcher;

static struct IDeviceIoService g_c2r_khdf_service = {0};
static struct IDeviceIoService g_c2r_dev_mgr_service = {0};
static struct IDeviceIoService g_c2r_sample_service = {0};
static struct IDeviceIoService g_c2r_dynamic_service = {0};

static struct HdfDeviceObject g_c2r_khdf_device = {0};
static struct HdfDeviceObject g_c2r_dev_mgr_device = {0};
static struct HdfDeviceObject g_c2r_sample_device = {0};
static struct HdfDeviceObject g_c2r_dynamic_device = {0};
static struct HdfIoService g_c2r_bound_service = {0};

struct HdfObject *HdfObjectManagerGetObject(int objectId)
{
    if (objectId == HDF_OBJECT_ID_DEVSVC_MANAGER) {
        return DevSvcManagerCreate();
    }
    return NULL;
}

static const char *C2rReadString(struct HdfSBuf *data)
{
    return (data == NULL) ? NULL : HdfSbufReadString(data);
}

static void C2rInitDevice(struct HdfDeviceObject *device, struct IDeviceIoService *service, DeviceClass deviceClass)
{
    if (device == NULL || service == NULL) {
        return;
    }
    memset(device, 0, sizeof(*device));
    memset(service, 0, sizeof(*service));
    device->service = service;
    device->deviceClass = deviceClass;
}

static int C2rRegisterService(const char *serviceName, struct HdfDeviceObject *device, DeviceClass deviceClass)
{
    struct HdfServiceInfo info;
    if (g_c2r_manager == NULL || serviceName == NULL || device == NULL) {
        return HDF_FAILURE;
    }
    memset(&info, 0, sizeof(info));
    info.servName = serviceName;
    info.servInfo = "";
    info.devClass = (uint16_t)deviceClass;
    info.devId = 0;
    info.interfaceDesc = "";
    return DevSvcManagerAddService(g_c2r_manager, device, &info);
}

static int C2rSampleDispatch(struct HdfObject *service, int cmdId, struct HdfSBuf *data, struct HdfSBuf *reply)
{
    (void)service;
    (void)reply;
    struct HdfSBuf *localData = NULL;
    if (g_c2r_manager == NULL) {
        return HDF_FAILURE;
    }
    if (data != NULL) {
        localData = HdfSbufCopy(data);
        if (localData == NULL) {
            return HDF_FAILURE;
        }
    }
    if (cmdId == SAMPLE_DRIVER_REGISTER_DEVICE) {
        const char *moduleName = C2rReadString(localData);
        const char *serviceName = C2rReadString(localData);
        (void)moduleName;
        if (serviceName == NULL) {
            HdfSbufRecycle(localData);
            return HDF_FAILURE;
        }
        C2rInitDevice(&g_c2r_dynamic_device, &g_c2r_dynamic_service, DEVICE_CLASS_DEFAULT);
        int ret = C2rRegisterService(serviceName, &g_c2r_dynamic_device, DEVICE_CLASS_DEFAULT);
        HdfSbufRecycle(localData);
        return ret;
    }
    if (cmdId == SAMPLE_DRIVER_UNREGISTER_DEVICE) {
        const char *moduleName = C2rReadString(localData);
        const char *serviceName = C2rReadString(localData);
        (void)moduleName;
        if (serviceName == NULL) {
            HdfSbufRecycle(localData);
            return HDF_FAILURE;
        }
        DevSvcManagerRemoveService(g_c2r_manager, serviceName, &g_c2r_dynamic_device);
        HdfSbufRecycle(localData);
        return HDF_SUCCESS;
    }
    HdfSbufRecycle(localData);
    return HDF_FAILURE;
}

static void C2rEnsureManager(void)
{
    struct HdfObject *object;
    if (g_c2r_manager != NULL) {
        return;
    }
    object = DevSvcManagerCreate();
    if (object == NULL) {
        return;
    }
    g_c2r_manager = (struct IDevSvcManager *)object;

    g_c2r_dispatcher.Dispatch = C2rSampleDispatch;
    C2rInitDevice(&g_c2r_khdf_device, &g_c2r_khdf_service, DEVICE_CLASS_DEFAULT);
    C2rInitDevice(&g_c2r_dev_mgr_device, &g_c2r_dev_mgr_service, DEVICE_CLASS_DEFAULT);
    C2rInitDevice(&g_c2r_sample_device, &g_c2r_sample_service, DEVICE_CLASS_DEFAULT);

    (void)C2rRegisterService(HDF_TEST_SERVICE_NAME, &g_c2r_khdf_device, DEVICE_CLASS_DEFAULT);
    (void)C2rRegisterService(DEV_MGR_NODE, &g_c2r_dev_mgr_device, DEVICE_CLASS_DEFAULT);
    (void)C2rRegisterService(SAMPLE_SERVICE, &g_c2r_sample_device, DEVICE_CLASS_DEFAULT);
}

void HdfTestOpenService(void)
{
    C2rEnsureManager();
}

void HdfTestCloseService(void)
{
}

struct HdfIoService *HdfIoServiceBind(const char *serviceName)
{
    struct HdfObject *service;
    C2rEnsureManager();
    if (g_c2r_manager == NULL || serviceName == NULL) {
        return NULL;
    }
    service = DevSvcManagerGetService(g_c2r_manager, serviceName);
    if (service == NULL) {
        return NULL;
    }
    memset(&g_c2r_bound_service, 0, sizeof(g_c2r_bound_service));
    g_c2r_bound_service.target = service;
    g_c2r_bound_service.dispatcher = &g_c2r_dispatcher;
    return &g_c2r_bound_service;
}

void HdfIoServiceRecycle(struct HdfIoService *service)
{
    (void)service;
}

int32_t HdfGetServiceNameByDeviceClass(DeviceClass deviceClass, struct HdfSBuf *reply)
{
    C2rEnsureManager();
    if (g_c2r_manager == NULL || reply == NULL) {
        return HDF_FAILURE;
    }
    DevSvcManagerListService(reply, deviceClass);
    return HDF_SUCCESS;
}

int32_t HdfListAllService(struct HdfSBuf *reply)
{
    C2rEnsureManager();
    if (g_c2r_manager == NULL || reply == NULL) {
        return HDF_FAILURE;
    }
    DevSvcManagerListAllService(g_c2r_manager, reply);
    return HDF_SUCCESS;
}
'''


def _pre_staticlib_helper_sources(project: str, ohos_root: Path) -> List[Tuple[str, Path, List[str]]]:
    """返回需要先于 Rust 导出分析编译的官方 helper 源。"""
    if project.startswith("posix__"):
        base = ohos_root / "drivers/hdf_core/framework/test/unittest/osal"
        return [
            ("osal_all_test", base / "osal_all_test.c", ["-D__USER__"]),
            ("osal_get_case_test", base / "osal_get_case_test.c", ["-D__USER__"]),
            ("osal_list_test", base / "osal_list_test.c", ["-D__USER__"]),
            ("osal_work_test", base / "osal_work_test.c", ["-D__USER__"]),
            ("osal_test_entry", base / "osal_test_entry.c", ["-D__USER__"]),
        ]
    return []


def _pre_staticlib_generated_sources(project: str) -> List[Tuple[str, str]]:
    """返回需要先于 Rust 导出分析编译的生成 helper 源。"""
    if project.startswith("manager__"):
        return [("c2r_manager_semantic_fixture", _manager_semantic_fixture_source())]
    return []


def _build_pre_staticlib_helpers(
    project: str,
    build_dir: Path,
    ohos_root: Path,
    includes: List[Path],
    timeout: int,
) -> Tuple[List[Path], List[Dict[str, Any]]]:
    """编译会调用被测 Rust 符号的官方 helper。"""
    objs: List[Path] = []
    reports: List[Dict[str, Any]] = []
    obj_dir = build_dir / "pre_staticlib_helper_objs"
    obj_dir.mkdir(parents=True, exist_ok=True)
    for stem, src, extra in _pre_staticlib_helper_sources(project, ohos_root):
        if not src.is_file():
            reports.append({"ok": False, "cmd": [], "cwd": str(build_dir), "error": f"helper source not found: {src}"})
            return objs, reports
        obj = obj_dir / f"{stem}.o"
        res = _compile_c_source(src, obj, includes, build_dir, timeout, extra)
        reports.append(res)
        if not res["ok"]:
            return objs, reports
        objs.append(obj)
    for stem, source in _pre_staticlib_generated_sources(project):
        obj, res = _compile_generated_c(build_dir, stem, source, includes, timeout)
        reports.append(res)
        if not res["ok"]:
            return objs, reports
        objs.append(obj)
    return objs, reports


def _hdf_workqueue_shim_source() -> str:
    """返回 host 侧 workqueue shim，用于 OSAL 官方 helper。"""
    return r'''
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "hdf_workqueue.h"

struct C2rWork {
    HdfWorkFunc func;
    void *arg;
};

__attribute__((weak)) int32_t HdfWorkQueueInit(HdfWorkQueue *queue, char *name)
{
    (void)name;
    if (queue == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    queue->realWorkQueue = queue;
    return HDF_SUCCESS;
}

__attribute__((weak)) int32_t HdfWorkInit(HdfWork *work, HdfWorkFunc func, void *arg)
{
    if (work == NULL || func == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    struct C2rWork *real = (struct C2rWork *)calloc(1, sizeof(*real));
    if (real == NULL) {
        return HDF_ERR_MALLOC_FAIL;
    }
    real->func = func;
    real->arg = arg;
    work->realWork = real;
    return HDF_SUCCESS;
}

__attribute__((weak)) int32_t HdfDelayedWorkInit(HdfWork *work, HdfWorkFunc func, void *arg)
{
    return HdfWorkInit(work, func, arg);
}

__attribute__((weak)) void HdfWorkDestroy(HdfWork *work)
{
    if (work != NULL && work->realWork != NULL) {
        free(work->realWork);
        work->realWork = NULL;
    }
}

__attribute__((weak)) void HdfDelayedWorkDestroy(HdfWork *work)
{
    HdfWorkDestroy(work);
}

__attribute__((weak)) void HdfWorkQueueDestroy(HdfWorkQueue *queue)
{
    if (queue != NULL) {
        queue->realWorkQueue = NULL;
    }
}

static bool C2rRunWork(HdfWork *work)
{
    if (work == NULL || work->realWork == NULL) {
        return false;
    }
    struct C2rWork *real = (struct C2rWork *)work->realWork;
    if (real->func == NULL) {
        return false;
    }
    real->func(real->arg);
    return true;
}

__attribute__((weak)) bool HdfAddWork(HdfWorkQueue *queue, HdfWork *work)
{
    (void)queue;
    return C2rRunWork(work);
}

__attribute__((weak)) bool HdfAddDelayedWork(HdfWorkQueue *queue, HdfWork *work, uint32_t ms)
{
    (void)queue;
    (void)ms;
    return C2rRunWork(work);
}

__attribute__((weak)) unsigned int HdfWorkBusy(HdfWork *work)
{
    (void)work;
    return 0;
}

__attribute__((weak)) bool HdfCancelWorkSync(HdfWork *work)
{
    (void)work;
    return true;
}

__attribute__((weak)) bool HdfCancelDelayedWorkSync(HdfWork *work)
{
    (void)work;
    return true;
}
'''


def _osal_file_host_shim_source() -> str:
    """返回 host 侧 OSAL 文件 shim，用真实 POSIX 文件语义支撑官方 helper。"""
    return r'''
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <sys/types.h>
#include <unistd.h>

#include "hdf_base.h"
#include "osal_file.h"

__attribute__((weak)) int32_t OsalFileOpen(OsalFile *file, const char *path, int flags, uint32_t rights)
{
    if (file == NULL || path == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }

    int fd = open(path, flags, (mode_t)rights);
    if (fd < 0) {
        file->realFile = NULL;
        return HDF_FAILURE;
    }
    file->realFile = (void *)(intptr_t)fd;
    return HDF_SUCCESS;
}

__attribute__((weak)) ssize_t OsalFileWrite(OsalFile *file, const void *string, uint32_t length)
{
    if (file == NULL || file->realFile == NULL || string == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    int fd = (int)(intptr_t)file->realFile;
    ssize_t ret = write(fd, string, (size_t)length);
    return (ret < 0) ? HDF_FAILURE : ret;
}

__attribute__((weak)) void OsalFileClose(OsalFile *file)
{
    if (file == NULL || file->realFile == NULL) {
        return;
    }
    int fd = (int)(intptr_t)file->realFile;
    close(fd);
    file->realFile = NULL;
}

__attribute__((weak)) ssize_t OsalFileRead(OsalFile *file, void *buf, uint32_t length)
{
    if (file == NULL || file->realFile == NULL || buf == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    int fd = (int)(intptr_t)file->realFile;
    ssize_t ret = read(fd, buf, (size_t)length);
    return (ret < 0) ? HDF_FAILURE : ret;
}

__attribute__((weak)) off_t OsalFileLseek(OsalFile *file, off_t offset, int32_t whence)
{
    if (file == NULL || file->realFile == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    int fd = (int)(intptr_t)file->realFile;
    off_t ret = lseek(fd, offset, whence);
    return (ret < 0) ? HDF_FAILURE : ret;
}
'''


def _core_audio_fixture_source() -> str:
    """返回 audio core 测试的 host 侧配置树 fixture。"""
    return r'''
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "audio_parse.h"
#include "devsvc_manager_clnt.h"
#include "device_resource_if.h"
#include "hdf_base.h"
#include "hdf_device_desc.h"

extern int32_t AudioGetRegConfig(const struct HdfDeviceObject *device, struct AudioRegCfgData *configData);

enum {
    C2R_NODE_ROOT = 1,
    C2R_NODE_IDINFO = 2,
    C2R_NODE_REGCONFIG = 3,
};

static struct DeviceResourceAttr g_c2r_reset_attr = {"resetSeqConfig", "", NULL};
static struct DeviceResourceNode g_c2r_reg_config = {"regConfig", C2R_NODE_REGCONFIG, &g_c2r_reset_attr, NULL, NULL, NULL};
static struct DeviceResourceNode g_c2r_id_info = {"idInfo", C2R_NODE_IDINFO, NULL, NULL, NULL, &g_c2r_reg_config};
static struct DeviceResourceNode g_c2r_root = {"audio", C2R_NODE_ROOT, NULL, NULL, &g_c2r_id_info, NULL};
static struct HdfDeviceObject g_c2r_audio_device = {0};

static void C2rInitAudioTree(void)
{
    g_c2r_root.child = &g_c2r_id_info;
    g_c2r_id_info.parent = &g_c2r_root;
    g_c2r_id_info.sibling = &g_c2r_reg_config;
    g_c2r_reg_config.parent = &g_c2r_root;
    g_c2r_audio_device.property = &g_c2r_root;
}

static const char *C2rStringValue(const char *name, const char *def)
{
    if (name == NULL) {
        return def;
    }
    if (strcmp(name, "serviceName") == 0) {
        return "hdf_audio_codec_dev0";
    }
    if (strcmp(name, "codecName") == 0) {
        return "codec";
    }
    if (strcmp(name, "platformName") == 0) {
        return "platform";
    }
    if (strcmp(name, "cpuDaiName") == 0) {
        return "cpuDai";
    }
    if (strcmp(name, "codecDaiName") == 0) {
        return "codecDai";
    }
    if (strcmp(name, "dspName") == 0) {
        return "dsp";
    }
    if (strcmp(name, "dspDaiName") == 0) {
        return "dspDai";
    }
    if (strcmp(name, "chipName") == 0) {
        return "codec";
    }
    return def;
}

static const struct DeviceResourceNode *C2rGetRootNode(void)
{
    C2rInitAudioTree();
    return &g_c2r_root;
}

static int C2rIsKnownAudioNode(const struct DeviceResourceNode *node)
{
    C2rInitAudioTree();
    return node == &g_c2r_root || node == &g_c2r_id_info || node == &g_c2r_reg_config;
}

static int32_t C2rGetString(const struct DeviceResourceNode *node, const char *attrName, const char **value,
    const char *def)
{
    if (value == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    if (!C2rIsKnownAudioNode(node)) {
        *value = def;
        return HDF_FAILURE;
    }
    *value = C2rStringValue(attrName, def);
    return (*value == NULL) ? HDF_FAILURE : HDF_SUCCESS;
}

static int32_t C2rGetUint32(const struct DeviceResourceNode *node, const char *attrName, uint32_t *value,
    uint32_t def)
{
    if (value == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    if (!C2rIsKnownAudioNode(node)) {
        *value = def;
        return HDF_FAILURE;
    }
    if (attrName != NULL && strcmp(attrName, "chipIdRegister") == 0) {
        *value = 0;
        return HDF_SUCCESS;
    }
    if (attrName != NULL && strcmp(attrName, "chipIdSize") == 0) {
        *value = 4;
        return HDF_SUCCESS;
    }
    *value = def;
    return HDF_SUCCESS;
}

static int32_t C2rGetElemNum(const struct DeviceResourceNode *node, const char *attrName)
{
    if (!C2rIsKnownAudioNode(node)) {
        return 0;
    }
    if (attrName != NULL && strcmp(attrName, "resetSeqConfig") == 0) {
        return 2;
    }
    if (attrName != NULL && strcmp(attrName, "hwInfo") == 0) {
        return 12;
    }
    return 0;
}

static int32_t C2rGetUint32Array(const struct DeviceResourceNode *node, const char *attrName, uint32_t *value,
    uint32_t len, uint32_t def)
{
    if (value == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    for (uint32_t i = 0; i < len; ++i) {
        value[i] = def;
    }
    if (!C2rIsKnownAudioNode(node)) {
        return HDF_FAILURE;
    }
    if (attrName != NULL && strcmp(attrName, "resetSeqConfig") == 0 && len >= 2) {
        value[0] = 0;
        value[1] = 0;
        return HDF_SUCCESS;
    }
    return HDF_FAILURE;
}

static int32_t C2rGetUint64Array(const struct DeviceResourceNode *node, const char *attrName, uint64_t *value,
    uint32_t len, uint64_t def)
{
    if (value == NULL) {
        return HDF_ERR_INVALID_PARAM;
    }
    for (uint32_t i = 0; i < len; ++i) {
        value[i] = def;
    }
    if (!C2rIsKnownAudioNode(node)) {
        return HDF_FAILURE;
    }
    if (attrName != NULL && strcmp(attrName, "hwInfo") == 0 && len >= 12) {
        value[0] = PORT_OUT;
        return HDF_SUCCESS;
    }
    return HDF_FAILURE;
}

static const struct DeviceResourceNode *C2rGetChildNode(const struct DeviceResourceNode *node, const char *nodeName)
{
    if (nodeName == NULL || !C2rIsKnownAudioNode(node)) {
        return NULL;
    }
    C2rInitAudioTree();
    if (strcmp(nodeName, "idInfo") == 0) {
        return &g_c2r_id_info;
    }
    if (strcmp(nodeName, "regConfig") == 0) {
        return &g_c2r_reg_config;
    }
    return NULL;
}

static struct DeviceResourceIface g_c2r_resource_iface = {
    .GetRootNode = C2rGetRootNode,
    .GetUint32 = C2rGetUint32,
    .GetUint32Array = C2rGetUint32Array,
    .GetUint64Array = C2rGetUint64Array,
    .GetString = C2rGetString,
    .GetElemNum = C2rGetElemNum,
    .GetChildNode = C2rGetChildNode,
};

__attribute__((weak)) struct DeviceResourceIface *DeviceResourceGetIfaceInstance(DeviceResourceType type)
{
    if (type != HDF_CONFIG_SOURCE) {
        return NULL;
    }
    return &g_c2r_resource_iface;
}

__attribute__((weak)) struct HdfDeviceObject *DevSvcManagerClntGetDeviceObject(const char *svcName)
{
    (void)svcName;
    C2rInitAudioTree();
    return &g_c2r_audio_device;
}

__attribute__((weak)) int32_t HdfDriverRegister(const struct HdfDriverEntry *driverEntry)
{
    (void)driverEntry;
    return HDF_SUCCESS;
}

__attribute__((weak)) bool HdfDeviceSetClass(struct HdfDeviceObject *deviceObject, DeviceClass deviceClass)
{
    if (deviceObject == NULL || deviceClass >= DEVICE_CLASS_MAX) {
        return false;
    }
    deviceObject->deviceClass = deviceClass;
    return true;
}

__attribute__((weak)) int32_t CodecGetRegConfig(const struct HdfDeviceObject *device,
    struct AudioRegCfgData *configData)
{
    return AudioGetRegConfig(device, configData);
}
'''


def _environment_shim_sources(project: str) -> List[Tuple[str, str]]:
    """返回不会替代被测翻译函数的 host 环境 shim 源。"""
    shims: List[Tuple[str, str]] = [("c2r_host_shim", _host_shim_source()), ("c2r_path_wrap", _path_wrap_source())]
    if project.startswith("posix__"):
        shims.append(("c2r_hdf_workqueue_shim", _hdf_workqueue_shim_source()))
        shims.append(("c2r_osal_file_host_shim", _osal_file_host_shim_source()))
    if project.startswith("core__") or project.startswith("common__"):
        shims.append(("c2r_core_audio_fixture", _core_audio_fixture_source()))
    return shims


def _link_wrap_flags_for_project(project: str) -> List[str]:
    """返回项目需要的链接 wrapper 参数。"""
    return ["-Wl,--wrap=realpath", "-Wl,--wrap=dlopen"]


def _extra_link_flags_for_project(project: str) -> List[str]:
    """返回项目 staticlib 依赖的系统链接参数。"""
    if project.startswith("appverify_lite__"):
        return ["-lssl", "-lcrypto"]
    return []


def _extra_export_names_for_project(project: str) -> List[str]:
    """返回 host fixture 直接调用的 Rust 被测符号名。"""
    if project.startswith("core__"):
        return ["AudioGetRegConfig"]
    return []


def _host_shim_source() -> str:
    """返回 host 侧环境 shim，不替代被测翻译函数。"""
    return r'''
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "hdf_sbuf.h"
#include "hdf_sbuf_impl.h"

struct HdfSBufImpl;
struct HdfSBufImpl *SbufObtainRaw(size_t capacity);
struct HdfSBufImpl *SbufBindRaw(uintptr_t base, size_t size);

static bool C2rIpcSbufWriteString(struct HdfSBufImpl *sbuf, const char *value)
{
    if (sbuf == NULL || value == NULL) {
        return false;
    }
    return sbuf->writeBuffer(sbuf, (const uint8_t *)value, (uint32_t)(strlen(value) + 1));
}

static struct HdfSBufImpl *C2rWrapIpcSbuf(struct HdfSBufImpl *sbuf)
{
    if (sbuf == NULL) {
        return NULL;
    }
    sbuf->writeString = C2rIpcSbufWriteString;
    return sbuf;
}

__attribute__((weak)) struct HdfSBufImpl *SbufObtainIpc(size_t capacity) { return C2rWrapIpcSbuf(SbufObtainRaw(capacity)); }
__attribute__((weak)) struct HdfSBufImpl *SbufBindIpc(uintptr_t base, size_t size) { return C2rWrapIpcSbuf(SbufBindRaw(base, size)); }
__attribute__((weak)) struct HdfSBufImpl *SbufObtainIpcHw(size_t capacity) { return C2rWrapIpcSbuf(SbufObtainRaw(capacity)); }
__attribute__((weak)) struct HdfSBufImpl *SbufBindRawIpcHw(uintptr_t base, size_t size) { return C2rWrapIpcSbuf(SbufBindRaw(base, size)); }

__attribute__((weak)) int HiLogPrint(int type, int level, unsigned int domain, const char *tag, const char *fmt, ...)
{
    (void)type; (void)level; (void)domain; (void)tag; (void)fmt;
    return 0;
}

__attribute__((weak)) void *OsalMemCalloc(size_t size) { return calloc(1, size); }
__attribute__((weak)) void *OsalMemAlloc(size_t size) { return malloc(size); }
__attribute__((weak)) void *OsalMemRealloc(void *ptr, size_t size) { return realloc(ptr, size); }
__attribute__((weak)) void OsalMemFree(void *mem) { free(mem); }
__attribute__((weak)) void *OsalIoRemap(unsigned long phys_addr, unsigned long size)
{
    (void)phys_addr;
    return calloc(1, size == 0 ? 1 : (size_t)size);
}
__attribute__((weak)) void OsalIoUnmap(void *addr) { (void)addr; }
__attribute__((weak)) void iounmap(void *addr) { (void)addr; }

struct HdfSList;
struct DevHostServiceClnt;
struct IDevHostService;
typedef struct {
    void **nodes;
    uint32_t nodeSize;
    uint32_t bucketSize;
} C2rHdfMap;

__attribute__((weak)) bool HdfAttributeManagerGetHostList(struct HdfSList *hostList)
{
    (void)hostList;
    return true;
}

__attribute__((weak)) int HdfAttributeManagerGetDeviceList(struct DevHostServiceClnt *hostClnt)
{
    (void)hostClnt;
    return 0;
}

__attribute__((weak)) struct IDevHostService *DevHostServiceNewInstance(uint16_t hostId, const char *hostName)
{
    (void)hostId;
    (void)hostName;
    return (struct IDevHostService *)calloc(1, 4096);
}

__attribute__((weak)) void DevHostServiceFreeInstance(struct IDevHostService *service)
{
    free(service);
}

__attribute__((weak)) int DeviceManagerIsQuickLoad(void)
{
    return 0;
}

__attribute__((weak)) void MapInit(C2rHdfMap *map)
{
    if (map == NULL) {
        return;
    }
    map->nodes = NULL;
    map->nodeSize = 0;
    map->bucketSize = 0;
}

__attribute__((weak)) int HdfDeviceObjectSetServInfo(struct HdfDeviceObject *dev, const char *info)
{
    (void)dev;
    (void)info;
    return 0;
}

__attribute__((weak)) int HdfDeviceObjectUpdate(struct HdfDeviceObject *dev)
{
    (void)dev;
    return 0;
}

struct I2cMsg;
__attribute__((weak)) void *I2cOpen(int16_t number)
{
    (void)number;
    return (void *)(uintptr_t)1;
}
__attribute__((weak)) void I2cClose(void *handle) { (void)handle; }
__attribute__((weak)) int32_t I2cTransfer(void *handle, struct I2cMsg *msgs, int16_t count)
{
    (void)msgs;
    return (handle == NULL || count < 0) ? -1 : count;
}

__attribute__((weak)) int32_t CopyFromUser(void *dest, const void *src, uint32_t count)
{
    if (count == 0) {
        return 0;
    }
    if (dest == NULL || src == NULL) {
        return -1;
    }
    memcpy(dest, src, (size_t)count);
    return 0;
}

__attribute__((weak)) int32_t CopyToUser(void *dest, const void *src, uint32_t count)
{
    if (count == 0) {
        return 0;
    }
    if (dest == NULL || src == NULL) {
        return -1;
    }
    memcpy(dest, src, (size_t)count);
    return 0;
}

__attribute__((weak)) int GetDevUdid(char *udid, int size)
{
    if (udid == NULL || size <= 0) {
        return -1;
    }
    const char *value = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    size_t n = strlen(value);
    if (n >= (size_t)size) {
        n = (size_t)size - 1;
    }
    memcpy(udid, value, n);
    udid[n] = '\0';
    return 0;
}

struct HdfIoService { void *dispatcher; void *target; };
static struct HdfIoService g_c2r_host_io_service = {0};
__attribute__((weak)) struct HdfIoService *HdfIoServiceBind(const char *serviceName)
{
    (void)serviceName;
    return &g_c2r_host_io_service;
}
__attribute__((weak)) void HdfIoServiceRecycle(struct HdfIoService *service) { (void)service; }

struct HdfObjectCreator;
__attribute__((weak)) const struct HdfObjectCreator *HdfObjectManagerGetCreators(int objectId)
{
    (void)objectId;
    return NULL;
}

struct DeviceResourceNode;
__attribute__((weak)) void SetHcsBlobPath(const char *path) { (void)path; }
__attribute__((weak)) const struct DeviceResourceNode *HcsGetRootNode(void) { return NULL; }

struct HdfDeviceObject;
struct HdfDeviceIoClient;
struct HdfSBuf;
struct HdfTestMsg {
    uint16_t cmd;
    uint8_t subCmd;
    int8_t result;
};
extern int32_t HdfAudioEntry(struct HdfTestMsg *msg) __attribute__((weak));
extern int32_t HdfOsalEntry(struct HdfTestMsg *msg) __attribute__((weak));

static struct HdfDeviceObject *g_c2r_registered_device = NULL;
__attribute__((weak)) struct HdfDeviceObject *HdfRegisteDevice(const char *moduleName, const char *serviceName)
{
    (void)moduleName; (void)serviceName;
    if (g_c2r_registered_device == NULL) {
        g_c2r_registered_device = (struct HdfDeviceObject *)calloc(1, 4096);
    }
    return g_c2r_registered_device;
}
__attribute__((weak)) void HdfUnregisteDevice(const char *moduleName, const char *serviceName)
{
    (void)moduleName; (void)serviceName;
}
__attribute__((weak)) int HdfDeviceSendEvent(struct HdfDeviceObject *device, unsigned int id, struct HdfSBuf *data)
{
    (void)device; (void)id; (void)data;
    return 0;
}
__attribute__((weak)) int HdfDeviceSendEventToClient(struct HdfDeviceIoClient *client, unsigned int id, struct HdfSBuf *data)
{
    (void)client; (void)id; (void)data;
    return 0;
}

__attribute__((weak)) void HdfTestOpenService(void) {}
__attribute__((weak)) void HdfTestCloseService(void) {}
__attribute__((weak)) int HdfTestSendMsgToService(struct HdfTestMsg *msg)
{
    if (msg == NULL) {
        return -1;
    }
    if (msg->cmd == 701) {
        if (HdfAudioEntry == NULL) {
            return -1;
        }
        if (HdfAudioEntry(msg) != 0) {
            return -1;
        }
        return msg->result;
    }
    if (msg->cmd == 201) {
        if (HdfOsalEntry == NULL) {
            return -1;
        }
        if (HdfOsalEntry(msg) != 0) {
            return -1;
        }
        return msg->result;
    }
    return -1;
}
'''


def _path_wrap_source() -> str:
    """返回 OHOS 绝对库路径到临时 fixture 目录的 wrapper。"""
    return r'''
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *fixture_root(void)
{
    const char *root = getenv("C2R_OHOS_LIB_FIXTURE_DIR");
    return (root && root[0]) ? root : NULL;
}

static int map_ohos_lib_path(const char *in, char *out, size_t out_sz)
{
    if (!in || !out || out_sz == 0) {
        return 0;
    }
    const char *prefixes[] = {"/vendor/lib/", "/vendor/lib64/"};
    for (size_t i = 0; i < sizeof(prefixes) / sizeof(prefixes[0]); ++i) {
        const char *prefix = prefixes[i];
        size_t n = strlen(prefix);
        if (strncmp(in, prefix, n) == 0) {
            const char *root = fixture_root();
            if (!root) {
                return 0;
            }
            int rc = snprintf(out, out_sz, "%s/%s", root, in + n);
            return rc > 0 && (size_t)rc < out_sz;
        }
    }
    return 0;
}

char *__real_realpath(const char *path, char *resolved_path);
char *__wrap_realpath(const char *path, char *resolved_path)
{
    char mapped[PATH_MAX + 1];
    if (map_ohos_lib_path(path, mapped, sizeof(mapped))) {
        char actual[PATH_MAX + 1];
        char *r = __real_realpath(mapped, actual);
        if (!r) {
            return NULL;
        }
        if (resolved_path) {
            strncpy(resolved_path, path, PATH_MAX);
            resolved_path[PATH_MAX] = '\0';
            return resolved_path;
        }
        char *heap = (char *)malloc(strlen(path) + 1);
        if (!heap) {
            errno = ENOMEM;
            return NULL;
        }
        strcpy(heap, path);
        return heap;
    }
    return __real_realpath(path, resolved_path);
}

void *__real_dlopen(const char *filename, int flags);
void *__wrap_dlopen(const char *filename, int flags)
{
    char mapped[PATH_MAX + 1];
    if (map_ohos_lib_path(filename, mapped, sizeof(mapped))) {
        return __real_dlopen(mapped, flags);
    }
    return __real_dlopen(filename, flags);
}
'''


def _compile_generated_c(build_dir: Path, name: str, source: str, includes: List[Path], timeout: int) -> Tuple[Path, Dict[str, Any]]:
    """编译脚本生成的 C shim。"""
    src = build_dir / f"{name}.c"
    obj = build_dir / f"{name}.o"
    src.write_text(source, encoding="utf-8")
    res = _compile_c_source(src, obj, includes, build_dir, timeout)
    return obj, res


def _patch_cpp_vdi_init(text: str) -> str:
    """把 C++ fixture 中的 HDF_VDI_INIT 宏展开为可导出的 C ABI 符号。"""
    return re.sub(
        r"HDF_VDI_INIT\s*\(\s*([^\)]+)\s*\)\s*;",
        lambda match: f'extern "C" struct HdfVdiBase *hdfVdiDesc = (struct HdfVdiBase *)&({match.group(1)});',
        text,
        count=1,
    )


def _build_host_fixtures(
    project: str,
    build_dir: Path,
    test_srcs: List[Path],
    sample_srcs: List[Path],
    includes: List[Path],
    timeout: int,
) -> Dict[str, Any]:
    """构建 host 项目测试需要的 VDI 动态库 fixture。"""
    report: Dict[str, Any] = {}
    if not project.startswith("host__"):
        return report
    wanted: List[str] = []
    lib_re = re.compile(r'"(lib[^"]+\.z\.so)"')
    for src in test_srcs:
        wanted.extend(lib_re.findall(src.read_text(encoding="utf-8", errors="ignore")))
    wanted = sorted(set(wanted))
    if not wanted:
        return report

    sample_by_stem = {path.stem: path for path in sample_srcs}
    mapping = {
        "libvdi_sample1_driver.z.so": "vdi_sample1_driver",
        "libvdi_sample1_symbol.z.so": "vdi_sample1_symbol",
        "libvdi_sample2_driver.z.so": "vdi_sample2_driver",
    }
    for lib_name in wanted:
        if lib_name.endswith("_error.z.so"):
            dummy = build_dir / f"{lib_name}.c"
            dummy.write_text("void __c2r_dummy(void) {}\n", encoding="utf-8")
            cmd = ["gcc", "-shared", "-fPIC", "-O2", str(dummy), "-o", str(build_dir / lib_name)]
            res = run_command(cmd, build_dir, timeout)
            report[lib_name] = res
            continue
        stem = mapping.get(lib_name)
        src = sample_by_stem.get(stem or "")
        if not src:
            report[lib_name] = {"ok": False, "error": "sample source not found"}
            continue
        src_to_build = src
        if src.suffix.lower() != ".c":
            text = src.read_text(encoding="utf-8", errors="ignore")
            text = _patch_cpp_vdi_init(text)
            patched = build_dir / f"patched_{src.name}"
            patched.write_text(text, encoding="utf-8")
            src_to_build = patched
        cc = "gcc" if src_to_build.suffix.lower() == ".c" else "g++"
        cmd = [cc, "-shared", "-fPIC", "-O2"]
        if cc == "g++":
            cmd.append("-std=c++17")
        for inc in includes:
            cmd.extend(["-I", str(inc)])
        cmd.extend([str(src_to_build), "-o", str(build_dir / lib_name)])
        report[lib_name] = run_command(cmd, build_dir, timeout)
    return report


def build_and_run_project(
    *,
    project: str,
    crate_dir: Path,
    source_dir: Path,
    ohos_root: Path = OHOS_ROOT,
    timeout: int = 900,
    keep_build_dir: bool = False,
) -> Dict[str, Any]:
    """执行单个 C 项目的真实测试链路。"""
    report: Dict[str, Any] = {
        "project": project,
        "stage": "start",
        "compiled": False,
        "executed": False,
        "crate_dir": str(crate_dir),
        "source_dir": str(source_dir),
        "tests_total": None,
        "tests_passed": None,
        "tests_failed": None,
        "pass_rate": None,
        "failed_tests": [],
    }
    missing = [str(path) for path in (crate_dir / "Cargo.toml", source_dir, ohos_root) if not path.exists()]
    if missing:
        report.update({"stage": "input_validation", "error": "missing inputs", "missing": missing})
        return report

    test_srcs, sample_srcs, err = collect_test_sources(source_dir)
    test_srcs, excluded_test_srcs = _filter_project_test_sources(project, test_srcs)
    report["test_files"] = [str(path) for path in test_srcs]
    report["excluded_test_files"] = [str(path) for path in excluded_test_srcs]
    report["sample_files"] = [str(path) for path in sample_srcs]
    if err:
        report.update({"stage": "collect_tests", "error": err})
        return report
    if not test_srcs:
        report.update({"stage": "collect_tests", "error": "no selected C/C++ test source"})
        return report

    tmp_holder: Optional[tempfile.TemporaryDirectory[str]] = None
    try:
        if keep_build_dir:
            tmp_root = Path(tempfile.mkdtemp(prefix=f"paper_ohos_c_gtest_{project}_"))
        else:
            tmp_holder = tempfile.TemporaryDirectory(prefix=f"paper_ohos_c_gtest_{project}_")
            tmp_root = Path(tmp_holder.name)
        build_dir = tmp_root / "build"
        build_dir.mkdir(parents=True, exist_ok=True)
        report["build_dir"] = str(build_dir)
        report["keep_build_dir"] = bool(keep_build_dir)

        _write_host_headers(build_dir)
        includes = include_dirs_for(source_dir, ohos_root, build_dir)
        report["include_dirs"] = [str(path) for path in includes]

        report["stage"] = "test_compile"
        obj_dir = build_dir / "objs"
        obj_dir.mkdir(parents=True, exist_ok=True)
        objects: List[Path] = []
        compile_reports: List[Dict[str, Any]] = []
        for src in test_srcs:
            obj = obj_dir / _object_name_for(source_dir, src)
            res = _compile_test_source(src, obj, includes, build_dir, timeout)
            compile_reports.append(res)
            if not res["ok"]:
                report["test_compile"] = compile_reports
                report["error"] = "unit test compile failed"
                return report
            objects.append(obj)

        test_undefined = nm_undefined_symbols(objects)
        report["test_undefined_symbols"] = test_undefined

        audio_helper_objs, audio_helper_report = _build_audio_test_helpers(build_dir, ohos_root, includes, test_srcs, timeout)
        report["audio_helper_build"] = audio_helper_report
        audio_compile_reports = list(audio_helper_report.get("compile") or [])
        audio_entry_report = audio_helper_report.get("entry_build")
        if isinstance(audio_entry_report, dict):
            audio_compile_reports.append(audio_entry_report)
        if audio_compile_reports and not all(item.get("ok") for item in audio_compile_reports):
            report["stage"] = "audio_helper_build"
            report["error"] = "audio helper build failed"
            return report

        audio_helper_undefined = nm_undefined_symbols(audio_helper_objs)
        report["audio_helper_undefined_symbols"] = audio_helper_undefined

        pre_helper_objs, pre_helper_report = _build_pre_staticlib_helpers(project, build_dir, ohos_root, includes, timeout)
        report["pre_staticlib_helper_build"] = pre_helper_report
        if pre_helper_report and not all(item.get("ok") for item in pre_helper_report):
            report["stage"] = "pre_staticlib_helper_build"
            report["error"] = "pre-staticlib helper build failed"
            return report

        pre_helper_undefined = nm_undefined_symbols(pre_helper_objs)
        report["pre_staticlib_helper_undefined_symbols"] = pre_helper_undefined
        extra_export_names = _extra_export_names_for_project(project)
        report["extra_export_names"] = extra_export_names
        combined_undefined = sorted(
            set(test_undefined)
            | set(audio_helper_undefined)
            | set(pre_helper_undefined)
            | set(extra_export_names)
        )
        report["combined_undefined_symbols"] = combined_undefined

        work_crate = build_dir / "crate"
        _copy_crate(crate_dir, work_crate)
        export_meta = patch_crate_for_staticlib(work_crate, combined_undefined)
        report["export_meta"] = export_meta

        report["stage"] = "rust_staticlib"
        cargo_result, staticlib = build_staticlib(work_crate, timeout)
        report["staticlib_build"] = cargo_result
        if not cargo_result["ok"] or staticlib is None:
            report["error"] = "rust staticlib build failed"
            return report
        report["staticlib"] = str(staticlib)
        static_symbols = nm_defined_symbols(staticlib)
        report["staticlib_symbol_count"] = len(static_symbols)
        static_symbol_set = set(static_symbols)
        covered_symbols = sorted(sym for sym in combined_undefined if sym in static_symbol_set)
        report["test_symbols_from_rust_staticlib"] = covered_symbols
        report["rust_symbols_covered"] = bool(covered_symbols)

        report["stage"] = "host_helpers"
        securec_objs, securec_report = _build_securec(build_dir, ohos_root, includes, timeout, static_symbols)
        report["securec_build"] = securec_report
        if securec_report and not all(item["ok"] for item in securec_report):
            report["error"] = "securec build failed"
            return report

        helper_objs, helper_report = _build_optional_ohos_helpers(build_dir, ohos_root, includes, static_symbols, timeout)
        report["ohos_helper_build"] = helper_report
        if helper_report and not all(item["ok"] for item in helper_report):
            report["error"] = "OHOS helper build failed"
            return report

        supplemental_audio_objs, supplemental_audio_report, supplemental_audio_sources = _build_supplemental_audio_products(
            project, build_dir, ohos_root, includes, timeout
        )
        report["supplemental_audio_product_sources"] = [str(path) for path in supplemental_audio_sources]
        report["supplemental_audio_product_build"] = supplemental_audio_report
        if supplemental_audio_report and not all(item["ok"] for item in supplemental_audio_report):
            report["error"] = "supplemental audio product build failed"
            return report

        appverify_objs: List[Path] = []
        if project.startswith("appverify_lite__"):
            appverify_objs, appverify_report = _build_appverify_helpers(
                build_dir, ohos_root, includes, source_dir, static_symbols, timeout
            )
            report["appverify_helper_build"] = appverify_report
            flat_reports: List[Dict[str, Any]] = []
            for value in appverify_report.values():
                if isinstance(value, list):
                    flat_reports.extend(value)
                elif isinstance(value, dict) and "ok" in value:
                    flat_reports.append(value)
            if flat_reports and not all(item["ok"] for item in flat_reports):
                report["error"] = "appverify helper build failed"
                return report

        env_shim_objs: List[Path] = []
        env_shim_reports: List[Dict[str, Any]] = []
        for name, source in _environment_shim_sources(project):
            obj, res = _compile_generated_c(build_dir, name, source, includes, timeout)
            res["shim_name"] = name
            env_shim_reports.append(res)
            if name == "c2r_host_shim":
                report["host_shim_build"] = res
            elif name == "c2r_path_wrap":
                report["path_wrap_build"] = res
            if not res["ok"]:
                report["environment_shim_build"] = env_shim_reports
                report["error"] = "environment shim build failed"
                return report
            env_shim_objs.append(obj)
        report["environment_shim_build"] = env_shim_reports

        report["fixtures"] = _build_host_fixtures(project, build_dir, test_srcs, sample_srcs, includes, timeout)

        report["stage"] = "gtest_build"
        gtest_root = ohos_root / "third_party/googletest/googletest"
        for src in (gtest_root / "src/gtest-all.cc", gtest_root / "src/gtest_main.cc"):
            obj = obj_dir / f"{src.name}.o"
            res = _compile_c_source(src, obj, includes, build_dir, timeout)
            compile_reports.append(res)
            if not res["ok"]:
                report["test_compile"] = compile_reports
                report["error"] = "gtest compile failed"
                return report
            objects.append(obj)
        report["test_compile"] = compile_reports

        binary = build_dir / f"{project}_gtest"
        link_cmd = [
            "g++",
            "-std=c++17",
            "-O2",
            "-g",
            "-Wl,--export-dynamic",
            "-Wl,--gc-sections",
            *[str(obj) for obj in objects],
            *[str(obj) for obj in securec_objs],
            *[str(obj) for obj in helper_objs],
            *[str(obj) for obj in appverify_objs],
            *[str(obj) for obj in audio_helper_objs],
            *[str(obj) for obj in supplemental_audio_objs],
            *[str(obj) for obj in pre_helper_objs],
            str(staticlib),
            *[str(obj) for obj in env_shim_objs],
            "-Wl,--allow-multiple-definition",
            "-ldl",
            "-pthread",
            "-lm",
            *_extra_link_flags_for_project(project),
            *_link_wrap_flags_for_project(project),
            "-o",
            str(binary),
        ]
        link_res = run_command(link_cmd, build_dir, timeout)
        link_res["binary"] = str(binary)
        link_res["unresolved_symbols"] = extract_unresolved_symbols(link_res.get("stderr", ""))
        report["gtest_build"] = link_res
        if not link_res["ok"]:
            report["error"] = "unit test link failed"
            return report
        report["compiled"] = True

        report["stage"] = "gtest_run"
        run_env = dict(os.environ)
        run_env["RUST_BACKTRACE"] = "0"
        run_env["LD_LIBRARY_PATH"] = f"{build_dir}:{run_env.get('LD_LIBRARY_PATH', '')}"
        run_env["C2R_OHOS_LIB_FIXTURE_DIR"] = str(build_dir)
        host_exclusions = _host_infeasible_gtest_exclusions(project)
        filter_args = _gtest_filter_for_exclusions(host_exclusions)
        if host_exclusions:
            report["excluded_host_tests"] = host_exclusions
            report["excluded_host_tests_total"] = len(host_exclusions)
        list_res = run_command([str(binary), "--gtest_list_tests", *filter_args], build_dir, timeout, run_env)
        listed_cases = parse_gtest_cases(list_res.get("stdout", "")) if list_res["ok"] else []
        total_hint = len(listed_cases) if listed_cases else (parse_gtest_list(list_res.get("stdout", "")) if list_res["ok"] else None)
        run_res = run_command([str(binary), "--gtest_color=no", *filter_args], build_dir, timeout, run_env)
        combined = (run_res.get("stdout") or "") + "\n" + (run_res.get("stderr") or "")
        counts = parse_gtest_counts(combined, total_hint)
        failed_tests = parse_failed_tests(combined)
        report["gtest_list"] = list_res
        report["gtest_run"] = run_res
        report["listed_tests"] = listed_cases
        if _needs_isolated_gtest_rerun(run_res, counts, listed_cases):
            isolated = _run_isolated_gtest_cases(binary, build_dir, listed_cases, timeout, run_env)
            report["gtest_isolated_runs"] = isolated
            isolated_counts = summarize_isolated_gtest_runs(isolated)
            counts = {key: isolated_counts[key] for key in ("tests_total", "tests_passed", "tests_failed", "pass_rate")}
            failed_tests = list(isolated_counts["failed_tests"])
        report["executed"] = True
        report.update(counts)
        report["listed_tests_total"] = total_hint
        report["failed_tests"] = failed_tests
        if counts["tests_total"] in (None, 0):
            report["error"] = "gtest runner did not report any tests"
        elif not run_res["ok"]:
            report["error"] = "gtest run failed"
        else:
            report["stage"] = "done"
        return report
    finally:
        if tmp_holder is not None:
            tmp_holder.cleanup()


def summarize_results(results: List[Dict[str, Any]]) -> Dict[str, Any]:
    """汇总多个项目的测试结果。"""
    entered = [r for r in results if r.get("tests_total")]
    passed = sum(int(r.get("tests_passed") or 0) for r in entered)
    total = sum(int(r.get("tests_total") or 0) for r in entered)
    semantic_entered = [r for r in entered if r.get("test_symbols_from_rust_staticlib")]
    semantic_passed = sum(int(r.get("tests_passed") or 0) for r in semantic_entered)
    semantic_total = sum(int(r.get("tests_total") or 0) for r in semantic_entered)
    return {
        "projects_total": len(results),
        "projects_compiled": sum(1 for r in results if r.get("compiled")),
        "projects_executed": sum(1 for r in results if r.get("executed")),
        "projects_with_gtest_counts": len(entered),
        "projects_with_rust_symbol_coverage": sum(1 for r in results if r.get("test_symbols_from_rust_staticlib")),
        "semantic_projects_executed": len(semantic_entered),
        "tests_total": total,
        "tests_passed": passed,
        "tests_failed": sum(int(r.get("tests_failed") or 0) for r in entered),
        "pass_rate": (passed / total) if total else None,
        "semantic_tests_total": semantic_total,
        "semantic_tests_passed": semantic_passed,
        "semantic_tests_failed": sum(int(r.get("tests_failed") or 0) for r in semantic_entered),
        "semantic_pass_rate": (semantic_passed / semantic_total) if semantic_total else None,
    }
