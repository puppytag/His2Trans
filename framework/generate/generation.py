from openai import OpenAI
import concurrent.futures
import sys
import json
import re
import os
import multiprocessing
import time
from pathlib import Path
from threading import Semaphore
from llm_global_concurrency import llm_global_slot
# from tenacity import retry, stop_after_attempt, wait_fixed

# --- LLM 并发控制 ---
# 限制同时发送到 LLM 服务的请求数，外部 API 和 vLLM 模式共用。
VLLM_CONCURRENT_LIMIT = int(os.environ.get("LLM_CONCURRENT_LIMIT", os.environ.get("VLLM_CONCURRENT_LIMIT", "120")))
_vllm_semaphore = Semaphore(VLLM_CONCURRENT_LIMIT)

def extract_code(content):


    # pattern = r'\[translated result\](.*?)'
    # translated_result = re.findall(pattern, content, re.DOTALL)[0].strip()
    translated_result = content
    # translated_result = content.split("[translated result]")[1]
    # print("---")
    # print(translated_result)
    # translated_result = translated_result.split("[#END]")[0]
    # print("---")
    # print(translated_result)

    translated_code = None

    patterns = [r'```rust(.*?)```',r'```Rust(.*?)```']

    for pattern in patterns:
        if translated_code == None:
            try:
                translated_code = re.findall(pattern, translated_result, re.DOTALL)[0].strip()

            except:
                translated_code = None
        else:
            break

    if translated_code == None:
        translated_code = translated_result


    return translated_code

# def extract_code(content):


#     pattern = r'\[translated result\](.*?)\[#END\]'
#     translated_result = re.findall(pattern, content, re.DOTALL)[0].strip()

#     translated_code = None

#     patterns = [r'```rust(.*?)```',r'```Rust(.*?)```']

#     for pattern in patterns:
#         if not translated_code:
#             try:
#                 translated_code = re.findall(pattern, translated_result, re.DOTALL)[0].strip()

#             except:
#                 translated_code = None
#         else:
#             break

#     if not translated_code:
#         translated_code = translated_result


#     return translated_code

# --- vLLM 配置 ---
# 支持环境变量配置，默认使用 vLLM。
#
# ⚠️ 重要：vLLM 的 OpenAI 兼容接口里 `model=` 必须使用 vLLM 暴露的 "served model name"
# 而不是 HuggingFace 的仓库名/本地路径。
# 例如 vLLM 可能以 served_model_name=["qwen3_coder"] 启动，则这里必须传 "qwen3_coder"。
VLLM_BASE_URL = os.environ.get("VLLM_BASE_URL", "http://localhost:8000/v1")
VLLM_API_KEY = os.environ.get("VLLM_API_KEY", "EMPTY")
VLLM_MODEL_NAME = (
    os.environ.get("VLLM_MODEL_NAME")
    or os.environ.get("LLM_NAME")
    or "qwen3_coder"
)
VLLM_NUM_WORKERS = int(os.environ.get("VLLM_NUM_WORKERS", "48"))
VLLM_REQUEST_TIMEOUT = float(os.environ.get("VLLM_REQUEST_TIMEOUT", os.environ.get("VLLM_TIMEOUT", "600.0")))
VLLM_MAX_RETRIES = int(os.environ.get("VLLM_MAX_RETRIES", "3"))
VLLM_RETRY_BACKOFF_SEC = float(os.environ.get("VLLM_RETRY_BACKOFF_SEC", "2.0"))
VLLM_RETRY_BACKOFF_MAX_SEC = float(os.environ.get("VLLM_RETRY_BACKOFF_MAX_SEC", "30.0"))


def _load_local_env_file_once():
    """按 C++2Rust 方式加载本地 LLM 配置文件，不覆盖已有环境变量。"""
    if getattr(_load_local_env_file_once, "_loaded", False):
        return
    candidates = []
    configured = os.environ.get("C2RUST_LOCAL_ENV_FILE", "").strip()
    if configured:
        candidates.append(Path(configured).expanduser())
    repo_root = Path(__file__).resolve().parents[1]
    candidates.append(repo_root / "output" / "mini-swe-agent-config" / ".env")
    for env_path in candidates:
        if not env_path.is_file():
            continue
        for raw_line in env_path.read_text(encoding="utf-8", errors="replace").splitlines():
            line = raw_line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, value = line.split("=", 1)
            key = key.strip()
            value = value.strip()
            if (value.startswith('"') and value.endswith('"')) or (value.startswith("'") and value.endswith("'")):
                value = value[1:-1]
            if key and key not in os.environ:
                os.environ[key] = value
        break
    _load_local_env_file_once._loaded = True


_load_local_env_file_once()

# --- 外部 API 配置 (当 USE_VLLM=false 时使用) ---
DEEPSEEK_API_KEY = os.environ.get("DEEPSEEK_API_KEY", "").strip()
DEEPSEEK_BASE_URL = os.environ.get("DEEPSEEK_BASE_URL", "https://api.deepseek.com").strip()
DEEPSEEK_MODEL = os.environ.get("DEEPSEEK_MODEL", "deepseek-v4-pro").strip()
# 框架统一使用 DeepSeek 最高推理深度；旧环境变量里的 high 不再下传到请求。
DEEPSEEK_REASONING_EFFORT = "max"
DEEPSEEK_THINKING = os.environ.get("DEEPSEEK_THINKING", "enabled").strip()

EXTERNAL_API_KEY = os.environ.get("EXTERNAL_API_KEY") or DEEPSEEK_API_KEY
EXTERNAL_API_BASE_URL = os.environ.get("EXTERNAL_API_BASE_URL") or (
    DEEPSEEK_BASE_URL if DEEPSEEK_API_KEY else "https://api.deepseek.com"
)
EXTERNAL_API_MODEL = os.environ.get("EXTERNAL_API_MODEL") or (
    DEEPSEEK_MODEL if DEEPSEEK_API_KEY else "deepseek-v4-pro"
)
_EXTERNAL_API_MAX_TOKENS_RAW = os.environ.get("EXTERNAL_API_MAX_TOKENS", "").strip()
EXTERNAL_API_MAX_TOKENS = int(_EXTERNAL_API_MAX_TOKENS_RAW) if _EXTERNAL_API_MAX_TOKENS_RAW else None
EXTERNAL_API_TEMPERATURE = float(os.environ.get("EXTERNAL_API_TEMPERATURE", "0.0"))
EXTERNAL_API_TOP_P = float(os.environ.get("EXTERNAL_API_TOP_P", "1.0"))
EXTERNAL_API_MAX_RETRIES = int(os.environ.get("EXTERNAL_API_MAX_RETRIES", "3"))
EXTERNAL_API_RETRY_DELAY = float(os.environ.get("EXTERNAL_API_RETRY_DELAY", "2.0"))
EXTERNAL_API_RETRY_MAX_DELAY = float(os.environ.get("EXTERNAL_API_RETRY_MAX_DELAY", "30.0"))
EXTERNAL_API_TIMEOUT = float(os.environ.get("EXTERNAL_API_TIMEOUT", "600.0"))


def _is_deepseek_v4_model(model_name: str, base_url: str) -> bool:
    """判断外部 API 请求是否应携带 DeepSeek V4 推理参数。"""
    name = (model_name or "").lower()
    url = (base_url or "").lower()
    if any(local in url for local in ("localhost", "127.0.0.1", "0.0.0.0")):
        return False
    return "deepseek" in name and "v4" in name


def deepseek_v4_request_kwargs(model_name: str, base_url: str) -> dict[str, object]:
    """返回 DeepSeek V4 OpenAI-compatible 请求需要统一携带的推理参数。"""
    if not _is_deepseek_v4_model(model_name, base_url):
        return {}
    kwargs: dict[str, object] = {}
    if DEEPSEEK_REASONING_EFFORT:
        kwargs["reasoning_effort"] = DEEPSEEK_REASONING_EFFORT
    if DEEPSEEK_THINKING:
        kwargs["extra_body"] = {
            "thinking": {"type": DEEPSEEK_THINKING}
        }
    return kwargs


def _chat_completion_with_global_limit(client, *, base_url: str, model_name: str, label: str, **kwargs):
    """调用 OpenAI-compatible chat completion，并限制外部 LLM 跨进程全局并发。"""
    with llm_global_slot(base_url=base_url, model=model_name, label=label):
        return client.chat.completions.create(**kwargs)


def _format_external_api_client_init_message(base_url: str, model_name: str, api_key: str) -> str:
    """生成外部 API 初始化日志，避免把缺 key 误报为真实可调用。"""
    if api_key:
        return f"外部 API 客户端初始化成功: {base_url}, 模型: {model_name}"
    return (
        f"外部 API 客户端已构造但缺少密钥: {base_url}, 模型: {model_name}; "
        "真实调用会失败，请设置 EXTERNAL_API_KEY 或 DEEPSEEK_API_KEY"
    )


def _is_timeout_error(e: Exception) -> bool:
    msg = str(e).lower()
    return ("timed out" in msg) or ("timeout" in msg)


def _usage_raw_dict(usage):
    """把 provider usage 对象尽量转成普通 dict，便于保留原始字段。"""
    if usage is None:
        return {}
    if isinstance(usage, dict):
        return usage
    for method in ("model_dump", "dict"):
        fn = getattr(usage, method, None)
        if callable(fn):
            try:
                value = fn()
                if isinstance(value, dict):
                    return value
            except Exception:
                pass
    try:
        return dict(vars(usage))
    except Exception:
        return {}


def _usage_value(usage, raw, key, default=0):
    """从 dict 或对象属性读取 usage 字段。"""
    if isinstance(raw, dict) and key in raw:
        return raw.get(key)
    if isinstance(usage, dict):
        return usage.get(key, default)
    return getattr(usage, key, default)


def _to_int(value) -> int:
    """安全转换 token 数。"""
    try:
        return int(value or 0)
    except (TypeError, ValueError):
        return 0


def normalize_usage(usage) -> dict:
    """归一化普通 token 与远端 prompt cache 命中字段。"""
    raw = _usage_raw_dict(usage)
    prompt_tokens = _to_int(_usage_value(usage, raw, "prompt_tokens"))
    completion_tokens = _to_int(_usage_value(usage, raw, "completion_tokens"))
    total_tokens = _to_int(_usage_value(usage, raw, "total_tokens"))

    prompt_details = _usage_value(usage, raw, "prompt_tokens_details", {})
    if prompt_details is None:
        prompt_details = {}
    if not isinstance(prompt_details, dict):
        prompt_details = _usage_raw_dict(prompt_details)

    openai_cached = _to_int(prompt_details.get("cached_tokens", 0) if isinstance(prompt_details, dict) else 0)
    deepseek_hit = _to_int(_usage_value(usage, raw, "prompt_cache_hit_tokens"))
    deepseek_miss = _to_int(_usage_value(usage, raw, "prompt_cache_miss_tokens"))
    generic_hit = _to_int(_usage_value(usage, raw, "cache_hit_tokens"))
    generic_miss = _to_int(_usage_value(usage, raw, "cache_miss_tokens"))

    cache_hit_tokens = deepseek_hit or generic_hit or openai_cached
    cache_miss_tokens = deepseek_miss or generic_miss
    if not cache_miss_tokens and prompt_tokens and cache_hit_tokens:
        cache_miss_tokens = max(0, prompt_tokens - cache_hit_tokens)

    return {
        "prompt_tokens": prompt_tokens,
        "completion_tokens": completion_tokens,
        "total_tokens": total_tokens,
        "cached_tokens": cache_hit_tokens,
        "cache_hit_tokens": cache_hit_tokens,
        "cache_miss_tokens": cache_miss_tokens,
        "prompt_cache_hit_tokens": deepseek_hit,
        "prompt_cache_miss_tokens": deepseek_miss,
        "prompt_tokens_details": prompt_details,
        "completion_tokens_details": _usage_raw_dict(_usage_value(usage, raw, "completion_tokens_details", {})),
        "provider_usage_raw": raw,
    }

# 是否使用 vLLM（默认启用）
USE_VLLM = os.environ.get("USE_VLLM", "false").lower() in ("true", "1", "yes")

# 如果使用 vLLM，设置 multiprocessing start method
if USE_VLLM:
    try:
        multiprocessing.set_start_method("spawn", force=True)
    except RuntimeError:
        pass  # 如果已经设置过，忽略错误

# vLLM 采样参数
# 代码翻译任务推荐使用低温度以获得更确定性的输出
VLLM_SAMPLING_PARAMS = {
    "temperature": float(os.environ.get("VLLM_TEMPERATURE", "0.0")),  # 0 = 最确定性输出，适合代码翻译
    "top_p": float(os.environ.get("VLLM_TOP_P", "1.0")),              # 1.0 = 不使用 nucleus sampling（配合 temperature=0）
    "top_k": int(os.environ.get("VLLM_TOP_K", "-1")),                 # -1 = 禁用 top_k（配合 temperature=0）
    "repetition_penalty": float(os.environ.get("VLLM_REPETITION_PENALTY", "1.0")),  # 1.0 = 不惩罚重复（代码中重复是正常的）
    # NOTE: keep this reasonably high to avoid truncation in long function bodies.
    "max_tokens": int(os.environ.get("VLLM_MAX_TOKENS", "16384"))
}

# Worker 全局变量（用于多进程）
WORKER_CLIENT = None

def init_vllm_worker():
    """初始化 vLLM worker（用于多进程）"""
    global WORKER_CLIENT
    worker_id = multiprocessing.current_process()._identity[0] - 1 if hasattr(multiprocessing.current_process(), '_identity') else 0
    try:
        WORKER_CLIENT = OpenAI(
            base_url=VLLM_BASE_URL,
            api_key=VLLM_API_KEY,
            timeout=VLLM_REQUEST_TIMEOUT
        )
        # 测试连接
        WORKER_CLIENT.models.list()
        print(f"[Worker {worker_id}]: vLLM 客户端初始化成功，已连接到 {VLLM_BASE_URL}")
    except Exception as e:
        print(f"[Worker {worker_id}]: vLLM 客户端初始化失败: {e}", file=sys.stderr)
        print(f"请确保 vLLM 服务器正在 {VLLM_BASE_URL} 运行", file=sys.stderr)
        sys.exit(1)

class Claude():
    """
    兼容旧版本的 Claude 类
    现在支持 vLLM 和外部 API 两种模式
    """
    def __init__(
        self,
        api_keys : list = None,
    ):
        self.use_vllm = USE_VLLM

        if self.use_vllm:
            # 使用 vLLM
            self.model_name = VLLM_MODEL_NAME
            self.base_url = VLLM_BASE_URL
            self.api_key = VLLM_API_KEY
            # 创建单个客户端用于单线程调用
            try:
                self.client = OpenAI(
                    base_url=self.base_url,
                    api_key=self.api_key,
                    timeout=VLLM_REQUEST_TIMEOUT
                )
            except Exception as e:
                print(f"警告: vLLM 客户端初始化失败: {e}", file=sys.stderr)
                print(f"回退到外部 API 模式", file=sys.stderr)
                self.use_vllm = False

        if not self.use_vllm:
            # 外部 API 模式
            self.model_name = EXTERNAL_API_MODEL
            self.base_url = EXTERNAL_API_BASE_URL
            self.api_key = EXTERNAL_API_KEY
            self.temperature = EXTERNAL_API_TEMPERATURE
            self.top_p = globals().get('EXTERNAL_API_TOP_P', None)
            self.max_tokens = EXTERNAL_API_MAX_TOKENS
            self.max_retries = EXTERNAL_API_MAX_RETRIES
            self.retry_delay = EXTERNAL_API_RETRY_DELAY
            self.retry_max_delay = EXTERNAL_API_RETRY_MAX_DELAY
            self.timeout = EXTERNAL_API_TIMEOUT
            self.is_deepseek_v4 = _is_deepseek_v4_model(self.model_name, self.base_url)
            self.deepseek_reasoning_effort = DEEPSEEK_REASONING_EFFORT
            self.deepseek_thinking = DEEPSEEK_THINKING
            try:
                self.client = OpenAI(
                    api_key=self.api_key,
                    base_url=self.base_url,
                    timeout=self.timeout
                )
                print(_format_external_api_client_init_message(self.base_url, self.model_name, self.api_key))
            except Exception as e:
                print(f"外部 API 客户端初始化失败: {e}", file=sys.stderr)
                raise


    def generation_slice_in_parallel(self, generator):
        """遗留方法：并行生成（已更新为使用单客户端）"""
        max_workers = 8  # 外部 API 限制并发数
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            future_to_path = {
                executor.submit(self.generation, [{"role": "user", "content": message}]): (path, message, query_func, cnt)
                for i, (path, message, query_func, cnt) in enumerate(generator)
            }

            for future in concurrent.futures.as_completed(future_to_path):
                path, message, query_func, cnt = future_to_path[future]
                response = future.result()
                translated_code = extract_code(response)
                with open(path, 'r', encoding='utf-8', errors='ignore') as file:
                    content = json.load(file)
                content.append(translated_code)
                with open(path, 'w', encoding='utf-8', errors='ignore') as file:
                    json.dump(content, file, indent=2)

                response_path = path.replace(".json", f"_{cnt}.txt")
                try:
                    with open(response_path, 'w', encoding='utf-8', errors='ignore') as output_file:
                        output_file.write(f"<message>\n{message}\n</message>\n")
                        output_file.write(f"<function>\n{query_func}\n</function>\n<response>\n{response}</response>")
                    print(f"Successfully processed: {response_path}")
                except Exception as e:
                    print(f"Error processing {response_path}: {e}")

    def generation_in_parallel(self, generator):
        """遗留方法：并行生成（已更新为使用单客户端）"""
        max_workers = 8  # 外部 API 限制并发数
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            future_to_path = {
                executor.submit(self.generation, [{"role": "user", "content": message}]): (path, message, query_func, original_function_name)
                for i, (path, message, query_func, original_function_name) in enumerate(generator)
            }

            for future in concurrent.futures.as_completed(future_to_path):
                path, message, query_func, original_function_name = future_to_path[future]
                response = future.result()

                translated_code = extract_code(response)
                new_function_name = translated_code.split("fn ")[1].split("{")[0].strip()
                translated_code = translated_code.replace(new_function_name, original_function_name, 1)

                with open(path, 'w', encoding='utf-8', errors='ignore') as output_file:
                    output_file.write(f"<message>\n{message}\n</message>\n")
                    output_file.write(f"<response>\n{response}\n</response>\n")
                    output_file.write(f"<function>\n{query_func}\n</function>\n<translated function>\n{translated_code}</translated function>")
                print(f"Successfully processed: {path}")

    # def generation(self, content, client, temperature=0):
    #     # print(content)
    #     response = client.chat.completions.create(
    #         model=self.model_name,
    #         messages=[
    #             {
    #                 "role": "user",
    #                 "content": content
    #             }
    #         ],
    #         temperature=temperature
    #     )
    #     # print(response)
    #     if response.choices[0].message.content:
    #         # print(response.choices[0].message.content)
    #         return response.choices[0].message.content
    #     else:
    #         raise ValueError("Empty response from API")

    # @retry(stop=stop_after_attempt(5), wait=wait_fixed(10))
    def generation(self, messages, temperature=0, return_usage=False):
        """
        生成翻译结果

        Args:
            messages: 消息列表（OpenAI 格式）
            temperature: 温度参数（vLLM 模式下使用配置中的值）
            return_usage: 是否返回 token 使用信息

        Returns:
            如果 return_usage=True，返回 {"content": ..., "usage": {...}}
            否则返回字符串内容
        """
        if self.use_vllm:
            # 使用 vLLM - 通过 Semaphore 控制并发
            _vllm_semaphore.acquire()
            try:
                # 准备 API 参数
                api_params = {
                    "temperature": VLLM_SAMPLING_PARAMS.get("temperature"),
                    "top_p": VLLM_SAMPLING_PARAMS.get("top_p"),
                    "max_tokens": VLLM_SAMPLING_PARAMS.get("max_tokens")
                }
                extra_body_params = {
                    "top_k": VLLM_SAMPLING_PARAMS.get("top_k"),
                    "repetition_penalty": VLLM_SAMPLING_PARAMS.get("repetition_penalty")
                }

                last_err = None
                for attempt in range(1, max(1, VLLM_MAX_RETRIES) + 1):
                    try:
                        response = _chat_completion_with_global_limit(
                            self.client,
                            base_url=self.base_url,
                            model_name=self.model_name,
                            label="generation_vllm",
                            model=self.model_name,
                            messages=messages,
                            stop=["<|im_end|>"],
                            **api_params,
                            extra_body=extra_body_params
                        )

                        if response.choices and response.choices[0].message.content:
                            if return_usage:
                                usage = normalize_usage(response.usage)
                                return {
                                    "content": response.choices[0].message.content,
                                    "usage": usage
                                }
                            return response.choices[0].message.content

                        raise ValueError("Empty response from vLLM")
                    except Exception as e:
                        last_err = e
                        if attempt < max(1, VLLM_MAX_RETRIES) and _is_timeout_error(e):
                            backoff = min(VLLM_RETRY_BACKOFF_SEC * (2 ** (attempt - 1)), VLLM_RETRY_BACKOFF_MAX_SEC)
                            print(
                                f"vLLM 调用超时，{backoff:.1f}s 后重试 (attempt {attempt}/{VLLM_MAX_RETRIES}): {e}",
                                file=sys.stderr,
                            )
                            time.sleep(backoff)
                            continue
                        print(f"vLLM 调用失败: {e}", file=sys.stderr)
                        raise

                # 理论上不会到这里
                raise last_err
            finally:
                _vllm_semaphore.release()
        else:
            # 外部 API 模式（带重试机制）
            if not getattr(self, "api_key", ""):
                raise ValueError("缺少外部 API 密钥：请设置 EXTERNAL_API_KEY 或 DEEPSEEK_API_KEY")
            last_err = None
            for attempt in range(1, self.max_retries + 1):
                try:
                    kwargs = {
                        "model": self.model_name,
                        "messages": messages,
                        "stream": False
                    }
                    if self.max_tokens is not None:
                        kwargs["max_tokens"] = self.max_tokens
                    # 只有当 temperature 不为 None 时才加入参数
                    # (注意：如果 EXTERNAL_API_TEMPERATURE 定义为 0.0，if 0.0 可能会被判为 False，
                    # 建议显式判断 is not None)
                    if not getattr(self, "is_deepseek_v4", False) and getattr(self, 'temperature', None) is not None:
                        kwargs["temperature"] = self.temperature

                    # 只有当 top_p 有值时才加入参数
                    if not getattr(self, "is_deepseek_v4", False) and self.top_p is not None:
                        kwargs["top_p"] = self.top_p

                    kwargs.update(deepseek_v4_request_kwargs(self.model_name, getattr(self, "base_url", EXTERNAL_API_BASE_URL)))

                    # 使用 **kwargs 解包参数发送请求
                    _vllm_semaphore.acquire()
                    try:
                        response = _chat_completion_with_global_limit(
                            self.client,
                            base_url=self.base_url,
                            model_name=self.model_name,
                            label="generation_external",
                            **kwargs,
                        )
                    finally:
                        _vllm_semaphore.release()

                    if response and response.choices and len(response.choices) > 0:
                        if response.choices[0].message and response.choices[0].message.content:
                            content = response.choices[0].message.content.strip()
                            if return_usage:
                                usage = normalize_usage(response.usage)
                                return {
                                    "content": content,
                                    "usage": usage
                                }
                            return content

                    # 空响应，重试
                    if attempt < self.max_retries:
                        backoff = min(self.retry_delay * (2 ** (attempt - 1)), self.retry_max_delay)
                        print(f"外部 API 返回空响应，{backoff:.1f}s 后重试 (attempt {attempt}/{self.max_retries})", file=sys.stderr)
                        time.sleep(backoff)
                        continue
                    raise ValueError("Empty response from external API")

                except Exception as e:
                    last_err = e
                    error_str = str(e).lower()
                    # 限流错误或超时：重试
                    if any(keyword in error_str for keyword in ['rate', 'limit', '429', 'too many', 'timeout', 'timed out']):
                        if attempt < self.max_retries:
                            backoff = min(self.retry_delay * (2 ** (attempt - 1)), self.retry_max_delay)
                            print(f"外部 API 调用失败 ({e})，{backoff:.1f}s 后重试 (attempt {attempt}/{self.max_retries})", file=sys.stderr)
                            time.sleep(backoff)
                            continue
                    print(f"外部 API 调用失败: {e}", file=sys.stderr)
                    raise

            # 所有重试都失败
            if last_err:
                raise last_err
            raise ValueError("External API call failed after all retries")


# 创建全局实例
claude = Claude()

def generation(message, return_usage=False):
    """
    生成翻译结果（单线程版本）

    Args:
        message: 消息列表（OpenAI 格式）或单个消息字符串
        return_usage: 是否返回 token 使用信息

    Returns:
        如果 return_usage=True，返回 {"content": ..., "usage": {...}}
        否则返回字符串内容
    """
    # 如果 message 是字符串，转换为消息格式（向后兼容）
    if isinstance(message, str):
        messages = [{"role": "user", "content": message}]
    elif isinstance(message, list):
        messages = message
    else:
        raise ValueError("message 必须是字符串或消息列表")

    return claude.generation(messages, return_usage=return_usage)

def generation_in_parallel_vllm(tasks, max_workers=None):
    """
    使用 vLLM 多进程并行生成（新函数）

    Args:
        tasks: 任务列表，每个任务是一个字典，包含 "messages" 键
        max_workers: 最大工作进程数，默认使用 VLLM_NUM_WORKERS

    Returns:
        结果列表，与 tasks 顺序对应
    """
    if not USE_VLLM:
        raise ValueError("generation_in_parallel_vllm 需要启用 vLLM (USE_VLLM=true)")

    if max_workers is None:
        max_workers = VLLM_NUM_WORKERS

    def process_single_task(task_data):
        """处理单个任务（用于多进程）"""
        global WORKER_CLIENT
        if WORKER_CLIENT is None:
            init_vllm_worker()

        try:
            messages = task_data.get("messages", [])
            if not messages:
                return {"error": "No messages in task", "task": task_data}

            # 准备 API 参数
            api_params = {
                "temperature": VLLM_SAMPLING_PARAMS.get("temperature"),
                "top_p": VLLM_SAMPLING_PARAMS.get("top_p"),
                "max_tokens": VLLM_SAMPLING_PARAMS.get("max_tokens")
            }
            extra_body_params = {
                "top_k": VLLM_SAMPLING_PARAMS.get("top_k"),
                "repetition_penalty": VLLM_SAMPLING_PARAMS.get("repetition_penalty")
            }

            response = _chat_completion_with_global_limit(
                WORKER_CLIENT,
                base_url=VLLM_BASE_URL,
                model_name=VLLM_MODEL_NAME,
                label="generation_vllm_worker",
                model=VLLM_MODEL_NAME,
                messages=messages,
                stop=["<|im_end|>"],
                **api_params,
                extra_body=extra_body_params
            )

            if response.choices[0].message.content:
                result = {
                    "content": response.choices[0].message.content,
                    "task": task_data
                }
                if response.usage:
                    result["usage"] = normalize_usage(response.usage)
                return result
            else:
                return {"error": "Empty response", "task": task_data}
        except Exception as e:
            return {"error": str(e), "task": task_data}

    # 使用多进程池
    with multiprocessing.Pool(processes=max_workers, initializer=init_vllm_worker) as pool:
        results = pool.map(process_single_task, tasks)

    return results

def generation_in_parallel(message):
    """
    并行生成（向后兼容，保留旧接口）
    如果启用 vLLM，使用多进程；否则使用传统方式
    """
    if USE_VLLM:
        # 注意：这个函数原本接受 generator，但为了简化，暂时不支持
        # 如果需要，可以后续扩展
        raise NotImplementedError("generation_in_parallel 在 vLLM 模式下需要使用 generation_in_parallel_vllm")
    else:
        return claude.generation_in_parallel(message)
