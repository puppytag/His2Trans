#!/usr/bin/env python3
"""
缺失符号诊断工具

这个脚本展示了如何使用诊断分析器来分析编译错误，
而不是简单地给占位符值。

使用方式:
    python3 diagnose_missing_symbols.py --rust-project ./output/my_project --c-source ./dlp_fuse

特点:
    1. 在 C 源码中搜索缺失符号的原始定义
    2. 分析转换失败的具体原因
    3. 给出可操作的建议
    4. 区分"可以安全用占位符"和"需要人工审查"
"""

import argparse
import json
import subprocess
import sys
import logging
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from diagnostic_analyzer import (
    DiagnosticAnalyzer,
    DiagnosticReport,
    DiagnosticResult,
    FailureReason,
    MissingSymbolType,
    format_report,
    save_report
)

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class SmartSymbolHandler:
    """
    智能符号处理器
    
    根据诊断结果，智能决定如何处理缺失的符号：
    - 对于"可以安全用占位符"的：自动生成
    - 对于"需要真实定义"的：从 C 源码提取
    - 对于"需要人工审查"的：标记并报告
    """
    
    # 可以安全使用占位符的情况
    SAFE_FOR_PLACEHOLDER = {
        FailureReason.OPAQUE_TYPE,  # 前向声明/不透明类型
    }
    
    # 需要从 C 源码提取真实定义的情况
    NEED_REAL_DEFINITION = {
        FailureReason.MACRO_NOT_EXPANDED,
        FailureReason.HEADER_NOT_INCLUDED,
        FailureReason.TYPEDEF_CHAIN,
    }
    
    # 需要人工审查的情况
    NEED_MANUAL_REVIEW = {
        FailureReason.CONDITIONAL_COMPILE,
        FailureReason.BINDGEN_FAILURE,
        FailureReason.UNKNOWN,
    }
    
    def __init__(self, report: DiagnosticReport):
        self.report = report
        self.safe_placeholders: List[DiagnosticResult] = []
        self.real_definitions: List[DiagnosticResult] = []
        self.manual_review: List[DiagnosticResult] = []
        
        self._categorize()
    
    def _categorize(self):
        """按处理方式分类"""
        for diag in self.report.diagnostics:
            if diag.failure_reason in self.SAFE_FOR_PLACEHOLDER:
                self.safe_placeholders.append(diag)
            elif diag.failure_reason in self.NEED_REAL_DEFINITION:
                self.real_definitions.append(diag)
            else:
                self.manual_review.append(diag)
    
    def generate_rust_types(self) -> str:
        """
        生成 Rust 类型定义
        
        对于可以安全使用占位符的类型，生成占位符
        对于需要真实定义的类型，尝试从 C 定义转换
        """
        lines = [
            "// =====================================================",
            "// 自动生成的类型定义",
            "// 来源: 诊断分析器",
            "// =====================================================",
            "",
            "use std::ffi::{c_void, c_char, c_int, c_long};",
            "",
        ]
        
        # 1. 安全的占位符类型
        if self.safe_placeholders:
            lines.append("// ----- 不透明类型 (安全使用占位符) -----")
            for diag in self.safe_placeholders:
                if diag.symbol_type == MissingSymbolType.TYPE:
                    lines.append(f"// 原因: {diag.failure_reason.value}")
                    if diag.c_source_evidence:
                        lines.append(f"// 来源: {diag.c_source_evidence.file_path}:{diag.c_source_evidence.line_number}")
                    lines.append(f"#[repr(C)]")
                    lines.append(f"pub struct {diag.symbol_name} {{ _opaque: [u8; 0] }}")
                    lines.append("")
        
        # 2. 需要真实定义的类型
        if self.real_definitions:
            lines.append("")
            lines.append("// ----- 需要真实定义的类型 (尝试从 C 转换) -----")
            for diag in self.real_definitions:
                if diag.symbol_type == MissingSymbolType.TYPE:
                    lines.append(f"// 原因: {diag.failure_reason.value}")
                    if diag.original_definition:
                        lines.append(f"// 原始 C 定义:")
                        for line in diag.original_definition.split('\n')[:5]:
                            lines.append(f"//   {line}")
                    
                    # 尝试简单转换
                    rust_def = self._try_convert_c_to_rust(diag)
                    if rust_def:
                        lines.append(rust_def)
                    else:
                        lines.append(f"// TODO: 需要手动转换")
                        lines.append(f"// #[repr(C)]")
                        lines.append(f"// pub struct {diag.symbol_name} {{ ... }}")
                    lines.append("")
        
        # 3. 需要人工审查的
        if self.manual_review:
            lines.append("")
            lines.append("// ----- 需要人工审查的类型 -----")
            for diag in self.manual_review:
                if diag.symbol_type == MissingSymbolType.TYPE:
                    lines.append(f"// ⚠️ {diag.symbol_name}: 需要人工审查")
                    lines.append(f"// 原因: {diag.failure_reason.value}")
                    for note in diag.diagnostic_notes:
                        lines.append(f"// {note}")
                    lines.append(f"// 临时占位符 (可能不正确):")
                    lines.append(f"#[repr(C)]")
                    lines.append(f"pub struct {diag.symbol_name} {{ _opaque: [u8; 0] }}")
                    lines.append("")
        
        return '\n'.join(lines)
    
    def generate_rust_constants(self) -> str:
        """生成 Rust 常量定义"""
        lines = [
            "// =====================================================",
            "// 自动生成的常量定义",
            "// =====================================================",
            "",
        ]
        
        for diag in self.report.diagnostics:
            if diag.symbol_type != MissingSymbolType.CONSTANT:
                continue
            
            lines.append(f"// {diag.symbol_name}")
            lines.append(f"// 原因: {diag.failure_reason.value}")
            
            if diag.original_definition:
                # 尝试从原始定义提取值
                value = self._extract_constant_value(diag)
                if value:
                    lines.append(f"// 原始: {diag.original_definition[:80]}")
                    lines.append(f"pub const {diag.symbol_name}: i32 = {value};")
                else:
                    lines.append(f"// ⚠️ 无法提取值，使用占位符")
                    lines.append(f"// 原始: {diag.original_definition[:80]}")
                    lines.append(f"pub const {diag.symbol_name}: i32 = 0; // TODO: 需要真实值")
            else:
                lines.append(f"// ⚠️ 未找到原始定义")
                lines.append(f"pub const {diag.symbol_name}: i32 = 0; // TODO: 需要查找真实值")
            
            lines.append("")
        
        return '\n'.join(lines)
    
    def _try_convert_c_to_rust(self, diag: DiagnosticResult) -> Optional[str]:
        """尝试将 C 类型定义转换为 Rust"""
        if not diag.original_definition:
            return None
        
        c_def = diag.original_definition
        
        # 简单的 typedef 转换
        # typedef int MyInt;
        typedef_match = re.match(
            r'typedef\s+(int|char|short|long|unsigned\s+\w+|size_t|ssize_t)\s+(\w+)\s*;',
            c_def
        )
        if typedef_match:
            c_type, name = typedef_match.groups()
            rust_type = self._c_type_to_rust(c_type)
            return f"pub type {name} = {rust_type};"
        
        # typedef struct
        struct_typedef_match = re.match(
            r'typedef\s+struct\s+\w*\s*\{([^}]*)\}\s*(\w+)\s*;',
            c_def,
            re.DOTALL
        )
        if struct_typedef_match:
            body, name = struct_typedef_match.groups()
            # 简化处理：生成占位符
            return f"#[repr(C)]\npub struct {name} {{ _opaque: [u8; 0] }} // TODO: 转换字段"
        
        return None
    
    def _c_type_to_rust(self, c_type: str) -> str:
        """C 类型到 Rust 类型的简单映射"""
        mapping = {
            'int': 'c_int',
            'char': 'c_char',
            'short': 'i16',
            'long': 'c_long',
            'unsigned int': 'u32',
            'unsigned char': 'u8',
            'unsigned short': 'u16',
            'unsigned long': 'u64',
            'size_t': 'usize',
            'ssize_t': 'isize',
        }
        return mapping.get(c_type.strip(), 'c_int')
    
    def _extract_constant_value(self, diag: DiagnosticResult) -> Optional[str]:
        """从原始定义提取常量值"""
        import re
        
        if not diag.original_definition:
            return None
        
        c_def = diag.original_definition
        
        # #define NAME value
        define_match = re.search(
            rf'#\s*define\s+{re.escape(diag.symbol_name)}\s+(-?\d+|0x[0-9a-fA-F]+)',
            c_def
        )
        if define_match:
            return define_match.group(1)
        
        # const int NAME = value;
        const_match = re.search(
            rf'{re.escape(diag.symbol_name)}\s*=\s*(-?\d+|0x[0-9a-fA-F]+)',
            c_def
        )
        if const_match:
            return const_match.group(1)
        
        return None
    
    def print_summary(self):
        """打印处理摘要"""
        print("\n" + "=" * 60)
        print("📊 诊断摘要")
        print("=" * 60)
        print(f"  总计缺失符号: {self.report.total_missing_symbols}")
        print(f"  ✅ 可安全使用占位符: {len(self.safe_placeholders)}")
        print(f"  📝 需要真实定义: {len(self.real_definitions)}")
        print(f"  ⚠️  需要人工审查: {len(self.manual_review)}")
        
        if self.real_definitions:
            print("\n需要真实定义的符号:")
            for diag in self.real_definitions[:10]:
                loc = ""
                if diag.c_source_evidence:
                    loc = f" (来源: {Path(diag.c_source_evidence.file_path).name}:{diag.c_source_evidence.line_number})"
                print(f"  - {diag.symbol_name}{loc}")
        
        if self.manual_review:
            print("\n需要人工审查的符号:")
            for diag in self.manual_review[:10]:
                print(f"  - {diag.symbol_name}: {diag.failure_reason.value}")
                for note in diag.diagnostic_notes[:2]:
                    print(f"      {note}")


def run_cargo_check(project_dir: Path) -> str:
    """运行 cargo check 并获取错误输出"""
    try:
        result = subprocess.run(
            ["cargo", "check", "--message-format=short"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=120
        )
        return result.stderr
    except subprocess.TimeoutExpired:
        logger.error("cargo check 超时")
        return ""
    except Exception as e:
        logger.error(f"运行 cargo check 失败: {e}")
        return ""


def main():
    parser = argparse.ArgumentParser(description="诊断缺失的符号")
    parser.add_argument("--rust-project", "-r", type=Path, required=True,
                       help="Rust 项目目录")
    parser.add_argument("--c-source", "-c", type=Path, required=True, nargs='+',
                       help="C 源码目录")
    parser.add_argument("--include", "-I", type=Path, nargs='*', default=[],
                       help="头文件搜索目录")
    parser.add_argument("--output", "-o", type=Path,
                       help="输出报告路径")
    parser.add_argument("--generate-code", "-g", action="store_true",
                       help="生成 Rust 代码")
    
    args = parser.parse_args()
    
    # 检查路径
    if not args.rust_project.exists():
        logger.error(f"Rust 项目目录不存在: {args.rust_project}")
        sys.exit(1)
    
    for c_dir in args.c_source:
        if not c_dir.exists():
            logger.warning(f"C 源码目录不存在: {c_dir}")
    
    # 运行 cargo check
    print(f"🔍 正在检查 {args.rust_project}...")
    error_output = run_cargo_check(args.rust_project)
    
    if not error_output:
        print("✅ 编译没有错误!")
        sys.exit(0)
    
    # 创建诊断分析器
    analyzer = DiagnosticAnalyzer(
        c_source_dirs=args.c_source,
        include_dirs=args.include
    )
    
    # 分析
    print("🔬 正在分析编译错误...")
    report = analyzer.analyze_compilation_errors(error_output, args.rust_project)
    
    # 打印报告
    print(format_report(report))
    
    # 保存报告
    if args.output:
        save_report(report, args.output)
    
    # 智能处理
    handler = SmartSymbolHandler(report)
    handler.print_summary()
    
    # 生成代码
    if args.generate_code:
        print("\n" + "=" * 60)
        print("📝 生成的 Rust 类型定义")
        print("=" * 60)
        print(handler.generate_rust_types())
        
        print("\n" + "=" * 60)
        print("📝 生成的 Rust 常量定义")
        print("=" * 60)
        print(handler.generate_rust_constants())


# 导入需要的模块
import re

if __name__ == "__main__":
    main()



