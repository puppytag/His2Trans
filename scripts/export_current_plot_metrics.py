#!/usr/bin/env python3
"""Export the paper-aligned metrics shipped with this repository."""

from __future__ import annotations

import csv
import json
from pathlib import Path
from typing import Any, Dict, Iterable, List


REPO_ROOT = Path(__file__).resolve().parent.parent
EXPORT_DIR = REPO_ROOT / "data" / "paper_metric_exports"
REFERENCE_DIR = EXPORT_DIR / "reference_tables"
STRUCTURED_DIR = EXPORT_DIR / "generated_structured_json"
LOG_DIR = EXPORT_DIR / "logs"
SUMMARY_JSON = EXPORT_DIR / "current_plot_metrics_alignment.json"
SUMMARY_MD = EXPORT_DIR / "current_plot_metrics_alignment.md"
SOURCE_SUMMARY_DIR = EXPORT_DIR / "source_summaries"

DEFAULT_REFERENCE_OUT_DIR = REFERENCE_DIR
DEFAULT_STRUCTURED_DIR = STRUCTURED_DIR
DEFAULT_LOG_DIR = LOG_DIR

METHOD_SUMMARY_JSON = SOURCE_SUMMARY_DIR / "method_comparison_metrics.json"
ABLATION_SUMMARY_JSON = SOURCE_SUMMARY_DIR / "ablation_ohos10_full_metrics_summary.json"
OHOS_ARCHIVE_JSON = (
    REPO_ROOT
    / "data"
    / "paper_artifacts"
    / "deepseek_v4_pro_ohos10_0613_harness_fixed_v2"
    / "results"
    / "ohos_metrics_deepseek-v4-pro-ohos10-full-0613-1_harness_fixed_v2.json"
)
OSS8_ARCHIVE_JSON = (
    REPO_ROOT
    / "data"
    / "paper_artifacts"
    / "deepseek_v4_pro_oss8_0613_rq2_100pct_minimal"
    / "results"
    / "summary.json"
)

METHOD_ORDER = ["Ours", "Claude Code", "C2Rust", "C2SaferRust", "EvoC2Rust", "Tymcrat"]
CSV_NAME_BY_RQ = {
    "rq1": "rq1_method_metric_avg.csv",
    "rq2": "rq2_method_metric_avg.csv",
    "rq3": "rq3_method_metric_avg.csv",
    "rq4": "rq4_method_metric_avg.csv",
}
STRUCTURED_NAME_BY_RQ = {
    "rq1": "rq1_ohos10_method_comparison.json",
    "rq2": "rq2_oss8_method_comparison.json",
    "rq3": "rq3_ohos10_ablation.json",
    "rq4": "rq4_case_evidence.json",
}
RQ_SECTION_TITLES = {
    "rq1": "RQ1: OHOS test5",
    "rq2": "RQ2: test_module",
    "rq3": "RQ3: Ablation Study",
    "rq4": "RQ4: Knowledge Base Ablation",
}


def load_json(path: Path) -> Dict[str, Any]:
    """读取 JSON 文件，缺失时直接报错。"""
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Dict[str, Any]) -> None:
    """写入格式化 JSON。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def write_csv(path: Path, fieldnames: List[str], rows: Iterable[Dict[str, str]]) -> None:
    """写入 CSV 表。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        for row in rows:
            writer.writerow(row)


def repo_relative_path(path: Path | str) -> str:
    """转换为仓库相对路径，便于结果可移植。"""
    candidate = Path(path)
    try:
        rel = candidate.resolve().relative_to(REPO_ROOT.resolve())
    except ValueError:
        return str(path)
    return "." if not rel.parts else rel.as_posix()


def maybe_number(value: Any) -> float | None:
    """解析可比较的数值；文本证据列不参与 metric diff。"""
    if value is None:
        return None
    text = str(value).strip()
    if not text or text == "--":
        return None
    try:
        return round(float(text), 2)
    except ValueError:
        return None


def pct(value: float | int | None) -> str:
    """把 0-1 比例格式化为百分数数值字符串。"""
    if value is None:
        return "--"
    return f"{float(value) * 100:.2f}"


def display_pct(value: float | int | None) -> str:
    """把 0-1 比例格式化为带百分号的展示字符串。"""
    if value is None:
        return "--"
    return f"{float(value) * 100:.2f}%"


def method_rows(method_summary: Dict[str, Any], suite: str) -> List[Dict[str, str]]:
    """提取 RQ1/RQ2 方法对比主表。"""
    entries = {item["method"]: item for item in method_summary["summaries"][suite]}
    rows: List[Dict[str, str]] = []
    for method in METHOD_ORDER:
        item = entries[method]
        metrics = item["metrics"]
        rows.append(
            {
                "Method": method,
                "ICompRate": pct(metrics["incremental_compile_rate"]),
                "TestPass": pct(metrics["test_pass_rate"]),
                "Unsafe": pct(metrics["raw_unsafe_rate"]),
                "RequiredUnsafe": pct(metrics["required_unsafe_rate"]),
            }
        )
    return rows


def ablation_rows(ablation_summary: Dict[str, Any]) -> List[Dict[str, str]]:
    """提取 RQ3 消融主表。"""
    rows: List[Dict[str, str]] = []
    for group in ablation_summary["groups"]:
        summary = group["summary"]
        rows.append(
            {
                "Group": group["group"],
                "Setting": group["setting"],
                "ICompRate": pct(summary["incremental_compile"]["macro_project_average"]),
                "TestPass": pct(summary["test_pass"]["macro_project_average"]),
                "Unsafe": pct(summary["raw_unsafe"]["macro_project_average"]),
                "RequiredUnsafe": pct(summary["required_unsafe"]["macro_project_average"]),
            }
        )
    return rows


def rq4_rows() -> List[Dict[str, str]]:
    """记录论文 RQ4 的定性证据索引。"""
    return [
        {
            "Case": "manager",
            "Boundary": "HDF service event notification",
            "Evidence": "His2Trans preserves the system-visible event path and cleanup logic.",
            "PaperSection": "RQ4 Historical Knowledge Reuse Analysis",
        },
        {
            "Case": "shared_12",
            "Boundary": "HDF SBuf wire-format serialization",
            "Evidence": "His2Trans preserves external HdfSbufRead*/Write* calls and passes 15/15 tests.",
            "PaperSection": "RQ4 Historical Knowledge Reuse Analysis",
        },
    ]


def assert_key_numbers(
    ohos_rows: List[Dict[str, str]],
    oss8_rows: List[Dict[str, str]],
    rq3_rows: List[Dict[str, str]],
) -> List[str]:
    """断言 README/论文中的关键数字没有漂移。"""
    checks = []
    ohos_ours = next(row for row in ohos_rows if row["Method"] == "Ours")
    oss8_ours = next(row for row in oss8_rows if row["Method"] == "Ours")
    expected = {
        "RQ1 Ours ICompRate": (ohos_ours["ICompRate"], "100.00"),
        "RQ1 Ours TestPass": (ohos_ours["TestPass"], "94.92"),
        "RQ1 Ours Unsafe": (ohos_ours["Unsafe"], "16.35"),
        "RQ2 Ours ICompRate": (oss8_ours["ICompRate"], "100.00"),
        "RQ2 Ours TestPass": (oss8_ours["TestPass"], "100.00"),
        "RQ2 Ours Unsafe": (oss8_ours["Unsafe"], "8.59"),
    }
    for index, expected_values in enumerate(
        [
            ("95.82", "39.29", "15.95"),
            ("100.00", "94.92", "22.52"),
            ("100.00", "94.92", "16.35"),
        ]
    ):
        row = rq3_rows[index]
        expected[f"RQ3 {row['Group']} ICompRate"] = (row["ICompRate"], expected_values[0])
        expected[f"RQ3 {row['Group']} TestPass"] = (row["TestPass"], expected_values[1])
        expected[f"RQ3 {row['Group']} Unsafe"] = (row["Unsafe"], expected_values[2])

    for name, (actual, want) in expected.items():
        if actual != want:
            raise AssertionError(f"{name}: expected {want}, got {actual}")
        checks.append(f"{name}: {actual}")
    return checks


def markdown_table(title: str, rows: List[Dict[str, str]], columns: List[str]) -> str:
    """生成简单 Markdown 表格。"""
    lines = [f"## {title}", ""]
    lines.append("| " + " | ".join(columns) + " |")
    lines.append("| " + " | ".join(["---"] + ["---:"] * (len(columns) - 1)) + " |")
    for row in rows:
        lines.append("| " + " | ".join(row[col] for col in columns) + " |")
    lines.append("")
    return "\n".join(lines)


def render_markdown_tables(tables: Dict[str, Dict[str, Any]]) -> str:
    """兼容旧测试入口，渲染任意 RQ 表。"""
    parts: List[str] = []
    for rq_key in ["rq1", "rq2", "rq3", "rq4"]:
        table = tables.get(rq_key)
        if not table:
            continue
        parts.append(markdown_table(RQ_SECTION_TITLES[rq_key], table["rows"], table["columns"]))
    return "\n".join(parts).rstrip() + "\n"


def load_reference_metrics(reference_out_dir: Path) -> Dict[str, Dict[str, Dict[str, float | None]]]:
    """读取 reference CSV 中的数值指标。"""
    reference: Dict[str, Dict[str, Dict[str, float | None]]] = {}
    for rq_key, csv_name in CSV_NAME_BY_RQ.items():
        path = reference_out_dir / csv_name
        if not path.is_file():
            continue
        with path.open(encoding="utf-8", newline="") as handle:
            reader = csv.DictReader(handle)
            fieldnames = list(reader.fieldnames or [])
            if not fieldnames:
                continue
            name_col = fieldnames[0]
            rows: Dict[str, Dict[str, float | None]] = {}
            for row in reader:
                metrics = {
                    key: maybe_number(value)
                    for key, value in row.items()
                    if key != name_col and maybe_number(value) is not None
                }
                rows[str(row[name_col])] = metrics
            reference[rq_key] = rows
    return reference


def compute_actual_metrics(structured_json_dir: Path) -> Dict[str, Dict[str, Dict[str, float | None]]]:
    """从当前 structured JSON 中读取数值指标。"""
    actual: Dict[str, Dict[str, Dict[str, float | None]]] = {}
    for rq_key, json_name in STRUCTURED_NAME_BY_RQ.items():
        path = structured_json_dir / json_name
        if not path.is_file():
            continue
        rows = load_json(path).get("rows") or []
        table: Dict[str, Dict[str, float | None]] = {}
        for row in rows:
            name_col = "Method" if "Method" in row else "Group" if "Group" in row else "Case"
            metrics = {
                key: maybe_number(value)
                for key, value in row.items()
                if key != name_col and maybe_number(value) is not None
            }
            table[str(row[name_col])] = metrics
        actual[rq_key] = table
    return actual


def build_summary_payload(
    ohos_rows: List[Dict[str, str]],
    oss8_rows: List[Dict[str, str]],
    rq3: List[Dict[str, str]],
    rq4: List[Dict[str, str]],
    checks: List[str],
) -> Dict[str, Any]:
    """组装当前论文指标总览。"""
    return {
        "metric_policy": "Macro project average for method and ablation tables unless a row explicitly reports micro evidence.",
        "sources": {
            "method_comparison": str(METHOD_SUMMARY_JSON.relative_to(REPO_ROOT)),
            "ablation": str(ABLATION_SUMMARY_JSON.relative_to(REPO_ROOT)),
            "ohos_archive": str(OHOS_ARCHIVE_JSON.relative_to(REPO_ROOT)),
            "oss8_archive": str(OSS8_ARCHIVE_JSON.relative_to(REPO_ROOT)),
        },
        "key_number_checks": checks,
        "tables": {
            "rq1_ohos10_method_comparison": ohos_rows,
            "rq2_oss8_method_comparison": oss8_rows,
            "rq3_ohos10_ablation": rq3,
            "rq4_case_evidence": rq4,
        },
    }


def write_markdown_summary(payload: Dict[str, Any]) -> None:
    """写入当前论文指标 Markdown 总览。"""
    tables = payload["tables"]
    parts = [
        "# Current Paper Metrics Alignment",
        "",
        payload["metric_policy"],
        "",
        "## Key Number Checks",
        "",
    ]
    parts.extend(f"- {item}" for item in payload["key_number_checks"])
    parts.append("")
    parts.append(
        markdown_table(
            "RQ1: OpenHarmony Module Dataset",
            tables["rq1_ohos10_method_comparison"],
            ["Method", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        )
    )
    parts.append(
        markdown_table(
            "RQ2: Open-Source Project Dataset",
            tables["rq2_oss8_method_comparison"],
            ["Method", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        )
    )
    parts.append(
        markdown_table(
            "RQ3: Ablation Study",
            tables["rq3_ohos10_ablation"],
            ["Group", "Setting", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        )
    )
    parts.append(
        markdown_table(
            "RQ4: Historical Knowledge Reuse Cases",
            tables["rq4_case_evidence"],
            ["Case", "Boundary", "Evidence", "PaperSection"],
        )
    )
    SUMMARY_MD.write_text("\n".join(parts).rstrip() + "\n", encoding="utf-8")


def export_current_plot_metrics(
    reference_out_dir: Path = DEFAULT_REFERENCE_OUT_DIR,
    structured_json_dir: Path = DEFAULT_STRUCTURED_DIR,
    log_dir: Path = DEFAULT_LOG_DIR,
    rerun: bool = False,
) -> Dict[str, Any]:
    """兼容旧入口，导出当前论文指标并返回相对路径摘要。"""
    del log_dir, rerun
    main()
    return {
        "repo_root": ".",
        "reference_out_dir": repo_relative_path(reference_out_dir),
        "structured_json_dir": repo_relative_path(structured_json_dir),
        "differences": {},
    }


def main() -> int:
    """导出当前论文口径指标。"""
    method_summary = load_json(METHOD_SUMMARY_JSON)
    ablation_summary = load_json(ABLATION_SUMMARY_JSON)
    load_json(OHOS_ARCHIVE_JSON)
    load_json(OSS8_ARCHIVE_JSON)

    ohos_rows = method_rows(method_summary, "OHOS10")
    oss8_rows = method_rows(method_summary, "OSS8")
    rq3 = ablation_rows(ablation_summary)
    rq4 = rq4_rows()
    checks = assert_key_numbers(ohos_rows, oss8_rows, rq3)

    write_csv(
        REFERENCE_DIR / "rq1_method_metric_avg.csv",
        ["Method", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        ohos_rows,
    )
    write_csv(
        REFERENCE_DIR / "rq2_method_metric_avg.csv",
        ["Method", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        oss8_rows,
    )
    write_csv(
        REFERENCE_DIR / "rq3_method_metric_avg.csv",
        ["Group", "Setting", "ICompRate", "TestPass", "Unsafe", "RequiredUnsafe"],
        rq3,
    )
    write_csv(
        REFERENCE_DIR / "rq4_method_metric_avg.csv",
        ["Case", "Boundary", "Evidence", "PaperSection"],
        rq4,
    )

    write_json(STRUCTURED_DIR / "rq1_ohos10_method_comparison.json", {"rows": ohos_rows})
    write_json(STRUCTURED_DIR / "rq2_oss8_method_comparison.json", {"rows": oss8_rows})
    write_json(STRUCTURED_DIR / "rq3_ohos10_ablation.json", {"rows": rq3})
    write_json(STRUCTURED_DIR / "rq4_case_evidence.json", {"rows": rq4})

    payload = build_summary_payload(ohos_rows, oss8_rows, rq3, rq4, checks)
    write_json(SUMMARY_JSON, payload)
    write_markdown_summary(payload)
    print(f"wrote {SUMMARY_MD.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
