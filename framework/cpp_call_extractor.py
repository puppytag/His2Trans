"""C/C++ 调用符号提取工具，供依赖分析和翻译提示词共用。"""

from __future__ import annotations

import re
from typing import Iterable


_STOP_WORDS = {
    "alignas",
    "alignof",
    "auto",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "decltype",
    "delete",
    "double",
    "enum",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "if",
    "inline",
    "int",
    "long",
    "namespace",
    "new",
    "nullptr",
    "NULL",
    "operator",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "sizeof",
    "signed",
    "static_assert",
    "static",
    "struct",
    "switch",
    "template",
    "this",
    "true",
    "typedef",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "while",
}


def is_cpp_call_identifier(name: str) -> bool:
    """判断标识符是否可作为 C/C++ 被调符号名。"""
    text = str(name or "").strip()
    return bool(re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", text) and text not in _STOP_WORDS)


def normalize_cpp_call_expression(call_expression: str) -> str:
    """把 tree-sitter call_expression 文本规整为可匹配的被调符号名。"""
    text = str(call_expression or "").strip()
    if not text:
        return ""
    callee = text.split("(", 1)[0].strip()
    callee = re.sub(r"<[^<>]*>\s*$", "", callee).strip()
    parts = [part for part in re.split(r"\s*(?:->|\.|::)\s*", callee) if part]
    name = parts[-1] if parts else callee
    name = name.strip()
    if not is_cpp_call_identifier(name):
        return ""
    return name


def extract_cpp_called_identifiers(code: str, call_expressions: Iterable[str] = ()) -> set[str]:
    """从 C/C++ 代码和可选 call_expression 文本中提取被调符号名。"""
    text = str(code or "")
    called: set[str] = set()

    for expr in call_expressions or ():
        name = normalize_cpp_call_expression(str(expr or ""))
        if name:
            called.add(name)

    for match in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^<>]*>\s*)?\(", text):
        name = match.group(1)
        if is_cpp_call_identifier(name):
            called.add(name)

    # C++ RAII/local object construction: `AutoMutex mutex(this->lock_);`
    for match in re.finditer(
        r"(?m)^\s*(?:const\s+)?([A-Za-z_][A-Za-z0-9_:]*)\s+[A-Za-z_][A-Za-z0-9_]*\s*\(",
        text,
    ):
        type_name = match.group(1).split("::")[-1]
        if is_cpp_call_identifier(type_name):
            called.add(type_name)

    return called
