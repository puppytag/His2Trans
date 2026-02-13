#!/usr/bin/env python3
"""
分层骨架构建器 (Layered Skeleton Builder)

基于论文方法实现的 C++ 到 Rust 骨架翻译：
- Rustine: 预处理阶段（宏展开）
- LLMigrate/EvoC2Rust: 使用 bindgen 生成类型骨架
- PTRMAPPER: 全局变量上下文分析
- EvoC2Rust: 增量式骨架构建

核心理念：
- 阶段 A (Truth Layer): 使用 bindgen 生成绝对正确的类型定义
- 阶段 B (State Layer): 使用 tree-sitter 精确提取全局/静态变量
- 阶段 C (Logic Skeleton): 生成仅包含签名的 unimplemented!() 桩代码
"""

import os
import re
import json
import subprocess
import tempfile
import time
from datetime import datetime
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Tuple, Set, Optional, Any, Sequence
from dataclasses import dataclass, field
import logging
import shutil

from tree_sitter import Language, Parser
import tree_sitter_cpp as tscpp

from workspace_config import safe_module_name

# 导入日志配置
from log_config import ensure_logging_setup, LogPrinter
ensure_logging_setup()

# 设置日志
logger = logging.getLogger(__name__)
log = LogPrinter(__name__)

# ============================================================
# Tier-0: Deterministic primitive typedef mapping
# ============================================================
#
# When bindgen fails (or tree-sitter collects tokens as "types"), common C typedefs
# like INT32/UINT32/CHAR/BOOL are often generated as opaque structs:
#   pub struct INT32 { _private: [u8; 0] }
# This causes massive downstream mismatches (E0308/E0599).
#
# We deterministically map these well-known aliases to Rust primitive/type aliases.
# This mapping is intentionally conservative and aims for "compile-friendly" FFI.
PRIMITIVE_TYPEDEF_ALIASES = {
    # Signed/unsigned integers
    "INT8": "i8",
    "INT16": "i16",
    "INT32": "i32",
    "INT64": "i64",
    "UINT8": "u8",
    "UINT16": "u16",
    "UINT32": "u32",
    "UINT64": "u64",
    "INT": "i32",
    "UINT": "u32",
    "SHORT": "i16",
    "USHORT": "u16",
    "LONG": "i64",
    "ULONG": "u64",
    "LONGLONG": "i64",
    "ULONGLONG": "u64",
    # Characters / bytes
    "CHAR": "core::ffi::c_char",
    "UCHAR": "u8",
    "BYTE": "u8",
    "WORD": "u16",
    "DWORD": "u32",
    # Boolean-like typedefs (C projects commonly treat BOOL as int)
    "BOOL": "i32",
    # Size / pointer sized
    "size_t": "usize",
    "ssize_t": "isize",
    "ptrdiff_t": "isize",
    "intptr_t": "isize",
    "uintptr_t": "usize",
    "UINTPTR": "usize",
    "UINTPTR_T": "usize",
    "VADDR_T": "usize",
    # Common file/offset types (compile-friendly)
    "off_t": "i64",
    "off64_t": "i64",
}

# 导入 compile_commands.json 解析器
try:
    from compile_commands_parser import CompileCommandsParser
    COMPILE_COMMANDS_AVAILABLE = True
except ImportError:
    COMPILE_COMMANDS_AVAILABLE = False
    logger.warning("compile_commands_parser 模块不可用，将跳过 compile_commands.json 支持")

# 导入确定性类型映射器
try:
    from type_mapper import TypeMapper
    TYPE_MAPPER_AVAILABLE = True
except ImportError:
    TYPE_MAPPER_AVAILABLE = False
    logger.warning("type_mapper 模块不可用，将使用旧的类型映射方法")

# 导入类型工具模块
try:
    from type_utils import is_valid_c_identifier, sanitize_parameter_names, extract_base_type
    TYPE_UTILS_AVAILABLE = True
except ImportError:
    TYPE_UTILS_AVAILABLE = False
    logger.warning("type_utils 模块不可用，将跳过类型收集和清洗功能")

# 导入增强预处理模块 (基于 EvoC2Rust 方法)
try:
    from preprocessing import (
        EnhancedPreprocessor,
        ASTHealthReport,
        PreprocessingResult,
        find_function_declarator_recursive,
        check_source_health
    )
    ENHANCED_PREPROCESSING_AVAILABLE = True
except ImportError:
    ENHANCED_PREPROCESSING_AVAILABLE = False
    logger.warning("preprocessing 模块不可用，将使用基本预处理")

# 导入微任务修复器 (AI 原生自愈架构)
try:
    from micro_task_repairer import SelfHealingLoop, MicroTaskRepairer, run_self_healing
    SELF_HEALING_AVAILABLE = True
except ImportError:
    SELF_HEALING_AVAILABLE = False
    logger.warning("micro_task_repairer 模块不可用，将跳过自愈循环")

# 导入 Rust 代码生成器 (安全的代码生成)
try:
    from rust_code_builder import RustCodeBuilder, create_opaque_type, create_const
    RUST_CODE_BUILDER_AVAILABLE = True
except ImportError:
    RUST_CODE_BUILDER_AVAILABLE = False
    logger.warning("rust_code_builder 模块不可用，将使用传统字符串拼接")

# 导入动态宏学习器
try:
    from macro_learner import MacroLearner, get_global_macro_learner, expand_macros, get_gcc_macro_args
    MACRO_LEARNER_AVAILABLE = True
except ImportError:
    MACRO_LEARNER_AVAILABLE = False
    logger.warning("macro_learner 模块不可用，将使用硬编码的宏展开")

# 导入 LLM 驱动的类型映射器
try:
    from llm_type_mapper import LLMTypeMapper, create_llm_type_mapper
    LLM_TYPE_MAPPER_AVAILABLE = True
except ImportError:
    LLM_TYPE_MAPPER_AVAILABLE = False
    logger.warning("llm_type_mapper 模块不可用，将跳过 LLM 辅助类型映射")

# 导入混合构建支持 (C/Rust 链接)
try:
    from hybrid_build import (
        HybridBuildManager, 
        NativeDirectoryManager,
        generate_build_rs,
        generate_extern_declarations,
        extract_compile_flags_from_commands,
        CSourceFile
    )
    HYBRID_BUILD_AVAILABLE = True
except ImportError:
    HYBRID_BUILD_AVAILABLE = False
    logger.warning("hybrid_build 模块不可用，将跳过混合构建支持")

# 导入自适应预定义管理器
try:
    from config.predefines import get_predefine_manager, PredefineManager
    PREDEFINES_AVAILABLE = True
except ImportError:
    PREDEFINES_AVAILABLE = False
    logger.warning("config.predefines 模块不可用，将使用内置的硬编码定义")

# 初始化 Tree-sitter C++ 解析器（兼容新版 bindings：Language(..., name)）
try:
    CPP_LANGUAGE = Language(tscpp.language(), "cpp")
except TypeError:
    CPP_LANGUAGE = Language(tscpp.language())
cpp_parser = Parser()
try:
    cpp_parser.set_language(CPP_LANGUAGE)
except Exception:
    cpp_parser = Parser(CPP_LANGUAGE)


@dataclass
class TypeInfo:
    """类型信息"""
    name: str
    kind: str  # struct, enum, union, typedef
    definition: str  # Rust 定义
    source: str  # 来源: bindgen, llm, placeholder


@dataclass
class VariableInfo:
    """变量信息"""
    name: str
    c_type: str
    rust_type: str
    rust_declaration: str
    is_static: bool
    is_pointer: bool
    is_array: bool
    # `extern` variable declaration (not a definition in this TU).
    # NOTE: Even if it is extern in C, for "pure Rust migration" we may still emit storage.
    is_extern: bool = False
    array_size: Optional[str] = None
    from_function: Optional[str] = None  # 来自哪个函数（用于函数内 static 变量提升）
    # Source file-group (safe module name) that this variable is extracted from.
    # Used to locate the exact preprocessed `.i` for bindgen-truth globals generation.
    origin_file: Optional[str] = None
    # C initialization expression (e.g., "{ 619, 720, 127, ... }" for array initializers)
    # This is used to preserve the original initialization values in the generated Rust code.
    init_value: Optional[str] = None


@dataclass
class ExtractedVariable:
    """提取的变量信息（用于 libclang 提取模式）"""
    name: str
    c_type: str
    rust_type: str
    initial_value: Optional[str] = None
    is_const: bool = False
    is_static: bool = False
    is_extern: bool = False
    is_pointer: bool = False
    from_function: Optional[str] = None  # 来自哪个函数（用于函数内 static 变量提升）
    array_size: Optional[str] = None
    is_array: bool = False  # 是否是数组类型
    
    @property
    def rust_declaration(self) -> str:
        """生成 Rust 变量声明语句"""
        # 确定是 static mut 还是 const
        if self.is_const:
            keyword = "pub const"
            # const 需要初始值，处理 C 风格初始化
            init = self._convert_initial_value(self.initial_value, self.rust_type)
            return f"{keyword} {self.name.upper()}: {self.rust_type} = {init};"
        else:
            keyword = "pub static mut"
            # static mut 使用零初始化或默认值
            if self.rust_type == "*mut c_void" or self.rust_type.startswith("*mut"):
                init = "std::ptr::null_mut()"
            elif self.rust_type.startswith("*const"):
                init = "std::ptr::null()"
            elif self.rust_type in ("i32", "i64", "u32", "u64", "i8", "u8", "i16", "u16", "isize", "usize", "c_int", "c_uint", "c_long", "c_ulong", "c_char", "c_uchar", "c_short", "c_ushort"):
                init = "0"
            elif self.rust_type in ("f32", "f64"):
                init = "0.0"
            elif self.rust_type == "bool":
                init = "false"
            elif self.initial_value and not self._is_c_struct_initializer(self.initial_value):
                # 只有当初始值不是 C 风格结构体初始化时才使用
                init = self._convert_initial_value(self.initial_value, self.rust_type)
            else:
                # 结构体/复杂类型使用 zeroed() 安全初始化
                return f"{keyword} {self.name}: {self.rust_type} = unsafe {{ std::mem::zeroed() }};"
            return f"{keyword} {self.name}: {self.rust_type} = {init};"
    
    def _is_c_struct_initializer(self, value: str) -> bool:
        """检查是否是 C 风格的结构体初始化器（如 {0}, {.member = val}）"""
        if not value:
            return False
        value = value.strip()
        return value.startswith('{') or value.startswith('(')
    
    def _convert_initial_value(self, value: str, rust_type: str) -> str:
        """转换 C 初始值为 Rust 兼容格式"""
        if not value:
            # 根据类型返回默认值
            if rust_type.startswith("*"):
                return "std::ptr::null_mut()" if "mut" in rust_type else "std::ptr::null()"
            elif rust_type in ("i32", "i64", "u32", "u64", "i8", "u8", "i16", "u16", "isize", "usize"):
                return "0"
            elif rust_type in ("f32", "f64"):
                return "0.0"
            elif rust_type == "bool":
                return "false"
            else:
                return "unsafe { std::mem::zeroed() }"
        
        value = value.strip()
        
        # C 风格结构体初始化 → zeroed()
        if value.startswith('{') or value.startswith('('):
            return "unsafe { std::mem::zeroed() }"
        
        # NULL → null_mut()/null()
        if value.upper() in ('NULL', '0', '((VOID*)0)', '((void*)0)'):
            if rust_type.startswith("*"):
                return "std::ptr::null_mut()" if "mut" in rust_type else "std::ptr::null()"
            return "0"
        
        # 布尔值转换
        if value.lower() in ('true', 'false'):
            return value.lower()
        
        # 数字保持不变
        return value


@dataclass
class FunctionSignature:
    """函数签名信息"""
    name: str
    c_signature: str
    rust_signature: str
    return_type: str
    parameters: List[Tuple[str, str]]  # [(param_name, param_type), ...]
    is_static: bool = False  # C static 函数
    is_callback: bool = False  # 用作回调/函数指针


@dataclass
class SkeletonComponents:
    """骨架组件"""
    types: Dict[str, TypeInfo] = field(default_factory=dict)
    variables: Dict[str, VariableInfo] = field(default_factory=dict)
    functions: Dict[str, FunctionSignature] = field(default_factory=dict)
    extern_c_declarations: List[str] = field(default_factory=list)
    macro_rules: List[str] = field(default_factory=list)
    imports: Set[str] = field(default_factory=set)


class SkeletonBuilder:
    """分层骨架构建器"""
    
    def __init__(
        self, 
        project_root: Path, 
        output_dir: Path,
        compile_commands_path: Path = None,
        ohos_root: Path = None
    ):
        """
        初始化骨架构建器
        
        Args:
            project_root: C++ 项目根目录
            output_dir: Rust 输出目录
            compile_commands_path: compile_commands.json 的路径（可选）
            ohos_root: OpenHarmony 源码根目录（可选，用于路径规范化）
        """
        self.project_root = Path(project_root)
        self.output_dir = Path(output_dir)
        self.components = SkeletonComponents()
        self.preprocessed_cache: Dict[str, str] = {}
        # bindgen allowlist cache (signatures): {(pre_path, lang, md5(sorted(names))): {name: rust_sig}}
        self._bindgen_fn_sig_cache: Dict[str, Dict[str, str]] = {}
        self.collected_custom_types = set()  # 用于收集所有非原生类型
        self.ohos_root = Path(ohos_root) if ohos_root else None
        self._ohos_build_out_dir: Optional[Path] = None
        # TU 上下文映射（由 get_dependencies.py 生成）：用于在 skeleton/bindgen 阶段复用“同一套 TU flags/宏/include 顺序”
        self._tu_context_map_path: Optional[Path] = None
        self._tu_context_files: Dict[str, Dict[str, Any]] = {}

        # SelfContained 模块常带 original_path.txt（OpenHarmony 源码中的原始相对路径）
        # 用于把 self.project_root 下的文件映射回 ohos_root 的真实路径，以便精确匹配 compile_commands.json。
        self._ohos_project_root: Optional[Path] = None
        self._ohos_project_rel: Optional[Path] = None
        try:
            original_path_file = self.project_root / "original_path.txt"
            if self.ohos_root and original_path_file.exists():
                original_rel = original_path_file.read_text(encoding="utf-8", errors="ignore").strip()
                if original_rel:
                    self._ohos_project_rel = Path(original_rel)
                    candidate = (self.ohos_root / original_rel).resolve()
                    if candidate.exists():
                        self._ohos_project_root = candidate
        except Exception:
            # 不影响主流程：无法映射则退回文件名匹配
            self._ohos_project_root = None
            self._ohos_project_rel = None
        
        # compile_commands.json 支持
        self.compile_commands_parser = None
        if compile_commands_path and COMPILE_COMMANDS_AVAILABLE:
            try:
                print(f"  [初始化] 创建 CompileCommandsParser...")
                self.compile_commands_parser = CompileCommandsParser(
                    compile_commands_path,
                    ohos_root
                )
                print(f"  ✓ CompileCommandsParser 初始化成功")
                logger.info(f"已加载 compile_commands.json: {compile_commands_path}")
            except Exception as e:
                logger.warning(f"加载 compile_commands.json 失败: {e}")
                print(f"  ✗ CompileCommandsParser 初始化失败: {e}")
                import traceback
                traceback.print_exc()

        # 推断 OpenHarmony 的真实 out_dir（用于生成 out/.../gen/... 生成物）
        try:
            self._ohos_build_out_dir = self._infer_ohos_build_out_dir()
        except Exception:
            self._ohos_build_out_dir = None
        
        # 收集所有可能的头文件目录
        print(f"  [收集] 正在收集头文件搜索目录...")
        self.include_dirs = self._collect_include_dirs()
        print(f"  ✓ 收集完成: {len(self.include_dirs)} 个头文件搜索目录")

        # 在 include 收集完成后加载 TU 上下文映射（可选）
        # 注意：映射文件位于 <workspace>/.preprocessed/tu_context_map.json，由阶段1依赖分析产生。
        self._load_tu_context_map()
        
        # 确保输出目录存在
        self.output_dir.mkdir(parents=True, exist_ok=True)
        (self.output_dir / "src").mkdir(exist_ok=True)

    def _load_tu_context_map(self) -> None:
        """
        Load TU context mapping produced by stage1 (get_dependencies.py).

        The mapping pins a file-group (safe_module_name) to the exact compile_commands entry and the
        resulting preprocessed `.i`, so later stages don't "re-pick" a different TU when multi-profile
        entries exist.
        """
        self._tu_context_files = {}
        candidates: List[Path] = []
        env_dir = os.environ.get("PREPROCESS_OUTPUT_DIR", "").strip()
        if env_dir:
            candidates.append(Path(env_dir) / "tu_context_map.json")
        # Best-effort fallback: infer workspace root from output_dir (workspace/skeletons/<proj>)
        try:
            ws_root = self.output_dir.parent.parent
            candidates.append(ws_root / ".preprocessed" / "tu_context_map.json")
        except Exception:
            pass

        for p in candidates:
            try:
                if not p.exists():
                    continue
                data = json.loads(p.read_text(encoding="utf-8", errors="ignore") or "{}")
                files = data.get("files") if isinstance(data, dict) else None
                if isinstance(files, dict):
                    self._tu_context_map_path = p
                    self._tu_context_files = {str(k): (v if isinstance(v, dict) else {}) for k, v in files.items()}
                    logger.info(f"已加载 TU 上下文映射: {p} (files={len(self._tu_context_files)})")
                    return
            except Exception as e:
                logger.debug(f"读取 TU 上下文映射失败: {p}: {e}")
                continue

    def _map_to_ohos_path(self, path: Path) -> Path:
        """
        将 SelfContained 模块中的路径映射到 OpenHarmony 源码树中的原始路径（如果可用）。

        目的：让 CompileCommandsParser 能用“精确路径匹配”命中 compile_commands.json 的条目，
        避免退化到文件名匹配导致拿到错误的编译上下文（include 顺序/宏定义等会错）。
        """
        if not self._ohos_project_root or not self.ohos_root:
            return Path(path)

        try:
            project_root_resolved = self.project_root.resolve()
            path_resolved = Path(path).resolve()
            rel = path_resolved.relative_to(project_root_resolved)
            candidate = (self._ohos_project_root / rel).resolve()
            return candidate if candidate.exists() else Path(path)
        except Exception:
            return Path(path)

    def _infer_ohos_build_out_dir(self) -> Optional[Path]:
        """
        尝试推断 OpenHarmony 真正的 build out_dir（用于生成 out/.../gen/... 生成物）。

        情况：
        1) compile_commands.json 位于 OHOS_ROOT/out/.../compile_commands.json
        2) compile_commands.json 是从 registry archive 解压出来的（translation_outputs/shared/...），
           同目录存在 profile.json，包含 out_dir 字段
        """
        if not self.ohos_root or not self.compile_commands_parser:
            return None
        try:
            ohos_root = Path(self.ohos_root).resolve()
        except Exception:
            return None

        try:
            cc_path = Path(self.compile_commands_parser.compile_db_path).resolve()
        except Exception:
            return None

        # Case 1: within OHOS tree.
        try:
            rel = cc_path.relative_to(ohos_root)
            if rel.parts and rel.parts[0] == "out":
                out_dir = (ohos_root / rel.parent).resolve()
                return out_dir if out_dir.exists() else out_dir
        except Exception:
            pass

        # Case 2: extracted archive with metadata.
        meta = cc_path.parent / "profile.json"
        if meta.exists():
            try:
                data = json.loads(meta.read_text(encoding="utf-8", errors="ignore"))
                out_dir_rel = str(data.get("out_dir") or "").replace("\\", "/").strip("/")
                if out_dir_rel:
                    out_dir = (ohos_root / out_dir_rel).resolve()
                    return out_dir if out_dir.exists() else out_dir
            except Exception:
                return None

        return None

    def _maybe_generate_ohos_out_gen_artifacts(self, attempt_debug: Dict[str, Any]) -> bool:
        """
        当 bindgen/clang 报错提示缺少 out/.../gen/... 相关 include 时，尝试通过 ninja 生成。

        这是“提升 stub 降级率”的关键：compile_commands.json 可能引用 out/.../gen/...，
        但仅 gn gen 并不会实际生成这些文件，需要 ninja/action 才会产出。
        """
        # Allow disabling for environments where building OpenHarmony is undesirable.
        if not self._env_flag("C2R_OHOS_GEN_ENSURE", default="1"):
            attempt_debug.setdefault("ohos_gen_ensure", {})["skipped"] = "disabled"
            return False
        if not self.ohos_root or not self.compile_commands_parser:
            return False
        if not self._ohos_build_out_dir:
            return False

        out_dir = Path(self._ohos_build_out_dir)
        build_ninja = out_dir / "build.ninja"
        if not build_ninja.exists():
            return False

        # Marker: avoid repeating heavy ninja work across retries in the same output tree.
        marker = None
        try:
            cc_path = Path(self.compile_commands_parser.compile_db_path).resolve()
            meta = cc_path.parent / "profile.json"
            if meta.exists():
                marker = cc_path.parent / "gen_ensured.marker"
                if marker.exists():
                    attempt_debug.setdefault("ohos_gen_ensure", {})["skipped"] = "marker_exists"
                    return False
        except Exception:
            marker = None

        # Only attempt if we can see that out/gen includes are currently missing.
        try:
            if (out_dir / "gen").exists():
                # Even if gen exists, some subpaths may still be missing; rely on bindgen include diag.
                pass
        except Exception:
            pass

        ninja = shutil.which("ninja")
        if not ninja:
            attempt_debug.setdefault("ohos_gen_ensure", {})["skipped"] = "ninja_not_found"
            return False

        # Seed targets: pick a few generated sources under out_dir/gen that are referenced by the compile DB
        # but not yet present on disk. Building generated *sources* tends to also materialize adjacent headers.
        seed_max = 12
        try:
            seed_max = int(os.environ.get("C2R_OHOS_GEN_SEED_MAX", str(seed_max)))
        except Exception:
            seed_max = 12

        targets: List[str] = []
        try:
            compile_db = getattr(self.compile_commands_parser, "compile_db", None) or []
            out_dir_str = str(out_dir).replace("\\", "/")
            for entry in compile_db:
                f = (entry or {}).get("file") or ""
                if not isinstance(f, str) or not f:
                    continue
                fp = f.replace("\\", "/")
                if "/gen/" not in fp:
                    continue
                # Absolute path within this out_dir
                if fp.startswith(out_dir_str + "/"):
                    p = Path(fp)
                else:
                    # Some compdb entries may be relative to `directory`
                    directory = (entry or {}).get("directory") or ""
                    if isinstance(directory, str) and directory:
                        p = (Path(directory) / f).resolve()
                    else:
                        p = Path(f)
                if not str(p).replace("\\", "/").startswith(out_dir_str + "/"):
                    continue
                if p.exists():
                    continue
                try:
                    rel = p.resolve().relative_to(out_dir.resolve())
                except Exception:
                    # best-effort
                    rel = Path(str(p).replace(out_dir_str + "/", ""))
                rel_s = str(rel).replace("\\", "/")
                if rel_s and rel_s not in targets:
                    targets.append(rel_s)
                if len(targets) >= seed_max:
                    break
        except Exception:
            targets = []

        if not targets:
            attempt_debug.setdefault("ohos_gen_ensure", {})["skipped"] = "no_missing_gen_targets"
            return False

        timeout_s = 600
        try:
            timeout_s = int(os.environ.get("C2R_OHOS_GEN_NINJA_TIMEOUT", str(timeout_s)))
        except Exception:
            timeout_s = 600

        attempt_debug.setdefault("ohos_gen_ensure", {}).update(
            {
                "out_dir": str(out_dir),
                "targets_count": len(targets),
                "targets_head": targets[:10],
                "timeout_s": timeout_s,
            }
        )

        try:
            proc = subprocess.run(
                [ninja, "-C", str(out_dir), *targets],
                capture_output=True,
                text=True,
                timeout=timeout_s,
            )
            attempt_debug["ohos_gen_ensure"].update(
                {
                    "returncode": proc.returncode,
                    "stdout_tail": (proc.stdout or "")[-2000:],
                    "stderr_tail": (proc.stderr or "")[-2000:],
                }
            )
            if proc.returncode == 0:
                if marker:
                    try:
                        marker.write_text(time.strftime("%Y-%m-%dT%H:%M:%S"), encoding="utf-8")
                    except Exception:
                        pass
                return True
            return False
        except subprocess.TimeoutExpired:
            attempt_debug["ohos_gen_ensure"].update({"returncode": "timeout"})
            return False
        except Exception as e:
            attempt_debug["ohos_gen_ensure"].update({"returncode": "exception", "error": str(e)[:300]})
            return False
    
    def _collect_include_dirs(self) -> List[Path]:
        """
        收集项目中所有可能的头文件目录
        
        优先使用 compile_commands.json 中的路径，然后添加项目内的路径
        """
        include_dirs = set()
        
        # 优先：从 compile_commands.json 获取所有 include 路径
        if self.compile_commands_parser:
            try:
                print(f"    [compile_commands] 从 compile_commands.json 提取所有 include 路径...")
                # 优化：不传入 source_files，直接提取所有 include 路径（更快）
                compile_includes = self.compile_commands_parser.get_all_include_dirs()
                include_dirs.update(compile_includes)
                print(f"    ✓ 从 compile_commands.json 获取了 {len(compile_includes)} 个 include 路径")
                logger.info(f"从 compile_commands.json 获取了 {len(compile_includes)} 个 include 路径")
            except Exception as e:
                logger.warning(f"从 compile_commands.json 获取 include 路径失败: {e}")
                print(f"    ✗ 从 compile_commands.json 获取 include 路径失败: {e}")
                import traceback
                traceback.print_exc()
        
        # 其次：添加项目根目录
        include_dirs.add(self.project_root)
        
        # 查找所有包含 .h 文件的目录
        for h_file in self.project_root.glob("**/*.h"):
            include_dirs.add(h_file.parent)
        for h_file in self.project_root.glob("**/*.hpp"):
            include_dirs.add(h_file.parent)
        
        # 常见的头文件目录名
        common_include_names = ['include', 'inc', 'headers', 'src', 'source']
        for name in common_include_names:
            for d in self.project_root.glob(f"**/{name}"):
                if d.is_dir():
                    include_dirs.add(d)
        
        return list(include_dirs)
    
    # =========================================================================
    # 阶段 A: 预处理与类型层 (The Truth Layer)
    # =========================================================================
    
    def _extract_source_includes(self, source_files: List[Path]) -> List[str]:
        """
        从源文件中提取 #include 指令引用的头文件名
        
        关键功能：分析源文件（.c/.cpp）中的 include 语句，
        找出那些不在项目 include 目录中的外部依赖头文件。
        
        注意：只检查项目自己的 include 目录，而不是全部 compile_commands.json
        中的 5000+ 个目录。这样才能正确识别需要额外包含的外部头文件。
        
        Args:
            source_files: 源文件列表
        
        Returns:
            外部头文件名列表（例如 ["softbus_error_code.h", "xxx.h"]）
        """
        external_includes = set()
        
        # 正则表达式匹配 #include 指令
        include_pattern = re.compile(r'#\s*include\s*[<"]([^>"]+)[>"]')
        
        # 只使用项目自己的 include 目录（不使用全部 5000+ 个目录）
        project_include_dirs = []
        for src_file in source_files:
            if src_file.exists():
                # 项目的 include 目录通常与 src 同级
                project_root = src_file.parent.parent
                include_dir = project_root / "include"
                if include_dir.exists() and include_dir not in project_include_dirs:
                    project_include_dirs.append(include_dir)
        
        for src_file in source_files:
            if not src_file.exists():
                continue
            
            try:
                with open(src_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                for match in include_pattern.finditer(content):
                    header_name = match.group(1)
                    
                    # 跳过标准库头文件（没有扩展名的通常是 C++ 标准库）
                    if not header_name.endswith('.h') and not header_name.endswith('.hpp'):
                        continue
                    
                    # 跳过系统头文件（以 <> 包含的）
                    # 正则已经匹配了 <> 和 "" 两种情况，这里通过路径判断
                    if '/' in header_name and header_name.startswith(('sys/', 'linux/', 'bits/')):
                        continue

                    # 跳过常见系统头文件（不需要显式加入 wrapper；应由 sysroot/标准 include 解析）
                    # 否则容易被错误解析到 OpenHarmony 源码树里的 musl 内部头（third_party/musl/src/include/*）
                    # 导致出现 `hidden` 等内部宏/关键字解析错误。
                    if header_name in {
                        "assert.h",
                        "ctype.h",
                        "errno.h",
                        "fcntl.h",
                        "inttypes.h",
                        "limits.h",
                        "locale.h",
                        "math.h",
                        "pthread.h",
                        "sched.h",
                        "setjmp.h",
                        "signal.h",
                        "stdarg.h",
                        "stdbool.h",
                        "stddef.h",
                        "stdint.h",
                        "stdio.h",
                        "stdlib.h",
                        "string.h",
                        "time.h",
                        "unistd.h",
                    }:
                        continue
                    
                    # 检查是否在项目自己的 include 目录中
                    is_in_project = False
                    for inc_dir in project_include_dirs:
                        if (inc_dir / header_name).exists():
                            is_in_project = True
                            break
                    
                    if not is_in_project:
                        external_includes.add(header_name)
            
            except Exception as e:
                logger.debug(f"分析 {src_file} 的 include 指令失败: {e}")
        
        return list(external_includes)
    
    def _resolve_external_headers(self, header_names: List[str], source_files: Optional[List[Path]] = None) -> List[Path]:
        """
        通过 compile_commands.json 解析外部头文件的真实路径
        
        注意：同一个头文件名可能存在于多个位置，优先选择：
        1. 在 interfaces/kits 目录下的（通常是官方 API）
        2. 在 foundation/communication 目录下的
        3. 其他位置
        
        Args:
            header_names: 头文件名列表（例如 ["softbus_error_code.h"]）
            source_files: 源文件列表（可选，用于按真实 TU 的 include 顺序解析同名头文件）
        
        Returns:
            找到的头文件完整路径列表
        """
        resolved_headers = []
        
        if not self.compile_commands_parser:
            logger.warning("没有 compile_commands.json，无法解析外部头文件")
            return resolved_headers
        
        # 关键点：同名头文件在 OpenHarmony 源码树里非常常见，必须尽量复现“真实 TU 的 include 搜索顺序”。
        # 否则会随机命中到 C++ 头（innerkits/ipc_core）等不相关实现，导致 bindgen 大面积失败。
        ordered_include_dirs: List[Path] = []
        if source_files:
            try:
                rep_src = None
                for s in source_files:
                    suf = str(s).lower()
                    if suf.endswith((".c", ".cc", ".cpp", ".cxx", ".c++")):
                        rep_src = Path(s)
                        break
                if rep_src:
                    ordered_include_dirs = list(
                        self.compile_commands_parser.get_includes_for_file(
                            self._map_to_ohos_path(rep_src),
                            normalize_paths=True,
                        ) or []
                    )
            except Exception:
                ordered_include_dirs = []

        # 获取所有已知的 include 目录（作为兜底）
        try:
            all_include_dirs = list(self.compile_commands_parser.get_all_include_dirs() or [])
        except Exception:
            all_include_dirs = []
        all_include_dirs = sorted(all_include_dirs, key=lambda p: str(p))
        
        for header_name in header_names:
            candidates = []
            
            # 1) 先按真实 TU include 顺序查找：找到第一个即为“真实命中”
            for inc_dir in ordered_include_dirs:
                header_path = Path(inc_dir) / header_name
                if header_path.exists():
                    candidates.append(header_path)
                    break

            # 2) 兜底：在所有 include 目录中查找所有匹配项（用于 TU 未覆盖到的 includes）
            if not candidates:
                for inc_dir in all_include_dirs:
                    header_path = inc_dir / header_name
                    if header_path.exists():
                        candidates.append(header_path)
            
            if candidates:
                # 优先选择最合适的路径（当 candidates>1 时需要稳定决策）
                filtered = []
                for c in candidates:
                    s = str(c).replace("\\", "/")
                    # Avoid musl source-internal headers (not public; also tends to be incompatible with target configs).
                    if "/third_party/musl/src/include/" in s:
                        continue
                    filtered.append(c)
                candidates = filtered or candidates

                def _rank(p: Path) -> Tuple[int, str]:
                    s = str(p).replace("\\", "/")
                    # Prefer C innerkits over C++ core headers when names collide.
                    if "/interfaces/innerkits/c/" in s:
                        return (0, s)
                    # Official kits APIs
                    if "interfaces/kits" in s:
                        return (1, s)
                    # Communication module (common for ipc/rpc)
                    if "foundation/communication" in s:
                        # De-prioritize ipc_core C++ headers when we're generating C bindings
                        if "/innerkits/ipc_core/" in s or "/include/c++/" in s or "/c++/" in s:
                            return (50, s)
                        return (2, s)
                    if "dsoftbus" in s:
                        return (3, s)
                    return (10, s)

                best_candidate = sorted(candidates, key=_rank)[0]

                resolved_headers.append(best_candidate)
                logger.info(f"✓ 找到外部头文件: {header_name} -> {best_candidate}")
            else:
                logger.debug(f"未找到外部头文件: {header_name}")
        
        return resolved_headers
    
    def preprocess_source(self, c_file_path: Path, include_dirs: List[Path] = None) -> str:
        """
        预处理 C/C++ 源文件
        
        策略（参考 EvoC2rust）：
        1. 首先尝试"轻量预处理": gcc -fpreprocessed -dD -E
           - 不展开 #include，只移除注释
           - 不需要头文件，永不失败
        2. 如果需要完整预处理（有 compile_commands.json），则尝试 gcc -E
        
        Args:
            c_file_path: C/C++ 源文件路径
            include_dirs: 额外的头文件搜索路径
        
        Returns:
            预处理后的源代码
        """
        if str(c_file_path) in self.preprocessed_cache:
            return self.preprocessed_cache[str(c_file_path)]
        
        # ========== 策略 1: 轻量预处理 + 宏展开 ==========
        # 增强版：在移除注释的同时，展开 OpenHarmony/LiteOS 常用宏
        # 解决 STATIC UINT32 TelnetOpen(...) 被解析为 fn UINT32 的问题
        try:
            # 先处理续行符 \\\n
            with open(c_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                raw_content = f.read()
            
            # 移除续行符
            content_no_continuation = raw_content.replace('\\\n', '')
            
            # 🔑 关键修复：展开 OpenHarmony/LiteOS 常用宏
            # 这些宏在源码中大量使用，如果不展开会导致 Tree-sitter 解析错误
            # 
            # ★★★ 新增：使用 MacroLearner 进行动态宏展开 ★★★
            # 优势：
            # 1. 可以跨项目共享学习到的宏
            # 2. 自动从解析错误中学习新宏
            # 3. 持久化保存，越用越聪明
            
            if MACRO_LEARNER_AVAILABLE:
                # 使用 MacroLearner (推荐)
                macro_learner = get_global_macro_learner()
                processed_content = macro_learner.expand_all(content_no_continuation)
                logger.debug(f"使用 MacroLearner 展开宏: {macro_learner.get_macro_count()}")
            else:
                # 回退到硬编码宏列表
                ohos_macro_expansions = {
                    # 存储类说明符宏
                    'STATIC': 'static',
                    'INLINE': 'inline',
                    'LITE_OS_SEC_TEXT': '',
                    'LITE_OS_SEC_TEXT_MINOR': '',
                    'LITE_OS_SEC_TEXT_INIT': '',
                    'LITE_OS_SEC_DATA': '',
                    'LITE_OS_SEC_DATA_INIT': '',
                    'LITE_OS_SEC_BSS': '',
                    'LITE_OS_SEC_RODATA': '',
                    
                    
                    # 基础类型宏
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
                }
                
                # 使用正则替换宏（只替换独立的标识符，不替换子串）
                processed_content = content_no_continuation
                for macro, expansion in ohos_macro_expansions.items():
                    # 使用 word boundary 避免部分匹配
                    pattern = rf'\b{re.escape(macro)}\b'
                    processed_content = re.sub(pattern, expansion, processed_content)
                
            
            # 写入临时文件
            import tempfile
            with tempfile.NamedTemporaryFile(mode='w', suffix=c_file_path.suffix, delete=False, encoding='utf-8') as tmp:
                tmp.write(processed_content)
                tmp_path = tmp.name
            
            try:
                # 轻量预处理：只移除注释，-P 移除行号标记
                light_cmd = ["gcc", "-fpreprocessed", "-dD", "-E", "-P", tmp_path]
                result = subprocess.run(light_cmd, capture_output=True, text=True, timeout=30)
                
                if result.returncode == 0 and result.stdout.strip():
                    preprocessed = result.stdout
                    self.preprocessed_cache[str(c_file_path)] = preprocessed
                    logger.info(f"轻量预处理成功 (含宏展开): {c_file_path.name}")
                    return preprocessed
            finally:
                # 清理临时文件
                try:
                    os.unlink(tmp_path)
                except:
                    pass
        except Exception as e:
            logger.debug(f"轻量预处理异常: {c_file_path.name}, {e}")
        
        # ========== 策略 2: 完整预处理 (需要头文件) ==========
        try:
            cmd = ["gcc", "-E", "-P"]  # -P 不输出行号标记
            
            # 优先：从 compile_commands.json 获取该文件的精确 include 路径
            compile_includes = []
            if self.compile_commands_parser:
                try:
                    compile_includes = self.compile_commands_parser.get_includes_for_file(
                        self._map_to_ohos_path(c_file_path),
                        normalize_paths=True
                    )
                    if compile_includes:
                        logger.debug(f"从 compile_commands.json 获取了 {len(compile_includes)} 个 include 路径: {c_file_path.name}")
                except Exception as e:
                    logger.debug(f"获取 compile_commands include 路径失败: {e}")
            
            # 添加 compile_commands.json 的路径（优先级最高）
            added_dirs = set()
            for inc_dir in compile_includes:
                inc_str = str(inc_dir)
                if inc_str not in added_dirs:
                    cmd.extend(["-I", inc_str])
                    added_dirs.add(inc_str)
            
            # 添加所有收集到的头文件搜索路径（作为补充）
            for inc_dir in self.include_dirs:
                inc_str = str(inc_dir)
                if inc_str not in added_dirs:
                    cmd.extend(["-I", inc_str])
                    added_dirs.add(inc_str)
            
            # 添加额外指定的搜索路径
            if include_dirs:
                for inc_dir in include_dirs:
                    inc_str = str(inc_dir)
                    if inc_str not in added_dirs:
                        cmd.extend(["-I", inc_str])
                        added_dirs.add(inc_str)
            
            # 添加源文件所在目录
            source_dir_str = str(c_file_path.parent)
            if source_dir_str not in added_dirs:
                cmd.extend(["-I", source_dir_str])
            
            # 定义常用宏以避免编译错误
            cmd.extend([
                "-D__attribute__(x)=",
                "-D__extension__=",
                "-D__restrict=",
                "-D__inline=inline",
                "-D__inline__=inline",
                "-D__asm__(x)=",
                "-D__volatile__=",
                "-D__builtin_va_list=void*",
            ])
            
            cmd.append(str(c_file_path))
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode == 0:
                preprocessed = result.stdout
                self.preprocessed_cache[str(c_file_path)] = preprocessed
                logger.info(f"预处理成功: {c_file_path.name}")
                return preprocessed
            else:
                logger.warning(f"预处理失败: {c_file_path.name}, 使用清理后的源码")
                logger.debug(f"错误: {result.stderr[:500]}")
                # 失败时返回清理后的源码（移除会导致 tree-sitter 解析失败的语法）
                with open(c_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    raw_code = f.read()
                return self._sanitize_source_for_treesitter(raw_code)
                    
        except subprocess.TimeoutExpired:
            logger.warning(f"预处理超时: {c_file_path.name}")
            with open(c_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                raw_code = f.read()
            return self._sanitize_source_for_treesitter(raw_code)
        except Exception as e:
            logger.error(f"预处理异常: {c_file_path.name}, {e}")
            with open(c_file_path, 'r', encoding='utf-8', errors='ignore') as f:
                raw_code = f.read()
            return self._sanitize_source_for_treesitter(raw_code)
    
    def _sanitize_source_for_treesitter(self, source_code: str) -> str:
        """
        清理源码以提高 tree-sitter 解析成功率
        
        当 gcc -E 预处理失败时使用此方法
        
        处理的问题：
        1. __attribute__((xxx)) - GCC 扩展语法
        2. #include "xxx.txt" - 嵌入式包含
        3. __asm__ 块
        4. 多行注释中的中文
        """
        import re
        
        result = source_code
        
        # 1. 移除 __attribute__((xxx))
        # 处理多行和嵌套括号
        result = re.sub(r'__attribute__\s*\(\([^)]*\)\)', '', result)
        result = re.sub(r'__attribute__\s*\(\(.*?\)\)', '', result, flags=re.DOTALL)
        
        # 2. 注释掉嵌入式 #include (如 #include "xxx.txt")
        result = re.sub(r'(#include\s+"[^"]+\.txt")', r'/* \1 */', result)
        
        # 3. 移除 __asm__ 和 __volatile__
        result = re.sub(r'__asm__\s*\([^;]*\);', ';', result)
        result = re.sub(r'__volatile__', '', result)
        
        # 4. 移除 __extension__
        result = re.sub(r'__extension__', '', result)
        
        # 5. 移除 __restrict
        result = re.sub(r'__restrict', '', result)
        
        # 6. 简化 __builtin_xxx 调用
        result = re.sub(r'__builtin_va_list', 'void*', result)
        result = re.sub(r'__builtin_offsetof\s*\([^)]+\)', '0', result)
        
        # 7. 移除 aligned(N) 属性 (如果残留)
        result = re.sub(r'__aligned\s*\(\s*\d+\s*\)', '', result)
        
        # 8. 移除多行注释中可能导致问题的内容（保留注释标记）
        def clean_multiline_comment(match):
            content = match.group(0)
            # 只保留 ASCII 字符和基本换行
            cleaned = ''.join(c if ord(c) < 128 or c in '\n\r\t ' else ' ' for c in content)
            return cleaned
        
        result = re.sub(r'/\*.*?\*/', clean_multiline_comment, result, flags=re.DOTALL)
        
        return result
    
    def _count_parse_errors(self, root_node) -> int:
        """统计 AST 中的 ERROR 节点数量"""
        count = 0
        if root_node.type == 'ERROR':
            count = 1
        for child in root_node.children:
            count += self._count_parse_errors(child)
        return count
    
    # =========================================================================
    # 头文件预检系统 (Pre-flight Include Check)
    # =========================================================================
    
    def _resolve_include_path(self, header_name: str, search_paths: List[Path]) -> Optional[Path]:
        """
        模拟编译器的查找逻辑：在搜索路径中检查头文件是否存在
        
        Args:
            header_name: 头文件名（可能包含相对路径，如 "sys/types.h"）
            search_paths: 搜索路径列表
        
        Returns:
            找到的完整路径，或 None
        """
        for path in search_paths:
            potential_path = Path(path) / header_name
            if potential_path.exists():
                return potential_path
        return None
    
    def _preflight_check_includes(
        self, 
        header_files: List[Path],
        verbose: bool = True
    ) -> Tuple[bool, Set[Path]]:
        """
        在运行 bindgen 前，预先检查所有头文件是否可达。
        如果不可达，尝试通过全局索引修补。
        
        这是一种主动修复策略，而非被动等待 bindgen 报错。
        
        Args:
            header_files: 要处理的头文件列表
            verbose: 是否输出详细信息
        
        Returns:
            (是否所有头文件都可达, 新增的搜索路径集合)
        """
        all_includes_found = True
        new_paths = set()
        missing_headers = []
        
        # 提取所有需要检查的 #include
        headers_to_check = set()
        
        include_pattern = re.compile(r'#\s*include\s*(?:"([^"]+)"|<([^>]+)>)')
        
        for header_file in header_files:
            if not header_file.exists():
                continue
            
            try:
                with open(header_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                matches = include_pattern.findall(content)
                for m in matches:
                    header_name = m[0] or m[1]
                    if header_name:
                        headers_to_check.add(header_name)
            except Exception as e:
                logger.debug(f"读取头文件 {header_file} 失败: {e}")
        
        if verbose:
            print(f"🔍 Pre-flight check: Verifying {len(headers_to_check)} includes...")
        
        # 当前的搜索路径
        current_search_paths = list(self.include_dirs)
        
        # 逐个检查
        for header in sorted(headers_to_check):
            # 1. 先在现有路径里找
            if self._resolve_include_path(header, current_search_paths):
                continue  # 找到了，下一个
            
            # 2. 没找到！触发暴力搜索
            if verbose:
                print(f"  ⚠️ Missing: {header}")
            
            found_path = None
            
            # 2.1 尝试通过 compile_commands.json 查找
            if self.compile_commands_parser:
                try:
                    found_path = self.compile_commands_parser.find_header_path(header)
                except Exception as e:
                    logger.debug(f"通过 compile_commands 查找 {header} 失败: {e}")
            
            # 2.2 尝试在项目目录递归搜索
            if not found_path and self.project_root:
                for candidate in self.project_root.rglob(Path(header).name):
                    if candidate.is_file():
                        found_path = candidate.parent
                        break
            
            if found_path:
                if found_path not in current_search_paths and found_path not in new_paths:
                    if verbose:
                        print(f"    ✅ Auto-fixed: Adding {found_path}")
                    new_paths.add(found_path)
                    current_search_paths.append(found_path)
            else:
                all_includes_found = False
                missing_headers.append(header)
                if verbose:
                    print(f"    ❌ Not found anywhere")
        
        # 将新路径添加到 include_dirs
        for p in new_paths:
            if p not in self.include_dirs:
                self.include_dirs.append(p)
        
        if missing_headers:
            logger.warning(f"Pre-flight: {len(missing_headers)} headers not found: {missing_headers[:10]}")
        elif verbose:
            print(f"✅ Pre-flight check passed! All includes resolved.")
        
        return all_includes_found, new_paths
    
    # =========================================================================
    # 动态宏学习系统 (Dynamic Macro Learning)
    # =========================================================================
    
    # 已学习的宏定义（跨项目持久化）
    _learned_macros: Dict[str, str] = {}
    
    def _learn_macro_from_parse_error(
        self, 
        source_code: str, 
        error_line: int
    ) -> Optional[Tuple[str, str]]:
        """
        从 Tree-sitter 解析错误中学习新的宏定义
        
        当 Tree-sitter 解析失败时，分析错误行的代码，
        尝试识别干扰解析的宏。
        
        Args:
            source_code: 源代码
            error_line: 错误行号（1-based）
        
        Returns:
            (宏名, 展开后的值) 或 None
        """
        lines = source_code.split('\n')
        if error_line < 1 or error_line > len(lines):
            return None
        
        line = lines[error_line - 1].strip()
        
        # 常见的宏模式
        # 模式1: MACRO type function(...) - 类型说明符宏
        type_specifier_pattern = re.match(
            r'^([A-Z][A-Z0-9_]*)\s+((?:unsigned\s+|signed\s+)?(?:int|long|short|char|void|float|double|_Bool|\w+_t))\s+(\w+)\s*\(',
            line
        )
        if type_specifier_pattern:
            potential_macro = type_specifier_pattern.group(1)
            # 检查是否是已知的类型修饰符宏
            if potential_macro in ['STATIC', 'INLINE', 'EXTERN', 'CONST', 
                                   'LITE_OS_SEC_TEXT', 'LITE_OS_SEC_DATA',
                                   'HDF_STATIC', 'OHOS_API']:
                # 这是存储类说明符宏，应该展开为空或 static/inline
                if 'STATIC' in potential_macro:
                    return (potential_macro, 'static')
                elif 'INLINE' in potential_macro:
                    return (potential_macro, 'inline')
                else:
                    return (potential_macro, '')
        
        # 模式2: MACRO function(...) - 宏被误认为函数名
        func_name_pattern = re.match(
            r'^(?:static\s+|inline\s+)*([A-Z][A-Z0-9_]*)\s+(\w+)\s*\(',
            line
        )
        if func_name_pattern:
            potential_macro = func_name_pattern.group(1)
            # 检查是否是已知的类型宏
            known_type_macros = {
                'VOID': 'void', 'CHAR': 'char', 'BOOL': '_Bool',
                'INT8': 'signed char', 'UINT8': 'unsigned char',
                'INT16': 'short', 'UINT16': 'unsigned short',
                'INT32': 'int', 'UINT32': 'unsigned int',
                'INT64': 'long long', 'UINT64': 'unsigned long long',
                'FLOAT': 'float', 'DOUBLE': 'double',
                'UINTPTR': 'uintptr_t', 'INTPTR': 'intptr_t',
            }
            if potential_macro in known_type_macros:
                return (potential_macro, known_type_macros[potential_macro])
            
            # 尝试推断：全大写标识符在函数定义开头，可能是类型宏
            if len(potential_macro) >= 2 and potential_macro.isupper():
                logger.info(f"🎓 Learned new type macro: {potential_macro} -> int")
                return (potential_macro, 'int')  # 默认假设为 int
        
        return None
    
    def _expand_learned_macros(self, source_code: str) -> str:
        """
        应用已学习的宏展开
        
        Args:
            source_code: 源代码
        
        Returns:
            宏展开后的源代码
        """
        result = source_code
        
        for macro, expansion in self._learned_macros.items():
            pattern = rf'\b{re.escape(macro)}\b'
            result = re.sub(pattern, expansion, result)
        
        return result
    
    def _parse_with_macro_learning(
        self, 
        source_code: str,
        max_learning_iterations: int = 3
    ) -> Tuple[Any, str]:
        """
        带宏学习的 Tree-sitter 解析
        
        如果解析失败（ERROR 节点过多），尝试自动学习新宏并重试。
        
        ★★★ 增强：使用 MacroLearner 模块进行持久化的宏学习 ★★★
        - 学习到的宏可以跨项目复用
        - 自动保存到 ~/.c2rust/learned_macros.json
        - 系统越用越聪明
        
        Args:
            source_code: 源代码
            max_learning_iterations: 最大学习迭代次数
        
        Returns:
            (AST tree, 处理后的源代码)
        """
        current_source = source_code
        
        # 优先使用 MacroLearner（如果可用）
        macro_learner = get_global_macro_learner() if MACRO_LEARNER_AVAILABLE else None
        
        for iteration in range(max_learning_iterations):
            # 解析
            tree = cpp_parser.parse(bytes(current_source, 'utf-8'))
            
            # 检查健康度
            error_rate = self._get_error_rate(tree.root_node)
            
            if error_rate < 0.1:  # 小于 10% 错误率，认为解析成功
                if iteration > 0:
                    logger.info(f"🎓 Macro learning successful after {iteration} iterations")
                return tree, current_source
            
            # 尝试从错误位置学习宏
            error_locations = self._get_error_locations(tree.root_node)
            
            if not error_locations:
                break
            
            learned_any = False
            
            if macro_learner:
                # ★ 使用 MacroLearner 学习新宏 ★
                result = macro_learner.learn_from_parse_errors(
                    current_source, 
                    error_locations,
                    max_learn=10
                )
                if result.learned:
                    logger.info(f"🎓 Learned {len(result.learned)} macros via MacroLearner")
                    learned_any = True
                    current_source = result.source_modified
            else:
                # 回退到本地学习（不持久化）
                for line_num, _ in error_locations[:5]:
                    result = self._learn_macro_from_parse_error(current_source, line_num)
                    if result:
                        macro_name, expansion = result
                        if macro_name not in self._learned_macros:
                            logger.info(f"🎓 Auto-learned macro: {macro_name} -> {expansion}")
                            self._learned_macros[macro_name] = expansion
                            learned_any = True
                
                if learned_any:
                    current_source = self._expand_learned_macros(current_source)
            
            if not learned_any:
                break
        
        return tree, current_source
    
    def _get_error_rate(self, root_node) -> float:
        """计算 AST 错误率"""
        total = [0]
        errors = [0]
        
        def traverse(node):
            total[0] += 1
            if node.type == 'ERROR':
                errors[0] += 1
            for child in node.children:
                traverse(child)
        
        traverse(root_node)
        return errors[0] / total[0] if total[0] > 0 else 0.0
    
    def _get_error_locations(self, root_node) -> List[Tuple[int, int]]:
        """获取所有 ERROR 节点的位置"""
        locations = []
        
        def traverse(node):
            if node.type == 'ERROR':
                locations.append((
                    node.start_point[0] + 1,  # 行号从 1 开始
                    node.start_point[1]
                ))
            for child in node.children:
                traverse(child)
        
        traverse(root_node)
        return locations
    
    # =========================================================================
    # 类型骨架生成
    # =========================================================================
    
    def generate_type_skeleton(
        self, 
        header_files: List[Path], 
        output_file: str = "types.rs",
        source_files: List[Path] = None
    ) -> bool:
        """
        使用 bindgen 生成 types.rs，包含所有 C 的 struct, enum, union, typedef
        
        这是项目的"真理层"——类型定义绝对正确
        
        参考 LLMigrate 和 EvoC2Rust 的方法
        
        增强功能：
        1. 基于构建数据库的智能寻路 (Build-Database Guided Discovery)
        2. 自动分析源文件中的 #include 指令，解析外部依赖头文件
        
        Args:
            header_files: 头文件列表
            output_file: 输出文件名
            source_files: 源文件列表（可选，用于分析外部依赖）
        
        Returns:
            是否成功
        """
        if not header_files:
            logger.warning("没有头文件，跳过 bindgen 类型生成")
            return False
        
        # ========== 增强：分析源文件中的外部依赖 ==========
        all_headers = list(header_files)  # 复制列表
        
        if source_files:
            external_includes = self._extract_source_includes(source_files)
            if external_includes:
                print(f"  ✓ 发现 {len(external_includes)} 个外部头文件依赖: {external_includes}")
                logger.info(f"发现 {len(external_includes)} 个外部头文件依赖: {external_includes}")
                
                # 尝试解析外部头文件的真实路径
                resolved_headers = self._resolve_external_headers(external_includes, source_files=source_files)
                if resolved_headers:
                    print(f"  ✓ 成功解析 {len(resolved_headers)} 个外部头文件")
                    logger.info(f"成功解析 {len(resolved_headers)} 个外部头文件")
                    all_headers.extend(resolved_headers)
        
        # ========== ★★★ 新增：预检与修补 ★★★ ==========
        # 在运行 bindgen 前，主动检查所有头文件是否可达
        # 如果发现缺失，立即触发全局搜索修补，避免被动等待 bindgen 报错
        print("\n📋 Stage A.1: Pre-flight Include Check")
        all_resolved, new_paths = self._preflight_check_includes(all_headers, verbose=True)
        
        if new_paths:
            print(f"  ✅ Pre-flight fixed {len(new_paths)} include paths")
            logger.info(f"预检修补了 {len(new_paths)} 个 include 路径")
        
        # ========== ★★★ 现在再运行 bindgen，成功率极高 ★★★ ==========
        print("\n📋 Stage A.2: Running Bindgen")
        # bindgen 的“智能寻路重试”次数。
        # 经验：很多 OHOS 头文件缺失是“链式”的（补齐一个头后，下一层又缺），
        # 为了尽量降低 stub 比例，默认给到更高的重试上限（仍会在无进展时提前退出）。
        try:
            max_retries = int(os.environ.get("C2R_BINDGEN_MAX_RETRIES", "12"))
            if max_retries < 1:
                max_retries = 1
        except Exception:
            max_retries = 12
        success = self._run_bindgen_with_smart_discovery(
            all_headers,
            output_file,
            max_retries=max_retries,
            source_files=source_files,
        )
        return success
    
    def _run_bindgen_with_smart_discovery(
        self,
        header_files: List[Path],
        output_file: str,
        max_retries: int = 5,
        source_files: List[Path] = None,
    ) -> bool:
        """
        三段式回退的 types.rs 生成器

        Phase 1 改进：移除 LLM 依赖，使用确定性的三段式回退

        策略：
        - Tier A: bindgen (primary)
        - Tier B: clang -E 预处理 → bindgen (secondary)
        - Tier C: stub types.rs (最终兜底，保证编译通过)

        同时生成 types_generation_report.json 用于调试

        Args:
            header_files: 头文件列表
            output_file: 输出文件名
            max_retries: 最大重试次数

        Returns:
            是否成功
        """
        import json

        truth_mode = self._env_flag("C2R_TRUTH_MODE", "0")
        # In truth-mode, disable any "make it compile" fallbacks that are not true C/Rust bindings.
        # Keep the code paths for future rollback/ablation via env overrides.
        enable_types_rs_sanitizer = (not truth_mode) or self._env_flag("C2R_TRUTH_ALLOW_TYPES_RS_SANITIZER", "0")
        enable_tier_b = (not truth_mode) or self._env_flag("C2R_TRUTH_ALLOW_BINDGEN_TIER_B", "0")
        enable_tier_c_stub = (not truth_mode) or self._env_flag("C2R_TRUTH_ALLOW_BINDGEN_TIER_C_STUB", "0")

        # 初始化 include 路径集合
        #
        # 重要：不要一开始就把“全局 include 并集”塞进 bindgen。
        # 这会引入大量同名头文件碰撞（例如 string.h/session.h），导致选错头文件而失败或生成错误语义。
        #
        # 如果有 compile_commands + source_files，我们优先依赖“单个 TU 的真实上下文（保序）”；
        # 缺啥头文件再按需补齐（smart discovery）。
        current_includes: Set[Path] = set()
        if not self.compile_commands_parser or not source_files:
            current_includes = set(self.include_dirs)

        # 记录已尝试查找但找不到的头文件（避免重复查找）
        unresolvable_headers = set()

        output_path = self.output_dir / "src" / output_file

        # 生成报告的数据
        report = {
            "mode": None,
            "success": False,
            "missing_types": [],
            "source_scan_files": [str(h) for h in header_files],
            "compile_commands_loaded": bool(self.compile_commands_parser),
            "compile_commands_path": str(self.compile_commands_parser.compile_db_path) if self.compile_commands_parser else None,
            "project_root": str(self.project_root),
            "ohos_root": str(self.ohos_root) if self.ohos_root else None,
            "ohos_project_rel": str(self._ohos_project_rel) if self._ohos_project_rel else None,
            "bindgen_debug": self._env_flag("C2R_BINDGEN_DEBUG"),
            "bindgen_debug_keep_files": self._env_flag("C2R_BINDGEN_DEBUG_KEEP_FILES"),
            "truth_mode": truth_mode,
            "enable_types_rs_sanitizer": enable_types_rs_sanitizer,
            "enable_tier_b": enable_tier_b,
            "enable_tier_c_stub": enable_tier_c_stub,
            # 是否存在多 build profile：同一 source file 在 compile_commands.json 中出现多条不同命令
            "has_multiple_build_profiles": False,
            # 仅记录前若干条样例，避免报告过大
            "multi_build_profiles": [],
            "attempts": [],
            "final_output_valid": False
        }

        # Hint subpaths (relative to OHOS root) for smarter missing-header resolution.
        # Use a few prefix segments to avoid overly-specific matches.
        preferred_subpaths: Optional[List[str]] = None
        try:
            if self._ohos_project_rel:
                parts = list(self._ohos_project_rel.parts)
                max_parts = min(len(parts), 6)
                preferred_subpaths = [str(Path(*parts[:n])) for n in range(max_parts, 1, -1)]
        except Exception:
            preferred_subpaths = None

        # ------------------------------------------------------------------
        # Build profile 诊断（不影响 types 生成，只用于输出可解释的“构建上下文风险”）
        # ------------------------------------------------------------------
        if source_files and self.compile_commands_parser:
            try:
                max_samples = 20
                multi_profiles = []
                for src in source_files:
                    mapped_src = self._map_to_ohos_path(Path(src))
                    try:
                        src_resolved = Path(mapped_src).resolve(strict=False)
                    except Exception:
                        src_resolved = Path(mapped_src)

                    entries = self.compile_commands_parser.get_all_entries_for_file(src_resolved)
                    if not entries or len(entries) <= 1:
                        continue

                    # 仅保留“确实命中同一路径”的条目，避免文件名碰撞导致误报
                    resolved_entries = []
                    for e in entries:
                        ef = e.get("file")
                        if not ef:
                            continue
                        entry_dir = e.get("directory") or ""
                        ef_path = Path(ef)
                        if not ef_path.is_absolute() and entry_dir:
                            ef_path = Path(entry_dir) / ef_path
                        try:
                            if ef_path.resolve(strict=False) == src_resolved:
                                resolved_entries.append(e)
                        except Exception:
                            continue

                    if len(resolved_entries) <= 1:
                        continue

                    # 统计 unique command（同一 file 若存在多条不同 command，视为 multi-profile）
                    unique_cmds = set()
                    for e in resolved_entries:
                        cmd = e.get("command")
                        if not cmd:
                            args = e.get("arguments")
                            if isinstance(args, list) and args:
                                cmd = " ".join(str(a) for a in args)
                        if cmd:
                            unique_cmds.add(cmd)

                    if len(unique_cmds) <= 1:
                        # 多条 entry 但命令相同 → 视为重复，不算多 profile
                        continue

                    multi_profiles.append({
                        "file": str(src_resolved),
                        "unique_commands": len(unique_cmds),
                        "entries": len(resolved_entries),
                    })

                    if len(multi_profiles) >= max_samples:
                        break

                if multi_profiles:
                    report["has_multiple_build_profiles"] = True
                    report["multi_build_profiles"] = multi_profiles
            except Exception as e:
                # 诊断失败不影响主流程
                report["build_profile_diagnostic_error"] = str(e)[:300]

        # ========== Tier A: 直接 bindgen ==========
        print("\n📋 Tier A: Direct Bindgen")
        for attempt in range(max_retries):
            # 尝试运行 bindgen
            success, error_msg, missing_files, attempt_debug = self._attempt_bindgen(
                header_files, output_file, current_includes, source_files=source_files
            )

            report["attempts"].append({
                "tier": "A",
                "attempt": attempt + 1,
                "success": success,
                "error": error_msg[:500] if error_msg else None,
                "missing_files": missing_files,
                "debug": attempt_debug,
            })

            if success:
                # 验证输出
                is_valid, validation_msg = self._validate_types_rs(output_path)

                # 如果验证失败，尝试净化
                if not is_valid:
                    if enable_types_rs_sanitizer:
                        logger.info(f"bindgen 输出验证失败: {validation_msg}，尝试净化...")
                        modified, fix_count = self._sanitize_types_rs(output_path)
                        if modified:
                            logger.info(f"净化器修复了 {fix_count} 个问题，重新验证...")
                            is_valid, validation_msg = self._validate_types_rs(output_path)
                    else:
                        logger.info("types.rs 净化器已禁用（truth-mode 或未显式开启），保留原始 bindgen 输出")

                if is_valid:
                    report["mode"] = "bindgen"
                    report["success"] = True
                    report["final_output_valid"] = True
                    self._write_types_generation_report(report)
                    if attempt > 0:
                        print(f"✅ Bindgen successful after {attempt} retries!")
                        logger.info(f"bindgen 成功（经过 {attempt} 次重试）")
                    else:
                        print("✅ Bindgen successful!")
                    return True
                else:
                    logger.warning(f"bindgen 输出验证失败（净化后仍然失败）: {validation_msg}")
                    # 记录验证失败原因，便于后续定位“bindgen 成功但输出不可用”的场景
                    try:
                        report["attempts"][-1]["validation_error"] = validation_msg
                    except Exception:
                        pass
                    # 继续尝试，可能是输出不完整
                    continue

            # === 失败处理：分析错误日志，寻找缺失的头文件 ===
            # 注意：仅对 bindgen 进程本身失败（success=False）执行该逻辑。
            if success:
                continue
            if not missing_files:
                # 非头文件缺失问题
                logger.warning(f"bindgen 失败（非头文件缺失问题）: {error_msg[:300]}")
                break

            # 过滤掉已知无法解决的头文件
            new_missing = [h for h in missing_files if h not in unresolvable_headers]

            if not new_missing:
                logger.warning("所有缺失的头文件都已尝试查找但失败")
                break

            # 尝试通过全局数据库找到缺失的文件
            found_new_path = False

            if self.compile_commands_parser:
                try:
                    new_includes, still_missing, resolved_map = self.compile_commands_parser.get_resolved_includes_for_bindgen(
                        new_missing,
                        current_includes,
                        preferred_subpaths=preferred_subpaths,
                    )

                    added_count = len(new_includes) - len(current_includes)
                    if added_count > 0:
                        found_new_path = True
                        print(f"  ✨ Found {added_count} new include paths")
                        current_includes = new_includes

                    # 记录本次“缺失头文件 → include 路径”的解析结果，便于定位到底是路径没加上还是文件根本不存在
                    try:
                        report["attempts"][-1]["auto_resolve"] = {
                            "resolved": {k: v for k, v in (resolved_map or {}).items() if v},
                            "unresolved": list(still_missing),
                            "include_dirs_added": added_count,
                        }
                    except Exception:
                        pass

                    unresolvable_headers.update(still_missing)
                except Exception as e:
                    logger.warning(f"智能寻路失败: {e}")
                    unresolvable_headers.update(new_missing)
            else:
                unresolvable_headers.update(new_missing)

            if not found_new_path:
                print(f"  ⚠️ Attempt {attempt + 1}/{max_retries} failed, no new paths found")
                break

        # ========== Tier B: clang -E 预处理后再 bindgen ==========
        print("\n📋 Tier B: Clang Preprocessed Bindgen")
        tier_b_success = False
        if enable_tier_b:
            tier_b_success = self._attempt_clang_preprocessed_bindgen(
                header_files, output_path, current_includes, report, source_files=source_files
            )
        else:
            report["attempts"].append({
                "tier": "B",
                "success": False,
                "skipped": True,
                "error": "disabled (truth-mode)" if truth_mode else "disabled",
            })

        if tier_b_success:
            # 验证输出
            is_valid, validation_msg = self._validate_types_rs(output_path)

            # 如果验证失败，尝试净化
            if not is_valid:
                if enable_types_rs_sanitizer:
                    logger.info(f"Tier B 输出验证失败: {validation_msg}，尝试净化...")
                    modified, fix_count = self._sanitize_types_rs(output_path)
                    if modified:
                        logger.info(f"净化器修复了 {fix_count} 个问题，重新验证...")
                        is_valid, validation_msg = self._validate_types_rs(output_path)
                else:
                    logger.info("types.rs 净化器已禁用（truth-mode 或未显式开启），保留 Tier B 原始输出")

            if is_valid:
                report["mode"] = "clang_preprocessed_bindgen"
                report["success"] = True
                report["final_output_valid"] = True
                self._write_types_generation_report(report)
                print("✅ Tier B (clang -E + bindgen) successful!")
                return True
            else:
                logger.warning(f"Tier B 输出验证失败（净化后仍然失败）: {validation_msg}")

        # ========== Tier C: Stub types.rs (保证编译通过) ==========
        if not enable_tier_c_stub:
            report["mode"] = "bindgen_failed"
            report["success"] = False
            report["final_output_valid"] = False
            report["stub_disabled"] = True
            self._write_types_generation_report(report)
            print("✗ Bindgen failed and stub fallback is disabled; leaving types.rs unresolved.")
            return False

        print("\n📋 Tier C: Generating Stub types.rs (guaranteed compilation)")
        self._generate_stub_types_rs(output_path, header_files)

        # 验证 stub 输出
        is_valid, validation_msg = self._validate_types_rs(output_path)

        # 即使是 stub 也可能需要净化（例如头文件扫描时引入了问题类型）
        if not is_valid:
            if enable_types_rs_sanitizer:
                logger.info(f"Stub types.rs 验证失败: {validation_msg}，尝试净化...")
                modified, fix_count = self._sanitize_types_rs(output_path)
                if modified:
                    logger.info(f"净化器修复了 {fix_count} 个问题，重新验证...")
                    is_valid, validation_msg = self._validate_types_rs(output_path)
            else:
                logger.info("types.rs 净化器已禁用，保留 stub 原始输出")

        report["mode"] = "stub"
        report["success"] = is_valid
        report["final_output_valid"] = is_valid

        if not is_valid:
            logger.error(f"Stub types.rs 验证失败: {validation_msg}")
            report["stub_validation_error"] = validation_msg

        self._write_types_generation_report(report)

        if is_valid:
            print("✅ Stub types.rs generated (guaranteed compilation)")
        else:
            print("⚠️ Stub types.rs generated but validation failed")

        return is_valid

    def _validate_types_rs(self, types_path: Path) -> Tuple[bool, str]:
        """
        验证 types.rs 的语法正确性

        Phase 1 的关键函数：检测 bindgen 输出是否有效

        检查项：
        1. 文件存在且非空
        2. 没有明显的语法错误（unclosed delimiter）
        3. 没有递归类型别名（E0391）
        4. 没有重复定义（E0428）

        Args:
            types_path: types.rs 文件路径

        Returns:
            (是否有效, 错误原因)
        """
        if not types_path.exists():
            return False, "file does not exist"

        try:
            with open(types_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return False, f"cannot read file: {e}"

        if len(content.strip()) < 50:
            return False, "file is too small (possibly empty or failed generation)"

        # 检查递归类型别名（E0391）
        # 例如: pub type i32 = i32;
        recursive_pattern = re.compile(r'pub\s+type\s+(\w+)\s*=\s*\1\s*;')
        recursive_match = recursive_pattern.search(content)
        if recursive_match:
            return False, f"recursive type alias: {recursive_match.group(0)}"

        # 检查是否包含明显的错误模式
        error_patterns = [
            (r'error\[E\d+\]', "contains error message"),
            (r'^\s*\d+\s*\|', "contains line number (possibly error output)"),
            (r'cannot find type', "contains 'cannot find type' error"),
        ]

        for pattern, msg in error_patterns:
            if re.search(pattern, content, re.MULTILINE):
                return False, msg

        # 最后用 rustc 做一次语法/类型检查，避免“括号计数”这类容易被 doc 注释/字符串误伤的假阳性。
        # 这里不追求完全通过（重复定义等会在后续净化修复），但要能被 rustc 正常解析。
        try:
            import tempfile

            with tempfile.TemporaryDirectory(prefix="c2r_types_check_") as td:
                cmd = [
                    "rustc",
                    "--edition",
                    "2021",
                    "--crate-type",
                    "lib",
                    "--emit=metadata",
                    "--out-dir",
                    td,
                    str(types_path),
                ]
                result = subprocess.run(
                    cmd,
                    capture_output=True,
                    text=True,
                    timeout=60,
                )

            if result.returncode != 0:
                stderr = (result.stderr or "").strip()
                if not stderr:
                    return False, "rustc validation failed (no stderr)"
                # 只保留尾部，避免把巨量错误刷屏
                tail = "\n".join(stderr.splitlines()[-30:])
                return False, f"rustc validation failed:\n{tail}"
        except FileNotFoundError:
            # 极端环境下 rustc 不可用：回退为“尽力而为”的检查
            pass
        except subprocess.TimeoutExpired:
            return False, "rustc validation timeout"
        except Exception as e:
            return False, f"rustc validation error: {e}"

        return True, "ok"

    def _sanitize_types_rs(self, types_path: Path) -> Tuple[bool, int]:
        """
        Phase 2: types.rs 净化器

        主动修复 types.rs 中的常见问题，确保编译通过。

        修复项：
        1. E0428 - 去重（重复定义）
        2. E0391 - 移除递归类型别名（如 pub type i32 = i32;）
        3. E0740 - 处理 union Copy 问题（移除 union 的 Copy/Clone derive）
        4. 处理 Rust 保留字标识符（r#前缀或重命名）
        5. 移除无效的 extern crate 和 use 语句
        6. 修复常见的 bindgen 生成问题（含孤立 attributes）
        6.5. E0425 - 补充缺失的 glibc 内部类型定义（如 __suseconds_t, __blksize_t 等）

        Args:
            types_path: types.rs 文件路径

        Returns:
            (是否修改了文件, 修复数量)
        """
        if not types_path.exists():
            return False, 0

        try:
            with open(types_path, 'r', encoding='utf-8') as f:
                original_content = f.read()
        except Exception as e:
            logger.error(f"无法读取 types.rs: {e}")
            return False, 0

        content = original_content
        fixes_applied = 0

        # ========== 1. 移除递归类型别名 (E0391) ==========
        # 例如: pub type i32 = i32;
        recursive_pattern = re.compile(r'^\s*pub\s+type\s+(\w+)\s*=\s*\1\s*;\s*$', re.MULTILINE)
        matches = recursive_pattern.findall(content)
        if matches:
            content = recursive_pattern.sub('', content)
            fixes_applied += len(matches)
            logger.info(f"移除 {len(matches)} 个递归类型别名: {matches[:5]}")

        # ========== 2. 去重 (E0428) ==========
        # 按定义类型分别处理，保留第一次出现的定义
        lines = content.split('\n')
        seen_definitions = {}  # {(type, name): first_line_idx}
        lines_to_remove = set()

        # 定义类型的正则表达式
        def_patterns = [
            (r'^\s*pub\s+type\s+(\w+)', 'type'),
            (r'^\s*pub\s+struct\s+(\w+)', 'struct'),
            (r'^\s*pub\s+enum\s+(\w+)', 'enum'),
            (r'^\s*pub\s+union\s+(\w+)', 'union'),
            # NOTE: Require ':' to avoid matching `pub const fn ...` (bindgen emits const fns).
            (r'^\s*pub\s+const\s+([A-Za-z_]\w*)\s*:', 'const'),
            # Handle both `pub static NAME: ...` and `pub static mut NAME: ...` without capturing `mut` as name.
            (r'^\s*pub\s+static\s+(?:mut\s+)?([A-Za-z_]\w*)\s*:', 'static'),
        ]

        def _find_item_start(def_line_idx: int) -> int:
            """
            For duplicate items, also remove preceding contiguous attributes / doc comments.

            bindgen 生成的属性通常紧贴在 item 之前，例如：
              #[repr(C)]
              #[derive(Debug, Copy, Clone)]
              pub struct Foo
              {
                  ...
              }
            """
            start = def_line_idx
            while start > 0:
                prev = lines[start - 1]
                if re.match(r'^\s*#\s*\[', prev):
                    start -= 1
                    continue
                if re.match(r'^\s*///', prev) or re.match(r'^\s*//!', prev):
                    start -= 1
                    continue
                break
            return start

        def _find_item_end(def_line_idx: int, def_type: str) -> int:
            """
            Find the end line (inclusive) of a Rust item starting at def_line_idx.

            Handles cases where '{' is on the next line (bindgen sometimes emits):
              pub struct Foo
              {
                  ...
              }
            """
            # Single-line items (type alias / const / static) usually end with ';' on the same line.
            if def_type not in ("struct", "enum", "union"):
                return def_line_idx

            # Multi-line items: scan until we hit ';' (tuple/unit struct) OR braces are balanced.
            brace_count = 0
            saw_open = False
            end = def_line_idx
            while end < len(lines):
                line = lines[end]

                # Tuple / unit-like forms end with ';' without a block.
                if not saw_open and (";" in line) and ("{" not in line):
                    return end

                brace_count += line.count("{") - line.count("}")
                if "{" in line:
                    saw_open = True

                if saw_open and brace_count == 0:
                    return end

                end += 1

            return len(lines) - 1

        i = 0
        while i < len(lines):
            line = lines[i]
            for pattern, def_type in def_patterns:
                match = re.match(pattern, line)
                if match:
                    name = match.group(1)
                    key = (def_type, name)

                    if key in seen_definitions:
                        # 找到重复定义，需要移除
                        # 对于多行定义（struct, enum, union），需要找到完整的定义块
                        if def_type in ('struct', 'enum', 'union'):
                            start_idx = _find_item_start(i)
                            end_idx = _find_item_end(i, def_type)
                            for j in range(start_idx, end_idx + 1):
                                lines_to_remove.add(j)
                            i = end_idx  # 跳过整个块，避免重复扫描
                        else:
                            start_idx = _find_item_start(i)
                            end_idx = _find_item_end(i, def_type)
                            for j in range(start_idx, end_idx + 1):
                                lines_to_remove.add(j)

                        fixes_applied += 1
                        logger.debug(f"移除重复定义: {def_type} {name}")
                    else:
                        seen_definitions[key] = i
                    break
            i += 1

        if lines_to_remove:
            new_lines = [line for idx, line in enumerate(lines) if idx not in lines_to_remove]
            content = '\n'.join(new_lines)
            logger.info(f"移除 {len(lines_to_remove)} 行重复定义")

        # ========== 3. 处理 union Copy 问题 (E0740) ==========
        # 移除 union 定义上的 Copy 和 Clone derive
        # 匹配: #[derive(...Copy...)] 后面紧跟 pub union
        union_copy_pattern = re.compile(
            r'(#\[derive\([^)]*)\bCopy\b([^)]*)\]\s*\n(\s*pub\s+union\s+)',
            re.MULTILINE
        )

        def remove_copy_from_union(match):
            prefix = match.group(1)
            suffix = match.group(2)
            union_decl = match.group(3)

            # 移除 Copy 和可能的逗号
            derives = prefix + suffix
            # 清理逗号
            derives = re.sub(r',\s*,', ',', derives)
            derives = re.sub(r'\(\s*,', '(', derives)
            derives = re.sub(r',\s*\)', ')', derives)

            # 如果 derive 为空，移除整个 derive 属性
            if re.match(r'#\[derive\(\s*\)\]', derives + ')]'):
                return union_decl

            return derives + ')]\n' + union_decl

        new_content = union_copy_pattern.sub(remove_copy_from_union, content)
        if new_content != content:
            fixes_applied += content.count('pub union') - new_content.count('#[derive')  # 估算
            content = new_content
            logger.info("移除 union 定义上的 Copy derive")

        # 同样处理 Clone（通常 Copy 需要 Clone，但 Clone 单独存在于 union 也可能有问题）
        union_clone_pattern = re.compile(
            r'(#\[derive\([^)]*)\bClone\b([^)]*)\]\s*\n(\s*pub\s+union\s+)',
            re.MULTILINE
        )
        new_content = union_clone_pattern.sub(remove_copy_from_union, content)
        if new_content != content:
            content = new_content
            fixes_applied += 1

        # ========== 4. 处理 Rust 保留字标识符 ==========
        rust_keywords = {
            'as', 'break', 'const', 'continue', 'crate', 'else', 'enum',
            'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let',
            'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
            'self', 'Self', 'static', 'struct', 'super', 'trait', 'true',
            'type', 'unsafe', 'use', 'where', 'while', 'async', 'await',
            'dyn', 'abstract', 'become', 'box', 'do', 'final', 'macro',
            'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield', 'try'
        }

        # 查找用作字段名或参数名的保留字
        # 模式: 字段声明 pub name: Type
        field_pattern = re.compile(r'(\s+pub\s+)(' + '|'.join(rust_keywords) + r')(\s*:\s*)')

        def escape_keyword_field(match):
            prefix = match.group(1)
            keyword = match.group(2)
            suffix = match.group(3)
            return f'{prefix}r#{keyword}{suffix}'

        new_content = field_pattern.sub(escape_keyword_field, content)
        if new_content != content:
            diff_count = len(field_pattern.findall(content))
            fixes_applied += diff_count
            content = new_content
            logger.info(f"转义 {diff_count} 个保留字字段名")

        # ========== 5. 移除无效的 extern crate 语句 ==========
        # 移除 extern crate std; 这类在 2018 edition 中不需要的语句
        extern_crate_pattern = re.compile(r'^\s*extern\s+crate\s+std\s*;\s*$', re.MULTILINE)
        new_content = extern_crate_pattern.sub('', content)
        if new_content != content:
            fixes_applied += 1
            content = new_content

        # ========== 6. 修复常见的 bindgen 生成问题 ==========
        # 6.1 移除空的 impl 块
        empty_impl_pattern = re.compile(r'impl\s+\w+\s*\{\s*\}', re.MULTILINE)
        new_content = empty_impl_pattern.sub('', content)
        if new_content != content:
            fixes_applied += len(empty_impl_pattern.findall(content))
            content = new_content

        # 6.2 修复 bindgen 生成的无效 repr 属性
        # 例如: #[repr(C, packed(0))] -> #[repr(C, packed)]
        packed_zero_pattern = re.compile(r'packed\s*\(\s*0\s*\)')
        new_content = packed_zero_pattern.sub('packed', content)
        if new_content != content:
            fixes_applied += len(packed_zero_pattern.findall(content))
            content = new_content

        # 6.3 移除无效的 #[link] 属性（针对不存在的库）
        # 这个比较危险，只移除明显有问题的
        invalid_link_pattern = re.compile(r'#\[link\s*\(\s*name\s*=\s*""\s*\)\]', re.MULTILINE)
        new_content = invalid_link_pattern.sub('', content)
        if new_content != content:
            fixes_applied += len(invalid_link_pattern.findall(content))
            content = new_content

        # 6.4 移除“孤立 attributes”（常见报错：expected item after attributes）
        # 当 types.rs 末尾或某处出现 #[...] 但后面没有任何 item，会导致语法错误。
        # 典型场景：bindgen/后处理过程中截断或移除了 item，但遗留了属性行。
        lines = content.split('\n')
        new_lines: List[str] = []
        i = 0
        removed_attrs = 0

        item_start_pattern = re.compile(
            r'^\s*(?:pub\s+)?(?:'
            r'struct|enum|union|type|const|static|fn|extern|mod|use|trait|impl|macro_rules!'
            r')\b'
        )

        while i < len(lines):
            line = lines[i]
            if line.lstrip().startswith('#['):
                # 收集连续的属性行
                attr_block = []
                while i < len(lines) and lines[i].lstrip().startswith('#['):
                    attr_block.append(lines[i])
                    i += 1

                # 找到下一个非空行
                j = i
                while j < len(lines) and lines[j].strip() == '':
                    j += 1

                # 文件结束：丢弃属性块
                if j >= len(lines):
                    removed_attrs += len(attr_block)
                    fixes_applied += len(attr_block)
                    continue

                next_line = lines[j].lstrip()

                # 下一行是 item / doc comment / 另一个属性：保留
                if next_line.startswith('#[') or next_line.startswith('///') or next_line.startswith('//!') or item_start_pattern.match(lines[j]):
                    new_lines.extend(attr_block)
                else:
                    # 孤立属性：丢弃
                    removed_attrs += len(attr_block)
                    fixes_applied += len(attr_block)
                continue

            new_lines.append(line)
            i += 1

        if removed_attrs:
            content = '\n'.join(new_lines)
            logger.info(f"移除 {removed_attrs} 行孤立 attributes")

        # ========== 6.5 修复 E0425: 补充缺失的 glibc 内部类型 ==========
        # bindgen 经常生成引用 glibc 内部类型的别名，如:
        #   pub type suseconds_t = __suseconds_t;
        # 但 __suseconds_t 没有定义，导致 E0425 错误。
        # 这里自动补充这些已知的 glibc 内部类型定义。

        # 已知的 glibc 内部类型映射 (Linux x86_64)
        glibc_internal_types = {
            # 时间相关类型
            "__suseconds_t": "i64",
            "__time_t": "i64",
            "__clock_t": "i64",
            "__clockid_t": "i32",
            "__timer_t": "*mut ::core::ffi::c_void",

            # 文件系统相关类型
            "__blksize_t": "i64",
            "__blkcnt_t": "i64",
            "__blkcnt64_t": "i64",
            "__fsblkcnt_t": "u64",
            "__fsblkcnt64_t": "u64",
            "__fsfilcnt_t": "u64",
            "__fsfilcnt64_t": "u64",
            "__fsword_t": "i64",
            "__off_t": "i64",
            "__off64_t": "i64",
            "__loff_t": "i64",
            "__ino_t": "u64",
            "__ino64_t": "u64",
            "__nlink_t": "u64",
            "__dev_t": "u64",
            "__mode_t": "u32",

            # 进程/用户相关类型
            "__pid_t": "i32",
            "__uid_t": "u32",
            "__gid_t": "u32",
            "__id_t": "u32",
            "__key_t": "i32",

            # 整数类型
            "__int8_t": "i8",
            "__int16_t": "i16",
            "__int32_t": "i32",
            "__int64_t": "i64",
            "__uint8_t": "u8",
            "__uint16_t": "u16",
            "__uint32_t": "u32",
            "__uint64_t": "u64",
            "__int_least8_t": "i8",
            "__int_least16_t": "i16",
            "__int_least32_t": "i32",
            "__int_least64_t": "i64",
            "__uint_least8_t": "u8",
            "__uint_least16_t": "u16",
            "__uint_least32_t": "u32",
            "__uint_least64_t": "u64",
            "__int_fast8_t": "i8",
            "__int_fast16_t": "i64",
            "__int_fast32_t": "i64",
            "__int_fast64_t": "i64",
            "__uint_fast8_t": "u8",
            "__uint_fast16_t": "u64",
            "__uint_fast32_t": "u64",
            "__uint_fast64_t": "u64",
            "__intmax_t": "i64",
            "__uintmax_t": "u64",

            # 指针相关类型
            "__intptr_t": "isize",
            "__uintptr_t": "usize",
            "__ssize_t": "isize",
            "__syscall_slong_t": "i64",
            "__syscall_ulong_t": "u64",

            # socket 相关类型
            "__socklen_t": "u32",
            "__sig_atomic_t": "i32",

            # 其他常见类型
            "__caddr_t": "*mut i8",
            "__daddr_t": "i32",
            "__swblk_t": "i64",
            "__quad_t": "i64",
            "__u_quad_t": "u64",
            "__qaddr_t": "*mut i64",
            "__rlim_t": "u64",
            "__rlim64_t": "u64",
            "__useconds_t": "u32",
            "__wchar_t": "i32",
            "__wint_t": "u32",
        }

        # 6.5.1 复杂的 glibc 内部结构体类型（需要生成 opaque struct）
        # 这些类型是结构体而不是简单的 typedef，需要生成 opaque 定义
        glibc_internal_structs = {
            # FILE 和相关类型
            "__fpos_t": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __fpos_t { _opaque: [u8; 16] }",
            "__fpos64_t": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __fpos64_t { _opaque: [u8; 16] }",
            "__mbstate_t": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __mbstate_t { _opaque: [u8; 8] }",

            # locale 相关
            "__locale_struct": "#[repr(C)]\npub struct __locale_struct { _opaque: [u8; 232] }",

            # va_list 相关
            "__va_list_tag": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __va_list_tag { pub gp_offset: u32, pub fp_offset: u32, pub overflow_arg_area: *mut ::core::ffi::c_void, pub reg_save_area: *mut ::core::ffi::c_void }",

            # pthread 相关
            "__pthread_mutex_s": "#[repr(C)]\npub struct __pthread_mutex_s { _opaque: [u8; 40] }",
            "__pthread_cond_s": "#[repr(C)]\npub struct __pthread_cond_s { _opaque: [u8; 48] }",
            "__pthread_rwlock_arch_t": "#[repr(C)]\npub struct __pthread_rwlock_arch_t { _opaque: [u8; 56] }",
            "__pthread_internal_list": "#[repr(C)]\npub struct __pthread_internal_list { pub __prev: *mut __pthread_internal_list, pub __next: *mut __pthread_internal_list }",
            "__pthread_internal_slist": "#[repr(C)]\npub struct __pthread_internal_slist { pub __next: *mut __pthread_internal_slist }",
            "__pthread_list_t": "pub type __pthread_list_t = __pthread_internal_list;",
            "__pthread_slist_t": "pub type __pthread_slist_t = __pthread_internal_slist;",

            # 目录相关
            "__dirstream": "#[repr(C)]\npub struct __dirstream { _opaque: [u8; 0] }",

            # sigset 相关
            "__sigset_t": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __sigset_t { pub __val: [u64; 16] }",

            # 文件系统相关
            "__fsid_t": "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct __fsid_t { pub __val: [i32; 2] }",

            # 其他
            "__once_flag": "#[repr(C)]\npub struct __once_flag { __data: i32 }",
        }

        # 1. 找出所有已定义的类型
        defined_types = set()
        type_def_pattern = re.compile(r'^\s*pub\s+(?:type|struct|enum|union)\s+(\w+)', re.MULTILINE)
        for match in type_def_pattern.finditer(content):
            defined_types.add(match.group(1))

        # 1.1 也检查 pub use 导入的类型（避免 E0255 重复定义）
        # 匹配: pub use module::{Type1, Type2, Type3};
        # 或: pub use module::*;
        pub_use_pattern = re.compile(r'pub\s+use\s+\w+::\{([^}]+)\}')
        for match in pub_use_pattern.finditer(content):
            imported_types = match.group(1)
            # 解析逗号分隔的类型名
            for type_name in imported_types.split(','):
                type_name = type_name.strip()
                # 处理 as 别名: Type as Alias
                if ' as ' in type_name:
                    type_name = type_name.split(' as ')[0].strip()
                if type_name:
                    defined_types.add(type_name)

        # 1.2 检查 pub use module::* 的情况 - 需要检查被包含的模块
        # 如果有 include! 宏，尝试读取并解析其中定义的类型
        include_pattern = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')
        for match in include_pattern.finditer(content):
            include_path = match.group(1)
            # 相对于 types.rs 所在目录解析路径
            types_dir = types_path.parent
            full_include_path = types_dir / include_path
            if full_include_path.exists():
                try:
                    with open(full_include_path, 'r', encoding='utf-8') as f:
                        include_content = f.read()
                    # 从包含的文件中提取类型定义
                    for def_match in type_def_pattern.finditer(include_content):
                        defined_types.add(def_match.group(1))
                except Exception:
                    pass  # 忽略读取错误

        # 2. 找出所有被引用但可能未定义的类型
        # 2.1 简单类型别名 (如 __suseconds_t)
        referenced_types = set()
        ref_pattern = re.compile(r'[=:,\s\(](__[a-z_]+_t)\b')
        for match in ref_pattern.finditer(content):
            type_name = match.group(1)
            if type_name not in defined_types and type_name in glibc_internal_types:
                referenced_types.add(type_name)

        # 2.2 复杂结构体类型 (如 __fpos_t, __locale_struct, __va_list_tag)
        # 使用更宽松的模式匹配，包括 __xxx_t 和 __xxx_struct 以及 __xxx_tag
        # 添加 \[ 以匹配数组中的类型，如 [__va_list_tag; 1usize]
        referenced_structs = set()
        struct_ref_pattern = re.compile(r'[=:,\s\(\*\[](__[a-z_]+(?:_t|_struct|_tag|_s|_list))\b')
        for match in struct_ref_pattern.finditer(content):
            type_name = match.group(1)
            if type_name not in defined_types and type_name in glibc_internal_structs:
                referenced_structs.add(type_name)

        # 3. 生成缺失类型的定义
        if referenced_types or referenced_structs:
            missing_defs = []
            missing_defs.append("")
            missing_defs.append("// ============================================================")
            missing_defs.append("// Auto-generated glibc internal type definitions (E0425 fix)")
            missing_defs.append("// ============================================================")

            # 3.1 先添加结构体定义（它们可能被类型别名引用）
            for type_name in sorted(referenced_structs):
                struct_def = glibc_internal_structs[type_name]
                missing_defs.append(struct_def)

            # 3.2 再添加类型别名定义
            for type_name in sorted(referenced_types):
                rust_type = glibc_internal_types[type_name]
                missing_defs.append(f"pub type {type_name} = {rust_type};")

            missing_defs.append("")

            # 4. 将定义插入到文件开头（在 #![allow(...)] 之后）
            # 找到第一个非属性、非注释的位置
            lines = content.split('\n')
            insert_idx = 0
            for idx, line in enumerate(lines):
                stripped = line.strip()
                if stripped.startswith('#![') or stripped.startswith('//!') or stripped == '':
                    insert_idx = idx + 1
                elif stripped.startswith('//'):
                    insert_idx = idx + 1
                else:
                    break

            # 插入缺失的类型定义
            new_lines = lines[:insert_idx] + missing_defs + lines[insert_idx:]
            content = '\n'.join(new_lines)
            total_added = len(referenced_types) + len(referenced_structs)
            fixes_applied += total_added
            all_types = sorted(list(referenced_types) + list(referenced_structs))
            logger.info(f"补充 {total_added} 个缺失的 glibc 内部类型: {all_types[:5]}...")

        # ========== 7. 清理多余空行 ==========
        # 连续超过2个空行压缩为2个
        content = re.sub(r'\n{4,}', '\n\n\n', content)

        # ========== 8. 确保文件末尾有换行 ==========
        if not content.endswith('\n'):
            content += '\n'

        # 检查是否有修改
        if content != original_content:
            try:
                with open(types_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                logger.info(f"types.rs 净化完成，共修复 {fixes_applied} 个问题")
                return True, fixes_applied
            except Exception as e:
                logger.error(f"写入净化后的 types.rs 失败: {e}")
                return False, 0

        return False, fixes_applied

    def _attempt_clang_preprocessed_bindgen(
        self,
        header_files: List[Path],
        output_path: Path,
        include_dirs: Set[Path],
        report: dict,
        source_files: List[Path] = None,
    ) -> bool:
        """
        Tier B: 使用 clang -E 预处理后再运行 bindgen

        策略：
        1. 合并所有头文件内容
        2. 使用 clang -E 预处理（展开宏、处理条件编译）
        3. 对预处理后的输出运行 bindgen

        Args:
            header_files: 头文件列表
            output_path: 输出路径
            include_dirs: include 路径集合
            report: 报告数据

        Returns:
            是否成功
        """
        try:
            keep_files = self._env_flag("C2R_BINDGEN_DEBUG_KEEP_FILES")

            # include 目录诊断（帮助定位 out/.../gen 缺失、目录不存在等）
            include_dirs_list = sorted(str(p) for p in (include_dirs or set()))
            nonexistent_dirs = [p for p in include_dirs_list if p and not Path(p).exists()]
            out_gen_dirs = [p for p in include_dirs_list if "/out/" in p.replace("\\", "/") and "/gen" in p.replace("\\", "/")]
            include_diag = {
                "include_dirs_count": len(include_dirs_list),
                "nonexistent_dir_count": len(nonexistent_dirs),
                "nonexistent_dir_sample": nonexistent_dirs[:20],
                "out_gen_dir_count": len(out_gen_dirs),
                "out_gen_dir_existing_count": sum(1 for p in out_gen_dirs if Path(p).exists()),
            }

            wrapper_path = self.output_dir / "wrapper_for_clang.h"
            wrapper_extra_system_includes: List[str] = []
            wrapper_type_shims: List[str] = []
            wrapper_enum_shims: List[str] = []
            wrapper_rename_dprintf: bool = False
            wrapper_shimmed_type_names: Set[str] = set()
            wrapper_shimmed_enum_names: Set[str] = set()
            wrapper_fixup_actions: List[Dict[str, Any]] = []

            def _write_wrapper_for_clang():
                # 创建临时的合并头文件（保持简单：不要用 extern \"C\" 包裹，避免 C++ 头被强行 C linkage）
                wrapper_content: List[str] = []
                wrapper_content.append("// Auto-generated wrapper for clang -E preprocessing")
                wrapper_content.append("")
                # musl's bits/alltypes.h uses __NEED_* gating. Keep only safe defaults here:
                # - For sched_param, prefer including <sched.h> (avoid defining __NEED_struct_sched_param manually).
                wrapper_content.append("#ifndef __NEED_struct_cpu_set_t")
                wrapper_content.append("#define __NEED_struct_cpu_set_t 1")
                wrapper_content.append("#endif")
                wrapper_content.append("")
                wrapper_content.append("#include <sched.h>")
                wrapper_content.append("")
                # Linux/UAPI-style annotation macros: make them parseable outside full kernel build context.
                wrapper_content.append("#ifndef __must_check")
                wrapper_content.append("#define __must_check __attribute__((warn_unused_result))")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef __packed")
                wrapper_content.append("#define __packed __attribute__((packed))")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef __aligned")
                wrapper_content.append("#define __aligned(x) __attribute__((aligned(x)))")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef __user")
                wrapper_content.append("#define __user")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef __force")
                wrapper_content.append("#define __force")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef __iomem")
                wrapper_content.append("#define __iomem")
                wrapper_content.append("#endif")
                wrapper_content.append("")
                # Common storage specifier macros in some vendor codebases.
                wrapper_content.append("#ifndef STATIC")
                wrapper_content.append("#define STATIC static")
                wrapper_content.append("#endif")
                wrapper_content.append("#ifndef INLINE")
                wrapper_content.append("#define INLINE inline")
                wrapper_content.append("#endif")
                wrapper_content.append("")

                # Provide common fundamental typedefs early.
                # Some OpenHarmony headers assume these are already available (via transitive includes in real TUs).
                wrapper_content.append("#include <stddef.h>")
                wrapper_content.append("#include <stdint.h>")
                wrapper_content.append("#include <stdbool.h>")
                wrapper_content.append("")

                # If we need to disambiguate libc-vs-LiteOS symbol prototypes (e.g., dprintf),
                # include the libc header first so its prototype wins, then rename the LiteOS one.
                if wrapper_rename_dprintf:
                    wrapper_content.append("#include <stdio.h>")
                    wrapper_content.append("")
                    wrapper_content.append("// Avoid signature conflict between musl's dprintf and LiteOS los_printf.h dprintf")
                    wrapper_content.append("#ifdef dprintf")
                    wrapper_content.append("#undef dprintf")
                    wrapper_content.append("#endif")
                    wrapper_content.append("#define dprintf c2r_liteos_dprintf")
                    wrapper_content.append("")

                if wrapper_type_shims or wrapper_enum_shims:
                    wrapper_content.append("// Type/enum shims (auto-fix for unknown/incomplete types)")
                    wrapper_content.extend(wrapper_type_shims)
                    wrapper_content.extend(wrapper_enum_shims)
                    wrapper_content.append("")

                if wrapper_extra_system_includes:
                    wrapper_content.append("// Extra system includes (auto-fix for unknown types)")
                    for inc in wrapper_extra_system_includes:
                        wrapper_content.append(f"#include {inc}")
                    wrapper_content.append("")

                for h in header_files:
                    if h.exists():
                        wrapper_content.append(f'#include "{h}"')

                wrapper_content.append("")

                if wrapper_rename_dprintf:
                    wrapper_content.append("#ifdef dprintf")
                    wrapper_content.append("#undef dprintf")
                    wrapper_content.append("#endif")
                    wrapper_content.append("")

                with open(wrapper_path, 'w', encoding='utf-8') as f:
                    f.write('\n'.join(wrapper_content))

            _write_wrapper_for_clang()

            # 选择 clang（优先 OpenHarmony 预置 clang，避免宿主机 toolchain 与 --target 不匹配）
            clang_bin = shutil.which("clang") or "clang"
            if self.ohos_root:
                ohos_clang = self.ohos_root / "prebuilts" / "clang" / "ohos" / "linux-x86_64" / "llvm" / "bin" / "clang"
                if ohos_clang.exists():
                    clang_bin = str(ohos_clang)

            # Build ordered clang context flags (prefer a representative TU's compile_commands entry).
            context_flags: List[str] = []
            added_dirs: Set[str] = set()
            cc_context_used = False

            rep_src: Optional[Path] = None
            if source_files:
                for src in source_files:
                    p = Path(src)
                    if p.exists():
                        rep_src = p
                        break

            prefer_cxx = False
            if rep_src and rep_src.suffix.lower() in {".cc", ".cpp", ".cxx", ".c++"}:
                prefer_cxx = True

            tu_ctx: Dict[str, Any] = {}
            if self.compile_commands_parser and rep_src:
                mapped_src = self._map_to_ohos_path(rep_src)
                tu_ctx.update({
                    "rep_src": str(rep_src),
                    "mapped_src": str(mapped_src),
                    "used_proxy": False,
                })
                try:
                    # Prefer stage1-selected TU context (pins flags/macros/include order).
                    entry: Optional[Dict[str, Any]] = None
                    match_info: Dict[str, Any] = {}
                    try:
                        safe_name = self._get_safe_module_name(rep_src)
                        rec = self._tu_context_files.get(safe_name) if getattr(self, "_tu_context_files", None) else None
                        if isinstance(rec, dict) and isinstance(rec.get("compile_commands_entry"), dict):
                            entry = rec.get("compile_commands_entry")
                            match_info = {
                                "reason": "tu_context_map",
                                "tu_context_map": str(getattr(self, "_tu_context_map_path", "")) if getattr(self, "_tu_context_map_path", None) else None,
                                "entry_hash": rec.get("entry_hash"),
                            }
                            tu_ctx["tu_context_map_used"] = True
                            tu_ctx["tu_context_safe_name"] = safe_name
                    except Exception:
                        entry = None
                        match_info = {}

                    if not entry:
                        entry, match_info = self.compile_commands_parser.get_entry_for_file_with_reason(mapped_src)
                    tu_ctx["match_info"] = match_info
                    tu_ctx["entry_found"] = bool(entry)
                    clang_flags: List[str] = []
                    if entry:
                        clang_flags = self.compile_commands_parser.get_clang_flags_for_entry(entry, normalize_paths=True)

                    # Proxy fallback if the module isn't compiled under this profile (common in SelfContained subsets).
                    # Default OFF: when the compile_commands closure is incomplete, treat it as an input issue and report it.
                    enable_proxy = os.environ.get("C2R_ENABLE_PROXY_TU_FALLBACK", "0").strip().lower() in ("1", "true", "yes")
                    if enable_proxy and (not clang_flags) and self._ohos_project_rel and len(self._ohos_project_rel.parts) >= 2:
                        max_parts = min(5, len(self._ohos_project_rel.parts))
                        proxy_file: Optional[Path] = None
                        proxy_key: Optional[str] = None
                        for n in range(max_parts, 1, -1):
                            candidate_key = str(Path(*self._ohos_project_rel.parts[:n]))
                            candidate_file = self.compile_commands_parser.find_first_source_file_containing(candidate_key)
                            if candidate_file:
                                proxy_key = candidate_key
                                proxy_file = candidate_file
                                break
                        if proxy_file and proxy_key:
                            tu_ctx["proxy_key"] = proxy_key
                            tu_ctx["proxy_file"] = str(proxy_file)
                            p_entry, p_match_info = self.compile_commands_parser.get_entry_for_file_with_reason(proxy_file)
                            tu_ctx["proxy_match_info"] = p_match_info
                            tu_ctx["proxy_entry_found"] = bool(p_entry)
                            if p_entry:
                                clang_flags = self.compile_commands_parser.get_clang_flags_for_entry(p_entry, normalize_paths=True)
                                tu_ctx["used_proxy"] = True

                    if clang_flags:
                        context_flags.extend(clang_flags)
                        # Track include dirs already present (avoid duplicates).
                        j = 0
                        while j < len(clang_flags):
                            flag = clang_flags[j]
                            if flag == "-I" and j + 1 < len(clang_flags):
                                added_dirs.add(clang_flags[j + 1])
                                j += 2
                                continue
                            if flag == "-isystem" and j + 1 < len(clang_flags):
                                added_dirs.add(clang_flags[j + 1])
                                j += 2
                                continue
                            if flag.startswith("-I") and flag != "-I":
                                added_dirs.add(flag[2:])
                            j += 1
                except Exception as e:
                    tu_ctx["error"] = str(e)[:300]

            # Add discovered include dirs as a supplement (do not disturb ordering of TU flags).
            for inc_dir in sorted(include_dirs or set(), key=lambda p: str(p)):
                inc_dir_str = str(inc_dir)
                if inc_dir_str not in added_dirs:
                    context_flags.extend(["-I", inc_dir_str])
                    added_dirs.add(inc_dir_str)

            # Add header dirs as a last-resort fallback.
            for h in sorted(header_files, key=lambda p: str(p)):
                h_dir_str = str(Path(h).parent)
                if h_dir_str not in added_dirs:
                    context_flags.extend(["-I", h_dir_str])
                    added_dirs.add(h_dir_str)

            def _make_include_diag(dirs: Set[Path]) -> Dict[str, Any]:
                include_dirs_list = sorted(str(p) for p in (dirs or set()))
                nonexistent_dirs = [p for p in include_dirs_list if p and not Path(p).exists()]
                out_gen_dirs = [p for p in include_dirs_list if "/out/" in p.replace("\\", "/") and "/gen" in p.replace("\\", "/")]
                return {
                    "include_dirs_count": len(include_dirs_list),
                    "nonexistent_dir_count": len(nonexistent_dirs),
                    "nonexistent_dir_sample": nonexistent_dirs[:20],
                    "out_gen_dir_count": len(out_gen_dirs),
                    "out_gen_dir_existing_count": sum(1 for p in out_gen_dirs if Path(p).exists()),
                }

            def _extract_missing_headers(stderr_text: str) -> List[str]:
                if not stderr_text:
                    return []
                missing = re.findall(r"'([^']+)' file not found", stderr_text)
                missing += re.findall(r"fatal error:\s*([^\s:]+)\s*file not found", stderr_text)
                if not missing:
                    return []
                seen = set()
                out: List[str] = []
                for f in missing:
                    f = (f or "").strip().strip('"').strip("'").strip("<>").strip()
                    if f and f not in seen:
                        seen.add(f)
                        out.append(f)
                return out

            # Hint subpaths (relative to OHOS root) for smarter missing-header resolution.
            preferred_subpaths: Optional[List[str]] = None
            try:
                if self._ohos_project_rel:
                    parts = list(self._ohos_project_rel.parts)
                    max_parts = min(len(parts), 6)
                    preferred_subpaths = [str(Path(*parts[:n])) for n in range(max_parts, 1, -1)]
            except Exception:
                preferred_subpaths = None

            # Build clang -E command (try preferred language mode first, then fallback if needed).
            # If clang -E fails due to missing headers, try to auto-resolve include dirs and retry a few times.
            try:
                max_preprocess_fixups = int(os.environ.get("C2R_CLANG_PREPROCESS_MAX_RETRIES", "5"))
                if max_preprocess_fixups < 0:
                    max_preprocess_fixups = 0
            except Exception:
                max_preprocess_fixups = 5
            # semantics: allow N fixups + 1 final attempt to validate the last added include dir
            max_preprocess_attempts = max_preprocess_fixups + 1

            # Tier-B unknown-type fixups: after preprocessing succeeds, bindgen can still fail on
            # "unknown type name ..." (not a missing file). In that case we can inject additional
            # system headers into the wrapper and redo clang -E + bindgen.
            try:
                max_unknown_fixups = int(os.environ.get("C2R_TIER_B_UNKNOWN_TYPE_FIXUPS", "2"))
                if max_unknown_fixups < 0:
                    max_unknown_fixups = 0
            except Exception:
                max_unknown_fixups = 2

            def _extract_unknown_types(stderr_text: str) -> List[str]:
                if not stderr_text:
                    return []
                names = re.findall(r"unknown type name '([^']+)'", stderr_text)
                # de-dupe in order
                seen = set()
                out: List[str] = []
                for n in names:
                    n = (n or "").strip()
                    if n and n not in seen:
                        seen.add(n)
                        out.append(n)
                return out

            def _extract_incomplete_enum_types(stderr_text: str) -> List[str]:
                if not stderr_text:
                    return []
                # Example: field has incomplete type 'enum dma_data_direction'
                names = re.findall(r"field has incomplete type 'enum\s+([^']+)'", stderr_text)
                # de-dupe in order
                seen = set()
                out: List[str] = []
                for n in names:
                    n = (n or "").strip()
                    if n and n not in seen:
                        seen.add(n)
                        out.append(n)
                return out

            def _ensure_shim_line(line: str):
                if not line:
                    return
                if line not in wrapper_type_shims and line not in wrapper_enum_shims:
                    # Keep ordering stable, just append.
                    wrapper_type_shims.append(line)

            def _add_type_shims(type_names: List[str]) -> bool:
                if not type_names:
                    return False

                added_any = False

                def _add_shim_for(name: str, lines: List[str], kind: str = "type"):
                    nonlocal added_any
                    if name in wrapper_shimmed_type_names:
                        return
                    for ln in lines:
                        if ln:
                            wrapper_type_shims.append(ln)
                    wrapper_shimmed_type_names.add(name)
                    wrapper_fixup_actions.append({
                        "action": "add_type_shim",
                        "type": name,
                        "lines": lines,
                        "reason": "tier_b_unknown_type_name",
                        "kind": kind,
                    })
                    added_any = True

                # Kernel-style integer typedefs (common in linux/uapi and kernel headers).
                for t in type_names:
                    t = (t or "").strip()
                    if not t:
                        continue

                    if t == "wait_queue_head_t":
                        # Prefer including linuxkpi's <linux/wait.h> (LiteOS) to avoid defining a wrong
                        # placeholder that later conflicts with the real typedef.
                        inc = "<linux/wait.h>"
                        try:
                            if self.ohos_root and Path(self.ohos_root).exists():
                                ohos_root = Path(self.ohos_root)
                                linuxkpi_wait = (
                                    ohos_root
                                    / "third_party"
                                    / "FreeBSD"
                                    / "sys"
                                    / "compat"
                                    / "linuxkpi"
                                    / "common"
                                    / "include"
                                    / "linux"
                                    / "wait.h"
                                )
                                if linuxkpi_wait.exists():
                                    inc = f"\"{linuxkpi_wait}\""
                        except Exception:
                            inc = "<linux/wait.h>"

                        if inc and inc not in wrapper_extra_system_includes:
                            wrapper_extra_system_includes.append(inc)
                            wrapper_fixup_actions.append({
                                "action": "add_system_include",
                                "include": inc,
                                "type": t,
                                "reason": "tier_b_unknown_type_name",
                                "kind": "kernel",
                            })
                            added_any = True
                        continue

                    # __u64/__s64/__u32/... and __le64/__be64...
                    m = re.fullmatch(r"__(u|s)(8|16|32|64)", t)
                    if m:
                        sign, bits = m.group(1), int(m.group(2))
                        c_type = {
                            (False, 8): "unsigned char",
                            (False, 16): "unsigned short",
                            (False, 32): "unsigned int",
                            (False, 64): "unsigned long long",
                            (True, 8): "signed char",
                            (True, 16): "short",
                            (True, 32): "int",
                            (True, 64): "long long",
                        }[(sign == "s", bits)]
                        _add_shim_for(t, [f"typedef {c_type} {t};"])
                        continue

                    m = re.fullmatch(r"__([bl]e)(16|32|64)", t)
                    if m:
                        bits = int(m.group(2))
                        c_type = {
                            16: "unsigned short",
                            32: "unsigned int",
                            64: "unsigned long long",
                        }[bits]
                        _add_shim_for(t, [f"typedef {c_type} {t};"])
                        continue

                    # Kernel short aliases: u8/u16/u32/u64 and s8/...
                    m = re.fullmatch(r"(u|s)(8|16|32|64)", t)
                    if m:
                        sign, bits = m.group(1), int(m.group(2))
                        c_type = {
                            (False, 8): "uint8_t",
                            (False, 16): "uint16_t",
                            (False, 32): "uint32_t",
                            (False, 64): "uint64_t",
                            (True, 8): "int8_t",
                            (True, 16): "int16_t",
                            (True, 32): "int32_t",
                            (True, 64): "int64_t",
                        }[(sign == "s", bits)]
                        _add_shim_for(t, [f"typedef {c_type} {t};"])
                        continue

                    if t == "gfp_t":
                        _add_shim_for("gfp_t", ["typedef unsigned int gfp_t;"])
                        continue

                    if t == "refcount_t":
                        # refcount_t is a kernel-internal typedef with heavy transitive includes.
                        # Provide a minimal placeholder and prevent linux/refcount.h from redefining it.
                        _add_shim_for(
                            "refcount_t",
                            [
                                "#ifndef _LINUX_REFCOUNT_H",
                                "#define _LINUX_REFCOUNT_H 1",
                                "#endif",
                                "typedef struct refcount_struct { int __c2r_dummy; } refcount_t;",
                            ],
                            kind="kernel",
                        )
                        continue

                    if t == "poll_table":
                        _add_shim_for(
                            "poll_table",
                            ["typedef struct poll_table_struct { int __c2r_dummy; } poll_table;"],
                            kind="kernel",
                        )
                        continue

                if added_any:
                    _write_wrapper_for_clang()
                return added_any

            def _add_enum_shims(enum_names: List[str]) -> bool:
                if not enum_names:
                    return False
                added_any = False

                for e in enum_names:
                    e = (e or "").strip()
                    if not e:
                        continue
                    if e in wrapper_shimmed_enum_names:
                        continue
                    if e == "dma_data_direction":
                        lines = [
                            "enum dma_data_direction {",
                            "  DMA_BIDIRECTIONAL = 0,",
                            "  DMA_TO_DEVICE = 1,",
                            "  DMA_FROM_DEVICE = 2,",
                            "  DMA_NONE = 3,",
                            "};",
                        ]
                        wrapper_enum_shims.extend(lines)
                        wrapper_shimmed_enum_names.add(e)
                        wrapper_fixup_actions.append({
                            "action": "add_enum_shim",
                            "enum": e,
                            "lines": lines,
                            "reason": "tier_b_incomplete_enum",
                            "kind": "kernel",
                        })
                        added_any = True

                if added_any:
                    _write_wrapper_for_clang()
                return added_any

            def _maybe_enable_dprintf_rename(stderr_text: str) -> bool:
                nonlocal wrapper_rename_dprintf
                if wrapper_rename_dprintf:
                    return False
                if "conflicting types for 'dprintf'" not in (stderr_text or ""):
                    return False
                wrapper_rename_dprintf = True
                wrapper_fixup_actions.append({
                    "action": "rename_conflicting_symbol",
                    "symbol": "dprintf",
                    "rename_to": "c2r_liteos_dprintf",
                    "reason": "tier_b_conflicting_types",
                })
                _write_wrapper_for_clang()
                return True

            # Resolve bindgen binary once.
            bindgen_path = shutil.which("bindgen")
            if not bindgen_path:
                cargo_bin = Path.home() / ".cargo" / "bin" / "bindgen"
                if cargo_bin.exists():
                    bindgen_path = str(cargo_bin)
            if not bindgen_path:
                logger.error("bindgen 未安装")
                report["attempts"].append({
                    "tier": "B",
                    "step": "bindgen_on_preprocessed",
                    "success": False,
                    "error": "bindgen not installed",
                    "debug": include_diag,
                })
                return False

            # Since the input is already preprocessed, keep only the minimal flags that influence target ABI.
            minimal_flags: List[str] = []
            i = 0
            while i < len(context_flags):
                f = context_flags[i]
                if isinstance(f, str) and (f.startswith("--target=") or f.startswith("--sysroot=") or f in {"-mthumb", "-marm"}):
                    minimal_flags.append(f)
                    i += 1
                    continue
                if f == "-target" and i + 1 < len(context_flags):
                    minimal_flags.extend([f, context_flags[i + 1]])
                    i += 2
                    continue
                if f == "-isysroot" and i + 1 < len(context_flags):
                    minimal_flags.extend([f, context_flags[i + 1]])
                    i += 2
                    continue
                if isinstance(f, str) and f.startswith((
                    "-march=", "-mcpu=", "-mfpu=", "-mfloat-abi=", "-mtune=", "-mabi=",
                    "-mno-", "-mno_", "-msoft-float", "-mhard-float",
                )):
                    minimal_flags.append(f)
                    i += 1
                    continue
                i += 1

            clang_common_flags = [
                "-Wno-error",
                "-Wno-error=register",
                "-Wno-incompatible-library-redeclaration",
                "-Wno-shift-op-parentheses",
                "-Wno-unused-function",
                "-Wno-unused-variable",
                "-Wno-macro-redefined",
                "-Wno-builtin-macro-redefined",
                "-Wno-pragma-once-outside-header",
                "-Wno-ignored-attributes",
            ]

            unresolvable_headers: Set[str] = set()

            for unknown_round in range(0, max_unknown_fixups + 1):
                # Make sure wrapper reflects the current extra includes.
                if unknown_round > 0:
                    _write_wrapper_for_clang()

                preprocess_result = None
                preprocess_lang = None
                preprocess_cmd = None
                include_diag = _make_include_diag(include_dirs or set())
                last_auto_resolve: Optional[Dict[str, Any]] = None

                for preprocess_round in range(1, max_preprocess_attempts + 1):
                    modes = ["c++", "c"] if prefer_cxx else ["c", "c++"]
                    preprocess_result = None
                    preprocess_lang = None
                    preprocess_cmd = None

                    for idx, lang in enumerate(modes, start=1):
                        lang_flags = ["-x", "c++", "-std=c++17"] if lang == "c++" else ["-x", "c"]
                        cmd = [clang_bin, "-E", "-P"] + lang_flags + context_flags + [str(wrapper_path)]

                        logger.info(
                            f"运行 clang -E[{idx}/{len(modes)}] (round {preprocess_round}/{max_preprocess_attempts}): "
                            f"lang={lang} clang={clang_bin} "
                            f"(include dirs={include_diag['include_dirs_count']}, missing dirs={include_diag['nonexistent_dir_count']}, out/gen dirs={include_diag['out_gen_dir_count']})"
                        )
                        r = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
                        preprocess_cmd = cmd
                        preprocess_result = r
                        preprocess_lang = lang
                        if r.returncode == 0:
                            break
                        # If we started from C and the error doesn't look C++-related, don't try C++ (save time).
                        if idx == 1 and not prefer_cxx:
                            cxx_signals = ["shared_mutex", "namespace ", "class ", "template<", "expected unqualified-id", "C++"]
                            if not any(s in (r.stderr or "") for s in cxx_signals):
                                break

                    if preprocess_result and preprocess_result.returncode == 0:
                        break

                    stderr = preprocess_result.stderr if preprocess_result else ""
                    missing = _extract_missing_headers(stderr)
                    if not missing:
                        break

                    # Only retry when we can actually add include dirs.
                    if not self.compile_commands_parser:
                        break

                    new_missing = [h for h in missing if h and h not in unresolvable_headers]
                    if not new_missing:
                        break

                    try:
                        new_includes, still_missing, resolved_map = self.compile_commands_parser.get_resolved_includes_for_bindgen(
                            new_missing,
                            include_dirs or set(),
                            preferred_subpaths=preferred_subpaths,
                        )
                    except Exception as e:
                        logger.warning(f"clang -E auto-resolve failed: {e}")
                        break

                    added_count = len(new_includes) - len(include_dirs or set())
                    unresolvable_headers.update(still_missing)
                    try:
                        last_auto_resolve = {
                            "resolved": {k: v for k, v in (resolved_map or {}).items() if v},
                            "unresolved": list(still_missing),
                            "include_dirs_added": int(added_count),
                        }
                    except Exception:
                        last_auto_resolve = None

                    # If this is the last attempt, do not waste time collecting more include dirs we won't validate.
                    if preprocess_round >= max_preprocess_attempts:
                        break

                    if added_count <= 0:
                        break

                    # Update include dirs + context_flags for next round.
                    include_dirs = new_includes
                    for inc_dir in sorted(include_dirs or set(), key=lambda p: str(p)):
                        inc_dir_str = str(inc_dir)
                        if inc_dir_str not in added_dirs:
                            context_flags.extend(["-I", inc_dir_str])
                            added_dirs.add(inc_dir_str)

                    include_diag = _make_include_diag(include_dirs or set())
                    logger.info(f"clang -E auto-resolved {added_count} include dirs; retrying...")

                if not preprocess_result or preprocess_result.returncode != 0:
                    stderr = preprocess_result.stderr if preprocess_result else ""
                    stderr_lines = (stderr or "").splitlines()
                    report["attempts"].append({
                        "tier": "B",
                        "step": "clang_preprocess",
                        "success": False,
                        "error": ("\n".join(stderr_lines[:80]) if stderr_lines else "unknown error"),
                        "debug": {
                            **include_diag,
                            "clang": clang_bin,
                            "lang": preprocess_lang,
                            "tu_context": tu_ctx,
                            "auto_resolve": last_auto_resolve,
                            "wrapper_extra_system_includes": list(wrapper_extra_system_includes),
                            "wrapper_fixup_actions": list(wrapper_fixup_actions),
                            "stderr_tail": ("\n".join(stderr_lines[-80:]) if len(stderr_lines) > 80 else ""),
                            "cmd_head": (preprocess_cmd or [])[:25],
                            "cmd_len": len(preprocess_cmd or []),
                        },
                    })
                    logger.warning(f"clang -E 失败: {(stderr or '')[:300]}")
                    return False

                preprocessed_content = preprocess_result.stdout

                if not preprocessed_content.strip():
                    report["attempts"].append({
                        "tier": "B",
                        "step": "clang_preprocess",
                        "success": False,
                        "error": "empty output"
                    })
                    return False

                # 写入预处理后的内容
                preprocessed_path = self.output_dir / "preprocessed_header.h"
                with open(preprocessed_path, 'w', encoding='utf-8') as f:
                    f.write(preprocessed_content)

                report["attempts"].append({
                    "tier": "B",
                    "step": "clang_preprocess",
                    "success": True,
                    "output_size": len(preprocessed_content),
                    "debug": {
                        **include_diag,
                        "clang": clang_bin,
                        "lang": preprocess_lang,
                        "tu_context": tu_ctx,
                        "auto_resolve": last_auto_resolve,
                        "wrapper_extra_system_includes": list(wrapper_extra_system_includes),
                        "wrapper_fixup_actions": list(wrapper_fixup_actions),
                        "cmd_head": (preprocess_cmd or [])[:25],
                        "cmd_len": len(preprocess_cmd or []),
                    },
                })

                bindgen_cmd = [
                    bindgen_path,
                    str(preprocessed_path),
                    "-o", str(output_path),
                    "--no-layout-tests",
                    "--no-doc-comments",
                    "--use-core",
                    "--default-enum-style=consts",
                    "--no-prepend-enum-name",
                    "--ignore-functions",
                    "--no-size_t-is-usize",
                    "--",
                    "-x", "c++" if preprocess_lang == "c++" else "c",
                ]
                if preprocess_lang == "c++":
                    bindgen_cmd.append("-std=c++17")
                bindgen_cmd.extend(clang_common_flags)
                bindgen_cmd.extend(minimal_flags)

                bindgen_result = subprocess.run(
                    bindgen_cmd,
                    capture_output=True,
                    text=True,
                    timeout=60
                )

                if bindgen_result.returncode == 0 and output_path.exists():
                    # 后处理
                    self._postprocess_bindgen_output(output_path)
                    report["attempts"].append({
                        "tier": "B",
                        "step": "bindgen_on_preprocessed",
                        "success": True,
                        "debug": {
                            **include_diag,
                            "returncode": bindgen_result.returncode,
                            "wrapper_extra_system_includes": list(wrapper_extra_system_includes),
                            "wrapper_fixup_actions": list(wrapper_fixup_actions),
                            "cmd_head": bindgen_cmd[:25],
                            "cmd_len": len(bindgen_cmd),
                        },
                    })
                    logger.info("Tier B: clang -E + bindgen 成功")
                    return True

                stderr = bindgen_result.stderr or ""
                stderr_lines = stderr.splitlines()
                report["attempts"].append({
                    "tier": "B",
                    "step": "bindgen_on_preprocessed",
                    "success": False,
                    "error": ("\n".join(stderr_lines[:80]) if stderr_lines else "unknown error"),
                    "debug": {
                        **include_diag,
                        "returncode": bindgen_result.returncode,
                        "wrapper_extra_system_includes": list(wrapper_extra_system_includes),
                        "wrapper_fixup_actions": list(wrapper_fixup_actions),
                        "stderr_tail": ("\n".join(stderr_lines[-80:]) if len(stderr_lines) > 80 else ""),
                        "cmd_head": bindgen_cmd[:25],
                        "cmd_len": len(bindgen_cmd),
                    },
                })

                unknown_types = _extract_unknown_types(stderr)
                incomplete_enums = _extract_incomplete_enum_types(stderr)

                fixups_applied = False
                if unknown_round < max_unknown_fixups:
                    # 1) Resolve known symbol prototype conflicts (e.g., dprintf: musl vs LiteOS).
                    fixups_applied = _maybe_enable_dprintf_rename(stderr) or fixups_applied
                    # 2) Add lightweight shims for unknown types (prefer shims over heavy kernel includes).
                    if unknown_types:
                        fixups_applied = _add_type_shims(unknown_types) or fixups_applied
                    # 3) Define missing enums when only forward-declared.
                    if incomplete_enums:
                        fixups_applied = _add_enum_shims(incomplete_enums) or fixups_applied

                if unknown_round < max_unknown_fixups and fixups_applied:
                    logger.info(
                        f"Tier B fixup round {unknown_round + 1}/{max_unknown_fixups}: "
                        f"applied (unknown_types={unknown_types}, incomplete_enums={incomplete_enums}, dprintf_renamed={wrapper_rename_dprintf}); retrying..."
                    )
                    continue

                logger.warning(f"Tier B bindgen 失败: {stderr[:300]}")
                return False

        except subprocess.TimeoutExpired:
            report["attempts"].append({
                "tier": "B",
                "success": False,
                "error": "timeout"
            })
            logger.warning("Tier B 超时")
            return False
        except Exception as e:
            report["attempts"].append({
                "tier": "B",
                "success": False,
                "error": str(e)
            })
            logger.warning(f"Tier B 异常: {e}")
            return False
        finally:
            # 清理临时文件
            if not self._env_flag("C2R_BINDGEN_DEBUG_KEEP_FILES"):
                for tmp_file in [self.output_dir / "wrapper_for_clang.h",
                               self.output_dir / "preprocessed_header.h"]:
                    if tmp_file.exists():
                        try:
                            tmp_file.unlink()
                        except:
                            pass

    def _generate_stub_types_rs(self, output_path: Path, header_files: List[Path] = None):
        """
        Tier C: 生成保证编译通过的 stub types.rs

        这是最终兜底方案，生成的 types.rs 一定能通过 cargo check

        包含：
        - 所有基础 C 类型映射
        - 常用系统类型的 opaque 定义
        - 从头文件中扫描到的类型的 opaque 占位符

        Args:
            output_path: 输出路径
            header_files: 头文件列表（可选，用于扫描类型名）
        """
        lines = []

        # 文件头
        lines.append('''//! Auto-generated stub type definitions
//!
//! This is a STUB file generated as a fallback when bindgen failed.
//! All custom types are declared as opaque structs to ensure compilation.
//!
//! Generation mode: Tier C (guaranteed compilation)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused)]

// ============================================================
// Core C Type Mappings (guaranteed correct)
// ============================================================

pub type c_void = core::ffi::c_void;
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

// Fixed-width integer types
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// Size types
pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

// POSIX types
pub type off_t = i64;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type mode_t = u32;
pub type time_t = i64;

// Boolean type
pub type BOOL = i32;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

// ============================================================
// Common System Types (opaque definitions)
// ============================================================

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILE { _opaque: [u8; 0] }

#[repr(C)]
pub struct pthread_mutex_t { _opaque: [u8; 40] }

#[repr(C)]
pub struct pthread_cond_t { _opaque: [u8; 48] }

#[repr(C)]
pub struct pthread_attr_t { _opaque: [u8; 56] }

pub type pthread_t = usize;

// PTHREAD initializers
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = unsafe { ::core::mem::zeroed() };
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = unsafe { ::core::mem::zeroed() };

// ============================================================
// Common Error Codes
// ============================================================

pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;
pub const EAGAIN: i32 = 11;
pub const ETIMEDOUT: i32 = 110;
pub const EBUSY: i32 = 16;
pub const EPERM: i32 = 1;
pub const EFAULT: i32 = 14;

// ============================================================
// Framework-specific Constants (OpenHarmony/HDF/LiteOS)
// ============================================================

// SoftBus
pub const SOFTBUS_OK: i32 = 0;
pub const SOFTBUS_ERR: i32 = -1;
pub const SOFTBUS_INVALID_PARAM: i32 = -3;

// HDF
pub const HDF_SUCCESS: i32 = 0;
pub const HDF_FAILURE: i32 = -1;
pub const HDF_ERR_INVALID_PARAM: i32 = -3;

// LiteOS
pub const LOS_OK: u32 = 0;
pub const LOS_NOK: u32 = 1;
pub const LOS_ERRNO_TSK_ID_INVALID: u32 = 0x02000207;
''')

        # 如果有头文件，扫描其中的类型名并生成 opaque 占位符
        scanned_types = set()
        # 新增：扫描简单 typedef 并记录其对应的 Rust 类型
        simple_typedefs: Dict[str, str] = {}

        # C 类型到 Rust 类型的映射（用于简单 typedef）
        c_to_rust_type_map = {
            'int': 'i32',
            'unsigned int': 'u32',
            'signed int': 'i32',
            'char': 'i8',
            'signed char': 'i8',
            'unsigned char': 'u8',
            'short': 'i16',
            'short int': 'i16',
            'signed short': 'i16',
            'signed short int': 'i16',
            'unsigned short': 'u16',
            'unsigned short int': 'u16',
            'long': 'i64',
            'long int': 'i64',
            'signed long': 'i64',
            'signed long int': 'i64',
            'unsigned long': 'u64',
            'unsigned long int': 'u64',
            'long long': 'i64',
            'long long int': 'i64',
            'signed long long': 'i64',
            'unsigned long long': 'u64',
            'float': 'f32',
            'double': 'f64',
            'long double': 'f64',
            'void': 'c_void',
            '_Bool': 'bool',
        }

        if header_files:
            for h in header_files:
                if h.exists():
                    try:
                        with open(h, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()

                        # 新增：提取简单 typedef（如 typedef int Int32;）
                        # 匹配模式：typedef <c_type> <name>;
                        simple_typedef_pattern = r'typedef\s+((?:(?:signed|unsigned|long|short|const|volatile)\s+)*(?:char|int|short|long|float|double|void|_Bool))\s+(\w+)\s*;'
                        for match in re.finditer(simple_typedef_pattern, content):
                            c_type = match.group(1).strip()
                            type_name = match.group(2).strip()
                            # 规范化 C 类型（去除多余空格）
                            c_type_normalized = ' '.join(c_type.split())
                            if c_type_normalized in c_to_rust_type_map:
                                rust_type = c_to_rust_type_map[c_type_normalized]
                                simple_typedefs[type_name] = rust_type

                        # 提取 typedef struct xxx
                        typedef_structs = re.findall(r'typedef\s+struct\s+\w*\s*\{[^}]*\}\s*(\w+)\s*;', content, re.DOTALL)
                        scanned_types.update(typedef_structs)

                        # 提取 struct xxx {
                        struct_names = re.findall(r'struct\s+(\w+)\s*\{', content)
                        scanned_types.update(struct_names)

                        # 提取 enum xxx {
                        enum_names = re.findall(r'enum\s+(\w+)\s*\{', content)
                        scanned_types.update(enum_names)

                    except Exception as e:
                        logger.debug(f"扫描头文件 {h} 失败: {e}")

        # 过滤掉已定义的类型和无效名称
        already_defined = {
            'FILE', 'pthread_mutex_t', 'pthread_cond_t', 'pthread_attr_t',
            'c_void', 'c_char', 'c_int', 'c_uint', 'c_long', 'c_ulong',
            'int8_t', 'int16_t', 'int32_t', 'int64_t',
            'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t',
            'size_t', 'ssize_t', 'BOOL', 'off_t', 'pid_t', 'uid_t', 'gid_t', 'mode_t', 'time_t'
        }

        # 过滤简单 typedef，排除已定义的类型
        valid_simple_typedefs = {
            name: rust_type for name, rust_type in simple_typedefs.items()
            if name not in already_defined and re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', name)
        }

        valid_scanned = set()
        for t in scanned_types:
            if t and t not in already_defined and t not in valid_simple_typedefs and re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', t):
                valid_scanned.add(t)

        # 新增：先输出简单 typedef 的类型别名
        if valid_simple_typedefs:
            lines.append("\n// ============================================================")
            lines.append("// Project-specific Type Aliases (scanned from headers)")
            lines.append("// ============================================================\n")

            for type_name in sorted(valid_simple_typedefs.keys()):
                rust_type = valid_simple_typedefs[type_name]
                lines.append(f"/// Type alias for `{type_name}` (from C typedef)")
                lines.append(f"pub type {type_name} = {rust_type};")
                lines.append("")

        # 然后输出 opaque structs（仅用于复杂类型）
        if valid_scanned:
            lines.append("\n// ============================================================")
            lines.append("// Project-specific Types (scanned from headers, opaque)")
            lines.append("// ============================================================\n")

            for type_name in sorted(valid_scanned):
                lines.append(f"/// Opaque placeholder for external type `{type_name}`")
                lines.append("#[repr(C)]")
                lines.append("#[derive(Debug, Copy, Clone)]")
                lines.append(f"pub struct {type_name} {{ _opaque: [u8; 0] }}")
                lines.append("")

        # 写入文件
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))

        logger.info(f"Stub types.rs 已生成: {len(valid_simple_typedefs)} 个类型别名, {len(valid_scanned)} 个 opaque 类型")

    def _env_flag(self, name: str, default: str = "0") -> bool:
        """Parse a boolean-ish environment flag safely (1/true/yes/on)."""
        v = os.environ.get(name, default)
        return str(v).strip().lower() in {"1", "true", "yes", "y", "on"}

    def _write_types_generation_report(self, report: dict):
        """
        写入 types 生成报告

        Args:
            report: 报告数据
        """
        import json

        report_path = self.output_dir / "types_generation_report.json"

        try:
            with open(report_path, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2, ensure_ascii=False, default=str)
            logger.debug(f"Types 生成报告已写入: {report_path}")
            if self._env_flag("C2R_BINDGEN_DEBUG") or report.get("mode") == "stub" or not report.get("success", False):
                print(f"  📄 types_generation_report.json: {report_path}")
        except Exception as e:
            logger.warning(f"写入 types 生成报告失败: {e}")
    
    def _attempt_bindgen(
        self, 
        header_files: List[Path], 
        output_file: str,
        include_dirs: Set[Path] = None,
        source_files: List[Path] = None,
    ) -> Tuple[bool, str, List[str], Dict[str, Any]]:
        """
        尝试运行 bindgen
        
        Args:
            header_files: 头文件列表
            output_file: 输出文件名
            include_dirs: 要使用的 include 路径集合（如果为 None，使用 self.include_dirs）
        
        Returns:
            (是否成功, 错误信息, 缺失的头文件列表, 调试信息)
        """
        attempt_debug: Dict[str, Any] = {
            "bindgen_debug": self._env_flag("C2R_BINDGEN_DEBUG"),
            "bindgen_debug_keep_files": self._env_flag("C2R_BINDGEN_DEBUG_KEEP_FILES"),
            "compile_commands_loaded": bool(self.compile_commands_parser),
            "compile_commands_path": str(self.compile_commands_parser.compile_db_path) if self.compile_commands_parser else None,
        }
        bindgen_debug = bool(attempt_debug["bindgen_debug"])
        keep_files = bool(attempt_debug["bindgen_debug_keep_files"])

        # 使用传入的 include_dirs 或默认的
        if include_dirs is None:
            include_dirs = set(self.include_dirs)
        attempt_debug["include_dirs_input_count"] = len(include_dirs)
        attempt_debug["self_include_dirs_count"] = len(getattr(self, "include_dirs", []) or [])
        
        # 创建临时 wrapper.h 包含所有头文件
        included_headers = [h for h in header_files if h.exists()]
        attempt_debug["header_files_count"] = len(header_files)
        attempt_debug["header_files_included_count"] = len(included_headers)
        attempt_debug["header_files_included_sample"] = [str(h) for h in included_headers[:10]]

        wrapper_path = self.output_dir / "wrapper.h"

        # Wrapper-level fixups that can be applied based on bindgen diagnostics.
        wrapper_extra_system_includes: List[str] = []
        wrapper_header_order: List[Path] = list(header_files)
        wrapper_fixup_actions: List[Dict[str, Any]] = []

        def _write_wrapper(
            force_need_struct_sched_param: bool,
            *,
            extra_system_includes: Optional[List[str]] = None,
            ordered_headers: Optional[List[Path]] = None,
        ):
            wrapper_content: List[str] = []
            wrapper_content.append("// Auto-generated wrapper header for bindgen")
            wrapper_content.append("")

            # musl's bits/alltypes.h uses __NEED_* gating; missing some __NEED_* may cause incomplete types.
            # For sched_param, prefer including <sched.h> on-demand (defining __NEED_struct_sched_param can
            # conflict with <sched.h> and trigger redefinition errors).
            if force_need_struct_sched_param:
                wrapper_content.append("#include <sched.h>")
                wrapper_content.append("")
            wrapper_content.append("#ifndef __NEED_struct_cpu_set_t")
            wrapper_content.append("#define __NEED_struct_cpu_set_t 1")
            wrapper_content.append("#endif")
            wrapper_content.append("")

            # Linux/UAPI-style annotation macros: make them parseable outside full kernel build context.
            wrapper_content.append("#ifndef __must_check")
            wrapper_content.append("#define __must_check __attribute__((warn_unused_result))")
            wrapper_content.append("#endif")
            wrapper_content.append("#ifndef __packed")
            wrapper_content.append("#define __packed __attribute__((packed))")
            wrapper_content.append("#endif")
            wrapper_content.append("#ifndef __aligned")
            wrapper_content.append("#define __aligned(x) __attribute__((aligned(x)))")
            wrapper_content.append("#endif")
            wrapper_content.append("#ifndef __user")
            wrapper_content.append("#define __user")
            wrapper_content.append("#endif")
            wrapper_content.append("#ifndef __force")
            wrapper_content.append("#define __force")
            wrapper_content.append("#endif")
            wrapper_content.append("#ifndef __iomem")
            wrapper_content.append("#define __iomem")
            wrapper_content.append("#endif")
            wrapper_content.append("")

            # Provide common fundamental typedefs early.
            # Some OpenHarmony headers assume these are already available (via transitive includes in real TUs).
            wrapper_content.append("#include <stddef.h>")
            wrapper_content.append("#include <stdint.h>")
            wrapper_content.append("#include <stdbool.h>")
            if extra_system_includes:
                for inc in extra_system_includes:
                    inc = (inc or "").strip()
                    if not inc:
                        continue
                    wrapper_content.append(f"#include {inc}")
            wrapper_content.append("")

            for h in (ordered_headers or header_files):
                if h.exists():
                    wrapper_content.append(f'#include "{h}"')

            wrapper_content.append("")

            with open(wrapper_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(wrapper_content))
            attempt_debug["wrapper_need_struct_sched_param"] = bool(force_need_struct_sched_param)
            attempt_debug["wrapper_extra_system_includes"] = list(extra_system_includes or [])
            try:
                hdrs = list(ordered_headers or header_files)
                attempt_debug["wrapper_headers_head"] = [str(p) for p in hdrs[:30]]
            except Exception:
                pass
            attempt_debug["wrapper_fixup_actions"] = list(wrapper_fixup_actions)

        def _extract_unknown_type_records(stderr_text: str) -> List[Tuple[str, str]]:
            if not stderr_text:
                return []
            pat = re.compile(
                r"^(?P<file>[^:\n]+):\d+:\d+:\s+error:\s+unknown type name\s+'(?P<type>[^']+)'",
                re.MULTILINE,
            )
            out: List[Tuple[str, str]] = []
            for m in pat.finditer(stderr_text):
                f = (m.group("file") or "").strip()
                t = (m.group("type") or "").strip()
                if f and t:
                    out.append((f, t))
            return out

        def _header_defines_typedef_like(h: Path, type_name: str) -> bool:
            try:
                content = h.read_text(encoding="utf-8", errors="ignore")
            except Exception:
                return False
            if type_name not in content:
                return False
            typedef_pat = re.compile(rf"\btypedef\s+(?:struct|enum|union)\b[^;]*\b{re.escape(type_name)}\b\s*;", re.MULTILINE)
            if typedef_pat.search(content):
                return True
            struct_pat = re.compile(rf"\bstruct\s+{re.escape(type_name)}\s*\{{", re.MULTILINE)
            if struct_pat.search(content):
                return True
            return False

        def _find_defining_header_for_type(type_name: str, headers: List[Path]) -> Optional[Path]:
            known = {
                "IpcObjectStub": "ipc_skeleton.h",
            }
            target_basename = known.get(type_name)
            if target_basename:
                for h in headers:
                    if h.name == target_basename:
                        return h
            for h in headers:
                if not h.exists():
                    continue
                if _header_defines_typedef_like(h, type_name):
                    return h
            return None

        def _move_header_before(headers: List[Path], header_to_move: Path, before_header: Path) -> Tuple[List[Path], bool]:
            if header_to_move == before_header:
                return headers, False
            try:
                i_move = next(i for i, p in enumerate(headers) if p == header_to_move)
                i_before = next(i for i, p in enumerate(headers) if p == before_header)
            except StopIteration:
                return headers, False
            if i_move < i_before:
                return headers, False
            new_headers = [p for i, p in enumerate(headers) if i != i_move]
            try:
                j_before = next(i for i, p in enumerate(new_headers) if p == before_header)
            except StopIteration:
                return headers, False
            new_headers.insert(j_before, header_to_move)
            return new_headers, True

        # First try without forcing sched_param (avoids <sched.h> redefinition); enable on-demand on specific errors.
        _write_wrapper(
            force_need_struct_sched_param=False,
            extra_system_includes=wrapper_extra_system_includes,
            ordered_headers=wrapper_header_order,
        )
        attempt_debug["wrapper_path"] = str(wrapper_path)
        attempt_debug["wrapper_has_extern_c_guard"] = False
        
        output_path = self.output_dir / "src" / output_file
        missing_files = []
        attempt_debug["output_path"] = str(output_path)

        def _extract_include_args(argv: List[str]) -> List[Tuple[str, str]]:
            """Extract include directory args (-I/-isystem) from clang argv in order."""
            res: List[Tuple[str, str]] = []
            i = 0
            while i < len(argv):
                a = argv[i]
                if a == "-I" and i + 1 < len(argv):
                    res.append(("-I", argv[i + 1]))
                    i += 2
                    continue
                if a.startswith("-I") and a != "-I":
                    res.append(("-I", a[len("-I"):]))
                    i += 1
                    continue
                if a == "-isystem" and i + 1 < len(argv):
                    res.append(("-isystem", argv[i + 1]))
                    i += 2
                    continue
                if a.startswith("-isystem") and a != "-isystem":
                    res.append(("-isystem", a[len("-isystem"):]))
                    i += 1
                    continue
                i += 1
            return res

        def _is_out_gen_path(p: str) -> bool:
            norm = (p or "").replace("\\", "/")
            return "/out/" in norm and "/gen" in norm

        def _summarize_clang_flags(flags: List[str]) -> Dict[str, int]:
            return {
                "total": len(flags),
                "D": sum(1 for f in flags if isinstance(f, str) and f.startswith("-D")),
                "U": sum(1 for f in flags if isinstance(f, str) and f.startswith("-U")),
                "I": sum(1 for f, _p in _extract_include_args(flags) if f == "-I"),
                "isystem": sum(1 for f, _p in _extract_include_args(flags) if f == "-isystem"),
                "include": sum(1 for f in flags if f == "-include"),
                "imacros": sum(1 for f in flags if f == "-imacros"),
                "sysroot": sum(1 for f in flags if isinstance(f, str) and (f.startswith("--sysroot=") or f == "--sysroot" or f == "-isysroot")),
                "target": sum(1 for f in flags if isinstance(f, str) and f.startswith("--target=")),
            }
        
        try:
            # 检查 bindgen 是否可用
            bindgen_path = shutil.which("bindgen")
            if not bindgen_path:
                # 尝试 cargo 安装目录
                cargo_bin = Path.home() / ".cargo" / "bin" / "bindgen"
                if cargo_bin.exists():
                    bindgen_path = str(cargo_bin)
                else:
                    logger.error("bindgen 未安装，无法生成类型骨架")
                    return False, "bindgen not installed", [], attempt_debug
            attempt_debug["bindgen_path"] = str(bindgen_path)
            
            # 构建 bindgen 命令（clang 参数在 `--` 之后）
            bindgen_cmd_prefix = [
                bindgen_path,
                str(wrapper_path),
                "-o", str(output_path),
                "--no-layout-tests",
                "--no-doc-comments",
                "--use-core",
                "--default-enum-style=consts",
                "--no-prepend-enum-name",
                "--ignore-functions",
                "--no-size_t-is-usize",
                "--",
            ]

            # ========== musl 兼容性 clang 参数（通用） ==========
            clang_common_flags = [
                # Do not fail the whole bindgen run due to warnings (some headers trigger noisy warnings).
                "-Wno-error",
                "-Wno-error=register",
                "-Wno-incompatible-library-redeclaration",
                "-Wno-shift-op-parentheses",
                "-Wno-unused-function",
                "-Wno-unused-variable",
                "-Wno-macro-redefined",
                "-Wno-builtin-macro-redefined",
                "-Wno-pragma-once-outside-header",
                "-Wno-ignored-attributes",
            ]

            # Decide language mode: prefer the representative TU's language (C vs C++).
            rep_src_suffix = ""
            try:
                rep_src_suffix = str(Path(source_files[0]).suffix).lower() if source_files else ""
            except Exception:
                rep_src_suffix = ""
            prefer_cxx = rep_src_suffix in {".cc", ".cpp", ".cxx", ".c++"}
            attempt_debug["clang_mode"] = {
                "rep_src_suffix": rep_src_suffix,
                "preferred": "c++" if prefer_cxx else "c",
                "attempts": [],
            }

            def _build_cpp_stdlib_flags() -> Tuple[List[str], Dict[str, Any]]:
                """
                Best-effort: add C++ standard library headers.
                Prefer OpenHarmony prebuilts' libc++ when available; otherwise fall back to host.
                """
                cpp_flags: List[str] = []
                meta: Dict[str, Any] = {"added": False, "mode": None, "paths": [], "paths_count": 0, "clang_resource": None}

                def _find_ohos_clang_resource_include() -> Optional[Path]:
                    """
                    Prefer OpenHarmony prebuilts' clang resource headers:
                      prebuilts/clang/ohos/linux-x86_64/llvm/lib/clang/<ver>/include
                    These provide fundamental headers like <stddef.h> with nullptr_t support.
                    """
                    if not self.ohos_root:
                        return None
                    base = (
                        self.ohos_root
                        / "prebuilts"
                        / "clang"
                        / "ohos"
                        / "linux-x86_64"
                        / "llvm"
                        / "lib"
                        / "clang"
                    )
                    if not base.exists():
                        return None
                    # Prefer "current" if present; otherwise pick the lexicographically largest version dir.
                    current = base / "current" / "include"
                    if current.exists():
                        return current
                    versions: List[Path] = []
                    try:
                        for p in base.iterdir():
                            inc = p / "include"
                            if inc.exists():
                                versions.append(inc)
                    except Exception:
                        versions = []
                    if not versions:
                        return None
                    versions.sort(key=lambda p: str(p))
                    return versions[-1]

                # Prefer OpenHarmony prebuilts (avoid host libstdc++ mismatches).
                if self.ohos_root:
                    ohos_prebuilts_base = self.ohos_root / "prebuilts" / "clang" / "ohos" / "linux-x86_64"
                    ohos_llvm_roots = [
                        ohos_prebuilts_base / "llvm_ndk",  # OpenHarmony NDK layout
                        ohos_prebuilts_base / "llvm",      # legacy layout
                    ]
                    for ohos_llvm in ohos_llvm_roots:
                        ohos_cpp_candidates = [
                            ohos_llvm / "include" / "libcxx-ohos" / "include" / "c++" / "v1",
                            ohos_llvm / "include" / "c++" / "v1",
                            ohos_llvm / "include" / "x86_64-unknown-linux-gnu" / "c++" / "v1",
                        ]
                        existing_ohos_cpp = [p for p in ohos_cpp_candidates if p.exists()]
                        if existing_ohos_cpp:
                            cpp_flags.append("-nostdinc++")
                            resource_inc = _find_ohos_clang_resource_include()
                            if resource_inc:
                                cpp_flags.extend(["-isystem", str(resource_inc)])
                                meta["clang_resource"] = str(resource_inc)
                            for p in existing_ohos_cpp:
                                cpp_flags.extend(["-isystem", str(p)])
                            meta.update({
                                "added": True,
                                "mode": "ohos_prebuilts",
                                "paths": [str(p) for p in existing_ohos_cpp[:10]],
                                "paths_count": len(existing_ohos_cpp),
                            })
                            return cpp_flags, meta

                # Host fallback
                host_cpp_candidates = [
                    "/usr/include/x86_64-linux-gnu/c++/11",
                    "/usr/include/c++/11",
                    "/usr/include/c++/v1",
                    "/usr/lib/llvm-14/include/c++/v1",
                    "/usr/lib/llvm-13/include/c++/v1",
                ]
                for cpp_path in host_cpp_candidates:
                    if Path(cpp_path).exists():
                        cpp_flags.extend(["-isystem", cpp_path])
                        meta.setdefault("paths", []).append(cpp_path)
                if meta.get("paths"):
                    meta["added"] = True
                    meta["mode"] = "host"
                    meta["paths_count"] = len(meta["paths"])
                return cpp_flags, meta

            # Build language-mode attempts: try preferred mode first, then fallback.
            mode_attempts: List[Dict[str, Any]] = []
            modes = ["c++", "c"] if prefer_cxx else ["c", "c++"]
            for lang in modes:
                mode_flags: List[str] = ["-x", "c++", "-std=c++17"] if lang == "c++" else ["-x", "c"]
                cpp_flags: List[str] = []
                cpp_meta: Dict[str, Any] = {"added": False}
                if lang == "c++":
                    cpp_flags, cpp_meta = _build_cpp_stdlib_flags()
                mode_attempts.append({
                    "lang": lang,
                    "clang_flags": mode_flags + clang_common_flags + cpp_flags,
                    "cpp_stdlib": cpp_meta,
                })
            
            # ========== ★★★ 关键优化：使用“单个 TU 的真实编译上下文（保序）”而不是全局 include 并集 ★★★ ==========
            # 背景：全局 include 并集 + set 去重会打乱 -I 顺序，导致 clang 选错同名头文件（如 los_config.h），
            # 进而触发 target_config.h/soc.h 等级联缺失，最后逼迫回退到 stub types.rs。
            #
            # 这里先构建一个“上下文 clang flags”列表（不含语言模式），后续每种语言模式尝试都会复用它。
            context_flags: List[str] = []
            added_dirs: Set[str] = set()
            cc_context_used = False

            # 1) 优先从 compile_commands.json 取“代表性源文件”的 clang flags（包含 -I/-isystem/-D 等，并保序）
            if self.compile_commands_parser and source_files:
                rep_src: Optional[Path] = None
                for src in source_files:
                    p = Path(src)
                    if p.exists():
                        rep_src = p
                        break

                if rep_src:
                    mapped_src = self._map_to_ohos_path(rep_src)
                    try:
                        tu_debug: Dict[str, Any] = {
                            "rep_src": str(rep_src),
                            "mapped_src": str(mapped_src),
                            "used_proxy": False,
                        }
                        # Prefer stage1-selected TU context (pins flags/macros/include order).
                        entry: Optional[Dict[str, Any]] = None
                        match_info: Dict[str, Any] = {}
                        try:
                            safe_name = self._get_safe_module_name(rep_src)
                            rec = self._tu_context_files.get(safe_name) if getattr(self, "_tu_context_files", None) else None
                            if isinstance(rec, dict) and isinstance(rec.get("compile_commands_entry"), dict):
                                entry = rec.get("compile_commands_entry")
                                match_info = {
                                    "reason": "tu_context_map",
                                    "tu_context_map": str(getattr(self, "_tu_context_map_path", "")) if getattr(self, "_tu_context_map_path", None) else None,
                                    "entry_hash": rec.get("entry_hash"),
                                }
                                tu_debug["tu_context_map_used"] = True
                                tu_debug["tu_context_safe_name"] = safe_name
                        except Exception:
                            entry = None
                            match_info = {}

                        if not entry:
                            entry, match_info = self.compile_commands_parser.get_entry_for_file_with_reason(mapped_src)
                        tu_debug["match_info"] = match_info
                        tu_debug["entry_found"] = bool(entry)

                        clang_flags_raw: List[str] = []
                        clang_flags: List[str] = []
                        if entry:
                            clang_flags_raw = self.compile_commands_parser.get_clang_flags_for_entry(entry, normalize_paths=False)
                            clang_flags = self.compile_commands_parser.get_clang_flags_for_entry(entry, normalize_paths=True)
                            tu_debug["entry_file"] = entry.get("file")
                            tu_debug["entry_directory"] = entry.get("directory")

                        # 如果该模块本身没有出现在 compile_commands 中（常见于 SelfContained 子集），
                        # 可选：退化为选择同一子系统（例如 kernel/liteos_a）的任意一个已编译 TU 作为“上下文代理”。
                        # Default OFF: when the compile_commands closure is incomplete, treat it as an input issue and report it.
                        enable_proxy = os.environ.get("C2R_ENABLE_PROXY_TU_FALLBACK", "0").strip().lower() in ("1", "true", "yes")
                        if enable_proxy and (not clang_flags) and self._ohos_project_rel and len(self._ohos_project_rel.parts) >= 2:
                            # Prefer a *nearby* proxy TU: try longer prefixes first (more specific), then fall back.
                            max_parts = min(5, len(self._ohos_project_rel.parts))
                            proxy_file: Optional[Path] = None
                            proxy_key: Optional[str] = None
                            for n in range(max_parts, 1, -1):
                                candidate_key = str(Path(*self._ohos_project_rel.parts[:n]))
                                candidate_file = self.compile_commands_parser.find_first_source_file_containing(candidate_key)
                                if candidate_file:
                                    proxy_key = candidate_key
                                    proxy_file = candidate_file
                                    break

                            if proxy_file and proxy_key:
                                tu_debug["proxy_key"] = proxy_key
                                tu_debug["proxy_file"] = str(proxy_file)
                                p_entry, p_match_info = self.compile_commands_parser.get_entry_for_file_with_reason(proxy_file)
                                tu_debug["proxy_match_info"] = p_match_info
                                tu_debug["proxy_entry_found"] = bool(p_entry)
                                if p_entry:
                                    clang_flags_raw = self.compile_commands_parser.get_clang_flags_for_entry(p_entry, normalize_paths=False)
                                    clang_flags = self.compile_commands_parser.get_clang_flags_for_entry(p_entry, normalize_paths=True)
                                    tu_debug["used_proxy"] = True
                                if clang_flags:
                                    logger.info(f"使用代理 TU 的 clang flags: {proxy_file} (key={proxy_key})")

                        raw_inc = set(p for _f, p in _extract_include_args(clang_flags_raw))
                        norm_inc = set(p for _f, p in _extract_include_args(clang_flags))
                        dropped_inc = sorted(raw_inc - norm_inc)
                        tu_debug["clang_flags_summary"] = _summarize_clang_flags(clang_flags)
                        tu_debug["clang_flags_raw_summary"] = _summarize_clang_flags(clang_flags_raw)
                        tu_debug["include_dirs_raw_count"] = len(raw_inc)
                        tu_debug["include_dirs_norm_count"] = len(norm_inc)
                        tu_debug["include_dirs_dropped_count"] = len(dropped_inc)
                        tu_debug["include_dirs_dropped_sample"] = dropped_inc[:20]
                        tu_debug["include_dirs_dropped_out_gen_count"] = sum(1 for p in dropped_inc if _is_out_gen_path(p))
                        attempt_debug["tu_context"] = tu_debug

                        if clang_flags:
                            cc_context_used = True
                            context_flags.extend(clang_flags)
                            # 记录已添加的 include 目录，避免后续重复
                            j = 0
                            while j < len(clang_flags):
                                flag = clang_flags[j]
                                if flag == "-I" and j + 1 < len(clang_flags):
                                    added_dirs.add(clang_flags[j + 1])
                                    j += 2
                                    continue
                                if flag == "-isystem" and j + 1 < len(clang_flags):
                                    added_dirs.add(clang_flags[j + 1])
                                    j += 2
                                    continue
                                if flag.startswith("-I") and flag != "-I":
                                    added_dirs.add(flag[2:])
                                j += 1
                            logger.info(f"使用 compile_commands 的 clang flags: {mapped_src} (args={len(clang_flags)})")
                    except Exception as e:
                        logger.warning(f"从 compile_commands.json 获取 clang flags 失败: {e}")
                        attempt_debug["tu_context_error"] = str(e)[:300]

            # 2) 添加智能寻路发现的 include 路径（仅作为补充）
            for inc_dir in sorted(include_dirs, key=lambda p: str(p)):
                inc_dir_str = str(inc_dir)
                if inc_dir_str not in added_dirs:
                    context_flags.extend(["-I", inc_dir_str])
                    added_dirs.add(inc_dir_str)

            # 3) 添加头文件所在目录（项目内相对 include 的兜底）
            for h in sorted(header_files, key=lambda p: str(p)):
                h_dir_str = str(Path(h).parent)
                if h_dir_str not in added_dirs:
                    context_flags.extend(["-I", h_dir_str])
                    added_dirs.add(h_dir_str)

            # 4) 添加所有收集到的头文件搜索路径（最后兜底，避免覆盖 TU 的优先级）
            # 仅在没有 compile_commands 上下文时启用：
            # 否则“全局 include 并集”容易把 libc/系统头文件解析到错误实现（同名碰撞）。
            if not cc_context_used:
                for inc_dir in sorted(self.include_dirs, key=lambda p: str(p)):
                    inc_str = str(inc_dir)
                    if inc_str not in added_dirs:
                        context_flags.extend(["-I", inc_str])
                        added_dirs.add(inc_str)

            # ------------------------------------------------------------------
            # Execute bindgen with one or two language-mode attempts.
            # ------------------------------------------------------------------
            def _extract_missing_headers(stderr_text: str) -> List[str]:
                if not stderr_text:
                    return []
                missing = re.findall(r"'([^']+)' file not found", stderr_text)
                # Some clang variants omit quotes: "fatal error: foo/bar.h file not found"
                missing += re.findall(r"fatal error:\s*([^\s:]+)\s*file not found", stderr_text)
                if not missing:
                    return []
                seen = set()
                unique_missing: List[str] = []
                for f in missing:
                    f = (f or "").strip().strip('"').strip("'").strip("<>").strip()
                    if f and f not in seen:
                        seen.add(f)
                        unique_missing.append(f)
                return unique_missing

            def _needs_sched_param_fixup(stderr_text: str) -> bool:
                if not stderr_text:
                    return False
                return (
                    "field has incomplete type 'struct sched_param'" in stderr_text
                    or "unknown type name 'struct sched_param'" in stderr_text
                )

            def _needs_target_cpu_fixup(stderr_text: str) -> bool:
                if not stderr_text:
                    return False
                low = stderr_text.lower()
                return "unknown target cpu" in low

            def _strip_arch_flags(flags: List[str]) -> List[str]:
                """Drop -mcpu/-march/-mtune/... flags when host clang cannot understand the target CPU."""
                out: List[str] = []
                i = 0
                while i < len(flags):
                    f = flags[i]
                    if not isinstance(f, str):
                        out.append(f)
                        i += 1
                        continue
                    # Two-token forms
                    if f in {"-mcpu", "-march", "-mtune", "-mabi", "-mfpu", "-mfloat-abi"}:
                        i += 2
                        continue
                    # One-token forms / prefixes
                    if f.startswith((
                        "-mcpu=",
                        "-march=",
                        "-mtune=",
                        "-mabi=",
                        "-mfpu=",
                        "-mfloat-abi=",
                        "-mno-",
                        "-mno_",
                        "-msoft-float",
                        "-mhard-float",
                    )):
                        i += 1
                        continue
                    # Clang cc1 target-cpu forms: -Xclang -target-cpu -Xclang ck802
                    if f == "-Xclang" and i + 1 < len(flags):
                        nxt = flags[i + 1]
                        if nxt in {"-target-cpu", "-target-feature"}:
                            # Skip the pair plus the following -Xclang <value> if present
                            i += 2
                            if i + 1 < len(flags) and flags[i] == "-Xclang":
                                i += 2
                            continue
                    out.append(f)
                    i += 1
                return out

            # Some LiteOS/musl headers can fail with incomplete struct sched_param depending on include order.
            # Retry once by injecting <sched.h> early (safer than defining __NEED_struct_sched_param, which can
            # conflict with <sched.h> and trigger redefinition errors).
            unknown_fixup_rounds = 0
            max_unknown_fixup_rounds = 2
            arch_flags_stripped = False
            gen_ensure_tried = False

            for wrapper_round, force_sched_param in enumerate([False, True], start=1):
                if force_sched_param:
                    _write_wrapper(
                        force_need_struct_sched_param=True,
                        extra_system_includes=wrapper_extra_system_includes,
                        ordered_headers=wrapper_header_order,
                    )

                need_sched_param_retry = False
                while True:
                    best_failure: Optional[Dict[str, Any]] = None

                    for idx, m in enumerate(mode_attempts, start=1):
                        lang = m.get("lang")
                        clang_mode_flags = m.get("clang_flags") or []
                        cmd = bindgen_cmd_prefix + clang_mode_flags + context_flags

                        # include 目录诊断（用于定位 out/.../gen 缺失、路径被 normalize 丢弃等）
                        clang_argv = cmd[cmd.index("--") + 1:] if "--" in cmd else []
                        include_args = _extract_include_args(clang_argv)
                        unique_include_dirs = sorted({p for _f, p in include_args})
                        nonexistent_dirs = [p for p in unique_include_dirs if p and not Path(p).exists()]
                        out_gen_dirs = [p for p in unique_include_dirs if _is_out_gen_path(p)]

                        logger.info(
                            f"运行 bindgen[{idx}/{len(mode_attempts)}]: lang={lang} wrapper_round={wrapper_round} "
                            f"(include dirs={len(unique_include_dirs)}, missing dirs={len(nonexistent_dirs)}, out/gen dirs={len(out_gen_dirs)})"
                        )

                        result = subprocess.run(
                            cmd,
                            capture_output=True,
                            text=True,
                            timeout=60
                        )

                        attempt_entry: Dict[str, Any] = {
                            "lang": lang,
                            "wrapper_round": wrapper_round,
                            "wrapper_need_struct_sched_param": bool(force_sched_param),
                            "returncode": result.returncode,
                            "cpp_stdlib": m.get("cpp_stdlib"),
                        }
                        if result.stderr:
                            stderr_lines = result.stderr.splitlines()
                            attempt_entry["stderr_head"] = "\n".join(stderr_lines[:50])
                            attempt_entry["stderr_tail"] = "\n".join(stderr_lines[-50:])

                        attempt_entry["include_dirs"] = {
                            "unique_dir_count": len(unique_include_dirs),
                            "nonexistent_dir_count": len(nonexistent_dirs),
                            "nonexistent_dir_sample": nonexistent_dirs[:20],
                            "out_gen_dir_count": len(out_gen_dirs),
                            "out_gen_dir_existing_count": sum(1 for p in out_gen_dirs if Path(p).exists()),
                            "include_order_sample": include_args[:30],
                        }
                        attempt_debug["clang_mode"]["attempts"].append(attempt_entry)

                        if result.returncode == 0 and output_path.exists():
                            attempt_debug["bindgen_cmd_len"] = len(cmd)
                            attempt_debug["bindgen_cmd_head"] = cmd[:25]
                            attempt_debug["bindgen_returncode"] = result.returncode
                            if bindgen_debug:
                                attempt_debug["bindgen_cmd"] = cmd
                            if result.stderr:
                                attempt_debug["bindgen_stderr_head"] = attempt_entry.get("stderr_head")
                                attempt_debug["bindgen_stderr_tail"] = attempt_entry.get("stderr_tail")

                            self._postprocess_bindgen_output(output_path)
                            logger.info(f"bindgen 成功生成类型骨架: {output_path} (lang={lang})")
                            return True, "", [], attempt_debug

                        error_msg = result.stderr if result.stderr else "Unknown error"
                        missing_files = _extract_missing_headers(error_msg)

                        failure = {
                            "cmd": cmd,
                            "lang": lang,
                            "error_msg": error_msg,
                            "missing_files": missing_files,
                            "nonexistent_dir_count": len(nonexistent_dirs),
                        }
                        if missing_files and (best_failure is None or not best_failure.get("missing_files")):
                            best_failure = failure
                        elif best_failure is None:
                            best_failure = failure

                        # Heuristic: if preferred mode is C and we see clear C++-only signals, try the C++ fallback.
                        if idx == 1 and not prefer_cxx:
                            cxx_signals = [
                                "_LIBCPP_BEGIN_NAMESPACE_STD",
                                "_LIBCPP_END_NAMESPACE_STD",
                                "std::",
                                "namespace ",
                                "class ",
                                "template<",
                                "expected unqualified-id",
                                "nullptr",
                                "operator new",
                                "C++",
                            ]

                            # Missing header with *no extension* is often a C++ standard header (e.g. <codecvt>, <map>, <memory>).
                            saw_probable_cxx_header = any(
                                ("/" not in h and "." not in h) for h in (missing_files or [])
                            )

                            if not any(s in error_msg for s in cxx_signals) and not saw_probable_cxx_header:
                                break

                    assert best_failure is not None
                    error_msg = best_failure.get("error_msg") or "Unknown error"
                    missing_files = best_failure.get("missing_files") or []

                    # Retry with sched_param fixup only when we saw that exact error and there are no missing headers.
                    if not force_sched_param and not missing_files and _needs_sched_param_fixup(error_msg):
                        need_sched_param_retry = True
                        break

                    # Some profiles (e.g. csky mini targets) export -mcpu=ck802 but miss a matching --target=,
                    # causing host libclang to reject the CPU. As a fallback, strip arch flags and retry.
                    if (not missing_files) and (not arch_flags_stripped) and _needs_target_cpu_fixup(error_msg):
                        has_target = any(
                            isinstance(f, str) and (f.startswith("--target=") or f == "-target")
                            for f in (context_flags or [])
                        )
                        if not has_target:
                            context_flags = _strip_arch_flags(context_flags)
                            arch_flags_stripped = True
                            wrapper_fixup_actions.append({
                                "action": "strip_arch_flags",
                                "reason": "unknown_target_cpu",
                            })
                            try:
                                attempt_debug["arch_flags_stripped"] = True
                            except Exception:
                                pass
                            continue

                    # Auto-fix: unknown type name usually means missing transitive includes or wrong include order.
                    if (not missing_files) and (unknown_fixup_rounds < max_unknown_fixup_rounds):
                        unknown_records = _extract_unknown_type_records(error_msg)
                        if unknown_records:
                            def _pick_best_pthread_include() -> str:
                                """
                                Pick a pthread.h that matches the current include context.

                                OpenHarmony ships multiple musl variants (porting/*) where <pthread.h> must be
                                consistent with <bits/alltypes.h>. If we accidentally include the generic
                                third_party/musl/include/pthread.h together with a porting/* alltypes.h, clang
                                will report many "unknown type name pthread_*" errors.
                                """
                                try:
                                    candidates: List[Path] = []
                                    # Prefer headers that already exist in current include dirs (most accurate).
                                    for d in (include_dirs or set()):
                                        try:
                                            ds = str(d).replace("\\", "/").lower()
                                        except Exception:
                                            continue
                                        if "pthread" not in ds and "musl" not in ds and "/usr/include" not in ds:
                                            continue
                                        cand = Path(d) / "pthread.h"
                                        if cand.exists():
                                            candidates.append(cand)

                                    # Fallback: well-known source-tree locations.
                                    if not candidates and self.ohos_root:
                                        ohos_root = Path(self.ohos_root)
                                        for cand in [
                                            ohos_root / "third_party" / "musl" / "porting" / "liteos_a" / "kernel" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "liteos_a_newlib" / "kernel" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "liteos_m" / "kernel" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "liteos_m" / "user" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "liteos_m_iccarm" / "kernel" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "uniproton" / "kernel" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "porting" / "linux" / "user" / "include" / "pthread.h",
                                            ohos_root / "third_party" / "musl" / "include" / "pthread.h",
                                        ]:
                                            if cand.exists():
                                                candidates.append(cand)

                                    if not candidates:
                                        return "<pthread.h>"

                                    def _score(p: Path) -> int:
                                        s = str(p).replace("\\", "/").lower()
                                        score = 0
                                        if "/porting/" in s:
                                            score += 10_000
                                        if "/liteos_a/" in s:
                                            score += 800
                                        if "/liteos_m" in s:
                                            score += 600
                                        if "/uniproton" in s:
                                            score += 500
                                        if "/kernel/" in s:
                                            score += 120
                                        if "/user/" in s:
                                            score += 80
                                        if "/usr/include" in s:
                                            score += 40
                                        if s.endswith("/pthread.h"):
                                            score += 5
                                        return score

                                    best = max(candidates, key=_score)
                                    return f"\"{best}\""
                                except Exception:
                                    return "<pthread.h>"

                            pthread_inc = _pick_best_pthread_include()
                            sys_map = {
                                # Prefer a matching pthread.h (see _pick_best_pthread_include above).
                                "pthread_mutex_t": pthread_inc,
                                "pthread_mutexattr_t": pthread_inc,
                                "pthread_cond_t": pthread_inc,
                                "pthread_condattr_t": pthread_inc,
                                "pthread_attr_t": pthread_inc,
                                "pthread_rwlock_t": pthread_inc,
                                "pthread_rwlockattr_t": pthread_inc,
                                "pthread_barrier_t": pthread_inc,
                                "pthread_barrierattr_t": pthread_inc,
                                "pthread_spinlock_t": pthread_inc,
                                "pthread_key_t": pthread_inc,
                                "pthread_once_t": pthread_inc,
                                # Kernel-style types (often missing in partial build contexts)
                                "wait_queue_head_t": "<linux/wait.h>",
                                "poll_table": "<linux/poll.h>",
                                "gfp_t": "<linux/types.h>",
                                "refcount_t": "<linux/refcount.h>",
                                # LwIP (network stack) common typedefs
                                "u8_t": "<lwip/arch.h>",
                                "u16_t": "<lwip/arch.h>",
                                "u32_t": "<lwip/arch.h>",
                                "u64_t": "<lwip/arch.h>",
                                "s8_t": "<lwip/arch.h>",
                                "s16_t": "<lwip/arch.h>",
                                "s32_t": "<lwip/arch.h>",
                                "s64_t": "<lwip/arch.h>",
                                "err_t": "<lwip/err.h>",
                                "ip_addr_t": "<lwip/ip_addr.h>",
                                # Some projects use STATIC as a macro-like storage specifier.
                                # Define it via wrapper macros (handled in _write_wrapper).
                            }

                            added_any = False
                            for _f, t in unknown_records:
                                inc = sys_map.get(t)
                                if not inc and isinstance(t, str) and t.startswith("pthread_"):
                                    inc = pthread_inc
                                # Prefer absolute includes from the OpenHarmony tree to avoid sysroot/uapi collisions.
                                # Example: sysroot's <linux/wait.h> is UAPI and may not define wait_queue_head_t,
                                # while the kernel headers do.
                                if self.ohos_root:
                                    try:
                                        ohos_root = Path(self.ohos_root)
                                        if t == "wait_queue_head_t":
                                            for cand in [
                                                ohos_root / "kernel" / "linux" / "linux-5.10" / "include" / "linux" / "wait.h",
                                                ohos_root / "kernel" / "linux" / "linux-6.6" / "include" / "linux" / "wait.h",
                                            ]:
                                                if cand.exists():
                                                    inc = f"\"{cand}\""
                                                    break
                                        elif t == "gfp_t":
                                            for cand in [
                                                ohos_root / "kernel" / "linux" / "linux-5.10" / "include" / "linux" / "types.h",
                                                ohos_root / "kernel" / "linux" / "linux-6.6" / "include" / "linux" / "types.h",
                                            ]:
                                                if cand.exists():
                                                    inc = f"\"{cand}\""
                                                    break
                                        elif t == "poll_table":
                                            for cand in [
                                                ohos_root / "kernel" / "linux" / "linux-5.10" / "include" / "linux" / "poll.h",
                                                ohos_root / "kernel" / "linux" / "linux-6.6" / "include" / "linux" / "poll.h",
                                            ]:
                                                if cand.exists():
                                                    inc = f"\"{cand}\""
                                                    break
                                        elif t == "refcount_t":
                                            for cand in [
                                                ohos_root / "kernel" / "linux" / "linux-5.10" / "include" / "linux" / "refcount.h",
                                                ohos_root / "kernel" / "linux" / "linux-6.6" / "include" / "linux" / "refcount.h",
                                            ]:
                                                if cand.exists():
                                                    inc = f"\"{cand}\""
                                                    break
                                        elif t in {"u8_t", "u16_t", "u32_t", "u64_t", "s8_t", "s16_t", "s32_t", "s64_t"}:
                                            cand = ohos_root / "third_party" / "lwip" / "src" / "include" / "lwip" / "arch.h"
                                            if cand.exists():
                                                inc = f"\"{cand}\""
                                        elif t == "err_t":
                                            cand = ohos_root / "third_party" / "lwip" / "src" / "include" / "lwip" / "err.h"
                                            if cand.exists():
                                                inc = f"\"{cand}\""
                                        elif t == "ip_addr_t":
                                            cand = ohos_root / "third_party" / "lwip" / "src" / "include" / "lwip" / "ip_addr.h"
                                            if cand.exists():
                                                inc = f"\"{cand}\""
                                    except Exception:
                                        pass
                                if inc and inc not in wrapper_extra_system_includes:
                                    wrapper_extra_system_includes.append(inc)
                                    wrapper_fixup_actions.append({
                                        "action": "add_system_include",
                                        "include": inc,
                                        "type": t,
                                        "reason": "unknown_type_name",
                                    })
                                    added_any = True

                            current_headers = list(wrapper_header_order)
                            for f, t in unknown_records:
                                failing_basename = ""
                                try:
                                    failing_basename = Path(f).name
                                except Exception:
                                    failing_basename = ""
                                failing_header = None
                                if failing_basename:
                                    for h in current_headers:
                                        if h.name == failing_basename:
                                            failing_header = h
                                            break
                                defining_header = _find_defining_header_for_type(t, current_headers)
                                if defining_header and failing_header:
                                    current_headers, moved = _move_header_before(current_headers, defining_header, failing_header)
                                    if moved:
                                        wrapper_fixup_actions.append({
                                            "action": "reorder_headers",
                                            "move": str(defining_header),
                                            "before": str(failing_header),
                                            "type": t,
                                            "reason": "unknown_type_name",
                                        })
                                        added_any = True

                            if added_any:
                                wrapper_header_order = current_headers
                                unknown_fixup_rounds += 1
                                try:
                                    last_actions = wrapper_fixup_actions[-10:]
                                    logger.info(
                                        "Bindgen wrapper fixup applied "
                                        f"(round={unknown_fixup_rounds}, sys_includes={wrapper_extra_system_includes}): "
                                        f"{last_actions}"
                                    )
                                except Exception:
                                    pass
                                _write_wrapper(
                                    force_need_struct_sched_param=bool(force_sched_param),
                                    extra_system_includes=wrapper_extra_system_includes,
                                    ordered_headers=wrapper_header_order,
                                )
                                continue

                    # Store the most relevant failure attempt for downstream retries and reports.
                    attempt_debug["bindgen_cmd_len"] = len(best_failure.get("cmd") or [])
                    attempt_debug["bindgen_cmd_head"] = (best_failure.get("cmd") or [])[:25]
                    if bindgen_debug:
                        attempt_debug["bindgen_cmd"] = best_failure.get("cmd") or []

                    if missing_files:
                        # If headers are missing under out/.../gen/..., try to materialize generated artifacts via ninja once.
                        if (not gen_ensure_tried) and self._maybe_generate_ohos_out_gen_artifacts(attempt_debug):
                            gen_ensure_tried = True
                            try:
                                logger.info("已尝试通过 ninja 生成 out/.../gen/... 生成物，重试 bindgen...")
                            except Exception:
                                pass
                            continue
                        logger.warning(f"bindgen 失败: 缺少外部头文件 {missing_files[:5]} (lang={best_failure.get('lang')})")
                        print(f"  缺少的头文件: {', '.join(missing_files[:10])}")
                        attempt_debug["missing_headers"] = list(missing_files)
                        attempt_debug["missing_headers_count"] = len(missing_files)
                        if not bindgen_debug:
                            tu_ctx = attempt_debug.get("tu_context") or {}
                            match_reason = (tu_ctx.get("match_info") or {}).get("reason") if isinstance(tu_ctx.get("match_info"), dict) else None
                            first_inc = (attempt_debug.get("clang_mode") or {}).get("attempts", [{}])[0].get("include_dirs", {})
                            print(
                                f"  [BindgenDiag] cc_loaded={bool(self.compile_commands_parser)} "
                                f"match={match_reason} include_dirs={first_inc.get('unique_dir_count')} "
                                f"missing_include_dirs={first_inc.get('nonexistent_dir_count')} out_gen_dirs={first_inc.get('out_gen_dir_count')}"
                            )
                        return False, error_msg, missing_files, attempt_debug

                    # Non-missing-header failures are very common (wrong TU context, sysroot not generated, C vs C++ mismatch).
                    # Emit a compact one-line diagnostic to make log triage easier.
                    if not bindgen_debug:
                        tu_ctx = attempt_debug.get("tu_context") or {}
                        match_reason = (tu_ctx.get("match_info") or {}).get("reason") if isinstance(tu_ctx.get("match_info"), dict) else None
                        clang_sum = (tu_ctx.get("clang_flags_summary") or {}) if isinstance(tu_ctx.get("clang_flags_summary"), dict) else {}
                        dropped = tu_ctx.get("include_dirs_dropped_count")
                        dropped_out_gen = tu_ctx.get("include_dirs_dropped_out_gen_count")
                        print(
                            f"  [BindgenDiag] cc_loaded={bool(self.compile_commands_parser)} match={match_reason} "
                            f"lang={best_failure.get('lang')} sysroot={clang_sum.get('sysroot')} target={clang_sum.get('target')} "
                            f"I={clang_sum.get('I')} isystem={clang_sum.get('isystem')} dropped_includes={dropped} "
                            f"dropped_out_gen={dropped_out_gen} host_suspect={bool(tu_ctx.get('used_proxy'))}"
                        )

                    logger.warning(f"bindgen 失败: {error_msg[:300]}")
                    return False, error_msg, [], attempt_debug

                if need_sched_param_retry:
                    continue
                
        except subprocess.TimeoutExpired:
            logger.warning("bindgen 超时")
            attempt_debug["bindgen_timeout"] = True
            return False, "Timeout", [], attempt_debug
        except Exception as e:
            logger.error(f"bindgen 异常: {e}")
            attempt_debug["bindgen_exception"] = str(e)[:500]
            return False, str(e), [], attempt_debug
        finally:
            # 清理 wrapper.h
            if wrapper_path.exists() and not keep_files:
                wrapper_path.unlink()
    
    def _postprocess_bindgen_output(self, types_file: Path):
        """后处理 bindgen 输出，清理不必要的内容"""
        # Truth-mode: keep bindgen output untouched (no regex deletion, no enum const aliases).
        # This makes `types.rs` a pure bindgen artifact, and lets compile errors expose TU/input-closure gaps.
        if self._env_flag("C2R_TRUTH_MODE", "0") or (not self._env_flag("C2R_ENABLE_BINDGEN_POSTPROCESS", "1")):
            logger.debug("bindgen types.rs postprocess skipped (truth-mode or disabled)")
            return

        if not types_file.exists():
            return
        
        with open(types_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 移除空的 extern "C" 块
        content = re.sub(r'extern\s+"C"\s*\{\s*\}', '', content)
        
        # 移除不必要的系统常量定义（这些通常是从系统头文件引入的）
        # 例如：pub const _STDINT_H: u32 = 1; 等
        system_const_patterns = [
            r'pub const _\w+_H: u32 = \d+;',  # _STDINT_H, _FEATURES_H 等
            r'pub const __\w+: u32 = \d+;',   # __GLIBC__, __WORDSIZE 等
            r'pub const _\w+SOURCE\w*: u32 = \d+;',  # _POSIX_SOURCE 等
            r'pub const __USE_\w+: u32 = \d+;',  # __USE_POSIX 等
            r'pub const __GLIBC_USE_\w+: u32 = \d+;',  # __GLIBC_USE_xxx
            r'pub const __STDC_\w+: u32 = \d+;',  # __STDC_xxx
            r'pub const __GNU_\w+: u32 = \d+;',  # __GNU_xxx
            r'pub const __HAVE_\w+: u32 = \d+;',  # __HAVE_xxx
            r'pub const __LDOUBLE_\w+: u32 = \d+;',
            r'pub const __OFF_\w+: u32 = \d+;',
            r'pub const __INO_\w+: u32 = \d+;',
            r'pub const __RLIM_\w+: u32 = \d+;',
            r'pub const __STATFS_\w+: u32 = \d+;',
            r'pub const __KERNEL_\w+: u32 = \d+;',
            r'pub const __FD_SETSIZE: u32 = \d+;',
            r'pub const __TIMESIZE: u32 = \d+;',
            r'pub const __SYSCALL_\w+: u32 = \d+;',
            r'pub const __WORDSIZE\w*: u32 = \d+;',
            r'pub const __glibc_\w+: u32 = \d+;',
        ]
        for pattern in system_const_patterns:
            content = re.sub(pattern + r'\n?', '', content)
        
        # 移除不必要的系统类型别名（以 __ 开头的内部类型）
        # 但保留可能有用的类型（如 __fsid_t 结构体）
        system_type_patterns = [
            r'pub type __u_char = [^;]+;\n?',
            r'pub type __u_short = [^;]+;\n?',
            r'pub type __u_int = [^;]+;\n?',
            r'pub type __u_long = [^;]+;\n?',
            r'pub type __int\d+_t = [^;]+;\n?',
            r'pub type __uint\d+_t = [^;]+;\n?',
            r'pub type __int_least\d+_t = [^;]+;\n?',
            r'pub type __uint_least\d+_t = [^;]+;\n?',
            r'pub type __quad_t = [^;]+;\n?',
            r'pub type __u_quad_t = [^;]+;\n?',
            r'pub type __intmax_t = [^;]+;\n?',
            r'pub type __uintmax_t = [^;]+;\n?',
            r'pub type __dev_t = [^;]+;\n?',
            r'pub type __uid_t = [^;]+;\n?',
            r'pub type __gid_t = [^;]+;\n?',
            r'pub type __ino\d*_t = [^;]+;\n?',
            r'pub type __mode_t = [^;]+;\n?',
            r'pub type __nlink_t = [^;]+;\n?',
            r'pub type __off\d*_t = [^;]+;\n?',
            r'pub type __pid_t = [^;]+;\n?',
            r'pub type __clock_t = [^;]+;\n?',
            r'pub type __rlim\d*_t = [^;]+;\n?',
            r'pub type __id_t = [^;]+;\n?',
            r'pub type __time_t = [^;]+;\n?',
            r'pub type __useconds_t = [^;]+;\n?',
            r'pub type __suseconds\d*_t = [^;]+;\n?',
            r'pub type __daddr_t = [^;]+;\n?',
            r'pub type __key_t = [^;]+;\n?',
            r'pub type __clockid_t = [^;]+;\n?',
            r'pub type __timer_t = [^;]+;\n?',
            r'pub type __blksize_t = [^;]+;\n?',
            r'pub type __blkcnt\d*_t = [^;]+;\n?',
            r'pub type __fsblkcnt\d*_t = [^;]+;\n?',
            r'pub type __fsfilcnt\d*_t = [^;]+;\n?',
            r'pub type __fsword_t = [^;]+;\n?',
            r'pub type __ssize_t = [^;]+;\n?',
            r'pub type __syscall_\w+_t = [^;]+;\n?',
            r'pub type __loff_t = [^;]+;\n?',
            r'pub type __caddr_t = [^;]+;\n?',
            r'pub type __intptr_t = [^;]+;\n?',
            r'pub type __socklen_t = [^;]+;\n?',
            r'pub type __sig_atomic_t = [^;]+;\n?',
        ]
        for pattern in system_type_patterns:
            content = re.sub(pattern, '', content)
        
        # 移除重复的 #![allow(...)]
        seen_allows = set()
        lines = content.split('\n')
        filtered_lines = []
        for line in lines:
            if line.strip().startswith('#![allow'):
                if line.strip() not in seen_allows:
                    seen_allows.add(line.strip())
                    filtered_lines.append(line)
            else:
                filtered_lines.append(line)
        content = '\n'.join(filtered_lines)
        
        # 确保文件开头有必要的 allow 属性
        required_allows = [
            '#![allow(non_camel_case_types)]',
            '#![allow(non_snake_case)]',
            '#![allow(non_upper_case_globals)]',
            '#![allow(dead_code)]',
            '#![allow(unused)]',
        ]
        
        # 检查是否已有这些属性
        existing_allows = set()
        for line in content.split('\n'):
            if line.strip().startswith('#![allow'):
                existing_allows.add(line.strip())
        
        # 添加缺失的属性
        missing_allows = []
        for allow in required_allows:
            if allow not in existing_allows:
                missing_allows.append(allow)
        
        if missing_allows:
            # 在文件开头添加缺失的 allow 属性
            # 找到 bindgen 的注释行之后插入
            insert_pos = 0
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.strip().startswith('/*') or line.strip().startswith('//'):
                    insert_pos = i + 1
                elif line.strip() and not line.strip().startswith('#'):
                    break
            
            for allow in reversed(missing_allows):
                lines.insert(insert_pos, allow)
            
            content = '\n'.join(lines)
        
        # ========== 增强：为枚举变体生成常量别名 ==========
        # C 代码中枚举值可以直接使用（如 SOFTBUS_OK），但 Rust 需要枚举前缀（如 SoftBusErrNo::SOFTBUS_OK）
        # 这里提取枚举变体并生成常量别名，使翻译后的代码更接近 C 的使用方式
        enum_aliases = self._generate_enum_const_aliases(content)
        if enum_aliases:
            # 在文件末尾添加枚举常量别名
            content = content.rstrip() + "\n\n" + "// ========== 枚举常量别名（方便 C 风格的直接使用）==========\n"
            content += enum_aliases
            print(f"  [后处理] 生成了 {enum_aliases.count('pub const')} 个枚举常量别名")
        
        # 清理多余空行
        content = re.sub(r'\n{3,}', '\n\n', content)
        
        with open(types_file, 'w', encoding='utf-8') as f:
            f.write(content)
    
    def _generate_enum_const_aliases(self, content: str) -> str:
        """
        为枚举变体生成常量别名
        
        C 代码中枚举值可以直接使用（如 SOFTBUS_OK），但 Rust 需要枚举前缀。
        这个函数提取枚举定义，并为每个变体生成对应的常量别名。
        
        例如：
        - enum SoftBusErrNo { SOFTBUS_OK = 0, ... }
        - 生成: pub const SOFTBUS_OK: i32 = SoftBusErrNo::SOFTBUS_OK as i32;
        
        Args:
            content: types.rs 的内容
        
        Returns:
            常量别名定义字符串
        """
        aliases = []
        
        # 匹配枚举定义
        # pub enum EnumName { ... }
        enum_pattern = re.compile(
            r'pub\s+enum\s+(\w+)\s*\{([^}]+)\}',
            re.DOTALL
        )
        
        # 匹配枚举变体
        # VARIANT_NAME = value, 或 VARIANT_NAME,
        variant_pattern = re.compile(
            r'(\w+)\s*(?:=\s*(-?\d+))?',
        )
        
        for enum_match in enum_pattern.finditer(content):
            enum_name = enum_match.group(1)
            enum_body = enum_match.group(2)
            
            # 跳过某些系统枚举
            if enum_name.startswith('_') or enum_name.startswith('__'):
                continue
            
            # 提取变体
            for variant_match in variant_pattern.finditer(enum_body):
                variant_name = variant_match.group(1)
                
                # 跳过空白匹配和无效名称
                if not variant_name or variant_name in ('pub', 'impl', 'fn', 'struct'):
                    continue
                
                # 只为全大写的变体生成别名（这通常是 C 风格的常量命名）
                if not variant_name.isupper() and '_' not in variant_name:
                    continue
                
                # 检查是否已经有同名的常量定义
                existing_const = re.search(rf'pub\s+const\s+{variant_name}\s*:', content)
                if existing_const:
                    continue
                
                # 生成常量别名
                alias = f"pub const {variant_name}: i32 = {enum_name}::{variant_name} as i32;"
                aliases.append(alias)
        
        return '\n'.join(aliases)
    
    def _try_llm_bindgen_fallback(self, header_files: List[Path], output_path: Path) -> bool:
        """
        bindgen 失败时的 LLM 兜底
        
        尝试使用 LLM 从头文件生成 Rust 类型定义。
        给 LLM 提供完整的上下文信息以确保生成正确的 FFI 类型。
        
        Args:
            header_files: 头文件列表
            output_path: 输出文件路径
        
        Returns:
            是否成功
        """
        import os

        # 检查是否配置了 LLM - 复用 generation.py 的配置
        llm_client = None
        try:
            from generate.generation import (
                USE_VLLM, VLLM_BASE_URL, VLLM_API_KEY, VLLM_MODEL_NAME, VLLM_REQUEST_TIMEOUT,
                EXTERNAL_API_BASE_URL, EXTERNAL_API_KEY, EXTERNAL_API_MODEL, EXTERNAL_API_TIMEOUT
            )
            if USE_VLLM:
                llm_model = VLLM_MODEL_NAME
                api_base = VLLM_BASE_URL
                api_key = VLLM_API_KEY
                timeout = VLLM_REQUEST_TIMEOUT
            else:
                llm_model = EXTERNAL_API_MODEL
                api_base = EXTERNAL_API_BASE_URL
                api_key = EXTERNAL_API_KEY
                timeout = EXTERNAL_API_TIMEOUT
        except ImportError:
            llm_model = os.environ.get("LLM_NAME", "qwen3_coder")
            api_base = os.environ.get("OPENAI_API_BASE", "http://localhost:8000/v1")
            api_key = "dummy"
            timeout = 600.0

        try:
            from openai import OpenAI
            llm_client = OpenAI(base_url=api_base, api_key=api_key, timeout=timeout)
            # 测试连接
            llm_client.models.list()
        except Exception as e:
            logger.warning(f"LLM 客户端初始化失败，跳过 LLM 兜底: {e}")
            return False
        
        try:
            from llm_signature_extractor import LLMBindgenFallback
            
            # 读取所有头文件内容
            header_content = []
            for hf in header_files:
                try:
                    with open(hf, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        header_content.append(f"// ========== {hf.name} ==========\n{content}")
                except Exception as e:
                    logger.warning(f"无法读取头文件 {hf}: {e}")
            
            if not header_content:
                logger.warning("没有可读取的头文件，跳过 LLM 兜底")
                return False
            
            combined_headers = '\n\n'.join(header_content)
            
            # 调用 LLM 生成类型定义
            print(f"  🤖 调用 LLM 生成类型定义（{len(header_files)} 个头文件）...")
            fallback = LLMBindgenFallback(llm_client, llm_model)
            rust_types = fallback.generate_types(combined_headers)
            
            if rust_types and len(rust_types) > 100:  # 确保生成了有意义的内容
                # 确保输出目录存在
                output_path.parent.mkdir(parents=True, exist_ok=True)
                
                # 写入生成的类型定义
                with open(output_path, 'w', encoding='utf-8') as f:
                    f.write(rust_types)
                
                logger.info(f"LLM bindgen 兜底成功，生成了 {len(rust_types)} 字节到 {output_path}")
                return True
            else:
                logger.warning("LLM 返回的类型定义内容不足")
                return False
                
        except ImportError as e:
            logger.warning(f"无法导入 LLM 模块: {e}")
            return False
        except Exception as e:
            logger.error(f"LLM bindgen 兜底异常: {e}")
            return False
    
    def _create_placeholder_types(self, output_path: Path):
        """创建占位类型文件（使用可配置的预定义）"""
        
        # 基础头部
        header = '''//! Auto-generated type definitions
//! 
//! Note: bindgen failed to generate types. These are placeholder definitions.
//! External library types are declared as opaque types for compilation.
//! 
//! Source: config/predefines.py (self-adaptive learning enabled)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused)]

use std::os::raw::*;

// Common C type aliases
pub type c_void = std::ffi::c_void;
pub type size_t = usize;
pub type ssize_t = isize;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;

// Common integer types
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// Boolean type
pub type BOOL = i32;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub const NULL: *mut c_void = std::ptr::null_mut();

'''
        lines = [header]
        
        # 从配置获取类型和常量
        if PREDEFINES_AVAILABLE:
            manager = get_predefine_manager(enable_ohos=True)
            
            # 添加类型定义
            lines.append("// ============================================================")
            lines.append("// Type Definitions (from config/predefines.py)")
            lines.append("// ============================================================")
            lines.append("")
            
            for name, definition in sorted(manager.get_all_types().items()):
                lines.append(definition)
                lines.append("")
            
            # 添加常量定义
            lines.append("// ============================================================")
            lines.append("// Constants (from config/predefines.py)")
            lines.append("// ============================================================")
            lines.append("")
            
            # 跳过已在 header 中定义的常量，避免重复定义
            header_defined_constants = {"TRUE", "FALSE", "NULL"}
            
            for name, rust_type, value in sorted(manager.get_all_constants(), key=lambda x: x[0]):
                if name not in header_defined_constants:
                    lines.append(f"pub const {name}: {rust_type} = {value};")
            
            lines.append("")
        else:
            # 回退到硬编码（兼容性）
            lines.append(self._get_fallback_placeholder_content())
        
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
    
    def _get_fallback_placeholder_content(self) -> str:
        """获取回退的占位内容（当配置模块不可用时）"""
        return '''
// ============================================================
// POSIX Error Codes
// ============================================================
pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;
pub const EAGAIN: i32 = 11;
pub const ETIMEDOUT: i32 = 110;

// ============================================================
// POSIX/System Types
// ============================================================
#[repr(C)]
pub struct pthread_mutex_t { _opaque: [u8; 40] }
#[repr(C)]
pub struct pthread_cond_t { _opaque: [u8; 48] }
pub type pthread_t = usize;
pub type atomic_bool = u8;
pub type atomic_int = i32;

#[repr(C)]
pub struct file { _opaque: [u8; 0] }
#[repr(C)]
pub struct FILE { _opaque: [u8; 0] }
pub type time_t = i64;
pub type off_t = i64;
pub type mode_t = u32;

'''
    
    def detect_and_generate_missing_symbols(
        self,
        source_files: List[Path],
        types_file: Path
    ):
        """
        检测源代码中使用但骨架中未定义的外部符号，并生成占位声明
        
        分析 C 源代码中的：
        1. 函数调用 - 生成 extern "C" 占位
        2. 宏常量使用 - 生成 pub const 占位
        
        Args:
            source_files: C 源文件列表
            types_file: types.rs 文件路径
        """
        # 1. 读取现有的 types.rs，提取已定义的符号
        existing_symbols = set()
        if types_file.exists():
            with open(types_file, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            # 提取已定义的类型、常量、函数
            existing_symbols.update(re.findall(r'pub\s+(?:type|struct|enum|const|fn)\s+(\w+)', content))
            existing_symbols.update(re.findall(r'type\s+(\w+)\s*=', content))
        
        # 2. 分析源代码，收集使用的符号
        used_functions = set()
        used_constants = set()

        for src_file in source_files:
            try:
                # 修复：使用预处理后的代码（与阶段 B/C 保持一致）
                # 如果 .c 文件只有 #include "xxx.h"，原始文件几乎为空
                source_code = None
                try:
                    safe_name = self._get_safe_module_name(src_file)
                    rec = (getattr(self, "_tu_context_files", {}) or {}).get(safe_name)
                    pre_path = (rec or {}).get("preprocessed_file") if isinstance(rec, dict) else None
                    if pre_path:
                        p = Path(str(pre_path)).expanduser()
                        if p.exists():
                            source_code = p.read_text(encoding="utf-8", errors="ignore")
                except Exception:
                    source_code = None

                if not source_code:
                    source_code = self.preprocess_source(src_file)

                # 提取函数调用（简单启发式）
                # 匹配模式：identifier( 但不是关键字
                func_calls = re.findall(r'\b([A-Z][A-Za-z0-9_]*)\s*\(', source_code)
                func_calls += re.findall(r'\b([a-z][A-Za-z0-9_]*_[A-Za-z0-9_]*)\s*\(', source_code)  # 蛇形命名
                used_functions.update(func_calls)

                # 提取可能的常量使用（大写字母命名）
                # 匹配模式：全大写或大写开头的标识符
                constants = re.findall(r'\b([A-Z][A-Z0-9_]{2,})\b', source_code)
                used_constants.update(constants)

            except Exception as e:
                logger.warning(f"分析源文件 {src_file.name} 失败: {e}")
        
        # 3. 过滤掉已存在的和明显的系统符号
        c_keywords = {'NULL', 'TRUE', 'FALSE', 'EOF', 'SEEK_SET', 'SEEK_CUR', 'SEEK_END'}
        existing_constants = existing_symbols | c_keywords
        
        # 过滤外部函数
        external_functions = used_functions - existing_symbols
        # C 标准库函数的正确 FFI 签名（用于自动生成）
        # 这些函数在 C 代码中经常使用，需要在 Rust 中有正确的 FFI 声明
        c_stdlib_signatures = {
            # 内存管理
            'malloc': 'pub fn malloc(size: usize) -> *mut core::ffi::c_void',
            'free': 'pub fn free(ptr: *mut core::ffi::c_void)',
            'calloc': 'pub fn calloc(nmemb: usize, size: usize) -> *mut core::ffi::c_void',
            'realloc': 'pub fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void',
            # 内存操作
            'memcpy': 'pub fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void',
            'memset': 'pub fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void',
            'memmove': 'pub fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void',
            'memcmp': 'pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32',
            # 字符串操作
            'strcpy': 'pub fn strcpy(dest: *mut i8, src: *const i8) -> *mut i8',
            'strncpy': 'pub fn strncpy(dest: *mut i8, src: *const i8, n: usize) -> *mut i8',
            'strcmp': 'pub fn strcmp(s1: *const i8, s2: *const i8) -> i32',
            'strncmp': 'pub fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32',
            'strlen': 'pub fn strlen(s: *const i8) -> usize',
            'strcat': 'pub fn strcat(dest: *mut i8, src: *const i8) -> *mut i8',
            'strncat': 'pub fn strncat(dest: *mut i8, src: *const i8, n: usize) -> *mut i8',
            'strdup': 'pub fn strdup(s: *const i8) -> *mut i8',
            'strstr': 'pub fn strstr(haystack: *const i8, needle: *const i8) -> *mut i8',
            'strchr': 'pub fn strchr(s: *const i8, c: i32) -> *mut i8',
            'strrchr': 'pub fn strrchr(s: *const i8, c: i32) -> *mut i8',
            # I/O
            'printf': 'pub fn printf(format: *const i8, ...) -> i32',
            'sprintf': 'pub fn sprintf(str: *mut i8, format: *const i8, ...) -> i32',
            'snprintf': 'pub fn snprintf(str: *mut i8, size: usize, format: *const i8, ...) -> i32',
            'fprintf': 'pub fn fprintf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32',
            'sscanf': 'pub fn sscanf(str: *const i8, format: *const i8, ...) -> i32',
            # 数值转换
            'atoi': 'pub fn atoi(nptr: *const i8) -> i32',
            'atol': 'pub fn atol(nptr: *const i8) -> i64',
            'atof': 'pub fn atof(nptr: *const i8) -> f64',
            'strtol': 'pub fn strtol(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> i64',
            'strtoul': 'pub fn strtoul(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u64',
            # 其他
            'abs': 'pub fn abs(x: i32) -> i32',
            'exit': 'pub fn exit(status: i32) -> !',
            'abort': 'pub fn abort() -> !',
            'sleep': 'pub fn sleep(seconds: u32) -> u32',
            'usleep': 'pub fn usleep(usec: u32) -> i32',
        }
        
        # 检测使用了哪些 C 标准库函数（需要添加到 types.rs）
        used_stdlib_funcs = used_functions & set(c_stdlib_signatures.keys())
        
        # 过滤掉已经有正确签名的标准库函数，因为它们会单独处理
        external_functions = external_functions - set(c_stdlib_signatures.keys())
        
        # 过滤外部常量
        external_constants = used_constants - existing_constants
        # 排除明显不是常量的（如类型名）
        external_constants = {c for c in external_constants 
                            if len(c) >= 3 and not c.startswith('_')}
        
        # 4. 生成占位声明并追加到 types.rs
        # 检查是否有任何需要添加的内容
        has_content_to_add = external_functions or external_constants or used_stdlib_funcs
        
        if has_content_to_add:
            additions = [
                "",
                "// ============================================================",
                "// Auto-detected External Symbols",
                "// These symbols were detected in C source code.",
                "// ============================================================",
                "",
            ]
            
            # 生成常量占位
            if external_constants:
                additions.append("// --- External Constants (detected from source) ---")
                # 常见的 OpenHarmony/HDF 常量前缀映射
                const_prefixes = {
                    'HDF_': ('i32', '0'),
                    'LOS_': ('u32', '0'),
                    'SOFTBUS_': ('i32', '0'),
                    'AUDIO_': ('i32', '0'),
                    'HNP_': ('i32', '0'),
                    'E': ('i32', '0'),  # EINVAL, ENOMEM 等
                    'O_': ('u32', '0'),  # O_RDONLY 等
                    'S_': ('u32', '0'),  # S_IRUSR 等
                }
                
                for const in sorted(external_constants)[:50]:  # 限制数量
                    const_type = 'i32'
                    const_val = '0'
                    for prefix, (ctype, cval) in const_prefixes.items():
                        if const.startswith(prefix):
                            const_type = ctype
                            const_val = cval
                            break
                    # 跳过已在占位中定义的常量
                    if const not in existing_symbols:
                        additions.append(f"pub const {const}: {const_type} = {const_val};  // placeholder")
                additions.append("")
            
            # 生成 C 标准库函数的正确 FFI 声明
            if used_stdlib_funcs:
                # 过滤掉已经存在于 types.rs 中的函数
                stdlib_to_add = {f for f in used_stdlib_funcs if f not in existing_symbols}
                if stdlib_to_add:
                    additions.append("// --- C Standard Library Functions (with correct signatures) ---")
                    additions.append("extern \"C\" {")
                    for func in sorted(stdlib_to_add):
                        if func in c_stdlib_signatures:
                            additions.append(f"    {c_stdlib_signatures[func]};")
                    additions.append("}")
                    additions.append("")
            
            # 生成其他外部函数占位（extern "C" 块）
            if external_functions:
                additions.append("// --- Project-specific External Functions (placeholders) ---")
                additions.append("// NOTE: These are declared as extern \"C\" for FFI compatibility.")
                additions.append("// The actual signatures may need to be corrected manually.")
                additions.append("extern \"C\" {")
                
                for func in sorted(external_functions)[:30]:  # 限制数量
                    # 简单的函数签名占位（返回 i32，无参数）
                    additions.append(f"    pub fn {func}() -> i32;  // placeholder - actual signature may differ")
                
                additions.append("}")
                additions.append("")
            
            # 追加到 types.rs
            with open(types_file, 'a', encoding='utf-8') as f:
                f.write('\n'.join(additions))
            
            stdlib_count = len(used_stdlib_funcs) if used_stdlib_funcs else 0
            logger.info(f"自动检测并生成 {len(external_constants)} 个常量 + {stdlib_count} 个标准库函数 + {len(external_functions)} 个外部函数占位")
    
    # =========================================================================
    # 阶段 B: 全局状态层 (The State Layer)
    # =========================================================================
    
    def extract_variables_with_treesitter(
        self, 
        source_code: str, 
        file_name: str = ""
    ) -> List[VariableInfo]:
        """
        使用 Tree-sitter 精确提取全局变量和函数内 static 变量
        
        参考 PTRMAPPER 的全局指针分析和 EvoC2Rust 的变量封装
        
        Args:
            source_code: C/C++ 源代码
            file_name: 文件名（用于生成唯一变量名）
        
        Returns:
            变量信息列表
        """
        variables = []
        seen_names = set()
        
        try:
            tree = cpp_parser.parse(bytes(source_code, 'utf-8'))
            root = tree.root_node
            
            # 关键检查：统计 ERROR 节点数量
            # 参考 EvoC2Rust: 如果有太多 ERROR，说明解析不可靠
            error_count = self._count_parse_errors(root)
            total_nodes = len(source_code) // 50  # 粗略估计节点数
            error_rate = error_count / max(total_nodes, 1)
            
            if error_count > 10 or error_rate > 0.1:
                logger.warning(
                    f"Tree-sitter 解析质量差: {file_name} 有 {error_count} 个 ERROR 节点 "
                    f"(错误率 {error_rate:.1%})，跳过此文件的变量提取"
                )
                # 当解析质量差时，直接返回空列表，不要提取垃圾数据
                return []
            
            # 1. 提取顶层全局变量声明
            global_vars = self._extract_global_declarations(root, source_code)
            for var in global_vars:
                if not getattr(var, "origin_file", None):
                    var.origin_file = file_name or None
                # 验证变量名是有效的 C 标识符
                if var.name and self._is_valid_c_identifier(var.name) and var.name not in seen_names:
                    variables.append(var)
                    seen_names.add(var.name)
            
            # 2. 深入函数体提取 static 变量并进行"变量提升"
            lifted_vars = self._lift_function_static_vars(root, source_code, file_name)
            for var in lifted_vars:
                if not getattr(var, "origin_file", None):
                    var.origin_file = file_name or None
                if var.name and self._is_valid_c_identifier(var.name) and var.name not in seen_names:
                    variables.append(var)
                    seen_names.add(var.name)
            
        except Exception as e:
            logger.warning(f"Tree-sitter 解析失败: {e}，回退到正则匹配")
            # 回退到正则匹配
            regex_vars = self._extract_variables_with_regex(source_code)
            for var in regex_vars:
                if not getattr(var, "origin_file", None):
                    var.origin_file = file_name or None
                if var.name and self._is_valid_c_identifier(var.name) and var.name not in seen_names:
                    variables.append(var)
                    seen_names.add(var.name)
        
        return variables
    
    def _is_valid_c_identifier(self, name: str) -> bool:
        """
        检查是否是有效的 C 标识符
        
        有效标识符：以字母或下划线开头，只包含字母、数字、下划线
        """
        import re
        if not name or len(name) > 200:
            return False
        # 必须是 ASCII 字符
        if not all(ord(c) < 128 for c in name):
            return False
        # 必须匹配 C 标识符规则
        return bool(re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', name))
    
    def _escape_rust_keyword(self, name: str) -> str:
        """
        转义 Rust 保留字
        
        注意：self, Self, super, crate 是特殊关键字，不能用 r# 转义
        """
        # 特殊关键字：不能用 r# 转义，需要重命名
        special_keywords = {'self', 'Self', 'super', 'crate'}
        if name in special_keywords:
            return f"{name}_"
        
        # 普通保留字：可以用 r# 转义
        rust_keywords = {
            'type', 'match', 'fn', 'mod', 'use', 'impl', 'trait', 'struct', 'enum',
            'in', 'ref', 'mut', 'const', 'static', 'priv', 'pub', 'let', 'loop',
            'while', 'for', 'if', 'else', 'return', 'break', 'continue', 'as',
            'where', 'async', 'await', 'dyn', 'move', 'box', 'extern', 'unsafe',
            'true', 'false', 'abstract', 'become', 'do', 'final', 'macro',
            'override', 'typeof', 'unsized', 'virtual', 'yield', 'try'
        }
        if name in rust_keywords:
            return f"r#{name}"
        
        return name
    
    def _extract_global_declarations(self, root_node, source_code: str) -> List[VariableInfo]:
        """提取顶层全局变量声明"""
        variables = []
        
        # 遍历顶层节点
        for child in root_node.children:
            if child.type == 'declaration':
                var_info = self._parse_declaration(child, source_code)
                if var_info:
                    variables.extend(var_info)
        
        return variables
    
    def _parse_declaration(self, node, source_code: str) -> List[VariableInfo]:
        """解析声明节点

        增强功能（2025-12-23）：
        - 支持 extern 变量识别
        - extern 变量会生成 extern "C" 块声明而非 static mut
        """
        variables = []

        try:
            decl_text = source_code[node.start_byte:node.end_byte]
            decl_tokens = decl_text.split('=')[0].split() if '=' in decl_text else decl_text.split()

            # 检查是否是 static 声明
            is_static = 'static' in decl_tokens[:3]

            # ★ 检查是否是 extern 声明
            is_extern = 'extern' in decl_tokens[:3]

            # 跳过 extern 函数声明（没有函数体的函数原型）
            # 函数声明会在其他地方处理
            if is_extern and '(' in decl_text:
                return variables

            # 提取类型和变量名
            # 使用 Tree-sitter 查找 declarator
            for child in node.children:
                if child.type in ['init_declarator', 'declarator', 'pointer_declarator', 'array_declarator']:
                    var_name = self._extract_declarator_name(child, source_code)
                    if var_name:
                        c_type = self._extract_type(node, source_code)
                        is_pointer = '*' in decl_text.split(var_name)[0] if var_name in decl_text else False
                        is_array = '[' in decl_text

                        # 收集类型（用于后续生成缺失的不透明结构体）
                        if TYPE_UTILS_AVAILABLE:
                            base_type = extract_base_type(c_type)
                            # 过滤掉基础类型
                            if base_type and base_type not in ['int', 'char', 'void', 'float', 'double', 'long', 'short', '_Bool', 'bool', 'unsigned', 'signed', 'size_t', 'ssize_t', 'ptrdiff_t', 'intptr_t', 'uintptr_t', 'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t', 'int8_t', 'int16_t', 'int32_t', 'int64_t']:
                                self.collected_custom_types.add(base_type)

                        rust_type = self._c_type_to_rust(c_type, is_pointer, is_array)

                        # ★ 为 extern 变量生成不同的声明
                        if is_extern:
                            rust_decl = self._generate_extern_variable_declaration(var_name, rust_type, is_pointer)
                        else:
                            rust_decl = self._generate_rust_static_declaration(var_name, rust_type, is_pointer, is_array)

                        variables.append(VariableInfo(
                            name=var_name,
                            c_type=c_type,
                            rust_type=rust_type,
                            rust_declaration=rust_decl,
                            is_static=is_static,
                            is_extern=is_extern,
                            is_pointer=is_pointer,
                            is_array=is_array
                        ))
        except Exception as e:
            logger.debug(f"解析声明失败: {e}")

        return variables

    def _generate_extern_variable_declaration(self, name: str, rust_type: str, is_pointer: bool) -> str:
        """为 extern 变量生成 Rust 声明

        extern 变量应放在 extern "C" 块中，而非普通的 static mut。

        Args:
            name: 变量名
            rust_type: Rust 类型
            is_pointer: 是否为指针

        Returns:
            extern "C" 块声明字符串
        """
        safe_name = self._escape_rust_keyword(name)

        # extern 变量不需要初始化器，使用 extern "C" 块
        return f'extern "C" {{ pub static mut {safe_name}: {rust_type}; }}'
    
    def _extract_declarator_name(self, node, source_code: str) -> Optional[str]:
        """从 declarator 节点提取变量名"""
        if node.type == 'identifier':
            return source_code[node.start_byte:node.end_byte]
        
        for child in node.children:
            name = self._extract_declarator_name(child, source_code)
            if name:
                return name
        
        return None
    
    def _extract_type(self, decl_node, source_code: str) -> str:
        """提取类型"""
        type_parts = []
        for child in decl_node.children:
            if child.type in ['type_identifier', 'primitive_type', 'sized_type_specifier']:
                type_text = source_code[child.start_byte:child.end_byte]
                # 验证类型名是有效的（只包含 ASCII 字符）
                if type_text and all(ord(c) < 128 for c in type_text):
                    # 过滤明显的垃圾
                    if not any(bad in type_text for bad in ['\n', '\r', '/*', '*/', '//', ';']):
                        type_parts.append(type_text)
            elif child.type in ['struct_specifier', 'enum_specifier', 'union_specifier']:
                # 处理类似 `struct Foo bar;` 这类声明：类型名不在 type_identifier 顶层，而在 *_specifier 内部
                # 不处理匿名 struct/enum/union（无 name），这类通常需要 bindgen 才能完整还原
                name_node = None
                try:
                    name_node = child.child_by_field_name("name")
                except Exception:
                    name_node = None

                if name_node is None:
                    # 兜底：在子节点里找类型名
                    for sub in getattr(child, "children", []) or []:
                        if sub.type in ("type_identifier", "identifier", "field_identifier"):
                            name_node = sub
                            break

                if name_node is not None:
                    type_text = source_code[name_node.start_byte:name_node.end_byte]
                    if type_text and all(ord(c) < 128 for c in type_text):
                        if not any(bad in type_text for bad in ['\n', '\r', '/*', '*/', '//', ';', '{', '}']):
                            type_parts.append(type_text)
            elif child.type == 'storage_class_specifier':
                # 跳过 static, extern 等
                continue
            elif child.type == 'type_qualifier':
                # const, volatile 等
                qual_text = source_code[child.start_byte:child.end_byte]
                if qual_text in ['const', 'volatile', 'restrict']:
                    type_parts.append(qual_text)
        
        result = ' '.join(type_parts) if type_parts else 'unknown'
        
        # 最终验证：类型名不应该包含中文或其他垃圾
        if not all(ord(c) < 128 for c in result):
            logger.warning(f"⚠️ Warning: Invalid type name '{result[:30]}', fallback to void*")
            return 'unknown'
        
        return result
    
    def _lift_function_static_vars(
        self, 
        root_node, 
        source_code: str, 
        file_name: str
    ) -> List[VariableInfo]:
        """
        提取函数内的 static 变量并进行"变量提升"
        
        将 static int count; 重命名为 static int {func_name}_count;
        这种"变量提升"策略能解决 Rust 不支持函数级 static 的问题
        """
        lifted_vars = []
        
        # 查询函数定义
        query = CPP_LANGUAGE.query("""
            (function_definition
                declarator: (function_declarator
                    declarator: (identifier) @func_name)
                body: (compound_statement) @body
            )
        """)
        
        captures = query.captures(root_node)
        
        func_name = None
        for node, name in captures:
            if name == 'func_name':
                func_name = source_code[node.start_byte:node.end_byte]
            elif name == 'body' and func_name:
                # 在函数体内查找 static 声明
                static_vars = self._find_static_in_compound(node, source_code, func_name, file_name)
                lifted_vars.extend(static_vars)
                func_name = None
        
        return lifted_vars
    
    def _find_static_in_compound(
        self, 
        compound_node, 
        source_code: str, 
        func_name: str,
        file_name: str
    ) -> List[VariableInfo]:
        """在复合语句中查找 static 声明"""
        variables = []
        
        for child in compound_node.children:
            if child.type == 'declaration':
                decl_text = source_code[child.start_byte:child.end_byte]
                if decl_text.strip().startswith('static'):
                    # 解析 static 声明
                    var_infos = self._parse_declaration(child, source_code)
                    for var in var_infos:
                        # 重命名：添加函数名前缀以确保唯一性
                        lifted_name = f"{func_name}_{var.name}"
                        var.name = lifted_name
                        var.from_function = func_name
                        var.rust_declaration = self._generate_rust_static_declaration(
                            lifted_name, var.rust_type, var.is_pointer, var.is_array
                        )
                        variables.append(var)
        
        return variables
    
    def _extract_variables_with_regex(self, source_code: str) -> List[VariableInfo]:
        """使用正则表达式提取变量（回退方案）"""
        variables = []
        
        # 匹配 static 变量
        static_patterns = [
            # static Type *var = value;
            r'static\s+(?:const\s+)?([\w:]+)\s*\*\s*(\w+)\s*(?:=\s*[^;]+)?\s*;',
            # static Type var = value;
            r'static\s+(?:const\s+)?([\w:]+)\s+(\w+)\s*(?:=\s*[^;]+)?\s*;',
            # static Type var[SIZE];
            r'static\s+(?:const\s+)?([\w:]+)\s+(\w+)\s*\[([^\]]*)\]\s*;',
        ]
        
        for pattern in static_patterns:
            for match in re.finditer(pattern, source_code, re.MULTILINE):
                c_type = match.group(1).strip()
                var_name = match.group(2).strip()
                is_pointer = '*' in match.group(0)
                is_array = '[' in match.group(0)
                
                rust_type = self._c_type_to_rust(c_type, is_pointer, is_array)
                rust_decl = self._generate_rust_static_declaration(var_name, rust_type, is_pointer, is_array)
                
                variables.append(VariableInfo(
                    name=var_name,
                    c_type=c_type,
                    rust_type=rust_type,
                    rust_declaration=rust_decl,
                    is_static=True,
                    is_pointer=is_pointer,
                    is_array=is_array
                ))
        
        # 匹配 g_xxx 或 G_XXX 全局变量
        global_patterns = [
            r'^[\t ]*([\w:]+)\s*\*?\s*(g_\w+|G_[A-Z_]+)\s*(?:=\s*[^;]+)?\s*;',
        ]
        
        for pattern in global_patterns:
            for match in re.finditer(pattern, source_code, re.MULTILINE):
                c_type = match.group(1).strip()
                var_name = match.group(2).strip()
                is_pointer = '*' in match.group(0)
                
                rust_type = self._c_type_to_rust(c_type, is_pointer, False)
                rust_decl = self._generate_rust_static_declaration(var_name, rust_type, is_pointer, False)
                
                variables.append(VariableInfo(
                    name=var_name,
                    c_type=c_type,
                    rust_type=rust_type,
                    rust_declaration=rust_decl,
                    is_static=False,
                    is_pointer=is_pointer,
                    is_array=False
                ))
        
        return variables
    
    def _c_type_to_rust(self, c_type: str, is_pointer: bool, is_array: bool) -> str:
        """
        将 C 类型转换为 Rust 类型
        
        基于 EvoC2Rust 和 LLMigrate 的类型映射规则
        
        注意：如果 TypeMapper 可用，优先使用 TypeMapper
        """
        # 优先使用 TypeMapper（如果可用）
        if TYPE_MAPPER_AVAILABLE:
            return TypeMapper.map_c_type(c_type, is_pointer, False)
        
        # 回退到旧的映射方法
        type_mapping = {
            # 基础整数类型
            'int': 'i32',
            'unsigned int': 'u32',
            'unsigned': 'u32',
            'long': 'i64',
            'unsigned long': 'u64',
            'long long': 'i64',
            'long long int': 'i64',
            'unsigned long long': 'u64',
            'unsigned long long int': 'u64',
            'short': 'i16',
            'short int': 'i16',
            'unsigned short': 'u16',
            'unsigned short int': 'u16',
            'char': 'i8',
            'unsigned char': 'u8',
            'signed char': 'i8',
            'signed': 'i32',
            
            # 浮点类型
            'float': 'f32',
            'double': 'f64',
            'long double': 'f64',
            
            # 特殊类型
            'void': 'std::ffi::c_void',
            'bool': 'bool',
            '_Bool': 'bool',
            
            # 固定宽度整数类型 (stdint.h)
            'size_t': 'usize',
            'ssize_t': 'isize',
            'ptrdiff_t': 'isize',
            'intptr_t': 'isize',
            'uintptr_t': 'usize',
            'uint8_t': 'u8',
            'uint16_t': 'u16',
            'uint32_t': 'u32',
            'uint64_t': 'u64',
            'int8_t': 'i8',
            'int16_t': 'i16',
            'int32_t': 'i32',
            'int64_t': 'i64',
            
            # 常见的 POSIX 类型
            'off_t': 'i64',
            'pid_t': 'i32',
            'uid_t': 'u32',
            'gid_t': 'u32',
            'mode_t': 'u32',
            'time_t': 'i64',
            'clock_t': 'i64',
            'socklen_t': 'u32',
            
            # C 标准库类型
            'FILE': 'std::ffi::c_void',  # FILE* 作为不透明类型
            'wchar_t': 'i32',
            
            # C++ 标准库类型（简化处理）
            'std::string': 'String',
            'string': 'String',
        }
        
        # 清理类型名
        clean_type = c_type.replace('const', '').replace('volatile', '').replace('struct', '').replace('enum', '').replace('union', '').strip()
        
        # 移除多余空格
        clean_type = ' '.join(clean_type.split())
        
        # 查找映射
        rust_type = type_mapping.get(clean_type)
        
        if rust_type is None:
            # 未知类型，保留原名（可能是自定义类型，在 types.rs 中定义）
            # 清理类型名使其成为有效的 Rust 标识符
            rust_type = clean_type.replace('::', '_').replace(' ', '_')
            if rust_type and not rust_type[0].isalpha() and rust_type[0] != '_':
                rust_type = '_' + rust_type
        
        if is_pointer:
            if clean_type == 'void':
                rust_type = '*mut std::ffi::c_void'
            elif clean_type == 'char':
                rust_type = '*mut i8'  # char* 通常是 C 字符串
            else:
                rust_type = f'*mut {rust_type}'
        
        if is_array and not is_pointer:
            # 数组类型，但没有指定大小时使用切片或指针
            rust_type = f'*mut {rust_type}'
        
        return rust_type
    
    def _generate_rust_static_declaration(
        self, 
        name: str, 
        rust_type: str, 
        is_pointer: bool, 
        is_array: bool
    ) -> str:
        """生成 Rust static mut 声明"""
        # 处理 Rust 保留字作为变量名
        safe_name = self._escape_rust_keyword(name)
        
        if is_pointer:
            return f'pub static mut {safe_name}: {rust_type} = std::ptr::null_mut();'
        elif is_array:
            return f'pub static mut {safe_name}: [{rust_type}; 0] = [];  // TODO: Set correct array size'
        elif rust_type in ['i32', 'u32', 'i64', 'u64', 'i16', 'u16', 'i8', 'u8', 'usize', 'isize']:
            return f'pub static mut {safe_name}: {rust_type} = 0;'
        elif rust_type in ['f32', 'f64']:
            return f'pub static mut {safe_name}: {rust_type} = 0.0;'
        elif rust_type == 'bool':
            return f'pub static mut {safe_name}: bool = false;'
        else:
            # 复杂类型使用 MaybeUninit
            return f'pub static mut {safe_name}: std::mem::MaybeUninit<{rust_type}> = std::mem::MaybeUninit::uninit();'

    # =========================================================================
    # Globals generation: Scheme A (bindgen-truth static storage)
    # =========================================================================

    @staticmethod
    def _find_bindgen_binary_for_globals() -> Optional[str]:
        """Locate `bindgen` CLI (best-effort)."""
        p = shutil.which("bindgen")
        if p:
            return p
        cargo_bin = Path.home() / ".cargo" / "bin" / "bindgen"
        if cargo_bin.exists():
            return str(cargo_bin)
        return None

    @staticmethod
    def _parse_defined_type_names_from_types_rs_text(types_rs: str) -> Set[str]:
        """Extract `pub (struct|enum|union|type) Name` items from types.rs (best-effort)."""
        if not types_rs:
            return set()
        return set(re.findall(r"\bpub\s+(?:struct|enum|union|type)\s+([A-Za-z_]\w*)\b", types_rs))

    @staticmethod
    def _prefix_types_in_rust_type_expr(type_expr: str, type_names: Set[str]) -> str:
        """
        Prefix standalone identifiers in a Rust type expression with `crate::types::`,
        if they are known type names from types.rs.
        """
        if not type_expr or not type_names:
            return type_expr or ""

        s = type_expr
        res: List[str] = []
        i = 0
        last_token = ""
        while i < len(s):
            if s.startswith("::", i):
                res.append("::")
                i += 2
                last_token = "::"
                continue
            m = re.match(r"[A-Za-z_][A-Za-z0-9_]*", s[i:])
            if m:
                ident = m.group(0)
                if ident in type_names and last_token != "::":
                    res.append(f"crate::types::{ident}")
                else:
                    res.append(ident)
                i += len(ident)
                last_token = ident
                continue
            res.append(s[i])
            i += 1
        return "".join(res).strip()

    @staticmethod
    def _parse_bindgen_extern_vars(bindgen_rs: str) -> Dict[str, Dict[str, str]]:
        """
        Parse bindgen output and extract variable type info (best-effort).

        Returns:
          {VarRustName: {"ty": TypeExpr, "init": OptionalInitExpr}}

        Notes:
        - bindgen may emit `pub const NAME: Ty = <const-expr>;` for var definitions with constant initializers.
        - bindgen may wrap variable decls in either `extern "C" { ... }` or `unsafe extern "C" { ... }`.
        """
        if not bindgen_rs:
            return {}

        lines = bindgen_rs.splitlines()
        i = 0
        in_extern = False
        vars_map: Dict[str, Dict[str, str]] = {}

        def _normalize_sig(sig_lines: List[str]) -> str:
            sig = " ".join((ln or "").strip() for ln in sig_lines if (ln or "").strip())
            return re.sub(r"\s+", " ", sig).strip()

        # Pass 1: capture top-level `pub const` (constified var definitions).
        for line in lines:
            s = (line or "").strip()
            if not s.startswith("pub const "):
                continue
            m = re.match(
                r"^pub\s+const\s+([A-Za-z_]\w*|r#[A-Za-z_]\w*)\s*:\s*(.+?)\s*=\s*(.+);$",
                s,
            )
            if not m:
                continue
            name = m.group(1).strip()
            ty = m.group(2).strip()
            init = m.group(3).strip()
            if name and ty:
                vars_map[name] = {"ty": ty, "init": init}

        # Pass 2: capture `pub static` from extern blocks.
        while i < len(lines):
            line = lines[i]
            s = (line or "").strip()
            if s in {'extern "C" {', 'unsafe extern "C" {'}:
                in_extern = True
                i += 1
                continue
            if in_extern and s == "}":
                in_extern = False
                i += 1
                continue
            if not in_extern:
                i += 1
                continue
            if not s:
                i += 1
                continue
            if s.startswith("#["):
                # ignore attrs for globals storage
                i += 1
                continue

            if s.startswith("pub static "):
                sig_lines = [line]
                i += 1
                while i < len(lines) and not (sig_lines[-1] or "").strip().endswith(";"):
                    sig_lines.append(lines[i])
                    i += 1
                sig = _normalize_sig(sig_lines)
                m = re.match(
                    r"^pub\s+static(?:\s+mut)?\s+([A-Za-z_]\w*|r#[A-Za-z_]\w*)\s*:\s*(.+);$",
                    sig,
                )
                if m:
                    name = m.group(1).strip()
                    ty = m.group(2).strip()
                    if name and ty:
                        vars_map.setdefault(name, {})
                        vars_map[name]["ty"] = ty
                continue

            i += 1

        # Normalize to always have `ty`.
        vars_map = {n: v for n, v in vars_map.items() if isinstance(v, dict) and v.get("ty")}
        return vars_map

    @staticmethod
    def _extract_c_array_initializer(c_source_files: List[Path], var_name: str) -> Optional[str]:
        """
        Extract array initializer from C source files for a given variable name.

        Looks for patterns like:
            Type VAR_NAME[SIZE] = { val1, val2, ... };
            Type VAR_NAME[] = { val1, val2, ... };

        Args:
            c_source_files: List of C source file paths to search
            var_name: Name of the variable to find

        Returns:
            Rust array literal (e.g., "[619, 720, 127, ...]") or None if not found
        """
        if not var_name or not c_source_files:
            return None

        # Pattern to match array definition with initializer
        # Captures: type, name, optional size, initializer content
        pattern = rf'\b{re.escape(var_name)}\s*\[[^\]]*\]\s*=\s*\{{\s*'

        for src_path in c_source_files:
            try:
                if not src_path.exists():
                    continue
                content = src_path.read_text(encoding="utf-8", errors="ignore")

                # Find the variable definition
                match = re.search(pattern, content, re.MULTILINE)
                if not match:
                    continue

                # Find the opening brace position within the matched text
                # The pattern ends with `\{\s*` so we need to find the { position
                matched_text = match.group(0)
                brace_offset = matched_text.rfind('{')
                if brace_offset < 0:
                    continue
                start_pos = match.start() + brace_offset  # Position of opening {

                # Extract the initializer block using brace matching
                brace_depth = 0
                end_pos = start_pos

                for i in range(start_pos, len(content)):
                    ch = content[i]
                    if ch == '{':
                        brace_depth += 1
                    elif ch == '}':
                        brace_depth -= 1
                        if brace_depth == 0:
                            end_pos = i + 1
                            break

                if end_pos <= start_pos:
                    continue

                # Extract the full initializer block
                initializer = content[start_pos:end_pos]

                # Convert C initializer to Rust array literal
                # { val1, val2, ... } -> [ val1, val2, ... ]
                rust_init = initializer.strip()
                if rust_init.startswith('{') and rust_init.endswith('}'):
                    # Replace outer braces with brackets
                    inner = rust_init[1:-1].strip()

                    # Handle nested initializers for structs (convert {} to {})
                    # But for simple numeric arrays, just replace outer braces

                    # Clean up whitespace and newlines
                    inner = re.sub(r'\s+', ' ', inner)

                    # Check if it's a simple numeric array (most common case)
                    # For complex nested initializers, we'd need more sophisticated handling
                    if not re.search(r'\{', inner):  # No nested braces
                        return f"[{inner}]"
                    else:
                        # Nested initializers - return None for now (complex case)
                        # Future enhancement: handle struct array initializers
                        return None

            except Exception as e:
                logger.debug(f"Failed to extract initializer from {src_path}: {e}")
                continue

        return None

    def _get_project_source_files(self) -> List[Path]:
        """Get list of C source files for the current project."""
        source_files: List[Path] = []
        c_exts = {'.c', '.cc', '.cpp', '.cxx'}

        # Try to get source files from TU context (most accurate)
        if hasattr(self, '_tu_context_files') and self._tu_context_files:
            for safe_name, rec in self._tu_context_files.items():
                if isinstance(rec, dict):
                    src = rec.get("source_file_abs") or rec.get("source_for_cc_abs")
                    if src:
                        try:
                            p = Path(str(src))
                            if p.exists() and p.suffix.lower() in c_exts:
                                source_files.append(p)
                        except Exception:
                            pass

        # Also try project_root if available
        if hasattr(self, 'project_root') and self.project_root:
            project_root = Path(self.project_root)
            try:
                for ext in ['*.c', '*.cc', '*.cpp']:
                    source_files.extend(project_root.rglob(ext))
            except Exception:
                pass

            # Also check the src/ subdirectory specifically
            src_dir = project_root / "src"
            if src_dir.exists():
                try:
                    for ext in ['*.c', '*.cc', '*.cpp']:
                        source_files.extend(src_dir.rglob(ext))
                except Exception:
                    pass

        # Fallback: check ComparisonMethod directory structure
        # This handles the case where project sources are in ComparisonMethod/Our/projects/{project_name}/
        if hasattr(self, 'output_dir') and self.output_dir:
            try:
                output_path = Path(self.output_dir)
                # Look for project name from output path
                # e.g., .../intermediate/bzip2/workspace/... -> bzip2
                project_name = None
                # Detect from intermediate directory structure (generic approach)
                if 'intermediate' in output_path.parts:
                    try:
                        idx = output_path.parts.index('intermediate')
                        if idx + 1 < len(output_path.parts):
                            project_name = output_path.parts[idx + 1]
                    except (ValueError, IndexError):
                        pass

                if project_name:
                    # Look for ComparisonMethod/Our/projects/{project_name}/
                    framework_root = Path(__file__).parent
                    comparison_paths = [
                        framework_root / "ComparisonMethod" / "Our" / "projects" / project_name,
                        framework_root / "ComparisonMethod" / "Our" / "projects" / project_name / "src",
                    ]
                    for comp_path in comparison_paths:
                        if comp_path.exists():
                            for ext in ['*.c', '*.cc', '*.cpp']:
                                source_files.extend(comp_path.rglob(ext))
            except Exception:
                pass

        unique_files = list(set(source_files))
        if unique_files:
            logger.debug(f"找到 {len(unique_files)} 个 C 源文件用于初始化值提取")
        return unique_files

    @staticmethod
    def _split_top_level_commas(src: str) -> List[str]:
        """Split a Rust-like param list by commas, ignoring nested (), [], <>."""
        s = src or ""
        out: List[str] = []
        buf: List[str] = []
        depth_paren = 0
        depth_angle = 0
        depth_brack = 0
        i = 0
        while i < len(s):
            ch = s[i]
            if ch == "(":
                depth_paren += 1
            elif ch == ")":
                depth_paren = max(0, depth_paren - 1)
            elif ch == "<":
                depth_angle += 1
            elif ch == ">":
                depth_angle = max(0, depth_angle - 1)
            elif ch == "[":
                depth_brack += 1
            elif ch == "]":
                depth_brack = max(0, depth_brack - 1)
            if ch == "," and depth_paren == 0 and depth_angle == 0 and depth_brack == 0:
                chunk = "".join(buf).strip()
                if chunk:
                    out.append(chunk)
                buf = []
                i += 1
                continue
            buf.append(ch)
            i += 1
        tail = "".join(buf).strip()
        if tail:
            out.append(tail)
        return out

    @staticmethod
    def _parse_bindgen_extern_fns(bindgen_rs: str) -> Dict[str, Dict[str, str]]:
        """
        Parse bindgen output and extract function signatures (best-effort).

        Returns:
          {FnName: {"params": "...", "ret": "..."}}

        Notes:
        - bindgen may wrap decls in either `extern "C" { ... }` or `unsafe extern "C" { ... }`.
        - We only capture `pub fn` items inside the extern block; types/consts are ignored here.
        """
        if not bindgen_rs:
            return {}

        lines = bindgen_rs.splitlines()
        i = 0
        in_extern = False
        out: Dict[str, Dict[str, str]] = {}

        def _normalize(sig_lines: List[str]) -> str:
            sig = " ".join((ln or "").strip() for ln in sig_lines if (ln or "").strip())
            sig = re.sub(r"\s+", " ", sig).strip()
            return sig

        def _parse_one(sig: str) -> Optional[Tuple[str, str, str]]:
            s = (sig or "").strip()
            if not s.endswith(";"):
                return None
            s = s[:-1].strip()
            # Find `fn name(`
            m = re.search(r"\bfn\s+([A-Za-z_]\w*|r#[A-Za-z_]\w*)\s*\(", s)
            if not m:
                return None
            name = m.group(1).strip()
            # Find params span (balanced parentheses) starting at the `(` after the name.
            start = s.find("(", m.end(1))
            if start == -1:
                return None
            depth = 0
            end = None
            for j in range(start, len(s)):
                ch = s[j]
                if ch == "(":
                    depth += 1
                elif ch == ")":
                    depth -= 1
                    if depth == 0:
                        end = j
                        break
            if end is None:
                return None
            params = s[start + 1 : end].strip()
            rest = s[end + 1 :].strip()
            ret = ""
            if rest.startswith("->"):
                ret = rest[2:].strip()
            # Strip trailing where-clause-like fragments (bindgen usually doesn't emit them for extern fns)
            return name, params, ret

        while i < len(lines):
            line = lines[i]
            s = (line or "").strip()
            if s in {'extern "C" {', 'unsafe extern "C" {'}:
                in_extern = True
                i += 1
                continue
            if in_extern and s == "}":
                in_extern = False
                i += 1
                continue
            if not in_extern:
                i += 1
                continue
            if not s:
                i += 1
                continue
            if s.startswith("#["):
                i += 1
                continue
            if s.startswith("pub ") and " fn " in s:
                sig_lines = [line]
                i += 1
                while i < len(lines) and not (sig_lines[-1] or "").strip().endswith(";"):
                    sig_lines.append(lines[i])
                    i += 1
                sig = _normalize(sig_lines)
                parsed = _parse_one(sig)
                if parsed:
                    name, params, ret = parsed
                    # Record both raw name and unescaped name (r#foo -> foo) for easier lookup.
                    out[name] = {"params": params, "ret": ret}
                    if name.startswith("r#"):
                        out.setdefault(name[2:], {"params": params, "ret": ret})
                continue
            i += 1

        return out

    def _run_bindgen_allowlist_fns(
        self,
        *,
        preprocessed_path: Path,
        fn_names: Sequence[str],
        lang: str,
        defined_types: Set[str],
        timeout_sec: int,
        tmp_dir: Path,
    ) -> Dict[str, str]:
        """
        Run bindgen on a preprocessed `.i` file and return `{fn_name: rust_signature}` for allowlisted fns.

        The returned signature is a Rust *definition* signature (for stubs), e.g.:
          `pub extern "C" fn foo(arg1: *const c_char) -> i32`
        """
        bindgen_bin = self._find_bindgen_binary_for_globals()
        if not bindgen_bin:
            return {}
        if not preprocessed_path or not preprocessed_path.exists():
            return {}
        if not fn_names:
            return {}

        try:
            tmp_dir.mkdir(parents=True, exist_ok=True)
        except Exception:
            pass

        # Cache by (file, lang, fn-set)
        try:
            import hashlib as _hashlib

            fn_hash = _hashlib.md5("\n".join(sorted(set(fn_names))).encode("utf-8", errors="ignore")).hexdigest()[:10]
            cache_key = f"{str(preprocessed_path)}|{(lang or 'c').lower()}|{fn_hash}"
        except Exception:
            cache_key = ""
        if cache_key and cache_key in self._bindgen_fn_sig_cache:
            return dict(self._bindgen_fn_sig_cache.get(cache_key) or {})

        out_rs = tmp_dir / f"{preprocessed_path.stem}.fns.rs"
        cmd: List[str] = [
            bindgen_bin,
            str(preprocessed_path),
            "-o",
            str(out_rs),
            "--generate",
            "functions",
            "--no-layout-tests",
            "--no-doc-comments",
            "--use-core",
            "--default-enum-style=consts",
            "--no-prepend-enum-name",
            "--no-size_t-is-usize",
        ]
        for name in sorted(set(fn_names)):
            cmd.extend(["--allowlist-function", rf"^{re.escape(name)}$"])
        cmd.append("--")
        if (lang or "c").lower().startswith("c++"):
            cmd.extend(["-x", "c++", "-std=c++17"])
        else:
            cmd.extend(["-x", "c"])
        cmd.extend(
            [
                "-Wno-error",
                "-Wno-macro-redefined",
                "-Wno-builtin-macro-redefined",
                "-Wno-ignored-attributes",
            ]
        )

        try:
            proc = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=max(10, int(timeout_sec or 60)),
            )
        except Exception as e:
            logger.debug(f"bindgen fns 失败: {e}")
            return {}
        if proc.returncode != 0 or not out_rs.exists():
            logger.debug(f"bindgen fns 失败 rc={proc.returncode}: {(proc.stderr or '')[:200]}")
            return {}

        try:
            bindgen_text = out_rs.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            bindgen_text = ""

        fns_map = self._parse_bindgen_extern_fns(bindgen_text)
        if not fns_map:
            return {}

        out: Dict[str, str] = {}
        for fn_name, info in fns_map.items():
            params_raw = str((info or {}).get("params") or "").strip()
            ret_raw = str((info or {}).get("ret") or "").strip()
            # Reject C-variadic decls (cannot define in stable Rust).
            if "..." in params_raw:
                continue

            params_out: List[str] = []
            for item in self._split_top_level_commas(params_raw):
                if item.strip() == "...":
                    params_out = []
                    break
                if ":" not in item:
                    # unexpected, keep as-is
                    params_out.append(item.strip())
                    continue
                n, ty = item.split(":", 1)
                n = (n or "").strip()
                ty = self._prefix_types_in_rust_type_expr((ty or "").strip(), defined_types)
                params_out.append(f"{n}: {ty}".strip())
            if not params_out and params_raw.strip():
                # Could not parse params reliably, skip this fn.
                continue

            ret = self._prefix_types_in_rust_type_expr(ret_raw, defined_types) if ret_raw else ""
            # Normalize void return
            ret_clause = ""
            if ret and ret != "()":
                ret_clause = f" -> {ret}"
            sig = f'pub extern "C" fn {fn_name}({", ".join(params_out)}){ret_clause}'
            out[fn_name] = sig

        if cache_key:
            try:
                self._bindgen_fn_sig_cache[cache_key] = dict(out)
            except Exception:
                pass
        return out

    def get_bindgen_function_signatures_from_tu(
        self,
        *,
        safe_name: str,
        fn_names: Sequence[str],
        source_file: Optional[Path] = None,
    ) -> Dict[str, str]:
        """
        Deterministic signature source: bindgen allowlist on stage1-pinned preprocessed `.i` (TU truth).

        Fallback (optional): if stage1 TU map is missing/unavailable, try to preprocess `source_file`
        using the loaded compile_commands.json, then run bindgen on the generated `.i`.

        Returns `{c_fn_name: rust_signature}` for functions in this file-group (safe_name).
        """
        if not safe_name or not fn_names:
            return {}
        try:
            require_tu = (os.environ.get("C2R_REQUIRE_TU_CLOSURE", "1") or "1").strip().lower() not in (
                "0",
                "false",
                "no",
            )
        except Exception:
            require_tu = True
        rec = self._tu_context_files.get(str(safe_name)) if getattr(self, "_tu_context_files", None) else None
        pre = rec.get("preprocessed_file") if isinstance(rec, dict) else None
        pre_path: Optional[Path] = None
        if pre:
            try:
                pre_path = Path(str(pre)).expanduser()
            except Exception:
                pre_path = None

        # Best-effort: regenerate preprocessed `.i` if it was not materialized but we have a pinned entry.
        if (not pre_path) or (not pre_path.exists()):
            try:
                entry = rec.get("compile_commands_entry") if isinstance(rec, dict) else None
                if entry and self.compile_commands_parser and self._env_flag("C2R_RETRY_PREPROCESS_MISSING_I", "0"):
                    out_dir = None
                    try:
                        env_dir = os.environ.get("PREPROCESS_OUTPUT_DIR", "").strip()
                        out_dir = Path(env_dir) if env_dir else None
                    except Exception:
                        out_dir = None
                    ctx = self.compile_commands_parser.preprocess_with_context(
                        Path(str(rec.get("source_for_cc_abs") or rec.get("source_file_abs") or "")),
                        entry,
                        output_dir=out_dir,
                        timeout_sec=int(os.environ.get("C2R_PREPROCESS_TIMEOUT_SEC", "60")),
                    )
                    if ctx and getattr(ctx, "preprocessed_file", None):
                        pre_path = Path(ctx.preprocessed_file)
            except Exception:
                pass

        # Fallback: if stage1 map is missing (or `.i` missing) but we do have a source file path,
        # try to derive a preprocessing context directly from compile_commands.json.
        if (not pre_path) or (not pre_path.exists()):
            if require_tu:
                # Strict TU-closure mode: never "re-pick" a TU/profile. Missing stage1 `.i` should be
                # treated as an input/closure issue and handled by the project-level gate.
                return {}
            try:
                enable_fallback = self._env_flag("C2R_BINDGEN_FN_SIG_FALLBACK_PREPROCESS", default="1")
            except Exception:
                enable_fallback = True
            # Truth mode: do NOT "re-pick" a TU. Missing stage1 `.i` should be treated as a data/closure issue.
            # (Override only if you explicitly opt in.)
            try:
                truth_mode = self._env_flag("C2R_TRUTH_MODE", "0")
            except Exception:
                truth_mode = False
            if truth_mode and (not self._env_flag("C2R_TRUTH_ALLOW_FN_SIG_FALLBACK_PREPROCESS", "0")):
                enable_fallback = False
            if enable_fallback and source_file and self.compile_commands_parser:
                try:
                    src_path = Path(source_file)
                    src_for_cc = self._map_to_ohos_path(src_path)
                    from compile_commands_parser import ContextSelectionStrategy  # type: ignore

                    strat_env = (
                        os.environ.get("C2R_PREPROCESSING_STRATEGY", "")
                        or os.environ.get("PREPROCESSING_STRATEGY", "")
                        or ""
                    ).strip().lower()
                    strategy = ContextSelectionStrategy.BEST
                    if strat_env in ("active", "auto"):
                        strategy = ContextSelectionStrategy.ACTIVE
                    elif strat_env == "union":
                        strategy = ContextSelectionStrategy.UNION
                    target_cfg = None
                    if strategy == ContextSelectionStrategy.ACTIVE:
                        target_cfg = (os.environ.get("TARGET_OUT_DIR") or os.environ.get("TARGET_CONFIG") or "").strip() or None

                    out_dir = None
                    try:
                        env_dir = os.environ.get("PREPROCESS_OUTPUT_DIR", "").strip()
                        out_dir = Path(env_dir) if env_dir else (self.output_dir / ".preprocessed_fallback")
                    except Exception:
                        out_dir = self.output_dir / ".preprocessed_fallback"

                    ctx = self.compile_commands_parser.select_preprocessing_context(
                        src_for_cc,
                        strategy=strategy,
                        target_config=target_cfg,
                        output_dir=out_dir,
                    )
                    if ctx and not getattr(ctx, "error", None) and getattr(ctx, "preprocessed_file", None):
                        pre_path = Path(ctx.preprocessed_file)
                        # Provide a minimal lang hint if stage1 record is absent.
                        if not isinstance(rec, dict):
                            rec = {"source_file_abs": str(src_path)}
                except Exception:
                    pass

        if not pre_path or not pre_path.exists():
            return {}

        defined_types: Set[str] = set()
        try:
            types_rs = (self.output_dir / "src" / "types.rs").read_text(encoding="utf-8", errors="ignore")
            defined_types = self._parse_defined_type_names_from_types_rs_text(types_rs)
        except Exception:
            defined_types = set()

        lang = "c"
        try:
            p = rec.get("source_file_abs") or rec.get("source_for_cc_abs") if isinstance(rec, dict) else ""
            if not p and source_file:
                p = str(source_file)
            suf = Path(str(p)).suffix.lower()
            if suf in {".cc", ".cpp", ".cxx", ".c++"}:
                lang = "c++"
        except Exception:
            lang = "c"

        try:
            timeout_sec = int(os.environ.get("C2R_BINDGEN_FN_SIG_TIMEOUT_SEC", "90"))
        except Exception:
            timeout_sec = 90
        tmp_dir = self.output_dir / ".c2r_bindgen_fns"

        return self._run_bindgen_allowlist_fns(
            preprocessed_path=pre_path,
            fn_names=fn_names,
            lang=lang,
            defined_types=defined_types,
            timeout_sec=timeout_sec,
            tmp_dir=tmp_dir,
        )

    def _run_bindgen_allowlist_vars(
        self,
        *,
        preprocessed_path: Path,
        var_names: Sequence[str],
        lang: str,
        defined_types: Set[str],
        timeout_sec: int,
        tmp_dir: Path,
    ) -> Dict[str, Dict[str, str]]:
        """
        Run bindgen on a preprocessed `.i` file and return `{var_name: rust_type_expr}` for allowlisted vars.
        """
        bindgen_bin = self._find_bindgen_binary_for_globals()
        if not bindgen_bin:
            return {}
        if not preprocessed_path or not preprocessed_path.exists():
            return {}
        if not var_names:
            return {}

        try:
            tmp_dir.mkdir(parents=True, exist_ok=True)
        except Exception:
            pass

        out_rs = tmp_dir / f"{preprocessed_path.stem}.globals.rs"
        cmd: List[str] = [
            bindgen_bin,
            str(preprocessed_path),
            "-o",
            str(out_rs),
            "--generate",
            "vars",
            "--no-layout-tests",
            "--no-doc-comments",
            "--use-core",
            "--default-enum-style=consts",
            "--no-prepend-enum-name",
            "--no-size_t-is-usize",
            "--ignore-functions",
        ]
        for name in sorted(set(var_names)):
            # NOTE: bindgen may emit some global var definitions as `pub const` (constified),
            # which are filtered out by `--allowlist-var`. Use `--allowlist-item` to capture both.
            cmd.extend(["--allowlist-item", rf"^{re.escape(name)}$"])
        cmd.append("--")
        if (lang or "c").lower().startswith("c++"):
            cmd.extend(["-x", "c++", "-std=c++17"])
        else:
            cmd.extend(["-x", "c"])
        cmd.extend(
            [
                "-Wno-error",
                "-Wno-macro-redefined",
                "-Wno-builtin-macro-redefined",
                "-Wno-ignored-attributes",
            ]
        )

        try:
            proc = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=max(10, int(timeout_sec or 60)),
            )
        except Exception as e:
            logger.debug(f"bindgen globals 失败: {e}")
            return {}
        if proc.returncode != 0 or not out_rs.exists():
            logger.debug(f"bindgen globals 失败 rc={proc.returncode}: {(proc.stderr or '')[:200]}")
            return {}

        try:
            bindgen_text = out_rs.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            bindgen_text = ""

        vars_map = self._parse_bindgen_extern_vars(bindgen_text)
        if not vars_map:
            return {}

        out: Dict[str, Dict[str, str]] = {}
        for n, info in vars_map.items():
            ty = self._prefix_types_in_rust_type_expr(str(info.get("ty") or ""), defined_types)
            if not ty:
                continue
            init = str(info.get("init") or "").strip()
            out[n] = {"ty": ty, "init": init}
        return out

    def _run_bindgen_allowlist_types(
        self,
        *,
        preprocessed_path: Path,
        type_names: Sequence[str],
        lang: str,
        timeout_sec: int,
        out_rs: Path,
    ) -> bool:
        """
        Run bindgen on a preprocessed `.i` file and emit Rust type declarations for allowlisted types.

        Notes:
        - Input is a stage1-pinned TU `.i` (macro-expanded + full include context).
        - Output is included via `include!()` (do NOT place it under `src/*.rs` root to avoid main.rs glob).
        """
        bindgen_bin = self._find_bindgen_binary_for_globals()
        if not bindgen_bin:
            return False
        if not preprocessed_path or not preprocessed_path.exists():
            return False
        names = [n for n in sorted(set(type_names or [])) if isinstance(n, str) and n.strip()]
        if not names:
            return False

        try:
            out_rs.parent.mkdir(parents=True, exist_ok=True)
        except Exception:
            pass

        cmd: List[str] = [
            bindgen_bin,
            str(preprocessed_path),
            "-o",
            str(out_rs),
            "--generate",
            "types",
            "--no-layout-tests",
            "--no-doc-comments",
            "--use-core",
            "--default-enum-style=consts",
            "--no-prepend-enum-name",
            "--no-size_t-is-usize",
            "--ignore-functions",
        ]
        for name in names:
            cmd.extend(["--allowlist-type", rf"^{re.escape(name)}$"])
        cmd.append("--")
        if (lang or "c").lower().startswith("c++"):
            cmd.extend(["-x", "c++", "-std=c++17"])
        else:
            cmd.extend(["-x", "c"])
        cmd.extend(
            [
                "-Wno-error",
                "-Wno-macro-redefined",
                "-Wno-builtin-macro-redefined",
                "-Wno-ignored-attributes",
            ]
        )

        try:
            proc = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=max(10, int(timeout_sec or 60)),
            )
        except Exception as e:
            logger.debug(f"bindgen types 失败: {e}")
            return False

        if proc.returncode != 0 or not out_rs.exists():
            try:
                logger.debug(f"bindgen types 失败 rc={proc.returncode}: {(proc.stderr or '')[:200]}")
            except Exception:
                pass
            try:
                if out_rs.exists():
                    out_rs.unlink()
            except Exception:
                pass
            return False

        return True

    def _extract_defined_type_names_from_bindgen_output(self, rs_path: Path) -> Set[str]:
        """Best-effort: parse bindgen-emitted Rust to find defined type names.

        We only care about top-level `pub {type|struct|enum|union} Name` items so that we don't
        re-export/import names that bindgen didn't actually emit (which would break baseline compile).
        """
        if not rs_path or not rs_path.exists():
            return set()
        try:
            txt = rs_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            return set()
        names: Set[str] = set()
        try:
            for m in re.finditer(r"(?m)^[ \t]*pub[ \t]+(?:type|struct|enum|union)[ \t]+([A-Za-z_][A-Za-z0-9_]*)\b", txt):
                n = m.group(1)
                if n:
                    names.add(n)
        except Exception:
            return set()
        return names

    def _get_pinned_preprocessed_path_for_safe_name(self, safe_name: str, *, source_file: Optional[Path] = None) -> Optional[Path]:
        """Return the stage1-pinned `.i` path for `safe_name` (best-effort)."""
        if not safe_name:
            return None
        rec = self._tu_context_files.get(str(safe_name)) if getattr(self, "_tu_context_files", None) else None
        pre = rec.get("preprocessed_file") if isinstance(rec, dict) else None
        pre_path: Optional[Path] = None
        if pre:
            try:
                pre_path = Path(str(pre)).expanduser()
            except Exception:
                pre_path = None
        if pre_path and pre_path.exists():
            return pre_path

        # Best-effort: regenerate missing `.i` from pinned compile_commands entry.
        try:
            entry = rec.get("compile_commands_entry") if isinstance(rec, dict) else None
            if entry and self.compile_commands_parser and self._env_flag("C2R_RETRY_PREPROCESS_MISSING_I", "0"):
                out_dir = None
                try:
                    env_dir = os.environ.get("PREPROCESS_OUTPUT_DIR", "").strip()
                    out_dir = Path(env_dir) if env_dir else None
                except Exception:
                    out_dir = None
                src_for_cc = None
                try:
                    src_for_cc = rec.get("source_for_cc_abs") if isinstance(rec, dict) else None
                except Exception:
                    src_for_cc = None
                if not src_for_cc and source_file:
                    src_for_cc = str(source_file)
                if src_for_cc:
                    ctx = self.compile_commands_parser.preprocess_with_context(
                        Path(str(src_for_cc)),
                        entry,
                        output_dir=out_dir,
                        timeout_sec=int(os.environ.get("C2R_PREPROCESS_TIMEOUT_SEC", "60")),
                    )
                    if ctx and getattr(ctx, "preprocessed_file", None):
                        p = Path(ctx.preprocessed_file)
                        if p.exists():
                            return p
        except Exception:
            pass

        return None

    def ensure_types_and_local_types_from_tu(
        self,
        *,
        module_name_to_src: Dict[str, Path],
        module_signatures: Dict[str, Sequence["FunctionSignature"]],
        header_files_present: bool,
    ) -> Dict[str, Any]:
        """
        Truth-first补全：
        - 全局 types.rs：缺失的“共享类型/全局变量类型”从 stage1 pinned `.i` 生成并 re-export。
        - 模块私有类型：仅在单个 `.c` 模块用到的类型，生成到该模块的 `local_types` 子模块（include!）。

        目标：
        - 不伪造宏/static inline extern 符号；
        - 不依赖 regex 修补“让它能编译”的派生层；
        - 让签名/globals/types 都来自同一套 tu_context_map.json pin 下来的 `.i`。
        """
        result: Dict[str, Any] = {
            "tu_context_map": str(getattr(self, "_tu_context_map_path", "") or ""),
            "tu_context_files": len(getattr(self, "_tu_context_files", {}) or {}),
            "global_missing_types": [],
            "local_missing_types": {},
            "generated_global_units": [],
            "generated_local_units": [],
            "skipped_missing_tu_i": [],
        }

        if not getattr(self, "_tu_context_files", None):
            return result

        src_dir = self.output_dir / "src"
        types_path = src_dir / "types.rs"
        globals_path = src_dir / "globals.rs"
        gen_dir = src_dir / "__c2r_generated"
        try:
            gen_dir.mkdir(parents=True, exist_ok=True)
        except Exception:
            pass

        # -----------------------------
        # Determine which types exist
        # -----------------------------
        defined_types: Set[str] = set()
        types_rs_text = ""
        try:
            if types_path.exists():
                types_rs_text = types_path.read_text(encoding="utf-8", errors="ignore")
                defined_types = self._parse_defined_type_names_from_types_rs_text(types_rs_text)
        except Exception:
            defined_types = set()
            types_rs_text = ""

        # If this is a headerless project, the stage-A placeholder `types.rs` is not truth.
        # Treat it as "no types defined" so we regenerate required items from pinned TU `.i`.
        if not header_files_present:
            if ("bindgen failed to generate types" in (types_rs_text or "")) or ("placeholder definitions" in (types_rs_text or "")):
                defined_types = set()

        # -----------------------------
        # Collect needed types (Rust signatures as compiled truth)
        # -----------------------------
        def _is_trivial_ident(name: str) -> bool:
            if not name:
                return True
            trivial = {
                # Rust primitives
                "i8",
                "u8",
                "i16",
                "u16",
                "i32",
                "u32",
                "i64",
                "u64",
                "i128",
                "u128",
                "isize",
                "usize",
                "f32",
                "f64",
                "bool",
                # Rust keywords / paths / common wrappers
                "pub",
                "unsafe",
                "extern",
                "fn",
                "mut",
                "const",
                "ref",
                "crate",
                "types",
                "core",
                "std",
                "alloc",
                "ffi",
                "Option",
                "Result",
                "Box",
                "Vec",
                "String",
                "MaybeUninit",
                "PhantomData",
                "NonNull",
                "ManuallyDrop",
                "CStr",
                "CString",
                # common core::ffi primitives
                "c_void",
                "c_char",
                "c_schar",
                "c_uchar",
                "c_short",
                "c_ushort",
                "c_int",
                "c_uint",
                "c_long",
                "c_ulong",
                "c_longlong",
                "c_ulonglong",
                "c_float",
                "c_double",
                # common tokens inside type exprs
                "where",
                "Self",
                "self",
            }
            return name in trivial

        def _idents_from_type_expr(expr: str) -> Set[str]:
            s = (expr or "").strip()
            if not s:
                return set()
            out: Set[str] = set()
            for m in re.findall(r"crate::types::([A-Za-z_]\w*)", s):
                if m and (not _is_trivial_ident(m)):
                    out.add(m)
            # Best-effort: take identifier tokens from the type expression.
            for ident in re.findall(r"[A-Za-z_]\w*", s):
                if ident and (not _is_trivial_ident(ident)):
                    out.add(ident)
            return out

        def _type_exprs_from_rust_sig(rust_sig: str) -> List[str]:
            rs = (rust_sig or "").strip()
            if not rs:
                return []
            # Drop any body if present.
            if "{" in rs:
                rs = rs.split("{", 1)[0].strip()
            m = re.search(r"\bfn\b", rs)
            if not m:
                return []
            lp = rs.find("(", m.end())
            if lp == -1:
                return []
            depth = 0
            rp = None
            for i in range(lp, len(rs)):
                ch = rs[i]
                if ch == "(":
                    depth += 1
                elif ch == ")":
                    depth = max(0, depth - 1)
                    if depth == 0:
                        rp = i
                        break
            if rp is None:
                return []
            params_str = rs[lp + 1 : rp]
            rest = rs[rp + 1 :]
            exprs: List[str] = []
            for item in self._split_top_level_commas(params_str):
                if ":" not in item:
                    continue
                _n, ty = item.split(":", 1)
                ty = (ty or "").strip()
                if ty:
                    exprs.append(ty)
            if "->" in rest:
                ret = rest.split("->", 1)[1]
                # Remove trailing where-clause or attributes.
                ret = ret.split(" where ", 1)[0].strip()
                if ret:
                    exprs.append(ret)
            return exprs

        sig_needed_by_mod: Dict[str, Set[str]] = {}
        for mod_name, sigs in (module_signatures or {}).items():
            need: Set[str] = set()
            for sig in sigs or []:
                try:
                    rs = str(getattr(sig, "rust_signature", "") or "").strip()
                except Exception:
                    rs = ""
                for expr in _type_exprs_from_rust_sig(rs):
                    need.update(_idents_from_type_expr(expr))
            sig_needed_by_mod[str(mod_name)] = need

        # Module-local statics (Scheme B): types used only by file-scope `static` vars should stay local.
        statics_needed_by_mod: Dict[str, Set[str]] = {}
        try:
            local_statics = getattr(self, "_module_local_file_statics", {}) or {}
            if isinstance(local_statics, dict):
                for mod_name, entries in local_statics.items():
                    need: Set[str] = set()
                    for ent in entries or []:
                        try:
                            ty = str((ent or {}).get("ty") or "").strip()
                        except Exception:
                            ty = ""
                        if ty:
                            need.update(_idents_from_type_expr(ty))
                    if need:
                        statics_needed_by_mod[str(mod_name)] = need
        except Exception:
            statics_needed_by_mod = {}

        # -----------------------------
        # Collect needed types (globals.rs)
        # -----------------------------
        global_needed: Set[str] = set()
        try:
            if globals_path.exists():
                txt = globals_path.read_text(encoding="utf-8", errors="ignore")
                # Prefer parsing actual static decl types.
                for ty_expr in re.findall(r"\bpub\s+static(?:\s+mut)?\s+[A-Za-z_]\w*\s*:\s*([^=;]+)", txt):
                    global_needed.update(_idents_from_type_expr(str(ty_expr)))
                # Also keep explicit `crate::types::X` occurrences.
                for m in re.findall(r"crate::types::([A-Za-z_]\w*)", txt):
                    if m and (not _is_trivial_ident(m)):
                        global_needed.add(m)
        except Exception:
            global_needed = set()

        # Compute usage counts across modules.
        type_to_mods: Dict[str, Set[str]] = defaultdict(set)
        for mod_name, types in sig_needed_by_mod.items():
            for t in types:
                type_to_mods[t].add(mod_name)

        # Decide whether to "globalize" signature types:
        # - When TU closure is required, skeleton signatures typically use `crate::types::T`.
        #   Any missing `T` must be present in types.rs, otherwise baseline compile fails.
        # - In relaxed mode, keep the older heuristic: only types shared by 2+ modules become global.
        try:
            require_tu = (os.environ.get("C2R_REQUIRE_TU_CLOSURE", "1") or "1").strip().lower() not in (
                "0",
                "false",
                "no",
            )
        except Exception:
            require_tu = True
        globalize_sig_types = self._env_flag(
            "C2R_TU_TRUTH_GLOBALIZE_SIGNATURE_TYPES",
            "1" if require_tu else "0",
        )

        # A type is "global" if:
        # - referenced by globals.rs, OR
        # - (TU-closure-required) referenced by ANY module signature, OR
        # - (legacy) referenced by 2+ modules.
        global_types: Set[str] = set(global_needed)
        if globalize_sig_types:
            for _mod, tys in sig_needed_by_mod.items():
                for t in tys:
                    if t:
                        global_types.add(t)
        else:
            for t, mods in type_to_mods.items():
                if len(mods) > 1:
                    global_types.add(t)

        # Missing types = not already defined in current types.rs.
        global_missing: Set[str] = {t for t in global_types if t and t not in defined_types}
        local_missing_by_mod: Dict[str, Set[str]] = {}
        if not globalize_sig_types:
            for mod_name, types in sig_needed_by_mod.items():
                local = {t for t in types if t and (t not in global_types) and (t not in defined_types)}
                if local:
                    local_missing_by_mod[mod_name] = set(local)
        # Always add file-scope static variable types as module-local (Scheme B), regardless of signature globalization.
        for mod_name, types in statics_needed_by_mod.items():
            local = {t for t in types if t and (t not in global_types) and (t not in defined_types)}
            if local:
                local_missing_by_mod.setdefault(mod_name, set()).update(local)

        result["global_missing_types"] = sorted(global_missing)
        result["local_missing_types"] = {k: sorted(v) for k, v in sorted(local_missing_by_mod.items(), key=lambda kv: kv[0])}

        # -----------------------------
        # Generate module-local types
        # -----------------------------
        def _lang_for_safe_name(safe_name: str) -> str:
            rec = self._tu_context_files.get(safe_name) if getattr(self, "_tu_context_files", None) else None
            if isinstance(rec, dict):
                p = rec.get("source_file_abs") or rec.get("source_for_cc_abs") or ""
                try:
                    suf = Path(str(p)).suffix.lower()
                except Exception:
                    suf = ""
                if suf in {".cc", ".cpp", ".cxx", ".c++"}:
                    return "c++"
            return "c"

        try:
            timeout_sec = int(os.environ.get("C2R_BINDGEN_TYPES_TIMEOUT_SEC", "120"))
        except Exception:
            timeout_sec = 120

        for mod_name, type_names in sorted(local_missing_by_mod.items(), key=lambda kv: kv[0]):
            pre_path = self._get_pinned_preprocessed_path_for_safe_name(mod_name, source_file=module_name_to_src.get(mod_name))
            if not pre_path:
                result["skipped_missing_tu_i"].append(mod_name)
                continue
            out_rs = gen_dir / f"local_types_{mod_name}.rs"
            ok = self._run_bindgen_allowlist_types(
                preprocessed_path=pre_path,
                type_names=sorted(type_names),
                lang=_lang_for_safe_name(mod_name),
                timeout_sec=timeout_sec,
                out_rs=out_rs,
            )
            if not ok:
                continue
            defined = self._extract_defined_type_names_from_bindgen_output(out_rs)
            requested = {n for n in (type_names or set()) if isinstance(n, str) and n.strip()}
            present = {n for n in requested if n in defined}
            missing = {n for n in requested if n not in defined}
            if present:
                result["generated_local_units"].append(mod_name)
            # Update remaining local-missing set to avoid importing non-existent names.
            if missing:
                local_missing_by_mod[mod_name] = set(missing)
            else:
                local_missing_by_mod.pop(mod_name, None)

            # Inject `local_types` include + import into the module file.
            try:
                mod_path = src_dir / f"{mod_name}.rs"
                if mod_path.exists():
                    txt = mod_path.read_text(encoding="utf-8", errors="ignore")
                    include_rel = f"__c2r_generated/local_types_{mod_name}.rs"
                    import_names = [n for n in sorted(present) if isinstance(n, str) and n.strip()]
                    import_line = f"use local_types::{{{', '.join(import_names)}}};" if import_names else ""
                    if "mod local_types" not in txt and include_rel not in txt:
                        lines = txt.splitlines()
                        insert_at = 0
                        for i, ln in enumerate(lines):
                            if ln.strip() == "use crate::compat::*;":
                                insert_at = i + 1
                                break
                        block = [
                            "",
                            "mod local_types {",
                            f'    include!(\"{include_rel}\");',
                            "}",
                        ]
                        if import_line:
                            block.append(import_line)
                        block.append("")
                        block.append("")
                        lines[insert_at:insert_at] = block
                        mod_path.write_text("\n".join(lines) + "\n", encoding="utf-8", errors="ignore")
                    else:
                        # Upgrade legacy glob import (can cause name ambiguity if bindgen emits dependent types).
                        if import_line and "use local_types::*;" in txt:
                            mod_path.write_text(txt.replace("use local_types::*;", import_line), encoding="utf-8", errors="ignore")
            except Exception:
                pass

        # -----------------------------
        # Generate missing *global* types from TU `.i` and re-export from crate::types
        # -----------------------------
        if global_missing:
            # Select a deterministic provider module per type (lexicographically smallest).
            providers: Dict[str, str] = {}
            all_modules = sorted(set((module_name_to_src or {}).keys()) or set(self._tu_context_files.keys()))
            for t in sorted(global_missing):
                mods = sorted(type_to_mods.get(t) or [])
                provider = mods[0] if mods else (all_modules[0] if all_modules else "")
                if provider:
                    providers[t] = provider

            by_provider: Dict[str, Set[str]] = defaultdict(set)
            for t, prov in providers.items():
                by_provider[prov].add(t)

            generated_units: List[str] = []
            exported_by_provider: Dict[str, List[str]] = {}
            exported_names: Set[str] = set()
            for prov, types in sorted(by_provider.items(), key=lambda kv: kv[0]):
                pre_path = self._get_pinned_preprocessed_path_for_safe_name(prov, source_file=module_name_to_src.get(prov))
                if not pre_path:
                    result["skipped_missing_tu_i"].append(prov)
                    continue
                out_rs = gen_dir / f"tu_types_{prov}.rs"
                ok = self._run_bindgen_allowlist_types(
                    preprocessed_path=pre_path,
                    type_names=sorted(types),
                    lang=_lang_for_safe_name(prov),
                    timeout_sec=timeout_sec,
                    out_rs=out_rs,
                )
                if not ok:
                    continue
                defined = self._extract_defined_type_names_from_bindgen_output(out_rs)
                requested = {n for n in (types or set()) if isinstance(n, str) and n.strip()}
                present = [n for n in sorted(requested) if n in defined]
                if present:
                    generated_units.append(prov)
                    exported_by_provider[prov] = present
                    exported_names.update(present)
            result["generated_global_units"] = generated_units

            if generated_units:
                # Write/update `types.rs` to re-export these items.
                reexport_block: List[str] = [
                    "",
                    "// ============================================================",
                    "// C2R: TU-pinned type supplements (from stage1 `.i` truth)",
                    "// ============================================================",
                ]
                for prov in generated_units:
                    mod_ident = f"__c2r_tu_types_{prov}"
                    include_rel = f"__c2r_generated/tu_types_{prov}.rs"
                    reexport_block.append(f"pub mod {mod_ident} {{")
                    reexport_block.append(f'    include!(\"{include_rel}\");')
                    reexport_block.append("}")
                    # Re-export only the types bindgen actually emitted to avoid E0432 at baseline compile.
                    names = exported_by_provider.get(prov) or []
                    if names:
                        reexport_block.append(f"pub use {mod_ident}::{{{', '.join(names)}}};")
                    reexport_block.append("")

                try:
                    if not types_path.exists() or (not header_files_present):
                        # Headerless project: overwrite placeholder types.rs with TU truth re-exports.
                        content = "\n".join(
                            [
                                "//! Auto-generated types module (TU truth entry)",
                                "//!",
                                "//! - Source of truth: stage1 pinned preprocessed `.i` from tu_context_map.json",
                                "//! - Shared/global types are re-exported here (crate::types::*).",
                                "//! - Per-module private types live in each module's `local_types` submodule.",
                                "",
                                "#![allow(non_camel_case_types)]",
                                "#![allow(non_snake_case)]",
                                "#![allow(non_upper_case_globals)]",
                                "#![allow(dead_code)]",
                                "#![allow(unused)]",
                            ]
                            + reexport_block
                        )
                        types_path.write_text(content + "\n", encoding="utf-8", errors="ignore")
                    else:
                        # Header-based types.rs exists: append TU supplements if not already present.
                        existing = types_path.read_text(encoding="utf-8", errors="ignore")
                        marker = "C2R: TU-pinned type supplements"
                        if marker not in existing:
                            types_path.write_text(existing.rstrip("\n") + "\n" + "\n".join(reexport_block) + "\n", encoding="utf-8", errors="ignore")
                except Exception:
                    pass

            # Update remaining global-missing set after generation (avoid stale report noise).
            try:
                global_missing = {t for t in global_missing if t and t not in exported_names}
            except Exception:
                pass

        # Finalize "still missing" lists after generation attempts.
        result["global_missing_types"] = sorted({t for t in (global_missing or set()) if t})
        result["local_missing_types"] = {k: sorted(v) for k, v in sorted((local_missing_by_mod or {}).items(), key=lambda kv: kv[0])}

        # Headerless projects: ensure `types.rs` is no longer the placeholder stub even if we didn't
        # need to re-export any shared/global types.
        if not header_files_present:
            try:
                existing = types_path.read_text(encoding="utf-8", errors="ignore") if types_path.exists() else ""
            except Exception:
                existing = ""
            if "Auto-generated types module (TU truth entry)" not in (existing or ""):
                if (not existing.strip()) or ("bindgen failed to generate types" in existing) or ("placeholder definitions" in existing):
                    try:
                        content = "\n".join(
                            [
                                "//! Auto-generated types module (TU truth entry)",
                                "//!",
                                "//! - Source of truth: stage1 pinned preprocessed `.i` from tu_context_map.json",
                                "//! - Shared/global types are re-exported here (crate::types::*).",
                                "//! - Per-module private types live in each module's `local_types` submodule.",
                                "",
                                "#![allow(non_camel_case_types)]",
                                "#![allow(non_snake_case)]",
                                "#![allow(non_upper_case_globals)]",
                                "#![allow(dead_code)]",
                                "#![allow(unused)]",
                                "",
                            ]
                        )
                        types_path.write_text(content, encoding="utf-8", errors="ignore")
                    except Exception:
                        pass

        # Best-effort: update types_generation_report.json for explainability.
        try:
            report_path = self.output_dir / "types_generation_report.json"
            base = {}
            if report_path.exists():
                try:
                    base = json.loads(report_path.read_text(encoding="utf-8", errors="ignore") or "{}")
                except Exception:
                    base = {}
            if not isinstance(base, dict):
                base = {}
            # Ensure the report stays compatible with Truth-mode gates in the pipeline.
            # Headerless projects may not have run Stage A bindgen, so the report could be empty
            # before TU-truth supplements are written here.
            base.setdefault("mode", "tu_truth")
            base.setdefault("success", True)
            base.setdefault("final_output_valid", True)
            # Backward-compatible fields expected by batch_test_staged.sh summary scripts.
            base.setdefault("compile_commands_loaded", bool(getattr(self, "compile_commands_parser", None)))
            base.setdefault(
                "compile_commands_path",
                str(self.compile_commands_parser.compile_db_path) if getattr(self, "compile_commands_parser", None) else None,
            )
            base.setdefault("tu_truth", {})
            base["tu_truth"].update(
                {
                    "tu_context_map": result.get("tu_context_map"),
                    "generated_global_units": result.get("generated_global_units"),
                    "generated_local_units": result.get("generated_local_units"),
                    "global_missing_types": result.get("global_missing_types"),
                    "local_missing_types": result.get("local_missing_types"),
                    "skipped_missing_tu_i": result.get("skipped_missing_tu_i"),
                }
            )
            report_path.write_text(json.dumps(base, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", errors="ignore")
        except Exception:
            pass

        return result

    def generate_globals_rs_bindgen_static(
        self,
        variables: List[VariableInfo],
        output_file: str = "globals.rs",
    ) -> Path:
        """
        Generate globals.rs using Scheme A:
        - Use bindgen allowlist on preprocessed `.i` to get *exact* Rust types for globals.
        - Emit real Rust storage: `pub static mut NAME: TYPE = zeroed();`
        - No Mutex/safe wrappers.
        """
        output_path = self.output_dir / "src" / output_file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        enable_fallback = (os.environ.get("C2R_ENABLE_GLOBALS_BINDGEN_FALLBACK", "false") or "false").strip().lower() in (
            "1",
            "true",
            "yes",
        )
        # Ensure stale RustMap safe-globals metadata doesn't affect later deterministic rewrites.
        try:
            meta_path = output_path.parent / "globals_accessors.json"
            if meta_path.exists():
                meta_path.unlink()
        except Exception:
            pass

        defined_types: Set[str] = set()
        try:
            types_rs = (self.output_dir / "src" / "types.rs").read_text(encoding="utf-8", errors="ignore")
            defined_types = self._parse_defined_type_names_from_types_rs_text(types_rs)
        except Exception:
            defined_types = set()

        # Split variables:
        # - global (external linkage)
        # - file-scope `static` (internal linkage) -> Scheme B: keep module-local, NOT in globals.rs
        # - lifted function-statics
        seen_names: Set[str] = set()
        unique_vars: List[Any] = []
        for v in variables or []:
            name = getattr(v, "name", None)
            if not isinstance(name, str) or not name:
                continue
            if name in seen_names:
                continue
            seen_names.add(name)
            unique_vars.append(v)

        global_vars: List[Any] = []
        file_static_vars: List[Any] = []
        lifted_vars: List[Any] = []
        for v in unique_vars:
            if getattr(v, "from_function", None):
                lifted_vars.append(v)
            else:
                if getattr(v, "is_static", False):
                    file_static_vars.append(v)
                else:
                    global_vars.append(v)

        vars_by_origin: Dict[str, Set[str]] = defaultdict(set)
        unassigned: Set[str] = set()
        # NOTE: For type-truth generation we still need bindgen info for module-local statics.
        for v in [*global_vars, *file_static_vars]:
            origin = getattr(v, "origin_file", None) or ""
            name = getattr(v, "name", "") or ""
            if not origin:
                unassigned.add(name)
            else:
                vars_by_origin[origin].add(name)

        timeout_sec = int(os.environ.get("C2R_BINDGEN_GLOBALS_TIMEOUT_SEC", "90"))
        tmp_dir = self.output_dir / ".c2r_bindgen_globals"
        bindgen_types: Dict[str, Dict[str, str]] = {}
        origin_by_var: Dict[str, str] = {}

        def _lang_for_safe_name(safe_name: str) -> str:
            rec = self._tu_context_files.get(safe_name) if getattr(self, "_tu_context_files", None) else None
            if isinstance(rec, dict):
                p = rec.get("source_file_abs") or rec.get("source_for_cc_abs") or ""
                try:
                    suf = Path(str(p)).suffix.lower()
                except Exception:
                    suf = ""
                if suf in {".cc", ".cpp", ".cxx", ".c++"}:
                    return "c++"
            return "c"

        for safe_name, names in sorted(vars_by_origin.items(), key=lambda kv: kv[0]):
            rec = self._tu_context_files.get(safe_name) if getattr(self, "_tu_context_files", None) else None
            pre = rec.get("preprocessed_file") if isinstance(rec, dict) else None
            if not pre:
                continue
            try:
                pre_path = Path(str(pre)).expanduser()
            except Exception:
                continue
            if not pre_path.exists():
                continue
            got = self._run_bindgen_allowlist_vars(
                preprocessed_path=pre_path,
                var_names=sorted(names),
                lang=_lang_for_safe_name(safe_name),
                defined_types=defined_types,
                timeout_sec=timeout_sec,
                tmp_dir=tmp_dir,
            )
            for n, info in got.items():
                if n and isinstance(info, dict) and info.get("ty"):
                    bindgen_types.setdefault(n, dict(info))
                    origin_by_var.setdefault(str(n), str(safe_name))

        # Best-effort: scan other `.i` files for remaining vars (including those without origin_file).
        missing: Set[str] = {
            getattr(v, "name", "")
            for v in [*global_vars, *file_static_vars]
            if getattr(v, "name", "") not in bindgen_types
        }
        missing.discard("")
        missing.update(unassigned)
        missing = {m for m in missing if m and m not in bindgen_types}
        if missing and getattr(self, "_tu_context_files", None):
            for safe_name, rec in sorted(self._tu_context_files.items(), key=lambda kv: kv[0]):
                if not missing:
                    break
                if not isinstance(rec, dict):
                    continue
                pre = rec.get("preprocessed_file")
                if not pre:
                    continue
                try:
                    pre_path = Path(str(pre)).expanduser()
                except Exception:
                    continue
                if not pre_path.exists():
                    continue
                got = self._run_bindgen_allowlist_vars(
                    preprocessed_path=pre_path,
                    var_names=sorted(missing),
                    lang=_lang_for_safe_name(safe_name),
                    defined_types=defined_types,
                    timeout_sec=timeout_sec,
                    tmp_dir=tmp_dir,
                )
                if not got:
                    continue
                for n, info in got.items():
                    if n and isinstance(info, dict) and info.get("ty"):
                        bindgen_types.setdefault(n, dict(info))
                        origin_by_var.setdefault(str(n), str(safe_name))
                    missing.discard(n)

        # Scheme B: file-scope `static` variables should stay module-local (internal linkage).
        # We still derive their Rust types from bindgen on the pinned TU `.i`, but we do NOT put them in globals.rs.
        module_local_statics: Dict[str, List[Dict[str, str]]] = defaultdict(list)
        omitted_local_statics: List[Dict[str, str]] = []
        for v in file_static_vars:
            name = getattr(v, "name", "") or ""
            if not name:
                continue
            safe_name = getattr(v, "origin_file", None) or origin_by_var.get(name) or ""
            safe_name = str(safe_name) if safe_name else ""
            info = bindgen_types.get(name) or {}
            ty = str(info.get("ty") or "").strip()
            init = str(info.get("init") or "").strip()
            if not ty and enable_fallback:
                rt = getattr(v, "rust_type", "") or ""
                ty = self._prefix_types_in_rust_type_expr(rt, defined_types) if rt else ""
            if not ty:
                omitted_local_statics.append({"name": name, "origin_file": safe_name})
                continue
            module_local_statics[safe_name].append(
                {
                    "name": name,
                    "ty": ty,
                    "init": init,
                    "c_type": str(getattr(v, "c_type", "") or ""),
                }
            )

        # Persist for later phases (module file generation + TU-truth local types extraction).
        try:
            self._module_local_file_statics = {
                k: sorted(vs, key=lambda x: x.get("name") or "")
                for k, vs in sorted(module_local_statics.items(), key=lambda kv: kv[0])
                if k and vs
            }
        except Exception:
            self._module_local_file_statics = {}

        # Emit globals.rs
        lines: List[str] = [
            "//! Global and Static Variable Declarations (Scheme A: bindgen-truth static storage)",
            "//!",
            "//! - No safe wrappers (Mutex/RwLock).",
            "//! - Types are derived from bindgen on the exact preprocessed `.i` TU.",
            "//! - Storage is real Rust `static mut`, zero-initialized (C-like).",
            "//! - NOTE: file-scope `static` (internal linkage) variables are emitted in each module file (Scheme B).",
            "",
            "#![allow(non_upper_case_globals)]",
            "#![allow(non_snake_case)]",
            "#![allow(dead_code)]",
            "#![allow(unused)]",
            "",
            "use core::mem::MaybeUninit;",
            "use crate::types::*;",
            "",
        ]
        omitted_globals: List[Dict[str, str]] = []

        if global_vars:
            lines.append("// ==========================================")
            lines.append("// Global Variables (top-level)")
            lines.append("// ==========================================")
            lines.append("")

            for name in sorted({getattr(v, "name", "") for v in global_vars if getattr(v, "name", "")}):
                info = bindgen_types.get(name) or {}
                ty = str(info.get("ty") or "").strip()
                init = str(info.get("init") or "").strip()
                if not ty:
                    if enable_fallback:
                        # Emergency-only fallback: use best-effort type (tree-sitter/type-mapper).
                        # Default OFF: if bindgen can't derive a decl, we treat it as an input-closure issue and report it.
                        rt = ""
                        try:
                            for v in global_vars:
                                if getattr(v, "name", "") == name:
                                    rt = getattr(v, "rust_type", "") or ""
                                    break
                        except Exception:
                            rt = ""
                        ty = self._prefix_types_in_rust_type_expr(rt, defined_types) if rt else ""
                        if not ty:
                            ty = "*mut core::ffi::c_void"
                        lines.append("// Source: fallback (no bindgen decl found)")
                    else:
                        omitted_globals.append(
                            {
                                "name": name,
                                "origin_file": str(
                                    next(
                                        (getattr(v, "origin_file", "") or "" for v in global_vars if getattr(v, "name", "") == name),
                                        "",
                                    )
                                ),
                            }
                        )
                        lines.append("// Source: bindgen missing (declaration omitted; see globals_generation_report.json)")
                        lines.append(f"// MISSING: {name}")
                        lines.append("")
                        continue
                else:
                    lines.append("// Source: bindgen on preprocessed TU")

                # Try to extract initializer from C source files if bindgen doesn't provide one
                c_init = None
                if not init:
                    try:
                        c_source_files = self._get_project_source_files()
                        c_init = self._extract_c_array_initializer(c_source_files, name)
                        if c_init:
                            logger.info(f"从 C 源文件提取全局变量 {name} 的初始化值")
                    except Exception as e:
                        logger.debug(f"提取 {name} 初始化值失败: {e}")

                if init:
                    lines.append(f"pub static mut {name}: {ty} = {init};")
                elif c_init:
                    lines.append(f"// Initializer extracted from C source")
                    lines.append(f"pub static mut {name}: {ty} = {c_init};")
                else:
                    lines.append(f"pub static mut {name}: {ty} = unsafe {{ MaybeUninit::<{ty}>::zeroed().assume_init() }};")
                lines.append("")

        if lifted_vars:
            lines.append("// ==========================================")
            lines.append("// Lifted Static Variables (from functions)")
            lines.append("// ==========================================")
            lines.append("")

            by_fn: Dict[str, List[Any]] = defaultdict(list)
            for v in lifted_vars:
                by_fn[str(getattr(v, "from_function", "") or "")].append(v)
            for fn in sorted(by_fn.keys()):
                if not fn:
                    continue
                lines.append(f"// From function: {fn}()")
                for v in by_fn[fn]:
                    name = getattr(v, "name", "") or ""
                    c_type = getattr(v, "c_type", "") or ""
                    rt = getattr(v, "rust_type", "") or ""
                    ty = self._prefix_types_in_rust_type_expr(rt, defined_types) if rt else ""
                    if not ty:
                        ty = "*mut core::ffi::c_void"
                    if c_type:
                        original_name = name.replace(f"{fn}_", "")
                        lines.append(f"/// Originally: static {c_type} {original_name}")
                    lines.append(f"pub static mut {name}: {ty} = unsafe {{ MaybeUninit::<{ty}>::zeroed().assume_init() }};")
                lines.append("")

        if not variables:
            lines.append("// No global or static variables found in this project.")
            lines.append("")

        output_path.write_text("\n".join(lines), encoding="utf-8")
        # Dedup across files: if globals.rs defines real Rust storage for a global, keep it only in globals.rs.
        # bindgen also emits `extern "C" { pub static mut NAME: ...; }` in types.rs, which causes
        # ambiguous-name errors (E0659) when modules import both `crate::types::*` and `crate::globals::*`.
        try:
            types_path = output_path.parent / "types.rs"
            removed = self._remove_duplicate_extern_statics_from_types_rs(
                globals_rs_path=output_path,
                types_rs_path=types_path,
            )
            if removed:
                logger.info(f"types.rs 去重: 移除 {removed} 个与 globals.rs 重复的 extern static 声明")
        except Exception:
            pass
        # 2026-01-07 修复: 同样去重 compat.rs，避免 E0659 歧义错误
        # compat.rs 的 C2R_EXTERN_VARS 块可能声明了与 globals.rs 重复的变量 (如 stderr, incs 等)
        try:
            compat_path = output_path.parent / "compat.rs"
            if compat_path.exists():
                removed_compat = self._remove_duplicate_extern_statics_from_types_rs(
                    globals_rs_path=output_path,
                    types_rs_path=compat_path,  # 复用同一去重逻辑
                )
                if removed_compat:
                    logger.info(f"compat.rs 去重: 移除 {removed_compat} 个与 globals.rs 重复的 extern static 声明")
        except Exception:
            pass
        # Save a machine-readable report for diagnosing "globals closure" issues.
        try:
            report_path = self.output_dir / "globals_generation_report.json"
            report = {
                "generated_at": datetime.now().isoformat(timespec="seconds"),
                "mode": "bindgen_static",
                "globals_total": len(global_vars),
                "file_static_total": len(file_static_vars),
                "module_local_static_total": int(
                    sum(len(v) for v in (getattr(self, "_module_local_file_statics", {}) or {}).values())
                ),
                "lifted_total": len(lifted_vars),
                "bindgen_ok": len(bindgen_types),
                "bindgen_missing_names": sorted(missing),
                "omitted_globals": omitted_globals,
                "omitted_module_local_statics": omitted_local_statics,
                "enable_fallback": enable_fallback,
                "tu_context_map": str(getattr(self, "_tu_context_map_path", "") or ""),
            }
            report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        except Exception:
            pass
        logger.info(
            f"生成 globals.rs (bindgen_static): {len(global_vars)} 个顶层变量 + {len(file_static_vars)} 个模块内 static + {len(lifted_vars)} 个提升变量 "
            f"(bindgen_ok={len(bindgen_types)}, fallback_missing={len(missing)})"
        )
        return output_path

    @staticmethod
    def _extract_pub_static_names_from_globals_rs(globals_text: str) -> Set[str]:
        """
        Extract `pub static (mut)? NAME:` names from globals.rs (Scheme A).
        """
        if not globals_text:
            return set()
        try:
            return set(re.findall(r"(?m)^\s*pub\s+static(?:\s+mut)?\s+([A-Za-z_]\w*)\s*:", globals_text))
        except Exception:
            return set()

    def _remove_duplicate_extern_statics_from_types_rs(
        self,
        *,
        globals_rs_path: Path,
        types_rs_path: Path,
    ) -> int:
        """
        Remove `extern "C" { pub static ...; }` entries from types.rs when the same global is defined
        in globals.rs (real Rust storage). This prevents E0659 ambiguous-name failures.
        """
        if not globals_rs_path or not globals_rs_path.exists():
            return 0
        if not types_rs_path or not types_rs_path.exists():
            return 0
        try:
            globals_text = globals_rs_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            globals_text = ""
        names = self._extract_pub_static_names_from_globals_rs(globals_text)
        if not names:
            return 0
        try:
            types_text = types_rs_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            types_text = ""
        if not types_text:
            return 0

        lines = types_text.splitlines()
        out: List[str] = []
        removed = 0
        i = 0
        while i < len(lines):
            ln = lines[i]
            s = (ln or "").strip()
            m = re.match(r"^(?:pub\s+)?static\s+(?:mut\s+)?([A-Za-z_]\w*)\s*:", s)
            if m:
                name = (m.group(1) or "").strip()
                if name and name in names:
                    # Drop directly-attached attributes above this item (bindgen uses #[doc = ...], #[link_name = ...], etc).
                    while out and (out[-1] or "").strip().startswith("#["):
                        out.pop()
                    removed += 1
                    # Skip until the end of the declaration (`;`), in case bindgen wraps long types.
                    while i < len(lines):
                        if ";" in (lines[i] or ""):
                            i += 1
                            break
                        i += 1
                    # Also drop a single trailing blank line to keep formatting tidy.
                    if i < len(lines) and (lines[i] or "").strip() == "":
                        i += 1
                    continue
            out.append(ln)
            i += 1

        if removed <= 0:
            return 0

        new_text = "\n".join(out) + ("\n" if types_text.endswith("\n") else "")
        try:
            types_rs_path.write_text(new_text, encoding="utf-8", errors="ignore")
        except Exception:
            return 0
        return removed
    
    def generate_globals_rs(
        self, 
        variables: List[VariableInfo], 
        output_file: str = "globals.rs",
        use_safe_wrappers: bool = False
    ) -> Path:
        """
        生成 globals.rs 文件
        
        基于 EvoC2Rust 的变量封装策略：
        - 简单类型使用 static mut
        - 复杂类型使用 static mut（需要 unsafe 访问）
        - 函数内提升的 static 变量添加函数名前缀
        
        增强功能：
        - use_safe_wrappers=True 时，使用 Mutex/RwLock 封装（更安全但性能略低）
        
        Args:
            variables: 变量列表
            output_file: 输出文件名
            use_safe_wrappers: 是否使用安全包装（Mutex/lazy_static）
        
        Returns:
            输出文件路径
        """
        output_path = self.output_dir / "src" / output_file
        
        if use_safe_wrappers:
            return self._generate_safe_globals_rs(variables, output_path)
        
        lines = [
            '//! Global and Static Variable Declarations',
            '//!',
            '//! Auto-generated from C source code using tree-sitter analysis.',
            '//!',
            '//! ## Variable Lifting (Static Promotion)',
            '//! C allows `static` variables inside functions, but Rust does not.',
            '//! These variables are "lifted" to module level with function name prefix.',
            '//!',
            '//! Example:',
            '//! ```c',
            '//! void foo() { static int count = 0; }  // C code',
            '//! ```',
            '//! Becomes:',
            '//! ```rust',
            '//! static mut foo_count: i32 = 0;  // Rust code',
            '//! ```',
            '//!',
            '//! ## Usage',
            '//! Access these variables using `unsafe`:',
            '//! ```rust',
            '//! unsafe { foo_count += 1; }',
            '//! ```',
            '',
            '#![allow(non_upper_case_globals)]',
            '#![allow(non_snake_case)]',
            '#![allow(dead_code)]',
            '#![allow(unused)]',
            '',
        ]

        # Normalize and qualify pthread initializers in generated declarations.
        # This avoids fragile `use crate::types::...` imports that can mismatch across libc/boards
        # (e.g. `__PTHREAD_MUTEX_INITIALIZER` vs `PTHREAD_MUTEX_INITIALIZER`).
        pthread_aliases = {
            "__PTHREAD_MUTEX_INITIALIZER": "PTHREAD_MUTEX_INITIALIZER",
            "__PTHREAD_COND_INITIALIZER": "PTHREAD_COND_INITIALIZER",
            "__PTHREAD_RWLOCK_INITIALIZER": "PTHREAD_RWLOCK_INITIALIZER",
            "__PTHREAD_ONCE_INIT": "PTHREAD_ONCE_INIT",
        }
        pthread_constants = sorted(set(pthread_aliases.values()))

        def _fix_pthread_initializers_in_decl(decl: str) -> str:
            if not decl:
                return decl
            for src, dst in pthread_aliases.items():
                if src in decl:
                    decl = decl.replace(src, dst)
            for name in pthread_constants:
                decl = re.sub(rf'(?<!crate::types::)\b{name}\b', f'crate::types::{name}', decl)
            return decl
        
        # 去重：防止同名变量重复声明（可能来自多个源文件）
        seen_names = set()
        unique_variables = []
        for var in variables:
            if var.name not in seen_names:
                seen_names.add(var.name)
                unique_variables.append(var)
            else:
                logger.debug(f"跳过重复变量: {var.name}")
        
        # 分类变量
        global_vars = []      # 顶层全局变量
        lifted_vars = []      # 从函数内提升的 static 变量
        
        for var in unique_variables:
            if var.from_function:
                lifted_vars.append(var)
            else:
                global_vars.append(var)
        
        # 生成顶层全局变量
        if global_vars:
            lines.append('// ==========================================')
            lines.append('// Global Variables (top-level declarations)')
            lines.append('// ==========================================')
            lines.append('')
            for var in global_vars:
                lines.append(f'/// C type: {var.c_type}')
                decl = _fix_pthread_initializers_in_decl(getattr(var, "rust_declaration", "") or "")
                lines.append(decl)
                lines.append('')
        
        # 生成提升的 static 变量
        if lifted_vars:
            lines.append('// ==========================================')
            lines.append('// Lifted Static Variables (from functions)')
            lines.append('// ==========================================')
            lines.append('')
            
            # 按源函数分组
            from collections import defaultdict
            by_function = defaultdict(list)
            for var in lifted_vars:
                by_function[var.from_function].append(var)
            
            for func_name in sorted(by_function.keys()):
                lines.append(f'// From function: {func_name}()')
                for var in by_function[func_name]:
                    original_name = var.name.replace(f"{func_name}_", "")
                    lines.append(f'/// Originally: static {var.c_type} {original_name}')
                    decl = _fix_pthread_initializers_in_decl(getattr(var, "rust_declaration", "") or "")
                    lines.append(decl)
                lines.append('')
        
        # 如果没有变量，添加占位符
        if not variables:
            lines.append('// No global or static variables found in this project.')
            lines.append('')
        
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        logger.info(f"生成 globals.rs: {len(variables)} 个变量 (其中 {len(lifted_vars)} 个提升)")
        return output_path
    
    def _generate_safe_globals_rs(self, variables: List[VariableInfo], output_path: Path) -> Path:
        """
        生成 RustMap 风格的“安全全局变量”globals.rs（规则为主，LLM 为辅）

        策略（与 RustMap 的描述对齐）：
        - 对“primitive/C-POD/指针”类全局变量：使用 `Mutex<T>` 托管，并生成 getter/setter/with_* API
          这样函数体里无需 `unsafe { static mut }`，并且天然线程安全。
        - 对复杂类型（struct/union/数组/未知初始化）：仍保持 `static mut`（保持可编译优先）

        说明：
        - 这里不依赖 `lazy_static`，避免额外 Cargo 依赖；Rust 1.9x 支持 `static Mutex::new(...)`。
        - 仅对“能 const 初始化”的默认值做 Mutex 初始化；复杂类型留给后续修复/语义阶段处理。
        """
        def _sanitize_ident(name: str) -> str:
            s = re.sub(r"[^0-9A-Za-z_]", "_", name or "")
            if not s:
                return "_"
            if s[0].isdigit():
                s = "_" + s
            keywords = {
                "as",
                "break",
                "const",
                "continue",
                "crate",
                "else",
                "enum",
                "extern",
                "false",
                "fn",
                "for",
                "if",
                "impl",
                "in",
                "let",
                "loop",
                "match",
                "mod",
                "move",
                "mut",
                "pub",
                "ref",
                "return",
                "self",
                "Self",
                "static",
                "struct",
                "super",
                "trait",
                "true",
                "type",
                "unsafe",
                "use",
                "where",
                "while",
                "async",
                "await",
                "dyn",
            }
            if s in keywords:
                s = s + "_"
            return s

        def _type_base(t: str) -> str:
            return (t or "").strip().replace("crate::types::", "")

        def _is_mutex_safe(var: VariableInfo) -> bool:
            if var.is_array:
                return False
            rt = (var.rust_type or "").strip()
            if not rt:
                return False
            if "[" in rt or "]" in rt:
                return False
            if "MaybeUninit" in rt:
                return False
            # Pointers and primitive scalars are OK.
            if rt.startswith("*mut ") or rt.startswith("*const "):
                return True
            base = _type_base(rt)
            primitives = {
                "i8",
                "u8",
                "i16",
                "u16",
                "i32",
                "u32",
                "i64",
                "u64",
                "isize",
                "usize",
                "f32",
                "f64",
                "bool",
                # common libc typedef aliases
                "c_char",
                "c_schar",
                "c_uchar",
                "c_short",
                "c_ushort",
                "c_int",
                "c_uint",
                "c_long",
                "c_ulong",
                "c_longlong",
                "c_ulonglong",
                "size_t",
                "ssize_t",
                "pid_t",
                "uid_t",
                "gid_t",
                "time_t",
                "off_t",
            }
            if base in primitives:
                return True
            # Anything else under crate::types is likely a struct/union -> not safe by default.
            if rt.startswith("crate::types::"):
                return False
            return False

        def _default_expr(rt: str) -> str:
            rt = (rt or "").strip()
            if rt.startswith("*mut "):
                # NOTE: pointer safe-globals use `Mutex<usize>` storage, so default is 0.
                return "0"
            if rt.startswith("*const "):
                return "0"
            base = _type_base(rt)
            if base == "bool":
                return "false"
            if base in {"f32", "f64"}:
                return "0.0"
            return "0"

        # Normalize and qualify pthread initializers in generated declarations.
        pthread_aliases = {
            "__PTHREAD_MUTEX_INITIALIZER": "PTHREAD_MUTEX_INITIALIZER",
            "__PTHREAD_COND_INITIALIZER": "PTHREAD_COND_INITIALIZER",
            "__PTHREAD_RWLOCK_INITIALIZER": "PTHREAD_RWLOCK_INITIALIZER",
            "__PTHREAD_ONCE_INIT": "PTHREAD_ONCE_INIT",
        }
        pthread_constants = sorted(set(pthread_aliases.values()))

        def _fix_pthread_initializers_in_decl(decl: str) -> str:
            if not decl:
                return decl
            for src, dst in pthread_aliases.items():
                if src in decl:
                    decl = decl.replace(src, dst)
            for name in pthread_constants:
                decl = re.sub(rf"(?<!crate::types::)\\b{name}\\b", f"crate::types::{name}", decl)
            return decl

        lines: List[str] = [
            '//! Global and Static Variable Declarations (RustMap-style Safe Globals)',
            '//!',
            '//! Auto-generated from C source code using tree-sitter analysis.',
            '//!',
            '//! Safe globals (primitive/pointer) are wrapped in `Mutex<T>` and accessed via getters/setters.',
            '//! Complex globals stay as `static mut` as a compilation-first fallback.',
            '',
            '#![allow(non_upper_case_globals)]',
            '#![allow(non_snake_case)]',
            '#![allow(dead_code)]',
            '#![allow(unused)]',
            '',
            'use std::sync::Mutex;',
            '',
        ]

        # 去重：防止同名变量重复声明（可能来自多个源文件）
        seen_names: Set[str] = set()
        unique_variables: List[VariableInfo] = []
        for var in variables:
            if var.name in seen_names:
                continue
            seen_names.add(var.name)
            unique_variables.append(var)

        # 分类变量
        global_vars: List[VariableInfo] = []
        lifted_vars: List[VariableInfo] = []
        for var in unique_variables:
            if var.from_function:
                lifted_vars.append(var)
            else:
                global_vars.append(var)

        meta: Dict[str, Any] = {
            "mode": "rustmap",
            "safe_globals": [],
            "unsafe_globals": [],
        }

        def _emit_var_group(title: str, vars_list: List[VariableInfo], *, is_lifted: bool):
            if not vars_list:
                return
            lines.append("// ==========================================")
            lines.append(f"// {title}")
            lines.append("// ==========================================")
            lines.append("")

            for var in vars_list:
                safe_ident = _sanitize_ident(var.name)
                is_safe = _is_mutex_safe(var)

                if is_safe:
                    cell_name = _sanitize_ident(var.name).upper()
                    get_fn = f"get_{safe_ident}"
                    set_fn = f"set_{safe_ident}"
                    with_fn = f"with_{safe_ident}"
                    default_expr = _default_expr(var.rust_type)
                    lines.append(f"/// C type: {var.c_type}")
                    if is_lifted and var.from_function:
                        original_name = var.name.replace(f"{var.from_function}_", "")
                        lines.append(f"/// Originally: static {var.c_type} {original_name} in {var.from_function}()")
                    # IMPORTANT:
                    # - `Mutex<T>` in a `static` requires `T: Send` (so that `Mutex<T>: Sync`).
                    # - Raw pointers / NonNull are NOT `Send` on recent Rust, so `Mutex<*mut T>` fails to compile.
                    # Strategy:
                    # - For pointer-typed globals, store as `usize` (which is `Send`) and cast in accessors.
                    rt = (var.rust_type or "").strip()
                    if rt.startswith("*mut ") or rt.startswith("*const "):
                        lines.append(f"// NOTE: pointer global stored as usize for `static Mutex` compatibility.")
                        lines.append(f"pub static {cell_name}: Mutex<usize> = Mutex::new({default_expr});")
                        lines.append(
                            f"pub fn {get_fn}() -> {rt} {{ *{cell_name}.lock().unwrap() as {rt} }}"
                        )
                        lines.append(
                            f"pub fn {set_fn}(v: {rt}) {{ *{cell_name}.lock().unwrap() = v as usize; }}"
                        )
                        lines.append(f"pub fn {with_fn}<R>(f: impl FnOnce(&mut {rt}) -> R) -> R {{")
                        lines.append(f"    let mut guard = {cell_name}.lock().unwrap();")
                        lines.append(f"    let mut tmp: {rt} = *guard as {rt};")
                        lines.append("    let r = f(&mut tmp);")
                        lines.append("    *guard = tmp as usize;")
                        lines.append("    r")
                        lines.append("}")
                    else:
                        lines.append(f"pub static {cell_name}: Mutex<{rt}> = Mutex::new({default_expr});")
                        lines.append(f"pub fn {get_fn}() -> {rt} {{ *{cell_name}.lock().unwrap() }}")
                        lines.append(f"pub fn {set_fn}(v: {rt}) {{ *{cell_name}.lock().unwrap() = v; }}")
                        lines.append(f"pub fn {with_fn}<R>(f: impl FnOnce(&mut {rt}) -> R) -> R {{")
                        lines.append(f"    let mut guard = {cell_name}.lock().unwrap();")
                        lines.append("    f(&mut *guard)")
                        lines.append("}")
                    lines.append("")
                    meta["safe_globals"].append(
                        {
                            "name": var.name,
                            "sanitized": safe_ident,
                            "rust_type": var.rust_type,
                            "cell": cell_name,
                            "get": get_fn,
                            "set": set_fn,
                            "with": with_fn,
                            "from_function": var.from_function,
                        }
                    )
                else:
                    # Keep original declaration (usually `pub static mut ...`) for complex types.
                    lines.append(f"/// C type: {var.c_type}")
                    if is_lifted and var.from_function:
                        original_name = var.name.replace(f"{var.from_function}_", "")
                        lines.append(f"/// Originally: static {var.c_type} {original_name} in {var.from_function}()")
                    decl = _fix_pthread_initializers_in_decl(getattr(var, "rust_declaration", "") or "")
                    lines.append(decl)
                    lines.append("")
                    meta["unsafe_globals"].append(
                        {
                            "name": var.name,
                            "rust_type": var.rust_type,
                            "decl": decl,
                            "from_function": var.from_function,
                        }
                    )

        _emit_var_group("Global Variables (top-level)", global_vars, is_lifted=False)
        _emit_var_group("Lifted Static Variables (from functions)", lifted_vars, is_lifted=True)

        if not variables:
            lines.append("// No global or static variables found in this project.")
            lines.append("")

        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text("\n".join(lines), encoding="utf-8")

        # Also emit a machine-readable map for deterministic rewriting passes.
        try:
            meta_path = output_path.parent / "globals_accessors.json"
            meta_path.write_text(json.dumps(meta, ensure_ascii=False, indent=2), encoding="utf-8")
        except Exception:
            pass

        logger.info(
            f"生成 RustMap 风格 globals.rs: {len(variables)} 个变量 "
            f"(safe={len(meta['safe_globals'])}, unsafe={len(meta['unsafe_globals'])})"
        )
        return output_path
    
    def _get_safe_type_and_default(self, var: VariableInfo) -> Tuple[str, str]:
        """
        获取用于 Mutex 封装的安全类型和默认值
        
        Returns:
            (类型字符串, 默认值字符串)
        """
        rust_type = var.rust_type
        
        # 处理指针类型 -> Option<NonNull<T>>
        if rust_type.startswith('*mut ') or rust_type.startswith('*const '):
            inner_type = rust_type.replace('*mut ', '').replace('*const ', '')
            return f"Option<std::ptr::NonNull<{inner_type}>>", "None"
        
        # 处理基础类型
        if rust_type in ['i32', 'u32', 'i64', 'u64', 'i16', 'u16', 'i8', 'u8', 'usize', 'isize']:
            return rust_type, "0"
        elif rust_type in ['f32', 'f64']:
            return rust_type, "0.0"
        elif rust_type == 'bool':
            return "bool", "false"
        
        # 复杂类型 -> Option<T>
        if rust_type.startswith('crate::types::'):
            return f"Option<{rust_type}>", "None"
        
        # MaybeUninit 类型 -> Option<T>
        if 'MaybeUninit' in rust_type:
            # 提取内部类型
            inner_match = re.search(r'MaybeUninit<(.+)>', rust_type)
            if inner_match:
                inner_type = inner_match.group(1)
                return f"Option<{inner_type}>", "None"
        
        # 默认情况
        return f"Option<{rust_type}>", "None"
    
    # =========================================================================
    # 阶段 C: 函数骨架层 (The Logic Skeleton)
    # =========================================================================
    
    def _preprocess_source_for_parsing(self, source_code: str) -> str:
        """
        预处理源码以提高 Tree-sitter 解析成功率
        
        处理的问题：
        1. Windows 换行符 (CRLF) 导致解析问题
        2. #ifdef __cplusplus extern "C" { #endif 导致解析混乱
        3. 条件编译包裹的 extern "C" 块
        
        策略：
        - 统一换行符为 LF
        - 移除 #ifdef __cplusplus ... extern "C" { ... #endif 包装
        - 移除对应的 #ifdef __cplusplus } #endif 结束标记
        - 保留实际的代码内容
        """
        import re
        
        # 首先统一换行符 (CRLF -> LF)
        processed = source_code.replace('\r\n', '\n').replace('\r', '\n')
        
        # 模式1: 移除开始的 #ifdef __cplusplus extern "C" { #endif
        # 这种模式会导致 Tree-sitter CPP parser 混乱
        pattern_start = r'#ifdef\s+__cplusplus\s*\n\s*extern\s+"C"\s*\{\s*\n\s*#endif'
        processed = re.sub(pattern_start, '/* extern "C" removed for parsing */', processed)
        
        # 模式2: 移除结束的 #ifdef __cplusplus } #endif  
        pattern_end = r'#ifdef\s+__cplusplus\s*\n\s*\}\s*\n\s*#endif'
        processed = re.sub(pattern_end, '/* extern "C" end removed */', processed)
        
        # 模式3: 处理更紧凑的格式
        pattern_start_compact = r'#ifdef\s+__cplusplus\s+extern\s+"C"\s*\{\s+#endif'
        processed = re.sub(pattern_start_compact, '', processed)
        
        pattern_end_compact = r'#ifdef\s+__cplusplus\s+\}\s+#endif'
        processed = re.sub(pattern_end_compact, '', processed)
        
        return processed
    
    def extract_function_signatures(self, source_code: str) -> List[FunctionSignature]:
        """
        使用 Tree-sitter 提取所有函数签名
        
        增强功能：
        1. 支持 C++ 类成员函数 (method_definition)
        2. 支持 extern "C" 块内的函数 (linkage_specification)
        3. 支持 namespace 内的函数
        4. 预处理移除 #ifdef __cplusplus extern "C" 包装（避免解析错误）
        5. AST 健康检查：检测 ERROR 节点，预警解析问题
        
        Returns:
            函数签名列表
        """
        signatures = []
        seen_funcs = set()  # 避免重复
        
        try:
            # 预处理源码，移除会导致解析问题的条件编译块
            processed_source = self._preprocess_source_for_parsing(source_code)
            
            # 关键：将字符串转换为 bytes，Tree-sitter 使用字节偏移
            # 必须用 bytes 来提取内容，否则中文注释等多字节字符会导致偏移错误
            processed_bytes = bytes(processed_source, 'utf-8')
            tree = cpp_parser.parse(processed_bytes)
            
            # ========== 0. AST 健康检查 (基于 EvoC2Rust 的策略) ==========
            health_report = self._check_ast_health(tree.root_node)
            if not health_report['is_healthy']:
                logger.warning(
                    f"⚠️ AST 解析质量警告: 错误率 {health_report['error_rate']:.1%} "
                    f"({health_report['error_nodes']}/{health_report['total_nodes']} 节点)"
                )
                if health_report['error_rate'] > 0.2:  # 超过 20% 错误率
                    logger.error(
                        "❌ AST 错误率过高，函数提取结果可能不可靠。"
                        "建议：1) 检查源文件语法；2) 尝试 gcc 预处理"
                    )
            
            # ========== 1. 递归提取所有函数定义 (包括 extern "C" 和 namespace 块) ==========
            all_func_nodes = self._find_all_function_definitions(tree.root_node)
            
            for func_node in all_func_nodes:
                sig = self._parse_function_definition(func_node, processed_bytes)
                if sig:
                    func_key = f"{sig.name}_{len(sig.parameters)}"
                    if func_key not in seen_funcs:
                        signatures.append(sig)
                        seen_funcs.add(func_key)
            
            # ========== 2. 查询 C++ 类成员函数 (解决 "No functions found" 问题) ==========
            cpp_methods = self._extract_cpp_method_signatures(tree.root_node, processed_bytes)
            for sig in cpp_methods:
                func_key = f"{sig.name}_{len(sig.parameters)}"
                if func_key not in seen_funcs:
                    signatures.append(sig)
                    seen_funcs.add(func_key)
                        
        except Exception as e:
            logger.warning(f"Tree-sitter 函数提取失败: {e}")
        
        return signatures
    
    def _check_ast_health(self, root_node) -> dict:
        """
        检查 AST 健康度（基于 EvoC2Rust 的 if_parse_error 方法）
        
        遍历 AST 统计 ERROR 节点数量，评估解析质量。
        
        Args:
            root_node: Tree-sitter AST 根节点
        
        Returns:
            dict: {
                'total_nodes': int,
                'error_nodes': int,
                'error_rate': float,
                'is_healthy': bool,
                'error_locations': list  # [(line, col), ...]
            }
        """
        total_nodes = 0
        error_nodes = 0
        error_locations = []
        
        def traverse(node):
            nonlocal total_nodes, error_nodes
            total_nodes += 1
            
            if node.type == 'ERROR':
                error_nodes += 1
                error_locations.append((
                    node.start_point[0] + 1,  # 行号从1开始
                    node.start_point[1]
                ))
            
            for child in node.children:
                traverse(child)
        
        traverse(root_node)
        
        error_rate = error_nodes / total_nodes if total_nodes > 0 else 0.0
        
        # 阈值：5% 错误率以下认为健康
        is_healthy = error_rate <= 0.05
        
        return {
            'total_nodes': total_nodes,
            'error_nodes': error_nodes,
            'error_rate': error_rate,
            'is_healthy': is_healthy,
            'error_locations': error_locations[:10]  # 只保留前10个
        }
    
    def _find_all_function_definitions(self, node) -> List:
        """
        递归查找所有 function_definition 节点
        
        解决的问题：
        1. extern "C" { } 块内的函数 (linkage_specification)
        2. namespace { } 块内的函数 (namespace_definition)
        3. #ifdef __cplusplus 块内的函数 (preproc_ifdef, preproc_if, preproc_else)
        4. 顶层函数
        
        Args:
            node: AST 节点
            
        Returns:
            所有 function_definition 节点列表
        """
        func_nodes = []
        
        if node.type == 'function_definition':
            func_nodes.append(node)
            return func_nodes  # 找到函数定义后不再深入
        
        # 需要递归处理的容器节点类型
        container_types = {
            'linkage_specification',    # extern "C" { }
            'namespace_definition',      # namespace X { }
            'declaration_list',          # { } 内的声明列表
            'translation_unit',          # 顶层
            'preproc_ifdef',             # #ifdef ... #endif
            'preproc_if',                # #if ... #endif
            'preproc_else',              # #else
            'preproc_elif',              # #elif
            'compound_statement',        # { } 块
        }
        
        # 递归处理所有子节点
        for child in node.children:
            if child.type == 'function_definition':
                func_nodes.append(child)
            else:
                # 对所有子节点都递归查找（确保不遗漏嵌套的函数）
                func_nodes.extend(self._find_all_function_definitions(child))
        
        return func_nodes
    
    def _extract_cpp_method_signatures(self, root_node, source_code: str) -> List[FunctionSignature]:
        """
        提取 C++ 类成员函数签名
        
        处理的情况：
        1. 类内定义的方法 (inline)
        2. 类外定义的方法 (ClassName::methodName)
        3. 嵌套命名空间内的方法 (Namespace::Class::method)
        
        修复：
        - 支持 type_identifier (类名) 作为 scope
        - 支持 namespace_identifier (命名空间) 作为 scope  
        - 支持嵌套的 qualified_identifier
        
        Returns:
            函数签名列表
        """
        methods = []
        seen_methods = set()  # 避免重复
        
        try:
            # 1. 查询类内方法定义
            # (class_specifier body: (field_declaration_list (function_definition)))
            class_query = CPP_LANGUAGE.query("""
                (class_specifier
                    name: (type_identifier) @class_name
                    body: (field_declaration_list) @class_body
                )
            """)
            
            class_captures = class_query.captures(root_node)
            
            class_name = None
            for node, cap_name in class_captures:
                if cap_name == 'class_name':
                    class_name = self._extract_text(source_code, node.start_byte, node.end_byte)
                elif cap_name == 'class_body' and class_name:
                    # 在类体内查找函数定义
                    class_methods = self._find_methods_in_class_body(node, source_code, class_name)
                    for sig in class_methods:
                        method_key = f"{sig.name}_{len(sig.parameters)}"
                        if method_key not in seen_methods:
                            methods.append(sig)
                            seen_methods.add(method_key)
                    class_name = None
            
            # 2. 查询类外方法定义 - 使用多种模式匹配
            # 模式 A: ClassName::methodName (type_identifier 作为 scope)
            cpp_methods_a = self._extract_methods_with_type_scope(root_node, source_code, seen_methods)
            methods.extend(cpp_methods_a)
            
            # 模式 B: Namespace::methodName (namespace_identifier 作为 scope)
            cpp_methods_b = self._extract_methods_with_namespace_scope(root_node, source_code, seen_methods)
            methods.extend(cpp_methods_b)
            
            # 模式 C: 递归遍历查找 qualified_identifier (处理复杂嵌套)
            cpp_methods_c = self._extract_methods_recursive(root_node, source_code, seen_methods)
            methods.extend(cpp_methods_c)
                    
        except Exception as e:
            logger.debug(f"C++ 方法提取失败: {e}")
        
        return methods
    
    def _extract_methods_with_type_scope(self, root_node, source_code: str, seen_methods: set) -> List[FunctionSignature]:
        """提取使用 type_identifier (类名) 作为 scope 的方法"""
        methods = []
        try:
            # 查询: ClassName::methodName 形式
            query = CPP_LANGUAGE.query("""
                (function_definition
                    declarator: (function_declarator
                        declarator: (qualified_identifier
                            scope: (type_identifier) @class_name
                            name: (identifier) @method_name
                        )
                    )
                ) @method_def
            """)
            
            captures = query.captures(root_node)
            
            class_name = None
            method_name = None
            for node, cap_name in captures:
                if cap_name == 'class_name':
                    class_name = self._extract_text(source_code, node.start_byte, node.end_byte)
                elif cap_name == 'method_name':
                    method_name = self._extract_text(source_code, node.start_byte, node.end_byte)
                elif cap_name == 'method_def' and class_name and method_name:
                    sig = self._parse_cpp_method_definition(node, source_code, class_name, method_name)
                    if sig:
                        method_key = f"{sig.name}_{len(sig.parameters)}"
                        if method_key not in seen_methods:
                            methods.append(sig)
                            seen_methods.add(method_key)
                    class_name = None
                    method_name = None
        except Exception as e:
            logger.debug(f"type_identifier 方法提取失败: {e}")
        
        return methods
    
    def _extract_methods_with_namespace_scope(self, root_node, source_code: str, seen_methods: set) -> List[FunctionSignature]:
        """提取使用 namespace_identifier 作为 scope 的方法"""
        methods = []
        try:
            query = CPP_LANGUAGE.query("""
                (function_definition
                    declarator: (function_declarator
                        declarator: (qualified_identifier
                            scope: (namespace_identifier) @ns_name
                            name: (identifier) @method_name
                        )
                    )
                ) @method_def
            """)
            
            captures = query.captures(root_node)
            
            ns_name = None
            method_name = None
            for node, cap_name in captures:
                if cap_name == 'ns_name':
                    ns_name = self._extract_text(source_code, node.start_byte, node.end_byte)
                elif cap_name == 'method_name':
                    method_name = self._extract_text(source_code, node.start_byte, node.end_byte)
                elif cap_name == 'method_def' and ns_name and method_name:
                    sig = self._parse_cpp_method_definition(node, source_code, ns_name, method_name)
                    if sig:
                        method_key = f"{sig.name}_{len(sig.parameters)}"
                        if method_key not in seen_methods:
                            methods.append(sig)
                            seen_methods.add(method_key)
                    ns_name = None
                    method_name = None
        except Exception as e:
            logger.debug(f"namespace_identifier 方法提取失败: {e}")
        
        return methods
    
    def _extract_methods_recursive(self, root_node, source_code: str, seen_methods: set) -> List[FunctionSignature]:
        """
        递归遍历 AST 查找所有带 qualified_identifier 的函数定义
        
        处理复杂嵌套情况如：
        - OHOS::NWeb::AccessTokenAdapterImpl::GetInstance()
        - namespace::class::method()
        """
        methods = []
        
        def extract_qualified_name(node) -> Tuple[Optional[str], Optional[str]]:
            """从 qualified_identifier 节点提取 scope 和 name"""
            scope_parts = []
            method_name = None
            
            for child in node.children:
                if child.type in ['namespace_identifier', 'type_identifier']:
                    scope_parts.append(self._extract_text(source_code, child.start_byte, child.end_byte))
                elif child.type == 'identifier':
                    method_name = self._extract_text(source_code, child.start_byte, child.end_byte)
                elif child.type == 'qualified_identifier':
                    # 递归处理嵌套的 qualified_identifier
                    inner_scope, inner_name = extract_qualified_name(child)
                    if inner_scope:
                        scope_parts.append(inner_scope)
                    if inner_name:
                        scope_parts.append(inner_name)
            
            scope = '_'.join(scope_parts) if scope_parts else None
            return scope, method_name
        
        def visit(node):
            if node.type == 'function_definition':
                # 查找 function_declarator
                for child in node.children:
                    if child.type == 'function_declarator':
                        for sub in child.children:
                            if sub.type == 'qualified_identifier':
                                scope, method_name = extract_qualified_name(sub)
                                if scope and method_name:
                                    sig = self._parse_cpp_method_definition(node, source_code, scope, method_name)
                                    if sig:
                                        method_key = f"{sig.name}_{len(sig.parameters)}"
                                        if method_key not in seen_methods:
                                            methods.append(sig)
                                            seen_methods.add(method_key)
                        break
            
            for child in node.children:
                visit(child)
        
        visit(root_node)
        return methods
    
    def _find_methods_in_class_body(
        self, 
        class_body_node, 
        source_code: str, 
        class_name: str
    ) -> List[FunctionSignature]:
        """在类体内查找方法定义"""
        methods = []
        
        for child in class_body_node.children:
            if child.type == 'function_definition':
                sig = self._parse_function_definition(child, source_code)
                if sig:
                    # 标记为类方法
                    sig.name = f"{class_name}_{sig.name}"  # 添加类名前缀
                    sig.is_callback = True  # 成员方法可能作为回调
                    methods.append(sig)
            elif child.type == 'declaration':
                # 可能是成员函数声明（只有声明没有定义）
                pass  # 我们主要关注定义
        
        return methods
    
    def _parse_cpp_method_definition(
        self, 
        func_node, 
        source_code,  # 支持 bytes 或 str
        class_name: str,
        method_name: str
    ) -> Optional[FunctionSignature]:
        """解析 C++ 类外方法定义（支持 bytes 和 str 输入）"""
        try:
            func_text = self._extract_text(source_code, func_node.start_byte, func_node.end_byte)
            
            # 提取函数签名（不含函数体）
            brace_pos = func_text.find('{')
            if brace_pos == -1:
                return None
            
            c_signature = func_text[:brace_pos].strip()
            
            # 提取返回类型
            return_type = self._extract_return_type(func_node, source_code)
            
            # 提取参数
            parameters = self._extract_parameters(func_node, source_code)
            
            # 参数名去重
            if TYPE_UTILS_AVAILABLE:
                parameters = sanitize_parameter_names(parameters)
            
            # 使用 ClassName_methodName 作为函数名
            full_name = f"{class_name}_{method_name}"
            
            return FunctionSignature(
                name=full_name,
                c_signature=c_signature,
                rust_signature="",
                return_type=return_type,
                parameters=parameters,
                is_static=False,
                is_callback=True  # C++ 方法通常需要特殊处理
            )
            
        except Exception as e:
            logger.debug(f"解析 C++ 方法失败: {e}")
            return None
    
    def _extract_text(self, source, start_byte: int, end_byte: int) -> str:
        """
        从 source 中提取文本，正确处理 bytes 和 str
        
        Tree-sitter 返回字节偏移，但 Python str 的索引是字符偏移。
        当源码包含多字节字符（如中文注释）时，会导致偏移不匹配。
        
        Args:
            source: bytes 或 str
            start_byte: 开始字节偏移
            end_byte: 结束字节偏移
            
        Returns:
            提取的文本字符串
        """
        if isinstance(source, bytes):
            return source[start_byte:end_byte].decode('utf-8', errors='ignore')
        else:
            # 如果是 str，需要转换为 bytes 再提取
            source_bytes = source.encode('utf-8')
            return source_bytes[start_byte:end_byte].decode('utf-8', errors='ignore')
    
    def _parse_function_definition(self, func_node, source_code) -> Optional[FunctionSignature]:
        """
        解析函数定义节点（支持 bytes 和 str 输入）
        
        增强功能（基于 EvoC2Rust 的递归声明器查找）：
        - 支持指针返回类型: Type* func()
        - 支持引用返回类型: Type& func() (C++)
        - 支持多级指针: Type** func()
        - 支持复杂嵌套: const Type* const * func()
        """
        try:
            func_text = self._extract_text(source_code, func_node.start_byte, func_node.end_byte)
            
            # 提取函数签名（不含函数体）
            brace_pos = func_text.find('{')
            if brace_pos == -1:
                return None
            
            c_signature = func_text[:brace_pos].strip()
            
            # 检查是否是 static 函数
            is_static = c_signature.strip().startswith('static')
            
            # ========== 增强：递归查找函数声明器 ==========
            # 解决 Type& func() 和 Type* func() 等复杂情况
            func_name = None
            declarator_wrappers = []  # 记录声明器包装层（指针/引用）
            
            # 使用递归查找函数声明器
            func_name, declarator_wrappers = self._find_function_declarator_recursive(func_node, source_code)
            
            if not func_name:
                # 回退到原始方法
                for child in func_node.children:
                    if child.type == 'function_declarator':
                        for sub in child.children:
                            if sub.type == 'identifier':
                                func_name = self._extract_text(source_code, sub.start_byte, sub.end_byte)
                                break
            
            if not func_name:
                return None
            
            # === 优化点 1: 过滤非法函数名 (解决 pathselect/pack/installer 崩溃问题) ===
            if TYPE_UTILS_AVAILABLE:
                if not is_valid_c_identifier(func_name):
                    logger.debug(f"跳过非法函数名: {func_name}")
                    return None
            
            # 提取返回类型
            return_type = self._extract_return_type(func_node, source_code)
            
            # === 优化点 2: 收集返回值类型 (解决 E0412) ===
            if TYPE_UTILS_AVAILABLE:
                ret_base = extract_base_type(return_type)
                # 过滤掉基础类型
                if ret_base and ret_base not in ['int', 'char', 'void', 'float', 'double', 'long', 'short', '_Bool', 'bool', 'unsigned', 'signed', 'size_t', 'ssize_t', 'ptrdiff_t', 'intptr_t', 'uintptr_t', 'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t', 'int8_t', 'int16_t', 'int32_t', 'int64_t']:
                    self.collected_custom_types.add(ret_base)
            
            # 提取参数
            parameters = self._extract_parameters(func_node, source_code)
            
            # === 优化点 3: 收集参数类型 (解决 E0412) ===
            if TYPE_UTILS_AVAILABLE:
                for _, p_type in parameters:
                    p_base = extract_base_type(p_type)
                    # 过滤掉基础类型
                    if p_base and p_base not in ['int', 'char', 'void', 'float', 'double', 'long', 'short', '_Bool', 'bool', 'unsigned', 'signed', 'size_t', 'ssize_t', 'ptrdiff_t', 'intptr_t', 'uintptr_t', 'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t', 'int8_t', 'int16_t', 'int32_t', 'int64_t']:
                        self.collected_custom_types.add(p_base)
            
            # === 优化点 4: 参数名去重 (解决 ipc_auth E0415) ===
            if TYPE_UTILS_AVAILABLE:
                parameters = sanitize_parameter_names(parameters)
            
            return FunctionSignature(
                name=func_name,
                c_signature=c_signature,
                rust_signature="",  # 稍后由 LLM 生成
                return_type=return_type,
                parameters=parameters,
                is_static=is_static
            )
            
        except Exception as e:
            logger.debug(f"解析函数定义失败: {e}")
            return None
    
    def _find_function_declarator_recursive(
        self, 
        node, 
        source_code
    ) -> Tuple[Optional[str], List[str]]:
        """
        递归查找函数声明器并提取函数名
        
        参考 EvoC2Rust 的 has_function_declarator 方法
        
        解决的问题：
        - Type& func() -> reference_declarator > function_declarator
        - Type* func() -> pointer_declarator > function_declarator
        - Type** func() -> pointer_declarator > pointer_declarator > function_declarator
        - const Type* const* func() -> 复杂嵌套
        
        Args:
            node: 要搜索的 AST 节点
            source_code: 源代码（bytes 或 str）
        
        Returns:
            (func_name, declarator_wrappers)
            - func_name: 函数名
            - declarator_wrappers: 声明器包装层列表 ["pointer", "reference", ...]
        """
        wrappers = []
        
        def search(n):
            """递归搜索"""
            nonlocal wrappers
            
            if n.type == 'function_declarator':
                # 找到函数声明器，提取函数名
                for child in n.children:
                    if child.type == 'identifier':
                        return self._extract_text(source_code, child.start_byte, child.end_byte)
                    elif child.type == 'field_identifier':
                        return self._extract_text(source_code, child.start_byte, child.end_byte)
                    elif child.type == 'qualified_identifier':
                        return self._extract_text(source_code, child.start_byte, child.end_byte)
                    elif child.type == 'parenthesized_declarator':
                        # 函数指针情况
                        result = search(child)
                        if result:
                            return result
                return None
            
            elif n.type == 'pointer_declarator':
                wrappers.append('pointer')
                # 继续向下搜索
                for child in n.children:
                    if child.type in ('function_declarator', 'pointer_declarator', 
                                     'reference_declarator', 'identifier'):
                        result = search(child)
                        if result:
                            return result
            
            elif n.type == 'reference_declarator':
                wrappers.append('reference')
                # 继续向下搜索
                for child in n.children:
                    if child.type in ('function_declarator', 'pointer_declarator', 
                                     'reference_declarator', 'identifier'):
                        result = search(child)
                        if result:
                            return result
            
            elif n.type == 'identifier':
                # 直接找到标识符
                return self._extract_text(source_code, n.start_byte, n.end_byte)
            
            # 递归搜索所有子节点
            for child in n.children:
                result = search(child)
                if result:
                    return result
            
            return None
        
        func_name = search(node)
        return func_name, wrappers
    
    def _extract_return_type(self, func_node, source_code) -> str:
        """提取返回类型（支持 bytes 和 str 输入）

        增强功能（2025-12-23）：
        - 支持返回指针类型，如 int*, char**, void*
        - 支持返回引用类型，如 int&
        - 正确处理声明器中的指针/引用信息
        """
        # 尝试提取完整的返回类型（包括 static, const 等修饰符）
        return_type_parts = []
        pointer_count = 0
        reference_count = 0

        # 查找类型说明符节点
        for child in func_node.children:
            if child.type in ['type_identifier', 'primitive_type', 'sized_type_specifier']:
                return_type_parts.append(self._extract_text(source_code, child.start_byte, child.end_byte))
            elif child.type == 'storage_class_specifier':
                # static 等存储类说明符
                text = self._extract_text(source_code, child.start_byte, child.end_byte)
                if text == 'static':
                    # static 不是返回类型的一部分，跳过
                    continue
            elif child.type == 'type_qualifier':
                # const, volatile 等类型限定符
                return_type_parts.append(self._extract_text(source_code, child.start_byte, child.end_byte))
            elif child.type == 'struct_specifier':
                # struct xxx 类型
                struct_text = self._extract_text(source_code, child.start_byte, child.end_byte)
                return_type_parts.append(struct_text)
            elif child.type == 'enum_specifier':
                # enum xxx 类型
                enum_text = self._extract_text(source_code, child.start_byte, child.end_byte)
                return_type_parts.append(enum_text)
            elif child.type == 'pointer_declarator':
                # ★ 增强：计算指针层数
                pointer_count += self._count_pointer_levels(child)
            elif child.type == 'reference_declarator':
                # ★ 增强：引用类型
                reference_count += 1

        # 构建返回类型字符串
        if return_type_parts:
            base_type = ' '.join(return_type_parts)
            # 添加指针标记
            if pointer_count > 0:
                base_type += '*' * pointer_count
            elif reference_count > 0:
                base_type += '&' * reference_count
            return base_type
        return 'void'

    def _count_pointer_levels(self, declarator_node) -> int:
        """计算声明器中的指针层数

        用于处理多级指针返回类型，如 char** 或 int***
        """
        count = 0
        node = declarator_node

        while node is not None:
            if node.type == 'pointer_declarator':
                count += 1
                # 继续查找嵌套的 pointer_declarator
                for child in node.children:
                    if child.type == 'pointer_declarator':
                        node = child
                        break
                else:
                    break
            else:
                break

        return count
    
    def _extract_parameters(self, func_node, source_code) -> List[Tuple[str, str]]:
        """提取参数列表（支持 bytes 和 str 输入）

        增强功能（2025-12-23）：
        - 递归处理嵌套声明器（多级指针、数组、函数指针）
        - 解决参数名提取失败导致的重复 'arg' 问题

        修复（2026-01-08）：
        - 递归查找 function_declarator，解决返回指针类型函数参数丢失问题
        - 对于 char * func(char *arg)，function_declarator 嵌套在 pointer_declarator 中
        """
        params = []

        def extract_identifier_recursive(node) -> Optional[str]:
            """递归提取标识符名称，处理嵌套声明器"""
            if node is None:
                return None

            # 直接是标识符
            if node.type == 'identifier':
                return self._extract_text(source_code, node.start_byte, node.end_byte)

            # 递归处理各种声明器类型
            if node.type in ['pointer_declarator', 'reference_declarator',
                             'array_declarator', 'parenthesized_declarator']:
                for child in node.children:
                    result = extract_identifier_recursive(child)
                    if result:
                        return result

            # 函数指针: void (*callback)(int)
            # 结构: function_declarator > parenthesized_declarator > pointer_declarator > identifier
            if node.type == 'function_declarator':
                for child in node.children:
                    if child.type in ['parenthesized_declarator', 'pointer_declarator']:
                        result = extract_identifier_recursive(child)
                        if result:
                            return result
                    elif child.type == 'identifier':
                        return self._extract_text(source_code, child.start_byte, child.end_byte)

            return None

        def find_function_declarator(node):
            """递归查找 function_declarator 节点

            解决返回指针类型函数的参数提取问题：
            - _Bool func(char *arg): function_definition → function_declarator (直接子节点)
            - char * func(char *arg): function_definition → pointer_declarator → function_declarator (嵌套)
            """
            if node is None:
                return None
            if node.type == 'function_declarator':
                return node
            # 在 pointer_declarator / reference_declarator / parenthesized_declarator 中递归查找
            if node.type in ['pointer_declarator', 'reference_declarator', 'parenthesized_declarator']:
                for child in node.children:
                    result = find_function_declarator(child)
                    if result:
                        return result
            return None

        # 查找 function_declarator（支持直接子节点和嵌套在 pointer_declarator 中的情况）
        func_declarator = None
        for child in func_node.children:
            if child.type == 'function_declarator':
                func_declarator = child
                break
            elif child.type in ['pointer_declarator', 'reference_declarator', 'parenthesized_declarator']:
                # 返回指针/引用类型的函数，function_declarator 嵌套在内部
                func_declarator = find_function_declarator(child)
                if func_declarator:
                    break

        if func_declarator:
            for sub in func_declarator.children:
                if sub.type == 'parameter_list':
                    for param in sub.children:
                        if param.type == 'parameter_declaration':
                            # 提取参数类型和名称
                            param_type_parts = []
                            param_name = None
                            is_pointer = False
                            is_array = False

                            for param_child in param.children:
                                if param_child.type in ['type_identifier', 'primitive_type', 'sized_type_specifier']:
                                    param_type_parts.append(self._extract_text(source_code, param_child.start_byte, param_child.end_byte))
                                elif param_child.type == 'type_qualifier':
                                    param_type_parts.append(self._extract_text(source_code, param_child.start_byte, param_child.end_byte))
                                elif param_child.type == 'struct_specifier':
                                    # 处理 struct xxx 类型
                                    struct_text = self._extract_text(source_code, param_child.start_byte, param_child.end_byte)
                                    param_type_parts.append(struct_text)
                                elif param_child.type == 'enum_specifier':
                                    # 处理 enum xxx 类型
                                    enum_text = self._extract_text(source_code, param_child.start_byte, param_child.end_byte)
                                    param_type_parts.append(enum_text)
                                elif param_child.type == 'union_specifier':
                                    # 处理 union xxx 类型
                                    union_text = self._extract_text(source_code, param_child.start_byte, param_child.end_byte)
                                    param_type_parts.append(union_text)
                                elif param_child.type == 'identifier':
                                    param_name = self._extract_text(source_code, param_child.start_byte, param_child.end_byte)
                                elif param_child.type in ['pointer_declarator', 'reference_declarator']:
                                    is_pointer = True
                                    # 使用递归提取
                                    param_name = extract_identifier_recursive(param_child)
                                elif param_child.type == 'array_declarator':
                                    # 数组参数: int arr[10]
                                    is_array = True
                                    param_name = extract_identifier_recursive(param_child)
                                elif param_child.type == 'function_declarator':
                                    # 函数指针参数: void (*callback)(int)
                                    is_pointer = True
                                    param_name = extract_identifier_recursive(param_child)
                                elif param_child.type == 'abstract_declarator':
                                    # 匿名参数（只有类型没有名字），如: void func(int *, char[])
                                    is_pointer = '*' in self._extract_text(source_code, param_child.start_byte, param_child.end_byte)

                            # 构建参数类型字符串
                            param_type = ' '.join(param_type_parts)
                            if is_pointer:
                                param_type += '*'
                            elif is_array:
                                param_type += '[]'

                            # 如果没有找到参数名，使用默认值
                            if not param_name:
                                param_name = 'arg'

                            params.append((param_name, param_type))

        return params
    
    def _collect_types_from_signature(self, sig: FunctionSignature):
        """
        从函数签名中收集自定义类型
        
        这是 finalize_types_rs() 能正确工作的关键。
        TypeMapper 本身不会收集类型，所以需要在调用 TypeMapper 之前手动收集。
        """
        if not TYPE_UTILS_AVAILABLE:
            return
        
        # 基础类型列表（不需要收集）
        primitives = {
            'int', 'char', 'void', 'float', 'double', 'long', 'short', '_Bool', 'bool',
            'unsigned', 'signed', 'size_t', 'ssize_t', 'ptrdiff_t', 'intptr_t', 'uintptr_t',
            'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t',
            'int8_t', 'int16_t', 'int32_t', 'int64_t',
            'u8', 'u16', 'u32', 'u64', 's8', 's16', 's32', 's64',
            '__u8', '__u16', '__u32', '__u64', '__s8', '__s16', '__s32', '__s64',
        }
        
        # 收集返回类型
        if sig.return_type:
            ret_base = extract_base_type(sig.return_type)
            if ret_base and ret_base not in primitives:
                self.collected_custom_types.add(ret_base)
        
        # 收集参数类型
        for _, param_type in sig.parameters:
            if param_type:
                p_base = extract_base_type(param_type)
                if p_base and p_base not in primitives:
                    self.collected_custom_types.add(p_base)
    
    def generate_function_stubs(
        self, 
        signatures: List[FunctionSignature], 
        llm_translate_fn=None,
        use_type_mapper: bool = True,
        use_llm_type_mapper: bool = False
    ) -> Dict[str, str]:
        """
        生成函数桩代码
        
        Args:
            signatures: 函数签名列表
            llm_translate_fn: 可选的 LLM 翻译函数，签名为 (c_signature: str) -> str
            use_type_mapper: 是否使用 TypeMapper（确定性规则引擎），默认 True
            use_llm_type_mapper: 是否使用 LLMTypeMapper（TypeMapper + LLM 验证/修正），默认 False
        
        Returns:
            Dict[func_name, rust_stub_code]
        """
        stubs = {}

        def _use_existing_rust_signature(sig: FunctionSignature) -> bool:
            """
            If `sig.rust_signature` is already populated (e.g., from bindgen allowlist on TU `.i`),
            we should treat it as the source of truth and avoid re-generating it via TypeMapper/LLM.
            """
            try:
                rs = (sig.rust_signature or "").strip()
            except Exception:
                rs = ""
            if not rs:
                return False
            # `bindgen` decls may end with `;` (extern blocks). We need a signature line for stub generation.
            if rs.endswith(";"):
                rs = rs[:-1].rstrip()
            # Basic sanity: must look like a Rust fn signature.
            if " fn " not in f" {rs} ":
                return False
            sig.rust_signature = rs
            try:
                stubs[sig.name] = self._generate_stub(sig)
            except Exception:
                stubs[sig.name] = f"{rs} {{\n    unimplemented!()\n}}"
            return True

        # LLM 对“规则签名”进行复核/裁决（默认开启，可通过环境变量关闭）。
        # - 关闭：C2R_LLM_REFINE_SIGNATURES=0
        # - types.rs 会做切片（避免把整份 types.rs 塞进 prompt）
        sig_refiner = None
        if self._env_flag("C2R_TRUTH_MODE", "0"):
            # Truth-mode: keep signatures fully deterministic (no LLM refiner).
            sig_refiner = None
        else:
            try:
                from llm_signature_refiner import build_refiner_from_env  # type: ignore

                sig_refiner = build_refiner_from_env(types_rs_path=(self.output_dir / "src" / "types.rs"))
                if sig_refiner is not None:
                    logger.info("已启用签名 LLM 复核（默认开启，可用 C2R_LLM_REFINE_SIGNATURES=0 关闭）")
            except Exception as e:
                logger.debug(f"初始化签名 LLM 复核失败，跳过: {e}")
        
        # 如果启用 LLMTypeMapper，创建实例
        # 注意：LLMTypeMapper 会对每个参数/返回类型分别调用一次 LLM，调用次数≈(参数数+1)。
        llm_type_mapper_instance = None
        if use_llm_type_mapper and LLM_TYPE_MAPPER_AVAILABLE:
            try:
                types_rs_path = self.output_dir / "src" / "types.rs"
                llm_type_mapper_instance = create_llm_type_mapper(
                    types_rs_path=types_rs_path if types_rs_path.exists() else None,
                    enable_llm=True
                )
                logger.info("LLMTypeMapper 已初始化，将使用 LLM 辅助验证类型映射")
            except Exception as e:
                logger.warning(f"LLMTypeMapper 初始化失败: {e}，回退到 TypeMapper")
        
        # 优先使用 TypeMapper（确定性规则引擎）
        if use_type_mapper and TYPE_MAPPER_AVAILABLE:
            for sig in signatures:
                try:
                    # If signature was already produced by a deterministic tool (e.g., bindgen),
                    # keep it intact. This avoids accidental parameter-name/type-name collisions
                    # and keeps skeleton signatures consistent with TU truth.
                    if _use_existing_rust_signature(sig):
                        continue

                    # ========== 关键修复：收集自定义类型（TypeMapper 本身不收集）==========
                    self._collect_types_from_signature(sig)
                    
                    # 如果启用 LLMTypeMapper，使用 LLM 辅助映射参数和返回类型
                    if llm_type_mapper_instance:
                        try:
                            # 用 LLMTypeMapper 处理每个参数类型
                            mapped_params = []
                            for param_name, param_type in sig.parameters:
                                # 解析指针/const/数组信息（从 C 类型字符串中提取）
                                c_type = (param_type or "").strip()
                                is_ptr = "*" in c_type
                                is_const = "const" in c_type
                                is_array = "[" in c_type and "]" in c_type

                                # 参数名需要做 Rust 关键字规避（与 TypeMapper 保持一致）
                                clean_name = param_name
                                if TYPE_MAPPER_AVAILABLE:
                                    clean_name = TypeMapper.sanitize_identifier(param_name)

                                result = llm_type_mapper_instance.map_type(
                                    c_type,
                                    is_pointer=is_ptr,
                                    is_const=is_const,
                                    is_array=is_array,
                                    context_location=f"param:{param_name}",
                                )
                                mapped_params.append((clean_name, result.final_rust_type))
                            
                            # 处理返回类型
                            ret_c_type = (sig.return_type or "").strip()
                            ret_is_ptr = "*" in ret_c_type
                            ret_is_const = "const" in ret_c_type
                            ret_result = llm_type_mapper_instance.map_type(
                                ret_c_type,
                                is_pointer=ret_is_ptr,
                                is_const=ret_is_const,
                                context_location="return",
                            )
                            
                            # 生成函数签名
                            func_mod = "fn" if sig.is_static else "pub extern \"C\" fn"
                            func_name = sig.name
                            if TYPE_MAPPER_AVAILABLE:
                                func_name = TypeMapper.sanitize_identifier(sig.name)
                            params_str = ", ".join([f"{name}: {rtype}" for name, rtype in mapped_params])
                            ret_str = f" -> {ret_result.final_rust_type}" if ret_result.final_rust_type != "()" else ""
                            
                            sig.rust_signature = f"{func_mod} {func_name}({params_str}){ret_str}"
                            # 可选：对 LLMTypeMapper 的结果做一次签名级复核（仍然是“失败回退到原结果”）
                            if sig_refiner is not None:
                                r = sig_refiner.refine(
                                    func_name=func_name,
                                    c_signature=sig.c_signature,
                                    candidate_rust_signature=sig.rust_signature,
                                    is_static=sig.is_static,
                                )
                                if r.ok and r.refined_signature:
                                    sig.rust_signature = r.refined_signature
                            stub = self._generate_stub(sig)
                            stubs[sig.name] = stub
                            continue  # 成功，跳过 TypeMapper 回退
                        except Exception as llm_e:
                            logger.debug(f"LLMTypeMapper 失败: {sig.name}, {llm_e}，回退到 TypeMapper")
                    
                    # 使用 TypeMapper 生成函数桩（默认或回退）
                    func_mod, params_str, ret_str = TypeMapper.process_function_signature(
                        sig.return_type, sig.parameters, sig.is_static
                    )
                    # 确保函数名与 stub 一致（TypeMapper 会规避 Rust 关键字）
                    func_name = TypeMapper.sanitize_identifier(sig.name)
                    sig.rust_signature = f"{func_mod} {func_name}({params_str}){ret_str}"

                    # 可选：用 LLM 复核规则签名（只在未启用 LLMTypeMapper 时）
                    if sig_refiner is not None:
                        r = sig_refiner.refine(
                            func_name=func_name,
                            c_signature=sig.c_signature,
                            candidate_rust_signature=sig.rust_signature,
                            is_static=sig.is_static,
                        )
                        if r.ok and r.refined_signature:
                            sig.rust_signature = r.refined_signature

                    stub = self._generate_stub(sig)
                    stubs[sig.name] = stub
                except Exception as e:
                    logger.warning(f"TypeMapper 生成失败: {sig.name}, {e}，使用回退方法")
                    # 回退到旧方法
                    if llm_translate_fn:
                        try:
                            rust_sig = llm_translate_fn(sig.c_signature)
                            sig.rust_signature = rust_sig
                        except Exception as e2:
                            logger.warning(f"LLM 翻译失败: {sig.name}, {e2}")
                            sig.rust_signature = self._fallback_signature_translation(sig)
                    else:
                        sig.rust_signature = self._fallback_signature_translation(sig)
                    stub = self._generate_stub(sig)
                    stubs[sig.name] = stub
        else:
            # 使用 LLM 或回退方法
            for sig in signatures:
                if _use_existing_rust_signature(sig):
                    continue
                if llm_translate_fn:
                    # 使用 LLM 翻译签名
                    try:
                        rust_sig = llm_translate_fn(sig.c_signature)
                        sig.rust_signature = rust_sig
                    except Exception as e:
                        logger.warning(f"LLM 翻译失败: {sig.name}, {e}")
                        sig.rust_signature = self._fallback_signature_translation(sig)
                else:
                    # 使用简单规则翻译
                    sig.rust_signature = self._fallback_signature_translation(sig)
                
                # 可选：对回退规则签名做一次复核（保持与 TypeMapper 分支一致的行为）
                if sig_refiner is not None:
                    # 尝试从签名中提取函数名（回退签名可能已做 sanitize）
                    fn_m = re.search(r"fn\s+([A-Za-z_][A-Za-z0-9_]*)", sig.rust_signature or "")
                    func_name = fn_m.group(1) if fn_m else sig.name
                    r = sig_refiner.refine(
                        func_name=func_name,
                        c_signature=sig.c_signature,
                        candidate_rust_signature=sig.rust_signature,
                        is_static=sig.is_static,
                    )
                    if r.ok and r.refined_signature:
                        sig.rust_signature = r.refined_signature

                # 生成桩代码
                stub = self._generate_stub(sig)
                stubs[sig.name] = stub
        
        return stubs
    
    def _fallback_signature_translation(self, sig: FunctionSignature) -> str:
        """
        回退的签名翻译（基于规则）
        
        基于 EvoC2Rust 的类型映射规则，不依赖 LLM
        """
        rust_params = []
        for param_name, param_type in sig.parameters:
            # 清理参数名（移除可能的 * 或 & 前缀）
            clean_param_name = param_name.strip('*&').strip()
            if not clean_param_name or clean_param_name in ['void']:
                continue
            
            # 检查参数类型是否为 void（C 中的 void 参数表示无参数）
            clean_param_type = param_type.replace('const', '').replace('volatile', '').strip()
            if clean_param_type == 'void' and '*' not in param_type:
                # void (非指针) 表示无参数，跳过
                continue
                
            # 转换参数类型
            is_pointer = '*' in param_type or '*' in param_name
            is_const = 'const' in param_type
            rust_type = self._c_type_to_rust(param_type.replace('const', '').strip(), is_pointer, '[' in param_type)
            
            # 处理 const 指针 -> *const T
            if is_pointer and is_const:
                if rust_type.startswith('*mut '):
                    rust_type = '*const ' + rust_type[5:]
            
            # 确保参数名是有效的 Rust 标识符
            # 注意：self, Self, super, crate 是特殊关键字，不能用 r# 转义，需要重命名
            if clean_param_name in ['self', 'Self']:
                clean_param_name = f"{clean_param_name}_"  # self -> self_, Self -> Self_
            elif clean_param_name in ['super', 'crate']:
                clean_param_name = f"_{clean_param_name}"  # super -> _super, crate -> _crate
            elif clean_param_name in ['type', 'match', 'fn', 'mod', 'use', 'impl', 'trait', 'struct', 'enum', 'in', 'ref', 'mut', 'const', 'static', 'priv', 'pub', 'let', 'loop', 'while', 'for', 'if', 'else', 'return', 'break', 'continue', 'as', 'where', 'async', 'await', 'dyn', 'move', 'box', 'extern', 'unsafe']:
                clean_param_name = f"r#{clean_param_name}"
            
            rust_params.append(f"{clean_param_name}: {rust_type}")
        
        # 转换返回类型
        rust_return = self._c_type_to_rust(sig.return_type, False, False)
        if rust_return == 'std::ffi::c_void' or sig.return_type.strip() == 'void':
            return_clause = ""
        else:
            return_clause = f" -> {rust_return}"
        
        params_str = ", ".join(rust_params)
        
        # 生成函数签名
        # static 函数: 普通 Rust 函数
        # 非 static 函数: pub extern "C" fn（可能被外部调用）
        if sig.is_static:
            return f"fn {sig.name}({params_str}){return_clause}"
        else:
            # 非 static C 函数通常可能被外部调用，使用 extern "C"
            return f"pub extern \"C\" fn {sig.name}({params_str}){return_clause}"
    
    def _generate_stub(self, sig: FunctionSignature) -> str:
        """
        生成函数桩代码
        
        重要：只生成函数签名 + unimplemented!()，不提取或翻译函数体
        """
        rust_sig = sig.rust_signature.strip()
        
        # 如果签名已经包含函数体，提取签名部分（忽略函数体）
        # 这是为了确保不会保留任何函数体内容
        if '{' in rust_sig:
            # 提取签名部分（到第一个 { 之前）
            brace_pos = rust_sig.find('{')
            sig_part = rust_sig[:brace_pos].strip()
            # 确保签名部分不以 { 结尾
            sig_part = sig_part.rstrip('{').strip()
        else:
            sig_part = rust_sig
        
        # 始终生成只包含 unimplemented!() 的桩代码
        # 不保留任何函数体内容
        return f"""{sig_part} {{
    unimplemented!()
}}"""
    
    # =========================================================================
    # 集成方法
    # =========================================================================
    
    def build_skeleton(
        self,
        source_files: List[Path],
        header_files: List[Path],
        llm_translate_fn=None,
        use_hybrid_build: bool = True,
        target_files: List[Path] = None,
        dependency_files: List[Path] = None
    ) -> Path:
        """
        构建完整的 Rust 骨架
        
        支持两种模式：
        1. 全量翻译模式：所有源文件都翻译成 Rust
        2. 混合构建模式：只翻译目标文件，其他文件保留为 C 通过 FFI 链接
        
        Args:
            source_files: 源文件列表
            header_files: 头文件列表
            llm_translate_fn: 可选的 LLM 翻译函数
            use_hybrid_build: 是否使用混合构建模式（推荐 True）
            target_files: 混合模式下要翻译的文件（默认全部）
            dependency_files: 混合模式下的依赖文件（保留为 C）
        
        Returns:
            骨架项目路径
        """
        truth_mode = self._env_flag("C2R_TRUTH_MODE", "0")

        # 如果未指定 target_files，则全部作为目标
        if target_files is None:
            target_files = source_files
        
        # 如果未指定 dependency_files，自动推断
        if dependency_files is None and use_hybrid_build:
            target_names = {f.name for f in target_files}
            dependency_files = [f for f in source_files if f.name not in target_names]
        
        logger.info(f"开始构建骨架: {len(source_files)} 个源文件, {len(header_files)} 个头文件")
        if use_hybrid_build and dependency_files:
            logger.info(f"混合构建模式: {len(target_files)} 个目标文件, {len(dependency_files)} 个 C 依赖")
        
        # 阶段 A: 类型骨架
        logger.info("阶段 A: 生成类型骨架 (bindgen)")
        # 增强：传递源文件以分析外部依赖
        self.generate_type_skeleton(header_files, source_files=source_files)
        
        # 阶段 B: 变量骨架
        logger.info("阶段 B: 提取全局变量 (tree-sitter)")
        all_variables = []
        for src_file in source_files:
            try:
                # Truth-first: reuse stage1 pinned TU `.i` when available; it has the exact flags/macros/include order.
                preprocessed = None
                try:
                    safe_name = self._get_safe_module_name(src_file)
                    rec = (getattr(self, "_tu_context_files", {}) or {}).get(safe_name)
                    pre_path = (rec or {}).get("preprocessed_file") if isinstance(rec, dict) else None
                    if pre_path:
                        p = Path(str(pre_path)).expanduser()
                        if p.exists():
                            preprocessed = p.read_text(encoding="utf-8", errors="ignore")
                except Exception:
                    preprocessed = None

                if not preprocessed:
                    preprocessed = self.preprocess_source(src_file)
                # 提取变量
                variables = self.extract_variables_with_treesitter(preprocessed, src_file.stem)
                all_variables.extend(variables)
            except Exception as e:
                logger.warning(f"处理 {src_file.name} 失败: {e}")
        
        self.generate_globals_rs(all_variables)

        # Centralized compat layer (placeholders + accessor shims)
        # Needed because module stubs import `crate::compat::*;`.
        self._generate_compat_rs()
        
        # 阶段 C: 函数骨架
        logger.info("阶段 C: 生成函数骨架")
        for src_file in source_files:
            try:
                # Truth-first: reuse stage1 pinned TU `.i` when available (与阶段 B 保持一致)
                # 修复：urlparser 等项目的 .c 文件只有 #include "xxx.h"，函数定义在头文件中
                # 必须使用预处理后的代码才能提取完整的函数签名
                source_code = None
                try:
                    safe_name = self._get_safe_module_name(src_file)
                    rec = (getattr(self, "_tu_context_files", {}) or {}).get(safe_name)
                    pre_path = (rec or {}).get("preprocessed_file") if isinstance(rec, dict) else None
                    if pre_path:
                        p = Path(str(pre_path)).expanduser()
                        if p.exists():
                            source_code = p.read_text(encoding="utf-8", errors="ignore")
                            logger.debug(f"阶段 C: 使用预处理文件 {p} 提取函数签名")
                except Exception:
                    source_code = None

                if not source_code:
                    source_code = self.preprocess_source(src_file)

                signatures = self.extract_function_signatures(source_code)
                stubs = self.generate_function_stubs(signatures, llm_translate_fn)

                # 生成模块文件（使用安全的模块名，避免文件名冲突）
                self._generate_module_file(src_file, stubs)

            except Exception as e:
                logger.warning(f"生成 {src_file.name} 骨架失败: {e}")
        
        # 生成 main.rs 和 Cargo.toml
        self._generate_main_rs()
        self._generate_cargo_toml(use_hybrid_build=use_hybrid_build)
        
        # 混合构建模式：设置 native/ 目录和 build.rs
        if use_hybrid_build and HYBRID_BUILD_AVAILABLE:
            logger.info("设置混合构建环境 (C/Rust 链接)")
            self._setup_hybrid_build(
                target_files=target_files,
                dependency_files=dependency_files or [],
                header_files=header_files
            )
        
        # 阶段 D: 类型修复与验证
        logger.info("阶段 D: 类型修复与验证")
        
        # D.1: 追加缺失的类型定义
        self.finalize_types_rs()

        # D.1.2: Tier-0 primitive typedef fixes (deterministic)
        if truth_mode:
            logger.info("Truth-mode: 跳过 primitive typedef fixes（保持 types.rs 真值层不做派生修补）")
        else:
            try:
                self.apply_primitive_typedef_fixes()
            except Exception as e:
                logger.debug(f"Tier-0 primitive typedef fixes failed: {e}")
        
        # D.1.5: 自动检测并生成缺失的外部符号占位
        types_file = self.output_dir / "src" / "types.rs"
        if truth_mode:
            logger.info("Truth-mode: 跳过自动缺失符号占位（不生成宏/extern/const 的占位定义）")
        else:
            try:
                self.detect_and_generate_missing_symbols(source_files, types_file)
            except Exception as e:
                logger.warning(f"缺失符号检测失败: {e}")
        
        # D.2: LLM 修复问题类型（可选，旧方法）
        # 用于查找 C 上下文的根目录：必须尽量覆盖整个项目树，否则容易“找不到定义 → LLM 无上下文 → 只能回退”。
        c_source_dir = self.project_root if self.project_root and self.project_root.exists() else (source_files[0].parent if source_files else None)
        if truth_mode:
            logger.info("Truth-mode: 跳过 LLM 骨架类型修复（types.rs 只接受 bindgen/编译数据库真值）")
        else:
            try:
                self.repair_skeleton_types(c_source_dir)
            except Exception as e:
                logger.warning(f"骨架类型修复失败: {e}")
        
        # D.3: 验证骨架编译
        success, error_msg = self.cargo_check()
        if success:
            logger.info("✓ 骨架编译验证通过")
        else:
            logger.warning(f"⚠ 骨架编译验证失败")
            
            # D.4: AI 原生自愈循环 (新架构)
            # 基于 rustc JSON 输出的确定性错误修复
            # 检查环境变量是否启用自愈循环
            # Truth-mode: never run self-healing (it will generate placeholders/extern/dummy defs).
            use_self_healing = (not truth_mode) and (os.environ.get("USE_SELF_HEALING", "true").lower() == "true")
            
            if not use_self_healing:
                if truth_mode:
                    logger.info("⏭️ Truth-mode: 自愈循环已禁用（保持真值，不做派生修补）")
                else:
                    logger.info("⏭️ 自愈循环已禁用 (USE_SELF_HEALING=false)")
                if truth_mode:
                    print("⏭️ Self-Healing disabled by truth-mode")
                else:
                    print("⏭️ Self-Healing disabled by environment variable")
            elif SELF_HEALING_AVAILABLE:
                logger.info("🔄 启动 AI 原生自愈循环...")
                try:
                    # Self-healing 的 LLM 需要的是“prompt -> raw response”的函数；
                    # llm_translate_fn 是“C signature -> Rust signature”的专用翻译器，不能复用。
                    llm_prompt_fn = None
                    try:
                        from generate.generation import generation as _generation  # prompt(str) -> str (兼容)
                        llm_prompt_fn = _generation
                    except Exception:
                        llm_prompt_fn = None

                    try:
                        max_cycles = int(os.environ.get("C2R_SELF_HEALING_MAX_CYCLES", "5"))
                        if max_cycles < 1:
                            max_cycles = 1
                    except Exception:
                        max_cycles = 5

                    print(f"  [SelfHealing] c_source_dir={c_source_dir}")
                    print(f"  [SelfHealing] llm_enabled={bool(llm_prompt_fn)} max_cycles={max_cycles}")

                    result = self._run_self_healing_loop(c_source_dir, llm_prompt_fn, max_cycles=max_cycles)
                    if result.success:
                        logger.info(f"✓ 自愈循环成功: {len(result.symbols_fixed)} 个符号已修复")
                        print(f"✅ Self-Healing Success: Fixed {len(result.symbols_fixed)} symbols")
                    else:
                        logger.warning(f"⚠ 自愈循环部分成功: 修复 {len(result.symbols_fixed)}, 剩余 {len(result.remaining_errors)}")
                        print(f"⚠️ Self-Healing Partial: Fixed {len(result.symbols_fixed)}, Remaining {len(result.remaining_errors)}")
                except Exception as e:
                    logger.warning(f"自愈循环异常: {e}")
            else:
                logger.warning("自愈模块不可用，跳过自愈循环")
        
        logger.info(f"骨架构建完成: {self.output_dir}")
        return self.output_dir
    
    def _run_self_healing_loop(
        self, 
        c_source_dir: Path = None,
        llm_fn=None,
        max_cycles: int = 5
    ):
        """
        运行 AI 原生自愈循环
        
        基于 rustc --message-format=json 的确定性错误修复。
        
        Args:
            c_source_dir: C 源码目录
            llm_fn: LLM 调用函数
            max_cycles: 最大循环次数
            
        Returns:
            SelfHealingResult
        """
        logger.info("🔄 启动 AI 原生自愈循环...")
        print("\n" + "="*60)
        print("🔄 AI-Native Self-Healing Loop")
        print("="*60)
        
        loop = SelfHealingLoop(
            project_root=self.output_dir,
            c_source_dir=c_source_dir,
            llm_fn=llm_fn,
            max_cycles=max_cycles
        )
        
        return loop.run()
    
    def _get_safe_module_name(self, src_file: Path) -> str:
        """
        辅助方法：根据文件相对路径生成唯一的模块名

        解决文件名冲突问题：
        - /abs/path/src/linux/ipc.c -> src_linux_ipc
        - /abs/path/src/liteos_m/ipc.c -> src_liteos_m_ipc

        Args:
            src_file: 源文件路径

        Returns:
            安全的 Rust 模块名
        """
        # 调用公共函数，确保与 get_dependencies.py 等其他模块命名一致
        return safe_module_name(self.project_root, src_file)
    
    def _generate_module_file(self, src_file_or_name, stubs: Dict[str, str]) -> str:
        """
        生成模块文件
        
        Args:
            src_file_or_name: 源文件路径 (Path) 或模块名 (str)
            stubs: 函数桩代码字典 {func_name: stub_code}
        
        Returns:
            生成的模块名
        """
        # 支持旧的调用方式（传入字符串模块名）和新的调用方式（传入 Path）
        if isinstance(src_file_or_name, Path):
            module_name = self._get_safe_module_name(src_file_or_name)
        else:
            module_name = src_file_or_name
        
        output_path = self.output_dir / "src" / f"{module_name}.rs"
        
        lines = [
            f'//! Module: {module_name}',
            '//!',
            '//! Auto-generated skeleton - function bodies are unimplemented.',
            '',
            '#![allow(unused_imports)]',
            '#![allow(dead_code)]',
            '#![allow(unused_variables)]',
            '#![allow(non_camel_case_types)]',
            '#![allow(non_snake_case)]',
            '',
            'use crate::types::*;',
            'use crate::globals::*;',
            'use crate::compat::*;',
            '',
        ]

        # Scheme B: file-scope `static` variables (internal linkage) live in the module, not in globals.rs.
        try:
            local_statics = getattr(self, "_module_local_file_statics", {}) or {}
            entries = local_statics.get(module_name) if isinstance(local_statics, dict) else None
        except Exception:
            entries = None
        if entries:
            lines.append("// === C2R_FILE_STATICS_BEGIN ===")
            lines.append("// File-scope `static` variables (internal linkage) from the original C TU.")
            lines.append("// These are module-local by design (Scheme B).")
            for ent in entries:
                try:
                    name = str(ent.get("name") or "").strip()
                    ty = str(ent.get("ty") or "").strip()
                    init = str(ent.get("init") or "").strip()
                    c_ty = str(ent.get("c_type") or "").strip()
                except Exception:
                    continue
                if not name or not ty:
                    continue
                if c_ty:
                    lines.append(f"/// C: static {c_ty} {name}")
                if init:
                    lines.append(f"static mut {name}: {ty} = {init};")
                else:
                    lines.append(
                        f"static mut {name}: {ty} = unsafe {{ core::mem::MaybeUninit::<{ty}>::zeroed().assume_init() }};"
                    )
                lines.append("")
            lines.append("// === C2R_FILE_STATICS_END ===")
            lines.append("")
        
        for func_name, stub in stubs.items():
            lines.append(stub)
            lines.append('')
        
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return module_name

    def _generate_compat_rs(self):
        """Generate `src/compat.rs` and `src/compatibility.rs` (centralized fallbacks + shims).

        Why both?
        - New skeletons should use `crate::compat`.
        - Some repair runs (or legacy outputs) may reference `crate::compatibility`.
          Keeping an alias module avoids noisy, non-semantic build failures.
        """
        src_dir = self.output_dir / "src"
        compat_path = src_dir / "compat.rs"

        compat_template: List[str] = [
            '//! Compatibility / Fallback Layer',
            '//!',
            '//! This module is auto-generated to keep the translated project compiling.',
            '//!',
            '//! Design goals:',
            '//! - Centralize placeholders and shims in ONE place (easy to audit & remove later).',
            '//! - Keep function bodies as close to translated semantics as possible.',
            '//!',
            '//! IMPORTANT:',
            '//! - Items here may be placeholders (value/layout unknown). Always review before relying on semantics.',
            '',
            '#![allow(dead_code)]',
            '#![allow(unused)]',
            '#![allow(non_snake_case)]',
            '#![allow(non_camel_case_types)]',
            '',
            '/// Minimal FFI prelude (for legacy skeletons that import `crate::compat::ffi::*`).',
            'pub mod ffi {',
            '    pub use core::ffi::*;',
            '}',
            '',
            '// === C2R_COMPAT_PLACEHOLDERS_BEGIN ===',
            '// (auto-appended placeholders will be inserted here)',
            '// === C2R_COMPAT_PLACEHOLDERS_END ===',
            '',
            '// === C2R_ACCESSOR_SHIMS_BEGIN ===',
            '// (auto-appended accessor shim declarations will be inserted here)',
            '// === C2R_ACCESSOR_SHIMS_END ===',
            '',
        ]

        src_dir.mkdir(parents=True, exist_ok=True)

        # Create-or-extend compat.rs (idempotent, non-destructive).
        if not compat_path.exists():
            compat_path.write_text("\n".join(compat_template), encoding="utf-8")
        else:
            try:
                existing = compat_path.read_text(encoding="utf-8", errors="ignore")
            except Exception:
                existing = ""
            # Avoid top-level `pub use core::ffi::*;` because it conflicts with `crate::types::*`
            # (e.g., `c_void` becomes ambiguous when both are glob-imported).
            lines = existing.splitlines()
            filtered: List[str] = []
            for line in lines:
                if line.strip() == "pub use core::ffi::*;" and not line.startswith(" "):
                    continue
                # Also drop the old comment line if it immediately precedes the removed export.
                if line.strip() == "/// Common C FFI types (c_int, c_char, c_void, ...)." and not line.startswith(" "):
                    continue
                filtered.append(line)
            updated = "\n".join(filtered)

            # Ensure minimal ffi prelude for older/LLM-modified imports.
            if "pub mod ffi" not in updated:
                updated = updated.rstrip() + "\n\n/// Minimal FFI prelude (for legacy skeletons that import `crate::compat::ffi::*`).\npub mod ffi {\n    pub use core::ffi::*;\n}\n"
            # Ensure insertion anchors exist for placeholder/shim injection.
            if "C2R_COMPAT_PLACEHOLDERS_BEGIN" not in updated:
                updated = updated.rstrip() + "\n\n// === C2R_COMPAT_PLACEHOLDERS_BEGIN ===\n// (auto-appended placeholders will be inserted here)\n// === C2R_COMPAT_PLACEHOLDERS_END ===\n"
            if "C2R_ACCESSOR_SHIMS_BEGIN" not in updated:
                updated = updated.rstrip() + "\n\n// === C2R_ACCESSOR_SHIMS_BEGIN ===\n// (auto-appended accessor shim declarations will be inserted here)\n// === C2R_ACCESSOR_SHIMS_END ===\n"
            if updated != existing and updated:
                compat_path.write_text(updated, encoding="utf-8")

        # Backward-compatibility alias: some outputs import `crate::compatibility::*;`
        compatibility_path = src_dir / "compatibility.rs"
        if not compatibility_path.exists():
            compatibility_path.write_text(
                "\n".join(
                    [
                        "//! Backward-compatibility alias for legacy skeleton outputs.",
                        "//!",
                        "//! Prefer `crate::compat` for new code.",
                        "",
                        "#![allow(dead_code)]",
                        "#![allow(unused)]",
                        "",
                        "pub use crate::compat::*;",
                        "",
                    ]
                ),
                encoding="utf-8",
            )
    
    def _generate_main_rs(self):
        """生成 main.rs"""
        output_path = self.output_dir / "src" / "main.rs"
        
        # 收集所有模块
        modules = []
        for rs_file in (self.output_dir / "src").glob("*.rs"):
            if rs_file.name not in ['main.rs', 'lib.rs']:
                modules.append(rs_file.stem)
        
        lines = [
            '//! Auto-generated main module',
            '',
            '#![allow(unused_imports)]',
            '#![allow(dead_code)]',
            '',
        ]
        
        for module in sorted(modules):
            lines.append(f'pub mod {module};')
        
        lines.extend([
            '',
            'fn main() {',
            '    println!("Skeleton project - implement function bodies");',
            '}',
        ])
        
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
    
    def _generate_cargo_toml(self, use_hybrid_build: bool = True):
        """
        生成 Cargo.toml
        
        Args:
            use_hybrid_build: 是否使用混合构建模式（C/Rust 链接）
        """
        output_path = self.output_dir / "Cargo.toml"
        # 从 project_root 或 output_dir 推断项目名称
        project_name = getattr(self, 'project_name', None) or self.project_root.name or "rust_skeleton"
        # 确保项目名称是有效的 Rust 包名（只包含字母数字和下划线）
        project_name = re.sub(r'[^a-zA-Z0-9_]', '_', project_name)
        
        if use_hybrid_build:
            # 混合构建模式：包含 cc 用于编译 C 依赖
            # 注意：不使用 [lib] 部分，因为骨架项目使用 main.rs
            content = f'''[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]
libc = "0.2"

[build-dependencies]
cc = "1.0"       # 编译 C/C++ 代码

[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = true
'''
        else:
            # 传统模式：纯 Rust 骨架
            content = f'''[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[profile.dev]
opt-level = 0
debug = true
'''
        
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        # 如果使用混合构建模式，确保 build.rs 存在（即使是空的）
        if use_hybrid_build:
            build_rs_path = self.output_dir / "build.rs"
            if not build_rs_path.exists():
                self._generate_empty_build_rs()
        
        logger.info(f"生成 Cargo.toml: {output_path} (混合构建: {use_hybrid_build})")
    
    def _setup_hybrid_build(
        self,
        target_files: List[Path],
        dependency_files: List[Path],
        header_files: List[Path] = None
    ) -> bool:
        """
        设置混合 C/Rust 构建环境
        
        这是渐进式重写的核心：
        - 只翻译 target_files
        - 其他文件保留为 C，编译成库通过 FFI 链接
        
        Args:
            target_files: 要翻译成 Rust 的文件
            dependency_files: 依赖的 C 文件（保留为 C）
            header_files: 头文件
        
        Returns:
            是否成功
        """
        if not HYBRID_BUILD_AVAILABLE:
            logger.warning("混合构建模块不可用，跳过")
            return False
        
        try:
            # 1. 创建 native 目录管理器
            native_manager = NativeDirectoryManager(self.output_dir)
            native_manager.setup()
            
            # 2. 复制依赖文件到 native/（排除目标文件）
            target_names = {f.name for f in target_files}
            for dep_file in dependency_files:
                if dep_file.name not in target_names and dep_file.exists():
                    native_manager.copy_source_file(dep_file, is_header=False)
            
            # 3. 复制头文件到 native/include/
            if header_files:
                for h in header_files:
                    if h.exists():
                        native_manager.copy_source_file(h, is_header=True)
            
            # 4. 从 compile_commands.json 提取编译参数
            include_dirs = list(self.include_dirs)
            defines = {}
            compiler_flags = []
            
            if self.compile_commands_parser:
                all_files = target_files + dependency_files
                cc_includes, cc_defines, cc_flags = extract_compile_flags_from_commands(
                    self.compile_commands_parser.compile_commands_path,
                    all_files
                )
                include_dirs.extend(cc_includes)
                defines.update(cc_defines)
                compiler_flags.extend(cc_flags)
            
            # 5. 添加 native/include 到 include 路径
            native_include = self.output_dir / "native" / "include"
            if native_include.exists():
                include_dirs.insert(0, native_include)
            
            # 6. 生成 build.rs
            c_sources = []
            for dep_file in dependency_files:
                if dep_file.name not in target_names:
                    c_sources.append(CSourceFile(
                        path=dep_file,
                        is_target=False
                    ))
            
            # 从 project_root 推断项目名称
            project_name = getattr(self, 'project_name', None) or self.project_root.name or "rust_skeleton"
            project_name = re.sub(r'[^a-zA-Z0-9_]', '_', project_name)

            # 生成 build.rs（动态扫描 native/ 下的 C/C++ 源文件）
            # 目的：后续阶段可以生成额外的 C shim（例如字段 accessor shims）而无需重写 build.rs。
            generate_build_rs(
                self.output_dir,
                c_sources,
                include_dirs,
                defines,
                compiler_flags,
                lib_name=f"{project_name}_native"
            )
            if c_sources:
                print(f"✅ 生成 build.rs：将编译 {len(c_sources)} 个 C 依赖文件（native/ 动态扫描包含额外 shim）")
            else:
                print("✅ 生成 build.rs：当前无 C 依赖文件（native/ 动态扫描允许后续 shim）")
            
            logger.info(f"混合构建环境设置完成: {len(c_sources)} 个 C 依赖文件")
            return True
            
        except Exception as e:
            logger.error(f"设置混合构建环境失败: {e}")
            return False
    
    def _generate_empty_build_rs(self):
        """生成空的 build.rs（当没有 C 依赖时）"""
        content = '''//! 构建脚本
//! 
//! 当前项目没有 C 依赖，此脚本为空。

fn main() {
    // 没有 C 代码需要编译
    println!("cargo:rerun-if-changed=build.rs");
}
'''
        build_rs_path = self.output_dir / "build.rs"
        with open(build_rs_path, 'w', encoding='utf-8') as f:
            f.write(content)
    
    def _generate_extern_bindings(
        self,
        external_functions: List[Dict],
        external_variables: List[Dict] = None
    ):
        """
        生成外部 C 函数/变量的 extern "C" 绑定
        
        这些函数来自 native/ 中编译的 C 代码，
        通过 FFI 链接而非翻译。
        
        Args:
            external_functions: 外部函数列表
            external_variables: 外部变量列表
        """
        if not HYBRID_BUILD_AVAILABLE:
            return
        
        output_file = self.output_dir / "src" / "extern_bindings.rs"
        
        generate_extern_declarations(
            external_functions or [],
            external_variables or [],
            output_file
        )
        
        # 在 lib.rs 中添加 mod extern_bindings
        lib_rs_path = self.output_dir / "src" / "lib.rs"
        if lib_rs_path.exists():
            with open(lib_rs_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            if 'mod extern_bindings' not in content:
                with open(lib_rs_path, 'a', encoding='utf-8') as f:
                    f.write('\n\npub mod extern_bindings;\n')
                    f.write('pub use extern_bindings::*;\n')
    
    def finalize_types_rs(self):
        """
        在 types.rs 末尾追加 bindgen 遗漏的类型定义。
        
        这是解决 "cannot find type" 的兜底方案。
        在所有文件处理完后调用此方法，生成缺失的不透明结构体。
        """
        # Truth-mode: do not generate defensive opaque types / common constants.
        # Keep missing types as compile-time signals (usually indicates incomplete TU/build context).
        if self._env_flag("C2R_TRUTH_MODE", "0") or (not self._env_flag("C2R_ENABLE_FINALIZE_TYPES_RS", "1")):
            logger.info("finalize_types_rs skipped (truth-mode or disabled)")
            return

        if not TYPE_UTILS_AVAILABLE:
            logger.warning("type_utils 不可用，跳过 finalize_types_rs")
            return
        
        types_rs_path = self.output_dir / "src" / "types.rs"
        
        # 1. 读取现有的 types.rs，看看 bindgen 已经生成了什么
        existing_content = ""
        if types_rs_path.exists():
            with open(types_rs_path, 'r', encoding='utf-8') as f:
                existing_content = f.read()
        
        # 简单正则提取已定义的 struct/enum/type 名称，避免重复定义
        defined_types = set(re.findall(r'pub (?:struct|enum|union|type) (\w+)', existing_content))

        # 同时提取通过 `pub use` 重新导出的类型 (例如 TU types: pub use module::{Type1, Type2};)
        # Pattern 1: pub use module::{Type1, Type2};
        pub_use_pattern = r'pub use [^;]+::\{([^}]+)\}'
        for match in re.findall(pub_use_pattern, existing_content):
            # 解析 {Type1, Type2, ...} 中的类型名
            for type_name in match.split(','):
                type_name = type_name.strip()
                # 处理 `as OtherName` 重命名的情况
                if ' as ' in type_name:
                    type_name = type_name.split(' as ')[1].strip()
                if type_name and type_name.isidentifier():
                    defined_types.add(type_name)

        # Pattern 2: pub use module::Type; (单个类型导出)
        pub_use_single_pattern = r'pub use \w+::(\w+);'
        for type_name in re.findall(pub_use_single_pattern, existing_content):
            if type_name and type_name.isidentifier():
                defined_types.add(type_name)
        
        # 2. 计算缺失的类型
        # 从收集到的所有类型中，减去 Rust 原生类型和已经定义的类型
        rust_primitives = {
            'i8', 'u8', 'i16', 'u16', 'i32', 'u32', 'i64', 'u64', 
            'f32', 'f64', 'bool', 'usize', 'isize', 'str', 'String',
            'Vec', 'Option', 'Result', 'Box', 'Rc', 'Arc', 'self', 'Self',
            'true', 'false', 'None', 'Some', 'Ok', 'Err',
            # C 基础类型（已在类型映射中处理）
            'int', 'char', 'void', 'float', 'double', 'long', 'short',
            '_Bool', 'bool', 'unsigned', 'signed',
            'size_t', 'ssize_t', 'ptrdiff_t', 'intptr_t', 'uintptr_t',
            'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t',
            'int8_t', 'int16_t', 'int32_t', 'int64_t'
        }
        missing_types = self.collected_custom_types - defined_types - rust_primitives
        
        if not missing_types:
            logger.debug("没有缺失的类型需要生成")
            # 即使没有缺失类型，也需要补齐常用常量（例如 pthread initializer）。
            self._append_common_constants(types_rs_path, existing_content)
            return
        
        # ========== 增强: 过滤非法类型名 (解决 pack 项目崩溃问题) ==========
        # 在生成之前先过滤掉所有非法类型名
        valid_types = set()
        skipped_types = []
        for type_name in missing_types:
            # 过滤掉显然非法的名字
            if not is_valid_c_identifier(type_name):
                skipped_types.append(type_name)
                logger.debug(f"跳过非法类型名: {type_name}")
                continue
            valid_types.add(type_name)
        
        if skipped_types:
            print(f"⚠️ Skipped {len(skipped_types)} invalid type names: {skipped_types[:5]}{'...' if len(skipped_types) > 5 else ''}")
        
        if not valid_types:
            logger.debug("过滤后没有需要生成的类型")
            return
        
        print(f"🛡️ Defensively generating {len(valid_types)} opaque types: {sorted(valid_types)}")
        logger.info(f"生成 {len(valid_types)} 个防御性不透明类型")
        
        # 3. 追加不透明定义 (使用 RustCodeBuilder 确保语法正确)
        with open(types_rs_path, 'a', encoding='utf-8') as f:
            if RUST_CODE_BUILDER_AVAILABLE:
                # ★ 使用 RustCodeBuilder 安全生成 ★
                builder = RustCodeBuilder()
                builder.add_line("")
                builder.add_line("// ============================================================")
                builder.add_line("// Auto-generated Defensive Opaque Types")
                builder.add_line("// These types were found in function signatures or variable")
                builder.add_line("// declarations but were not generated by bindgen.")
                builder.add_line("// ============================================================")
                builder.add_line("")
                
                # 已知系统类型的特殊定义（比空结构体更正确）
                known_system_types = {
                    'file': 'pub struct file { _opaque: [u8; 0] }',
                    'FILE': 'pub struct FILE { _opaque: [u8; 0] }',
                    'pthread_mutex_t': 'pub struct pthread_mutex_t { _opaque: [u8; 40] }',
                    'pthread_cond_t': 'pub struct pthread_cond_t { _opaque: [u8; 48] }',
                    'pthread_t': 'pub type pthread_t = usize;',
                    'pthread_attr_t': 'pub struct pthread_attr_t { _opaque: [u8; 56] }',
                    'sem_t': 'pub struct sem_t { _opaque: [u8; 32] }',
                    'AudioPort': 'pub struct AudioPort { _opaque: [u8; 0] }',
                    'AudioPortCapability': 'pub struct AudioPortCapability { _opaque: [u8; 0] }',
                    'AudioAdapter': 'pub struct AudioAdapter { _opaque: [u8; 0] }',
                    'AudioCapture': 'pub struct AudioCapture { _opaque: [u8; 0] }',
                    'AudioRender': 'pub struct AudioRender { _opaque: [u8; 0] }',
                    'AudioDeviceDescriptor': 'pub struct AudioDeviceDescriptor { _opaque: [u8; 0] }',
                }
                
                for type_name in sorted(valid_types):
                    # 确保类型名是有效的 Rust 标识符
                    rust_safe_name = type_name.replace(' ', '_')
                    if rust_safe_name and not rust_safe_name[0].isalpha() and rust_safe_name[0] != '_':
                        rust_safe_name = '_' + rust_safe_name
                    
                    # Tier-0: map common primitive typedefs to real aliases
                    if type_name in PRIMITIVE_TYPEDEF_ALIASES:
                        alias = PRIMITIVE_TYPEDEF_ALIASES[type_name]
                        builder.add_line(f"/// C2R_PRIMITIVE_TYPEDEF: rule-mapped `{type_name}` -> `{alias}`")
                        builder.add_line(f"pub type {rust_safe_name} = {alias};")
                        builder.add_line("")
                        continue

                    # 检查是否是已知系统类型
                    if type_name in known_system_types:
                        builder.add_line(f"/// Known system type `{type_name}`")
                        stmt = known_system_types[type_name]
                        if stmt.strip().startswith("pub type"):
                            builder.add_line(stmt)
                        else:
                            builder.add_line("#[repr(C)]")
                            builder.add_line(stmt)
                        builder.add_line("")
                    else:
                        # 使用 Builder 生成不透明结构体
                        builder.add_opaque_struct(
                            rust_safe_name, 
                            doc=f"Opaque placeholder for external type `{type_name}`"
                        )
                
                f.write(builder.build())
            else:
                # 回退到传统字符串拼接
                f.write("\n\n// --- Auto-generated Defensive Opaque Types ---\n")
                f.write("// These types were found in function signatures or variable declarations\n")
                f.write("// but were not generated by bindgen. They are defined as opaque types\n")
                f.write("// to allow compilation without the actual header definitions.\n\n")
                
                for type_name in sorted(valid_types):
                    rust_safe_name = type_name.replace(' ', '_')
                    if rust_safe_name and not rust_safe_name[0].isalpha() and rust_safe_name[0] != '_':
                        rust_safe_name = '_' + rust_safe_name

                    # Tier-0: map common primitive typedefs to real aliases
                    if type_name in PRIMITIVE_TYPEDEF_ALIASES:
                        alias = PRIMITIVE_TYPEDEF_ALIASES[type_name]
                        f.write(f"/// C2R_PRIMITIVE_TYPEDEF: rule-mapped `{type_name}` -> `{alias}`\n")
                        f.write(f"pub type {rust_safe_name} = {alias};\n\n")
                        continue

                    f.write(f"/// Opaque placeholder for external type `{type_name}`\n")
                    f.write(f"#[repr(C)]\n")
                    f.write(f"#[derive(Debug, Copy, Clone)]\n")
                    f.write(f"pub struct {rust_safe_name} {{\n")
                    f.write(f"    _private: [u8; 0],\n")
                    f.write(f"}}\n\n")
        
        logger.info(f"已追加 {len(valid_types)} 个不透明类型到 types.rs")
        
        # 3. 追加常用常量定义（无论 bindgen 是否成功都需要）
        try:
            updated_content = types_rs_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            updated_content = existing_content
        self._append_common_constants(types_rs_path, updated_content)

    def apply_primitive_typedef_fixes(self) -> dict:
        """
        Tier-0: Replace trivial opaque structs for common typedefs with Rust type aliases.

        This is a post-process step after types.rs generation/finalization, and it is
        intentionally deterministic (no LLM). It targets cases like:

            pub struct INT32 { _private: [u8; 0] }

        and rewrites them to:

            pub type INT32 = i32;
        """
        import json
        import re

        types_rs_path = self.output_dir / "src" / "types.rs"
        if not types_rs_path.exists():
            return {"applied": 0, "fixes": []}

        try:
            content = types_rs_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            return {"applied": 0, "fixes": []}

        allowed_field_names = {"_opaque", "_private", "_unused", "_c2r_private", "_reserved"}

        def _is_trivial_opaque_struct(block_text: str) -> bool:
            body_match = re.search(r"(?s)\{(.*)\}", block_text)
            if not body_match:
                return False
            body = body_match.group(1)
            for m in re.finditer(r"\b(pub\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:", body):
                name = m.group(2)
                if name not in allowed_field_names:
                    return False
            return True

        def _find_struct_block_span(src: str, struct_name: str) -> Optional[Tuple[int, int, str]]:
            m = re.search(rf"(?m)^\s*pub\s+struct\s+{re.escape(struct_name)}\b", src)
            if not m:
                return None
            # expand upward to include contiguous doc/attr lines
            start = src.rfind("\n", 0, m.start()) + 1
            while start > 0:
                prev_nl = src.rfind("\n", 0, start - 1)
                if prev_nl == -1:
                    break
                prev_line_start = prev_nl + 1
                prev_line = src[prev_line_start:start].strip()
                if prev_line.startswith("///") or prev_line.startswith("#["):
                    start = prev_line_start
                    continue
                break

            # find matching brace block
            brace_open = src.find("{", m.end())
            if brace_open == -1:
                return None
            depth = 0
            i = brace_open
            while i < len(src):
                ch = src[i]
                if ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                    if depth == 0:
                        end = i + 1
                        # include trailing whitespace/newlines
                        while end < len(src) and src[end] in " \t\r":
                            end += 1
                        if end < len(src) and src[end] == "\n":
                            end += 1
                        block = src[start:end]
                        return start, end, block
                i += 1
            return None

        fixes: List[dict] = []
        new_content = content

        for type_name, alias in PRIMITIVE_TYPEDEF_ALIASES.items():
            span = _find_struct_block_span(new_content, type_name)
            if not span:
                continue
            start, end, block = span
            if not _is_trivial_opaque_struct(block):
                continue
            replacement = (
                f"/// C2R_PRIMITIVE_TYPEDEF: rule-mapped `{type_name}` -> `{alias}`\n"
                f"pub type {type_name} = {alias};\n"
            )
            new_content = new_content[:start] + replacement + new_content[end:]
            fixes.append({"name": type_name, "mapped_to": alias, "action": "replace_opaque_struct"})

        if new_content == content:
            # Still record the mapping table (best effort) for traceability.
            try:
                report_path = self.output_dir / "types_generation_report.json"
                if report_path.exists():
                    report = json.loads(report_path.read_text(encoding="utf-8", errors="ignore") or "{}")
                else:
                    report = {}
                report.setdefault("primitive_typedef_aliases", PRIMITIVE_TYPEDEF_ALIASES)
                report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2, default=str), encoding="utf-8")
            except Exception:
                pass
            return {"applied": 0, "fixes": []}

        try:
            types_rs_path.write_text(new_content, encoding="utf-8", errors="ignore")
        except Exception:
            return {"applied": 0, "fixes": []}

        # Update types_generation_report.json (best effort)
        report_path = self.output_dir / "types_generation_report.json"
        try:
            if report_path.exists():
                report = json.loads(report_path.read_text(encoding="utf-8", errors="ignore") or "{}")
            else:
                report = {}
            report.setdefault("primitive_typedef_aliases", PRIMITIVE_TYPEDEF_ALIASES)
            report["primitive_typedef_fixes"] = fixes
            report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2, default=str), encoding="utf-8")
        except Exception:
            pass

        logger.info(f"Tier-0 primitive typedef fixes applied: {len(fixes)}")
        return {"applied": len(fixes), "fixes": fixes}
    
    def _append_common_constants(self, types_rs_path: Path, existing_content: str):
        """
        追加常用常量定义到 types.rs
        
        使用可配置的 PredefineManager 获取常量定义。
        这些常量在 OpenHarmony/SoftBus 项目中经常使用，
        LLM 翻译时会保留原始常量名，因此需要预先定义。
        """
        # 检查已存在的常量，避免重复定义
        defined_constants = set(re.findall(r'pub const (\w+):', existing_content))
        
        constants_to_add = []
        
        # 从配置获取所有常量
        if PREDEFINES_AVAILABLE:
            manager = get_predefine_manager(enable_ohos=True)
            all_constants = manager.get_all_constants()
        else:
            # 回退到硬编码（兼容性）
            all_constants = [
                # SoftBus 常量
                ("SOFTBUS_OK", "i32", "0"),
                ("SOFTBUS_ERR", "i32", "-1"),
                ("SOFTBUS_INVALID_PARAM", "i32", "-3"),
                # HDF 常量
                ("HDF_SUCCESS", "i32", "0"),
                ("HDF_FAILURE", "i32", "-1"),
                # LOS 常量
                ("LOS_OK", "i32", "0"),
                ("LOS_NOK", "i32", "-1"),
                # POSIX 常量
                ("PTHREAD_MUTEX_INITIALIZER", "pthread_mutex_t", "unsafe { ::core::mem::zeroed() }"),
            ]
        
        # 先确保 pthread 类型存在（在常量之前添加）
        self._ensure_pthread_types(types_rs_path, existing_content)
        
        for name, typ, value in all_constants:
            if name not in defined_constants:
                constants_to_add.append((name, typ, value))
        
        if not constants_to_add:
            return
        
        # 追加到 types.rs (使用 RustCodeBuilder 确保语法正确)
        with open(types_rs_path, 'a', encoding='utf-8') as f:
            if RUST_CODE_BUILDER_AVAILABLE:
                builder = RustCodeBuilder()
                builder.add_line("")
                builder.add_line("// ============================================================")
                builder.add_line("// Common Constants (自动追加)")
                builder.add_line("// ============================================================")
                builder.add_line("")
                
                for name, typ, value in constants_to_add:
                    builder.add_const(name, typ, value)
                
                f.write(builder.build())
            else:
                f.write("\n// ============================================================\n")
                f.write("// Common Constants (自动追加)\n")
                f.write("// ============================================================\n\n")
                
                for name, typ, value in constants_to_add:
                    f.write(f"pub const {name}: {typ} = {value};\n")
        
        logger.info(f"已追加 {len(constants_to_add)} 个常用常量到 types.rs")
    
    def _ensure_pthread_types(self, types_rs_path: Path, existing_content: str):
        """确保 pthread 相关类型存在（PTHREAD_MUTEX_INITIALIZER 等常量需要它们）"""
        pthread_types = [
            ('pthread_mutex_t', '#[repr(C)]\npub struct pthread_mutex_t { _opaque: [u8; 40] }'),
            ('pthread_cond_t', '#[repr(C)]\npub struct pthread_cond_t { _opaque: [u8; 48] }'),
            ('pthread_rwlock_t', '#[repr(C)]\npub struct pthread_rwlock_t { _opaque: [u8; 56] }'),
            ('pthread_once_t', 'pub type pthread_once_t = i32;'),
        ]
        
        types_to_add = []
        for type_name, definition in pthread_types:
            # 检查是否已定义
            if f'struct {type_name}' not in existing_content and f'type {type_name}' not in existing_content:
                types_to_add.append(definition)
        
        if types_to_add:
            with open(types_rs_path, 'a', encoding='utf-8') as f:
                f.write("\n// --- POSIX Thread Types ---\n")
                for definition in types_to_add:
                    f.write(definition + '\n')
                f.write('\n')
    
    def repair_skeleton_types(self, c_source_dir: Path = None, max_rounds: int = 3) -> bool:
        """
        LLM 辅助类型定义修复 (参考 EvoC2Rust & Tymcrat)
        
        循环修复机制：修复一个类型可能会暴露下一个依赖类型的错误，
        因此需要多轮修复直到没有新错误或达到最大轮次。
        
        Args:
            c_source_dir: C 源码目录（用于查找类型上下文）
            max_rounds: 最大修复轮次（默认 3）
            
        Returns:
            是否骨架能通过编译
        """
        types_rs_path = self.output_dir / "src" / "types.rs"
        if not types_rs_path.exists():
            logger.warning("types.rs 不存在，跳过骨架修复")
            return True
        
        total_fixed = 0
        total_fallback = 0
        attempted_types = set()  # 避免重复尝试同一个类型
        
        for round_num in range(1, max_rounds + 1):
            with open(types_rs_path, 'r', encoding='utf-8') as f:
                types_content = f.read()
            
            # 1. 检测问题类型（排除已尝试的）
            problem_types = self._detect_problem_types(types_content)
            new_problems = {k: v for k, v in problem_types.items() if k not in attempted_types}
            
            if not new_problems:
                if round_num == 1:
                    logger.info("未检测到问题类型，骨架类型定义正常")
                else:
                    logger.info(f"第 {round_num} 轮：无新问题类型")
                break
            
            print(f"\n🔧 第 {round_num}/{max_rounds} 轮：检测到 {len(new_problems)} 个问题类型")
            logger.info(f"第 {round_num} 轮问题类型: {list(new_problems.keys())}")
            
            # 2. 对每个问题类型，尝试 LLM 修复
            round_fixed = 0
            round_fallback = 0
            
            for type_name, problem_desc in new_problems.items():
                attempted_types.add(type_name)
                
                # 查找类型在 C 源码中的上下文
                c_context = ""
                if c_source_dir and c_source_dir.exists():
                    c_context = self._find_type_context(type_name, c_source_dir)
                
                # 调用 LLM 修复
                fixed_def = self._llm_fix_type(type_name, problem_desc, c_context, types_content)
                
                if fixed_def:
                    # LLM 修复成功
                    types_content = self._replace_type_definition(types_content, type_name, fixed_def)
                    round_fixed += 1
                    print(f"  ✓ LLM 修复: {type_name}")
                else:
                    # LLM 修复失败，使用 FIXME 回退方案
                    fallback_def = self._generate_fixme_fallback(type_name, problem_desc, c_context)
                    types_content = self._replace_type_definition(types_content, type_name, fallback_def)
                    round_fallback += 1
                    print(f"  ⚠ FIXME 回退: {type_name}")
            
            # 3. 写回文件
            with open(types_rs_path, 'w', encoding='utf-8') as f:
                f.write(types_content)
            
            total_fixed += round_fixed
            total_fallback += round_fallback
            
            print(f"  第 {round_num} 轮完成: LLM 修复 {round_fixed}, FIXME 回退 {round_fallback}")
            
            # 4. 验证编译（如果这轮有修改）
            if round_fixed + round_fallback > 0:
                success, error_msg = self.cargo_check()
                if success:
                    print(f"✓ 骨架编译验证通过")
                    break
                else:
                    # 解析编译错误，可能有新的类型问题
                    new_type_errors = self._extract_type_errors_from_cargo(error_msg)
                    if new_type_errors:
                        logger.info(f"编译发现新类型错误: {new_type_errors}")
        
        # 最终统计
        if total_fixed + total_fallback > 0:
            print(f"\n📊 骨架修复统计: LLM 成功 {total_fixed}, FIXME 回退 {total_fallback}")
        
        # 最终编译验证
        success, _ = self.cargo_check()
        return success
    
    def _generate_fixme_fallback(self, type_name: str, problem_desc: str, c_context: str) -> str:
        """
        生成 FIXME 回退类型定义
        
        当 LLM 无法修复类型时，生成一个 libc::c_void 占位符，
        确保骨架能通过编译，而不是卡死。
        """
        # 生成详细的 FIXME 注释
        lines = [
            f"/// FIXME: {type_name} - {problem_desc}",
            f"/// ",
            f"/// This type could not be automatically translated.",
            f"/// Original C context (if found):",
        ]
        
        if c_context:
            for line in c_context.split('\n')[:5]:  # 最多 5 行 C 代码
                lines.append(f"/// {line}")
        else:
            lines.append("/// (no C definition found)")
        
        lines.append(f"/// ")
        lines.append(f"/// Manual translation required.")
        # 使用 Zero-Sized Type (ZST) 而非 c_void，防止类型混淆
        lines.append(f"#[repr(C)]")
        lines.append(f"#[derive(Debug, Copy, Clone)]")
        lines.append(f"pub struct {type_name} {{")
        lines.append(f"    _private: [u8; 0],")
        lines.append(f"}}")
        
        return '\n'.join(lines)
    
    def _extract_type_errors_from_cargo(self, error_msg: str) -> List[str]:
        """
        从 cargo check 错误信息中提取类型相关错误
        
        Returns:
            缺失或错误的类型名列表
        """
        type_errors = []
        
        # 匹配 "cannot find type `X`" 错误
        pattern1 = re.compile(r"cannot find type `(\w+)`")
        for match in pattern1.finditer(error_msg):
            type_errors.append(match.group(1))
        
        # 匹配 "not found in this scope" 类型错误
        pattern2 = re.compile(r"`(\w+)` not found in this scope")
        for match in pattern2.finditer(error_msg):
            type_errors.append(match.group(1))
        
        return list(set(type_errors))
    
    def _detect_problem_types(self, types_content: str) -> Dict[str, str]:
        """
        检测 types.rs 中的问题类型
        
        Returns:
            {type_name: problem_description}
        """
        problems = {}
        
        # 检测 unknown 类型（TypeMapper 的占位符）
        unknown_pattern = r'pub type (\w+)\s*=\s*(?:\*mut\s+)?c_void\s*;.*unknown'
        for match in re.finditer(unknown_pattern, types_content, re.IGNORECASE):
            type_name = match.group(1)
            problems[type_name] = "unknown type (placeholder from TypeMapper)"
        
        # 检测空结构体（可能是不完整的定义）
        empty_struct_pattern = r'pub struct (\w+)\s*\{\s*_private:\s*\[u8;\s*0\]'
        for match in re.finditer(empty_struct_pattern, types_content):
            type_name = match.group(1)
            # 检查是否有 "Opaque placeholder" 注释（这是正常的）
            context_start = max(0, match.start() - 100)
            context = types_content[context_start:match.start()]
            if "Opaque placeholder" not in context and "opaque" not in context.lower():
                problems[type_name] = "empty struct (possibly incomplete)"
        
        # 检测包含非法字符的类型名
        all_types = re.findall(r'pub (?:struct|enum|type|union) (\w+)', types_content)
        for type_name in all_types:
            if ' ' in type_name or not type_name[0].isalpha() and type_name[0] != '_':
                problems[type_name] = "invalid type name"
        
        return problems
    
    def _find_type_context(self, type_name: str, c_source_dir: Path, max_lines: int = 50) -> str:
        """
        在 C 源码中查找类型定义的上下文
        
        Args:
            type_name: 类型名
            c_source_dir: C 源码目录
            max_lines: 最大返回行数
            
        Returns:
            C 源码上下文
        """
        contexts = []
        
        # 在头文件和源文件中搜索
        for ext in ['*.h', '*.hpp', '*.c', '*.cpp']:
            for file_path in c_source_dir.rglob(ext):
                try:
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                    
                    # 查找类型定义
                    patterns = [
                        rf'typedef\s+[^;]*\b{re.escape(type_name)}\b[^;]*;',
                        rf'struct\s+{re.escape(type_name)}\s*\{{[^}}]*\}}',
                        rf'enum\s+{re.escape(type_name)}\s*\{{[^}}]*\}}',
                        rf'#define\s+{re.escape(type_name)}\b[^\n]*',
                    ]
                    
                    for pattern in patterns:
                        matches = re.findall(pattern, content, re.DOTALL)
                        for match in matches:
                            if len(match) < 2000:  # 避免过长
                                contexts.append(f"// From {file_path.name}:\n{match}")
                    
                    if len(contexts) >= 3:
                        break
                except Exception:
                    continue
            
            if len(contexts) >= 3:
                break
        
        return "\n\n".join(contexts[:3])
    
    def _llm_fix_type(self, type_name: str, problem_desc: str, c_context: str, existing_types: str) -> Optional[str]:
        """
        调用 LLM 修复类型定义
        
        Returns:
            修复后的 Rust 类型定义，或 None（如果修复失败）
        """
        try:
            from generate.generation import generation
        except ImportError:
            logger.warning("无法导入 generation 模块，跳过 LLM 类型修复")
            return None
        
        system_prompt = """You are a C to Rust type translation expert.

Your task is to generate a valid Rust type definition for a C type.

RULES:
1. Generate a compilable Rust struct/enum/type alias
2. Use #[repr(C)] for C compatibility
3. If the type is complex or unknown, generate an opaque Zero-Sized Type (ZST):
   ```rust
   #[repr(C)]
   #[derive(Debug, Copy, Clone)]
   pub struct TypeName {
       _private: [u8; 0],
   }
   ```
   DO NOT use `pub type TypeName = *mut std::ffi::c_void;` - it loses type safety!
4. Add helpful comments explaining the type

Output ONLY the Rust type definition (no markdown, no explanation)."""

        user_prompt = f"""Fix this problematic type: {type_name}
Problem: {problem_desc}

C source context (if available):
```c
{c_context if c_context else "(no context found)"}
```

Existing Rust types (for reference):
```rust
{existing_types[:2000]}
```

Generate a valid Rust type definition for `{type_name}`:"""

        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ]
        
        try:
            response = generation(messages)
            if isinstance(response, dict):
                response = response.get("content", "")
            
            # 清理响应（移除 markdown）
            response = response.strip()
            if response.startswith("```"):
                response = re.sub(r'^```\w*\n?', '', response)
                response = re.sub(r'\n?```$', '', response)
            
            # 验证响应是否包含类型定义
            if type_name in response and ('pub ' in response or 'type ' in response or 'struct ' in response):
                return response.strip()
            else:
                logger.warning(f"LLM 响应不包含有效的类型定义: {response[:100]}")
                return None
        except Exception as e:
            logger.warning(f"LLM 类型修复失败 [{type_name}]: {e}")
            return None
    
    def _replace_type_definition(self, content: str, type_name: str, new_definition: str) -> str:
        """
        替换 types.rs 中的类型定义
        """
        # 尝试匹配并替换现有定义
        patterns = [
            rf'(///[^\n]*\n)?#\[repr\(C\)\]\s*#\[derive[^\]]*\]\s*pub struct {re.escape(type_name)}\s*\{{[^}}]*\}}',
            rf'(///[^\n]*\n)?pub type {re.escape(type_name)}\s*=[^;]*;',
            rf'(///[^\n]*\n)?pub struct {re.escape(type_name)}\s*\{{[^}}]*\}}',
        ]
        
        for pattern in patterns:
            if re.search(pattern, content):
                content = re.sub(pattern, new_definition, content)
                return content
        
        # 如果没找到现有定义，追加到末尾
        content += f"\n\n// LLM-generated type definition\n{new_definition}\n"
        return content
    
    def cargo_check(
        self,
        log_suffix: str = "full",
        use_offline: bool = True
    ) -> Tuple[bool, str]:
        """
        运行 cargo check 验证骨架

        Phase 0 改进：
        - 使用离线模式避免网络问题干扰真实错误
        - 完整日志落盘便于调试

        使用 RUSTFLAGS 抑制骨架阶段常见的无害警告：
        - unused_imports: 占位用的 use 语句
        - dead_code: 未被 main 调用的函数（骨架中间状态）
        - unused_variables: 占位参数
        - unused_mut: 防御性可变声明

        Args:
            log_suffix: 日志文件后缀，用于区分不同阶段
                       - "full": 初始编译 -> compile_error_full.log
                       - "after_rule_fix": 规则修复后 -> compile_error_after_rule_fix.log
                       - "after_precise_fix": 精确修复后 -> compile_error_after_precise_fix.log
            use_offline: 是否使用离线模式（默认 True，避免网络干扰）

        Returns:
            (是否成功, 错误信息)
        """
        try:
            # 设置环境变量
            env = os.environ.copy()
            # RUSTFLAGS 抑制无害警告
            env["RUSTFLAGS"] = "-A unused_imports -A dead_code -A unused_variables -A unused_mut"
            # 离线模式：避免 crates.io index / lock 把真正错误淹没
            if use_offline:
                env["CARGO_NET_OFFLINE"] = "true"

            # 构建命令
            cmd = ["cargo", "check"]
            if use_offline:
                cmd.append("--offline")
            try:
                from cargo_utils import with_cargo_jobs
                cmd = with_cargo_jobs(cmd)
            except Exception:
                pass

            try:
                result = subprocess.run(
                    cmd,
                    cwd=self.output_dir,
                    capture_output=True,
                    text=True,
                    timeout=120,
                    env=env,
                )
            except subprocess.TimeoutExpired as e:
                output = ""
                if e.stdout:
                    output += e.stdout
                if e.stderr:
                    output += e.stderr
                msg = f"cargo check timed out after {e.timeout} seconds"
                if output:
                    msg += "\n\n" + output
                # 写入日志文件
                self._write_compile_log(msg, log_suffix)
                return False, msg

            # 合并输出
            output = result.stdout + result.stderr

            # 写入日志文件（无论成功失败都写）
            if output.strip():
                self._write_compile_log(output, log_suffix)

            if result.returncode == 0:
                return True, ""

            # 即使返回码非0，如果只有警告没有错误，也视为成功
            has_error = bool(re.search(r'error\[E\d+\]:', output))
            if not has_error and "Finished" in output:
                return True, ""

            return False, output

        except Exception as e:
            error_msg = str(e)
            self._write_compile_log(f"Exception: {error_msg}", log_suffix)
            return False, error_msg

    def _write_compile_log(self, content: str, log_suffix: str):
        """
        写入编译日志文件

        Args:
            content: 日志内容
            log_suffix: 日志后缀
        """
        log_filename = f"compile_error_{log_suffix}.log"
        log_path = self.output_dir / log_filename

        try:
            with open(log_path, 'w', encoding='utf-8') as f:
                f.write(f"# Compilation Log ({log_suffix})\n")
                f.write(f"# Generated at: {__import__('datetime').datetime.now().isoformat()}\n")
                f.write(f"# Project: {self.output_dir}\n")
                f.write("=" * 60 + "\n\n")
                f.write(content)
            logger.debug(f"编译日志已写入: {log_path}")
        except Exception as e:
            logger.warning(f"写入编译日志失败: {e}")


# =========================================================================
# LLM 签名翻译辅助
# =========================================================================

def create_signature_translation_prompt(c_signature: str) -> str:
    """
    创建签名翻译的 LLM 提示词
    
    这个提示词专门用于翻译签名，难度低，准确率高
    """
    return f'''Translate the following C function signature to Rust.

Rules:
1. Use types from `crate::types::*` for custom types.
2. Keep the function body as `unimplemented!();` - DO NOT implement the logic.
3. Common type mappings: int→i32, unsigned int→u32, char→i8, void*→*mut std::ffi::c_void
4. Pointer parameters: Type* → *mut Type or *const Type
5. For callbacks/function pointers, use: extern "C" fn(...) -> ...

C Signature:
{c_signature}

Output only the Rust function definition (signature + unimplemented body), no explanation:
'''


def parse_llm_signature_response(response: str) -> str:
    """解析 LLM 签名翻译响应"""
    # 提取代码块
    code_match = re.search(r'```rust\s*(.*?)\s*```', response, re.DOTALL)
    if code_match:
        return code_match.group(1).strip()
    
    # 尝试直接提取函数定义
    fn_match = re.search(r'((?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+\w+\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{[^}]*\})', response, re.DOTALL)
    if fn_match:
        return fn_match.group(1).strip()
    
    return response.strip()


# =========================================================================
# 便捷函数
# =========================================================================

def build_skeleton_for_project(
    project_root: str,
    output_dir: str,
    llm_generation_fn=None
) -> Path:
    """
    为项目构建骨架的便捷函数
    
    Args:
        project_root: C++ 项目根目录
        output_dir: 输出目录
        llm_generation_fn: 可选的 LLM 生成函数，签名为 (messages: List[Dict]) -> str
    
    Returns:
        骨架项目路径
    """
    project_root = Path(project_root)
    output_dir = Path(output_dir)
    
    # 收集源文件和头文件
    source_files = list(project_root.glob("**/*.c")) + list(project_root.glob("**/*.cpp"))
    header_files = list(project_root.glob("**/*.h")) + list(project_root.glob("**/*.hpp"))
    
    # 创建构建器
    builder = SkeletonBuilder(project_root, output_dir)
    
    # LLM 翻译包装器
    llm_translate_fn = None
    if llm_generation_fn:
        def translate_signature(c_sig: str) -> str:
            prompt = create_signature_translation_prompt(c_sig)
            messages = [
                {"role": "system", "content": "You are a C to Rust translation expert. Translate function signatures accurately."},
                {"role": "user", "content": prompt}
            ]
            response = llm_generation_fn(messages)
            return parse_llm_signature_response(response)
        
        llm_translate_fn = translate_signature
    
    # 构建骨架
    return builder.build_skeleton(source_files, header_files, llm_translate_fn)


if __name__ == "__main__":
    # 测试代码
    import sys
    
    if len(sys.argv) < 3:
        print("Usage: python skeleton_builder.py <project_root> <output_dir>")
        sys.exit(1)
    
    project_root = sys.argv[1]
    output_dir = sys.argv[2]
    
    logging.basicConfig(level=logging.INFO)
    
    result = build_skeleton_for_project(project_root, output_dir)
    print(f"骨架已生成: {result}")
