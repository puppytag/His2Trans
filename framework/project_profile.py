"""统一判断当前翻译项目 profile/suite。"""

from __future__ import annotations

import os


_OSS_ALIASES = {"oss", "generic"}
_OHOS_ALIASES = {"ohos", "openharmony"}


def normalize_project_suite(value: str | None = None) -> str:
    """规范化项目 suite；generic 在当前框架中映射为 oss。"""
    raw = str(
        value
        or os.environ.get("C2R_PROJECT_SUITE")
        or os.environ.get("PROJECT_SUITE")
        or os.environ.get("PROJECT_PROFILE")
        or os.environ.get("C2RUST_PROJECT_PROFILE")
        or os.environ.get("C2R_PROJECT_PROFILE")
        or "ohos"
    ).strip().lower()
    if raw in _OSS_ALIASES:
        return "oss"
    if raw in _OHOS_ALIASES:
        return "ohos"
    return "ohos"


def is_oss_suite(value: str | None = None) -> bool:
    """判断当前是否为普通 OSS/generic 项目翻译。"""
    return normalize_project_suite(value) == "oss"


def is_ohos_suite(value: str | None = None) -> bool:
    """判断当前是否为 OHOS 项目翻译。"""
    return normalize_project_suite(value) == "ohos"
