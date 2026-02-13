#!/usr/bin/env python3
"""
动态宏学习器 (Dynamic Macro Learner)

当 Tree-sitter 解析失败时，自动学习新的宏定义。
学习到的宏可以跨项目复用，并持久化保存。

实现思路:
1. 当 Tree-sitter 解析报错时，分析报错行的 C 源码
2. 自动提取干扰解析的宏
3. 动态添加到宏展开字典中
4. 保存到文件，供后续项目复用
"""

import re
import json
import logging
import os
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Set
from dataclasses import dataclass, field

logger = logging.getLogger(__name__)

# =========================================================================
# 内置宏定义 (OpenHarmony/LiteOS 系统宏)
# =========================================================================

# 基础系统宏
BUILTIN_MACRO_EXPANSIONS = {
    # LiteOS/OpenHarmony 存储类说明符
    'STATIC': 'static',
    'INLINE': 'inline',
    'LITE_OS_SEC_TEXT': '',
    'LITE_OS_SEC_TEXT_INIT': '',
    'LITE_OS_SEC_TEXT_MINOR': '',
    'LITE_OS_SEC_DATA': '',
    'LITE_OS_SEC_DATA_INIT': '',
    'LITE_OS_SEC_DATA_VEC': '',
    'LITE_OS_SEC_BSS': '',
    'LITE_OS_SEC_BSS_INIT': '',
    'LITE_OS_SEC_BSS_MINOR': '',
    'LITE_OS_SEC_RODATA': '',
    'LITE_OS_SEC_ITCM': '',
    'LITE_OS_SEC_DTCM': '',
    'LITE_OS_SECTION': '',
    
    # HDF 驱动框架宏
    'HDF_STATIC': 'static',
    'HDF_INIT': '',
    'HDF_LOG_TAG': '',
    
    # OHOS API 宏
    'OHOS_API': '',
    'OHOS_INIT': '',
    
    # 基本类型宏 (LiteOS 风格)
    'VOID': 'void',
    'CHAR': 'char',
    'BOOL': '_Bool',
    'INT8': 'signed char',
    'UINT8': 'unsigned char',
    'INT16': 'short',
    'UINT16': 'unsigned short',
    'INT32': 'int',
    'UINT32': 'unsigned int',
    'INT64': 'long long',
    'UINT64': 'unsigned long long',
    'FLOAT': 'float',
    'DOUBLE': 'double',
    'UINTPTR': 'uintptr_t',
    'INTPTR': 'intptr_t',
    'AARCHPTR': 'uintptr_t',
    'size_t': 'unsigned long',
    'ssize_t': 'long',
    
    # DSoftBus 宏
    'SOFTBUS_DPRINTF_ATTR': '',
    'SOFTBUS_API': '',
    'NO_SANITIZE': '',
    
    # 通用编译器属性宏 (简化处理)
    '__unused': '',
    '__weak': '',
    '__aligned': '',
    '__section': '',
    '__always_inline': 'inline',
    '__noinline': '',
    '__packed': '',
    '__deprecated': '',
    '__printf': '',
}

# =========================================================================
# 宏模式识别器
# =========================================================================

@dataclass
class LearnedMacro:
    """学习到的宏信息"""
    name: str
    expansion: str
    source: str = "auto-learned"  # 来源：auto-learned, builtin, user-defined
    confidence: float = 0.8
    context: str = ""  # 发现该宏的上下文
    
    # 诊断信息 - 记录宏的来源和为什么使用这个展开值
    is_placeholder: bool = False              # 展开值是否是猜测的（而非真实定义）
    c_source_file: str = ""                   # 原始宏定义的文件位置
    c_source_line: int = 0                    # 原始宏定义的行号
    original_definition: str = ""             # 原始宏定义内容
    failure_reason: str = ""                  # 如果是占位符，为什么无法获取真实展开
    diagnostic_notes: List[str] = field(default_factory=list)  # 诊断备注

@dataclass
class MacroLearningResult:
    """学习结果"""
    learned: List[LearnedMacro]
    source_modified: str
    success: bool


class MacroLearner:
    """
    动态宏学习器
    
    特性:
    - 从解析错误中自动学习新宏
    - 支持多种宏模式识别
    - 持久化保存学习结果
    - 跨项目共享知识
    """
    
    def __init__(
        self, 
        storage_path: Optional[Path] = None,
        include_builtin: bool = True
    ):
        """
        初始化宏学习器
        
        Args:
            storage_path: 持久化存储路径，默认为项目 .cache 目录
            include_builtin: 是否包含内置宏
        """
        # 默认存储到项目的 .cache/learned_data 目录（更安全，不会被意外删除）
        if storage_path is None:
            cache_root_env = os.environ.get("C2R_CACHE_ROOT", "").strip()
            if cache_root_env:
                cache_dir = Path(cache_root_env).expanduser().resolve() / "learned_data"
            else:
                # 尝试找到项目根目录
                project_root = Path(__file__).parent
                cache_dir = project_root / ".cache" / "learned_data"
            cache_dir.mkdir(parents=True, exist_ok=True)
            self.storage_path = cache_dir / "learned_macros.json"
        else:
            self.storage_path = storage_path
            
        self.macros: Dict[str, LearnedMacro] = {}
        
        # 加载内置宏
        if include_builtin:
            for name, expansion in BUILTIN_MACRO_EXPANSIONS.items():
                self.macros[name] = LearnedMacro(
                    name=name, 
                    expansion=expansion, 
                    source="builtin"
                )
        
        # 加载持久化的学习结果
        self._load_from_storage()
    
    def _load_from_storage(self):
        """从文件加载已学习的宏（包含诊断信息）"""
        if not self.storage_path.exists():
            return
        
        try:
            with open(self.storage_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            for entry in data.get("macros", []):
                name = entry.get("name")
                if name and name not in self.macros:  # 不覆盖内置宏
                    self.macros[name] = LearnedMacro(
                        name=name,
                        expansion=entry.get("expansion", ""),
                        source=entry.get("source", "loaded"),
                        confidence=entry.get("confidence", 0.5),
                        context=entry.get("context", ""),
                        # 诊断信息
                        is_placeholder=entry.get("is_placeholder", False),
                        c_source_file=entry.get("c_source_file", ""),
                        c_source_line=entry.get("c_source_line", 0),
                        original_definition=entry.get("original_definition", ""),
                        failure_reason=entry.get("failure_reason", ""),
                        diagnostic_notes=entry.get("diagnostic_notes", [])
                    )
            
            # 统计占位符数量
            placeholder_count = sum(1 for m in self.macros.values() if m.is_placeholder and m.source != "builtin")
            logger.info(f"从 {self.storage_path} 加载了 {len(data.get('macros', []))} 个已学习的宏 (其中 {placeholder_count} 个是占位符)")
        except Exception as e:
            logger.warning(f"加载已学习的宏失败: {e}")
    
    def save_to_storage(self):
        """保存学习结果到文件（包含诊断信息）"""
        try:
            self.storage_path.parent.mkdir(parents=True, exist_ok=True)
            
            # 只保存非内置宏
            learned_macros = [
                {
                    "name": m.name,
                    "expansion": m.expansion,
                    "source": m.source,
                    "confidence": m.confidence,
                    "context": m.context,
                    # 诊断信息
                    "is_placeholder": m.is_placeholder,
                    "c_source_file": m.c_source_file,
                    "c_source_line": m.c_source_line,
                    "original_definition": m.original_definition[:500] if m.original_definition else "",
                    "failure_reason": m.failure_reason,
                    "diagnostic_notes": m.diagnostic_notes
                }
                for m in self.macros.values()
                if m.source != "builtin"
            ]
            
            # 分类统计
            placeholder_count = sum(1 for m in learned_macros if m.get("is_placeholder", False))
            real_count = len(learned_macros) - placeholder_count
            
            data = {
                "version": "2.0",
                "statistics": {
                    "total": len(learned_macros),
                    "real_expansions": real_count,
                    "placeholders": placeholder_count
                },
                "macros": learned_macros
            }
            
            with open(self.storage_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            logger.info(f"已保存 {len(learned_macros)} 个学习的宏到 {self.storage_path} (真实: {real_count}, 占位符: {placeholder_count})")
        except Exception as e:
            logger.warning(f"保存学习的宏失败: {e}")
    
    def get_expansion(self, macro_name: str) -> Optional[str]:
        """获取宏的展开值"""
        macro = self.macros.get(macro_name)
        return macro.expansion if macro else None
    
    def add_macro(
        self, 
        name: str, 
        expansion: str, 
        source: str = "auto-learned",
        confidence: float = 0.8,
        context: str = "",
        # 诊断信息参数
        is_placeholder: bool = False,
        c_source_file: str = "",
        c_source_line: int = 0,
        original_definition: str = "",
        failure_reason: str = "",
        diagnostic_notes: List[str] = None
    ):
        """
        添加新宏
        
        Args:
            name: 宏名称
            expansion: 展开后的值
            source: 来源
            confidence: 置信度
            context: 上下文
            is_placeholder: 展开值是否是猜测的
            c_source_file: 原始定义文件
            c_source_line: 原始定义行号
            original_definition: 原始宏定义
            failure_reason: 失败原因
            diagnostic_notes: 诊断备注
        """
        if name in self.macros and self.macros[name].source == "builtin":
            # 不覆盖内置宏
            return
        
        self.macros[name] = LearnedMacro(
            name=name,
            expansion=expansion,
            source=source,
            confidence=confidence,
            context=context,
            is_placeholder=is_placeholder,
            c_source_file=c_source_file,
            c_source_line=c_source_line,
            original_definition=original_definition,
            failure_reason=failure_reason,
            diagnostic_notes=diagnostic_notes or []
        )
        
        # 日志输出
        if is_placeholder:
            logger.warning(f"⚠️ 学习占位符宏: {name} -> {expansion} (原因: {failure_reason})")
        else:
            logger.info(f"✓ 学习宏: {name} -> {expansion}")
    
    def expand_all(self, source_code: str) -> str:
        """
        展开源代码中的所有已知宏
        
        Args:
            source_code: 源代码
        
        Returns:
            展开后的源代码
        """
        result = source_code
        
        # 按宏名长度降序排序，避免短宏名被先替换导致问题
        sorted_macros = sorted(
            self.macros.items(), 
            key=lambda x: len(x[0]), 
            reverse=True
        )
        
        for name, macro in sorted_macros:
            if macro.expansion is not None:
                pattern = rf'\b{re.escape(name)}\b'
                result = re.sub(pattern, macro.expansion, result)
        
        return result
    
    def learn_from_error_line(
        self, 
        source_code: str, 
        error_line: int,
        save: bool = True
    ) -> List[LearnedMacro]:
        """
        从解析错误行学习新宏
        
        Args:
            source_code: 源代码
            error_line: 错误行号 (1-based)
            save: 是否立即保存
        
        Returns:
            学习到的宏列表
        """
        lines = source_code.split('\n')
        if error_line < 1 or error_line > len(lines):
            return []
        
        line = lines[error_line - 1].strip()
        learned = []
        
        # =========================================================================
        # 模式1: 存储类说明符宏 (如 STATIC void func())
        # =========================================================================
        storage_class_pattern = re.match(
            r'^([A-Z][A-Z0-9_]*)\s+((?:unsigned\s+|signed\s+)?'
            r'(?:int|long|short|char|void|float|double|_Bool|\w+_t))\s+(\w+)\s*\(',
            line
        )
        if storage_class_pattern:
            potential_macro = storage_class_pattern.group(1)
            # 已知的存储类说明符宏
            if potential_macro not in self.macros:
                if 'STATIC' in potential_macro:
                    expansion = 'static'
                elif 'INLINE' in potential_macro:
                    expansion = 'inline'
                elif 'EXTERN' in potential_macro:
                    expansion = 'extern'
                else:
                    expansion = ''  # 默认展开为空（如段属性）
                
                self.add_macro(potential_macro, expansion, context=line[:100])
                learned.append(self.macros[potential_macro])
        
        # =========================================================================
        # 模式2: 类型宏 (如 UINT32 func_name())
        # =========================================================================
        type_macro_pattern = re.match(
            r'^(?:static\s+|inline\s+|extern\s+)*([A-Z][A-Z0-9_]*)\s+(\w+)\s*\(',
            line
        )
        if type_macro_pattern and type_macro_pattern.group(1) not in self.macros:
            potential_type = type_macro_pattern.group(1)
            
            # 已知的类型宏映射
            known_types = {
                'VOID': 'void', 'CHAR': 'char', 'BOOL': '_Bool',
                'INT8': 'signed char', 'UINT8': 'unsigned char',
                'INT16': 'short', 'UINT16': 'unsigned short',
                'INT32': 'int', 'UINT32': 'unsigned int',
                'INT64': 'long long', 'UINT64': 'unsigned long long',
                'FLOAT': 'float', 'DOUBLE': 'double',
                'SIZE_T': 'unsigned long', 'SSIZE_T': 'long',
            }
            
            if potential_type in known_types:
                expansion = known_types[potential_type]
            else:
                # 推断：以 U 开头的全大写标识符可能是无符号整数
                if potential_type.startswith('U') and potential_type[1:].isalpha():
                    expansion = 'unsigned int'
                else:
                    expansion = 'int'  # 默认假设为 int
            
            self.add_macro(potential_type, expansion, context=line[:100])
            learned.append(self.macros[potential_type])
        
        # =========================================================================
        # 模式3: 参数列表中的宏 (如 func(UINT32 arg))
        # =========================================================================
        param_macro_pattern = re.findall(r'\(.*?([A-Z][A-Z0-9_]*)\s+\w+', line)
        for param_type in param_macro_pattern:
            if param_type not in self.macros:
                # 同上，推断类型
                known_types = {
                    'VOID': 'void', 'CHAR': 'char', 'BOOL': '_Bool',
                    'INT8': 'signed char', 'UINT8': 'unsigned char',
                    'INT16': 'short', 'UINT16': 'unsigned short',
                    'INT32': 'int', 'UINT32': 'unsigned int',
                    'INT64': 'long long', 'UINT64': 'unsigned long long',
                }
                
                if param_type in known_types:
                    expansion = known_types[param_type]
                else:
                    expansion = 'int'
                
                self.add_macro(param_type, expansion, context=line[:100])
                learned.append(self.macros[param_type])
        
        if save and learned:
            self.save_to_storage()
        
        return learned
    
    def learn_from_parse_errors(
        self, 
        source_code: str, 
        error_locations: List[Tuple[int, int]],
        max_learn: int = 10
    ) -> MacroLearningResult:
        """
        从多个解析错误位置学习宏
        
        Args:
            source_code: 源代码
            error_locations: 错误位置列表 [(line, col), ...]
            max_learn: 最大学习数量
        
        Returns:
            学习结果
        """
        all_learned = []
        
        # 去重错误行
        error_lines = set(loc[0] for loc in error_locations[:max_learn])
        
        for line_num in sorted(error_lines):
            learned = self.learn_from_error_line(source_code, line_num, save=False)
            all_learned.extend(learned)
        
        # 应用学习到的宏展开
        modified_source = self.expand_all(source_code)
        
        # 保存
        if all_learned:
            self.save_to_storage()
        
        return MacroLearningResult(
            learned=all_learned,
            source_modified=modified_source,
            success=len(all_learned) > 0
        )
    
    def get_gcc_define_args(self) -> List[str]:
        """
        生成 GCC 预处理器的 -D 参数
        
        Returns:
            参数列表 ['-DSTATIC=static', '-DUINT32=unsigned int', ...]
        """
        args = []
        for name, macro in self.macros.items():
            if macro.expansion:
                args.append(f'-D{name}={macro.expansion}')
            else:
                args.append(f'-D{name}=')
        return args
    
    def get_macro_count(self) -> Dict[str, int]:
        """获取宏统计信息"""
        counts = {"builtin": 0, "auto-learned": 0, "loaded": 0, "user-defined": 0}
        for macro in self.macros.values():
            source = macro.source if macro.source in counts else "other"
            counts[source] = counts.get(source, 0) + 1
        return counts
    
    def get_placeholder_macros(self) -> List[LearnedMacro]:
        """获取所有占位符宏（需要人工审查）"""
        return [m for m in self.macros.values() if m.is_placeholder and m.source != "builtin"]
    
    def get_statistics(self) -> Dict:
        """获取统计信息（包含诊断信息）"""
        learned_macros = [m for m in self.macros.values() if m.source != "builtin"]
        placeholder_macros = [m for m in learned_macros if m.is_placeholder]
        
        return {
            "builtin_count": sum(1 for m in self.macros.values() if m.source == "builtin"),
            "learned_count": len(learned_macros),
            "learned_real": len(learned_macros) - len(placeholder_macros),
            "learned_placeholder": len(placeholder_macros),
            "storage_path": str(self.storage_path)
        }
    
    def generate_diagnostic_report(self, output_path: Optional[Path] = None) -> str:
        """
        生成诊断报告
        
        显示所有占位符宏及其失败原因
        
        Args:
            output_path: 可选，保存报告的路径
            
        Returns:
            格式化的报告文本
        """
        from datetime import datetime
        
        lines = [
            "=" * 70,
            "宏学习器诊断报告",
            f"生成时间: {datetime.now().isoformat()}",
            f"存储路径: {self.storage_path}",
            "=" * 70,
            "",
        ]
        
        stats = self.get_statistics()
        lines.append("📊 统计信息")
        lines.append(f"  内置宏: {stats['builtin_count']}")
        lines.append(f"  学习宏: {stats['learned_count']} (真实展开: {stats['learned_real']}, 占位符: {stats['learned_placeholder']})")
        lines.append("")
        
        placeholder_macros = self.get_placeholder_macros()
        
        if placeholder_macros:
            lines.append("-" * 70)
            lines.append(f"⚠️ 需要审查的宏 ({len(placeholder_macros)} 个)")
            lines.append("-" * 70)
            
            for m in placeholder_macros:
                lines.append(f"\n[宏] {m.name}")
                lines.append(f"  当前展开: {m.expansion}")
                lines.append(f"  失败原因: {m.failure_reason}")
                if m.c_source_file:
                    lines.append(f"  C 源码位置: {m.c_source_file}:{m.c_source_line}")
                if m.original_definition:
                    lines.append(f"  原始定义: {m.original_definition[:100]}...")
                if m.context:
                    lines.append(f"  上下文: {m.context[:80]}...")
                if m.diagnostic_notes:
                    lines.append("  诊断备注:")
                    for note in m.diagnostic_notes:
                        lines.append(f"    - {note}")
        else:
            lines.append("✅ 所有学习的宏都有真实展开值，无需审查")
        
        report = '\n'.join(lines)
        
        if output_path:
            output_path = Path(output_path)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(report)
            logger.info(f"宏诊断报告已保存到: {output_path}")
        
        return report


# =========================================================================
# 全局实例（延迟初始化）
# =========================================================================

_global_learner: Optional[MacroLearner] = None

def get_global_macro_learner() -> MacroLearner:
    """获取全局宏学习器实例"""
    global _global_learner
    if _global_learner is None:
        _global_learner = MacroLearner()
    return _global_learner


def expand_macros(source_code: str) -> str:
    """快速展开宏的便捷函数"""
    return get_global_macro_learner().expand_all(source_code)


def get_gcc_macro_args() -> List[str]:
    """获取 GCC -D 参数的便捷函数"""
    return get_global_macro_learner().get_gcc_define_args()


# =========================================================================
# 测试
# =========================================================================

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    
    # 创建学习器
    learner = MacroLearner()
    
    # 测试代码
    test_code = """
STATIC UINT32 GetFatBlockNums(INT32 diskId, UINT64* fatBlocks)
{
    // implementation
}

VOID OsMain(VOID)
{
    UINT32 ret;
    ret = SOFTBUS_OK;
}
"""
    
    # 展开宏
    expanded = learner.expand_all(test_code)
    print("=== Original ===")
    print(test_code)
    print("\n=== Expanded ===")
    print(expanded)
    
    # 获取 GCC 参数
    gcc_args = learner.get_gcc_define_args()
    print(f"\n=== GCC Args ({len(gcc_args)} total) ===")
    print(' '.join(gcc_args[:10]))
    print("...")
    
    # 统计
    counts = learner.get_macro_count()
    print(f"\n=== Statistics ===")
    for source, count in counts.items():
        print(f"  {source}: {count}")












