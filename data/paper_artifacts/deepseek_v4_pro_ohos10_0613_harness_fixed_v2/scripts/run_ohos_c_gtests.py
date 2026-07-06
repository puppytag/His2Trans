#!/usr/bin/env python3
"""论文实验：批量运行 OHOS C 项目原始测试到翻译 Rust staticlib 的转发测试。"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Dict, List

from ohos_c_gtest_harness import (
    C_PROJECT_SOURCES,
    DEFAULT_RUN_DIR,
    REPO_ROOT,
    build_and_run_project,
    crate_dir_for,
    summarize_results,
)


def _result_dir(run_dir: str) -> Path:
    """返回默认结果目录。"""
    return REPO_ROOT / "paper_experiments/results" / f"ohos_c_gtests_{run_dir}"


def _write_json(path: Path, data: Dict[str, Any]) -> None:
    """写入 JSON 结果文件。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8")


def _markdown_table(results: List[Dict[str, Any]], summary: Dict[str, Any]) -> str:
    """生成便于论文记录查看的 Markdown 汇总。"""
    summary_rate = summary.get("pass_rate")
    summary_rate_text = "None" if summary_rate is None else f"{summary_rate:.2%}"
    semantic_rate = summary.get("semantic_pass_rate")
    semantic_rate_text = "None" if semantic_rate is None else f"{semantic_rate:.2%}"
    lines = [
        "# OHOS C gtest Results",
        "",
        "| project | stage | compiled | executed | total | passed | failed | pass_rate | rust_symbols | semantic_covered | error |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|",
    ]
    for item in results:
        total = item.get("tests_total")
        passed = item.get("tests_passed")
        failed = item.get("tests_failed")
        rate = item.get("pass_rate")
        rate_text = "" if rate is None else f"{rate:.2%}"
        symbols = len(item.get("test_symbols_from_rust_staticlib") or [])
        semantic_covered = "yes" if symbols else "no"
        error = (item.get("error") or "").replace("|", "\\|")
        lines.append(
            "| {project} | {stage} | {compiled} | {executed} | {total} | {passed} | {failed} | {rate} | {symbols} | {semantic_covered} | {error} |".format(
                project=item.get("project", ""),
                stage=item.get("stage", ""),
                compiled="yes" if item.get("compiled") else "no",
                executed="yes" if item.get("executed") else "no",
                total="" if total is None else total,
                passed="" if passed is None else passed,
                failed="" if failed is None else failed,
                rate=rate_text,
                symbols=symbols,
                semantic_covered=semantic_covered,
                error=error,
            )
        )
    lines.extend(
        [
            "",
            "## Summary",
            "",
            f"- projects_total: {summary.get('projects_total')}",
            f"- projects_compiled: {summary.get('projects_compiled')}",
            f"- projects_executed: {summary.get('projects_executed')}",
            f"- projects_with_gtest_counts: {summary.get('projects_with_gtest_counts')}",
            f"- projects_with_rust_symbol_coverage: {summary.get('projects_with_rust_symbol_coverage')}",
            f"- semantic_projects_executed: {summary.get('semantic_projects_executed')}",
            f"- tests_total: {summary.get('tests_total')}",
            f"- tests_passed: {summary.get('tests_passed')}",
            f"- tests_failed: {summary.get('tests_failed')}",
            f"- pass_rate: {summary_rate_text}",
            f"- semantic_tests_total: {summary.get('semantic_tests_total')}",
            f"- semantic_tests_passed: {summary.get('semantic_tests_passed')}",
            f"- semantic_tests_failed: {summary.get('semantic_tests_failed')}",
            f"- semantic_pass_rate: {semantic_rate_text}",
            "",
        ]
    )
    return "\n".join(lines)


def parse_args() -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", default=DEFAULT_RUN_DIR, help="experiment_runs 下的运行目录名")
    parser.add_argument(
        "--project",
        action="append",
        choices=sorted(C_PROJECT_SOURCES),
        help="只运行指定项目；可重复传入，默认运行全部 10 个 C 项目",
    )
    parser.add_argument("--output-dir", type=Path, help="结果输出目录")
    parser.add_argument("--timeout", type=int, default=900, help="单个编译/运行命令超时时间秒数")
    parser.add_argument("--keep-build-dir", action="store_true", help="保留临时构建目录用于排查")
    return parser.parse_args()


def main() -> int:
    """运行批量测试入口。"""
    args = parse_args()
    projects = args.project or list(C_PROJECT_SOURCES)
    output_dir = args.output_dir or _result_dir(args.run_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    results: List[Dict[str, Any]] = []
    for project in projects:
        print(f"[run] {project}", flush=True)
        result = build_and_run_project(
            project=project,
            crate_dir=crate_dir_for(args.run_dir, project),
            source_dir=C_PROJECT_SOURCES[project],
            timeout=args.timeout,
            keep_build_dir=args.keep_build_dir,
        )
        results.append(result)
        _write_json(output_dir / f"{project}.json", result)
        print(
            "[done] {project} stage={stage} compiled={compiled} executed={executed} total={total} passed={passed} error={error}".format(
                project=project,
                stage=result.get("stage"),
                compiled=result.get("compiled"),
                executed=result.get("executed"),
                total=result.get("tests_total"),
                passed=result.get("tests_passed"),
                error=result.get("error") or "",
            ),
            flush=True,
        )

    summary = summarize_results(results)
    summary["run_dir"] = args.run_dir
    summary["projects"] = projects
    summary["output_dir"] = str(output_dir)
    _write_json(output_dir / "summary.json", {"summary": summary, "results": results})
    (output_dir / "summary.md").write_text(_markdown_table(results, summary), encoding="utf-8")
    print(json.dumps(summary, indent=2, ensure_ascii=False), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
