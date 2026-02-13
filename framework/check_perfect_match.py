#!/usr/bin/env python3
"""
检查骨架翻译的完全匹配项目

"完全匹配"定义：
- 函数提取数量 = 骨架函数数量
- 即：extracted/functions/*.txt 的数量 = skeletons/src/*.rs 中 unimplemented!() 的数量

这意味着所有提取的函数都有对应的骨架签名，可以直接进行后续翻译。

支持两种模式：
- tree-sitter 模式（默认）：统计 extracted/functions/*.txt 的数量
- libclang 模式（--use-libclang）：从 libclang_extracted.json 读取函数数量
"""

import json
from pathlib import Path
from typing import List, Dict, Tuple, Optional
import re

# 全局标志：是否使用 libclang 模式
USE_LIBCLANG = False


def extract_project_name(project_dir: Path) -> str:
    """从目录名提取项目名（去掉日期后缀）"""
    name = project_dir.name
    # 格式: project_name_YYYYMMDD
    if '_' in name:
        parts = name.rsplit('_', 1)
        # 检查最后一部分是否是日期格式（8位数字）
        if len(parts[-1]) == 8 and parts[-1].isdigit():
            return parts[0]
    return name


def count_function_files_treesitter(project_dir: Path) -> int:
    """统计函数文件数量（tree-sitter 模式）"""
    project_name = extract_project_name(project_dir)
    functions_dir = project_dir / "workspace" / "extracted" / project_name / "functions"
    if not functions_dir.exists():
        return 0
    return len(list(functions_dir.glob("*.txt")))


def count_function_files_libclang(project_dir: Path) -> Tuple[int, Optional[List[str]]]:
    """
    从 libclang_extracted.json 读取函数数量（libclang 模式）
    
    Returns:
        (函数数量, 函数名列表或None)
    """
    project_name = extract_project_name(project_dir)
    
    # 优先级1: source_skeletons 目录（translate.py 新保存位置）
    json_path = project_dir / "workspace" / "source_skeletons" / project_name / "libclang_extracted.json"
    
    if not json_path.exists():
        # 优先级2: extracted 目录（旧路径）
        json_path = project_dir / "workspace" / "extracted" / project_name / "libclang_extracted.json"
    
    if not json_path.exists():
        # 优先级3: 更旧的路径
        json_path = project_dir / "workspace" / "extracted" / "libclang_extracted.json"
    
    if not json_path.exists():
        return 0, None
    
    try:
        with open(json_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        functions = data.get('functions', [])
        func_names = [f.get('name', '') for f in functions]
        return len(functions), func_names
    except Exception as e:
        print(f"  ⚠ 读取 libclang JSON 失败: {e}")
        return 0, None


def count_function_files(project_dir: Path, use_libclang: bool = False) -> int:
    """统计函数文件数量"""
    if use_libclang:
        count, _ = count_function_files_libclang(project_dir)
        return count
    else:
        return count_function_files_treesitter(project_dir)


def count_skeleton_functions(project_dir: Path) -> int:
    """统计骨架函数数量（unimplemented!() 的数量）"""
    project_name = extract_project_name(project_dir)
    skeleton_dir = project_dir / "workspace" / "skeletons" / project_name / "src"
    if not skeleton_dir.exists():
        return 0
    
    total_count = 0
    for rs_file in skeleton_dir.glob("*.rs"):
        if rs_file.name in ["main.rs", "types.rs", "globals.rs"]:
            continue
        
        try:
            content = rs_file.read_text(encoding='utf-8', errors='ignore')
            # 统计 unimplemented!() 的数量
            count = len(re.findall(r'unimplemented!\s*\(\)', content))
            total_count += count
        except Exception:
            pass
    
    return total_count


def check_perfect_match(project_dir: Path, use_libclang: bool = False) -> Tuple[bool, int, int, Optional[Dict]]:
    """
    检查项目是否完全匹配
    
    Args:
        project_dir: 项目目录
        use_libclang: 是否使用 libclang 模式
    
    Returns:
        (is_perfect_match, function_count, skeleton_count, details_dict)
    """
    func_count = count_function_files(project_dir, use_libclang)
    skeleton_count = count_skeleton_functions(project_dir)
    
    is_perfect = (func_count == skeleton_count) and (func_count > 0)
    
    details = None
    if use_libclang:
        _, func_names = count_function_files_libclang(project_dir)
        skeleton_names = get_skeleton_function_names(project_dir)
        
        if func_names and skeleton_names:
            missing = set(func_names) - set(skeleton_names)
            excess = set(skeleton_names) - set(func_names)
            details = {
                'missing_in_skeleton': sorted(missing),
                'excess_in_skeleton': sorted(excess)
            }
    
    return is_perfect, func_count, skeleton_count, details


def get_skeleton_function_names(project_dir: Path) -> List[str]:
    """提取骨架中的函数名"""
    project_name = extract_project_name(project_dir)
    skeleton_dir = project_dir / "workspace" / "skeletons" / project_name / "src"
    if not skeleton_dir.exists():
        return []
    
    func_names = []
    # 匹配函数定义: pub fn xxx( 或 fn xxx(
    func_pattern = re.compile(r'(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(')
    
    for rs_file in skeleton_dir.glob("*.rs"):
        if rs_file.name in ["main.rs", "types.rs", "globals.rs"]:
            continue
        
        try:
            content = rs_file.read_text(encoding='utf-8', errors='ignore')
            matches = func_pattern.findall(content)
            func_names.extend(matches)
        except Exception:
            pass
    
    return func_names


def scan_all_projects(outputs_dir: Path, use_libclang: bool = False) -> Dict[str, Dict]:
    """
    扫描所有项目，检查完全匹配状态
    
    Args:
        outputs_dir: 输出目录
        use_libclang: 是否使用 libclang 模式
    
    Returns:
        {
            "project_name": {
                "is_perfect_match": bool,
                "function_count": int,
                "skeleton_count": int,
                "diff": int,
                "path": str,
                "details": {...}  # 仅在 libclang 模式下
            }
        }
    """
    results = {}
    
    for project_dir in sorted(outputs_dir.iterdir()):
        if not project_dir.is_dir():
            continue
        # 只把“有 workspace 的目录”当作项目输出目录，避免把 evaluation_reports 等杂项目录算进去
        if not (project_dir / "workspace").exists():
            continue
        
        # 提取项目名（去掉日期后缀）
        project_name = extract_project_name(project_dir)
        
        is_perfect, func_count, skeleton_count, details = check_perfect_match(project_dir, use_libclang)
        diff = func_count - skeleton_count
        
        result = {
            "is_perfect_match": is_perfect,
            "function_count": func_count,
            "skeleton_count": skeleton_count,
            "diff": diff,
            "path": str(project_dir)
        }
        
        if details:
            result["details"] = details
        
        results[project_name] = result
    
    return results


def main():
    """主函数"""
    global USE_LIBCLANG
    import argparse
    
    parser = argparse.ArgumentParser(
        description="检查骨架翻译的完全匹配项目",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
"完全匹配"定义：
  函数提取数量 = 骨架函数数量
  
  这意味着所有提取的函数都有对应的骨架签名，可以直接进行后续翻译。

验证模式:
  - 默认 (tree-sitter): 统计 extracted/functions/*.txt 的数量
  - libclang 模式: 从 libclang_extracted.json 读取（更精确）

示例：
  # 检查所有项目（tree-sitter 模式）
  python check_perfect_match.py
  
  # 使用 libclang 模式检查
  python check_perfect_match.py --use-libclang
  
  # 只输出完全匹配的项目名（用于 --only 参数）
  python check_perfect_match.py --list-only
  
  # 输出 JSON 格式（包含详细的缺失/多余函数列表）
  python check_perfect_match.py --use-libclang --json
        """
    )
    parser.add_argument(
        "--outputs-dir",
        type=Path,
        default=Path("translation_outputs"),
        help="翻译输出目录（默认: translation_outputs）"
    )
    parser.add_argument(
        "--use-libclang",
        action="store_true",
        help="使用 libclang 模式：从 libclang_extracted.json 读取函数列表（更精确）"
    )
    parser.add_argument(
        "--list-only",
        action="store_true",
        help="只输出完全匹配的项目名列表（用逗号分隔）"
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="以 JSON 格式输出"
    )
    parser.add_argument(
        "--show-details",
        action="store_true",
        help="显示不匹配项目的详细函数列表"
    )
    parser.add_argument(
        "--summary-only",
        action="store_true",
        help="只显示摘要信息（不列出每个项目）"
    )
    args = parser.parse_args()
    
    USE_LIBCLANG = args.use_libclang
    
    outputs_dir = args.outputs_dir.resolve()
    if not outputs_dir.exists():
        print(f"错误: 输出目录不存在: {outputs_dir}")
        return

    # 兼容 run 目录布局：translation_outputs/<run_dir>/intermediate/<project>/workspace
    projects_root = outputs_dir
    if (outputs_dir / "intermediate").is_dir():
        projects_root = outputs_dir / "intermediate"
    
    results = scan_all_projects(projects_root, use_libclang=args.use_libclang)
    
    perfect_matches = [name for name, info in results.items() if info["is_perfect_match"]]
    partial_matches = [name for name, info in results.items() if not info["is_perfect_match"] and info["function_count"] > 0]
    
    if args.list_only:
        # 只输出完全匹配的项目名（用逗号分隔）
        print(",".join(sorted(perfect_matches)))
        return
    
    if args.json:
        output = {
            "mode": "libclang" if args.use_libclang else "tree-sitter",
            "perfect_matches": perfect_matches,
            "all_projects": results
        }
        print(json.dumps(output, indent=2, ensure_ascii=False))
        return
    
    # 文本输出
    print("=" * 80)
    mode_str = "libclang 模式" if args.use_libclang else "tree-sitter 模式"
    print(f"骨架翻译完全匹配检查结果 ({mode_str})")
    print("=" * 80)
    print(f"\n总项目数: {len(results)}")
    print(f"完全匹配: {len(perfect_matches)} 个")
    print(f"部分匹配: {len(partial_matches)} 个")
    print(f"无函数: {len(results) - len(perfect_matches) - len(partial_matches)} 个")
    
    # 如果是 summary-only 模式，只输出摘要
    if args.summary_only:
        if perfect_matches:
            print(f"\n✅ 完全匹配项目: {','.join(sorted(perfect_matches)[:10])}")
            if len(perfect_matches) > 10:
                print(f"   ... 还有 {len(perfect_matches) - 10} 个")
        if partial_matches:
            print(f"\n⚠️ 部分匹配项目: {','.join(sorted(partial_matches)[:10])}")
            if len(partial_matches) > 10:
                print(f"   ... 还有 {len(partial_matches) - 10} 个")
        return
    
    if perfect_matches:
        print("\n" + "=" * 80)
        print("✅ 完全匹配的项目:")
        print("=" * 80)
        print(f"{'项目名':<30} {'函数数':>8} {'骨架数':>8} {'状态':<10}")
        print("-" * 80)
        for name in sorted(perfect_matches):
            info = results[name]
            print(f"{name:<30} {info['function_count']:>8} {info['skeleton_count']:>8} {'✅ 匹配':<10}")
        
        print("\n" + "=" * 80)
        print("完全匹配项目列表（用于 --only 参数）:")
        print("=" * 80)
        print(",".join(sorted(perfect_matches)))
    
    if partial_matches:
        print("\n" + "=" * 80)
        print("⚠️ 部分匹配的项目:")
        print("=" * 80)
        print(f"{'项目名':<30} {'函数数':>8} {'骨架数':>8} {'差异':>8} {'状态':<10}")
        print("-" * 80)
        for name in sorted(partial_matches):
            info = results[name]
            status = f"缺失 {info['diff']}" if info['diff'] > 0 else f"多余 {-info['diff']}"
            print(f"{name:<30} {info['function_count']:>8} {info['skeleton_count']:>8} {info['diff']:>8} {'⚠️ ' + status:<10}")
            
            # 显示详细信息
            if args.show_details and 'details' in info:
                details = info['details']
                if details.get('missing_in_skeleton'):
                    print(f"    缺失的函数: {', '.join(details['missing_in_skeleton'][:5])}")
                    if len(details['missing_in_skeleton']) > 5:
                        print(f"    ... 还有 {len(details['missing_in_skeleton']) - 5} 个")
                if details.get('excess_in_skeleton'):
                    print(f"    多余的函数: {', '.join(details['excess_in_skeleton'][:5])}")
                    if len(details['excess_in_skeleton']) > 5:
                        print(f"    ... 还有 {len(details['excess_in_skeleton']) - 5} 个")


if __name__ == "__main__":
    main()
