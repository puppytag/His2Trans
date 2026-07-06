"""基于 rustc JSON diagnostics 应用机器可判定文本修复。"""

from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
from typing import Any, Iterable


@dataclass(frozen=True)
class RustcTextEdit:
    """rustc 建议对应的单个文件文本编辑。"""

    file_path: Path
    byte_start: int
    byte_end: int
    replacement: str
    message: str = ""
    code: str = ""


@dataclass(frozen=True)
class RustcSuggestionApplyResult:
    """机器可判定 rustc 建议的应用结果。"""

    changed_files: tuple[Path, ...]
    edits_applied: int
    edits_skipped: int

    @property
    def changed(self) -> bool:
        """是否实际改动文件。"""
        return bool(self.changed_files)


def _iter_messages(cargo_json_output: str) -> Iterable[dict[str, Any]]:
    """遍历 cargo --message-format=json 输出里的 compiler-message。"""
    for line in (cargo_json_output or "").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get("reason") != "compiler-message":
            continue
        message = item.get("message")
        if isinstance(message, dict):
            yield message


def _walk_diagnostic_messages(message: dict[str, Any]) -> Iterable[dict[str, Any]]:
    """遍历 diagnostic 及其 children。"""
    yield message
    for child in message.get("children") or []:
        if isinstance(child, dict):
            yield from _walk_diagnostic_messages(child)


def _resolve_file(path_text: str, project_root: Path | None) -> Path:
    """解析 rustc span 文件路径。"""
    path = Path(path_text)
    if path.is_absolute() or project_root is None:
        return path
    return (project_root / path).resolve()


def collect_machine_applicable_edits(
    cargo_json_output: str,
    *,
    project_root: Path | None = None,
    target_files: Iterable[Path] | None = None,
) -> list[RustcTextEdit]:
    """提取 rustc 标记为 MachineApplicable 的文本编辑。"""
    root = Path(project_root).resolve() if project_root is not None else None
    allowed = None
    if target_files is not None:
        allowed = {Path(p).resolve() for p in target_files}

    edits: list[RustcTextEdit] = []
    seen: set[tuple[Path, int, int, str]] = set()

    for message in _iter_messages(cargo_json_output):
        code_obj = message.get("code") if isinstance(message.get("code"), dict) else {}
        code = str(code_obj.get("code") or "")
        for node in _walk_diagnostic_messages(message):
            for span in node.get("spans") or []:
                if not isinstance(span, dict):
                    continue
                if span.get("suggestion_applicability") != "MachineApplicable":
                    continue
                replacement = span.get("suggested_replacement")
                if replacement is None:
                    continue
                file_name = str(span.get("file_name") or "")
                if not file_name:
                    continue
                file_path = _resolve_file(file_name, root)
                if allowed is not None and file_path.resolve() not in allowed:
                    continue
                try:
                    byte_start = int(span.get("byte_start"))
                    byte_end = int(span.get("byte_end"))
                except Exception:
                    continue
                if byte_start < 0 or byte_end < byte_start:
                    continue
                key = (file_path.resolve(), byte_start, byte_end, str(replacement))
                if key in seen:
                    continue
                seen.add(key)
                edits.append(
                    RustcTextEdit(
                        file_path=file_path,
                        byte_start=byte_start,
                        byte_end=byte_end,
                        replacement=str(replacement),
                        message=str(node.get("message") or message.get("message") or ""),
                        code=code,
                    )
                )
    return edits


def apply_machine_applicable_edits(
    cargo_json_output: str,
    *,
    project_root: Path,
    target_files: Iterable[Path] | None = None,
) -> RustcSuggestionApplyResult:
    """将 rustc JSON 中机器可判定的建议应用到本地文件。"""
    edits = collect_machine_applicable_edits(
        cargo_json_output,
        project_root=project_root,
        target_files=target_files,
    )
    by_file: dict[Path, list[RustcTextEdit]] = {}
    for edit in edits:
        by_file.setdefault(edit.file_path.resolve(), []).append(edit)

    changed: list[Path] = []
    applied = 0
    skipped = 0

    for file_path, file_edits in by_file.items():
        if not file_path.exists():
            skipped += len(file_edits)
            continue
        content = file_path.read_bytes()
        ordered = sorted(file_edits, key=lambda e: (e.byte_start, e.byte_end), reverse=True)
        last_start = len(content) + 1
        new_content = content
        file_applied = 0
        for edit in ordered:
            if edit.byte_end > len(new_content) or edit.byte_start < 0:
                skipped += 1
                continue
            if edit.byte_end > last_start:
                skipped += 1
                continue
            replacement = edit.replacement.encode("utf-8")
            new_content = new_content[: edit.byte_start] + replacement + new_content[edit.byte_end :]
            last_start = edit.byte_start
            applied += 1
            file_applied += 1
        if file_applied and new_content != content:
            file_path.write_bytes(new_content)
            changed.append(file_path)

    return RustcSuggestionApplyResult(
        changed_files=tuple(changed),
        edits_applied=applied,
        edits_skipped=skipped,
    )
