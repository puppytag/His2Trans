#!/usr/bin/env python3
"""后置 agentic repair 上下文与 gate bundle 的闭环测试。"""

from __future__ import annotations

import argparse
import contextlib
import io
import json
import subprocess
import sys
import tempfile
import types
import unittest
from pathlib import Path
from unittest import mock

from scripts.agentic_repair.openai_action_runner import OpenAIActionRunner
from scripts.agentic_repair.openai_action_runner import _write_roots
from scripts.agentic_repair.openai_action_runner import SYSTEM_PROMPT as ACTION_RUNNER_SYSTEM_PROMPT
from scripts.agentic_repair.post_repair_agent import (
    DEFAULT_OHOS_ROOT,
    SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT,
    _agent_returncode,
    _apply_post_repair_rule_fixes,
    _blocking_summary,
    _build_body_target_scope,
    _build_repair_context,
    _build_repair_prompt,
    _build_semantic_audit_prompt,
    _build_unsafe_refactor_audit_prompt,
    _cargo_command_with_optional_offline,
    _cargo_env,
    _scan_body_completeness,
    _semantic_audit_passed,
    _semantic_audit_result_executed,
    _skipped_semantic_audit_payload,
    _run_semantic_audit,
    _run_unsafe_refactor_audit,
    _run_suite_rustc_check,
    _write_ohos_rustc_wrapper,
    _write_gate_bundle,
    run_agentic,
)


class TestPostRepairContext(unittest.TestCase):
    """验证 stub 细节通过路径传递，不塞进 agent prompt/context 摘要。"""

    def _value_from_task(self, task: str, prefix: str) -> str:
        """从 agent 任务文本中读取指定前缀后的值。"""
        for line in task.splitlines():
            stripped = line.strip()
            if stripped.startswith(prefix):
                return stripped.split(":", 1)[1].strip()
        self.fail(f"missing task line prefix: {prefix}")

    def _path_from_task(self, task: str, prefix: str) -> str:
        """从 agent 任务文本中读取指定前缀后的路径。"""
        return self._value_from_task(task, prefix)

    def test_unsafe_refactor_audit_prompt_keeps_dynamic_inputs_late(self) -> None:
        """unsafe auditor prompt 稳定规则前置，动态路径/指纹后置。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            manifest = rendered / "Cargo.toml"
            context = root / "agent_context.json"
            ledger = root / "unsafe_refactor_ledger.json"
            report = root / "unsafe_report.md"

            prompt = _build_unsafe_refactor_audit_prompt(
                rendered_root=rendered,
                manifest_path=manifest,
                context_path=context,
                ledger_path=ledger,
                report_path=report,
                expected_fingerprint="abc123",
                feedback_paths=[str(root / "feedback.jsonl")],
            )

        self.assertLess(prompt.index("## Ledger JSON Contract"), prompt.index("## Dynamic Run Inputs"))
        self.assertIn('"source_fingerprint_sha256": "<expected_source_fingerprint_sha256 from Dynamic Run Inputs>"', prompt)
        self.assertIn("expected_source_fingerprint_sha256: abc123", prompt)
        self.assertIn("样例包括但不限于 safe 控制流", prompt)
        self.assertIn("这些只是样例，你必须基于源码自行识别其他可安全外移或重写的内容", prompt)
        self.assertNotIn("top N", prompt)
        self.assertNotIn("accepted 前", prompt)

    def test_oss_unsafe_refactor_prompts_use_standalone_rust_native_context(self) -> None:
        """OSS suite 下 unsafe 后修只传达独立 Rust-native 小项目背景。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            manifest = rendered / "Cargo.toml"
            context = root / "agent_context.json"
            ledger = root / "unsafe_refactor_ledger.json"
            report = root / "unsafe_report.md"

            audit_prompt = _build_unsafe_refactor_audit_prompt(
                rendered_root=rendered,
                manifest_path=manifest,
                context_path=context,
                ledger_path=ledger,
                report_path=report,
                expected_fingerprint="abc123",
                feedback_paths=[],
                suite="oss",
            )
            repair_context = {
                "suite": "oss",
                "information_paths": {
                    "rendered_root": str(rendered),
                    "cargo_manifest": str(manifest),
                    "rust_native_refactor_task_json": str(root / "review.json"),
                    "unsafe_scope_gate_json": str(root / "unsafe.json"),
                    "unsafe_scope_gate_markdown": str(root / "unsafe.md"),
                    "abi_refactor_inventory_json": str(root / "abi.json"),
                    "abi_refactor_inventory_markdown": str(root / "abi.md"),
                    "unsafe_refactor_ledger": str(ledger),
                    "unsafe_refactor_report": str(report),
                    "unsafe_refactor_feedback_jsonl": str(root / "feedback.jsonl"),
                },
                "repair_context_summary": {},
                "semantic_repair_bundle": {},
                "unsafe_refactor_audit": {"open_reducible_item_count": 1},
                "write_scope": {"preferred_edit_targets": ["src/*.rs"]},
            }
            context.write_text(json.dumps(repair_context, ensure_ascii=False), encoding="utf-8")
            repair_prompt = _build_repair_prompt(context)

            self.assertIn("当前项目是独立 OSS/C 小项目", audit_prompt)
            self.assertIn("翻译目标是 Rust-native 独立项目", audit_prompt)
            self.assertIn("C 头文件/源码测试引用本身不是 hard_required 证据", audit_prompt)
            self.assertIn("当前项目是独立 OSS/C 小项目", repair_prompt)
            self.assertIn("翻译目标是 Rust-native 独立项目", repair_prompt)
            self.assertIn("目标是 raw unsafe=0 或接近 0", repair_prompt)
            self.assertNotIn("改变 public ABI", repair_prompt)
            self.assertNotIn("thin extern/unsafe ABI thunk", repair_prompt)
            self.assertNotIn("OHOS rustc", repair_prompt)

    def test_oss_semantic_prompt_uses_rust_native_contract(self) -> None:
        """OSS semantic audit 不把 C public API/ABI 形状作为目标合同。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prompt = _build_semantic_audit_prompt(
                rendered_root=root / "rendered",
                manifest_path=root / "rendered" / "Cargo.toml",
                context_path=root / "context.json",
                ledger_path=root / "ledger.json",
                report_path=root / "report.md",
                suite="oss",
            )

        self.assertIn("独立 Rust-native 项目", prompt)
        self.assertNotIn("C/C++ header/public/exported API", prompt)
        self.assertNotIn("优先覆盖 public API", prompt)
        self.assertNotIn("public observable semantic mismatch", prompt)
        self.assertNotIn("EXT::Project::PublicApi::dimension", prompt)
        self.assertNotIn('"api": "PublicApi"', prompt)
        self.assertNotIn("one public-observable contract", prompt)
        self.assertNotIn("ABI anchor", prompt)
        self.assertNotIn("DLD anchor", prompt)

    def test_ohos_refactor_prompts_preserve_integration_contract(self) -> None:
        """OHOS 后置重构提示不能误用独立 OSS 项目的 raw-unsafe 目标。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            context = root / "context.json"
            context.write_text(
                json.dumps(
                    {
                        "suite": "ohos",
                        "information_paths": {
                            "rendered_root": str(root / "rendered"),
                            "cargo_manifest": str(root / "rendered" / "Cargo.toml"),
                            "rust_native_refactor_task_json": str(root / "review.json"),
                            "unsafe_refactor_ledger": str(root / "ledger.json"),
                        },
                        "repair_context_summary": {},
                        "semantic_repair_bundle": {},
                        "unsafe_refactor_audit": {"open_reducible_item_count": 1},
                        "write_scope": {"preferred_edit_targets": ["src/*.rs"]},
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            repair_prompt = _build_repair_prompt(context)
            audit_prompt = _build_unsafe_refactor_audit_prompt(
                rendered_root=root / "rendered",
                manifest_path=root / "rendered" / "Cargo.toml",
                context_path=context,
                ledger_path=root / "ledger.json",
                report_path=root / "report.md",
                expected_fingerprint="abc",
                feedback_paths=[],
                suite="ohos",
            )

        self.assertIn("OHOS 集成项目", repair_prompt)
        self.assertIn("OHOS 集成项目", audit_prompt)
        self.assertIn("public ABI", repair_prompt)
        self.assertIn("public ABI", audit_prompt)
        self.assertNotIn("完整 OSS/C 项目", repair_prompt)
        self.assertNotIn("完整 OSS/C 项目", audit_prompt)

    def test_oss_gate_bundle_does_not_configure_ohos_rustc(self) -> None:
        """OSS cheap gate 只要求 cargo，不运行或阻塞于 OHOS rustc。"""
        with tempfile.TemporaryDirectory() as tmp:
            bundle_path = Path(tmp) / "gate_bundle.json"
            _write_gate_bundle(
                bundle_path,
                "oss",
                {"returncode": 0, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 0, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 1, "text_log_path": "/tmp/ohos.log"},
                suite="oss",
            )
            bundle = json.loads(bundle_path.read_text(encoding="utf-8"))
            summary = _blocking_summary(bundle_path)

        self.assertEqual(bundle["cheap_gates"], ["cargo"])
        self.assertNotIn("ohos_rustc", bundle["full_gates"])
        self.assertTrue(summary["cheap_gates_passed"])
        self.assertNotIn("ohos_rustc", summary["blocking_gates"])

    def test_oss_suite_does_not_invoke_ohos_rustc(self) -> None:
        """OSS suite 必须完全跳过 OHOS rustc 进程。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            with mock.patch(
                "scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check",
                side_effect=AssertionError("OHOS rustc must not run for OSS"),
            ):
                payload = _run_suite_rustc_check(
                    "oss",
                    root,
                    root / "Cargo.toml",
                    root / "ohos.json",
                    root / "missing-rustc",
                    "x86_64-unknown-linux-ohos",
                )

        self.assertFalse(payload["configured"])
        self.assertEqual(payload["status"], "not_configured")

    def test_cargo_env_reuses_populated_default_cache(self) -> None:
        """默认 Cargo home 有 registry 缓存时，不创建空项目缓存导致联网失败。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            fake_home = root / "home"
            default_cargo = fake_home / ".cargo"
            cache_dir = default_cargo / "registry" / "cache" / "index.crates.io-demo"
            cache_dir.mkdir(parents=True)
            (cache_dir / "libc-0.2.186.crate").write_text("crate", encoding="utf-8")
            crate = root / "crate"
            crate.mkdir()
            manifest = crate / "Cargo.toml"
            manifest.write_text("[package]\nname='demo'\nversion='0.1.0'\nedition='2021'\n", encoding="utf-8")

            with mock.patch.dict("os.environ", {}, clear=True), mock.patch("pathlib.Path.home", return_value=fake_home):
                env = _cargo_env(manifest)

            self.assertEqual(env["CARGO_HOME"], str(default_cargo))
            self.assertEqual(env["CARGO_NET_OFFLINE"], "true")
            self.assertFalse((crate / ".cargo-home").exists())
            self.assertIn("--offline", _cargo_command_with_optional_offline(["cargo", "check"], env))

    def test_ohos_rustc_wrapper_filters_new_cargo_check_cfg(self) -> None:
        """OHOS rustc wrapper 过滤新 Cargo 传给旧 rustc 的 check-cfg 参数。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            capture = root / "args.txt"
            fake_rustc = root / "fake-rustc.sh"
            fake_rustc.write_text(
                "#!/usr/bin/env bash\nprintf '%s\\n' \"$@\" > " + str(capture) + "\n",
                encoding="utf-8",
            )
            fake_rustc.chmod(0o755)

            wrapper = _write_ohos_rustc_wrapper(root, fake_rustc)
            subprocess.run(
                [
                    str(wrapper),
                    "--crate-name",
                    "demo",
                    "--check-cfg",
                    "cfg(docsrs,test)",
                    "--allow=unexpected_cfgs",
                    "--emit=metadata",
                ],
                check=True,
            )

            args = capture.read_text(encoding="utf-8").splitlines()
            self.assertEqual(args, ["--crate-name", "demo", "--emit=metadata"])

    def test_repair_context_references_stub_report_path(self) -> None:
        """stub report 保留为绝对路径和摘要，不把大段 missing_types 嵌进 prompt。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("", encoding="utf-8")

            report = workspace / "skeletons" / "demo" / "types_generation_report.json"
            report.parent.mkdir(parents=True)
            large_missing = "VeryLargeMissingType_" + ("X" * 4000)
            report.write_text(
                json.dumps({"mode": "stub", "success": True, "missing_types": [large_missing]}, ensure_ascii=False),
                encoding="utf-8",
            )

            gate_bundle = root / "gate_bundle.json"
            _write_gate_bundle(
                gate_bundle,
                "initial",
                {"returncode": 1, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 0, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 0, "text_log_path": "/tmp/ohos.log"},
                _skipped_semantic_audit_payload("initial", "cheap gates failed", root / "semantic.json"),
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=gate_bundle,
                round_index=0,
                output_dir=root / "post_repair",
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            facts_path = Path(context["information_paths"]["repair_context_facts"])
            facts = json.loads(facts_path.read_text(encoding="utf-8"))
            self.assertEqual(context["repair_context_summary"]["stub_report_count"], 1)
            self.assertEqual(facts["stub_report_count"], 1)
            self.assertEqual(facts["type_report_paths"], [str(report.resolve())])
            self.assertNotIn(large_missing, context_path.read_text(encoding="utf-8"))
            self.assertNotIn(large_missing, (facts_path.parent / "repair_context.md").read_text(encoding="utf-8"))

    def test_skipped_semantic_blocks_acceptance_until_cheap_gates_pass(self) -> None:
        """cheap gate 失败时 semantic audit 记为 skipped，不能复用旧 accepted 语义结果。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            semantic = _skipped_semantic_audit_payload("round", "cheap gates failed", root / "semantic.json")
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "round",
                {"returncode": 1, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 0, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 0, "text_log_path": "/tmp/ohos.log"},
                semantic,
            )
            summary = _blocking_summary(gate_bundle)
            self.assertFalse(summary["accepted_by_gates"])
            self.assertFalse(summary["cheap_gates_passed"])
            self.assertEqual(summary["gate_statuses"]["semantic_audit"], "skipped")
            self.assertIn("cargo", summary["blocking_gates"])

    def test_clippy_gate_is_not_configured_for_post_repair(self) -> None:
        """后置 repair 不把 cargo clippy 当作阻塞 gate。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "round",
                {"returncode": 0, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 101, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 0, "text_log_path": "/tmp/ohos.log"},
                {"mode": "off"},
            )
            payload = json.loads(gate_bundle.read_text(encoding="utf-8"))
            summary = _blocking_summary(gate_bundle)

            self.assertEqual(payload["cheap_gates"], ["cargo", "ohos_rustc"])
            self.assertNotIn("cargo_clippy", payload["full_gates"])
            self.assertTrue(summary["cheap_gates_passed"])
            self.assertEqual(summary["gate_statuses"]["cargo_clippy"], "not_configured")
            self.assertNotIn("cargo_clippy", summary["blocking_gates"])

    def test_unsafe_scope_gate_is_not_exposed_before_review_phase(self) -> None:
        """普通 compile/semantic 阶段不把 unsafe 信息塞进 repair context。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            unsafe_json = root / "unsafe_scope.json"
            unsafe_md = root / "unsafe_scope.md"
            unsafe_summary = root / "unsafe_scope_summary.json"
            unsafe_json.write_text("{}", encoding="utf-8")
            unsafe_md.write_text("# Unsafe Scope Gate\n\n| id | kind |\n|---|---|\n| U0001 | unsafe_block |\n", encoding="utf-8")
            unsafe_summary.write_text("{}", encoding="utf-8")
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "round",
                {"returncode": 0, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 0, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 0, "text_log_path": "/tmp/ohos.log"},
                {"mode": "off"},
                unsafe_scope={
                    "gate": "unsafe_scope",
                    "mode": "informational",
                    "status": "available",
                    "passed": True,
                    "json_path": str(unsafe_json),
                    "markdown_path": str(unsafe_md),
                    "summary_path": str(unsafe_summary),
                    "summary": {"scope_count": 1, "unsafe_total_lines": 1, "code_lines": 1},
                },
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            summary = _blocking_summary(gate_bundle)
            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=gate_bundle,
                round_index=0,
                output_dir=root / "post_repair",
            )
            context = json.loads(context_path.read_text(encoding="utf-8"))
            prompt = _build_repair_prompt(context_path)

            self.assertTrue(summary["accepted_by_gates"])
            self.assertEqual(summary["gate_statuses"]["unsafe_scope"], "available")
            self.assertNotIn("unsafe_scope", summary["blocking_gates"])
            self.assertNotIn("unsafe_scope_gate_markdown", context["information_paths"])
            self.assertNotIn("unsafe_scope_gate_json", context["information_paths"])
            self.assertNotIn("unsafe_review_task_json", context["information_paths"])
            self.assertNotIn("unsafe_scope_summary", context["repair_context_summary"])
            self.assertEqual(context["repair_progress"]["phase"], "compile_or_full_gate_repair")
            self.assertNotIn("gate:unsafe_scope", context["repair_progress"]["recommended_focus"])
            self.assertNotIn(str(unsafe_md), prompt)
            self.assertNotIn("## Synchronized Unsafe Scope Optimization", prompt)
            self.assertNotIn("## Mandatory Unsafe Scope Optimization", prompt)
            self.assertNotIn("## Mandatory Rust-native Refactor", prompt)
            self.assertNotIn("顺手", prompt)

    def test_compile_failure_does_not_expose_unsafe_scope(self) -> None:
        """compile gate 未通过时，不运行也不暴露 unsafe scope 信息。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=0,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_check", return_value={"returncode": 101, "text_log_path": str(root / "cargo.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._apply_post_repair_rule_fixes", return_value={}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit") as semantic_mock, \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_unsafe_scope_gate") as unsafe_mock:
                rc = run_agentic(args)

            self.assertEqual(rc, 1)
            semantic_mock.assert_not_called()
            unsafe_mock.assert_not_called()
            summary = json.loads((output_dir / "post_repair_summary.json").read_text(encoding="utf-8"))
            first_round = summary["rounds"][0]
            initial_bundle = json.loads(Path(first_round["gate_bundle"]).read_text(encoding="utf-8"))
            final_bundle = json.loads((output_dir / "final_gate_bundle.json").read_text(encoding="utf-8"))
            context = json.loads(Path(first_round["agent_context"]).read_text(encoding="utf-8"))
            prompt = _build_repair_prompt(Path(first_round["agent_context"]))

            self.assertFalse(first_round["blocking_summary"]["cheap_gates_passed"])
            self.assertNotIn("unsafe_scope", initial_bundle)
            self.assertNotIn("unsafe_scope", final_bundle)
            self.assertNotIn("unsafe_scope_gate_markdown", context["information_paths"])
            self.assertNotIn("unsafe_scope_summary", context["repair_context_summary"])
            self.assertNotIn("unsafe_scope", prompt)
            self.assertNotIn("Unsafe Scope", prompt)
            self.assertNotIn("Synchronized Unsafe Scope Optimization", prompt)

    def test_gate_bundle_discards_unsafe_scope_when_compile_fails(self) -> None:
        """即使调用方误传 unsafe payload，compile 失败 bundle 也不暴露它。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            unsafe_md = root / "unsafe_scope.md"
            unsafe_md.write_text("# Unsafe Scope Gate\n", encoding="utf-8")
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "round",
                {"returncode": 101, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                {"mode": "off"},
                unsafe_scope={
                    "gate": "unsafe_scope",
                    "mode": "informational",
                    "status": "available",
                    "passed": True,
                    "json_path": str(root / "unsafe_scope.json"),
                    "markdown_path": str(unsafe_md),
                    "summary_path": str(root / "unsafe_scope_summary.json"),
                    "summary": {"scope_count": 1},
                },
            )
            payload = json.loads(gate_bundle.read_text(encoding="utf-8"))
            summary = _blocking_summary(gate_bundle)

            self.assertFalse(payload["cheap_gates_passed"])
            self.assertNotIn("unsafe_scope", payload)
            self.assertNotIn("unsafe_scope", payload["full_gates"])
            self.assertNotIn("unsafe_scope", summary["gate_statuses"])
            self.assertNotIn("unsafe_scope_gate_markdown", summary)

    def test_body_completeness_scan_is_not_a_default_gate(self) -> None:
        """残留占位扫描仍可用，但默认不再作为 accepted gate。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "crate"
            src = crate / "src"
            src.mkdir(parents=True)
            (src / "lib.rs").write_text(
                "pub fn ready() -> i32 { 1 }\n"
                "pub fn missing() -> i32 { unimplemented!() }\n",
                encoding="utf-8",
            )

            completeness = _scan_body_completeness(crate)
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "round",
                {"returncode": 0, "text_log_path": "/tmp/cargo.log"},
                {"returncode": 0, "text_log_path": "/tmp/clippy.log"},
                {"returncode": 0, "text_log_path": "/tmp/ohos.log"},
                {"mode": "off"},
                completeness,
            )
            payload = json.loads(gate_bundle.read_text(encoding="utf-8"))
            summary = _blocking_summary(gate_bundle)

            self.assertFalse(completeness["passed"])
            self.assertTrue(payload["accepted_by_gates"])
            self.assertNotIn("body_completeness", payload["full_gates"])
            self.assertNotIn("body_completeness", payload)
            self.assertNotIn("body_completeness", summary["gate_statuses"])
            self.assertNotIn("body_completeness", summary["blocking_gates"])

    def test_body_completeness_scope_only_counts_source_targets(self) -> None:
        """body gate 有目标映射时，只统计源函数一对一翻译目标。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            mapping = workspace / "source_skeletons" / "demo" / "func_file_to_rust_sig.json"
            mapping.parent.mkdir(parents=True)
            mapping.write_text(json.dumps({"src_lib_1": "pub fn target() -> i32"}), encoding="utf-8")

            crate = root / "crate"
            src = crate / "src"
            src.mkdir(parents=True)
            target_file = src / "src_lib.rs"
            target_file.write_text("pub fn target() -> i32 { 1 }\n", encoding="utf-8")
            (src / "shim.rs").write_text("pub fn shim() -> i32 { unimplemented!() }\n", encoding="utf-8")

            scope = _build_body_target_scope(workspace, "demo", output_path=root / "body_target_scope.json")
            scoped = _scan_body_completeness(crate, body_target_scope=scope)
            all_rs = _scan_body_completeness(crate)

            self.assertTrue(scoped["passed"])
            self.assertEqual(scoped["scope"], "source_target_functions")
            self.assertEqual(scoped["target_count"], 1)
            self.assertEqual(scoped["unmatched_target_count"], 0)
            self.assertEqual(scoped["total_findings"], 0)
            self.assertFalse(all_rs["passed"])
            self.assertEqual(all_rs["total_findings"], 1)

            target_file.write_text("pub fn target() -> i32 { unimplemented!() }\n", encoding="utf-8")
            scoped_after_target_regression = _scan_body_completeness(crate, body_target_scope=root / "body_target_scope.json")
            self.assertFalse(scoped_after_target_regression["passed"])
            self.assertEqual(scoped_after_target_regression["total_findings"], 1)
            self.assertEqual(scoped_after_target_regression["findings"][0]["path"], "src/src_lib.rs")

    def test_run_agentic_missing_manifest_writes_rejected_canonical(self) -> None:
        """最终 crate 缺 Cargo.toml 时，post-repair 应自然写 rejected canonical。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            out = root / "post_repair"
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(out),
                max_rounds=1,
                semantic_audit_step_limit=1,
                agent_step_limit=1,
                agent_timeout_sec=1.0,
                ohos_rustc=str(root / "missing-rustc"),
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            rc = run_agentic(args)

            self.assertEqual(rc, 1)
            summary = json.loads((out / "post_repair_summary.json").read_text(encoding="utf-8"))
            canonical = json.loads((out / "canonical_post_repair_result.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["final_status"], "rejected")
            self.assertEqual(summary["failure_kind"], "missing_rendered_crate")
            self.assertEqual(canonical["final_status"], "rejected")
            self.assertTrue((out / "final_verify_summary.json").is_file())

    def test_run_agentic_repair_runner_exception_writes_rejected_canonical(self) -> None:
        """repair runner 外层异常不能绕过 final_verify/canonical。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            src = rendered / "src"
            src.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text(
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"src/lib.rs\"\n",
                encoding="utf-8",
            )
            (src / "lib.rs").write_text("pub fn missing() -> i32 { unimplemented!() }\n", encoding="utf-8")
            out = root / "post_repair"
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(out),
                max_rounds=1,
                semantic_audit_step_limit=1,
                agent_step_limit=1,
                agent_timeout_sec=1.0,
                ohos_rustc=str(root / "missing-rustc"),
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch(
                "scripts.agentic_repair.post_repair_agent.OpenAIActionRunner.run",
                side_effect=RuntimeError("runner exploded"),
            ):
                rc = run_agentic(args)

            self.assertEqual(rc, 1)
            summary = json.loads((out / "post_repair_summary.json").read_text(encoding="utf-8"))
            canonical = json.loads((out / "canonical_post_repair_result.json").read_text(encoding="utf-8"))
            latest_round = summary["rounds"][-1]
            self.assertEqual(summary["final_status"], "rejected")
            self.assertEqual(latest_round["repair_agent"]["returncode"], 127)
            self.assertIn("runner exploded", latest_round["repair_agent"]["error"])
            self.assertEqual(canonical["final_status"], "rejected")

    def test_repair_context_keeps_prior_semantic_audit_after_cheap_gate_regression(self) -> None:
        """cheap gate 回退后，context 仍应携带上一轮真实 semantic blocker。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("", encoding="utf-8")

            current_gate = _write_gate_bundle(
                root / "current_gate_bundle.json",
                "repair_round_02",
                {"returncode": 101, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                _skipped_semantic_audit_payload("repair_round_02", "cheap gates failed", root / "current_semantic.json"),
            )
            ledger_path = root / "semantic_blockers_ledger.json"
            ledger_path.write_text(
                json.dumps(
                    {
                        "schema_version": "rust_semantic_blockers_ledger_v1",
                        "verdict": "rejected",
                        "semantic_obligations": [
                            {
                                "id": "EXT::demo::success_path",
                                "api": "demo",
                                "dimension": "success_path",
                                "status": "blocking_mismatch",
                                "root_cause_cluster_id": "RC::demo::wrong_return",
                                "repair_target": "Return the same value as the C implementation.",
                            }
                        ],
                        "root_cause_clusters": [
                            {
                                "id": "RC::demo::wrong_return",
                                "status": "open",
                                "affected_obligations": ["EXT::demo::success_path"],
                                "summary": "Rust returns the wrong success value.",
                                "repair_kind": "rust_only",
                                "repair_strategy": "Align the Rust return path with the C evidence.",
                            },
                            {
                                "id": "RC::demo::old_fixed",
                                "status": "fixed",
                                "affected_obligations": [],
                                "summary": "Already fixed issue.",
                                "repair_kind": "rust_only",
                                "repair_strategy": "",
                            },
                        ],
                        "blockers": [
                            {
                                "id": "SEM::demo::wrong_return",
                                "status": "open",
                                "summary": "wrong return",
                                "repair_target": "Return the C-compatible value.",
                            }
                        ],
                    }
                ),
                encoding="utf-8",
            )
            prior_gate = _write_gate_bundle(
                root / "prior_gate_bundle.json",
                "repair_round_01",
                {"returncode": 0, "text_log_path": str(root / "old_cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "old_clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "old_ohos.log")},
                {
                    "gate": "semantic_audit",
                    "mode": "required",
                    "round": "repair_round_01",
                    "status": "rejected",
                    "passed": False,
                    "accepted": False,
                    "returncode": 1,
                    "verdict": "rejected",
                    "ledger_path": str(ledger_path),
                    "report_path": str(root / "semantic_audit_report.md"),
                    "text_log_path": str(root / "semantic_audit.log"),
                    "blocking_ids": ["SEM::demo::wrong_return"],
                    "open_blocker_ids": ["SEM::demo::wrong_return"],
                    "blocked_external_ids": [],
                    "open_obligation_ids": ["EXT::demo::success_path"],
                    "semantic_obligation_blocking_ids": ["EXT::demo::success_path"],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 1, "by_status": {"blocking_mismatch": 1}},
                    "semantic_obligation_count": 1,
                    "diagnostics": [],
                },
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=current_gate,
                round_index=2,
                output_dir=root / "post_repair",
                semantic_reference_gate_bundle_path=prior_gate,
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            gates = context["current_gates"]
            self.assertEqual(gates["blocking_gates"], ["cargo"])
            self.assertTrue(gates["semantic_repair_mode"]["active"])
            self.assertEqual(gates["semantic_repair_mode"]["reason"], "cheap_gate_regression_after_semantic_audit")
            self.assertEqual(gates["semantic_repair_mode"]["semantic_reference_gate_bundle"], str(prior_gate.resolve()))
            self.assertEqual(gates["semantic_blockers_ledger_path"], str(root / "semantic_blockers_ledger.json"))
            self.assertEqual(gates["semantic_audit_report_path"], str(root / "semantic_audit_report.md"))
            self.assertEqual(gates["semantic_audit_open_blocker_ids"], ["SEM::demo::wrong_return"])
            self.assertEqual(gates["semantic_obligation_open_ids"], ["EXT::demo::success_path"])
            self.assertEqual(gates["semantic_obligation_blocking_ids"], ["EXT::demo::success_path"])
            self.assertEqual(gates["semantic_obligation_summary"], {"total": 1, "by_status": {"blocking_mismatch": 1}})
            self.assertEqual(gates["semantic_obligation_count"], 1)

            semantic_bundle = context["semantic_repair_bundle"]
            self.assertTrue(semantic_bundle["active"])
            self.assertFalse(semantic_bundle["semantic_locked"])
            self.assertEqual(semantic_bundle["verdict"], "rejected")
            self.assertEqual([item["id"] for item in semantic_bundle["open_root_cause_clusters"]], ["RC::demo::wrong_return"])
            self.assertEqual(semantic_bundle["blocked_external_root_cause_clusters"], [])
            self.assertEqual([item["id"] for item in semantic_bundle["repair_root_cause_clusters"]], ["RC::demo::wrong_return"])
            self.assertEqual([item["id"] for item in semantic_bundle["fixed_root_cause_clusters"]], ["RC::demo::old_fixed"])
            self.assertEqual([item["id"] for item in semantic_bundle["blocking_obligations"]], ["EXT::demo::success_path"])
            self.assertEqual(semantic_bundle["blocked_external_obligations"], [])
            self.assertEqual([item["id"] for item in semantic_bundle["repair_obligations"]], ["EXT::demo::success_path"])
            self.assertEqual(context["repair_progress"]["current_blocking_gate"], "cargo")
            self.assertEqual(context["repair_progress"]["phase"], "semantic_root_cause_repair")
            self.assertIn("gate:cargo", context["repair_progress"]["recommended_focus"])
            self.assertIn("root_cause:RC::demo::wrong_return", context["repair_progress"]["recommended_focus"])
            self.assertTrue(context["repair_progress"]["do_not_reaudit_semantics"])
            self.assertNotIn("semantic_audit_owned", context["write_policy"])
            self.assertTrue(context["information_paths"]["ohos_root"].endswith("OpenHarmony-v5.0.1-Release/OpenHarmony"))
            self.assertEqual(context["write_scope"]["preferred_edit_targets"], ["src/*.rs", "src/types.rs", "src/compat.rs", "build.rs", "native/*"])

            prompt = _build_repair_prompt(context_path)
            self.assertIn("ohos_root:", prompt)
            self.assertIn("Primary Semantic Repair Targets", prompt)
            self.assertIn("repair agent 不是 semantic audit agent", prompt)
            self.assertIn("按 repair_root_cause_clusters 全量修复", prompt)
            self.assertIn("blocked_external_root_cause_clusters / blocked_external_obligations 是可选参考输入", prompt)
            self.assertIn("src/compat.rs", prompt)
            self.assertIn("src/types.rs", prompt)
            self.assertNotIn("body_completeness", prompt)
            self.assertNotIn("body_target_scope", prompt)
            self.assertNotIn("unimplemented!()/todo!()", prompt)
            self.assertIn("不要为了记录分析过程创建额外的 trace/review/scratch 文档", prompt)
            self.assertNotIn("Synchronized Unsafe Scope Optimization", prompt)
            self.assertNotIn("unsafe_scope", prompt)
            self.assertIn("finish.message 保持 1-2 句", prompt)
            self.assertNotIn("semantic_repair_mode.active=true", prompt)
            self.assertNotIn("语义留痕中标记", prompt)
            self.assertNotIn("COMPLETE_TASK_AND_SUBMIT_FINAL_OUTPUT", prompt)
            self.assertIn("resolved_source_evidence", prompt)
            self.assertIn("extracted_functions 和 functions_manifest 只能作为索引或片段线索", prompt)
            self.assertIn("不能替代源码证据", prompt)

    def test_repair_context_without_prior_semantic_keeps_current_compile_only_behavior(self) -> None:
        """没有历史 semantic audit 时，初始 cheap gate 失败不能硬塞语义内容。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("pub fn demo() {}\n", encoding="utf-8")
            current_gate = _write_gate_bundle(
                root / "current_gate_bundle.json",
                "initial",
                {"returncode": 101, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                _skipped_semantic_audit_payload("initial", "cheap gates failed", root / "current_semantic.json"),
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=current_gate,
                round_index=0,
                output_dir=root / "post_repair",
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            gates = context["current_gates"]
            self.assertFalse(gates["semantic_repair_mode"]["active"])
            self.assertEqual(gates["semantic_blockers_ledger_path"], "")
            self.assertEqual(gates["semantic_audit_report_path"], "")
            self.assertEqual(gates["semantic_audit_open_blocker_ids"], [])
            self.assertEqual(gates["semantic_obligation_open_ids"], [])
            self.assertEqual(gates["semantic_obligation_summary"], {"total": 0, "by_status": {}})
            self.assertEqual(
                context["semantic_repair_bundle"],
                {
                    "active": False,
                    "semantic_locked": False,
                    "verdict": "skipped",
                    "open_root_cause_clusters": [],
                    "blocked_external_root_cause_clusters": [],
                    "repair_root_cause_clusters": [],
                    "fixed_root_cause_clusters": [],
                    "blocking_obligations": [],
                    "blocked_external_obligations": [],
                    "repair_obligations": [],
                },
            )
            self.assertEqual(context["repair_progress"]["current_blocking_gate"], "cargo")
            self.assertEqual(context["repair_progress"]["phase"], "cheap_gate_repair")
            self.assertEqual(context["repair_progress"]["last_round_changed_files"], [])
            self.assertEqual(context["repair_progress"]["last_round_failed_attempts"], [])
            self.assertEqual(context["repair_progress"]["recommended_focus"], ["gate:cargo"])
            self.assertTrue(context["repair_progress"]["do_not_reaudit_semantics"])

    def test_repair_context_resolves_original_source_evidence(self) -> None:
        """repair context 顶层提供真实 C/C++ 源码、header 和测试/示例用法入口。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            source_root = workspace / "c_source" / "demo"
            (source_root / "src").mkdir(parents=True)
            (source_root / "include").mkdir()
            (source_root / "tests").mkdir()
            (source_root / "src" / "core.c").write_text("int core(void) { return 7; }\n", encoding="utf-8")
            (source_root / "include" / "core.h").write_text("int core(void);\n", encoding="utf-8")
            (source_root / "tests" / "core_test.c").write_text("int main(void) { return core(); }\n", encoding="utf-8")
            extracted_root = workspace / "extracted" / "demo"
            (extracted_root / "functions").mkdir(parents=True)
            (extracted_root / "functions_manifest.json").write_text(
                json.dumps(
                    {
                        "functions": [
                            {
                                "source_file": "src_core",
                                "uid": "src_core:1:core",
                                "func_file": "src_core_1",
                                "start_line": 1,
                                "end_line": 1,
                            },
                            {
                                "source_file": "missing_source",
                                "uid": "missing_source:1:missing",
                                "func_file": "missing_source_1",
                            },
                        ]
                    }
                ),
                encoding="utf-8",
            )
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("pub fn core() -> i32 { 7 }\n", encoding="utf-8")
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "initial",
                {"returncode": 101, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                _skipped_semantic_audit_payload("initial", "cheap gates failed", root / "semantic.json"),
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(source_root),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=gate_bundle,
                round_index=0,
                output_dir=root / "post_repair",
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            facts = json.loads(Path(context["information_paths"]["repair_context_facts"]).read_text(encoding="utf-8"))
            evidence = facts["resolved_source_evidence"]
            self.assertEqual(evidence["search_roots"], [str(source_root.resolve())])
            self.assertEqual(evidence["production_sources"][0]["path"], str((source_root / "src" / "core.c").resolve()))
            self.assertEqual(evidence["production_sources"][0]["source_key"], "src_core")
            self.assertEqual(evidence["production_sources"][0]["start_line"], 1)
            self.assertEqual(evidence["public_headers"][0]["path"], str((source_root / "include" / "core.h").resolve()))
            self.assertEqual(evidence["test_or_example_usage"][0]["path"], str((source_root / "tests" / "core_test.c").resolve()))
            self.assertEqual(evidence["unresolved_sources"][0]["source_key"], "missing_source")
            self.assertEqual(context["information_paths"]["resolved_source_evidence"]["counts"]["production_sources"], 1)
            self.assertEqual(context["repair_context_summary"]["resolved_source_evidence_counts"]["public_headers"], 1)

            prompt = _build_repair_prompt(context_path)
            self.assertIn("resolved_source_evidence", prompt)
            self.assertIn("先打开 information_paths.resolved_source_evidence", prompt)
            self.assertIn("extracted_functions 和 functions_manifest 只能作为索引或片段线索", prompt)
            self.assertIn("不能替代源码证据", prompt)

    def test_repair_context_locks_accepted_prior_semantic_without_open_clusters(self) -> None:
        """历史 semantic audit 已 accepted 且无 open cluster 时，不触发语义泛化修复。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("pub fn demo() {}\n", encoding="utf-8")
            current_gate = _write_gate_bundle(
                root / "current_gate_bundle.json",
                "repair_round_02",
                {"returncode": 101, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                _skipped_semantic_audit_payload("repair_round_02", "cheap gates failed", root / "current_semantic.json"),
            )
            ledger_path = root / "semantic_blockers_ledger.json"
            ledger_path.write_text(
                json.dumps(
                    {
                        "schema_version": "rust_semantic_blockers_ledger_v1",
                        "verdict": "accepted",
                        "semantic_obligations": [
                            {
                                "id": "EXT::demo::success_path",
                                "status": "equivalent",
                                "root_cause_cluster_id": "RC::demo::old_fixed",
                            }
                        ],
                        "root_cause_clusters": [
                            {
                                "id": "RC::demo::old_fixed",
                                "status": "fixed",
                                "affected_obligations": ["EXT::demo::success_path"],
                                "summary": "Already fixed issue.",
                            }
                        ],
                        "blockers": [],
                    }
                ),
                encoding="utf-8",
            )
            prior_gate = _write_gate_bundle(
                root / "prior_gate_bundle.json",
                "repair_round_01",
                {"returncode": 0, "text_log_path": str(root / "old_cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "old_clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "old_ohos.log")},
                {
                    "gate": "semantic_audit",
                    "mode": "required",
                    "round": "repair_round_01",
                    "status": "accepted",
                    "passed": True,
                    "accepted": True,
                    "returncode": 0,
                    "verdict": "accepted",
                    "ledger_path": str(ledger_path),
                    "report_path": str(root / "semantic_audit_report.md"),
                    "text_log_path": str(root / "semantic_audit.log"),
                    "blocking_ids": [],
                    "open_blocker_ids": [],
                    "blocked_external_ids": [],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 1, "by_status": {"equivalent": 1}},
                    "semantic_obligation_count": 1,
                    "diagnostics": [],
                },
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=current_gate,
                round_index=2,
                output_dir=root / "post_repair",
                semantic_reference_gate_bundle_path=prior_gate,
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            semantic_bundle = context["semantic_repair_bundle"]
            self.assertFalse(context["current_gates"]["semantic_repair_mode"]["active"])
            self.assertTrue(semantic_bundle["active"])
            self.assertTrue(semantic_bundle["semantic_locked"])
            self.assertEqual(semantic_bundle["verdict"], "accepted")
            self.assertEqual(semantic_bundle["open_root_cause_clusters"], [])
            self.assertEqual(semantic_bundle["blocked_external_root_cause_clusters"], [])
            self.assertEqual(semantic_bundle["repair_root_cause_clusters"], [])
            self.assertEqual([item["id"] for item in semantic_bundle["fixed_root_cause_clusters"]], ["RC::demo::old_fixed"])
            self.assertEqual(semantic_bundle["blocking_obligations"], [])
            self.assertEqual(semantic_bundle["blocked_external_obligations"], [])
            self.assertEqual(semantic_bundle["repair_obligations"], [])
            self.assertEqual(context["repair_progress"]["current_blocking_gate"], "cargo")
            self.assertEqual(context["repair_progress"]["phase"], "cheap_gate_repair")
            prompt = _build_repair_prompt(context_path)
            self.assertIn("semantic_repair_bundle.semantic_locked=true", prompt)
            self.assertIn("不做语义泛化审计", prompt)

    def test_agentic_loop_keeps_semantic_anchor_after_repair_breaks_compile(self) -> None:
        """主循环中上一轮真实 semantic audit 结果不能被后续 cheap gate 失败覆盖。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo() {}\n", encoding="utf-8")

            class FakeAgentRun:
                def __init__(self, out: Path) -> None:
                    self.stdout = out / "fake.stdout.log"
                    self.stderr = out / "fake.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("COMPLETE_TASK_AND_SUBMIT_FINAL_OUTPUT\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def __init__(self, *args: object, **kwargs: object) -> None:
                    pass

                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str) -> FakeAgentRun:
                    return FakeAgentRun(output_dir)

            def fake_semantic(**kwargs: object) -> dict[str, object]:
                out = Path(kwargs["log_path"]).parent.parent
                ledger_path = out / "semantic_blockers_ledger.json"
                ledger_path.write_text(
                    json.dumps(
                        {
                            "schema_version": "rust_semantic_blockers_ledger_v1",
                            "verdict": "rejected",
                            "semantic_obligations": [
                                {
                                    "id": "EXT::demo::success_path",
                                    "status": "blocking_mismatch",
                                    "root_cause_cluster_id": "RC::demo::wrong_return",
                                    "repair_target": "Return the C-compatible value.",
                                }
                            ],
                            "root_cause_clusters": [
                                {
                                    "id": "RC::demo::wrong_return",
                                    "status": "open",
                                    "affected_obligations": ["EXT::demo::success_path"],
                                    "summary": "wrong return",
                                    "repair_kind": "rust_only",
                                    "repair_strategy": "Return the C-compatible value.",
                                }
                            ],
                            "blockers": [
                                {
                                    "id": "SEM::demo::wrong_return",
                                    "status": "open",
                                    "summary": "wrong return",
                                    "repair_target": "Return the C-compatible value.",
                                }
                            ],
                        }
                    ),
                    encoding="utf-8",
                )
                return {
                    "gate": "semantic_audit",
                    "mode": "required",
                    "round": kwargs["round_label"],
                    "status": "rejected",
                    "passed": False,
                    "accepted": False,
                    "returncode": 1,
                    "verdict": "rejected",
                    "ledger_path": str(ledger_path),
                    "report_path": str(out / "semantic_audit_report.md"),
                    "text_log_path": str(out / "semantic_audit.log"),
                    "blocking_ids": ["SEM::demo::wrong_return"],
                    "open_blocker_ids": ["SEM::demo::wrong_return"],
                    "blocked_external_ids": [],
                    "open_obligation_ids": ["EXT::demo::success_path"],
                    "semantic_obligation_blocking_ids": ["EXT::demo::success_path"],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 1, "by_status": {"blocking_mismatch": 1}},
                    "semantic_obligation_count": 1,
                    "diagnostics": [],
                }

            args = argparse.Namespace(
                workspace_dir=str(workspace),
                output_dir=str(root / "post_repair"),
                rendered_root="",
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                max_rounds=1,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1,
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent.OpenAIActionRunner", FakeRunner), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_check", side_effect=[
                    {"returncode": 0, "text_log_path": str(root / "initial_cargo.log")},
                    {"returncode": 101, "text_log_path": str(root / "round1_cargo.log")},
                    {"returncode": 101, "text_log_path": str(root / "final_cargo.log")},
                ]), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", side_effect=fake_semantic):
                self.assertEqual(run_agentic(args), 1)

            context = json.loads((root / "post_repair" / "repair_context" / "agent_context_round_01.json").read_text(encoding="utf-8"))
            gates = context["current_gates"]
            self.assertEqual(gates["blocking_gates"], ["cargo"])
            self.assertTrue(gates["semantic_repair_mode"]["active"])
            self.assertEqual(gates["semantic_audit_open_blocker_ids"], ["SEM::demo::wrong_return"])
            self.assertEqual(gates["semantic_obligation_blocking_ids"], ["EXT::demo::success_path"])
            self.assertEqual(gates["semantic_obligation_summary"], {"total": 1, "by_status": {"blocking_mismatch": 1}})
            self.assertEqual([item["id"] for item in context["semantic_repair_bundle"]["repair_root_cause_clusters"]], ["RC::demo::wrong_return"])
            self.assertEqual(context["repair_progress"]["phase"], "semantic_root_cause_repair")

    def test_repair_context_exposes_failed_translation_artifact_paths(self) -> None:
        """后置 repair prompt 必须拿到失败译文、错误、尝试版本和真实 LLM prompt 的绝对路径。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered_src = rendered / "src"
            rendered_src.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            rust_file = rendered_src / "lib.rs"
            rust_file.write_text(
                "pub fn demo() { unimplemented!() }\n"
                "// === C2R_FAILED_TRANSLATION_BEGIN func_key: demo__0 ===\n"
                "// pub fn demo() { missing_symbol(); }\n"
                "// === C2R_FAILED_TRANSLATION_END func_key: demo__0 ===\n",
                encoding="utf-8",
            )

            manual_root = workspace / "repair_history" / "demo" / "translate_by_llm" / "_manual_fix" / "demo__0"
            manual_root.mkdir(parents=True)
            failed_translation = manual_root / "translated_rust.rs"
            compile_error = manual_root / "compile_error.log"
            attempts = manual_root / "attempts.json"
            closure_manifest = manual_root / "dependency_closure.json"
            prompt_path = workspace / "llm_prompts" / "demo" / "translate_by_llm" / "incremental_translate_demo_20260101.txt"
            prompt_path.parent.mkdir(parents=True)
            failed_translation.write_text("pub fn demo() { missing_symbol(); }\n", encoding="utf-8")
            compile_error.write_text("error[E0425]: cannot find function `missing_symbol`\n", encoding="utf-8")
            attempts.write_text(json.dumps({"attempts": [{"attempt_num": 1, "error_path": str(compile_error.resolve())}]}), encoding="utf-8")
            closure_manifest.write_text(json.dumps({"facts": [], "hints": [], "gaps": []}), encoding="utf-8")
            prompt_path.write_text("real prompt", encoding="utf-8")
            meta = {
                "func_key": "demo__0",
                "c_func": "demo",
                "reason": "repair_failed_after_1",
                "meta_path": str((manual_root / "meta.json").resolve()),
                "failed_translation_path": str(failed_translation.resolve()),
                "compile_error_path": str(compile_error.resolve()),
                "attempts_path": str(attempts.resolve()),
                "dependency_closure_manifest_path": str(closure_manifest.resolve()),
                "prompt_paths": [str(prompt_path.resolve())],
                "repair_history_dir": str((manual_root.parent.parent / "demo__0").resolve()),
                "rust_file_path": str(rust_file.resolve()),
                "source_comment_marker": "C2R_FAILED_TRANSLATION_BEGIN func_key: demo__0",
            }
            (manual_root / "meta.json").write_text(json.dumps(meta, ensure_ascii=False), encoding="utf-8")
            manifest = manual_root.parent / "manifest.jsonl"
            manifest.write_text(json.dumps(meta, ensure_ascii=False) + "\n", encoding="utf-8")

            gate_bundle = root / "gate_bundle.json"
            _write_gate_bundle(
                gate_bundle,
                "initial",
                {"returncode": 1, "text_log_path": str((root / "cargo.log").resolve())},
                {"returncode": 0, "text_log_path": str((root / "clippy.log").resolve())},
                {"returncode": 0, "text_log_path": str((root / "ohos.log").resolve())},
                _skipped_semantic_audit_payload("initial", "cheap gates failed", root / "semantic.json"),
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=gate_bundle,
                round_index=0,
                output_dir=root / "post_repair",
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            facts = json.loads(Path(context["information_paths"]["repair_context_facts"]).read_text(encoding="utf-8"))
            self.assertEqual(context["repair_context_summary"]["failed_translation_artifact_count"], 1)
            self.assertEqual(context["information_paths"]["manual_fix_manifest"], str(manifest.resolve()))
            self.assertEqual(context["information_paths"]["source_project_root"], "")
            self.assertEqual(context["information_paths"]["copied_c_source"], "")
            self.assertEqual(context["information_paths"]["extracted_functions"], "")
            self.assertEqual(context["information_paths"]["functions_manifest"], "")
            self.assertEqual(facts["manual_fix_artifacts"]["artifact_count"], 1)
            self.assertNotIn("artifacts", facts["manual_fix_artifacts"])
            artifact = json.loads(manifest.read_text(encoding="utf-8").strip())
            self.assertEqual(artifact["failed_translation_path"], str(failed_translation.resolve()))
            self.assertEqual(artifact["compile_error_path"], str(compile_error.resolve()))
            self.assertEqual(artifact["attempts_path"], str(attempts.resolve()))
            self.assertEqual(artifact["dependency_closure_manifest_path"], str(closure_manifest.resolve()))
            self.assertEqual(artifact["prompt_paths"], [str(prompt_path.resolve())])
            self.assertIn(str(rust_file.resolve()), facts["rust_marker_scan"]["absolute_files"]["failed_translation_comment"])

            prompt = _build_repair_prompt(context_path)
            self.assertIn(str(manifest.resolve()), prompt)
            self.assertIn(str((workspace / "repair_history" / "demo" / "translate_by_llm").resolve()), prompt)
            self.assertIn(str(prompt_path.parent.resolve()), prompt)
            self.assertIn("dependency_closure_manifest_path", prompt)
            self.assertIn("prompt_paths", prompt)
            self.assertIn("## Evidence Policy", prompt)
            self.assertIn("已经生成的 Rust crate", prompt)
            self.assertIn("不是重新运行 C/C++2Rust 翻译器", prompt)
            self.assertIn("只读取非空且存在的路径", prompt)
            self.assertIn("旧提示词里不存在于当前 crate", prompt)
            self.assertIn("不要新增 crates.io/registry 依赖", prompt)
            self.assertIn("cargo dependency/registry/workspace/timeout", prompt)
            self.assertNotIn("完成 token", prompt)

    def test_run_agentic_writes_gate_aware_canonical_result(self) -> None:
        """gate 全过后自动 accepted，并写出 canonical_post_repair_result.json。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=0,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )
            semantic_payload = {
                "mode": "required",
                "status": "accepted",
                "passed": True,
                "accepted": True,
                "verdict": "accepted",
                "ledger_path": "",
                "report_path": "",
                "text_log_path": str(root / "semantic.log"),
                "open_blocker_ids": [],
                "blocked_external_ids": [],
                "open_obligation_ids": [],
                "semantic_obligation_blocking_ids": [],
                "blocked_external_obligation_ids": [],
                "semantic_obligation_summary": {"total": 0, "by_status": {}},
                "semantic_obligation_count": 0,
            }
            unsafe_refactor_payload = {
                "gate": "unsafe_refactor_audit",
                "mode": "required",
                "status": "accepted",
                "passed": True,
                "accepted": True,
                "verdict": "accepted",
                "ledger_path": str(output_dir / "unsafe_refactor_ledger.json"),
                "report_path": str(output_dir / "unsafe_refactor_report.md"),
                "text_log_path": str(root / "unsafe_refactor.log"),
                "open_reducible_items": [],
                "open_reducible_item_count": 0,
                "diagnostics": [],
            }

            with mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_check", return_value={"returncode": 0, "text_log_path": str(root / "cargo.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", return_value=semantic_payload), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_unsafe_refactor_audit", return_value=unsafe_refactor_payload):
                rc = run_agentic(args)

            self.assertEqual(rc, 0)
            canonical = json.loads((output_dir / "canonical_post_repair_result.json").read_text(encoding="utf-8"))
            self.assertEqual(canonical["success_definition"], "accepted_by_gates")
            self.assertEqual(canonical["final_status"], "accepted")
            self.assertTrue(canonical["post_repair"]["accepted_by_gates"])
            self.assertTrue(canonical["post_repair"]["semantic_audit_ok"])
            self.assertTrue((output_dir / "final_verify").is_dir())
            self.assertTrue((output_dir / "final_gate_bundle.json").is_file())
            self.assertTrue((output_dir / "final_verify_summary.json").is_file())
            self.assertTrue((output_dir / "source_fingerprint.json").is_file())
            self.assertEqual(canonical["artifacts"]["final_gate_bundle"], str((output_dir / "final_gate_bundle.json").resolve()))
            self.assertEqual(canonical["artifacts"]["source_fingerprint"], str((output_dir / "source_fingerprint.json").resolve()))

    def test_run_agentic_runs_unsafe_auditor_after_first_semantic_acceptance(self) -> None:
        """semantic 首次通过后启动 unsafe auditor，repair 只消费 auditor ledger。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            repair_tasks: list[str] = []
            audit_names: list[str] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def __init__(self, *args: object, **kwargs: object) -> None:
                    pass

                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    if name == "unsafe_refactor_audit":
                        audit_names.append(name)
                        self_ref.assertTrue(callable(finish_validator))
                        ledger_path = self_ref._path_from_task(task, "- unsafe_refactor_ledger.json:")
                        report_path = self_ref._path_from_task(task, "- Markdown report:")
                        fingerprint = self_ref._value_from_task(task, "- expected_source_fingerprint_sha256:")
                        if len(audit_names) == 1:
                            payload = {
                                "schema_version": "c2r_unsafe_refactor_ledger_v1",
                                "status": "needs_repair",
                                "verdict": "needs_repair",
                                "done": False,
                                "source_fingerprint_sha256": fingerprint,
                                "items": [],
                                "open_reducible_items": [
                                    {
                                        "id": "UNSAFE::demo::demo::block",
                                        "scope_ids": ["U0001"],
                                        "file": "src/lib.rs",
                                        "span": "1-1",
                                        "problem": "ordinary return expression is inside unsafe block",
                                        "repair_instruction": "keep only pointer dereference in a narrow unsafe expression",
                                        "must_preserve": ["raw pointer dereference result"],
                                        "evidence": ["unsafe_scope:U0001", "src/lib.rs:1"],
                                    }
                                ],
                                "hard_required_items": [],
                                "repair_feedback_consumed": [],
                                "report_path": report_path,
                                "summary": "one reducible unsafe scope",
                            }
                        else:
                            payload = {
                                "schema_version": "c2r_unsafe_refactor_ledger_v1",
                                "status": "accepted",
                                "verdict": "accepted",
                                "done": True,
                                "source_fingerprint_sha256": fingerprint,
                                "items": [
                                    {
                                        "id": "UNSAFE::demo::demo::deref",
                                        "scope_ids": ["U0001"],
                                        "file": "src/lib.rs",
                                        "span": "1-1",
                                        "function": "demo",
                                        "classification": "hard_required",
                                        "status": "hard_required",
                                        "problem": "raw pointer dereference remains required",
                                        "repair_instruction": "",
                                        "must_preserve": ["raw pointer dereference result"],
                                        "evidence": ["unsafe_scope:U0001", "src/lib.rs:1"],
                                        "expected_effect": "none",
                                    }
                                ],
                                "open_reducible_items": [],
                                "hard_required_items": [{"id": "UNSAFE::demo::demo::deref", "evidence": ["src/lib.rs:1"]}],
                                "repair_feedback_consumed": [],
                                "report_path": report_path,
                                "summary": "no open reducible unsafe items",
                            }
                        Path(ledger_path).write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                        Path(report_path).parent.mkdir(parents=True, exist_ok=True)
                        Path(report_path).write_text("# Unsafe Refactor Report\n", encoding="utf-8")
                        self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "audit finished", "step": 1, "elapsed_sec": 0.1}))
                    elif name == "agentic_repair_round_01":
                        repair_tasks.append(task)
                        self_ref.assertIn("## Unsafe Refactor Repair Inputs", task)
                        self_ref.assertIn("unsafe_refactor_ledger", task)
                        self_ref.assertIn("unsafe_refactor_feedback_jsonl", task)
                        self_ref.assertIn("会改变 public observable semantics", task)
                        self_ref.assertNotIn("## Mandatory Unsafe Reduction / Rust-native Refactor", task)
                        self_ref.assertNotIn("必须读取并更新此绝对路径 JSON", task)
                    else:
                        self_ref.fail(f"unexpected agent run: {name}")
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            def accepted_semantic(**kwargs: object) -> dict[str, object]:
                return {
                    "mode": "required",
                    "round": kwargs["round_label"],
                    "status": "accepted",
                    "passed": True,
                    "accepted": True,
                    "verdict": "accepted",
                    "ledger_path": "",
                    "report_path": "",
                    "text_log_path": str(root / f"{kwargs['round_label']}_semantic.log"),
                    "open_blocker_ids": [],
                    "blocked_external_ids": [],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 0, "by_status": {}},
                    "semantic_obligation_count": 0,
                    "diagnostics": [],
                }

            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=1,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent.OpenAIActionRunner", FakeRunner), \
                mock.patch(
                    "scripts.agentic_repair.post_repair_agent._run_cargo_check",
                    side_effect=[
                        {"returncode": 0, "text_log_path": str(root / "initial_cargo.log")},
                        {"returncode": 0, "text_log_path": str(root / "round1_cargo.log")},
                        {"returncode": 0, "text_log_path": str(root / "final_cargo.log")},
                    ],
                ), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", side_effect=accepted_semantic):
                rc = run_agentic(args)

            self.assertEqual(rc, 0)
            summary = json.loads((output_dir / "post_repair_summary.json").read_text(encoding="utf-8"))
            first_round = summary["rounds"][0]
            self.assertEqual(summary["final_status"], "accepted")
            self.assertEqual(summary["accepted_round"], 1)
            self.assertEqual(len(repair_tasks), 1)
            self.assertEqual(audit_names, ["unsafe_refactor_audit", "unsafe_refactor_audit"])
            context = json.loads(Path(first_round["agent_context"]).read_text(encoding="utf-8"))
            self.assertTrue(context["unsafe_optimization"]["active"])
            self.assertTrue(context["rust_native_refactor"]["active"])
            self.assertTrue(context["unsafe_refactor_audit"]["active"])
            self.assertEqual(context["unsafe_refactor_audit"]["open_reducible_item_count"], 1)
            self.assertTrue(Path(context["information_paths"]["unsafe_refactor_ledger"]).is_file())
            self.assertTrue(Path(context["information_paths"]["unsafe_review_task_json"]).is_file())
            self.assertTrue(Path(context["information_paths"]["rust_native_refactor_task_json"]).is_file())
            review_payload = json.loads(Path(context["information_paths"]["unsafe_review_task_json"]).read_text(encoding="utf-8"))
            self.assertEqual(review_payload["schema_version"], "c2r_rust_native_refactor_task_v1")
            self.assertTrue(review_payload["items"])
            self.assertEqual(context["repair_progress"]["phase"], "compile_or_full_gate_repair")
            self.assertTrue(Path(context["information_paths"]["unsafe_scope_gate_json"]).is_file())
            self.assertTrue(Path(context["information_paths"]["abi_refactor_inventory_json"]).is_file())
            gate_bundle = json.loads(Path(first_round["gate_bundle"]).read_text(encoding="utf-8"))
            final_bundle = json.loads((output_dir / "final_gate_bundle.json").read_text(encoding="utf-8"))
            self.assertIn("unsafe_refactor_audit", gate_bundle)
            self.assertIn("unsafe_scope", final_bundle)

    def test_run_agentic_keeps_unsafe_task_after_final_verify_failure(self) -> None:
        """final verify 失败后，下一轮 repair context 仍保留 unsafe 辅助路径。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            repair_contexts: list[dict[str, object]] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def __init__(self, *args: object, **kwargs: object) -> None:
                    pass

                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str) -> FakeAgentRun:
                    if name == "unsafe_refactor_audit":
                        ledger_path = self_ref._path_from_task(task, "- unsafe_refactor_ledger.json:")
                        report_path = self_ref._path_from_task(task, "- Markdown report:")
                        fingerprint = self_ref._value_from_task(task, "- expected_source_fingerprint_sha256:")
                        payload = {
                            "schema_version": "c2r_unsafe_refactor_ledger_v1",
                            "status": "accepted",
                            "verdict": "accepted",
                            "done": True,
                            "source_fingerprint_sha256": fingerprint,
                            "items": [],
                            "open_reducible_items": [],
                            "hard_required_items": [{"id": "UNSAFE::demo::demo::deref", "evidence": ["src/lib.rs:1"]}],
                            "repair_feedback_consumed": [],
                            "report_path": report_path,
                            "summary": "no open reducible unsafe items",
                        }
                        Path(ledger_path).write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                        Path(report_path).parent.mkdir(parents=True, exist_ok=True)
                        Path(report_path).write_text("# Unsafe Refactor Report\n", encoding="utf-8")
                    elif name == "agentic_repair_round_01":
                        context_path = Path(self_ref._path_from_task(task, "- agent_context_path:"))
                        context = json.loads(context_path.read_text(encoding="utf-8"))
                        repair_contexts.append(context)
                        info = context["information_paths"]
                        for key in (
                            "unsafe_review_task_json",
                            "rust_native_refactor_task_json",
                            "unsafe_scope_gate_json",
                            "abi_refactor_inventory_json",
                            "unsafe_refactor_ledger",
                        ):
                            self_ref.assertTrue(Path(info[key]).is_file(), key)
                        self_ref.assertIn("可参考 unsafe scope 原始 JSON：", task)
                        self_ref.assertIn(str(info["unsafe_scope_gate_json"]), task)
                        self_ref.assertIn(str(info["abi_refactor_inventory_json"]), task)
                    else:
                        self_ref.fail(f"unexpected agent run: {name}")
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            def accepted_semantic(**kwargs: object) -> dict[str, object]:
                return {
                    "mode": "required",
                    "round": kwargs["round_label"],
                    "status": "accepted",
                    "passed": True,
                    "accepted": True,
                    "verdict": "accepted",
                    "ledger_path": "",
                    "report_path": "",
                    "text_log_path": str(root / f"{kwargs['round_label']}_semantic.log"),
                    "open_blocker_ids": [],
                    "blocked_external_ids": [],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 0, "by_status": {}},
                    "semantic_obligation_count": 0,
                    "diagnostics": [],
                }

            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=1,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent.OpenAIActionRunner", FakeRunner), \
                mock.patch(
                    "scripts.agentic_repair.post_repair_agent._run_cargo_check",
                    side_effect=[
                        {"returncode": 0, "text_log_path": str(root / "initial_cargo.log")},
                        {"returncode": 101, "text_log_path": str(root / "final_cargo.log")},
                        {"returncode": 101, "text_log_path": str(root / "round1_cargo.log")},
                    ],
                ), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", side_effect=accepted_semantic):
                rc = run_agentic(args)

            self.assertEqual(rc, 1)
            self.assertEqual(len(repair_contexts), 1)
            self.assertTrue(repair_contexts[0]["rust_native_refactor"]["active"])

    def test_run_agentic_retries_invalid_unsafe_auditor_output(self) -> None:
        """unsafe auditor 产物缺失时，框架反馈 diagnostics 并补齐重试。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            recorded_tasks: list[tuple[str, str]] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def __init__(self, *args: object, **kwargs: object) -> None:
                    pass

                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    recorded_tasks.append((name, task))
                    self_ref.assertEqual(name, "unsafe_refactor_audit")
                    self_ref.assertIn("# Unsafe Refactor Audit Task", task)
                    self_ref.assertTrue(callable(finish_validator))
                    retry_prompt = finish_validator({"finish_status": "done", "finish_message": "first finish", "step": 1, "elapsed_sec": 0.1})
                    self_ref.assertIsInstance(retry_prompt, str)
                    self_ref.assertIn("# Unsafe Refactor Audit Output Fix Required", retry_prompt)
                    self_ref.assertIn("unsafe refactor ledger missing or invalid", retry_prompt)
                    ledger_path = self_ref._path_from_task(retry_prompt, "- unsafe_refactor_ledger_json:")
                    report_path = self_ref._path_from_task(retry_prompt, "- unsafe_refactor_report:")
                    fingerprint = self_ref._value_from_task(task, "- expected_source_fingerprint_sha256:")
                    payload = {
                        "schema_version": "c2r_unsafe_refactor_ledger_v1",
                        "status": "accepted",
                        "verdict": "accepted",
                        "done": True,
                        "source_fingerprint_sha256": fingerprint,
                        "items": [
                            {
                                "id": "UNSAFE::demo::demo::deref",
                                "scope_ids": ["U0001"],
                                "file": "src/lib.rs",
                                "span": "1-1",
                                "function": "demo",
                                "classification": "hard_required",
                                "status": "hard_required",
                                "problem": "raw pointer dereference remains required",
                                "repair_instruction": "",
                                "must_preserve": ["raw pointer dereference result"],
                                "evidence": ["unsafe_scope:U0001", "src/lib.rs:1"],
                                "expected_effect": "none",
                            }
                        ],
                        "open_reducible_items": [],
                        "hard_required_items": [{"id": "UNSAFE::demo::demo::deref", "evidence": ["src/lib.rs:1"]}],
                        "repair_feedback_consumed": [],
                        "report_path": report_path,
                        "summary": "accepted after retry",
                    }
                    Path(ledger_path).write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                    Path(report_path).parent.mkdir(parents=True, exist_ok=True)
                    Path(report_path).write_text("# Unsafe Refactor Report\n", encoding="utf-8")
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "fixed", "step": 2, "elapsed_sec": 0.2}))
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            def accepted_semantic(**kwargs: object) -> dict[str, object]:
                return {
                    "mode": "required",
                    "round": kwargs["round_label"],
                    "status": "accepted",
                    "passed": True,
                    "accepted": True,
                    "verdict": "accepted",
                    "ledger_path": "",
                    "report_path": "",
                    "text_log_path": str(root / f"{kwargs['round_label']}_semantic.log"),
                    "open_blocker_ids": [],
                    "blocked_external_ids": [],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 0, "by_status": {}},
                    "semantic_obligation_count": 0,
                    "diagnostics": [],
                }

            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=1,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent.OpenAIActionRunner", FakeRunner), \
                mock.patch(
                    "scripts.agentic_repair.post_repair_agent._run_cargo_check",
                    side_effect=[
                        {"returncode": 0, "text_log_path": str(root / "initial_cargo.log")},
                        {"returncode": 0, "text_log_path": str(root / "final_cargo.log")},
                    ],
                ), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", side_effect=accepted_semantic):
                rc = run_agentic(args)

            self.assertEqual(rc, 0)
            self.assertEqual([name for name, _task in recorded_tasks], ["unsafe_refactor_audit"])
            summary = json.loads((output_dir / "post_repair_summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["final_status"], "accepted")
            audit = summary["rounds"][0]["unsafe_refactor_audit"]
            self.assertTrue(audit["accepted"])
            self.assertEqual(len(audit["audit_attempts"]), 2)

    def test_unsafe_refactor_audit_continues_for_uncovered_scopes(self) -> None:
        """unsafe auditor accepted 但漏掉 scope 时，复用历史补审后才能 accepted。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text(
                "pub fn a(p: *const i32) -> i32 { unsafe { *p } }\n"
                "pub fn b(p: *const i32) -> i32 { unsafe { *p + 1 } }\n",
                encoding="utf-8",
            )
            context_path = root / "agent_context.json"
            context_path.write_text(
                json.dumps(
                    {
                        "schema_version": "c2r_post_repair_agent_context_v1",
                        "information_paths": {"rendered_root": str(rendered)},
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            log_path = root / "post_repair" / "unsafe_refactor_audit_logs" / "initial_unsafe_refactor_audit.json"
            calls: list[tuple[str, str]] = []
            continued_base_names: list[str] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str, *, reused_session: bool = False) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")
                    self.reused_session = reused_session

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                        "reused_session": self.reused_session,
                    }

            def write_ledger(task: str, *, covered: list[str]) -> tuple[Path, Path]:
                ledger_path = Path(self._path_from_task(task, "- unsafe_refactor_ledger.json:"))
                report_path = Path(self._path_from_task(task, "- Markdown report:"))
                fingerprint = self._value_from_task(task, "- expected_source_fingerprint_sha256:")
                report_path.parent.mkdir(parents=True, exist_ok=True)
                report_path.write_text("# Unsafe Refactor Report\n", encoding="utf-8")
                ledger_path.parent.mkdir(parents=True, exist_ok=True)
                ledger_path.write_text(
                    json.dumps(
                        {
                            "schema_version": "c2r_unsafe_refactor_ledger_v1",
                            "status": "accepted",
                            "verdict": "accepted",
                            "done": True,
                            "source_fingerprint_sha256": fingerprint,
                            "items": [
                                {
                                    "id": f"UNSAFE::demo::{scope_id}",
                                    "file": "src/lib.rs",
                                    "span": "1-2",
                                    "function": "demo",
                                    "scope_ids": [scope_id],
                                    "classification": "hard_required",
                                    "status": "hard_required",
                                    "problem": "raw pointer dereference remains required",
                                    "repair_instruction": "",
                                    "must_preserve": ["raw pointer behavior"],
                                    "evidence": [f"unsafe_scope:{scope_id}", "src/lib.rs:1"],
                                    "expected_effect": "none",
                                }
                                for scope_id in covered
                            ],
                            "open_reducible_items": [],
                            "hard_required_items": covered,
                            "repair_feedback_consumed": [],
                            "report_path": str(report_path),
                            "summary": "accepted",
                        },
                        ensure_ascii=False,
                        indent=2,
                        sort_keys=True,
                    )
                    + "\n",
                    encoding="utf-8",
                )
                return ledger_path, report_path

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    del cwd
                    calls.append(("run", name))
                    self_ref.assertEqual(name, "unsafe_refactor_audit")
                    self_ref.assertIn("# Unsafe Refactor Audit Task", task)
                    self_ref.assertTrue(callable(finish_validator))
                    write_ledger(task, covered=["U0001"])
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "accepted", "step": 1, "elapsed_sec": 0.1}))
                    return FakeAgentRun(output_dir, name)

                def continue_run(
                    self,
                    *,
                    task: str,
                    cwd: Path,
                    output_dir: Path,
                    name: str,
                    base_name: str,
                    allowed_write_paths: tuple[Path, ...],
                ) -> FakeAgentRun:
                    del cwd, allowed_write_paths
                    calls.append(("continue_run", name))
                    continued_base_names.append(base_name)
                    self_ref.assertIn("# Unsafe Refactor Audit Coverage Continuation", task)
                    self_ref.assertIn("Missing Unsafe Scope IDs", task)
                    self_ref.assertIn("U0002", task)
                    write_ledger(task, covered=["U0001", "U0002"])
                    return FakeAgentRun(output_dir, name, reused_session=True)

            self_ref = self
            payload = _run_unsafe_refactor_audit(
                runner=FakeRunner(),
                round_label="initial",
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                context_path=context_path,
                log_path=log_path,
                suite="oss",
            )

            self.assertTrue(payload["accepted"], payload["diagnostics"])
            self.assertEqual(payload["unsafe_refactor_coverage_gaps"], [])
            self.assertEqual(calls, [("run", "unsafe_refactor_audit"), ("continue_run", "unsafe_refactor_audit_coverage_01")])
            self.assertEqual(continued_base_names, ["unsafe_refactor_audit"])
            self.assertEqual(len(payload["unsafe_refactor_coverage_continuations"]), 1)
            self.assertTrue(payload["unsafe_refactor_coverage_satisfied"])

    def test_unsafe_refactor_audit_records_residual_coverage_gaps_without_blocking(self) -> None:
        """unsafe coverage 补审耗尽后仍有缺口时只记录，不阻断 accepted。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text(
                "pub fn a(p: *const i32) -> i32 { unsafe { *p } }\n"
                "pub fn b(p: *const i32) -> i32 { unsafe { *p + 1 } }\n",
                encoding="utf-8",
            )
            context_path = root / "agent_context.json"
            context_path.write_text(json.dumps({"information_paths": {"rendered_root": str(rendered)}}, ensure_ascii=False), encoding="utf-8")
            log_path = root / "post_repair" / "unsafe_refactor_audit_logs" / "initial_unsafe_refactor_audit.json"

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {"returncode": 0, "stdout_path": str(self.stdout), "stderr_path": str(self.stderr), "timed_out": False}

            def write_partial_ledger(task: str) -> None:
                ledger_path = Path(self._path_from_task(task, "- unsafe_refactor_ledger.json:"))
                report_path = Path(self._path_from_task(task, "- Markdown report:"))
                fingerprint = self._value_from_task(task, "- expected_source_fingerprint_sha256:")
                report_path.parent.mkdir(parents=True, exist_ok=True)
                report_path.write_text("# Unsafe Refactor Report\n", encoding="utf-8")
                ledger_path.parent.mkdir(parents=True, exist_ok=True)
                ledger_path.write_text(
                    json.dumps(
                        {
                            "schema_version": "c2r_unsafe_refactor_ledger_v1",
                            "status": "accepted",
                            "verdict": "accepted",
                            "done": True,
                            "source_fingerprint_sha256": fingerprint,
                            "items": [
                                {
                                    "id": "UNSAFE::demo::U0001",
                                    "scope_ids": ["U0001"],
                                    "file": "src/lib.rs",
                                    "span": "1-1",
                                    "function": "a",
                                    "classification": "hard_required",
                                    "status": "hard_required",
                                    "problem": "raw pointer dereference remains required",
                                    "repair_instruction": "",
                                    "must_preserve": ["raw pointer behavior"],
                                    "evidence": ["unsafe_scope:U0001"],
                                    "expected_effect": "none",
                                }
                            ],
                            "open_reducible_items": [],
                            "hard_required_items": ["U0001"],
                            "repair_feedback_consumed": [],
                            "report_path": str(report_path),
                            "summary": "accepted but incomplete",
                        },
                        ensure_ascii=False,
                        indent=2,
                        sort_keys=True,
                    )
                    + "\n",
                    encoding="utf-8",
                )

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    del cwd
                    self_ref.assertEqual(name, "unsafe_refactor_audit")
                    self_ref.assertTrue(callable(finish_validator))
                    write_partial_ledger(task)
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "accepted", "step": 1, "elapsed_sec": 0.1}))
                    return FakeAgentRun(output_dir, name)

            self_ref = self
            with mock.patch("scripts.agentic_repair.post_repair_agent.UNSAFE_REFACTOR_AUDIT_COVERAGE_CONTINUATION_LIMIT", 0):
                payload = _run_unsafe_refactor_audit(
                    runner=FakeRunner(),
                    round_label="initial",
                    rendered_root=rendered,
                    manifest_path=rendered / "Cargo.toml",
                    context_path=context_path,
                    log_path=log_path,
                    suite="oss",
                )

            self.assertTrue(payload["accepted"])
            self.assertEqual(payload["status"], "accepted")
            self.assertEqual(payload["unsafe_refactor_coverage_gap_count"], 1)
            self.assertFalse(payload["unsafe_refactor_coverage_satisfied"])
            self.assertEqual(payload["diagnostics"], [])

    def test_run_agentic_rejects_after_three_invalid_unsafe_auditor_outputs(self) -> None:
        """unsafe auditor 输出 3 次修正后仍无效时，最终不能 accepted。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            recorded_tasks: list[str] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def __init__(self, *args: object, **kwargs: object) -> None:
                    pass

                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    recorded_tasks.append(name)
                    self_ref.assertEqual(name, "unsafe_refactor_audit")
                    self_ref.assertIn("# Unsafe Refactor Audit Task", task)
                    self_ref.assertTrue(callable(finish_validator))
                    for attempt_index in range(4):
                        retry_prompt = finish_validator(
                            {
                                "finish_status": "done",
                                "finish_message": f"invalid {attempt_index}",
                                "step": attempt_index + 1,
                                "elapsed_sec": 0.1 * (attempt_index + 1),
                            }
                        )
                        if attempt_index < 3:
                            self_ref.assertIsInstance(retry_prompt, str)
                            self_ref.assertIn("# Unsafe Refactor Audit Output Fix Required", retry_prompt)
                        else:
                            self_ref.assertIsNone(retry_prompt)
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            def accepted_semantic(**kwargs: object) -> dict[str, object]:
                return {
                    "mode": "required",
                    "round": kwargs["round_label"],
                    "status": "accepted",
                    "passed": True,
                    "accepted": True,
                    "verdict": "accepted",
                    "ledger_path": "",
                    "report_path": "",
                    "text_log_path": str(root / f"{kwargs['round_label']}_semantic.log"),
                    "open_blocker_ids": [],
                    "blocked_external_ids": [],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": [],
                    "semantic_obligation_summary": {"total": 0, "by_status": {}},
                    "semantic_obligation_count": 0,
                    "diagnostics": [],
                }

            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=0,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )

            with mock.patch("scripts.agentic_repair.post_repair_agent.OpenAIActionRunner", FakeRunner), \
                mock.patch(
                    "scripts.agentic_repair.post_repair_agent._run_cargo_check",
                    side_effect=[
                        {"returncode": 0, "text_log_path": str(root / "initial_cargo.log")},
                        {"returncode": 0, "text_log_path": str(root / "final_cargo.log")},
                    ],
                ), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", side_effect=accepted_semantic):
                rc = run_agentic(args)

            self.assertEqual(rc, 1)
            self.assertEqual(recorded_tasks, ["unsafe_refactor_audit"])
            summary = json.loads((output_dir / "post_repair_summary.json").read_text(encoding="utf-8"))
            self.assertEqual(len(summary["rounds"]), 1)
            audit = summary["rounds"][0]["unsafe_refactor_audit"]
            self.assertFalse(audit["accepted"])
            self.assertEqual(len(audit["audit_attempts"]), 4)
            self.assertEqual(summary["final_verify_summary"]["unsafe_refactor_failure_kind"], "unsafe_refactor_audit_not_accepted_after_repair_budget")

    def test_repair_context_keeps_blocked_external_optional_residual(self) -> None:
        """blocked_external 只作为可选参考残留，不进入主修复输入。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            rendered.mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src").mkdir()
            (rendered / "src" / "lib.rs").write_text("pub fn demo() {}\n", encoding="utf-8")

            report = workspace / "skeletons" / "demo" / "types_generation_report.json"
            report.parent.mkdir(parents=True)
            report.write_text(
                json.dumps(
                    {
                        "mode": "bindgen",
                        "success": True,
                        "compile_commands_loaded": True,
                        "ohos_root": str(DEFAULT_OHOS_ROOT.resolve()),
                        "ohos_project_rel": "base/demo",
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            ledger_path = root / "semantic_blockers_ledger.json"
            ledger_path.write_text(
                json.dumps(
                    {
                        "schema_version": "rust_semantic_blockers_ledger_v1",
                        "verdict": "accepted_with_residual_risks",
                        "semantic_obligations": [
                            {
                                "id": "EXT::demo::needs_binding",
                                "api": "demo",
                                "dimension": "dependency_call",
                                "status": "blocked_external",
                                "root_cause_cluster_id": "RC::demo::missing_binding",
                                "repair_target": "src/compat.rs",
                            }
                        ],
                        "root_cause_clusters": [
                            {
                                "id": "RC::demo::missing_binding",
                                "status": "blocked_external",
                                "affected_obligations": ["EXT::demo::needs_binding"],
                                "summary": "Missing C binding",
                                "repair_kind": "needs_external_binding",
                                "repair_strategy": "Search OHOS source and add the required binding or shim.",
                            }
                        ],
                        "blockers": [
                            {
                                "id": "SEM::demo::needs_binding",
                                "status": "blocked_external",
                                "summary": "Missing binding",
                                "repair_target": "src/compat.rs",
                            }
                        ],
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            gate_bundle = _write_gate_bundle(
                root / "gate_bundle.json",
                "repair_round_01",
                {"returncode": 0, "text_log_path": str(root / "cargo.log")},
                {"returncode": 0, "text_log_path": str(root / "clippy.log")},
                {"returncode": 0, "text_log_path": str(root / "ohos.log")},
                {
                    "gate": "semantic_audit",
                    "mode": "required",
                    "round": "repair_round_01",
                    "status": "rejected",
                    "passed": False,
                    "accepted": False,
                    "returncode": 1,
                    "verdict": "accepted_with_residual_risks",
                    "ledger_path": str(ledger_path),
                    "report_path": str(root / "semantic_audit_report.md"),
                    "text_log_path": str(root / "semantic_audit.log"),
                    "blocking_ids": [],
                    "open_blocker_ids": [],
                    "blocked_external_ids": ["SEM::demo::needs_binding"],
                    "open_obligation_ids": [],
                    "semantic_obligation_blocking_ids": [],
                    "blocked_external_obligation_ids": ["EXT::demo::needs_binding"],
                    "semantic_obligation_summary": {"total": 1, "by_status": {"blocked_external": 1}},
                    "semantic_obligation_count": 1,
                    "diagnostics": [],
                },
            )
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
            )

            context_path = _build_repair_context(
                args=args,
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                latest_gate_bundle_path=gate_bundle,
                round_index=1,
                output_dir=root / "post_repair",
            )

            context = json.loads(context_path.read_text(encoding="utf-8"))
            semantic_bundle = context["semantic_repair_bundle"]
            self.assertTrue(semantic_bundle["semantic_locked"])
            self.assertEqual(context["information_paths"]["ohos_root"], str(DEFAULT_OHOS_ROOT.resolve()))
            self.assertTrue(context["current_gates"]["accepted_by_gates"])
            self.assertEqual(context["current_gates"]["gate_statuses"]["semantic_audit"], "accepted")
            self.assertEqual(context["current_gates"]["semantic_audit_blocked_external_ids"], ["SEM::demo::needs_binding"])
            self.assertEqual([item["id"] for item in semantic_bundle["blocked_external_root_cause_clusters"]], ["RC::demo::missing_binding"])
            self.assertEqual(semantic_bundle["repair_root_cause_clusters"], [])
            self.assertEqual([item["id"] for item in semantic_bundle["blocked_external_obligations"]], ["EXT::demo::needs_binding"])
            self.assertEqual(semantic_bundle["repair_obligations"], [])
            self.assertEqual(context["repair_progress"]["phase"], "compile_or_full_gate_repair")
            self.assertEqual(context["repair_progress"]["recommended_focus"], [])
            self.assertTrue(
                _semantic_audit_passed(
                    {
                        "mode": "required",
                        "status": "rejected",
                        "accepted": False,
                        "passed": False,
                        "verdict": "accepted_with_residual_risks",
                        "open_blocker_ids": [],
                        "blocked_external_ids": ["SEM::demo::needs_binding"],
                        "open_obligation_ids": [],
                        "blocked_external_obligation_ids": ["EXT::demo::needs_binding"],
                        "diagnostics": [],
                    }
                )
            )

            prompt = _build_repair_prompt(context_path)
            self.assertIn(str(DEFAULT_OHOS_ROOT.resolve()), prompt)
            self.assertNotIn("RC::demo::missing_binding | status=blocked_external", prompt)
            self.assertIn("blocked_external_root_cause_clusters / blocked_external_obligations 是可选参考输入", prompt)
            self.assertNotIn("blocked_external 也是主修复输入的一部分", prompt)
            self.assertIn("src/compat.rs", prompt)
            self.assertIn("src/types.rs", prompt)
            self.assertNotIn("body_completeness", prompt)
            self.assertNotIn("body_target_scope", prompt)
            self.assertNotIn("unimplemented!()/todo!()", prompt)

    def test_semantic_audit_prompt_owns_mismatch_and_root_cause_discovery(self) -> None:
        """semantic audit prompt 负责发现语义不一致并维护 root-cause ledger。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prompt = _build_semantic_audit_prompt(
                rendered_root=root / "rendered",
                manifest_path=root / "rendered" / "Cargo.toml",
                context_path=root / "agent_context.json",
                ledger_path=root / "semantic_blockers_ledger.json",
                report_path=root / "semantic_audit_report.md",
            )
            self.assertIn("发现 high-confidence public observable semantic mismatch", prompt)
            self.assertIn("维护 obligation、root cause、ledger/report", prompt)
            self.assertIn("Phase A: prior blocker re-evaluation", prompt)
            self.assertIn("Phase B: exhaustive translated-result semantic discovery", prompt)
            self.assertIn("Phase A 全 fixed 不能直接 accept", prompt)
            self.assertIn("不能只抽样 3-5 个 anchor", prompt)
            self.assertIn("不要人为限制为 3 个", prompt)
            self.assertIn("Residual downgrade guard", prompt)
            self.assertIn("C/C++ header/public/exported API 声明存在但 Rust 没有等价 public entry", prompt)
            self.assertIn("每条 residual 必须写明 searched_paths", prompt)
            self.assertIn("coverage_summary", prompt)
            self.assertIn("information_paths.resolved_source_evidence", prompt)
            self.assertIn("不能替代源码证据", prompt)
            self.assertIn("status=`equivalent` / status=`redesigned_equivalent` / proved", prompt)
            self.assertIn("不能只基于中间产物摘要", prompt)
            self.assertIn("cpp_trace、cpp_evidence、searched_paths 优先写原始", prompt)
            self.assertIn("report 保持简短", prompt)
            self.assertIn("不要写长篇审查日志", prompt)
            self.assertIn("不要创建额外 trace/review/scratch 文档", prompt)
            self.assertIn("root_cause_clusters 是 repair agent 的唯一语义修复输入", prompt)
            self.assertIn("finish 前必须重新读取 semantic_blockers_ledger.json", prompt)
            self.assertIn("确认文件非空、可解析 JSON", prompt)

    def test_semantic_audit_retries_zero_byte_ledger_and_preserves_blocker(self) -> None:
        """semantic ledger 首次 0 字节时，框架应重试并保留真实 blocker。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo() -> i32 { 0 }\n", encoding="utf-8")
            context_path = root / "agent_context.json"
            context_path.write_text(json.dumps({"information_paths": {"rendered_root": str(rendered)}}), encoding="utf-8")
            log_path = root / "post_repair" / "semantic_audit_logs" / "initial_semantic_audit.json"
            recorded_names: list[str] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                    }

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    recorded_names.append(name)
                    self_ref.assertEqual(name, "semantic_audit")
                    self_ref.assertIn("# Semantic Audit Task", task)
                    self_ref.assertTrue(callable(finish_validator))
                    ledger_path = Path(self_ref._path_from_task(task, "- semantic_blockers_ledger.json:"))
                    ledger_path.parent.mkdir(parents=True, exist_ok=True)
                    ledger_path.write_bytes(b"")
                    retry_prompt = finish_validator({"finish_status": "done", "finish_message": "zero byte", "step": 1, "elapsed_sec": 0.1})
                    self_ref.assertIsInstance(retry_prompt, str)
                    self_ref.assertIn("# Semantic Audit Output Fix Required", retry_prompt)
                    self_ref.assertIn("semantic audit ledger missing or invalid", retry_prompt)
                    retry_ledger_path = Path(self_ref._path_from_task(retry_prompt, "- semantic_blockers_ledger_json:"))
                    report_path = Path(self_ref._path_from_task(retry_prompt, "- semantic_audit_report:"))
                    self_ref.assertEqual(retry_ledger_path, ledger_path)
                    ledger_path.write_text(
                        json.dumps(
                            {
                                "schema_version": "rust_semantic_blockers_ledger_v1",
                                "verdict": "rejected",
                                "semantic_obligations": [
                                    {
                                        "id": "EXT::demo::success_path",
                                        "api": "demo",
                                        "dimension": "success_path",
                                        "status": "blocking_mismatch",
                                        "cpp_trace": ["demo.c:1"],
                                        "rust_trace": ["src/lib.rs:1"],
                                        "observable_contract": "demo returns the C-compatible success value",
                                        "root_cause_cluster_id": "RC::demo::wrong_return",
                                        "repair_target": "src/lib.rs",
                                    }
                                ],
                                "root_cause_clusters": [
                                    {
                                        "id": "RC::demo::wrong_return",
                                        "status": "open",
                                        "affected_obligations": ["EXT::demo::success_path"],
                                        "summary": "Rust returns the wrong value.",
                                        "cpp_evidence": "demo.c:1",
                                        "rust_evidence": "src/lib.rs:1",
                                        "repair_kind": "rust_only",
                                        "repair_strategy": "Return the same value as C.",
                                    }
                                ],
                                "blockers": [
                                    {
                                        "id": "SEM::demo::wrong_return",
                                        "status": "open",
                                        "summary": "wrong return",
                                        "cpp_evidence": "demo.c:1",
                                        "rust_evidence": "src/lib.rs:1",
                                        "repair_target": "src/lib.rs",
                                    }
                                ],
                                "residual_risks": [],
                                "report_path": str(report_path),
                                "summary": "one blocker",
                            },
                            ensure_ascii=False,
                            indent=2,
                            sort_keys=True,
                        )
                        + "\n",
                        encoding="utf-8",
                    )
                    report_path.parent.mkdir(parents=True, exist_ok=True)
                    report_path.write_text("# Semantic Audit Report\n", encoding="utf-8")
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "fixed", "step": 2, "elapsed_sec": 0.2}))
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            payload = _run_semantic_audit(
                args=argparse.Namespace(),
                runner=FakeRunner(),
                round_label="initial",
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                context_path=context_path,
                log_path=log_path,
            )

            self.assertEqual(recorded_names, ["semantic_audit"])
            self.assertEqual(len(payload["audit_attempts"]), 2)
            self.assertEqual(payload["retry_limit"], 3)
            self.assertEqual(payload["status"], "rejected")
            self.assertEqual(payload["diagnostics"], [])
            self.assertEqual(payload["open_blocker_ids"], ["SEM::demo::wrong_return"])
            self.assertEqual(payload["semantic_obligation_blocking_ids"], ["EXT::demo::success_path"])
            self.assertFalse(payload["accepted"])
            self.assertTrue(_semantic_audit_result_executed(payload))
            self.assertGreater(Path(payload["ledger_path"]).stat().st_size, 0)
            self.assertTrue(Path(payload["report_path"]).is_file())

    def test_semantic_audit_retries_missing_coverage_summary(self) -> None:
        """accepted ledger 缺 coverage_summary 时应继续 semantic audit action loop 修格式。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo() -> i32 { 1 }\n", encoding="utf-8")
            context_path = root / "agent_context.json"
            context_path.write_text(json.dumps({"information_paths": {"rendered_root": str(rendered)}}), encoding="utf-8")
            log_path = root / "post_repair" / "semantic_audit_logs" / "initial_semantic_audit.json"
            recorded_names: list[str] = []

            def accepted_ledger(report_path: Path, *, include_coverage: bool) -> dict[str, object]:
                ledger: dict[str, object] = {
                    "schema_version": "rust_semantic_blockers_ledger_v1",
                    "verdict": "accepted",
                    "semantic_obligations": [],
                    "root_cause_clusters": [],
                    "blockers": [],
                    "residual_risks": [],
                    "report_path": str(report_path),
                    "summary": "accepted",
                }
                if include_coverage:
                    ledger["coverage_summary"] = {
                        "rust_source_files_scanned": 1,
                        "translated_functions_scanned": 1,
                        "public_or_exported_items_scanned": 1,
                        "private_helpers_scanned": 0,
                        "state_or_layout_items_scanned": 0,
                        "reviewed_non_observable": 0,
                        "unmapped_items": [],
                    }
                return ledger

            class FakeAgentRun:
                def __init__(self, out: Path, name: str) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")

                def to_dict(self) -> dict[str, object]:
                    return {"returncode": 0, "stdout_path": str(self.stdout), "stderr_path": str(self.stderr), "timed_out": False}

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    recorded_names.append(name)
                    self_ref.assertEqual(name, "semantic_audit")
                    self_ref.assertTrue(callable(finish_validator))
                    ledger_path = Path(self_ref._path_from_task(task, "- semantic_blockers_ledger.json:"))
                    report_path = Path(self_ref._path_from_task(task, "- Markdown report:"))
                    ledger_path.parent.mkdir(parents=True, exist_ok=True)
                    report_path.parent.mkdir(parents=True, exist_ok=True)
                    report_path.write_text("# Semantic Audit Report\n\nVerdict: accepted\n", encoding="utf-8")
                    ledger_path.write_text(json.dumps(accepted_ledger(report_path, include_coverage=False), ensure_ascii=False), encoding="utf-8")
                    retry_prompt = finish_validator({"finish_status": "done", "finish_message": "missing coverage", "step": 1, "elapsed_sec": 0.1})
                    self_ref.assertIsInstance(retry_prompt, str)
                    self_ref.assertIn("semantic audit coverage_summary must be an object", retry_prompt)
                    self_ref.assertIn("必须补齐 coverage_summary 的所有计数字段", retry_prompt)
                    ledger_path.write_text(json.dumps(accepted_ledger(report_path, include_coverage=True), ensure_ascii=False), encoding="utf-8")
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "fixed", "step": 2, "elapsed_sec": 0.2}))
                    return FakeAgentRun(output_dir, name)

            self_ref = self

            payload = _run_semantic_audit(
                args=argparse.Namespace(),
                runner=FakeRunner(),
                round_label="initial",
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                context_path=context_path,
                log_path=log_path,
            )

            self.assertEqual(recorded_names, ["semantic_audit"])
            self.assertEqual(len(payload["audit_attempts"]), 2)
            self.assertEqual(payload["status"], "accepted")
            self.assertEqual(payload["diagnostics"], [])
            self.assertTrue(payload["accepted"])
            self.assertTrue(_semantic_audit_result_executed(payload))

    def test_semantic_audit_coverage_gaps_continue_then_block_acceptance(self) -> None:
        """coverage gaps 会复用 semantic 历史补审；预算耗尽后必须阻断本轮 accepted。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo() -> i32 { 1 }\n", encoding="utf-8")
            context_path = root / "agent_context.json"
            context_path.write_text(
                json.dumps(
                    {
                        "schema_version": "c2r_post_repair_agent_context_v1",
                        "information_paths": {
                            "rendered_root": str(rendered),
                            "resolved_source_evidence": {
                                "schema_version": "c2r_resolved_source_evidence_v1",
                                "production_sources": [],
                                "public_headers": [],
                                "test_or_example_usage": [],
                                "unresolved_sources": [],
                                "counts": {},
                            },
                        },
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            log_path = root / "post_repair" / "semantic_audit_logs" / "initial_semantic_audit.json"
            calls: list[tuple[str, str]] = []
            continued_base_names: list[str] = []

            class FakeAgentRun:
                def __init__(self, out: Path, name: str, *, reused_session: bool = False) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")
                    self.reused_session = reused_session

                def to_dict(self) -> dict[str, object]:
                    return {
                        "returncode": 0,
                        "stdout_path": str(self.stdout),
                        "stderr_path": str(self.stderr),
                        "timed_out": False,
                        "reused_session": self.reused_session,
                    }

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    del cwd
                    calls.append(("run", name))
                    self_ref.assertEqual(name, "semantic_audit")
                    self_ref.assertIn("semantic_review_plan:", task)
                    self_ref.assertTrue(callable(finish_validator))
                    ledger_path = Path(self_ref._path_from_task(task, "- semantic_blockers_ledger.json:"))
                    report_path = Path(self_ref._path_from_task(task, "- Markdown report:"))
                    report_path.parent.mkdir(parents=True, exist_ok=True)
                    report_path.write_text("# Semantic Audit Report\n", encoding="utf-8")
                    ledger_path.parent.mkdir(parents=True, exist_ok=True)
                    ledger_path.write_text(
                        json.dumps(
                            {
                                "schema_version": "rust_semantic_blockers_ledger_v1",
                                "verdict": "accepted",
                                "semantic_obligations": [],
                                "root_cause_clusters": [],
                                "blockers": [],
                                "coverage_summary": {
                                    "rust_source_files_scanned": 1,
                                    "translated_functions_scanned": 1,
                                    "public_or_exported_items_scanned": 1,
                                    "private_helpers_scanned": 0,
                                    "state_or_layout_items_scanned": 0,
                                    "reviewed_non_observable": 0,
                                    "unmapped_items": [],
                                },
                                "residual_risks": [],
                                "report_path": str(report_path),
                                "summary": "accepted but uncovered",
                            },
                            ensure_ascii=False,
                        ),
                        encoding="utf-8",
                    )
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "accepted", "step": 1, "elapsed_sec": 0.1}))
                    return FakeAgentRun(output_dir, name)

                def continue_run(
                    self,
                    *,
                    task: str,
                    cwd: Path,
                    output_dir: Path,
                    name: str,
                    base_name: str,
                    allowed_write_paths: tuple[Path, ...],
                ) -> FakeAgentRun:
                    del cwd, allowed_write_paths
                    calls.append(("continue_run", name))
                    continued_base_names.append(base_name)
                    self_ref.assertIn("# Semantic Audit Coverage Continuation", task)
                    self_ref.assertIn("Missing Seed IDs", task)
                    return FakeAgentRun(output_dir, name, reused_session=True)

            self_ref = self
            payload = _run_semantic_audit(
                args=argparse.Namespace(),
                runner=FakeRunner(),
                round_label="initial",
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                context_path=context_path,
                log_path=log_path,
            )

            self.assertEqual(calls[0], ("run", "semantic_audit"))
            self.assertEqual(
                [call for call in calls if call[0] == "continue_run"],
                [("continue_run", f"semantic_audit_coverage_{index:02d}") for index in range(1, SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT + 1)],
            )
            self.assertEqual(continued_base_names, ["semantic_audit"] * SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT)
            self.assertFalse(payload["accepted"])
            self.assertEqual(payload["status"], "rejected")
            self.assertEqual(payload["returncode"], 1)
            self.assertIn("semantic audit coverage gaps remain after continuation: rust_file::src/lib.rs", payload["diagnostics"])
            self.assertEqual(len(payload["semantic_review_continuations"]), SEMANTIC_AUDIT_COVERAGE_CONTINUATION_LIMIT)
            self.assertEqual(payload["semantic_review_coverage_gaps"], ["rust_file::src/lib.rs"])

    def test_semantic_audit_coverage_continuation_mutation_still_blocks(self) -> None:
        """coverage 补审仍是只读审计；如果补审改 Rust 源码必须阻断。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "rendered"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo() -> i32 { 1 }\n", encoding="utf-8")
            context_path = root / "agent_context.json"
            context_path.write_text(
                json.dumps(
                    {
                        "schema_version": "c2r_post_repair_agent_context_v1",
                        "information_paths": {
                            "rendered_root": str(rendered),
                            "resolved_source_evidence": {
                                "schema_version": "c2r_resolved_source_evidence_v1",
                                "production_sources": [],
                                "public_headers": [],
                                "test_or_example_usage": [],
                                "unresolved_sources": [],
                                "counts": {},
                            },
                        },
                    },
                    ensure_ascii=False,
                ),
                encoding="utf-8",
            )
            log_path = root / "post_repair" / "semantic_audit_logs" / "initial_semantic_audit.json"

            class FakeAgentRun:
                def __init__(self, out: Path, name: str, *, reused_session: bool = False) -> None:
                    self.stdout = out / f"{name}.stdout.log"
                    self.stderr = out / f"{name}.stderr.log"
                    self.stdout.parent.mkdir(parents=True, exist_ok=True)
                    self.stdout.write_text("done\n", encoding="utf-8")
                    self.stderr.write_text("", encoding="utf-8")
                    self.reused_session = reused_session

                def to_dict(self) -> dict[str, object]:
                    return {"returncode": 0, "stdout_path": str(self.stdout), "stderr_path": str(self.stderr), "timed_out": False, "reused_session": self.reused_session}

            class FakeRunner:
                def run(self, *, task: str, cwd: Path, output_dir: Path, name: str, finish_validator: object | None = None) -> FakeAgentRun:
                    del cwd
                    self_ref.assertEqual(name, "semantic_audit")
                    self_ref.assertTrue(callable(finish_validator))
                    ledger_path = Path(self_ref._path_from_task(task, "- semantic_blockers_ledger.json:"))
                    report_path = Path(self_ref._path_from_task(task, "- Markdown report:"))
                    report_path.parent.mkdir(parents=True, exist_ok=True)
                    report_path.write_text("# Semantic Audit Report\n", encoding="utf-8")
                    ledger_path.parent.mkdir(parents=True, exist_ok=True)
                    ledger_path.write_text(
                        json.dumps(
                            {
                                "schema_version": "rust_semantic_blockers_ledger_v1",
                                "verdict": "accepted",
                                "semantic_obligations": [],
                                "root_cause_clusters": [],
                                "blockers": [],
                                "coverage_summary": {
                                    "rust_source_files_scanned": 1,
                                    "translated_functions_scanned": 1,
                                    "public_or_exported_items_scanned": 1,
                                    "private_helpers_scanned": 0,
                                    "state_or_layout_items_scanned": 0,
                                    "reviewed_non_observable": 0,
                                    "unmapped_items": [],
                                },
                                "residual_risks": [],
                                "report_path": str(report_path),
                                "summary": "accepted but uncovered",
                            },
                            ensure_ascii=False,
                        ),
                        encoding="utf-8",
                    )
                    self_ref.assertIsNone(finish_validator({"finish_status": "done", "finish_message": "accepted", "step": 1, "elapsed_sec": 0.1}))
                    return FakeAgentRun(output_dir, name)

                def continue_run(
                    self,
                    *,
                    task: str,
                    cwd: Path,
                    output_dir: Path,
                    name: str,
                    base_name: str,
                    allowed_write_paths: tuple[Path, ...],
                ) -> FakeAgentRun:
                    del task, output_dir, allowed_write_paths
                    self_ref.assertEqual(base_name, "semantic_audit")
                    self_ref.assertEqual(name, "semantic_audit_coverage_01")
                    (cwd / "src" / "lib.rs").write_text("pub fn demo() -> i32 { 2 }\n", encoding="utf-8")
                    ledger_path = root / "post_repair" / "semantic_blockers_ledger.json"
                    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
                    ledger["semantic_obligations"] = [
                        {
                            "id": "OBL::demo",
                            "status": "equivalent",
                            "seed_ids": ["rust_file::src/lib.rs"],
                            "evidence": [],
                        }
                    ]
                    ledger_path.write_text(json.dumps(ledger, ensure_ascii=False), encoding="utf-8")
                    return FakeAgentRun(root / "post_repair" / "semantic_audit_logs" / "initial_semantic_audit", name, reused_session=True)

            self_ref = self
            payload = _run_semantic_audit(
                args=argparse.Namespace(),
                runner=FakeRunner(),
                round_label="initial",
                rendered_root=rendered,
                manifest_path=rendered / "Cargo.toml",
                context_path=context_path,
                log_path=log_path,
            )

            self.assertFalse(payload["accepted"])
            self.assertEqual(payload["status"], "rejected")
            self.assertEqual(payload["semantic_review_coverage_gaps"], [])
            self.assertIn("semantic audit modified Rust project files: src/lib.rs", payload["diagnostics"])

    def test_run_agentic_prints_fixed_progress_lines(self) -> None:
        """post-repair 主循环应打印固定阶段进度，避免 agent 阶段无输出。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            workspace = root / "workspace"
            rendered = workspace / "final_projects" / "demo" / "translate_by_llm"
            (rendered / "src").mkdir(parents=True)
            (rendered / "Cargo.toml").write_text("[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", encoding="utf-8")
            (rendered / "src" / "lib.rs").write_text("pub fn demo(p: *const i32) -> i32 { unsafe { *p } }\n", encoding="utf-8")
            output_dir = workspace / "post_repair" / "demo" / "translate_by_llm"
            args = argparse.Namespace(
                workspace_dir=str(workspace),
                project="demo",
                llm_name="llm",
                source_project_root=str(workspace / "c_source" / "demo"),
                rendered_root=str(rendered),
                output_dir=str(output_dir),
                max_rounds=0,
                agent_step_limit=1,
                semantic_audit_step_limit=3,
                agent_timeout_sec=1.0,
                ohos_rustc="/tmp/missing-rustc",
                ohos_rust_target="x86_64-unknown-linux-ohos",
                allow_external_blockers=False,
            )
            semantic_payload = {
                "mode": "required",
                "round": "initial",
                "status": "accepted",
                "passed": True,
                "accepted": True,
                "verdict": "accepted",
                "ledger_path": "",
                "report_path": "",
                "text_log_path": str(root / "semantic.log"),
                "open_blocker_ids": [],
                "blocked_external_ids": [],
                "open_obligation_ids": [],
                "semantic_obligation_blocking_ids": [],
                "blocked_external_obligation_ids": [],
                "semantic_obligation_summary": {"total": 0, "by_status": {}},
                "semantic_obligation_count": 0,
                "diagnostics": [],
            }
            unsafe_refactor_payload = {
                "gate": "unsafe_refactor_audit",
                "mode": "required",
                "status": "accepted",
                "passed": True,
                "accepted": True,
                "verdict": "accepted",
                "ledger_path": str(output_dir / "unsafe_refactor_ledger.json"),
                "report_path": str(output_dir / "unsafe_refactor_report.md"),
                "text_log_path": str(root / "unsafe_refactor.log"),
                "open_reducible_items": [],
                "open_reducible_item_count": 0,
                "diagnostics": [],
            }

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_check", return_value={"returncode": 0, "text_log_path": str(root / "cargo.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_cargo_clippy_gate", return_value={"returncode": 0, "status": "not_configured", "text_log_path": str(root / "clippy.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_ohos_rustc_check", return_value={"returncode": 0, "text_log_path": str(root / "ohos.log")}), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_semantic_audit", return_value=semantic_payload), \
                mock.patch("scripts.agentic_repair.post_repair_agent._run_unsafe_refactor_audit", return_value=unsafe_refactor_payload):
                rc = run_agentic(args)

            output = stdout.getvalue()
            self.assertEqual(rc, 0)
            self.assertIn("[post-repair]", output)
            self.assertIn("run start project=demo", output)
            self.assertIn("initial: round start", output)
            self.assertIn("initial: cheap gates start", output)
            self.assertIn("initial: cheap gates end", output)
            self.assertIn("initial: semantic gate status=accepted", output)
            self.assertIn("initial: unsafe phase active", output)
            self.assertIn("initial: combined gate summary accepted=True", output)
            self.assertIn("initial: final verify start", output)
            self.assertIn("initial: final verify end accepted=True", output)
            self.assertIn("initial: run accepted", output)

    def test_repair_runner_write_roots_do_not_include_parent_when_depth_zero(self) -> None:
        """repair agent 日志目录 depth=0 时不能写 post_repair 根目录里的 ledger。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cwd = root / "crate"
            log_dir = root / "post_repair" / "agentic_logs" / "repair_round_01"
            cwd.mkdir(parents=True)
            log_dir.mkdir(parents=True)
            roots = _write_roots(cwd, log_dir, parent_depth=0)
            self.assertEqual(roots, (cwd.resolve(), log_dir.resolve()))

    def test_agent_returncode_keeps_zero(self) -> None:
        """returncode=0 必须保持成功，不能被 `or 1` 误判成失败。"""
        self.assertEqual(_agent_returncode({"returncode": 0}), 0)

    def test_action_runner_system_prompt_discourages_trace_files(self) -> None:
        """action runner 不应诱导 agent 生成额外 trace 文档。"""
        self.assertNotIn("semantic_repair_trace.md", ACTION_RUNNER_SYSTEM_PROMPT)
        self.assertIn("不要为了记录思考生成 trace/review/scratch", ACTION_RUNNER_SYSTEM_PROMPT)
        self.assertEqual(_agent_returncode({"returncode": "0"}), 0)
        self.assertEqual(_agent_returncode({}), 1)

    def test_action_runner_finish_validator_continues_same_action_loop_messages(self) -> None:
        """结构重试必须在同一 action loop 内追加 diagnostics 后继续。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cwd = root / "crate"
            out = root / "agent_logs"
            cwd.mkdir()
            calls: list[list[dict[str, object]]] = []

            def fake_generation(messages: list[dict[str, object]], return_usage: bool = True) -> dict[str, object]:
                calls.append(messages)
                content = {
                    "summary": "first finish" if len(calls) == 1 else "second finish",
                    "actions": [{"type": "finish", "status": "done", "message": f"finish {len(calls)}"}],
                }
                return {"content": json.dumps(content, ensure_ascii=False), "usage": {"total_tokens": 1}}

            validator_calls = 0

            def finish_validator(state: dict[str, object]) -> str | None:
                nonlocal validator_calls
                validator_calls += 1
                if validator_calls == 1:
                    return "# Semantic Audit Output Fix Required\n\nsemantic audit ledger missing or invalid"
                return None

            fake_generate_package = types.ModuleType("generate")
            fake_generation_module = types.ModuleType("generate.generation")
            fake_generation_module.generation = fake_generation
            fake_generate_package.generation = fake_generation_module
            with mock.patch.dict(sys.modules, {"generate": fake_generate_package, "generate.generation": fake_generation_module}):
                result = OpenAIActionRunner(step_limit=4, timeout_sec=30).run(
                    task="# Semantic Audit Task\n\ninitial task",
                    cwd=cwd,
                    output_dir=out,
                    name="semantic_audit",
                    finish_validator=finish_validator,
                )

            self.assertEqual(result.returncode, 0)
            self.assertEqual(validator_calls, 2)
            self.assertEqual(len(calls), 2)
            second_joined = "\n".join(str(item["content"]) for item in calls[1])
            self.assertIn("# Semantic Audit Task", second_joined)
            self.assertIn('"summary":"first finish"', second_joined)
            self.assertIn('"observations":[{"message":"finish 1"', second_joined)
            self.assertIn("# Semantic Audit Output Fix Required", second_joined)
            self.assertIn("semantic audit ledger missing or invalid", second_joined)

    def test_action_runner_continue_run_reuses_previous_messages(self) -> None:
        """跨 run continuation 必须复用 base action loop 的历史 messages。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cwd = root / "crate"
            out = root / "agent_logs"
            cwd.mkdir()
            calls: list[list[dict[str, object]]] = []

            def fake_generation(messages: list[dict[str, object]], return_usage: bool = True) -> dict[str, object]:
                calls.append(messages)
                content = {
                    "summary": f"finish {len(calls)}",
                    "actions": [{"type": "finish", "status": "done", "message": f"finish {len(calls)}"}],
                }
                return {"content": json.dumps(content, ensure_ascii=False), "usage": {"total_tokens": 1}}

            fake_generate_package = types.ModuleType("generate")
            fake_generation_module = types.ModuleType("generate.generation")
            fake_generation_module.generation = fake_generation
            fake_generate_package.generation = fake_generation_module
            runner = OpenAIActionRunner(step_limit=2, timeout_sec=30)
            with mock.patch.dict(sys.modules, {"generate": fake_generate_package, "generate.generation": fake_generation_module}):
                first = runner.run(task="# Semantic Audit Task\n\ninitial task", cwd=cwd, output_dir=out / "initial", name="semantic_audit")
                second = runner.continue_run(
                    task="# Semantic Audit Coverage Continuation\n\nmissing seeds",
                    cwd=cwd,
                    output_dir=out / "continuation",
                    name="semantic_audit_coverage_01",
                    base_name="semantic_audit",
                )

            self.assertEqual(first.returncode, 0)
            self.assertEqual(second.returncode, 0)
            self.assertTrue(second.reused_session)
            self.assertEqual(len(calls), 2)
            second_joined = "\n".join(str(item["content"]) for item in calls[1])
            self.assertIn("# Semantic Audit Task", second_joined)
            self.assertIn('"summary":"finish 1"', second_joined)
            self.assertIn("# Semantic Audit Coverage Continuation", second_joined)

    def test_post_repair_rule_fix_is_disabled_by_default(self) -> None:
        """默认后修复链路不跑具体规则修补。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "crate"
            src = rendered / "src"
            src.mkdir(parents=True)
            compat = src / "compat.rs"
            compat.write_text("pub mod ffi {}\n", encoding="utf-8")

            with mock.patch.dict("os.environ", {}, clear=True):
                result = _apply_post_repair_rule_fixes(
                    rendered,
                    "error[E0425]: cannot find function `AnyMissingSymbol`",
                    root / "post_repair",
                    "round",
                )

            self.assertEqual(result["status"], "skipped")
            self.assertEqual(compat.read_text(encoding="utf-8"), "pub mod ffi {}\n")

    def test_post_repair_rule_fix_applies_dlist_and_pointer_fixes(self) -> None:
        """post-repair gate 错误应先经过确定性规则修复。"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rendered = root / "crate"
            src = rendered / "src"
            src.mkdir(parents=True)
            (src / "compat.rs").write_text("pub mod ffi {}\n", encoding="utf-8")
            (src / "lib.rs").write_text(
                "pub mod compat;\n"
                "pub mod types;\n"
                "pub fn demo() {\n"
                "    unsafe { crate::compat::OsalMutexLock(&(*notifier).mutex); }\n"
                "}\n",
                encoding="utf-8",
            )
            error = """
error[E0425]: cannot find function, tuple struct or tuple variant `DListForEachEntry` in module `crate::compat`
  --> src/lib.rs:4:21
error[E0308]: mismatched types
  --> src/lib.rs:4:44
   |
4 |     unsafe { crate::compat::OsalMutexLock(&(*notifier).mutex); }
   |                                            ^^^^^^^^^^^^^^^^^^ types differ in mutability
   = note: expected raw pointer `*mut OsalMutex`
                 found reference `&OsalMutex`
"""

            with mock.patch.dict("os.environ", {"C2R_POST_REPAIR_RULE_FIX": "1"}):
                result = _apply_post_repair_rule_fixes(rendered, error, root / "post_repair", "round")

            self.assertEqual(result["status"], "applied")
            self.assertIn(str((src / "compat.rs").resolve()), result["changed_files"])
            self.assertIn(str((src / "lib.rs").resolve()), result["changed_files"])
            self.assertIn("DListForEachEntry", (src / "compat.rs").read_text(encoding="utf-8"))
            self.assertIn("core::ptr::addr_of_mut!((*notifier).mutex)", (src / "lib.rs").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
