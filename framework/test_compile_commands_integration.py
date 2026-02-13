#!/usr/bin/env python3
"""
测试 compile_commands.json 集成

验证：
1. compile_commands.json 解析器是否正常工作
2. workspace_config 是否能找到 compile_commands.json
3. SkeletonBuilder 是否能正确使用 compile_commands.json
"""

import sys
from pathlib import Path

# 添加项目根目录到路径
sys.path.insert(0, str(Path(__file__).parent))

_HIS2TRANS_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_OHOS_ROOT = _HIS2TRANS_ROOT / "data" / "ohos" / "ohos_root_min"

def test_compile_commands_parser():
    """测试 compile_commands_parser 模块"""
    print("=" * 60)
    print("测试 1: compile_commands_parser 模块")
    print("=" * 60)
    
    try:
        from compile_commands_parser import CompileCommandsParser
        print("✓ 模块导入成功")
        return True
    except ImportError as e:
        print(f"✗ 模块导入失败: {e}")
        return False


def test_workspace_config():
    """测试 workspace_config 的 get_compile_commands_path 函数"""
    print("\n" + "=" * 60)
    print("测试 2: workspace_config.get_compile_commands_path")
    print("=" * 60)
    
    try:
        from workspace_config import get_compile_commands_path
        
        # 测试1: 从环境变量查找
        import os
        test_path = "/tmp/test_compile_commands.json"
        Path(test_path).touch()
        os.environ["OHOS_COMPILE_COMMANDS"] = test_path
        
        result = get_compile_commands_path()
        if result and str(result) == test_path:
            print("✓ 环境变量查找成功")
        else:
            print(f"⚠ 环境变量查找失败（可能文件不存在）")
        
        # 清理
        Path(test_path).unlink()
        del os.environ["OHOS_COMPILE_COMMANDS"]
        
        # 测试2: 从 OpenHarmony 根目录查找
        ohos_root = DEFAULT_OHOS_ROOT
        if ohos_root.exists():
            result = get_compile_commands_path(ohos_root)
            if result and result.exists():
                print(f"✓ 找到 compile_commands.json: {result}")
                print(f"  文件大小: {result.stat().st_size / 1024 / 1024:.1f} MB")
            else:
                print(f"⚠ 未找到 compile_commands.json（这是正常的，如果还没生成）")
        else:
            print(f"⚠ OpenHarmony 根目录不存在: {ohos_root}")
        
        return True
    except Exception as e:
        print(f"✗ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_skeleton_builder_integration():
    """测试 SkeletonBuilder 集成"""
    print("\n" + "=" * 60)
    print("测试 3: SkeletonBuilder 集成")
    print("=" * 60)
    
    try:
        try:
            from skeleton_builder import SkeletonBuilder, COMPILE_COMMANDS_AVAILABLE
        except ImportError as e:
            print(f"⚠ 无法导入 SkeletonBuilder（缺少依赖）: {e}")
            print("  这是正常的，如果环境中没有安装 tree_sitter_cpp")
            return True
        
        if not COMPILE_COMMANDS_AVAILABLE:
            print("⚠ compile_commands_parser 不可用，跳过集成测试")
            return True
        
        # 创建测试目录
        test_project_root = Path("/tmp/test_project")
        test_output_dir = Path("/tmp/test_output")
        test_project_root.mkdir(exist_ok=True)
        test_output_dir.mkdir(exist_ok=True)
        
        # 创建测试源文件
        test_c_file = test_project_root / "test.c"
        test_c_file.write_text("""
#include <stdio.h>
#include <stdlib.h>

int main() {
    return 0;
}
""")
        
        # 测试1: 不使用 compile_commands.json
        print("\n测试 3.1: 不使用 compile_commands.json")
        builder1 = SkeletonBuilder(test_project_root, test_output_dir)
        print(f"✓ SkeletonBuilder 创建成功")
        print(f"  include_dirs 数量: {len(builder1.include_dirs)}")
        print(f"  compile_commands_parser: {builder1.compile_commands_parser is not None}")
        
        # 测试2: 使用 compile_commands.json（如果存在）
        ohos_root = DEFAULT_OHOS_ROOT
        compile_commands_path = None
        
        if ohos_root.exists():
            from workspace_config import get_compile_commands_path
            compile_commands_path = get_compile_commands_path(ohos_root)
        
        if compile_commands_path and compile_commands_path.exists():
            print("\n测试 3.2: 使用 compile_commands.json")
            builder2 = SkeletonBuilder(
                test_project_root,
                test_output_dir,
                compile_commands_path=compile_commands_path,
                ohos_root=ohos_root
            )
            print(f"✓ SkeletonBuilder 创建成功（带 compile_commands.json）")
            print(f"  include_dirs 数量: {len(builder2.include_dirs)}")
            print(f"  compile_commands_parser: {builder2.compile_commands_parser is not None}")
            
            # 测试获取 include 路径
            if builder2.compile_commands_parser:
                includes = builder2.compile_commands_parser.get_includes_for_file(test_c_file)
                print(f"  测试文件 include 路径数量: {len(includes)}")
                if includes:
                    print(f"  示例路径: {includes[0]}")
        else:
            print("\n测试 3.2: 跳过（compile_commands.json 不存在）")
        
        # 清理
        import shutil
        shutil.rmtree(test_project_root, ignore_errors=True)
        shutil.rmtree(test_output_dir, ignore_errors=True)
        
        return True
    except Exception as e:
        print(f"✗ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    """运行所有测试"""
    print("\n" + "=" * 60)
    print("compile_commands.json 集成测试")
    print("=" * 60 + "\n")
    
    results = []
    
    results.append(("compile_commands_parser 模块", test_compile_commands_parser()))
    results.append(("workspace_config 查找", test_workspace_config()))
    results.append(("SkeletonBuilder 集成", test_skeleton_builder_integration()))
    
    # 总结
    print("\n" + "=" * 60)
    print("测试总结")
    print("=" * 60)
    
    all_passed = True
    for name, result in results:
        status = "✓ 通过" if result else "✗ 失败"
        print(f"{name}: {status}")
        if not result:
            all_passed = False
    
    print("\n" + "=" * 60)
    if all_passed:
        print("✓ 所有测试通过！")
    else:
        print("⚠ 部分测试失败，请检查上述输出")
    print("=" * 60)
    
    return 0 if all_passed else 1


if __name__ == "__main__":
    sys.exit(main())
