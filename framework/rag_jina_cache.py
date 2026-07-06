"""Jina reranker 内容寻址缓存工具。

该模块只处理 BM25 输入、rerank 输出和缓存 manifest，不加载模型。
"""

from __future__ import annotations

import hashlib
import json
import shutil
from pathlib import Path
from typing import Any


CACHE_SCHEMA_VERSION = "c2r_jina_rerank_cache_v3_candidate_index"


def _sha256_file(path: Path) -> str:
    """计算文件 SHA256。"""
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def _stable_json_bytes(payload: Any) -> bytes:
    """序列化为稳定 JSON bytes。"""
    return json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")


def build_jina_cache_manifest(
    *,
    project_name: str,
    bm25_dir: Path,
    query_files: list[Path],
    model_name: str,
    top_k: int,
    extra_params: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """根据 BM25 文件内容和 reranker 参数构建缓存 manifest。"""
    root = Path(bm25_dir).resolve()
    files: list[dict[str, Any]] = []
    for path in sorted((Path(p).resolve() for p in query_files), key=lambda p: p.name):
        try:
            rel = path.relative_to(root).as_posix()
        except ValueError:
            rel = path.name
        files.append(
            {
                "name": path.name,
                "relative_path": rel,
                "size": path.stat().st_size,
                "sha256": _sha256_file(path),
            }
        )
    fingerprint = {
        "schema_version": CACHE_SCHEMA_VERSION,
        "project": str(project_name),
        "model_name": str(model_name),
        "top_k": int(top_k),
        "extra_params": extra_params or {},
        "query_files": files,
    }
    cache_key = hashlib.sha256(_stable_json_bytes(fingerprint)).hexdigest()
    return {
        **fingerprint,
        "cache_key": cache_key,
        "query_count": len(files),
    }


def _entry_dir(cache_root: Path, manifest: dict[str, Any]) -> Path:
    """返回缓存条目目录。"""
    key = str(manifest.get("cache_key") or "").strip()
    if not key:
        raise ValueError("manifest 缺少 cache_key")
    return Path(cache_root).expanduser().resolve() / key[:2] / key


def _query_file_map(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    """按文件名索引 manifest 中的 BM25 query 指纹。"""
    result: dict[str, dict[str, Any]] = {}
    for item in manifest.get("query_files", []):
        if not isinstance(item, dict):
            continue
        name = str(item.get("name") or "").strip()
        if name:
            result[name] = item
    return result


def _same_rerank_params(current: dict[str, Any], cached: dict[str, Any]) -> bool:
    """检查两个缓存 manifest 是否使用同一重排参数。"""
    keys = ("schema_version", "project", "model_name", "top_k", "extra_params")
    return all(current.get(key) == cached.get(key) for key in keys)


def _manifest_covers_current_queries(current: dict[str, Any], cached: dict[str, Any]) -> bool:
    """严格检查 cached 是否是 current query 集合的内容超集。"""
    current_items = _query_file_map(current)
    cached_items = _query_file_map(cached)
    if not current_items or len(current_items) > len(cached_items):
        return False

    for name, current_item in current_items.items():
        cached_item = cached_items.get(name)
        if not cached_item:
            return False
        if current_item.get("size") != cached_item.get("size"):
            return False
        if current_item.get("sha256") != cached_item.get("sha256"):
            return False
    return True


def _same_query_fingerprint(current_item: dict[str, Any], cached_item: dict[str, Any] | None) -> bool:
    """检查两个 query 文件条目是否是同名同内容。"""
    if not cached_item:
        return False
    return (
        current_item.get("name") == cached_item.get("name")
        and current_item.get("size") == cached_item.get("size")
        and current_item.get("sha256") == cached_item.get("sha256")
    )


def _copy_cached_outputs(
    *,
    outputs_dir: Path,
    manifest: dict[str, Any],
    output_dir: Path,
) -> tuple[list[str], list[str]]:
    """复制当前 manifest 需要的 rerank 输出，返回已复制和缺失列表。"""
    out = Path(output_dir)
    out.mkdir(parents=True, exist_ok=True)
    restored = []
    missing = []
    for item in manifest.get("query_files", []):
        name = str(item.get("name") or "")
        if not name:
            continue
        src = outputs_dir / name
        if not src.is_file():
            missing.append(name)
            continue
        shutil.copy2(src, out / name)
        restored.append(name)
    return restored, missing


def _try_restore_subset_cache(cache_root: Path, manifest: dict[str, Any], output_dir: Path) -> dict[str, Any]:
    """精确 key 未命中时，从已校验的同参数超集缓存恢复当前 query 子集。"""
    root = Path(cache_root).expanduser().resolve()
    if not root.is_dir():
        return {"hit": False, "reason": "cache_root_missing", "cache_root": str(root)}

    candidates = []
    for manifest_path in sorted(root.glob("*/*/manifest.json")):
        try:
            cached_manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if cached_manifest.get("cache_key") == manifest.get("cache_key"):
            continue
        if not _same_rerank_params(manifest, cached_manifest):
            continue
        if not _manifest_covers_current_queries(manifest, cached_manifest):
            continue
        candidates.append((len(_query_file_map(cached_manifest)), manifest_path, cached_manifest))

    if not candidates:
        return {"hit": False, "reason": "cache_manifest_missing"}

    _, manifest_path, cached_manifest = sorted(candidates, key=lambda item: (item[0], str(item[1])))[0]
    entry = manifest_path.parent
    outputs_dir = entry / "outputs"
    query_names = [name for name in _query_file_map(manifest)]
    missing = [name for name in query_names if not (outputs_dir / name).is_file()]
    if missing:
        return {
            "hit": False,
            "reason": "cache_subset_outputs_missing",
            "missing_outputs": missing,
            "cache_dir": str(entry),
            "source_cache_key": str(cached_manifest.get("cache_key") or ""),
        }

    cached_validation = validate_rerank_outputs([Path(name) for name in query_names], outputs_dir)
    if not cached_validation.get("ok"):
        return {
            "hit": False,
            "reason": "cache_subset_outputs_invalid",
            "validation": cached_validation,
            "cache_dir": str(entry),
            "source_cache_key": str(cached_manifest.get("cache_key") or ""),
        }

    restored, copy_missing = _copy_cached_outputs(outputs_dir=outputs_dir, manifest=manifest, output_dir=output_dir)
    if copy_missing:
        return {
            "hit": False,
            "reason": "cache_subset_outputs_missing",
            "missing_outputs": copy_missing,
            "cache_dir": str(entry),
            "source_cache_key": str(cached_manifest.get("cache_key") or ""),
        }
    return {
        "hit": True,
        "reason": "cache_subset_hit",
        "restored_outputs": restored,
        "restored_count": len(restored),
        "cache_dir": str(entry),
        "source_cache_key": str(cached_manifest.get("cache_key") or ""),
        "source_query_count": len(_query_file_map(cached_manifest)),
    }


def restore_jina_partial_cache(cache_root: Path, manifest: dict[str, Any], output_dir: Path) -> dict[str, Any]:
    """从同参数缓存中恢复内容指纹一致的部分 rerank 输出。"""
    root = Path(cache_root).expanduser().resolve()
    if not root.is_dir():
        return {"hit": False, "reason": "cache_root_missing", "cache_root": str(root)}

    current_items = _query_file_map(manifest)
    if not current_items:
        return {"hit": False, "reason": "cache_current_queries_empty", "cache_root": str(root)}

    candidates: list[tuple[int, str, Path, dict[str, Any]]] = []
    for manifest_path in sorted(root.glob("*/*/manifest.json")):
        try:
            cached_manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if not _same_rerank_params(manifest, cached_manifest):
            continue
        cached_items = _query_file_map(cached_manifest)
        match_count = 0
        for name, current_item in current_items.items():
            if _same_query_fingerprint(current_item, cached_items.get(name)):
                match_count += 1
        if match_count:
            candidates.append((match_count, str(manifest_path), manifest_path, cached_manifest))

    if not candidates:
        return {
            "hit": False,
            "reason": "cache_partial_manifest_missing",
            "missing_outputs": sorted(current_items),
            "cache_root": str(root),
        }

    restored: list[str] = []
    restored_sources: dict[str, str] = {}
    invalid_source_outputs: dict[str, str] = {}
    source_cache_keys: list[str] = []
    pending = set(current_items)
    out = Path(output_dir)
    out.mkdir(parents=True, exist_ok=True)

    for _match_count, _path_key, manifest_path, cached_manifest in sorted(
        candidates,
        key=lambda item: (-item[0], item[1]),
    ):
        if not pending:
            break
        cached_items = _query_file_map(cached_manifest)
        outputs_dir = manifest_path.parent / "outputs"
        source_key = str(cached_manifest.get("cache_key") or "")

        for name in sorted(list(pending)):
            current_item = current_items[name]
            if not _same_query_fingerprint(current_item, cached_items.get(name)):
                continue

            src = outputs_dir / name
            if not src.is_file():
                continue

            valid, reason = _validate_rerank_output_file(src)
            if not valid:
                invalid_source_outputs[name] = reason
                continue

            shutil.copy2(src, out / name)
            restored.append(name)
            restored_sources[name] = source_key
            if source_key and source_key not in source_cache_keys:
                source_cache_keys.append(source_key)
            pending.remove(name)

    if not restored:
        return {
            "hit": False,
            "reason": "cache_partial_outputs_missing",
            "missing_outputs": sorted(current_items),
            "invalid_source_outputs": invalid_source_outputs,
            "cache_root": str(root),
        }

    return {
        "hit": True,
        "reason": "cache_partial_hit",
        "restored_outputs": sorted(restored),
        "restored_count": len(restored),
        "missing_outputs": sorted(pending),
        "source_cache_keys": source_cache_keys,
        "restored_sources": restored_sources,
        "invalid_source_outputs": invalid_source_outputs,
        "cache_root": str(root),
    }


def _validate_rerank_output_file(path: Path) -> tuple[bool, str]:
    """检查单个 rerank 文件是否符合 v3 可回溯输出结构。"""
    try:
        text = Path(path).read_text(encoding="utf-8", errors="ignore")
    except OSError as exc:
        return False, f"output_unreadable: {exc}"

    candidate_blocks = 0
    for block in text.split("-" * 50):
        if not block.strip():
            continue
        has_result_fields = "C_Code:" in block or "Function:" in block or "Unixcoder Score:" in block
        if not has_result_fields:
            continue
        candidate_blocks += 1
        index_pos = block.find("Candidate_Index:")
        c_code_pos = block.find("C_Code:")
        if index_pos < 0:
            return False, "candidate_index_missing"
        if c_code_pos >= 0 and index_pos > c_code_pos:
            return False, "candidate_index_after_c_code"
        first_line = block[index_pos:].splitlines()[0]
        raw_index = first_line.split("Candidate_Index:", 1)[1].strip()
        if not raw_index.isdigit():
            return False, "candidate_index_invalid"

    if candidate_blocks == 0:
        return False, "candidate_blocks_missing"
    return True, "ok"


def validate_rerank_outputs(query_files: list[Path], output_dir: Path) -> dict[str, Any]:
    """检查每个 BM25 query 是否都有结构完整的 rerank 输出。"""
    out = Path(output_dir)
    missing = []
    present = []
    invalid = []
    invalid_details = {}
    for path in sorted((Path(p) for p in query_files), key=lambda p: p.name):
        output_file = out / path.name
        if output_file.is_file():
            present.append(path.name)
            valid, reason = _validate_rerank_output_file(output_file)
            if not valid:
                invalid.append(path.name)
                invalid_details[path.name] = reason
        else:
            missing.append(path.name)
    return {
        "ok": not missing and not invalid,
        "present_outputs": present,
        "missing_outputs": missing,
        "invalid_outputs": invalid,
        "invalid_details": invalid_details,
        "expected_count": len(present) + len(missing),
        "present_count": len(present),
    }


def restore_jina_cache(cache_root: Path, manifest: dict[str, Any], output_dir: Path) -> dict[str, Any]:
    """缓存命中时把 rerank 输出恢复到当前 workspace。"""
    try:
        entry = _entry_dir(cache_root, manifest)
    except ValueError as exc:
        return {"hit": False, "reason": str(exc)}
    manifest_path = entry / "manifest.json"
    outputs_dir = entry / "outputs"
    if not manifest_path.is_file():
        subset = _try_restore_subset_cache(cache_root, manifest, output_dir)
        if subset.get("hit"):
            return subset
        return {"hit": False, "reason": subset.get("reason", "cache_manifest_missing"), "cache_dir": str(entry)}
    try:
        cached_manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"hit": False, "reason": f"cache_manifest_invalid: {exc}", "cache_dir": str(entry)}
    if cached_manifest.get("cache_key") != manifest.get("cache_key"):
        return {"hit": False, "reason": "cache_key_mismatch", "cache_dir": str(entry)}

    missing = []
    for item in manifest.get("query_files", []):
        name = str(item.get("name") or "")
        if name and not (outputs_dir / name).is_file():
            missing.append(name)
    if missing:
        return {"hit": False, "reason": "cache_outputs_missing", "missing_outputs": missing, "cache_dir": str(entry)}

    cached_validation = validate_rerank_outputs(
        [Path(str(item.get("name") or "")) for item in manifest.get("query_files", []) if str(item.get("name") or "")],
        outputs_dir,
    )
    if not cached_validation.get("ok"):
        return {
            "hit": False,
            "reason": "cache_outputs_invalid",
            "validation": cached_validation,
            "cache_dir": str(entry),
        }

    restored, copy_missing = _copy_cached_outputs(outputs_dir=outputs_dir, manifest=manifest, output_dir=output_dir)
    if copy_missing:
        return {"hit": False, "reason": "cache_outputs_missing", "missing_outputs": copy_missing, "cache_dir": str(entry)}
    return {"hit": True, "reason": "cache_hit", "restored_outputs": restored, "cache_dir": str(entry)}


def store_jina_cache(cache_root: Path, manifest: dict[str, Any], output_dir: Path) -> dict[str, Any]:
    """把当前完整 rerank 输出写入内容缓存。"""
    query_names = [str(item.get("name") or "") for item in manifest.get("query_files", []) if str(item.get("name") or "")]
    out = Path(output_dir)
    missing = [name for name in query_names if not (out / name).is_file()]
    if missing:
        return {"stored": False, "reason": "outputs_missing", "missing_outputs": missing}

    validation = validate_rerank_outputs([Path(name) for name in query_names], out)
    if not validation.get("ok"):
        return {"stored": False, "reason": "outputs_invalid", "validation": validation}

    try:
        entry = _entry_dir(cache_root, manifest)
    except ValueError as exc:
        return {"stored": False, "reason": str(exc)}
    outputs_dir = entry / "outputs"
    outputs_dir.mkdir(parents=True, exist_ok=True)
    for name in query_names:
        shutil.copy2(out / name, outputs_dir / name)
    manifest_path = entry / "manifest.json"
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return {"stored": True, "reason": "stored", "cache_dir": str(entry), "outputs": query_names}
