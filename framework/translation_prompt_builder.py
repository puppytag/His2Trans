"""构建单元翻译和单元修复的结构化 LLM prompt。"""

from __future__ import annotations

import hashlib
import json
import os
from typing import Any


UNIT_TRANSLATION_TASK_CONTRACT = """# Unit Translation Contract

## Objective
Translate exactly one C/C++ function into one Rust item that compiles in the provided crate.

## Context Layout
The following messages are organized as: stable Rust Compilation Context (when available) -> function-specific Rust Context -> Target Rust Signature -> C/C++ Source -> Constraints/Hints -> Function-Specific Evidence -> Retrieved Knowledge -> Output Contract.

## Acceptance
- The output must match the required Rust signature.
- The output must use only available crate types, globals, compat externs, and internal module paths.
- The output must be raw Rust code only.
- Do not use Markdown fences, prose explanations, comments about the task, or extra text outside the Rust item.

## Evidence Policy
- Treat the Rust compilation context as the available API surface.
- Treat dependency-closure facts as high-confidence evidence.
- Treat the target source layer as authoritative for the function body. If the target source is preprocessed `.i`, original-source snippets are explanatory only and must not reintroduce unexpanded macro calls or unavailable helper APIs.
- Evidence blocks may carry `source_layer=preprocessed|original|heuristic`; prefer `preprocessed` and current crate APIs when evidence conflicts.
- Treat hints and gaps only as investigation aids; do not invent ABI, struct layout, or external signatures from them.
- RAG/oracle snippets are advisory; prefer the target source and crate context when evidence conflicts.

## Safe Rust Preference
- Compilation, required signature, ABI-sensitive types, control flow, error paths, logging, cleanup intent, global state updates, and externally visible side effects come first.
- Prefer safe Rust constructs such as references, slices, Vec, Box, String, and standard library APIs only when the replacement is directly supported by the current evidence and does not require guessing missing types, ABI, layout, ownership, or external API behavior.
- Unsafe is allowed when required for FFI, raw pointer dereference, callback invocation, global mutable state, libc/platform C API interaction, layout-sensitive casts, or ABI compatibility.
- If a safe rewrite is uncertain, keep the low-level representation and isolate unsafe in the smallest necessary block.
- Do not change the required function signature, add new dependencies, invent extern declarations, or rewrite caller contracts for the purpose of reducing unsafe.

## Required Work
1. Read "Rust Compilation Context" before choosing names, modules, constants, fields, and call paths.
2. Read "Target Rust Signature" and preserve it exactly.
3. Translate "C/C++ Source To Translate" directly.
4. Use "Function-Specific Evidence" and retrieved knowledge only to resolve callees, globals, pointer contracts, and known gaps.
5. Return only the target Rust item."""


UNIT_REPAIR_TASK_CONTRACT = """# Unit Repair Contract

## Objective
Fix exactly one Rust translation so it compiles in the provided crate while preserving the intended C/C++ behavior as much as the available evidence permits.

## Context Layout
The following messages are organized as: stable Rust Compilation Context (when available) -> function-specific Rust Context -> Required Rust Signature -> Current Rust Code -> Compilation Error -> Error Analysis -> Previous Repair Attempts -> Original C/C++ Source -> Constraints/Hints -> Function-Specific Evidence -> Output Contract.

## Acceptance
- The output must match the required Rust signature.
- The output must fix the current compiler error without adding extra Rust items.
- The output must be raw Rust code only.
- Do not use Markdown fences, prose explanations, comments about the task, or extra text outside the Rust item.

## Evidence Policy
- Compiler diagnostics identify the immediate blocker.
- The Rust compilation context is the available API surface.
- Repair history shows approaches that failed; use it to avoid repeating the same mistake.
- Historical prompts and failed translations are diagnostic evidence only; do not treat names, modules, externs, constants, or helper APIs from them as available unless they exist in the current crate or current gate artifacts.
- If source evidence layers conflict, current Rust crate state, current compiler diagnostics, dependency-closure facts, and preprocessed target source take precedence over original-source snippets.
- Do not invent struct layouts, external ABI, modules, imports, or extern blocks when the crate evidence does not support them.

## Safe Rust Preference
- Fixing the current compiler diagnostics and preserving the required signature, ABI-sensitive types, control flow, error paths, logging, cleanup intent, global state updates, and externally visible side effects come first.
- Do not remove unsafe just to make the code look idiomatic; only prefer safe Rust when the local replacement is obvious, evidence-backed, and directly related to the repair.
- Unsafe is allowed when required for FFI, raw pointer dereference, callback invocation, global mutable state, libc/platform C API interaction, layout-sensitive casts, or ABI compatibility.
- If a safe rewrite is uncertain, keep the existing low-level representation and isolate unsafe in the smallest necessary block.
- Do not change the required function signature, add new dependencies, invent extern declarations, or rewrite caller contracts for the purpose of reducing unsafe.

## Required Work
1. Read "Rust Compilation Context" and "Required Rust Signature" first.
2. Compare "Current Rust Code With Errors" against "Original C/C++ Source".
3. Fix the specific compiler diagnostics and any directly related local errors using the provided evidence.
4. Keep the result to one target Rust item."""


def _section(title: str, body: str, *, fence: str | None = None) -> str:
    """渲染一个非空 prompt section。"""
    text = (body or "").strip()
    if not text:
        return ""
    if fence:
        return f"## {title}\n```{fence}\n{text}\n```"
    return f"## {title}\n{text}"


def _join_sections(sections: list[str]) -> str:
    """拼接非空 section。"""
    return "\n\n".join(section for section in sections if section.strip()).strip()


def _target_project_context() -> str:
    """返回当前 suite 的稳定翻译目标说明。"""
    suite = str(os.environ.get("C2R_PROJECT_SUITE") or os.environ.get("PROJECT_SUITE") or "ohos").strip().lower()
    if suite == "oss":
        return "当前目标是独立 Rust-native OSS 项目。项目内已定义函数使用 Rust ABI；只有真实外部 FFI 或 callback/function-pointer 边界保留 C ABI。"
    return "当前目标是 OHOS 集成项目。必须保持目标签名中要求的 C ABI、平台调用和外部集成约束。"


def _hash_messages(messages: list[dict[str, str]]) -> str:
    """计算稳定的 prompt hash。"""
    payload = json.dumps(messages, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def _split_rust_context(rust_context: str, stable_prefix_chars: int) -> tuple[str, str]:
    """按已知稳定前缀拆分 Rust 上下文。"""
    text = (rust_context or "").strip()
    if not text:
        return "", ""
    try:
        cut = int(stable_prefix_chars or 0)
    except Exception:
        cut = 0
    if cut <= 0:
        return "", text
    cut = min(cut, len(text))
    stable = text[:cut].strip()
    dynamic = text[cut:].strip()
    return stable, dynamic


def _cache_metadata(messages: list[dict[str, str]], *, static_message_count: int, stable_prefix_chars: int = 0) -> dict[str, Any]:
    """生成用于分析远端 prompt cache 的最小元数据。"""
    static_count = max(0, min(static_message_count, len(messages)))
    static_chars = sum(len(message.get("content") or "") for message in messages[:static_count])
    full_chars = sum(len(message.get("content") or "") for message in messages)
    return {
        "prompt_cache": {
            "static_message_count": static_count,
            "static_message_chars": static_chars,
            "stable_dynamic_prefix_chars": max(0, int(stable_prefix_chars or 0)),
            "dynamic_suffix_chars": max(0, full_chars - static_chars),
            "static_prefix_hash": _hash_messages(messages[:static_count]) if static_count else "",
            "full_prompt_hash": _hash_messages(messages),
        }
    }


def build_unit_translation_prompt(
    *,
    system_prompt: str,
    target_signature: str,
    c_code: str,
    rust_context: str,
    function_evidence: str,
    rag_knowledge: str,
    output_requirements: str,
    opaque_type_warning: str = "",
    accessor_shim_hints: str = "",
    stable_rust_context_prefix_chars: int = 0,
) -> tuple[list[dict[str, str]], dict[str, Any]]:
    """构建单元翻译 messages 和缓存分析元数据。"""
    stable_context, dynamic_context = _split_rust_context(rust_context, stable_rust_context_prefix_chars)
    dynamic_prompt = _join_sections(
        [
            _section(
                "Function-Specific Rust Context" if stable_context else "Rust Compilation Context",
                dynamic_context,
                fence="rust",
            ),
            _section("Target Rust Signature", target_signature, fence="rust"),
            _section("C/C++ Source To Translate", c_code, fence="cpp"),
            _section("Opaque Type Constraints", opaque_type_warning),
            _section("Accessor Shim Hints", accessor_shim_hints, fence="rust"),
            _section("Function-Specific Evidence", function_evidence, fence="text"),
            _section("Retrieved Translation Knowledge", rag_knowledge),
            _section("Output Contract", output_requirements),
        ]
    )
    messages = [
        {"role": "system", "content": system_prompt.strip()},
        {"role": "user", "content": UNIT_TRANSLATION_TASK_CONTRACT.strip() + "\n\n## Target Project Context\n" + _target_project_context()},
    ]
    if stable_context:
        messages.append(
            {
                "role": "user",
                "content": _section("Rust Compilation Context (stable crate API)", stable_context, fence="rust"),
            }
        )
    messages.append({"role": "user", "content": dynamic_prompt})
    static_message_count = 3 if stable_context else 2
    metadata = _cache_metadata(messages, static_message_count=static_message_count, stable_prefix_chars=0)
    metadata["prompt_cache"]["stable_context_chars"] = len(stable_context)
    metadata["prompt_cache"]["dynamic_rust_context_chars"] = len(dynamic_context)
    metadata["prompt_layout"] = "unit_translation_v3"
    return messages, metadata


def build_unit_repair_prompt(
    *,
    system_prompt: str,
    target_signature: str,
    c_code: str,
    current_code: str,
    error_msg: str,
    error_analysis: str,
    rust_context: str,
    function_evidence: str,
    output_requirements: str,
    history_hint: str = "",
    opaque_type_info: str = "",
    stable_rust_context_prefix_chars: int = 0,
) -> tuple[list[dict[str, str]], dict[str, Any]]:
    """构建单元修复 messages 和缓存分析元数据。"""
    stable_context, dynamic_context = _split_rust_context(rust_context, stable_rust_context_prefix_chars)
    dynamic_prompt = _join_sections(
        [
            _section(
                "Function-Specific Rust Context" if stable_context else "Rust Compilation Context",
                dynamic_context,
                fence="rust",
            ),
            _section("Required Rust Signature", target_signature, fence="rust"),
            _section("Current Rust Code With Errors", current_code, fence="rust"),
            _section("Compilation Error", error_msg, fence="text"),
            _section("Error Analysis", error_analysis),
            _section("Previous Repair Attempts", history_hint),
            _section("Original C/C++ Source", c_code, fence="cpp"),
            _section("Opaque Type Constraints", opaque_type_info),
            _section("Function-Specific Evidence", function_evidence, fence="text"),
            _section("Output Contract", output_requirements),
        ]
    )
    messages = [
        {"role": "system", "content": system_prompt.strip()},
        {"role": "user", "content": UNIT_REPAIR_TASK_CONTRACT.strip() + "\n\n## Target Project Context\n" + _target_project_context()},
    ]
    if stable_context:
        messages.append(
            {
                "role": "user",
                "content": _section("Rust Compilation Context (stable crate API)", stable_context, fence="rust"),
            }
        )
    messages.append({"role": "user", "content": dynamic_prompt})
    static_message_count = 3 if stable_context else 2
    metadata = _cache_metadata(messages, static_message_count=static_message_count, stable_prefix_chars=0)
    metadata["prompt_cache"]["stable_context_chars"] = len(stable_context)
    metadata["prompt_cache"]["dynamic_rust_context_chars"] = len(dynamic_context)
    metadata["prompt_layout"] = "unit_repair_v3"
    return messages, metadata
