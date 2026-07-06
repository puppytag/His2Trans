#!/usr/bin/env python3
"""论文实验：汇总 OHOS 10 项翻译结果指标表。"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence


REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from analyze_required_unsafe import DEFAULT_OHOS_C_PROJECTS, _resolve_run_dir  # noqa: E402


DISPLAY_NAMES = {
    "host__25c1898e1626": "host",
    "appverify_lite__e5ebe91a98b9": "appverify_lite",
    "manager__c248934e0221": "manager",
    "shared__541f4e547bdb": "shared_541",
    "posix__1b7f59c68bbc": "posix",
    "common__89d5ecaafdff": "common",
    "core__ef5242b7ab08": "core",
    "shared__12e38ea922f7": "shared_12",
    "osal__0bc4f21396ad": "osal",
    "sapm__193cdeb43a97": "sapm",
}


def _load_json(path: Path) -> Dict[str, Any]:
    """读取 JSON 文件，缺失时直接失败。"""
    if not path.is_file():
        raise FileNotFoundError(f"JSON 文件不存在: {path}")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError(f"JSON 顶层必须是对象: {path}")
    return data


def _parse_projects(value: Optional[str]) -> List[str]:
    """解析项目列表。"""
    if not value:
        return list(DEFAULT_OHOS_C_PROJECTS)
    return [item.strip() for item in value.split(",") if item.strip()]


def _run_name(run_dir: Path) -> str:
    """提取实验 run 名称。"""
    return run_dir.resolve().name


def _default_incremental_json(run_dir: Path) -> Path:
    """生成默认增量编译 JSON 路径。"""
    return Path("paper_experiments/results") / f"incremental_compile_{_run_name(run_dir)}.json"


def _default_unsafe_json(run_dir: Path) -> Path:
    """生成默认 unsafe JSON 路径。"""
    return Path("paper_experiments/results") / f"required_unsafe_{_run_name(run_dir)}.json"


def _default_gtest_dir(run_dir: Path) -> Path:
    """生成默认 gtest 结果目录。"""
    return Path("paper_experiments/results") / f"ohos_c_gtests_{_run_name(run_dir)}_envfix_final"


def _default_analysis_json(run_dir: Path) -> Path:
    """生成默认 analyzer JSON 路径，只读取 warnings 和 cargo check。"""
    return _resolve_run_dir(run_dir) / "results" / "compilation_analysis_ohos_test5.json"


def _default_output(run_dir: Path) -> Path:
    """生成默认汇总 JSON 路径。"""
    return Path("paper_experiments/results") / f"ohos_metrics_{_run_name(run_dir)}.json"


def _ratio(num: int, den: int) -> float:
    """计算比例。"""
    return (num / den) if den else 0.0


def _mean(values: Sequence[float]) -> float:
    """计算项目级宏平均。"""
    return (sum(values) / len(values)) if values else 0.0


def _format_rate(num: int, den: int) -> str:
    """格式化分数和百分比。"""
    return f"{num}/{den}={_ratio(num, den) * 100:.2f}%"


def _format_percent(value: float) -> str:
    """格式化 0-1 比例。"""
    return f"{value * 100:.2f}%"


def _project_item(data: Dict[str, Any], key: str, project: str, source: Path) -> Dict[str, Any]:
    """读取项目字段，缺失时直接报错。"""
    projects = data.get(key)
    if not isinstance(projects, dict) or project not in projects:
        raise KeyError(f"{source} 缺少项目 {project}")
    item = projects[project]
    if not isinstance(item, dict):
        raise ValueError(f"{source} 中项目 {project} 必须是对象")
    return item


def _gtest_results(gtest_dir: Path) -> Dict[str, Any]:
    """读取 gtest summary 的逐项目结果。"""
    summary_path = gtest_dir / "summary.json"
    data = _load_json(summary_path)
    results = data.get("results")
    if not isinstance(results, list):
        raise ValueError(f"{summary_path} 缺少 results 列表")
    by_project: Dict[str, Any] = {}
    for item in results:
        if isinstance(item, dict) and item.get("project"):
            by_project[str(item["project"])] = item
    return {"summary": data.get("summary") or {}, "projects": by_project, "path": str(summary_path)}


def summarize_metrics(
    *,
    run_dir: Path,
    projects: Sequence[str],
    incremental_json: Path,
    unsafe_json: Path,
    gtest_dir: Path,
    analysis_json: Path,
) -> Dict[str, Any]:
    """合并逐项目指标。"""
    incremental = _load_json(incremental_json)
    unsafe = _load_json(unsafe_json)
    gtest = _gtest_results(gtest_dir)
    analysis = _load_json(analysis_json)

    if not isinstance(incremental.get("projects"), dict):
        raise ValueError(f"{incremental_json} 缺少逐项目 projects；请先重新运行 verify_incremental_compilation.py")

    rows: List[Dict[str, Any]] = []
    totals = {
        "incremental_compiled": 0,
        "incremental_restored": 0,
        "gtest_passed": 0,
        "gtest_total": 0,
        "warnings": 0,
        "compiled_projects": 0,
        "projects": 0,
        "raw_unsafe_lines": 0,
        "required_unsafe_lines": 0,
        "total_lines": 0,
    }

    for project in projects:
        inc_item = _project_item(incremental, "projects", project, incremental_json)
        unsafe_item = _project_item(unsafe, "projects", project, unsafe_json)
        analysis_item = _project_item(analysis, "projects", project, analysis_json)
        gtest_item = _project_item(gtest, "projects", project, gtest_dir / "summary.json")

        inc_compiled = int(inc_item.get("compiled_functions") or 0)
        inc_restored = int(inc_item.get("restored_functions") or 0)
        gtest_passed = int(gtest_item.get("tests_passed") or 0)
        gtest_total = int(gtest_item.get("tests_total") or 0)
        warnings = int((analysis_item.get("clippy") or {}).get("warning_count_total") or 0)
        compile_ok = bool((analysis_item.get("cargo_check") or {}).get("passed"))
        raw_unsafe = int(unsafe_item.get("raw_unsafe_lines") or 0)
        required_unsafe = int(unsafe_item.get("required_unsafe_lines") or 0)
        total_lines = int(unsafe_item.get("total_lines") or 0)

        rows.append(
            {
                "project": project,
                "display_project": DISPLAY_NAMES.get(project, project),
                "incremental_compiled": inc_compiled,
                "incremental_restored": inc_restored,
                "incremental_rate": _ratio(inc_compiled, inc_restored),
                "gtest_passed": gtest_passed,
                "gtest_total": gtest_total,
                "gtest_pass_rate": _ratio(gtest_passed, gtest_total),
                "warnings": warnings,
                "compile_ok": compile_ok,
                "raw_unsafe_lines": raw_unsafe,
                "required_unsafe_lines": required_unsafe,
                "total_lines": total_lines,
                "raw_unsafe_rate": _ratio(raw_unsafe, total_lines),
                "required_unsafe_rate": _ratio(required_unsafe, total_lines),
            }
        )

        totals["incremental_compiled"] += inc_compiled
        totals["incremental_restored"] += inc_restored
        totals["gtest_passed"] += gtest_passed
        totals["gtest_total"] += gtest_total
        totals["warnings"] += warnings
        totals["compiled_projects"] += 1 if compile_ok else 0
        totals["projects"] += 1
        totals["raw_unsafe_lines"] += raw_unsafe
        totals["required_unsafe_lines"] += required_unsafe
        totals["total_lines"] += total_lines

    micro_summary = {
        **totals,
        "incremental_rate": _ratio(totals["incremental_compiled"], totals["incremental_restored"]),
        "gtest_pass_rate": _ratio(totals["gtest_passed"], totals["gtest_total"]),
        "compile_project_rate": _ratio(totals["compiled_projects"], totals["projects"]),
        "raw_unsafe_rate": _ratio(totals["raw_unsafe_lines"], totals["total_lines"]),
        "required_unsafe_rate": _ratio(totals["required_unsafe_lines"], totals["total_lines"]),
    }
    summary = {
        **totals,
        "metric_policy": "paper_macro_project_average",
        "incremental_rate": _mean([float(row["incremental_rate"]) for row in rows]),
        "gtest_pass_rate": _mean([float(row["gtest_pass_rate"]) for row in rows]),
        "compile_project_rate": micro_summary["compile_project_rate"],
        "raw_unsafe_rate": _mean([float(row["raw_unsafe_rate"]) for row in rows]),
        "required_unsafe_rate": _mean([float(row["required_unsafe_rate"]) for row in rows]),
        "micro_evidence": micro_summary,
    }
    return {
        "run_dir": str(run_dir.resolve()),
        "inputs": {
            "incremental_json": str(incremental_json),
            "unsafe_json": str(unsafe_json),
            "gtest_summary_json": gtest["path"],
            "analysis_json": str(analysis_json),
        },
        "projects": rows,
        "summary": summary,
    }


def render_markdown(report: Dict[str, Any]) -> str:
    """生成论文实验指标总表。"""
    lines = [
        "| 项目 | 增量编译率 | gtest 通过率 | warnings | 可编译 | raw unsafe 率 | required unsafe 率 |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for item in report.get("projects") or []:
        lines.append(
            f"| {item['display_project']} | "
            f"{_format_rate(int(item['incremental_compiled']), int(item['incremental_restored']))} | "
            f"{_format_rate(int(item['gtest_passed']), int(item['gtest_total']))} | "
            f"{int(item['warnings'])} | "
            f"{'是' if item['compile_ok'] else '否'} | "
            f"{_format_rate(int(item['raw_unsafe_lines']), int(item['total_lines']))} | "
            f"{_format_rate(int(item['required_unsafe_lines']), int(item['total_lines']))} |"
        )
    summary = report.get("summary") or {}
    lines.append(
        f"| **Paper macro average** | "
        f"{_format_percent(float(summary.get('incremental_rate') or 0.0))} | "
        f"{_format_percent(float(summary.get('gtest_pass_rate') or 0.0))} | "
        f"{int(summary.get('warnings') or 0)} | "
        f"{int(summary.get('compiled_projects') or 0)}/{int(summary.get('projects') or 0)} | "
        f"{_format_percent(float(summary.get('raw_unsafe_rate') or 0.0))} | "
        f"{_format_percent(float(summary.get('required_unsafe_rate') or 0.0))} |"
    )
    lines.extend(
        [
            "",
            "The final row is the paper metric policy: macro average over project-level rates.",
        ]
    )
    return "\n".join(lines) + "\n"


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    parser = argparse.ArgumentParser(description="汇总 OHOS 10 项论文实验指标表。")
    parser.add_argument("--run-dir", type=Path, required=True, help="batch run 目录")
    parser.add_argument("--projects", help="逗号分隔项目名；默认 10 个 OHOS C gtest 项目")
    parser.add_argument("--incremental-json", type=Path, help="verify_incremental_compilation.py 输出 JSON")
    parser.add_argument("--unsafe-json", type=Path, help="analyze_required_unsafe.py 输出 JSON")
    parser.add_argument("--gtest-dir", type=Path, help="run_ohos_c_gtests.py 输出目录")
    parser.add_argument("--analysis-json", type=Path, help="只用于读取 warnings 和 cargo check 的 analyzer JSON")
    parser.add_argument("--output", "-o", type=Path, help="汇总 JSON 输出路径")
    parser.add_argument("--markdown", type=Path, help="Markdown 表格输出路径")
    args = parser.parse_args(argv)

    projects = _parse_projects(args.projects)
    out_path = args.output or _default_output(args.run_dir)
    report = summarize_metrics(
        run_dir=args.run_dir,
        projects=projects,
        incremental_json=args.incremental_json or _default_incremental_json(args.run_dir),
        unsafe_json=args.unsafe_json or _default_unsafe_json(args.run_dir),
        gtest_dir=args.gtest_dir or _default_gtest_dir(args.run_dir),
        analysis_json=args.analysis_json or _default_analysis_json(args.run_dir),
    )

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    md_path = args.markdown or out_path.with_suffix(".md")
    md_path.write_text(render_markdown(report), encoding="utf-8")

    summary = report["summary"]
    print(f"JSON: {out_path}")
    print(f"Markdown: {md_path}")
    print(f"incremental compile: {_format_percent(float(summary['incremental_rate']))}")
    print(f"gtest: {_format_percent(float(summary['gtest_pass_rate']))}")
    print(f"raw unsafe: {_format_percent(float(summary['raw_unsafe_rate']))}")
    print(f"required unsafe: {_format_percent(float(summary['required_unsafe_rate']))}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
