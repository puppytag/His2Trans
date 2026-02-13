#!/usr/bin/env python3
"""
检查翻译输出状态和错误
"""
import os
import json
from pathlib import Path
from collections import defaultdict
import re

def check_output_status():
    """检查输出状态"""
    # Open-source layout: keep everything relative to this repo.
    script_dir = Path(__file__).resolve().parent
    outputs_dir = script_dir / "translation_outputs"
    # Logs are stored under: translation_outputs/<run>/intermediate/logs/
    logs_dir = outputs_dir
    
    print("=" * 80)
    print("翻译输出状态检查")
    print("=" * 80)
    print()
    
    # 检查输出目录
    print("1. 检查输出目录...")
    if not outputs_dir.exists():
        print(f"   ✗ translation_outputs 目录不存在")
        return
    else:
        print(f"   ✓ translation_outputs 目录存在")
    
    # 列出所有项目输出目录
    try:
        project_dirs = [d for d in outputs_dir.iterdir() if d.is_dir()]
        print(f"   找到 {len(project_dirs)} 个项目输出目录")
        
        if project_dirs:
            print("\n   项目列表:")
            for proj_dir in sorted(project_dirs)[:20]:
                print(f"     - {proj_dir.name}")
        else:
            print("   ⚠ 目录为空，可能还没有运行过翻译脚本")
    except Exception as e:
        print(f"   ✗ 无法读取目录内容: {e}")
        project_dirs = []
    
    print()
    
    # 检查日志目录
    print("2. 检查日志目录...")
    if not logs_dir.exists():
        print(f"   ✗ logs 目录不存在")
    else:
        print(f"   ✓ logs 目录存在")
        log_files = list(logs_dir.glob("*/intermediate/logs/batch_staged_*.log"))
        print(f"   找到 {len(log_files)} 个批量测试日志文件")
        
        if log_files:
            print("\n   最近的日志文件:")
            for log_file in sorted(log_files, key=lambda x: x.stat().st_mtime, reverse=True)[:5]:
                mtime = log_file.stat().st_mtime
                import datetime
                mtime_str = datetime.datetime.fromtimestamp(mtime).strftime("%Y-%m-%d %H:%M:%S")
                print(f"     - {log_file.name} ({mtime_str})")
    
    print()
    
    # 检查每个项目的状态
    print("3. 检查各项目状态...")
    project_status = defaultdict(lambda: {"skeleton": False, "translated": False, "errors": []})
    
    for proj_dir in project_dirs:
        proj_name = proj_dir.name.split("_")[0] if "_" in proj_dir.name else proj_dir.name
        workspace_dir = proj_dir / "workspace"
        
        if not workspace_dir.exists():
            continue
        
        # 检查骨架文件
        skeleton_dir = workspace_dir / "skeletons" / proj_name
        if skeleton_dir.exists():
            rs_files = list(skeleton_dir.glob("**/*.rs"))
            if rs_files:
                project_status[proj_name]["skeleton"] = True
        
        # 检查翻译文件
        translated_dir = workspace_dir / "translated" / proj_name
        if translated_dir.exists():
            rs_files = list(translated_dir.glob("**/*.rs"))
            if rs_files:
                project_status[proj_name]["translated"] = True
        
        # 检查错误日志
        logs_subdir = proj_dir / "logs"
        if logs_subdir.exists():
            error_logs = list(logs_subdir.glob("*.log"))
            if error_logs:
                project_status[proj_name]["errors"].extend([f.name for f in error_logs])
    
    print(f"\n   检查了 {len(project_status)} 个项目:")
    success_count = 0
    skeleton_only_count = 0
    failed_count = 0
    
    for proj_name, status in sorted(project_status.items()):
        if status["translated"]:
            success_count += 1
            print(f"     ✓ {proj_name}: 骨架✓ 翻译✓")
        elif status["skeleton"]:
            skeleton_only_count += 1
            print(f"     ⚠ {proj_name}: 骨架✓ 翻译✗")
        else:
            failed_count += 1
            print(f"     ✗ {proj_name}: 骨架✗ 翻译✗")
            if status["errors"]:
                print(f"        错误日志: {', '.join(status['errors'][:3])}")
    
    print()
    print(f"   统计: 成功 {success_count}, 仅骨架 {skeleton_only_count}, 失败 {failed_count}")
    print()
    
    # 检查最近的错误日志
    print("4. 检查最近的错误...")
    if logs_dir.exists():
        recent_logs = sorted(logs_dir.glob("*/intermediate/logs/batch_staged_*.log"), 
                           key=lambda x: x.stat().st_mtime, reverse=True)[:3]
        
        for log_file in recent_logs:
            print(f"\n   检查日志: {log_file.name}")
            try:
                with open(log_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                    
                # 查找错误关键词
                error_keywords = [
                    "bindgen", "clang", "file not found", "error:", 
                    "FAIL", "失败", "Error", "ERROR"
                ]
                
                errors_found = []
                lines = content.split('\n')
                for i, line in enumerate(lines[-500:], start=len(lines)-500):  # 只检查最后500行
                    for keyword in error_keywords:
                        if keyword.lower() in line.lower():
                            errors_found.append((i+1, line.strip()[:100]))
                            break
                
                if errors_found:
                    print(f"     找到 {len(errors_found)} 个可能的错误:")
                    for line_num, error_line in errors_found[:10]:
                        print(f"       行 {line_num}: {error_line}")
                else:
                    print(f"     未找到明显错误")
                    
            except Exception as e:
                print(f"     读取日志失败: {e}")
    
    print()
    print("=" * 80)
    print("检查完成")
    print("=" * 80)

if __name__ == "__main__":
    check_output_status()







































