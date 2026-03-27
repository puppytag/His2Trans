#!/usr/bin/env python3

from __future__ import annotations

import argparse
import csv
import json
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Dict, List


THIS_DIR = Path(__file__).resolve().parent
REPO_ROOT = THIS_DIR.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.analysis import paper_metrics


DEFAULT_EXPORT_DIR = REPO_ROOT / "data" / "paper_metric_exports"
DEFAULT_REFERENCE_OUT_DIR = DEFAULT_EXPORT_DIR / "reference_tables"
DEFAULT_STRUCTURED_DIR = DEFAULT_EXPORT_DIR / "generated_structured_json"
DEFAULT_LOG_DIR = DEFAULT_EXPORT_DIR / "logs"
DEFAULT_SUMMARY_JSON = DEFAULT_EXPORT_DIR / "current_plot_metrics_alignment.json"
DEFAULT_SUMMARY_MD = DEFAULT_EXPORT_DIR / "current_plot_metrics_alignment.md"

GENERIC_SCRIPT = REPO_ROOT / "scripts" / "analyze_c2r_compilation_rate.py"
OHOS_SCRIPT = REPO_ROOT / "scripts" / "analyze_c2r_compilation_rate_ohos_test5.py"
DEFAULT_C2R_TESTS_DIR = REPO_ROOT / "data" / "test_module_rust_tests"
DEFAULT_HUAWEI_PROJECTS_TSV = REPO_ROOT / "data" / "ohos" / "huawei_projects.tsv"
DEFAULT_OHOS_ROOT = REPO_ROOT / "data" / "ohos" / "ohos_root_min"

CSV_NAME_BY_RQ = {
    "rq1": "rq1_method_metric_avg.csv",
    "rq2": "rq2_method_metric_avg.csv",
    "rq3": "rq3_method_metric_avg.csv",
    "rq4": "rq4_method_metric_avg.csv",
}

REFERENCE_METRIC_RENAMES = {
    "Warnings": "Clippy",
}

RQ_SECTION_TITLES = {
    "rq1": "RQ1: OHOS test5",
    "rq2": "RQ2: test_module",
    "rq3": "RQ3: Ablation Study",
    "rq4": "RQ4: Knowledge Base Ablation",
}

RQ1_METHODS = {
    "Ours(DS-V3.2 K = 1)": "rq1_k1.json",
    "Ours(DS-V3.2 K = 3)": "rq1_k3.json",
    "Ours(DS-V3.2 K = 5)": "rq1_k5.json",
    "Ours(DS-V3.2 K = 10)": "rq1_k10.json",
    "Ours(Claude-4.5 K = 5)": "rq1_claude.json",
}
RQ2_METHODS = {
    "Ours(DS-V3.2)": "rq2_deepseek.json",
    "Ours(Claude-4.5)": "rq2_claude.json",
}
RQ3_METHODS = {
    "Base-1Shot": "rq3_c0.json",
    "Base-Rep": "rq3_c1.json",
    "Pred-1Shot": "rq3_c2.json",
    "Pred-Rep": "rq3_c3.json",
    "GT-API": "rq3_c4.json",
    "GT-Frag": "rq3_c5.json",
    "GT-Full": "rq3_c6.json",
}
RQ4_METHODS = {
    "Base KB only": "rq4_base_kb_only.json",
    "Base KB + Accumulated KB": "rq4_base_kb_sedimented.json",
}


@dataclass(frozen=True)
class RunSpec:
    rq_key: str
    method: str
    output_name: str
    script_path: Path
    run_dir: Path
    extra_args: tuple[str, ...]


def build_run_specs() -> List[RunSpec]:
    specs: List[RunSpec] = []
    rq1_dirs = {
        "Ours(DS-V3.2 K = 1)": REPO_ROOT / "data" / "rq1" / "k1",
        "Ours(DS-V3.2 K = 3)": REPO_ROOT / "data" / "rq1" / "k3",
        "Ours(DS-V3.2 K = 5)": REPO_ROOT / "data" / "rq1" / "k5",
        "Ours(DS-V3.2 K = 10)": REPO_ROOT / "data" / "rq1" / "k10",
        "Ours(Claude-4.5 K = 5)": REPO_ROOT / "data" / "rq1" / "claude",
    }
    for method, run_dir in rq1_dirs.items():
        specs.append(
            RunSpec(
                rq_key="rq1",
                method=method,
                output_name=RQ1_METHODS[method],
                script_path=OHOS_SCRIPT,
                run_dir=run_dir,
                extra_args=(
                    "--all",
                    "--verify-incremental",
                    "--huawei-projects-tsv",
                    str(DEFAULT_HUAWEI_PROJECTS_TSV),
                    "--ohos-root",
                    str(DEFAULT_OHOS_ROOT),
                ),
            )
        )

    rq2_dirs = {
        "Ours(DS-V3.2)": REPO_ROOT / "data" / "rq2" / "deepseek",
        "Ours(Claude-4.5)": REPO_ROOT / "data" / "rq2" / "claude",
    }
    for method, run_dir in rq2_dirs.items():
        specs.append(
            RunSpec(
                rq_key="rq2",
                method=method,
                output_name=RQ2_METHODS[method],
                script_path=GENERIC_SCRIPT,
                run_dir=run_dir,
                extra_args=(
                    "--all",
                    "--verify-incremental",
                    "--quiet",
                    "--c2r-tests-dir",
                    str(DEFAULT_C2R_TESTS_DIR),
                ),
            )
        )

    rq3_dirs = {
        "Base-1Shot": REPO_ROOT / "data" / "rq3" / "c0",
        "Base-Rep": REPO_ROOT / "data" / "rq3" / "c1",
        "Pred-1Shot": REPO_ROOT / "data" / "rq3" / "c2",
        "Pred-Rep": REPO_ROOT / "data" / "rq3" / "c3",
        "GT-API": REPO_ROOT / "data" / "rq3" / "c4",
        "GT-Frag": REPO_ROOT / "data" / "rq3" / "c5",
        "GT-Full": REPO_ROOT / "data" / "rq3" / "c6",
    }
    for method, run_dir in rq3_dirs.items():
        specs.append(
            RunSpec(
                rq_key="rq3",
                method=method,
                output_name=RQ3_METHODS[method],
                script_path=OHOS_SCRIPT,
                run_dir=run_dir,
                extra_args=(
                    "--run-ohos-tests",
                    "--verify-incremental",
                    "--huawei-projects-tsv",
                    str(DEFAULT_HUAWEI_PROJECTS_TSV),
                    "--ohos-root",
                    str(DEFAULT_OHOS_ROOT),
                ),
            )
        )

    rq4_dirs = {
        "Base KB only": REPO_ROOT / "data" / "rq4" / "base_kb_only",
        "Base KB + Accumulated KB": REPO_ROOT / "data" / "rq4" / "base_kb_sedimented",
    }
    for method, run_dir in rq4_dirs.items():
        specs.append(
            RunSpec(
                rq_key="rq4",
                method=method,
                output_name=RQ4_METHODS[method],
                script_path=OHOS_SCRIPT,
                run_dir=run_dir,
                extra_args=(
                    "--run-ohos-tests",
                    "--verify-incremental",
                    "--huawei-projects-tsv",
                    str(DEFAULT_HUAWEI_PROJECTS_TSV),
                    "--ohos-root",
                    str(DEFAULT_OHOS_ROOT),
                ),
            )
        )
    return specs


def load_json(path: Path) -> Dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def repo_relative_path(path: Path | str) -> str:
    candidate = Path(path)
    try:
        resolved = candidate.resolve()
    except Exception:
        resolved = candidate
    try:
        rel = resolved.relative_to(REPO_ROOT)
        return "." if not rel.parts else rel.as_posix()
    except ValueError:
        return str(candidate)


def repo_relative_cmd(cmd: List[str]) -> List[str]:
    return [repo_relative_path(part) if part.startswith("/") else part for part in cmd]


def load_reference_metrics(reference_out_dir: Path) -> Dict[str, Dict[str, Dict[str, float | None]]]:
    methods_by_rq = {
        "rq1": RQ1_METHODS,
        "rq2": RQ2_METHODS,
        "rq3": RQ3_METHODS,
        "rq4": RQ4_METHODS,
    }
    reference: Dict[str, Dict[str, Dict[str, float | None]]] = {}
    for rq_key, csv_name in CSV_NAME_BY_RQ.items():
        csv_path = reference_out_dir / csv_name
        rows: Dict[str, Dict[str, float | None]] = {}
        with csv_path.open(encoding="utf-8", newline="") as handle:
            reader = csv.DictReader(handle)
            first_col = reader.fieldnames[0] if reader.fieldnames else None
            if first_col is None:
                raise RuntimeError(f"CSV 表头为空: {csv_path}")
            for row in reader:
                method = row[first_col]
                if method not in methods_by_rq[rq_key]:
                    continue
                rows[method] = {
                    REFERENCE_METRIC_RENAMES.get(key, key): paper_metrics.normalize_metric_value(value)
                    for key, value in row.items()
                    if key != first_col
                }
        reference[rq_key] = rows
    return reference


def load_reference_tables(reference_out_dir: Path) -> Dict[str, Dict[str, object]]:
    tables: Dict[str, Dict[str, object]] = {}
    for rq_key, csv_name in CSV_NAME_BY_RQ.items():
        csv_path = reference_out_dir / csv_name
        with csv_path.open(encoding="utf-8", newline="") as handle:
            reader = csv.DictReader(handle)
            fieldnames = list(reader.fieldnames or [])
            if not fieldnames:
                raise RuntimeError(f"CSV 表头为空: {csv_path}")
            rows = [{key: (value if value else "--") for key, value in row.items()} for row in reader]
        tables[rq_key] = {
            "columns": fieldnames,
            "rows": rows,
        }
    return tables


def render_markdown_tables(reference_tables: Dict[str, Dict[str, object]]) -> str:
    lines: List[str] = []
    for rq_key in ("rq1", "rq2", "rq3", "rq4"):
        table = reference_tables[rq_key]
        columns = list(table["columns"])
        rows = list(table["rows"])
        lines.append(f"## {RQ_SECTION_TITLES[rq_key]}")
        lines.append("")
        lines.append("| " + " | ".join(columns) + " |")
        lines.append("| " + " | ".join(["---"] * len(columns)) + " |")
        for row in rows:
            lines.append("| " + " | ".join(str(row.get(column, "--")) for column in columns) + " |")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def compute_actual_metrics(structured_json_dir: Path) -> Dict[str, Dict[str, Dict[str, float | None]]]:
    actual = {
        "rq1": {},
        "rq2": {},
        "rq3": {},
        "rq4": {},
    }
    for method, filename in RQ1_METHODS.items():
        actual["rq1"][method] = paper_metrics.rq1_summary_metrics(load_json(structured_json_dir / filename))
    for method, filename in RQ2_METHODS.items():
        actual["rq2"][method] = paper_metrics.rq2_summary_metrics(load_json(structured_json_dir / filename))
    for method, filename in RQ3_METHODS.items():
        actual["rq3"][method] = paper_metrics.rq3_or_rq4_summary_metrics(load_json(structured_json_dir / filename))
    for method, filename in RQ4_METHODS.items():
        actual["rq4"][method] = paper_metrics.rq3_or_rq4_summary_metrics(load_json(structured_json_dir / filename))
    return actual


def run_spec(spec: RunSpec, structured_json_dir: Path, log_dir: Path) -> Dict[str, object]:
    output_path = structured_json_dir / spec.output_name
    log_path = log_dir / f"{spec.rq_key}_{spec.output_name[:-5]}.log"
    cmd = [
        sys.executable,
        str(spec.script_path),
        "--run-dir",
        str(spec.run_dir),
        "--output",
        str(output_path),
        *spec.extra_args,
    ]
    log_dir.mkdir(parents=True, exist_ok=True)
    with log_path.open("w", encoding="utf-8") as handle:
        handle.write("+ " + " ".join(cmd) + "\n")
        handle.flush()
        proc = subprocess.run(
            cmd,
            cwd=str(REPO_ROOT),
            stdout=handle,
            stderr=subprocess.STDOUT,
            text=True,
        )
    if proc.returncode != 0:
        raise RuntimeError(f"{spec.method} 导出失败，详见日志: {log_path}")
    return {
        "method": spec.method,
        "rq": spec.rq_key,
        "run_dir": repo_relative_path(spec.run_dir),
        "output": repo_relative_path(output_path),
        "log": repo_relative_path(log_path),
        "cmd": repo_relative_cmd(cmd),
    }


def export_current_plot_metrics(
    *,
    reference_out_dir: Path,
    structured_json_dir: Path,
    log_dir: Path,
    rerun: bool,
) -> Dict[str, object]:
    commands: List[Dict[str, object]] = []
    structured_json_dir.mkdir(parents=True, exist_ok=True)
    if rerun:
        for spec in build_run_specs():
            commands.append(run_spec(spec, structured_json_dir, log_dir))

    actual = compute_actual_metrics(structured_json_dir)
    reference = load_reference_metrics(reference_out_dir)
    diffs = paper_metrics.compare_metric_tables(actual, reference)
    return {
        "generated_at": datetime.now().isoformat(timespec="seconds"),
        "repo_root": ".",
        "reference_out_dir": repo_relative_path(reference_out_dir),
        "structured_json_dir": repo_relative_path(structured_json_dir),
        "all_match": not bool(diffs),
        "actual": actual,
        "reference": reference,
        "diffs": diffs,
        "commands": commands,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="一键重建并校验当前论文口径下的 His2Trans 结果。")
    parser.add_argument(
        "--reference-out-dir",
        type=Path,
        default=DEFAULT_REFERENCE_OUT_DIR,
        help=f"仓库内置的论文参考 CSV 目录（默认：{DEFAULT_REFERENCE_OUT_DIR}）",
    )
    parser.add_argument(
        "--structured-json-dir",
        type=Path,
        default=DEFAULT_STRUCTURED_DIR,
        help=f"中间 structured_json 输出目录（默认：{DEFAULT_STRUCTURED_DIR}）",
    )
    parser.add_argument(
        "--log-dir",
        type=Path,
        default=DEFAULT_LOG_DIR,
        help=f"分析日志目录（默认：{DEFAULT_LOG_DIR}）",
    )
    parser.add_argument(
        "--out-json",
        type=Path,
        default=DEFAULT_SUMMARY_JSON,
        help=f"汇总导出文件（默认：{DEFAULT_SUMMARY_JSON}）",
    )
    parser.add_argument(
        "--out-md",
        type=Path,
        default=DEFAULT_SUMMARY_MD,
        help=f"英文 Markdown 表格导出文件（默认：{DEFAULT_SUMMARY_MD}）",
    )
    parser.add_argument(
        "--skip-rerun",
        action="store_true",
        help="跳过重新运行分析脚本，直接复用 structured_json-dir 里的结果。",
    )
    args = parser.parse_args()

    summary = export_current_plot_metrics(
        reference_out_dir=args.reference_out_dir,
        structured_json_dir=args.structured_json_dir,
        log_dir=args.log_dir,
        rerun=not args.skip_rerun,
    )
    reference_tables = load_reference_tables(args.reference_out_dir)
    markdown = render_markdown_tables(reference_tables)
    write_json(args.out_json, summary)
    write_text(args.out_md, markdown)
    if summary["all_match"]:
        print(f"[OK] 当前数值与参考绘图一致: {args.out_json}")
        print(f"[OK] Markdown tables exported: {args.out_md}")
        return 0
    print(f"[DIFF] 当前数值与参考绘图不一致，详见: {args.out_json}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
