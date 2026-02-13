#!/usr/bin/env python3
"""
诊断分析器 (Diagnostic Analyzer)

与其单纯为了通过编译而给占位符值，不如：
1. 追溯来源 - 在 C 源码中找到原始定义
2. 分析原因 - 为什么转换/识别失败
3. 收集证据 - 给出具体的代码位置和上下文
4. 提供可操作的建议

核心理念：
- 不要用"糊弄"的方式通过编译
- 要找到问题的根本原因
- 生成可读的诊断报告
"""

import re
import os
import json
import logging
import subprocess
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Set, Any
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum

logger = logging.getLogger(__name__)


class MissingSymbolType(Enum):
    """缺失符号的类型"""
    TYPE = "type"           # 类型（struct, enum, typedef）
    CONSTANT = "constant"   # 常量（#define, const）
    FUNCTION = "function"   # 函数
    MACRO = "macro"         # 宏


class FailureReason(Enum):
    """失败原因分类"""
    MACRO_NOT_EXPANDED = "macro_not_expanded"           # 宏未展开
    HEADER_NOT_INCLUDED = "header_not_included"         # 头文件未包含
    CROSS_FILE_DEPENDENCY = "cross_file_dependency"     # 跨文件依赖
    CONDITIONAL_COMPILE = "conditional_compile"         # 条件编译排除
    TYPEDEF_CHAIN = "typedef_chain"                     # typedef 链未解析
    OPAQUE_TYPE = "opaque_type"                         # 不透明类型（前向声明）
    BINDGEN_FAILURE = "bindgen_failure"                 # bindgen 转换失败
    TREE_SITTER_PARSE_ERROR = "tree_sitter_parse_error" # Tree-sitter 解析失败
    UNKNOWN = "unknown"                                  # 未知原因


@dataclass
class SourceEvidence:
    """来源证据"""
    file_path: str                  # 文件路径
    line_number: int                # 行号
    column: int = 0                 # 列号
    code_snippet: str = ""          # 代码片段（含上下文）
    surrounding_context: str = ""   # 更大范围的上下文


@dataclass
class DiagnosticResult:
    """诊断结果"""
    symbol_name: str                           # 符号名称
    symbol_type: MissingSymbolType             # 符号类型
    failure_reason: FailureReason = FailureReason.UNKNOWN  # 失败原因
    confidence: float = 0.0                    # 置信度 (0-1)
    
    # 证据
    c_source_evidence: Optional[SourceEvidence] = None  # C 源码中的定义位置
    header_evidence: Optional[SourceEvidence] = None    # 头文件中的定义位置
    rust_error_location: Optional[SourceEvidence] = None # Rust 编译错误位置
    
    # 分析结果
    original_definition: str = ""              # 原始定义（如果找到）
    dependent_symbols: List[str] = field(default_factory=list)  # 依赖的其他符号
    suggested_fix: str = ""                    # 建议的修复方式
    manual_review_required: bool = False       # 是否需要人工审查
    
    # 元数据
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())
    diagnostic_notes: List[str] = field(default_factory=list)  # 诊断备注


@dataclass
class DiagnosticReport:
    """诊断报告"""
    project_name: str
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())
    
    # 分类统计
    total_missing_symbols: int = 0
    by_type: Dict[str, int] = field(default_factory=dict)
    by_reason: Dict[str, int] = field(default_factory=dict)
    
    # 详细结果
    diagnostics: List[DiagnosticResult] = field(default_factory=list)
    
    # 需要人工审查的
    manual_review_items: List[DiagnosticResult] = field(default_factory=list)
    
    # 汇总建议
    summary_recommendations: List[str] = field(default_factory=list)


class DiagnosticAnalyzer:
    """
    诊断分析器
    
    职责：
    1. 解析编译错误，提取缺失的符号
    2. 在 C 源码中搜索原始定义
    3. 分析失败原因
    4. 生成诊断报告
    """
    
    def __init__(
        self,
        c_source_dirs: List[Path],
        include_dirs: List[Path] = None,
        compile_commands_path: Optional[Path] = None
    ):
        """
        初始化诊断分析器
        
        Args:
            c_source_dirs: C 源码目录列表
            include_dirs: 头文件搜索目录
            compile_commands_path: compile_commands.json 路径（可选）
        """
        self.c_source_dirs = [Path(d) for d in c_source_dirs]
        self.include_dirs = [Path(d) for d in (include_dirs or [])]
        self.compile_commands_path = compile_commands_path
        
        # 缓存：文件内容
        self._file_cache: Dict[str, str] = {}
        
        # 缓存：符号定义位置
        self._symbol_location_cache: Dict[str, SourceEvidence] = {}
        
        # 加载 compile_commands.json（如果有）
        self._compile_commands = self._load_compile_commands()
    
    def _load_compile_commands(self) -> Dict[str, Dict]:
        """加载 compile_commands.json"""
        if not self.compile_commands_path or not self.compile_commands_path.exists():
            return {}
        
        try:
            with open(self.compile_commands_path, 'r') as f:
                commands = json.load(f)
            return {cmd['file']: cmd for cmd in commands}
        except Exception as e:
            logger.warning(f"加载 compile_commands.json 失败: {e}")
            return {}
    
    def _get_file_content(self, file_path: str) -> Optional[str]:
        """获取文件内容（带缓存）"""
        if file_path not in self._file_cache:
            try:
                with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
                    self._file_cache[file_path] = f.read()
            except Exception as e:
                logger.debug(f"无法读取文件 {file_path}: {e}")
                return None
        return self._file_cache[file_path]
    
    def analyze_compilation_errors(
        self,
        error_output: str,
        rust_project_dir: Path
    ) -> DiagnosticReport:
        """
        分析编译错误并生成诊断报告
        
        Args:
            error_output: cargo check 的错误输出
            rust_project_dir: Rust 项目目录
            
        Returns:
            诊断报告
        """
        report = DiagnosticReport(
            project_name=rust_project_dir.name
        )
        
        # 提取缺失的符号
        missing_symbols = self._extract_missing_symbols(error_output)
        report.total_missing_symbols = len(missing_symbols)
        
        # 对每个符号进行诊断
        for symbol_name, symbol_type, error_location in missing_symbols:
            diagnostic = self._diagnose_symbol(
                symbol_name, 
                symbol_type, 
                error_location,
                rust_project_dir
            )
            
            report.diagnostics.append(diagnostic)
            
            # 统计
            type_key = diagnostic.symbol_type.value
            reason_key = diagnostic.failure_reason.value
            report.by_type[type_key] = report.by_type.get(type_key, 0) + 1
            report.by_reason[reason_key] = report.by_reason.get(reason_key, 0) + 1
            
            # 需要人工审查的
            if diagnostic.manual_review_required:
                report.manual_review_items.append(diagnostic)
        
        # 生成汇总建议
        report.summary_recommendations = self._generate_recommendations(report)
        
        return report
    
    def _extract_missing_symbols(
        self, 
        error_output: str
    ) -> List[Tuple[str, MissingSymbolType, Optional[SourceEvidence]]]:
        """
        从编译错误中提取缺失的符号
        
        Returns:
            [(符号名, 符号类型, 错误位置), ...]
        """
        results = []
        seen = set()
        
        # E0412: cannot find type
        for match in re.finditer(
            r'error\[E0412\]: cannot find type `(\w+)`.*?'
            r'(?:--> ([^:]+):(\d+):(\d+))?',
            error_output, 
            re.DOTALL
        ):
            symbol_name = match.group(1)
            if symbol_name in seen:
                continue
            seen.add(symbol_name)
            
            error_loc = None
            if match.group(2):
                error_loc = SourceEvidence(
                    file_path=match.group(2),
                    line_number=int(match.group(3)) if match.group(3) else 0,
                    column=int(match.group(4)) if match.group(4) else 0
                )
            
            results.append((symbol_name, MissingSymbolType.TYPE, error_loc))
        
        # E0425: cannot find value (常量)
        for match in re.finditer(
            r'error\[E0425\]: cannot find value `(\w+)`.*?'
            r'(?:--> ([^:]+):(\d+):(\d+))?',
            error_output,
            re.DOTALL
        ):
            symbol_name = match.group(1)
            if symbol_name in seen:
                continue
            seen.add(symbol_name)
            
            error_loc = None
            if match.group(2):
                error_loc = SourceEvidence(
                    file_path=match.group(2),
                    line_number=int(match.group(3)) if match.group(3) else 0,
                    column=int(match.group(4)) if match.group(4) else 0
                )
            
            results.append((symbol_name, MissingSymbolType.CONSTANT, error_loc))
        
        # E0433: failed to resolve (可能是模块或类型)
        for match in re.finditer(
            r'error\[E0433\]: failed to resolve.*?`(\w+)`',
            error_output
        ):
            symbol_name = match.group(1)
            if symbol_name not in seen:
                seen.add(symbol_name)
                results.append((symbol_name, MissingSymbolType.TYPE, None))
        
        return results
    
    def _diagnose_symbol(
        self,
        symbol_name: str,
        symbol_type: MissingSymbolType,
        error_location: Optional[SourceEvidence],
        rust_project_dir: Path
    ) -> DiagnosticResult:
        """
        诊断单个符号
        
        这是核心方法：找到符号在 C 源码中的原始定义，分析为什么转换失败
        """
        diagnostic = DiagnosticResult(
            symbol_name=symbol_name,
            symbol_type=symbol_type,
            rust_error_location=error_location
        )
        
        # 步骤 1: 在 C 源码中搜索定义
        c_definition = self._find_c_definition(symbol_name, symbol_type)
        
        if c_definition:
            diagnostic.c_source_evidence = c_definition['evidence']
            diagnostic.original_definition = c_definition['definition']
            diagnostic.confidence = 0.9
            
            # 步骤 2: 分析为什么转换失败
            failure_analysis = self._analyze_failure_reason(
                symbol_name, 
                symbol_type,
                c_definition
            )
            diagnostic.failure_reason = failure_analysis['reason']
            diagnostic.diagnostic_notes = failure_analysis['notes']
            diagnostic.dependent_symbols = failure_analysis.get('dependencies', [])
            diagnostic.suggested_fix = failure_analysis.get('suggested_fix', '')
            
            # 如果原因复杂，标记需要人工审查
            if diagnostic.failure_reason in [
                FailureReason.CONDITIONAL_COMPILE,
                FailureReason.TYPEDEF_CHAIN,
                FailureReason.UNKNOWN
            ]:
                diagnostic.manual_review_required = True
        else:
            # 没找到 C 定义
            diagnostic.failure_reason = FailureReason.UNKNOWN
            diagnostic.confidence = 0.3
            diagnostic.manual_review_required = True
            diagnostic.diagnostic_notes.append(
                f"⚠️ 未能在 C 源码中找到 '{symbol_name}' 的定义"
            )
            diagnostic.diagnostic_notes.append(
                "可能原因：1) 来自未包含的头文件 2) 是宏定义 3) 是条件编译排除的代码"
            )
            
            # 尝试在头文件中搜索
            header_result = self._search_in_headers(symbol_name, symbol_type)
            if header_result:
                diagnostic.header_evidence = header_result['evidence']
                diagnostic.original_definition = header_result['definition']
                diagnostic.failure_reason = FailureReason.HEADER_NOT_INCLUDED
                diagnostic.confidence = 0.7
                diagnostic.diagnostic_notes.append(
                    f"✓ 在头文件中找到定义: {header_result['evidence'].file_path}"
                )
        
        return diagnostic
    
    def _find_c_definition(
        self, 
        symbol_name: str, 
        symbol_type: MissingSymbolType
    ) -> Optional[Dict]:
        """
        在 C 源码中搜索符号的定义
        
        Returns:
            {
                'evidence': SourceEvidence,
                'definition': str,
                'context': str
            }
        """
        # 构建搜索模式
        patterns = self._get_search_patterns(symbol_name, symbol_type)
        
        # 在所有源文件中搜索
        for source_dir in self.c_source_dirs:
            if not source_dir.exists():
                continue
                
            for file_path in source_dir.rglob('*'):
                if not file_path.suffix in ['.c', '.cpp', '.h', '.hpp', '.cc']:
                    continue
                
                content = self._get_file_content(str(file_path))
                if not content:
                    continue
                
                for pattern, pattern_desc in patterns:
                    match = re.search(pattern, content, re.MULTILINE | re.DOTALL)
                    if match:
                        line_number = content[:match.start()].count('\n') + 1
                        
                        # 提取代码片段（包含上下文）
                        lines = content.split('\n')
                        start_line = max(0, line_number - 4)
                        end_line = min(len(lines), line_number + 6)
                        code_snippet = '\n'.join(
                            f"{i+1:4d} | {lines[i]}" 
                            for i in range(start_line, end_line)
                        )
                        
                        return {
                            'evidence': SourceEvidence(
                                file_path=str(file_path),
                                line_number=line_number,
                                code_snippet=code_snippet,
                                surrounding_context=match.group(0)[:500]
                            ),
                            'definition': match.group(0).strip(),
                            'pattern_desc': pattern_desc
                        }
        
        return None
    
    def _get_search_patterns(
        self, 
        symbol_name: str, 
        symbol_type: MissingSymbolType
    ) -> List[Tuple[str, str]]:
        """
        获取搜索模式
        
        Returns:
            [(regex_pattern, description), ...]
        """
        patterns = []
        
        if symbol_type == MissingSymbolType.TYPE:
            # struct 定义
            patterns.append((
                rf'\bstruct\s+{re.escape(symbol_name)}\s*\{{[^}}]*\}}',
                f"struct {symbol_name} {{ ... }}"
            ))
            # typedef struct
            patterns.append((
                rf'\btypedef\s+struct\s+\w*\s*\{{[^}}]*\}}\s*{re.escape(symbol_name)}\s*;',
                f"typedef struct {{ ... }} {symbol_name};"
            ))
            # typedef 其他类型
            patterns.append((
                rf'\btypedef\s+[^;]+\s+{re.escape(symbol_name)}\s*;',
                f"typedef ... {symbol_name};"
            ))
            # enum 定义
            patterns.append((
                rf'\benum\s+{re.escape(symbol_name)}\s*\{{[^}}]*\}}',
                f"enum {symbol_name} {{ ... }}"
            ))
            # 前向声明
            patterns.append((
                rf'\bstruct\s+{re.escape(symbol_name)}\s*;',
                f"struct {symbol_name}; (forward declaration)"
            ))
            
        elif symbol_type == MissingSymbolType.CONSTANT:
            # #define 常量
            patterns.append((
                rf'#\s*define\s+{re.escape(symbol_name)}\s+[^\n]+',
                f"#define {symbol_name} ..."
            ))
            # const 变量
            patterns.append((
                rf'\bconst\s+\w+\s+{re.escape(symbol_name)}\s*=\s*[^;]+;',
                f"const ... {symbol_name} = ...;"
            ))
            # enum 成员
            patterns.append((
                rf'\b{re.escape(symbol_name)}\s*=\s*\d+',
                f"{symbol_name} = ... (enum member)"
            ))
            patterns.append((
                rf'\b{re.escape(symbol_name)}\s*,',
                f"{symbol_name}, (enum member)"
            ))
            
        elif symbol_type == MissingSymbolType.MACRO:
            # 宏定义
            patterns.append((
                rf'#\s*define\s+{re.escape(symbol_name)}(?:\([^)]*\))?\s*[^\n]+',
                f"#define {symbol_name}..."
            ))
        
        return patterns
    
    def _search_in_headers(
        self, 
        symbol_name: str, 
        symbol_type: MissingSymbolType
    ) -> Optional[Dict]:
        """在头文件目录中搜索"""
        patterns = self._get_search_patterns(symbol_name, symbol_type)
        
        for include_dir in self.include_dirs:
            if not include_dir.exists():
                continue
                
            for file_path in include_dir.rglob('*.h'):
                content = self._get_file_content(str(file_path))
                if not content:
                    continue
                
                for pattern, pattern_desc in patterns:
                    match = re.search(pattern, content, re.MULTILINE | re.DOTALL)
                    if match:
                        line_number = content[:match.start()].count('\n') + 1
                        
                        lines = content.split('\n')
                        start_line = max(0, line_number - 3)
                        end_line = min(len(lines), line_number + 5)
                        code_snippet = '\n'.join(
                            f"{i+1:4d} | {lines[i]}" 
                            for i in range(start_line, end_line)
                        )
                        
                        return {
                            'evidence': SourceEvidence(
                                file_path=str(file_path),
                                line_number=line_number,
                                code_snippet=code_snippet
                            ),
                            'definition': match.group(0).strip()
                        }
        
        return None
    
    def _analyze_failure_reason(
        self,
        symbol_name: str,
        symbol_type: MissingSymbolType,
        c_definition: Dict
    ) -> Dict:
        """
        分析转换失败的原因
        
        这是诊断的核心：我们找到了 C 定义，但为什么 Rust 转换失败了？
        """
        result = {
            'reason': FailureReason.UNKNOWN,
            'notes': [],
            'dependencies': [],
            'suggested_fix': ''
        }
        
        definition = c_definition['definition']
        evidence = c_definition['evidence']
        file_path = evidence.file_path
        
        # 分析 1: 是否是条件编译导致的
        content = self._get_file_content(file_path)
        if content:
            # 检查定义前是否有 #ifdef / #if
            lines = content.split('\n')
            start_line = evidence.line_number - 1
            
            # 向上搜索条件编译指令
            conditional_stack = []
            for i in range(start_line - 1, max(0, start_line - 50), -1):
                line = lines[i].strip()
                if line.startswith('#endif'):
                    conditional_stack.append('#endif')
                elif line.startswith(('#ifdef', '#ifndef', '#if ')):
                    if conditional_stack:
                        conditional_stack.pop()
                    else:
                        # 找到了包围定义的条件编译
                        result['reason'] = FailureReason.CONDITIONAL_COMPILE
                        result['notes'].append(
                            f"🔍 定义被条件编译包围: {line}"
                        )
                        result['notes'].append(
                            f"   位置: {file_path}:{i+1}"
                        )
                        result['suggested_fix'] = (
                            f"检查条件编译: {line}\n"
                            f"确保在编译时定义了正确的宏"
                        )
                        return result
        
        # 分析 2: 是否是宏展开问题
        # 检查定义中是否包含其他宏
        macro_pattern = r'\b([A-Z][A-Z0-9_]{2,})\b'
        macros_in_def = set(re.findall(macro_pattern, definition))
        macros_in_def.discard(symbol_name.upper())
        
        if macros_in_def:
            result['reason'] = FailureReason.MACRO_NOT_EXPANDED
            result['dependencies'] = list(macros_in_def)
            result['notes'].append(
                f"🔍 定义中使用了其他宏: {', '.join(macros_in_def)}"
            )
            result['suggested_fix'] = (
                f"需要先展开这些宏: {', '.join(macros_in_def)}\n"
                f"可以在 macro_learner.py 中添加这些宏的定义"
            )
            return result
        
        # 分析 3: 是否是 typedef 链
        if 'typedef' in definition:
            # 检查 typedef 指向的类型是否是另一个自定义类型
            typedef_match = re.search(r'typedef\s+(\w+)\s+' + re.escape(symbol_name), definition)
            if typedef_match:
                base_type = typedef_match.group(1)
                if not self._is_primitive_type(base_type):
                    result['reason'] = FailureReason.TYPEDEF_CHAIN
                    result['dependencies'] = [base_type]
                    result['notes'].append(
                        f"🔍 typedef 指向自定义类型: {base_type}"
                    )
                    result['suggested_fix'] = (
                        f"需要先定义类型: {base_type}\n"
                        f"然后再定义 {symbol_name}"
                    )
                    return result
        
        # 分析 4: 是否是前向声明（不透明类型）
        if re.match(rf'\bstruct\s+{re.escape(symbol_name)}\s*;$', definition.strip()):
            result['reason'] = FailureReason.OPAQUE_TYPE
            result['notes'].append(
                f"🔍 这是一个前向声明（不透明类型）"
            )
            result['notes'].append(
                f"   完整定义可能在其他文件中"
            )
            result['suggested_fix'] = (
                f"在 Rust 中使用不透明类型:\n"
                f"#[repr(C)]\n"
                f"pub struct {symbol_name} {{ _opaque: [u8; 0] }}"
            )
            return result
        
        # 分析 5: bindgen 可能失败的原因
        if symbol_type == MissingSymbolType.TYPE:
            # 检查是否有复杂的联合体或位域
            if 'union' in definition:
                result['reason'] = FailureReason.BINDGEN_FAILURE
                result['notes'].append(
                    "🔍 定义包含 union，bindgen 可能无法正确处理"
                )
            elif re.search(r':\s*\d+\s*[,;]', definition):  # 位域
                result['reason'] = FailureReason.BINDGEN_FAILURE
                result['notes'].append(
                    "🔍 定义包含位域，bindgen 可能无法正确处理"
                )
        
        # 如果到这里还是 UNKNOWN，尝试给出更多信息
        if result['reason'] == FailureReason.UNKNOWN:
            result['notes'].append(
                f"🔍 找到了原始定义，但无法确定失败原因"
            )
            result['notes'].append(
                f"   文件: {file_path}:{evidence.line_number}"
            )
            result['notes'].append(
                f"   定义: {definition[:200]}..."
            )
            result['suggested_fix'] = (
                f"请人工检查原始定义:\n{definition[:500]}"
            )
        
        return result
    
    def _is_primitive_type(self, type_name: str) -> bool:
        """检查是否是 C 原始类型"""
        primitives = {
            'int', 'char', 'short', 'long', 'float', 'double', 'void',
            'unsigned', 'signed', 'size_t', 'ssize_t', 'ptrdiff_t',
            'int8_t', 'int16_t', 'int32_t', 'int64_t',
            'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t',
            'intptr_t', 'uintptr_t', 'bool', '_Bool'
        }
        return type_name.lower() in primitives
    
    def _generate_recommendations(self, report: DiagnosticReport) -> List[str]:
        """生成汇总建议"""
        recommendations = []
        
        # 按失败原因统计
        reason_counts = report.by_reason
        
        if reason_counts.get('macro_not_expanded', 0) > 0:
            recommendations.append(
                f"📌 有 {reason_counts['macro_not_expanded']} 个符号因宏未展开而失败。\n"
                f"   建议: 在 macro_learner.py 中添加这些宏的定义"
            )
        
        if reason_counts.get('header_not_included', 0) > 0:
            recommendations.append(
                f"📌 有 {reason_counts['header_not_included']} 个符号定义在未包含的头文件中。\n"
                f"   建议: 检查头文件搜索路径，或在 include_dirs 中添加路径"
            )
        
        if reason_counts.get('conditional_compile', 0) > 0:
            recommendations.append(
                f"📌 有 {reason_counts['conditional_compile']} 个符号被条件编译排除。\n"
                f"   建议: 检查编译选项，确保定义了正确的预处理宏"
            )
        
        if reason_counts.get('opaque_type', 0) > 0:
            recommendations.append(
                f"📌 有 {reason_counts['opaque_type']} 个不透明类型（前向声明）。\n"
                f"   这些类型可以安全地使用 [u8; 0] 占位符"
            )
        
        if len(report.manual_review_items) > 0:
            recommendations.append(
                f"⚠️ 有 {len(report.manual_review_items)} 个符号需要人工审查"
            )
        
        return recommendations


def generate_diagnostic_report(
    error_output: str,
    rust_project_dir: Path,
    c_source_dirs: List[Path],
    include_dirs: List[Path] = None,
    output_path: Optional[Path] = None
) -> DiagnosticReport:
    """
    生成诊断报告的便捷函数
    
    Args:
        error_output: cargo check 的错误输出
        rust_project_dir: Rust 项目目录
        c_source_dirs: C 源码目录
        include_dirs: 头文件目录
        output_path: 报告输出路径
        
    Returns:
        诊断报告
    """
    analyzer = DiagnosticAnalyzer(c_source_dirs, include_dirs)
    report = analyzer.analyze_compilation_errors(error_output, rust_project_dir)
    
    if output_path:
        save_report(report, output_path)
    
    return report


def save_report(report: DiagnosticReport, output_path: Path):
    """保存诊断报告"""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # JSON 格式
    json_path = output_path.with_suffix('.json')
    with open(json_path, 'w', encoding='utf-8') as f:
        json.dump(_report_to_dict(report), f, indent=2, ensure_ascii=False)
    
    # 人类可读格式
    txt_path = output_path.with_suffix('.txt')
    with open(txt_path, 'w', encoding='utf-8') as f:
        f.write(format_report(report))
    
    logger.info(f"诊断报告已保存: {json_path}, {txt_path}")


def _report_to_dict(report: DiagnosticReport) -> Dict:
    """将报告转换为字典"""
    return {
        'project_name': report.project_name,
        'timestamp': report.timestamp,
        'total_missing_symbols': report.total_missing_symbols,
        'by_type': report.by_type,
        'by_reason': report.by_reason,
        'summary_recommendations': report.summary_recommendations,
        'diagnostics': [
            {
                'symbol_name': d.symbol_name,
                'symbol_type': d.symbol_type.value,
                'failure_reason': d.failure_reason.value,
                'confidence': d.confidence,
                'original_definition': d.original_definition[:500] if d.original_definition else '',
                'c_source_location': (
                    f"{d.c_source_evidence.file_path}:{d.c_source_evidence.line_number}"
                    if d.c_source_evidence else None
                ),
                'diagnostic_notes': d.diagnostic_notes,
                'suggested_fix': d.suggested_fix,
                'manual_review_required': d.manual_review_required
            }
            for d in report.diagnostics
        ]
    }


def format_report(report: DiagnosticReport) -> str:
    """格式化报告为人类可读文本"""
    lines = [
        "=" * 70,
        f"诊断报告: {report.project_name}",
        f"生成时间: {report.timestamp}",
        "=" * 70,
        "",
        f"📊 统计摘要",
        f"   缺失符号总数: {report.total_missing_symbols}",
        "",
        f"   按类型分布:",
    ]
    
    for type_name, count in report.by_type.items():
        lines.append(f"     - {type_name}: {count}")
    
    lines.append("")
    lines.append(f"   按原因分布:")
    for reason, count in report.by_reason.items():
        lines.append(f"     - {reason}: {count}")
    
    lines.append("")
    lines.append("-" * 70)
    lines.append("📋 建议")
    lines.append("-" * 70)
    for rec in report.summary_recommendations:
        lines.append(rec)
    
    lines.append("")
    lines.append("-" * 70)
    lines.append("📝 详细诊断")
    lines.append("-" * 70)
    
    for i, d in enumerate(report.diagnostics, 1):
        lines.append("")
        lines.append(f"[{i}] {d.symbol_name} ({d.symbol_type.value})")
        lines.append(f"    失败原因: {d.failure_reason.value}")
        lines.append(f"    置信度: {d.confidence:.0%}")
        
        if d.c_source_evidence:
            lines.append(f"    C 源码位置: {d.c_source_evidence.file_path}:{d.c_source_evidence.line_number}")
            if d.c_source_evidence.code_snippet:
                lines.append("    代码片段:")
                for line in d.c_source_evidence.code_snippet.split('\n')[:10]:
                    lines.append(f"      {line}")
        
        if d.diagnostic_notes:
            lines.append("    诊断备注:")
            for note in d.diagnostic_notes:
                lines.append(f"      {note}")
        
        if d.suggested_fix:
            lines.append("    建议修复:")
            for line in d.suggested_fix.split('\n'):
                lines.append(f"      {line}")
        
        if d.manual_review_required:
            lines.append("    ⚠️ 需要人工审查")
    
    return '\n'.join(lines)


# =============================================================================
# 测试代码
# =============================================================================

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    
    # 模拟编译错误
    fake_errors = """
error[E0412]: cannot find type `HdfDeviceInfo` in this scope
 --> src/lib.rs:42:15
  |
42 |     info: *mut HdfDeviceInfo,
  |               ^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `HDF_ERR_INVALID_OBJECT` in this scope
  --> src/lib.rs:88:16
   |
88 |         return HDF_ERR_INVALID_OBJECT;
   |                ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `AudioSubPortCapability` in this scope
  --> src/audio.rs:156:22
   |
156|     capability: *mut AudioSubPortCapability,
   |                      ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
"""
    
    # 创建分析器
    analyzer = DiagnosticAnalyzer(
        c_source_dirs=[Path("./dlp_fuse")],
        include_dirs=[Path("/usr/include")]
    )
    
    # 分析
    report = analyzer.analyze_compilation_errors(
        fake_errors,
        Path("./output/test_project")
    )
    
    # 输出报告
    print(format_report(report))



