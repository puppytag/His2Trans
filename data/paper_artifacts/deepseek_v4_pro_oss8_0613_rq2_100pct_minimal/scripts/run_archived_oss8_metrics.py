#!/usr/bin/env python3
"""从归档的 OSS8 Rust crate 一键复跑论文指标。"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence, Tuple


ARCHIVE_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = Path(__file__).resolve().parents[4]
RUN_NAME = "deepseek-v4-pro-oss10-full-0613-1"
ARCHIVE_RUN_NAME = "archived_deepseek_v4_pro_oss8_0613_rq2_100pct"
PROJECTS = [
    "urlparser",
    "avl",
    "ht",
    "qsort",
    "buffer",
    "rgba",
    "quadtree",
    "genann",
]


def _project_csv() -> str:
    """返回固定 OSS8 项目列表。"""
    return ",".join(PROJECTS)


def _run(cmd: Sequence[str], *, cwd: Path) -> None:
    """执行外部脚本，失败时保留原始错误。"""
    print("[run] " + " ".join(cmd), flush=True)
    env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1"}
    subprocess.run(list(cmd), cwd=cwd, env=env, check=True)


def _link_dir(src: Path, dst: Path) -> None:
    """把归档 crate 软链接到临时 run layout。"""
    if dst.exists() or dst.is_symlink():
        if dst.is_symlink() or dst.is_file():
            dst.unlink()
        else:
            shutil.rmtree(dst)
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.symlink_to(src.resolve(), target_is_directory=True)


def _prepare_archived_run_layout(archive_root: Path, output_dir: Path) -> Path:
    """构造现有指标脚本可识别的 raw/framework_output 目录。"""
    run_root = output_dir / ARCHIVE_RUN_NAME
    framework_root = run_root / "raw" / "framework_output"
    (framework_root / "results").mkdir(parents=True, exist_ok=True)

    for project in PROJECTS:
        crate_src = archive_root / "projects" / project
        if not (crate_src / "Cargo.toml").is_file():
            raise FileNotFoundError(f"缺少归档 Rust crate: {crate_src}")
        crate_dst = (
            framework_root
            / "intermediate"
            / project
            / "workspace"
            / "final_projects"
            / project
            / "translate_by_qwen3_coder"
        )
        _link_dir(crate_src, crate_dst)
    return run_root


def _run_cmd_capture(
    cmd: Sequence[str],
    *,
    cwd: Path,
    env: Dict[str, str],
    timeout: int,
) -> Tuple[int, str, str]:
    """运行 cargo 命令并捕获输出。"""
    try:
        proc = subprocess.run(
            list(cmd),
            cwd=cwd,
            env=env,
            text=True,
            capture_output=True,
            timeout=timeout,
            check=False,
        )
        return proc.returncode, proc.stdout or "", proc.stderr or ""
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout if isinstance(exc.stdout, str) else ""
        stderr = exc.stderr if isinstance(exc.stderr, str) else ""
        return 124, stdout, stderr


def _count_json_warnings(output: str) -> Dict[str, Any]:
    """统计 cargo JSON diagnostics 中的 rustc/clippy warnings。"""
    clippy_warnings = 0
    rustc_warnings = 0
    errors = 0
    clippy_codes: Dict[str, int] = {}
    rustc_codes: Dict[str, int] = {}
    for line in output.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        if obj.get("reason") != "compiler-message":
            continue
        msg = obj.get("message") or {}
        level = msg.get("level")
        code_obj = msg.get("code") or {}
        code = str(code_obj.get("code") or "general")
        if level == "warning":
            if code.startswith("clippy::"):
                clippy_warnings += 1
                clippy_codes[code] = clippy_codes.get(code, 0) + 1
            else:
                rustc_warnings += 1
                rustc_codes[code] = rustc_codes.get(code, 0) + 1
        elif level == "error":
            errors += 1
    return {
        "warning_count": clippy_warnings,
        "rustc_warning_count": rustc_warnings,
        "warning_count_total": clippy_warnings + rustc_warnings,
        "error_count": errors,
        "clippy_warning_codes": dict(sorted(clippy_codes.items(), key=lambda kv: (-kv[1], kv[0]))),
        "rustc_warning_codes": dict(sorted(rustc_codes.items(), key=lambda kv: (-kv[1], kv[0]))),
    }


def _run_cargo_clippy(crate_dir: Path, timeout: int) -> Dict[str, Any]:
    """对单个 crate 运行真实 cargo clippy 并统计 warnings。"""
    result: Dict[str, Any] = {
        "crate_dir": str(crate_dir.resolve()),
        "executed": False,
        "returncode": None,
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
        "clippy_warning_codes": {},
        "rustc_warning_codes": {},
        "error": None,
        "stdout_tail": "",
        "stderr_tail": "",
    }
    if not (crate_dir / "Cargo.toml").is_file():
        result["error"] = "Cargo.toml not found"
        return result

    with tempfile.TemporaryDirectory(prefix="oss8_archive_clippy_target_") as target_dir:
        env = {**os.environ, "CARGO_TARGET_DIR": target_dir, "RUST_BACKTRACE": "0"}
        cmd = ["cargo", "clippy", "--offline", "--message-format=json", "--", "-W", "clippy::all"]
        rc, stdout, stderr = _run_cmd_capture(cmd, cwd=crate_dir, env=env, timeout=timeout)
    counts = _count_json_warnings(stdout)
    result.update(counts)
    result["executed"] = True
    result["returncode"] = rc
    result["stdout_tail"] = stdout[-20000:]
    result["stderr_tail"] = stderr[-20000:]
    if rc == 124:
        result["error"] = f"Timeout after {timeout}s"
    elif rc != 0 and int(counts.get("error_count") or 0) > 0:
        result["error"] = "cargo clippy failed"
    return result


def _analyze_warnings(archive_root: Path, timeout: int) -> Dict[str, Any]:
    """统计全部 OSS8 项目的 warnings。"""
    projects: Dict[str, Any] = {}
    totals = {
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
    }
    for project in PROJECTS:
        print(f"[warnings] {project}", flush=True)
        item = _run_cargo_clippy(archive_root / "projects" / project, timeout)
        projects[project] = item
        for key in totals:
            totals[key] += int(item.get(key) or 0)
    return {
        "warning_metric_note": "主表 warnings 使用 rustc_warning_count；warning_count_total 同时包含 clippy lint 和 rustc warnings。",
        "projects": projects,
        "summary": totals,
    }


def _write_json(path: Path, data: Dict[str, Any]) -> None:
    """写入 JSON 文件。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8")


def _load_json(path: Path) -> Dict[str, Any]:
    """读取 JSON 文件。"""
    return json.loads(path.read_text(encoding="utf-8"))


def _format_percent(value: float) -> str:
    """格式化百分比。"""
    return f"{value * 100:.2f}%"


def _mean(values: Sequence[float]) -> float:
    """计算项目级宏平均。"""
    return (sum(values) / len(values)) if values else 0.0


def _rate_text(numerator: int, denominator: int) -> str:
    """格式化带分子分母的百分比。"""
    rate = (numerator / denominator) if denominator else 0.0
    return f"{numerator}/{denominator} ({_format_percent(rate)})"


def _make_summary(
    rq2: Dict[str, Any],
    incremental: Dict[str, Any],
    unsafe_report: Dict[str, Any],
    warnings: Dict[str, Any],
) -> Dict[str, Any]:
    """合并各脚本输出为论文表格所需摘要。"""
    rows: Dict[str, Any] = {}
    for project in PROJECTS:
        rq2_item = (rq2.get("projects") or {}).get(project) or {}
        inc_item = (incremental.get("projects") or {}).get(project) or {}
        unsafe_item = (unsafe_report.get("projects") or {}).get(project) or {}
        warning_item = (warnings.get("projects") or {}).get(project) or {}
        rows[project] = {
            "incremental_compilation": {
                "compiled_functions": int(inc_item.get("compiled_functions") or 0),
                "restored_functions": int(inc_item.get("restored_functions") or 0),
                "compile_rate": float(inc_item.get("compile_rate") or 0.0),
            },
            "rq2_tests": {
                "tests_passed": int(rq2_item.get("tests_passed") or 0),
                "total_tests": int(rq2_item.get("total_tests") or rq2_item.get("expected_total_tests") or 0),
                "pass_rate": float(rq2_item.get("pass_rate") or 0.0),
            },
            "warnings": {
                "rustc_warning_count": int(warning_item.get("rustc_warning_count") or 0),
                "clippy_warning_count": int(warning_item.get("warning_count") or 0),
                "warning_count_total": int(warning_item.get("warning_count_total") or 0),
            },
            "unsafe": {
                "total_lines": int(unsafe_item.get("total_lines") or 0),
                "raw_unsafe_lines": int(unsafe_item.get("raw_unsafe_lines") or 0),
                "raw_unsafe_ratio": float(unsafe_item.get("raw_unsafe_ratio") or 0.0),
                "required_unsafe_lines": int(unsafe_item.get("required_unsafe_lines") or 0),
                "required_unsafe_ratio": float(unsafe_item.get("required_unsafe_ratio") or 0.0),
            },
        }

    inc_summary = incremental.get("incremental_compilation") or {}
    rq2_summary = rq2.get("summary") or {}
    unsafe_summary = unsafe_report.get("summary") or {}
    warning_summary = warnings.get("summary") or {}
    row_values = list(rows.values())
    micro_incremental_rate = (
        int(inc_summary.get("compiled_functions") or 0) / int(inc_summary.get("restored_functions") or 0)
        if int(inc_summary.get("restored_functions") or 0)
        else 0.0
    )
    micro_test_rate = (
        int(rq2_summary.get("tests_passed") or 0) / int(rq2_summary.get("total_tests") or 0)
        if int(rq2_summary.get("total_tests") or 0)
        else 0.0
    )
    micro_raw_unsafe_rate = (
        int(unsafe_summary.get("raw_unsafe_lines") or 0) / int(unsafe_summary.get("total_lines") or 0)
        if int(unsafe_summary.get("total_lines") or 0)
        else 0.0
    )
    micro_required_unsafe_rate = (
        int(unsafe_summary.get("required_unsafe_lines") or 0) / int(unsafe_summary.get("total_lines") or 0)
        if int(unsafe_summary.get("total_lines") or 0)
        else 0.0
    )
    return {
        "run_name": RUN_NAME,
        "projects": PROJECTS,
        "rows": rows,
        "summary": {
            "metric_policy": "paper_macro_project_average",
            "incremental_compilation": {
                "compiled_functions": int(inc_summary.get("compiled_functions") or 0),
                "restored_functions": int(inc_summary.get("restored_functions") or 0),
                "compile_rate": _mean(
                    [float(((row.get("incremental_compilation") or {}).get("compile_rate") or 0.0)) for row in row_values]
                ),
                "micro_compile_rate": micro_incremental_rate,
            },
            "rq2_tests": {
                "tests_passed": int(rq2_summary.get("tests_passed") or 0),
                "total_tests": int(rq2_summary.get("total_tests") or 0),
                "pass_rate": _mean(
                    [float(((row.get("rq2_tests") or {}).get("pass_rate") or 0.0)) for row in row_values]
                ),
                "micro_pass_rate": micro_test_rate,
            },
            "warnings": {
                "rustc_warning_count": int(warning_summary.get("rustc_warning_count") or 0),
                "clippy_warning_count": int(warning_summary.get("warning_count") or 0),
                "warning_count_total": int(warning_summary.get("warning_count_total") or 0),
            },
            "unsafe": {
                "total_lines": int(unsafe_summary.get("total_lines") or 0),
                "raw_unsafe_lines": int(unsafe_summary.get("raw_unsafe_lines") or 0),
                "raw_unsafe_ratio": _mean(
                    [float(((row.get("unsafe") or {}).get("raw_unsafe_ratio") or 0.0)) for row in row_values]
                ),
                "micro_raw_unsafe_ratio": micro_raw_unsafe_rate,
                "required_unsafe_lines": int(unsafe_summary.get("required_unsafe_lines") or 0),
                "required_unsafe_ratio": _mean(
                    [float(((row.get("unsafe") or {}).get("required_unsafe_ratio") or 0.0)) for row in row_values]
                ),
                "micro_required_unsafe_ratio": micro_required_unsafe_rate,
            },
        },
    }


def _render_summary_markdown(summary: Dict[str, Any]) -> str:
    """生成 OSS8 指标总表 Markdown。"""
    lines = [
        "| project | incremental compile rate | RQ2 test pass rate | warnings | raw unsafe | required unsafe |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    rows = summary.get("rows") or {}
    for project in PROJECTS:
        row = rows.get(project) or {}
        inc = row.get("incremental_compilation") or {}
        rq2 = row.get("rq2_tests") or {}
        warns = row.get("warnings") or {}
        unsafe_item = row.get("unsafe") or {}
        lines.append(
            "| {project} | {inc_rate} | {rq2_rate} | {warnings} | {raw} | {required} |".format(
                project=project,
                inc_rate=_rate_text(int(inc.get("compiled_functions") or 0), int(inc.get("restored_functions") or 0)),
                rq2_rate=_rate_text(int(rq2.get("tests_passed") or 0), int(rq2.get("total_tests") or 0)),
                warnings=int(warns.get("rustc_warning_count") or 0),
                raw=_rate_text(int(unsafe_item.get("raw_unsafe_lines") or 0), int(unsafe_item.get("total_lines") or 0)),
                required=_rate_text(
                    int(unsafe_item.get("required_unsafe_lines") or 0),
                    int(unsafe_item.get("total_lines") or 0),
                ),
            )
        )
    total = summary.get("summary") or {}
    inc_t = total.get("incremental_compilation") or {}
    rq2_t = total.get("rq2_tests") or {}
    warn_t = total.get("warnings") or {}
    unsafe_t = total.get("unsafe") or {}
    lines.append(
        "| **Paper macro average** | {inc_rate} | {rq2_rate} | {warnings} | {raw} | {required} |".format(
            inc_rate=_format_percent(float(inc_t.get("compile_rate") or 0.0)),
            rq2_rate=_format_percent(float(rq2_t.get("pass_rate") or 0.0)),
            warnings=int(warn_t.get("rustc_warning_count") or 0),
            raw=_format_percent(float(unsafe_t.get("raw_unsafe_ratio") or 0.0)),
            required=_format_percent(float(unsafe_t.get("required_unsafe_ratio") or 0.0)),
        )
    )
    lines.extend(
        [
            "",
            "The final row is the paper metric policy: macro average over project-level rates.",
            "warnings 列为 `rustc_warning_count`；`results/warnings.json` 同时保留 clippy warnings 和总 warnings。",
        ]
    )
    return "\n".join(lines) + "\n"


def _write_manifest(archive_root: Path, output_dir: Path, run_root: Path) -> None:
    """记录本次归档复现的输入输出路径。"""
    manifest = {
        "archive_root": str(archive_root.resolve()),
        "output_dir": str(output_dir.resolve()),
        "run_root": str(run_root.resolve()),
        "projects": PROJECTS,
        "tests_dir": str((archive_root / "tests" / "source_rq2_tests").resolve()),
    }
    _write_json(output_dir / "archive_run_manifest.json", manifest)


def parse_args(argv: Optional[Sequence[str]] = None) -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive-root", type=Path, default=ARCHIVE_ROOT, help="归档根目录")
    parser.add_argument("--output-dir", type=Path, required=True, help="复现输出目录")
    parser.add_argument("--rq2-timeout", type=int, default=240, help="单个 RQ2 cargo test 超时秒数")
    parser.add_argument("--incremental-timeout", type=int, default=300, help="单函数增量编译超时秒数")
    parser.add_argument("--warnings-timeout", type=int, default=600, help="单项目 cargo clippy 超时秒数")
    parser.add_argument("--details-limit", type=int, default=500, help="每项目 unsafe findings 样例上限")
    return parser.parse_args(argv)


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    args = parse_args(argv)
    archive_root = args.archive_root.resolve()
    output_dir = args.output_dir.resolve()
    scripts_dir = archive_root / "scripts"
    tests_dir = archive_root / "tests" / "source_rq2_tests"
    if not tests_dir.is_dir():
        raise FileNotFoundError(f"缺少归档 RQ2 测试模板: {tests_dir}")

    output_dir.mkdir(parents=True, exist_ok=True)
    results_dir = output_dir / "results"
    results_dir.mkdir(parents=True, exist_ok=True)
    run_root = _prepare_archived_run_layout(archive_root, output_dir)

    rq2_json = results_dir / "oss8_rq2.json"
    rq2_md = results_dir / "oss8_rq2.md"
    _run(
        [
            sys.executable,
            str(scripts_dir / "run_oss_rq2_rust_tests.py"),
            "--run-dir",
            str(run_root),
            "--projects",
            _project_csv(),
            "--tests-dir",
            str(tests_dir),
            "--timeout",
            str(args.rq2_timeout),
            "--work-dir",
            str(output_dir / "tmp" / "rq2_work"),
            "--output",
            str(rq2_json),
            "--markdown",
            str(rq2_md),
        ],
        cwd=REPO_ROOT,
    )

    incremental_json = results_dir / "incremental_compile.json"
    incremental_md = results_dir / "incremental_compile.md"
    _run(
        [
            sys.executable,
            str(scripts_dir / "verify_incremental_compilation.py"),
            "--run-dir",
            str(run_root),
            "--projects",
            _project_csv(),
            "--timeout",
            str(args.incremental_timeout),
            "--output",
            str(incremental_json),
            "--markdown",
            str(incremental_md),
        ],
        cwd=REPO_ROOT,
    )

    unsafe_json = results_dir / "required_unsafe.json"
    unsafe_md = results_dir / "required_unsafe.md"
    _run(
        [
            sys.executable,
            str(scripts_dir / "analyze_required_unsafe.py"),
            "--run-dir",
            str(run_root),
            "--projects",
            _project_csv(),
            "--details-limit",
            str(args.details_limit),
            "--output",
            str(unsafe_json),
            "--markdown",
            str(unsafe_md),
        ],
        cwd=REPO_ROOT,
    )

    warnings_json = results_dir / "warnings.json"
    warnings_report = _analyze_warnings(archive_root, args.warnings_timeout)
    _write_json(warnings_json, warnings_report)

    summary = _make_summary(
        _load_json(rq2_json),
        _load_json(incremental_json),
        _load_json(unsafe_json),
        warnings_report,
    )
    summary_json = results_dir / "summary.json"
    summary_md = results_dir / "summary.md"
    _write_json(summary_json, summary)
    summary_md.write_text(_render_summary_markdown(summary), encoding="utf-8")
    _write_manifest(archive_root, output_dir, run_root)

    rq2_summary = summary["summary"]["rq2_tests"]
    inc_summary = summary["summary"]["incremental_compilation"]
    print(f"summary: {summary_md}", flush=True)
    print(
        "RQ2 tests: " f"{_format_percent(float(rq2_summary['pass_rate']))}",
        flush=True,
    )
    print(
        "incremental compile: " f"{_format_percent(float(inc_summary['compile_rate']))}",
        flush=True,
    )
    unsafe_summary = summary["summary"]["unsafe"]
    print("raw unsafe: " f"{_format_percent(float(unsafe_summary['raw_unsafe_ratio']))}", flush=True)
    print("required unsafe: " f"{_format_percent(float(unsafe_summary['required_unsafe_ratio']))}", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
