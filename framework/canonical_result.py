"""生成当前框架使用的最小 canonical 结果。"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


COMPILE_SCHEMA_VERSION = "c2r_compile_result_v1"
POST_REPAIR_SCHEMA_VERSION = "c2r_canonical_post_repair_result_v1"
COMPILE_SUCCESS_DEFINITION = "compile_ok"
POST_REPAIR_SUCCESS_DEFINITION = "accepted_by_gates"


def _int_value(value: Any) -> int:
    """把统计值规整为非负整数。"""
    try:
        return max(0, int(value or 0))
    except (TypeError, ValueError):
        return 0


def build_compile_result(
    *,
    run_id: str,
    stats: dict[str, Any],
    work_dir: Path,
    final_project_dir: Path | None = None,
    final_compile_ok: bool | None = None,
) -> dict[str, Any]:
    """根据当前内部编译门控生成最小 canonical result。"""
    total = _int_value(stats.get("total"))
    compiled = _int_value(stats.get("compiled"))
    failed = _int_value(stats.get("failed"))
    skipped = _int_value(stats.get("skipped"))
    injection_failed = _int_value(stats.get("injection_failed"))
    still_unimplemented = _int_value(stats.get("still_unimplemented"))
    if final_compile_ok is None:
        compile_ok = (
            total > 0
            and compiled == total
            and failed == 0
            and skipped == 0
            and injection_failed == 0
        )
    else:
        compile_ok = bool(final_compile_ok)

    artifacts = {"work_dir": str(Path(work_dir))}
    if final_project_dir is not None:
        artifacts["final_project_dir"] = str(Path(final_project_dir))

    return {
        "schema_version": COMPILE_SCHEMA_VERSION,
        "run_id": run_id,
        "final_status": "accepted" if compile_ok else "rejected",
        "success_definition": COMPILE_SUCCESS_DEFINITION,
        "compile": {
            "compile_ok": compile_ok,
            "total": total,
            "compiled": compiled,
            "failed": failed,
            "skipped": skipped,
            "injection_failed": injection_failed,
            "still_unimplemented": still_unimplemented,
        },
        "fallback": {
            "c2rust_used": _int_value(stats.get("c2rust_fallback")),
        },
        "artifacts": artifacts,
    }


def _latest_round(summary: dict[str, Any]) -> dict[str, Any]:
    """取 post repair summary 的最后一轮记录。"""
    rounds = summary.get("rounds")
    if not isinstance(rounds, list) or not rounds:
        return {}
    latest = rounds[-1]
    return latest if isinstance(latest, dict) else {}


def _gate_bool(statuses: dict[str, Any], name: str) -> bool:
    """把 gate status 规整为布尔通过值。"""
    return str(statuses.get(name, "")).strip() in {"accepted", "not_configured"}


def build_post_repair_result(
    *,
    run_id: str,
    post_repair_summary: dict[str, Any],
    work_dir: Path,
    final_project_dir: Path | None = None,
    post_repair_summary_path: Path | None = None,
) -> dict[str, Any]:
    """根据后置 repair gate summary 生成最小 canonical result。"""
    latest = _latest_round(post_repair_summary)
    final_verify_summary = post_repair_summary.get("final_verify_summary")
    gate_summary = final_verify_summary if isinstance(final_verify_summary, dict) else {}
    if not gate_summary:
        gate_summary = latest.get("blocking_summary") if isinstance(latest.get("blocking_summary"), dict) else {}
    gate_statuses = gate_summary.get("gate_statuses") if isinstance(gate_summary.get("gate_statuses"), dict) else {}

    cargo_ok = _gate_bool(gate_statuses, "cargo")
    clippy_status = str(gate_statuses.get("cargo_clippy", "not_configured")).strip() or "not_configured"
    clippy_ok = clippy_status in {"accepted", "not_configured"}
    ohos_ok = _gate_bool(gate_statuses, "ohos_rustc")
    semantic_status = str(gate_statuses.get("semantic_audit", "not_configured")).strip() or "not_configured"
    semantic_audit_ok = semantic_status in {"accepted", "not_configured"}
    cheap_gates_ok = bool(gate_summary.get("cheap_gates_passed")) if gate_summary else cargo_ok and clippy_ok and ohos_ok
    accepted_by_gates = bool(gate_summary.get("accepted_by_gates")) and semantic_audit_ok

    final_status = str(post_repair_summary.get("final_status") or ("accepted" if accepted_by_gates else "rejected"))
    if final_status == "accepted" and not accepted_by_gates:
        final_status = "rejected"

    artifacts = {"work_dir": str(Path(work_dir))}
    if final_project_dir is not None:
        artifacts["final_project_dir"] = str(Path(final_project_dir))
    if post_repair_summary_path is not None:
        artifacts["post_repair_summary"] = str(Path(post_repair_summary_path))
    gate_bundle = latest.get("gate_bundle")
    if gate_bundle:
        artifacts["latest_gate_bundle"] = str(gate_bundle)
    if isinstance(final_verify_summary, dict):
        final_gate_bundle = final_verify_summary.get("final_gate_bundle")
        source_fingerprint = final_verify_summary.get("source_fingerprint")
        final_verify_dir = final_verify_summary.get("final_verify_dir")
        if final_gate_bundle:
            artifacts["final_gate_bundle"] = str(final_gate_bundle)
        if source_fingerprint:
            artifacts["source_fingerprint"] = str(source_fingerprint)
        if final_verify_dir:
            artifacts["final_verify_dir"] = str(final_verify_dir)

    return {
        "schema_version": POST_REPAIR_SCHEMA_VERSION,
        "run_id": run_id,
        "final_status": final_status,
        "success_definition": POST_REPAIR_SUCCESS_DEFINITION,
        "compile": {
            "compile_ok": cheap_gates_ok,
            "cargo_ok": cargo_ok,
            "cargo_clippy_ok": clippy_ok,
            "ohos_rustc_ok": ohos_ok,
        },
        "post_repair": {
            "accepted_by_gates": accepted_by_gates,
            "accepted_round": post_repair_summary.get("accepted_round"),
            "rounds": len(post_repair_summary.get("rounds") or []),
            "semantic_audit_ok": semantic_audit_ok,
            "semantic_audit_status": semantic_status,
            "blocking_gates": gate_summary.get("blocking_gates") or [],
            "pending_gates": gate_summary.get("pending_gates") or [],
            "final_gate_bundle": str(gate_summary.get("final_gate_bundle") or ""),
            "source_fingerprint_sha256": str(gate_summary.get("source_fingerprint_sha256") or ""),
        },
        "artifacts": artifacts,
    }


def write_canonical_result(path: Path, result: dict[str, Any]) -> None:
    """写入 canonical result JSON。"""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def write_compile_result(path: Path, result: dict[str, Any]) -> None:
    """写入 canonical result JSON。"""
    write_canonical_result(path, result)
