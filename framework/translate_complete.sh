#!/bin/bash

################################################################################
# C2Rust 完整翻译流程脚本（按照原版逻辑，包含骨架和函数体完整流程）
# 项目: filemanagement_dfs_service
# 
# 硬编码配置：
#   - 项目名称: filemanagement_dfs_service
#   - 项目路径: workspace/projects/filemanagement_dfs_service
#   - 虚拟环境: c2r_frame
#   - LLM 模型: gpt_5_nano
# 
# 完整流程（按照原版逻辑）：
#   阶段1: 骨架翻译与修复
#     1. 提取依赖
#     2. 翻译骨架
#     3. 创建 main.rs
#     4. 添加依赖
#     5. 骨架编译测试与修复（最多3轮）
#   
#   阶段2: 函数体翻译与修复
#     3. 匹配函数签名
#     4. RAG 检索（BM25 + Jina）
#     6. 填充函数体
#     8. 函数体编译测试
#     9. 函数体自动修复（最多5轮，原版逻辑）
# 
# 使用方法：
#   bash get_function_complete.sh
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
# 配置参数（支持环境变量覆盖）
# ============================================================================
PROJECT_NAME=${PROJECT_NAME:-"filemanagement_dfs_service"}
PROJECT_ROOT=${PROJECT_ROOT:-"workspace/projects/filemanagement_dfs_service"}
VIRTUAL_ENV_NAME=${VIRTUAL_ENV_NAME:-"c2r_frame"}
LLM_NAME=${LLM_NAME:-"gpt_5_nano"}

# 并行处理配置
TRANSLATE_MAX_WORKERS=${TRANSLATE_MAX_WORKERS:-"160"}                 # 骨架翻译并行数
TRANSLATE_FUNCTION_MAX_WORKERS=${TRANSLATE_FUNCTION_MAX_WORKERS:-"160"}  # 函数体翻译并行数
TEST_MAX_WORKERS=${TEST_MAX_WORKERS:-"4096"}                           # 编译测试并行数
REPAIR_MAX_WORKERS=${REPAIR_MAX_WORKERS:-"48"}                       # 自动修复并行数

# 智能跳过与强制执行控制
SMART_SKIP=${SMART_SKIP:-"1"}                # 1=启用智能跳过；0=关闭
FORCE_SKELETON=${FORCE_SKELETON:-"0"}        # 1=强制重新翻译骨架
FORCE_FUNCTIONS=${FORCE_FUNCTIONS:-"0"}      # 1=强制重新翻译函数体
FORCE_TEST=${FORCE_TEST:-"0"}                 # 1=强制重新编译测试
FORCE_REPAIR=${FORCE_REPAIR:-"0"}            # 1=强制执行修复

# 输出控制
NO_PROGRESS=${NO_PROGRESS:-"0"}              # 1=隐藏进度条；0=显示进度条

# vLLM 配置
USE_VLLM=${USE_VLLM:-"true"}
VLLM_BASE_URL=${VLLM_BASE_URL:-"http://localhost:8000/v1"}
VLLM_API_KEY=${VLLM_API_KEY:-"EMPTY"}
VLLM_MODEL_NAME=${VLLM_MODEL_NAME:-"qwen3_coder"}
VLLM_NUM_WORKERS=${VLLM_NUM_WORKERS:-"160"}
VLLM_REQUEST_TIMEOUT=${VLLM_REQUEST_TIMEOUT:-"600"}
# Default higher to avoid function-body truncation (can be overridden by env).
VLLM_MAX_TOKENS=${VLLM_MAX_TOKENS:-"16384"}
# GPU 配置（使用 GPU 0,1,2,3）
VLLM_CUDA_VISIBLE_DEVICES=${VLLM_CUDA_VISIBLE_DEVICES:-"0,1,2,3"}
VLLM_TENSOR_PARALLEL_SIZE=${VLLM_TENSOR_PARALLEL_SIZE:-"4"}

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 转换为绝对路径
if [[ ! "$PROJECT_ROOT" = /* ]]; then
    PROJECT_ROOT="$SCRIPT_DIR/$PROJECT_ROOT"
fi

# --- 日志配置 ---
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/translate_complete_${PROJECT_NAME}_${TIMESTAMP}.log"
exec > >(tee -a "$LOG_FILE") 2>&1

print_info() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')] $1${NC}"
}

# --- 激活虚拟环境 ---
print_info "激活虚拟环境: $VIRTUAL_ENV_NAME"
if command -v conda >/dev/null 2>&1; then
    source "$(conda info --base)/etc/profile.d/conda.sh"
    conda activate "$VIRTUAL_ENV_NAME" || {
        print_error "无法激活 conda 环境: $VIRTUAL_ENV_NAME"
        exit 1
    }
    print_success "已激活 conda 环境: $VIRTUAL_ENV_NAME"
else
    print_warning "未找到 conda，跳过虚拟环境激活"
fi

# --- 导出环境变量 ---
export PROJECT_NAME PROJECT_ROOT LLM_NAME
export TRANSLATE_MAX_WORKERS TRANSLATE_FUNCTION_MAX_WORKERS TEST_MAX_WORKERS REPAIR_MAX_WORKERS
export SMART_SKIP FORCE_SKELETON FORCE_FUNCTIONS FORCE_TEST FORCE_REPAIR NO_PROGRESS
export USE_VLLM VLLM_BASE_URL VLLM_API_KEY VLLM_MODEL_NAME VLLM_NUM_WORKERS
export VLLM_REQUEST_TIMEOUT VLLM_MAX_TOKENS
export VLLM_CUDA_VISIBLE_DEVICES VLLM_TENSOR_PARALLEL_SIZE

# --- Cargo 编译优化配置 ---
print_info "配置 Cargo 编译优化..."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/workspace/.cargo_target_shared}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-1}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-32}"

if command -v clang >/dev/null 2>&1; then
  if command -v ld.lld >/dev/null 2>&1 || command -v lld >/dev/null 2>&1; then
    export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16} -C linker=clang -C link-arg=-fuse-ld=lld"
  else
    export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16}"
  fi
else
  export RUSTFLAGS="${RUSTFLAGS:--C codegen-units=16}"
fi

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="$(command -v sccache)"
  sccache --start-server >/dev/null 2>&1 || true
  echo "  已启用 sccache: $RUSTC_WRAPPER"
else
  echo "  未找到 sccache，跳过该优化（可选安装以提速）"
fi

# --- vLLM 按需启动/停止函数 ---
VLLM_HOST="${VLLM_HOST:-127.0.0.1}"
VLLM_PORT="${VLLM_PORT:-8000}"
VLLM_URL="${VLLM_BASE_URL:-http://$VLLM_HOST:$VLLM_PORT/v1}"

start_vllm() {
  # 如果使用外部 API（USE_VLLM=false），跳过本地 vLLM 启动
  if [ "${USE_VLLM:-true}" = "false" ] || [ "${USE_VLLM:-true}" = "0" ] || [ "${USE_VLLM:-true}" = "no" ]; then
    echo "USE_VLLM=$USE_VLLM，使用外部 API，跳过本地 vLLM 启动"
    return 0
  fi
  if curl -s "$VLLM_URL/models" >/dev/null 2>&1; then
    echo "vLLM 已在运行: $VLLM_URL"
    return 0
  fi
  DEPLOY_VLLM_SCRIPT="${DEPLOY_VLLM_SCRIPT:-}"
  if [ -z "$DEPLOY_VLLM_SCRIPT" ]; then
    if [ -f "$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_4bit_AWQ.sh" ]; then
      DEPLOY_VLLM_SCRIPT="$SCRIPT_DIR/../rag_builder/qwen3_coder/deploy_4bit_AWQ.sh"
    else
      print_error "未找到 deploy_4bit_AWQ.sh，请设置 DEPLOY_VLLM_SCRIPT"
      return 1
    fi
  fi
  echo "启动 vLLM: $DEPLOY_VLLM_SCRIPT"
  echo "GPU 配置: $VLLM_CUDA_VISIBLE_DEVICES (Tensor Parallel: $VLLM_TENSOR_PARALLEL_SIZE)"
  VLLM_LOG="$LOG_DIR/vllm_$(date +%Y%m%d_%H%M%S).log"
  # 通过环境变量传递 GPU 配置（使用 env 命令确保环境变量正确传递）
  env CUDA_VISIBLE_DEVICES="$VLLM_CUDA_VISIBLE_DEVICES" \
      VLLM_TENSOR_PARALLEL_SIZE="$VLLM_TENSOR_PARALLEL_SIZE" \
      nohup bash "$DEPLOY_VLLM_SCRIPT" >"$VLLM_LOG" 2>&1 &
  VLLM_PID=$!
  echo "vLLM 启动中 (pid=$VLLM_PID)，日志: $VLLM_LOG"
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
  # 如果使用外部 API（USE_VLLM=false），跳过停止
  if [ "${USE_VLLM:-true}" = "false" ] || [ "${USE_VLLM:-true}" = "0" ] || [ "${USE_VLLM:-true}" = "no" ]; then
    return 0
  fi
  pkill -f "vllm.entrypoints.openai.api_server" >/dev/null 2>&1 || true
}

# --- 检查知识库文件 ---
if [ ! -f "workspace/rag/knowledge_base.json" ] || [ ! -f "workspace/rag/bm25_index.pkl" ]; then
    print_warning "知识库文件不存在，需要先构建知识库"
    print_step "步骤 0: 构建知识库"
    
    if [ ! -d "../rag_builder/qwen3_coder/filtered_code_pairs" ]; then
        print_error "知识库源目录不存在: ../rag_builder/qwen3_coder/filtered_code_pairs"
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
fi

echo ""

# ============================================================================
# 阶段1: 骨架翻译与修复（按照原版逻辑）
# ============================================================================

print_step "阶段1: 骨架翻译与修复"

print_step "步骤 1: 分析 C/C++ 依赖"
if python3 get_dependencies.py --project "$PROJECT_NAME"; then
    print_success "依赖分析完成"
else
    print_error "依赖分析失败"
    exit 1
fi

echo ""

print_step "步骤 2: 翻译代码骨架（包含编译修复）"
export TRANSLATE_MAX_WORKERS
SKE_DIR="workspace/skeletons/${PROJECT_NAME}/src"
SRC_SKE_DIR="workspace/source_skeletons/${PROJECT_NAME}/src"
REPAIR_SKE_DIR="workspace/repair_results/${PROJECT_NAME}/translate_by_default/round_3/src" # 骨架修复的最终结果目录
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
    print_info "启动 vLLM（用于骨架翻译与修复）"
    start_vllm || exit 1
    
    # 按照原版逻辑：translate.py 内部会调用 create_main(), add_dependencies(), repair()
    # 注意：translate.py 的 if __name__ == "__main__" 会执行完整流程（包括修复）
    # 但需要确保 PROJECT_NAME 环境变量已设置
    if python3 translate.py; then
        print_success "骨架翻译与修复完成"
    else
        print_error "骨架翻译与修复失败"
        exit 1
    fi
    
    print_info "关闭 vLLM（骨架翻译与修复结束）"
    stop_vllm
fi

echo ""

# ============================================================================
# 阶段2: 函数体翻译与修复（按照原版逻辑）
# ============================================================================

print_step "阶段2: 函数体翻译与修复"

print_step "步骤 3: 匹配 C++ 函数和 Rust 函数签名"
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

print_step "步骤 6: 核心翻译 - 填充函数体"
export TRANSLATE_FUNCTION_MAX_WORKERS
TRANS_DIR="workspace/translated/${PROJECT_NAME}/translate_by_${LLM_NAME}"
FUNC_SRC_DIR="workspace/extracted/${PROJECT_NAME}/functions"
TRANS_COUNT=$(ls -1 ${TRANS_DIR}/*.txt 2>/dev/null | wc -l | tr -d ' ')
FUNC_TOTAL=$(ls -1 ${FUNC_SRC_DIR}/*.txt 2>/dev/null | wc -l | tr -d ' ')
RUN_FUNCTIONS="1"
if [ "$SMART_SKIP" = "1" ] && [ "$FORCE_FUNCTIONS" != "1" ]; then
  if [ "$TRANS_COUNT" -gt 0 ] && [ "$FUNC_TOTAL" -gt 0 ] && [ "$TRANS_COUNT" -ge "$FUNC_TOTAL" ]; then
    RUN_FUNCTIONS="0"
  fi
fi
if [ "$RUN_FUNCTIONS" = "0" ]; then
    print_success "检测到函数体翻译结果已完整存在（${TRANS_COUNT}/${FUNC_TOTAL}），跳过函数体翻译与 vLLM 启动"
else 
    print_info "启动 vLLM（用于函数体翻译）"
    start_vllm || exit 1
    if python3 translate_function.py workspace/extracted/$PROJECT_NAME/functions workspace/translated/$PROJECT_NAME "$LLM_NAME" workspace/skeletons/$PROJECT_NAME workspace/rag/reranked_results/$PROJECT_NAME test; then
        print_success "函数体翻译完成"
    else
        print_error "函数体翻译失败"
        exit 1
    fi
    print_info "关闭 vLLM（函数体翻译结束）"
    stop_vllm
fi

echo ""

print_step "步骤 8: 函数体编译测试"
export TEST_MAX_WORKERS
PROJECT_TRANSLATED_DIR="workspace/translated/${PROJECT_NAME}/translate_by_${LLM_NAME}"
PROJECT_TEST_RESULTS_DIR="workspace/test_results/${PROJECT_NAME}/translate_by_${LLM_NAME}"
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

print_step "步骤 9: 函数体自动修复 [最多 5 轮，按照原版逻辑]"
FAIL_CNT=$(grep -l "^Fail" "$PROJECT_TEST_RESULTS_DIR"/*.txt 2>/dev/null | wc -l | tr -d ' ')
TESTED_CNT=$(ls -1 "$PROJECT_TEST_RESULTS_DIR"/*.txt 2>/dev/null | wc -l | tr -d ' ')
FAIL_CNT=${FAIL_CNT:-0}
TESTED_CNT=${TESTED_CNT:-0}
FUNC_TOTAL=${FUNC_TOTAL:-0}
NEED_REPAIR="0"
if [ "$FORCE_REPAIR" = "1" ]; then
  NEED_REPAIR="1"
elif [ "$FAIL_CNT" -gt 0 ] 2>/dev/null; then
  NEED_REPAIR="1"
elif [ "$FUNC_TOTAL" -gt 0 ] 2>/dev/null && [ "$TESTED_CNT" -lt "$FUNC_TOTAL" ] 2>/dev/null; then
  NEED_REPAIR="1"
fi
if [ "$NEED_REPAIR" = "1" ]; then
    print_info "检测到 ${FAIL_CNT} 个失败用例，启动 vLLM（函数体自动修复）"
    start_vllm || exit 1
    # 按照原版逻辑：auto_repair_rust.py 会进行最多 5 轮修复
    if python3 auto_repair_rust.py workspace/extracted/$PROJECT_NAME/functions workspace/translated/$PROJECT_NAME workspace/test_results workspace/repair_results "$LLM_NAME" workspace/skeletons/$PROJECT_NAME; then
        print_success "自动修复完成"
    else
        print_warning "自动修复完成（可能仍有未修复的错误，这是正常的）"
    fi
    print_info "关闭 vLLM（函数体自动修复结束）"
    stop_vllm
else
    print_success "未发现失败用例，跳过函数体自动修复与 vLLM 启动"
fi

echo ""

# --- 完成 ---
print_step "所有步骤完成！"
print_success "翻译结果保存在以下目录："
echo "  - 翻译骨架: workspace/skeletons/$PROJECT_NAME/"
echo "  - 翻译函数: workspace/translated/$PROJECT_NAME/translate_by_${LLM_NAME}/"
echo "  - 测试结果: workspace/test_results/${PROJECT_NAME}/translate_by_${LLM_NAME}/"
echo "  - 修复结果: workspace/repair_results/${PROJECT_NAME}/translate_by_${LLM_NAME}/"
echo ""
print_success "流程执行完成！"
print_info "=========================================="
print_info "执行完成时间: $(date '+%Y-%m-%d %H:%M:%S')"
print_info "完整日志已保存到: $LOG_FILE"
print_info "=========================================="
