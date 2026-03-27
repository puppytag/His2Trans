import json
import tempfile
import unittest
from pathlib import Path

from scripts.analysis import quality_taxonomy


class QualityTaxonomyTests(unittest.TestCase):
    def test_categorize_warning_code_maps_core_buckets(self) -> None:
        self.assertEqual(
            quality_taxonomy.categorize_warning_code("clippy::not_unsafe_ptr_arg_deref"),
            "ffi_unsafe_idiom",
        )
        self.assertEqual(
            quality_taxonomy.categorize_warning_code("clippy::ptr_offset_with_cast"),
            "suspicious_cast_pointer",
        )
        self.assertEqual(
            quality_taxonomy.categorize_warning_code("unused_assignments"),
            "unused_dead_code",
        )
        self.assertEqual(
            quality_taxonomy.categorize_warning_code("clippy::needless_return"),
            "naming_style",
        )

    def test_extract_warning_codes_prefers_json_output(self) -> None:
        project = {
            "cargo_clippy_result": {
                "output": "\n".join(
                    [
                        json.dumps(
                            {
                                "reason": "compiler-message",
                                "message": {
                                    "level": "warning",
                                    "code": {"code": "clippy::zero_ptr"},
                                },
                            }
                        ),
                        json.dumps(
                            {
                                "reason": "compiler-message",
                                "message": {
                                    "level": "warning",
                                    "code": {"code": "unused_assignments"},
                                },
                            }
                        ),
                        json.dumps(
                            {
                                "reason": "compiler-message",
                                "message": {
                                    "level": "warning",
                                    "code": None,
                                },
                            }
                        ),
                    ]
                )
            }
        }
        counter = quality_taxonomy.extract_warning_codes(project)
        self.assertEqual(counter["clippy::zero_ptr"], 1)
        self.assertEqual(counter["unused_assignments"], 1)
        self.assertEqual(counter["rustc::uncoded_warning"], 1)

    def test_find_unsafe_sites_in_text_finds_block_and_fn(self) -> None:
        content = """
pub unsafe extern "C" fn foo(p: *mut i32) -> i32 {
    (*p) += 1;
    *p
}

fn bar(p: *mut i32) {
    unsafe {
        std::ptr::write_bytes(p, 0, 4);
    }
}
"""
        sites = quality_taxonomy.find_unsafe_sites_in_text(content, Path("sample.rs"))
        self.assertEqual(len(sites), 2)
        self.assertEqual(sites[0].kind, "unsafe_extern")
        self.assertEqual(sites[1].kind, "unsafe_block")

    def test_classify_unsafe_tags_marks_expected_categories(self) -> None:
        ffi_site = quality_taxonomy.UnsafeSite(
            file_path=Path("ffi.rs"),
            line_no=1,
            kind="unsafe_fn",
            text='unsafe extern "C" fn foo(p: *mut core::ffi::c_void) { libc::free(p); }',
            tags=(),
        )
        tags = quality_taxonomy.classify_unsafe_tags(ffi_site)
        self.assertIn("ffi_boundary", tags)
        self.assertIn("manual_memory_ops", tags)

        ptr_site = quality_taxonomy.UnsafeSite(
            file_path=Path("ptr.rs"),
            line_no=1,
            kind="unsafe_block",
            text="unsafe { let x = (*p).field; let q = p.offset(1); let r = p as *mut u8; }",
            tags=(),
        )
        ptr_tags = quality_taxonomy.classify_unsafe_tags(ptr_site)
        self.assertIn("raw_pointer_traversal", ptr_tags)
        self.assertIn("layout_preserving_casts", ptr_tags)

    def test_summarize_warning_taxonomy_aggregates_categories(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            path = Path(td) / "sample.json"
            payload = {
                "projects": {
                    "buffer": {
                        "cargo_clippy_result": {
                            "warnings_by_type": {
                                "clippy::not_unsafe_ptr_arg_deref": 3,
                                "clippy::zero_ptr": 2,
                                "unused_assignments": 1,
                            }
                        }
                    }
                }
            }
            path.write_text(json.dumps(payload), encoding="utf-8")
            summary = quality_taxonomy.summarize_warning_taxonomy(
                [
                    quality_taxonomy.DatasetSpec(
                        method="Demo",
                        rq="rq2",
                        kind="structured",
                        path=path,
                    )
                ]
            )[0]
            self.assertEqual(summary.total_warnings, 6)
            self.assertEqual(summary.category_counts["ffi_unsafe_idiom"], 3)
            self.assertEqual(summary.category_counts["suspicious_cast_pointer"], 2)
            self.assertEqual(summary.category_counts["unused_dead_code"], 1)

    def test_summarize_warning_taxonomy_reads_project_json_dir(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            (root / "a.json").write_text(
                json.dumps(
                    {
                        "projects": {
                            "buffer": {
                                "cargo_clippy_result": {
                                    "warnings_by_type": {
                                        "clippy::zero_ptr": 2,
                                    }
                                }
                            }
                        }
                    }
                ),
                encoding="utf-8",
            )
            (root / "b.json").write_text(
                json.dumps(
                    {
                        "projects": {
                            "ht": {
                                "cargo_clippy_result": {
                                    "warnings_by_type": {
                                        "clippy::not_unsafe_ptr_arg_deref": 1,
                                    }
                                }
                            }
                        }
                    }
                ),
                encoding="utf-8",
            )
            summary = quality_taxonomy.summarize_warning_taxonomy(
                [
                    quality_taxonomy.DatasetSpec(
                        method="DemoDir",
                        rq="rq2",
                        kind="project_json_dir",
                        path=root,
                    )
                ]
            )[0]
            self.assertEqual(summary.total_warnings, 3)
            self.assertEqual(summary.category_counts["suspicious_cast_pointer"], 2)
            self.assertEqual(summary.category_counts["ffi_unsafe_idiom"], 1)


if __name__ == "__main__":
    unittest.main()
