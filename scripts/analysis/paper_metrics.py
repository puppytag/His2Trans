from __future__ import annotations

from typing import Any, Dict, Mapping, Optional


MetricRow = Dict[str, Optional[float]]
MethodMetricTable = Dict[str, MetricRow]
RQMetricTable = Dict[str, MethodMetricTable]


def pct(value: Optional[float]) -> Optional[float]:
    if value is None:
        return None
    return round(float(value) * 100.0, 2)


def normalize_metric_value(value: Any) -> Optional[float]:
    if value is None:
        return None
    if isinstance(value, str):
        text = value.strip()
        if not text or text == "--":
            return None
        return round(float(text), 2)
    return round(float(value), 2)


def mean_ignore_none(values: list[Optional[float]]) -> Optional[float]:
    nums = [float(v) for v in values if v is not None]
    if not nums:
        return None
    return round(sum(nums) / len(nums), 2)


def compile_ok(project: Mapping[str, Any]) -> Optional[bool]:
    if project.get("analysis_compilation_succeeded") is not None:
        return bool(project.get("analysis_compilation_succeeded"))
    cargo_check = project.get("cargo_check")
    if isinstance(cargo_check, dict) and cargo_check.get("passed") is not None:
        return bool(cargo_check.get("passed"))
    if project.get("cargo_check_passed") is not None:
        return bool(project.get("cargo_check_passed"))
    for key in ("cargo_clippy_result", "clippy_results", "clippy"):
        obj = project.get(key)
        if isinstance(obj, dict) and obj.get("compilation_succeeded") is not None:
            return bool(obj.get("compilation_succeeded"))
    return None


def compiled_only(value: Optional[float], compile_state: Optional[bool]) -> Optional[float]:
    if compile_state is not True:
        return None
    return value


def warning_total(obj: Any, compile_ok: Optional[bool]) -> Optional[float]:
    if not isinstance(obj, dict):
        return None
    if obj.get("executed") is False:
        return None
    if "compilation_succeeded" in obj and not bool(obj.get("compilation_succeeded")):
        return None
    if compile_ok is False:
        return None
    if obj.get("warning_count_total") is not None:
        return float(obj["warning_count_total"])
    if obj.get("warning_count") is not None:
        return float(obj.get("warning_count") or 0) + float(obj.get("rustc_warning_count") or 0)
    return None


def incremental_percent(project: Mapping[str, Any]) -> Optional[float]:
    inc = project.get("incremental_compilation")
    if not isinstance(inc, dict):
        return None
    rate = inc.get("compile_rate")
    if rate is None:
        rate = inc.get("pass_rate")
    if rate is None:
        total = inc.get("total_functions") or inc.get("tested_functions")
        ok = inc.get("compiled_functions")
        if ok is None:
            ok = inc.get("passed_total")
        if total:
            rate = float(ok or 0) / float(total)
    return pct(rate if rate is not None else None)


def unsafe_percent(project: Mapping[str, Any]) -> Optional[float]:
    ua = project.get("unsafe_analysis")
    if not isinstance(ua, dict) or ua.get("error"):
        return None
    ratio = ua.get("unsafe_total_ratio")
    if ratio is None:
        code = float(ua.get("code_lines") or 0)
        unsafe = float(ua.get("unsafe_total_lines") or 0)
        if code <= 0:
            return None
        ratio = unsafe / code
    return pct(ratio)


def generic_fc_percent(project: Mapping[str, Any], test_key: str) -> Optional[float]:
    tests = project.get(test_key)
    if not isinstance(tests, dict) or tests.get("executed") is False:
        return None
    return pct(tests.get("pass_rate"))


def ohos_fc_percent(project: Mapping[str, Any], test_key: str) -> Optional[float]:
    tests = project.get(test_key)
    if not isinstance(tests, dict):
        return None
    if bool(tests.get("skipped")) and bool(tests.get("needs_qemu")):
        return None
    passed = int(tests.get("tests_passed") or 0)
    denom = tests.get("expected_tests")
    if denom is None:
        denom = tests.get("expected_total_tests")
    if denom is None:
        denom = tests.get("total_tests")
    try:
        denom_int = int(denom or 0)
    except Exception:
        denom_int = 0
    if denom_int > 0:
        return round(passed / denom_int * 100.0, 2)
    if tests.get("error") or tests.get("compiled") is False:
        return 0.0
    return None


def project_avg_repair(project: Mapping[str, Any]) -> Optional[float]:
    repair_rounds = project.get("repair_rounds")
    if not isinstance(repair_rounds, dict):
        return None
    value = repair_rounds.get("avg_rounds_per_total_function")
    if value is None:
        return None
    return round(float(value), 2)


def weighted_incremental_from_projects(projects: Mapping[str, Mapping[str, Any]]) -> Optional[float]:
    total = 0
    ok = 0
    seen = False
    for project in projects.values():
        inc = project.get("incremental_compilation")
        if not isinstance(inc, dict):
            continue
        seen = True
        total_i = int(inc.get("total_functions") or inc.get("tested_functions") or 0)
        ok_i = inc.get("compiled_functions")
        if ok_i is None:
            ok_i = inc.get("passed_total")
        total += total_i
        ok += int(ok_i or 0)
    if not seen:
        return None
    return round((ok / total) * 100.0, 2) if total > 0 else 0.0


def weighted_fc_from_projects(
    projects: Mapping[str, Mapping[str, Any]],
    test_key: str,
    ohos: bool = False,
) -> Optional[float]:
    passed = 0
    total = 0
    seen = False
    for project in projects.values():
        tests = project.get(test_key)
        if not isinstance(tests, dict):
            continue
        seen = True
        passed += int(tests.get("tests_passed", tests.get("passed", 0)) or 0)
        if ohos:
            denom = tests.get("expected_tests")
            if denom is None:
                denom = tests.get("expected_total_tests")
            if denom is None:
                denom = tests.get("total_tests")
        else:
            denom = tests.get("total_tests")
            if denom is None:
                denom = tests.get("total")
            if denom is None:
                denom = tests.get("expected_total_tests")
        total += int(denom or 0)
    if not seen:
        return None
    return round((passed / total) * 100.0, 2) if total > 0 else 0.0


def rq1_summary_metrics(report: Mapping[str, Any]) -> MetricRow:
    projects = report["projects"]
    unsafe_values: list[Optional[float]] = []
    clippy_values: list[Optional[float]] = []
    for project in projects.values():
        state = compile_ok(project)
        unsafe_values.append(compiled_only(unsafe_percent(project), state))
        clippy_values.append(warning_total(project.get("clippy"), state))

    summary = report.get("summary", {})
    ohos_summary = summary.get("ohos_unit_test_summary")
    fc_value = weighted_fc_from_projects(projects, "ohos_unit_tests", ohos=True)
    if isinstance(ohos_summary, dict):
        passed = int(ohos_summary.get("tests_passed", 0) or 0)
        total = int(ohos_summary.get("total_tests", 0) or 0)
        fc_value = round((passed / total) * 100.0, 2) if total > 0 else 0.0

    return {
        "ICompRate": weighted_incremental_from_projects(projects),
        "FC": fc_value,
        "Unsafe": mean_ignore_none(unsafe_values),
        "Clippy": mean_ignore_none(clippy_values),
    }


def rq2_summary_metrics(report: Mapping[str, Any]) -> MetricRow:
    projects = report["projects"]
    unsafe_values: list[Optional[float]] = []
    clippy_values: list[Optional[float]] = []
    for project in projects.values():
        state = compile_ok(project)
        unsafe_values.append(compiled_only(unsafe_percent(project), state))
        clippy_values.append(warning_total(project.get("cargo_clippy_result"), state))

    return {
        "ICompRate": weighted_incremental_from_projects(projects),
        "FC": weighted_fc_from_projects(projects, "c2r_test_result", ohos=False),
        "Unsafe": mean_ignore_none(unsafe_values),
        "Clippy": mean_ignore_none(clippy_values),
    }


def rq3_or_rq4_summary_metrics(report: Mapping[str, Any]) -> MetricRow:
    projects = report["projects"]
    summary = report["summary"]
    repair_summary = summary.get("repair_rounds_summary", {})
    avg_repair = repair_summary.get("avg_rounds_per_total_function")
    return {
        "ICompRate": weighted_incremental_from_projects(projects),
        "FC": weighted_fc_from_projects(projects, "ohos_unit_tests", ohos=True),
        "AvgRepair": round(float(avg_repair), 2) if avg_repair is not None else None,
    }


def compare_metric_tables(
    actual: Mapping[str, MethodMetricTable],
    reference: Mapping[str, MethodMetricTable],
    *,
    tol: float = 1e-2,
) -> Dict[str, Dict[str, Dict[str, Dict[str, Optional[float]]]]]:
    diffs: Dict[str, Dict[str, Dict[str, Dict[str, Optional[float]]]]] = {}
    for rq_key in sorted(set(actual) | set(reference)):
        rq_actual = actual.get(rq_key, {})
        rq_reference = reference.get(rq_key, {})
        rq_diff: Dict[str, Dict[str, Dict[str, Optional[float]]]] = {}
        for method in sorted(set(rq_actual) | set(rq_reference)):
            row_actual = rq_actual.get(method, {})
            row_reference = rq_reference.get(method, {})
            method_diff: Dict[str, Dict[str, Optional[float]]] = {}
            for metric in sorted(set(row_actual) | set(row_reference)):
                actual_value = normalize_metric_value(row_actual.get(metric))
                reference_value = normalize_metric_value(row_reference.get(metric))
                if actual_value is None and reference_value is None:
                    continue
                if actual_value is None or reference_value is None:
                    delta = None if actual_value is None or reference_value is None else round(actual_value - reference_value, 2)
                    method_diff[metric] = {
                        "actual": actual_value,
                        "reference": reference_value,
                        "delta": delta,
                    }
                    continue
                delta = round(actual_value - reference_value, 2)
                if abs(delta) > tol:
                    method_diff[metric] = {
                        "actual": actual_value,
                        "reference": reference_value,
                        "delta": delta,
                    }
            if method_diff:
                rq_diff[method] = method_diff
        if rq_diff:
            diffs[rq_key] = rq_diff
    return diffs
