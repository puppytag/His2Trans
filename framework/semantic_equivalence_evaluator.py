#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
C2Rust 语义等价性评估器

使用 LLM 进行结构化功能点分析，评估 C 到 Rust 翻译的语义等价性。
采用三阶段评估方法：
1. 功能点提取：分析 C 代码，提取所有功能点
2. 功能点匹配：分析 Rust 代码，匹配 C 功能点
3. 得分计算：基于匹配结果计算等价性得分

使用方法：
    python semantic_equivalence_evaluator.py --projects qos,mini --outputs-dir ./translation_outputs
    python semantic_equivalence_evaluator.py --all --outputs-dir ./translation_outputs
"""

import os
import sys
import json
import re
import argparse
import logging
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, field, asdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from threading import Semaphore
import traceback

# 添加项目根目录到 path
SCRIPT_DIR = Path(__file__).parent.resolve()
sys.path.insert(0, str(SCRIPT_DIR))

# 导入提示词模板
from config.evaluation_prompts import (
    C_FEATURE_EXTRACTION_SYSTEM_PROMPT,
    C_FEATURE_EXTRACTION_USER_PROMPT,
    RUST_FEATURE_MATCHING_SYSTEM_PROMPT,
    RUST_FEATURE_MATCHING_USER_PROMPT,
    C_FEATURE_EXTRACTION_SIMPLE_PROMPT,
    RUST_FEATURE_MATCHING_SIMPLE_PROMPT,
    LARGE_FILE_THRESHOLD,
    JSON_EXTRACT_PATTERNS,
)
from llm_global_concurrency import llm_global_slot

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)


def _client_base_url(llm_client: object) -> str:
    """读取 OpenAI-compatible client 的 base_url。"""
    value = getattr(llm_client, "base_url", "") or getattr(llm_client, "_base_url", "")
    return str(value or "")


def _deepseek_request_kwargs(model_name: str, llm_client: object = None) -> Dict[str, object]:
    """复用 generation.py 的 DeepSeek V4 请求参数。"""
    try:
        from generate.generation import EXTERNAL_API_BASE_URL, deepseek_v4_request_kwargs
        return deepseek_v4_request_kwargs(model_name, _client_base_url(llm_client) or EXTERNAL_API_BASE_URL)
    except Exception:
        return {}


# ============================================================
# 评估长度控制（避免“截断后评估”导致不公平）
# ============================================================

# 单个文件（C 或 Rust）最大 token（估计值，rough）；超过则跳过该文件评估
SEMANTIC_EVAL_MAX_CODE_TOKENS = int(os.environ.get("SEMANTIC_EVAL_MAX_CODE_TOKENS", "50000"))
# 是否跳过超长文件（默认 true）
SEMANTIC_EVAL_SKIP_TOO_LONG_FILES = os.environ.get("SEMANTIC_EVAL_SKIP_TOO_LONG_FILES", "true").lower() == "true"
# C 功能点 JSON 最大长度（字符）；超过则跳过该文件评估（默认 20000，与原逻辑保持一致但不再截断）
SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS = int(os.environ.get("SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS", "20000"))


def rough_token_estimate(text: str) -> int:
    """粗略估算 token 数（不引入 tokenizer 依赖）。"""
    if not text:
        return 0
    # 对代码类文本，用 utf-8 bytes/4 作为近似
    try:
        return max(1, len(text.encode("utf-8", errors="ignore")) // 4)
    except Exception:
        return max(1, len(text) // 4)

# ============================================================
# 数据类定义
# ============================================================

@dataclass
class FeaturePoint:
    """C 代码功能点"""
    id: str
    type: str  # FUNC, STRUCT, GLOBAL, MEMORY, ERROR, EXTERN
    name: str
    description: str
    importance: str  # 高, 中, 低
    code_snippet: str = ""
    details: Dict = field(default_factory=dict)


@dataclass
class MatchingResult:
    """功能点匹配结果"""
    c_feature_id: str
    c_feature_name: str
    match_status: str  # FULL_MATCH, PARTIAL_MATCH, MISSING, ADAPTED
    rust_element: Dict = field(default_factory=dict)
    semantic_equivalence: bool = False
    differences: List[Dict] = field(default_factory=list)
    notes: str = ""


@dataclass
class FileEvaluation:
    """单个文件的评估结果"""
    c_file: str
    rust_file: str
    c_features: List[FeaturePoint] = field(default_factory=list)
    matching_results: List[MatchingResult] = field(default_factory=list)
    additional_rust_features: List[Dict] = field(default_factory=list)
    score: float = 0.0
    error: str = ""


@dataclass
class ProjectEvaluation:
    """项目级评估结果"""
    project_name: str
    evaluation_time: str
    overall_score: float = 0.0
    summary: Dict = field(default_factory=dict)
    file_evaluations: List[FileEvaluation] = field(default_factory=list)
    recommendations: List[str] = field(default_factory=list)
    error: str = ""


# ============================================================
# 核心评估器类
# ============================================================

class SemanticEquivalenceEvaluator:
    """语义等价性评估器"""

    # vLLM 并发控制信号量 - 限制同时最多 120 个请求
    # 从环境变量获取配置，默认 120
    VLLM_CONCURRENT_LIMIT = int(os.environ.get("SEMANTIC_EVAL_VLLM_CONCURRENT_LIMIT", "120"))
    _vllm_semaphore = Semaphore(VLLM_CONCURRENT_LIMIT)

    def __init__(
        self,
        vllm_url: str = None,
        model_name: str = None,
        max_retries: int = 3,
        timeout: float = None,
        file_parallel_workers: int = 8  # 新增：文件级别并行数
    ):
        """
        初始化评估器

        Args:
            vllm_url: vLLM 服务地址（默认从 generation.py 获取）
            model_name: 模型名称（默认从 generation.py 获取）
            max_retries: 最大重试次数
            timeout: 请求超时时间（秒）（默认从 generation.py 获取）
            file_parallel_workers: 文件级别并行评估数（默认8）
        """
        # 复用 generation.py 的配置
        try:
            from generate.generation import (
                USE_VLLM, VLLM_BASE_URL, VLLM_API_KEY, VLLM_MODEL_NAME, VLLM_REQUEST_TIMEOUT,
                EXTERNAL_API_BASE_URL, EXTERNAL_API_KEY, EXTERNAL_API_MODEL, EXTERNAL_API_TIMEOUT
            )
            inferred_use_vllm = USE_VLLM
            if USE_VLLM:
                default_url = VLLM_BASE_URL
                default_model = VLLM_MODEL_NAME
                self.api_key = VLLM_API_KEY
                default_timeout = VLLM_REQUEST_TIMEOUT
            else:
                default_url = EXTERNAL_API_BASE_URL
                default_model = EXTERNAL_API_MODEL
                self.api_key = EXTERNAL_API_KEY
                default_timeout = EXTERNAL_API_TIMEOUT
        except ImportError:
            default_url = "http://localhost:8000/v1"
            default_model = "qwen3_coder"
            self.api_key = "EMPTY"
            default_timeout = 300.0
            inferred_use_vllm = True

        self.vllm_url = vllm_url or default_url
        self.model_name = model_name or default_model
        self.use_vllm = inferred_use_vllm
        self.max_retries = max_retries
        self.timeout = timeout or default_timeout
        self.file_parallel_workers = file_parallel_workers
        self.client = None

        self._init_client()

    def _init_client(self):
        """初始化 OpenAI 客户端"""
        try:
            from openai import OpenAI
            self.client = OpenAI(
                base_url=self.vllm_url,
                api_key=self.api_key,
                timeout=self.timeout
            )
            # 测试连接
            self.client.models.list()
            logger.info(f"✓ 已连接到 vLLM: {self.vllm_url}")
        except Exception as e:
            logger.error(f"✗ 无法连接到 vLLM ({self.vllm_url}): {e}")
            raise RuntimeError(f"vLLM 连接失败: {e}")
    
    def _call_llm(self, system_prompt: str, user_prompt: str) -> str:
        """
        调用 LLM 生成响应（带并发控制）

        Args:
            system_prompt: 系统提示词
            user_prompt: 用户提示词

        Returns:
            LLM 响应内容
        """
        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ]

        # 使用信号量控制并发：同时最多 VLLM_CONCURRENT_LIMIT 个请求
        self._vllm_semaphore.acquire()
        try:
            for attempt in range(self.max_retries):
                try:
                    kwargs = {
                        "model": self.model_name,
                        "messages": messages,
                        "temperature": 0.0,  # 确定性输出
                    }
                    if self.use_vllm:
                        kwargs["max_tokens"] = 8192
                        kwargs["stop"] = ["<|im_end|>"]
                    else:
                        kwargs.update(_deepseek_request_kwargs(self.model_name, self.client))
                    with llm_global_slot(
                        base_url=self.vllm_url,
                        model=self.model_name,
                        label="semantic_equivalence_evaluator",
                    ):
                        response = self.client.chat.completions.create(**kwargs)

                    if response.choices and response.choices[0].message.content:
                        return response.choices[0].message.content
                    else:
                        raise ValueError("LLM 返回空响应")

                except Exception as e:
                    logger.warning(f"LLM 调用失败 (尝试 {attempt + 1}/{self.max_retries}): {e}")
                    if attempt == self.max_retries - 1:
                        raise
        finally:
            self._vllm_semaphore.release()

        return ""
    
    def _extract_json(self, text: str) -> Optional[Dict]:
        """
        从 LLM 响应中提取 JSON
        
        Args:
            text: LLM 响应文本
            
        Returns:
            解析后的 JSON 字典，失败返回 None
        """
        # 尝试多种模式提取 JSON
        for pattern in JSON_EXTRACT_PATTERNS:
            matches = re.findall(pattern, text, re.DOTALL)
            for match in matches:
                try:
                    # 清理可能的格式问题
                    cleaned = match.strip()
                    # 移除可能的 BOM 和特殊字符
                    cleaned = cleaned.replace('\ufeff', '').replace('\u200b', '')
                    result = json.loads(cleaned)
                    return result
                except json.JSONDecodeError:
                    continue
        
        # 最后尝试直接解析整个文本
        try:
            return json.loads(text.strip())
        except json.JSONDecodeError:
            return None
    
    def _extract_c_features(self, c_code: str, filename: str, max_retries: int = 3) -> Tuple[List[FeaturePoint], str, str]:
        """
        阶段 1: 提取 C 代码功能点（带重试机制）
        
        Args:
            c_code: C 源代码
            filename: 文件名
            max_retries: 最大重试次数
            
        Returns:
            (功能点列表, 文件摘要, 错误原因(成功为空字符串))
        """
        # 超长文件：直接跳过（避免截断后评估带来的不公平）
        if SEMANTIC_EVAL_SKIP_TOO_LONG_FILES:
            est_tokens = rough_token_estimate(c_code)
            if est_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS:
                return [], "", f"跳过评估：C 代码过长 (≈{est_tokens} tokens > {SEMANTIC_EVAL_MAX_CODE_TOKENS})"
        
        user_prompt = C_FEATURE_EXTRACTION_USER_PROMPT.format(
            filename=filename,
            c_code=c_code
        )
        system_prompt = C_FEATURE_EXTRACTION_SYSTEM_PROMPT
        
        last_error = None
        last_reason = ""
        for attempt in range(max_retries):
            try:
                response = self._call_llm(system_prompt, user_prompt)
                
                if not response:
                    logger.warning(f"  ⚠ LLM 返回空响应 (尝试 {attempt + 1}/{max_retries})")
                    last_reason = "LLM 返回空响应"
                    continue
                
                result = self._extract_json(response)
                
                if not result:
                    logger.warning(f"  ⚠ 无法解析 C 功能点 JSON (尝试 {attempt + 1}/{max_retries})，尝试修复")
                    # 尝试修复常见的 JSON 格式问题
                    result = self._try_fix_json(response)
                
                if not result:
                    # JSON 解析失败，尝试更简化的提示词重试
                    if attempt < max_retries - 1:
                        logger.info(f"  🔄 JSON 解析失败，使用简化提示词重试...")
                        last_reason = "LLM 返回无法解析的 JSON"
                        # 使用更简化的提示词
                        user_prompt = f"""分析以下 C 代码文件，提取所有功能点。

文件名: {filename}

```c
{c_code}
```

请只输出 JSON 格式，结构如下：
{{
  "feature_points": [
    {{"id": "F001", "type": "FUNC", "name": "函数名", "description": "描述", "importance": "高"}}
  ],
  "summary": "文件摘要"
}}"""
                        system_prompt = "你是 C 语言专家。只输出有效的 JSON，不要其他文字。不要使用 markdown 代码块。"
                        continue
                    else:
                        logger.error(f"  ✗ C 功能点提取失败: JSON 解析错误 (已重试 {max_retries} 次)")
                        return [], "", "LLM 返回无法解析的 JSON"
                
                # 成功解析
                features = []
                for fp in result.get("feature_points", []):
                    features.append(FeaturePoint(
                        id=fp.get("id", f"F{len(features)+1:03d}"),
                        type=fp.get("type", "UNKNOWN"),
                        name=fp.get("name", ""),
                        description=fp.get("description", ""),
                        importance=fp.get("importance", "中"),
                        code_snippet=fp.get("code_snippet", ""),
                        details=fp.get("details", {})
                    ))
                
                summary = result.get("summary", "")
                logger.info(f"  ✓ 提取了 {len(features)} 个功能点" + (f" (尝试 {attempt + 1})" if attempt > 0 else ""))
                if not features:
                    # 有些文件会因为提示词不稳/代码风格导致提取到 0；先用 SIMPLE_PROMPT 再试一次。
                    if attempt < max_retries - 1:
                        logger.info("  🔄 提取到 0 个功能点，使用 SIMPLE_PROMPT 重试...")
                        user_prompt = C_FEATURE_EXTRACTION_SIMPLE_PROMPT.format(
                            filename=filename,
                            c_code=c_code
                        )
                        system_prompt = "你是 C 语言专家。只输出有效的 JSON，不要其他文字。不要使用 markdown 代码块。"
                        last_reason = "LLM 提取到 0 个 feature_points"
                        continue
                    return [], summary, "LLM 提取到 0 个 feature_points（可能该文件无可评估行为/提示词不稳）"
                return features, summary, ""
                
            except Exception as e:
                last_error = e
                logger.warning(f"  ⚠ C 功能点提取异常 (尝试 {attempt + 1}/{max_retries}): {e}")
                last_reason = str(e)
                if attempt < max_retries - 1:
                    continue
        
        logger.error(f"  ✗ C 功能点提取失败: {last_error}")
        return [], "", (last_reason or str(last_error) or "unknown error")
    
    def _match_rust_features(
        self,
        rust_code: str,
        rust_filename: str,
        c_features: List[FeaturePoint],
        max_retries: int = 3
    ) -> Tuple[List[MatchingResult], List[Dict], str]:
        """
        阶段 2: 匹配 Rust 代码功能点（带重试机制）
        
        Args:
            rust_code: Rust 源代码
            rust_filename: Rust 文件名
            c_features: C 功能点列表
            max_retries: 最大重试次数
            
        Returns:
            (匹配结果列表, 新增的 Rust 功能列表, 错误原因(成功为空字符串))
        """
        # 超长文件：直接跳过（避免截断后评估带来的不公平）
        if SEMANTIC_EVAL_SKIP_TOO_LONG_FILES:
            est_tokens = rough_token_estimate(rust_code)
            if est_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS:
                return [], [], f"跳过评估：Rust 代码过长 (≈{est_tokens} tokens > {SEMANTIC_EVAL_MAX_CODE_TOKENS})"
        
        # 构建 C 功能点 JSON
        def _compact_features(level: int) -> List[Dict]:
            """
            level:
              0 - keep id/type/name/importance/description (truncated)
              1 - keep id/type/name/importance
              2 - keep id/name
            """
            out: List[Dict] = []
            for fp in c_features:
                desc = (fp.description or "").replace("\n", " ").strip()
                if len(desc) > 240:
                    desc = desc[:240] + "..."
                if level == 0:
                    out.append(
                        {
                            "id": fp.id,
                            "type": fp.type,
                            "name": fp.name,
                            "importance": fp.importance,
                            "description": desc,
                        }
                    )
                elif level == 1:
                    out.append(
                        {
                            "id": fp.id,
                            "type": fp.type,
                            "name": fp.name,
                            "importance": fp.importance,
                        }
                    )
                else:
                    out.append(
                        {
                            "id": fp.id,
                            "name": fp.name,
                        }
                    )
            return out

        # 先用紧凑 JSON（避免因为 `indent=2` 放大导致“评估跳过”）
        compact_level = 0
        c_features_json = json.dumps(
            _compact_features(compact_level),
            ensure_ascii=False,
            separators=(",", ":")
        )

        # 如果仍过长，逐级降采样字段
        while len(c_features_json) > SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS and compact_level < 2:
            compact_level += 1
            c_features_json = json.dumps(
                _compact_features(compact_level),
                ensure_ascii=False,
                separators=(",", ":")
            )

        if len(c_features_json) > SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS:
            return [], [], (
                f"跳过评估：C 功能点 JSON 过长 "
                f"({len(c_features_json)} chars > {SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS}); "
                f"可通过 SEMANTIC_EVAL_MAX_FEATURES_JSON_CHARS 调整阈值。"
            )
        
        user_prompt = RUST_FEATURE_MATCHING_USER_PROMPT.format(
            c_features_json=c_features_json,
            rust_filename=rust_filename,
            rust_code=rust_code
        )
        system_prompt = RUST_FEATURE_MATCHING_SYSTEM_PROMPT
        
        last_error = None
        for attempt in range(max_retries):
            try:
                response = self._call_llm(system_prompt, user_prompt)
                
                if not response:
                    logger.warning(f"  ⚠ LLM 返回空响应 (尝试 {attempt + 1}/{max_retries})")
                    continue
                
                result = self._extract_json(response)
                
                if not result:
                    logger.warning(f"  ⚠ 无法解析匹配结果 JSON (尝试 {attempt + 1}/{max_retries})，尝试修复")
                    result = self._try_fix_json(response)
                
                if not result:
                    # JSON 解析失败，尝试更简化的提示词重试
                    if attempt < max_retries - 1:
                        logger.info(f"  🔄 JSON 解析失败，使用简化提示词重试...")
                        # 只传递功能点名称列表
                        feature_names = [fp.name for fp in c_features]
                        user_prompt = f"""比较 C 功能点与 Rust 代码的匹配情况。

C 功能点列表: {feature_names}

Rust 代码 ({rust_filename}):
```rust
{rust_code}
```

请只输出 JSON，结构如下：
{{
  "matching_results": [
    {{"c_feature_id": "F001", "c_feature_name": "名称", "match_status": "FULL_MATCH|PARTIAL_MATCH|MISSING", "semantic_equivalence": true/false}}
  ]
}}"""
                        system_prompt = "你是代码翻译专家。只输出有效的 JSON，不要其他文字。不要使用 markdown 代码块。"
                        continue
                    else:
                        logger.error(f"  ✗ Rust 匹配分析失败: JSON 解析错误 (已重试 {max_retries} 次)")
                        # 返回默认的 MISSING 结果
                        return [
                            MatchingResult(
                                c_feature_id=fp.id,
                                c_feature_name=fp.name,
                                match_status="MISSING",
                                semantic_equivalence=False,
                                notes="JSON 解析失败，无法分析"
                            ) for fp in c_features
                        ], [], ""
                
                # 成功解析
                matching_results = []
                for mr in result.get("matching_results", []):
                    matching_results.append(MatchingResult(
                        c_feature_id=mr.get("c_feature_id", ""),
                        c_feature_name=mr.get("c_feature_name", ""),
                        match_status=mr.get("match_status", "MISSING"),
                        rust_element=mr.get("rust_element", {}),
                        semantic_equivalence=mr.get("semantic_equivalence", False),
                        differences=mr.get("differences", []),
                        notes=mr.get("notes", "")
                    ))
                
                # 确保每个 C 功能点都有匹配结果
                matched_ids = {mr.c_feature_id for mr in matching_results}
                for fp in c_features:
                    if fp.id not in matched_ids:
                        matching_results.append(MatchingResult(
                            c_feature_id=fp.id,
                            c_feature_name=fp.name,
                            match_status="MISSING",
                            semantic_equivalence=False,
                            notes="未在匹配结果中找到"
                        ))
                
                additional_features = result.get("additional_rust_features", [])
                
                logger.info(f"  ✓ 完成匹配分析，{len(matching_results)} 个结果" + (f" (尝试 {attempt + 1})" if attempt > 0 else ""))
                return matching_results, additional_features, ""
                
            except Exception as e:
                last_error = e
                logger.warning(f"  ⚠ Rust 匹配分析异常 (尝试 {attempt + 1}/{max_retries}): {e}")
                if attempt < max_retries - 1:
                    continue
        
        logger.error(f"  ✗ Rust 匹配分析失败: {last_error}")
        return [
            MatchingResult(
                c_feature_id=fp.id,
                c_feature_name=fp.name,
                match_status="MISSING",
                semantic_equivalence=False,
                notes=f"分析异常: {str(last_error)}"
            ) for fp in c_features
        ], [], ""
    
    def _try_fix_json(self, text: str) -> Optional[Dict]:
        """
        尝试修复常见的 JSON 格式问题
        
        修复策略：
        1. 移除 markdown 代码块标记
        2. 提取花括号包围的内容
        3. 修复常见的格式错误（尾随逗号、单引号等）
        4. 尝试多种提取模式
        """
        if not text:
            return None
        
        # 策略 1: 移除 markdown 代码块
        cleaned = text.strip()
        
        # 移除 ```json ... ``` 或 ```...```
        code_block_patterns = [
            r'```json\s*(.*?)\s*```',
            r'```\s*(.*?)\s*```',
        ]
        for pattern in code_block_patterns:
            match = re.search(pattern, cleaned, re.DOTALL)
            if match:
                cleaned = match.group(1).strip()
                break
        
        # 策略 2: 尝试直接解析
        try:
            return json.loads(cleaned)
        except json.JSONDecodeError:
            pass
        
        # 策略 3: 提取花括号包围的内容
        lines = cleaned.split('\n')
        json_lines = []
        in_json = False
        brace_count = 0
        
        for line in lines:
            if '{' in line and not in_json:
                in_json = True
                # 只取从第一个 { 开始的部分
                idx = line.index('{')
                line = line[idx:]
            if in_json:
                json_lines.append(line)
                brace_count += line.count('{') - line.count('}')
                if brace_count <= 0:
                    break
        
        if json_lines:
            json_text = '\n'.join(json_lines)
            
            # 策略 4: 修复常见格式错误
            # 移除尾随逗号
            json_text = re.sub(r',\s*([\]}])', r'\1', json_text)
            # 单引号替换为双引号
            json_text = re.sub(r"'([^']*)'", r'"\1"', json_text)
            # 修复没有引号的键名
            json_text = re.sub(r'(\{|\,)\s*(\w+)\s*:', r'\1"\2":', json_text)
            
            try:
                return json.loads(json_text)
            except json.JSONDecodeError:
                pass
            
            # 策略 5: 更激进的清理
            # 只保留 ASCII 和常见 Unicode
            json_text_ascii = ''.join(c if ord(c) < 128 or c in '中文' else ' ' for c in json_text)
            try:
                return json.loads(json_text_ascii)
            except json.JSONDecodeError:
                pass
        
        # 策略 6: 尝试找到任何有效的 JSON 对象
        # 使用更宽松的正则表达式
        json_pattern = r'\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\}'
        matches = re.findall(json_pattern, cleaned, re.DOTALL)
        for match in matches:
            try:
                result = json.loads(match)
                if isinstance(result, dict):
                    return result
            except json.JSONDecodeError:
                continue
        
        return None
    
    def _calculate_score(
        self,
        matching_results: List[MatchingResult],
        c_features: List[FeaturePoint]
    ) -> float:
        """
        阶段 3: 计算语义等价性得分
        
        Args:
            matching_results: 匹配结果列表
            c_features: C 功能点列表
            
        Returns:
            等价性得分 (0-100)
        """
        if not c_features:
            return 0.0
        
        # 关键程度权重映射
        importance_weights = {"高": 3.0, "中": 2.0, "低": 1.0}
        
        # 匹配状态基础得分
        match_scores = {
            "FULL_MATCH": 1.0,
            "ADAPTED": 0.95,
            "PARTIAL_MATCH": 0.6,
            "MISSING": 0.0
        }
        
        # 构建功能点查找字典
        feature_dict = {fp.id: fp for fp in c_features}
        
        total_weight = 0.0
        weighted_score = 0.0
        
        for result in matching_results:
            # 获取对应的 C 功能点
            c_feature = feature_dict.get(result.c_feature_id)
            if not c_feature:
                continue
            
            # 获取权重
            weight = importance_weights.get(c_feature.importance, 1.0)
            
            # 获取基础得分
            base_score = match_scores.get(result.match_status, 0.0)
            
            # 如果部分匹配但语义等价，提升得分
            if result.match_status == "PARTIAL_MATCH" and result.semantic_equivalence:
                base_score = 0.85
            
            total_weight += weight
            weighted_score += weight * base_score
        
        if total_weight == 0:
            return 0.0
        
        final_score = (weighted_score / total_weight) * 100
        return round(final_score, 2)
    
    def _generate_recommendations(
        self,
        matching_results: List[MatchingResult],
        c_features: List[FeaturePoint]
    ) -> List[str]:
        """生成改进建议"""
        recommendations = []
        feature_dict = {fp.id: fp for fp in c_features}
        
        for result in matching_results:
            c_feature = feature_dict.get(result.c_feature_id)
            if not c_feature:
                continue
            
            if result.match_status == "MISSING":
                recommendations.append(
                    f"[缺失] {result.c_feature_id} ({c_feature.name}): "
                    f"建议实现该功能 - {c_feature.description}"
                )
            elif result.match_status == "PARTIAL_MATCH" and not result.semantic_equivalence:
                diff_summary = "; ".join(
                    d.get("type", "未知差异") for d in result.differences[:2]
                )
                recommendations.append(
                    f"[部分匹配] {result.c_feature_id} ({c_feature.name}): "
                    f"{diff_summary} - {result.notes}"
                )
        
        return recommendations[:10]  # 最多返回 10 条建议
    
    def evaluate_file_pair(
        self,
        c_file_path: str,
        rust_file_path: str
    ) -> FileEvaluation:
        """
        评估单个文件对（文件级别评估）
        
        注意：这是文件级别的评估，每个 C 文件对应一个 Rust 文件，
        不会将所有代码合并在一起分析。每个文件对独立评估。
        
        Args:
            c_file_path: C 文件路径
            rust_file_path: Rust 文件路径
            
        Returns:
            文件评估结果
        """
        c_filename = os.path.basename(c_file_path)
        rust_filename = os.path.basename(rust_file_path)
        
        logger.info(f"📊 评估文件对: {c_filename} ↔ {rust_filename}")
        
        evaluation = FileEvaluation(
            c_file=c_filename,
            rust_file=rust_filename
        )
        
        try:
            # 读取文件内容
            with open(c_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                c_code = f.read()
            
            with open(rust_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                rust_code = f.read()
            
            if not c_code.strip():
                evaluation.error = "C 文件为空"
                return evaluation
            
            if not rust_code.strip():
                evaluation.error = "Rust 文件为空"
                return evaluation

            # 超长文件：直接跳过（避免截断后评估带来的不公平）
            if SEMANTIC_EVAL_SKIP_TOO_LONG_FILES:
                c_tokens = rough_token_estimate(c_code)
                r_tokens = rough_token_estimate(rust_code)
                if c_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS or r_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS:
                    parts = []
                    if c_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS:
                        parts.append(f"C≈{c_tokens}")
                    if r_tokens > SEMANTIC_EVAL_MAX_CODE_TOKENS:
                        parts.append(f"Rust≈{r_tokens}")
                    evaluation.error = (
                        f"跳过评估：代码过长 ({', '.join(parts)} tokens > {SEMANTIC_EVAL_MAX_CODE_TOKENS}). "
                        f"可通过 SEMANTIC_EVAL_MAX_CODE_TOKENS 调整阈值。"
                    )
                    return evaluation
            
            # 阶段 1: 提取 C 功能点
            logger.info(f"  [1/3] 提取 C 代码功能点...")
            c_features, summary, c_err = self._extract_c_features(c_code, c_filename)
            
            if not c_features:
                evaluation.error = f"无法提取 C 功能点: {c_err}" if c_err else "无法提取 C 功能点"
                return evaluation
            
            evaluation.c_features = c_features
            
            # 阶段 2: 匹配 Rust 功能点
            logger.info(f"  [2/3] 匹配 Rust 代码功能点...")
            matching_results, additional_features, match_err = self._match_rust_features(
                rust_code, rust_filename, c_features
            )

            if match_err:
                evaluation.error = match_err
                return evaluation
            
            evaluation.matching_results = matching_results
            evaluation.additional_rust_features = additional_features
            
            # 阶段 3: 计算得分
            logger.info(f"  [3/3] 计算等价性得分...")
            evaluation.score = self._calculate_score(matching_results, c_features)
            
            logger.info(f"  ✓ 评估完成，得分: {evaluation.score}")
            
        except Exception as e:
            evaluation.error = f"评估异常: {str(e)}"
            logger.error(f"  ✗ {evaluation.error}")
            traceback.print_exc()
        
        return evaluation
    
    def evaluate_project(
        self,
        project_name: str,
        workspace_dir: str,
        max_files: int = 0
    ) -> ProjectEvaluation:
        """
        评估单个项目
        
        Args:
            project_name: 项目名称
            workspace_dir: 工作空间目录
            max_files: 最大评估文件数 (0=全部)
            
        Returns:
            项目评估结果
        """
        logger.info(f"\n{'='*60}")
        logger.info(f"🔍 开始评估项目: {project_name}")
        logger.info(f"{'='*60}")
        
        evaluation = ProjectEvaluation(
            project_name=project_name,
            evaluation_time=datetime.now().isoformat()
        )
        
        try:
            workspace_path = Path(workspace_dir)
            
            # 查找 C 源代码目录
            # 注意：目录名可能与 project_name 不完全匹配
            # 例如 project_name="event" 但目录是 "event_manager"
            c_source_base = workspace_path / "c_source"
            c_source_dir = None
            
            if c_source_base.exists():
                # 1. 首先尝试精确匹配
                exact_match = c_source_base / project_name
                if exact_match.exists() and exact_match.is_dir():
                    c_source_dir = exact_match
                else:
                    # 2. 尝试前缀匹配（project_name 是目录名的前缀）
                    for item in c_source_base.iterdir():
                        if item.is_dir():
                            # 检查是否以 project_name 开头或包含 project_name
                            if item.name.startswith(project_name) or project_name in item.name:
                                c_source_dir = item
                                logger.info(f"  ✓ 匹配 C 源码目录: {project_name} -> {item.name}")
                                break
            
            if not c_source_dir or not c_source_dir.exists():
                evaluation.error = f"C 源代码目录不存在: {c_source_base / project_name}"
                logger.error(f"✗ {evaluation.error}")
                return evaluation
            
            # 获取实际的目录名（用于后续查找 Rust 目录）
            actual_dir_name = c_source_dir.name
            
            # 查找 Rust 代码目录（优先 final_projects，其次 skeletons）
            rust_dir = None
            for rust_base in ["final_projects", "skeletons"]:
                # 尝试使用实际目录名匹配
                for candidate_name in [actual_dir_name, project_name]:
                    candidate = workspace_path / rust_base / candidate_name
                    if candidate.exists():
                        # 查找 src 目录
                        src_dir = candidate / "src"
                        if not src_dir.exists():
                            # 可能在 translate_by_xxx 子目录下
                            for subdir in candidate.iterdir():
                                if subdir.is_dir() and subdir.name.startswith("translate_by_"):
                                    src_dir = subdir / "src"
                                    if src_dir.exists():
                                        rust_dir = src_dir
                                        break
                        else:
                            rust_dir = src_dir
                        
                        if rust_dir:
                            break
                
                if rust_dir:
                    break
            
            if not rust_dir or not rust_dir.exists():
                evaluation.error = f"Rust 源代码目录不存在"
                logger.error(f"✗ {evaluation.error}")
                return evaluation
            
            logger.info(f"  C 源码: {c_source_dir}")
            logger.info(f"  Rust 源码: {rust_dir}")
            
            # 收集文件对
            file_pairs = self._collect_file_pairs(c_source_dir, rust_dir)
            
            if not file_pairs:
                evaluation.error = "未找到可评估的文件对"
                logger.warning(f"⚠ {evaluation.error}")
                return evaluation
            
            logger.info(f"  找到 {len(file_pairs)} 个文件对")
            
            # 限制文件数
            if max_files > 0 and len(file_pairs) > max_files:
                logger.info(f"  限制评估前 {max_files} 个文件")
                file_pairs = file_pairs[:max_files]
            
            # 评估每个文件对（文件级别评估，每个文件对独立分析）
            # 注意：不会将所有代码合并，而是逐个文件对进行评估
            all_recommendations = []

            # 【优化】使用多线程并行评估文件对
            logger.info(f"  使用 {self.file_parallel_workers} 个并行工作线程评估文件")

            with ThreadPoolExecutor(max_workers=self.file_parallel_workers) as executor:
                # 提交所有评估任务
                future_to_files = {
                    executor.submit(self.evaluate_file_pair, c_file, rust_file): (c_file, rust_file)
                    for c_file, rust_file in file_pairs
                }

                # 收集结果
                for future in as_completed(future_to_files):
                    c_file, rust_file = future_to_files[future]
                    try:
                        file_eval = future.result()
                        evaluation.file_evaluations.append(file_eval)

                        # 收集建议
                        if file_eval.c_features and file_eval.matching_results:
                            recs = self._generate_recommendations(
                                file_eval.matching_results,
                                file_eval.c_features
                            )
                            all_recommendations.extend(recs)
                    except Exception as e:
                        logger.error(f"  ✗ 评估文件对失败 ({c_file} -> {rust_file}): {e}")
                        # 创建错误的 file_eval
                        error_eval = FileEvaluation(
                            c_file=str(c_file),
                            rust_file=str(rust_file),
                            error=f"评估异常: {str(e)}"
                        )
                        evaluation.file_evaluations.append(error_eval)
            
            # 计算整体得分和统计
            evaluation.overall_score = self._calculate_project_score(evaluation)
            evaluation.summary = self._calculate_summary(evaluation)
            evaluation.recommendations = all_recommendations[:15]

            # 如果所有文件都报错/被跳过，则标记项目级 error，避免在汇总里被当作“0 分语义不等价”
            try:
                if (
                    evaluation.summary.get("files_evaluated", 0) > 0
                    and evaluation.summary.get("files_with_errors", 0) == evaluation.summary.get("files_evaluated", 0)
                ):
                    evaluation.error = (
                        "所有文件评估失败/被跳过（常见原因：代码过长、C 功能点 JSON 过长、LLM 输出异常）。"
                    )
            except Exception:
                pass
            
            logger.info(f"\n✓ 项目 {project_name} 评估完成")
            logger.info(f"  整体得分: {evaluation.overall_score}")
            logger.info(f"  统计: {evaluation.summary}")
            
        except Exception as e:
            evaluation.error = f"项目评估异常: {str(e)}"
            logger.error(f"✗ {evaluation.error}")
            traceback.print_exc()
        
        return evaluation
    
    def _should_skip_rust_file(self, rust_filename: str) -> bool:
        """
        判断是否应该跳过某个 Rust 文件的评估
        
        Args:
            rust_filename: Rust 文件名
            
        Returns:
            True 表示应该跳过，False 表示需要评估
        """
        # 不需要评估的文件列表（这些是自动生成的或辅助文件）
        skip_patterns = [
            "types.rs",           # bindgen 生成的类型定义
            "globals.rs",         # 全局变量定义
            "main.rs",            # 主入口文件（通常不包含业务逻辑）
            "lib.rs",             # 库入口文件（通常只是模块声明）
            "build.rs",           # 构建脚本
            "mod.rs",             # 模块声明文件
            "bindings.rs",        # FFI 绑定文件
            "ffi.rs",             # FFI 相关
            "wrapper.rs",         # 包装器文件
        ]
        
        rust_filename_lower = rust_filename.lower()
        for pattern in skip_patterns:
            if rust_filename_lower == pattern.lower():
                return True
        
        return False
    
    def _collect_file_pairs(
        self,
        c_source_dir: Path,
        rust_dir: Path
    ) -> List[Tuple[str, str]]:
        """
        收集可匹配的 C/Rust 文件对
        
        文件级别对应规则：
        1. C 文件 xxx.c 对应 Rust 文件 xxx.rs 或 src_xxx.rs
        2. 跳过不需要评估的 Rust 文件（types.rs, globals.rs 等）
        3. 每个文件对独立评估，不合并代码
        """
        file_pairs = []
        
        # 遍历 C 源文件
        for c_file in c_source_dir.rglob("*.c"):
            # 构建可能的 Rust 文件名（按优先级排序）
            c_basename = c_file.stem  # 不含扩展名
            
            # 尝试多种命名模式（按常见程度排序）
            possible_rust_names = [
                f"{c_basename}.rs",                    # 直接对应：xxx.c -> xxx.rs
                f"src_{c_basename}.rs",                # 带 src_ 前缀：xxx.c -> src_xxx.rs
                f"{c_file.parent.name}_{c_basename}.rs", # 带父目录前缀
            ]
            
            # 如果 C 文件在子目录中，也尝试路径组合
            rel_path = c_file.relative_to(c_source_dir)
            if len(rel_path.parts) > 1:
                path_prefix = "_".join(rel_path.parts[:-1])
                possible_rust_names.append(f"{path_prefix}_{c_basename}.rs")
            
            # 尝试匹配
            matched = False
            for rust_name in possible_rust_names:
                rust_file = rust_dir / rust_name
                if rust_file.exists():
                    # 检查是否需要跳过
                    if self._should_skip_rust_file(rust_name):
                        logger.info(f"  ⏭ 跳过文件: {rust_name} (不需要评估)")
                        continue
                    
                    file_pairs.append((str(c_file), str(rust_file)))
                    matched = True
                    logger.info(f"  ✓ 匹配文件对: {c_file.name} ↔ {rust_name}")
                    break
            
            if not matched:
                logger.warning(f"  ⚠ 未找到对应的 Rust 文件: {c_file.name}")
        
        logger.info(f"  共找到 {len(file_pairs)} 个文件对")
        return file_pairs
    
    def _calculate_project_score(self, evaluation: ProjectEvaluation) -> float:
        """计算项目整体得分"""
        if not evaluation.file_evaluations:
            return 0.0
        
        valid_scores = [
            fe.score for fe in evaluation.file_evaluations
            if fe.score > 0 or not fe.error
        ]
        
        if not valid_scores:
            return 0.0
        
        return round(sum(valid_scores) / len(valid_scores), 2)
    
    def _calculate_summary(self, evaluation: ProjectEvaluation) -> Dict:
        """计算统计摘要"""
        summary = {
            "total_c_features": 0,
            "full_match": 0,
            "adapted": 0,
            "partial_match": 0,
            "missing": 0,
            "files_evaluated": len(evaluation.file_evaluations),
            "files_with_errors": 0
        }
        
        for fe in evaluation.file_evaluations:
            if fe.error:
                summary["files_with_errors"] += 1
                continue
            
            summary["total_c_features"] += len(fe.c_features)
            
            for mr in fe.matching_results:
                status = mr.match_status.upper()
                if status == "FULL_MATCH":
                    summary["full_match"] += 1
                elif status == "ADAPTED":
                    summary["adapted"] += 1
                elif status == "PARTIAL_MATCH":
                    summary["partial_match"] += 1
                elif status == "MISSING":
                    summary["missing"] += 1
        
        return summary


# ============================================================
# 报告生成
# ============================================================

def generate_json_report(evaluation: ProjectEvaluation, output_path: str):
    """生成 JSON 格式报告"""
    
    def convert_to_dict(obj):
        """递归转换数据类为字典"""
        if hasattr(obj, '__dataclass_fields__'):
            return {k: convert_to_dict(v) for k, v in asdict(obj).items()}
        elif isinstance(obj, list):
            return [convert_to_dict(item) for item in obj]
        elif isinstance(obj, dict):
            return {k: convert_to_dict(v) for k, v in obj.items()}
        else:
            return obj
    
    report_dict = convert_to_dict(evaluation)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(report_dict, f, ensure_ascii=False, indent=2)
    
    logger.info(f"📝 JSON 报告已保存: {output_path}")


def generate_markdown_report(evaluation: ProjectEvaluation, output_path: str):
    """生成 Markdown 格式报告"""
    lines = []
    # 预先定义 summary，避免后续改动引入“先使用后赋值”导致 UnboundLocalError
    s = evaluation.summary or {}
    
    lines.append(f"# 语义等价性评估报告: {evaluation.project_name}")
    lines.append("")
    lines.append(f"**评估时间**: {evaluation.evaluation_time}")
    lines.append("")
    
    if evaluation.error:
        lines.append(f"## ❌ 评估错误")
        lines.append(f"```")
        lines.append(evaluation.error)
        lines.append(f"```")
    else:
        # 总体评分
        lines.append(f"## 总体评分: {evaluation.overall_score} / 100")
        lines.append("")
        
        # 评分分布
        lines.append("### 评分分布")
        lines.append("")
        lines.append("| 匹配状态 | 数量 | 说明 |")
        lines.append("|---------|-----|------|")
        lines.append(f"| ✅ 完全匹配 | {s.get('full_match', 0)} | 语义完全等价 |")
        lines.append(f"| 🔄 Rust 适配 | {s.get('adapted', 0)} | 按 Rust 惯用法改造 |")
        lines.append(f"| ⚠️ 部分匹配 | {s.get('partial_match', 0)} | 存在差异但核心正确 |")
        lines.append(f"| ❌ 缺失 | {s.get('missing', 0)} | 未实现 |")
        lines.append("")
        lines.append(f"**总功能点数**: {s.get('total_c_features', 0)}")
        lines.append(f"**评估文件数**: {s.get('files_evaluated', 0)}")
        lines.append("")
        
        # 详细分析
        lines.append("---")
        lines.append("")
        lines.append("## 详细分析")
        lines.append("")
        
        for fe in evaluation.file_evaluations:
            lines.append(f"### 📄 {fe.c_file} → {fe.rust_file}")
            lines.append("")
            
            if fe.error:
                lines.append(f"**错误**: {fe.error}")
                lines.append("")
                continue
            
            lines.append(f"**得分**: {fe.score}")
            lines.append("")
            
            # 按状态分组
            full_matches = []
            adapted = []
            partial_matches = []
            missing = []
            
            feature_dict = {fp.id: fp for fp in fe.c_features}
            
            for mr in fe.matching_results:
                fp = feature_dict.get(mr.c_feature_id)
                if not fp:
                    continue
                
                item = (mr, fp)
                status = mr.match_status.upper()
                if status == "FULL_MATCH":
                    full_matches.append(item)
                elif status == "ADAPTED":
                    adapted.append(item)
                elif status == "PARTIAL_MATCH":
                    partial_matches.append(item)
                else:
                    missing.append(item)
            
            if full_matches:
                lines.append("#### ✅ 完全匹配")
                for mr, fp in full_matches:
                    lines.append(f"- **{fp.id}**: {fp.name} ({fp.type}) - {fp.description[:50]}...")
                lines.append("")
            
            if adapted:
                lines.append("#### 🔄 Rust 适配")
                for mr, fp in adapted:
                    lines.append(f"- **{fp.id}**: {fp.name}")
                    if mr.notes:
                        lines.append(f"  - 说明: {mr.notes}")
                lines.append("")
            
            if partial_matches:
                lines.append("#### ⚠️ 部分匹配")
                for mr, fp in partial_matches:
                    lines.append(f"- **{fp.id}**: {fp.name}")
                    for diff in mr.differences[:2]:
                        lines.append(f"  - {diff.get('type', '差异')}: {diff.get('reason', '')}")
                lines.append("")
            
            if missing:
                lines.append("#### ❌ 缺失")
                for mr, fp in missing:
                    lines.append(f"- **{fp.id}**: {fp.name} - {fp.description[:50]}...")
                lines.append("")
        
        # 改进建议
        if evaluation.recommendations:
            lines.append("---")
            lines.append("")
            lines.append("## 改进建议")
            lines.append("")
            for i, rec in enumerate(evaluation.recommendations, 1):
                lines.append(f"{i}. {rec}")
            lines.append("")
    
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))
    
    logger.info(f"📝 Markdown 报告已保存: {output_path}")


# ============================================================
# 命令行入口
# ============================================================

def main():
    parser = argparse.ArgumentParser(
        description="C2Rust 语义等价性评估器",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 评估指定项目
  python semantic_equivalence_evaluator.py --projects qos,mini --outputs-dir ./translation_outputs

  # 评估所有项目
  python semantic_equivalence_evaluator.py --all --outputs-dir ./translation_outputs

  # 限制每个项目评估的文件数
  python semantic_equivalence_evaluator.py --projects qos --max-files 5 --outputs-dir ./translation_outputs
        """
    )
    
    parser.add_argument(
        "--projects",
        type=str,
        help="要评估的项目名称，逗号分隔"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="评估所有项目"
    )
    parser.add_argument(
        "--outputs-dir",
        type=str,
        default="./translation_outputs",
        help="翻译输出目录 (默认: ./translation_outputs)"
    )
    parser.add_argument(
        "--report-dir",
        type=str,
        default="./evaluation_reports",
        help="报告输出目录 (默认: ./evaluation_reports)"
    )
    parser.add_argument(
        "--max-files",
        type=int,
        default=0,
        help="每个项目最大评估文件数 (0=全部)"
    )
    parser.add_argument(
        "--vllm-url",
        type=str,
        default=None,
        help="vLLM 服务地址（默认从 generation.py 获取）"
    )
    parser.add_argument(
        "--model",
        type=str,
        default=None,
        help="模型名称（默认从 generation.py 获取）"
    )
    parser.add_argument(
        "--parallel",
        type=int,
        default=4,
        help="并行评估的项目数 (默认: 4，建议不超过 vLLM 最大并发数)"
    )
    
    args = parser.parse_args()
    
    # 验证参数
    if not args.projects and not args.all:
        parser.error("必须指定 --projects 或 --all")
    
    outputs_dir = Path(args.outputs_dir)
    if not outputs_dir.exists():
        logger.error(f"翻译输出目录不存在: {outputs_dir}")
        sys.exit(1)

    # 兼容新的 run 目录布局：
    # - translation_outputs/<run_dir>/
    #     - intermediate/<project>/workspace
    #     - results/evaluation_reports/...
    #
    # 此时用户可能传入的是 <run_dir>，而不是 intermediate/。
    projects_root = outputs_dir
    if (outputs_dir / "intermediate").is_dir():
        projects_root = outputs_dir / "intermediate"
    
    # 创建报告目录
    report_dir = Path(args.report_dir)
    report_dir.mkdir(parents=True, exist_ok=True)
    
    # 收集要评估的项目
    projects_to_evaluate = []
    
    if args.all:
        # 扫描所有项目目录
        for item in projects_root.iterdir():
            if not item.is_dir():
                continue
            workspace_dir = item / "workspace"
            if not workspace_dir.exists():
                continue

            # 兼容两种目录布局：
            # 1) 旧布局: translation_outputs/<project>_YYYYMMDD/workspace
            # 2) 新布局: translation_outputs/<run_dir>/<project>/workspace（项目目录名不带日期）
            dir_name = item.name
            project_name = dir_name
            parts = dir_name.rsplit("_", 1)
            if len(parts) == 2 and len(parts[1]) == 8 and parts[1].isdigit():
                project_name = parts[0]

            projects_to_evaluate.append((project_name, str(workspace_dir)))
    else:
        # 指定的项目
        for project_name in args.projects.split(","):
            project_name = project_name.strip()
            if not project_name:
                continue
            
            # 查找项目目录（兼容：精确目录名 or 带日期后缀）
            candidates = []
            for item in projects_root.iterdir():
                if not item.is_dir():
                    continue
                if item.name != project_name and not item.name.startswith(f"{project_name}_"):
                    continue
                workspace_dir = item / "workspace"
                if workspace_dir.exists():
                    candidates.append(item)

            if not candidates:
                logger.warning(f"⚠ 未找到项目: {project_name}")
                continue

            # 多个候选时选择最新的（按 mtime）
            try:
                candidates.sort(key=lambda p: p.stat().st_mtime, reverse=True)
            except Exception:
                pass
            best = candidates[0]
            projects_to_evaluate.append((project_name, str(best / "workspace")))
    
    if not projects_to_evaluate:
        logger.error("没有可评估的项目")
        sys.exit(1)
    
    logger.info(f"准备评估 {len(projects_to_evaluate)} 个项目")
    
    # 初始化评估器
    try:
        evaluator = SemanticEquivalenceEvaluator(
            vllm_url=args.vllm_url,
            model_name=args.model
        )
    except Exception as e:
        logger.error(f"评估器初始化失败: {e}")
        sys.exit(1)
    
    # 评估项目
    all_evaluations = []
    
    total_projects = len(projects_to_evaluate)
    completed_count = 0
    
    def evaluate_with_progress(project_name, workspace_dir, idx):
        """带进度显示的评估"""
        nonlocal completed_count
        try:
            logger.info(f"\n{'='*60}")
            logger.info(f"📊 评估项目 [{idx+1}/{total_projects}]: {project_name}")
            logger.info(f"{'='*60}")
            evaluation = evaluator.evaluate_project(
                project_name,
                workspace_dir,
                args.max_files
            )
            completed_count += 1
            logger.info(f"✓ 进度: {completed_count}/{total_projects} ({100*completed_count/total_projects:.1f}%)")
            return evaluation
        except Exception as e:
            completed_count += 1
            logger.error(f"✗ 项目 {project_name} 评估失败: {e}")
            logger.info(f"✓ 进度: {completed_count}/{total_projects} ({100*completed_count/total_projects:.1f}%)")
            return None
    
    if args.parallel > 1:
        # 并行评估
        logger.info(f"🚀 启用并行评估 (workers={args.parallel})")
        with ThreadPoolExecutor(max_workers=args.parallel) as executor:
            futures = {}
            for idx, (project_name, workspace_dir) in enumerate(projects_to_evaluate):
                future = executor.submit(
                    evaluate_with_progress,
                    project_name,
                    workspace_dir,
                    idx
                )
                futures[future] = project_name
            
            for future in as_completed(futures):
                project_name = futures[future]
                try:
                    evaluation = future.result()
                    if evaluation:
                        all_evaluations.append(evaluation)
                except Exception as e:
                    logger.error(f"项目 {project_name} 评估异常: {e}")
    else:
        # 串行评估
        logger.info(f"🚀 串行评估模式")
        for idx, (project_name, workspace_dir) in enumerate(projects_to_evaluate):
            evaluation = evaluate_with_progress(project_name, workspace_dir, idx)
            if evaluation:
                all_evaluations.append(evaluation)
    
    # 生成报告
    logger.info(f"\n{'='*60}")
    logger.info("📊 生成评估报告")
    logger.info(f"{'='*60}")
    
    for evaluation in all_evaluations:
        # JSON 报告
        json_path = report_dir / f"{evaluation.project_name}_evaluation.json"
        generate_json_report(evaluation, str(json_path))
        
        # Markdown 报告
        md_path = report_dir / f"{evaluation.project_name}_evaluation.md"
        generate_markdown_report(evaluation, str(md_path))
    
    # 生成汇总报告
    summary_path = report_dir / "evaluation_summary.json"
    summary_data = {
        "evaluation_time": datetime.now().isoformat(),
        "total_projects": len(all_evaluations),
        "projects": [
            {
                "name": e.project_name,
                "score": e.overall_score,
                "summary": e.summary,
                "error": e.error
            }
            for e in all_evaluations
        ],
        "average_score": round(
            sum(e.overall_score for e in all_evaluations if not e.error) /
            max(1, len([e for e in all_evaluations if not e.error])),
            2
        )
    }
    
    with open(summary_path, 'w', encoding='utf-8') as f:
        json.dump(summary_data, f, ensure_ascii=False, indent=2)
    
    logger.info(f"📝 汇总报告已保存: {summary_path}")
    
    # 打印汇总
    logger.info(f"\n{'='*60}")
    logger.info("📈 评估汇总")
    logger.info(f"{'='*60}")
    logger.info(f"评估项目数: {len(all_evaluations)}")
    logger.info(f"平均得分: {summary_data['average_score']}")
    logger.info("")
    
    for e in sorted(all_evaluations, key=lambda x: x.overall_score, reverse=True):
        status = "✓" if not e.error else "✗"
        logger.info(f"  {status} {e.project_name}: {e.overall_score}")
    
    logger.info(f"\n报告目录: {report_dir}")


if __name__ == "__main__":
    main()
