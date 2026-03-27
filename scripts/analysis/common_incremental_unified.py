from __future__ import annotations

import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


def _normalize_function_name(name: str) -> str:
    return "".join(ch for ch in name.lower() if ch.isalnum())


def _skip_ws(s: str, pos: int) -> int:
    while pos < len(s) and s[pos].isspace():
        pos += 1
    return pos


def _skip_line_comment(s: str, pos: int) -> int:
    nl = s.find("\n", pos + 2)
    return len(s) if nl == -1 else nl + 1


def _skip_block_comment(s: str, pos: int) -> int:
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


def _skip_string(s: str, pos: int) -> int:
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


def _scan_balanced(s: str, pos: int, open_ch: str, close_ch: str) -> Optional[int]:
    if pos >= len(s) or s[pos] != open_ch:
        return None
    depth = 1
    pos += 1
    while pos < len(s) and depth > 0:
        if s.startswith("//", pos):
            pos = _skip_line_comment(s, pos)
            continue
        if s.startswith("/*", pos):
            pos = _skip_block_comment(s, pos)
            continue
        c = s[pos]
        if c == '"':
            pos = _skip_string(s, pos)
            continue
        if c == "'":
            lookahead = s[pos + 1:pos + 12]
            closing_quote_pos = lookahead.find("'")
            if 0 < closing_quote_pos < 10:
                pos += closing_quote_pos + 2
                continue
            pos += 1
            continue
        if c == open_ch:
            depth += 1
        elif c == close_ch:
            depth -= 1
        pos += 1
    return pos if depth == 0 else None


def _find_fn_item_span(s: str, fn_kw_pos: int, fn_name: str) -> Optional[Tuple[int, int, int]]:
    pos = fn_kw_pos + 2
    pos = _skip_ws(s, pos)
    if not s.startswith(fn_name, pos):
        return None
    pos += len(fn_name)
    pos = _skip_ws(s, pos)

    if pos < len(s) and s[pos] == "<":
        end = _scan_balanced(s, pos, "<", ">")
        if end is None:
            return None
        pos = _skip_ws(s, end)

    if pos >= len(s) or s[pos] != "(":
        return None
    end = _scan_balanced(s, pos, "(", ")")
    if end is None:
        return None
    pos = _skip_ws(s, end)

    angle = 0
    paren = 0
    bracket = 0
    while pos < len(s):
        if s.startswith("//", pos):
            pos = _skip_line_comment(s, pos)
            continue
        if s.startswith("/*", pos):
            pos = _skip_block_comment(s, pos)
            continue
        c = s[pos]
        if c == '"':
            pos = _skip_string(s, pos)
            continue
        if c == "'":
            lookahead = s[pos + 1:pos + 12]
            closing_quote_pos = lookahead.find("'")
            if 0 < closing_quote_pos < 10:
                pos += closing_quote_pos + 2
                continue
            pos += 1
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
            body_end = _scan_balanced(s, pos, "{", "}")
            if body_end is None:
                return None
            line_start = s.rfind("\n", 0, fn_kw_pos)
            item_start = 0 if line_start == -1 else line_start + 1
            return (item_start, body_start, body_end)
        elif c == ";" and paren == 0 and angle == 0 and bracket == 0:
            return None
        pos += 1
    return None


def _is_placeholder_body(body_with_braces: str) -> bool:
    body = body_with_braces.strip()
    if body.startswith("{") and body.endswith("}"):
        body = body[1:-1].strip()
    if not body:
        return False
    if re.match(r"^\s*unimplemented!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True
    if re.match(r"^\s*todo!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True
    if re.match(r"^\s*unreachable!\s*\([^)]*\)\s*;?\s*$", body, re.DOTALL):
        return True
    if re.match(
        r"^\s*panic!\s*\(\s*['\"].*not\s*implement.*['\"]\s*\)\s*;?\s*$",
        body,
        re.IGNORECASE | re.DOTALL,
    ):
        return True
    return False


def stub_all_functions_in_content(content: str) -> str:
    fn_pat = re.compile(r"\bfn\s+([A-Za-z_]\w*)\b")
    out: List[str] = []
    last = 0
    for m in fn_pat.finditer(content):
        if m.start() < last:
            continue
        fn_name = m.group(1)
        span = _find_fn_item_span(content, m.start(), fn_name)
        if span is None:
            continue
        _item_start, body_start, body_end = span
        if body_start < last:
            continue
        out.append(content[last:body_start])
        out.append("{ unimplemented!() }")
        last = body_end
    out.append(content[last:])
    return "".join(out)


def extract_function_from_content(content: str, func_name: str) -> Optional[str]:
    pat = re.compile(rf"\bfn\s+{re.escape(func_name)}\b")
    for m in pat.finditer(content):
        span = _find_fn_item_span(content, m.start(), func_name)
        if span is None:
            continue
        item_start, _body_start, body_end = span
        return content[item_start:body_end]
    return None


def restore_function_in_content(content: str, func_name: str, original_code: str) -> str:
    pat = re.compile(rf"\bfn\s+{re.escape(func_name)}\b")
    for m in pat.finditer(content):
        span = _find_fn_item_span(content, m.start(), func_name)
        if span is None:
            continue
        item_start, body_start, body_end = span
        body = content[body_start:body_end]
        if not re.search(r"^\{\s*unimplemented!\s*\(\s*\)\s*\}", body):
            continue
        return content[:item_start] + original_code + content[body_end:]
    return content


def resolve_standard_source_dir(tests_base_dir: Optional[Path], project_name: str) -> Optional[Path]:
    if tests_base_dir is None:
        return None
    base = Path(tests_base_dir)
    candidates = [
        base / project_name / "c2rust",
        base.parent / "source_rq2_tests" / project_name / "c2rust",
    ]
    for cand in candidates:
        if cand.exists() and list(cand.glob("src_*.rs")):
            return cand
    return None


def load_standard_function_names_from_c2rust(tests_base_dir: Optional[Path], project_name: str) -> List[str]:
    src_dir = resolve_standard_source_dir(tests_base_dir, project_name)
    if src_dir is None:
        return []

    fn_pat = re.compile(r"\bfn\s+([A-Za-z_]\w*)\b")
    names: Dict[str, str] = {}
    for f in sorted(src_dir.glob("src_*.rs")):
        try:
            s = f.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        for m in fn_pat.finditer(s):
            name = m.group(1)
            if name in ("main", "test") or name.startswith("main_") or name.startswith("test_"):
                continue
            span = _find_fn_item_span(s, m.start(), name)
            if span is None:
                continue
            norm = _normalize_function_name(name)
            if norm not in names:
                names[norm] = name
    return [names[k] for k in sorted(names)]


def _default_source_rel_paths(project_dir: Path) -> List[Path]:
    rels: List[Path] = []
    for p in sorted(project_dir.rglob("*.rs")):
        if not p.is_file():
            continue
        if p.name == "build.rs":
            continue
        if "target" in p.parts or ".git" in p.parts:
            continue
        rels.append(p.relative_to(project_dir))
    return rels


def _run_cargo_check(project_dir: Path, timeout: int, env_updates: Optional[Dict[str, str]] = None) -> Tuple[bool, str]:
    with tempfile.TemporaryDirectory(prefix="icomp_target_") as target_dir:
        return _run_cargo_check_with_target(
            project_dir=project_dir,
            target_dir=Path(target_dir),
            timeout=timeout,
            env_updates=env_updates,
        )


def _run_cargo_check_with_target(
    project_dir: Path,
    target_dir: Path,
    timeout: int,
    env_updates: Optional[Dict[str, str]] = None,
) -> Tuple[bool, str]:
    env = {
        **os.environ,
        "CARGO_TARGET_DIR": str(target_dir),
        "RUSTFLAGS": "-Awarnings",
        "RUSTC_BOOTSTRAP": "1",
        "RUST_BACKTRACE": "0",
    }
    if env_updates:
        env.update(env_updates)
    proc = subprocess.run(
        ["cargo", "check", "--offline"],
        cwd=project_dir,
        capture_output=True,
        text=True,
        timeout=timeout,
        env=env,
    )
    output = (proc.stderr or "") + (proc.stdout or "")
    return proc.returncode == 0, output


def verify_incremental_compilation_unified(
    project_dir: Path,
    project_name: str,
    tests_base_dir: Optional[Path],
    timeout: int = 30,
    source_rel_paths: Optional[List[Path]] = None,
    env_updates: Optional[Dict[str, str]] = None,
) -> Dict[str, Any]:
    result: Dict[str, Any] = {
        "total_functions": 0,
        "tested_functions": 0,
        "compiled_functions": 0,
        "compile_rate": 0.0,
        "compiled_functions_all": 0,
        "compile_rate_all": 0.0,
        "llm_functions": 0,
        "c2rust_fallback_functions": 0,
        "unimplemented_functions": 0,
        "missing_functions": 0,
        "llm_compiled_functions": 0,
        "functions_detail": {},
        "baseline_compilation_succeeded": None,
        "baseline_error": None,
        "skeleton_compilation_succeeded": None,
        "skeleton_error": None,
        "error": None,
    }

    if not project_dir.exists():
        result["error"] = "Project directory does not exist"
        return result
    if not (project_dir / "Cargo.toml").exists():
        result["error"] = "Cargo.toml not found"
        return result

    standard_names = load_standard_function_names_from_c2rust(tests_base_dir, project_name)
    if not standard_names:
        result["error"] = "No standard functions found from C2Rust reference sources"
        return result
    result["total_functions"] = len(standard_names)
    result["tested_functions"] = len(standard_names)

    rel_paths = source_rel_paths or _default_source_rel_paths(project_dir)
    if not rel_paths:
        result["error"] = "No Rust source files found"
        return result

    with tempfile.TemporaryDirectory(prefix="icomp_unified_") as td:
        temp_project = Path(td) / project_dir.name
        shutil.copytree(project_dir, temp_project, ignore=shutil.ignore_patterns("target", ".git"))
        cargo_target_dir = Path(td) / ".cargo_target"
        cargo_target_dir.mkdir(parents=True, exist_ok=True)

        ok, output = _run_cargo_check_with_target(
            temp_project,
            cargo_target_dir,
            timeout=max(timeout, 120),
            env_updates=env_updates,
        )
        result["baseline_compilation_succeeded"] = ok
        if not ok:
            result["baseline_error"] = output[:1500]
            result["error"] = "Baseline project does not compile"
            return result

        original_contents: Dict[Path, str] = {}
        candidates_by_norm: Dict[str, List[Tuple[str, Path, str]]] = {}
        fn_pat = re.compile(r"\bfn\s+([A-Za-z_]\w*)\b")

        for rel in rel_paths:
            src_file = temp_project / rel
            if not src_file.exists():
                continue
            content = src_file.read_text(encoding="utf-8", errors="ignore")
            original_contents[rel] = content
            for m in fn_pat.finditer(content):
                func_name = m.group(1)
                if func_name in ("main", "test") or func_name.startswith("main_") or func_name.startswith("test_"):
                    continue
                func_code = extract_function_from_content(content, func_name)
                if not func_code:
                    continue
                norm = _normalize_function_name(func_name)
                candidates_by_norm.setdefault(norm, []).append((func_name, rel, func_code))

        for norm in candidates_by_norm:
            candidates_by_norm[norm].sort(key=lambda item: (item[0], str(item[1])))

        stubbed_baseline: Dict[Path, str] = {}
        for rel, content in original_contents.items():
            stubbed = stub_all_functions_in_content(content)
            stubbed_baseline[rel] = stubbed
            (temp_project / rel).write_text(stubbed, encoding="utf-8")

        ok, output = _run_cargo_check_with_target(
            temp_project,
            cargo_target_dir,
            timeout=max(timeout, 120),
            env_updates=env_updates,
        )
        result["skeleton_compilation_succeeded"] = ok
        if not ok:
            result["skeleton_error"] = output[:1500]
            result["error"] = "Stubbed skeleton does not compile"
            return result

        compiled_count = 0
        fallback_count = 0
        placeholder_count = 0
        missing_count = 0

        for std_name in standard_names:
            norm = _normalize_function_name(std_name)
            candidates = candidates_by_norm.get(norm) or []
            if not candidates:
                missing_count += 1
                result["functions_detail"][std_name] = {
                    "compiled": False,
                    "source": "missing",
                    "counted_in_icomp": True,
                    "error": "Function not found in final output",
                }
                continue

            chosen = None
            for candidate in candidates:
                if candidate[0] == std_name:
                    chosen = candidate
                    break
            if chosen is None:
                chosen = candidates[0]

            func_name, rel, func_code = chosen
            brace_pos = func_code.find("{")
            body = func_code[brace_pos:] if brace_pos != -1 else ""
            source = "fallback" if ("__c2rust_fallback" in func_code or "C2Rust fallback" in func_code) else "final"
            if source == "fallback":
                fallback_count += 1

            if _is_placeholder_body(body):
                placeholder_count += 1
                result["functions_detail"][std_name] = {
                    "compiled": False,
                    "source": "unimplemented",
                    "matched_func": func_name,
                    "matched_file": str(rel),
                    "counted_in_icomp": True,
                    "error": "Function body is a placeholder",
                }
                continue

            current_content = (temp_project / rel).read_text(encoding="utf-8", errors="ignore")
            restored = restore_function_in_content(current_content, func_name, func_code)
            (temp_project / rel).write_text(restored, encoding="utf-8")
            try:
                ok, output = _run_cargo_check_with_target(
                    temp_project,
                    cargo_target_dir,
                    timeout=timeout,
                    env_updates=env_updates,
                )
            except subprocess.TimeoutExpired:
                ok = False
                output = "Compilation timeout"
            finally:
                (temp_project / rel).write_text(stubbed_baseline[rel], encoding="utf-8")

            if ok:
                compiled_count += 1
                result["functions_detail"][std_name] = {
                    "compiled": True,
                    "source": source,
                    "matched_func": func_name,
                    "matched_file": str(rel),
                    "counted_in_icomp": True,
                }
            else:
                result["functions_detail"][std_name] = {
                    "compiled": False,
                    "source": source,
                    "matched_func": func_name,
                    "matched_file": str(rel),
                    "counted_in_icomp": True,
                    "error": output[:500],
                }

        result["compiled_functions"] = compiled_count
        result["compiled_functions_all"] = compiled_count
        result["llm_compiled_functions"] = compiled_count
        result["compile_rate"] = compiled_count / len(standard_names) if standard_names else 0.0
        result["compile_rate_all"] = result["compile_rate"]
        result["c2rust_fallback_functions"] = fallback_count
        result["unimplemented_functions"] = placeholder_count
        result["missing_functions"] = missing_count
        result["llm_functions"] = max(0, len(standard_names) - fallback_count - placeholder_count - missing_count)
        return result
