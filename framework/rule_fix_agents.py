"""
规则修复模块 (rule_fix.py)

提供基于规则的代码修复功能，用于修复常见的编译错误。
这些规则修复比 LLM 修复更快速、更可靠。

设计原则：
1. 每个修复器专注于一类特定的错误模式
2. 修复应该是确定性的、可预测的
3. 如果规则修复失败，再使用 LLM 修复
"""

import re
import logging
from typing import List, Tuple, Optional, Callable

logger = logging.getLogger(__name__)


class RuleFixer:
    """基于规则的代码修复器基类"""
    
    def __init__(self, name: str, description: str):
        self.name = name
        self.description = description
        self.fix_count = 0
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        """检查是否可以修复该错误"""
        raise NotImplementedError
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        """
        应用修复，返回修复后的代码候选列表
        
        Args:
            code: 原始代码
            error_msg: 编译错误消息
            
        Returns:
            修复后的代码候选列表（可能有多个）
        """
        raise NotImplementedError
    
    def try_fix(self, code: str, error_msg: str) -> Tuple[Optional[str], bool]:
        """
        尝试修复代码
        
        Returns:
            (修复后的代码, 是否成功)
        """
        if not self.can_fix(code, error_msg):
            return None, False
        
        candidates = self.apply(code, error_msg)
        if candidates:
            self.fix_count += 1
            return candidates[0], True
        return None, False


# ============================================================
# 具体的修复器实现
# ============================================================

class BracketMismatchFixer(RuleFixer):
    """修复括号不匹配错误"""
    
    def __init__(self):
        super().__init__(
            name="BracketMismatchFixer",
            description="修复括号/花括号/方括号不匹配的错误"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "unclosed delimiter",
            "mismatched closing delimiter",
            "unexpected closing",
            "expected `}`",
            "expected `)`",
            "expected `]`"
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        candidates = []
        
        # 策略 1: 统计括号，尝试补全
        bracket_pairs = [('(', ')'), ('{', '}'), ('[', ']')]
        
        for open_b, close_b in bracket_pairs:
            open_count = code.count(open_b)
            close_count = code.count(close_b)
            
            if open_count > close_count:
                # 缺少闭合括号，在末尾添加
                diff = open_count - close_count
                new_code = code.rstrip() + '\n' + (close_b * diff)
                candidates.append(new_code)
            elif close_count > open_count:
                # 多余的闭合括号，尝试删除最后一个
                diff = close_count - open_count
                new_code = code
                for _ in range(diff):
                    idx = new_code.rfind(close_b)
                    if idx != -1:
                        new_code = new_code[:idx] + new_code[idx+1:]
                candidates.append(new_code)
        
        # 策略 2: 从错误信息中提取行号，尝试修复特定行
        line_match = re.search(r'line\s*(\d+)', error_msg)
        if line_match:
            error_line = int(line_match.group(1))
            lines = code.split('\n')
            if 0 < error_line <= len(lines):
                # 检查该行是否缺少括号
                line = lines[error_line - 1]
                for open_b, close_b in bracket_pairs:
                    if line.count(open_b) > line.count(close_b):
                        lines[error_line - 1] = line + close_b
                        candidates.append('\n'.join(lines))
        
        return candidates


class TypeCastFixer(RuleFixer):
    """修复类型转换错误"""
    
    def __init__(self):
        super().__init__(
            name="TypeCastFixer",
            description="修复类型不匹配错误，添加 'as' 转换"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "mismatched types",
            "expected",
            "found",
            "e0308"  # Rust 类型不匹配错误码
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        candidates = []
        
        # 解析错误信息，提取期望类型和实际类型
        # 格式: "expected `i32`, found `u32`"
        expected_match = re.search(r"expected\s+`([^`]+)`.*found\s+`([^`]+)`", error_msg)
        if not expected_match:
            return candidates
        
        expected_type = expected_match.group(1)
        found_type = expected_match.group(2)
        
        # 解析错误行号
        line_match = re.search(r":(\d+):", error_msg)
        if not line_match:
            return candidates
        
        error_line = int(line_match.group(1))
        lines = code.split('\n')
        
        if 0 < error_line <= len(lines):
            line = lines[error_line - 1]
            
            # 策略 1: 在行尾添加 as 转换
            if expected_type in ['i32', 'u32', 'i64', 'u64', 'usize', 'isize']:
                # 查找可能需要转换的表达式
                # 简单策略：在分号前添加类型转换
                if line.strip().endswith(';'):
                    new_line = re.sub(r'(\S+)\s*;$', rf'\1 as {expected_type};', line)
                    lines[error_line - 1] = new_line
                    candidates.append('\n'.join(lines))
        
        return candidates


class BorrowCheckerAgent(RuleFixAgent):
    """修复借用检查问题 - 索引相关"""
    
    def __init__(self):
        super().__init__(
            name="BorrowCheckerAgent",
            description="修复结构体字段互相索引导致的借用检查错误"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "cannot borrow",
            "borrowed as",
            "while",
            "e0502",  # 借用冲突
            "e0499"   # 多次可变借用
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        """
        修复结构体自引用问题，如:
        s.arr[s.idx] → let tmp = s.idx; s.arr[tmp]
        """
        candidates = []
        
        # 匹配模式: something.field[something.other_field]
        pattern = r'(\w+)\.(\w+)\[(\1)\.(\w+)\]'
        
        lines = code.split('\n')
        modified = False
        
        for i, line in enumerate(lines):
            matches = list(re.finditer(pattern, line))
            if matches:
                indent = len(line) - len(line.lstrip())
                new_lines = []
                curr_start = 0
                
                for idx, match in enumerate(matches):
                    # 提取临时变量
                    var_name = match.group(1)
                    field1 = match.group(2)
                    field2 = match.group(4)
                    
                    tmp_var = f"_tmp_idx_{idx}"
                    
                    # 在当前行前插入临时变量声明
                    new_lines.append(' ' * indent + f"let {tmp_var} = {var_name}.{field2};")
                    
                    # 修改当前行
                    line = line[:match.start()] + f"{var_name}.{field1}[{tmp_var}]" + line[match.end():]
                
                if new_lines:
                    lines[i] = '\n'.join(new_lines) + '\n' + line
                    modified = True
        
        if modified:
            candidates.append('\n'.join(lines))
        
        return candidates


class MacroConfusionAgent(RuleFixAgent):
    """修复宏/常量混淆错误"""
    
    def __init__(self):
        super().__init__(
            name="MacroConfusionAgent",
            description="修复宏与常量混淆的错误 (MY_MACRO vs MY_MACRO!())"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "cannot find macro",
            "not a macro",
            "cannot find value",
            "expected macro",
            "e0433"  # 找不到宏
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        candidates = []
        
        # 从错误信息中提取标识符名称
        # 格式: "cannot find macro `MY_MACRO`"
        macro_match = re.search(r"cannot find macro\s+`(\w+)`", error_msg)
        if macro_match:
            macro_name = macro_match.group(1)
            # 将 MACRO!() 改为 MACRO (移除宏调用语法)
            new_code = re.sub(rf'\b{macro_name}!\s*\(\s*\)', macro_name, code)
            if new_code != code:
                candidates.append(new_code)
        
        # 反向：尝试将常量改为宏调用
        value_match = re.search(r"cannot find value\s+`(\w+)`", error_msg)
        if value_match:
            value_name = value_match.group(1)
            # 如果是全大写标识符，可能是宏
            if value_name.isupper() and '_' in value_name:
                new_code = re.sub(rf'\b{value_name}\b(?!\s*!)', f'{value_name}!()', code)
                if new_code != code:
                    candidates.append(new_code)
        
        return candidates


class FunctionCallSyntaxAgent(RuleFixAgent):
    """修复函数调用语法错误"""
    
    def __init__(self):
        super().__init__(
            name="FunctionCallSyntaxAgent",
            description="修复结构体字段函数指针调用语法错误"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "no method named",
            "method not found",
            "e0599"  # 方法未找到
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        """
        修复函数指针调用语法:
        s.callback(arg) → (s.callback)(arg)
        """
        candidates = []
        
        # 从错误信息提取方法名
        method_match = re.search(r"no method named\s+`(\w+)`", error_msg)
        if not method_match:
            return candidates
        
        method_name = method_match.group(1)
        
        # 模式: obj.method(args) → (obj.method)(args)
        pattern = rf'(\w+)\.({method_name})\s*\('
        
        def replace_func(m):
            return f"({m.group(1)}.{m.group(2)})("
        
        new_code = re.sub(pattern, replace_func, code)
        if new_code != code:
            candidates.append(new_code)
        
        return candidates


class NullPointerInitAgent(RuleFixAgent):
    """修复指针初始化问题"""
    
    def __init__(self):
        super().__init__(
            name="NullPointerInitAgent",
            description="修复指针初始化为 NULL 的问题"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "cannot find value `null`",
            "cannot find value `NULL`",
            "use of undeclared identifier"
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        candidates = []
        
        # 替换 NULL 为 std::ptr::null_mut()
        new_code = code
        new_code = re.sub(r'\bNULL\b', 'std::ptr::null_mut()', new_code)
        new_code = re.sub(r'\bnull\b', 'std::ptr::null_mut()', new_code)
        
        if new_code != code:
            candidates.append(new_code)
        
        # 替换 NULL 为 std::ptr::null() (const 版本)
        new_code2 = code
        new_code2 = re.sub(r'\bNULL\b', 'std::ptr::null()', new_code2)
        if new_code2 != code and new_code2 != new_code:
            candidates.append(new_code2)
        
        return candidates


class CharLiteralAgent(RuleFixAgent):
    """修复字符字面量问题"""
    
    def __init__(self):
        super().__init__(
            name="CharLiteralAgent",
            description="修复 char 字面量类型不匹配 ('\\0' vs b'\\0')"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return "expected `u8`, found `char`" in error_msg
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        candidates = []
        
        # 替换 'x' 为 b'x'
        # 匹配单字符字面量
        pattern = r"(?<![b'])'(\\.|[^'\\])'"
        new_code = re.sub(pattern, r"b'\1' as u8", code)
        
        if new_code != code:
            candidates.append(new_code)
        
        return candidates


class DereferenceOperatorAgent(RuleFixAgent):
    """修复解引用操作符问题"""
    
    def __init__(self):
        super().__init__(
            name="DereferenceOperatorAgent",
            description="修复解引用和引用操作符问题"
        )
    
    def can_fix(self, code: str, error_msg: str) -> bool:
        return any(keyword in error_msg.lower() for keyword in [
            "type `*mut",
            "type `*const",
            "cannot be dereferenced",
            "cannot be applied"
        ])
    
    def apply(self, code: str, error_msg: str) -> List[str]:
        # 这个修复比较复杂，暂时返回空
        # 后续可以根据具体错误模式添加规则
        return []


# ============================================================
# 规则修复管理器
# ============================================================

class RuleFixManager:
    """规则修复管理器，管理所有修复 Agent"""
    
    def __init__(self):
        self.agents: List[RuleFixAgent] = [
            BracketMismatchAgent(),
            TypeCastAgent(),
            BorrowCheckerAgent(),
            MacroConfusionAgent(),
            FunctionCallSyntaxAgent(),
            NullPointerInitAgent(),
            CharLiteralAgent(),
            DereferenceOperatorAgent(),
        ]
        
        self.total_fixes = 0
        self.fix_history: List[Tuple[str, str, str]] = []  # (agent_name, before, after)
    
    def add_agent(self, agent: RuleFixAgent):
        """添加自定义修复 Agent"""
        self.agents.append(agent)
    
    def try_fix(self, code: str, error_msg: str) -> Tuple[Optional[str], Optional[str]]:
        """
        尝试使用规则修复代码
        
        Args:
            code: 原始代码
            error_msg: 编译错误消息
            
        Returns:
            (修复后的代码, 使用的 Agent 名称) 或 (None, None)
        """
        for agent in self.agents:
            fixed_code, success = agent.try_fix(code, error_msg)
            if success and fixed_code:
                self.total_fixes += 1
                self.fix_history.append((agent.name, code[:100], fixed_code[:100]))
                logger.info(f"规则修复成功: {agent.name}")
                return fixed_code, agent.name
        
        return None, None
    
    def get_all_candidates(self, code: str, error_msg: str) -> List[Tuple[str, str]]:
        """
        获取所有可能的修复候选
        
        Returns:
            [(修复后的代码, Agent 名称), ...]
        """
        candidates = []
        for agent in self.agents:
            if agent.can_fix(code, error_msg):
                for fixed in agent.apply(code, error_msg):
                    candidates.append((fixed, agent.name))
        return candidates
    
    def get_stats(self) -> dict:
        """获取修复统计信息"""
        return {
            "total_fixes": self.total_fixes,
            "agents": {agent.name: agent.fix_count for agent in self.agents},
            "history_count": len(self.fix_history)
        }


# ============================================================
# 便捷函数
# ============================================================

_default_manager: Optional[RuleFixManager] = None

def get_rule_fix_manager() -> RuleFixManager:
    """获取默认的规则修复管理器（单例）"""
    global _default_manager
    if _default_manager is None:
        _default_manager = RuleFixManager()
    return _default_manager


def try_rule_fix(code: str, error_msg: str) -> Tuple[Optional[str], Optional[str]]:
    """
    便捷函数：尝试规则修复
    
    Args:
        code: 原始代码
        error_msg: 编译错误消息
        
    Returns:
        (修复后的代码, Agent 名称) 或 (None, None)
    """
    return get_rule_fix_manager().try_fix(code, error_msg)


def apply_all_rule_fixes(code: str, error_msgs: List[str]) -> str:
    """
    应用所有可能的规则修复
    
    Args:
        code: 原始代码
        error_msgs: 编译错误消息列表
        
    Returns:
        修复后的代码（可能多次修复）
    """
    manager = get_rule_fix_manager()
    current_code = code
    
    for error_msg in error_msgs:
        fixed, agent_name = manager.try_fix(current_code, error_msg)
        if fixed:
            current_code = fixed
            logger.info(f"应用规则修复: {agent_name}")
    
    return current_code


# ============================================================
# 测试代码
# ============================================================

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    
    # 测试括号修复
    code1 = """
    pub fn test() {
        if x > 0 {
            println!("hello"
        }
    }
    """
    error1 = "error: unclosed delimiter at line 4"
    
    manager = RuleFixManager()
    fixed, agent = manager.try_fix(code1, error1)
    print(f"Agent: {agent}")
    print(f"Fixed: {fixed}")
    
    # 测试借用检查修复
    code2 = """
    pub fn test(s: &mut MyStruct) {
        s.arr[s.idx] = 10;
    }
    """
    error2 = "cannot borrow `s.idx` as immutable"
    
    fixed, agent = manager.try_fix(code2, error2)
    print(f"Agent: {agent}")
    print(f"Fixed: {fixed}")
    
    print("\n统计信息:", manager.get_stats())

