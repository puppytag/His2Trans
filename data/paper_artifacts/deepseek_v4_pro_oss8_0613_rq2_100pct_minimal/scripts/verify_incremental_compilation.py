#!/usr/bin/env python3
"""论文实验：按最终 Rust 结果自身逐函数恢复计算增量编译率。"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence


REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from analyze_required_unsafe import DEFAULT_OHOS_C_PROJECTS, _resolve_run_dir, find_project_crate_dir  # noqa: E402
from ohos_incremental_core import verify_incremental_compilation_from_rust_result  # noqa: E402


def _parse_projects(value: Optional[str], run_dir: Path) -> List[str]:
    """解析项目列表；未指定时从 run 目录发现 final crate。"""
    if value:
        return [item.strip() for item in value.split(",") if item.strip()]
    intermediate = _resolve_run_dir(run_dir) / "intermediate"
    if intermediate.is_dir():
        discovered = [
            p.name
            for p in intermediate.iterdir()
            if (p / "workspace" / "final_projects").is_dir()
        ]
        if discovered:
            return sorted(discovered)
    return list(DEFAULT_OHOS_C_PROJECTS)


def _default_output_for_run(run_dir: Path) -> Path:
    """生成默认输出路径。"""
    return Path("paper_experiments/results") / f"incremental_compile_{run_dir.resolve().name}.json"


def _format_percent(value: float) -> str:
    """格式化百分比。"""
    return f"{value * 100:.2f}%"


def _summarize(raw: Dict[str, Any]) -> Dict[str, Any]:
    """压缩单项目结果。"""
    return {
        "denominator_kind": raw.get("denominator_kind"),
        "compiled_functions": int(raw.get("compiled_functions") or 0),
        "restored_functions": int(raw.get("restored_functions") or 0),
        "scanned_functions": int(raw.get("total_functions") or 0),
        "compile_rate": float(raw.get("compile_rate") or 0.0),
        "llm_functions": int(raw.get("llm_functions") or 0),
        "c2rust_fallback_functions": int(raw.get("c2rust_fallback_functions") or 0),
        "unimplemented_functions": int(raw.get("unimplemented_functions") or 0),
        "baseline_compilation_succeeded": raw.get("baseline_compilation_succeeded"),
        "skeleton_compilation_succeeded": raw.get("skeleton_compilation_succeeded"),
        "error": raw.get("error"),
    }


def analyze_run(
    run_dir: Path,
    projects: Sequence[str],
    timeout: int,
) -> Dict[str, Any]:
    """复算多个项目的 Rust-result 增量编译率。"""
    framework_run_dir = _resolve_run_dir(run_dir)
    compiled_total = 0
    restored_total = 0
    projects_report: Dict[str, Any] = {}
    errors: List[str] = []

    for project in projects:
        crate_dir = find_project_crate_dir(framework_run_dir, project)
        if crate_dir is None:
            projects_report[project] = {
                "project": project,
                "crate_dir": None,
                "compiled_functions": 0,
                "restored_functions": 0,
                "compile_rate": 0.0,
                "error": "crate not found",
            }
            errors.append(f"{project}: crate not found")
            continue
        raw = verify_incremental_compilation_from_rust_result(
            crate_dir=crate_dir,
            project_name=project,
            timeout=timeout,
            count_sources={"llm", "c2rust_fallback"},
            default_source="llm",
        )
        summary = _summarize(raw)
        summary["project"] = project
        summary["crate_dir"] = str(crate_dir.resolve())
        projects_report[project] = summary
        if summary.get("error"):
            errors.append(f"{project}: {summary['error']}")
        compiled_total += int(summary["compiled_functions"])
        restored_total += int(summary["restored_functions"])

    return {
        "run_dir": str(framework_run_dir.resolve()),
        "denominator_kind": "rust_result_function_instances",
        "projects": projects_report,
        "errors": errors,
        "incremental_compilation": {
            "compiled_functions": compiled_total,
            "restored_functions": restored_total,
            "compile_rate": (compiled_total / restored_total) if restored_total else 0.0,
        },
    }


def render_markdown(report: Dict[str, Any]) -> str:
    """生成 Markdown 表。"""
    inc = report.get("incremental_compilation") or {}
    lines = [
        "| project | compiled functions | restored functions | incremental compile rate | placeholders | error |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for project, item in (report.get("projects") or {}).items():
        lines.append(
            f"| {project} | {item.get('compiled_functions', 0)} | {item.get('restored_functions', 0)} | "
            f"{_format_percent(float(item.get('compile_rate') or 0.0))} | "
            f"{item.get('unimplemented_functions', 0)} | {item.get('error') or ''} |"
        )
    lines.extend(
        [
            "",
            "| compiled functions | restored functions | incremental compile rate |",
            "|---:|---:|---:|",
            f"| {inc.get('compiled_functions', 0)} | {inc.get('restored_functions', 0)} | "
            f"{_format_percent(float(inc.get('compile_rate') or 0.0))} |",
        ]
    )
    return "\n".join(lines) + "\n"


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    parser = argparse.ArgumentParser(description="按最终 Rust 结果自身逐函数恢复计算增量编译率。")
    parser.add_argument("--run-dir", type=Path, required=True, help="batch run 目录")
    parser.add_argument("--projects", help="逗号分隔项目名；默认从 run 目录发现")
    parser.add_argument("--timeout", type=int, default=300, help="单个函数 cargo check 超时秒数")
    parser.add_argument("--output", "-o", type=Path, help="JSON 输出路径")
    parser.add_argument("--markdown", type=Path, help="Markdown 输出路径")
    args = parser.parse_args(argv)

    projects = _parse_projects(args.projects, args.run_dir)
    report = analyze_run(args.run_dir, projects, args.timeout)

    out_path = args.output or _default_output_for_run(args.run_dir)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    md_path = args.markdown or out_path.with_suffix(".md")
    md_path.write_text(render_markdown(report), encoding="utf-8")

    inc = report["incremental_compilation"]
    print(f"JSON: {out_path}")
    print(f"Markdown: {md_path}")
    print(
        "incremental compile: "
        f"{inc['compiled_functions']}/{inc['restored_functions']} "
        f"({_format_percent(float(inc['compile_rate']))})"
    )
    if report.get("errors"):
        print("warnings/errors:", file=sys.stderr)
        for error in report["errors"]:
            print(f"- {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
