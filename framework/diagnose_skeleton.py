#!/usr/bin/env python3
"""
诊断骨架翻译为空的根本原因
检查：
1. 原始C/C++项目文件内容
2. 提取的函数内容
3. 骨架翻译的输入（源代码骨架）
4. LLM生成的骨架输出
5. 最终保存的骨架文件
"""
import sys
from pathlib import Path
import re

def check_project_files(project_dir: Path):
    """检查原始项目文件"""
    print("=" * 60)
    print("1. 检查原始项目文件")
    print("=" * 60)
    
    if not project_dir.exists():
        print(f"❌ 项目目录不存在: {project_dir}")
        return False
    
    # 查找C/C++文件
    cpp_files = list(project_dir.rglob("*.c")) + list(project_dir.rglob("*.cpp")) + \
                list(project_dir.rglob("*.h")) + list(project_dir.rglob("*.hpp"))
    
    if not cpp_files:
        print(f"❌ 未找到任何C/C++源文件")
        return False
    
    print(f"✓ 找到 {len(cpp_files)} 个源文件")
    
    # 检查每个文件的内容
    total_functions = 0
    for cpp_file in cpp_files[:5]:  # 只检查前5个
        print(f"\n  文件: {cpp_file.name}")
        try:
            with open(cpp_file, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            # 统计函数定义
            # 简单的函数匹配（可能有误，但能大致判断）
            fn_patterns = [
                r'\b\w+\s+\w+\s*\([^)]*\)\s*\{',  # 函数定义
                r'\b\w+\s+\w+\s*\([^)]*\)\s*;',   # 函数声明
            ]
            
            functions = []
            for pattern in fn_patterns:
                matches = re.findall(pattern, content)
                functions.extend(matches)
            
            fn_count = len(set(functions))
            total_functions += fn_count
            
            # 检查文件是否只有注释或空白
            code_lines = [l for l in content.splitlines() 
                         if l.strip() and not l.strip().startswith("//") 
                         and not l.strip().startswith("/*")]
            
            print(f"    行数: {len(content.splitlines())}, 有效代码行: {len(code_lines)}")
            print(f"    函数数（估算）: {fn_count}")
            
            if len(code_lines) < 5:
                print(f"    ⚠ 警告: 文件内容很少，可能只有声明或注释")
            
            # 显示前几行
            print(f"    前5行:")
            for i, line in enumerate(content.splitlines()[:5], 1):
                print(f"      {i}: {line[:60]}")
                
        except Exception as e:
            print(f"    ❌ 读取失败: {e}")
    
    print(f"\n  总计函数数（估算）: {total_functions}")
    return True

def check_extracted_functions(extracted_dir: Path):
    """检查提取的函数"""
    print("\n" + "=" * 60)
    print("2. 检查提取的函数")
    print("=" * 60)
    
    functions_dir = extracted_dir / "functions"
    if not functions_dir.exists():
        print(f"❌ 函数目录不存在: {functions_dir}")
        return False
    
    function_files = list(functions_dir.glob("*.txt"))
    print(f"✓ 找到 {len(function_files)} 个函数文件")
    
    if len(function_files) == 0:
        print("❌ 没有提取到任何函数！")
        return False
    
    # 检查前3个函数文件
    for func_file in function_files[:3]:
        print(f"\n  函数: {func_file.name}")
        try:
            with open(func_file, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            print(f"    长度: {len(content)} 字符")
            print(f"    行数: {len(content.splitlines())}")
            print(f"    内容预览:")
            for i, line in enumerate(content.splitlines()[:5], 1):
                print(f"      {i}: {line[:60]}")
                
        except Exception as e:
            print(f"    ❌ 读取失败: {e}")
    
    return True

def check_source_skeleton(source_skeleton_dir: Path):
    """检查源代码骨架（LLM的输入）"""
    print("\n" + "=" * 60)
    print("3. 检查源代码骨架（LLM翻译的输入）")
    print("=" * 60)
    
    if not source_skeleton_dir.exists():
        print(f"❌ 源代码骨架目录不存在: {source_skeleton_dir}")
        return False
    
    skeleton_files = list(source_skeleton_dir.glob("*.txt"))
    print(f"✓ 找到 {len(skeleton_files)} 个源代码骨架文件")
    
    for skeleton_file in skeleton_files[:2]:
        print(f"\n  文件: {skeleton_file.name}")
        try:
            with open(skeleton_file, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            print(f"    长度: {len(content)} 字符")
            print(f"    行数: {len(content.splitlines())}")
            
            # 检查是否有函数定义
            fn_matches = re.findall(r'\b\w+\s+\w+\s*\([^)]*\)\s*\{', content)
            print(f"    函数定义数（估算）: {len(fn_matches)}")
            
            if len(content.strip()) < 50:
                print(f"    ⚠ 警告: 内容过短，可能为空或只有注释")
            
            print(f"    内容预览:")
            for i, line in enumerate(content.splitlines()[:10], 1):
                print(f"      {i}: {line[:60]}")
                
        except Exception as e:
            print(f"    ❌ 读取失败: {e}")
    
    return True

def check_skeleton_output(skeleton_dir: Path):
    """检查骨架翻译输出"""
    print("\n" + "=" * 60)
    print("4. 检查骨架翻译输出（LLM生成的结果）")
    print("=" * 60)
    
    src_dir = skeleton_dir / "src"
    if not src_dir.exists():
        print(f"❌ 骨架src目录不存在: {src_dir}")
        return False
    
    rs_files = list(src_dir.glob("*.rs"))
    print(f"✓ 找到 {len(rs_files)} 个Rust骨架文件")
    
    empty_count = 0
    for rs_file in rs_files:
        print(f"\n  文件: {rs_file.name}")
        try:
            with open(rs_file, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            print(f"    长度: {len(content)} 字符")
            print(f"    行数: {len(content.splitlines())}")
            
            # 检查是否只有注释
            code_lines = [l for l in content.splitlines() 
                         if l.strip() and not l.strip().startswith("//")]
            
            # 检查函数定义
            fn_matches = re.findall(r'\bfn\s+\w+\s*\(', content)
            print(f"    有效代码行: {len(code_lines)}")
            print(f"    函数定义数: {len(fn_matches)}")
            
            if len(code_lines) < 3 or len(fn_matches) == 0:
                empty_count += 1
                print(f"    ❌ 警告: 骨架文件为空或没有函数定义！")
            
            print(f"    内容预览:")
            for i, line in enumerate(content.splitlines()[:10], 1):
                print(f"      {i}: {line[:60]}")
                
        except Exception as e:
            print(f"    ❌ 读取失败: {e}")
    
    if empty_count > 0:
        print(f"\n  ⚠ 警告: {empty_count}/{len(rs_files)} 个骨架文件为空或无函数")
    
    return True

def check_llm_response(llm_prompts_dir: Path, project_name: str):
    """检查LLM的响应"""
    print("\n" + "=" * 60)
    print("5. 检查LLM响应（如果存在）")
    print("=" * 60)
    
    prompt_dir = llm_prompts_dir / project_name / "translate_by_qwen3_coder"
    if not prompt_dir.exists():
        print(f"⚠ LLM提示词目录不存在: {prompt_dir}")
        return False
    
    # 查找骨架翻译的提示词文件
    skeleton_files = list(prompt_dir.glob("*_skeleton.txt"))
    if not skeleton_files:
        skeleton_files = list(prompt_dir.glob("*_skeleton.json"))
    
    print(f"✓ 找到 {len(skeleton_files)} 个骨架翻译提示词文件")
    
    for prompt_file in skeleton_files[:1]:
        print(f"\n  文件: {prompt_file.name}")
        try:
            if prompt_file.suffix == '.json':
                import json
                with open(prompt_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                # 查找响应
                if isinstance(data, list):
                    for msg in data:
                        if msg.get("role") == "assistant":
                            response = msg.get("content", "")
                            print(f"    LLM响应长度: {len(response)} 字符")
                            print(f"    响应预览:")
                            for i, line in enumerate(response.splitlines()[:10], 1):
                                print(f"      {i}: {line[:60]}")
                            break
            else:
                with open(prompt_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                print(f"    文件长度: {len(content)} 字符")
                # 查找代码块
                code_blocks = re.findall(r'```rust(.*?)```', content, re.DOTALL)
                if code_blocks:
                    print(f"    找到 {len(code_blocks)} 个代码块")
                    print(f"    第一个代码块预览:")
                    for i, line in enumerate(code_blocks[0].splitlines()[:10], 1):
                        print(f"      {i}: {line[:60]}")
                else:
                    print(f"    ⚠ 未找到代码块")
                    
        except Exception as e:
            print(f"    ❌ 读取失败: {e}")
    
    return True

def main():
    if len(sys.argv) < 2:
        print("用法: python3 diagnose_skeleton.py <项目名>")
        print("示例: python3 diagnose_skeleton.py mini")
        sys.exit(1)
    
    project_name = sys.argv[1]
    
    # 构建路径
    workspace_base = Path("translation_outputs") / f"{project_name}_20251204" / "workspace"
    
    project_dir = workspace_base / "projects" / project_name
    extracted_dir = workspace_base / "extracted" / project_name
    source_skeleton_dir = workspace_base / "source_skeletons" / project_name / "src"
    skeleton_dir = workspace_base / "skeletons" / project_name
    llm_prompts_dir = workspace_base / "llm_prompts"
    
    print(f"\n诊断项目: {project_name}")
    print(f"工作空间: {workspace_base}")
    print()
    
    # 执行检查
    results = []
    results.append(("原始项目", check_project_files(project_dir)))
    results.append(("提取的函数", check_extracted_functions(extracted_dir)))
    results.append(("源代码骨架", check_source_skeleton(source_skeleton_dir)))
    results.append(("骨架输出", check_skeleton_output(skeleton_dir)))
    results.append(("LLM响应", check_llm_response(llm_prompts_dir, project_name)))
    
    # 总结
    print("\n" + "=" * 60)
    print("诊断总结")
    print("=" * 60)
    for name, result in results:
        status = "✓" if result else "❌"
        print(f"{status} {name}")
    
    print("\n可能的原因:")
    if not results[0][1]:
        print("  1. 原始项目文件不存在或为空")
    if not results[1][1]:
        print("  2. 函数提取失败（没有提取到函数）")
    if not results[2][1]:
        print("  3. 源代码骨架生成失败（LLM输入为空）")
    if not results[3][1]:
        print("  4. LLM生成的骨架为空（LLM只返回了占位符）")
    if not results[4][1]:
        print("  5. 无法检查LLM响应（提示词文件不存在）")

if __name__ == "__main__":
    main()

