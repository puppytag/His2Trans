#!/usr/bin/env python3
"""生成 ABI 保留与 Rust-native 重构候选清单，供后置重构 agent 使用。"""

from __future__ import annotations

import argparse
import bisect
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.agentic_repair.unsafe_scope_gate import IGNORED_DIRS, IGNORED_GENERATED_DIRS, _mask_comments_and_strings

ABI_REFACTOR_INVENTORY = "abi_refactor_inventory"
C_SOURCE_SUFFIXES = {".c", ".cc", ".cpp", ".cxx"}
C_HEADER_SUFFIXES = {".h", ".hh", ".hpp", ".hxx"}
ABI_IGNORED_GENERATED_DIRS = IGNORED_GENERATED_DIRS | {
    ".c2r_bindgen_extern_vars",
    ".c2r_bindgen_globals",
}
SUPPORT_RUST_FILES = {
    "build.rs",
    "src/compat.rs",
    "src/compatibility.rs",
    "src/globals.rs",
    "src/main.rs",
    "src/types.rs",
}
RUST_FN_RE = re.compile(
    r"(?m)^(?P<attrs>(?:\s*#\[[^\n]*\]\s*\n)*)\s*"
    r"(?P<vis>pub(?:\([^)]*\))?\s+)?"
    r"(?P<unsafe>unsafe\s+)?"
    r"(?P<extern>extern\s+(?:\"[^\"]+\"|\s+)?\s*)?"
    r"fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\("
)
C_FN_NAME_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(")
C_CONTROL_WORDS = {
    "if",
    "for",
    "while",
    "switch",
    "return",
    "sizeof",
    "defined",
    "catch",
}
PLACEHOLDER_RE = re.compile(
    r"(?i)\b("
    r"unimplemented!|todo!|panic!|placeholder|not available|unavailable|no-?op|"
    r"omitted|skip call|assume success|fallback|allow compilation|simulate|dummy|"
    r"null as placeholder|not implemented|stub"
    r")\b"
)


def _normalize_suite(suite: str | None) -> str:
    """规范化实验 suite；默认保持 OHOS/旧行为。"""
    value = str(suite or "").strip().lower()
    return value if value in {"oss", "ohos"} else "ohos"


def _rust_native_refactor_policy(suite: str = "ohos") -> dict[str, list[str]]:
    """返回 Rust-native 重构阶段需要遵守的 ABI 边界和可重写范围。"""
    suite = _normalize_suite(suite)
    if suite == "oss":
        return {
            "preserve": [
                "safe Rust observable behavior for the standalone project",
                "public Rust API behavior chosen by the translated crate",
                "algorithm results, return values, resource lifetimes, and side effects observable through Rust callers",
            ],
            "may_rewrite": [
                "C external ABI shape when it is only needed by source tests or the original C entry points",
                "raw-pointer public entry points that can become safe Rust APIs",
                "private/internal data structures and state representation",
                "static/private helpers, private structs, intrusive lists, and hand-written global state",
                "internal helper APIs and control flow when the observable behavior remains equivalent",
            ],
            "preferred_internal_rewrites": [
                "target raw unsafe = 0 or near 0 for small standalone OSS/C projects when semantics allow",
                "prefer safe Rust APIs and safe cores over extern C raw-pointer APIs",
                "use Rust-owned data structures or safe containers for private state when semantics allow",
                "use NonNull/newtype wrappers and narrow unsafe helpers only when safe Rust cannot express the behavior",
                "delete confirmed unused generated or translated symbols after checking references",
            ],
        }
    return {
        "preserve": [
            "public ABI, exported symbols, and extern calling conventions",
            "public repr(C) type layouts that external callers observe",
            "caller/test observable behavior, including return codes, out-params, callbacks, locks, resources, and side effects",
        ],
        "may_rewrite": [
            "OHOS project internals when no public ABI or platform-observable behavior requires the C shape",
            "private/internal data structures and state representation",
            "static/private helpers, private structs, intrusive lists, and hand-written global state",
            "internal helper APIs and control flow when the observable behavior remains equivalent",
        ],
        "preferred_internal_rewrites": [
            "reduce unnecessary unsafe while preserving OHOS integration ABI and platform behavior",
            "move public extern C wrappers to thin ABI thunks around Rust-native safe cores",
            "use Rust-owned registries or safe containers for private state when semantics allow",
            "use NonNull/newtype wrappers and narrow unsafe helpers instead of broad unsafe blocks",
            "delete confirmed unused generated or translated symbols after checking references",
        ],
    }


@dataclass(frozen=True)
class RustFunction:
    """保存 Rust 函数声明的轻量信息。"""

    name: str
    file: str
    abs_file: str
    line: int
    is_pub: bool
    is_extern_c: bool
    is_unsafe_fn: bool
    no_mangle: bool


def _write_json(path: Path, payload: Any) -> None:
    """写入稳定 JSON。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def _line_starts(text: str) -> list[int]:
    """生成行首偏移表。"""
    starts = [0]
    for match in re.finditer("\n", text):
        starts.append(match.end())
    return starts


def _line_no(starts: list[int], offset: int) -> int:
    """把字符偏移转换为 1-based 行号。"""
    return bisect.bisect_right(starts, offset)


def _iter_rust_files(crate_dir: Path) -> list[Path]:
    """列出当前 crate 中参与重构分析的 Rust 文件。"""
    files: list[Path] = []
    for path in sorted(crate_dir.rglob("*.rs")):
        rel = path.relative_to(crate_dir)
        if any(part in IGNORED_DIRS or part in ABI_IGNORED_GENERATED_DIRS for part in rel.parts):
            continue
        files.append(path)
    return files


def _is_support_rust_file(rel: str) -> bool:
    """判断 Rust 文件是否属于框架支撑层而非翻译实现。"""
    return rel in SUPPORT_RUST_FILES or rel.startswith("src/__c2r_generated/")


def _iter_c_files(root: Path | None) -> list[Path]:
    """列出 C/C++ 源码与头文件。"""
    if root is None or not root.is_dir():
        return []
    files: list[Path] = []
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        if path.suffix.lower() in C_SOURCE_SUFFIXES | C_HEADER_SUFFIXES:
            files.append(path)
    return files


def _is_test_c_file(path: Path, root: Path) -> bool:
    """按 source root 下的相对路径判断 C/C++ 文件是否为测试代码。"""
    try:
        rel = path.relative_to(root)
    except ValueError:
        rel = Path(path.name)
    parts = [part.lower() for part in rel.parts]
    ignored_dataset_parts = {"with_test", "src_test_no_include"}
    test_dirs = {"test", "tests", "unittest", "unittests", "gtest", "gtests", "testcase", "testcases"}
    if any(part in test_dirs for part in parts if part not in ignored_dataset_parts):
        return True
    stem = path.stem.lower()
    return bool(re.search(r"(?:^|[_-])(?:test|unittest|gtest)(?:$|[_-])", stem) or stem.endswith("_test") or stem.endswith("_unittest"))


def _strip_c_comments_and_strings(text: str) -> str:
    """粗略屏蔽 C/C++ 注释和字符串，避免误判函数声明。"""
    chars = list(text)
    index = 0
    while index < len(text):
        if text.startswith("//", index):
            end = text.find("\n", index)
            end = len(text) if end < 0 else end
            for i in range(index, end):
                chars[i] = " "
            index = end
            continue
        if text.startswith("/*", index):
            end = text.find("*/", index + 2)
            end = len(text) if end < 0 else end + 2
            for i in range(index, end):
                if chars[i] != "\n":
                    chars[i] = " "
            index = end
            continue
        if text[index] in {"\"", "'"}:
            quote = text[index]
            cursor = index + 1
            while cursor < len(text):
                if text[cursor] == "\\":
                    cursor += 2
                    continue
                if text[cursor] == quote:
                    cursor += 1
                    break
                cursor += 1
            for i in range(index, min(cursor, len(text))):
                if chars[i] != "\n":
                    chars[i] = " "
            index = cursor
            continue
        index += 1
    return "".join(chars)


def _parse_rust_functions(crate_dir: Path) -> tuple[list[RustFunction], dict[str, str]]:
    """解析 Rust 函数和源码文本。"""
    functions: list[RustFunction] = []
    rust_texts: dict[str, str] = {}
    crate_dir = crate_dir.expanduser().resolve()
    for path in _iter_rust_files(crate_dir):
        rel = path.relative_to(crate_dir).as_posix()
        text = path.read_text(encoding="utf-8", errors="replace")
        rust_texts[rel] = text
        masked = _mask_comments_and_strings(text)
        starts = _line_starts(text)
        for match in RUST_FN_RE.finditer(masked):
            attrs = match.group("attrs") or ""
            original_decl = text[match.start() : min(len(text), match.end() + 32)]
            extern_kw = bool(match.group("extern")) or bool(re.search(r"\bextern\b", original_decl))
            functions.append(
                RustFunction(
                    name=match.group("name"),
                    file=rel,
                    abs_file=str(path.resolve()),
                    line=_line_no(starts, match.start()),
                    is_pub=bool(match.group("vis")),
                    is_extern_c=extern_kw and bool(re.search(r'\bextern\s+"C"', original_decl)),
                    is_unsafe_fn=bool(match.group("unsafe")),
                    no_mangle=bool(re.search(r"no_mangle|export_name", attrs)),
                )
            )
    return functions, rust_texts


def _definition_line_spans(masked: str, name: str) -> list[tuple[int, int]]:
    """返回 Rust 函数定义的大致行范围，便于排除自身引用。"""
    spans: list[tuple[int, int]] = []
    starts = _line_starts(masked)
    pattern = re.compile(
        rf"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?"
        rf"(?:extern\s+(?:\"[^\"]+\"|\s+)?\s*)?fn\s+{re.escape(name)}\s*\("
    )
    for match in pattern.finditer(masked):
        start_line = _line_no(starts, match.start())
        brace = masked.find("{", match.end())
        if brace < 0:
            spans.append((start_line, start_line))
            continue
        depth = 0
        end = brace
        for index in range(brace, len(masked)):
            if masked[index] == "{":
                depth += 1
            elif masked[index] == "}":
                depth -= 1
                if depth == 0:
                    end = index
                    break
        spans.append((start_line, _line_no(starts, end)))
    return spans


def _line_in_spans(line: int, spans: list[tuple[int, int]]) -> bool:
    """判断行号是否落在函数定义范围内。"""
    return any(start <= line <= end for start, end in spans)


def _rust_reference_is_callback_like(text: str, masked: str, name: str, start: int, end: int) -> bool:
    """判断某个 Rust 引用是否出现在 callback/vtable/function pointer 语境。"""
    statement_start = max(masked.rfind(";", 0, start) + 1, masked.rfind("{", 0, start) + 1)
    semi = masked.find(";", end)
    brace = masked.find("}", end)
    statement_end_candidates = [value for value in (semi, brace) if value >= 0]
    statement_end = min(statement_end_candidates) if statement_end_candidates else min(len(masked), end + 240)
    raw_statement = text[statement_start:statement_end]
    masked_statement = masked[statement_start:statement_end]
    name_ref = re.escape(name)
    qualified_name = rf"(?:crate::[A-Za-z0-9_:]+::)?{name_ref}"
    if re.search(rf"\b{qualified_name}\b\s+as\s+(?:unsafe\s+)?extern\s+\"C\"\s+fn", raw_statement):
        return True
    if re.search(rf"\bSome\s*\(\s*{qualified_name}\b", masked_statement) and re.search(r"\b(?:Callback|callback|vtable|VTable|fn_ptr)\b|\bextern\b[^;]*\bfn\b", masked_statement):
        return True
    if re.search(rf"\btransmute\s*\([^;]*\b{qualified_name}\b", masked_statement):
        return True
    if re.search(rf"\b(?:Callback|callback|vtable|VTable|fn_ptr)\b[^;]*\b{qualified_name}\b|\b{qualified_name}\b[^;]*\b(?:Callback|callback|vtable|VTable|fn_ptr)\b", masked_statement):
        return True
    return False


def _scan_rust_references(functions: list[RustFunction], rust_texts: dict[str, str]) -> dict[str, dict[str, Any]]:
    """扫描 Rust 内部引用、callback/vtable 赋值和占位痕迹。"""
    names = sorted({fn.name for fn in functions})
    evidence: dict[str, dict[str, Any]] = {
        name: {
            "rust_reference_count": 0,
            "rust_reference_samples": [],
            "callback_or_vtable_refs": [],
            "placeholder_hits_nearby": [],
        }
        for name in names
    }
    definition_spans: dict[tuple[str, str], list[tuple[int, int]]] = {}
    for rel, text in rust_texts.items():
        masked = _mask_comments_and_strings(text)
        for name in names:
            definition_spans[(rel, name)] = _definition_line_spans(masked, name)

    for rel, text in rust_texts.items():
        masked = _mask_comments_and_strings(text)
        starts = _line_starts(text)
        lines = text.splitlines()
        for name in names:
            ref_re = re.compile(rf"\b(?:crate::[A-Za-z0-9_:]+::)?{re.escape(name)}\b")
            for match in ref_re.finditer(masked):
                if match.start() > 0 and masked[match.start() - 1] == ".":
                    continue
                line = _line_no(starts, match.start())
                if _line_in_spans(line, definition_spans.get((rel, name), [])):
                    continue
                detail = lines[line - 1].strip() if 0 < line <= len(lines) else ""
                evidence[name]["rust_reference_count"] += 1
                if len(evidence[name]["rust_reference_samples"]) < 5:
                    evidence[name]["rust_reference_samples"].append({"file": rel, "line": line, "detail": detail[:180]})
                if _rust_reference_is_callback_like(text, masked, name, match.start(), match.end()):
                    evidence[name]["callback_or_vtable_refs"].append({"file": rel, "line": line, "detail": detail[:180]})
        for hit in PLACEHOLDER_RE.finditer(text):
            line = text[: hit.start()].count("\n") + 1
            detail = lines[line - 1].strip() if 0 < line <= len(lines) else ""
            for name in names:
                if re.search(rf"\b{re.escape(name)}\b", detail):
                    evidence[name]["placeholder_hits_nearby"].append({"file": rel, "line": line, "detail": detail[:180]})
    return evidence


def _scan_c_facts(source_roots: list[Path]) -> dict[str, dict[str, Any]]:
    """扫描 C/C++ 头文件声明、static 定义、测试调用和动态符号证据。"""
    facts: dict[str, dict[str, Any]] = {}

    def entry(name: str) -> dict[str, Any]:
        return facts.setdefault(
            name,
            {
                "header_declarations": [],
                "static_definitions": [],
                "nonstatic_definitions": [],
                "c_test_calls": [],
                "dlsym_or_symbol_refs": [],
                "c_callback_or_vtable_refs": [],
            },
        )

    for root in source_roots:
        for path in _iter_c_files(root):
            text = path.read_text(encoding="utf-8", errors="replace")
            masked = _strip_c_comments_and_strings(text)
            starts = _line_starts(text)
            rel = str(path)
            is_header = path.suffix.lower() in C_HEADER_SUFFIXES
            is_test = _is_test_c_file(path, root)
            lines = text.splitlines()
            for match in C_FN_NAME_RE.finditer(masked):
                name = match.group(1)
                if name in C_CONTROL_WORDS:
                    continue
                line = _line_no(starts, match.start())
                line_text = lines[line - 1].strip() if 0 < line <= len(lines) else ""
                before = masked[max(0, match.start() - 160) : match.start()]
                after = masked[match.end() : min(len(masked), match.end() + 220)]
                if is_header and ";" in after.split("\n", 1)[0] and "typedef" not in before[-80:]:
                    entry(name)["header_declarations"].append({"file": rel, "line": line, "detail": line_text[:180]})
                if "{" in after.split("\n", 2)[0] or re.match(r"^[^{;]*\)\s*\{", after):
                    target = "static_definitions" if re.search(r"\bstatic\b", before[-120:]) else "nonstatic_definitions"
                    entry(name)[target].append({"file": rel, "line": line, "detail": line_text[:180]})
                if is_test:
                    entry(name)["c_test_calls"].append({"file": rel, "line": line, "detail": line_text[:180]})
            for line_no, detail in enumerate(lines, start=1):
                if not re.search(r"\b(?:dlsym|HdfLoad|GetObject|GetSymbol)\b", detail):
                    continue
                for string_name in re.findall(r'"([A-Za-z_][A-Za-z0-9_]*)"', detail):
                    entry(string_name)["dlsym_or_symbol_refs"].append({"file": rel, "line": line_no, "detail": detail.strip()[:180]})
            for assign in re.finditer(r"(?:\.|->)\s*[A-Za-z_][A-Za-z0-9_]*\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\b", masked):
                name = assign.group(1)
                line = _line_no(starts, assign.start())
                detail = lines[line - 1].strip() if 0 < line <= len(lines) else ""
                entry(name)["c_callback_or_vtable_refs"].append({"file": rel, "line": line, "detail": detail[:180]})
    return facts


def _sample(values: list[dict[str, Any]], limit: int = 5) -> list[dict[str, Any]]:
    """限制证据样本数量。"""
    return values[:limit]


def _classify_function(fn: RustFunction, c_fact: dict[str, Any], rust_fact: dict[str, Any], *, suite: str = "ohos") -> tuple[str, str, list[str], str]:
    """根据证据分类 ABI 保留需求。"""
    suite = _normalize_suite(suite)
    reasons: list[str] = []
    if fn.no_mangle:
        reasons.append("Rust function has no_mangle/export_name attribute")
    if c_fact.get("header_declarations") and suite != "oss":
        reasons.append("function is declared in C/C++ header")
    if c_fact.get("c_test_calls") and suite != "oss":
        reasons.append("function is referenced by C/C++ test source")
    if c_fact.get("dlsym_or_symbol_refs"):
        reasons.append("function name appears in dynamic symbol lookup/string evidence")
    if reasons:
        return "keep_c_abi", "high", reasons, "Preserve the public extern C boundary; rewrite private internals behind a thin Rust-native wrapper when semantics stay equivalent."

    callback_reasons: list[str] = []
    if rust_fact.get("callback_or_vtable_refs"):
        callback_reasons.append("Rust code uses the function as callback/vtable/function pointer")
    if c_fact.get("c_callback_or_vtable_refs"):
        callback_reasons.append("C/C++ source assigns the function to a callback/vtable-like field")
    if callback_reasons:
        return "extern_thunk_to_safe_core", "high", callback_reasons, "Keep the callback/function-pointer ABI thunk; move implementation state and logic into Rust-native helpers where possible."

    if c_fact.get("static_definitions"):
        return "rust_internal_candidate", "high", ["original C/C++ definition is static"], "Prefer private Rust-native helper/state after checking Rust call sites; no need to preserve the C internal shape."

    if suite == "oss" and c_fact.get("nonstatic_definitions") and not fn.is_extern_c:
        return "not_extern_c", "high", ["standalone OSS project already exposes this as ordinary Rust, not a translated-project C ABI"], "No C ABI boundary remains; keep the Rust-native API behavior and continue safe Rust cleanup if needed."

    if c_fact.get("nonstatic_definitions") and not fn.is_extern_c:
        return "review_required", "medium", ["original C/C++ definition has external linkage but no header/test export evidence"], "First decide whether this is a public ABI boundary; if not, treat it as internal Rust-native rewrite candidate."

    if fn.is_extern_c and not fn.no_mangle and rust_fact.get("rust_reference_count", 0) > 0 and not c_fact.get("nonstatic_definitions"):
        return "rust_internal_candidate", "medium", ["no public ABI evidence found and function is only referenced inside Rust crate"], "Candidate to remove extern C/pub and rewrite as normal Rust helper or merge into a Rust-native safe core."

    if suite == "oss" and fn.is_extern_c and not fn.no_mangle and (c_fact.get("c_test_calls") or c_fact.get("header_declarations")):
        return "rust_internal_candidate", "high", ["standalone OSS project; C headers/tests are source semantics evidence, not required translated-project ABI"], "Prefer a safe Rust-native API/core while preserving standalone project behavior."

    if fn.is_extern_c and not fn.no_mangle:
        return "review_required", "medium", ["extern C function has no direct export evidence, but source visibility is not decisive"], "Review source header/call graph; preserve only if it is a real ABI boundary, otherwise rewrite internal implementation freely."

    return "not_extern_c", "high", ["ordinary Rust function"], "No public ABI boundary detected; Rust-native cleanup may still apply."


def build_abi_refactor_inventory(crate_dir: Path, source_roots: list[Path] | None = None, *, suite: str = "ohos") -> dict[str, Any]:
    """构建 ABI/refactor 清单。"""
    suite = _normalize_suite(suite)
    crate_dir = crate_dir.expanduser().resolve()
    roots = [path.expanduser().resolve() for path in (source_roots or []) if path and path.expanduser().is_dir()]
    functions, rust_texts = _parse_rust_functions(crate_dir)
    rust_facts = _scan_rust_references(functions, rust_texts)
    c_facts = _scan_c_facts(roots)
    entries: list[dict[str, Any]] = []
    support_entries: list[dict[str, Any]] = []
    by_decision: dict[str, int] = {}
    placeholder_hits: list[dict[str, Any]] = []
    support_placeholder_hit_count = 0

    for rel, text in rust_texts.items():
        lines = text.splitlines()
        for hit in PLACEHOLDER_RE.finditer(text):
            line = text[: hit.start()].count("\n") + 1
            if _is_support_rust_file(rel):
                support_placeholder_hit_count += 1
            else:
                placeholder_hits.append({"file": rel, "line": line, "detail": lines[line - 1].strip()[:180] if 0 < line <= len(lines) else ""})

    for fn in functions:
        if _is_support_rust_file(fn.file):
            support_entries.append(
                {
                    "name": fn.name,
                    "rust_file": fn.file,
                    "rust_line": fn.line,
                    "is_pub": fn.is_pub,
                    "is_extern_c": fn.is_extern_c,
                    "is_unsafe_fn": fn.is_unsafe_fn,
                    "no_mangle": fn.no_mangle,
                }
            )
            continue
        c_fact = c_facts.get(fn.name, {})
        rust_fact = rust_facts.get(fn.name, {})
        decision, confidence, reasons, recommended = _classify_function(fn, c_fact, rust_fact, suite=suite)
        by_decision[decision] = by_decision.get(decision, 0) + 1
        entries.append(
            {
                "name": fn.name,
                "rust_file": fn.file,
                "rust_line": fn.line,
                "is_pub": fn.is_pub,
                "is_extern_c": fn.is_extern_c,
                "is_unsafe_fn": fn.is_unsafe_fn,
                "no_mangle": fn.no_mangle,
                "decision": decision,
                "confidence": confidence,
                "reasons": reasons,
                "recommended_action": recommended,
                "evidence": {
                    "header_declarations": _sample(c_fact.get("header_declarations", [])),
                    "static_definitions": _sample(c_fact.get("static_definitions", [])),
                    "nonstatic_definitions": _sample(c_fact.get("nonstatic_definitions", [])),
                    "c_test_calls": _sample(c_fact.get("c_test_calls", [])),
                    "dlsym_or_symbol_refs": _sample(c_fact.get("dlsym_or_symbol_refs", [])),
                    "c_callback_or_vtable_refs": _sample(c_fact.get("c_callback_or_vtable_refs", [])),
                    "rust_reference_count": rust_fact.get("rust_reference_count", 0),
                    "rust_reference_samples": _sample(rust_fact.get("rust_reference_samples", [])),
                    "callback_or_vtable_refs": _sample(rust_fact.get("callback_or_vtable_refs", [])),
                    "placeholder_hits_nearby": _sample(rust_fact.get("placeholder_hits_nearby", [])),
                },
            }
        )
    summary = {
        "function_count": len(entries),
        "extern_c_function_count": sum(1 for item in entries if item["is_extern_c"]),
        "public_extern_c_function_count": sum(1 for item in entries if item["is_extern_c"] and item["is_pub"]),
        "no_mangle_count": sum(1 for item in entries if item["no_mangle"]),
        "by_decision": by_decision,
        "placeholder_hit_count": len(placeholder_hits),
        "support_function_count": len(support_entries),
        "support_placeholder_hit_count": support_placeholder_hit_count,
        "excluded_support_files": sorted(SUPPORT_RUST_FILES),
        "source_roots": [str(path) for path in roots],
        "suite": suite,
    }
    return {
        "schema_version": "c2r_abi_refactor_inventory_v1",
        "gate": ABI_REFACTOR_INVENTORY,
        "suite": suite,
        "crate_dir": str(crate_dir),
        "rust_native_refactor_policy": _rust_native_refactor_policy(suite),
        "summary": summary,
        "functions": sorted(entries, key=lambda item: (item["decision"], item["rust_file"], item["rust_line"], item["name"])),
        "support_functions": sorted(support_entries, key=lambda item: (item["rust_file"], item["rust_line"], item["name"])),
        "placeholder_hits": placeholder_hits[:200],
    }


def render_markdown(payload: dict[str, Any]) -> str:
    """渲染给 agent 阅读的精简 Markdown。"""
    summary = payload.get("summary", {})
    suite = _normalize_suite(str(payload.get("suite") or summary.get("suite") or "ohos"))
    if suite == "oss":
        policy_lines = [
            "This is a standalone OSS/C project translated as a Rust-native project. Preserve the safe Rust observable behavior of the project, not the C external entry-point shape used by source tests.",
            "",
            "Target raw unsafe = 0 or near 0 where semantics allow. C test references are harness evidence for later paper experiments; by themselves they do not require the translated crate to keep extern C raw-pointer APIs.",
        ]
    else:
        policy_lines = [
            "This is an OHOS integration project. Public ABI, exported symbols, extern calling conventions, public `repr(C)` layouts, return codes, out-params, callback timing, locks, resources, and platform side effects must remain equivalent.",
            "",
            "Reduce unnecessary unsafe only within those integration constraints. Private/internal data structures, state representation, static/private helpers, private structs, intrusive lists, hand-written global state, internal helper APIs, and control flow may be rewritten into Rust-native forms when platform-observable semantics remain equivalent. Do not preserve C internal structure one-to-one when no integration contract requires it.",
        ]
    lines = [
        "# ABI Refactor Inventory",
        "",
        f"- crate_dir: `{payload.get('crate_dir', '')}`",
        f"- suite: `{suite}`",
        f"- function_count: {summary.get('function_count', 0)}",
        f"- extern_c_function_count: {summary.get('extern_c_function_count', 0)}",
        f"- public_extern_c_function_count: {summary.get('public_extern_c_function_count', 0)}",
        f"- no_mangle_count: {summary.get('no_mangle_count', 0)}",
        f"- placeholder_hit_count: {summary.get('placeholder_hit_count', 0)}",
        f"- support_function_count: {summary.get('support_function_count', 0)}",
        f"- support_placeholder_hit_count: {summary.get('support_placeholder_hit_count', 0)}",
        f"- by_decision: `{json.dumps(summary.get('by_decision', {}), ensure_ascii=False, sort_keys=True)}`",
        "",
        "## Rust-native Refactor Policy",
        "",
        *policy_lines,
        "",
        "## Decisions",
        "",
        "| decision | confidence | function | file:line | reason | action |",
        "|---|---|---|---|---|---|",
    ]
    priority = {"keep_c_abi": 0, "extern_thunk_to_safe_core": 1, "rust_internal_candidate": 2, "review_required": 3, "not_extern_c": 4}
    functions = sorted(payload.get("functions", []), key=lambda item: (priority.get(item.get("decision"), 9), item.get("rust_file", ""), item.get("rust_line", 0)))
    for item in functions:
        if item.get("decision") == "not_extern_c":
            continue
        reason = "; ".join(item.get("reasons", [])) or "-"
        lines.append(
            "| "
            + " | ".join(
                [
                    _md(item.get("decision", "")),
                    _md(item.get("confidence", "")),
                    _md(item.get("name", "")),
                    _md(f"{item.get('rust_file', '')}:{item.get('rust_line', '')}"),
                    _md(reason),
                    _md(item.get("recommended_action", "")),
                ]
            )
            + " |"
        )
    placeholders = payload.get("placeholder_hits", [])
    if placeholders:
        lines.extend(["", "## Placeholder / Fallback Signals", "", "| file:line | detail |", "|---|---|"])
        for item in placeholders[:80]:
            location = f"{item.get('file', '')}:{item.get('line', '')}"
            lines.append(f"| {_md(location)} | {_md(item.get('detail', ''))} |")
    return "\n".join(lines) + "\n"


def _md(value: Any) -> str:
    """转义 Markdown 表格单元格。"""
    return str(value).replace("|", "\\|").replace("\n", " ")


def run_abi_refactor_inventory(crate_dir: Path, output_json: Path, output_md: Path, source_roots: list[Path] | None = None, *, suite: str = "ohos") -> dict[str, Any]:
    """运行 inventory 并写出 JSON/Markdown。"""
    payload = build_abi_refactor_inventory(crate_dir, source_roots=source_roots, suite=suite)
    _write_json(output_json, payload)
    output_md.parent.mkdir(parents=True, exist_ok=True)
    output_md.write_text(render_markdown(payload), encoding="utf-8")
    return {
        "status": "available",
        "passed": True,
        "returncode": 0,
        "json_path": str(output_json),
        "markdown_path": str(output_md),
        "summary": payload.get("summary", {}),
    }


def _parse_args() -> argparse.Namespace:
    """解析命令行参数。"""
    parser = argparse.ArgumentParser(description="Generate ABI/refactor inventory for a translated Rust crate.")
    parser.add_argument("--crate-dir", required=True)
    parser.add_argument("--source-root", action="append", default=[])
    parser.add_argument("--suite", default="ohos", choices=["ohos", "oss"])
    parser.add_argument("--output-json", required=True)
    parser.add_argument("--output-md", required=True)
    return parser.parse_args()


def main() -> int:
    """命令行入口。"""
    args = _parse_args()
    payload = run_abi_refactor_inventory(
        Path(args.crate_dir),
        Path(args.output_json),
        Path(args.output_md),
        source_roots=[Path(item) for item in args.source_root],
        suite=args.suite,
    )
    print(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
