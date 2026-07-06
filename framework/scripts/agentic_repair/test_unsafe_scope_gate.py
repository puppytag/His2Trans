#!/usr/bin/env python3
"""unsafe scope 信息 gate 的闭环测试。"""

from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from scripts.agentic_repair.unsafe_scope_gate import analyze_unsafe_scopes, render_markdown, run_unsafe_scope_gate, unsafe_review_status, write_unsafe_review_task


class TestUnsafeScopeGate(unittest.TestCase):
    """验证 unsafe scope gate 只产出索引信息，不贴源码。"""

    def test_scans_all_unsafe_scopes_and_ignores_comments_strings(self) -> None:
        """扫描所有 unsafe scope，同时忽略注释和字符串中的 unsafe。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            src = root / "src"
            src.mkdir()
            (src / "lib.rs").write_text(
                "pub fn safe_text() {\n"
                "    let _s = \"unsafe { fake(); }\";\n"
                "    // unsafe { ignored(); }\n"
                "}\n"
                "pub unsafe fn raw_fn(p: *const i32) -> i32 {\n"
                "    unsafe { *p }\n"
                "}\n"
                "unsafe extern \"C\" {\n"
                "    pub fn ffi_fn(p: *mut i32);\n"
                "}\n"
                "unsafe impl Send for Demo {}\n"
                "pub struct Demo;\n",
                encoding="utf-8",
            )
            (root / "target").mkdir()
            (root / "target" / "ignored.rs").write_text("pub fn x() { unsafe { x(); } }\n", encoding="utf-8")

            report = analyze_unsafe_scopes(root)
            kinds = [item["kind"] for item in report["scopes"]]

            self.assertEqual(report["summary"]["scope_count"], 4)
            self.assertEqual(kinds, ["unsafe_fn", "unsafe_block", "unsafe_extern", "unsafe_impl"])
            self.assertEqual({item["file"] for item in report["scopes"]}, {"src/lib.rs"})
            self.assertTrue(all(Path(item["abs_file"]).is_absolute() for item in report["scopes"]))
            self.assertNotIn("target/ignored.rs", json.dumps(report, ensure_ascii=False))
            self.assertGreater(report["summary"]["unsafe_total_lines"], 0)
            self.assertLess(report["summary"]["unsafe_total_lines"], report["summary"]["code_lines"])
            self.assertEqual(report["summary"]["unsafe_total_ratio"], round(report["summary"]["unsafe_total_lines"] / report["summary"]["total_lines"], 6))

            markdown = render_markdown(report)
            self.assertIn("U0001", markdown)
            self.assertIn("U0004", markdown)
            self.assertIn(str((src / "lib.rs").resolve()), markdown)
            self.assertNotIn("ignored();", markdown)
            self.assertNotIn("*p", markdown)

    def test_cli_writes_json_and_markdown(self) -> None:
        """命令行入口写出 JSON 和精简 Markdown。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_json = root / "unsafe.json"
            output_md = root / "unsafe.md"

            payload = run_unsafe_scope_gate(crate, output_json, output_md)
            completed = subprocess.run(
                [
                    "python3",
                    "scripts/agentic_repair/unsafe_scope_gate.py",
                    "--crate-dir",
                    str(crate),
                    "--output-json",
                    str(root / "cli_unsafe.json"),
                    "--output-md",
                    str(root / "cli_unsafe.md"),
                ],
                cwd=Path(__file__).resolve().parents[2],
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(payload["status"], "available")
            self.assertEqual(payload["summary"]["scope_count"], 1)
            self.assertTrue(output_json.is_file())
            self.assertTrue(output_md.is_file())
            self.assertEqual(completed.returncode, 0, completed.stderr)
            cli_payload = json.loads(completed.stdout)
            self.assertEqual(cli_payload["summary"]["scope_count"], 1)
            self.assertTrue((root / "cli_unsafe.json").is_file())
            self.assertTrue((root / "cli_unsafe.md").is_file())

    def test_excludes_generated_ffi_dirs_and_reviews_by_file(self) -> None:
        """排除纯生成 FFI/TU 目录，review 以可编辑文件为粒度。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src" / "__c2r_generated").mkdir(parents=True)
            (crate / ".c2r_bindgen_extern").mkdir()
            (crate / ".c2r_bindgen_fns").mkdir()
            (crate / "src" / "lib.rs").write_text(
                "pub fn a(p: *const i32) -> i32 { unsafe { *p } }\n"
                "pub fn b(p: *const i32) -> i32 { unsafe { *p + 1 } }\n",
                encoding="utf-8",
            )
            (crate / ".c2r_bindgen_extern" / "ffi.rs").write_text(
                'unsafe extern "C" { pub fn ffi_fn(p: *mut i32); }\n',
                encoding="utf-8",
            )
            (crate / ".c2r_bindgen_fns" / "fns.rs").write_text(
                'unsafe extern "C" { pub fn translated_fn(); }\n',
                encoding="utf-8",
            )
            (crate / "src" / "__c2r_generated" / "types.rs").write_text(
                "pub type Callback = Option<unsafe extern \"C\" fn(*mut i32)>;\n",
                encoding="utf-8",
            )
            review = root / "review.json"

            report = analyze_unsafe_scopes(crate)
            write_unsafe_review_task(crate, root / "unsafe.json", root / "unsafe.md", review)
            review_payload = json.loads(review.read_text(encoding="utf-8"))

            self.assertEqual({item["file"] for item in report["scopes"]}, {"src/lib.rs"})
            self.assertEqual(report["summary"]["scope_count"], 2)
            self.assertEqual(review_payload["excluded_generated_dirs"], [".c2r_bindgen_extern", ".c2r_bindgen_fns", "__c2r_generated"])
            self.assertEqual(len(review_payload["items"]), 1)
            item = review_payload["items"][0]
            self.assertEqual(item["kind"], "unsafe_file_review")
            self.assertEqual(item["file"], "src/lib.rs")
            self.assertEqual(item["scope_count"], 2)
            self.assertEqual(len(item["top_scopes"]), 2)
            self.assertTrue(all(not str(scope["file"]).startswith(".c2r_bindgen_") for scope in report["scopes"]))
            self.assertTrue(all(not str(review_item["file"]).startswith(".c2r_bindgen_") for review_item in review_payload["items"]))

    def test_unsafe_rate_excludes_structural_block_lines(self) -> None:
        """unsafe 率分子不统计纯 unsafe block 起始行和括号行。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "\n"
                "pub fn read_value(p: *const i32) -> i32 {\n"
                "    unsafe {\n"
                "        *p\n"
                "    }\n"
                "}\n",
                encoding="utf-8",
            )

            report = analyze_unsafe_scopes(root)

            self.assertEqual(report["summary"]["total_lines"], 6)
            self.assertEqual(report["summary"]["unsafe_keyword_lines"], 0)
            self.assertEqual(report["summary"]["unsafe_context_lines"], 1)
            self.assertEqual(report["summary"]["unsafe_total_lines"], 1)
            self.assertEqual(report["summary"]["unsafe_total_ratio"], round(1 / 6, 6))
            self.assertEqual(report["by_file"][0]["unsafe_total_lines"], 1)
            self.assertEqual(report["by_file"][0]["total_lines"], 6)

    def test_callback_type_does_not_create_unsafe_scope(self) -> None:
        """函数指针类型里的 unsafe extern fn 不应吞掉后续普通代码。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "pub struct Holder {\n"
                "    pub cb: Option<unsafe extern \"C\" fn(*mut core::ffi::c_void)>,\n"
                "    pub raw: *mut core::ffi::c_void,\n"
                "}\n"
                "\n"
                "pub fn safe_after() -> i32 {\n"
                "    let value = 42;\n"
                "    value\n"
                "}\n",
                encoding="utf-8",
            )

            report = analyze_unsafe_scopes(root)

            self.assertEqual(report["summary"]["scope_count"], 0)
            self.assertEqual(report["summary"]["unsafe_keyword_lines"], 1)
            self.assertEqual(report["summary"]["unsafe_context_lines"], 0)
            self.assertEqual(report["summary"]["unsafe_total_lines"], 1)

    def test_unsafe_review_status_reports_missing_decisions_and_fingerprint(self) -> None:
        """unsafe review 状态报告缺项和源码指纹失配。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            review = root / "review.json"

            write_unsafe_review_task(crate, root / "unsafe.json", root / "unsafe.md", review)
            status = unsafe_review_status(review, crate)

            self.assertFalse(status["ok"])
            self.assertEqual(status["item_count"], 1)
            self.assertEqual(status["missing_count"], 1)
            self.assertEqual(status["missing_items"][0]["missing_fields"], ["decision", "reason", "result"])
            self.assertTrue(status["fingerprint_match"])

            payload = json.loads(review.read_text(encoding="utf-8"))
            payload["items"][0]["decision"] = "kept_required"
            payload["items"][0]["reason"] = "raw pointer dereference is required at the C ABI boundary"
            payload["items"][0]["result"] = "kept unchanged"
            review.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            self.assertTrue(unsafe_review_status(review, crate)["ok"])

            (crate / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } + 1 }\n", encoding="utf-8")
            stale_status = unsafe_review_status(review, crate)
            self.assertFalse(stale_status["ok"])
            self.assertFalse(stale_status["fingerprint_match"])
            self.assertIn("源码指纹", stale_status["diagnostics"][0])


if __name__ == "__main__":
    unittest.main()
