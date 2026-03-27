import unittest

from scripts.analysis import paper_metrics
from scripts import export_current_plot_metrics


class PaperMetricTests(unittest.TestCase):
    def test_weighted_incremental_uses_total_functions(self) -> None:
        projects = {
            "p1": {"incremental_compilation": {"compiled_functions": 1, "total_functions": 1}},
            "p2": {"incremental_compilation": {"compiled_functions": 0, "total_functions": 99}},
        }
        self.assertEqual(paper_metrics.weighted_incremental_from_projects(projects), 1.0)

    def test_weighted_ohos_fc_prefers_expected_tests(self) -> None:
        projects = {
            "p1": {"ohos_unit_tests": {"tests_passed": 3, "expected_tests": 5, "total_tests": 100}},
            "p2": {"ohos_unit_tests": {"tests_passed": 1, "expected_tests": 5, "total_tests": 100}},
        }
        self.assertEqual(
            paper_metrics.weighted_fc_from_projects(projects, "ohos_unit_tests", ohos=True),
            40.0,
        )

    def test_warning_total_ignores_failed_compile(self) -> None:
        self.assertIsNone(
            paper_metrics.warning_total(
                {"executed": True, "warning_count_total": 8, "compilation_succeeded": False},
                compile_ok=True,
            )
        )
        self.assertIsNone(
            paper_metrics.warning_total(
                {"executed": True, "warning_count_total": 8},
                compile_ok=False,
            )
        )

    def test_compare_metric_tables_reports_only_real_diffs(self) -> None:
        actual = {
            "rq1": {
                "MethodA": {"ICompRate": 87.85, "FC": 75.0},
            }
        }
        reference = {
            "rq1": {
                "MethodA": {"ICompRate": 87.85, "FC": 74.0},
            }
        }
        diffs = paper_metrics.compare_metric_tables(actual, reference)
        self.assertEqual(list(diffs.keys()), ["rq1"])
        self.assertEqual(list(diffs["rq1"].keys()), ["MethodA"])
        self.assertEqual(list(diffs["rq1"]["MethodA"].keys()), ["FC"])

    def test_render_markdown_tables_outputs_paper_style_sections(self) -> None:
        tables = {
            "rq1": {
                "columns": ["Method", "ICompRate", "FC", "Unsafe", "Clippy"],
                "rows": [{"Method": "M1", "ICompRate": "87.85", "FC": "75.00", "Unsafe": "32.15", "Clippy": "464.00"}],
            },
            "rq2": {
                "columns": ["Method", "ICompRate", "FC", "Unsafe", "Clippy"],
                "rows": [{"Method": "M2", "ICompRate": "62.96", "FC": "46.43", "Unsafe": "37.28", "Clippy": "136.44"}],
            },
            "rq3": {
                "columns": ["Config ID", "ICompRate", "FC", "AvgRepair"],
                "rows": [{"Config ID": "Base-1Shot", "ICompRate": "42.50", "FC": "100.00", "AvgRepair": "0.00"}],
            },
            "rq4": {
                "columns": ["Setting", "ICompRate", "FC", "AvgRepair"],
                "rows": [{"Setting": "Base KB only", "ICompRate": "41.28", "FC": "37.50", "AvgRepair": "1.55"}],
            },
        }
        text = export_current_plot_metrics.render_markdown_tables(tables)
        self.assertIn("## RQ1: OHOS test5", text)
        self.assertIn("## RQ2: test_module", text)
        self.assertIn("## RQ3: Ablation Study", text)
        self.assertIn("## RQ4: Knowledge Base Ablation", text)
        self.assertIn("| Method | ICompRate | FC | Unsafe | Clippy |", text)
        self.assertIn("| Config ID | ICompRate | FC | AvgRepair |", text)
        self.assertIn("| Base KB only | 41.28 | 37.50 | 1.55 |", text)

    def test_rq2_summary_metrics_ignores_projects_outside_current_rq2_set(self) -> None:
        report = {
            "projects": {
                "ht": {
                    "incremental_compilation": {"compiled_functions": 2, "total_functions": 2, "compile_rate": 1.0},
                    "c2r_test_result": {"tests_passed": 3, "total_tests": 3, "pass_rate": 1.0, "executed": True},
                    "unsafe_analysis": {"unsafe_total_ratio": 0.1, "code_lines": 10, "unsafe_total_lines": 1},
                    "cargo_clippy_result": {"executed": True, "warning_count_total": 5, "compilation_succeeded": True},
                    "cargo_check_passed": True,
                },
                "legacy_project": {
                    "incremental_compilation": {"compiled_functions": 0, "total_functions": 100, "compile_rate": 0.0},
                    "c2r_test_result": {"tests_passed": 0, "total_tests": 1, "pass_rate": 0.0, "executed": True},
                    "unsafe_analysis": {"unsafe_total_ratio": 0.9, "code_lines": 10, "unsafe_total_lines": 9},
                    "cargo_clippy_result": {"executed": True, "warning_count_total": 500, "compilation_succeeded": True},
                    "cargo_check_passed": True,
                },
            }
        }
        row = paper_metrics.rq2_summary_metrics(report)
        self.assertEqual(row["ICompRate"], 100.0)
        self.assertEqual(row["FC"], 100.0)
        self.assertEqual(row["Unsafe"], 10.0)
        self.assertEqual(row["Clippy"], 5.0)

    def test_reference_tables_default_to_repo_local_snapshot(self) -> None:
        self.assertEqual(
            export_current_plot_metrics.DEFAULT_REFERENCE_OUT_DIR,
            export_current_plot_metrics.REPO_ROOT / "data" / "paper_metric_exports" / "reference_tables",
        )

    def test_current_generated_json_matches_local_reference_tables(self) -> None:
        actual = export_current_plot_metrics.compute_actual_metrics(
            export_current_plot_metrics.DEFAULT_STRUCTURED_DIR
        )
        reference = export_current_plot_metrics.load_reference_metrics(
            export_current_plot_metrics.DEFAULT_REFERENCE_OUT_DIR
        )
        self.assertEqual(paper_metrics.compare_metric_tables(actual, reference), {})

    def test_rq2_build_scripts_use_repo_relative_include_paths(self) -> None:
        build_scripts = sorted(
            (export_current_plot_metrics.REPO_ROOT / "data" / "rq2").rglob("build.rs")
        )
        self.assertTrue(build_scripts)
        for path in build_scripts:
            text = path.read_text(encoding="utf-8")
            if 'native/c2r_accessors.c' not in text:
                continue
            self.assertNotIn("/data/home/wangshb/His2Trans", text)
            self.assertIn('manifest_dir.join("native/include")', text)


if __name__ == "__main__":
    unittest.main()
