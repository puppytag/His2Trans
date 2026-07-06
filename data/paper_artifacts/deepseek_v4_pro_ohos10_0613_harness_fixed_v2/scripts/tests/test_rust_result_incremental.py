"""测试 Rust 结果口径的逐函数恢复增量编译率。"""

from __future__ import annotations

import subprocess
import tempfile
import textwrap
import unittest
from pathlib import Path

from ohos_incremental_core import verify_incremental_compilation_from_rust_result


class RustResultIncrementalTest(unittest.TestCase):
    """覆盖最终 Rust 结果自身作为分母的增量编译逻辑。"""

    def test_uses_rust_functions_without_c_manifest(self) -> None:
        """不依赖 C manifest，直接统计翻译结果文件中的函数。"""
        with tempfile.TemporaryDirectory() as td:
            crate = Path(td) / "demo"
            (crate / "src").mkdir(parents=True)
            (crate / "Cargo.toml").write_text(
                textwrap.dedent(
                    """
                    [package]
                    name = "demo"
                    version = "0.1.0"
                    edition = "2021"
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            (crate / "src" / "main.rs").write_text("mod src_demo;\nfn main() {}\n", encoding="utf-8")
            (crate / "src" / "src_demo.rs").write_text(
                textwrap.dedent(
                    """
                    pub fn add_one(x: i32) -> i32 { x + 1 }
                    fn internal(x: i32) -> i32 { add_one(x) * 2 }
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            subprocess.run(["cargo", "check", "--offline"], cwd=crate, check=True, capture_output=True, text=True)

            result = verify_incremental_compilation_from_rust_result(
                crate_dir=crate,
                project_name="demo",
                timeout=60,
                count_sources={"llm", "c2rust_fallback"},
            )

        self.assertIsNone(result["error"])
        self.assertEqual(result["denominator_kind"], "rust_result_function_instances")
        self.assertEqual(result["total_functions"], 2)
        self.assertEqual(result["restored_functions"], 2)
        self.assertEqual(result["compiled_functions"], 2)
        self.assertEqual(result["compile_rate"], 1.0)

    def test_excludes_placeholder_from_restored_denominator(self) -> None:
        """占位函数保留诊断，但不进入逐函数恢复分母。"""
        with tempfile.TemporaryDirectory() as td:
            crate = Path(td) / "demo_placeholder"
            (crate / "src").mkdir(parents=True)
            (crate / "Cargo.toml").write_text(
                textwrap.dedent(
                    """
                    [package]
                    name = "demo_placeholder"
                    version = "0.1.0"
                    edition = "2021"
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            (crate / "src" / "main.rs").write_text("mod src_demo;\nfn main() {}\n", encoding="utf-8")
            (crate / "src" / "src_demo.rs").write_text(
                textwrap.dedent(
                    """
                    pub fn implemented() -> i32 { 1 }
                    pub fn placeholder() -> i32 { unimplemented!() }
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            subprocess.run(["cargo", "check", "--offline"], cwd=crate, check=True, capture_output=True, text=True)

            result = verify_incremental_compilation_from_rust_result(
                crate_dir=crate,
                project_name="demo_placeholder",
                timeout=60,
                count_sources={"llm", "c2rust_fallback"},
            )

        self.assertIsNone(result["error"])
        self.assertEqual(result["total_functions"], 2)
        self.assertEqual(result["restored_functions"], 1)
        self.assertEqual(result["compiled_functions"], 1)
        self.assertEqual(result["unimplemented_functions"], 1)
        self.assertEqual(result["compile_rate"], 1.0)

    def test_ignores_nested_local_functions_when_stubbing(self) -> None:
        """局部函数不作为独立分母，避免嵌套 span 替换切坏源码。"""
        with tempfile.TemporaryDirectory() as td:
            crate = Path(td) / "demo_nested"
            (crate / "src").mkdir(parents=True)
            (crate / "Cargo.toml").write_text(
                textwrap.dedent(
                    """
                    [package]
                    name = "demo_nested"
                    version = "0.1.0"
                    edition = "2021"
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            (crate / "src" / "main.rs").write_text("mod src_demo;\nfn main() {}\n", encoding="utf-8")
            (crate / "src" / "src_demo.rs").write_text(
                textwrap.dedent(
                    """
                    pub fn outer(x: i32) -> i32 {
                        fn inner(y: i32) -> i32 { y + 1 }
                        inner(x)
                    }
                    pub fn sibling(x: i32) -> i32 { x * 2 }
                    """
                ).strip()
                + "\n",
                encoding="utf-8",
            )
            subprocess.run(["cargo", "check", "--offline"], cwd=crate, check=True, capture_output=True, text=True)

            result = verify_incremental_compilation_from_rust_result(
                crate_dir=crate,
                project_name="demo_nested",
                timeout=60,
                count_sources={"llm", "c2rust_fallback"},
            )

        self.assertIsNone(result["error"])
        self.assertEqual(result["total_functions"], 2)
        self.assertEqual(result["restored_functions"], 2)
        self.assertEqual(result["compiled_functions"], 2)
        self.assertEqual(result["compile_rate"], 1.0)


if __name__ == "__main__":
    unittest.main()
