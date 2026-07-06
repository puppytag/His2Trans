#!/usr/bin/env python3
"""Rust-native refactor gate 的闭环测试。"""

from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from scripts.agentic_repair.rust_native_refactor_gate import (
    rust_native_refactor_status,
    write_rust_native_refactor_task,
)


class TestRustNativeRefactorGate(unittest.TestCase):
    """验证统一重构任务包含 unsafe、ABI 与死符号候选。"""

    def test_cli_and_task_json_include_abi_and_dead_symbol_candidates(self) -> None:
        """命令行入口写出统一 review JSON，并要求逐文件写回结论。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            source = root / "source"
            (crate / "src").mkdir(parents=True)
            (source / "include").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text(
                "#[no_mangle]\n"
                "pub unsafe extern \"C\" fn Exported(p: *mut i32) -> i32 {\n"
                "    unsafe {\n"
                "        if p.is_null() { return -1; }\n"
                "        *p += 1;\n"
                "        *p\n"
                "    }\n"
                "}\n"
                "\n"
                "pub extern \"C\" fn InternalCandidate(x: i32) -> i32 { x + 1 }\n"
                "fn dead_helper() -> i32 { 7 }\n",
                encoding="utf-8",
            )
            (source / "include" / "api.h").write_text("int Exported(int *p);\n", encoding="utf-8")

            direct_payload = write_rust_native_refactor_task(
                crate,
                root / "unsafe.json",
                root / "unsafe.md",
                root / "abi.json",
                root / "abi.md",
                root / "review.json",
                source_roots=[source],
            )
            completed = subprocess.run(
                [
                    "python3",
                    "scripts/agentic_repair/rust_native_refactor_gate.py",
                    "--crate-dir",
                    str(crate),
                    "--source-root",
                    str(source),
                    "--unsafe-json",
                    str(root / "cli_unsafe.json"),
                    "--unsafe-md",
                    str(root / "cli_unsafe.md"),
                    "--abi-json",
                    str(root / "cli_abi.json"),
                    "--abi-md",
                    str(root / "cli_abi.md"),
                    "--review-json",
                    str(root / "cli_review.json"),
                ],
                cwd=Path(__file__).resolve().parents[2],
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertEqual(direct_payload["item_count"], 1)
            cli_payload = json.loads(completed.stdout)
            self.assertEqual(cli_payload["item_count"], 1)
            review = json.loads((root / "cli_review.json").read_text(encoding="utf-8"))
            self.assertEqual(review["schema_version"], "c2r_rust_native_refactor_task_v1")
            self.assertEqual(review["gate"], "rust_native_refactor")
            self.assertEqual(review["suite"], "ohos")
            self.assertIn("OHOS integration project", review["unsafe_reduction_goal"]["primary_goal"])
            self.assertNotIn("complete OSS/C project", review["unsafe_reduction_goal"]["primary_goal"])
            self.assertEqual(review["items"][0]["kind"], "rust_native_refactor_file_review")
            self.assertGreater(review["items"][0]["unsafe_total_lines"], 0)
            self.assertTrue(any(item["name"] == "Exported" for item in review["items"][0]["abi_candidates"]))
            self.assertTrue(any(item["name"] == "dead_helper" for item in review["items"][0]["dead_symbol_candidates"]))

            status = rust_native_refactor_status(root / "cli_review.json", crate)
            self.assertFalse(status["ok"])
            self.assertEqual(status["missing_count"], 1)

            for item in review["items"]:
                item["decision"] = "refactored"
                item["reason"] = "unsafe block can be audited at file level and ABI candidate was reviewed"
                item["result"] = "review completed for test fixture"
            (root / "cli_review.json").write_text(json.dumps(review, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            no_reduction_status = rust_native_refactor_status(root / "cli_review.json", crate)
            self.assertFalse(no_reduction_status["ok"])
            self.assertEqual(no_reduction_status["missing_items"][0]["missing_fields"], ["unsafe_reduction_evidence"])

            for item in review["items"]:
                item["decision"] = "kept_required"
                item["reason"] = "raw pointer dereference remains required by exported C ABI in this fixture"
                item["result"] = "review completed without claiming unsafe reduction"
            (root / "cli_review.json").write_text(json.dumps(review, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            self.assertTrue(rust_native_refactor_status(root / "cli_review.json", crate)["ok"])


if __name__ == "__main__":
    unittest.main()
