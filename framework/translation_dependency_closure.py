"""构建单元翻译所需的依赖闭包事实和审计 manifest。"""

from __future__ import annotations

import json
import os
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class DependencyClosureBundle:
    """依赖闭包输出：prompt 只放高置信事实，完整信息落 manifest。"""

    prompt_block: str
    manifest_path: Path
    manifest: dict[str, Any]


def _safe_name(value: str) -> str:
    """把函数名规整为可用于文件名的短字符串。"""
    text = str(value or "")
    safe = "".join(c if c.isalnum() or c in ("_", "-") else "_" for c in text)
    return safe or "unknown"


def _func_key(func_info: Any) -> str:
    """返回阶段间一致的函数 key。"""
    return f"{getattr(func_info, 'file_name', '')}_{getattr(func_info, 'index', '')}"


def _called_symbols(func_info: Any, state: Any) -> set[str]:
    """从 C 源码中提取函数式调用符号。"""
    code = str(getattr(func_info, "c_code", "") or "")
    try:
        symbols = set(state._extract_called_identifiers_from_c(code))
    except Exception:
        symbols = set(re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(", code))
    symbols.discard(str(getattr(func_info, "name", "") or ""))
    return {item for item in symbols if item}


def _append_unique(items: list[dict[str, Any]], item: dict[str, Any]) -> None:
    """按 JSON 内容去重追加。"""
    key = json.dumps(item, ensure_ascii=False, sort_keys=True)
    for existing in items:
        if json.dumps(existing, ensure_ascii=False, sort_keys=True) == key:
            return
    items.append(item)


def _path_text(path: Any) -> str:
    """返回存在路径的绝对字符串。"""
    if path is None:
        return ""
    try:
        return str(Path(path).expanduser().resolve())
    except Exception:
        return str(path)


def _get_preprocessed_text(state: Any, preprocessed_file: Path | None) -> str:
    """读取并缓存预处理 TU 文本。"""
    if preprocessed_file is None:
        return ""
    cache = getattr(state, "_preprocessed_text_cache", None)
    if isinstance(cache, dict) and preprocessed_file in cache:
        return str(cache.get(preprocessed_file) or "")
    try:
        text = preprocessed_file.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        text = ""
    if isinstance(cache, dict):
        cache[preprocessed_file] = text
    return text


def _internal_dependency_facts(func_info: Any, state: Any, facts: list[dict[str, Any]], gaps: list[dict[str, Any]]) -> set[str]:
    """收集直接和递归内部 callee 的 Rust 路径与签名。"""
    functions = getattr(state, "_functions_by_func_file", None)
    if not isinstance(functions, dict) or not functions:
        return set()
    try:
        max_depth = max(1, int(os.environ.get("C2R_DEPENDENCY_CLOSURE_MAX_DEPTH", "3")))
    except Exception:
        max_depth = 3
    try:
        max_items = max(1, int(os.environ.get("C2R_DEPENDENCY_CLOSURE_MAX_INTERNAL", "80")))
    except Exception:
        max_items = 80

    covered: set[str] = set()
    queue: list[tuple[str, int]] = [(str(dep), 1) for dep in sorted(getattr(func_info, "dependencies", set()) or set())]
    seen: set[str] = set()
    while queue and len(covered) < max_items:
        dep_key, depth = queue.pop(0)
        if dep_key in seen or depth > max_depth:
            continue
        seen.add(dep_key)
        callee = functions.get(dep_key)
        if callee is None:
            _append_unique(
                gaps,
                {
                    "kind": "internal_dependency_missing",
                    "symbol": dep_key,
                    "evidence": "call_graph_dependency_key",
                },
            )
            continue
        name = str(getattr(callee, "name", "") or "")
        module = str(getattr(callee, "file_name", "") or "")
        if not name or not module:
            continue
        fact: dict[str, Any] = {
            "kind": "internal_function",
            "symbol": name,
            "dependency_key": dep_key,
            "depth": depth,
            "rust_path": f"crate::{module}::{name}",
            "source": "call_graph",
            "confidence": "high",
        }
        signature = str(getattr(callee, "rust_signature", "") or "").strip()
        if signature:
            fact["rust_signature"] = signature
        _append_unique(facts, fact)
        covered.add(name)
        if depth < max_depth:
            for child in sorted(getattr(callee, "dependencies", set()) or set()):
                queue.append((str(child), depth + 1))
    return covered


def _rust_signature_facts(called: set[str], state: Any, facts: list[dict[str, Any]], covered: set[str]) -> None:
    """收集 signature_matches 中的 Rust callee 签名。"""
    try:
        state._ensure_rust_signature_index()
    except Exception:
        return
    index = getattr(state, "_rust_signature_index", None)
    if not isinstance(index, dict) or not index:
        return
    for symbol in sorted(called):
        if symbol in covered:
            continue
        candidates = index.get(symbol) or []
        if not candidates:
            continue
        try:
            signature = state._choose_best_rust_signature(candidates)
        except Exception:
            signature = str(candidates[0] or "")
        if not signature:
            continue
        path_fact = _resolve_rust_signature_path_fact(symbol, signature, state)
        fact = {
            "kind": "rust_signature",
            "symbol": symbol,
            "rust_signature": signature,
            "source": "signature_matches",
            "confidence": "high",
        }
        if path_fact:
            fact.update(path_fact)
        _append_unique(
            facts,
            fact,
        )
        covered.add(symbol)


def _quarantine_ambiguous_internal_callees(
    facts: list[dict[str, Any]],
    gaps: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    """把同一 C 符号对应多个 Rust 路径的 callee 从高置信事实中移出。"""
    by_symbol: dict[str, list[dict[str, Any]]] = {}
    for item in facts:
        if item.get("kind") not in {"internal_function", "rust_signature"}:
            continue
        symbol = str(item.get("symbol") or "").strip()
        rust_path = str(item.get("rust_path") or "").strip()
        if not symbol or not rust_path:
            continue
        by_symbol.setdefault(symbol, []).append(item)

    ambiguous_symbols = {
        symbol
        for symbol, items in by_symbol.items()
        if len({str(item.get("rust_path") or "").strip() for item in items if item.get("rust_path")}) > 1
    }
    if not ambiguous_symbols:
        return []

    ambiguous: list[dict[str, Any]] = []
    kept: list[dict[str, Any]] = []
    for item in facts:
        symbol = str(item.get("symbol") or "").strip()
        if symbol in ambiguous_symbols and item.get("kind") in {"internal_function", "rust_signature"} and item.get("rust_path"):
            ambiguous_item = dict(item)
            ambiguous_item["confidence"] = "ambiguous"
            _append_unique(ambiguous, ambiguous_item)
            continue
        kept.append(item)
    facts[:] = kept

    for symbol in sorted(ambiguous_symbols):
        _append_unique(
            gaps,
            {
                "kind": "ambiguous_internal_callee",
                "symbol": symbol,
                "candidate_paths": sorted(
                    {
                        str(item.get("rust_path") or "").strip()
                        for item in by_symbol.get(symbol, [])
                        if item.get("rust_path")
                    }
                ),
                "evidence": "multiple_distinct_rust_paths_for_same_called_symbol",
            },
        )

    return ambiguous


def _resolve_rust_signature_path_fact(symbol: str, signature: str, state: Any) -> dict[str, str]:
    """用确定性签名映射为 signature fact 补唯一 Rust 模块路径。"""
    candidates: list[dict[str, str]] = []

    fact_index = getattr(state, "_rust_signature_fact_index", None)
    if isinstance(fact_index, dict):
        for item in fact_index.get(symbol) or []:
            if not isinstance(item, dict):
                continue
            if str(item.get("rust_signature") or "").strip() != signature:
                continue
            rust_path = str(item.get("rust_path") or "").strip()
            if not rust_path:
                continue
            candidates.append(
                {
                    "dependency_key": str(item.get("func_key") or ""),
                    "rust_path": rust_path,
                }
            )

    functions = getattr(state, "_functions_by_func_file", None)
    functions = functions if isinstance(functions, dict) else {}
    for func_key, mapped_sig in (getattr(state, "func_file_to_rust_sig", None) or {}).items():
        if str(mapped_sig or "").strip() != signature:
            continue
        callee = functions.get(func_key)
        callee_name = str(getattr(callee, "name", "") or symbol)
        if callee_name != symbol:
            continue
        module = str(getattr(callee, "file_name", "") or "")
        if not module:
            module = str(func_key).rsplit("_", 1)[0]
        if not module:
            continue
        candidates.append(
            {
                "dependency_key": str(func_key),
                "rust_path": f"crate::{module}::{symbol}",
            }
        )

    unique: dict[str, dict[str, str]] = {}
    for item in candidates:
        rust_path = item.get("rust_path") or ""
        if rust_path:
            unique[rust_path] = item
    if len(unique) != 1:
        return {}
    return next(iter(unique.values()))


def _static_inline_facts(
    called: set[str],
    state: Any,
    preprocessed_file: Path | None,
    facts: list[dict[str, Any]],
    covered: set[str],
) -> None:
    """收集预处理 TU 中被调用的 static inline helper 定义。"""
    if preprocessed_file is None:
        return
    text = _get_preprocessed_text(state, preprocessed_file)
    if not text:
        return
    cache = getattr(state, "_preprocessed_fn_index_cache", None)
    index = cache.get(preprocessed_file) if isinstance(cache, dict) else None
    if index is None:
        try:
            index = state._build_preprocessed_fn_index(preprocessed_file, text)
        except Exception:
            index = {}
        if isinstance(cache, dict):
            cache[preprocessed_file] = index
    if not isinstance(index, dict) or not index:
        return
    try:
        max_defs = max(0, int(getattr(state, "_preprocessed_inline_helper_hints_max", 8) or 8))
    except Exception:
        max_defs = 8
    try:
        max_chars = max(0, int(getattr(state, "_preprocessed_inline_helper_hints_max_chars", 12000) or 12000))
    except Exception:
        max_chars = 12000
    used_chars = 0
    picked = 0
    for symbol in sorted(called):
        if picked >= max_defs:
            break
        spans: list[tuple[int, int]] = []
        for key, values in index.items():
            if not key or key[0] != symbol:
                continue
            for span in values or []:
                if isinstance(span, tuple) and len(span) == 2:
                    spans.append((int(span[0]), int(span[1])))
        best = ""
        for start, end in spans:
            if start < 0 or end <= start or end > len(text):
                continue
            snippet = text[start:end].strip()
            if not snippet:
                continue
            try:
                is_inline = state._looks_like_static_inline_c_function(snippet)
            except Exception:
                is_inline = "static" in snippet.split("{", 1)[0] and "inline" in snippet.split("{", 1)[0]
            if not is_inline:
                continue
            if not best or len(snippet) < len(best):
                best = snippet
        if not best or used_chars + len(best) > max_chars:
            continue
        _append_unique(
            facts,
            {
                "kind": "static_inline",
                "symbol": symbol,
                "c_definition": best,
                "source": _path_text(preprocessed_file),
                "confidence": "high",
            },
        )
        covered.add(symbol)
        used_chars += len(best)
        picked += 1


def _c_prototype_facts(
    called: set[str],
    func_info: Any,
    state: Any,
    dependency_file: Path,
    preprocessed_file: Path | None,
    facts: list[dict[str, Any]],
    covered: set[str],
) -> None:
    """收集依赖块和预处理 TU 中的 C callee 原型。"""
    prototypes_by_name: dict[str, list[str]] = {}
    dep_text = ""
    if dependency_file.is_file():
        try:
            dep_text = dependency_file.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            dep_text = ""
    if dep_text:
        try:
            dep_prototypes = state._extract_c_function_prototypes(dep_text)
        except Exception:
            dep_prototypes = {}
        for name, protos in (dep_prototypes or {}).items():
            prototypes_by_name.setdefault(name, []).extend([str(item) for item in protos if item])
    try:
        preprocessed_decls = state._get_preprocessed_decl_prototypes(func_info)
    except Exception:
        preprocessed_decls = {}
    for name in called:
        for proto in (preprocessed_decls.get(name) or []) if isinstance(preprocessed_decls, dict) else []:
            prototypes_by_name.setdefault(name, []).append(str(proto))

    try:
        local_types = state._extract_local_type_names_from_c(str(getattr(func_info, "c_code", "") or ""))
    except Exception:
        local_types = set()
    for symbol in sorted(called):
        protos = prototypes_by_name.get(symbol) or []
        if not protos:
            continue
        try:
            prototype = state._choose_best_c_prototype(protos, local_types)
        except Exception:
            prototype = max(protos, key=len)
        if not prototype:
            continue
        source = _path_text(preprocessed_file) if symbol in (preprocessed_decls or {}) else _path_text(dependency_file)
        _append_unique(
            facts,
            {
                "kind": "c_prototype",
                "symbol": symbol,
                "c_prototype": prototype,
                "source": source,
                "confidence": "high",
            },
        )
        covered.add(symbol)


def _looks_like_c_constant_identifier(name: str) -> bool:
    """判断 C 标识符是否像宏/枚举常量。"""
    return bool(re.match(r"^[A-Z_][A-Z0-9_]{2,}$", name or ""))


def _typed_constant_facts(func_info: Any, state: Any, facts: list[dict[str, Any]], gaps: list[dict[str, Any]]) -> None:
    """收集 types.rs 中与当前 C 代码相关的常量类型。"""
    const_names = getattr(state, "_types_const_names", None)
    const_types = getattr(state, "_types_const_type_map", None)
    if not const_names or not isinstance(const_types, dict):
        return
    const_name_set = {str(name) for name in const_names if str(name)}
    code = str(getattr(func_info, "c_code", "") or "")
    identifiers = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", code))

    def add_constant_fact(c_symbol: str, rust_symbol: str) -> None:
        ty = str(const_types.get(rust_symbol) or "")
        if not ty:
            return
        _append_unique(
            facts,
            {
                "kind": "typed_constant",
                "symbol": rust_symbol,
                "c_symbol": c_symbol,
                "rust_expr": f"crate::types::{rust_symbol}",
                "rust_type": ty,
                "source": "types.rs",
                "confidence": "high",
            },
        )

    for symbol in sorted(identifiers & const_name_set):
        add_constant_fact(symbol, symbol)

    for c_symbol in sorted(identifiers - const_name_set):
        if not _looks_like_c_constant_identifier(c_symbol):
            continue
        candidates = sorted(name for name in const_name_set if name.endswith(f"_{c_symbol}"))
        if len(candidates) == 1:
            add_constant_fact(c_symbol, candidates[0])
        elif len(candidates) > 1:
            _append_unique(
                gaps,
                {
                    "kind": "ambiguous_typed_constant",
                    "symbol": c_symbol,
                    "candidate_symbols": candidates[:20],
                    "evidence": "multiple_types_rs_constants_share_suffix",
                },
            )


def _low_confidence_hints(func_info: Any, state: Any) -> list[dict[str, Any]]:
    """把推断类信息放入 manifest，不直接注入主 prompt。"""
    hints: list[dict[str, Any]] = []
    for kind, method_name in (
        ("c_field_access", "_build_c_field_access_hints"),
        ("c_pointer_contract", "_build_c_pointer_contract_hints"),
    ):
        method = getattr(state, method_name, None)
        if not callable(method):
            continue
        try:
            text = str(method(func_info) or "").strip()
        except Exception:
            text = ""
        if text:
            hints.append({"kind": kind, "content": text, "confidence": "hint"})
    return hints


def _prompt_block(
    manifest_path: Path,
    facts: list[dict[str, Any]],
    gaps: list[dict[str, Any]],
    ambiguous_callees: list[dict[str, Any]] | None = None,
) -> str:
    """渲染只包含高置信事实的 prompt 块。"""
    ambiguous_callees = ambiguous_callees or []
    if not facts and not gaps and not ambiguous_callees:
        return ""
    max_facts = int(os.environ.get("C2R_DEPENDENCY_CLOSURE_PROMPT_MAX_FACTS", "48") or "48")
    lines: list[str] = [
        "## Dependency Closure (high-confidence)",
        f"Manifest: {manifest_path.resolve()}",
        "Use callable Rust APIs only when a CALL AS path is listed. C/C++ facts are explanatory and are not Rust call targets.",
    ]

    callable_items = [
        item
        for item in facts
        if item.get("kind") in {"internal_function", "rust_signature"} and item.get("rust_path")
    ][:max_facts]
    if callable_items:
        lines.append("### Callable Rust APIs")
        for item in callable_items:
            sig = f"; signature: `{item.get('rust_signature')}`" if item.get("rust_signature") else ""
            depth = f"; depth={item.get('depth')}" if item.get("depth") else ""
            lines.append(f"- {item.get('symbol')}: CALL AS `{item.get('rust_path')}(...)`{depth}{sig}")

    unresolved_rust_sigs = [
        item
        for item in facts
        if item.get("kind") == "rust_signature" and item.get("rust_signature") and not item.get("rust_path")
    ][:max_facts]
    if unresolved_rust_sigs:
        lines.append("### Rust signatures without resolved module path")
        lines.append("These signatures are not callable targets unless the current crate context separately provides an import/path.")
        lines.append("```rust")
        lines.extend(str(item.get("rust_signature") or "") for item in unresolved_rust_sigs)
        lines.append("```")

    c_protos = [item for item in facts if item.get("kind") == "c_prototype"][:max_facts]
    if c_protos:
        lines.append("### C/C++ explanatory prototypes (not Rust call targets)")
        lines.append("```c")
        lines.extend(str(item.get("c_prototype") or "") for item in c_protos if item.get("c_prototype"))
        lines.append("```")
        lines.append("Use these only for argument constness/mutability and ABI intent; do not emit calls to unavailable Rust helpers from this section.")

    inlines = [item for item in facts if item.get("kind") == "static_inline"][:max_facts]
    if inlines:
        lines.append("### Static inline C helpers")
        lines.append("```c")
        lines.extend(str(item.get("c_definition") or "") for item in inlines if item.get("c_definition"))
        lines.append("```")
        lines.append("Do not declare static inline helpers as extern symbols; inline their logic inside the target function body.")

    consts = [item for item in facts if item.get("kind") == "typed_constant"][:max_facts]
    if consts:
        lines.append("### Typed constants")
        for item in consts:
            c_symbol = str(item.get("c_symbol") or item.get("symbol") or "")
            rust_expr = str(item.get("rust_expr") or f"crate::types::{item.get('symbol')}")
            lines.append(f"- C `{c_symbol}` -> Rust `{rust_expr}`: `{item.get('rust_type')}`")

    if ambiguous_callees:
        lines.append("### Ambiguous internal callees")
        lines.append("These are NOT high-confidence call targets. Do not call a candidate unless the current receiver/source context proves it.")
        grouped: dict[str, list[str]] = {}
        for item in ambiguous_callees:
            symbol = str(item.get("symbol") or "").strip()
            rust_path = str(item.get("rust_path") or "").strip()
            if symbol and rust_path:
                grouped.setdefault(symbol, []).append(rust_path)
        for symbol in sorted(grouped)[:max_facts]:
            paths = ", ".join(f"`{path}`" for path in sorted(set(grouped[symbol])))
            lines.append(f"- {symbol}: {paths}")

    unresolved = [item for item in gaps if item.get("kind") == "unresolved_called_symbol"]
    if unresolved:
        names = ", ".join(str(item.get("symbol")) for item in unresolved[:40])
        lines.append(f"### Gaps: unresolved called symbols")
        lines.append(f"- {names}")
        lines.append("Do not guess missing ABI/signatures from these gaps; inspect manifest/source evidence first.")

    ambiguous_consts = [item for item in gaps if item.get("kind") == "ambiguous_typed_constant"]
    if ambiguous_consts:
        lines.append("### Gaps: ambiguous typed constants")
        for item in ambiguous_consts[:20]:
            candidates = ", ".join(str(x) for x in (item.get("candidate_symbols") or [])[:8])
            lines.append(f"- {item.get('symbol')}: {candidates}")
        lines.append("Do not use a short constant name unless the crate declares it; choose a listed Rust constant only with unique evidence.")

    return "\n".join(lines).strip()


def build_dependency_closure(func_info: Any, state: Any, output_dir: Path | None = None) -> DependencyClosureBundle:
    """构建依赖闭包 manifest，并返回可注入 prompt 的高置信块。"""
    called = _called_symbols(func_info, state)
    func_key = _func_key(func_info)
    dependency_file = Path(getattr(state, "dependencies_dir", Path("."))) / f"{func_key}.txt"
    try:
        preprocessed_file = state._locate_preprocessed_file(func_info)
    except Exception:
        preprocessed_file = None
    preprocessed_path = Path(preprocessed_file) if preprocessed_file else None
    manifest_dir = Path(output_dir) if output_dir is not None else Path(getattr(state, "context_cache_dir", Path("."))) / "dependency_closure"
    manifest_dir.mkdir(parents=True, exist_ok=True)
    manifest_path = manifest_dir / f"closure_{_safe_name(func_key)}.json"

    facts: list[dict[str, Any]] = []
    gaps: list[dict[str, Any]] = []
    covered = _internal_dependency_facts(func_info, state, facts, gaps)
    _static_inline_facts(called, state, preprocessed_path, facts, covered)
    _c_prototype_facts(called, func_info, state, dependency_file, preprocessed_path, facts, covered)
    _rust_signature_facts(called, state, facts, covered)
    _typed_constant_facts(func_info, state, facts, gaps)
    ambiguous_callees = _quarantine_ambiguous_internal_callees(facts, gaps)
    hints = _low_confidence_hints(func_info, state)

    for symbol in sorted(called - covered):
        _append_unique(
            gaps,
            {
                "kind": "unresolved_called_symbol",
                "symbol": symbol,
                "evidence": "call_expression_in_current_c_code",
            },
        )

    source_paths = {
        "dependency_file": _path_text(dependency_file) if dependency_file.exists() else "",
        "preprocessed_file": _path_text(preprocessed_path) if preprocessed_path and preprocessed_path.exists() else "",
        "signature_dir": _path_text(getattr(state, "signature_dir", None)),
    }
    manifest = {
        "schema_version": "c2r_dependency_closure_v1",
        "function": {
            "func_key": func_key,
            "c_name": str(getattr(func_info, "name", "") or ""),
            "rust_signature": str(getattr(func_info, "rust_signature", "") or ""),
        },
        "called_symbols": sorted(called),
        "facts": facts,
        "ambiguous_callees": ambiguous_callees,
        "hints": hints,
        "gaps": gaps,
        "source_paths": source_paths,
    }
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return DependencyClosureBundle(
        prompt_block=_prompt_block(manifest_path, facts, gaps, ambiguous_callees),
        manifest_path=manifest_path.resolve(),
        manifest=manifest,
    )
