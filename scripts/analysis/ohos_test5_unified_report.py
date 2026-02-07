#!/usr/bin/env python3
"""
Shared console report helpers for OHOS(test5) evaluation scripts.

Goal:
  Make the 5 analyzers (our framework + 4 baselines) print a *consistent*
  "key metrics" table for paper-friendly comparison.

This module is intentionally small and dependency-free.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional, Tuple


def _pct(x: Optional[float]) -> str:
    try:
        if x is None:
            return "-"
        return f"{(float(x) * 100.0):.1f}%"
    except Exception:
        return "-"


def _i(x: Any) -> Optional[int]:
    try:
        if x is None:
            return None
        return int(x)
    except Exception:
        return None


def _get_subdict(pr: Dict[str, Any], keys: List[str]) -> Dict[str, Any]:
    for k in keys:
        v = pr.get(k)
        if isinstance(v, dict):
            return v
    return {}


def _extract_inc(pr: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    inc = pr.get("incremental_compilation")
    if not isinstance(inc, dict):
        return None, None, None

    # Our framework: passed_total / total_functions
    total = _i(inc.get("total_functions"))
    ok = _i(inc.get("passed_total"))
    rate = inc.get("pass_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    # Most baselines: compiled_functions / total_functions
    ok = _i(inc.get("compiled_functions"))
    rate = inc.get("compile_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    # C2SaferRust: compiled_chunks / total_chunks
    total = _i(inc.get("total_chunks"))
    ok = _i(inc.get("compiled_chunks"))
    rate = inc.get("compile_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    return None, None, None


def _extract_tests(pr: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    ut = _get_subdict(pr, ["ohos_unit_tests", "ohos_unit_test_results"])
    if not ut:
        return None, None, None
    executed = bool(ut.get("executed"))
    passed = _i(ut.get("tests_passed"))
    total = _i(ut.get("total_tests"))
    rate = ut.get("pass_rate")
    # If not executed:
    # - keep QEMU/device-only skips as missing ('-')
    # - but treat host-attempted compile/build failures as 0% (so they don't look like "not measured")
    if not executed and (total is None or total == 0):
        gtest = ut.get("gtest") if isinstance(ut.get("gtest"), dict) else {}
        skipped = bool(ut.get("skipped") or gtest.get("skipped"))
        needs_qemu = bool(ut.get("needs_qemu") or gtest.get("needs_qemu"))
        if skipped and needs_qemu:
            return None, None, None
        # If we have an explicit failure signal, show 0.0% for this project.
        if ut.get("error") or gtest.get("error") or (ut.get("compiled") is False):
            return 0, 0, 0.0
        return None, None, None
    if passed is None or total is None:
        return None, None, None
    if rate is None and total > 0:
        rate = passed / total
    return passed, total, float(rate) if rate is not None else None


def _extract_clippy(pr: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[int]]:
    cl = _get_subdict(pr, ["clippy", "clippy_results"])
    if not cl:
        return None, None, None
    executed = bool(cl.get("executed"))
    if not executed:
        return None, None, None
    total = _i(cl.get("warning_count_total"))
    c = _i(cl.get("warning_count"))
    r = _i(cl.get("rustc_warning_count"))
    return total, c, r


def _extract_unsafe(pr: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    ua = pr.get("unsafe_analysis")
    if not isinstance(ua, dict) or ua.get("error"):
        return None, None, None
    code = _i(ua.get("code_lines"))
    unsafe_lines = _i(ua.get("unsafe_total_lines"))
    ratio = ua.get("unsafe_total_ratio")
    if code is None or unsafe_lines is None:
        return None, None, None
    if ratio is None and code > 0:
        ratio = unsafe_lines / code
    return unsafe_lines, code, float(ratio) if ratio is not None else None


def _extract_inc_total(summary: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    s = summary.get("incremental_compilation_summary")
    if not isinstance(s, dict):
        return None, None, None
    total = _i(s.get("total_functions"))
    ok = _i(s.get("passed_total"))
    rate = s.get("pass_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    ok = _i(s.get("compiled_functions"))
    rate = s.get("compile_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    total = _i(s.get("total_chunks"))
    ok = _i(s.get("compiled_chunks"))
    rate = s.get("compile_rate")
    if total is not None and ok is not None:
        if rate is None and total > 0:
            rate = ok / total
        return ok, total, float(rate) if rate is not None else None

    return None, None, None


def _extract_tests_total(summary: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    s = summary.get("ohos_unit_test_summary")
    if not isinstance(s, dict):
        return None, None, None
    passed = _i(s.get("tests_passed"))
    total = _i(s.get("total_tests"))
    rate = s.get("overall_test_pass_rate")
    if passed is None or total is None:
        return None, None, None
    # If no test cases ran at all, keep the table output as '-' (unknown), not 0%.
    if total == 0:
        return None, None, None
    if rate is None and total > 0:
        rate = passed / total
    return passed, total, float(rate) if rate is not None else None


def _extract_clippy_total(summary: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[int]]:
    s = summary.get("clippy_summary")
    if not isinstance(s, dict):
        return None, None, None
    total = _i(s.get("total_warnings_including_rustc"))
    c = _i(s.get("total_clippy_warnings"))
    r = _i(s.get("total_rustc_warnings"))
    return total, c, r


def _extract_unsafe_total(summary: Dict[str, Any]) -> Tuple[Optional[int], Optional[int], Optional[float]]:
    s = summary.get("unsafe_summary")
    if not isinstance(s, dict):
        return None, None, None
    code = _i(s.get("total_code_lines"))
    unsafe_lines = _i(s.get("total_unsafe_total_lines"))
    ratio = s.get("unsafe_total_ratio")
    if code is None or unsafe_lines is None:
        return None, None, None
    if ratio is None and code > 0:
        ratio = unsafe_lines / code
    return unsafe_lines, code, float(ratio) if ratio is not None else None


def _fmt_frac(passed: Optional[int], total: Optional[int], rate: Optional[float]) -> str:
    if rate is None:
        return "-"
    # For some failure modes we intentionally report 0% even when the test runner
    # couldn't produce a (passed/total) fraction (e.g., build failed before gtest ran).
    return _pct(rate)


def _fmt_clippy(total: Optional[int], c: Optional[int], r: Optional[int]) -> str:
    if total is None:
        return "-"
    # Only keep the total warning count.
    return str(total)


def _fmt_unsafe(unsafe_lines: Optional[int], code_lines: Optional[int], ratio: Optional[float]) -> str:
    if unsafe_lines is None or code_lines is None or code_lines <= 0 or ratio is None:
        return "-"
    # Only keep the headline percentage for paper tables.
    return _pct(ratio)


def print_ohos_test5_key_metrics_table(
    *,
    method: str,
    project_order: List[str],
    projects: Dict[str, Any],
    summary: Dict[str, Any],
) -> None:
    """
    Print a single unified table containing the 4 required metrics:
      - incremental compilation rate
      - external tests (gtest) pass rate
      - clippy warnings count
      - unsafe ratio
    """
    width = 100
    print("\n" + "=" * width)
    print(f"OHOS(test5) 关键指标汇总（统一格式） - {method}")
    print("=" * width)
    print(
        f"{'项目':<28} {'增量编译率':>18} {'外部测试通过率':>18} {'Clippy警告(总)':>18} {'Unsafe率':>14}"
    )
    print("-" * width)

    for name in project_order:
        pr = projects.get(name) or {}
        if not isinstance(pr, dict):
            pr = {}

        inc_ok, inc_total, inc_rate = _extract_inc(pr)
        t_pass, t_total, t_rate = _extract_tests(pr)
        c_total, c_c, c_r = _extract_clippy(pr)
        u_unsafe, u_code, u_rate = _extract_unsafe(pr)

        inc_s = _fmt_frac(inc_ok, inc_total, inc_rate)
        t_s = _fmt_frac(t_pass, t_total, t_rate)
        c_s = _fmt_clippy(c_total, c_c, c_r)
        u_s = _fmt_unsafe(u_unsafe, u_code, u_rate)

        print(f"{name:<28} {inc_s:>18} {t_s:>18} {c_s:>18} {u_s:>14}")

    print("-" * width)

    inc_ok, inc_total, inc_rate = _extract_inc_total(summary)
    t_pass, t_total, t_rate = _extract_tests_total(summary)
    c_total, c_c, c_r = _extract_clippy_total(summary)
    u_unsafe, u_code, u_rate = _extract_unsafe_total(summary)

    inc_s = _fmt_frac(inc_ok, inc_total, inc_rate)
    t_s = _fmt_frac(t_pass, t_total, t_rate)
    c_s = _fmt_clippy(c_total, c_c, c_r)
    u_s = _fmt_unsafe(u_unsafe, u_code, u_rate)
    print(f"{'总计':<28} {inc_s:>18} {t_s:>18} {c_s:>18} {u_s:>14}")

    print()
    print("注: '-' 表示该指标未执行/不可用；请加 --all 或相应开关启用计算。")
