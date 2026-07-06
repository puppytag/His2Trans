"""测试 OHOS 论文指标总表的合并口径。"""

import json
from pathlib import Path

import summarize_ohos_metrics_table as sm


def _write_json(path: Path, data) -> Path:
    """写入测试 JSON。"""
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_summary_uses_incremental_json_not_analyzer_incremental(tmp_path: Path) -> None:
    """增量编译率必须来自专门复算脚本，而不是旧 analyzer 字段。"""
    project = "host__25c1898e1626"
    incremental_json = _write_json(
        tmp_path / "incremental.json",
        {
            "projects": {
                project: {
                    "compiled_functions": 2,
                    "restored_functions": 2,
                    "compile_rate": 1.0,
                }
            }
        },
    )
    unsafe_json = _write_json(
        tmp_path / "unsafe.json",
        {
            "projects": {
                project: {
                    "total_lines": 10,
                    "raw_unsafe_lines": 4,
                    "required_unsafe_lines": 2,
                }
            }
        },
    )
    gtest_dir = tmp_path / "gtest"
    gtest_dir.mkdir()
    _write_json(
        gtest_dir / "summary.json",
        {
            "results": [
                {
                    "project": project,
                    "compiled": True,
                    "tests_passed": 3,
                    "tests_total": 4,
                }
            ]
        },
    )
    analysis_json = _write_json(
        tmp_path / "analysis.json",
        {
            "projects": {
                project: {
                    "incremental_compilation": {
                        "compiled_functions": 1,
                        "total_functions": 2,
                        "compile_rate": 0.5,
                    },
                    "cargo_check": {"passed": True},
                    "clippy": {"warning_count_total": 7},
                }
            }
        },
    )

    report = sm.summarize_metrics(
        run_dir=tmp_path,
        projects=[project],
        incremental_json=incremental_json,
        unsafe_json=unsafe_json,
        gtest_dir=gtest_dir,
        analysis_json=analysis_json,
    )

    row = report["projects"][0]
    assert row["incremental_compiled"] == 2
    assert row["incremental_restored"] == 2
    assert row["incremental_rate"] == 1.0
    assert row["warnings"] == 7
    assert row["compile_ok"] is True
    assert "| host | 2/2=100.00% | 3/4=75.00% | 7 | 是 |" in sm.render_markdown(report)


def test_summary_default_uses_paper_macro_average(tmp_path: Path) -> None:
    """总表默认展示论文宏平均，不展示原始总分子分母比例。"""
    projects = ["host__25c1898e1626", "osal__0bc4f21396ad"]
    incremental_json = _write_json(
        tmp_path / "incremental.json",
        {
            "projects": {
                projects[0]: {"compiled_functions": 1, "restored_functions": 1},
                projects[1]: {"compiled_functions": 1, "restored_functions": 1},
            }
        },
    )
    unsafe_json = _write_json(
        tmp_path / "unsafe.json",
        {
            "projects": {
                projects[0]: {"total_lines": 10, "raw_unsafe_lines": 1, "required_unsafe_lines": 1},
                projects[1]: {"total_lines": 90, "raw_unsafe_lines": 45, "required_unsafe_lines": 9},
            }
        },
    )
    gtest_dir = tmp_path / "gtest"
    gtest_dir.mkdir()
    _write_json(
        gtest_dir / "summary.json",
        {
            "results": [
                {"project": projects[0], "tests_passed": 1, "tests_total": 1},
                {"project": projects[1], "tests_passed": 0, "tests_total": 9},
            ]
        },
    )
    analysis_json = _write_json(
        tmp_path / "analysis.json",
        {
            "projects": {
                projects[0]: {"cargo_check": {"passed": True}, "clippy": {"warning_count_total": 1}},
                projects[1]: {"cargo_check": {"passed": True}, "clippy": {"warning_count_total": 2}},
            }
        },
    )

    report = sm.summarize_metrics(
        run_dir=tmp_path,
        projects=projects,
        incremental_json=incremental_json,
        unsafe_json=unsafe_json,
        gtest_dir=gtest_dir,
        analysis_json=analysis_json,
    )

    summary = report["summary"]
    assert summary["metric_policy"] == "paper_macro_project_average"
    assert summary["gtest_pass_rate"] == 0.5
    assert summary["raw_unsafe_rate"] == 0.3
    assert summary["micro_evidence"]["gtest_pass_rate"] == 0.1
    markdown = sm.render_markdown(report)
    assert "| **Paper macro average** | 100.00% | 50.00% | 3 | 2/2 | 30.00% | 10.00% |" in markdown
    assert "| **Paper macro average** | 1/2=50.00%" not in markdown
