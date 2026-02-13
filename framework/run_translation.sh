#!/bin/bash

################################################################################
# 一键运行翻译脚本 - filemanagement_dfs_service
# 
# 此脚本会自动：
#   1. 激活虚拟环境 c2r_frame
#   2. 设置项目配置
#   3. 运行完整翻译流程
################################################################################

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# --- 配置项目信息 ---
PROJECT_NAME="filemanagement_dfs_service"
VIRTUAL_ENV_NAME="c2r_frame"
LLM_NAME=${LLM_NAME:-gpt_5_nano}

# --- 查找项目路径 ---
print_step "查找项目路径"

# 如果已经设置了 PROJECT_ROOT 环境变量，直接使用
if [ -n "$PROJECT_ROOT" ] && [ -d "$PROJECT_ROOT" ]; then
    print_success "使用环境变量中的项目路径: $PROJECT_ROOT"
else
    # 可能的项目路径（按优先级排序）
    POSSIBLE_PATHS=(
        # 1. 工作空间目录（最常用）
        "$SCRIPT_DIR/workspace/projects/$PROJECT_NAME"
        # 2. 相对路径
        "$SCRIPT_DIR/../$PROJECT_NAME"
        "$SCRIPT_DIR/../../$PROJECT_NAME"
        # 3. 用户目录（可选）
        "$HOME/$PROJECT_NAME"
    )
    
    PROJECT_ROOT=""
    for path in "${POSSIBLE_PATHS[@]}"; do
        if [ -d "$path" ]; then
            PROJECT_ROOT="$path"
            print_success "找到项目: $PROJECT_ROOT"
            break
        fi
    done
    
    if [ -z "$PROJECT_ROOT" ]; then
        print_error "未找到项目目录: $PROJECT_NAME"
        echo ""
        echo "已检查以下路径："
        for path in "${POSSIBLE_PATHS[@]}"; do
            echo "  - $path"
        done
        echo ""
        print_error "请手动设置 PROJECT_ROOT 环境变量"
        echo ""
        echo "例如："
        echo "  export PROJECT_ROOT=\"$SCRIPT_DIR/workspace/projects/$PROJECT_NAME\""
        echo "  bash $0"
        echo ""
        echo "或者将项目放在以下任一位置："
        echo "  - $SCRIPT_DIR/workspace/projects/$PROJECT_NAME"
        echo "  - $SCRIPT_DIR/../$PROJECT_NAME"
        exit 1
    fi
fi

# 转换为绝对路径
if [[ ! "$PROJECT_ROOT" = /* ]]; then
    PROJECT_ROOT="$(cd "$PROJECT_ROOT" && pwd)"
fi

# --- 激活虚拟环境 ---
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

# --- 设置环境变量并运行主脚本 ---
print_step "开始翻译流程"

# 导出环境变量，供 get_function.sh 使用
export PROJECT_NAME
export PROJECT_ROOT
export LLM_NAME
export VIRTUAL_ENV_NAME

# 设置并行处理配置（如果未设置，使用默认值48）
export TRANSLATE_MAX_WORKERS=${TRANSLATE_MAX_WORKERS:-"48"}
export TRANSLATE_FUNCTION_MAX_WORKERS=${TRANSLATE_FUNCTION_MAX_WORKERS:-"48"}
export TEST_MAX_WORKERS=${TEST_MAX_WORKERS:-"48"}

# vLLM 配置（用于翻译）
export USE_VLLM=${USE_VLLM:-"true"}  # 是否使用 vLLM（默认启用）
export VLLM_BASE_URL=${VLLM_BASE_URL:-"http://localhost:8000/v1"}  # vLLM 服务器地址
export VLLM_API_KEY=${VLLM_API_KEY:-"EMPTY"}  # vLLM API Key
export VLLM_MODEL_NAME=${VLLM_MODEL_NAME:-"qwen3_coder"}  # vLLM 模型名称
export VLLM_NUM_WORKERS=${VLLM_NUM_WORKERS:-"48"}  # vLLM 并行工作进程数

print_success "配置信息："
echo "  项目名称: $PROJECT_NAME"
echo "  项目路径: $PROJECT_ROOT"
echo "  LLM 模型: $LLM_NAME"
echo "  虚拟环境: $VIRTUAL_ENV_NAME"
echo "  骨架翻译并行数: $TRANSLATE_MAX_WORKERS"
echo "  函数体翻译并行数: $TRANSLATE_FUNCTION_MAX_WORKERS"
echo "  编译测试并行数: $TEST_MAX_WORKERS"
if [ "$USE_VLLM" = "true" ] || [ "$USE_VLLM" = "1" ] || [ "$USE_VLLM" = "yes" ]; then
    echo "  vLLM 模式: 启用"
    echo "  vLLM 地址: $VLLM_BASE_URL"
    echo "  vLLM 模型: $VLLM_MODEL_NAME"
    echo "  vLLM 并行数: $VLLM_NUM_WORKERS"
else
    echo "  vLLM 模式: 禁用（使用传统 API）"
fi
echo ""

# 验证项目目录
if [ ! -d "$PROJECT_ROOT" ]; then
    print_error "项目目录不存在: $PROJECT_ROOT"
    exit 1
fi

# 检查项目目录是否为空
if [ -z "$(ls -A "$PROJECT_ROOT" 2>/dev/null)" ]; then
    print_warning "项目目录为空: $PROJECT_ROOT"
    print_warning "请确保项目目录包含源代码文件"
fi

# 运行主脚本
bash get_function.sh
