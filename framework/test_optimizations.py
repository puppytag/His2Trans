#!/usr/bin/env python3
"""
优化验证测试脚本

测试三个优化：
1. 头文件预检方案 (Pre-flight Include Check)
2. RustCodeBuilder 安全代码生成
3. 动态宏学习系统 (MacroLearner)
"""

import sys
from pathlib import Path

def test_rust_code_builder():
    """测试 RustCodeBuilder"""
    print("\n" + "=" * 60)
    print("测试 1: RustCodeBuilder")
    print("=" * 60)
    
    try:
        from rust_code_builder import RustCodeBuilder, create_opaque_type, create_const
        
        # 测试基本功能
        builder = RustCodeBuilder()
        
        # 添加模块头
        builder.add_line("//! Test module")
        builder.add_allow("dead_code", "unused_variables")
        builder.add_line("")
        
        # 添加不透明类型
        builder.add_opaque_struct("HnpCfgInfo", doc="Test opaque type")
        
        # 添加普通结构体
        builder.add_struct("Point", doc="A 2D point")
        builder.add_field("x", "i32")
        builder.add_field("y", "i32")
        builder.end_block()
        
        # 添加枚举
        builder.add_enum("Color", repr_type="u32")
        builder.add_variant("Red", 0)
        builder.add_variant("Green", 1)
        builder.add_variant("Blue", 2)
        builder.end_block()
        
        # 添加常量
        builder.add_const("MAX_SIZE", "usize", "1024")
        
        # 添加类型别名
        builder.add_type_alias("BOOL", "i32")
        
        code = builder.build()
        
        # 验证生成的代码
        assert "#[repr(C)]" in code, "Missing #[repr(C)]"
        assert "pub struct HnpCfgInfo" in code, "Missing HnpCfgInfo struct"
        assert "pub struct Point" in code, "Missing Point struct"
        assert "pub x: i32" in code, "Missing x field"
        assert "pub enum Color" in code, "Missing Color enum"
        assert "Red = 0" in code, "Missing Red variant"
        assert "pub const MAX_SIZE: usize = 1024;" in code, "Missing MAX_SIZE const"
        assert "pub type BOOL = i32;" in code, "Missing BOOL type alias"
        
        print("✅ RustCodeBuilder 测试通过")
        print(f"   生成了 {len(code.split(chr(10)))} 行代码")
        
        # 测试便捷函数
        opaque = create_opaque_type("TestType", "Test doc")
        assert "pub struct TestType" in opaque
        print("✅ create_opaque_type 测试通过")
        
        const = create_const("TEST_CONST", "i32", "42")
        assert "pub const TEST_CONST: i32 = 42;" in const
        print("✅ create_const 测试通过")
        
        return True
    except Exception as e:
        print(f"❌ RustCodeBuilder 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_macro_learner():
    """测试 MacroLearner"""
    print("\n" + "=" * 60)
    print("测试 2: MacroLearner")
    print("=" * 60)
    
    try:
        from macro_learner import MacroLearner, get_global_macro_learner, expand_macros, get_gcc_macro_args
        
        # 创建临时学习器（不污染全局存储）
        import tempfile
        with tempfile.NamedTemporaryFile(suffix='.json', delete=False) as tmp:
            tmp_path = Path(tmp.name)
        
        learner = MacroLearner(storage_path=tmp_path)
        
        # 测试内置宏
        assert learner.get_expansion("STATIC") == "static"
        assert learner.get_expansion("UINT32") == "unsigned int"
        assert learner.get_expansion("VOID") == "void"
        print("✅ 内置宏测试通过")
        
        # 测试宏展开
        test_code = """
STATIC UINT32 TestFunction(VOID)
{
    UINT32 result = 0;
    return result;
}
"""
        expanded = learner.expand_all(test_code)
        assert "static" in expanded
        assert "unsigned int" in expanded
        assert "void" in expanded
        assert "STATIC" not in expanded  # 宏应该被展开
        print("✅ 宏展开测试通过")
        
        # 测试 GCC 参数生成
        gcc_args = learner.get_gcc_define_args()
        assert any("-DSTATIC=static" in arg for arg in gcc_args)
        assert any("-DUINT32=unsigned int" in arg for arg in gcc_args)
        print(f"✅ GCC 参数生成测试通过 (共 {len(gcc_args)} 个参数)")
        
        # 测试动态学习
        learner.add_macro("NEW_MACRO", "new_expansion", source="test")
        assert learner.get_expansion("NEW_MACRO") == "new_expansion"
        print("✅ 动态宏添加测试通过")
        
        # 测试持久化
        learner.save_to_storage()
        
        # 创建新学习器并加载
        learner2 = MacroLearner(storage_path=tmp_path, include_builtin=False)
        assert learner2.get_expansion("NEW_MACRO") == "new_expansion"
        print("✅ 持久化测试通过")
        
        # 清理
        tmp_path.unlink()
        
        # 测试全局实例
        global_learner = get_global_macro_learner()
        assert global_learner is not None
        print("✅ 全局实例测试通过")
        
        # 统计
        counts = learner.get_macro_count()
        print(f"   宏统计: {counts}")
        
        return True
    except Exception as e:
        print(f"❌ MacroLearner 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_preflight_check():
    """测试头文件预检"""
    print("\n" + "=" * 60)
    print("测试 3: 头文件预检 (Pre-flight Check)")
    print("=" * 60)
    
    try:
        from skeleton_builder import SkeletonBuilder
        
        # 创建临时目录进行测试
        import tempfile
        with tempfile.TemporaryDirectory() as tmp_dir:
            tmp_path = Path(tmp_dir)
            
            # 创建测试头文件
            header_dir = tmp_path / "include"
            header_dir.mkdir()
            
            header1 = header_dir / "test.h"
            header1.write_text("""
#ifndef TEST_H
#define TEST_H

#include <stdio.h>
#include "other.h"

typedef int TestType;

#endif
""")
            
            other_h = header_dir / "other.h"
            other_h.write_text("""
#ifndef OTHER_H
#define OTHER_H

typedef struct {
    int x;
    int y;
} Point;

#endif
""")
            
            # 创建输出目录
            output_dir = tmp_path / "output"
            output_dir.mkdir()
            (output_dir / "src").mkdir()
            
            # 创建 SkeletonBuilder
            builder = SkeletonBuilder(
                project_root=tmp_path,
                output_dir=output_dir
            )
            
            # 手动添加 include_dirs
            builder.include_dirs.append(header_dir)
            
            # 测试 _resolve_include_path
            result = builder._resolve_include_path("test.h", [header_dir])
            assert result is not None
            print("✅ _resolve_include_path 测试通过")
            
            # 测试 _preflight_check_includes
            all_resolved, new_paths = builder._preflight_check_includes([header1], verbose=False)
            print(f"   预检结果: all_resolved={all_resolved}, new_paths={len(new_paths)}")
            print("✅ _preflight_check_includes 测试通过")
        
        return True
    except Exception as e:
        print(f"❌ 头文件预检测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    """运行所有测试"""
    print("=" * 60)
    print("C2Rust 框架优化验证测试")
    print("=" * 60)
    
    results = []
    
    # 测试 RustCodeBuilder
    results.append(("RustCodeBuilder", test_rust_code_builder()))
    
    # 测试 MacroLearner
    results.append(("MacroLearner", test_macro_learner()))
    
    # 测试头文件预检
    results.append(("Pre-flight Check", test_preflight_check()))
    
    # 总结
    print("\n" + "=" * 60)
    print("测试结果汇总")
    print("=" * 60)
    
    all_passed = True
    for name, passed in results:
        status = "✅ 通过" if passed else "❌ 失败"
        print(f"  {name}: {status}")
        if not passed:
            all_passed = False
    
    print("")
    if all_passed:
        print("🎉 所有测试通过！")
        return 0
    else:
        print("⚠️ 部分测试失败，请检查错误信息。")
        return 1


if __name__ == "__main__":
    sys.exit(main())

