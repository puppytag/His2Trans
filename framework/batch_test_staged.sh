#!/bin/bash

################################################################################
# C2Rust 阶段同步批量项目测试脚本
# 
# 实现阶段同步：所有项目在同一个阶段完成后，再一起进入下一个阶段
# - 阶段1（需要vLLM）：依赖分析 + 骨架翻译
# - 阶段2（需要GPU）：签名匹配 + BM25 + Jina Reranker
# - 阶段3（需要vLLM）：函数体翻译 + 测试 + 修复
# 
# 这样vLLM只需要启动两次（阶段1开始和阶段3开始），而不是每个项目启动一次
#
# 使用方法：
#   bash batch_test_staged.sh [选项]
#
# 可选参数：
#   --incremental              使用增量式翻译模式
#   --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)
#   --rq3-body C0..C6          RQ3.3 函数体消融条件开关（知识通道 + 修复轮数；不指定则保持默认行为）
#                             C0=N/N/0, C1=N/N/3, C2=P/P/0, C3=P/P/3, C4=O/P/3, C5=P/O/3, C6=O/O/3
#                             其中 N/P/O=none/predicted/oracle（API_Mapping / Partial-Idiom / max-repair）
#   --oracle-knowledge-root DIR  Oracle 知识根目录（默认: ./oracle_rust；目录结构: <root>/<project>/*.json）
#   --oracle-auto-extract true|false
#                             Oracle 条件下是否允许自动抽取并缓存（复用既有知识抽取代码）
#   --llm NAME                 LLM 名称 (默认: qwen3_coder)
#   --suite ohos|oss           切换项目集合（默认: ohos；oss 为开源小项目集合）
#   --skip PROJECT1,PROJECT2   跳过指定的项目（用逗号分隔）
#   --only PROJECT1,PROJECT2   只运行指定的项目（用逗号分隔）
#   --max-parallel N           每个阶段的最大并行数（默认: 4）
#   --max-parallel-workers N   单项目 LLM 翻译阶段最大并行数（默认: 4；外部 API 建议 <= 4）
#   --run-dir NAME             本次运行输出目录名（ohos: translation_outputs/NAME/；oss: translation_outputs/Our/NAME/；包含 results/ 与 intermediate/），用于实验隔离
#   --extra-rag-kb-dir DIRS    额外 RAG 知识库目录（独立可删）。可用逗号/分号/冒号分隔多个目录
#                             （每个目录需包含 knowledge_base.json + bm25_index.pkl）。不指定则不启用
#   --run-rag true|false       是否执行 BM25 + Jina Reranker（默认: false）
#   --use-rag true|false       是否在 LLM prompt 注入 RAG 知识（默认: true）
#   --skeleton-only            只运行阶段1（骨架翻译），用于调试
#   --bindgen-debug            输出更多 bindgen/types 诊断信息（用于定位 stub 原因）
#   --bindgen-debug-keep-files 保留 wrapper/preprocessed 临时文件（调试用，磁盘占用会增加）
#   --jina-workers N           Jina Reranker 并行 worker 数（默认: 4；仅 --jina-parallel 生效）
#   --help, -h                 显示帮助信息
################################################################################

set -e
set -o pipefail

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 输出目录布局（用于消融实验/多次运行隔离）：
# - 同一次运行（一个 run_dir）统一写到 translation_outputs/<run_dir>/
# - run_dir 下再分为：
#   - results/       ：最终结果（汇总报告、评估报告、便于对比的产物）
#   - intermediate/  ：所有中间产物（每项目 workspace、日志、缓存等）
# - 项目目录名不再追加日期，避免跨午夜导致阶段间读写不同目录
# 可通过环境变量或参数覆盖：
#   RUN_DIR_NAME=my_exp bash batch_test_staged.sh ...
#   bash batch_test_staged.sh --run-dir my_exp ...
OUTPUTS_BASE_DIR="$SCRIPT_DIR/translation_outputs"
RUN_DIR_NAME="${RUN_DIR_NAME:-$(date +%Y%m%d_%H%M%S)}"
RUN_ROOT_DIR=""
RUN_RESULTS_DIR=""
RUN_INTERMEDIATE_DIR=""

# 默认配置
# Truth-mode（真值优先）：尽量不做派生层修补（types 后处理/LLM fallback/防御性补全等）。
# - 1: 开启
# - 0: 关闭（默认；更“可编译优先”的行为）
C2R_TRUTH_MODE="${C2R_TRUTH_MODE:-0}"
LLM_NAME="qwen3_coder"
USE_INCREMENTAL=false
USE_LAYERED_SKELETON="${USE_LAYERED_SKELETON:-$([[ \"$C2R_TRUTH_MODE\" != \"0\" ]] && echo true || echo false)}"  # 分层骨架构建模式（Truth-mode=1 时默认启用）
USE_BINDGEN=true            # 使用 bindgen 生成类型（仅分层模式）
USE_LLM_SIGNATURES=false    # 使用 LLM 翻译签名（仅分层模式），默认 false（使用 TypeMapper）
USE_LLM_TYPE_MAPPER=false   # 使用 LLMTypeMapper（TypeMapper + LLM 验证），默认 false
USE_LLM_SIG_REFINER=$([[ "$C2R_TRUTH_MODE" != "0" ]] && echo false || echo true)    # Truth-mode 默认关闭（避免 LLM 介入骨架真值层）
USE_RULE_FIX=$([[ "$C2R_TRUTH_MODE" != "0" ]] && echo false || echo true)           # Truth-mode 默认关闭（避免规则修补影响真值）
MAX_REPAIR=5
PROJECT_SUITE="${PROJECT_SUITE:-ohos}"
SKIP_PROJECTS=""
ONLY_PROJECTS=""
# vLLM 请求超时（秒）：OpenAI 兼容客户端的网络超时
# - generate/generation.py 优先读取 VLLM_REQUEST_TIMEOUT，其次读取 VLLM_TIMEOUT
# - batch_test_staged.sh 将默认值设为 600s，以减少大函数/排队时的超时误判
VLLM_REQUEST_TIMEOUT="${VLLM_REQUEST_TIMEOUT:-${VLLM_TIMEOUT:-600}}"
VLLM_TIMEOUT="${VLLM_TIMEOUT:-$VLLM_REQUEST_TIMEOUT}"
# 根据 CPU 核心数自动设置并行度（最大 8，避免内存压力）
MAX_PARALLEL=${MAX_PARALLEL:-$(nproc --ignore=2 2>/dev/null || echo 4)}
MAX_PARALLEL=$((MAX_PARALLEL > 8 ? 8 : MAX_PARALLEL))
# 单项目（每层）LLM 翻译并行度；过大容易触发外部 API 429
MAX_PARALLEL_WORKERS="${MAX_PARALLEL_WORKERS:-4}"
RUN_RAG=false  # 默认跳过 BM25 和 Jina Reranker（如果知识库和项目不变）
USE_RAG_CONTEXT=true  # 是否在 LLM prompt 注入 RAG 知识（默认 true；即便 RUN_RAG=false 也可复用已有 reranked_results）
# RQ3.2: RAG top-k（仅当显式设置时生效；默认保持旧行为）
RAG_TOPK=""
# RQ3.3: 函数体消融（知识通道 + 修复轮数）。不设置则保持当前默认行为不变。
RQ3_BODY_CONDITION=""
MAX_REPAIR_EXPLICIT=false
# Oracle 知识根目录（用于 C4/C5/C6），目录结构约定：<root>/<project>/*.json
ORACLE_KNOWLEDGE_ROOT="${C2R_ORACLE_KNOWLEDGE_ROOT:-$SCRIPT_DIR/oracle_rust}"
# Oracle 条件下是否允许自动抽取并缓存（默认关闭；若使用 --rq3-body 且涉及 oracle，则默认开启）
ORACLE_AUTO_EXTRACT=false
# RQ3.x: 复用历史 Stage1 产物（依赖分析 + 骨架翻译），避免重复生成骨架
# - 传入一个 run_dir 根目录（包含 intermediate/），或直接传入 intermediate/ 目录
REUSE_STAGE1_FROM=""
# 是否跳过“知识沉淀（Learned KB）”
# - 默认不跳过：保持原流程（post-run 自动生成 Learned KB）
SKIP_LEARNED_KB=false
SKELETON_ONLY=false  # 仅运行骨架翻译部分（阶段1），用于调试
JINA_SERIAL_MODE=true  # Jina Reranker 串行模式（避免 OOM，默认开启）
JINA_WORKERS="${JINA_WORKERS:-4}"  # Jina Reranker 并行 worker 数（仅 --jina-parallel 生效）
USE_SELF_HEALING="${USE_SELF_HEALING:-$([[ \"$C2R_TRUTH_MODE\" != \"0\" ]] && echo false || echo true)}"  # AI 原生自愈循环（Truth-mode 默认关闭）
USE_LIBCLANG=false     # 使用 libclang 提取函数（更准确，默认关闭）
# bindgen/types 诊断输出（可选）
BINDGEN_DEBUG="${C2R_BINDGEN_DEBUG:-0}"
BINDGEN_DEBUG_KEEP_FILES="${C2R_BINDGEN_DEBUG_KEEP_FILES:-0}"
# TU 闭包门禁：若某项目存在 compile_commands 缺失/预处理失败，则跳过后续阶段。
# 开源最小输入通常不包含 OpenHarmony 的 compile_commands registry，因此默认关闭（=0），
# 需要时可通过环境变量显式开启：C2R_REQUIRE_TU_CLOSURE=1 bash batch_test_staged.sh ...
C2R_REQUIRE_TU_CLOSURE="${C2R_REQUIRE_TU_CLOSURE:-0}"
# 额外 RAG KB（默认不启用；用命令行 --extra-rag-kb-dir 指定）
EXTRA_RAG_KB_DIRS=""

# 推荐的项目列表（默认使用开源最小 OHOS(test5) 子集；--suite oss 时会在后续覆盖）
declare -A RECOMMENDED_PROJECTS=(
    ["shared__541f4e547bdb"]="$SCRIPT_DIR/../data/ohos/source_projects/shared__541f4e547bdb"
    ["shared__12e38ea922f7"]="$SCRIPT_DIR/../data/ohos/source_projects/shared__12e38ea922f7"
    ["host__25c1898e1626"]="$SCRIPT_DIR/../data/ohos/source_projects/host__25c1898e1626"
    ["osal__0bc4f21396ad"]="$SCRIPT_DIR/../data/ohos/source_projects/osal__0bc4f21396ad"
    ["appverify_lite__e5ebe91a98b9"]="$SCRIPT_DIR/../data/ohos/source_projects/appverify_lite__e5ebe91a98b9"
)

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        --incremental)
            USE_INCREMENTAL=true
            shift
            ;;
        --max-repair)
            MAX_REPAIR="$2"
            MAX_REPAIR_EXPLICIT=true
            shift 2
            ;;
        --rq3-body|--rq3-body-condition|--ablation-body)
            if [[ -z "${2:-}" ]]; then
                echo "错误: --rq3-body 需要一个条件参数（C0..C6）" >&2
                exit 2
            fi
            RQ3_BODY_CONDITION="$2"
            shift 2
            ;;
        --oracle-knowledge-root|--oracle-root)
            if [[ -z "${2:-}" ]]; then
                echo "错误: --oracle-knowledge-root 需要一个目录参数" >&2
                exit 2
            fi
            ORACLE_KNOWLEDGE_ROOT="$2"
            shift 2
            ;;
        --oracle-auto-extract)
            if [[ "${2:-}" == "true" ]] || [[ "${2:-}" == "false" ]]; then
                ORACLE_AUTO_EXTRACT="$2"
                shift 2
            else
                echo "错误: --oracle-auto-extract 需要 true|false，但得到了: ${2:-}" >&2
                exit 2
            fi
            ;;
        --llm)
            LLM_NAME="$2"
            shift 2
            ;;
        --suite|--project-suite)
            PROJECT_SUITE="$2"
            shift 2
            ;;
        --skip)
            SKIP_PROJECTS="$2"
            shift 2
            ;;
        --only)
            ONLY_PROJECTS="$2"
            shift 2
            ;;
        --max-parallel)
            MAX_PARALLEL="$2"
            shift 2
            ;;
		--max-parallel-workers)
			MAX_PARALLEL_WORKERS="$2"
			shift 2
			;;
        --run-dir)
            RUN_DIR_NAME="$2"
            shift 2
            ;;
        --extra-rag-kb-dir)
            EXTRA_RAG_KB_DIRS="$2"
            shift 2
            ;;
        --run-rag)
            if [[ "$2" == "true" ]] || [[ "$2" == "false" ]]; then
                RUN_RAG="$2"
                shift 2
            else
                # 兼容旧的方式：只有 --run-rag 时默认为 true
                RUN_RAG=true
                shift
            fi
            ;;
        --use-rag)
            if [[ "$2" == "true" ]] || [[ "$2" == "false" ]]; then
                USE_RAG_CONTEXT="$2"
                shift 2
            else
                # 兼容旧的方式：只有 --use-rag 时默认为 true
                USE_RAG_CONTEXT=true
                shift
            fi
            ;;
        --rag-topk)
            if [[ -z "${2:-}" ]]; then
                echo "错误: --rag-topk 需要一个正整数参数（例如 3/5/10）" >&2
                exit 2
            fi
            if [[ "$2" =~ ^[0-9]+$ ]] && [[ "$2" -gt 0 ]]; then
                RAG_TOPK="$2"
                shift 2
            else
                echo "错误: --rag-topk 需要正整数，但得到了: $2" >&2
                exit 2
            fi
            ;;
        --reuse-stage1-from)
            if [[ -z "${2:-}" ]]; then
                echo "错误: --reuse-stage1-from 需要一个目录参数（run_dir 或 intermediate 目录）" >&2
                exit 2
            fi
            REUSE_STAGE1_FROM="$2"
            shift 2
            ;;
        --skip-learned-kb|--no-learned-kb)
            SKIP_LEARNED_KB=true
            shift
            ;;
        --vllm-global-limit|--vllm-concurrent-limit)
            VLLM_CONCURRENT_LIMIT="$2"
            shift 2
            ;;
        --layered)
            USE_LAYERED_SKELETON=true
            shift
            ;;
        --skeleton-only)
            SKELETON_ONLY=true
            shift
            ;;
        --jina-parallel)
            JINA_SERIAL_MODE=false
            shift
            ;;
        --jina-workers)
            JINA_WORKERS="$2"
            shift 2
            ;;
        --no-bindgen)
            USE_BINDGEN=false
            shift
            ;;
        --no-llm-signatures)
            USE_LLM_SIGNATURES=false
            shift
            ;;
        --use-llm-type-mapper)
            USE_LLM_TYPE_MAPPER=true
            shift
            ;;
        --no-rule-fix)
            USE_RULE_FIX=false
            shift
            ;;
        --no-self-healing)
            USE_SELF_HEALING=false
            shift
            ;;
        --use-libclang)
            USE_LIBCLANG=true
            shift
            ;;
        --bindgen-debug)
            BINDGEN_DEBUG=1
            shift
            ;;
        --bindgen-debug-keep-files)
            BINDGEN_DEBUG=1
            BINDGEN_DEBUG_KEEP_FILES=1
            shift
            ;;
        --help|-h)
            echo "C2Rust 阶段同步批量项目测试脚本"
            echo ""
            echo "使用方法："
            echo "  bash batch_test_staged.sh [选项]"
            echo ""
            echo "可选参数："
            echo "  --incremental              使用增量式翻译模式"
            echo "  --layered                  使用分层骨架构建模式（bindgen + tree-sitter）"
            echo "  --no-bindgen               禁用 bindgen 类型生成（仅分层模式）"
            echo "  --no-llm-signatures        禁用 LLM 签名翻译（仅分层模式）"
            echo "  --use-llm-type-mapper      启用 LLMTypeMapper（TypeMapper + LLM 验证）"
            echo "  --no-rule-fix              禁用规则修复（直接走增量修复/LLM 修复）"
            echo "  --no-self-healing          禁用 AI 原生自愈循环（可以不启动 vLLM）"
            echo "  --use-libclang             使用 libclang 提取函数（更准确，能处理复杂宏和属性）"
            echo "  --bindgen-debug            输出更多 bindgen/types 诊断信息（用于定位 stub 原因）"
            echo "  --bindgen-debug-keep-files 保留 wrapper/preprocessed 临时文件（调试用，磁盘占用会增加）"
            echo "  --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)"
            echo "  --rq3-body C0..C6          RQ3.3 函数体消融条件（知识通道 + 修复轮数；不指定则保持默认行为）"
            echo "                             C0=N/N/0, C1=N/N/3, C2=P/P/0, C3=P/P/3, C4=O/P/3, C5=P/O/3, C6=O/O/3"
            echo "  --oracle-knowledge-root DIR Oracle 知识根目录（默认: ./oracle_rust；目录结构: <root>/<project>/*.json）"
            echo "  --oracle-auto-extract true|false"
            echo "                             Oracle 条件下是否允许自动抽取并缓存（默认: false；若使用 --rq3-body 且涉及 oracle，则默认开启）"
            echo "  --llm NAME                 LLM 名称 (默认: qwen3_coder)"
            echo "  --suite ohos|oss           切换项目集合（默认: ohos）"
            echo "                             - ohos: SelfContained/OpenHarmony 模块"
            echo "                             - oss:  ComparisonMethod/Our/oss_projects.sh (开源小项目集合)"
            echo "  --skip PROJECT1,PROJECT2   跳过指定的项目（用逗号分隔）"
            echo "  --only PROJECT1,PROJECT2   只运行指定的项目（用逗号分隔）"
            echo "  --max-parallel N           每个阶段的最大并行数（默认: 4）"
            echo "  --max-parallel-workers N   单项目 LLM 翻译阶段最大并行数（默认: 4；外部 API 建议 <= 4）"
            echo "  --run-dir NAME             本次运行输出目录名（包含 results/ 与 intermediate/）"
            echo "                             - ohos: translation_outputs/NAME/"
            echo "                             - oss : translation_outputs/Our/NAME/"
            echo "  --extra-rag-kb-dir DIRS    额外 RAG 知识库目录（独立可删）。可用逗号/分号/冒号分隔多个目录"
            echo "                             （每个目录需包含 knowledge_base.json + bm25_index.pkl）。不指定则不启用"
            echo "  --run-rag true|false       是否执行 BM25 和 Jina Reranker（默认: false）"
            echo "                             如果知识库和项目不变，可以设置为 false 跳过"
            echo "  --use-rag true|false       是否在 LLM prompt 注入 RAG 知识（默认: true）"
            echo "  --rag-topk N               RAG 注入知识的 top-k（正整数；用于 RQ3.2，例如 3/5/10；"
            echo "                             限制注入的知识条目数=K，而非前 K 个代码对；默认不设置=保持旧行为：API=10/Partial=2）"
            echo "  --reuse-stage1-from DIR    复用历史 Stage1 产物（依赖分析+骨架翻译），避免重复生成骨架"
            echo "                             DIR 可为 run_dir 根目录（包含 intermediate/）或直接为 intermediate 目录"
            echo "  --skip-learned-kb          跳过知识沉淀（Learned KB）生成（默认: 不跳过）"
            echo "  --jina-parallel            Jina Reranker 并行模式（使用智能GPU排队调度）"
            echo "                             默认: 串行模式（一次处理一个项目，避免 OOM）"
            echo "                             并行模式会根据 GPU 内存自动排队，多GPU可同时处理多项目"
            echo "  --jina-workers N           Jina Reranker 并行 worker 数（默认: 4；仅 --jina-parallel 生效）"
            echo "  --vllm-global-limit N      设置 LLM 请求并发上限（VLLM_CONCURRENT_LIMIT=N；vLLM/外部 API 均生效）"
            echo "  --skeleton-only            只运行阶段1（骨架翻译），用于调试骨架生成问题"
            echo "  --help, -h                 显示帮助信息"
            echo ""
            echo "阶段说明："
            echo "  阶段1: 依赖分析 + 骨架翻译（需要vLLM）"
            echo "  阶段2: 签名匹配 + BM25 + Jina Reranker（需要GPU）"
            echo "  阶段3: 函数体翻译 + 测试 + 修复（需要vLLM）"
            echo ""
            echo "分层骨架构建模式 (--layered)："
            echo "  阶段 A: 使用 bindgen 生成类型定义（绝对正确）"
            echo "  阶段 B: 使用 tree-sitter 提取全局/静态变量"
            echo "  阶段 C: 生成函数骨架（仅签名，body 为 unimplemented!()）"
            echo ""
            echo "推荐的项目："
            if [[ "$PROJECT_SUITE" == "oss" ]]; then
                OSS_PROJECTS_FILE="$SCRIPT_DIR/ComparisonMethod/Our/oss_projects.sh"
                if [[ -f "$OSS_PROJECTS_FILE" ]]; then
                    # shellcheck source=/dev/null
                    source "$OSS_PROJECTS_FILE"
                    if [[ ${#OSS_PROJECTS[@]} -eq 0 ]]; then
                        echo "  (OSS_PROJECTS 为空: $OSS_PROJECTS_FILE)"
                    else
                        for proj_name in "${!OSS_PROJECTS[@]}"; do
                            echo "  - $proj_name"
                        done
                    fi
                else
                    echo "  (未找到开源项目集合: $OSS_PROJECTS_FILE)"
                fi
            else
                for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
                    echo "  - $proj_name"
                done
            fi
            exit 0
            ;;
        *)
            echo "错误: 未知参数: $1"
            echo "使用 --help 查看帮助信息"
            exit 1
            ;;
    esac
done

# 解析参数后再次限制并行度（防止 --max-parallel 覆盖上面的默认 cap）
if [[ "$MAX_PARALLEL" =~ ^[0-9]+$ ]] && [[ $MAX_PARALLEL -gt 8 ]]; then
    MAX_PARALLEL=8
fi
if [[ ! "$MAX_PARALLEL_WORKERS" =~ ^[0-9]+$ ]] || [[ $MAX_PARALLEL_WORKERS -lt 1 ]]; then
    MAX_PARALLEL_WORKERS=4
fi

# ============================================================================
# RQ3.3: 函数体消融（知识通道 + 修复轮数）
# - 仅当显式传入 --rq3-body 时生效；否则不改变默认行为
# ============================================================================
RQ3_API_MODE=""
RQ3_PARTIAL_MODE=""
RQ3_MAX_REPAIR=""
if [[ -n "${RQ3_BODY_CONDITION:-}" ]]; then
    case "$RQ3_BODY_CONDITION" in
        C0)
            RQ3_API_MODE="none"
            RQ3_PARTIAL_MODE="none"
            RQ3_MAX_REPAIR="0"
            ;;
        C1)
            RQ3_API_MODE="none"
            RQ3_PARTIAL_MODE="none"
            RQ3_MAX_REPAIR="3"
            ;;
        C2)
            RQ3_API_MODE="predicted"
            RQ3_PARTIAL_MODE="predicted"
            RQ3_MAX_REPAIR="0"
            ;;
        C3)
            RQ3_API_MODE="predicted"
            RQ3_PARTIAL_MODE="predicted"
            RQ3_MAX_REPAIR="3"
            ;;
        C4)
            RQ3_API_MODE="oracle"
            RQ3_PARTIAL_MODE="predicted"
            RQ3_MAX_REPAIR="3"
            ;;
        C5)
            RQ3_API_MODE="predicted"
            RQ3_PARTIAL_MODE="oracle"
            RQ3_MAX_REPAIR="3"
            ;;
        C6)
            RQ3_API_MODE="oracle"
            RQ3_PARTIAL_MODE="oracle"
            RQ3_MAX_REPAIR="3"
            ;;
        *)
            echo "错误: --rq3-body 仅支持 C0..C6，但得到了: $RQ3_BODY_CONDITION" >&2
            exit 2
            ;;
    esac

    # 若条件涉及 oracle，但用户未显式指定 --oracle-auto-extract，则默认开启自动抽取（复用既有知识抽取代码）
    if [[ "$ORACLE_AUTO_EXTRACT" == "false" ]] && ([[ "$RQ3_API_MODE" == "oracle" ]] || [[ "$RQ3_PARTIAL_MODE" == "oracle" ]]); then
        ORACLE_AUTO_EXTRACT=true
    fi

    # 若用户未显式指定 --max-repair，则按条件默认值覆盖；否则保持用户输入不变
    if [[ "$MAX_REPAIR_EXPLICIT" != "true" ]] && [[ -n "${RQ3_MAX_REPAIR:-}" ]]; then
        MAX_REPAIR="$RQ3_MAX_REPAIR"
    fi

    export C2R_RQ3_BODY_CONDITION="$RQ3_BODY_CONDITION"
    export C2R_RQ3_API_MAPPING_MODE="$RQ3_API_MODE"
    export C2R_RQ3_PARTIAL_IDIOM_MODE="$RQ3_PARTIAL_MODE"
    export C2R_ORACLE_KNOWLEDGE_ROOT="$ORACLE_KNOWLEDGE_ROOT"
    export C2R_ORACLE_AUTO_EXTRACT=$([[ "$ORACLE_AUTO_EXTRACT" == "true" ]] && echo 1 || echo 0)
fi

# ============================================================================
# 项目集合切换（ohos vs oss）
# ============================================================================
case "$PROJECT_SUITE" in
    ohos)
        # 默认：保持原行为（输出写到 translation_outputs/<run_dir>/）
        ;;
    oss)
        # 开源小项目：输出隔离到 translation_outputs/Our/<run_dir>/
        OUTPUTS_BASE_DIR="$SCRIPT_DIR/translation_outputs/Our"

        OSS_PROJECTS_FILE="$SCRIPT_DIR/ComparisonMethod/Our/oss_projects.sh"
        if [[ ! -f "$OSS_PROJECTS_FILE" ]]; then
            echo "错误: 未找到开源项目集合定义文件: $OSS_PROJECTS_FILE" >&2
            echo "请先创建/检查 ComparisonMethod/Our/oss_projects.sh" >&2
            exit 2
        fi
        # shellcheck source=/dev/null
        source "$OSS_PROJECTS_FILE"
        if [[ ${#OSS_PROJECTS[@]} -eq 0 ]]; then
            echo "错误: OSS_PROJECTS 为空（$OSS_PROJECTS_FILE）" >&2
            exit 2
        fi

        # Replace the default OHOS project map with OSS map (keep downstream code untouched).
        unset RECOMMENDED_PROJECTS
        declare -A RECOMMENDED_PROJECTS=()
        for proj_name in "${!OSS_PROJECTS[@]}"; do
            RECOMMENDED_PROJECTS["$proj_name"]="${OSS_PROJECTS[$proj_name]}"
        done
        ;;
    *)
        echo "错误: --suite 取值必须是 ohos 或 oss，但得到了: $PROJECT_SUITE" >&2
        exit 2
        ;;
esac

# 是否注入 RAG 上下文（由 translate_function.py / incremental_translate.py 消费）
export C2R_USE_RAG_CONTEXT="${USE_RAG_CONTEXT}"
# RQ3.2: RAG top-k（仅当显式传入 --rag-topk 时导出；默认不干预）
if [[ -n "${RAG_TOPK:-}" ]]; then
    export C2R_RAG_TOPK="${RAG_TOPK}"
fi

# 颜色输出（必须在使用 print_* 函数之前定义）
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_section() {
    echo ""
    echo -e "${BLUE}============================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}============================================================${NC}"
    echo ""
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# ========== ★★★ 资源检查和信息展示 ★★★ ==========
check_resources() {
    print_section "资源检查"

    # 统计项目数量
    local project_count=0
    if [[ -n "$ONLY_PROJECTS" ]]; then
        IFS=',' read -ra PROJ_ARRAY <<< "$ONLY_PROJECTS"
        project_count=${#PROJ_ARRAY[@]}
    else
        project_count=${#RECOMMENDED_PROJECTS[@]}
    fi

    # 检查内存（仅作为信息展示，不强制修改参数）
    if command -v free >/dev/null 2>&1; then
        local available_mem_gb
        available_mem_gb=$(free -g | awk '/^Mem:/ {print $7}')
        local total_mem_gb
        total_mem_gb=$(free -g | awk '/^Mem:/ {print $2}')
        print_info "系统内存: 总计 ${total_mem_gb}GB, 可用 ${available_mem_gb}GB"

        # 仅警告，不自动修改
        local estimated_mem_gb=$((MAX_PARALLEL * 2))  # libclang 约 2GB/进程
        if [[ $available_mem_gb -lt $estimated_mem_gb ]]; then
            print_warning "注意: 当前并行度 ($MAX_PARALLEL) 预计需要约 ${estimated_mem_gb}GB 内存"
            print_warning "      可用内存 ${available_mem_gb}GB 可能不足，如遇问题请降低 --max-parallel"
        fi
    fi

    # 显示 Jina Reranker 模式信息
    if [[ "$RUN_RAG" == "true" ]]; then
        if [[ "$JINA_SERIAL_MODE" == "true" ]]; then
            print_info "Jina Reranker: 串行模式"
        else
            print_info "Jina Reranker: 并行模式（GPU 排队机制已启用）"
        fi
    fi

    # 显示运行配置
    echo ""
    print_info "运行配置："
    print_info "  - MAX_PARALLEL: $MAX_PARALLEL"
    print_info "  - USE_LIBCLANG: $USE_LIBCLANG (CPU 密集型)"
    print_info "  - RUN_RAG: $RUN_RAG"
    print_info "  - USE_RAG_CONTEXT: $USE_RAG_CONTEXT"
    if [[ "$RUN_RAG" == "true" ]]; then
        print_info "  - JINA_SERIAL_MODE: $JINA_SERIAL_MODE"
    fi
    print_info "  - 项目数量: $project_count"
    echo ""

    # 说明阶段同步模式的资源隔离
    print_info "阶段同步模式资源隔离："
    print_info "  - 阶段1/3: vLLM 独占 GPU"
    print_info "  - 阶段2.5: Jina Reranker 独占 GPU（vLLM 已关闭）"
    print_info "  - libclang: 仅使用 CPU，不占用 GPU"
    echo ""
}

# 执行资源检查
check_resources

# 初始化本次运行的输出根目录（所有项目共享同一个 RUN 目录）
RUN_ROOT_DIR="$OUTPUTS_BASE_DIR/$RUN_DIR_NAME"
RUN_RESULTS_DIR="$RUN_ROOT_DIR/results"
RUN_INTERMEDIATE_DIR="$RUN_ROOT_DIR/intermediate"
mkdir -p "$RUN_RESULTS_DIR" "$RUN_INTERMEDIATE_DIR"
# shared：不随 run 清理的公共缓存/产物（例如 OpenHarmony compile_commands 提取结果）
mkdir -p "$OUTPUTS_BASE_DIR/shared/ohos_profiles/compile_commands" "$OUTPUTS_BASE_DIR/shared/ohos_profiles/projects"

# 检查项目路径是否存在
check_project_paths() {
    local missing=0
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
        if [[ ! -d "$proj_path" ]]; then
            print_warning "项目路径不存在: $proj_name -> $proj_path"
            missing=$((missing + 1))
        fi
    done

    if [[ $missing -gt 0 ]]; then
        print_error "有 $missing 个项目路径不存在"
        exit 1
    fi
}

# 检查是否应该跳过项目
should_skip_project() {
    local proj_name="$1"
    
    # 如果指定了 --only，只运行指定的项目
    if [[ -n "$ONLY_PROJECTS" ]]; then
        IFS=',' read -ra ONLY_ARRAY <<< "$ONLY_PROJECTS"
        local found=false
        for item in "${ONLY_ARRAY[@]}"; do
            if [[ "$item" == "$proj_name" ]]; then
                found=true
                break
            fi
        done
        if [[ "$found" == false ]]; then
            return 0  # 跳过
        fi
    fi
    
    # 如果指定了 --skip，跳过指定的项目
    if [[ -n "$SKIP_PROJECTS" ]]; then
        IFS=',' read -ra SKIP_ARRAY <<< "$SKIP_PROJECTS"
        for item in "${SKIP_ARRAY[@]}"; do
            if [[ "$item" == "$proj_name" ]]; then
                return 0  # 跳过
            fi
        done
    fi
    
    return 1  # 不跳过
}

# vLLM 管理函数
VLLM_URL="${VLLM_URL:-http://localhost:8000/v1}"
LOG_DIR="$RUN_INTERMEDIATE_DIR/logs"
VIRTUAL_ENV_NAME="${VIRTUAL_ENV_NAME:-c2r_frame}"
RERANKER_ENV_NAME="${RERANKER_ENV_NAME:-jina_reranker_env}"
mkdir -p "$LOG_DIR"

# 激活 conda 环境
activate_conda_env() {
    local target_env="${1:-$VIRTUAL_ENV_NAME}"
    if command -v conda >/dev/null 2>&1; then
        # Conda plugins occasionally crash (e.g. CUDA virtual package probing with permission issues).
        # Keep this script robust by disabling plugins unless user explicitly overrides.
        export CONDA_NO_PLUGINS="${CONDA_NO_PLUGINS:-true}"

        eval "$(conda shell.bash hook 2>/dev/null)" || {
            if [ -f "$(conda info --base)/etc/profile.d/conda.sh" ]; then
                source "$(conda info --base)/etc/profile.d/conda.sh"
            fi
        }
        
        # Don't call `conda env list` here: it's slow and can crash when plugins misbehave.
        conda activate "$target_env" >/dev/null 2>&1 || true
        if [ "$CONDA_DEFAULT_ENV" = "$target_env" ]; then
            return 0
        fi
    fi
    return 1
}

start_vllm() {
    # vLLM 只在语义评估和知识沉淀时启动，由调用方控制
    # （阶段4语义评估和知识沉淀必须使用本地 vLLM）

    # Observability for callers:
    # - "already_running": vLLM was detected running before this function started anything
    # - "started": this function started vLLM
    # - "failed": start attempt failed
    VLLM_START_ACTION="failed"

    if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
        print_info "vLLM 已在运行: $VLLM_URL"
        VLLM_START_ACTION="already_running"
        return 0
    fi

    # Fast preflight: if CUDA is unavailable, vLLM will fail with "Failed to infer device type".
    # This avoids waiting the full MAX_WAIT when vLLM crashes immediately.
    if command -v python >/dev/null 2>&1; then
        if ! python - <<'PY' >/dev/null 2>&1
import torch
import sys
sys.exit(0 if torch.cuda.is_available() else 1)
PY
        then
            print_warning "检测到 torch.cuda 不可用：当前环境可能没有可用 GPU（或 NVIDIA 驱动/NVML 不可用）"
            if command -v nvidia-smi >/dev/null 2>&1; then
                print_info "nvidia-smi 检查（可能失败也正常，用于诊断）："
                (nvidia-smi -L 2>&1 | sed 's/^/  /') || true
            fi
            print_warning "vLLM 可能无法启动。如需使用 vLLM，请在可用 GPU 的节点/环境中运行，或先修复驱动/权限。"
        fi
    fi
    
	    DEPLOY_VLLM_SCRIPT="${DEPLOY_VLLM_SCRIPT:-}"
		    if [ -z "$DEPLOY_VLLM_SCRIPT" ]; then
		        if [ -f "$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_no_quantization.sh" ]; then
		            DEPLOY_VLLM_SCRIPT="$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_no_quantization.sh"
		        elif [ -f "$SCRIPT_DIR/qwen3_coder/deploy_no_quantization.sh" ]; then
		            DEPLOY_VLLM_SCRIPT="$SCRIPT_DIR/qwen3_coder/deploy_no_quantization.sh"
		        else
		            print_error "未找到 deploy_no_quantization.sh，请设置 DEPLOY_VLLM_SCRIPT"
		            return 1
		        fi
		    fi

	    # Default vLLM env: some deploy scripts treat "unset" as "set to empty", which can crash vLLM.
	    # Keep these defaults conservative and overrideable by the caller.
	    if [ -z "${VLLM_GPU_MEMORY_UTILIZATION:-}" ]; then
	        export VLLM_GPU_MEMORY_UTILIZATION=0.75
	    fi
	    if [ -z "${VLLM_MAX_TOKENS:-}" ]; then
	        # Higher default to reduce truncation for long C->Rust translations.
	        export VLLM_MAX_TOKENS=16384
	    fi
	    if [ -z "${VLLM_LOGGING_LEVEL:-}" ]; then
	        export VLLM_LOGGING_LEVEL=INFO
	    fi
	    
	    print_info "启动 vLLM: $DEPLOY_VLLM_SCRIPT"
    
    VLLM_LOG="$LOG_DIR/vllm_$(date +%Y%m%d_%H%M%S).log"
    TEMP_START_SCRIPT=$(mktemp)
    
cat > "$TEMP_START_SCRIPT" << EOFSCRIPT
#!/bin/bash
VIRTUAL_ENV_NAME="${VIRTUAL_ENV_NAME}"
DEPLOY_SCRIPT="\$1"
LOG_FILE="\$2"

if command -v conda >/dev/null 2>&1; then
    export CONDA_NO_PLUGINS="\${CONDA_NO_PLUGINS:-true}"
    eval "\$(conda shell.bash hook 2>/dev/null)" || {
        if [ -f "\$(conda info --base)/etc/profile.d/conda.sh" ]; then
            source "\$(conda info --base)/etc/profile.d/conda.sh"
        fi
    }
    
    conda activate "\${VIRTUAL_ENV_NAME}" >/dev/null 2>&1 || true
    if [ "\$CONDA_DEFAULT_ENV" = "\${VIRTUAL_ENV_NAME}" ]; then
        echo "[vLLM启动] 已激活 conda 环境: \${VIRTUAL_ENV_NAME}" >> "\${LOG_FILE}" 2>&1
    fi
fi

# 透传关键 vLLM 启动参数（即使调用方只设置了 shell 变量但未 export）
export CUDA_VISIBLE_DEVICES="${CUDA_VISIBLE_DEVICES}"
export VLLM_TENSOR_PARALLEL_SIZE="${VLLM_TENSOR_PARALLEL_SIZE}"
export VLLM_GPU_MEMORY_UTILIZATION="${VLLM_GPU_MEMORY_UTILIZATION}"
	# vLLM will crash if VLLM_LOGGING_LEVEL is set but empty (Unknown level: '').
	# Some deploy scripts may export an empty value when unset; force a safe default.
	if [ -n "${VLLM_LOGGING_LEVEL}" ]; then
	    export VLLM_LOGGING_LEVEL="${VLLM_LOGGING_LEVEL}"
	else
	    export VLLM_LOGGING_LEVEL="INFO"
	fi

bash "\${DEPLOY_SCRIPT}" >> "\${LOG_FILE}" 2>&1
EOFSCRIPT

    chmod +x "$TEMP_START_SCRIPT"
    nohup bash "$TEMP_START_SCRIPT" "$DEPLOY_VLLM_SCRIPT" "$VLLM_LOG" >/dev/null 2>&1 &
    VLLM_PID=$!
    
    (sleep 10 && rm -f "$TEMP_START_SCRIPT") &
    
    print_info "vLLM 启动中 (pid=$VLLM_PID)，日志: $VLLM_LOG"
    MAX_WAIT=1200  # 增加到 10 分钟（大模型可能需要更长时间）
    WAITED=0
    until curl -s "$VLLM_URL/models" >/dev/null 2>&1; do
        # If the process died, fail fast with log tail.
        if ! kill -0 "$VLLM_PID" >/dev/null 2>&1; then
            print_error "vLLM 进程已退出（pid=$VLLM_PID），无法就绪。日志: $VLLM_LOG"
            if [ -f "$VLLM_LOG" ] && [ -s "$VLLM_LOG" ]; then
                print_info "vLLM 日志末尾（最近 60 行）："
                tail -60 "$VLLM_LOG" 2>/dev/null | while read line; do
                    echo "    $line"
                done
            fi
            print_info "建议: 设置 VLLM_LOGGING_LEVEL=DEBUG 后重试，以获得更详细的 device/platform 探测日志。"
            return 1
        fi
        sleep 2
        WAITED=$((WAITED+2))
        # 每 20 秒显示一次进度
        if [ $((WAITED % 20)) -eq 0 ]; then
            print_info "vLLM 启动中... (已等待 ${WAITED}s / ${MAX_WAIT}s)"
            # 显示日志最后几行（如果有）
            if [ -f "$VLLM_LOG" ] && [ -s "$VLLM_LOG" ]; then
                tail -2 "$VLLM_LOG" 2>/dev/null | while read line; do
                    echo "    $line"
                done
            fi
        fi
        if [ $WAITED -ge $MAX_WAIT ]; then
            print_error "vLLM 启动超时（>$MAX_WAIT 秒）。日志: $VLLM_LOG"
            print_info "提示: 如果是首次启动或模型很大，可能需要更长时间。请检查日志: $VLLM_LOG"
            return 1
        fi
	    done
	    print_success "vLLM 已就绪: $VLLM_URL"
	    VLLM_START_ACTION="started"
	    return 0
	}

stop_vllm() {
    # vLLM 只在语义评估和知识沉淀完成后关闭
    print_info "正在关闭 vLLM 服务..."
    pkill -f "vllm.entrypoints.openai.api_server" >/dev/null 2>&1 || true
    sleep 2
    print_success "vLLM 已关闭"
}

# ---------------------------------------------------------------------------
# 统计：类型骨架是否降级为 stub
# ---------------------------------------------------------------------------

_get_latest_workspace_dir_for_project() {
    local proj_name="$1"
    # 优先使用本次 RUN 目录（避免跨运行/跨天混淆）
    if [[ -n "${RUN_INTERMEDIATE_DIR:-}" ]] && [[ -d "$RUN_INTERMEDIATE_DIR/$proj_name/workspace" ]]; then
        echo "$RUN_INTERMEDIATE_DIR/$proj_name/workspace"
        return 0
    fi
    # 兼容：旧布局（project dir 直接在 run 根目录下）
    if [[ -n "${RUN_ROOT_DIR:-}" ]] && [[ -d "$RUN_ROOT_DIR/$proj_name/workspace" ]]; then
        echo "$RUN_ROOT_DIR/$proj_name/workspace"
        return 0
    fi
    # 选择最近一次生成的 workspace 目录（按 mtime 排序）
    # 兼容多次运行/跨天运行：不依赖固定日期
    # 注意：必须避免前缀碰撞，例如 proj=tcp 时不能误匹配 tcp_direct_20251219。
    # 我们约定输出目录为 <proj>_YYYYMMDD[...]，因此强制 '_' 后跟数字开头。
    ls -dt "$SCRIPT_DIR/translation_outputs/${proj_name}_"[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]*/workspace 2>/dev/null | head -n 1
}

_read_types_generation_mode() {
    local report_path="$1"
    if [[ -z "$report_path" ]] || [[ ! -f "$report_path" ]]; then
        echo ""
        return 0
    fi
    python3 - <<'PY' "$report_path" 2>/dev/null || true
import json
import sys
path = sys.argv[1]
try:
    with open(path, "r", encoding="utf-8", errors="ignore") as f:
        data = json.load(f)
    mode = data.get("mode") or ""
    if isinstance(mode, str):
        print(mode.strip())
except Exception:
    pass
PY
}

print_stub_projects_summary() {
    local llm_name="$1"
    shift || true
    local projects=("$@")

    local stub_projects=()
    local stub_reports=()

    for proj in "${projects[@]}"; do
        local workspace_dir
        workspace_dir=$(_get_latest_workspace_dir_for_project "$proj")
        if [[ -z "$workspace_dir" ]]; then
            continue
        fi

        # 优先读取 skeletons 下的 report（阶段1产生，最稳定）
        local report_path="$workspace_dir/skeletons/$proj/types_generation_report.json"
        if [[ ! -f "$report_path" ]]; then
            # 兼容：某些流程可能只在 final_projects 下保存 report
            local fallback="$workspace_dir/final_projects/$proj/translate_by_${llm_name}/types_generation_report.json"
            if [[ -f "$fallback" ]]; then
                report_path="$fallback"
            fi
        fi

        local mode
        mode=$(_read_types_generation_mode "$report_path")
        if [[ "$mode" == "stub" ]]; then
            stub_projects+=("$proj")
            stub_reports+=("$report_path")
        fi
    done

    print_section "类型骨架降级为 stub 的项目"
    echo "stub 项目数: ${#stub_projects[@]} / ${#projects[@]}"
    if [[ ${#stub_projects[@]} -eq 0 ]]; then
        echo ""
        return 0
    fi

    # 记录到文件，便于后续分析
    local stamp
    stamp=$(date +%Y%m%d_%H%M%S)
    local out_file="$LOG_DIR/stub_projects_${stamp}.txt"
    {
        echo "# stub projects (mode=stub)"
        echo "# generated_at=$(date -Iseconds)"
        echo "# llm=${llm_name}"
        echo "# count=${#stub_projects[@]}"
        echo ""
        for i in "${!stub_projects[@]}"; do
            echo "${stub_projects[$i]}\t${stub_reports[$i]}"
        done
    } > "$out_file" 2>/dev/null || true

    echo "项目列表（含 report 路径）:"
    for i in "${!stub_projects[@]}"; do
        echo "  - ${stub_projects[$i]}  (${stub_reports[$i]})"
    done
    echo "已保存: $out_file"
    echo ""
}

# ---------------------------------------------------------------------------
# 统计：构建上下文问题（缺头文件/缺宏/多 build profile）
# ---------------------------------------------------------------------------

_read_types_generation_context_flags() {
    local report_path="$1"
    if [[ -z "$report_path" ]] || [[ ! -f "$report_path" ]]; then
        echo ""
        return 0
    fi
    python3 - <<'PY' "$report_path" 2>/dev/null || true
import json
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
try:
    data = json.loads(path.read_text(encoding="utf-8", errors="ignore"))
except Exception:
    sys.exit(0)

mode = (data.get("mode") or "").strip()
success = bool(data.get("success"))
valid = bool(data.get("final_output_valid"))
cc_loaded_val = data.get("compile_commands_loaded")
cc_loaded = bool(cc_loaded_val)
# Backward/forward compatibility:
# - In tu_truth mode, types_generation_report.json may not include compile_commands_loaded in older/newer schemas.
# - Prefer deriving from tu_context_map.json when available.
if (not cc_loaded) and (cc_loaded_val is None) and mode == "tu_truth":
    try:
        tu_truth = data.get("tu_truth") or {}
        tu_map = tu_truth.get("tu_context_map") or data.get("tu_context_map")
        if tu_map:
            p = Path(str(tu_map))
            if p.exists():
                m = json.loads(p.read_text(encoding="utf-8", errors="ignore") or "{}")
                sp = m.get("selected_profile") or {}
                cc_json = sp.get("compile_commands_json") or ""
                if isinstance(cc_json, str) and cc_json.strip():
                    if Path(cc_json).exists():
                        cc_loaded = True
                if not cc_loaded:
                    files = m.get("files") or {}
                    if isinstance(files, dict):
                        for _, v in files.items():
                            if not isinstance(v, dict):
                                continue
                            ce = v.get("compile_commands_entry")
                            if isinstance(ce, dict) and (ce.get("command") or ce.get("arguments") or ce.get("file")):
                                cc_loaded = True
                                break
    except Exception:
        pass
has_multi = bool(data.get("has_multiple_build_profiles"))

attempts = data.get("attempts") or []
missing_files = []
errors = []
for a in attempts:
    for f in (a.get("missing_files") or []):
        if f:
            missing_files.append(str(f))
    err = a.get("error") or ""
    if err:
        errors.append(str(err))

blob = "\n".join(errors)
low = blob.lower()

header_hit = bool(missing_files) or ("file not found" in low) or ("no such file or directory" in low)
macro_hit = bool(
    re.search(
        r"(unknown type name|use of undeclared identifier|function-like macro|macro.*not defined|not defined|implicit declaration|expected\\s+identifier)",
        low,
    )
)

# “需要手动排除”的场景：types 进入 stub/最终输出无效/整体失败
unresolved = (mode == "stub") or (not success) or (not valid)

missing_headers_unresolved = 1 if (unresolved and header_hit) else 0
missing_macros_unresolved = 1 if (unresolved and macro_hit) else 0

preprocessed_mode = 1 if (mode == "clang_preprocessed_bindgen") else 0

sample = ""
if has_multi:
    mps = data.get("multi_build_profiles") or []
    if mps:
        first = mps[0] or {}
        sample = f"{first.get('file','')}|unique_commands={first.get('unique_commands','')}|entries={first.get('entries','')}"

missing_files_str = ",".join(sorted(set(missing_files)))

print(
    "\t".join(
        [
            mode,
            str(missing_headers_unresolved),
            str(missing_macros_unresolved),
            str(int(has_multi)),
            str(preprocessed_mode),
            str(int(cc_loaded)),
            missing_files_str[:2000],
            sample,
        ]
    )
)
PY
}

print_build_context_problem_projects_summary() {
    local llm_name="$1"
    shift || true
    local projects=("$@")

    local missing_header_projects=()
    local missing_header_reports=()
    local missing_header_missing_files=()

    local missing_macro_projects=()
    local missing_macro_reports=()

    local multi_profile_projects=()
    local multi_profile_reports=()
    local multi_profile_samples=()

    local preprocessed_projects=()
    local preprocessed_reports=()

    local no_compile_commands_projects=()
    local no_compile_commands_reports=()

    for proj in "${projects[@]}"; do
        local workspace_dir
        workspace_dir=$(_get_latest_workspace_dir_for_project "$proj")
        if [[ -z "$workspace_dir" ]]; then
            continue
        fi

        # 优先读取 skeletons 下的 report（阶段1产生，最稳定）
        local report_path="$workspace_dir/skeletons/$proj/types_generation_report.json"
        if [[ ! -f "$report_path" ]]; then
            local fallback="$workspace_dir/final_projects/$proj/translate_by_${llm_name}/types_generation_report.json"
            if [[ -f "$fallback" ]]; then
                report_path="$fallback"
            fi
        fi

        if [[ ! -f "$report_path" ]]; then
            continue
        fi

        local flags
        flags=$(_read_types_generation_context_flags "$report_path")
        if [[ -z "$flags" ]]; then
            continue
        fi

        local mode mh mm mp pre cc_loaded miss_files sample
        IFS=$'\t' read -r mode mh mm mp pre cc_loaded miss_files sample <<< "$flags"

        if [[ "$cc_loaded" == "0" ]]; then
            no_compile_commands_projects+=("$proj")
            no_compile_commands_reports+=("$report_path")
        fi

        if [[ "$mh" == "1" ]]; then
            missing_header_projects+=("$proj")
            missing_header_reports+=("$report_path")
            missing_header_missing_files+=("$miss_files")
        fi

        if [[ "$mm" == "1" ]]; then
            missing_macro_projects+=("$proj")
            missing_macro_reports+=("$report_path")
        fi

        if [[ "$mp" == "1" ]]; then
            multi_profile_projects+=("$proj")
            multi_profile_reports+=("$report_path")
            multi_profile_samples+=("$sample")
        fi

        if [[ "$pre" == "1" ]]; then
            preprocessed_projects+=("$proj")
            preprocessed_reports+=("$report_path")
        fi
    done

    print_section "构建上下文问题项目（缺头文件/缺宏/多 build profile）"

    echo "缺头文件（最终未解决）: ${#missing_header_projects[@]} / ${#projects[@]}"
    if [[ ${#missing_header_projects[@]} -gt 0 ]]; then
        for i in "${!missing_header_projects[@]}"; do
            local mf="${missing_header_missing_files[$i]}"
            if [[ -n "$mf" ]]; then
                echo "  - ${missing_header_projects[$i]}  (${missing_header_reports[$i]})  missing=${mf}"
            else
                echo "  - ${missing_header_projects[$i]}  (${missing_header_reports[$i]})"
            fi
        done
    fi
    echo ""

    echo "缺宏/typedef（最终未解决）: ${#missing_macro_projects[@]} / ${#projects[@]}"
    if [[ ${#missing_macro_projects[@]} -gt 0 ]]; then
        for i in "${!missing_macro_projects[@]}"; do
            echo "  - ${missing_macro_projects[$i]}  (${missing_macro_reports[$i]})"
        done
    fi
    echo ""

    echo "多 build profile（同一文件多条命令）: ${#multi_profile_projects[@]} / ${#projects[@]}"
    if [[ ${#multi_profile_projects[@]} -gt 0 ]]; then
        for i in "${!multi_profile_projects[@]}"; do
            local smp="${multi_profile_samples[$i]}"
            if [[ -n "$smp" ]]; then
                echo "  - ${multi_profile_projects[$i]}  (${multi_profile_reports[$i]})  sample=${smp}"
            else
                echo "  - ${multi_profile_projects[$i]}  (${multi_profile_reports[$i]})"
            fi
        done
    fi
    echo ""

    # 额外信息：虽然最终成功，但触发 clang -E（说明宏/条件编译更复杂）
    echo "触发 clang -E 才成功（可选关注）: ${#preprocessed_projects[@]} / ${#projects[@]}"
    if [[ ${#preprocessed_projects[@]} -gt 0 ]]; then
        for i in "${!preprocessed_projects[@]}"; do
            echo "  - ${preprocessed_projects[$i]}  (${preprocessed_reports[$i]})"
        done
    fi
    echo ""

    # 额外信息：compile_commands 未加载（会退化到基于规则的 include 收集）
    echo "compile_commands 未加载（可选关注）: ${#no_compile_commands_projects[@]} / ${#projects[@]}"
    if [[ ${#no_compile_commands_projects[@]} -gt 0 ]]; then
        for i in "${!no_compile_commands_projects[@]}"; do
            echo "  - ${no_compile_commands_projects[$i]}  (${no_compile_commands_reports[$i]})"
        done
    fi
    echo ""

    # 保存到文件，便于后续用 --skip 复用
    local stamp
    stamp=$(date +%Y%m%d_%H%M%S)
    local out_file="$LOG_DIR/build_context_problem_projects_${stamp}.txt"

    # 生成建议的 --skip 列表（去重、保持原 project 顺序）
    declare -A skip_seen=()
    for proj in "${missing_header_projects[@]}" "${missing_macro_projects[@]}" "${multi_profile_projects[@]}"; do
        skip_seen["$proj"]=1
    done
    local skip_projects=()
    for proj in "${projects[@]}"; do
        if [[ -n "${skip_seen[$proj]:-}" ]]; then
            skip_projects+=("$proj")
        fi
    done
    local skip_csv=""
    if [[ ${#skip_projects[@]} -gt 0 ]]; then
        skip_csv=$(IFS=,; echo "${skip_projects[*]}")
    fi

    {
        echo "# build context problem projects"
        echo "# generated_at=$(date -Iseconds)"
        echo "# llm=${llm_name}"
        echo ""
        echo "[missing_headers_unresolved]"
        for i in "${!missing_header_projects[@]}"; do
            echo -e "${missing_header_projects[$i]}\t${missing_header_reports[$i]}\tmissing=${missing_header_missing_files[$i]}"
        done
        echo ""
        echo "[missing_macros_unresolved]"
        for i in "${!missing_macro_projects[@]}"; do
            echo -e "${missing_macro_projects[$i]}\t${missing_macro_reports[$i]}"
        done
        echo ""
        echo "[multi_build_profiles]"
        for i in "${!multi_profile_projects[@]}"; do
            echo -e "${multi_profile_projects[$i]}\t${multi_profile_reports[$i]}\tsample=${multi_profile_samples[$i]}"
        done
        echo ""
        echo "[preprocessed_bindgen_success]"
        for i in "${!preprocessed_projects[@]}"; do
            echo -e "${preprocessed_projects[$i]}\t${preprocessed_reports[$i]}"
        done
        echo ""
        echo "[compile_commands_not_loaded]"
        for i in "${!no_compile_commands_projects[@]}"; do
            echo -e "${no_compile_commands_projects[$i]}\t${no_compile_commands_reports[$i]}"
        done
        echo ""
        if [[ -n "$skip_csv" ]]; then
            echo "[suggested_skip_csv]"
            echo "$skip_csv"
        fi
    } > "$out_file" 2>/dev/null || true

    if [[ -n "$skip_csv" ]]; then
        echo "建议 --skip: $skip_csv"
    else
        echo "建议 --skip: (无)"
    fi
    echo "已保存: $out_file"
    echo ""
}

# 准备项目的输出目录（一次运行统一写入 RUN_INTERMEDIATE_DIR）
prepare_project_output_dir() {
    local proj_name="$1"
    local proj_path="$2"
    
    # 使用 translation_outputs/<run_dir>/intermediate/<project>/workspace
    local output_dir="$RUN_INTERMEDIATE_DIR/${proj_name}"
    local workspace_dir="$output_dir/workspace"
    
    # 创建目录结构
    mkdir -p "$workspace_dir/projects"
    mkdir -p "$workspace_dir/skeletons"
    mkdir -p "$workspace_dir/source_skeletons"
    mkdir -p "$workspace_dir/extracted"
    mkdir -p "$workspace_dir/translated"
    mkdir -p "$workspace_dir/test_results"
    mkdir -p "$workspace_dir/repair_results"
    mkdir -p "$workspace_dir/signature_matches"
    mkdir -p "$workspace_dir/rag/elastic_search_results"
    mkdir -p "$workspace_dir/rag/reranked_results"
    mkdir -p "$workspace_dir/c_source"  # ★★★ 新增：存放原始 C 源代码副本 ★★★
    mkdir -p "$output_dir/logs"
    
    # 创建项目软链接
    if [ ! -e "$workspace_dir/projects/$proj_name" ]; then
        ln -sf "$proj_path" "$workspace_dir/projects/$proj_name"
    fi
    
    # ★★★ 新增：复制原始 C 源代码到 c_source 目录 ★★★
    # 只复制 .c, .cpp, .h, .hpp 等源文件，不复制编译产物
    local c_source_dir="$workspace_dir/c_source/$proj_name"
    if [ ! -d "$c_source_dir" ] && [ -d "$proj_path" ]; then
        mkdir -p "$c_source_dir"
        # 使用 rsync 复制源文件（排除编译产物和隐藏文件）
        if command -v rsync >/dev/null 2>&1; then
            rsync -a --include='*/' \
                --include='*.c' --include='*.cpp' --include='*.cc' --include='*.cxx' \
                --include='*.h' --include='*.hpp' --include='*.hh' --include='*.hxx' \
                --include='*.gn' --include='*.gni' --include='BUILD.gn' \
                --include='Makefile' --include='CMakeLists.txt' \
                --exclude='*.o' --exclude='*.a' --exclude='*.so' \
                --exclude='.git' --exclude='build' --exclude='out' \
                --exclude='*' \
                "$proj_path/" "$c_source_dir/" 2>/dev/null || true
        else
            # 备选方案：使用 find + cp
            find "$proj_path" -type f \( \
                -name "*.c" -o -name "*.cpp" -o -name "*.cc" -o -name "*.cxx" -o \
                -name "*.h" -o -name "*.hpp" -o -name "*.hh" -o -name "*.hxx" -o \
                -name "*.gn" -o -name "*.gni" -o -name "BUILD.gn" -o \
                -name "Makefile" -o -name "CMakeLists.txt" \
            \) -exec cp --parents {} "$c_source_dir" \; 2>/dev/null || true
        fi
        echo "已复制原始 C 源代码到: $c_source_dir" >&2
    fi
    
    # 链接知识库（使用框架目录的知识库）
    if [ -f "$SCRIPT_DIR/workspace/rag/knowledge_base.json" ] && [ -f "$SCRIPT_DIR/workspace/rag/bm25_index.pkl" ]; then
        rm -f "$workspace_dir/rag/knowledge_base.json" 2>/dev/null
        rm -f "$workspace_dir/rag/bm25_index.pkl" 2>/dev/null
        ln -sf "$SCRIPT_DIR/workspace/rag/knowledge_base.json" "$workspace_dir/rag/knowledge_base.json"
        ln -sf "$SCRIPT_DIR/workspace/rag/bm25_index.pkl" "$workspace_dir/rag/bm25_index.pkl"
    fi

    # 可选：预置的 reranked_results（用于“无 GPU / reranker 失败”时仍能注入 RAG 知识）
    # 说明：
    # - 我们把预置 reranked_results 放在框架目录的 workspace/rag/reranked_results/<project>/ 下；
    # - 每次 run 初始化项目 workspace 时，将其复制到 <workspace>/rag/reranked_results/<project>/，
    #   保持 run 自洽（避免 symlink 到共享目录导致后续 reranker 覆盖/并发污染）。
    local prebuilt_reranked_dir="$SCRIPT_DIR/workspace/rag/reranked_results/$proj_name"
    local dst_reranked_dir="$workspace_dir/rag/reranked_results/$proj_name"
    if [[ -d "$prebuilt_reranked_dir" ]]; then
        mkdir -p "$dst_reranked_dir" 2>/dev/null || true
        if command -v rsync >/dev/null 2>&1; then
            rsync -a --include='*.txt' --exclude='*' "$prebuilt_reranked_dir/" "$dst_reranked_dir/" 2>/dev/null || true
        else
            find "$prebuilt_reranked_dir" -maxdepth 1 -type f -name "*.txt" -exec cp -f {} "$dst_reranked_dir/" \; 2>/dev/null || true
        fi
    fi
    
    echo "$workspace_dir"
}

# 设置项目环境变量
setup_project_env() {
    local proj_name="$1"
    local proj_path="$2"
    
    # 准备输出目录
    local workspace_dir=$(prepare_project_output_dir "$proj_name" "$proj_path")
    
    export PROJECT_NAME="$proj_name"
    export PROJECT_ROOT="$workspace_dir/projects/$proj_name"
    export LLM_NAME="$LLM_NAME"
    export C2R_TRUTH_MODE="$C2R_TRUTH_MODE"
    export C2R_REQUIRE_TU_CLOSURE="$C2R_REQUIRE_TU_CLOSURE"
    export VIRTUAL_ENV_NAME="$VIRTUAL_ENV_NAME"
    export C2R_WORKSPACE_ROOT="$workspace_dir"
    # run 级别目录（用于把缓存/日志/结果都收敛到同一个实验目录）
    export C2R_RUN_ROOT="$RUN_ROOT_DIR"
    export C2R_RUN_RESULTS_DIR="$RUN_RESULTS_DIR"
    export C2R_RUN_INTERMEDIATE_DIR="$RUN_INTERMEDIATE_DIR"
    export C2R_CACHE_ROOT="${C2R_CACHE_ROOT:-$RUN_INTERMEDIATE_DIR/cache}"
    mkdir -p "$C2R_CACHE_ROOT" 2>/dev/null || true

    # ---------------------------------------------------------------------
    # OpenAI-compat 配置对齐（vLLM vs 外部 API）
    #
    # - generate/generation.py 会根据 USE_VLLM 选择 vLLM 或外部 API；
    # - 但仍有部分模块会读取 OPENAI_API_BASE / VLLM_BASE_URL / VLLM_MODEL_NAME，
    #   因此这里按模式把这些变量对齐，避免“静默全部失败/打到错误 endpoint”。
    # ---------------------------------------------------------------------
    # generate/generation.py uses USE_VLLM to decide vLLM vs external API.
    # Default: local vLLM.
    export USE_VLLM="${USE_VLLM:-true}"

    if [[ "$USE_VLLM" =~ ^(true|1|yes)$ ]]; then
        # vLLM 模式：model= 必须使用 vLLM 启动时的 served_model_name（通常与 LLM_NAME 一致）
        export VLLM_BASE_URL="$VLLM_URL"
        export OPENAI_API_BASE="$VLLM_URL"
        export VLLM_MODEL_NAME="$LLM_NAME"
    else
        # 外部 API 模式：让“legacy 读取 VLLM_* / OPENAI_API_BASE 的模块”也能打到外部 endpoint。
        export VLLM_BASE_URL="${EXTERNAL_API_BASE_URL:-https://api.deepseek.com/beta}"
        export OPENAI_API_BASE="$VLLM_BASE_URL"
        export VLLM_MODEL_NAME="${EXTERNAL_API_MODEL:-deepseek-coder}"
        # Best-effort: align key for call sites that still read VLLM_API_KEY
        if [[ -n "${EXTERNAL_API_KEY:-}" ]]; then
            export VLLM_API_KEY="$EXTERNAL_API_KEY"
        fi
    fi

    export VLLM_REQUEST_TIMEOUT="$VLLM_REQUEST_TIMEOUT"
    export VLLM_TIMEOUT="$VLLM_TIMEOUT"

    # Truth-mode defaults: keep bindgen outputs as truth (avoid derived-layer mutations/fallbacks).
    # Can still be overridden by explicitly exporting these env vars before running the script.
    if [[ "$C2R_TRUTH_MODE" != "0" ]]; then
        export C2R_ENABLE_BINDGEN_POSTPROCESS="${C2R_ENABLE_BINDGEN_POSTPROCESS:-0}"
        export C2R_ENABLE_FINALIZE_TYPES_RS="${C2R_ENABLE_FINALIZE_TYPES_RS:-0}"
        export C2R_ENABLE_SKELETON_REPAIRS="${C2R_ENABLE_SKELETON_REPAIRS:-false}"
    fi

    # ---------------------------------------------------------------------
    # 额外 RAG KB（默认不启用；只有显式传入 --extra-rag-kb-dir 才启用）
    # ---------------------------------------------------------------------
    if [[ -n "${EXTRA_RAG_KB_DIRS:-}" ]]; then
        export C2R_EXTRA_RAG_KB_DIR="$EXTRA_RAG_KB_DIRS"
    else
        unset C2R_EXTRA_RAG_KB_DIR
    fi
    
    # 分层骨架构建模式相关环境变量
    export USE_LAYERED_SKELETON="$USE_LAYERED_SKELETON"
    export USE_BINDGEN="$USE_BINDGEN"
    export USE_LLM_SIGNATURES="$USE_LLM_SIGNATURES"
    export USE_LLM_TYPE_MAPPER="$USE_LLM_TYPE_MAPPER"  # LLMTypeMapper（TypeMapper + LLM 验证）
    # 签名复核器默认开启（见 llm_signature_refiner.py）；这里显式导出便于脚本内一致管理
    if [[ "$USE_LLM_SIG_REFINER" == "false" ]]; then
        export C2R_LLM_REFINE_SIGNATURES="0"
    else
        export C2R_LLM_REFINE_SIGNATURES="1"
    fi
    export USE_TYPE_MAPPER="true"  # 默认启用 TypeMapper（确定性规则引擎）
    export USE_SELF_HEALING="$USE_SELF_HEALING"  # AI 原生自愈循环
    export USE_LIBCLANG="$USE_LIBCLANG"  # 使用 libclang 提取函数

    # 规则修复（rule_fix.py）：默认启用；关闭后将直接进入增量修复/LLM 修复路径
    if [[ "$USE_RULE_FIX" == "false" ]]; then
        export C2R_ENABLE_RULE_FIX="0"
    else
        export C2R_ENABLE_RULE_FIX="${C2R_ENABLE_RULE_FIX:-1}"
    fi

    # bindgen/types 诊断输出（可选）
    export C2R_BINDGEN_DEBUG="$BINDGEN_DEBUG"
    export C2R_BINDGEN_DEBUG_KEEP_FILES="$BINDGEN_DEBUG_KEEP_FILES"
    
    # ---------------------------------------------------------------------
    # 预处理上下文选择（TU pin）：OHOS vs OSS
    #
    # - ohos: 使用 OpenHarmony build profile（out_dir 主键）自动选择（compile_commands_all registry）
    # - oss:  使用项目目录下的 compile_commands.json（不触发 OHOS profile 选择）
    # ---------------------------------------------------------------------
    export USE_PREPROCESSING="${USE_PREPROCESSING:-true}"
    if [[ "$PROJECT_SUITE" == "oss" ]]; then
        export PREPROCESSING_STRATEGY="${PREPROCESSING_STRATEGY:-best}"
    else
        export PREPROCESSING_STRATEGY="${PREPROCESSING_STRATEGY:-auto}"
    fi
    export PREPROCESS_OUTPUT_DIR="${PREPROCESS_OUTPUT_DIR:-$workspace_dir/.preprocessed}"

    if [[ "$PROJECT_SUITE" == "oss" ]]; then
        # Force using the project's own compile_commands.json (avoid OHOS default fallback).
        if [[ -f "$PROJECT_ROOT/compile_commands.json" ]]; then
            export COMPILE_COMMANDS_PATH="$PROJECT_ROOT/compile_commands.json"
        else
            unset COMPILE_COMMANDS_PATH
        fi
        # OSS smoke runs: prefer "compile success = success" so stage3 won't hard-fail on placeholders.
        # (still recorded in translation_stats.json / summary reports)
        export C2R_TRUTH_STRICT_EXIT="${C2R_TRUTH_STRICT_EXIT:-0}"
        # IMPORTANT: do NOT expose OHOS registry env vars for OSS runs,
        # otherwise get_dependencies.py may spend a long time auto-selecting an OHOS profile.
        unset OHOS_ROOT
        unset OHOS_CC_REGISTRY
        unset OHOS_COMPILE_COMMANDS_REGISTRY
        unset OHOS_COMPILE_COMMANDS
        unset C2R_OHOS_CC_EXTRACT_ROOT
        unset C2R_PROFILE_CACHE_DIR
    else
        # OpenHarmony 根目录与 registry（可被外部环境覆盖）
        export OHOS_ROOT="${OHOS_ROOT:-$SCRIPT_DIR/../data/ohos/ohos_root_min}"
        # Registry is optional in the open-source layout; only set it if the default exists.
        if [[ -z "${OHOS_CC_REGISTRY:-}" ]] && [[ -f "$OHOS_ROOT/compile_commands_all/summary.tsv" ]]; then
            export OHOS_CC_REGISTRY="$OHOS_ROOT/compile_commands_all/summary.tsv"
        fi
        # 开源最小输入：允许每个子项目自带 compile_commands.json（只需覆盖该项目用到的最小编译上下文）
        # 这样可以启用预处理上下文选择与更完整的 bindgen/types 生成，从而显著提升函数体翻译可编译率。
        if [[ -f "$PROJECT_ROOT/compile_commands.json" ]]; then
            export COMPILE_COMMANDS_PATH="$PROJECT_ROOT/compile_commands.json"
            export OHOS_COMPILE_COMMANDS="$COMPILE_COMMANDS_PATH"
        else
            unset COMPILE_COMMANDS_PATH
            unset OHOS_COMPILE_COMMANDS
        fi
        # shared（不属于某一次 run 的产物）：以 translation_outputs/shared 为默认放置点
        export C2R_OHOS_CC_EXTRACT_ROOT="${C2R_OHOS_CC_EXTRACT_ROOT:-$OUTPUTS_BASE_DIR/shared/ohos_profiles/compile_commands}"
        export C2R_PROFILE_CACHE_DIR="${C2R_PROFILE_CACHE_DIR:-$OUTPUTS_BASE_DIR/shared/ohos_profiles/projects}"
    fi

    # ---------------------------------------------------------------------
    # 并行优化：限制每个子任务的线程/构建并行度，避免 MAX_PARALLEL 很大时系统过载
    # ---------------------------------------------------------------------
    local cpu_total
    cpu_total=$(nproc 2>/dev/null || echo 4)
    local per_task_jobs=$((cpu_total / MAX_PARALLEL))
    if [[ "$per_task_jobs" -lt 1 ]]; then
        per_task_jobs=1
    fi
    # cargo/build/check 的并行度（Python 侧会注入 -j）
    export C2R_CARGO_JOBS="${C2R_CARGO_JOBS:-$per_task_jobs}"
    # 常见线程池/数值库的线程数（尽量不干扰外部显式设置）
    export OMP_NUM_THREADS="${OMP_NUM_THREADS:-$per_task_jobs}"
    export OPENBLAS_NUM_THREADS="${OPENBLAS_NUM_THREADS:-1}"
    export MKL_NUM_THREADS="${MKL_NUM_THREADS:-1}"
    export NUMEXPR_NUM_THREADS="${NUMEXPR_NUM_THREADS:-1}"
    export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-$per_task_jobs}"
    export TOKENIZERS_PARALLELISM="${TOKENIZERS_PARALLELISM:-false}"

    # LLM 并行度：不限制单项目并发，由 Semaphore(VLLM_CONCURRENT_LIMIT) 全局控制
    export PARALLEL_TRANSLATE="${PARALLEL_TRANSLATE:-1}"
    # LLM 并行度：限制每个项目内的“翻译-only”并发，避免外部 API 429
    export MAX_PARALLEL_WORKERS="${MAX_PARALLEL_WORKERS:-4}"

    # LLM 请求并发限制：进程内 Semaphore 控制单进程并发；跨项目总并发仍主要由 MAX_PARALLEL/MAX_PARALLEL_WORKERS 决定
    export VLLM_CONCURRENT_LIMIT="${VLLM_CONCURRENT_LIMIT:-120}"
    print_info "LLM 并发限制: VLLM_CONCURRENT_LIMIT=$VLLM_CONCURRENT_LIMIT (进程内 Semaphore)"

    # 导出其他必要的环境变量
    export SKIP_VLLM_MANAGEMENT=1  # 阻止子进程管理vLLM
}

# ---------------------------------------------------------------------------
# Stage1 Reuse Helpers (RQ3.x)
# ---------------------------------------------------------------------------
# When running ablations/sensitivity experiments, we may want to reuse a previous
# run's Stage1 outputs (dependency analysis + skeleton translation) to avoid
# re-running expensive skeleton generation.
#
# This is opt-in via: --reuse-stage1-from <DIR>
# - DIR can be a run_dir root (contains intermediate/), or the intermediate/ dir.
# - For safety, we COPY artifacts into the new workspace and patch
#   `.preprocessed/tu_context_map.json` to point to the new workspace paths.
_resolve_reuse_stage1_workspace_dir() {
    local base="$1"
    local proj_name="$2"

    if [[ -z "$base" ]] || [[ -z "$proj_name" ]]; then
        return 1
    fi

    # Case 1: base is a run_dir root (contains intermediate/)
    if [[ -d "$base/intermediate/$proj_name/workspace" ]]; then
        echo "$base/intermediate/$proj_name/workspace"
        return 0
    fi

    # Case 2: base is already the intermediate/ directory
    if [[ -d "$base/$proj_name/workspace" ]]; then
        echo "$base/$proj_name/workspace"
        return 0
    fi

    # Case 3: base is per-project directory (contains workspace/)
    if [[ -d "$base/workspace" ]] && [[ "$(basename "$base")" == "$proj_name" ]]; then
        echo "$base/workspace"
        return 0
    fi

    # Case 4: base itself is a workspace directory
    if [[ -d "$base" ]] && [[ "$(basename "$base")" == "workspace" ]]; then
        echo "$base"
        return 0
    fi

    return 1
}

_copy_dir_tree() {
    local src="$1"
    local dst="$2"

    if [[ ! -d "$src" ]]; then
        echo "missing dir: $src" >&2
        return 1
    fi

    rm -rf "$dst" 2>/dev/null || true
    mkdir -p "$(dirname "$dst")" 2>/dev/null || true

    if command -v rsync >/dev/null 2>&1; then
        rsync -a "$src/" "$dst/"
    else
        cp -a "$src" "$dst"
    fi
}

_patch_tu_context_map_json_inplace() {
    local tu_map="$1"
    local old_ws="$2"
    local new_ws="$3"

    if [[ ! -f "$tu_map" ]]; then
        echo "missing tu_context_map.json: $tu_map" >&2
        return 1
    fi

    python3 - "$tu_map" "$old_ws" "$new_ws" <<'PY'
import json
import os
import sys

path = sys.argv[1]
old_ws = sys.argv[2]
new_ws = sys.argv[3]

old_candidates = []
for p in (old_ws, os.path.realpath(old_ws)):
    if not p:
        continue
    p = p.rstrip(os.sep)
    if p and p not in old_candidates:
        old_candidates.append(p)

new_ws = os.path.realpath(new_ws).rstrip(os.sep)

def rewrite(x):
    if isinstance(x, str):
        for old in old_candidates:
            if x == old or x.startswith(old + os.sep):
                return new_ws + x[len(old):]
        return x
    if isinstance(x, list):
        return [rewrite(i) for i in x]
    if isinstance(x, dict):
        return {k: rewrite(v) for k, v in x.items()}
    return x

with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

data2 = rewrite(data)

with open(path, "w", encoding="utf-8") as f:
    json.dump(data2, f, ensure_ascii=False, indent=2)
PY
}

reuse_stage1_artifacts() {
    local old_ws="$1"
    local new_ws="$2"
    local proj_name="$3"

    if [[ -z "$old_ws" ]] || [[ -z "$new_ws" ]] || [[ -z "$proj_name" ]]; then
        echo "usage: reuse_stage1_artifacts <old_ws> <new_ws> <proj_name>" >&2
        return 1
    fi

    # Stage2/3 (incremental) rely on these stage1 outputs:
    _copy_dir_tree "$old_ws/extracted/$proj_name" "$new_ws/extracted/$proj_name"
    _copy_dir_tree "$old_ws/skeletons/$proj_name" "$new_ws/skeletons/$proj_name"
    _copy_dir_tree "$old_ws/source_skeletons/$proj_name" "$new_ws/source_skeletons/$proj_name"
    _copy_dir_tree "$old_ws/.preprocessed" "$new_ws/.preprocessed"

    # Fix absolute paths inside tu_context_map.json to match the new workspace.
    _patch_tu_context_map_json_inplace "$new_ws/.preprocessed/tu_context_map.json" "$old_ws" "$new_ws"

    return 0
}

# 执行单个项目的阶段1（依赖分析 + 骨架翻译）
run_stage1() {
    local proj_name="$1"
    local proj_path="$2"
    local log_file="$3"
    local result_file="$4"
    
    local start_time=$(date +%s)
    
    {
        echo "============================================================"
        echo "项目 $proj_name - 阶段1: 依赖分析 + 骨架翻译"
        echo "============================================================"
        
        setup_project_env "$proj_name" "$proj_path"
        
        # 关键：激活默认翻译环境（包含依赖分析与 BM25 所需依赖）
        if ! activate_conda_env "$VIRTUAL_ENV_NAME"; then
            echo "FAIL|conda 环境激活失败" > "$result_file"
            return 1
        fi

        # 可选：复用历史 Stage1 产物（避免重复依赖分析/骨架翻译）
        if [[ -n "${REUSE_STAGE1_FROM:-}" ]]; then
            local old_ws
            old_ws=$(_resolve_reuse_stage1_workspace_dir "$REUSE_STAGE1_FROM" "$proj_name" || true)
            if [[ -z "$old_ws" ]] || [[ ! -d "$old_ws" ]]; then
                echo "FAIL|未找到可复用的 Stage1 workspace: from=$REUSE_STAGE1_FROM proj=$proj_name" > "$result_file"
                return 1
            fi

            local new_ws="$C2R_WORKSPACE_ROOT"
            echo "ℹ 复用 Stage1 产物（copy + patch tu_context_map.json）:" | tee -a "$log_file.step1" "$log_file.step2"
            echo "  - from: $old_ws" | tee -a "$log_file.step1" "$log_file.step2"
            echo "  - to  : $new_ws" | tee -a "$log_file.step1" "$log_file.step2"

            if ! reuse_stage1_artifacts "$old_ws" "$new_ws" "$proj_name" 2>&1 | tee -a "$log_file.step1" "$log_file.step2"; then
                echo "FAIL|Stage1复用失败" > "$result_file"
                return 1
            fi

            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            echo "SUCCESS|$start_time|$end_time|$duration" > "$result_file"
            return 0
        fi
        
        # 步骤1: 分析 C/C++ 依赖（步骤0准备工作由get_dependencies.py内部处理）
        # 注意：项目名称通过环境变量 PROJECT_NAME 传递，不使用命令行参数
        cd "$SCRIPT_DIR"
        if [[ "$USE_LIBCLANG" == "true" ]]; then
            # 使用 libclang 模式时，步骤1 只创建必要的目录结构
            # 实际的函数提取将在步骤2（translate.py）的阶段 A.5.1 中完成
            echo "ℹ 使用 libclang 模式：跳过 tree-sitter 提取，函数将在步骤2中由 libclang 提取" | tee -a "$log_file.step1"
            # 确保目录存在（get_dependencies.py 的 ensure_dirs 功能）
            python3 -c "
from workspace_config import ensure_dirs
from project_config import PROJECT_NAME
ensure_dirs(PROJECT_NAME)
print('✓ 工作目录已创建')
" 2>&1 | tee -a "$log_file.step1"
        else
            # 传统模式：使用 tree-sitter 提取
            if ! python3 get_dependencies.py 2>&1 | tee -a "$log_file.step1"; then
                echo "FAIL|步骤1失败" > "$result_file"
                return 1
            fi
        fi
        
        # 步骤2: 翻译代码骨架（vLLM应该已经在运行）
        # 注意：项目名称通过环境变量 PROJECT_NAME 传递
        # 根据环境变量选择分层模式或传统模式
        local translate_args=""
        if [[ "$USE_LAYERED_SKELETON" == "true" ]]; then
            translate_args="--layered"
            if [[ "$USE_BINDGEN" == "false" ]]; then
                translate_args="$translate_args --no-bindgen"
            fi
            if [[ "$USE_LLM_SIGNATURES" == "false" ]]; then
                translate_args="$translate_args --no-llm-signatures"
            fi
            if [[ "$USE_LIBCLANG" == "true" ]]; then
                translate_args="$translate_args --use-libclang"
            fi
        fi
        if ! python3 translate.py $translate_args 2>&1 | tee -a "$log_file.step2"; then
            echo "FAIL|步骤2失败" > "$result_file"
            return 1
        fi
        
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "SUCCESS|$start_time|$end_time|$duration" > "$result_file"
        return 0
    } 2>&1 | tee "$log_file"
}

# 执行单个项目的阶段2（签名匹配 + BM25 + Jina Reranker）
run_stage2() {
    local proj_name="$1"
    local proj_path="$2"
    local log_file="$3"
    local result_file="$4"
    
    local start_time=$(date +%s)
    
    {
        if [[ "$RUN_RAG" == true ]]; then
            echo "============================================================"
            echo "项目 $proj_name - 阶段2: 签名匹配 + BM25 + Jina Reranker"
            echo "============================================================"
        else
            echo "============================================================"
            echo "项目 $proj_name - 阶段2: 签名匹配（跳过 RAG）"
            echo "============================================================"
        fi
        
        setup_project_env "$proj_name" "$proj_path"
        
        # 关键：激活默认翻译环境（包含依赖分析 + BM25 所需依赖）
        if ! activate_conda_env "$VIRTUAL_ENV_NAME"; then
            echo "FAIL|conda 环境激活失败" > "$result_file"
            return 1
        fi
        
        # 步骤3: 匹配 C++ 函数和 Rust 函数签名
        cd "$SCRIPT_DIR"
        if ! python3 get_dependencies_match.py "$LLM_NAME" 2>&1 | tee -a "$log_file.step3"; then
            echo "FAIL|步骤3失败" > "$result_file"
            return 1
        fi
        
        # 步骤4和5: RAG 检索（仅在 --run-rag 时执行）
        if [[ "$RUN_RAG" == true ]]; then
            # 步骤4: RAG 检索 - BM25 粗筛
            if ! python3 elastic_search.py 2>&1 | tee -a "$log_file.step4"; then
                echo "FAIL|步骤4失败" > "$result_file"
                return 1
            fi

            # 步骤5: RAG 检索 - Jina-Reranker 精排
            # 注意：Jina Reranker 将在阶段2.5统一运行（避免并发OOM）
            echo "Jina Reranker 将在阶段2.5统一运行（使用GPU排队机制）" | tee -a "$log_file.step5"
        else
            echo "跳过 BM25 和 Jina Reranker（知识库和项目未变化）" | tee -a "$log_file.step4" | tee -a "$log_file.step5"
        fi
        
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "SUCCESS|$start_time|$end_time|$duration" > "$result_file"
        return 0
    } 2>&1 | tee "$log_file"
}

# 执行单个项目的阶段2.5（Jina Reranker，需要GPU）
run_stage2_5() {
    local proj_name="$1"
    local proj_path="$2"
    local log_file="$3"
    local result_file="$4"

    local start_time=$(date +%s)

    {
        echo "============================================================"
        echo "项目 $proj_name - 阶段2.5: Jina Reranker 精排"
        echo "============================================================"

        setup_project_env "$proj_name" "$proj_path"

        # 激活 Jina Reranker 环境
        if ! activate_conda_env "$RERANKER_ENV_NAME"; then
            echo "FAIL|Jina Reranker 环境激活失败" > "$result_file"
            return 1
        fi

        # 步骤5: Jina Reranker 精排
        cd "$SCRIPT_DIR"
        if ! python3 resort_by_unixcoder.py 2>&1 | tee -a "$log_file.step5"; then
            echo "FAIL|步骤5失败（Jina Reranker）" > "$result_file"
            # 切回默认环境
            activate_conda_env "$VIRTUAL_ENV_NAME" >/dev/null 2>&1 || true
            return 1
        fi

        # 切回默认环境
        activate_conda_env "$VIRTUAL_ENV_NAME" >/dev/null 2>&1 || true

        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "SUCCESS|$start_time|$end_time|$duration" > "$result_file"
        return 0
    } 2>&1 | tee "$log_file"
}

# 执行单个项目的阶段3（函数体翻译 + 测试 + 修复）
run_stage3() {
    local proj_name="$1"
    local proj_path="$2"
    local log_file="$3"
    local result_file="$4"
    
    local start_time=$(date +%s)
    
    {
        echo "============================================================"
        echo "项目 $proj_name - 阶段3: 函数体翻译 + 测试 + 修复"
        echo "============================================================"
        
        setup_project_env "$proj_name" "$proj_path"
        
        # 关键：激活 conda 环境（确保 Python 模块可用）
        if ! activate_conda_env; then
            echo "FAIL|conda 环境激活失败" > "$result_file"
            return 1
        fi
        
        cd "$SCRIPT_DIR"
        
        # 根据模式选择翻译方式
        if [[ "$USE_INCREMENTAL" == true ]]; then
            # 增量式翻译模式
            if ! python3 incremental_translate.py "$proj_name" "$LLM_NAME" "$MAX_REPAIR" 2>&1 | tee -a "$log_file.step7"; then
                echo "FAIL|步骤7失败（增量翻译）" > "$result_file"
                return 1
            fi
        else
            # 传统并行模式
            # 使用环境变量中的工作空间路径
            local WORKSPACE_BASE="${C2R_WORKSPACE_ROOT}"
            local FUNCTIONS_DIR="${WORKSPACE_BASE}/extracted/${proj_name}/functions"
            local TRANSLATED_DIR="${WORKSPACE_BASE}/translated/${proj_name}"
            local SKELETON_DIR="${WORKSPACE_BASE}/skeletons/${proj_name}"
            local RAG_DIR="${WORKSPACE_BASE}/rag/reranked_results/${proj_name}"
            
            # 步骤7: 函数体翻译
            if ! python3 translate_function.py "$FUNCTIONS_DIR" "$TRANSLATED_DIR" "$LLM_NAME" "$SKELETON_DIR" "$RAG_DIR" test 2>&1 | tee -a "$log_file.step7"; then
                echo "FAIL|步骤7失败（函数体翻译）" > "$result_file"
                return 1
            fi
            
            # 步骤9: 项目级编译测试
            local TRANSLATED_PROJECT_DIR="${TRANSLATED_DIR}/translate_by_${LLM_NAME}"
            local TEST_RESULTS_DIR="${WORKSPACE_BASE}/test_results/${proj_name}/translate_by_${LLM_NAME}"
            
            if ! python3 auto_test_rust.py "$TRANSLATED_PROJECT_DIR" "$TEST_RESULTS_DIR" "translate_by_${LLM_NAME}" "$proj_name" 2>&1 | tee -a "$log_file.step9"; then
                echo "FAIL|步骤9失败（编译测试）" > "$result_file"
                return 1
            fi
            
            # 步骤10: 项目级自动修复
            local REPAIR_RESULTS_DIR="${WORKSPACE_BASE}/repair_results/${proj_name}"
            if ! python3 auto_repair_rust.py "$FUNCTIONS_DIR" "$TRANSLATED_DIR" "${WORKSPACE_BASE}/test_results/${proj_name}" "$REPAIR_RESULTS_DIR" "$LLM_NAME" "$SKELETON_DIR" 2>&1 | tee -a "$log_file.step10"; then
                echo "WARN|步骤10失败（自动修复），继续执行" > "$result_file.warn"
            fi
        fi
        
        # 步骤11: 最终合并和完整项目测试
        # 增量翻译模式已在步骤7中保存了最终项目，跳过最终合并步骤
        if [[ "$USE_INCREMENTAL" == true ]]; then
            echo "增量翻译模式：最终项目已在步骤7中保存，跳过最终合并步骤" | tee -a "$log_file.step11"
        else
            if ! python3 merge_final_project.py "$proj_name" "$LLM_NAME" 2>&1 | tee -a "$log_file.step11"; then
                echo "WARN|步骤11失败（最终合并），继续执行" > "$result_file.warn"
            fi
        fi
        
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "SUCCESS|$start_time|$end_time|$duration" > "$result_file"
        return 0
    } 2>&1 | tee "$log_file"
}

# 并行运行阶段（带同步等待）
run_stage_parallel() {
    local stage_name="$1"
    local stage_func="$2"
    local projects=("${@:3}")
    
    print_section "阶段: $stage_name"
    
    local temp_dir
    temp_dir=$(mktemp -d -t batch_stage_XXXXXX 2>/dev/null) || temp_dir="/tmp/batch_stage_$$"
    mkdir -p "$temp_dir"
    
    if [[ -z "$temp_dir" || "$temp_dir" == "/" ]]; then
        print_error "无法创建临时目录，使用备用路径"
        temp_dir="$SCRIPT_DIR/.temp_logs_$$"
        mkdir -p "$temp_dir"
    fi
    
    declare -A pids
    declare -A log_files
    declare -A result_files
    
    local running_count=0
    local completed_count=0
    local total_projects=${#projects[@]}
    
    # 启动所有项目的这个阶段
    for proj_name in "${projects[@]}"; do
        local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
        local log_file="$temp_dir/${proj_name}.log"
        local result_file="$temp_dir/${proj_name}.result"
        
        log_files[$proj_name]="$log_file"
        result_files[$proj_name]="$result_file"
        
        # 等待直到有可用槽位
        while [[ $running_count -ge $MAX_PARALLEL ]]; do
            for pid in "${!pids[@]}"; do
                if ! kill -0 "${pids[$pid]}" 2>/dev/null; then
                    # 进程已完成，等待它以确保完全退出
                    wait "${pids[$pid]}" 2>/dev/null || true
                    unset pids[$pid]
                    running_count=$((running_count - 1))
                    completed_count=$((completed_count + 1))
                fi
            done
            if [[ $running_count -ge $MAX_PARALLEL ]]; then
                sleep 1
            fi
        done
        
        # 启动后台进程
        print_info "[$((completed_count + running_count + 1))/$total_projects] 启动项目: $proj_name"
        $stage_func "$proj_name" "$proj_path" "$log_file" "$result_file" &
        local pid=$!
        pids[$proj_name]=$pid
        running_count=$((running_count + 1))
    done
    
    # 等待所有进程完成
    print_info "等待所有项目完成阶段: $stage_name"
    while [[ ${#pids[@]} -gt 0 ]]; do
        for proj_name in "${!pids[@]}"; do
            if ! kill -0 "${pids[$proj_name]}" 2>/dev/null; then
                # 进程已完成，等待它以确保完全退出
                wait "${pids[$proj_name]}" 2>/dev/null || true
                unset pids[$proj_name]
                completed_count=$((completed_count + 1))
                print_info "[$completed_count/$total_projects] 项目完成: $proj_name"
            fi
        done
        if [[ ${#pids[@]} -gt 0 ]]; then
            sleep 2
        fi
    done
    
    # 收集结果
    local success_count=0
    local fail_count=0
    
    for proj_name in "${projects[@]}"; do
        local result_file="${result_files[$proj_name]}"
        if [[ -f "$result_file" ]]; then
            local result_line=$(cat "$result_file" 2>/dev/null || echo "FAIL|未知错误")
            local status=$(echo "$result_line" | cut -d'|' -f1)
            if [[ "$status" == "SUCCESS" ]]; then
                success_count=$((success_count + 1))
                local duration=$(echo "$result_line" | cut -d'|' -f4)
                local minutes=$((duration / 60))
                local seconds=$((duration % 60))
                print_success "$proj_name: 成功 (${minutes}分${seconds}秒)"
            else
                fail_count=$((fail_count + 1))
                local error=$(echo "$result_line" | cut -d'|' -f2)
                print_error "$proj_name: 失败 ($error)"
            fi
        else
            fail_count=$((fail_count + 1))
            print_error "$proj_name: 结果文件未找到"
        fi
    done
    
    print_info "阶段 $stage_name 完成: 成功 $success_count, 失败 $fail_count"
    
    # 保存日志到持久化位置
    for proj_name in "${projects[@]}"; do
        local log_file="${log_files[$proj_name]}"
        if [[ -f "$log_file" ]]; then
            # 保存主日志文件（包含所有步骤的输出）
            local persistent_log="$LOG_DIR/batch_staged_${proj_name}.log"
            cp "$log_file" "$persistent_log" 2>/dev/null || true
            
            # 创建项目日志子目录
            local project_log_dir="$LOG_DIR/batch_staged_${proj_name}"
            mkdir -p "$project_log_dir"
            
            # 保存所有步骤日志文件到子目录
            local log_base=$(dirname "$log_file")
            for step_log in "$log_base/${proj_name}.log.step"*; do
                if [[ -f "$step_log" ]]; then
                    # 提取步骤编号 (step1, step2, 等)
                    local step_num=$(basename "$step_log" | sed -n "s/.*\.step\([0-9]*\)$/\1/p")
                    if [[ -n "$step_num" ]]; then
                        cp "$step_log" "$project_log_dir/step${step_num}.log" 2>/dev/null || true
                    fi
                fi
            done
        fi
    done
    
    # 返回失败的项目列表（用于后续决定是否继续）
    # 注意：失败项目列表输出到标准错误，避免与标准输出混淆
    # 但需要检查是否有失败项目列表的输出文件（通过环境变量传递）
    local failed_list_output="${FAILED_LIST_OUTPUT:-}"
    if [[ $fail_count -gt 0 ]]; then
        for proj_name in "${projects[@]}"; do
            local result_file="${result_files[$proj_name]}"
            if [[ -f "$result_file" ]]; then
                local result_line=$(cat "$result_file" 2>/dev/null || echo "FAIL|")
                local status=$(echo "$result_line" | cut -d'|' -f1)
                if [[ "$status" != "SUCCESS" ]]; then
                    if [[ -n "$failed_list_output" ]]; then
                        echo "$proj_name" >> "$failed_list_output"
                    else
                        echo "$proj_name" >&2
                    fi
                fi
            fi
        done
        return 1
    fi
    
    return 0
}

# ============================================
# 流水线调度函数（单个项目完整流程）
# ============================================
run_project_pipeline() {
    local proj_name="$1"
    local proj_path="$2"

    local project_start_time=$(date +%s)
    print_info "[流水线] 启动项目: $proj_name"

    # 阶段1
    local stage1_log="$LOG_DIR/pipeline_${proj_name}_stage1.log"
    local stage1_result="$LOG_DIR/pipeline_${proj_name}_stage1.result"

    if ! run_stage1 "$proj_name" "$proj_path" "$stage1_log" "$stage1_result"; then
        print_error "[流水线] $proj_name 阶段1失败"
        return 1
    fi

    # 阶段2
    local stage2_log="$LOG_DIR/pipeline_${proj_name}_stage2.log"
    local stage2_result="$LOG_DIR/pipeline_${proj_name}_stage2.result"

    if ! run_stage2 "$proj_name" "$proj_path" "$stage2_log" "$stage2_result"; then
        print_error "[流水线] $proj_name 阶段2失败"
        return 1
    fi

    # 阶段2.5: Jina Reranker（仅在 --run-rag true 时执行）
    if [[ "$RUN_RAG" == true ]]; then
        local stage2_5_log="$LOG_DIR/pipeline_${proj_name}_stage2.5.log"
        local stage2_5_result="$LOG_DIR/pipeline_${proj_name}_stage2.5.result"

        if ! run_stage2_5 "$proj_name" "$proj_path" "$stage2_5_log" "$stage2_5_result"; then
            print_error "[流水线] $proj_name 阶段2.5失败（Jina Reranker）"
            return 1
        fi
    else
        print_info "[流水线] $proj_name 跳过阶段2.5（RUN_RAG=false）"
    fi

    # 骨架翻译模式下跳过阶段3
    if [[ "$SKELETON_ONLY" == "true" ]]; then
        local project_end_time=$(date +%s)
        local project_duration=$((project_end_time - project_start_time))
        print_success "[流水线] $proj_name 完成（骨架模式，耗时: ${project_duration}秒）"
        return 0
    fi

    # 阶段3
    local stage3_log="$LOG_DIR/pipeline_${proj_name}_stage3.log"
    local stage3_result="$LOG_DIR/pipeline_${proj_name}_stage3.result"

    if ! run_stage3 "$proj_name" "$proj_path" "$stage3_log" "$stage3_result"; then
        print_error "[流水线] $proj_name 阶段3失败"
        return 1
    fi

    local project_end_time=$(date +%s)
    local project_duration=$((project_end_time - project_start_time))
    local project_minutes=$((project_duration / 60))
    local project_seconds=$((project_duration % 60))

    print_success "[流水线] $proj_name 完成全部阶段（耗时: ${project_minutes}分${project_seconds}秒）"
    return 0
}

# ============================================
# 流水线模式主调度器
# ============================================
run_pipeline_mode() {
    local projects=("$@")

    print_section "流水线模式启动"
    print_info "项目数量: ${#projects[@]}"
    print_info "最大并行数: $MAX_PARALLEL"
    print_info "每个项目将独立完成所有阶段"
    echo ""

    local pids=()
    local running_count=0
    local completed_count=0
    local failed_count=0
    local total=${#projects[@]}

    declare -A project_pids
    declare -A project_status

    # 流水线模式：阶段1/3 使用外部 API，不启动本地 vLLM
    # vLLM 仅在阶段4（语义评估）和知识沉淀时启动
    print_info "流水线模式使用外部 API，跳过本地 vLLM 启动"

    # 启动项目（流水线方式）
    for proj_name in "${projects[@]}"; do
        local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"

        # 等待槽位可用
        while [[ $running_count -ge $MAX_PARALLEL ]]; do
            # 检查已完成的进程
            for pid in "${!project_pids[@]}"; do
                if ! kill -0 "$pid" 2>/dev/null; then
                    wait "$pid" 2>/dev/null
                    local exit_code=$?
                    local finished_proj="${project_pids[$pid]}"
                    unset project_pids[$pid]

                    running_count=$((running_count - 1))

                    if [[ $exit_code -eq 0 ]]; then
                        completed_count=$((completed_count + 1))
                        project_status[$finished_proj]="SUCCESS"
                    else
                        failed_count=$((failed_count + 1))
                        project_status[$finished_proj]="FAILED"
                    fi

                    print_info "[流水线] 进度: 完成=$completed_count, 失败=$failed_count, 运行=$running_count, 总计=$total"
                fi
            done

            if [[ $running_count -ge $MAX_PARALLEL ]]; then
                sleep 2
            fi
        done

        # 启动新项目
        print_info "[流水线] [$((completed_count + failed_count + running_count + 1))/$total] 启动: $proj_name"
        run_project_pipeline "$proj_name" "$proj_path" &
        local pid=$!
        project_pids[$pid]="$proj_name"
        pids+=($pid)
        running_count=$((running_count + 1))
    done

    # 等待所有项目完成
    print_info "等待所有项目完成..."
    for pid in "${pids[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            wait "$pid" 2>/dev/null
            local exit_code=$?
            local finished_proj="${project_pids[$pid]}"

            if [[ $exit_code -eq 0 ]]; then
                completed_count=$((completed_count + 1))
                project_status[$finished_proj]="SUCCESS"
            else
                failed_count=$((failed_count + 1))
                project_status[$finished_proj]="FAILED"
            fi
        fi
    done

    # 关闭 vLLM（流水线结束后不再需要）
    if [[ "$VLLM_STARTED_BY_SCRIPT" == "true" ]]; then
        print_info "流水线执行完成，关闭 vLLM..."
        stop_vllm
        VLLM_STARTED_BY_SCRIPT=false
    fi

    # 输出结果
    print_section "流水线执行完成"
    echo "成功: $completed_count"
    echo "失败: $failed_count"
    echo "总计: $total"

    if [[ $failed_count -gt 0 ]]; then
        echo ""
        echo "失败的项目:"
        for proj in "${!project_status[@]}"; do
            if [[ "${project_status[$proj]}" == "FAILED" ]]; then
                echo "  - $proj"
            fi
        done
    fi

    return $([[ $failed_count -eq 0 ]] && echo 0 || echo 1)
}

# 主函数
main() {
    print_section "C2Rust 流水线批量项目测试"
    echo "模式: 流水线并行执行"
    echo "最大并行数: $MAX_PARALLEL"
    if [[ "$SKELETON_ONLY" == "true" ]]; then
        echo "运行模式: 仅骨架翻译（--skeleton-only）"
    fi
    if [[ "$USE_LAYERED_SKELETON" == "true" ]]; then
        echo "骨架模式: 分层构建 (bindgen + tree-sitter)"
        echo "  - bindgen: $USE_BINDGEN"
        echo "  - LLM 签名翻译: $USE_LLM_SIGNATURES"
        echo "  - LLMTypeMapper: $USE_LLM_TYPE_MAPPER"
        echo "  - AI 自愈循环: $USE_SELF_HEALING"
        echo "  - libclang 提取: $USE_LIBCLANG"
    else
        echo "骨架模式: 传统 LLM 单次生成"
    fi
    echo "输出目录: $RUN_ROOT_DIR"
    echo "  - results:      $RUN_RESULTS_DIR"
    echo "  - intermediate: $RUN_INTERMEDIATE_DIR"
    echo ""
    echo "推荐的项目列表："
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        echo "  - $proj_name"
    done
    echo ""
    
    # 检查项目路径
    check_project_paths
    
    # 收集要运行的项目
    local projects_to_run=()
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        if ! should_skip_project "$proj_name"; then
            projects_to_run+=("$proj_name")
        fi
    done
    
    if [[ ${#projects_to_run[@]} -eq 0 ]]; then
        print_error "没有要运行的项目"
        exit 0
    fi
    
    # 按项目大小排序（大项目优先），避免最后等待大项目
    print_info "正在按项目大小排序（大项目优先处理）..."
    local sorted_projects=()
    local project_sizes=""
    
    for proj_name in "${projects_to_run[@]}"; do
        local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
        # 计算 C/C++ 源文件数量作为项目大小指标
        local file_count=0
        if [[ -d "$proj_path" ]]; then
            file_count=$(find "$proj_path" -type f \( -name "*.c" -o -name "*.cpp" -o -name "*.cc" -o -name "*.h" -o -name "*.hpp" \) 2>/dev/null | wc -l)
        fi
        project_sizes+="$file_count $proj_name"$'\n'
    done
    
    # 按文件数量降序排序
    while IFS=' ' read -r count name; do
        [[ -n "$name" ]] && sorted_projects+=("$name")
    done < <(echo "$project_sizes" | sort -t' ' -k1 -nr)
    
    if [[ ${#sorted_projects[@]} -gt 0 ]]; then
        projects_to_run=("${sorted_projects[@]}")
        print_info "项目处理顺序（按文件数量降序）:"
        for proj_name in "${projects_to_run[@]}"; do
            local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
            local file_count=$(find "$proj_path" -type f \( -name "*.c" -o -name "*.cpp" -o -name "*.cc" -o -name "*.h" -o -name "*.hpp" \) 2>/dev/null | wc -l)
            echo "  - $proj_name ($file_count 个文件)"
        done
    fi
    
    local total_projects=${#projects_to_run[@]}
    local overall_start_time=$(date +%s)
    
    # vLLM URL（用于清理时检查，必须在 cleanup_on_exit 函数之前定义）
    VLLM_HOST="${VLLM_HOST:-127.0.0.1}"
    VLLM_PORT="${VLLM_PORT:-8000}"
    VLLM_URL="${VLLM_BASE_URL:-http://$VLLM_HOST:$VLLM_PORT/v1}"
    
    # 跟踪 vLLM 是否由本脚本启动（用于判断是否需要清理）
    VLLM_STARTED_BY_SCRIPT=false
    
    # 关键：设置退出时的清理函数（确保 vLLM 被关闭）
    cleanup_on_exit() {
        local exit_code=$?
        # 检查 vLLM 是否在运行（由本脚本启动的）
        if [[ "$VLLM_STARTED_BY_SCRIPT" == "true" ]]; then
            if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
                print_warning "检测到脚本退出，正在清理 vLLM..."
                stop_vllm
            fi
        fi
        exit $exit_code
    }
    
    # 注册 trap：在脚本退出（正常或异常）时执行清理
    trap cleanup_on_exit EXIT INT TERM

    # ========================================================================
    # 阶段1: 依赖分析 + 骨架翻译（使用外部 API，不启动本地 vLLM）
    # ========================================================================
    print_section "========== 阶段1: 依赖分析 + 骨架翻译 =========="

    # 阶段1/3 使用外部 API，不需要启动本地 vLLM
    # vLLM 仅在阶段4（语义评估）和知识沉淀时启动
    print_info "阶段1使用外部 API 进行骨架翻译，跳过本地 vLLM 启动"

    # 并行运行阶段1
    local failed_stage1=()
    local skipped_tu_closure=()
    local skipped_no_built_sources=()
    local failed_list_file=$(mktemp)
    FAILED_LIST_OUTPUT="$failed_list_file" run_stage_parallel "阶段1: 依赖分析 + 骨架翻译" "run_stage1" "${projects_to_run[@]}" || true
    if [[ -f "$failed_list_file" ]] && [[ -s "$failed_list_file" ]]; then
        while IFS= read -r proj_name; do
            [[ -n "$proj_name" ]] && failed_stage1+=("$proj_name")
        done < "$failed_list_file"
    fi
    rm -f "$failed_list_file"

    # 如果阶段1全部失败，退出
    if [[ ${#failed_stage1[@]} -eq ${#projects_to_run[@]} ]]; then
        print_error "所有项目的阶段1都失败了，退出"
        exit 1
    fi

    # 只继续运行阶段1成功的项目
    if [[ ${#failed_stage1[@]} -gt 0 ]]; then
        print_warning "以下项目阶段1失败，将跳过后续阶段: ${failed_stage1[*]}"
        local temp_projects=()
        for proj_name in "${projects_to_run[@]}"; do
            local skip=false
            for failed in "${failed_stage1[@]}"; do
                if [[ "$proj_name" == "$failed" ]]; then
                    skip=true
                    break
                fi
            done
            if [[ "$skip" == false ]]; then
                temp_projects+=("$proj_name")
            fi
        done
        projects_to_run=("${temp_projects[@]}")
    fi

    # ------------------------------------------------------------------------
    # TU 闭包门禁：若缺失 TU/预处理失败，则不进入后续阶段（不检查 test 的 TU）
    # 判定依据：workspace/.preprocessed/tu_context_map.json -> files[*]
    # - compile_commands_entry == null  -> 缺失 TU 条目
    # - error 非空                      -> 预处理失败（无法获得宏展开/.i）
    # - preprocessed_file 为空/不存在   -> 无 .i 产物（宏/inline 不可靠）
    # ------------------------------------------------------------------------
    if [[ "$C2R_REQUIRE_TU_CLOSURE" != "0" ]]; then
        local tu_ok_projects=()
        for proj_name in "${projects_to_run[@]}"; do
            local tu_map="$RUN_INTERMEDIATE_DIR/${proj_name}/workspace/.preprocessed/tu_context_map.json"
            local tu_rc=0
            if python3 - "$tu_map" >/dev/null 2>&1 <<'PY'
import json, sys
from pathlib import Path

p = Path(sys.argv[1])
if not p.exists():
    sys.exit(1)
try:
    data = json.loads(p.read_text(encoding="utf-8", errors="ignore"))
except Exception:
    sys.exit(1)

files = data.get("files") or {}
if not isinstance(files, dict) or not files:
    # No per-file TU info -> treat as not OK for later stages
    sys.exit(1)

included = 0
excluded = 0
missing_entry = 0
preprocess_error = 0
preprocessed_missing = 0

for _k, v in files.items():
    if not isinstance(v, dict):
        preprocess_error += 1
        continue
    if v.get("excluded_by_compile_commands"):
        excluded += 1
        continue
    included += 1
    if v.get("compile_commands_entry") is None:
        missing_entry += 1
    err = str(v.get("error") or "").strip()
    if err:
        preprocess_error += 1

    pre = v.get("preprocessed_file")
    if not pre:
        preprocessed_missing += 1
    else:
        try:
            if not Path(str(pre)).exists():
                preprocessed_missing += 1
        except Exception:
            preprocessed_missing += 1

# Route A: compile_commands defines "what is built". Excluded files do not require TU closure.
if included == 0:
    sys.exit(2)
sys.exit(0 if (missing_entry == 0 and preprocess_error == 0 and preprocessed_missing == 0) else 1)
PY
            then
                tu_rc=0
            else
                tu_rc=$?
            fi
            if [[ "$tu_rc" -eq 0 ]]; then
                tu_ok_projects+=("$proj_name")
            elif [[ "$tu_rc" -eq 2 ]]; then
                skipped_no_built_sources+=("$proj_name")
            else
                skipped_tu_closure+=("$proj_name")
            fi
        done

        if [[ ${#skipped_tu_closure[@]} -gt 0 ]]; then
            print_warning "以下项目 TU 闭包不完整（缺 TU 条目/预处理失败），将跳过后续阶段: ${skipped_tu_closure[*]}"
            printf "%s\n" "${skipped_tu_closure[@]}" > "$LOG_DIR/tu_closure_skipped_projects.txt" 2>/dev/null || true
        fi
        if [[ ${#skipped_no_built_sources[@]} -gt 0 ]]; then
            print_warning "以下项目在所选 profile 下无可构建源文件（Route A: compile_commands 定义源文件集合），将跳过后续阶段: ${skipped_no_built_sources[*]}"
            printf "%s\n" "${skipped_no_built_sources[@]}" > "$LOG_DIR/route_a_no_built_sources_projects.txt" 2>/dev/null || true
        fi

        projects_to_run=("${tu_ok_projects[@]}")

        if [[ ${#projects_to_run[@]} -eq 0 ]]; then
            print_error "所有项目都因 TU 闭包不完整而跳过后续阶段（可设置 C2R_REQUIRE_TU_CLOSURE=0 关闭门禁）"
            exit 1
        fi
    fi

    # ------------------------------------------------------------------------
    # Truth-mode 门禁：types.rs 必须来自 bindgen（禁止 stub types.rs 继续进入后续阶段）
    # 判定依据：workspace/skeletons/<proj>/types_generation_report.json -> mode != "stub"
    # ------------------------------------------------------------------------
    local skipped_stub_types=()
    if [[ "$C2R_TRUTH_MODE" != "0" ]]; then
        local truth_ok_projects=()
        for proj_name in "${projects_to_run[@]}"; do
            local report_path="$RUN_INTERMEDIATE_DIR/${proj_name}/workspace/skeletons/${proj_name}/types_generation_report.json"
            if python3 - "$report_path" <<'PY' >/dev/null 2>&1; then
import json, sys
from pathlib import Path

p = Path(sys.argv[1])
if not p.exists():
    sys.exit(1)
try:
    data = json.loads(p.read_text(encoding="utf-8", errors="ignore"))
except Exception:
    sys.exit(1)

mode = str(data.get("mode") or "").strip().lower()
success = bool(data.get("success"))
valid = bool(data.get("final_output_valid"))
# Accept either:
# - classic bindgen report: success && valid && mode != stub
# - TU-truth supplement report (headerless or bindgen-failed but recovered from pinned `.i`)
if success and valid and mode and mode != "stub":
    sys.exit(0)
tu_truth = data.get("tu_truth")
if isinstance(tu_truth, dict) and str(tu_truth.get("tu_context_map") or "").strip():
    sys.exit(0)
sys.exit(1)
PY
                truth_ok_projects+=("$proj_name")
            else
                skipped_stub_types+=("$proj_name")
            fi
        done

        if [[ ${#skipped_stub_types[@]} -gt 0 ]]; then
            print_warning "Truth-mode: 以下项目 types.rs 未得到 bindgen 真值（stub/failed/invalid），将跳过后续阶段: ${skipped_stub_types[*]}"
            printf "%s\n" "${skipped_stub_types[@]}" > "$LOG_DIR/truth_mode_skipped_stub_types_projects.txt" 2>/dev/null || true
        fi

        projects_to_run=("${truth_ok_projects[@]}")
        if [[ ${#projects_to_run[@]} -eq 0 ]]; then
            print_error "Truth-mode: 所有项目都因 types.rs=stub 而跳过后续阶段（可设置 C2R_TRUTH_MODE=0 关闭）"
            exit 1
        fi
    fi

    echo ""
    sleep 2

    # ========================================================================
    # Skeleton-only 模式：到此结束
    # ========================================================================
    if [[ "$SKELETON_ONLY" == "true" ]]; then
        # 关闭 vLLM（如果启动了）
        if [[ "$VLLM_STARTED_BY_SCRIPT" == "true" ]]; then
            print_info "骨架翻译完成，关闭 vLLM"
            stop_vllm
            VLLM_STARTED_BY_SCRIPT=false
        fi

        # 取消 trap
        trap - EXIT INT TERM

        # 总结
        local overall_end_time=$(date +%s)
        local overall_duration=$((overall_end_time - overall_start_time))
        local overall_minutes=$((overall_duration / 60))
        local overall_seconds=$((overall_duration % 60))

        print_section "骨架翻译完成！（仅阶段1）"
        echo "总项目数: ${#RECOMMENDED_PROJECTS[@]}"
        echo "运行项目: $total_projects"
        echo "阶段1失败: ${#failed_stage1[@]}"
        echo "总耗时: ${overall_minutes}分${overall_seconds}秒"
        echo ""
        echo "骨架输出目录: translation_outputs/${RUN_DIR_NAME}/intermediate/<项目名>/workspace/skeletons/"

        # 骨架编译统计
        print_info "统计骨架编译情况..."
        local compile_success=0
        local compile_fail=0
        local compile_fail_projects=()
        for proj in "${projects_to_run[@]}"; do
            local skeleton_dir="$RUN_INTERMEDIATE_DIR/${proj}/workspace/skeletons/${proj}"
            if [[ -d "$skeleton_dir" ]]; then
                if cd "$skeleton_dir" && cargo check --quiet --offline 2>/dev/null; then
                    ((compile_success++)) || true
                else
                    ((compile_fail++)) || true
                    compile_fail_projects+=("$proj")
                fi
                cd "$SCRIPT_DIR"
            else
                ((compile_fail++)) || true
                compile_fail_projects+=("$proj")
            fi
        done

        print_section "骨架编译统计"
        echo "编译成功: $compile_success / $total_projects"
        echo "编译失败: $compile_fail / $total_projects"
        if [[ ${#compile_fail_projects[@]} -gt 0 ]]; then
            echo ""
            echo "编译失败的项目 (前20个):"
            for i in "${!compile_fail_projects[@]}"; do
                if [[ $i -lt 20 ]]; then
                    echo "  - ${compile_fail_projects[$i]}"
                fi
            done
            if [[ ${#compile_fail_projects[@]} -gt 20 ]]; then
                echo "  ... 还有 $((${#compile_fail_projects[@]} - 20)) 个"
            fi
        fi
        echo ""

        # 记录并输出 stub 项目列表
        print_stub_projects_summary "$LLM_NAME" "${projects_to_run[@]}"
        print_build_context_problem_projects_summary "$LLM_NAME" "${projects_to_run[@]}"

        if [[ ${#failed_stage1[@]} -gt 0 ]]; then
            exit 1
        else
            exit 0
        fi
    fi

    # ========================================================================
    # 阶段2: 签名匹配 + BM25（不需要vLLM，关闭以释放显存给Jina）
    # ========================================================================
	    if [[ "$RUN_RAG" == true ]]; then
        print_section "========== 阶段2: 签名匹配 + BM25 =========="
    else
        print_section "========== 阶段2: 签名匹配（跳过 RAG）=========="
    fi

    # 关闭 vLLM 以释放显存给 Jina Reranker
    if [[ "$VLLM_STARTED_BY_SCRIPT" == "true" ]]; then
        print_info "阶段2不需要 vLLM，关闭以释放显存给 Jina Reranker..."
        stop_vllm
        VLLM_STARTED_BY_SCRIPT=false
        sleep 3
    fi

    # 并行运行阶段2
    local failed_stage2=()
    local failed_list_file=$(mktemp)
    FAILED_LIST_OUTPUT="$failed_list_file" run_stage_parallel "阶段2: 签名匹配 + BM25" "run_stage2" "${projects_to_run[@]}" || true
    if [[ -f "$failed_list_file" ]] && [[ -s "$failed_list_file" ]]; then
        while IFS= read -r proj_name; do
            [[ -n "$proj_name" ]] && failed_stage2+=("$proj_name")
        done < "$failed_list_file"
    fi
    rm -f "$failed_list_file"

    # 如果阶段2全部失败，退出
    if [[ ${#failed_stage2[@]} -eq ${#projects_to_run[@]} ]]; then
        print_error "所有项目的阶段2都失败了，退出"
        exit 1
    fi

    # 只继续运行阶段2成功的项目
    if [[ ${#failed_stage2[@]} -gt 0 ]]; then
        print_warning "以下项目阶段2失败，将跳过后续阶段: ${failed_stage2[*]}"
        local temp_projects=()
        for proj_name in "${projects_to_run[@]}"; do
            local skip=false
            for failed in "${failed_stage2[@]}"; do
                if [[ "$proj_name" == "$failed" ]]; then
                    skip=true
                    break
                fi
            done
            if [[ "$skip" == false ]]; then
                temp_projects+=("$proj_name")
            fi
        done
        projects_to_run=("${temp_projects[@]}")
    fi

    echo ""
    sleep 2

    # ========================================================================
    # 阶段2.5: Jina Reranker 统一运行（所有项目一起处理）
    # ========================================================================
    if [[ "$RUN_RAG" == true ]]; then
        if [[ "$JINA_SERIAL_MODE" == true ]]; then
            print_section "========== 阶段2.5: Jina Reranker 统一运行（串行模式）=========="
            print_info "使用串行模式运行 Jina Reranker，避免 GPU OOM..."
        else
            print_section "========== 阶段2.5: Jina Reranker 统一运行（智能GPU排队）=========="
            print_info "使用智能GPU排队机制运行 Jina Reranker，最大化GPU利用率并避免OOM..."
        fi

        # 激活 Jina Reranker 环境
	        if ! activate_conda_env "$RERANKER_ENV_NAME"; then
	            print_error "无法激活 Jina Reranker 环境"
	        else
	            # 如果 vLLM 仍在运行（通常表示“外部已启动/脚本未关闭”），为避免 OOM 降低每 GPU 并发槽位。
	            # 下游 `resort_by_unixcoder.py` 会读取 `JINA_MAX_SLOTS_PER_GPU`（默认=3）。
	            if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
	                export JINA_MAX_SLOTS_PER_GPU=2
	                print_info "检测到 vLLM 正在运行：设置 JINA_MAX_SLOTS_PER_GPU=2（避免 Jina Reranker OOM）"
	            fi

	            # 收集所有需要处理的项目
	            local jina_projects=""
	            for proj_name in "${projects_to_run[@]}"; do
	                if [[ -n "$jina_projects" ]]; then
	                    jina_projects="${jina_projects},${proj_name}"
	                else
	                    jina_projects="${proj_name}"
	                fi
	            done

            cd "$SCRIPT_DIR"

            # 根据模式选择运行方式
            if [[ "$JINA_SERIAL_MODE" == true ]]; then
                print_info "准备处理 ${#projects_to_run[@]} 个项目的 Jina Reranker（串行）..."

                if ! python3 run_jina_reranker_queued.py \
                    --projects "$jina_projects" \
                    --intermediate-dir "$RUN_INTERMEDIATE_DIR" \
                    --serial \
                    2>&1 | tee -a "$LOG_DIR/jina_reranker_batch.log"; then
                    print_warning "部分项目 Jina Reranker 失败，继续执行"
                else
                    print_success "所有项目的 Jina Reranker 处理完成"
                fi
            else
                print_info "准备处理 ${#projects_to_run[@]} 个项目的 Jina Reranker（智能GPU排队）..."
                print_info "GPU排队机制：当GPU内存不足时自动等待，避免OOM"

                if ! python3 run_jina_reranker_queued.py \
                    --projects "$jina_projects" \
                    --intermediate-dir "$RUN_INTERMEDIATE_DIR" \
                    --workers "$JINA_WORKERS" \
                    2>&1 | tee -a "$LOG_DIR/jina_reranker_batch.log"; then
                    print_warning "部分项目 Jina Reranker 失败，继续执行"
                else
                    print_success "所有项目的 Jina Reranker 处理完成"
                fi
            fi

            # 切回默认环境
            activate_conda_env "$VIRTUAL_ENV_NAME" >/dev/null 2>&1 || true
        fi

        echo ""
        sleep 2
    fi

    # ========================================================================
    # 阶段3: 函数体翻译 + 测试 + 修复（需要 LLM；默认使用本地 vLLM）
    # ========================================================================
    print_section "========== 阶段3: 函数体翻译 + 测试 + 修复 =========="

    # 阶段3 需要 LLM。默认使用本地 vLLM（USE_VLLM=true），并在此处统一启动，避免每项目重复启动。
    # 若你希望使用外部 API，请设置：USE_VLLM=false 并提供 EXTERNAL_API_KEY（且需要可用网络）。
    if [[ "${USE_VLLM:-true}" =~ ^(true|1|yes)$ ]]; then
        if ! curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
            print_info "阶段3需要本地 vLLM，启动 vLLM 服务..."
            if ! start_vllm; then
                print_error "无法启动 vLLM：阶段3可能失败（可改用 USE_VLLM=false + EXTERNAL_API_KEY）"
            else
                if [[ "${VLLM_START_ACTION:-}" == "started" ]]; then
                    VLLM_STARTED_BY_SCRIPT=true
                fi
            fi
        else
            print_info "vLLM 已就绪: $VLLM_URL"
        fi
    else
        print_info "USE_VLLM=false：阶段3将使用外部 API（需 EXTERNAL_API_KEY + 可用网络）"
    fi
    echo ""

    # 并行运行阶段3
    local failed_stage3=()
    local failed_list_file=$(mktemp)
    FAILED_LIST_OUTPUT="$failed_list_file" run_stage_parallel "阶段3: 函数体翻译 + 测试 + 修复" "run_stage3" "${projects_to_run[@]}" || true
    if [[ -f "$failed_list_file" ]] && [[ -s "$failed_list_file" ]]; then
        while IFS= read -r proj_name; do
            [[ -n "$proj_name" ]] && failed_stage3+=("$proj_name")
        done < "$failed_list_file"
    fi
    rm -f "$failed_list_file"

    # 注意：此处不要提前关闭 vLLM。
    # 后续的“知识沉淀（Learned KB）”在启用 --extract-knowledge 时仍需要 vLLM（或外部 API）。

    # ========================================================================
    # 总结
    # ========================================================================
    local overall_end_time=$(date +%s)
    local overall_duration=$((overall_end_time - overall_start_time))
    local overall_minutes=$((overall_duration / 60))
    local overall_seconds=$((overall_duration % 60))

    print_section "批量测试完成！"
    echo "总项目数: ${#RECOMMENDED_PROJECTS[@]}"
    echo "运行项目: $total_projects"
    echo "阶段1失败: ${#failed_stage1[@]}"
    if [[ "$C2R_REQUIRE_TU_CLOSURE" != "0" ]]; then
        echo "TU 闭包不完整跳过: ${#skipped_tu_closure[@]}"
        echo "RouteA 无可构建源文件跳过: ${#skipped_no_built_sources[@]}"
    fi
    if [[ "$C2R_TRUTH_MODE" != "0" ]]; then
        echo "Truth-mode stub types 跳过: ${#skipped_stub_types[@]}"
    fi
    echo "阶段2失败: ${#failed_stage2[@]}"
    echo "阶段3失败: ${#failed_stage3[@]}"
    echo "总耗时: ${overall_minutes}分${overall_seconds}秒"
    echo ""

    if [[ "$C2R_REQUIRE_TU_CLOSURE" != "0" ]] && [[ ${#skipped_tu_closure[@]} -gt 0 ]]; then
        print_section "未进入后续翻译的项目（TU 闭包不完整）"
        for proj_name in "${skipped_tu_closure[@]}"; do
            echo "  - $proj_name"
        done
        echo ""
        echo "已保存: $LOG_DIR/tu_closure_skipped_projects.txt"
        echo ""
    fi

    if [[ "$C2R_REQUIRE_TU_CLOSURE" != "0" ]] && [[ ${#skipped_no_built_sources[@]} -gt 0 ]]; then
        print_section "未进入后续翻译的项目（Route A: 所选 profile 下无可构建源文件）"
        for proj_name in "${skipped_no_built_sources[@]}"; do
            echo "  - $proj_name"
        done
        echo ""
        echo "已保存: $LOG_DIR/route_a_no_built_sources_projects.txt"
        echo ""
    fi

    # Route A report: projects that contain files excluded as "not built in selected profile".
    # This is not a TU-closure failure; it reflects multi-variant source trees.
    if [[ "$C2R_REQUIRE_TU_CLOSURE" != "0" ]]; then
        local route_a_excluded_report="$LOG_DIR/route_a_excluded_sources_projects.txt"
        python3 - "$RUN_INTERMEDIATE_DIR" > "$route_a_excluded_report" 2>/dev/null <<'PY' || true
import json, sys
from pathlib import Path

root = Path(sys.argv[1])
projects = []
for p in sorted(root.iterdir()):
    if not p.is_dir() or p.name in ("logs", "cache"):
        continue
    tu = p / "workspace" / ".preprocessed" / "tu_context_map.json"
    if not tu.exists():
        continue
    try:
        data = json.loads(tu.read_text(encoding="utf-8", errors="ignore") or "{}")
    except Exception:
        continue
    files = data.get("files")
    if not isinstance(files, dict) or not files:
        continue
    excluded = 0
    included = 0
    for _k, rec in files.items():
        if not isinstance(rec, dict):
            continue
        if rec.get("excluded_by_compile_commands"):
            excluded += 1
        else:
            included += 1
    if excluded > 0 and included > 0:
        projects.append((p.name, excluded, included))

for name, excluded, included in projects:
    print(f\"{name}\\texcluded={excluded}\\tincluded={included}\")
PY
        if [[ -s "$route_a_excluded_report" ]]; then
            print_section "Route A: 项目包含未编译源文件（已按 compile_commands 排除）"
            while IFS= read -r line; do
                echo "  - $line"
            done < "$route_a_excluded_report"
            echo ""
            echo "已保存: $route_a_excluded_report"
            echo ""
        fi
    fi

    if [[ "$C2R_TRUTH_MODE" != "0" ]] && [[ ${#skipped_stub_types[@]} -gt 0 ]]; then
        print_section "未进入后续翻译的项目（Truth-mode: types.rs 为 stub）"
        for proj_name in "${skipped_stub_types[@]}"; do
            echo "  - $proj_name"
        done
        echo ""
        echo "已保存: $LOG_DIR/truth_mode_skipped_stub_types_projects.txt"
        echo ""
    fi

    # ========================================================================
    # 生成翻译结果汇总报告 JSON
    # ========================================================================
    print_section "生成翻译结果汇总报告"
    if activate_conda_env "$VIRTUAL_ENV_NAME" >/dev/null 2>&1; then
        cd "$SCRIPT_DIR"
        if [[ -f "$SCRIPT_DIR/generate_summary_report.py" ]]; then
            python3 "$SCRIPT_DIR/generate_summary_report.py" \
                --outputs-dir "$RUN_INTERMEDIATE_DIR" \
                --report-out "$RUN_RESULTS_DIR/translation_summary_report.json" \
                2>&1 || print_warning "报告生成失败"
        else
            print_warning "未找到 generate_summary_report.py，跳过报告生成"
        fi
    else
        print_warning "无法激活 conda 环境，跳过报告生成"
    fi
    echo ""

    # 输出 stub 项目列表
    print_stub_projects_summary "$LLM_NAME" "${projects_to_run[@]}"
    print_build_context_problem_projects_summary "$LLM_NAME" "${projects_to_run[@]}"

    # ========================================================================
    # Post-run: unsafe 比例 / CodeBLEU proxy / 失败分布 + Learned KB
    # ========================================================================
    print_section "Post-run 分析（unsafe/CodeBLEU/失败分布）"
    if [[ -f "$SCRIPT_DIR/post_run_analysis.py" ]]; then
        python3 "$SCRIPT_DIR/post_run_analysis.py" \
            --run-intermediate-dir "$RUN_INTERMEDIATE_DIR" \
            --run-results-dir "$RUN_RESULTS_DIR" \
            --llm-name "$LLM_NAME" \
            2>&1 | tee -a "$LOG_DIR/post_run_analysis.log" || \
            print_warning "post_run_analysis.py 运行失败（已忽略）"
    else
        print_warning "未找到 post_run_analysis.py，跳过 post-run 分析"
    fi

    # NOTE (open-source cleanup):
    # We no longer run the paper-style OHOS(test5) evaluation at the end of the framework pipeline.
    # If you need paper metrics (CR/FC/Unsafe/Clippy), run the scripts under repo `scripts/` manually.

    if [[ "$SKIP_LEARNED_KB" == "true" ]]; then
        print_info "跳过知识沉淀（--skip-learned-kb）"
    else
        print_section "知识沉淀（Learned KB，可独立删除）"
        if [[ -f "$SCRIPT_DIR/distill_learned_kb.py" ]]; then
	            if ls "$RUN_INTERMEDIATE_DIR"/*/workspace/incremental_work/*/translate_by_* >/dev/null 2>&1; then
	                LEARNED_KB_DIR="$OUTPUTS_BASE_DIR/shared/learned_kb/$RUN_DIR_NAME"
	                local distill_extract_args="--extract-knowledge --vllm-resume"

	                if [[ "${USE_VLLM:-true}" =~ ^(true|1|yes)$ ]]; then
	                    # vLLM 模式：知识沉淀依赖本地 vLLM
	                    print_info "知识沉淀使用本地 vLLM..."

	                    # 确保本地 vLLM 正在运行（如果阶段4已启动则复用）
	                    if ! curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
	                        print_info "启动 vLLM 服务（知识沉淀需要）..."
	                        if ! start_vllm; then
	                            print_error "无法启动 vLLM，跳过知识沉淀"
	                        else
	                            if [[ "${VLLM_START_ACTION:-}" == "started" ]]; then
	                                VLLM_STARTED_BY_SCRIPT=true
	                            fi
	                        fi
	                    fi

	                    if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
	                        print_info "vLLM 已就绪: $VLLM_URL"
	                        python3 "$SCRIPT_DIR/distill_learned_kb.py" \
	                            --run-intermediate-dir "$RUN_INTERMEDIATE_DIR" \
	                            --out-dir "$LEARNED_KB_DIR" \
	                            --llm-name "$LLM_NAME" \
	                            --vllm-base-url "$VLLM_URL" \
	                            --vllm-model "$LLM_NAME" \
	                            $distill_extract_args \
	                            2>&1 | tee -a "$LOG_DIR/distill_learned_kb.log" || \
	                            print_warning "distill_learned_kb.py 运行失败（已忽略）"
	                    else
	                        print_warning "vLLM 服务不可用，跳过知识沉淀"
	                    fi
	                else
	                    # 外部 API 模式：不启动 vLLM；distill 脚本将从 generation.py 读取 EXTERNAL_API_* 配置
	                    print_info "知识沉淀使用外部 API（USE_VLLM=false）"
	                    if [[ -z "${EXTERNAL_API_KEY:-}" ]]; then
	                        print_warning "EXTERNAL_API_KEY 未设置，跳过知识沉淀"
	                    else
	                        python3 "$SCRIPT_DIR/distill_learned_kb.py" \
	                            --run-intermediate-dir "$RUN_INTERMEDIATE_DIR" \
	                            --out-dir "$LEARNED_KB_DIR" \
	                            --llm-name "$LLM_NAME" \
	                            $distill_extract_args \
	                            2>&1 | tee -a "$LOG_DIR/distill_learned_kb.log" || \
	                            print_warning "distill_learned_kb.py 运行失败（已忽略）"
	                    fi
	                fi

	                echo ""
	                print_info "Learned KB 目录: $LEARNED_KB_DIR"
	                print_info "删除命令: rm -rf $LEARNED_KB_DIR"
	                print_info "下次运行可叠加使用（不修改原KB）:"
	                print_info "  bash batch_test_staged.sh --extra-rag-kb-dir \"$LEARNED_KB_DIR/rag\" ..."
	                print_info "  （等价环境变量：C2R_EXTRA_RAG_KB_DIR=\"$LEARNED_KB_DIR/rag\"）"
	            else
	                print_info "未检测到阶段3输出（incremental_work），跳过 Learned KB 生成"
	            fi
	        else
            print_warning "未找到 distill_learned_kb.py，跳过知识沉淀"
        fi
    fi

	    # 关闭 vLLM（在所有需要 vLLM 的阶段完成后）
	    if [[ "$VLLM_STARTED_BY_SCRIPT" == "true" ]]; then
	        print_info "流水线执行完成，关闭 vLLM..."
	        stop_vllm
	        VLLM_STARTED_BY_SCRIPT=false
	    fi

	    # 取消 trap（正常完成，不需要清理）
	    trap - EXIT INT TERM

	    # 返回适当的退出码
	    if [[ ${#failed_stage3[@]} -gt 0 ]] || [[ ${#failed_stage2[@]} -gt 0 ]] || [[ ${#failed_stage1[@]} -gt 0 ]]; then
	        exit 1
	    else
        exit 0
    fi
}

# 运行主函数
main "$@"
