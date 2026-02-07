#!/usr/bin/env python3
"""
OHOS(10) evaluation for *our framework* outputs.

This script is intentionally separate from `analyze_c2r_compilation_rate.py` to avoid
breaking the original (test_module / 10-project) workflow.

Goals (paper-friendly, no cheating):
- Do NOT modify translation outputs on disk.
- Compute incremental compilation pass rate from our run artifacts (translation summary/stats).
- Run *real* Clippy (JSON) and count Clippy vs Rustc warnings.
- Compute unsafe rate on translated Rust sources.
- Reuse the *original OHOS gtest unit tests* to evaluate correctness (best-effort).

Typical usage:
  cd /data/home/wangshb/c2-rust_framework/scripts/analysis
  python3 analyze_c2r_compilation_rate_ohos10.py \\
    --run-dir /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos10 \\
    --all
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# =============================================================================
# Project order (OHOS 10)
# =============================================================================

DISPLAY_PROJECT_ORDER: List[str] = [
    "appverify_lite__e5ebe91a98b9",
    "common__89d5ecaafdff",
    "core__ef5242b7ab08",
    "host__25c1898e1626",
    "manager__c248934e0221",
    "osal__0bc4f21396ad",
    "posix__1b7f59c68bbc",
    "sapm__193cdeb43a97",
    "shared__12e38ea922f7",
    "shared__541f4e547bdb",
]
_ORDER_INDEX = {n: i for i, n in enumerate(DISPLAY_PROJECT_ORDER)}


def iter_projects_in_display_order(projects: Dict[str, Any]) -> List[Tuple[str, Any]]:
    return sorted(projects.items(), key=lambda kv: (_ORDER_INDEX.get(kv[0], 1_000_000), kv[0]))


# =============================================================================
# Paths
# =============================================================================

DEFAULT_HUAWEI_PROJECTS_TSV = Path(
    "/data/home/wangshb/c2-rust_framework/ComparisonMethod/unified/huawei_projects.tsv"
)
DEFAULT_OHOS_ROOT = Path(
    "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony"
)


def load_huawei_projects_map(tsv_path: Path) -> Dict[str, Path]:
    mapping: Dict[str, Path] = {}
    if not tsv_path.exists():
        return mapping
    for raw in tsv_path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        name = parts[0].strip()
        path = parts[1].strip()
        if name.lower() in ("name", "project", "project_name") and path.lower() in ("path", "dir", "directory"):
            continue
        if not name or not path:
            continue
        mapping[name] = Path(path).expanduser().resolve()
    return mapping


# =============================================================================
# Subprocess helpers
# =============================================================================

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


# =============================================================================
# Cargo helpers (no touching translation outputs)
# =============================================================================

def run_cargo_check(project_dir: Path, timeout: int = 300) -> Dict[str, Any]:
    res: Dict[str, Any] = {"executed": False, "passed": False, "error": None, "stdout": "", "stderr": ""}
    if not (project_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="our_ohos_cargo_target_") as td:
        env = {**os.environ, "CARGO_TARGET_DIR": td, "RUSTFLAGS": "-Awarnings", "RUST_BACKTRACE": "0"}
        rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["passed"] = rc == 0
        res["stdout"] = out[-8000:]
        res["stderr"] = err[-12000:]
        if rc != 0:
            res["error"] = "cargo check failed"
    return res


def run_cargo_clippy(project_dir: Path, timeout: int = 600) -> Dict[str, Any]:
    """
    Run `cargo clippy` with JSON diagnostics and count:
    - clippy warnings (code starts with "clippy::")
    - rustc warnings emitted during clippy run
    - errors
    """
    res: Dict[str, Any] = {
        "executed": False,
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "error_count": 0,
        "error": None,
        "stdout": "",
        "stderr": "",
    }
    if not (project_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="our_ohos_clippy_target_") as td:
        env = {**os.environ, "CARGO_TARGET_DIR": td, "RUST_BACKTRACE": "0"}
        cmd = ["cargo", "clippy", "--offline", "--message-format=json", "--", "-W", "clippy::all"]
        rc, out, err = _run_cmd_capture(cmd, cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["stdout"] = out[-200000:]
        res["stderr"] = err[-200000:]

        clippy_warn = 0
        rustc_warn = 0
        err_cnt = 0
        for line in out.splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
            except Exception:
                continue
            if obj.get("reason") != "compiler-message":
                continue
            msg = obj.get("message") or {}
            level = msg.get("level")
            code_obj = msg.get("code") or {}
            code = (code_obj.get("code") or "")
            if level == "warning":
                if code.startswith("clippy::"):
                    clippy_warn += 1
                else:
                    rustc_warn += 1
            elif level == "error":
                err_cnt += 1

        res["warning_count"] = clippy_warn
        res["rustc_warning_count"] = rustc_warn
        res["warning_count_total"] = clippy_warn + rustc_warn
        res["error_count"] = err_cnt
        if rc != 0 and err_cnt > 0:
            res["error"] = "cargo clippy failed"
    return res


# =============================================================================
# Build staticlib for C/C++ unit tests (work on a temp copy)
# =============================================================================

def _parse_crate_names(cargo_toml_text: str) -> Tuple[Optional[str], Optional[str]]:
    pkg_name = None
    lib_name = None
    in_pkg = False
    in_lib = False
    for line in cargo_toml_text.splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            sec = s.strip("[]").strip()
            in_pkg = sec == "package"
            in_lib = sec == "lib"
            continue
        m = re.match(r'name\s*=\s*"([^"]+)"', s)
        if m and in_pkg and pkg_name is None:
            pkg_name = m.group(1)
        if m and in_lib and lib_name is None:
            lib_name = m.group(1)
    return pkg_name, lib_name


def _strip_rust_comments(text: str) -> str:
    # Best-effort: sufficient for our generated code (avoid matching signatures inside saved LLM outputs).
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    return text


def _split_top_level_commas(text: str) -> List[str]:
    parts: List[str] = []
    buf: List[str] = []
    depth_paren = depth_angle = depth_brack = 0
    for ch in text:
        if ch == "(":
            depth_paren += 1
        elif ch == ")":
            depth_paren = max(depth_paren - 1, 0)
        elif ch == "<":
            depth_angle += 1
        elif ch == ">":
            depth_angle = max(depth_angle - 1, 0)
        elif ch == "[":
            depth_brack += 1
        elif ch == "]":
            depth_brack = max(depth_brack - 1, 0)

        if ch == "," and depth_paren == 0 and depth_angle == 0 and depth_brack == 0:
            part = "".join(buf).strip()
            if part:
                parts.append(part)
            buf = []
            continue
        buf.append(ch)
    tail = "".join(buf).strip()
    if tail:
        parts.append(tail)
    return parts


def _split_param_name_and_type(param: str) -> Optional[Tuple[str, str]]:
    depth_paren = depth_angle = depth_brack = 0
    for i, ch in enumerate(param):
        if ch == "(":
            depth_paren += 1
        elif ch == ")":
            depth_paren = max(depth_paren - 1, 0)
        elif ch == "<":
            depth_angle += 1
        elif ch == ">":
            depth_angle = max(depth_angle - 1, 0)
        elif ch == "[":
            depth_brack += 1
        elif ch == "]":
            depth_brack = max(depth_brack - 1, 0)

        if depth_paren == 0 and depth_angle == 0 and depth_brack == 0 and ch == ":":
            # Avoid `::` paths.
            prevc = param[i - 1] if i > 0 else ""
            nextc = param[i + 1] if i + 1 < len(param) else ""
            if prevc == ":" or nextc == ":":
                continue
            return param[:i].strip(), param[i + 1 :].strip()
    return None


def _extract_export_wrappers_from_file(rs_path: Path) -> List[Dict[str, Any]]:
    """
    Return a list of exportable functions from a generated module file:
      pub (unsafe)? extern "C" fn Name(args...) -> Ret
    """
    try:
        raw = rs_path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return []
    text = _strip_rust_comments(raw)
    out: List[Dict[str, Any]] = []

    # Match the start; then parse params with a small scanner (handles nested fn(...) pointers).
    start_re = re.compile(r'\bpub\s+(unsafe\s+)?extern\s+"C"\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(')
    for m in start_re.finditer(text):
        unsafe_kw = bool(m.group(1))
        fn_name = m.group(2)
        # Find the parameter list boundaries.
        open_paren = m.end() - 1
        i = open_paren
        depth = 0
        close_paren = None
        while i < len(text):
            ch = text[i]
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth == 0:
                    close_paren = i
                    break
            i += 1
        if close_paren is None:
            continue
        params_str = text[open_paren + 1 : close_paren].strip()

        # Return type: `-> ... {`
        j = close_paren + 1
        while j < len(text) and text[j].isspace():
            j += 1
        ret_str = ""
        if text[j : j + 2] == "->":
            j += 2
            while j < len(text) and text[j].isspace():
                j += 1
            k = j
            while k < len(text) and text[k] != "{":
                k += 1
            ret_str = text[j:k].strip()

        # Params: rename `_` to `argN` so wrapper body can forward.
        params_out: List[str] = []
        arg_names: List[str] = []
        if params_str:
            for idx, p in enumerate(_split_top_level_commas(params_str)):
                if not p:
                    continue
                split = _split_param_name_and_type(p)
                if not split:
                    # Unhandled (e.g., varargs). Skip exporting this fn.
                    params_out = []
                    arg_names = []
                    break
                name_part, type_part = split
                toks = [t for t in name_part.split() if t]
                is_mut = False
                if toks and toks[0] == "mut":
                    is_mut = True
                    toks = toks[1:]
                ident = toks[-1] if toks else "_"
                if ident == "_" or not re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", ident):
                    ident = f"arg{idx}"
                arg_names.append(ident)
                params_out.append(f"{'mut ' if is_mut else ''}{ident}: {type_part}")
        if params_str and not params_out:
            continue

        out.append(
            {
                "name": fn_name,
                "unsafe": unsafe_kw,
                "params": params_out,
                "args": arg_names,
                "ret": ret_str,
            }
        )
    return out


def _extract_public_fns_from_file(rs_path: Path, *, allow_private: bool) -> List[Dict[str, Any]]:
    """
    Return a list of *callable* functions for derived-test generation.

    Supported patterns (best-effort):
      - pub fn Name(args...) -> Ret
      - pub(crate) fn Name(...)
      - pub unsafe fn Name(...)
      - pub extern "C" fn Name(...)
      - (optionally) private `fn Name(...)` when allow_private=True (used for single-file outputs like simcrat)
    """
    try:
        raw = rs_path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return []

    text = _strip_rust_comments(raw)
    out: List[Dict[str, Any]] = []

    # Group 1: optional `pub` / `pub(crate)`; Group 2: optional `unsafe`; Group 3: fn name
    start_re = re.compile(
        r'\b(pub(?:\s*\(\s*crate\s*\))?\s+)?(unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\('
    )
    for m in start_re.finditer(text):
        vis_kw = (m.group(1) or "").strip()
        unsafe_kw = bool(m.group(2))
        fn_name = m.group(3)

        is_private = not vis_kw
        if is_private and not allow_private:
            continue

        # Find the parameter list boundaries.
        open_paren = m.end() - 1
        i = open_paren
        depth = 0
        close_paren = None
        while i < len(text):
            ch = text[i]
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth == 0:
                    close_paren = i
                    break
            i += 1
        if close_paren is None:
            continue
        params_str = text[open_paren + 1 : close_paren].strip()

        # Return type: `-> ... {`
        j = close_paren + 1
        while j < len(text) and text[j].isspace():
            j += 1
        ret_str = ""
        if text[j : j + 2] == "->":
            j += 2
            while j < len(text) and text[j].isspace():
                j += 1
            k = j
            while k < len(text) and text[k] != "{":
                k += 1
            ret_str = text[j:k].strip()

        # Params: rename `_` to `argN` so we can call safely from generated tests.
        params_out: List[str] = []
        arg_names: List[str] = []
        if params_str:
            for idx, p in enumerate(_split_top_level_commas(params_str)):
                if not p:
                    continue
                split = _split_param_name_and_type(p)
                if not split:
                    params_out = []
                    arg_names = []
                    break
                name_part, type_part = split
                toks = [t for t in name_part.split() if t]
                is_mut = False
                if toks and toks[0] == "mut":
                    is_mut = True
                    toks = toks[1:]
                ident = toks[-1] if toks else "_"
                if ident == "_" or not re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", ident):
                    ident = f"arg{idx}"
                arg_names.append(ident)
                params_out.append(f"{'mut ' if is_mut else ''}{ident}: {type_part}")
        if params_str and not params_out:
            continue

        out.append(
            {
                "name": fn_name,
                "unsafe": unsafe_kw,
                "params": params_out,
                "args": arg_names,
                "ret": ret_str,
                "visibility": ("private" if is_private else ("pub_crate" if "pub(" in vis_kw else "pub")),
            }
        )

    return out


def build_staticlib_from_crate(crate_dir: Path, *, export_names: Optional[set[str]] = None, timeout: int = 600) -> Dict[str, Any]:
    """
    Build a Rust staticlib from a Cargo crate directory (without touching the original crate on disk).

    Our framework's OHOS outputs are commonly *binary* crates (only `src/main.rs`).
    For unit tests we need a linkable `staticlib`, so here we:
      - copy the crate to a temp dir
      - add a `[lib]` target (path = src/main.rs if src/lib.rs doesn't exist)
      - set crate-type = ["staticlib"]
      - build with `cargo build --release --lib`
    """
    res: Dict[str, Any] = {
        "executed": False,
        "ok": False,
        "staticlib": None,
        "error": None,
        "stdout": "",
        "stderr": "",
        "crate_dir": str(crate_dir),
    }
    cargo_toml = crate_dir / "Cargo.toml"
    if not cargo_toml.is_file():
        res["error"] = f"Cargo.toml not found: {cargo_toml}"
        return res

    try:
        with tempfile.TemporaryDirectory(prefix="our_ohos_staticlib_") as td:
            td_path = Path(td)
            tmp_crate = td_path / "crate"
            shutil.copytree(crate_dir, tmp_crate, ignore=shutil.ignore_patterns("target", ".git"))

            txt = (tmp_crate / "Cargo.toml").read_text(encoding="utf-8", errors="ignore")
            pkg_name, lib_name = _parse_crate_names(txt)
            name = lib_name or pkg_name
            if not name:
                res["error"] = "Failed to parse crate name from Cargo.toml"
                return res
            libfile = f"lib{name.replace('-', '_')}.a"

            # Patch Cargo.toml to ensure a staticlib library target exists.
            has_lib_section = "[lib]" in txt
            src_dir = tmp_crate / "src"
            lib_rs = src_dir / "lib.rs"
            main_rs = src_dir / "main.rs"

            if has_lib_section:
                if "crate-type" not in txt:
                    txt += "\n\n[lib]\ncrate-type = [\"staticlib\"]\n"
            else:
                if lib_rs.is_file():
                    txt += "\n\n[lib]\ncrate-type = [\"staticlib\"]\n"
                elif main_rs.is_file():
                    # Use main.rs as the library root (its `fn main()` is a normal mangled Rust fn in a lib target).
                    txt += "\n\n[lib]\npath = \"src/main.rs\"\ncrate-type = [\"staticlib\"]\n"
                else:
                    res["error"] = "No src/lib.rs or src/main.rs found to build staticlib"
                    return res

            # Generate `#[no_mangle]` export wrappers so C/C++ unit tests can link against this staticlib.
            # We do it on the temp copy only, and forward to the real translated functions.
            export_mod_rs = src_dir / "c2r_exports.rs"
            exports: List[str] = []
            for mod_file in sorted(src_dir.glob("src_*.rs")):
                # Skip generated fallback internals; we only want the top-level translated modules.
                if "__c2r_generated" in str(mod_file):
                    continue
                module = mod_file.stem
                for fninfo in _extract_export_wrappers_from_file(mod_file):
                    fn_name = fninfo["name"]
                    if export_names is not None and fn_name not in export_names:
                        continue
                    unsafe_kw = "unsafe " if fninfo["unsafe"] else ""
                    params = ", ".join(fninfo["params"])
                    args = ", ".join(fninfo["args"])
                    ret = fninfo["ret"]
                    ret_part = f" -> {ret}" if ret else ""
                    call = f"crate::{module}::{fn_name}({args})"
                    if ret:
                        body = f"    {call}\n"
                    else:
                        body = f"    {call};\n"
                    exports.append(
                        "\n".join(
                            [
                                "#[no_mangle]",
                                f'pub {unsafe_kw}extern "C" fn {fn_name}({params}){ret_part} {{',
                                body.rstrip("\n"),
                                "}",
                            ]
                        )
                    )
            export_mod_rs.write_text(
                "\n".join(
                    [
                        "// Auto-generated export shims for C/C++ unit tests (temp build only).",
                        "#![allow(non_snake_case)]",
                        "#![allow(dead_code)]",
                        "#![allow(unused_variables)]",
                        "#![allow(unused_imports)]",
                        "use crate::compat::*;",
                        "use crate::compatibility::*;",
                        "use crate::globals::*;",
                        "use crate::types::*;",
                        "",
                        *exports,
                        "",
                    ]
                ),
                encoding="utf-8",
            )

            # Ensure the export module is compiled into the crate.
            root_rs = lib_rs if lib_rs.is_file() else main_rs
            if root_rs.is_file():
                root_txt = root_rs.read_text(encoding="utf-8", errors="ignore")
                if "pub mod c2r_exports;" not in root_txt and "mod c2r_exports;" not in root_txt:
                    root_rs.write_text(root_txt + "\n\npub mod c2r_exports;\n", encoding="utf-8")

            (tmp_crate / "Cargo.toml").write_text(txt, encoding="utf-8")

            with tempfile.TemporaryDirectory(prefix="our_ohos_staticlib_target_") as tgt:
                # Keep consistent with our pipeline: allow lints so we can build & run tests on the produced artifacts.
                debug_build = os.environ.get("OHOS_STATICLIB_DEBUG", "").strip() in {"1", "true", "TRUE", "yes", "YES"}
                profile_dir = "debug" if debug_build else "release"
                env = {
                    **os.environ,
                    "CARGO_TARGET_DIR": tgt,
                    # Some generated OHOS crates have codegen-only overflow lints (e.g., shift by >= width).
                    # Allow them so we can build the staticlib for unit test linkage.
                    # For debugging segfaults, allow switching to an unoptimized build with frame pointers.
                    "RUSTFLAGS": (
                        "-Awarnings -A arithmetic_overflow"
                        + (" -C debuginfo=2 -C force-frame-pointers=yes -C opt-level=0" if debug_build else "")
                    ),
                    "RUST_BACKTRACE": "0",
                }
                rc, out, err = _run_cmd_capture(
                    ["cargo", "build", "--offline", *( [] if debug_build else ["--release"] ), "--lib"],
                    cwd=tmp_crate,
                    env=env,
                    timeout=timeout,
                )
                res["executed"] = True
                res["stdout"] = out[-8000:]
                res["stderr"] = err[-12000:]
                if rc != 0:
                    res["error"] = "cargo build staticlib failed"
                    return res

                built = Path(tgt) / profile_dir / libfile
                if not built.is_file():
                    res["error"] = f"staticlib not found after build: {built}"
                    return res

                out_dir = Path(tempfile.mkdtemp(prefix="our_ohos_staticlib_out_"))
                out_lib = out_dir / built.name
                shutil.copy(built, out_lib)
                res["staticlib"] = str(out_lib)
                res["ok"] = True
                return res
    except Exception as e:
        res["error"] = str(e)
        return res


# =============================================================================
# OHOS unit tests (gtest) - aligned with simcrat OHOS harness
# =============================================================================

def _default_ohos_test_include_dirs(ohos_root: Path) -> List[Path]:
    # Keep this list aligned with the simcrat OHOS harness (practical + minimal).
    dirs: List[Path] = []

    # HDF public/inner headers
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/utils")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/core")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/uhdf")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host/uhdf")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/ipc")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/hdi")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/host/shared")
    dirs.append(ohos_root / "drivers/hdf_core/interfaces/inner_api/osal/shared")

    # Framework internal headers that unit tests often include directly.
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/shared/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/manager/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/core/host/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/utils/include")

    # uhdf2 adapter headers used by manager tests
    dirs.append(ohos_root / "drivers/hdf_core/adapter/uhdf2/ipc/include")

    # HDF unit test headers (e.g., hdf_uhdf_test.h)
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/include")
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/pm")
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/manager")
    dirs.append(ohos_root / "drivers/hdf_core/framework/test/unittest/osal")

    # Audio model unit test headers (sapm tests depend on these).
    dirs.append(ohos_root / "drivers/hdf_core/framework/model/audio/common/test/unittest/common")
    dirs.append(ohos_root / "drivers/hdf_core/framework/model/audio/core/test/unittest/common")

    # IPC/SAMGR headers pulled by some manager tests.
    dirs.append(ohos_root / "foundation/communication/ipc/interfaces/innerkits/ipc_core/include")
    dirs.append(ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/samgr_proxy/include")
    dirs.append(ohos_root / "foundation/systemabilitymgr/samgr/interfaces/innerkits/dynamic_cache/include")
    dirs.append(ohos_root / "base/notification/eventhandler/interfaces/inner_api")
    dirs.append(ohos_root / "base/notification/eventhandler/frameworks/eventhandler/include")
    dirs.append(ohos_root / "base/hiviewdfx/hisysevent/interfaces/native/innerkits/hisysevent/include")

    # securec
    dirs.append(ohos_root / "third_party/bounds_checking_function/include")

    # mbedtls (appverify_lite unit tests include mbedtls headers)
    dirs.append(ohos_root / "third_party/mbedtls/include")
    # Some mbedtls sources (e.g., net_sockets.c) include "socket_compat.h".
    # On a normal Linux host we use the posix compat layer from the OHOS tree.
    dirs.append(ohos_root / "third_party/mbedtls/port/config/compat_posix")

    # googletest
    dirs.append(ohos_root / "third_party/googletest/googletest/include")
    dirs.append(ohos_root / "third_party/googletest/googlemock/include")
    dirs.append(ohos_root / "third_party/googletest/googletest")  # for gtest-all.cc includes "src/..."

    # Extra includes sometimes needed by tests.
    dirs.append(ohos_root / "base/hiviewdfx/hilog/interfaces/native/innerkits/include")
    dirs.append(ohos_root / "commonlibrary/c_utils/base/include")

    return [d for d in dirs if d.is_dir()]


def _parse_gtest_counts(output: str) -> Tuple[int, int]:
    passed = 0
    failed = 0
    m = re.search(r"\[\s*PASSED\s*\]\s+(\d+)\s+tests?\b", output)
    if m:
        passed = int(m.group(1))
    m = re.search(r"\[\s*FAILED\s*\]\s+(\d+)\s+tests?\b", output)
    if m:
        failed = int(m.group(1))
    return passed, failed


def _collect_ohos_test_sources(source_project_dir: Path) -> Tuple[List[Path], List[Path], Optional[str]]:
    """
    Returns: (test_srcs, sample_srcs, error)
    """
    test_roots: List[Path] = []
    for rel in ("test", "unittest"):
        d = source_project_dir / rel
        if d.is_dir():
            test_roots.append(d)
    if not test_roots:
        return [], [], "no test/ or unittest/ directory"

    test_srcs: List[Path] = []
    sample_srcs: List[Path] = []
    for root in test_roots:
        for p in root.rglob("*"):
            if not p.is_file():
                continue
            if p.suffix.lower() not in (".c", ".cc", ".cpp", ".cxx"):
                continue
            parts = [x.lower() for x in p.parts]
            if "sample" in parts:
                sample_srcs.append(p)
            else:
                test_srcs.append(p)
    test_srcs.sort()
    sample_srcs.sort()
    if not test_srcs:
        return [], [], "no C/C++ test sources found"
    return test_srcs, sample_srcs, None


_C_CALL_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(")
_C_CALL_EXCLUDE = {
    # C/C++ keywords / builtins
    "if",
    "for",
    "while",
    "switch",
    "return",
    "sizeof",
    "static_cast",
    "reinterpret_cast",
    "const_cast",
    # gtest / hwtest macros
    "TEST",
    "TEST_F",
    "HWTEST_F",
    "ASSERT_TRUE",
    "ASSERT_FALSE",
    "ASSERT_EQ",
    "ASSERT_NE",
    "EXPECT_TRUE",
    "EXPECT_FALSE",
    "EXPECT_EQ",
    "EXPECT_NE",
}


def _collect_called_identifiers(test_srcs: List[Path]) -> set[str]:
    names: set[str] = set()
    for p in test_srcs:
        try:
            txt = p.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        for m in _C_CALL_RE.finditer(txt):
            n = m.group(1)
            if n in _C_CALL_EXCLUDE:
                continue
            names.add(n)
    return names


def _collect_exportable_fn_names(crate_dir: Path) -> set[str]:
    src_dir = crate_dir / "src"
    names: set[str] = set()
    for mod_file in sorted(src_dir.glob("src_*.rs")):
        if "__c2r_generated" in str(mod_file):
            continue
        for fninfo in _extract_export_wrappers_from_file(mod_file):
            names.add(str(fninfo.get("name") or ""))
    names.discard("")
    return names


def _canonicalize_symbol_name(name: str) -> str:
    """Normalize symbol names for fuzzy matching (e.g., HdfLoadVdi <-> hdf_load_vdi)."""
    return "".join(ch.lower() for ch in str(name) if ch.isalnum())


def _parse_lib_path_from_cargo_toml(cargo_toml: Path) -> Optional[str]:
    """
    Best-effort parse `[lib] path = "..."` from Cargo.toml.

    We keep this lightweight (no toml dependency) and only parse what we need.
    """
    try:
        txt = cargo_toml.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return None
    in_lib = False
    for raw in txt.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("[") and line.endswith("]"):
            in_lib = line.strip("[]").strip() == "lib"
            continue
        if not in_lib:
            continue
        m = re.match(r'path\s*=\s*"(.*)"\s*$', line)
        if m:
            return m.group(1).strip()
    return None


def _find_crate_root_rs_for_tests(crate_dir: Path) -> Optional[Path]:
    """
    Find the Rust crate root file (lib.rs/main.rs/custom lib path) for inserting unit tests.
    """
    cargo_toml = crate_dir / "Cargo.toml"
    if cargo_toml.is_file():
        lib_path = _parse_lib_path_from_cargo_toml(cargo_toml)
        if lib_path:
            p = (crate_dir / lib_path).resolve()
            if p.is_file():
                return p
    # Conventional locations
    if (crate_dir / "src" / "lib.rs").is_file():
        return (crate_dir / "src" / "lib.rs").resolve()
    if (crate_dir / "src" / "main.rs").is_file():
        return (crate_dir / "src" / "main.rs").resolve()
    return None


def _module_path_from_rs_file(crate_dir: Path, rs_path: Path, *, root_rs: Optional[Path]) -> Optional[str]:
    """
    Compute a Rust module path (relative to crate root) for a source file.

    Notes:
    - For the crate root file (lib.rs/main.rs/custom lib path), module path is "".
    - For files under `<crate>/src`, module path follows the usual folder -> module mapping.
    - For `.rs` files at crate root (e.g., c2rust's `<mod>.rs`), module path is the file stem.
    """
    try:
        p = rs_path.resolve()
    except Exception:
        p = rs_path

    if root_rs is not None:
        try:
            if p == root_rs.resolve():
                return ""
        except Exception:
            if p == root_rs:
                return ""

    src_dir = crate_dir / "src"
    try:
        if src_dir.is_dir() and p.is_relative_to(src_dir.resolve()):  # py>=3.9
            rel = p.relative_to(src_dir.resolve())
            if rel.name == "mod.rs":
                parts = rel.parts[:-1]
            else:
                parts = rel.with_suffix("").parts
            return "::".join(parts)
    except Exception:
        # Fallback for py<3.9 or path issues.
        try:
            if src_dir.is_dir() and str(p).startswith(str(src_dir.resolve()) + os.sep):
                rel = Path(str(p)[len(str(src_dir.resolve())) + 1 :])
                if rel.name == "mod.rs":
                    parts = rel.parts[:-1]
                else:
                    parts = rel.with_suffix("").parts
                return "::".join(parts)
        except Exception:
            pass

    # Root-level modules (e.g., c2rust: hdf_device_info.rs)
    if rs_path.parent.resolve() == crate_dir.resolve():
        # Do not treat build scripts as modules.
        if rs_path.name == "build.rs":
            return None
        stem = rs_path.stem.replace("-", "_")
        # Skip empty stems.
        return stem or None

    return None


def _collect_exportable_fns_with_modules(crate_dir: Path) -> Dict[str, List[Dict[str, Any]]]:
    """
    Return mapping: fn_name -> list of {module, unsafe, params, args, ret, visibility}.

    Despite the legacy name, this is used by *derived Rust tests* and therefore supports
    multiple layouts / method outputs:
    - our framework: `<crate>/src/src_*.rs` (module "src_*")
    - Evo/C2SaferRust: `<crate>/src/src/*.rs` (module "src::<name>")
    - c2rust: `<crate>/*.rs` modules with `[lib] path = "c2rust-lib.rs"`
    - simcrat (temp crates): functions defined in crate root (module "")
    """
    out: Dict[str, List[Dict[str, Any]]] = {}
    root_rs = _find_crate_root_rs_for_tests(crate_dir)

    rs_files: List[Path] = []
    if root_rs and root_rs.is_file():
        rs_files.append(root_rs)

    src_dir = crate_dir / "src"
    if src_dir.is_dir():
        for p in src_dir.rglob("*.rs"):
            # Avoid scanning build artifacts and internal generated fallbacks.
            if "target" in p.parts:
                continue
            if "__c2r_generated" in str(p):
                continue
            rs_files.append(p)

    # Root-level module files (needed for c2rust outputs).
    for p in crate_dir.glob("*.rs"):
        rs_files.append(p)

    # Deterministic order.
    uniq_files: List[Path] = sorted({p.resolve() for p in rs_files if p.is_file()})

    for mod_file in uniq_files:
        module = _module_path_from_rs_file(crate_dir, mod_file, root_rs=root_rs)
        if module is None:
            continue
        allow_private = (module == "") and (root_rs is not None) and (mod_file.resolve() == root_rs.resolve())

        for fninfo in _extract_public_fns_from_file(mod_file, allow_private=allow_private):
            fn_name = str(fninfo.get("name") or "")
            if not fn_name:
                continue
            item = dict(fninfo)
            item["module"] = module
            out.setdefault(fn_name, []).append(item)
    return out


def _looks_like_null_token(tok: str) -> bool:
    return tok in ("nullptr", "NULL")


def _split_c_args(args: str) -> List[str]:
    # Best-effort split; good enough for our "pure logic" subset.
    parts = []
    cur = []
    depth_paren = depth_angle = depth_brack = 0
    for ch in args:
        if ch == "(":
            depth_paren += 1
        elif ch == ")":
            depth_paren = max(depth_paren - 1, 0)
        elif ch == "<":
            depth_angle += 1
        elif ch == ">":
            depth_angle = max(depth_angle - 1, 0)
        elif ch == "[":
            depth_brack += 1
        elif ch == "]":
            depth_brack = max(depth_brack - 1, 0)
        if ch == "," and depth_paren == 0 and depth_angle == 0 and depth_brack == 0:
            parts.append("".join(cur).strip())
            cur = []
            continue
        cur.append(ch)
    tail = "".join(cur).strip()
    if tail:
        parts.append(tail)
    return [p for p in parts if p]


_PURE_EXPECT_TOKENS = {
    "nullptr",
    "NULL",
    "false",
    "true",
    "0",
    "-1",
    "HDF_ERR_INVALID_PARAM",
    "HDF_FAILURE",
    "HDF_SUCCESS",
}


def _extract_gtest_blocks(text: str) -> List[Tuple[str, str]]:
    """
    Return list of (test_name, body_text) for HWTEST_F blocks (best-effort).
    """
    blocks: List[Tuple[str, str]] = []
    pat = re.compile(r"HWTEST_F\s*\(\s*[A-Za-z_][A-Za-z0-9_]*\s*,\s*([A-Za-z_][A-Za-z0-9_]*)\s*,[^)]*\)\s*\{")
    for m in pat.finditer(text):
        test_name = m.group(1)
        i = m.end()  # position after '{'
        depth = 1
        j = i
        while j < len(text):
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    body = text[i:j]
                    blocks.append((test_name, body))
                    break
            j += 1
    return blocks


def _extract_pure_logic_cases_from_gtest(test_srcs: List[Path]) -> List[Dict[str, Any]]:
    """
    Extract a conservative subset of gtest assertions that can be reasonably ported to Rust unit tests:
    - nullptr/NULL arguments only (plus -1/0 where present)
    - expected tokens: nullptr/NULL/true/false/0/-1/HDF_ERR_INVALID_PARAM/HDF_FAILURE/HDF_SUCCESS
    """
    cases: List[Dict[str, Any]] = []
    for src in test_srcs:
        try:
            raw = src.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue

        for test_name, body in _extract_gtest_blocks(raw):
            # Collect simple assignments: var = Fn(args);
            assigns: Dict[str, Tuple[str, str]] = {}
            for am in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(([^;]*)\)\s*;", body):
                var = am.group(1)
                fn = am.group(2)
                args = am.group(3).strip()
                assigns[var] = (fn, args)

            # ASSERT_FALSE(var)
            for m in re.finditer(r"\bASSERT_FALSE\s*\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*;", body):
                var = m.group(1)
                if var not in assigns:
                    continue
                fn, args = assigns[var]
                arg_toks = _split_c_args(args)
                if not arg_toks or not all(t in ("nullptr", "NULL") for t in arg_toks):
                    continue
                cases.append(
                    {
                        "origin": "gtest",
                        "test_name": test_name,
                        "source": str(src),
                        "fn": fn,
                        "args": arg_toks,
                        "expect_kind": "bool",
                        "expect": "false",
                    }
                )

            # ASSERT_EQ(expected, var) where var assigned from Fn(...)
            for m in re.finditer(
                r"\bASSERT_EQ\s*\(\s*([A-Za-z_][A-Za-z0-9_]*|nullptr|NULL|-?\d+)\s*,\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*;",
                body,
            ):
                expected = m.group(1)
                var = m.group(2)
                if expected not in _PURE_EXPECT_TOKENS:
                    continue
                if var not in assigns:
                    continue
                fn, args = assigns[var]
                arg_toks = _split_c_args(args)
                if not arg_toks or not all(t in ("nullptr", "NULL", "0", "-1") for t in arg_toks):
                    continue
                cases.append(
                    {
                        "origin": "gtest",
                        "test_name": test_name,
                        "source": str(src),
                        "fn": fn,
                        "args": arg_toks,
                        "expect_kind": "eq",
                        "expect": expected,
                    }
                )

            # ASSERT_EQ(nullptr, Fn(args)) direct form
            for m in re.finditer(
                r"\bASSERT_EQ\s*\(\s*(nullptr|NULL|-?\d+|[A-Za-z_][A-Za-z0-9_]*)\s*,\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*([^\)]*)\)\s*\)\s*;",
                body,
            ):
                expected = m.group(1).strip()
                fn = m.group(2).strip()
                args = m.group(3).strip()
                if expected not in _PURE_EXPECT_TOKENS:
                    continue
                arg_toks = _split_c_args(args)
                if not arg_toks or not all(t in ("nullptr", "NULL", "0", "-1") for t in arg_toks):
                    continue
                cases.append(
                    {
                        "origin": "gtest",
                        "test_name": test_name,
                        "source": str(src),
                        "fn": fn,
                        "args": arg_toks,
                        "expect_kind": "eq",
                        "expect": expected,
                    }
                )

            # Bare call with nullptr (ensure no crash): Fn(nullptr,...);
            for m in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*([^\)]*)\)\s*;", body):
                fn = m.group(1).strip()
                args = m.group(2).strip()
                if fn in ("ASSERT_TRUE", "ASSERT_FALSE", "ASSERT_EQ", "EXPECT_TRUE", "EXPECT_FALSE", "EXPECT_EQ"):
                    continue
                arg_toks = _split_c_args(args)
                if not arg_toks or not all(t in ("nullptr", "NULL") for t in arg_toks):
                    continue
                cases.append(
                    {
                        "origin": "gtest",
                        "test_name": test_name,
                        "source": str(src),
                        "fn": fn,
                        "args": arg_toks,
                        "expect_kind": "no_panic",
                        "expect": None,
                    }
                )

    # De-dup (source + test_name + fn + args + expect)
    uniq: Dict[str, Dict[str, Any]] = {}
    for c in cases:
        key = f"{c.get('source')}::{c.get('test_name')}::{c.get('fn')}::{','.join(c.get('args') or [])}::{c.get('expect_kind')}::{c.get('expect')}"
        uniq[key] = c
    return list(uniq.values())


def _patch_cargo_toml_for_tests(cargo_toml: Path) -> None:
    txt = cargo_toml.read_text(encoding="utf-8", errors="ignore")

    # Ensure [lib] exists for binary crates.
    has_lib = "[lib]" in txt
    if not has_lib:
        src_dir = cargo_toml.parent / "src"
        lib_rs = src_dir / "lib.rs"
        main_rs = src_dir / "main.rs"
        if lib_rs.is_file():
            txt += "\n\n[lib]\n"
        elif main_rs.is_file():
            txt += "\n\n[lib]\npath = \"src/main.rs\"\n"

    # Ensure build script is enabled.
    # Insert `build = "build.rs"` under [package] if absent.
    lines = txt.splitlines()
    out_lines: List[str] = []
    in_pkg = False
    inserted = False
    for i, line in enumerate(lines):
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            if in_pkg and not inserted:
                out_lines.append('build = "build.rs"')
                inserted = True
            sec = s.strip("[]").strip()
            in_pkg = sec == "package"
        if in_pkg and s.startswith("build") and "build.rs" in s:
            inserted = True
        out_lines.append(line)
    if in_pkg and not inserted:
        out_lines.append('build = "build.rs"')
        inserted = True
    txt2 = "\n".join(out_lines) + "\n"
    cargo_toml.write_text(txt2, encoding="utf-8")


def run_derived_rust_tests_from_gtest(
    *,
    project_name: str,
    crate_dir: Path,
    cases: List[Dict[str, Any]],
    ohos_root: Path,
    timeout: int = 600,
) -> Dict[str, Any]:
    """
    Create a temp copy of the translated crate, generate Rust unit tests for a small
    pure-logic subset of gtest, and run `cargo test`.

    This does NOT modify translation outputs on disk.
    """
    res: Dict[str, Any] = {
        "executed": False,
        "generated_tests": 0,
        "passed": 0,
        "failed": 0,
        "error": None,
        "stdout": "",
        "stderr": "",
        "cases": cases,
    }
    if not cases:
        res["error"] = "no derived cases"
        return res
    if not (crate_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    # Map function names to Rust modules/signatures.
    # NOTE: some methods rename symbols (e.g., camelCase <-> snake_case). We therefore
    # allow a conservative fuzzy match via canonicalization.
    fn_map = _collect_exportable_fns_with_modules(crate_dir)
    canon_map: Dict[str, List[Dict[str, Any]]] = {}
    for n, items in fn_map.items():
        canon_map.setdefault(_canonicalize_symbol_name(n), []).extend(items)

    usable: List[Dict[str, Any]] = []
    for c in cases:
        c_fn = str(c.get("fn") or "")
        if not c_fn:
            continue
        candidates = fn_map.get(c_fn)
        if not candidates:
            candidates = canon_map.get(_canonicalize_symbol_name(c_fn))
        if not candidates:
            continue
        c2 = dict(c)
        c2["rust_candidates"] = candidates
        c2["rust"] = candidates[0]  # deterministic
        usable.append(c2)

    if not usable:
        res["error"] = "no cases matched translated Rust functions"
        return res

    try:
        with tempfile.TemporaryDirectory(prefix=f"our_ohos10_derived_{project_name}_") as td:
            td_path = Path(td)
            tmp_crate = td_path / "crate"
            shutil.copytree(crate_dir, tmp_crate, ignore=shutil.ignore_patterns("target", ".git"))

            cargo_toml = tmp_crate / "Cargo.toml"
            _patch_cargo_toml_for_tests(cargo_toml)

            # Build script: compile C shims + a minimal subset of OHOS C support code so the translated Rust
            # crate can link on a normal Linux host (best-effort).
            #
            # IMPORTANT: Some baselines (e.g., c2rust) ship their own build.rs. Do not overwrite it.
            build_rs = tmp_crate / "build.rs"
            if not build_rs.is_file():
                build_rs.write_text(
                "\n".join(
                    [
                        "use std::path::PathBuf;",
                        "use std::process::Command;",
                        "use std::fs;",
                        "",
                        "fn run(cmd: &mut Command) {",
                        "  let status = cmd.status().expect(\"spawn failed\");",
                        "  if !status.success() {",
                        "    panic!(\"command failed\");",
                        "  }",
                        "}",
                        "",
                        "fn main() {",
                        "  let out_dir = PathBuf::from(std::env::var(\"OUT_DIR\").unwrap());",
                        "  let ohos_root = match std::env::var(\"OHOS_ROOT\") {",
                        "    Ok(v) => PathBuf::from(v),",
                        "    Err(_) => PathBuf::new(),",
                        "  };",
                        "  let c_path = out_dir.join(\"c2r_ohos_shims.c\");",
                        "  fs::write(&c_path, r#\"",
                        "#include <stdarg.h>",
                        "#include <stddef.h>",
                        "#include <stdint.h>",
                        "#include <stdio.h>",
                        "#include <stdlib.h>",
                        "",
                        "__attribute__((weak)) int HiLogPrint(int type, int level, unsigned int domain, const char* tag, const char* fmt, ...) {",
                        "  (void)type; (void)level; (void)domain; (void)tag; (void)fmt;",
                        "  return 0;",
                        "}",
                        "",
                        "__attribute__((weak)) void* OsalMemCalloc(size_t size) { return calloc(1, size); }",
                        "__attribute__((weak)) void* OsalMemAlloc(size_t size) { return malloc(size); }",
                        "__attribute__((weak)) void* OsalMemRealloc(void* p, size_t size) { return realloc(p, size); }",
                        "__attribute__((weak)) void OsalMemFree(void* p) { free(p); }",
                        "",
                        "struct HdfObjectCreator; // opaque",
                        "__attribute__((weak)) const struct HdfObjectCreator *HdfObjectManagerGetCreators(int objectId) { (void)objectId; return NULL; }",
                        "\"#).unwrap();",
                        "",
                        "  let o_path = out_dir.join(\"c2r_ohos_shims.o\");",
                        "  run(Command::new(\"gcc\").args([\"-O2\", \"-c\", c_path.to_str().unwrap(), \"-o\", o_path.to_str().unwrap()]));",
                        "",
                        "  // Build securec (snprintf_s, memcpy_s, ...).",
                        "  let mut objs: Vec<PathBuf> = vec![o_path.clone()];",
                        "  if !ohos_root.as_os_str().is_empty() {",
                        "    let securec_src = ohos_root.join(\"third_party/bounds_checking_function/src\");",
                        "    let securec_inc = ohos_root.join(\"third_party/bounds_checking_function/include\");",
                        "    if securec_src.is_dir() {",
                        "      for ent in fs::read_dir(&securec_src).unwrap() {",
                        "        let p = ent.unwrap().path();",
                        "        if p.extension().and_then(|s| s.to_str()) != Some(\"c\") { continue; }",
                        "        let obj = out_dir.join(format!(\"securec_{}.o\", p.file_stem().unwrap().to_string_lossy()));",
                        "        let mut cmd = Command::new(\"gcc\");",
                        "        cmd.args([\"-O2\", \"-c\"]);",
                        "        cmd.arg(format!(\"-I{}\", securec_inc.display()));",
                        "        cmd.arg(format!(\"-I{}\", securec_src.display()));",
                        "        cmd.arg(p.to_str().unwrap());",
                        "        cmd.args([\"-o\", obj.to_str().unwrap()]);",
                        "        run(&mut cmd);",
                        "        objs.push(obj);",
                        "      }",
                        "    }",
                        "  }",
                        "",
                        "  // Build HDF sbuf RAW impl for host linkage (provides HdfSbufReadUint16, etc).",
                        "  if !ohos_root.as_os_str().is_empty() {",
                        "    let utils_src = ohos_root.join(\"drivers/hdf_core/framework/utils/src\");",
                        "    let files = [utils_src.join(\"hdf_sbuf_impl_raw.c\"), utils_src.join(\"hdf_sbuf.c\")];",
                        "    let incs = [",
                        "      ohos_root.join(\"drivers/hdf_core/framework/utils/include\"),",
                        "      ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/utils\"),",
                        "      ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/osal/shared\"),",
                        "      ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/osal/uhdf\"),",
                        "      ohos_root.join(\"base/hiviewdfx/hilog/interfaces/native/innerkits/include\"),",
                        "      ohos_root.join(\"commonlibrary/c_utils/base/include\"),",
                        "      ohos_root.join(\"third_party/bounds_checking_function/include\"),",
                        "    ];",
                        "    for p in files.iter() {",
                        "      if !p.is_file() { continue; }",
                        "      let obj = out_dir.join(format!(\"hdf_{}.o\", p.file_stem().unwrap().to_string_lossy()));",
                        "      let mut cmd = Command::new(\"gcc\");",
                        "      cmd.args([\"-O2\", \"-c\"]);",
                        "      for inc in incs.iter() { if inc.is_dir() { cmd.arg(format!(\"-I{}\", inc.display())); } }",
                        "      cmd.arg(p.to_str().unwrap());",
                        "      cmd.args([\"-o\", obj.to_str().unwrap()]);",
                        "      run(&mut cmd);",
                        "      objs.push(obj);",
                        "    }",
                        "  }",
                        "",
                        "  // Build HDF device info if present (provides HdfDeviceInfoNewInstance for some modules).",
                        "  if !ohos_root.as_os_str().is_empty() {",
                        "    let p = ohos_root.join(\"drivers/hdf_core/framework/core/shared/src/hdf_device_info.c\");",
                        "    if p.is_file() {",
                        "      let obj = out_dir.join(\"hdf_device_info.o\");",
                        "      let incs = [",
                        "        ohos_root.join(\"drivers/hdf_core/framework/core/shared/include\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/core\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/host/uhdf\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/host/shared\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/utils\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/osal/shared\"),",
                        "        ohos_root.join(\"drivers/hdf_core/interfaces/inner_api/osal/uhdf\"),",
                        "        ohos_root.join(\"base/hiviewdfx/hilog/interfaces/native/innerkits/include\"),",
                        "        ohos_root.join(\"commonlibrary/c_utils/base/include\"),",
                        "        ohos_root.join(\"third_party/bounds_checking_function/include\"),",
                        "      ];",
                        "      let mut cmd = Command::new(\"gcc\");",
                        "      cmd.args([\"-O2\", \"-c\"]);",
                        "      for inc in incs.iter() { if inc.is_dir() { cmd.arg(format!(\"-I{}\", inc.display())); } }",
                        "      cmd.arg(p.to_str().unwrap());",
                        "      cmd.args([\"-o\", obj.to_str().unwrap()]);",
                        "      run(&mut cmd);",
                        "      objs.push(obj);",
                        "    }",
                        "  }",
                        "",
                        "  let a_path = out_dir.join(\"libc2r_ohos_shims.a\");",
                        "  let mut cmd = Command::new(\"ar\");",
                        "  cmd.arg(\"crus\");",
                        "  cmd.arg(a_path.to_str().unwrap());",
                        "  for o in objs.iter() { cmd.arg(o.to_str().unwrap()); }",
                        "  run(&mut cmd);",
                        "",
                        "  println!(\"cargo:rustc-link-search=native={}\", out_dir.display());",
                        "  println!(\"cargo:rustc-link-lib=static=c2r_ohos_shims\");",
                        "}",
                        "",
                    ]
                ),
                encoding="utf-8",
            )

            # -----------------------------------------------------------------
            # Derived tests are generated as *unit tests inside the crate*:
            # - Works for pub(crate)/private module layouts (integration tests cannot access them).
            # - Does not modify translation outputs on disk (temp copy only).
            # -----------------------------------------------------------------
            root_rs = _find_crate_root_rs_for_tests(tmp_crate)
            if not root_rs or not root_rs.is_file():
                res["error"] = "failed to locate crate root .rs for derived tests"
                return res

            test_mod_rs = root_rs.parent / "derived_pure_logic_tests.rs"
            root_txt = root_rs.read_text(encoding="utf-8", errors="ignore")
            if "mod derived_pure_logic_tests;" not in root_txt:
                root_rs.write_text(root_txt + "\n\n#[cfg(test)]\nmod derived_pure_logic_tests;\n", encoding="utf-8")

            def _is_int_ty(ty: str) -> bool:
                t = ty.strip()
                return t in {
                    "i8",
                    "i16",
                    "i32",
                    "i64",
                    "i128",
                    "isize",
                    "u8",
                    "u16",
                    "u32",
                    "u64",
                    "u128",
                    "usize",
                }

            def _rust_null_for(param_ty: str) -> Optional[str]:
                t = param_ty.strip()
                if "*const" in t:
                    return "std::ptr::null()"
                if "*mut" in t or t.startswith("*"):
                    return "std::ptr::null_mut()"
                # Common pointer wrapper patterns (Ptr<T>, Option<Ptr<T>>): use Default::default().
                if "Ptr<" in t or "Option<" in t:
                    return "Default::default()"
                return None

            # Best-effort: detect OHOS error/status constants in a `types` module.
            available_type_consts: set[str] = set()
            for p in [tmp_crate / "src" / "types.rs", tmp_crate / "types.rs"]:
                if not p.is_file():
                    continue
                try:
                    txt = p.read_text(encoding="utf-8", errors="ignore")
                except Exception:
                    continue
                for m in re.finditer(r"\bpub\s+const\s+([A-Za-z_][A-Za-z0-9_]*)\b", txt):
                    available_type_consts.add(m.group(1))

            def _rust_expected_token(tok: str) -> Optional[str]:
                if tok in ("0", "-1", "false", "true"):
                    return tok
                if tok in available_type_consts:
                    return f"crate::types::{tok}"
                return None

            lines: List[str] = []
            lines.append("#![allow(non_snake_case)]")
            lines.append("#![allow(dead_code)]")
            lines.append("#![allow(unused_imports)]")
            lines.append("#![allow(unused_variables)]")
            lines.append("")
            lines.append("use std::ptr;")
            lines.append("")

            generated = 0
            for idx, c in enumerate(usable):
                rust = c["rust"]
                module = str(rust.get("module") or "")
                rust_name = str(rust.get("name") or "")
                params = rust.get("params") or []
                ret_ty = str(rust.get("ret") or "").strip()
                expect_kind = c.get("expect_kind")
                expect = c.get("expect")

                arg_toks = c.get("args") or []
                if len(arg_toks) != len(params):
                    continue

                rust_args: List[str] = []
                ok = True
                for tok, p in zip(arg_toks, params):
                    m = re.search(r":\s*(.+)$", str(p))
                    if not m:
                        ok = False
                        break
                    ty = m.group(1).strip()
                    if _looks_like_null_token(str(tok)):
                        v = _rust_null_for(ty)
                        if v is None:
                            ok = False
                            break
                        rust_args.append(v)
                    elif str(tok) in ("0", "-1"):
                        rust_args.append(str(tok))
                    else:
                        ok = False
                        break
                if not ok:
                    continue

                test_name = f"derived_pure_logic_{project_name}_{idx}_{c.get('test_name')}_{c.get('fn')}"
                test_name = re.sub(r"[^A-Za-z0-9_]", "_", test_name)
                call_path = f"crate::{module}::{rust_name}" if module else f"crate::{rust_name}"
                call = f"{call_path}({', '.join(rust_args)})"

                lines.append("#[test]")
                lines.append(f"fn {test_name}() {{")

                if expect_kind == "no_panic":
                    lines.append(f"    let _ = unsafe {{ {call} }};")
                    lines.append("}")
                    lines.append("")
                    generated += 1
                    continue

                lines.append(f"    let ret = unsafe {{ {call} }};")

                if expect_kind == "bool" and expect in ("false", "true"):
                    if "bool" in ret_ty:
                        lines.append("    assert!(!ret);" if expect == "false" else "    assert!(ret);")
                    elif _is_int_ty(ret_ty):
                        lines.append("    assert_eq!(ret as i64, 0);" if expect == "false" else "    assert_ne!(ret as i64, 0);")
                    else:
                        continue
                elif expect_kind == "eq" and expect is not None:
                    exp = str(expect)
                    if exp in ("nullptr", "NULL"):
                        if "*" in ret_ty:
                            lines.append("    assert!(ret.is_null());")
                        elif "Option<" in ret_ty:
                            lines.append("    assert!(ret.is_none());")
                        elif "Ptr<" in ret_ty:
                            lines.append("    assert!(!ret.as_bool());")
                        else:
                            continue
                    else:
                        exp_rs = _rust_expected_token(exp)
                        if exp_rs is None:
                            continue
                        if _is_int_ty(ret_ty):
                            lines.append(f"    assert_eq!(ret as i64, ({exp_rs}) as i64);")
                        else:
                            continue
                else:
                    continue

                lines.append("}")
                lines.append("")
                generated += 1

            if generated == 0:
                res["error"] = "no usable derived tests after signature matching"
                return res

            test_mod_rs.write_text("\n".join(lines), encoding="utf-8")

            with tempfile.TemporaryDirectory(prefix="our_ohos10_derived_target_") as tgt:
                env = {
                    **os.environ,
                    "CARGO_TARGET_DIR": tgt,
                    "RUSTFLAGS": "-Awarnings -A arithmetic_overflow",
                    "RUST_BACKTRACE": "0",
                    "OHOS_ROOT": str(ohos_root),
                }
                rc, out, err = _run_cmd_capture(
                    # Filter to only run our generated derived tests (avoid mixing counts with any existing tests).
                    ["cargo", "test", "--offline", "derived_pure_logic_", "--", "--test-threads=1"],
                    cwd=tmp_crate,
                    env=env,
                    timeout=timeout,
                )
                res["executed"] = True
                res["stdout"] = out[-200000:]
                res["stderr"] = err[-200000:]
                res["generated_tests"] = generated
                if rc != 0:
                    res["error"] = "cargo test failed"
                m = re.search(
                    r"test result: (ok|FAILED)\.\s*(\d+) passed;\s*(\d+) failed;",
                    out + "\n" + err,
                )
                if m:
                    res["passed"] = int(m.group(2))
                    res["failed"] = int(m.group(3))
                return res
    except Exception as e:
        res["error"] = str(e)
        return res


def run_ohos_unit_tests(
    *,
    project_name: str,
    source_project_dir: Path,
    translated_staticlib: Path,
    ohos_root: Path,
    test_srcs: Optional[List[Path]] = None,
    sample_srcs: Optional[List[Path]] = None,
    timeout: int = 600,
) -> Dict[str, Any]:
    result: Dict[str, Any] = {
        "executed": False,
        "compiled": False,
        "tests_passed": 0,
        "tests_failed": 0,
        "total_tests": 0,
        "pass_rate": 0.0,
        "error": None,
        "build": {},
        "run": {},
    }

    if not source_project_dir.is_dir():
        result["error"] = f"source project dir not found: {source_project_dir}"
        return result
    if not translated_staticlib.is_file():
        result["error"] = f"translated staticlib not found: {translated_staticlib}"
        return result
    if not ohos_root.is_dir():
        result["error"] = f"OHOS root not found: {ohos_root}"
        return result

    # Collect test sources and sample sources (fixtures).
    # NOTE: we allow the caller to provide a filtered set of test sources
    # (e.g., host-runnable subset) so we can separate QEMU-required tests.
    if test_srcs is None or sample_srcs is None:
        all_tests, all_samples, err = _collect_ohos_test_sources(source_project_dir)
        if err:
            result["error"] = err
            return result
        if test_srcs is None:
            test_srcs = all_tests
        if sample_srcs is None:
            sample_srcs = all_samples
    if not test_srcs:
        result["error"] = "no C/C++ test sources found"
        return result

    gtest_root = ohos_root / "third_party/googletest/googletest"
    gtest_all = gtest_root / "src/gtest-all.cc"
    gtest_main = gtest_root / "src/gtest_main.cc"
    # NOTE: OHOS' gtest-all.cc already includes hwext/gtest-ext.cc; compiling it separately causes ODR violations.

    # Debug helper: keep the temporary build directory for post-mortem (e.g., gdb).
    # Disabled by default to avoid leaking /tmp content across runs.
    keep_build = os.environ.get("OHOS_KEEP_BUILD", "").strip() in {"1", "true", "TRUE", "yes", "YES"}
    build_dir_ctx: Optional[tempfile.TemporaryDirectory] = None
    if keep_build:
        build_dir = Path(tempfile.mkdtemp(prefix=f"our_ohos_unittest_{project_name}_"))
    else:
        build_dir_ctx = tempfile.TemporaryDirectory(prefix=f"our_ohos_unittest_{project_name}_")
        build_dir = Path(build_dir_ctx.name)
    result["build_dir"] = str(build_dir)

    try:
        bin_path = build_dir / f"{project_name}_unittests"

        env = {**os.environ}
        env.setdefault("RUST_BACKTRACE", "0")

        _nm_syms: Optional[set[str]] = None

        def _staticlib_symbols() -> set[str]:
            nonlocal _nm_syms
            if _nm_syms is not None:
                return _nm_syms
            try:
                p = subprocess.run(
                    ["nm", "-g", "--defined-only", str(translated_staticlib)],
                    capture_output=True,
                    text=True,
                    timeout=120,
                )
            except Exception:
                _nm_syms = set()
                return _nm_syms
            syms: set[str] = set()
            if p.returncode == 0:
                for line in (p.stdout or "").splitlines():
                    line = line.strip()
                    if not line:
                        continue
                    # Format: <addr> <type> <name>
                    parts = line.split()
                    if parts:
                        syms.add(parts[-1])
            _nm_syms = syms
            return syms

        def _staticlib_has_symbol(sym: str) -> bool:
            return sym in _staticlib_symbols()

        # ---------------------------------------------------------------------
        # Host build shims (do not touch OHOS source or translation outputs)
        # ---------------------------------------------------------------------

        # Some OHOS headers include <linux/ashmem.h>; provide a minimal stub so the
        # unit tests can compile on a normal Linux host.
        linux_dir = build_dir / "linux"
        linux_dir.mkdir(parents=True, exist_ok=True)
        (linux_dir / "ashmem.h").write_text(
            "\n".join(
                [
                    "#ifndef _LINUX_ASHMEM_H",
                    "#define _LINUX_ASHMEM_H",
                    "#define ASHMEM_NAME_LEN 256",
                    "#define ASHMEM_SET_NAME 0x41007701",
                    "#define ASHMEM_GET_NAME 0x41007702",
                    "#define ASHMEM_SET_SIZE 0x40087703",
                    "#define ASHMEM_GET_SIZE 0x40087704",
                    "#define ASHMEM_SET_PROT_MASK 0x40087705",
                    "#define ASHMEM_GET_PROT_MASK 0x40087706",
                    "#endif",
                    "",
                ]
            ),
            encoding="utf-8",
        )

        include_dirs: List[Path] = [build_dir]  # for <linux/ashmem.h> stub
        for rel in (
            "include",
            "src",
            "test",
            "test/unittest",
            "test/unittest/common",
            "test/sample",
            "unittest",
            "unittest/src",
            "unittest/packets",
        ):
            d = source_project_dir / rel
            if d.is_dir():
                include_dirs.append(d)
        include_dirs.extend(_default_ohos_test_include_dirs(ohos_root))
        include_flags = [f"-I{d}" for d in include_dirs]

        # Provide minimal runtime shims required by many translated crates.
        shim_c = build_dir / "c2r_ohos_shims.c"
        # Some OHOS unit tests rely on IPC sbuf (SBUF_IPC). On a plain Linux host, the real
        # hipc implementation is unavailable; we provide a best-effort shim that maps IPC sbuf
        # obtain/bind to RAW sbuf obtain/bind (only for host-side evaluation).
        need_sbuf_impl = not _staticlib_has_symbol("HdfSbufReadUint32")
        sbuf_obtain_body = "return SbufObtainRaw(capacity);" if need_sbuf_impl else "return NULL;"
        sbuf_bind_body = "return SbufBindRaw(base, size);" if need_sbuf_impl else "return NULL;"
        shim_c.write_text(
            "\n".join(
                [
                    "#include <stdarg.h>",
                    "#include <stddef.h>",
                    "#include <stdint.h>",
                    "#include <stdio.h>",
                    "#include <stdlib.h>",
                    "#include <string.h>",
                    "#include <execinfo.h>",
                    "#include <signal.h>",
                    "#include <unistd.h>",
                    "",
                    "static void c2r_segv_handler(int sig) {",
                    "  void *bt[64];",
                    "  int n = backtrace(bt, 64);",
                    "  fprintf(stderr, \"\\n[c2r] Caught signal %d\\n\", sig);",
                    "  backtrace_symbols_fd(bt, n, 2);",
                    "  _exit(128 + sig);",
                    "}",
                    "",
                    "__attribute__((constructor)) static void c2r_install_segv_handler(void) {",
                    "  const char* en = getenv(\"OHOS_SEGV_BACKTRACE\");",
                    "  if (en && en[0] == '1') {",
                    "    (void)signal(SIGSEGV, c2r_segv_handler);",
                    "    (void)signal(SIGABRT, c2r_segv_handler);",
                    "  }",
                    "}",
                    "",
                    "struct HdfSBufImpl; // opaque",
                    "struct HdfSBufImpl *SbufObtainRaw(size_t capacity);",
                    "struct HdfSBufImpl *SbufBindRaw(uintptr_t base, size_t size);",
                    "",
                    "__attribute__((weak)) struct HdfSBufImpl *SbufObtainIpc(size_t capacity) { " + sbuf_obtain_body + " }",
                    "__attribute__((weak)) struct HdfSBufImpl *SbufBindIpc(uintptr_t base, size_t size) { " + sbuf_bind_body + " }",
                    "__attribute__((weak)) struct HdfSBufImpl *SbufObtainIpcHw(size_t capacity) { " + sbuf_obtain_body + " }",
                    "__attribute__((weak)) struct HdfSBufImpl *SbufBindRawIpcHw(uintptr_t base, size_t size) { " + sbuf_bind_body + " }",
                    "",
                    "struct HdfObjectCreator; // opaque",
                    "__attribute__((weak)) const struct HdfObjectCreator *HdfObjectManagerGetCreators(int objectId) {",
                    "  (void)objectId;",
                    "  return NULL;",
                    "}",
                    "",
                    "int HiLogPrint(int type, int level, unsigned int domain, const char* tag, const char* fmt, ...) {",
                    "  (void)type; (void)domain;",
                    "  // Best-effort logging for host-side debugging. OHOS format strings use",
                    "  // extensions like %{public}s which are not printf-compatible; we print",
                    "  // the raw format string only (no varargs formatting).",
                    "  const char* en = getenv(\"OHOS_HOST_LOG\");",
                    "  if (en && en[0] == '1' && level >= 3) {",
                    "    fprintf(stderr, \"[HiLog][%s] %s\\n\", tag ? tag : \"\", fmt ? fmt : \"\");",
                    "  }",
                    "  return 0;",
                    "}",
                    "",
                    "void* OsalMemCalloc(size_t size) {",
                    "  return calloc(1, size);",
                    "}",
                    "",
                    "void* OsalMemAlloc(size_t size) {",
                    "  return malloc(size);",
                    "}",
                    "",
                    "void* OsalMemRealloc(void* ptr, size_t size) {",
                    "  return realloc(ptr, size);",
                    "}",
                    "",
                    "void OsalMemFree(void* mem) {",
                    "  free(mem);",
                    "}",
                    "",
                    "// -------------------------------------------------------------------------",
                    "// Minimal device-info shim for host-side unit tests",
                    "// -------------------------------------------------------------------------",
                    "// Some modules (e.g., appverify_lite) depend on GetDevUdid(), which is normally",
                    "// provided by OHOS system parameter/device-info services. On a plain Linux",
                    "// host we provide a deterministic placeholder to allow unit tests to link/run.",
                    "__attribute__((weak)) int GetDevUdid(char *udid, int size) {",
                    "  if (udid == NULL || size <= 0) {",
                    "    return -1;",
                    "  }",
                    "  const char *k = \"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\"; // 64 chars",
                    "  size_t n = strlen(k);",
                    "  if (n >= (size_t)size) {",
                    "    n = (size_t)size - 1;",
                    "  }",
                    "  (void)memcpy(udid, k, n);",
                    "  udid[n] = '\\0';",
                    "  return 0;",
                    "}",
                    "",
                ]
            ),
            encoding="utf-8",
        )
        shim_o = build_dir / "c2r_ohos_shims.o"
        rc, o_out, o_err = _run_cmd_capture(
            ["gcc", "-O2", "-c", str(shim_c), "-o", str(shim_o)],
            cwd=build_dir,
            env=env,
            timeout=timeout,
        )
        result["host_shims"] = {"cmd": ["gcc", "-O2", "-c", str(shim_c), "-o", str(shim_o)], "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
        if rc != 0 or not shim_o.is_file():
            result["error"] = "host shim build failed"
            return result

        # For host__ project: create expected *.z.so fixtures referenced by tests (best-effort).
        # Match shared-lib names referenced in unit tests like:
        #   HdfLoadVdi("libvdi_sample1_driver.z.so");
        lib_name_re = re.compile(r'"(lib[^"]+\.z\.so)"')
        wanted_libs: List[str] = []
        try:
            for ts in test_srcs:
                wanted_libs.extend(lib_name_re.findall(ts.read_text(encoding="utf-8", errors="ignore")))
        except Exception:
            pass
        wanted_libs = sorted(set(wanted_libs))

        if project_name.startswith("host__") and wanted_libs:
            sample_map = {
                "vdi_sample1_driver": "libvdi_sample1_driver.z.so",
                "vdi_sample1_symbol": "libvdi_sample1_symbol.z.so",
                "vdi_sample2_driver": "libvdi_sample2_driver.z.so",
            }
            for stem, so_name in sample_map.items():
                if so_name not in wanted_libs:
                    continue
                src = next((p for p in sample_srcs if p.stem == stem), None)
                if not src:
                    continue
                out_so = build_dir / so_name
                src_to_build = src
                cc = "gcc" if src.suffix.lower() == ".c" else "g++"
                # The VDI descriptor symbol `hdfVdiDesc` must have C linkage (unmangled) so
                # HdfLoadVdi can dlsym("hdfVdiDesc"). For C++ sample drivers, patch the macro
                # call to an explicit `extern "C"` definition on a temp copy.
                if cc == "g++":
                    try:
                        txt = src.read_text(encoding="utf-8", errors="ignore")
                        if "HDF_VDI_INIT(" in txt and "extern \"C\"" not in txt:
                            txt = re.sub(
                                r"HDF_VDI_INIT\\s*\\(\\s*([^\\)]+)\\s*\\)\\s*;",
                                r"extern \"C\" struct HdfVdiBase *hdfVdiDesc = (struct HdfVdiBase *)&(\\1);",
                                txt,
                                count=1,
                            )
                            patched = build_dir / f"patched_{src.name}"
                            patched.write_text(txt, encoding="utf-8")
                            src_to_build = patched
                    except Exception:
                        pass
                cmd = [
                    cc,
                    "-shared",
                    "-fPIC",
                    *([] if cc == "gcc" else ["-std=c++17"]),
                    "-O2",
                    *include_flags,
                    str(src_to_build),
                    "-o",
                    str(out_so),
                ]
                rc, so_out, so_err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("fixtures", {})[so_name] = {
                    "built": rc == 0,
                    "cmd": cmd,
                    "stdout": so_out[-4000:],
                    "stderr": so_err[-8000:],
                }

            # For "*_error.z.so" expected by tests: generate a dummy shared lib file that exists
            # (but without required symbols, so the test path should fail as intended).
            for so_name in wanted_libs:
                if not so_name.endswith("_error.z.so"):
                    continue
                out_so = build_dir / so_name
                dummy_c = build_dir / f"{so_name}.c"
                dummy_c.write_text("void __ohos_dummy(void) {}\n", encoding="utf-8")
                cmd = ["gcc", "-shared", "-fPIC", "-O2", str(dummy_c), "-o", str(out_so)]
                rc, so_out, so_err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("fixtures", {})[so_name] = {
                    "built": rc == 0,
                    "cmd": cmd,
                    "stdout": so_out[-4000:],
                    "stderr": so_err[-8000:],
                }

        # Build securec objects if available (memcpy_s, memset_s, ...).
        securec_src_dir = ohos_root / "third_party/bounds_checking_function/src"
        securec_inc_dir = ohos_root / "third_party/bounds_checking_function/include"
        securec_objs: List[Path] = []
        if securec_src_dir.is_dir():
            obj_dir = build_dir / "securec_objs"
            obj_dir.mkdir(parents=True, exist_ok=True)
            for cfile in sorted([p for p in securec_src_dir.glob("*.c") if p.is_file()]):
                obj = obj_dir / (cfile.stem + ".o")
                cc_cmd = [
                    "gcc",
                    "-O2",
                    "-c",
                    f"-I{securec_inc_dir}" if securec_inc_dir.is_dir() else "",
                    f"-I{securec_src_dir}",
                    str(cfile),
                    "-o",
                    str(obj),
                ]
                cc_cmd = [x for x in cc_cmd if x]
                rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("securec_build", []).append(
                    {"src": str(cfile), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
                )
                if rc != 0:
                    result["error"] = "securec build failed"
                    return result
                securec_objs.append(obj)

        # Build minimal HDF utils objects (host-side deps for many unit tests).
        # Only do this when the translated staticlib does NOT already export these sbuf symbols.
        hdf_utils_objs: List[Path] = []
        if need_sbuf_impl:
            utils_src_dir = ohos_root / "drivers/hdf_core/framework/utils/src"
            # Host-side NOTE:
            # We map SBUF_IPC obtain/bind to the RAW sbuf implementation. The raw impl accepts
            # writeString(NULL) (writes a 0-length buffer), but IPC sbuf semantics treat NULL
            # strings as invalid. Some upstream tests rely on this, so we patch a temp copy.
            raw_impl = utils_src_dir / "hdf_sbuf_impl_raw.c"
            raw_impl_to_compile = raw_impl
            if raw_impl.is_file():
                try:
                    txt = raw_impl.read_text(encoding="utf-8", errors="ignore")
                    needle = "return SbufRawImplWriteBuffer(impl, (const uint8_t *)value, value ? (strlen(value) + 1) : 0);"
                    if needle in txt:
                        txt = txt.replace(
                            needle,
                            "\n".join(
                                [
                                    "if (value == NULL) {",
                                    "    return false;",
                                    "}",
                                    "return SbufRawImplWriteBuffer(impl, (const uint8_t *)value, strlen(value) + 1);",
                                ]
                            ),
                            1,
                        )
                        patched_raw = build_dir / "patched_hdf_sbuf_impl_raw.c"
                        patched_raw.write_text(txt, encoding="utf-8")
                        raw_impl_to_compile = patched_raw
                except Exception:
                    pass

            for cfile in (
                raw_impl_to_compile,
                utils_src_dir / "hdf_sbuf.c",
            ):
                if not cfile.is_file():
                    continue
                obj = build_dir / (cfile.stem + ".o")
                cc_cmd = ["gcc", "-O2", "-c", *include_flags, str(cfile), "-o", str(obj)]
                rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("hdf_utils_build", []).append(
                    {"src": str(cfile), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
                )
                if rc != 0 or not obj.is_file():
                    result["error"] = "hdf utils build failed"
                    return result
                hdf_utils_objs.append(obj)

        # HDF slist (used by osal_slist_test.cpp). Compile only if missing.
        hdf_slist_objs: List[Path] = []
        if not _staticlib_has_symbol("HdfSListAdd"):
            slist_c = ohos_root / "drivers/hdf_core/framework/utils/src/hdf_slist.c"
            if slist_c.is_file():
                obj = build_dir / "hdf_slist.o"
                cc_cmd = ["gcc", "-O2", "-c", *include_flags, str(slist_c), "-o", str(obj)]
                rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("hdf_slist_build", []).append(
                    {"src": str(slist_c), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
                )
                if rc != 0 or not obj.is_file():
                    result["error"] = "hdf slist build failed"
                    return result
                hdf_slist_objs.append(obj)

        # HDF device info (needed by some shared-module tests). Compile only if missing.
        hdf_device_info_objs: List[Path] = []
        if not _staticlib_has_symbol("HdfDeviceInfoNewInstance"):
            di_c = ohos_root / "drivers/hdf_core/framework/core/shared/src/hdf_device_info.c"
            if di_c.is_file():
                obj = build_dir / "hdf_device_info.o"
                cc_cmd = ["gcc", "-O2", "-c", *include_flags, str(di_c), "-o", str(obj)]
                rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("hdf_device_info_build", []).append(
                    {"src": str(di_c), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
                )
                if rc != 0 or not obj.is_file():
                    result["error"] = "hdf device info build failed"
                    return result
                hdf_device_info_objs.append(obj)

        # Build appverify_lite: link required mbedtls objects.
        #
        # Note: appverify_lite unit tests call `mbedtls_base64_decode`, and the translated Rust
        # code itself references additional `mbedtls_*` symbols (e.g., x509 parsing) via externs.
        # We compile a host-side set of mbedtls objects from the OHOS tree to satisfy linkage.
        mbedtls_objs: List[Path] = []
        if project_name.startswith("appverify_lite__"):
            mbedtls_lib = ohos_root / "third_party/mbedtls/library"
            if not mbedtls_lib.is_dir():
                result["error"] = "mbedtls library dir not found"
                return result

            mbedtls_obj_dir = build_dir / "mbedtls_objs"
            mbedtls_obj_dir.mkdir(parents=True, exist_ok=True)
            # Avoid known duplicate-definition variants when compiling on host.
            mbedtls_skip = {
                # Both wrapper variants define the same `psa_driver_wrapper_*` symbols.
                "psa_crypto_driver_wrappers_no_static.c",
            }
            for cfile in sorted([p for p in mbedtls_lib.glob("*.c") if p.is_file()]):
                if cfile.name in mbedtls_skip:
                    continue
                obj = mbedtls_obj_dir / (cfile.stem + ".o")
                cc_cmd = ["gcc", "-O2", "-c", *include_flags, str(cfile), "-o", str(obj)]
                rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
                result.setdefault("mbedtls_build", []).append(
                    {"src": str(cfile), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
                )
                if rc != 0 or not obj.is_file():
                    result["error"] = "mbedtls build failed"
                    return result
                mbedtls_objs.append(obj)

        # Build appverify_lite: link required cJSON objects.
        #
        # The translated Rust code references cJSON_* symbols for parsing profile/provision JSON.
        # In a full OHOS build these come from the cJSON component; for host-side evaluation we
        # compile the upstream cJSON implementation from the OHOS tree.
        cjson_objs: List[Path] = []
        if project_name.startswith("appverify_lite__"):
            cjson_dir = ohos_root / "third_party/cJSON"
            cjson_c = cjson_dir / "cJSON.c"
            if not cjson_c.is_file():
                result["error"] = "cJSON.c not found"
                return result
            cjson_obj_dir = build_dir / "cjson_objs"
            cjson_obj_dir.mkdir(parents=True, exist_ok=True)
            obj = cjson_obj_dir / "cJSON.o"
            cc_cmd = ["gcc", "-O2", "-c", f"-I{cjson_dir}", *include_flags, str(cjson_c), "-o", str(obj)]
            rc, o_out, o_err = _run_cmd_capture(cc_cmd, cwd=build_dir, env=env, timeout=timeout)
            result.setdefault("cjson_build", []).append(
                {"src": str(cjson_c), "obj": str(obj), "cmd": cc_cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
            )
            if rc != 0 or not obj.is_file():
                result["error"] = "cJSON build failed"
                return result
            cjson_objs.append(obj)

        # Compile test sources: use gcc for C, g++ for C++ (do NOT compile *.c as C++).
        obj_dir = build_dir / "objs"
        obj_dir.mkdir(parents=True, exist_ok=True)
        test_objs: List[Path] = []

        # Some upstream OHOS unit tests assume 32-bit (size_t == uint32_t) and contain
        # UB on x86_64 hosts. We patch a copy in the temp build dir so we can run the
        # original tests without modifying the source tree.
        patched_test_srcs: Dict[Path, Path] = {}
        if project_name.startswith("appverify_lite__"):
            for src in test_srcs:
                if src.name != "write_file.cpp":
                    continue
                try:
                    txt = src.read_text(encoding="utf-8", errors="ignore")
                except Exception:
                    continue
                # Fix stack smashing: mbedtls_base64_decode writes a size_t output length.
                # The original test uses `int32_t len` and casts `&len` to `size_t*`,
                # which overwrites stack memory on 64-bit.
                if "reinterpret_cast<size_t *>(&len)" in txt and "outLen" not in txt:
                    txt = txt.replace(
                        "int32_t len = 0;",
                        "int32_t len = 0;\n    size_t outLen = 0;",
                        1,
                    )
                    txt = txt.replace("reinterpret_cast<size_t *>(&len)", "&outLen")
                    call_idx = txt.find("mbedtls_base64_decode")
                    if call_idx != -1:
                        end = txt.find(");", call_idx)
                        if end != -1:
                            end += 2
                            txt = (
                                txt[:end]
                                + "\n    len = static_cast<int32_t>(outLen);"
                                + txt[end:]
                            )
                patched = build_dir / "patched_write_file.cpp"
                patched.write_text(txt, encoding="utf-8")
                patched_test_srcs[src] = patched

        def _compile_one(src: Path) -> Tuple[bool, Dict[str, Any], Optional[Path]]:
            ext = src.suffix.lower()
            is_c = ext == ".c"
            cc = "gcc" if is_c else "g++"
            obj = obj_dir / (src.name.replace("/", "_") + ".o")
            cmd = [
                cc,
                "-O2",
                "-c",
                *([] if is_c else ["-std=c++17"]),
                *include_flags,
                str(src),
                "-o",
                str(obj),
            ]
            rc, o_out, o_err = _run_cmd_capture(cmd, cwd=build_dir, env=env, timeout=timeout)
            info = {"src": str(src), "obj": str(obj), "cmd": cmd, "ok": rc == 0, "stdout": o_out[-4000:], "stderr": o_err[-8000:]}
            return rc == 0 and obj.is_file(), info, obj if obj.is_file() else None

        for src in test_srcs:
            ok, info, obj = _compile_one(patched_test_srcs.get(src, src))
            result.setdefault("test_compile", []).append(info)
            if not ok or obj is None:
                result["error"] = "unit test compile failed"
                return result
            test_objs.append(obj)

        # Compile gtest sources.
        gtest_objs: List[Path] = []
        for src in (gtest_all, gtest_main):
            ok, info, obj = _compile_one(src)
            result.setdefault("gtest_compile", []).append(info)
            if not ok or obj is None:
                result["error"] = "gtest compile failed"
                return result
            gtest_objs.append(obj)

        # Link unit test binary.
        link_cmd: List[str] = [
            "g++",
            "-std=c++17",
            "-O2",
            # Ensure host binary exports its symbols so dlopen()'d *.so fixtures can resolve
            # dependencies like HiLogPrint / OsalMem* at load time.
            "-Wl,--export-dynamic",
            *[str(o) for o in gtest_objs],
            *[str(o) for o in test_objs],
            *[str(o) for o in securec_objs],
            *[str(o) for o in hdf_utils_objs],
            *[str(o) for o in hdf_slist_objs],
            *[str(o) for o in hdf_device_info_objs],
            *[str(o) for o in mbedtls_objs],
            *[str(o) for o in cjson_objs],
            str(shim_o),
            str(translated_staticlib),
            "-ldl",
            "-pthread",
            "-lm",
            "-o",
            str(bin_path),
        ]
        link_cmd = [x for x in link_cmd if x]
        rc, out, err = _run_cmd_capture(link_cmd, cwd=build_dir, env=env, timeout=timeout)
        result["build"] = {"cmd": link_cmd, "ok": rc == 0, "exit_code": rc, "stdout": out[-4000:], "stderr": err[-8000:]}
        if rc != 0:
            result["error"] = "unit test link failed"
            return result

        result["compiled"] = True

        run_env = dict(env)
        run_env["LD_LIBRARY_PATH"] = f"{build_dir}:{run_env.get('LD_LIBRARY_PATH', '')}"
        # host__ VDI loader uses a fixed /vendor/lib prefix in upstream code; our translated
        # implementation supports a host-side override via env var so we can run in /tmp.
        if project_name.startswith("host__"):
            run_env["HDF_VDI_PATH"] = str(build_dir)
        if project_name.startswith("appverify_lite__"):
            run_env["OHOS_HOST_LOG"] = "1"
        run_cmd = [str(bin_path), "--gtest_color=no"]
        rc, out, err = _run_cmd_capture(run_cmd, cwd=build_dir, env=run_env, timeout=timeout)
        result["run"] = {"cmd": run_cmd, "ok": rc == 0, "exit_code": rc, "stdout": out[-8000:], "stderr": err[-8000:]}
        result["executed"] = True

        combined_out = out + "\n" + err
        passed, failed = _parse_gtest_counts(combined_out)
        result["tests_passed"] = passed
        result["tests_failed"] = failed
        result["total_tests"] = passed + failed
        result["pass_rate"] = (passed / result["total_tests"]) if result["total_tests"] else 0.0
        # When tests crash/panic, gtest may not print the final [PASSED]/[FAILED] summary.
        # Capture the expected test count from the header so downstream reports can show 0/N.
        m = re.search(r"\\[\\s*=+\\s*\\]\\s+Running\\s+(\\d+)\\s+tests?\\b", combined_out)
        if m:
            try:
                result["expected_total_tests"] = int(m.group(1))
            except Exception:
                pass
        try:
            result["started_tests"] = len(re.findall(r"^\\[\\s*RUN\\s*\\]", combined_out, re.MULTILINE))
        except Exception:
            pass
        if rc != 0 and result["total_tests"] == 0:
            result["error"] = "unit test runtime failed (no summary)"
        return result
    finally:
        if build_dir_ctx is not None:
            try:
                build_dir_ctx.cleanup()
            except Exception:
                pass


# =============================================================================
# Incremental compilation stats (from our run artifacts)
# =============================================================================

def load_incremental_stats_from_summary(run_dir: Path) -> Dict[str, Any]:
    """
    Prefer `results/translation_summary_report.json` (generated by our framework),
    fall back to per-project `translation_stats.json` if needed.
    """
    summary_path = run_dir / "results" / "translation_summary_report.json"
    if summary_path.is_file():
        try:
            obj = json.loads(summary_path.read_text(encoding="utf-8", errors="replace"))
            projects = obj.get("projects_detail") or obj.get("projects") or []
            out: Dict[str, Any] = {}
            for p in projects:
                name = p.get("project")
                if not name:
                    continue
                out[name] = {
                    "total_functions": int(p.get("total_functions") or 0),
                    "passed_total": int(p.get("passed_total") or 0),
                    "passed_directly": int(p.get("passed_directly") or 0),
                    "passed_after_repair": int(p.get("passed_after_repair") or 0),
                    "failed_reverted": int(p.get("failed_reverted") or 0),
                    "injection_failed": int(p.get("injection_failed") or 0),
                    "pass_rate": float(p.get("pass_rate_percent") or 0.0) / 100.0,
                    "source": "translation_summary_report.json",
                    "stats_file": p.get("stats_file"),
                }
            return out
        except Exception:
            pass

    # Fallback: scan per-project stats files.
    out: Dict[str, Any] = {}
    intermediate = run_dir / "intermediate"
    if not intermediate.is_dir():
        return out
    for proj in DISPLAY_PROJECT_ORDER:
        proj_dir = intermediate / proj
        if not proj_dir.is_dir():
            continue
        stats = next(
            (p for p in proj_dir.rglob("translation_stats.json") if "incremental_work" in str(p)),
            None,
        )
        if not stats:
            continue
        try:
            s = json.loads(stats.read_text(encoding="utf-8", errors="replace"))
        except Exception:
            continue
        total = int(s.get("total") or 0)
        compiled = int(s.get("compiled") or 0)
        out[proj] = {
            "total_functions": total,
            "passed_total": compiled,
            "passed_directly": int(s.get("compiled") or 0) - int(s.get("repaired") or 0),
            "passed_after_repair": int(s.get("repaired") or 0),
            "failed_reverted": int(s.get("failed") or 0),
            "injection_failed": int(s.get("injection_failed") or 0),
            "pass_rate": (compiled / total) if total else 0.0,
            "source": "translation_stats.json",
            "stats_file": str(stats),
        }
    return out


def find_project_crate_dir(run_dir: Path, project_name: str) -> Optional[Path]:
    """
    Prefer final translated project; fall back to skeleton if final project is missing.

    - final:     intermediate/<proj>/workspace/final_projects/<proj>/translate_by_*/Cargo.toml
    - skeleton:  intermediate/<proj>/workspace/skeletons/<proj>/Cargo.toml
    """
    base = run_dir / "intermediate" / project_name / "workspace" / "final_projects" / project_name
    if base.is_dir():
        candidates = [p for p in base.glob("translate_by_*") if (p / "Cargo.toml").is_file()]
        if not candidates:
            # sometimes nested differently; be defensive
            candidates = [p.parent for p in base.rglob("Cargo.toml") if p.is_file()]
        if candidates:
            # Choose deterministically.
            return sorted(candidates, key=lambda p: p.name)[0]

    sk = run_dir / "intermediate" / project_name / "workspace" / "skeletons" / project_name
    if (sk / "Cargo.toml").is_file():
        return sk
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description="分析我们的框架 OHOS(10) 运行结果（真实指标，不改翻译结果）。")
    parser.add_argument("--run-dir", type=Path, required=True, help="运行目录（例如 translation_outputs/deepseek-coder-ohos10）")
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        default=None,
        help="输出 JSON 路径（默认: <run>/results/compilation_analysis_ohos10.json）",
    )

    parser.add_argument("--run-clippy", action="store_true", help="Run cargo clippy and count warnings")
    parser.add_argument("--analyze-unsafe", action="store_true", help="Analyze unsafe rate")
    parser.add_argument("--run-ohos-tests", action="store_true", help="Run OHOS source gtest unit tests (best-effort)")
    parser.add_argument("--ohos-test-timeout", type=int, default=900, help="Timeout seconds for OHOS unit tests")
    parser.add_argument(
        "--run-derived-tests",
        action="store_true",
        help="Extract a conservative pure-logic subset from OHOS gtest and run as Rust unit tests (temp copy).",
    )
    parser.add_argument("--derived-test-timeout", type=int, default=900, help="Timeout seconds for derived Rust tests")
    parser.add_argument("--huawei-projects-tsv", type=Path, default=DEFAULT_HUAWEI_PROJECTS_TSV)
    parser.add_argument("--ohos-root", type=Path, default=DEFAULT_OHOS_ROOT)
    parser.add_argument("--all", action="store_true", help="Run: clippy + unsafe + ohos-tests")

    args = parser.parse_args()

    run_dir = args.run_dir
    if not run_dir.is_dir():
        print(f"Error: run dir not found: {run_dir}", file=sys.stderr)
        return 2

    out_path = args.output or (run_dir / "results" / "compilation_analysis_ohos10.json")

    run_clippy = args.run_clippy or args.all
    analyze_unsafe = args.analyze_unsafe or args.all
    run_ohos_tests = args.run_ohos_tests or args.all
    run_derived_tests = args.run_derived_tests  # keep separate from --all (avoid surprise)

    huawei_map: Dict[str, Path] = load_huawei_projects_map(args.huawei_projects_tsv) if run_ohos_tests else {}
    ohos_root = args.ohos_root

    inc_map = load_incremental_stats_from_summary(run_dir)

    # Reuse unsafe analyzer from the main script (already used across other parts of the repo).
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    try:
        import analyze_c2r_compilation_rate as base  # type: ignore
    except Exception:
        base = None

    projects: Dict[str, Any] = {}

    total_projects = len(DISPLAY_PROJECT_ORDER)
    crates_found = 0
    cargo_ok = 0
    total_clippy = total_rustc = total_warn = total_err = 0
    unsafe_projects = 0
    total_code_lines = total_unsafe_total_lines = total_unsafe_items = 0
    inc_total = inc_ok = 0
    ohos_attempted = 0
    ohos_compiled = 0
    ohos_tests_total = 0
    ohos_tests_passed = 0
    ohos_tests_failed = 0
    ohos_skipped_qemu = 0
    ohos_host_projects = 0
    derived_projects = 0
    derived_tests_generated = 0
    derived_tests_passed = 0
    derived_tests_failed = 0

    for project_name in DISPLAY_PROJECT_ORDER:
        crate_dir = find_project_crate_dir(run_dir, project_name)
        proj_res: Dict[str, Any] = {"project_dir": str(crate_dir) if crate_dir else None}
        if not crate_dir:
            proj_res["error"] = "未找到项目目录（final_projects 或 skeletons）"
            projects[project_name] = proj_res
            continue
        crates_found += 1

        # Incremental compilation stats (from our framework outputs).
        inc = inc_map.get(project_name)
        if inc:
            proj_res["incremental_compilation"] = inc
            inc_total += int(inc.get("total_functions") or 0)
            inc_ok += int(inc.get("passed_total") or 0)

        # Cargo check (project compiles as a Rust crate).
        cc = run_cargo_check(crate_dir)
        proj_res["cargo_check"] = cc
        if cc.get("passed"):
            cargo_ok += 1

        # Clippy warnings (real JSON).
        if run_clippy:
            cl = run_cargo_clippy(crate_dir)
            proj_res["clippy"] = cl
            total_clippy += int(cl.get("warning_count") or 0)
            total_rustc += int(cl.get("rustc_warning_count") or 0)
            total_warn += int(cl.get("warning_count_total") or 0)
            total_err += int(cl.get("error_count") or 0)

        # Unsafe analysis (scan *.rs in translated crate).
        if analyze_unsafe:
            if base is not None and hasattr(base, "analyze_unsafe_code"):
                try:
                    ua_obj = base.analyze_unsafe_code(crate_dir)  # returns UnsafeAnalysis dataclass
                    ua = ua_obj.to_dict() if hasattr(ua_obj, "to_dict") else dict(ua_obj)
                except Exception as e:
                    ua = {"error": str(e)}
            else:
                ua = {"error": "unsafe analyzer import failed"}
            proj_res["unsafe_analysis"] = ua
            if not ua.get("error"):
                unsafe_projects += 1
                total_code_lines += int(ua.get("code_lines", 0) or 0)
                total_unsafe_total_lines += int(ua.get("unsafe_total_lines", 0) or 0)
                total_unsafe_items += int(ua.get("total_unsafe_items", 0) or 0)

        # OHOS gtest unit tests (best-effort).
        if run_ohos_tests:
            src_dir = huawei_map.get(project_name)
            if not src_dir:
                proj_res["ohos_unit_tests"] = {"executed": False, "compiled": False, "error": "project not in huawei_projects.tsv"}
            else:
                # Split tests into:
                # - host-runnable subset (best-effort): does NOT call HdfTestOpenService / use hdf_uhdf_test.h
                # - QEMU/device required: depends on HDF test service runtime
                all_tests, all_samples, err = _collect_ohos_test_sources(src_dir)
                if err:
                    proj_res["ohos_unit_tests"] = {"executed": False, "compiled": False, "error": err}
                else:
                    # Classify tests that cannot be reasonably executed on a plain host:
                    # - depend on HDF test service/runtime (HdfTestOpenService / HdfTestSendMsgToService)
                    # - depend on OHOS IPC/binder types (MessageParcel / IRemoteObject / sptr / remote adapter)
                    # - depend on kernel nodes (/sys/...) or device registration APIs
                    skip_re = re.compile(
                        r"("
                        r"hdf_uhdf_test\.h|\bHdfTestOpenService\b|\bHdfTestSendMsgToService\b|"
                        r"\bMessageParcel\b|\bIRemoteObject\b|\bIProxyBroker\b|\bsptr\s*<|"
                        r"\bSbufToParcel\b|\bParcelToSbuf\b|\bHdfRemoteAdapter\b|"
                        r"\bHdfRegisteDevice\b|\bHdfUnregisteDevice\b|\bHdfDeviceSendEvent\b|\bHdfDeviceSendEventToClient\b|"
                        r"\"/sys/|/sys/"
                        r")"
                    )
                    qemu_tests: List[Path] = []
                    host_tests: List[Path] = []
                    for ts in all_tests:
                        try:
                            txt = ts.read_text(encoding="utf-8", errors="ignore")
                        except Exception:
                            txt = ""
                        if skip_re.search(txt):
                            qemu_tests.append(ts)
                        else:
                            host_tests.append(ts)

                    proj_res["ohos_tests_split"] = {
                        "host_test_files": len(host_tests),
                        "qemu_test_files": len(qemu_tests),
                        "qemu_test_files_list": [str(p.relative_to(src_dir)) for p in qemu_tests],
                    }

                    if not host_tests:
                        ohos_skipped_qemu += 1
                        proj_res["ohos_unit_tests"] = {
                            "executed": False,
                            "compiled": False,
                            "skipped": True,
                            "needs_qemu": True,
                            "error": "该项目的 gtest 依赖 OHOS 运行时/服务/IPC/内核节点（建议在 OHOS QEMU/真机环境跑），已跳过主机侧执行。",
                            "qemu_test_files": [str(p.relative_to(src_dir)) for p in qemu_tests],
                        }
                    else:
                        ohos_attempted += 1
                        ohos_host_projects += 1

                        exportable = _collect_exportable_fn_names(crate_dir)
                        called = _collect_called_identifiers(host_tests)
                        export_names = (called & exportable) if exportable else set()
                        if not export_names:
                            # Fallback: export all pub extern \"C\" fns (may increase link deps, but avoids false negatives).
                            export_names = None

                        static_res = build_staticlib_from_crate(
                            crate_dir, export_names=export_names, timeout=args.ohos_test_timeout
                        )
                        proj_res["staticlib_build"] = static_res
                        if not static_res.get("ok"):
                            proj_res["ohos_unit_tests"] = {
                                "executed": False,
                                "compiled": False,
                                "error": static_res.get("error") or "staticlib build failed",
                            }
                        else:
                            tr_lib = Path(static_res["staticlib"])
                            ut = run_ohos_unit_tests(
                                project_name=project_name,
                                source_project_dir=src_dir,
                                translated_staticlib=tr_lib,
                                ohos_root=ohos_root,
                                test_srcs=host_tests,
                                sample_srcs=all_samples,
                                timeout=args.ohos_test_timeout,
                            )
                            ut["skipped_qemu_test_files"] = [str(p.relative_to(src_dir)) for p in qemu_tests]
                            proj_res["ohos_unit_tests"] = ut
                            if ut.get("compiled"):
                                ohos_compiled += 1
                            ohos_tests_total += int(ut.get("total_tests") or 0)
                            ohos_tests_passed += int(ut.get("tests_passed") or 0)
                            ohos_tests_failed += int(ut.get("tests_failed") or 0)

        # Derived Rust unit tests (pure-logic subset extracted from gtest).
        if run_derived_tests:
            src_dir = huawei_map.get(project_name)
            if not src_dir:
                proj_res["derived_rust_tests"] = {"executed": False, "error": "project not in huawei_projects.tsv"}
            else:
                all_tests, _, err = _collect_ohos_test_sources(src_dir)
                if err:
                    proj_res["derived_rust_tests"] = {"executed": False, "error": err}
                else:
                    cases = _extract_pure_logic_cases_from_gtest(all_tests)
                    dr = run_derived_rust_tests_from_gtest(
                        project_name=project_name,
                        crate_dir=crate_dir,
                        cases=cases,
                        ohos_root=ohos_root,
                        timeout=args.derived_test_timeout,
                    )
                    proj_res["derived_rust_tests"] = dr
                    if dr.get("executed"):
                        derived_projects += 1
                    derived_tests_generated += int(dr.get("generated_tests") or 0)
                    derived_tests_passed += int(dr.get("passed") or 0)
                    derived_tests_failed += int(dr.get("failed") or 0)

        projects[project_name] = proj_res

    summary: Dict[str, Any] = {
        "run_dir": str(run_dir),
        "projects_total": total_projects,
        "projects_with_crate_dir": crates_found,
        "cargo_check_passed": cargo_ok,
        "cargo_check_pass_rate": (cargo_ok / total_projects) if total_projects else 0.0,
        "incremental_compilation": {
            "total_functions": inc_total,
            "passed_total": inc_ok,
            "pass_rate": (inc_ok / inc_total) if inc_total else 0.0,
            "source": "translation_summary_report.json or translation_stats.json",
        },
    }

    if run_clippy:
        summary["clippy_summary"] = {
            "clippy_warnings": total_clippy,
            "rustc_warnings": total_rustc,
            "warnings_total": total_warn,
            "errors_total": total_err,
        }

    if analyze_unsafe:
        summary["unsafe_summary"] = {
            "projects_analyzed": unsafe_projects,
            "code_lines": total_code_lines,
            "unsafe_total_lines": total_unsafe_total_lines,
            "unsafe_items": total_unsafe_items,
            "unsafe_total_ratio": (total_unsafe_total_lines / total_code_lines) if total_code_lines else 0.0,
        }

    if run_ohos_tests:
        summary["ohos_unit_tests_summary"] = {
            "projects_attempted": ohos_attempted,
            "projects_compiled": ohos_compiled,
            "compile_rate": (ohos_compiled / ohos_attempted) if ohos_attempted else 0.0,
            "projects_skipped_qemu": ohos_skipped_qemu,
            "projects_with_host_tests": ohos_host_projects,
            "tests_total": ohos_tests_total,
            "tests_passed": ohos_tests_passed,
            "tests_failed": ohos_tests_failed,
            "pass_rate": (ohos_tests_passed / ohos_tests_total) if ohos_tests_total else 0.0,
        }

    if run_derived_tests:
        summary["derived_rust_tests_summary"] = {
            "projects_executed": derived_projects,
            "tests_generated": derived_tests_generated,
            "tests_passed": derived_tests_passed,
            "tests_failed": derived_tests_failed,
            "pass_rate": (derived_tests_passed / derived_tests_generated) if derived_tests_generated else 0.0,
        }

    report = {"projects": projects, "summary": summary}

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    # ---------------- stdout (brief, paper-friendly) ----------------
    print(f"分析运行目录: {run_dir}")
    print(f"输出 JSON: {out_path}")
    print("\n======================================================================")
    print("我们的办法 - OHOS(10) 分析（真实指标）")
    print("======================================================================")
    print(f"项目数(目标): {total_projects}")
    print(f"Rust 项目目录存在: {crates_found}/{total_projects}")
    print(f"cargo check 通过: {cargo_ok}/{total_projects} ({summary['cargo_check_pass_rate']*100:.1f}%)")
    inc_rate = summary["incremental_compilation"]["pass_rate"]
    print(f"增量编译通过率(来自运行统计): {inc_ok}/{inc_total} ({inc_rate*100:.2f}%)")

    if run_clippy and "clippy_summary" in summary:
        cs = summary["clippy_summary"]
        print(
            f"警告统计: clippy={cs['clippy_warnings']} rustc={cs['rustc_warnings']} "
            f"总计={cs['warnings_total']} 错误={cs['errors_total']}"
        )
    if analyze_unsafe and "unsafe_summary" in summary:
        us = summary["unsafe_summary"]
        print(
            f"unsafe: {us['unsafe_total_lines']}/{us['code_lines']} ({us['unsafe_total_ratio']*100:.2f}%), "
            f"items={us['unsafe_items']}"
        )
    if run_ohos_tests and "ohos_unit_tests_summary" in summary:
        ts = summary["ohos_unit_tests_summary"]
        print(
            f"OHOS gtest: 编译成功 {ts['projects_compiled']}/{ts['projects_attempted']} ({ts['compile_rate']*100:.1f}%), "
            f"通过 {ts['tests_passed']}/{ts['tests_total']} ({ts['pass_rate']*100:.1f}%)"
        )
        if ts.get("projects_skipped_qemu"):
            print(f"OHOS gtest: 需 QEMU/真机环境的项目数（主机侧跳过）= {ts['projects_skipped_qemu']}")
    if run_derived_tests and "derived_rust_tests_summary" in summary:
        ds = summary["derived_rust_tests_summary"]
        print(
            f"派生 Rust 单测(纯逻辑子集): 用例数 {ds['tests_generated']}, "
            f"通过 {ds['tests_passed']}/{ds['tests_generated']} ({ds['pass_rate']*100:.1f}%), "
            f"失败 {ds['tests_failed']}"
        )

    header = f"{'项目':<24} {'cargo':<6} {'警告':<6} {'错误':<6} {'unsafe%':<8} {'增量%':<8} {'OHOS测试':<18} {'派生Rust':<12}"
    print("\n" + header)
    print("-" * len(header))
    for name, pr in iter_projects_in_display_order(projects):
        cargo = "OK" if pr.get("cargo_check", {}).get("passed") else "FAIL"
        warn = "-"
        err = "-"
        if run_clippy:
            cl = pr.get("clippy") or {}
            warn = str(int(cl.get("warning_count_total") or 0))
            err = str(int(cl.get("error_count") or 0))
        unsafe = "-"
        if analyze_unsafe:
            ua = pr.get("unsafe_analysis") or {}
            if not ua.get("error") and ua.get("code_lines"):
                unsafe = f"{float(ua.get('unsafe_total_ratio', 0.0) or 0.0)*100:.1f}"
            else:
                unsafe = "ERR"
        inc = "-"
        inc_obj = pr.get("incremental_compilation") or {}
        if inc_obj.get("total_functions"):
            inc = f"{float(inc_obj.get('pass_rate', 0.0) or 0.0)*100:.1f}"
        ohos = "-"
        if run_ohos_tests:
            ut = pr.get("ohos_unit_tests") or {}
            if ut.get("skipped") and ut.get("needs_qemu"):
                ohos = "跳过(QEMU)"
            elif ut.get("compiled") and ut.get("total_tests") is not None:
                ohos = f"{ut.get('tests_passed', 0)}/{ut.get('total_tests', 0)}"
            else:
                ohos = "失败/不可用"
        drs = "-"
        if run_derived_tests:
            dr = pr.get("derived_rust_tests") or {}
            if dr.get("generated_tests"):
                drs = f"{dr.get('passed', 0)}/{dr.get('generated_tests', 0)}"
            elif dr.get("error"):
                drs = "ERR"
        print(f"{name:<24} {cargo:<6} {warn:<6} {err:<6} {unsafe:<8} {inc:<8} {ohos:<18} {drs:<12}")

    print("======================================================================")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
