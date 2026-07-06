"""跨进程限制外部 LLM 请求全局并发。"""

from __future__ import annotations

import errno
import fcntl
import json
import os
import socket
import time
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterator
from urllib.parse import urlparse


_LIMIT_ENVS = (
    "C2R_LLM_GLOBAL_CONCURRENCY_LIMIT",
    "C2RUST_LLM_GLOBAL_CONCURRENCY_LIMIT",
    "LLM_GLOBAL_CONCURRENCY_LIMIT",
)
_DIR_ENVS = (
    "C2R_LLM_GLOBAL_CONCURRENCY_DIR",
    "C2RUST_LLM_GLOBAL_CONCURRENCY_DIR",
    "LLM_GLOBAL_CONCURRENCY_DIR",
)
_WAIT_TIMEOUT_ENVS = (
    "C2R_LLM_GLOBAL_CONCURRENCY_WAIT_TIMEOUT_SEC",
    "C2RUST_LLM_GLOBAL_CONCURRENCY_WAIT_TIMEOUT_SEC",
    "LLM_GLOBAL_CONCURRENCY_WAIT_TIMEOUT_SEC",
)
_POLL_ENVS = (
    "C2R_LLM_GLOBAL_CONCURRENCY_POLL_SEC",
    "C2RUST_LLM_GLOBAL_CONCURRENCY_POLL_SEC",
    "LLM_GLOBAL_CONCURRENCY_POLL_SEC",
)
_DEFAULT_LIMIT = 100
_MAX_LIMIT = 100
_DEFAULT_POLL_SEC = 0.2
_LOCAL_HOSTS = {"localhost", "127.0.0.1", "::1", "0.0.0.0"}


@dataclass(slots=True)
class LLMGlobalSlot:
    """一次外部 LLM 请求持有的全局并发槽位。"""

    enabled: bool = False
    limit: int = 0
    wait_sec: float = 0.0
    slot_index: int | None = None
    lock_path: Path | None = None
    _handle: Any | None = None

    def release(self) -> None:
        """释放全局并发槽位。"""
        handle = self._handle
        if handle is None:
            return
        self._handle = None
        try:
            fcntl.flock(handle.fileno(), fcntl.LOCK_UN)
        finally:
            handle.close()

    def log_fields(self) -> dict[str, Any]:
        """返回可写入调用日志的限流字段。"""
        return {
            "llm_global_concurrency_enabled": self.enabled,
            "llm_global_concurrency_limit": self.limit,
            "llm_global_concurrency_wait_sec": round(self.wait_sec, 3),
            "llm_global_concurrency_slot": self.slot_index,
            "llm_global_concurrency_lock_path": str(self.lock_path) if self.lock_path is not None else "",
        }


def _first_env(names: tuple[str, ...]) -> str:
    """读取第一项非空环境变量。"""
    for name in names:
        value = os.environ.get(name, "").strip()
        if value:
            return value
    return ""


def _nonnegative_int_env(names: tuple[str, ...], default: int) -> int:
    """读取非负整数环境变量，配置错误直接失败。"""
    raw = _first_env(names)
    if not raw:
        return default
    try:
        value = int(raw)
    except ValueError as exc:
        raise ValueError(f"{names[0]} must be a non-negative integer: {raw}") from exc
    if value < 0:
        raise ValueError(f"{names[0]} must be a non-negative integer: {raw}")
    return value


def _global_limit_env() -> int:
    """读取外部 LLM 全局令牌数；配置超过 100 时钳制为 100。"""
    value = _nonnegative_int_env(_LIMIT_ENVS, _DEFAULT_LIMIT)
    return min(value, _MAX_LIMIT)


def _nonnegative_float_env(names: tuple[str, ...], default: float) -> float:
    """读取非负浮点环境变量，配置错误直接失败。"""
    raw = _first_env(names)
    if not raw:
        return default
    try:
        value = float(raw)
    except ValueError as exc:
        raise ValueError(f"{names[0]} must be a non-negative number: {raw}") from exc
    if value < 0:
        raise ValueError(f"{names[0]} must be a non-negative number: {raw}")
    return value


def _global_lock_dir() -> Path:
    """返回全局 LLM 槽位锁目录。"""
    configured = _first_env(_DIR_ENVS)
    if configured:
        return Path(configured).expanduser().resolve()
    return (Path(__file__).resolve().parent / "experiment_runs" / "shared" / "llm_global_concurrency").resolve()


def is_local_llm_url(base_url: str | None) -> bool:
    """判断 OpenAI-compatible endpoint 是否为本机服务。"""
    text = str(base_url or "").strip()
    if not text:
        return False
    parsed = urlparse(text)
    host = parsed.hostname or ""
    if host.lower() in _LOCAL_HOSTS:
        return True
    try:
        return socket.gethostbyname(host) in {"127.0.0.1", "0.0.0.0"}
    except OSError:
        return False


def _try_acquire_slot(lock_dir: Path, limit: int, *, label: str) -> LLMGlobalSlot | None:
    """非阻塞获取一个全局并发槽位。"""
    lock_dir.mkdir(parents=True, exist_ok=True)
    for slot_index in range(limit):
        lock_path = lock_dir / f"slot_{slot_index:04d}.lock"
        handle = lock_path.open("a+", encoding="utf-8")
        try:
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError as exc:
            handle.close()
            if exc.errno in {errno.EACCES, errno.EAGAIN}:
                continue
            raise
        handle.seek(0)
        handle.truncate()
        handle.write(
            json.dumps(
                {
                    "pid": os.getpid(),
                    "slot": slot_index,
                    "label": label,
                    "acquired_at": time.time(),
                },
                ensure_ascii=False,
                sort_keys=True,
            )
            + "\n"
        )
        handle.flush()
        return LLMGlobalSlot(enabled=True, limit=limit, slot_index=slot_index, lock_path=lock_path, _handle=handle)
    return None


def acquire_llm_global_slot(*, base_url: str | None = "", model: str | None = "", label: str | None = None) -> LLMGlobalSlot:
    """获取外部 LLM 全局并发槽位；本地 vLLM 不占用槽位。"""
    if is_local_llm_url(base_url):
        return LLMGlobalSlot(enabled=False)
    limit = _global_limit_env()
    if limit <= 0:
        return LLMGlobalSlot(enabled=False, limit=0)
    wait_timeout_sec = _nonnegative_float_env(_WAIT_TIMEOUT_ENVS, 0.0)
    poll_sec = _nonnegative_float_env(_POLL_ENVS, _DEFAULT_POLL_SEC)
    if poll_sec <= 0:
        raise ValueError(f"{_POLL_ENVS[0]} must be greater than 0")
    lock_dir = _global_lock_dir()
    slot_label = str(label or model or base_url or "llm").strip()
    started = time.perf_counter()
    while True:
        slot = _try_acquire_slot(lock_dir, limit, label=slot_label)
        if slot is not None:
            slot.wait_sec = time.perf_counter() - started
            return slot
        waited = time.perf_counter() - started
        if wait_timeout_sec > 0 and waited >= wait_timeout_sec:
            raise TimeoutError(
                f"timed out waiting for LLM global concurrency slot: "
                f"limit={limit} dir={lock_dir} timeout={wait_timeout_sec:.3f}s"
            )
        time.sleep(min(poll_sec, wait_timeout_sec - waited) if wait_timeout_sec > 0 else poll_sec)


@contextmanager
def llm_global_slot(*, base_url: str | None = "", model: str | None = "", label: str | None = None) -> Iterator[LLMGlobalSlot]:
    """上下文形式持有外部 LLM 全局并发槽位。"""
    slot = acquire_llm_global_slot(base_url=base_url, model=model, label=label)
    try:
        yield slot
    finally:
        slot.release()
