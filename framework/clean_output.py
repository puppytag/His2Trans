#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
清理脚本（仅针对 c2-rust_framework）
只清理 c2-rust_framework/workspace 下的中间产物与输出。

用法:
    python clean_output.py framework     # 清理 c2-rust_framework/workspace
    python clean_output.py all           # 等同于 framework（为兼容保留）
    python clean_output.py --dry-run     # 仅预览将要删除的文件
"""

import os
import sys
import shutil
from pathlib import Path

# 定位 c2-rust_framework（脚本所在目录）
SCRIPT_DIR = Path(__file__).parent.resolve()
FRAME_DIR = SCRIPT_DIR

# c2-rust_framework 工作区需要清理的路径（相对 FRAME_DIR）
# 注意：以下目录会被清理，但会保留：
#   - workspace/projects/ (待翻译的源码)
#   - workspace/rag/ (RAG 知识库和检索结果，包含 knowledge_base.json, bm25_index.pkl, elastic_search_results/, reranked_results/)
FRAMEWORK_PATHS = [
    # 工作区中间产物（按生成顺序）
    "workspace/extracted/",                    # 步骤1: 提取的代码
    "workspace/skeletons/",                   # 步骤2: 翻译的骨架
    "workspace/source_skeletons/",            # 步骤2: 源代码骨架（用于修复）
    "workspace/signature_matches/",           # 步骤3: 函数签名匹配
    "workspace/dependencies/",                # 步骤1: 依赖关系
    "workspace/dependencies_not_in_file/",    # 步骤1: 不在当前文件的依赖
    "workspace/dependencies_not_in_file_rs/", # 步骤1: Rust 格式的依赖
    "workspace/translated/",                  # 步骤7: 翻译的函数体
    "workspace/test_results/",               # 步骤9: 编译测试结果
    "workspace/repair_results/",              # 步骤10: 自动修复结果
    # 共享编译缓存（仅中间产物）
    "workspace/.cargo_target_shared/",       # Cargo 共享 target 目录
    "workspace/.rust_pool/",                 # Rust 编译池目录
]


def format_size(size_bytes):
    """格式化文件大小"""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.2f} TB"


def get_path_size(path):
    """获取文件或目录的大小"""
    if not path.exists():
        return 0
    
    if path.is_file():
        return path.stat().st_size
    elif path.is_dir():
        total = 0
        for dirpath, dirnames, filenames in os.walk(path):
            for filename in filenames:
                filepath = Path(dirpath) / filename
                try:
                    total += filepath.stat().st_size
                except:
                    pass
        return total
    return 0


def delete_path(path, base_dir, dry_run=False):
    """删除文件或目录"""
    full_path = base_dir / path
    
    if not full_path.exists():
        return False, 0
    
    size = get_path_size(full_path)
    
    if dry_run:
        print(f"  [DRY-RUN] 将删除: {full_path}")
        if size > 0:
            print(f"           大小: {format_size(size)}")
        return True, size
    
    try:
        if full_path.is_file():
            full_path.unlink()
        elif full_path.is_dir():
            shutil.rmtree(full_path)
        return True, size
    except Exception as e:
        print(f"  ❌ 删除失败: {full_path}")
        print(f"     错误: {e}")
        return False, size


def cleanup_framework(dry_run=False):
    """
    清理 c2-rust_framework/workspace 下的所有中间产物与输出
    
    保留的内容：
    - workspace/projects/ (待翻译的项目源码)
    - workspace/rag/ (RAG 知识库和检索结果)
      - workspace/rag/knowledge_base.json (RAG 知识库)
      - workspace/rag/bm25_index.pkl (BM25 索引)
      - workspace/rag/elastic_search_results/ (BM25 检索结果)
      - workspace/rag/reranked_results/ (Jina 重排结果)
    
    删除的内容：
    - 所有其他 workspace/ 下的中间产物目录
    """
    print(f"\n{'='*70}")
    print(f"清理 c2-rust_framework 工作区")
    print(f"{'='*70}")
    print(f"工作目录: {FRAME_DIR}")
    print(f"保留: workspace/projects/, workspace/rag/ (全部)")
    print(f"{'='*70}\n")

    total_deleted = 0
    total_size = 0

    for path_str in FRAMEWORK_PATHS:
        path = Path(path_str)
        deleted, size = delete_path(path, FRAME_DIR, dry_run)
        if deleted:
            total_deleted += 1
            total_size += size
            if not dry_run:
                print(f"  ✅ 已删除: {path}")
                if size > 0:
                    print(f"     大小: {format_size(size)}")

    print(f"\n{'='*70}")
    print(f"c2-rust_framework 清理完成:")
    print(f"  删除项目数: {total_deleted}")
    print(f"  释放空间: {format_size(total_size)}")
    print(f"{'='*70}\n")
    return True


def main():
    """主函数"""
    if len(sys.argv) < 2:
        print("用法:")
        print("  python clean_output.py framework [--dry-run]")
        print("  python clean_output.py all       [--dry-run]  # 等同于 framework")
        sys.exit(1)
    
    source = sys.argv[1].lower()
    dry_run = "--dry-run" in sys.argv
    
    if source in ("all", "framework"):
        cleanup_framework(dry_run)
    else:
        print(f"❌ 不支持的参数: {source}")
        print("   仅支持: framework, all")
        sys.exit(1)
    
    if dry_run:
        print("\n⚠️  DRY-RUN 模式: 没有实际删除任何文件")
        print("   要实际删除，请运行不带 --dry-run 的命令")


if __name__ == "__main__":
    main()


