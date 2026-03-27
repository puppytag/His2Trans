#!/usr/bin/env python3
"""
OHOS(test5) analysis for **C2Rust baseline** on Huawei self-contained modules.

We reuse the c2rust-converted crates under:
  ComparisonMethod/c2saferrust/ohos_converted_test5/<project>/

Metrics (truthful, no cheating):
- cargo check pass/fail (without writing target/ into the crate)
- Clippy vs Rustc warnings (JSON diagnostics)
- unsafe rate (keyword/context line union, excluding comments/strings)
- incremental compilation (best-effort per-function isolated compile in a temp copy)
- OHOS gtest unit tests from the source tree (best-effort), linked against the translated staticlib

Note:
- This script does NOT modify translation outputs or OHOS source tests on disk.
- For OHOS gtest building/running we reuse the shared host harness in
  `scripts/analysis/analyze_c2r_compilation_rate_ohos10.py` to handle projects
  like appverify_lite (mbedtls linkage, small host-side patches, etc.).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple

# Reuse baseline helpers (cargo check/clippy/unsafe/incremental/staticlib build).
from analyze_c2rust_compilation_ohos5 import (  # type: ignore
    analyze_unsafe_code,
    build_staticlib_from_crate,
    load_huawei_projects_map,
    run_cargo_check,
    run_cargo_clippy,
    verify_incremental_compilation,
)


DISPLAY_PROJECT_ORDER: List[str] = [
    "appverify_lite__e5ebe91a98b9",
    "host__25c1898e1626",
    "osal__0bc4f21396ad",
    "shared__12e38ea922f7",
    "shared__541f4e547bdb",
]
_ORDER_INDEX = {n: i for i, n in enumerate(DISPLAY_PROJECT_ORDER)}

DEFAULT_BASE_DIR = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/c2saferrust/ohos_converted_test5")
DEFAULT_HUAWEI_PROJECTS_TSV = Path("/data/home/wangshb/c2-rust_framework/ComparisonMethod/unified/huawei_projects.tsv")
DEFAULT_OHOS_ROOT = Path("/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony")


def iter_projects_in_display_order(projects: Dict[str, Any]) -> List[Tuple[str, Any]]:
    return sorted(projects.items(), key=lambda kv: (_ORDER_INDEX.get(kv[0], 1_000_000), kv[0]))


# Reuse our framework's host-runnable OHOS gtest harness (no translation-output modifications).
_REPO_ROOT = Path(__file__).resolve().parents[3]
if str(_REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(_REPO_ROOT))
try:
    from scripts.analysis import analyze_c2r_compilation_rate_ohos10 as ohos_harness  # type: ignore
except Exception:
    ohos_harness = None
try:
    from scripts.analysis.ohos_test5_unified_report import (  # type: ignore
        print_ohos_test5_key_metrics_table,
    )
except Exception:
    print_ohos_test5_key_metrics_table = None
try:
    from scripts.analysis.ohos_test5_incremental import (  # type: ignore
        load_standard_function_names as load_standard_function_names_ohos5,
        verify_incremental_compilation_standard as verify_incremental_compilation_standard_ohos5,
        get_expected_test_count as get_expected_test_count_ohos5,
        STANDARD_TEST_COUNTS,
        STANDARD_TEST_TOTAL,
    )
except Exception:
    load_standard_function_names_ohos5 = None
    verify_incremental_compilation_standard_ohos5 = None
    get_expected_test_count_ohos5 = None
    STANDARD_TEST_COUNTS = {}
    STANDARD_TEST_TOTAL = 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Analyze C2Rust baseline on OHOS(test5) projects.")
    parser.add_argument("--base-dir", type=Path, default=DEFAULT_BASE_DIR)
    parser.add_argument("--output", "-o", type=Path, default=None)
    parser.add_argument("--run-clippy", action="store_true")
    parser.add_argument("--analyze-unsafe", action="store_true")
    parser.add_argument("--verify-incremental", action="store_true")
    parser.add_argument(
        "--verify-incremental-timeout",
        type=int,
        default=60,
        help="--verify-incremental: per-function cargo check timeout (seconds)",
    )
    parser.add_argument("--run-ohos-tests", action="store_true")
    parser.add_argument("--ohos-test-timeout", type=int, default=600)
    parser.add_argument(
        "--run-derived-tests",
        action="store_true",
        help="Extract a conservative pure-logic subset from OHOS gtest and run as Rust unit tests (temp copy; includes signature matching).",
    )
    parser.add_argument("--derived-test-timeout", type=int, default=600)
    parser.add_argument(
        "--standard-run-dir",
        type=Path,
        default=None,
        help=(
            "Canonical standard-function source (our framework run dir). "
            "If provided, use <dir>/intermediate/<proj>/workspace/extracted/<proj>/functions_manifest.json "
            "as the denominator for ICompRate."
        ),
    )
    parser.add_argument("--huawei-projects-tsv", type=Path, default=DEFAULT_HUAWEI_PROJECTS_TSV)
    parser.add_argument("--ohos-root", type=Path, default=DEFAULT_OHOS_ROOT)
    parser.add_argument("--all", action="store_true")
    args = parser.parse_args()

    run_clippy = args.run_clippy or args.all
    analyze_unsafe = args.analyze_unsafe or args.all
    verify_incremental = args.verify_incremental or args.all
    run_ohos_tests = args.run_ohos_tests or args.all
    run_derived_tests = args.run_derived_tests

    base_dir = args.base_dir
    if not base_dir.is_dir():
        print(f"Error: base-dir not found: {base_dir}", file=sys.stderr)
        return 2

    out_path = args.output or (base_dir / "compilation_analysis_ohos_test5.json")
    huawei_map = load_huawei_projects_map(args.huawei_projects_tsv) if (run_ohos_tests or run_derived_tests) else {}
    ohos_root = args.ohos_root

    projects: Dict[str, Any] = {}

    total_projects = 0
    cargo_ok = 0
    clippy_warn = rustc_warn = warn_total = clippy_err = 0
    code_lines = unsafe_lines = unsafe_items = 0
    inc_total = inc_ok = 0
    ohos_attempted = ohos_compiled = 0
    ohos_tests_total = ohos_tests_passed = ohos_tests_failed = 0
    derived_projects = 0
    derived_tests_generated = 0
    derived_tests_passed = 0
    derived_tests_failed = 0

    for name in DISPLAY_PROJECT_ORDER:
        total_projects += 1
        crate_dir = base_dir / name
        pr: Dict[str, Any] = {"project_name": name, "crate_dir": str(crate_dir)}

        # Canonical denominator: standard function set extracted from the C sources (our framework).
        std_names: List[str] = []
        if verify_incremental and load_standard_function_names_ohos5 is not None:
            try:
                std_names = load_standard_function_names_ohos5(name, args.standard_run_dir)
            except Exception:
                std_names = []
        if not (crate_dir / "Cargo.toml").is_file():
            pr["error"] = "Cargo.toml not found"
            if verify_incremental:
                pr["incremental_compilation"] = {
                    "total_functions": len(std_names),
                    "compiled_functions": 0,
                    "compile_rate": 0.0,
                    "error": "Cargo.toml not found",
                }
                inc_total += len(std_names)
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
            if verify_incremental_compilation_standard_ohos5 is None:
                inc = {
                    "total_functions": len(std_names),
                    "compiled_functions": 0,
                    "compile_rate": 0.0,
                    "error": "incremental verifier import failed (scripts.analysis.ohos_test5_incremental)",
                }
            else:
                try:
                    inc = verify_incremental_compilation_standard_ohos5(
                        crate_dir=crate_dir,
                        project_name=name,
                        standard_func_names=std_names,
                        timeout=int(args.verify_incremental_timeout),
                        # For the C2Rust baseline, we count baseline bodies as the "counted" source.
                        count_sources={"baseline"},
                        llm_success_names=None,
                        default_source="baseline",
                    )
                except Exception as e:
                    inc = {
                        "total_functions": len(std_names),
                        "compiled_functions": 0,
                        "compile_rate": 0.0,
                        "error": f"verify_incremental_compilation_standard failed: {e}",
                    }
            pr["incremental_compilation"] = inc
            # Truthful aggregation: error projects still contribute denominator (=> 0% for that project).
            inc_total += int(inc.get("total_functions", 0) or 0)
            inc_ok += int(inc.get("compiled_functions", 0) or 0)

        if run_ohos_tests:
            utr: Dict[str, Any] = {
                "executed": False,
                "compiled": False,
                "tests_passed": 0,
                "tests_failed": 0,
                "total_tests": 0,
                "pass_rate": 0.0,
                "error": None,
                "staticlib_build": {},
                "gtest": {},
            }

            src_proj = huawei_map.get(name)
            if not src_proj:
                utr["error"] = f"source project not found in mapping: {name}"
            elif ohos_harness is None:
                utr["error"] = "ohos test harness import failed (scripts.analysis.analyze_c2r_compilation_rate_ohos10)"
            else:
                sb = build_staticlib_from_crate(crate_dir, timeout=args.ohos_test_timeout)
                utr["staticlib_build"] = sb
                if not sb.get("ok") or not sb.get("staticlib"):
                    utr["error"] = sb.get("error") or "staticlib build failed"
                else:
                    # Align with our framework's OHOS(test5) harness: split host-runnable vs QEMU-required tests
                    # and only run the host subset to avoid link failures on runtime-dependent files (e.g., sample_driver.c).
                    tests_all, samples_all, err = ohos_harness._collect_ohos_test_sources(src_proj)  # type: ignore[attr-defined]
                    if err:
                        utr["error"] = err
                    else:
                        skip_re = re.compile(
                            r"("
                            r"hdf_uhdf_test\.h|\bHdfTestOpenService\b|\bHdfTestSendMsgToService\b|"
                            r"\bMessageParcel\b|\bIRemoteObject\b|\bIProxyBroker\b|\bsptr\s*<|"
                            r"\bSbufToParcel\b|\bParcelToSbuf\b|\bHdfRemoteAdapter\b|"
                            r"\bHdfRegisteDevice\b|\bHdfUnregisteDevice\b|\bHdfDeviceSendEvent\b|\bHdfDeviceSendEventToClient\b|"
                            r"\"/sys/|/sys/"
                            r")"
                        )
                        qemu_tests: List[Path] = []
                        host_tests: List[Path] = []
                        for ts in tests_all:
                            try:
                                txt = ts.read_text(encoding="utf-8", errors="ignore")
                            except Exception:
                                txt = ""
                            if skip_re.search(txt):
                                qemu_tests.append(ts)
                            else:
                                host_tests.append(ts)

                        pr["ohos_tests_split"] = {
                            "host_test_files": len(host_tests),
                            "qemu_test_files": len(qemu_tests),
                            "qemu_test_files_list": [str(p.relative_to(src_proj)) for p in qemu_tests],
                        }

                        if not host_tests:
                            utr["error"] = (
                                "该项目的 gtest 依赖 OHOS 运行时/服务/IPC/内核节点（建议在 OHOS QEMU/真机环境跑），"
                                "已跳过主机侧执行。"
                            )
                            utr["skipped"] = True
                            utr["needs_qemu"] = True
                            utr["qemu_test_files"] = [str(p.relative_to(src_proj)) for p in qemu_tests]
                        else:
                            gt = ohos_harness.run_ohos_unit_tests(  # type: ignore[attr-defined]
                                project_name=name,
                                source_project_dir=src_proj,
                                translated_staticlib=Path(sb["staticlib"]),
                                ohos_root=ohos_root,
                                test_srcs=host_tests,
                                sample_srcs=samples_all,
                                timeout=args.ohos_test_timeout,
                            )
                            gt["skipped_qemu_test_files"] = [str(p.relative_to(src_proj)) for p in qemu_tests]
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
                # Use unified denominator: for each project, expected_count comes from STANDARD_TEST_COUNTS.
                # If tests didn't run, passed = 0, failed = expected_count.
                expected_count = 0
                if get_expected_test_count_ohos5 is not None:
                    expected_count = get_expected_test_count_ohos5(name)
                else:
                    expected_count = STANDARD_TEST_COUNTS.get(name, 0)
                actual_passed = int(utr.get("tests_passed", 0) or 0)
                # Unified: passed is the actual passed, failed = expected - passed
                ohos_tests_passed += actual_passed
                ohos_tests_failed += (expected_count - actual_passed)
                # Store per-project expected count in results for transparency
                utr["expected_tests"] = expected_count
                utr["unified_tests_failed"] = expected_count - actual_passed

            pr["ohos_unit_test_results"] = utr

        if run_derived_tests:
            dr: Dict[str, Any] = {
                "executed": False,
                "generated_tests": 0,
                "passed": 0,
                "failed": 0,
                "error": None,
            }
            src_proj = huawei_map.get(name)
            if not src_proj:
                dr["error"] = f"source project not found in mapping: {name}"
            elif ohos_harness is None:
                dr["error"] = "ohos harness import failed (scripts.analysis.analyze_c2r_compilation_rate_ohos10)"
            else:
                test_srcs, _sample_srcs, err = ohos_harness._collect_ohos_test_sources(src_proj)  # type: ignore[attr-defined]
                if err:
                    dr["error"] = err
                else:
                    cases = ohos_harness._extract_pure_logic_cases_from_gtest(test_srcs)  # type: ignore[attr-defined]
                    dr = ohos_harness.run_derived_rust_tests_from_gtest(  # type: ignore[attr-defined]
                        project_name=name,
                        crate_dir=crate_dir,
                        cases=cases,
                        ohos_root=ohos_root,
                        timeout=args.derived_test_timeout,
                    )
                    if dr.get("executed"):
                        derived_projects += 1
                    derived_tests_generated += int(dr.get("generated_tests", 0) or 0)
                    derived_tests_passed += int(dr.get("passed", 0) or 0)
                    derived_tests_failed += int(dr.get("failed", 0) or 0)
            pr["derived_rust_tests"] = dr

        projects[name] = pr

    summary: Dict[str, Any] = {
        "total_projects": total_projects,
        "projects_compiled": cargo_ok,
        "project_compile_rate": (cargo_ok / total_projects) if total_projects else 0.0,
    }
    if run_clippy:
        summary["clippy_summary"] = {
            "total_clippy_warnings": clippy_warn,
            "total_rustc_warnings": rustc_warn,
            "total_warnings_including_rustc": warn_total,
            "total_errors": clippy_err,
        }
    if analyze_unsafe:
        summary["unsafe_summary"] = {
            "total_code_lines": code_lines,
            "total_unsafe_total_lines": unsafe_lines,
            "unsafe_total_ratio": (unsafe_lines / code_lines) if code_lines else 0.0,
            "unsafe_keyword_occurrences": unsafe_items,
        }
    if verify_incremental:
        summary["incremental_compilation_summary"] = {
            "total_functions": inc_total,
            "compiled_functions": inc_ok,
            "compile_rate": (inc_ok / inc_total) if inc_total else 0.0,
        }
    if run_ohos_tests:
        # Use STANDARD_TEST_TOTAL (40) as the unified denominator for fair cross-method comparison.
        unified_total = STANDARD_TEST_TOTAL if STANDARD_TEST_TOTAL > 0 else (ohos_tests_passed + ohos_tests_failed)
        summary["ohos_unit_test_summary"] = {
            "projects_executed": ohos_attempted,
            "projects_compiled": ohos_compiled,
            "compile_success_rate": (ohos_compiled / ohos_attempted) if ohos_attempted else 0.0,
            "total_tests": unified_total,
            "tests_passed": ohos_tests_passed,
            "tests_failed": ohos_tests_failed,
            "overall_test_pass_rate": (ohos_tests_passed / unified_total) if unified_total else 0.0,
        }
    if run_derived_tests:
        summary["derived_pure_logic_summary"] = {
            "projects_executed": derived_projects,
            "generated_tests": derived_tests_generated,
            "tests_passed": derived_tests_passed,
            "tests_failed": derived_tests_failed,
        }

    result: Dict[str, Any] = {"base_dir": str(base_dir), "projects": projects, "summary": summary}
    out_path.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"Analysis saved to: {out_path}")

    # Unified key metrics table across 5 analyzers (our framework + 4 baselines).
    if print_ohos_test5_key_metrics_table is not None:
        print_ohos_test5_key_metrics_table(
            method="c2rust",
            project_order=DISPLAY_PROJECT_ORDER,
            projects=projects,
            summary=summary,
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
