#!/bin/bash

################################################################################
# C2Rust 完整翻译流程脚本
# 
# 使用方法：
#   bash get_function.sh [选项]
#
# 选项：
#   --project, -p NAME         项目名称 (默认: filemanagement_dfs_service)
#   --project-root PATH        项目路径 (默认: workspace/projects/$PROJECT_NAME)
#   --llm NAME                 LLM 模型名称 (默认: gpt_5_nano)
#   --env NAME                 虚拟环境名称 (默认: c2r_frame)
#   
#   --force-all                强制重新执行所有步骤 (等同于 --no-skip)
#   --no-skip                  禁用智能跳过
#   --force-skeleton           强制重新翻译骨架
#   --force-functions          强制重新翻译函数体
#   --force-test               强制重新测试
#   --force-repair             强制重新修复
#   
#   --workers-skeleton N       骨架翻译并行数 (默认: 160)
#   --workers-function N       函数体翻译并行数 (默认: 160)
#   --workers-test N           编译测试并行数 (默认: 4096)
#   --workers-repair N         自动修复并行数 (默认: 48)
#   
#   --no-vllm                  禁用 vLLM (使用传统 API)
#   --vllm-url URL             vLLM 服务器地址 (默认: http://localhost:8000/v1)
#   --vllm-model NAME          vLLM 模型名称 (默认: qwen3_coder)
#   --vllm-workers N           vLLM 并行数 (默认: 160)
#   
#   --incremental              使用增量式翻译模式（填一个测一个，即时修复）
#   --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)
#   
#   --no-progress              隐藏进度条
#   --help, -h                 显示帮助信息
#
# 示例：
#   bash get_function.sh --project my_project --force-all
#   bash get_function.sh -p my_project --force-functions --force-test
#   bash get_function.sh --llm qwen3 --workers-function 200
################################################################################

# 不设置 set -e，因为测试和修复步骤允许部分失败

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_step() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# ============================================================================
# 显示帮助信息
# ============================================================================
show_help() {
    head -n 40 "$0" | tail -n +3 | sed 's/^# //' | sed 's/^#//'
    exit 0
}

# ============================================================================
# 默认配置（最可能的情况）
# ============================================================================
PROJECT_NAME="filemanagement_dfs_service"
PROJECT_ROOT=""  # 稍后根据 PROJECT_NAME 设置
VIRTUAL_ENV_NAME="c2r_frame"
LLM_NAME="gpt_5_nano"

# 并行处理配置
TRANSLATE_MAX_WORKERS="160"
TRANSLATE_FUNCTION_MAX_WORKERS="160"
TEST_MAX_WORKERS="4096"
REPAIR_MAX_WORKERS="48"

# 智能跳过与强制执行控制（默认启用智能跳过）
SMART_SKIP="1"
FORCE_SKELETON="0"
FORCE_FUNCTIONS="0"
FORCE_TEST="0"
FORCE_REPAIR="0"

# 输出控制
NO_PROGRESS="0"

# 增量翻译模式
USE_INCREMENTAL="0"
MAX_REPAIR_ATTEMPTS="5"

# vLLM 配置
USE_VLLM="true"
VLLM_BASE_URL="http://localhost:8000/v1"
VLLM_API_KEY="EMPTY"
VLLM_MODEL_NAME="qwen3_coder"
VLLM_NUM_WORKERS="160"

# ============================================================================
# 解析命令行参数
# ============================================================================
while [[ $# -gt 0 ]]; do
    case $1 in
        --project|-p)
            PROJECT_NAME="$2"
            shift 2
            ;;
        --project-root)
            PROJECT_ROOT="$2"
            shift 2
            ;;
        --llm)
            LLM_NAME="$2"
            shift 2
            ;;
        --env)
            VIRTUAL_ENV_NAME="$2"
            shift 2
            ;;
        --force-all|--no-skip)
            SMART_SKIP="0"
            shift
            ;;
        --force-skeleton)
            FORCE_SKELETON="1"
            shift
            ;;
        --force-functions)
            FORCE_FUNCTIONS="1"
            shift
            ;;
        --force-test)
            FORCE_TEST="1"
            shift
            ;;
        --force-repair)
            FORCE_REPAIR="1"
            shift
            ;;
        --workers-skeleton)
            TRANSLATE_MAX_WORKERS="$2"
            shift 2
            ;;
        --workers-function)
            TRANSLATE_FUNCTION_MAX_WORKERS="$2"
            shift 2
            ;;
        --workers-test)
            TEST_MAX_WORKERS="$2"
            shift 2
            ;;
        --workers-repair)
            REPAIR_MAX_WORKERS="$2"
            shift 2
            ;;
        --no-vllm)
            USE_VLLM="false"
            shift
            ;;
        --vllm-url)
            VLLM_BASE_URL="$2"
            shift 2
            ;;
        --vllm-model)
            VLLM_MODEL_NAME="$2"
            shift 2
            ;;
        --vllm-workers)
            VLLM_NUM_WORKERS="$2"
            shift 2
            ;;
        --no-progress)
            NO_PROGRESS="1"
            shift
            ;;
        --incremental)
            USE_INCREMENTAL="1"
            shift
            ;;
        --max-repair)
            MAX_REPAIR_ATTEMPTS="$2"
            shift 2
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "未知参数: $1"
            echo "使用 --help 查看帮助信息"
            exit 1
            ;;
    esac
done

# 环境变量覆盖（优先级低于命令行参数）
PROJECT_NAME=${PROJECT_NAME:-${C2R_PROJECT_NAME:-"filemanagement_dfs_service"}}
VIRTUAL_ENV_NAME=${VIRTUAL_ENV_NAME:-${C2R_ENV:-"c2r_frame"}}
LLM_NAME=${LLM_NAME:-${C2R_LLM:-"gpt_5_nano"}}

# 工作空间根目录（支持通过环境变量 C2R_WORKSPACE_ROOT 自定义）
WORKSPACE_BASE="${C2R_WORKSPACE_ROOT:-workspace}"

# 如果未指定 PROJECT_ROOT，则根据 PROJECT_NAME 自动设置
if [ -z "$PROJECT_ROOT" ]; then
    PROJECT_ROOT="${WORKSPACE_BASE}/projects/$PROJECT_NAME"
fi

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 转换为绝对路径
if [[ ! "$PROJECT_ROOT" = /* ]]; then
    PROJECT_ROOT="$SCRIPT_DIR/$PROJECT_ROOT"
fi

# 转换 WORKSPACE_BASE 为绝对路径
if [[ ! "$WORKSPACE_BASE" = /* ]]; then
    WORKSPACE_BASE="$SCRIPT_DIR/$WORKSPACE_BASE"
fi

# --- 日志配置 ---
# 创建日志目录
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

# 生成日志文件名（包含时间戳和项目名）
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/get_function_${PROJECT_NAME}_${TIMESTAMP}.log"

# 同时输出到终端和日志文件
exec > >(tee -a "$LOG_FILE") 2>&1

print_info() {
    # 注意：由于 exec 已经将输出重定向到 tee，这里不需要再用 tee
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

print_info "=========================================="
print_info "开始执行 C2Rust 翻译流程"
print_info "项目: $PROJECT_NAME"
print_info "日志文件: $LOG_FILE"
print_info "=========================================="

# --- 虚拟环境激活 ---
print_step "激活虚拟环境: $VIRTUAL_ENV_NAME"

# 初始化 conda（如果还没有）
if command -v conda &> /dev/null; then
    eval "$(conda shell.bash hook 2>/dev/null)"
    if conda env list | grep -q "^$VIRTUAL_ENV_NAME "; then
        conda activate "$VIRTUAL_ENV_NAME"
        if [ "$CONDA_DEFAULT_ENV" = "$VIRTUAL_ENV_NAME" ]; then
            print_success "已激活 conda 环境: $VIRTUAL_ENV_NAME"
            echo "  Python 路径: $(which python)"
            echo "  Python 版本: $(python --version 2>&1)"
        else
            print_error "conda 环境激活失败"
            exit 1
        fi
    else
        print_error "conda 环境中未找到: $VIRTUAL_ENV_NAME"
        echo "可用环境列表："
        conda env list
        exit 1
    fi
else
    print_error "未找到 conda 命令"
    exit 1
fi

# --- 配置检查 ---
print_step "配置检查"

print_success "项目配置（硬编码）："
echo "  项目名称: $PROJECT_NAME"
echo "  项目路径: $PROJECT_ROOT"
echo "  LLM 模型: $LLM_NAME"
echo "  虚拟环境: $VIRTUAL_ENV_NAME"
if [ "$USE_VLLM" = "true" ] || [ "$USE_VLLM" = "1" ] || [ "$USE_VLLM" = "yes" ]; then
    echo "  vLLM 模式: 启用"
    echo "  vLLM 地址: $VLLM_BASE_URL"
    echo "  vLLM 模型: $VLLM_MODEL_NAME"
    echo "  vLLM 并行数: $VLLM_NUM_WORKERS"
else
    echo "  vLLM 模式: 禁用（使用传统 API）"
fi
echo "  并行配置:"
echo "    TRANSLATE_MAX_WORKERS: $TRANSLATE_MAX_WORKERS"
echo "    TRANSLATE_FUNCTION_MAX_WORKERS: $TRANSLATE_FUNCTION_MAX_WORKERS"
echo "    TEST_MAX_WORKERS: $TEST_MAX_WORKERS"
echo "    REPAIR_MAX_WORKERS: $REPAIR_MAX_WORKERS"
echo "  控制开关:"
echo "    SMART_SKIP: $SMART_SKIP (1=启用智能跳过)"
echo "    FORCE_SKELETON: $FORCE_SKELETON (1=强制重译骨架)"
echo "    FORCE_FUNCTIONS: $FORCE_FUNCTIONS (1=强制重译函数体)"
echo "    FORCE_TEST: $FORCE_TEST (1=强制重新测试)"
echo "    FORCE_REPAIR: $FORCE_REPAIR (1=强制修复)"
echo "    NO_PROGRESS: $NO_PROGRESS (1=隐藏进度条)"

# 检查项目目录是否存在
if [ ! -d "$PROJECT_ROOT" ]; then
    print_error "项目目录不存在: $PROJECT_ROOT"
    print_error "请检查项目路径是否正确"
    exit 1
fi

print_success "项目目录存在: $PROJECT_ROOT"

# 导出环境变量，供 Python 脚本使用
export PROJECT_NAME
export PROJECT_ROOT
export LLM_NAME
# 关键：导出工作空间路径给所有 Python 脚本使用
# 必须在所有 Python 脚本调用前设置，因为 workspace_config.py 在导入时读取
export C2R_WORKSPACE_ROOT="${WORKSPACE_BASE}"
# 导出控制变量
export SMART_SKIP FORCE_SKELETON FORCE_FUNCTIONS FORCE_TEST FORCE_REPAIR NO_PROGRESS

# 导出 vLLM 配置
export USE_VLLM
export VLLM_BASE_URL
export VLLM_API_KEY
export VLLM_MODEL_NAME
export VLLM_NUM_WORKERS
# vLLM 客户端超时（秒），默认 600，可外部覆盖
export VLLM_REQUEST_TIMEOUT=${VLLM_REQUEST_TIMEOUT:-600}
# vLLM 单次生成最大新token数，默认 2048，可外部覆盖
# Default higher to avoid truncation (can be overridden by env).
export VLLM_MAX_TOKENS=${VLLM_MAX_TOKENS:-16384}

# 设置 vLLM 显存占用率 (默认 0.6，即 60%，可通过环境变量覆盖)
# 注意：如果 GPU 上已有其他进程占用显存，需要降低此值
# 如果环境变量未设置，强制使用 0.6；如果已设置，则使用设置的值
if [ -z "$VLLM_GPU_MEMORY_UTILIZATION" ]; then
    export VLLM_GPU_MEMORY_UTILIZATION=0.6
    echo "  使用默认显存利用率: 0.6 (60%)"
else
    # 将小数转换为百分比显示（例如 0.6 -> 60%）
    PERCENT=$(awk "BEGIN {printf \"%.0f\", $VLLM_GPU_MEMORY_UTILIZATION * 100}")
    echo "  使用环境变量显存利用率: $VLLM_GPU_MEMORY_UTILIZATION (${PERCENT}%)"
fi

echo ""

# --- 性能加速环境（仅影响编译速度，不改变业务逻辑） ---
print_step "启用编译性能优化环境变量"
# 共享构建产物，避免每个临时工程重复编译
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/workspace/.cargo_target_shared}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
# 增量编译
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-1}"
# rustc 内部并行度（默认值，可被外部覆盖）
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-32}"
# 更快的链接器与并行代码生成
if command -v clang >/dev/null 2>&1; then
  if command -v ld.lld >/dev/null 2>&1 || command -v lld >/dev/null 2>&1; then
    export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16} -C linker=clang -C link-arg=-fuse-ld=lld"
  else
    export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16}"
  fi
else
  export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16}"
fi
# sccache（如果存在则启用）
if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="$(command -v sccache)"
  sccache --start-server >/dev/null 2>&1 || true
  echo "  已启用 sccache: $RUSTC_WRAPPER"
else
  echo "  未找到 sccache，跳过该优化（可选安装以提速）"
fi
echo "  CARGO_TARGET_DIR: $CARGO_TARGET_DIR"
echo "  CARGO_INCREMENTAL: $CARGO_INCREMENTAL"
# echo "  CARGO_BUILD_JOBS: $CARGO_BUILD_JOBS"
echo "  RUSTFLAGS: $RUSTFLAGS"
echo ""

# --- 准备工作 ---
print_step "步骤 0: 准备工作"

# vLLM 按需启动/停止函数
VLLM_HOST="${VLLM_HOST:-127.0.0.1}"
VLLM_PORT="${VLLM_PORT:-8000}"
VLLM_URL="${VLLM_BASE_URL:-http://$VLLM_HOST:$VLLM_PORT/v1}"

start_vllm() {
  if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
    echo "vLLM 已在运行: $VLLM_URL"
    return 0
  fi
  DEPLOY_VLLM_SCRIPT="${DEPLOY_VLLM_SCRIPT:-}"
  if [ -z "$DEPLOY_VLLM_SCRIPT" ]; then
    if [ -f "$SCRIPT_DIR/qwen3_coder/deploy_no_quantization.sh" ]; then
      DEPLOY_VLLM_SCRIPT="$SCRIPT_DIR/qwen3_coder/deploy_no_quantization.sh"
    elif [ -f "$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_no_quantization.sh" ]; then
      DEPLOY_VLLM_SCRIPT="$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_no_quantization.sh"
    else
      print_error "未找到 deploy_no_quantization.sh，请设置 DEPLOY_VLLM_SCRIPT"
      return 1
    fi
  fi
  echo "启动 vLLM: $DEPLOY_VLLM_SCRIPT"
  VLLM_LOG="$LOG_DIR/vllm_$(date +%Y%m%d_%H%M%S).log"
  # 使用 unbuffered 模式确保实时输出到日志文件
  # stdbuf -oL 表示行缓冲 stdout，-eL 表示行缓冲 stderr，确保日志实时写入
  nohup bash -c "stdbuf -oL -eL bash '$DEPLOY_VLLM_SCRIPT'" >"$VLLM_LOG" 2>&1 &
  VLLM_PID=$!
  echo "vLLM 启动中 (pid=$VLLM_PID)，日志: $VLLM_LOG"
  echo "  提示: 可使用 'tail -f $VLLM_LOG' 实时查看 vLLM 日志"
  MAX_WAIT=300
  WAITED=0
  until curl -s "$VLLM_URL/models" >/dev/null 2>&1; do
    sleep 2
    WAITED=$((WAITED+2))
    if [ $WAITED -ge $MAX_WAIT ]; then
      print_error "vLLM 启动超时（>$MAX_WAIT 秒）。日志: $VLLM_LOG"
      return 1
    fi
  done
  echo "vLLM 已就绪: $VLLM_URL"
  return 0
}

stop_vllm() {
  # 查找并关闭所有 vLLM 相关进程
  # 1. 关闭 API server 进程
  pkill -f "vllm.entrypoints.openai.api_server" >/dev/null 2>&1 || true
  # 2. 关闭 EngineCore 进程
  pkill -f "EngineCore" >/dev/null 2>&1 || true
  # 3. 关闭 WorkerProc 进程
  pkill -f "WorkerProc" >/dev/null 2>&1 || true
  # 4. 等待进程完全退出
  sleep 2
  # 5. 如果还有残留进程，强制杀死
  pkill -9 -f "vllm" >/dev/null 2>&1 || true
  # 6. 验证是否已关闭
  if pgrep -f "vllm.entrypoints.openai.api_server" >/dev/null 2>&1; then
    echo "警告: vLLM 进程可能仍在运行"
    return 1
  else
    return 0
  fi
}

# 检查知识库文件是否存在（-e 检查包括软链接）
KB_FILE="${WORKSPACE_BASE}/rag/knowledge_base.json"
BM25_FILE="${WORKSPACE_BASE}/rag/bm25_index.pkl"

if [ ! -e "$KB_FILE" ] || [ ! -e "$BM25_FILE" ]; then
    print_warning "知识库文件不存在，需要先构建知识库"
    print_info "  检查路径: $KB_FILE"
    print_info "  检查路径: $BM25_FILE"
    print_step "步骤 0.1: 构建知识库"
    
    # 检查 filtered_code_pairs 目录
    if [ ! -d "../qwen3_coder/filtered_code_pairs" ]; then
        print_error "知识库源目录不存在: ../qwen3_coder/filtered_code_pairs"
        print_error "请先运行 process_judgments.py 生成代码对"
        print_error "或者手动创建 filtered_code_pairs 目录并放入 C/Rust 代码对"
        exit 1
    fi
    
    python3 build_knowledge_base.py
    if [ $? -eq 0 ]; then
        print_success "知识库构建完成"
    else
        print_error "知识库构建失败"
        exit 1
    fi
else
    print_success "知识库文件已存在，跳过构建"
    # 显示知识库路径信息
    if [ -L "$KB_FILE" ]; then
        print_info "  知识库 (软链接): $KB_FILE -> $(readlink -f $KB_FILE)"
    else
        print_info "  知识库: $KB_FILE"
    fi
fi

# 设置退出时自动关闭 vLLM（批量测试模式下不关闭）
cleanup_vllm() {
    if [[ -z "$SKIP_VLLM_MANAGEMENT" ]]; then
        print_info "正在关闭 vLLM 服务..."
        stop_vllm
        print_success "vLLM 已关闭"
    fi
}
trap cleanup_vllm EXIT

echo ""

# --- 翻译流程 ---
print_step "步骤 1: 分析 C/C++ 依赖"
# get_dependencies.py 通过环境变量 PROJECT_NAME 和 PROJECT_ROOT 获取配置
if python3 get_dependencies.py --project "$PROJECT_NAME"; then
    print_success "依赖分析完成"
else
    print_error "依赖分析失败"
    exit 1
fi

echo ""

print_step "步骤 2: 翻译代码骨架（包含编译修复）"
# translate.py 通过环境变量 PROJECT_NAME 和 PROJECT_ROOT 获取配置
# 设置并行数量环境变量
export TRANSLATE_MAX_WORKERS
# 智能跳过：若骨架完整存在且修复结果也存在，则跳过；可用 FORCE_SKELETON=1 覆盖
SKE_DIR="${WORKSPACE_BASE}/skeletons/${PROJECT_NAME}/src"
SRC_SKE_DIR="${WORKSPACE_BASE}/source_skeletons/${PROJECT_NAME}/src"
REPAIR_SKE_DIR="${WORKSPACE_BASE}/repair_results/${PROJECT_NAME}/translate_by_default/round_3/src" # 骨架修复的最终结果目录
SKE_RS_COUNT=$(ls -1 ${SKE_DIR}/*.rs 2>/dev/null | wc -l | tr -d ' ')
SRC_TXT_COUNT=$(ls -1 ${SRC_SKE_DIR}/*.txt 2>/dev/null | wc -l | tr -d ' ')
REPAIR_SKE_RS_COUNT=$(ls -1 ${REPAIR_SKE_DIR}/*.rs 2>/dev/null | wc -l | tr -d ' ')
RUN_SKELETON="1"
if [ "$SMART_SKIP" = "1" ] && [ "$FORCE_SKELETON" != "1" ]; then
  if [ "$SKE_RS_COUNT" -gt 0 ] && [ "$SRC_TXT_COUNT" -gt 0 ] && [ "$SKE_RS_COUNT" -ge "$SRC_TXT_COUNT" ] && \
     [ "$REPAIR_SKE_RS_COUNT" -gt 0 ] && [ "$REPAIR_SKE_RS_COUNT" -ge "$SRC_TXT_COUNT" ]; then
    RUN_SKELETON="0"
  fi
fi
if [ "$RUN_SKELETON" = "0" ]; then
    print_success "检测到骨架已完整存在且已修复（${SKE_RS_COUNT}/${SRC_TXT_COUNT}，修复结果：${REPAIR_SKE_RS_COUNT}），跳过骨架翻译与修复"
else 
    # 启动 vLLM（用于骨架翻译）
    print_info "启动 vLLM（用于骨架翻译与修复）"
    start_vllm || exit 1
    # translate.py 内部会调用 translate_skeleton(), create_main(), add_dependencies(), repair()
    # 注意：translate.py 的 if __name__ == "__main__" 会执行完整流程（包括修复）
    if python3 translate.py --project "$PROJECT_NAME"; then
        print_success "骨架翻译与修复完成"
    else
        print_error "骨架翻译与修复失败"
        exit 1
    fi
    # 关闭 vLLM（为后续 Jina Reranker 释放显存）
    # 但在批量测试模式下，不关闭 vLLM（由批量脚本统一管理）
    if [[ -z "$SKIP_VLLM_MANAGEMENT" ]]; then
        print_info "关闭 vLLM（为 Jina Reranker 释放显存）"
    stop_vllm
    else
        print_info "批量测试模式：保持 vLLM 运行（由批量脚本统一管理）"
    fi
fi

echo ""

print_step "步骤 3: 匹配 C++ 函数和 Rust 函数签名"
# get_dependencies_match.py 第一个参数是 LLM 名称，通过环境变量获取项目配置
if python3 get_dependencies_match.py "$LLM_NAME"; then
    print_success "函数签名匹配完成"
else
    print_error "函数签名匹配失败"
    exit 1
fi

echo ""

print_step "步骤 4: RAG 检索 - BM25 粗筛 [Top-100]"
if python3 elastic_search.py; then
    print_success "BM25 检索完成"
else
    print_error "BM25 检索失败"
    exit 1
fi

echo ""

print_step "步骤 5: RAG 检索 - Jina-Reranker 精排 [Top-2]"
if python3 resort_by_unixcoder.py; then
    print_success "Jina 重排完成"
else
    print_error "Jina 重排失败"
    exit 1
fi

echo ""

# ============================================================================
# 增量翻译模式 vs 传统并行模式
# ============================================================================
if [ "$USE_INCREMENTAL" = "1" ]; then
    # 增量式翻译模式：填一个测一个，即时修复
    print_step "步骤 7-11: 增量式翻译与验证（填一个测一个）"
    print_info "使用增量式翻译模式..."
    print_info "  - 拓扑排序：按依赖关系决定翻译顺序"
    print_info "  - 增量验证：每翻译一个函数立即编译测试"
    print_info "  - 即时修复：编译失败立即尝试修复（最多 ${MAX_REPAIR_ATTEMPTS} 次）"
    
    # 启动 vLLM
    print_info "启动 vLLM（用于增量翻译与修复）"
    start_vllm || exit 1
    
    # 运行增量翻译
    export C2R_WORKSPACE_ROOT="${WORKSPACE_BASE}"
    if python3 incremental_translate.py "$PROJECT_NAME" "$LLM_NAME" "$MAX_REPAIR_ATTEMPTS"; then
        print_success "增量式翻译完成"
    else
        print_warning "增量式翻译完成（可能仍有未通过的函数）"
    fi
else
    # 传统并行模式：先全部翻译，再全部测试，再全部修复
print_step "步骤 7: 核心翻译 - 填充函数体"
# 设置并行数量环境变量
export TRANSLATE_FUNCTION_MAX_WORKERS

TRANS_DIR="${WORKSPACE_BASE}/translated/${PROJECT_NAME}/translate_by_${LLM_NAME}"
FUNC_SRC_DIR="${WORKSPACE_BASE}/extracted/${PROJECT_NAME}/functions"
TRANS_COUNT=$(ls -1 ${TRANS_DIR}/*.txt 2>/dev/null | wc -l | tr -d ' ')
FUNC_TOTAL=$(ls -1 ${FUNC_SRC_DIR}/*.txt 2>/dev/null | wc -l | tr -d ' ')
RUN_FUNCTIONS="1"
if [ "$SMART_SKIP" = "1" ] && [ "$FORCE_FUNCTIONS" != "1" ]; then
  if [ "$TRANS_COUNT" -gt 0 ] && [ "$FUNC_TOTAL" -gt 0 ] && [ "$TRANS_COUNT" -ge "$FUNC_TOTAL" ]; then
    RUN_FUNCTIONS="0"
  fi
fi
if [ "$RUN_FUNCTIONS" = "0" ]; then
    print_success "检测到函数体翻译结果已完整存在（${TRANS_COUNT}/${FUNC_TOTAL}），跳过函数体翻译"
else 
    # 启动 vLLM（用于函数体翻译）
    print_info "启动 vLLM（用于函数体翻译）"
    start_vllm || exit 1
    if python3 translate_function.py "${WORKSPACE_BASE}/extracted/$PROJECT_NAME/functions" "${WORKSPACE_BASE}/translated/$PROJECT_NAME" "$LLM_NAME" "${WORKSPACE_BASE}/skeletons/$PROJECT_NAME" "${WORKSPACE_BASE}/rag/reranked_results/$PROJECT_NAME" test; then
        print_success "函数体翻译完成"
    else
        print_error "函数体翻译失败"
        exit 1
    fi
    # 注意：vLLM 保持运行，后续修复步骤可能需要
fi

echo ""

print_info "根据新策略：已禁用函数级隔离编译与隔离修复，直接进入项目级流程。"

print_step "步骤 9: 项目级编译测试（整仓）"
# 项目名会自动从 translated_functions 目录中检测，或使用环境变量
# 注意：测试步骤允许部分失败，所以不退出
# 设置并行数量环境变量
export TEST_MAX_WORKERS
PROJECT_TRANSLATED_DIR="${WORKSPACE_BASE}/translated/${PROJECT_NAME}/translate_by_${LLM_NAME}"
PROJECT_TEST_RESULTS_DIR="${WORKSPACE_BASE}/test_results/${PROJECT_NAME}/translate_by_${LLM_NAME}"
# 强制重新测试：如果设置了 FORCE_TEST=1，删除测试结果目录
if [ "${FORCE_TEST:-0}" = "1" ]; then
    print_info "FORCE_TEST=1，清理旧测试结果: $PROJECT_TEST_RESULTS_DIR"
    rm -rf "$PROJECT_TEST_RESULTS_DIR"
fi
if python3 auto_test_rust.py "$PROJECT_TRANSLATED_DIR" "$PROJECT_TEST_RESULTS_DIR" "translate_by_${LLM_NAME}" "$PROJECT_NAME"; then
    print_success "编译测试完成"
else
    print_warning "编译测试完成（可能有失败的测试，这是正常的）"
fi

echo ""

print_step "步骤 10: 项目级自动修复 [最多 3 轮]"
# 注意：修复步骤允许部分失败，所以不退出
# 智能跳过：若存在失败，或测试未覆盖全部函数，则执行项目级修复；可用 FORCE_REPAIR=1 强制执行
FAIL_CNT=$(grep -l "^Fail" "$PROJECT_TEST_RESULTS_DIR"/*.txt 2>/dev/null | wc -l | tr -d ' ')
TESTED_CNT=$(ls -1 "$PROJECT_TEST_RESULTS_DIR"/*.txt 2>/dev/null | wc -l | tr -d ' ')
# 确保是数字，避免空字符串导致比较错误
FAIL_CNT=${FAIL_CNT:-0}
TESTED_CNT=${TESTED_CNT:-0}
FUNC_TOTAL=${FUNC_TOTAL:-0}
NEED_REPAIR="0"
if [ "$FORCE_REPAIR" = "1" ]; then
  NEED_REPAIR="1"
elif [ "$FAIL_CNT" -gt 0 ] 2>/dev/null; then
  NEED_REPAIR="1"
elif [ "$FUNC_TOTAL" -gt 0 ] 2>/dev/null && [ "$TESTED_CNT" -lt "$FUNC_TOTAL" ] 2>/dev/null; then
  # 测试未覆盖全部函数，触发修复流程（便于补齐/重测）
  NEED_REPAIR="1"
fi
if [ "$NEED_REPAIR" = "1" ]; then
    print_info "检测到 ${FAIL_CNT} 个失败用例，开始项目级自动修复"
    # 确保 vLLM 已启动（如果之前跳过了函数体翻译，vLLM 可能未启动）
    start_vllm || exit 1
    if python3 auto_repair_rust.py "${WORKSPACE_BASE}/extracted/$PROJECT_NAME/functions" "${WORKSPACE_BASE}/translated/$PROJECT_NAME" "${WORKSPACE_BASE}/test_results/$PROJECT_NAME" "${WORKSPACE_BASE}/repair_results/$PROJECT_NAME" "$LLM_NAME" "${WORKSPACE_BASE}/skeletons/$PROJECT_NAME"; then
        print_success "自动修复完成"
    else
        print_warning "自动修复完成（可能仍有未修复的错误，这是正常的）"
    fi
else
    print_success "未发现失败用例，跳过项目级自动修复"
fi

fi  # END of USE_INCREMENTAL check

echo ""

SKE_PROJECT_DIR="${WORKSPACE_BASE}/skeletons/${PROJECT_NAME}"
SKE_SRC_DIR="${SKE_PROJECT_DIR}/src"
if [ -d "$SKE_SRC_DIR" ]; then
    print_step "静态规范化与合并前预检"
    python3 "$SCRIPT_DIR/scripts/normalize_rust_keywords.py" "$SKE_SRC_DIR" || true
    python3 "$SCRIPT_DIR/scripts/detect_duplicate_symbols.py" "$SKE_SRC_DIR" || true
    python3 "$SCRIPT_DIR/scripts/check_type_modeling_consistency.py" "$SKE_SRC_DIR" || true
    python3 "$SCRIPT_DIR/scripts/precheck_compile.py" "$SKE_PROJECT_DIR" || true
fi

# --- 步骤 11: 最终合并和完整项目测试 ---

print_step "步骤 11: 最终合并和完整项目测试"

# 增量翻译模式已在步骤7中保存了最终项目，跳过最终合并步骤
if [ "$USE_INCREMENTAL" = "1" ]; then
    print_info "增量翻译模式：最终项目已在步骤7中保存，跳过最终合并步骤"
    FINAL_PROJECT_DIR="${WORKSPACE_BASE}/final_projects/$PROJECT_NAME/translate_by_${LLM_NAME}"
    if [ -d "$FINAL_PROJECT_DIR" ]; then
        print_success "最终项目目录: $FINAL_PROJECT_DIR"
        FINAL_SRC_DIR="${FINAL_PROJECT_DIR}/src"
        if [ -d "$FINAL_SRC_DIR" ]; then
            print_step "增量翻译结果规范化与预检"
            python3 "$SCRIPT_DIR/scripts/normalize_rust_keywords.py" "$FINAL_SRC_DIR" || true
            python3 "$SCRIPT_DIR/scripts/detect_duplicate_symbols.py" "$FINAL_SRC_DIR" || true
            python3 "$SCRIPT_DIR/scripts/check_type_modeling_consistency.py" "$FINAL_SRC_DIR" || true
            python3 "$SCRIPT_DIR/scripts/precheck_compile.py" "$FINAL_PROJECT_DIR" || true
        fi
    fi
else
    # 传统并行模式的最终合并逻辑
SKIP_FINAL_MERGE="0"
if [ "$SMART_SKIP" = "1" ] && [ "$FORCE_SKELETON" != "1" ] && [ "$FORCE_FUNCTIONS" != "1" ] && [ "$FORCE_TEST" != "1" ] && [ "$FORCE_REPAIR" != "1" ]; then
    FINAL_PROJECT_DIR="${WORKSPACE_BASE}/final_projects/$PROJECT_NAME/translate_by_${LLM_NAME}"
    # 修复：同时检查目录和文件是否存在，避免误判
    if [ -d "$FINAL_PROJECT_DIR" ] && [ -f "$FINAL_PROJECT_DIR/build_result.txt" ]; then
        BUILD_RESULT=$(head -n 1 "$FINAL_PROJECT_DIR/build_result.txt" 2>/dev/null || echo "")
        if [ "$BUILD_RESULT" = "Success" ]; then
            # 额外验证：检查 src 目录是否存在且包含 .rs 文件
            if [ -d "$FINAL_PROJECT_DIR/src" ] && [ -n "$(find "$FINAL_PROJECT_DIR/src" -name "*.rs" -type f 2>/dev/null | head -1)" ]; then
                print_info "最终合并项目已存在且编译成功，跳过最终合并步骤"
                SKIP_FINAL_MERGE="1"
            else
                print_warning "最终项目目录存在但 src 目录无效，将重新合并"
            fi
        else
            print_warning "最终项目目录存在但编译未成功，将重新合并"
        fi
    fi
fi

if [ "$SKIP_FINAL_MERGE" = "0" ]; then
    print_info "将所有编译通过的函数合并到骨架中，进行完整项目编译测试..."
    # 确保环境变量被传递，使 merge_final_project.py 能正确读取工作空间路径
    export C2R_WORKSPACE_ROOT="${WORKSPACE_BASE}"
    if python3 merge_final_project.py "$PROJECT_NAME" "$LLM_NAME"; then
        print_success "最终合并和测试完成"
    else
        print_warning "最终合并和测试完成（完整项目可能仍有编译错误，这是正常的）"
    fi
    FINAL_PROJECT_DIR="${WORKSPACE_BASE}/final_projects/$PROJECT_NAME/translate_by_${LLM_NAME}"
    FINAL_SRC_DIR="${FINAL_PROJECT_DIR}/src"
    if [ -d "$FINAL_SRC_DIR" ]; then
        print_step "合并后规范化与预检"
        python3 "$SCRIPT_DIR/scripts/normalize_rust_keywords.py" "$FINAL_SRC_DIR" || true
        python3 "$SCRIPT_DIR/scripts/detect_duplicate_symbols.py" "$FINAL_SRC_DIR" || true
        python3 "$SCRIPT_DIR/scripts/check_type_modeling_consistency.py" "$FINAL_SRC_DIR" || true
        python3 "$SCRIPT_DIR/scripts/precheck_compile.py" "$FINAL_PROJECT_DIR" || true
    fi
else
    print_success "跳过最终合并步骤"
    fi
fi

echo ""

# --- 完成 ---

echo ""

print_step "所有步骤完成！"
print_success "翻译结果保存在以下目录："
echo "  - 翻译骨架: ${WORKSPACE_BASE}/skeletons/$PROJECT_NAME/"
echo "  - 翻译函数: ${WORKSPACE_BASE}/translated/$PROJECT_NAME/translate_by_${LLM_NAME}/"
echo "  - 测试结果: ${WORKSPACE_BASE}/test_results/$PROJECT_NAME/translate_by_${LLM_NAME}/"
echo "  - 修复结果: ${WORKSPACE_BASE}/repair_results/$PROJECT_NAME/translate_by_${LLM_NAME}/"
echo "  - 最终项目: ${WORKSPACE_BASE}/final_projects/$PROJECT_NAME/translate_by_${LLM_NAME}/"
echo ""
print_success "流程执行完成！"
print_info "=========================================="
print_info "执行完成时间: $(date '+%Y-%m-%d %H:%M:%S')"
print_info "完整日志已保存到: $LOG_FILE"
print_info "=========================================="
