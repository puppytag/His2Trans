#!/usr/bin/env python3
"""从归档的 OHOS10 Rust crate 复跑论文指标。"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence

import ohos_c_gtest_harness as gtest_harness
import run_ohos_c_gtests as gtest_runner


ARCHIVE_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = Path(__file__).resolve().parents[4]
RUN_NAME = "deepseek-v4-pro-ohos10-full-0613-1"
ARCHIVE_RUN_NAME = "archived_deepseek-v4-pro-ohos10-full-0613-1"
GTEST_DIR_NAME = f"ohos_c_gtests_{RUN_NAME}_harness_fixed_v2"
PROJECTS = [
    "host__25c1898e1626",
    "appverify_lite__e5ebe91a98b9",
    "manager__c248934e0221",
    "shared__541f4e547bdb",
    "posix__1b7f59c68bbc",
    "common__89d5ecaafdff",
    "core__ef5242b7ab08",
    "shared__12e38ea922f7",
    "osal__0bc4f21396ad",
    "sapm__193cdeb43a97",
]
OHOS_PROJECT_SOURCE_ROOT = Path(
    os.environ.get("OHOS_PROJECT_SOURCE_ROOT", str(REPO_ROOT / "data" / "ohos" / "source_projects"))
).expanduser()
REPO_PROJECT_SOURCES = {project: OHOS_PROJECT_SOURCE_ROOT / project for project in PROJECTS}
REPO_OHOS_ROOT = Path(os.environ.get("OHOS_ROOT", str(REPO_ROOT / "data" / "ohos" / "ohos_root_min"))).expanduser()


def _run(cmd: Sequence[str], *, cwd: Path) -> None:
    """执行命令，失败时直接暴露原始错误。"""
    print("[run] " + " ".join(cmd), flush=True)
    subprocess.run(list(cmd), cwd=cwd, check=True)


def _copytree_or_link(src: Path, dst: Path) -> None:
    """复制小文件目录；Rust crate 目录使用软链接避免重复落盘。"""
    if dst.exists() or dst.is_symlink():
        if dst.is_symlink() or dst.is_file():
            dst.unlink()
        else:
            shutil.rmtree(dst)
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.symlink_to(src.resolve(), target_is_directory=True)


def _prepare_archived_run_layout(archive_root: Path, output_dir: Path) -> Path:
    """构造原脚本可识别的 raw/framework_output 运行目录布局。"""
    run_root = output_dir / ARCHIVE_RUN_NAME
    framework_root = run_root / "raw" / "framework_output"
    results_root = framework_root / "results"
    results_root.mkdir(parents=True, exist_ok=True)

    analysis_src = archive_root / "results" / "compilation_analysis_ohos_test5.json"
    if not analysis_src.is_file():
        raise FileNotFoundError(f"缺少 warnings 输入: {analysis_src}")
    shutil.copy2(analysis_src, results_root / analysis_src.name)

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
        _copytree_or_link(crate_src, crate_dst)
    return run_root


def _copy_existing_gtest(archive_root: Path, output_dir: Path) -> Path:
    """复制归档中的已验证 gtest 结果，供跳过 gtest 时汇总使用。"""
    src = archive_root / "results" / GTEST_DIR_NAME
    dst = output_dir / "results" / GTEST_DIR_NAME
    if not (src / "summary.json").is_file():
        raise FileNotFoundError(f"缺少归档 gtest summary: {src / 'summary.json'}")
    if dst.exists():
        shutil.rmtree(dst)
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(src, dst)
    return dst


def _write_json(path: Path, data: Dict[str, Any]) -> None:
    """写入 JSON 文件。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8")


def _run_archived_gtests(archive_root: Path, output_dir: Path, timeout: int) -> Path:
    """用归档 Rust crate 和仓库 OHOS C 测试源码运行 gtest。"""
    if not REPO_OHOS_ROOT.is_dir():
        raise FileNotFoundError(f"缺少 OHOS 测试依赖: {REPO_OHOS_ROOT}")
    gtest_harness.OHOS_ROOT = REPO_OHOS_ROOT
    gtest_harness.C_PROJECT_SOURCES.clear()
    gtest_harness.C_PROJECT_SOURCES.update(REPO_PROJECT_SOURCES)
    gtest_runner.C_PROJECT_SOURCES.clear()
    gtest_runner.C_PROJECT_SOURCES.update(REPO_PROJECT_SOURCES)

    gtest_dir = output_dir / "results" / GTEST_DIR_NAME
    gtest_dir.mkdir(parents=True, exist_ok=True)
    results: List[Dict[str, Any]] = []
    for project in PROJECTS:
        print(f"[gtest] {project}", flush=True)
        result = gtest_harness.build_and_run_project(
            project=project,
            crate_dir=archive_root / "projects" / project,
            source_dir=REPO_PROJECT_SOURCES[project],
            ohos_root=REPO_OHOS_ROOT,
            timeout=timeout,
        )
        results.append(result)
        _write_json(gtest_dir / f"{project}.json", result)
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

    summary = gtest_harness.summarize_results(results)
    summary["run_dir"] = ARCHIVE_RUN_NAME
    summary["projects"] = PROJECTS
    summary["output_dir"] = str(gtest_dir)
    _write_json(gtest_dir / "summary.json", {"summary": summary, "results": results})
    (gtest_dir / "summary.md").write_text(gtest_runner._markdown_table(results, summary), encoding="utf-8")
    return gtest_dir


def _write_manifest(archive_root: Path, output_dir: Path, run_root: Path, gtest_dir: Path) -> None:
    """记录本次归档复跑的输入路径。"""
    manifest = {
        "archive_root": str(archive_root.resolve()),
        "output_dir": str(output_dir.resolve()),
        "run_root": str(run_root.resolve()),
        "gtest_dir": str(gtest_dir.resolve()),
        "projects": PROJECTS,
    }
    (output_dir / "archive_run_manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )


def parse_args(argv: Optional[Sequence[str]] = None) -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive-root", type=Path, default=ARCHIVE_ROOT, help="归档根目录")
    parser.add_argument("--output-dir", type=Path, required=True, help="复跑输出目录")
    parser.add_argument("--timeout", type=int, default=900, help="单项目 gtest 编译/运行超时秒数")
    parser.add_argument("--incremental-timeout", type=int, default=300, help="单函数增量编译超时秒数")
    parser.add_argument("--skip-gtest", action="store_true", default=True, help="复用归档中的已验证 gtest 结果")
    parser.add_argument("--run-gtest", action="store_false", dest="skip_gtest", help="重新运行完整 OHOS C gtest")
    parser.add_argument("--skip-incremental", action="store_true", help="复用归档中的已验证增量编译结果")
    parser.add_argument("--skip-unsafe", action="store_true", help="复用归档中的已验证 unsafe 结果")
    return parser.parse_args(argv)


def main(argv: Optional[Sequence[str]] = None) -> int:
    """命令行入口。"""
    args = parse_args(argv)
    archive_root = args.archive_root.resolve()
    output_dir = args.output_dir.resolve()
    scripts_dir = archive_root / "scripts"
    output_dir.mkdir(parents=True, exist_ok=True)

    run_root = _prepare_archived_run_layout(archive_root, output_dir)
    results_dir = output_dir / "results"
    results_dir.mkdir(parents=True, exist_ok=True)

    incremental_json = results_dir / f"incremental_compile_{RUN_NAME}.json"
    incremental_md = incremental_json.with_suffix(".md")
    if args.skip_incremental:
        shutil.copy2(archive_root / "results" / incremental_json.name, incremental_json)
        shutil.copy2(archive_root / "results" / incremental_md.name, incremental_md)
    else:
        _run(
            [
                sys.executable,
                str(scripts_dir / "verify_incremental_compilation.py"),
                "--run-dir",
                str(run_root),
                "--timeout",
                str(args.incremental_timeout),
                "--output",
                str(incremental_json),
                "--markdown",
                str(incremental_md),
            ],
            cwd=REPO_ROOT,
        )

    unsafe_json = results_dir / f"required_unsafe_{RUN_NAME}.json"
    unsafe_md = unsafe_json.with_suffix(".md")
    if args.skip_unsafe:
        shutil.copy2(archive_root / "results" / unsafe_json.name, unsafe_json)
        shutil.copy2(archive_root / "results" / unsafe_md.name, unsafe_md)
    else:
        _run(
            [
                sys.executable,
                str(scripts_dir / "analyze_required_unsafe.py"),
                "--run-dir",
                str(run_root),
                "--output",
                str(unsafe_json),
                "--markdown",
                str(unsafe_md),
            ],
            cwd=REPO_ROOT,
        )

    if args.skip_gtest:
        gtest_dir = _copy_existing_gtest(archive_root, output_dir)
    else:
        gtest_dir = _run_archived_gtests(archive_root, output_dir, args.timeout)

    metrics_json = results_dir / f"ohos_metrics_{RUN_NAME}_harness_fixed_v2.json"
    metrics_md = metrics_json.with_suffix(".md")
    _run(
        [
            sys.executable,
            str(scripts_dir / "summarize_ohos_metrics_table.py"),
            "--run-dir",
            str(run_root),
            "--incremental-json",
            str(incremental_json),
            "--unsafe-json",
            str(unsafe_json),
            "--gtest-dir",
            str(gtest_dir),
            "--analysis-json",
            str(run_root / "raw" / "framework_output" / "results" / "compilation_analysis_ohos_test5.json"),
            "--output",
            str(metrics_json),
            "--markdown",
            str(metrics_md),
        ],
        cwd=REPO_ROOT,
    )

    _write_manifest(archive_root, output_dir, run_root, gtest_dir)
    print(f"metrics: {metrics_md}", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
