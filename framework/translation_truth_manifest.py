"""生成函数级翻译真值 manifest，并渲染紧凑 prompt 证据块。"""

from __future__ import annotations

from dataclasses import dataclass
import json
import re
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class TruthManifestBundle:
    """函数级 truth manifest 输出。"""

    prompt_block: str
    manifest_path: Path
    manifest: dict[str, Any]


def _func_key(func_info: Any) -> str:
    """返回阶段间一致的函数 key。"""
    return f"{getattr(func_info, 'file_name', '')}_{getattr(func_info, 'index', '')}"


def _safe_name(value: str) -> str:
    """把函数 key 转成文件名安全字符串。"""
    text = str(value or "")
    return "".join(c if c.isalnum() or c in ("_", "-") else "_" for c in text) or "unknown"


def _append_unique(items: list[dict[str, Any]], item: dict[str, Any]) -> None:
    """按 JSON 内容去重追加。"""
    key = json.dumps(item, ensure_ascii=False, sort_keys=True)
    for existing in items:
        if json.dumps(existing, ensure_ascii=False, sort_keys=True) == key:
            return
    items.append(item)


def _preprocessed_truth_info(func_info: Any, state: Any) -> dict[str, Any]:
    """读取当前函数是否来自预处理 TU 的真值信息。"""
    try:
        info = state._preprocessed_truth_info(func_info)
    except Exception:
        info = None
    return dict(info) if isinstance(info, dict) else {}


def _summarize_facts(dependency_manifest: dict[str, Any] | None) -> dict[str, Any]:
    """按 kind 汇总 dependency closure facts/gaps。"""
    manifest = dependency_manifest if isinstance(dependency_manifest, dict) else {}
    by_kind: dict[str, int] = {}
    for item in manifest.get("facts") or []:
        if not isinstance(item, dict):
            continue
        kind = str(item.get("kind") or "unknown")
        by_kind[kind] = by_kind.get(kind, 0) + 1
    gaps_by_kind: dict[str, int] = {}
    for item in manifest.get("gaps") or []:
        if not isinstance(item, dict):
            continue
        kind = str(item.get("kind") or "unknown")
        gaps_by_kind[kind] = gaps_by_kind.get(kind, 0) + 1
    return {
        "called_symbols": list(manifest.get("called_symbols") or []),
        "fact_counts_by_kind": by_kind,
        "gap_counts_by_kind": gaps_by_kind,
        "ambiguous_callees": list(manifest.get("ambiguous_callees") or []),
    }


def _strip_comments(code: str) -> str:
    """移除 C/C++ 注释，降低正则提取噪声。"""
    code = re.sub(r"/\*.*?\*/", " ", code or "", flags=re.DOTALL)
    return re.sub(r"//.*", " ", code)


def _extract_cpp_receiver_facts(code: str) -> dict[str, Any]:
    """保守提取 C++ 对象 receiver、成员调用和字段访问事实。"""
    text = _strip_comments(code)
    objects: dict[str, dict[str, str]] = {}
    type_name = r"(?:[A-Za-z_]\w*::)*[A-Za-z_]\w*(?:\s*<[^;{}()]*>)?"

    # new 出来的指针对象：`Type *var = new Type(...)`
    new_pat = re.compile(
        rf"\b(?P<decl_type>{type_name})\s*(?P<ptr>[*&]?)\s*(?P<var>[A-Za-z_]\w*)\s*=\s*new\s+(?P<ctor>{type_name})\s*\(",
        re.MULTILINE,
    )
    for match in new_pat.finditer(text):
        var = match.group("var")
        objects[var] = {
            "name": var,
            "type": re.sub(r"\s+", " ", match.group("decl_type")).strip(),
            "storage": "heap_pointer" if match.group("ptr") == "*" else "heap_value",
            "constructor": re.sub(r"\s+", " ", match.group("ctor")).strip(),
            "evidence": match.group(0).strip()[:200],
        }

    # 栈对象构造：`Type var(args);`。限制 Type 首字母大写或带 namespace，避免误报普通函数调用。
    stack_pat = re.compile(
        rf"\b(?P<decl_type>{type_name})\s+(?P<var>[A-Za-z_]\w*)\s*\([^;{{}}]*\)\s*;",
        re.MULTILINE,
    )
    for match in stack_pat.finditer(text):
        decl_type = re.sub(r"\s+", " ", match.group("decl_type")).strip()
        var = match.group("var")
        if var in {"if", "for", "while", "switch", "return"}:
            continue
        if "::" not in decl_type and not decl_type[:1].isupper():
            continue
        objects.setdefault(
            var,
            {
                "name": var,
                "type": decl_type,
                "storage": "stack_object",
                "constructor": decl_type,
                "evidence": match.group(0).strip()[:200],
            },
        )

    member_calls: list[dict[str, str]] = []
    call_spans: list[tuple[int, int]] = []
    member_call_pat = re.compile(r"\b(?P<receiver>[A-Za-z_]\w*)\s*(?P<op>\.|->)\s*(?P<member>[A-Za-z_]\w*)\s*\(")
    for match in member_call_pat.finditer(text):
        receiver = match.group("receiver")
        item = {
            "receiver": receiver,
            "operator": match.group("op"),
            "member": match.group("member"),
            "receiver_type": objects.get(receiver, {}).get("type", ""),
            "call": f"{receiver}{match.group('op')}{match.group('member')}",
        }
        _append_unique(member_calls, item)
        call_spans.append((match.start(), match.end()))

    field_accesses: list[dict[str, str]] = []
    field_pat = re.compile(r"\b(?P<receiver>[A-Za-z_]\w*)\s*(?P<op>\.|->)\s*(?P<field>[A-Za-z_]\w*)\b")
    for match in field_pat.finditer(text):
        if any(start <= match.start() < end for start, end in call_spans):
            continue
        receiver = match.group("receiver")
        item = {
            "receiver": receiver,
            "operator": match.group("op"),
            "field": match.group("field"),
            "receiver_type": objects.get(receiver, {}).get("type", ""),
            "access": f"{receiver}{match.group('op')}{match.group('field')}",
        }
        _append_unique(field_accesses, item)

    return {
        "objects": list(objects.values()),
        "member_calls": member_calls,
        "field_accesses": field_accesses,
    }


def _render_prompt_block(manifest_path: Path, manifest: dict[str, Any]) -> str:
    """渲染紧凑 prompt block，只放可操作事实和关键警告。"""
    lines = [
        "## Translation Truth Manifest",
        f"Manifest: {manifest_path.resolve()}",
        f"Function key: {manifest['function'].get('func_key')}",
        f"Source layer: {manifest['function'].get('source_layer')}",
        "Use this as structured truth. RAG and weak declarations are advisory and must not override it.",
    ]

    summary = manifest.get("dependency_summary") or {}
    fact_counts = summary.get("fact_counts_by_kind") or {}
    if fact_counts:
        counts = ", ".join(f"{kind}={count}" for kind, count in sorted(fact_counts.items()))
        lines.append(f"Dependency facts: {counts}")

    cpp = manifest.get("cpp_receiver_facts") or {}
    objects = cpp.get("objects") or []
    member_calls = cpp.get("member_calls") or []
    field_accesses = cpp.get("field_accesses") or []
    if objects:
        lines.append("C++ local/heap objects:")
        for item in objects[:12]:
            typ = f": {item.get('type')}" if item.get("type") else ""
            lines.append(f"- {item.get('name')}{typ} ({item.get('storage')})")
    if member_calls:
        lines.append("C++ member calls are receiver-bound; do not translate them as free functions without proof:")
        for item in member_calls[:20]:
            receiver_type = f" [{item.get('receiver_type')}]" if item.get("receiver_type") else ""
            lines.append(f"- {item.get('call')}(...) receiver={item.get('receiver')}{receiver_type}")
    if field_accesses:
        lines.append("C++ field/member accesses need receiver state or accessor facts:")
        for item in field_accesses[:12]:
            receiver_type = f" [{item.get('receiver_type')}]" if item.get("receiver_type") else ""
            lines.append(f"- {item.get('access')} receiver={item.get('receiver')}{receiver_type}")
    if member_calls or field_accesses:
        rust_sig = str(manifest.get("function", {}).get("rust_signature") or "")
        if "&self" not in rust_sig and "&mut self" not in rust_sig:
            lines.append(
                "Receiver contract: the target Rust signature has no implicit `self`; do not invent globals, bare fields, "
                "or replacement storage for receiver-bound C++ state. Use only explicit local values, declared globals/accessors, "
                "or Callable Rust APIs listed by dependency closure."
            )

    ambiguous = summary.get("ambiguous_callees") or []
    if ambiguous:
        grouped: dict[str, set[str]] = {}
        for item in ambiguous:
            if not isinstance(item, dict):
                continue
            symbol = str(item.get("symbol") or "").strip()
            rust_path = str(item.get("rust_path") or "").strip()
            if symbol and rust_path:
                grouped.setdefault(symbol, set()).add(rust_path)
        if grouped:
            lines.append("Ambiguous same-name callees are not hard call targets:")
            for symbol in sorted(grouped)[:12]:
                paths = ", ".join(sorted(grouped[symbol]))
                lines.append(f"- {symbol}: {paths}")

    gaps = summary.get("gap_counts_by_kind") or {}
    if gaps:
        gap_counts = ", ".join(f"{kind}={count}" for kind, count in sorted(gaps.items()))
        lines.append(f"Known truth gaps: {gap_counts}. Do not invent ABI, constructors, fields, or module paths for gaps.")

    return "\n".join(lines).strip()


def build_truth_manifest(
    func_info: Any,
    state: Any,
    *,
    dependency_manifest: dict[str, Any] | None = None,
    dependency_manifest_path: Path | str | None = None,
    output_dir: Path | None = None,
) -> TruthManifestBundle:
    """生成函数级 truth manifest 和 prompt block。"""
    func_key = _func_key(func_info)
    truth_info = _preprocessed_truth_info(func_info, state)
    manifest_dir = Path(output_dir) if output_dir is not None else Path(getattr(state, "context_cache_dir", Path("."))) / "truth_manifest"
    manifest_dir.mkdir(parents=True, exist_ok=True)
    manifest_path = manifest_dir / f"truth_{_safe_name(func_key)}.json"

    source_layer = "preprocessed" if truth_info else "original"
    dependency_summary = _summarize_facts(dependency_manifest)
    cpp_receiver_facts = _extract_cpp_receiver_facts(str(getattr(func_info, "c_code", "") or ""))
    manifest = {
        "schema_version": "c2r_translation_truth_manifest_v1",
        "function": {
            "func_key": func_key,
            "c_name": str(getattr(func_info, "name", "") or ""),
            "file_name": str(getattr(func_info, "file_name", "") or ""),
            "index": int(getattr(func_info, "index", 0) or 0),
            "rust_signature": str(getattr(func_info, "rust_signature", "") or ""),
            "source_layer": source_layer,
            "preprocessed_file": str(truth_info.get("preprocessed_file") or ""),
        },
        "evidence_paths": {
            "dependency_closure_manifest": str(Path(dependency_manifest_path).resolve()) if dependency_manifest_path else "",
        },
        "dependency_summary": dependency_summary,
        "cpp_receiver_facts": cpp_receiver_facts,
        "policy": {
            "rag": "advisory_only",
            "weak_declarations": "advisory_only",
            "member_calls": "receiver_bound_not_free_functions",
        },
    }
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return TruthManifestBundle(
        prompt_block=_render_prompt_block(manifest_path, manifest),
        manifest_path=manifest_path.resolve(),
        manifest=manifest,
    )
