#!/bin/bash

################################################################################
# C2Rust 项目翻译脚本 - 统一输出管理版
# 
# 此脚本会为每个翻译任务创建独立的输出目录，所有输出都在该目录下，
# 便于管理多个项目的翻译结果，且不会相互覆盖。
#
# 使用方法：
#   bash translate_project.sh --source <C项目路径> [选项]
#
# 必需参数：
#   --source, -s PATH          C/C++ 项目源代码路径（必需）
#
# 可选参数：
#   --name, -n NAME            项目名称 (默认: 从路径自动提取)
#   --output-name NAME         输出目录名 (默认: {项目名}_{日期})
#   --llm NAME                 LLM 名称 (默认: qwen3_coder)
#   
#   --force-all                强制重新执行所有步骤
#   --force-skeleton           强制重新翻译骨架
#   --force-functions          强制重新翻译函数体
#   --force-test               强制重新测试
#   --force-repair             强制重新修复
#   
#   --incremental              使用增量式翻译模式（填一个测一个，即时修复）
#   --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)
#   
#   --list                     列出所有已有的翻译输出
#   --help, -h                 显示帮助信息
#
# 示例：
#   # 翻译 cJson 项目
#   bash translate_project.sh -s /path/to/cJson
#   
#   # 指定项目名和输出目录名
#   bash translate_project.sh -s /path/to/cJson -n cJson --output-name cJson_v1
#   
#   # 强制从头翻译
#   bash translate_project.sh -s /path/to/cJson --force-all
#   
#   # 列出所有翻译输出
#   bash translate_project.sh --list
################################################################################

set -e

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 统一输出管理目录
OUTPUTS_BASE_DIR="$SCRIPT_DIR/translation_outputs"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_help() {
    head -n 45 "$0" | tail -n +3 | sed 's/^# //' | sed 's/^#//'
    exit 0
}

list_outputs() {
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}已有的翻译输出目录${NC}"
    echo -e "${CYAN}========================================${NC}"
    
    if [ ! -d "$OUTPUTS_BASE_DIR" ]; then
        echo "  (无)"
        exit 0
    fi
    
    for dir in "$OUTPUTS_BASE_DIR"/*/; do
        if [ -d "$dir" ]; then
            dirname=$(basename "$dir")
            # 检查是否有翻译结果
            if [ -d "${dir}workspace/translated" ]; then
                status="${GREEN}[已翻译]${NC}"
            else
                status="${YELLOW}[未完成]${NC}"
            fi
            echo -e "  $status $dirname"
            echo "         路径: $dir"
        fi
    done
    exit 0
}

# ============================================================================
# 默认配置
# ============================================================================
SOURCE_PATH=""
PROJECT_NAME=""
OUTPUT_NAME=""
LLM_NAME="qwen3_coder"

# 传递给 get_function.sh 的额外参数
EXTRA_ARGS=""

# ============================================================================
# 解析命令行参数
# ============================================================================
while [[ $# -gt 0 ]]; do
    case $1 in
        --source|-s)
            SOURCE_PATH="$2"
            shift 2
            ;;
        --name|-n)
            PROJECT_NAME="$2"
            shift 2
            ;;
        --output-name)
            OUTPUT_NAME="$2"
            shift 2
            ;;
        --llm)
            LLM_NAME="$2"
            shift 2
            ;;
        --force-all|--no-skip)
            EXTRA_ARGS="$EXTRA_ARGS --force-all"
            shift
            ;;
        --force-skeleton)
            EXTRA_ARGS="$EXTRA_ARGS --force-skeleton"
            shift
            ;;
        --force-functions)
            EXTRA_ARGS="$EXTRA_ARGS --force-functions"
            shift
            ;;
        --force-test)
            EXTRA_ARGS="$EXTRA_ARGS --force-test"
            shift
            ;;
        --force-repair)
            EXTRA_ARGS="$EXTRA_ARGS --force-repair"
            shift
            ;;
        --incremental)
            EXTRA_ARGS="$EXTRA_ARGS --incremental"
            shift
            ;;
        --max-repair)
            EXTRA_ARGS="$EXTRA_ARGS --max-repair $2"
            shift 2
            ;;
        --list)
            list_outputs
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

# ============================================================================
# 参数验证
# ============================================================================
if [ -z "$SOURCE_PATH" ]; then
    print_error "必须指定 C/C++ 项目源代码路径"
    echo "使用方法: bash translate_project.sh --source <C项目路径>"
    echo "使用 --help 查看更多帮助"
    exit 1
fi

# 转换为绝对路径
if [[ ! "$SOURCE_PATH" = /* ]]; then
    SOURCE_PATH="$(cd "$SOURCE_PATH" 2>/dev/null && pwd)" || {
        print_error "源代码路径不存在: $SOURCE_PATH"
        exit 1
    }
fi

if [ ! -d "$SOURCE_PATH" ]; then
    print_error "源代码路径不存在: $SOURCE_PATH"
    exit 1
fi

# 自动提取项目名（如果未指定）
if [ -z "$PROJECT_NAME" ]; then
    PROJECT_NAME=$(basename "$SOURCE_PATH")
fi

# 自动生成输出目录名（如果未指定）
if [ -z "$OUTPUT_NAME" ]; then
    DATE_SUFFIX=$(date +"%Y%m%d")
    OUTPUT_NAME="${PROJECT_NAME}_${DATE_SUFFIX}"
fi

# 输出目录
OUTPUT_DIR="$OUTPUTS_BASE_DIR/$OUTPUT_NAME"

# ============================================================================
# 显示配置信息
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}C2Rust 项目翻译${NC}"
echo -e "${CYAN}========================================${NC}"
print_info "源代码路径: $SOURCE_PATH"
print_info "项目名称: $PROJECT_NAME"
print_info "输出目录: $OUTPUT_DIR"
print_info "LLM: $LLM_NAME"
if [ -n "$EXTRA_ARGS" ]; then
    print_info "额外参数: $EXTRA_ARGS"
fi
echo -e "${CYAN}========================================${NC}"

# ============================================================================
# 创建输出目录结构
# ============================================================================
print_info "创建输出目录结构..."

mkdir -p "$OUTPUT_DIR/workspace/projects"
mkdir -p "$OUTPUT_DIR/workspace/skeletons"
mkdir -p "$OUTPUT_DIR/workspace/source_skeletons"
mkdir -p "$OUTPUT_DIR/workspace/extracted"
mkdir -p "$OUTPUT_DIR/workspace/translated"
mkdir -p "$OUTPUT_DIR/workspace/test_results"
mkdir -p "$OUTPUT_DIR/workspace/repair_results"
mkdir -p "$OUTPUT_DIR/workspace/signature_matches"
mkdir -p "$OUTPUT_DIR/workspace/rag/elastic_search_results"
mkdir -p "$OUTPUT_DIR/workspace/rag/reranked_results"
mkdir -p "$OUTPUT_DIR/logs"

# 创建项目软链接
if [ ! -e "$OUTPUT_DIR/workspace/projects/$PROJECT_NAME" ]; then
    print_info "创建项目软链接..."
    ln -sf "$SOURCE_PATH" "$OUTPUT_DIR/workspace/projects/$PROJECT_NAME"
fi

# 链接知识库（使用框架目录的知识库）
print_info "链接知识库..."
if [ -f "$SCRIPT_DIR/workspace/rag/knowledge_base.json" ] && [ -f "$SCRIPT_DIR/workspace/rag/bm25_index.pkl" ]; then
    # 删除可能存在的旧链接或文件
    rm -f "$OUTPUT_DIR/workspace/rag/knowledge_base.json" 2>/dev/null
    rm -f "$OUTPUT_DIR/workspace/rag/bm25_index.pkl" 2>/dev/null
    # 创建软链接
    ln -sf "$SCRIPT_DIR/workspace/rag/knowledge_base.json" "$OUTPUT_DIR/workspace/rag/knowledge_base.json"
    ln -sf "$SCRIPT_DIR/workspace/rag/bm25_index.pkl" "$OUTPUT_DIR/workspace/rag/bm25_index.pkl"
    print_success "知识库已链接"
else
    print_warning "知识库不存在，将在翻译时自动构建"
    print_warning "或者您可以先运行: python3 build_knowledge_base.py"
fi

print_success "目录结构准备完成"

# ============================================================================
# 设置环境变量并执行翻译
# ============================================================================
print_info "开始翻译流程..."

# 关键：设置工作空间根目录环境变量
export C2R_WORKSPACE_ROOT="$OUTPUT_DIR/workspace"
export PROJECT_NAME="$PROJECT_NAME"
export PROJECT_ROOT="$OUTPUT_DIR/workspace/projects/$PROJECT_NAME"
export LLM_NAME="$LLM_NAME"

print_info "环境变量:"
print_info "  C2R_WORKSPACE_ROOT=$C2R_WORKSPACE_ROOT"
print_info "  PROJECT_NAME=$PROJECT_NAME"
print_info "  PROJECT_ROOT=$PROJECT_ROOT"

# 调用主翻译脚本
bash "$SCRIPT_DIR/get_function.sh" \
    --project "$PROJECT_NAME" \
    --project-root "$OUTPUT_DIR/workspace/projects/$PROJECT_NAME" \
    --llm "$LLM_NAME" \
    $EXTRA_ARGS

RESULT=$?

# ============================================================================
# 完成
# ============================================================================
if [ $RESULT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}翻译完成！${NC}"
    echo -e "${GREEN}========================================${NC}"
    print_info "所有输出保存在: $OUTPUT_DIR"
    print_info ""
    print_info "目录结构:"
    print_info "  workspace/skeletons/$PROJECT_NAME/     - 翻译骨架"
    print_info "  workspace/translated/$PROJECT_NAME/    - 翻译结果"
    print_info "  workspace/test_results/$PROJECT_NAME/  - 测试结果"
    print_info "  workspace/repair_results/$PROJECT_NAME/ - 修复结果"
    print_info "  logs/                                  - 日志文件"
else
    print_error "翻译过程中出现错误 (退出码: $RESULT)"
    print_error "请检查日志: $SCRIPT_DIR/logs/"
    exit $RESULT
fi
