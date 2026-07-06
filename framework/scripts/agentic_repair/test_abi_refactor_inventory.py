#!/usr/bin/env python3
"""ABI refactor inventory 的闭环测试。"""

from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from scripts.agentic_repair.abi_refactor_inventory import (
    _rust_native_refactor_policy,
    build_abi_refactor_inventory,
    run_abi_refactor_inventory,
)


def _by_name(payload: dict[str, object]) -> dict[str, dict[str, object]]:
    """按函数名索引 inventory 条目。"""
    return {str(item["name"]): item for item in payload["functions"]}  # type: ignore[index]


class TestAbiRefactorInventory(unittest.TestCase):
    """验证 ABI 保留和 Rust-native 重构候选分类。"""

    def test_ohos_policy_preserves_platform_integration(self) -> None:
        """OHOS 策略不能误用独立 OSS 项目的 raw-unsafe 目标。"""
        policy = _rust_native_refactor_policy("ohos")
        joined = "\n".join(policy["preserve"] + policy["may_rewrite"] + policy["preferred_internal_rewrites"])

        self.assertIn("OHOS", joined)
        self.assertIn("public ABI", joined)
        self.assertNotIn("complete OSS/C", joined)
        self.assertNotIn("raw unsafe = 0 or near 0", joined)

    def test_classifies_public_abi_internal_helpers_callbacks_and_placeholders(self) -> None:
        """基于 Rust/C 真实临时文件分类 ABI 决策。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            src = crate / "src"
            src.mkdir(parents=True)
            (src / "lib.rs").write_text(
                "#[no_mangle]\n"
                "pub unsafe extern \"C\" fn ExportedNoMangle(p: *mut i32) -> i32 {\n"
                "    unsafe { *p }\n"
                "}\n"
                "\n"
                "pub extern \"C\" fn HeaderApi(x: i32) -> i32 { x + 1 }\n"
                "pub extern \"C\" fn DynamicApi(x: i32) -> i32 { x + 2 }\n"
                "pub extern \"C\" fn StaticHelper(x: i32) -> i32 { x + 3 }\n"
                "pub extern \"C\" fn CallbackTarget(x: i32) -> i32 { x + 4 }\n"
                "pub extern \"C\" fn InternalHelper(x: i32) -> i32 { x + 5 }\n"
                "pub extern \"C\" fn ReviewMe() -> i32 { 0 }\n"
                "pub extern \"C\" fn PlaceholderFunc() -> i32 {\n"
                "    panic!(\"stub fallback\");\n"
                "}\n"
                "fn OrdinaryRust() -> i32 { 7 }\n"
                "\n"
                "pub fn use_internal() -> i32 {\n"
                "    InternalHelper(1) + OrdinaryRust()\n"
                "}\n"
                "\n"
                "pub fn register_callback() {\n"
                "    let _cb = Some(CallbackTarget as unsafe extern \"C\" fn(i32) -> i32);\n"
                "}\n",
                encoding="utf-8",
            )
            (crate / ".c2r_bindgen_extern").mkdir()
            (crate / ".c2r_bindgen_extern" / "ignored.rs").write_text(
                "#[no_mangle]\npub extern \"C\" fn IgnoredGenerated() {}\n",
                encoding="utf-8",
            )
            source = root / "source"
            (source / "include").mkdir(parents=True)
            (source / "src").mkdir()
            (source / "tests").mkdir()
            (source / "include" / "api.h").write_text(
                "int HeaderApi(int x);\n",
                encoding="utf-8",
            )
            (source / "src" / "helpers.c").write_text(
                "static int StaticHelper(int x) { return x + 3; }\n"
                "struct Ops { int (*cb)(int); };\n"
                "void setup(struct Ops *ops) { ops->cb = CallbackTarget; }\n",
                encoding="utf-8",
            )
            (source / "tests" / "api_test.c").write_text(
                "void test_api(void) { HeaderApi(1); }\n"
                "void *load(void *handle) { return dlsym(handle, \"DynamicApi\"); }\n",
                encoding="utf-8",
            )

            payload = build_abi_refactor_inventory(crate, [source])
            items = _by_name(payload)

            self.assertEqual(payload["summary"]["function_count"], 11)
            self.assertNotIn("IgnoredGenerated", items)
            self.assertEqual(items["ExportedNoMangle"]["decision"], "keep_c_abi")
            self.assertEqual(items["HeaderApi"]["decision"], "keep_c_abi")
            self.assertEqual(items["DynamicApi"]["decision"], "keep_c_abi")
            self.assertEqual(items["StaticHelper"]["decision"], "rust_internal_candidate")
            self.assertEqual(items["CallbackTarget"]["decision"], "extern_thunk_to_safe_core")
            self.assertEqual(items["InternalHelper"]["decision"], "rust_internal_candidate")
            self.assertEqual(items["ReviewMe"]["decision"], "review_required")
            self.assertEqual(items["OrdinaryRust"]["decision"], "not_extern_c")
            policy = payload["rust_native_refactor_policy"]  # type: ignore[index]
            self.assertIn("public ABI, exported symbols, and extern calling conventions", policy["preserve"])  # type: ignore[index]
            self.assertIn("private/internal data structures and state representation", policy["may_rewrite"])  # type: ignore[index]
            self.assertIn("no need to preserve the C internal shape", items["StaticHelper"]["recommended_action"])
            self.assertIn("rewrite internal implementation freely", items["ReviewMe"]["recommended_action"])
            self.assertGreaterEqual(payload["summary"]["placeholder_hit_count"], 1)
            self.assertTrue(items["HeaderApi"]["evidence"]["header_declarations"])  # type: ignore[index]
            self.assertTrue(items["DynamicApi"]["evidence"]["dlsym_or_symbol_refs"])  # type: ignore[index]
            self.assertTrue(items["CallbackTarget"]["evidence"]["callback_or_vtable_refs"])  # type: ignore[index]
            self.assertEqual(payload["summary"]["by_decision"]["keep_c_abi"], 3)  # type: ignore[index]

    def test_cli_and_markdown_outputs_are_usable(self) -> None:
        """命令行入口写出 JSON 与 Markdown。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text(
                "#[no_mangle]\npub extern \"C\" fn Exported() {}\n",
                encoding="utf-8",
            )
            output_json = root / "abi.json"
            output_md = root / "abi.md"

            payload = run_abi_refactor_inventory(crate, output_json, output_md)
            completed = subprocess.run(
                [
                    "python3",
                    "scripts/agentic_repair/abi_refactor_inventory.py",
                    "--crate-dir",
                    str(crate),
                    "--output-json",
                    str(root / "cli_abi.json"),
                    "--output-md",
                    str(root / "cli_abi.md"),
                ],
                cwd=Path(__file__).resolve().parents[2],
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(payload["status"], "available")
            self.assertEqual(payload["summary"]["function_count"], 1)
            self.assertTrue(output_json.is_file())
            self.assertTrue(output_md.is_file())
            self.assertIn("ABI Refactor Inventory", output_md.read_text(encoding="utf-8"))
            self.assertIn("Rust-native Refactor Policy", output_md.read_text(encoding="utf-8"))
            self.assertIn("Do not preserve C internal structure one-to-one", output_md.read_text(encoding="utf-8"))
            self.assertEqual(completed.returncode, 0, completed.stderr)
            cli_payload = json.loads(completed.stdout)
            self.assertEqual(cli_payload["summary"]["function_count"], 1)
            self.assertTrue((root / "cli_abi.json").is_file())
            self.assertTrue((root / "cli_abi.md").is_file())

    def test_support_files_are_reported_separately_and_method_calls_are_not_callbacks(self) -> None:
        """支撑层函数不进入主重构清单，方法调用不误判为 callback。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "compat.rs").write_text(
                "pub extern \"C\" fn SupportAbi() {}\n",
                encoding="utf-8",
            )
            (crate / "src" / "types.rs").write_text(
                "impl Demo {\n"
                "    pub fn as_ptr(&self) -> *const i32 { core::ptr::null() }\n"
                "}\n"
                "pub struct Demo;\n",
                encoding="utf-8",
            )
            (crate / "src" / "src_demo.rs").write_text(
                "pub extern \"C\" fn RealTranslated() -> i32 {\n"
                "    let demo = crate::types::Demo;\n"
                "    let _ptr = demo.as_ptr() as *const i32;\n"
                "    0\n"
                "}\n",
                encoding="utf-8",
            )

            payload = build_abi_refactor_inventory(crate)
            items = _by_name(payload)

            self.assertEqual(payload["summary"]["function_count"], 1)
            self.assertEqual(payload["summary"]["support_function_count"], 2)
            self.assertIn("RealTranslated", items)
            self.assertNotIn("SupportAbi", items)
            self.assertNotIn("as_ptr", items)
            self.assertEqual(items["RealTranslated"]["decision"], "review_required")
            self.assertEqual(payload["summary"]["by_decision"], {"review_required": 1})

    def test_dataset_root_with_test_does_not_mark_all_sources_as_tests(self) -> None:
        """source root 上层含 with_test 时，普通 src 文件不能变成 test call 证据。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "src_demo.rs").write_text(
                "pub extern \"C\" fn NormalSourceFn() -> i32 { 1 }\n"
                "pub extern \"C\" fn RealTestFn() -> i32 { 2 }\n",
                encoding="utf-8",
            )
            source = root / "with_test" / "module"
            (source / "src").mkdir(parents=True)
            (source / "tests").mkdir()
            (source / "src" / "demo.c").write_text(
                "int NormalSourceFn(void) { return 1; }\n",
                encoding="utf-8",
            )
            (source / "tests" / "demo_test.c").write_text(
                "void test_demo(void) { RealTestFn(); }\n",
                encoding="utf-8",
            )

            payload = build_abi_refactor_inventory(crate, [source])
            items = _by_name(payload)

            self.assertEqual(items["NormalSourceFn"]["decision"], "review_required")
            self.assertFalse(items["NormalSourceFn"]["evidence"]["c_test_calls"])  # type: ignore[index]
            self.assertEqual(items["RealTestFn"]["decision"], "keep_c_abi")
            self.assertTrue(items["RealTestFn"]["evidence"]["c_test_calls"])  # type: ignore[index]

    def test_oss_header_and_source_tests_are_not_required_c_abi(self) -> None:
        """OSS suite 下 C header/test 只是语义证据，不强制保留 extern C ABI。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text(
                "pub extern \"C\" fn HeaderApi(x: i32) -> i32 { x + 1 }\n"
                "pub extern \"C\" fn RealTestFn(x: i32) -> i32 { x + 2 }\n"
                "pub extern \"C\" fn DynamicApi(x: i32) -> i32 { x + 3 }\n",
                encoding="utf-8",
            )
            source = root / "source"
            (source / "include").mkdir(parents=True)
            (source / "tests").mkdir()
            (source / "include" / "api.h").write_text(
                "int HeaderApi(int x);\n"
                "int DynamicApi(int x);\n",
                encoding="utf-8",
            )
            (source / "tests" / "api_test.c").write_text(
                "void test_api(void) { HeaderApi(1); RealTestFn(2); }\n"
                "void *load(void *handle) { return dlsym(handle, \"DynamicApi\"); }\n",
                encoding="utf-8",
            )

            ohos_payload = build_abi_refactor_inventory(crate, [source], suite="ohos")
            oss_payload = build_abi_refactor_inventory(crate, [source], suite="oss")
            ohos_items = _by_name(ohos_payload)
            oss_items = _by_name(oss_payload)

            self.assertEqual(ohos_items["HeaderApi"]["decision"], "keep_c_abi")
            self.assertEqual(ohos_items["RealTestFn"]["decision"], "keep_c_abi")
            self.assertEqual(oss_items["HeaderApi"]["decision"], "rust_internal_candidate")
            self.assertEqual(oss_items["RealTestFn"]["decision"], "rust_internal_candidate")
            self.assertEqual(oss_items["DynamicApi"]["decision"], "keep_c_abi")
            self.assertEqual(oss_payload["suite"], "oss")
            self.assertTrue(
                any("C external ABI shape" in item for item in oss_payload["rust_native_refactor_policy"]["may_rewrite"])  # type: ignore[index]
            )
            self.assertTrue(oss_items["HeaderApi"]["evidence"]["header_declarations"])  # type: ignore[index]
            self.assertTrue(oss_items["RealTestFn"]["evidence"]["c_test_calls"])  # type: ignore[index]

    def test_oss_plain_rust_api_is_not_pulled_back_to_c_abi_review(self) -> None:
        """OSS suite 下已是普通 Rust API 的函数不因原 C external linkage 进入 ABI review。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            (crate / "src").mkdir(parents=True)
            (crate / "src" / "lib.rs").write_text(
                "pub fn quickSort(arr: &mut [i32], low: i32, high: i32) { let _ = (arr, low, high); }\n",
                encoding="utf-8",
            )
            source = root / "source"
            (source / "src").mkdir(parents=True)
            (source / "src" / "qsort.c").write_text(
                "void quickSort(int arr[], int low, int high) {}\n",
                encoding="utf-8",
            )

            ohos_payload = build_abi_refactor_inventory(crate, [source], suite="ohos")
            oss_payload = build_abi_refactor_inventory(crate, [source], suite="oss")
            ohos_items = _by_name(ohos_payload)
            oss_items = _by_name(oss_payload)

            self.assertEqual(ohos_items["quickSort"]["decision"], "review_required")
            self.assertEqual(oss_items["quickSort"]["decision"], "not_extern_c")
            self.assertIn("ordinary Rust", oss_items["quickSort"]["reasons"][0])  # type: ignore[index]


if __name__ == "__main__":
    unittest.main()
