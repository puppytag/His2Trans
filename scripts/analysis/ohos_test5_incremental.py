#!/usr/bin/env python3
"""
OHOS(test5) incremental compilation verification helpers.

Why this module exists:
- We want a *method-independent* denominator ("standard functions") and a truthful,
  comparable ICompRate definition across different translation methods.
- We want robust stubbing that works for const fn etc. (use `{ loop {} }` instead of
  `unimplemented!()`), and we must not silently drop error projects from the summary.

Standard functions source:
- Prefer our framework's extracted `functions_manifest.json` under:
    <standard_run_dir>/intermediate/<project>/workspace/extracted/<project>/functions_manifest.json
  which is generated from the C sources and thus defines the canonical target set.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple


_REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_STANDARD_RUN_DIR = _REPO_ROOT / "translation_outputs" / "deepseek-coder-ohos5"

# Standard test counts per project (from Ours full run).
# This is the canonical expected test count for each project.
STANDARD_TEST_COUNTS: Dict[str, int] = {
    "appverify_lite__e5ebe91a98b9": 8,
    "host__25c1898e1626": 8,
    "osal__0bc4f21396ad": 4,
    "shared__12e38ea922f7": 15,
    "shared__541f4e547bdb": 5,
}
STANDARD_TEST_TOTAL = sum(STANDARD_TEST_COUNTS.values())  # 40


def load_standard_test_counts(
    project_name: Optional[str] = None,
    standard_run_dir: Optional[Path] = None,
) -> Dict[str, int]:
    """
    Load canonical per-project test counts from Ours analysis result.
    If project_name is given, returns {project_name: count}; otherwise returns all.
    Falls back to hardcoded STANDARD_TEST_COUNTS if the JSON is unavailable.
    """
    # Try to load from the actual analysis JSON for freshness.
    if standard_run_dir is None:
        standard_run_dir = DEFAULT_STANDARD_RUN_DIR if DEFAULT_STANDARD_RUN_DIR.is_dir() else None
    if standard_run_dir is not None:
        analysis_json = standard_run_dir / "results" / "compilation_analysis_ohos_test5.json"
        if analysis_json.is_file():
            try:
                obj = json.loads(analysis_json.read_text(encoding="utf-8", errors="replace"))
                projects = obj.get("projects") if isinstance(obj, dict) else {}
                if isinstance(projects, dict):
                    counts: Dict[str, int] = {}
                    for name, pr in projects.items():
                        if not isinstance(pr, dict):
                            continue
                        ut = pr.get("ohos_unit_tests") or {}
                        total = ut.get("total_tests")
                        if isinstance(total, int) and total > 0:
                            counts[name] = total
                    if counts:
                        if project_name:
                            return {project_name: counts.get(project_name, 0)}
                        return counts
            except Exception:
                pass

    # Fallback to hardcoded values.
    if project_name:
        return {project_name: STANDARD_TEST_COUNTS.get(project_name, 0)}
    return dict(STANDARD_TEST_COUNTS)


def get_expected_test_count(project_name: str, standard_run_dir: Optional[Path] = None) -> int:
    """Get the expected test count for a single project."""
    counts = load_standard_test_counts(project_name, standard_run_dir)
    return counts.get(project_name, 0)


def load_standard_function_names(project_name: str, standard_run_dir: Optional[Path]) -> List[str]:
    """
    Load canonical per-project function names from our extracted manifest (C-derived).
    Returns [] if the manifest is not available.
    """
    if standard_run_dir is None:
        standard_run_dir = DEFAULT_STANDARD_RUN_DIR if DEFAULT_STANDARD_RUN_DIR.is_dir() else None
    if standard_run_dir is None:
        return []
    manifest = (
        standard_run_dir
        / "intermediate"
        / project_name
        / "workspace"
        / "extracted"
        / project_name
        / "functions_manifest.json"
    )
    if not manifest.is_file():
        return []
    try:
        obj = json.loads(manifest.read_text(encoding="utf-8", errors="replace"))
    except Exception:
        return []
    funcs = obj.get("functions") if isinstance(obj, dict) else None
    if not isinstance(funcs, list):
        return []
    out: List[str] = []
    for f in funcs:
        if not isinstance(f, dict):
            continue
        name = str(f.get("name") or "").strip()
        if not name:
            continue
        # Keep consistency with existing analyzers: exclude harness entrypoints.
        if name in ("main", "test") or name.startswith("main_"):
            continue
        out.append(name)
    # Manifests are already unique in our pipeline; keep deterministic order anyway.
    return sorted(out)


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

        # Rust raw string literals: r"…", r#"…"#, br#"…"#, etc.
        if quote == '"':
            hashes = 0
            j = pos - 1
            while j >= 0 and s[j] == "#":
                hashes += 1
                j -= 1
            if j >= 0 and s[j] == "r":
                prefix_start = j - 1 if j - 1 >= 0 and s[j - 1] == "b" else j
                if prefix_start == 0 or not (s[prefix_start - 1].isalnum() or s[prefix_start - 1] == "_"):
                    tail = "#" * hashes
                    end = pos + 1
                    while True:
                        end = s.find('"', end)
                        if end == -1:
                            return len(s)
                        if s.startswith(tail, end + 1):
                            return end + 1 + hashes
                        end += 1

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
        """
        Skip a Rust char literal like: 'a', '\\n', '\\x7F', '\\u{10FFFF}'.
        Return the end position (exclusive) if this is a char literal; otherwise return None
        (so callers can treat it as a lifetime/label like `'a` / `'exit:`).
        """
        i = pos + 1
        if i >= len(s):
            return None
        hexdigits = "0123456789abcdefABCDEF"
        if s[i] == "\\":
            i += 1
            if i >= len(s):
                return None
            esc = s[i]
            if esc in ("\\", "'", '"', "n", "r", "t", "0"):
                i += 1
            elif esc == "x":
                i += 1
                if i + 1 >= len(s):
                    return None
                if (s[i] not in hexdigits) or (s[i + 1] not in hexdigits):
                    return None
                i += 2
            elif esc == "u":
                i += 1
                if i >= len(s) or s[i] != "{":
                    return None
                i += 1
                start = i
                while i < len(s) and s[i] != "}":
                    if s[i] not in hexdigits:
                        return None
                    i += 1
                    if i - start > 6:
                        return None
                if i >= len(s) or s[i] != "}":
                    return None
                if i == start:
                    return None
                i += 1
            else:
                return None
        else:
            # Consume one Unicode scalar (1 Python char). If it's a lifetime/label, there is no closing quote.
            i += 1
        if i < len(s) and s[i] == "'":
            return i + 1
        return None

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
            if c == '"':
                pos = skip_string(pos)
                continue
            if c == "'":
                end = skip_char_literal(pos)
                if end is not None:
                    pos = end
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

        # Rust raw string literals: r"…", r#"…"#, br#"…"#, etc.
        if quote == '"':
            hashes = 0
            j = pos - 1
            while j >= 0 and s[j] == "#":
                hashes += 1
                j -= 1
            if j >= 0 and s[j] == "r":
                prefix_start = j - 1 if j - 1 >= 0 and s[j - 1] == "b" else j
                if prefix_start == 0 or not (s[prefix_start - 1].isalnum() or s[prefix_start - 1] == "_"):
                    tail = "#" * hashes
                    end = pos + 1
                    while True:
                        end = s.find('"', end)
                        if end == -1:
                            return len(s)
                        if s.startswith(tail, end + 1):
                            return end + 1 + hashes
                        end += 1

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

    def skip_char_literal(s: str, pos: int) -> Optional[int]:
        """
        Skip a Rust char literal like: 'a', '\\n', '\\x7F', '\\u{10FFFF}'.
        Return the end position (exclusive) if this is a char literal; otherwise return None
        (so callers can treat it as a lifetime/label like `'a` / `'exit:`).
        """
        i = pos + 1
        if i >= len(s):
            return None
        hexdigits = "0123456789abcdefABCDEF"
        if s[i] == "\\":
            i += 1
            if i >= len(s):
                return None
            esc = s[i]
            if esc in ("\\", "'", '"', "n", "r", "t", "0"):
                i += 1
            elif esc == "x":
                i += 1
                if i + 1 >= len(s):
                    return None
                if (s[i] not in hexdigits) or (s[i + 1] not in hexdigits):
                    return None
                i += 2
            elif esc == "u":
                i += 1
                if i >= len(s) or s[i] != "{":
                    return None
                i += 1
                start = i
                while i < len(s) and s[i] != "}":
                    if s[i] not in hexdigits:
                        return None
                    i += 1
                    if i - start > 6:
                        return None
                if i >= len(s) or s[i] != "}":
                    return None
                if i == start:
                    return None
                i += 1
            else:
                return None
        else:
            # Consume one Unicode scalar (1 Python char). If it's a lifetime/label, there is no closing quote.
            i += 1
        if i < len(s) and s[i] == "'":
            return i + 1
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
            c = s[pos]
            if c == '"':
                pos = skip_string(s, pos)
                continue
            if c == "'":
                end = skip_char_literal(s, pos)
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
            c = s[pos]
            if c == '"':
                pos = skip_string(s, pos)
                continue
            if c == "'":
                end = skip_char_literal(s, pos)
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

    out: List[Tuple[str, int, int, int]] = []
    seen: Set[Tuple[int, int, int, str]] = set()
    # IMPORTANT: don't regex-scan blindly, because `fn foo(...)` may appear in comments/strings.
    pos = 0
    n = len(content)
    while pos < n:
        if content.startswith("//", pos):
            pos = skip_line_comment(content, pos)
            continue
        if content.startswith("/*", pos):
            pos = skip_block_comment(content, pos)
            continue
        c = content[pos]
        if c == '"':
            pos = skip_string(content, pos)
            continue
        if c == "'":
            end = skip_char_literal(content, pos)
            if end is not None:
                pos = end
                continue

        if c == "f" and content.startswith("fn", pos):
            before = content[pos - 1] if pos > 0 else " "
            after = content[pos + 2] if pos + 2 < n else " "
            if (not (before.isalnum() or before == "_")) and after.isspace():
                name_pos = skip_ws(content, pos + 2)
                if name_pos < n and (content[name_pos].isalpha() or content[name_pos] == "_"):
                    end = name_pos + 1
                    while end < n and (content[end].isalnum() or content[end] == "_"):
                        end += 1
                    name = content[name_pos:end]
                    span = find_fn_item_span(content, pos, name)
                    if span is not None:
                        item_start, body_start, body_end = span
                        key = (item_start, body_start, body_end, name)
                        if key not in seen:
                            seen.add(key)
                            out.append((name, item_start, body_start, body_end))
            pos += 2
            continue

        pos += 1
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


def verify_incremental_compilation_standard(
    *,
    crate_dir: Path,
    project_name: str,
    standard_func_names: List[str],
    timeout: int = 60,
    count_sources: Set[str],
    # If provided: only functions in this set are considered "LLM" for counting.
    llm_success_names: Optional[Set[str]] = None,
    default_source: str = "llm",
) -> Dict[str, Any]:
    """
    Per-function isolated compilation:
    - Denominator is `standard_func_names` (C-derived).
    - Numerator counts only functions whose *source* is in `count_sources` and whose body compiles.
    - Placeholder/unimplemented bodies are treated as failures (not counted), even though they may compile.
    """
    res: Dict[str, Any] = {
        "total_functions": len(standard_func_names),
        "compiled_functions": 0,
        "compile_rate": 0.0,
        "llm_functions": 0,
        "c2rust_fallback_functions": 0,
        "unimplemented_functions": 0,
        "missing_functions": 0,
        "functions_detail": {},
        "baseline_compilation_succeeded": None,
        "baseline_error": None,
        "skeleton_compilation_succeeded": None,
        "skeleton_error": None,
        "error": None,
    }

    if not standard_func_names:
        res["error"] = "standard function list is empty (manifest missing?)"
        return res

    if not (crate_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        # Keep denom from standard list; treat as 0% instead of dropping it from summaries.
        return res

    # Copy crate into a temp dir so we never mutate translation outputs.
    with tempfile.TemporaryDirectory(prefix=f"ohos5_inc_{project_name}_") as td:
        tmp_root = Path(td)
        tmp_crate = tmp_root / crate_dir.name
        shutil.copytree(crate_dir, tmp_crate, ignore=shutil.ignore_patterns("target", ".git"))

        # Collect .rs files across common layouts (crate root, src/, nested modules).
        rs_files: List[Path] = []
        for p in tmp_crate.rglob("*.rs"):
            if "target" in p.parts or ".git" in p.parts:
                continue
            rs_files.append(p)
        rs_files.sort()
        if not rs_files:
            res["error"] = "no .rs files found under crate"
            return res

        originals: Dict[Path, str] = {}
        for p in rs_files:
            try:
                originals[p] = p.read_text(encoding="utf-8", errors="ignore")
            except Exception:
                originals[p] = ""

        # Pre-scan function spans and mod ffi spans per file.
        file_spans: Dict[Path, List[Tuple[str, int, int, int]]] = {}
        ffi_spans_by_file: Dict[Path, List[Tuple[int, int]]] = {}
        by_name: Dict[str, List[_FnCandidate]] = {}
        for p, txt in originals.items():
            spans = _iter_rust_fn_items(txt)
            file_spans[p] = spans
            ffi_spans_by_file[p] = _find_mod_block_spans(txt, "ffi") if spans else []
            is_stub_file = p.name.endswith("_stub.rs")
            ffi_spans = ffi_spans_by_file[p]
            for idx, (fn_name, item_start, body_start, body_end) in enumerate(spans):
                in_mod_ffi = any(start <= item_start < end for start, end in ffi_spans)
                func_code = txt[item_start:body_end] if 0 <= item_start < body_end <= len(txt) else ""
                by_name.setdefault(fn_name, []).append(
                    _FnCandidate(
                        file_path=p,
                        span_index=idx,
                        fn_name=fn_name,
                        item_start=item_start,
                        body_start=body_start,
                        body_end=body_end,
                        in_mod_ffi=in_mod_ffi,
                        is_stub_file=is_stub_file,
                        func_code=func_code,
                    )
                )

        # Prepare env for cargo check (do not write target/ into the crate).
        with tempfile.TemporaryDirectory(prefix=f"ohos5_inc_target_{project_name}_") as tgt:
            env = {
                **os.environ,
                "CARGO_TARGET_DIR": str(Path(tgt)),
                "RUSTFLAGS": "-Awarnings",
                "RUSTC_BOOTSTRAP": "1",
                "RUST_BACKTRACE": "0",
            }

            # Baseline compile: if this fails, incremental is 0% (truthful).
            # Large OHOS crates can take a while to (re)build from a clean target dir.
            # Use a generous setup timeout so we don't incorrectly report 0% due to timeouts.
            setup_timeout = max(timeout, 900)
            rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=setup_timeout)
            if rc != 0:
                res["baseline_compilation_succeeded"] = False
                res["baseline_error"] = (err or out)[-4000:]
                res["error"] = "baseline project does not compile"
                return res
            res["baseline_compilation_succeeded"] = True

            # Select exactly one candidate per *standard* function name. We will stub only those
            # selected spans (avoid touching wrapper shims / helper functions outside the denominator).
            chosen_by_name: Dict[str, _FnCandidate] = {}
            stub_indices_by_file: Dict[Path, Set[int]] = {}
            for fn_name in sorted(standard_func_names):
                cands = by_name.get(fn_name, [])
                filtered = [c for c in cands if (not c.in_mod_ffi) and (not c.is_stub_file)]
                chosen = (
                    sorted(filtered, key=lambda c: (str(c.file_path), c.item_start, c.span_index))[0]
                    if filtered
                    else (
                        sorted(cands, key=lambda c: (str(c.file_path), c.item_start, c.span_index))[0]
                        if cands
                        else None
                    )
                )
                if chosen is None:
                    continue
                chosen_by_name[fn_name] = chosen
                stub_indices_by_file.setdefault(chosen.file_path, set()).add(chosen.span_index)

            # Stub all files once (baseline skeleton).
            stubbed_baseline: Dict[Path, str] = {}
            for p, txt in originals.items():
                spans = file_spans.get(p) or []
                stubbed = (
                    _stub_file_content(
                        txt,
                        spans,
                        stub_indices=stub_indices_by_file.get(p, set()),
                        skip_span_index=None,
                    )
                    if spans
                    else txt
                )
                stubbed_baseline[p] = stubbed
                p.write_text(stubbed, encoding="utf-8")

            # Skeleton compile: must compile for per-function isolation to be meaningful.
            rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=setup_timeout)
            if rc != 0:
                res["skeleton_compilation_succeeded"] = False
                res["skeleton_error"] = (err or out)[-4000:]
                res["error"] = "stubbed skeleton does not compile"
                return res
            res["skeleton_compilation_succeeded"] = True

            ok = 0
            for fn_name in sorted(standard_func_names):
                chosen = chosen_by_name.get(fn_name)
                if chosen is None:
                    res["missing_functions"] += 1
                    res["functions_detail"][fn_name] = {
                        "compiled": False,
                        "source": "missing",
                        "counted_in_icomp": True,
                        "error": "function not found in Rust output",
                    }
                    continue

                # Classify source.
                if llm_success_names is not None:
                    source = "llm" if fn_name in llm_success_names else "c2rust_fallback"
                else:
                    source = "c2rust_fallback" if _is_c2rust_fallback(chosen.func_code) else default_source

                # Detect placeholders (treated as failures; do not attempt to compile them).
                file_txt = originals.get(chosen.file_path, "")
                body = file_txt[chosen.body_start:chosen.body_end] if 0 <= chosen.body_start < chosen.body_end <= len(file_txt) else ""
                is_ph, ph_reason = _is_placeholder_body(body)
                if is_ph:
                    res["unimplemented_functions"] += 1
                    res["functions_detail"][fn_name] = {
                        "compiled": False,
                        "source": "unimplemented",
                        "counted_in_icomp": True,
                        "is_placeholder": True,
                        "placeholder_type": ph_reason,
                        "error": f"placeholder body: {ph_reason}",
                    }
                    continue

                if source == "llm":
                    res["llm_functions"] += 1
                elif source == "c2rust_fallback":
                    res["c2rust_fallback_functions"] += 1

                # If this source is not counted, treat as failure in the paper metric.
                if source not in count_sources:
                    res["functions_detail"][fn_name] = {
                        "compiled": False,
                        "source": source,
                        "counted_in_icomp": True,
                        "error": "not counted source for ICompRate",
                    }
                    continue

                # Test: restore only this function (in this file), keep others stubbed.
                spans = file_spans.get(chosen.file_path) or []
                chosen.file_path.write_text(
                    _stub_file_content(
                        originals[chosen.file_path],
                        spans,
                        stub_indices=stub_indices_by_file.get(chosen.file_path, set()),
                        skip_span_index=chosen.span_index,
                    ),
                    encoding="utf-8",
                )
                rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=tmp_crate, env=env, timeout=timeout)
                if rc == 0:
                    ok += 1
                    res["functions_detail"][fn_name] = {
                        "compiled": True,
                        "source": source,
                        "counted_in_icomp": True,
                    }
                else:
                    res["functions_detail"][fn_name] = {
                        "compiled": False,
                        "source": source,
                        "counted_in_icomp": True,
                        "error": (err or out)[-800:],
                    }
                # Revert back to fully stubbed baseline for next iteration.
                chosen.file_path.write_text(stubbed_baseline[chosen.file_path], encoding="utf-8")

            res["compiled_functions"] = ok
            res["compile_rate"] = (ok / res["total_functions"]) if res["total_functions"] else 0.0
            return res
