"""构建增量翻译 prompt 的上下文块和可审计 manifest。"""

from __future__ import annotations

import os
from typing import Any


def context_prefix_limit_from_env(env_name: str = "C2R_CONTEXT_PREFIX_MAX_CHARS") -> int | None:
    """读取上下文前缀字符上限；空值/0/off 表示不裁剪。"""
    raw = os.environ.get(env_name, "").strip().lower()
    if raw in ("", "0", "none", "off", "false", "no", "unbounded"):
        return None
    try:
        value = int(raw)
    except ValueError as exc:
        raise ValueError(f"{env_name} 必须是正整数或 off/none/0。") from exc
    if value <= 0:
        return None
    return value


def build_context_prefix(
    blocks: list[dict[str, Any]],
    *,
    max_chars: int | None = None,
) -> tuple[str, dict[str, Any]]:
    """按顺序拼接上下文块；默认不裁剪，只在显式 max_chars 时跳过超限块。"""
    selected: list[str] = []
    manifest_blocks: list[dict[str, Any]] = []
    used_chars = 0

    for block in blocks:
        label = str(block.get("label") or "unknown")
        content = str(block.get("content") or "").strip()
        stats = block.get("stats") or {}
        if not content:
            manifest_blocks.append(
                {
                    "label": label,
                    "length": 0,
                    "included": False,
                    "reason": "empty",
                    "stats": stats,
                }
            )
            continue

        block_text = f"// ===== {label} =====\n{content}"
        block_length = len(block_text)
        if max_chars is not None and used_chars + block_length > max_chars:
            manifest_blocks.append(
                {
                    "label": label,
                    "length": block_length,
                    "included": False,
                    "reason": "budget",
                    "stats": stats,
                }
            )
            continue

        selected.append(block_text)
        used_chars += block_length
        manifest_blocks.append(
            {
                "label": label,
                "length": block_length,
                "included": True,
                "stats": stats,
            }
        )

    return "\n\n".join(selected).strip(), {
        "context_budget": max_chars,
        "context_used": used_chars,
        "context_blocks": manifest_blocks,
    }

