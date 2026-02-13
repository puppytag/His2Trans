#!/usr/bin/env python3
"""
检查骨架构建输出的脚本
用于分析生成的骨架文件是否符合预期（只包含函数签名 + unimplemented!()）
"""

import re
from pathlib import Path
from typing import List, Tuple

def analyze_rust_file(file_path: Path) -> dict:
    """分析 Rust 文件，检查是否包含函数体"""
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # 统计信息
    stats = {
        'file': str(file_path),
        'total_lines': len(content.split('\n')),
        'functions_with_body': 0,
        'functions_with_unimplemented': 0,
        'functions_with_other_stubs': 0,
        'has_types_import': False,
        'has_globals_import': False,
        'has_llm_markers': False,
        'function_examples': []
    }
    
    # 检查导入
    stats['has_types_import'] = 'use crate::types::' in content or 'use crate::types;' in content
    stats['has_globals_import'] = 'use crate::globals::' in content or 'use crate::globals;' in content
    
    # 检查是否有 LLM 标记（不应该有）
    stats['has_llm_markers'] = '```rust' in content or '```' in content
    
    # 查找所有函数定义
    # 匹配模式：pub fn, fn, pub extern "C" fn 等
    function_pattern = r'(?:pub\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{'
    functions = re.finditer(function_pattern, content)
    
    for match in functions:
        func_name = match.group(1)
        start_pos = match.end()
        
        # 查找函数体的结束位置
        brace_count = 1
        pos = start_pos
        func_body = ""
        
        while pos < len(content) and brace_count > 0:
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
            func_body += content[pos]
            pos += 1
        
        func_body = func_body.rstrip('}').strip()
        
        # 分析函数体
        if 'unimplemented!' in func_body:
            stats['functions_with_unimplemented'] += 1
        elif 'todo!' in func_body or 'panic!' in func_body:
            stats['functions_with_other_stubs'] += 1
        elif func_body and len(func_body) > 10:  # 有实际函数体
            stats['functions_with_body'] += 1
            # 保存前3个示例
            if len(stats['function_examples']) < 3:
                body_preview = func_body[:200].replace('\n', ' ')
                stats['function_examples'].append({
                    'name': func_name,
                    'body_preview': body_preview
                })
    
    return stats

def check_skeleton_directory(skeleton_dir: Path) -> dict:
    """检查整个骨架目录"""
    results = {
        'total_files': 0,
        'files_with_bodies': 0,
        'files_with_llm_markers': 0,
        'files_with_types_import': 0,
        'files_with_globals_import': 0,
        'file_details': []
    }
    
    # 查找所有 .rs 文件（排除 main.rs, lib.rs, types.rs, globals.rs）
    exclude_files = {'main.rs', 'lib.rs', 'types.rs', 'globals.rs'}
    
    for rs_file in skeleton_dir.rglob("*.rs"):
        if rs_file.name in exclude_files:
            continue
        
        results['total_files'] += 1
        stats = analyze_rust_file(rs_file)
        results['file_details'].append(stats)
        
        if stats['functions_with_body'] > 0:
            results['files_with_bodies'] += 1
        
        if stats['has_llm_markers']:
            results['files_with_llm_markers'] += 1
        
        if stats['has_types_import']:
            results['files_with_types_import'] += 1
        
        if stats['has_globals_import']:
            results['files_with_globals_import'] += 1
    
    return results

def main():
    import sys
    
    if len(sys.argv) < 2:
        print("用法: python3 check_skeleton_output.py <skeleton_dir>")
        print("示例: python3 check_skeleton_output.py workspace/skeletons/my_project")
        sys.exit(1)
    
    skeleton_dir = Path(sys.argv[1])
    
    if not skeleton_dir.exists():
        print(f"错误: 目录不存在: {skeleton_dir}")
        sys.exit(1)
    
    print(f"\n{'='*60}")
    print(f"检查骨架输出: {skeleton_dir}")
    print(f"{'='*60}\n")
    
    results = check_skeleton_directory(skeleton_dir)
    
    print(f"📊 统计信息:")
    print(f"  总文件数: {results['total_files']}")
    print(f"  包含函数体的文件: {results['files_with_bodies']}")
    print(f"  包含 LLM 标记的文件: {results['files_with_llm_markers']}")
    print(f"  包含 types 导入的文件: {results['files_with_types_import']}")
    print(f"  包含 globals 导入的文件: {results['files_with_globals_import']}")
    print()
    
    # 显示有问题的文件
    if results['files_with_bodies'] > 0:
        print(f"⚠️  发现 {results['files_with_bodies']} 个文件包含函数体:")
        for detail in results['file_details']:
            if detail['functions_with_body'] > 0:
                print(f"\n  📄 {detail['file']}")
                print(f"     包含函数体的函数数: {detail['functions_with_body']}")
                print(f"     只有 unimplemented!() 的函数数: {detail['functions_with_unimplemented']}")
                if detail['function_examples']:
                    print(f"     示例函数:")
                    for ex in detail['function_examples']:
                        print(f"       - {ex['name']}: {ex['body_preview'][:100]}...")
    
    if results['files_with_llm_markers'] > 0:
        print(f"\n⚠️  发现 {results['files_with_llm_markers']} 个文件包含 LLM 标记（```rust）:")
        for detail in results['file_details']:
            if detail['has_llm_markers']:
                print(f"     - {detail['file']}")
    
    # 检查是否符合新流程的特征
    print(f"\n{'='*60}")
    print("✅ 符合新流程特征的文件:")
    new_style_count = 0
    for detail in results['file_details']:
        if (detail['has_types_import'] or detail['has_globals_import']) and \
           detail['functions_with_body'] == 0 and \
           not detail['has_llm_markers']:
            new_style_count += 1
            print(f"  ✓ {detail['file']}")
    
    print(f"\n总结: {new_style_count}/{results['total_files']} 个文件符合新流程特征")
    print(f"{'='*60}\n")

if __name__ == "__main__":
    main()


















































