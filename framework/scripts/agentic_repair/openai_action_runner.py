"""OpenAI-compatible JSON action runner for local Rust repair agents.

This module is adapted from `/data/home/wangshb/C++2Rust/framework/agentic_codegen/openai_repair.py`.
It keeps the same small action interface while using this repository's `generate.generation`
client so the runner follows the current framework LLM configuration.
"""

from __future__ import annotations

import difflib
import json
import re
import subprocess
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from scripts.agentic_repair.rust_native_refactor_gate import write_rust_native_refactor_task


DEFAULT_OBSERVATION_CHARS = 16000
DEFAULT_READ_LINES = 180
MAX_READ_LINES = 600
RG_TIMEOUT_SEC = 45.0

SYSTEM_PROMPT = """你是本地 Rust 迁移 JSON action runner。只能返回 JSON object。

你不能假装已经执行命令或读取文件；需要信息时输出 actions，由框架执行后返回 observation。
每轮 JSON schema：
{
  "summary": "本轮简短判断",
  "actions": [
    {"type": "read_file", "path": "src/main.rs", "start_line": 1, "line_count": 120},
    {"type": "list_dir", "path": "."},
    {"type": "rg", "pattern": "symbol_name", "path": "."},
    {"type": "replace_text", "path": "src/main.rs", "old": "exact text", "new": "replacement text"},
    {"type": "write_file", "path": "build.rs", "content": "..."},
    {"type": "append_file", "path": "src/compat.rs", "content": "..."},
    {"type": "run_verify"},
    {"type": "regenerate_unsafe_review"},
    {"type": "regenerate_rust_native_refactor_review"},
    {"type": "finish", "status": "done|blocked", "message": "..."}
  ]
}

规则：
- `summary` 只写一句短判断，不要复述大段上下文。
- 每轮至少给出一个 action。
- 修改文件优先用 replace_text；只有生成新文件或完整重写小文件时才用 write_file。
- 写入范围只允许当前 Rust 项目、agent 输出目录及其上层运行目录。
- 不要为了记录思考生成 trace/review/scratch markdown/json 文件；只有修复目标本身需要新增源码、shim 或构建文件时才写新文件。
- run_verify 只会执行 agent context 中的 verify_commands.fast。
- regenerate_rust_native_refactor_review 会在 Rust-native 重构任务中按当前 Rust 源码覆盖生成 review JSON。
- regenerate_unsafe_review 是兼容别名，行为同 regenerate_rust_native_refactor_review。
- 认为任务完成时使用 finish action。
"""


@dataclass(frozen=True)
class AgentRun:
    """Result paths and metadata for one local JSON action-loop run."""

    task_path: Path
    trajectory_path: Path
    stdout_path: Path
    stderr_path: Path
    result_path: Path
    workspace_diff_path: Path
    cwd: Path
    started_at: str
    finished_at: str
    elapsed_sec: float
    returncode: int
    command: tuple[str, str]
    timed_out: bool
    message_count: int
    reused_session: bool = False

    def to_dict(self) -> dict[str, Any]:
        """Return a JSON-serializable result."""
        return {
            "task_path": str(self.task_path),
            "trajectory_path": str(self.trajectory_path),
            "stdout_path": str(self.stdout_path),
            "stderr_path": str(self.stderr_path),
            "result_path": str(self.result_path),
            "workspace_diff_path": str(self.workspace_diff_path),
            "cwd": str(self.cwd),
            "started_at": self.started_at,
            "finished_at": self.finished_at,
            "elapsed_sec": self.elapsed_sec,
            "returncode": self.returncode,
            "command": list(self.command),
            "timed_out": self.timed_out,
            "message_count": self.message_count,
            "reused_session": self.reused_session,
        }


class OpenAIActionRunner:
    """Drive a local JSON action loop with this repository's OpenAI-compatible client."""

    def __init__(
        self,
        *,
        step_limit: int = 80,
        timeout_sec: float = 3600.0,
        observation_char_limit: int = DEFAULT_OBSERVATION_CHARS,
        log_parent_write_depth: int = 2,
    ) -> None:
        """Store loop budgets."""
        self._step_limit = max(1, int(step_limit))
        self._timeout_sec = float(timeout_sec)
        self._observation_char_limit = max(1000, int(observation_char_limit))
        self._log_parent_write_depth = max(0, int(log_parent_write_depth))
        self._sessions: dict[str, list[dict[str, Any]]] = {}

    def run(
        self,
        *,
        task: str,
        cwd: str | Path,
        output_dir: str | Path,
        name: str,
        finish_validator: Callable[[dict[str, Any]], str | None] | None = None,
    ) -> AgentRun:
        """Execute one agent run and persist trace, stdout/stderr, result and workspace diff."""
        task_text = str(task).strip()
        messages = [{"role": "user", "content": _initial_user_prompt(task_text)}]
        result, updated_messages = self._run_messages(
            task=task,
            cwd=cwd,
            output_dir=output_dir,
            name=name,
            finish_validator=finish_validator,
            messages=messages,
            reused_session=False,
        )
        self._sessions[str(name).strip()] = updated_messages
        return result

    def continue_run(
        self,
        *,
        task: str,
        cwd: str | Path,
        output_dir: str | Path,
        name: str,
        base_name: str,
        allowed_write_paths: tuple[Path, ...] | None = None,
        finish_validator: Callable[[dict[str, Any]], str | None] | None = None,
    ) -> AgentRun:
        """Continue a previous named action loop by appending one user task."""
        del allowed_write_paths
        base_key = str(base_name).strip()
        if not base_key or base_key not in self._sessions:
            return self.run(task=task, cwd=cwd, output_dir=output_dir, name=name, finish_validator=finish_validator)
        task_text = str(task).strip()
        messages = [dict(item) for item in self._sessions[base_key]]
        messages.append({"role": "user", "content": task_text})
        result, updated_messages = self._run_messages(
            task=task,
            cwd=cwd,
            output_dir=output_dir,
            name=name,
            finish_validator=finish_validator,
            messages=messages,
            reused_session=True,
        )
        self._sessions[base_key] = updated_messages
        self._sessions[str(name).strip()] = updated_messages
        return result

    def _run_messages(
        self,
        *,
        task: str,
        cwd: str | Path,
        output_dir: str | Path,
        name: str,
        finish_validator: Callable[[dict[str, Any]], str | None] | None,
        messages: list[dict[str, Any]],
        reused_session: bool,
    ) -> tuple[AgentRun, list[dict[str, Any]]]:
        """Execute an action loop over an existing message list."""
        task_text = str(task).strip()
        if not task_text:
            raise ValueError("repair task 不能为空。")
        workdir = Path(cwd).expanduser().resolve()
        if not workdir.is_dir():
            raise FileNotFoundError(f"repair 工作目录不存在：{workdir}")
        log_dir = Path(output_dir).expanduser().resolve()
        log_dir.mkdir(parents=True, exist_ok=True)

        task_path = log_dir / f"{name}.task.md"
        trajectory_path = log_dir / f"{name}.trajectory.json"
        stdout_path = log_dir / f"{name}.stdout.log"
        stderr_path = log_dir / f"{name}.stderr.log"
        result_path = log_dir / f"{name}.result.json"
        workspace_diff_path = log_dir / f"{name}.workspace.diff"
        task_path.write_text(task_text + "\n", encoding="utf-8")

        before_snapshot = _snapshot_workspace(workdir)
        started_at = _utc_now()
        started_time = time.time()
        timed_out = False
        returncode = 1
        stdout_chunks: list[str] = []
        stderr_chunks: list[str] = []
        trajectory_messages: list[dict[str, Any]] = []
        steps: list[dict[str, Any]] = []
        write_roots = _write_roots(workdir, log_dir, parent_depth=self._log_parent_write_depth)

        try:
            from generate.generation import generation

            print(f"[post-repair-agent] {_utc_now()} {name}: start step_limit={self._step_limit} cwd={workdir} output={log_dir}", flush=True)
            for step in range(1, self._step_limit + 1):
                if self._timeout_sec > 0 and time.time() - started_time > self._timeout_sec:
                    raise TimeoutError(f"timeout_sec={self._timeout_sec}")
                print(f"[post-repair-agent] {_utc_now()} {name}: step {step}/{self._step_limit} request start", flush=True)
                raw_result = generation(
                    [
                        {"role": "system", "content": SYSTEM_PROMPT},
                        *messages,
                    ],
                    return_usage=True,
                )
                llm_usage = {}
                raw_response = raw_result
                if isinstance(raw_result, dict):
                    llm_usage = raw_result.get("usage", {}) or {}
                    raw_response = raw_result.get("content", "")
                response = _parse_json_response(raw_response)
                actions = _normalize_actions(response)
                action_types = [str(action.get("type") or action.get("action") or "").strip() or "unknown" for action in actions]
                print(f"[post-repair-agent] {_utc_now()} {name}: step {step}/{self._step_limit} response actions={action_types}", flush=True)
                observations: list[dict[str, Any]] = []
                finish_status = ""
                finish_message = ""
                for action in actions:
                    observation = self._execute_action(
                        action,
                        cwd=workdir,
                        log_dir=log_dir,
                        write_roots=write_roots,
                        task_text=task_text,
                    )
                    observations.append(observation)
                    if observation.get("type") == "finish":
                        finish_status = str(observation.get("status", "")).strip()
                        finish_message = str(observation.get("message", "")).strip()
                ok_count = sum(1 for observation in observations if bool(observation.get("ok")))
                print(
                    f"[post-repair-agent] {_utc_now()} {name}: step {step}/{self._step_limit} observations_ok={ok_count}/{len(observations)} finish={finish_status or '<none>'}",
                    flush=True,
                )
                step_payload = {"step": step, "response": response, "observations": observations, "llm_usage": llm_usage}
                steps.append(step_payload)
                trajectory_messages.append({"role": "assistant", "content": _stable_json(response)})
                stdout_chunks.append(_stable_json({"step": step, "observations": observations}))
                messages.append({"role": "assistant", "content": _stable_json(response)})
                messages.append({"role": "user", "content": "执行结果 JSON：\n" + _stable_json({"observations": observations})})
                if finish_status:
                    continuation_prompt = ""
                    if finish_validator is not None:
                        continuation_prompt = str(
                            finish_validator(
                                {
                                    "name": name,
                                    "step": step,
                                    "cwd": str(workdir),
                                    "output_dir": str(log_dir),
                                    "task_path": str(task_path),
                                    "finish_status": finish_status,
                                    "finish_message": finish_message,
                                    "elapsed_sec": round(time.time() - started_time, 3),
                                    "steps": steps,
                                }
                            )
                            or ""
                        ).strip()
                    if continuation_prompt:
                        print(f"[post-repair-agent] {_utc_now()} {name}: finish validator requested continuation", flush=True)
                        messages.append({"role": "user", "content": continuation_prompt})
                        stdout_chunks.append("finish validator requested continuation")
                        continue
                    if finish_status in {"done", "complete", "completed", "submitted"}:
                        returncode = 0
                    else:
                        returncode = 1
                    if finish_message:
                        stdout_chunks.append(finish_message)
                    break
            else:
                stderr_chunks.append(f"repair agent 达到 step_limit={self._step_limit} 后仍未 finish。")
        except TimeoutError as exc:
            timed_out = True
            returncode = 124
            stderr_chunks.append(f"repair agent 超时退出：{exc}")
        except Exception as exc:  # noqa: BLE001
            returncode = 127
            stderr_chunks.append(f"repair agent 失败：{type(exc).__name__}: {exc}")

        finished_at = _utc_now()
        elapsed_sec = round(time.time() - started_time, 3)
        after_snapshot = _snapshot_workspace(workdir)
        _write_workspace_diff(workspace_diff_path, before_snapshot, after_snapshot)
        stdout_path.write_text("\n".join(stdout_chunks).rstrip() + ("\n" if stdout_chunks else ""), encoding="utf-8")
        stderr_path.write_text("\n".join(stderr_chunks).rstrip() + ("\n" if stderr_chunks else ""), encoding="utf-8")
        trajectory_path.write_text(
            json.dumps({"info": {"exit_status": "Submitted" if returncode == 0 else "Incomplete"}, "messages": trajectory_messages, "steps": steps}, ensure_ascii=False, indent=2, sort_keys=True)
            + "\n",
            encoding="utf-8",
        )
        result = AgentRun(
            task_path=task_path,
            trajectory_path=trajectory_path,
            stdout_path=stdout_path,
            stderr_path=stderr_path,
            result_path=result_path,
            workspace_diff_path=workspace_diff_path,
            cwd=workdir,
            started_at=started_at,
            finished_at=finished_at,
            elapsed_sec=elapsed_sec,
            returncode=returncode,
            command=("openai-compatible-repair-agent", name),
            timed_out=timed_out,
            message_count=len(messages),
            reused_session=reused_session,
        )
        result_path.write_text(json.dumps(result.to_dict(), ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        _append_run_trace(log_dir / "agent_trace.md", name, task_text, result)
        print(f"[post-repair-agent] {_utc_now()} {name}: finished returncode={returncode} elapsed={elapsed_sec}s result={result_path} diff={workspace_diff_path}", flush=True)
        return result, messages

    def _execute_action(
        self,
        action: dict[str, Any],
        *,
        cwd: Path,
        log_dir: Path,
        write_roots: tuple[Path, ...],
        task_text: str,
    ) -> dict[str, Any]:
        """Execute one action and return an observation."""
        action_type = str(action.get("type") or action.get("action") or "").strip()
        if action_type == "read_file":
            return self._read_file(action, cwd=cwd)
        if action_type == "list_dir":
            return self._list_dir(action, cwd=cwd)
        if action_type == "rg":
            return self._rg(action, cwd=cwd)
        if action_type == "replace_text":
            return self._replace_text(action, cwd=cwd, write_roots=write_roots)
        if action_type == "write_file":
            return self._write_file(action, cwd=cwd, write_roots=write_roots, append=False)
        if action_type == "append_file":
            return self._write_file(action, cwd=cwd, write_roots=write_roots, append=True)
        if action_type == "run_verify":
            return self._run_verify(action, cwd=cwd, log_dir=log_dir, task_text=task_text)
        if action_type in {"regenerate_unsafe_review", "regenerate_rust_native_refactor_review"}:
            return self._regenerate_unsafe_review(action, cwd=cwd, write_roots=write_roots, task_text=task_text, action_type=action_type)
        if action_type == "finish":
            return {
                "type": "finish",
                "ok": True,
                "status": str(action.get("status", "done")).strip() or "done",
                "message": str(action.get("message", "")).strip(),
            }
        return {"type": action_type or "unknown", "ok": False, "error": f"未知 action type：{action_type or '<empty>'}"}

    def _read_file(self, action: dict[str, Any], *, cwd: Path) -> dict[str, Any]:
        """Read a text file fragment."""
        path = _resolve_path(action.get("path"), cwd=cwd)
        if not path.is_file():
            return {"type": "read_file", "ok": False, "path": str(path), "error": "file not found"}
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError as exc:
            return {"type": "read_file", "ok": False, "path": str(path), "error": str(exc)}
        start_line = max(1, _int_value(action.get("start_line"), 1))
        line_count = min(MAX_READ_LINES, max(1, _int_value(action.get("line_count"), DEFAULT_READ_LINES)))
        selected = lines[start_line - 1 : start_line - 1 + line_count]
        text = "\n".join(f"{line_no}: {line}" for line_no, line in enumerate(selected, start=start_line))
        return {
            "type": "read_file",
            "ok": True,
            "path": str(path),
            "start_line": start_line,
            "line_count": len(selected),
            "total_lines": len(lines),
            "content": _trim_text(text, self._observation_char_limit),
        }

    def _list_dir(self, action: dict[str, Any], *, cwd: Path) -> dict[str, Any]:
        """List directory entries."""
        path = _resolve_path(action.get("path") or ".", cwd=cwd)
        if not path.is_dir():
            return {"type": "list_dir", "ok": False, "path": str(path), "error": "directory not found"}
        try:
            entries = [item.name + ("/" if item.is_dir() else "") for item in sorted(path.iterdir(), key=lambda value: value.name)[:200]]
        except OSError as exc:
            return {"type": "list_dir", "ok": False, "path": str(path), "error": str(exc)}
        return {"type": "list_dir", "ok": True, "path": str(path), "entries": entries}

    def _rg(self, action: dict[str, Any], *, cwd: Path) -> dict[str, Any]:
        """Run a bounded ripgrep query."""
        pattern = str(action.get("pattern", "")).strip()
        if not pattern:
            return {"type": "rg", "ok": False, "error": "pattern 不能为空"}
        path = _resolve_path(action.get("path") or ".", cwd=cwd)
        command = ["rg", "-n", "--hidden", "--glob", "!target", "--", pattern, str(path)]
        try:
            completed = subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False, timeout=RG_TIMEOUT_SEC)
        except Exception as exc:  # noqa: BLE001
            return {"type": "rg", "ok": False, "command": command, "error": str(exc)}
        return {
            "type": "rg",
            "ok": completed.returncode in {0, 1},
            "returncode": completed.returncode,
            "command": command,
            "stdout": _trim_text(completed.stdout, self._observation_char_limit),
            "stderr": _trim_text(completed.stderr, self._observation_char_limit // 2),
        }

    def _replace_text(self, action: dict[str, Any], *, cwd: Path, write_roots: tuple[Path, ...]) -> dict[str, Any]:
        """Replace exact text once unless allow_multiple=true."""
        try:
            path = _resolve_write_path(action.get("path"), cwd=cwd, write_roots=write_roots)
        except ValueError as exc:
            return {"type": "replace_text", "ok": False, "error": str(exc)}
        old = str(action.get("old", ""))
        new = str(action.get("new", ""))
        if not old:
            return {"type": "replace_text", "ok": False, "path": str(path), "error": "old 不能为空"}
        try:
            text = path.read_text(encoding="utf-8")
        except OSError as exc:
            return {"type": "replace_text", "ok": False, "path": str(path), "error": str(exc)}
        occurrence_count = text.count(old)
        if occurrence_count == 0:
            return {"type": "replace_text", "ok": False, "path": str(path), "error": "old text not found"}
        allow_multiple = bool(action.get("allow_multiple", False))
        if occurrence_count > 1 and not allow_multiple:
            return {"type": "replace_text", "ok": False, "path": str(path), "error": f"old text matched {occurrence_count} times"}
        updated = text.replace(old, new) if allow_multiple else text.replace(old, new, 1)
        path.write_text(updated, encoding="utf-8")
        return {"type": "replace_text", "ok": True, "path": str(path), "replacements": occurrence_count if allow_multiple else 1}

    def _write_file(self, action: dict[str, Any], *, cwd: Path, write_roots: tuple[Path, ...], append: bool) -> dict[str, Any]:
        """Write or append a text file."""
        try:
            path = _resolve_write_path(action.get("path"), cwd=cwd, write_roots=write_roots)
        except ValueError as exc:
            return {"type": "append_file" if append else "write_file", "ok": False, "error": str(exc)}
        content = str(action.get("content", ""))
        path.parent.mkdir(parents=True, exist_ok=True)
        try:
            if append:
                with path.open("a", encoding="utf-8") as handle:
                    handle.write(content)
            else:
                path.write_text(content, encoding="utf-8")
        except OSError as exc:
            return {"type": "append_file" if append else "write_file", "ok": False, "path": str(path), "error": str(exc)}
        return {"type": "append_file" if append else "write_file", "ok": True, "path": str(path), "bytes": len(content.encode("utf-8"))}

    def _run_verify(self, action: dict[str, Any], *, cwd: Path, log_dir: Path, task_text: str) -> dict[str, Any]:
        """Run verify_commands.fast from agent_context."""
        context_path = _agent_context_path(task_text, action.get("context_path"))
        if context_path is None or not context_path.is_file():
            return {"type": "run_verify", "ok": False, "error": "agent context not found"}
        try:
            context = json.loads(context_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            return {"type": "run_verify", "ok": False, "context_path": str(context_path), "error": str(exc)}
        verify = context.get("verify_commands") if isinstance(context, dict) else {}
        command = verify.get("fast") if isinstance(verify, dict) else None
        if not isinstance(command, list) or not all(isinstance(part, str) for part in command):
            return {"type": "run_verify", "ok": False, "context_path": str(context_path), "error": "verify_commands.fast missing"}
        timeout_sec = max(1.0, float(action.get("timeout_sec") or 900.0))
        try:
            completed = subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False, timeout=timeout_sec)
        except Exception as exc:  # noqa: BLE001
            return {"type": "run_verify", "ok": False, "command": command, "error": str(exc)}
        verify_log = log_dir / "openai_repair_verify.log"
        verify_log.write_text(
            "COMMAND: " + json.dumps(command, ensure_ascii=False) + "\n\nSTDOUT:\n" + completed.stdout + "\n\nSTDERR:\n" + completed.stderr,
            encoding="utf-8",
        )
        return {
            "type": "run_verify",
            "ok": completed.returncode == 0,
            "returncode": completed.returncode,
            "command": command,
            "log_path": str(verify_log),
            "stdout": _trim_text(completed.stdout, self._observation_char_limit),
            "stderr": _trim_text(completed.stderr, self._observation_char_limit),
        }

    def _regenerate_unsafe_review(self, action: dict[str, Any], *, cwd: Path, write_roots: tuple[Path, ...], task_text: str, action_type: str = "regenerate_unsafe_review") -> dict[str, Any]:
        """Regenerate Rust-native refactor review task JSON for the current crate source."""
        context_path = _agent_context_path(task_text, action.get("context_path"))
        if context_path is None or not context_path.is_file():
            return {"type": action_type, "ok": False, "error": "agent context not found"}
        try:
            context = json.loads(context_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            return {"type": action_type, "ok": False, "context_path": str(context_path), "error": str(exc)}
        native_info = context.get("rust_native_refactor") if isinstance(context, dict) else {}
        unsafe_info = context.get("unsafe_optimization") if isinstance(context, dict) else {}
        review_info = native_info if isinstance(native_info, dict) and native_info.get("active") else unsafe_info
        if not isinstance(review_info, dict) or not review_info.get("active"):
            return {"type": action_type, "ok": False, "context_path": str(context_path), "error": "rust-native refactor task is not active"}
        review_json = _resolve_write_path(review_info.get("review_task_json") or review_info.get("task_json"), cwd=cwd, write_roots=write_roots)
        scope_json = _resolve_write_path(review_info.get("scope_json"), cwd=cwd, write_roots=write_roots)
        scope_md = _resolve_write_path(review_info.get("scope_markdown"), cwd=cwd, write_roots=write_roots)
        abi_json = _resolve_write_path(review_info.get("abi_inventory_json") or review_json.with_name(review_json.name.replace("_unsafe_review_task.json", "_abi_refactor_inventory.json")), cwd=cwd, write_roots=write_roots)
        abi_md = _resolve_write_path(review_info.get("abi_inventory_markdown") or abi_json.with_suffix(".md"), cwd=cwd, write_roots=write_roots)
        info_paths = context.get("information_paths") if isinstance(context.get("information_paths"), dict) else {}
        source_roots = []
        for key in ("source_project_root", "copied_c_source"):
            path_text = str(info_paths.get(key) or "").strip()
            if path_text and Path(path_text).expanduser().is_dir():
                source_roots.append(Path(path_text).expanduser().resolve())
        suite = str(context.get("suite") or "ohos").strip().lower()
        if suite not in {"ohos", "oss"}:
            suite = "ohos"
        try:
            payload = write_rust_native_refactor_task(cwd, scope_json, scope_md, abi_json, abi_md, review_json, source_roots=source_roots, suite=suite)
        except Exception as exc:  # noqa: BLE001
            return {"type": action_type, "ok": False, "error": f"{type(exc).__name__}: {exc}"}
        return {
            "type": action_type,
            "ok": True,
            "review_json_path": payload.get("review_json_path", ""),
            "scope_json_path": payload.get("scope_json_path", ""),
            "scope_markdown_path": payload.get("scope_markdown_path", ""),
            "abi_inventory_json_path": payload.get("abi_inventory_json_path", ""),
            "abi_inventory_markdown_path": payload.get("abi_inventory_markdown_path", ""),
            "item_count": payload.get("item_count", 0),
            "summary": payload.get("summary", {}),
        }


def _parse_json_response(raw_response: Any) -> dict[str, Any]:
    """Parse a JSON object response; tolerate fenced JSON."""
    if isinstance(raw_response, dict):
        return raw_response
    text = str(raw_response or "").strip()
    for pattern in (r"```json\s*(.*?)```", r"```\s*(.*?)```"):
        match = re.search(pattern, text, re.DOTALL | re.IGNORECASE)
        if match:
            text = match.group(1).strip()
            break
    try:
        payload = json.loads(text)
    except json.JSONDecodeError as exc:
        return {"actions": [{"type": "invalid", "error": f"JSON parse failed: {exc}", "raw": _trim_text(text, 2000)}]}
    if not isinstance(payload, dict):
        return {"actions": [{"type": "invalid", "error": "response JSON must be object"}]}
    return payload


def _initial_user_prompt(task: str) -> str:
    """Wrap original task for the JSON action loop."""
    return "下面是原 repair/audit 任务。先读取其中列出的 context、日志和源码路径；需要框架执行读取、搜索、写文件或验证时，按 system JSON schema 输出 actions。\n\n" + task


def _normalize_actions(response: dict[str, Any]) -> list[dict[str, Any]]:
    """Extract action objects from a response."""
    raw_actions = response.get("actions")
    if raw_actions is None and isinstance(response.get("action"), dict):
        raw_actions = [response["action"]]
    if not isinstance(raw_actions, list):
        return [{"type": "invalid", "error": "response.actions 必须是 array"}]
    actions = [item for item in raw_actions if isinstance(item, dict)]
    return actions or [{"type": "invalid", "error": "response.actions 没有 object action"}]


def _resolve_path(value: object, *, cwd: Path) -> Path:
    """Resolve a read path."""
    text = str(value or ".").strip()
    path = Path(text).expanduser()
    if not path.is_absolute():
        path = cwd / path
    return path.resolve()


def _resolve_write_path(value: object, *, cwd: Path, write_roots: tuple[Path, ...]) -> Path:
    """Resolve and validate a write path."""
    path = _resolve_path(value, cwd=cwd)
    if not _is_under_any(path, write_roots):
        allowed = ", ".join(str(root) for root in write_roots)
        raise ValueError(f"写路径不在允许范围内：{path}；allowed={allowed}")
    return path


def _write_roots(cwd: Path, log_dir: Path, *, parent_depth: int = 2) -> tuple[Path, ...]:
    """Return allowed write roots."""
    roots = [cwd.resolve(), log_dir.resolve()]
    current = log_dir.resolve()
    for _ in range(max(0, int(parent_depth))):
        current = current.parent
        roots.append(current)
    deduped: list[Path] = []
    for root in roots:
        if root not in deduped:
            deduped.append(root)
    return tuple(deduped)


def _is_under_any(path: Path, roots: tuple[Path, ...]) -> bool:
    """Return true when path is under one allowed root."""
    for root in roots:
        try:
            path.resolve().relative_to(root.resolve())
            return True
        except ValueError:
            continue
    return False


def _trim_text(text: str, limit: int) -> str:
    """Trim long observations."""
    if len(text) <= limit:
        return text
    head = text[: limit // 2]
    tail = text[-limit // 2 :]
    return head + "\n...[truncated]...\n" + tail


def _int_value(value: object, default: int) -> int:
    """Read an integer safely."""
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def _agent_context_path(task_text: str, explicit: object = None) -> Path | None:
    """Extract agent context path from action or task text."""
    explicit_text = str(explicit or "").strip()
    if explicit_text:
        return Path(explicit_text).expanduser().resolve()
    for pattern in (
        r"agent context[：:]\s*([^\s`]+)",
        r"agent_context(?:_path)?[\"']?\s*[:=]\s*[\"']?([/\w.\-+]+)",
        r"agent_context_path[：:]\s*([^\s`]+)",
    ):
        match = re.search(pattern, task_text)
        if match:
            return Path(match.group(1)).expanduser().resolve()
    return None


def _utc_now() -> str:
    """Return an ISO UTC timestamp."""
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _stable_json(payload: Any) -> str:
    """Dump compact stable JSON."""
    return json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def _snapshot_workspace(root: Path) -> dict[str, str]:
    """Snapshot text-ish workspace files for diff logging."""
    snapshot: dict[str, str] = {}
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        rel = path.relative_to(root).as_posix()
        if rel.startswith("target/") or "/target/" in rel:
            continue
        if path.suffix in {".rlib", ".rmeta", ".o", ".a", ".so", ".dylib", ".dll"}:
            continue
        try:
            snapshot[rel] = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        except OSError:
            continue
    return snapshot


def _write_workspace_diff(path: Path, before: dict[str, str], after: dict[str, str]) -> None:
    """Write a unified diff for changed workspace files."""
    lines: list[str] = []
    for rel in sorted(set(before) | set(after)):
        old = before.get(rel)
        new = after.get(rel)
        if old == new:
            continue
        old_lines = [] if old is None else old.splitlines(keepends=True)
        new_lines = [] if new is None else new.splitlines(keepends=True)
        lines.extend(difflib.unified_diff(old_lines, new_lines, fromfile=f"a/{rel}", tofile=f"b/{rel}"))
    path.write_text("".join(lines), encoding="utf-8")


def _append_run_trace(path: Path, name: str, task_text: str, result: AgentRun) -> None:
    """Append a short trace entry."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(f"\n## {name}\n\n")
        handle.write(f"- started_at: {result.started_at}\n")
        handle.write(f"- finished_at: {result.finished_at}\n")
        handle.write(f"- returncode: {result.returncode}\n")
        handle.write(f"- task: {result.task_path}\n")
        handle.write(f"- trajectory: {result.trajectory_path}\n")
        handle.write(f"- diff: {result.workspace_diff_path}\n\n")
        handle.write("### Task Preview\n\n")
        handle.write(_trim_text(task_text, 4000) + "\n")


__all__ = ["OpenAIActionRunner", "AgentRun"]
