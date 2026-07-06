#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
C2Rust 语义等价性评估 - 提示词模板

提供结构化的提示词用于 LLM 进行功能点提取和匹配分析。
"""

# ============================================================
# 阶段 1: C 代码功能点提取提示词
# ============================================================

C_FEATURE_EXTRACTION_SYSTEM_PROMPT = """你是一位资深的 C 语言和系统编程专家。你的任务是分析 C 代码并提取所有功能点。

请严格按照指定的 JSON 格式输出，不要添加任何额外的解释文字。"""

C_FEATURE_EXTRACTION_USER_PROMPT = """请分析以下 C 代码，提取所有功能点。

## 分析要求

请从以下维度提取功能点，每个功能点需要：
1. 唯一标识符 (F001, F002, ...)
2. 类型分类
3. 功能描述
4. 关键程度 (高/中/低)
5. 相关代码片段（简短摘要，不超过3行）

## 功能点分类

1. **FUNC** - 函数功能
   - 函数名称、参数、返回值
   - 核心逻辑和算法
   - 调用的外部函数

2. **STRUCT** - 数据结构
   - 结构体定义和字段
   - 枚举类型
   - 类型别名

3. **GLOBAL** - 全局状态
   - 全局变量
   - 静态变量
   - 常量定义

4. **MEMORY** - 内存管理
   - 动态内存分配
   - 资源释放
   - 指针操作模式

5. **ERROR** - 错误处理
   - 返回值检查
   - 错误码定义
   - 异常处理模式

6. **EXTERN** - 外部依赖
   - 外部函数声明
   - 系统调用
   - 第三方库调用

## 关键程度判断标准

- **高**: 核心业务逻辑、主要函数、关键数据结构
- **中**: 辅助函数、配置相关、日志输出
- **低**: 调试代码、注释说明、可选功能

## 输出格式 (严格 JSON)

```json
{{
  "file_name": "文件名",
  "summary": "文件整体功能概述（一句话）",
  "feature_points": [
    {{
      "id": "F001",
      "type": "FUNC",
      "name": "函数/结构体/变量名",
      "description": "功能描述",
      "importance": "高",
      "code_snippet": "关键代码片段摘要",
      "details": {{
        "parameters": ["参数列表"],
        "return_type": "返回类型",
        "key_logic": "核心逻辑说明"
      }}
    }}
  ]
}}
```

注意：
1. 只输出 JSON，不要有其他文字
2. 确保 JSON 格式正确，可以被解析
3. code_snippet 要简短，只保留关键行

## C 代码

文件名: {filename}

```c
{c_code}
```"""

# ============================================================
# 阶段 2: Rust 代码功能点匹配提示词
# ============================================================

RUST_FEATURE_MATCHING_SYSTEM_PROMPT = """你是一位精通 C 和 Rust 的代码翻译专家。你的任务是分析 Rust 代码是否正确实现了 C 代码的所有功能点。

请严格按照指定的 JSON 格式输出，不要添加任何额外的解释文字。"""

RUST_FEATURE_MATCHING_USER_PROMPT = """请分析 Rust 代码是否正确实现了以下 C 代码功能点。

## C 代码功能点列表

```json
{c_features_json}
```

## Rust 代码

文件名: {rust_filename}

```rust
{rust_code}
```

## 分析要求

对于每个 C 功能点，请分析：

1. **匹配状态**
   - `FULL_MATCH`: 完全实现，语义等价
   - `PARTIAL_MATCH`: 部分实现，存在差异但核心功能正确
   - `MISSING`: 未实现或找不到对应代码
   - `ADAPTED`: 按 Rust 惯用法适配（如内存安全改造、Option/Result 使用）

2. **语义等价性判断**
   - 即使代码形式不同，只要行为等价就算语义等价
   - Rust 的安全改造（如 unsafe 包装、Option 类型）如果不改变逻辑，算语义等价

3. **差异分析** (如果不是 FULL_MATCH)
   - 说明具体差异
   - 评估是否影响语义等价性

## 输出格式 (严格 JSON)

```json
{{
  "rust_file": "Rust文件名",
  "matching_results": [
    {{
      "c_feature_id": "F001",
      "c_feature_name": "对应的C功能点名称",
      "match_status": "FULL_MATCH",
      "rust_element": {{
        "type": "function/struct/const/static/impl",
        "name": "Rust中对应的名称",
        "found": true
      }},
      "semantic_equivalence": true,
      "differences": [],
      "notes": ""
    }},
    {{
      "c_feature_id": "F002",
      "c_feature_name": "对应的C功能点名称",
      "match_status": "PARTIAL_MATCH",
      "rust_element": {{
        "type": "function",
        "name": "rust_func_name",
        "found": true
      }},
      "semantic_equivalence": true,
      "differences": [
        {{
          "type": "类型变更",
          "c_version": "int*",
          "rust_version": "*mut c_int",
          "reason": "FFI 兼容性转换",
          "impact": "无语义影响"
        }}
      ],
      "notes": "类型转换是合理的FFI适配"
    }},
    {{
      "c_feature_id": "F003",
      "c_feature_name": "对应的C功能点名称",
      "match_status": "MISSING",
      "rust_element": {{
        "type": "unknown",
        "name": "",
        "found": false
      }},
      "semantic_equivalence": false,
      "differences": [
        {{
          "type": "功能缺失",
          "c_version": "完整的错误处理",
          "rust_version": "未实现",
          "reason": "翻译遗漏",
          "impact": "可能影响错误处理"
        }}
      ],
      "notes": "建议补充该功能的实现"
    }}
  ],
  "additional_rust_features": [
    {{
      "name": "新增的Rust功能名",
      "description": "功能描述",
      "reason": "添加原因（如Rust安全要求、辅助函数等）"
    }}
  ],
  "overall_assessment": "整体评估说明（一句话）"
}}
```

注意：
1. 只输出 JSON，不要有其他文字
2. 确保 JSON 格式正确
3. 每个 C 功能点都必须有对应的匹配结果
4. differences 数组可以为空（当 FULL_MATCH 时）
5. 对于 `unimplemented!()` 占位符，应标记为 MISSING"""

# ============================================================
# 简化版提示词（用于大文件，减少 token 消耗）
# ============================================================

C_FEATURE_EXTRACTION_SIMPLE_PROMPT = """分析以下 C 代码，提取关键功能点。只关注：
1. 公开函数（非 static）
2. 重要的数据结构
3. 全局变量

输出 JSON 格式：
```json
{{
  "file_name": "文件名",
  "summary": "功能概述",
  "feature_points": [
    {{"id": "F001", "type": "FUNC/STRUCT/GLOBAL", "name": "名称", "description": "描述", "importance": "高/中/低"}}
  ]
}}
```

C 代码 ({filename}):
```c
{c_code}
```"""

RUST_FEATURE_MATCHING_SIMPLE_PROMPT = """对比 C 功能点和 Rust 代码，判断匹配状态。

C 功能点:
{c_features_json}

Rust 代码 ({rust_filename}):
```rust
{rust_code}
```

输出 JSON：
```json
{{
  "matching_results": [
    {{"c_feature_id": "F001", "match_status": "FULL_MATCH/PARTIAL_MATCH/MISSING/ADAPTED", "semantic_equivalence": true/false, "notes": "说明"}}
  ]
}}
```"""

# ============================================================
# 配置常量
# ============================================================

# 文件大小阈值（字符数），超过则使用简化提示词
LARGE_FILE_THRESHOLD = 15000

# 每个功能点的最大代码片段长度
MAX_CODE_SNIPPET_LENGTH = 200

# JSON 提取的正则模式
JSON_EXTRACT_PATTERNS = [
    r'```json\s*([\s\S]*?)\s*```',  # markdown 代码块
    r'```\s*([\s\S]*?)\s*```',       # 普通代码块
    r'(\{[\s\S]*\})',                # 直接的 JSON 对象
]



