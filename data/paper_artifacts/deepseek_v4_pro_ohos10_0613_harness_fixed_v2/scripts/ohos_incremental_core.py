#!/usr/bin/env python3
"""最终 Rust 结果口径的逐函数恢复增量编译率核心实现。"""

from __future__ import annotations

import os
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple


def _run_cmd_capture(cmd: List[str], cwd: Path, env: Dict[str, str], timeout: int) -> Tuple[int, str, str]:
    try:
        p = subprocess.run(
            cmd,
            cwd=cwd,
            env=env,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return p.returncode, p.stdout or "", p.stderr or ""
    except subprocess.TimeoutExpired:
        return 124, "", f"Timeout after {timeout}s"


def _is_placeholder_body(body_with_braces: str) -> Tuple[bool, str]:
    """
    Detect pure placeholder bodies we should NOT treat as "successful LLM output".
    We match only the whole body being a single placeholder statement to avoid false positives.
    """
    body = body_with_braces.strip()
    if body.startswith("{") and body.endswith("}"):
        body = body[1:-1].strip()
    if not body:
        # Empty body could be a valid no-op implementation; don't treat as placeholder.
        return False, ""
    if re.match(r"^\s*unimplemented!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True, "unimplemented!()"
    if re.match(r"^\s*todo!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True, "todo!()"
    if re.match(r"^\s*unreachable!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True, "unreachable!()"
    if re.match(
        r"^\s*panic!\s*\(\s*['\"].*not\s*implement.*['\"]\s*\)\s*;?\s*$",
        body,
        re.IGNORECASE | re.DOTALL,
    ):
        return True, "panic!(not implement)"
    return False, ""


def _is_c2rust_fallback(func_code: str) -> bool:
    return ("__c2rust_fallback" in func_code) or ("C2Rust fallback" in func_code)


def _should_skip_incremental_rs_file(path: Path) -> bool:
    """
    Skip generated/helper Rust files that are not part of the paper denominator.

    In final crates we often keep:
    - `src/__c2r_generated/c2rust_fallback/**`: raw C2Rust fallback dump
    - `.c2r_c2rust_fallback/**`: framework cache
    - `.c2r_bindgen_*/**`: bindgen shims / helper glue

    These helpers may define same-name functions as the real wrapper functions. If we
    scan them here, candidate selection can accidentally pick the fallback body and
    misclassify it as an LLM function.
    """
    parts = set(path.parts)
    if "target" in parts or ".git" in parts:
        return True
    if "__c2r_generated" in parts or "c2rust_fallback" in parts or "__c2rust_fallback" in parts:
        return True
    if ".c2r_c2rust_fallback" in parts:
        return True
    return any(part.startswith(".c2r_bindgen_") for part in path.parts)


def _find_mod_block_spans(s: str, mod_name: str) -> List[Tuple[int, int]]:
    """
    Return spans of `mod <mod_name> { ... }` blocks as [(start_brace, end_brace_excl), ...].
    Used to ignore wrapper shims like `mod ffi { ... }`.
    """
    spans: List[Tuple[int, int]] = []

    def skip_ws(pos: int) -> int:
        while pos < len(s) and s[pos].isspace():
            pos += 1
        return pos

    def skip_line_comment(pos: int) -> int:
        nl = s.find("\n", pos + 2)
        return len(s) if nl == -1 else nl + 1

    def skip_block_comment(pos: int) -> int:
        depth = 1
        pos += 2
        while pos < len(s) and depth > 0:
            if s.startswith("/*", pos):
                depth += 1
                pos += 2
            elif s.startswith("*/", pos):
                depth -= 1
                pos += 2
            else:
                pos += 1
        return pos

    def skip_string(pos: int) -> int:
        quote = s[pos]
        pos += 1
        while pos < len(s):
            c = s[pos]
            if c == "\\":
                pos += 2
                continue
            if c == quote:
                return pos + 1
            pos += 1
        return pos

    def skip_char_literal(pos: int) -> Optional[int]:
        if pos + 2 >= len(s) or s[pos] != "'":
            return None
        if s[pos + 1] == "\\":
            end = skip_string(pos)
            return end if end > pos + 2 and end <= len(s) else None
        if s[pos + 1] in "\r\n":
            return None
        return pos + 3 if s[pos + 2] == "'" else None

    def scan_balanced(pos: int, open_ch: str, close_ch: str) -> Optional[int]:
        if pos >= len(s) or s[pos] != open_ch:
            return None
        depth = 1
        pos += 1
        while pos < len(s) and depth > 0:
            if s.startswith("//", pos):
                pos = skip_line_comment(pos)
                continue
            if s.startswith("/*", pos):
                pos = skip_block_comment(pos)
                continue
            c = s[pos]
            if c == '"':
                pos = skip_string(pos)
                continue
            if c == "'":
                end = skip_char_literal(pos)
                if end is not None:
                    pos = end
                    continue
            if c == open_ch:
                depth += 1
            elif c == close_ch:
                depth -= 1
            pos += 1
        return pos if depth == 0 else None

    pat = re.compile(rf"\bmod\s+{re.escape(mod_name)}\b")
    for m in pat.finditer(s):
        pos = skip_ws(m.end())
        while pos < len(s):
            if s.startswith("//", pos):
                pos = skip_line_comment(pos)
                continue
            if s.startswith("/*", pos):
                pos = skip_block_comment(pos)
                continue
            c = s[pos]
            if c in ('"', "'"):
                pos = skip_string(pos)
                continue
            if c == ";":
                break
            if c == "{":
                end = scan_balanced(pos, "{", "}")
                if end is not None:
                    spans.append((pos, end))
                break
            pos += 1
    return spans


def _iter_rust_fn_items(content: str) -> List[Tuple[str, int, int, int]]:
    """
    Return a list of function items as tuples:
      (fn_name, item_start, body_start, body_end_excl)

    The parser is best-effort but is robust against nested generics/paren/brackets and
    skips comments/strings.
    """

    def mask_non_code(s: str) -> str:
        """
        Return a same-length string with comments and literals replaced by spaces.
        This prevents `fn ... { ... }` text inside comments/strings from being
        treated as real Rust items while keeping byte offsets stable.
        """
        chars = list(s)
        pos = 0
        while pos < len(s):
            if s.startswith("//", pos):
                end = skip_line_comment(s, pos)
                for i in range(pos, end):
                    if chars[i] != "\n":
                        chars[i] = " "
                pos = end
                continue
            if s.startswith("/*", pos):
                end = skip_block_comment(s, pos)
                for i in range(pos, min(end, len(s))):
                    if chars[i] != "\n":
                        chars[i] = " "
                pos = end
                continue

            end = skip_rust_string_literal(s, pos)
            if end is not None:
                for i in range(pos, min(end, len(s))):
                    if chars[i] != "\n":
                        chars[i] = " "
                pos = end
                continue

            end = skip_rust_char_literal(s, pos)
            if end is not None:
                for i in range(pos, min(end, len(s))):
                    if chars[i] != "\n":
                        chars[i] = " "
                pos = end
                continue
            pos += 1
        return "".join(chars)

    def skip_ws(s: str, pos: int) -> int:
        while pos < len(s) and s[pos].isspace():
            pos += 1
        return pos

    def skip_line_comment(s: str, pos: int) -> int:
        nl = s.find("\n", pos + 2)
        return len(s) if nl == -1 else nl + 1

    def skip_block_comment(s: str, pos: int) -> int:
        depth = 1
        pos += 2
        while pos < len(s) and depth > 0:
            if s.startswith("/*", pos):
                depth += 1
                pos += 2
            elif s.startswith("*/", pos):
                depth -= 1
                pos += 2
            else:
                pos += 1
        return pos

    def skip_string(s: str, pos: int) -> int:
        quote = s[pos]
        pos += 1
        while pos < len(s):
            c = s[pos]
            if c == "\\":
                pos += 2
                continue
            if c == quote:
                return pos + 1
            pos += 1
        return pos

    def skip_rust_string_literal(s: str, pos: int) -> Optional[int]:
        """Skip Rust string-like literals, including byte/c/raw strings."""
        for prefix in ("br", "cr", "r"):
            if not s.startswith(prefix, pos):
                continue
            hash_pos = pos + len(prefix)
            qpos = hash_pos
            while qpos < len(s) and s[qpos] == "#":
                qpos += 1
            if qpos >= len(s) or s[qpos] != '"':
                continue
            hashes = s[hash_pos:qpos]
            terminator = '"' + hashes
            search_start = qpos + 1
            end_idx = s.find(terminator, search_start)
            return len(s) if end_idx == -1 else end_idx + len(terminator)

        if s.startswith("b\"", pos) or s.startswith("c\"", pos):
            return skip_string(s, pos + 1)
        if pos < len(s) and s[pos] == '"':
            return skip_string(s, pos)
        return None

    def skip_char_literal(s: str, pos: int) -> Optional[int]:
        if pos + 2 >= len(s) or s[pos] != "'":
            return None
        if s[pos + 1] == "\\":
            end = skip_string(s, pos)
            return end if end > pos + 2 and end <= len(s) else None
        if s[pos + 1] in "\r\n":
            return None
        return pos + 3 if s[pos + 2] == "'" else None

    def skip_rust_char_literal(s: str, pos: int) -> Optional[int]:
        """Skip Rust char and byte-char literals while leaving lifetimes visible."""
        if s.startswith("b'", pos):
            end = skip_char_literal(s, pos + 1)
            return end if end is not None else None
        if pos < len(s) and s[pos] == "'":
            nxt = s[pos + 1] if pos + 1 < len(s) else ""
            if nxt and (nxt == "\\" or nxt == "'" or not (nxt.isalpha() or nxt == "_")):
                return skip_char_literal(s, pos)
        return None

    def scan_balanced(s: str, pos: int, open_ch: str, close_ch: str) -> Optional[int]:
        if pos >= len(s) or s[pos] != open_ch:
            return None
        depth = 1
        pos += 1
        while pos < len(s) and depth > 0:
            if s.startswith("//", pos):
                pos = skip_line_comment(s, pos)
                continue
            if s.startswith("/*", pos):
                pos = skip_block_comment(s, pos)
                continue
            end = skip_rust_string_literal(s, pos)
            if end is not None:
                pos = end
                continue
            c = s[pos]
            end = skip_rust_char_literal(s, pos)
            if end is not None:
                pos = end
                continue
            if c == open_ch:
                depth += 1
            elif c == close_ch:
                depth -= 1
            pos += 1
        return pos if depth == 0 else None

    def find_fn_item_span(s: str, fn_kw_pos: int, fn_name: str) -> Optional[Tuple[int, int, int]]:
        pos = fn_kw_pos + 2
        pos = skip_ws(s, pos)
        if not s.startswith(fn_name, pos):
            return None
        pos += len(fn_name)
        pos = skip_ws(s, pos)

        if pos < len(s) and s[pos] == "<":
            end = scan_balanced(s, pos, "<", ">")
            if end is None:
                return None
            pos = skip_ws(s, end)

        if pos >= len(s) or s[pos] != "(":
            return None
        end = scan_balanced(s, pos, "(", ")")
        if end is None:
            return None
        pos = skip_ws(s, end)

        angle = paren = bracket = 0
        while pos < len(s):
            if s.startswith("//", pos):
                pos = skip_line_comment(s, pos)
                continue
            if s.startswith("/*", pos):
                pos = skip_block_comment(s, pos)
                continue
            end = skip_rust_string_literal(s, pos)
            if end is not None:
                pos = end
                continue
            c = s[pos]
            end = skip_rust_char_literal(s, pos)
            if end is not None:
                pos = end
                continue
            if c == "(":
                paren += 1
            elif c == ")":
                paren = max(0, paren - 1)
            elif c == "[":
                bracket += 1
            elif c == "]":
                bracket = max(0, bracket - 1)
            elif c == "<":
                angle += 1
            elif c == ">":
                if pos > 0 and s[pos - 1] == "-":
                    pass
                else:
                    angle = max(0, angle - 1)
            elif c == "{" and paren == 0 and angle == 0 and bracket == 0:
                body_start = pos
                body_end = scan_balanced(s, pos, "{", "}")
                if body_end is None:
                    return None
                line_start = s.rfind("\n", 0, fn_kw_pos)
                item_start = 0 if line_start == -1 else line_start + 1
                return (item_start, body_start, body_end)
            elif c == ";" and paren == 0 and angle == 0 and bracket == 0:
                return None
            pos += 1
        return None

    masked = mask_non_code(content)
    fn_pat = re.compile(r"\bfn\s+([A-Za-z_]\w*)\b")
    out: List[Tuple[str, int, int, int]] = []
    seen: Set[Tuple[int, int, int, str]] = set()
    for m in fn_pat.finditer(masked):
        name = m.group(1)
        span = find_fn_item_span(content, m.start(), name)
        if span is None:
            continue
        item_start, body_start, body_end = span
        key = (item_start, body_start, body_end, name)
        if key in seen:
            continue
        seen.add(key)
        out.append((name, item_start, body_start, body_end))
    out.sort(key=lambda t: (t[1], t[2], t[3], t[0]))
    return out


def _stub_file_content(
    content: str,
    spans: List[Tuple[str, int, int, int]],
    *,
    stub_indices: Set[int],
    skip_span_index: Optional[int],
) -> str:
    """
    Replace selected function bodies `{ ... }` with `{ loop {} }`.
    Only spans whose indices are in `stub_indices` are stubbed; `skip_span_index` is kept intact.
    We apply replacements from the end to keep indices valid.
    """
    out = content
    repls: List[Tuple[int, int, str]] = []
    for i, (fn_name, _item_start, body_start, body_end) in enumerate(spans):
        if i not in stub_indices:
            continue
        if skip_span_index is not None and i == skip_span_index:
            continue
        repls.append((body_start, body_end, fn_name))
    for body_start, body_end, _fn_name in sorted(repls, key=lambda t: t[0], reverse=True):
        out = out[:body_start] + "{ loop {} }" + out[body_end:]
    return out


def _non_nested_fn_span_indices(spans: List[Tuple[str, int, int, int]]) -> Set[int]:
    """返回不在其它函数体内部的函数 span 序号，避免局部函数嵌套替换互相破坏坐标。"""
    result: Set[int] = set()
    for index, (_name, item_start, _body_start, _body_end) in enumerate(spans):
        nested = False
        for other_index, (_other_name, _other_item_start, other_body_start, other_body_end) in enumerate(spans):
            if index == other_index:
                continue
            if other_body_start < item_start < other_body_end:
                nested = True
                break
        if not nested:
            result.add(index)
    return result


@dataclass(frozen=True)
class _FnCandidate:
    file_path: Path
    span_index: int
    fn_name: str
    item_start: int
    body_start: int
    body_end: int
    in_mod_ffi: bool
    is_stub_file: bool
    func_code: str


def _is_support_rs_file(path: Path) -> bool:
    """判断是否为框架支撑文件，Rust 结果增量分母不统计这些文件。"""
    if _should_skip_incremental_rs_file(path):
        return True
    name = path.name
    if name in {"build.rs", "main.rs", "lib.rs", "types.rs", "globals.rs", "compat.rs", "compatibility.rs"}:
        return True
    if name.endswith("_stub.rs") or name.endswith("_bindings.rs"):
        return True
    parts = set(path.parts)
    return "tests" in parts or "benches" in parts or "examples" in parts


def _collect_rust_result_function_candidates(crate_dir: Path) -> Tuple[List[Path], Dict[Path, str], Dict[Path, List[Tuple[str, int, int, int]]], List[_FnCandidate]]:
    """从最终 Rust crate 收集实际翻译结果中的函数实例。"""
    rs_files: List[Path] = []
    for p in crate_dir.rglob("*.rs"):
        rel = p.relative_to(crate_dir)
        if _is_support_rs_file(rel):
            continue
        rs_files.append(p)
    rs_files.sort()

    originals: Dict[Path, str] = {}
    file_spans: Dict[Path, List[Tuple[str, int, int, int]]] = {}
    candidates: List[_FnCandidate] = []
    for p in rs_files:
        try:
            txt = p.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            txt = ""
        originals[p] = txt
        spans = _iter_rust_fn_items(txt)
        file_spans[p] = spans
        non_nested_indices = _non_nested_fn_span_indices(spans)
        ffi_spans = _find_mod_block_spans(txt, "ffi") if spans else []
        for idx, (fn_name, item_start, body_start, body_end) in enumerate(spans):
            if idx not in non_nested_indices:
                continue
            in_mod_ffi = any(start <= item_start < end for start, end in ffi_spans)
            if in_mod_ffi:
                continue
            func_code = txt[item_start:body_end] if 0 <= item_start < body_end <= len(txt) else ""
            candidates.append(
                _FnCandidate(
                    file_path=p,
                    span_index=idx,
                    fn_name=fn_name,
                    item_start=item_start,
                    body_start=body_start,
                    body_end=body_end,
                    in_mod_ffi=False,
                    is_stub_file=False,
                    func_code=func_code,
                )
            )
    return rs_files, originals, file_spans, candidates


def _rust_result_function_id(crate_dir: Path, candidate: _FnCandidate) -> str:
    """生成稳定的 Rust 函数实例 ID。"""
    rel = candidate.file_path.relative_to(crate_dir).as_posix()
    line = candidate.file_path.read_text(encoding="utf-8", errors="ignore").count("\n", 0, candidate.item_start) + 1
    return f"{rel}:{line}:{candidate.fn_name}"


def verify_incremental_compilation_from_rust_result(
    *,
    crate_dir: Path,
    project_name: str,
    timeout: int = 60,
    count_sources: Set[str],
    default_source: str = "llm",
) -> Dict[str, Any]:
    """
    从最终 Rust 结果自身生成骨架并逐函数恢复验证编译。

    分母为最终 Rust crate 中实际存在的非支撑文件函数实例，不读取 C manifest。
    """
    res: Dict[str, Any] = {
        "denominator_kind": "rust_result_function_instances",
        "total_functions": 0,
        "restored_functions": 0,
        "compiled_functions": 0,
        "compile_rate": 0.0,
        "llm_functions": 0,
        "c2rust_fallback_functions": 0,
        "unimplemented_functions": 0,
        "functions_detail": {},
        "baseline_compilation_succeeded": None,
        "baseline_error": None,
        "skeleton_compilation_succeeded": None,
        "skeleton_error": None,
        "error": None,
    }

    if not (crate_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix=f"rust_result_inc_{project_name}_") as td:
        tmp_root = Path(td)
        tmp_crate = tmp_root / crate_dir.name
        shutil.copytree(crate_dir, tmp_crate, ignore=shutil.ignore_patterns("target", ".git"))

        rs_files, originals, file_spans, candidates = _collect_rust_result_function_candidates(tmp_crate)
        if not rs_files:
            res["error"] = "no translated .rs files found under crate"
            return res

        candidates.sort(key=lambda c: (str(c.file_path), c.item_start, c.span_index, c.fn_name))
        res["total_functions"] = len(candidates)
        if not candidates:
            res["error"] = "no Rust function bodies found in translated result files"
            return res

        stub_indices_by_file: Dict[Path, Set[int]] = {}
        for candidate in candidates:
            stub_indices_by_file.setdefault(candidate.file_path, set()).add(candidate.span_index)

        with tempfile.TemporaryDirectory(prefix=f"rust_result_inc_target_{project_name}_") as tgt:
            env = {
                **os.environ,
                "CARGO_TARGET_DIR": str(Path(tgt)),
                "RUSTFLAGS": "-Awarnings",
                "RUSTC_BOOTSTRAP": "1",
                "RUST_BACKTRACE": "0",
            }
            setup_timeout = max(timeout, 900)
            rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=setup_timeout)
            if rc != 0:
                res["baseline_compilation_succeeded"] = False
                res["baseline_error"] = (err or out)[-4000:]
                res["error"] = "baseline project does not compile"
                return res
            res["baseline_compilation_succeeded"] = True

            stubbed_baseline: Dict[Path, str] = {}
            for p, txt in originals.items():
                spans = file_spans.get(p) or []
                stubbed = _stub_file_content(
                    txt,
                    spans,
                    stub_indices=stub_indices_by_file.get(p, set()),
                    skip_span_index=None,
                )
                stubbed_baseline[p] = stubbed
                p.write_text(stubbed, encoding="utf-8")

            rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=setup_timeout)
            if rc != 0:
                res["skeleton_compilation_succeeded"] = False
                res["skeleton_error"] = (err or out)[-4000:]
                res["error"] = "stubbed skeleton does not compile"
                return res
            res["skeleton_compilation_succeeded"] = True

            ok = 0
            restored_total = 0
            for candidate in candidates:
                fn_id = _rust_result_function_id(tmp_crate, candidate)
                source = "c2rust_fallback" if _is_c2rust_fallback(candidate.func_code) else default_source
                body = originals[candidate.file_path][candidate.body_start:candidate.body_end]
                is_ph, ph_reason = _is_placeholder_body(body)
                if is_ph:
                    res["unimplemented_functions"] += 1
                    res["functions_detail"][fn_id] = {
                        "function": candidate.fn_name,
                        "compiled": False,
                        "source": "unimplemented",
                        "is_placeholder": True,
                        "placeholder_type": ph_reason,
                        "error": f"placeholder body: {ph_reason}",
                    }
                    continue

                restored_total += 1
                if source == "llm":
                    res["llm_functions"] += 1
                elif source == "c2rust_fallback":
                    res["c2rust_fallback_functions"] += 1

                if source not in count_sources:
                    res["functions_detail"][fn_id] = {
                        "function": candidate.fn_name,
                        "compiled": False,
                        "source": source,
                        "error": "not counted source for ICompRate",
                    }
                    continue

                spans = file_spans.get(candidate.file_path) or []
                candidate.file_path.write_text(
                    _stub_file_content(
                        originals[candidate.file_path],
                        spans,
                        stub_indices=stub_indices_by_file.get(candidate.file_path, set()),
                        skip_span_index=candidate.span_index,
                    ),
                    encoding="utf-8",
                )
                rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=timeout)
                if rc == 0:
                    ok += 1
                    res["functions_detail"][fn_id] = {
                        "function": candidate.fn_name,
                        "compiled": True,
                        "source": source,
                    }
                else:
                    res["functions_detail"][fn_id] = {
                        "function": candidate.fn_name,
                        "compiled": False,
                        "source": source,
                        "error": (err or out)[-800:],
                    }
                candidate.file_path.write_text(stubbed_baseline[candidate.file_path], encoding="utf-8")

            res["compiled_functions"] = ok
            res["restored_functions"] = restored_total
            res["compile_rate"] = (ok / restored_total) if restored_total else 0.0
            return res
