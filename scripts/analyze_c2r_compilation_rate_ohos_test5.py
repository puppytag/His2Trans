#!/usr/bin/env python3
"""
OHOS(test5) evaluation for *our framework* outputs.

This script is intentionally separate from:
- `analyze_c2r_compilation_rate.py` (test_module 10 projects), and
- `analyze_c2r_compilation_rate_ohos10.py` (OHOS 10 projects),
so we can evaluate the **testable OHOS 5-project subset** without polluting other workflows.

Goals (paper-friendly, no cheating):
- Do NOT modify translation outputs on disk.
- Compute incremental compilation pass rate (prefer real per-function verification via stubbing).
- Run *real* Clippy (JSON) and count Clippy vs Rustc warnings.
- Compute unsafe rate on translated Rust sources.
- Reuse the *original OHOS gtest unit tests* to evaluate correctness (best-effort).
- Optionally extract a conservative pure-logic subset from OHOS gtest and run as Rust unit tests
  (includes signature matching / adaptation, executed on a temp copy).

Typical usage:
  cd /data/home/wangshb/c2-rust_framework/scripts/analysis
  python3 analyze_c2r_compilation_rate_ohos_test5.py \\
    --run-dir /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5 \\
    --all --run-derived-tests
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
from collections import Counter
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# =============================================================================
# Project order (paper order; see paper_tables_postprocess/tables_data.py)
# =============================================================================

# Internal -> paper display name.
PAPER_PROJECT_NAME: Dict[str, str] = {
    "osal__0bc4f21396ad": "osal",
    "shared__12e38ea922f7": "uhdf2_shared",
    "shared__541f4e547bdb": "core_shared",
    "host__25c1898e1626": "host",
    "appverify_lite__e5ebe91a98b9": "appverify_lite",
}

# Paper row order for RQ1 (OHOS test5 full 5 projects): osal, uhdf2_shared, core_shared, host, appverify_lite.
DISPLAY_PROJECT_ORDER: List[str] = [
    "osal__0bc4f21396ad",
    "shared__12e38ea922f7",
    "shared__541f4e547bdb",
    "host__25c1898e1626",
    "appverify_lite__e5ebe91a98b9",
]

PAPER_RQ_PROJECT_ORDER: Dict[str, List[str]] = {
    # RQ1 uses all 5 projects in the paper order.
    "rq1": list(DISPLAY_PROJECT_ORDER),
    # RQ3 uses the 3 shared-core projects in the same paper order prefix.
    "rq3": [
        "osal__0bc4f21396ad",
        "shared__12e38ea922f7",
        "shared__541f4e547bdb",
    ],
    # RQ4 uses a different order in the paper: appverify_lite, host.
    "rq4": [
        "appverify_lite__e5ebe91a98b9",
        "host__25c1898e1626",
    ],
}
_ORDER_INDEX = {n: i for i, n in enumerate(DISPLAY_PROJECT_ORDER)}


def iter_projects_in_display_order(projects: Dict[str, Any]) -> List[Tuple[str, Any]]:
    return sorted(projects.items(), key=lambda kv: (_ORDER_INDEX.get(kv[0], 1_000_000), kv[0]))


# =============================================================================
# Paths
# =============================================================================

_HIS2TRANS_ROOT = Path(__file__).resolve().parents[1]

# NOTE: For open-sourcing, defaults must be self-contained under this repository.
DEFAULT_HUAWEI_PROJECTS_TSV = _HIS2TRANS_ROOT / "data" / "ohos" / "huawei_projects.tsv"
DEFAULT_OHOS_ROOT = _HIS2TRANS_ROOT / "data" / "ohos" / "ohos_root_min"


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
        p = Path(path).expanduser()
        # Allow relative paths in the TSV for open-source portability.
        if not p.is_absolute():
            p = (tsv_path.parent / p)
        mapping[name] = p.resolve()
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

    with tempfile.TemporaryDirectory(prefix="our_ohos_test5_cargo_target_") as td:
        env = {
            **os.environ,
            "CARGO_TARGET_DIR": td,
            "RUSTFLAGS": "-Awarnings",
            "RUST_BACKTRACE": "0",
        }
        rc, out, err = _run_cmd_capture(["cargo", "check", "--offline"], cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["passed"] = rc == 0
        res["stdout"] = out[-8000:]
        res["stderr"] = err[-12000:]
        if rc != 0:
            res["error"] = "cargo check failed"
    return res


def run_cargo_clippy(project_dir: Path, timeout: int = 600) -> Dict[str, Any]:
    res: Dict[str, Any] = {
        "executed": False,
        "warning_count": 0,
        "rustc_warning_count": 0,
        "warning_count_total": 0,
        "clippy_warning_codes": {},
        "rustc_warning_codes": {},
        "error_count": 0,
        "error": None,
        "stdout": "",
        "stderr": "",
    }
    if not (project_dir / "Cargo.toml").is_file():
        res["error"] = "Cargo.toml not found"
        return res

    with tempfile.TemporaryDirectory(prefix="our_ohos_test5_clippy_target_") as td:
        env = {**os.environ, "CARGO_TARGET_DIR": td, "RUST_BACKTRACE": "0"}
        cmd = ["cargo", "clippy", "--offline", "--message-format=json", "--", "-W", "clippy::all"]
        rc, out, err = _run_cmd_capture(cmd, cwd=project_dir, env=env, timeout=timeout)
        res["executed"] = True
        res["stdout"] = out[-200000:]
        res["stderr"] = err[-200000:]

        clippy_warn = 0
        rustc_warn = 0
        err_cnt = 0
        clippy_codes: Dict[str, int] = {}
        rustc_codes: Dict[str, int] = {}
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
            code = (code_obj.get("code") or "") or "general"
            if level == "warning":
                if code.startswith("clippy::"):
                    clippy_warn += 1
                    clippy_codes[code] = clippy_codes.get(code, 0) + 1
                else:
                    rustc_warn += 1
                    rustc_codes[str(code)] = rustc_codes.get(str(code), 0) + 1
            elif level == "error":
                err_cnt += 1

        res["warning_count"] = clippy_warn
        res["rustc_warning_count"] = rustc_warn
        res["warning_count_total"] = clippy_warn + rustc_warn
        res["clippy_warning_codes"] = dict(sorted(clippy_codes.items(), key=lambda kv: (-kv[1], kv[0])))
        res["rustc_warning_codes"] = dict(sorted(rustc_codes.items(), key=lambda kv: (-kv[1], kv[0])))
        res["error_count"] = err_cnt
        if rc != 0 and err_cnt > 0:
            res["error"] = "cargo clippy failed"
    return res


# =============================================================================
# Unsafe analysis (reuse from analyze_c2r_compilation_rate.py)
# =============================================================================

def _import_base_unsafe_analyzer():
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    try:
        import analyze_c2r_compilation_rate as base  # type: ignore
        return base
    except Exception:
        return None


# =============================================================================
# Incremental compilation stats (from our outputs)
# =============================================================================

def load_incremental_stats_from_summary(run_dir: Path) -> Dict[str, Any]:
    """
    Load our per-project incremental compilation stats from:
      results/translation_summary_report.json
    or, as fallback:
      intermediate/<proj>/**/translation_stats.json
    """
    out: Dict[str, Any] = {}

    summary = run_dir / "results" / "translation_summary_report.json"
    if summary.is_file():
        try:
            obj = json.loads(summary.read_text(encoding="utf-8", errors="replace"))

            # Format A (older): {"projects": {"name": {...}}}
            projects = obj.get("projects") or {}
            if isinstance(projects, dict) and projects:
                for name, p in projects.items():
                    if name not in DISPLAY_PROJECT_ORDER:
                        continue
                    out[name] = {
                        "total_functions": int(p.get("total_functions") or 0),
                        "passed_total": int(p.get("compiled_total") or 0),
                        "passed_directly": int(p.get("compiled_direct") or 0),
                        "passed_after_repair": int(p.get("compiled_after_repair") or 0),
                        "failed_reverted": int(p.get("failed_reverted") or 0),
                        "injection_failed": int(p.get("injection_failed") or 0),
                        "pass_rate": float(p.get("pass_rate") or 0.0),
                        "source": "translation_summary_report.json:projects",
                    }
                return out

            # Format B (current): {"projects_detail": [{"project": "...", ...}, ...]}
            detail = obj.get("projects_detail") or []
            if isinstance(detail, list) and detail:
                for p in detail:
                    if not isinstance(p, dict):
                        continue
                    name = str(p.get("project") or "")
                    if name not in DISPLAY_PROJECT_ORDER:
                        continue
                    out[name] = {
                        "total_functions": int(p.get("total_functions") or 0),
                        "translated": int(p.get("translated") or 0),
                        "passed_total": int(p.get("passed_total") or 0),
                        "passed_directly": int(p.get("passed_directly") or 0),
                        "passed_after_repair": int(p.get("passed_after_repair") or 0),
                        "failed_reverted": int(p.get("failed_reverted") or 0),
                        "injection_failed": int(p.get("injection_failed") or 0),
                        "pass_rate": float(p.get("pass_rate_percent") or 0.0) / 100.0,
                        "source": "translation_summary_report.json:projects_detail",
                        "stats_file": p.get("stats_file"),
                    }
                return out
        except Exception:
            pass

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
            "translated": int(s.get("translated") or 0),
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


def scan_repair_rounds_from_run_dir(run_dir: Path, project_name: str) -> Dict[str, Any]:
    """
    Compute repair rounds for a project by scanning attempt_*_error.txt files.

    Layout (our framework):
      <run_dir>/intermediate/<proj>/workspace/repair_history/<proj>/translate_by_*/<func>/attempt_<k>_error.txt
    """
    repair_base = run_dir / "intermediate" / project_name / "workspace" / "repair_history" / project_name
    if not repair_base.is_dir():
        return {"executed": False, "reason": "repair_history_missing", "repair_base": str(repair_base)}

    translate_dirs = sorted([p for p in repair_base.glob("translate_by_*") if p.is_dir()])
    if not translate_dirs:
        return {"executed": False, "reason": "no_translate_by_dir", "repair_base": str(repair_base)}

    # Best-effort: pick the first translate_by_* dir (runs normally have only one).
    repair_root = translate_dirs[0]

    funcs_in_repair_dir = 0
    funcs_with_attempts = 0
    attempts_total = 0
    hist: Counter[int] = Counter()

    for func_dir in sorted(repair_root.iterdir()):
        if not func_dir.is_dir() or func_dir.name.startswith("_"):
            continue
        funcs_in_repair_dir += 1
        n = len(list(func_dir.glob("attempt_*_error.txt")))
        hist[n] += 1
        attempts_total += n
        if n > 0:
            funcs_with_attempts += 1

    return {
        "executed": True,
        "repair_root": str(repair_root),
        "functions_in_repair_dir": funcs_in_repair_dir,
        "functions_with_attempts": funcs_with_attempts,
        "repair_attempts_total": attempts_total,
        "attempts_histogram": {str(k): int(v) for k, v in sorted(hist.items(), key=lambda x: x[0])},
    }


# =============================================================================
# Locate translated crate dirs
# =============================================================================

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
            candidates = [p.parent for p in base.rglob("Cargo.toml") if p.is_file()]
        if candidates:
            return sorted(candidates, key=lambda p: p.name)[0]

    sk = run_dir / "intermediate" / project_name / "workspace" / "skeletons" / project_name
    if (sk / "Cargo.toml").is_file():
        return sk
    return None


# =============================================================================
# Reuse shared OHOS gtest + derived-test harness (no touching translation outputs)
# =============================================================================

# Make sure we import the harness from *his2trans/scripts/analysis* (not the parent repo).
if str(_HIS2TRANS_ROOT) not in sys.path:
    sys.path.insert(0, str(_HIS2TRANS_ROOT))
try:
    from scripts.analysis import analyze_c2r_compilation_rate_ohos10 as ohos_harness  # type: ignore
except Exception:
    ohos_harness = None

try:
    from scripts.analysis.ohos_test5_incremental import (  # type: ignore
        load_standard_function_names as load_standard_function_names_ohos5,
        verify_incremental_compilation_standard as verify_incremental_compilation_standard_ohos5,
        get_expected_test_count as get_expected_test_count_ohos5,
        STANDARD_TEST_COUNTS,
        STANDARD_TEST_TOTAL,
    )
except Exception:
    load_standard_function_names_ohos5 = None
    verify_incremental_compilation_standard_ohos5 = None
    get_expected_test_count_ohos5 = None
    STANDARD_TEST_COUNTS = {}
    STANDARD_TEST_TOTAL = 0


def main() -> int:
    parser = argparse.ArgumentParser(description="分析我们的框架 OHOS(test5) 运行结果（真实指标，不改翻译结果）。")
    parser.add_argument("--run-dir", type=Path, required=True, help="运行目录（例如 translation_outputs/deepseek-coder-ohos5）")
    parser.add_argument(
        "--paper-rq",
        choices=["rq1", "rq3", "rq4"],
        default=None,
        help="指定论文 RQ 以控制终端仅输出论文指标（默认会根据 --run-dir 自动推断）。",
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
    parser.add_argument(
        "--verify-incremental",
        action="store_true",
        help="验证增量编译成功率（逐个函数 stub->restore 后 cargo check；比读取运行产物更真实，但更慢）。",
    )
    parser.add_argument(
        "--verify-incremental-timeout",
        type=int,
        default=30,
        help="--verify-incremental: 单个函数 cargo check 超时（秒）",
    )
    parser.add_argument(
        "--standard-run-dir",
        type=Path,
        default=None,
        help=(
            "Canonical standard-function source (our framework run dir). "
            "If provided, use <dir>/intermediate/<proj>/workspace/extracted/<proj>/functions_manifest.json "
            "as the denominator for ICompRate. Defaults to the current --run-dir when available."
        ),
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

    # Decide which paper-RQ formatting/metric conventions to use as early as possible.
    # (Some RQs use a different CR definition; see tables_data.py.)
    def _infer_paper_rq(p: Path) -> str:
        s = str(p).replace("\\", "/")
        if "/rq3/" in s:
            return "rq3"
        if "/rq4/" in s:
            return "rq4"
        return "rq1"

    paper_rq = args.paper_rq or _infer_paper_rq(run_dir)

    # RQ3 (ablation) CR in the paper uses the per-function ICompRate metric where
    # C2Rust-fallback/unimplemented bodies are treated as failures. We cache these
    # results in `paper_compilation_analysis_ohos_test5.json` to avoid re-running
    # hundreds of cargo checks during table regeneration.
    def _load_cached_icomp_rate(run_dir: Path) -> Dict[str, Dict[str, Any]]:
        candidates = [
            run_dir / "results" / "paper_compilation_analysis_ohos_test5.json",
            run_dir / "results" / "compilation_analysis_ohos_test5.json",
        ]
        for p in candidates:
            if not p.is_file():
                continue
            try:
                obj = json.loads(p.read_text(encoding="utf-8", errors="replace"))
            except Exception:
                continue
            projects = obj.get("projects")
            if not isinstance(projects, dict):
                continue
            out: Dict[str, Dict[str, Any]] = {}
            for name, pr in projects.items():
                if not isinstance(pr, dict):
                    continue
                inc = pr.get("incremental_compilation")
                if isinstance(inc, dict):
                    out[str(name)] = inc
            if out:
                return out
        return {}

    cached_icomp = _load_cached_icomp_rate(run_dir) if paper_rq == "rq3" else {}

    standard_run_dir = args.standard_run_dir or run_dir

    run_clippy = args.run_clippy or args.all
    analyze_unsafe = args.analyze_unsafe or args.all
    run_ohos_tests = args.run_ohos_tests or args.all
    run_derived_tests = args.run_derived_tests
    verify_incremental = args.verify_incremental

    huawei_map: Dict[str, Path] = load_huawei_projects_map(args.huawei_projects_tsv) if run_ohos_tests else {}
    ohos_root = args.ohos_root

    base = _import_base_unsafe_analyzer()
    inc_map = load_incremental_stats_from_summary(run_dir)  # framework artifacts (for repair stats, etc.)

    projects: Dict[str, Any] = {}

    total_projects = len(DISPLAY_PROJECT_ORDER)
    crates_found = 0
    cargo_ok = 0
    total_clippy = total_rustc = total_warn = total_err = 0
    unsafe_projects = 0
    total_code_lines = total_unsafe_total_lines = total_unsafe_items = 0
    # Incremental compilation summary (verified or artifact-based depending on --verify-incremental).
    inc_total = inc_ok = 0
    # Framework incremental stats totals (always artifact-based; used for repair rounds summary).
    inc_art_total = inc_art_ok = 0
    translated_total = 0
    repair_attempts_total = 0
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
        # Canonical denominator: standard function set extracted from the C sources (our framework).
        std_names: List[str] = []
        if verify_incremental and load_standard_function_names_ohos5 is not None:
            try:
                std_names = load_standard_function_names_ohos5(project_name, standard_run_dir)
            except Exception:
                std_names = []

        crate_dir = find_project_crate_dir(run_dir, project_name)
        proj_res: Dict[str, Any] = {"project_dir": str(crate_dir) if crate_dir else None}
        if not crate_dir:
            proj_res["error"] = "未找到项目目录（final_projects 或 skeletons）"
            if verify_incremental:
                proj_res["incremental_compilation"] = {
                    "total_functions": len(std_names),
                    "compiled_functions": 0,
                    "compile_rate": 0.0,
                    "error": "crate_dir not found",
                }
                inc_total += len(std_names)
            projects[project_name] = proj_res
            continue
        crates_found += 1

        # Framework incremental stats (from run artifacts). We keep these for repair-round analysis.
        inc_fw = inc_map.get(project_name)
        if inc_fw:
            proj_res["framework_incremental_stats"] = inc_fw
            inc_art_total += int(inc_fw.get("total_functions") or 0)
            inc_art_ok += int(inc_fw.get("passed_total") or 0)
            translated_total += int(inc_fw.get("translated") or 0)

        # Incremental compilation rate:
        # - default: compatibility mode (use run artifacts)
        # - if --verify-incremental: run real per-function verification with a canonical denominator
        if verify_incremental:
            if verify_incremental_compilation_standard_ohos5 is None:
                inc_v = {
                    "total_functions": len(std_names),
                    "compiled_functions": 0,
                    "compile_rate": 0.0,
                    "error": "incremental verifier import failed (scripts.analysis.ohos_test5_incremental)",
                }
            else:
                try:
                    inc_v = verify_incremental_compilation_standard_ohos5(
                        crate_dir=crate_dir,
                        project_name=project_name,
                        standard_func_names=std_names,
                        timeout=int(args.verify_incremental_timeout),
                        count_sources={"llm"},
                        llm_success_names=None,
                        default_source="llm",
                    )
                except Exception as e:
                    inc_v = {
                        "total_functions": len(std_names),
                        "compiled_functions": 0,
                        "compile_rate": 0.0,
                        "error": f"verify_incremental_compilation_standard failed: {e}",
                    }
            proj_res["incremental_compilation"] = inc_v
            inc_total += int(inc_v.get("total_functions") or 0)
            inc_ok += int(inc_v.get("compiled_functions") or 0)
        else:
            # Default (artifact-based) CR:
            # - RQ1/RQ4: use framework stats pass_rate (includes fallback); this matches the paper.
            # - RQ3: use cached ICompRate (LLM-only; fallback counts as failure) from paper JSON if present.
            if cached_icomp:
                inc_cached = cached_icomp.get(project_name)
                if isinstance(inc_cached, dict) and inc_cached.get("compile_rate") is not None:
                    proj_res["incremental_compilation"] = inc_cached
                    inc_total += int(inc_cached.get("total_functions") or 0)
                    inc_ok += int(inc_cached.get("compiled_functions") or 0)
                elif inc_fw:
                    proj_res["incremental_compilation"] = inc_fw
                    inc_total += int(inc_fw.get("total_functions") or 0)
                    inc_ok += int(inc_fw.get("passed_total") or 0)
            elif inc_fw:
                proj_res["incremental_compilation"] = inc_fw
                inc_total += int(inc_fw.get("total_functions") or 0)
                inc_ok += int(inc_fw.get("passed_total") or 0)

        # Avg repair rounds (compile-guided repair iterations).
        rr = scan_repair_rounds_from_run_dir(run_dir, project_name)
        if rr.get("executed"):
            attempts = int(rr.get("repair_attempts_total") or 0)
            repair_attempts_total += attempts
            total_funcs = int((inc_fw or {}).get("total_functions") or 0)
            translated_funcs = int((inc_fw or {}).get("translated") or 0)
            rr["avg_rounds_per_total_function"] = (attempts / total_funcs) if total_funcs else 0.0
            rr["avg_rounds_per_translated_function"] = (attempts / translated_funcs) if translated_funcs else 0.0
        proj_res["repair_rounds"] = rr

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

        # OHOS gtest unit tests + derived pure-logic tests.
        if run_ohos_tests:
            src_dir = huawei_map.get(project_name)
            if not src_dir:
                proj_res["ohos_unit_tests"] = {"executed": False, "compiled": False, "error": "project not in huawei_projects.tsv"}
            elif ohos_harness is None:
                proj_res["ohos_unit_tests"] = {"executed": False, "compiled": False, "error": "ohos harness import failed"}
            else:
                # Split tests into:
                # - host-runnable subset (best-effort): does NOT require OHOS runtime/services/IPC/kernel nodes
                # - QEMU/device required: depends on HDF test service runtime / binder / /sys nodes, etc.
                tests_all, samples_all, err = ohos_harness._collect_ohos_test_sources(src_dir)  # type: ignore[attr-defined]
                if err:
                    proj_res["ohos_unit_tests"] = {"executed": False, "compiled": False, "error": err}
                else:
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
                    for ts in tests_all:
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

                    if host_tests:
                        ohos_attempted += 1
                        ohos_host_projects += 1

                        exportable = ohos_harness._collect_exportable_fn_names(crate_dir)  # type: ignore[attr-defined]
                        called = ohos_harness._collect_called_identifiers(host_tests)  # type: ignore[attr-defined]
                        export_names = (called & exportable) if exportable else set()
                        if not export_names:
                            export_names = None

                        sb = ohos_harness.build_staticlib_from_crate(  # type: ignore[attr-defined]
                            crate_dir, export_names=export_names, timeout=args.ohos_test_timeout
                        )
                        proj_res["staticlib_build"] = sb
                        if not sb.get("ok") or not sb.get("staticlib"):
                            proj_res["ohos_unit_tests"] = {
                                "executed": False,
                                "compiled": False,
                                "error": sb.get("error") or "staticlib build failed",
                            }
                        else:
                            gt = ohos_harness.run_ohos_unit_tests(  # type: ignore[attr-defined]
                                project_name=project_name,
                                source_project_dir=src_dir,
                                translated_staticlib=Path(sb["staticlib"]),
                                ohos_root=ohos_root,
                                test_srcs=host_tests,
                                sample_srcs=samples_all,
                                timeout=args.ohos_test_timeout,
                            )
                            gt["skipped_qemu_test_files"] = [str(p.relative_to(src_dir)) for p in qemu_tests]
                            proj_res["ohos_unit_tests"] = gt
                            if gt.get("compiled"):
                                ohos_compiled += 1

                        # Use unified denominator: for each project, expected_count comes from STANDARD_TEST_COUNTS.
                        # If tests didn't run, passed = 0, failed = expected_count.
                        expected_count = 0
                        if get_expected_test_count_ohos5 is not None:
                            expected_count = get_expected_test_count_ohos5(project_name)
                        else:
                            expected_count = STANDARD_TEST_COUNTS.get(project_name, 0)
                        gt_res = proj_res.get("ohos_unit_tests", {})
                        actual_passed = int(gt_res.get("tests_passed", 0) or 0)
                        ohos_tests_passed += actual_passed
                        ohos_tests_failed += (expected_count - actual_passed)
                        gt_res["expected_tests"] = expected_count
                        gt_res["unified_tests_failed"] = expected_count - actual_passed
                    else:
                        ohos_skipped_qemu += 1
                        proj_res["ohos_unit_tests"] = {
                            "executed": False,
                            "compiled": False,
                            "skipped": True,
                            "needs_qemu": True,
                            "error": "该项目的 gtest 依赖 OHOS 运行时/服务/IPC/内核节点（建议在 OHOS QEMU/真机环境跑），已跳过主机侧执行。",
                            "qemu_test_files": [str(p.relative_to(src_dir)) for p in qemu_tests],
                        }

                    # Derived Rust unit tests (pure-logic subset extracted from gtest; includes signature matching).
                    if run_derived_tests:
                        cases = ohos_harness._extract_pure_logic_cases_from_gtest(tests_all)  # type: ignore[attr-defined]
                        dr = ohos_harness.run_derived_rust_tests_from_gtest(  # type: ignore[attr-defined]
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
        "total_projects": total_projects,
        "crates_found": crates_found,
        "cargo_check_passed_projects": cargo_ok,
        "cargo_check_pass_rate": (cargo_ok / crates_found) if crates_found else 0.0,
        "incremental_compilation_summary": (
            {
                "method": "verify_incremental_compilation",
                "total_functions": inc_total,
                "compiled_functions": inc_ok,
                "compile_rate": (inc_ok / inc_total) if inc_total else 0.0,
            }
            if verify_incremental
            else {
                "method": "framework_run_artifacts",
                "total_functions": inc_total,
                "passed_total": inc_ok,
                "pass_rate": (inc_ok / inc_total) if inc_total else 0.0,
            }
        ),
        "repair_rounds_summary": {
            "repair_attempts_total": repair_attempts_total,
            # AvgRepair: average repair iterations per function (direct-pass functions contribute 0).
            "avg_rounds_per_total_function": (repair_attempts_total / inc_art_total) if inc_art_total else 0.0,
            "avg_rounds_per_translated_function": (repair_attempts_total / translated_total) if translated_total else 0.0,
        },
    }
    if run_clippy:
        summary["clippy_summary"] = {
            "total_clippy_warnings": total_clippy,
            "total_rustc_warnings": total_rustc,
            "total_warnings_including_rustc": total_warn,
            "total_errors": total_err,
        }
    if analyze_unsafe:
        summary["unsafe_summary"] = {
            "projects_analyzed": unsafe_projects,
            "total_code_lines": total_code_lines,
            "total_unsafe_total_lines": total_unsafe_total_lines,
            "unsafe_total_ratio": (total_unsafe_total_lines / total_code_lines) if total_code_lines else 0.0,
            "total_unsafe_items": total_unsafe_items,
        }
    if run_ohos_tests:
        # Use STANDARD_TEST_TOTAL (40) as the unified denominator for fair cross-method comparison.
        unified_total = STANDARD_TEST_TOTAL if STANDARD_TEST_TOTAL > 0 else (ohos_tests_passed + ohos_tests_failed)
        summary["ohos_unit_test_summary"] = {
            "projects_attempted": ohos_attempted,
            "projects_compiled": ohos_compiled,
            "compile_success_rate": (ohos_compiled / ohos_attempted) if ohos_attempted else 0.0,
            "host_projects": ohos_host_projects,
            "qemu_skipped_projects": ohos_skipped_qemu,
            "total_tests": unified_total,
            "tests_passed": ohos_tests_passed,
            "tests_failed": ohos_tests_failed,
            "overall_test_pass_rate": (ohos_tests_passed / unified_total) if unified_total else 0.0,
        }
    if run_derived_tests:
        summary["derived_pure_logic_summary"] = {
            "projects_executed": derived_projects,
            "generated_tests": derived_tests_generated,
            "tests_passed": derived_tests_passed,
            "tests_failed": derived_tests_failed,
        }

    result: Dict[str, Any] = {
        "run_dir": str(run_dir),
        "suite": "ohos_test5",
        "projects": projects,
        "summary": summary,
    }

    def _f2(x: Optional[float]) -> str:
        if x is None:
            return "-"
        return f"{x:.2f}"

    def _get_cr_percent(pr: Dict[str, Any]) -> Optional[float]:
        inc = pr.get("incremental_compilation")
        if not isinstance(inc, dict):
            return None
        rate = inc.get("pass_rate")
        if rate is None:
            rate = inc.get("compile_rate")
        if rate is None:
            total = int(inc.get("total_functions", 0) or 0)
            ok = int(inc.get("passed_total", inc.get("compiled_functions", 0)) or 0)
            if total <= 0:
                return None
            rate = ok / total
        try:
            return float(rate) * 100.0
        except Exception:
            return None

    def _get_fc_percent(pr: Dict[str, Any]) -> Optional[float]:
        ut = pr.get("ohos_unit_tests")
        if not isinstance(ut, dict):
            return None
        # If explicitly skipped because it needs QEMU/device, treat as missing.
        if bool(ut.get("skipped")) and bool(ut.get("needs_qemu")):
            return None
        passed = int(ut.get("tests_passed", 0) or 0)
        denom_raw = ut.get("expected_tests") or ut.get("expected_total_tests") or ut.get("total_tests")
        try:
            denom = int(denom_raw or 0)
        except Exception:
            denom = 0
        if denom > 0:
            return (passed / denom) * 100.0
        # If build/run failed but we attempted on host, treat as 0% (paper convention).
        if ut.get("error") or (ut.get("compiled") is False):
            return 0.0
        return None

    def _get_unsafe_percent(pr: Dict[str, Any]) -> Optional[float]:
        ua = pr.get("unsafe_analysis")
        if not isinstance(ua, dict) or ua.get("error"):
            return None
        ratio = ua.get("unsafe_total_ratio")
        if ratio is None:
            code = int(ua.get("code_lines", 0) or 0)
            unsafe_lines = int(ua.get("unsafe_total_lines", 0) or 0)
            if code <= 0:
                return None
            ratio = unsafe_lines / code
        try:
            return float(ratio) * 100.0
        except Exception:
            return None

    def _get_clippy_total(pr: Dict[str, Any]) -> Optional[int]:
        cl = pr.get("clippy")
        if not isinstance(cl, dict) or not bool(cl.get("executed")):
            return None
        try:
            return int(cl.get("warning_count_total", 0) or 0)
        except Exception:
            return None

    def _get_avg_repair(pr: Dict[str, Any]) -> Optional[float]:
        rr = pr.get("repair_rounds")
        if not isinstance(rr, dict) or not bool(rr.get("executed")):
            return None
        try:
            return float(rr.get("avg_rounds_per_total_function"))
        except Exception:
            return None

    proj_order = PAPER_RQ_PROJECT_ORDER.get(paper_rq, DISPLAY_PROJECT_ORDER)

    if paper_rq == "rq1":
        # Paper metrics (tables_data.py): CR, FC, Unsafe, Clippy
        print("\t".join(["project", "CR", "FC", "Unsafe", "Clippy"]))
        for internal in proj_order:
            pr = projects.get(internal, {}) or {}
            name = PAPER_PROJECT_NAME.get(internal, internal)
            cr = _get_cr_percent(pr)
            fc = _get_fc_percent(pr)
            unsafe = _get_unsafe_percent(pr)
            clippy = _get_clippy_total(pr)
            print(
                "\t".join(
                    [
                        name,
                        _f2(cr),
                        _f2(fc),
                        _f2(unsafe),
                        "-" if clippy is None else str(clippy),
                    ]
                )
            )
    else:
        # Paper metrics (tables_data.py): CR, FC, AvgRepair
        print("\t".join(["project", "CR", "FC", "AvgRepair"]))
        for internal in proj_order:
            pr = projects.get(internal, {}) or {}
            name = PAPER_PROJECT_NAME.get(internal, internal)
            cr = _get_cr_percent(pr)
            fc = _get_fc_percent(pr)
            ar = _get_avg_repair(pr)
            print("\t".join([name, _f2(cr), _f2(fc), _f2(ar)]))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
