#!/usr/bin/env python3
"""
预处理功能测试示例

演示如何使用 Preprocess-First 功能
"""

import os
import sys
from pathlib import Path

# 添加项目根目录到 Python 路径
sys.path.insert(0, str(Path(__file__).parent.parent))

from compile_commands_parser import (
    CompileCommandsParser,
    ContextSelectionStrategy,
    PreprocessingContext
)


def example_basic_preprocessing():
    """示例1：基本预处理功能"""
    print("=" * 60)
    print("示例1：基本预处理功能")
    print("=" * 60)

    # 假设你有一个 compile_commands.json 文件
    compile_db_path = Path("path/to/compile_commands.json")

    if not compile_db_path.exists():
        print(f"错误：compile_commands.json 不存在: {compile_db_path}")
        print("请修改路径或创建测试文件")
        return

    # 创建解析器
    parser = CompileCommandsParser(compile_db_path)

    # 测试源文件
    source_file = Path("path/to/your/source.c")

    if not source_file.exists():
        print(f"错误：源文件不存在: {source_file}")
        return

    # 使用 ACTIVE 策略选择预处理上下文（默认策略）
    print(f"\n正在预处理: {source_file.name}")
    context = parser.select_preprocessing_context(
        source_file,
        strategy=ContextSelectionStrategy.ACTIVE,
        target_config="rk3568",  # 指定目标配置
        output_dir=Path("./test_preprocessed")
    )

    if context and context.preprocessed_file:
        print(f"✓ 预处理成功！")
        print(f"  - 预处理文件: {context.preprocessed_file}")
        print(f"  - 函数数量: {context.function_count}")
        print(f"  - 宏数量: {context.macro_count}")
        print(f"  - 行号映射: {len(context.line_mapping)} 行")

        # 显示前几行映射
        print(f"\n前10行映射:")
        for i, (line_num, (orig_file, orig_line)) in enumerate(list(context.line_mapping.items())[:10]):
            print(f"  .i:{line_num} → {Path(orig_file).name}:{orig_line}")
    else:
        print(f"✗ 预处理失败")
        if context:
            print(f"  错误: {context.error}")


def example_strategy_comparison():
    """示例2：比较不同策略"""
    print("\n" + "=" * 60)
    print("示例2：比较不同预处理策略")
    print("=" * 60)

    compile_db_path = Path("path/to/compile_commands.json")
    source_file = Path("path/to/your/source.c")

    if not compile_db_path.exists() or not source_file.exists():
        print("跳过（文件不存在）")
        return

    parser = CompileCommandsParser(compile_db_path)

    # 获取所有编译条目
    entries = parser.get_all_entries_for_file(source_file)
    print(f"\n找到 {len(entries)} 个编译配置")

    if len(entries) > 1:
        # 对每个配置预处理并比较
        print(f"\n比较各配置的预处理结果:")
        for i, entry in enumerate(entries[:3], 1):  # 只测试前3个
            directory = entry.get('directory', '未知')
            print(f"\n配置 {i}: {directory}")

            context = parser.preprocess_with_context(
                source_file,
                entry,
                output_dir=Path("./test_preprocessed")
            )

            if context and not context.error:
                print(f"  ✓ 函数: {context.function_count}, 宏: {context.macro_count}")
                print(f"  评分: {context.function_count * 10 + context.macro_count}")
            else:
                print(f"  ✗ 失败: {context.error if context else '未知错误'}")


def example_line_mapping():
    """示例3：行号映射使用"""
    print("\n" + "=" * 60)
    print("示例3：行号映射实际应用")
    print("=" * 60)

    compile_db_path = Path("path/to/compile_commands.json")
    source_file = Path("path/to/your/source.c")

    if not compile_db_path.exists() or not source_file.exists():
        print("跳过（文件不存在）")
        return

    parser = CompileCommandsParser(compile_db_path)
    context = parser.select_preprocessing_context(
        source_file,
        strategy=ContextSelectionStrategy.BEST,
        output_dir=Path("./test_preprocessed")
    )

    if not context or not context.preprocessed_file:
        print("预处理失败")
        return

    # 读取预处理文件并显示映射
    print(f"\n预处理文件内容片段（含映射）:")
    with open(context.preprocessed_file, 'r', encoding='utf-8', errors='ignore') as f:
        for line_num, line in enumerate(f, 1):
            if line_num > 50:  # 只显示前50行
                break

            # 获取映射
            if line_num in context.line_mapping:
                orig_file, orig_line = context.line_mapping[line_num]
                orig_file_name = Path(orig_file).name

                # 只显示源文件的行（不显示头文件）
                if orig_file_name == source_file.name:
                    print(f"[{orig_file_name}:{orig_line:4d}] {line.rstrip()}")
            elif not line.startswith('#'):
                # 非 #line 指令的行
                print(f"[{line_num:4d}] {line.rstrip()}")


def example_environment_config():
    """示例4：通过环境变量配置"""
    print("\n" + "=" * 60)
    print("示例4：环境变量配置示例")
    print("=" * 60)

    # 设置环境变量
    os.environ["USE_PREPROCESSING"] = "true"
    os.environ["PREPROCESSING_STRATEGY"] = "active"  # 默认策略
    os.environ["TARGET_CONFIG"] = "rk3568"  # ACTIVE 策略需要
    os.environ["COMPILE_COMMANDS_PATH"] = "/path/to/compile_commands.json"
    os.environ["PREPROCESS_OUTPUT_DIR"] = "./translation_outputs/.preprocessed"

    print("环境变量配置:")
    for key in ["USE_PREPROCESSING", "PREPROCESSING_STRATEGY", "TARGET_CONFIG", "COMPILE_COMMANDS_PATH", "PREPROCESS_OUTPUT_DIR"]:
        print(f"  {key} = {os.environ.get(key)}")

    print("\n现在运行 get_dependencies.py 或 translate.py 将自动使用预处理")
    print("示例:")
    print("  python get_dependencies.py")
    print("  python translate.py --stage 1")
    print("\n注意：ACTIVE 策略（默认）需要设置 TARGET_CONFIG 环境变量")


def main():
    """主函数"""
    print("Preprocess-First 功能测试示例\n")

    # 运行各个示例
    example_basic_preprocessing()
    example_strategy_comparison()
    example_line_mapping()
    example_environment_config()

    print("\n" + "=" * 60)
    print("测试完成")
    print("=" * 60)
    print("\n提示：请修改示例中的文件路径以匹配你的实际项目")


if __name__ == "__main__":
    main()
