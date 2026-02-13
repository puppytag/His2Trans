#!/bin/bash

################################################################################
# C2Rust 批量项目测试脚本
# 
# 一次性测试多个推荐的项目，验证翻译框架的效果
#
# 使用方法：
#   bash batch_test_projects.sh [选项]
#
# 可选参数：
#   --incremental              使用增量式翻译模式
#   --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)
#   --llm NAME                 LLM 名称 (默认: qwen3_coder)
#   --skip PROJECT1,PROJECT2   跳过指定的项目（用逗号分隔）
#   --only PROJECT1,PROJECT2   只运行指定的项目（用逗号分隔）
#   --help, -h                 显示帮助信息
################################################################################

set -e

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 默认配置
LLM_NAME="qwen3_coder"
USE_INCREMENTAL=false
MAX_REPAIR=5
SKIP_PROJECTS=""
ONLY_PROJECTS=""
PARALLEL=true  # 默认并行执行
MAX_PARALLEL=4  # 最大并行数

# 推荐的项目列表（开源版默认使用随仓库提供的 OHOS(test5) 最小项目集）
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
            shift 2
            ;;
        --llm)
            LLM_NAME="$2"
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
        --parallel)
            PARALLEL=true
            shift
            ;;
        --sequential|--no-parallel)
            PARALLEL=false
            shift
            ;;
        --max-parallel)
            MAX_PARALLEL="$2"
            shift 2
            ;;
        --help|-h)
            echo "C2Rust 批量项目测试脚本"
            echo ""
            echo "使用方法："
            echo "  bash batch_test_projects.sh [选项]"
            echo ""
            echo "可选参数："
            echo "  --incremental              使用增量式翻译模式"
            echo "  --max-repair N             增量模式下每个函数最大修复次数 (默认: 5)"
            echo "  --llm NAME                 LLM 名称 (默认: qwen3_coder)"
            echo "  --skip PROJECT1,PROJECT2   跳过指定的项目（用逗号分隔）"
            echo "  --only PROJECT1,PROJECT2   只运行指定的项目（用逗号分隔）"
            echo "  --parallel                 并行执行所有项目（默认）"
            echo "  --sequential               串行执行所有项目"
            echo "  --max-parallel N           最大并行数（默认: 4）"
            echo "  --help, -h                 显示帮助信息"
            echo ""
            echo "推荐的项目："
            for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
                echo "  - $proj_name"
            done
            exit 0
            ;;
        *)
            echo "错误: 未知参数: $1"
            echo "使用 --help 查看帮助信息"
            exit 1
            ;;
    esac
done

# 检查项目路径是否存在
check_project_paths() {
    local missing=0
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
        if [[ ! -d "$proj_path" ]]; then
            echo "⚠ 警告: 项目路径不存在: $proj_name -> $proj_path"
            missing=$((missing + 1))
        fi
    done
    
    if [[ $missing -gt 0 ]]; then
        echo "错误: 有 $missing 个项目路径不存在"
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

# vLLM 管理函数（从 get_function.sh 复制）
VLLM_URL="${VLLM_URL:-http://localhost:8000/v1}"
LOG_DIR="$SCRIPT_DIR/logs"
VIRTUAL_ENV_NAME="${VIRTUAL_ENV_NAME:-c2r_frame}"
mkdir -p "$LOG_DIR"

# 激活 conda 环境的辅助函数
activate_conda_env() {
    if command -v conda >/dev/null 2>&1; then
        if conda env list | grep -q "^$VIRTUAL_ENV_NAME "; then
            # 初始化 conda（如果还没有）
            eval "$(conda shell.bash hook 2>/dev/null)" || {
                # 如果 conda shell hook 失败，尝试直接 source
                if [ -f "$(conda info --base)/etc/profile.d/conda.sh" ]; then
                    source "$(conda info --base)/etc/profile.d/conda.sh"
                fi
            }
            conda activate "$VIRTUAL_ENV_NAME"
            if [ "$CONDA_DEFAULT_ENV" = "$VIRTUAL_ENV_NAME" ]; then
                echo "✓ 已激活 conda 环境: $VIRTUAL_ENV_NAME"
                return 0
            fi
        fi
    fi
    echo "⚠ 警告: 无法激活 conda 环境 $VIRTUAL_ENV_NAME，将使用系统 Python"
    return 1
}

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
            echo "错误: 未找到 deploy_no_quantization.sh，请设置 DEPLOY_VLLM_SCRIPT"
            return 1
        fi
    fi
    
    echo "启动 vLLM: $DEPLOY_VLLM_SCRIPT"
    
    # 创建启动脚本包装器，确保 conda 环境被激活
    VLLM_LOG="$LOG_DIR/vllm_$(date +%Y%m%d_%H%M%S).log"
    TEMP_START_SCRIPT=$(mktemp)
    
    # 将 VIRTUAL_ENV_NAME 写入包装脚本
    cat > "$TEMP_START_SCRIPT" << EOFSCRIPT
#!/bin/bash
# 临时启动脚本，确保 conda 环境被激活
VIRTUAL_ENV_NAME="${VIRTUAL_ENV_NAME}"
DEPLOY_SCRIPT="\$1"
LOG_FILE="\$2"

# 初始化 conda
if command -v conda >/dev/null 2>&1; then
    eval "\$(conda shell.bash hook 2>/dev/null)" || {
        if [ -f "\$(conda info --base)/etc/profile.d/conda.sh" ]; then
            source "\$(conda info --base)/etc/profile.d/conda.sh"
        fi
    }
    
    if conda env list | grep -q "^\${VIRTUAL_ENV_NAME} "; then
        conda activate "\${VIRTUAL_ENV_NAME}"
        echo "[vLLM启动] 已激活 conda 环境: \${VIRTUAL_ENV_NAME}" >> "\${LOG_FILE}" 2>&1
        echo "[vLLM启动] Python路径: \$(which python)" >> "\${LOG_FILE}" 2>&1
    else
        echo "[vLLM启动] 警告: conda 环境 \${VIRTUAL_ENV_NAME} 不存在" >> "\${LOG_FILE}" 2>&1
    fi
fi

# 执行部署脚本
bash "\${DEPLOY_SCRIPT}" >> "\${LOG_FILE}" 2>&1
EOFSCRIPT

    chmod +x "$TEMP_START_SCRIPT"
    nohup bash "$TEMP_START_SCRIPT" "$DEPLOY_VLLM_SCRIPT" "$VLLM_LOG" >/dev/null 2>&1 &
    VLLM_PID=$!
    
    # 清理临时脚本（延迟删除，确保启动完成）
    (sleep 10 && rm -f "$TEMP_START_SCRIPT") &
    
    echo "vLLM 启动中 (pid=$VLLM_PID)，日志: $VLLM_LOG"
    MAX_WAIT=300
    WAITED=0
    until curl -s "$VLLM_URL/models" >/dev/null 2>&1; do
        sleep 2
        WAITED=$((WAITED+2))
        if [ $WAITED -ge $MAX_WAIT ]; then
            echo "错误: vLLM 启动超时（>$MAX_WAIT 秒）。日志: $VLLM_LOG"
            echo "请查看日志文件: $VLLM_LOG"
            return 1
        fi
    done
    echo "vLLM 已就绪: $VLLM_URL"
    return 0
}

stop_vllm() {
    echo "正在关闭 vLLM 服务..."
    pkill -f "vllm.entrypoints.openai.api_server" >/dev/null 2>&1 || true
    sleep 2
    echo "✓ vLLM 已关闭"
}

# 运行单个项目的翻译（用于并行执行）
run_project_translation() {
    local proj_name="$1"
    local proj_path="$2"
    local log_file="$3"
    local result_file="$4"
    
    # 记录开始时间
    local start_time=$(date +%s)
    
    # 构建翻译命令
    local cmd="bash translate_project.sh -s \"$proj_path\" -n \"$proj_name\" --llm \"$LLM_NAME\""
    
    if [[ "$USE_INCREMENTAL" == true ]]; then
        cmd="$cmd --incremental --max-repair $MAX_REPAIR"
    fi
    
    # 运行翻译，输出重定向到独立日志文件
    {
        echo "============================================================"
        echo "开始翻译项目: $proj_name"
        echo "项目路径: $proj_path"
        echo "LLM: $LLM_NAME"
        echo "增量模式: $USE_INCREMENTAL"
        if [[ "$USE_INCREMENTAL" == true ]]; then
            echo "最大修复次数: $MAX_REPAIR"
        fi
        echo "============================================================"
        echo ""
        
        if eval "$cmd" >> "$log_file" 2>&1; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            local minutes=$((duration / 60))
            local seconds=$((duration % 60))
            
            echo "SUCCESS|$proj_name|$start_time|$end_time|$duration" > "$result_file"
            
            echo ""
            echo "============================================================"
            echo "✓ 项目 $proj_name 翻译完成！"
            echo "耗时: ${minutes}分${seconds}秒"
            echo "日志: $log_file"
            echo "============================================================"
            return 0
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            local minutes=$((duration / 60))
            local seconds=$((duration % 60))
            
            echo "FAIL|$proj_name|$start_time|$end_time|$duration" > "$result_file"
            
            echo ""
            echo "============================================================"
            echo "✗ 项目 $proj_name 翻译失败！"
            echo "耗时: ${minutes}分${seconds}秒"
            echo "日志: $log_file"
            echo "============================================================"
            return 1
        fi
    }
}

# 主函数
main() {
    echo "============================================================"
    echo "C2Rust 批量项目测试"
    if [[ "$PARALLEL" == true ]]; then
        echo "模式: 并行执行（最大并行数: $MAX_PARALLEL）"
    else
        echo "模式: 串行执行"
    fi
    echo "============================================================"
    echo ""
    echo "推荐的项目列表："
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        echo "  - $proj_name"
    done
    echo ""
    
    # 检查项目路径
    check_project_paths
    
    # 创建临时目录存储结果
    local temp_dir=$(mktemp -d -t batch_test_XXXXXX)
    trap "rm -rf $temp_dir" EXIT
    
    # 收集要运行的项目
    local projects_to_run=()
    for proj_name in "${!RECOMMENDED_PROJECTS[@]}"; do
        if ! should_skip_project "$proj_name"; then
            projects_to_run+=("$proj_name")
        fi
    done
    
    if [[ ${#projects_to_run[@]} -eq 0 ]]; then
        echo "没有要运行的项目"
        exit 0
    fi
    
    local total_projects=${#projects_to_run[@]}
    local skip_count=$((${#RECOMMENDED_PROJECTS[@]} - total_projects))
    
    # 记录总体开始时间
    local overall_start_time=$(date +%s)
    
    # 如果使用并行模式，先启动 vLLM
    if [[ "$PARALLEL" == true ]]; then
        echo "准备并行执行 $total_projects 个项目..."
        echo ""
        echo "============================================================"
        echo "启动共享的 vLLM 服务..."
        echo "============================================================"
        if ! start_vllm; then
            echo "错误: 无法启动 vLLM，退出"
            exit 1
        fi
        echo ""
        
        # 设置环境变量，告诉子进程不要管理 vLLM（因为由批量脚本统一管理）
        export BATCH_TEST_MODE=1
        export SKIP_VLLM_MANAGEMENT=1
        
        # 设置退出时自动关闭 vLLM
        trap "stop_vllm; rm -rf $temp_dir" EXIT
    fi
    
    # 存储后台进程的 PID
    declare -A pids
    declare -A log_files
    declare -A result_files
    
    # 运行项目（并行或串行）
    if [[ "$PARALLEL" == true ]]; then
        echo "============================================================"
        echo "开始并行执行项目..."
        echo "============================================================"
        echo ""
        
        local running_count=0
        local completed_count=0
        
        for proj_name in "${projects_to_run[@]}"; do
            local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
            local log_file="$temp_dir/${proj_name}.log"
            local result_file="$temp_dir/${proj_name}.result"
            
            log_files[$proj_name]="$log_file"
            result_files[$proj_name]="$result_file"
            
            # 等待直到有可用槽位
            while [[ $running_count -ge $MAX_PARALLEL ]]; do
                # 检查是否有进程完成
                for pid in "${!pids[@]}"; do
                    if ! kill -0 "${pids[$pid]}" 2>/dev/null; then
                        # 进程已完成
                        unset pids[$pid]
                        running_count=$((running_count - 1))
                        completed_count=$((completed_count + 1))
                    fi
                done
                sleep 1
            done
            
            # 启动后台进程
            echo "[$((completed_count + running_count + 1))/$total_projects] 启动项目: $proj_name"
            run_project_translation "$proj_name" "$proj_path" "$log_file" "$result_file" &
            local pid=$!
            pids[$proj_name]=$pid
            running_count=$((running_count + 1))
        done
        
        # 等待所有进程完成
        echo ""
        echo "等待所有项目完成..."
        while [[ ${#pids[@]} -gt 0 ]]; do
            for proj_name in "${!pids[@]}"; do
                if ! kill -0 "${pids[$proj_name]}" 2>/dev/null; then
                    unset pids[$proj_name]
                    completed_count=$((completed_count + 1))
                    echo "[$completed_count/$total_projects] 项目完成: $proj_name"
                fi
            done
            sleep 2
        done
        
        echo ""
        echo "所有项目已启动，等待全部完成..."
        wait
    else
        # 串行执行
        local current=0
        for proj_name in "${projects_to_run[@]}"; do
            local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
            current=$((current + 1))
            
            echo ""
            echo "[$current/$total_projects] 开始翻译项目: $proj_name"
            
            local log_file="$temp_dir/${proj_name}.log"
            local result_file="$temp_dir/${proj_name}.result"
            
            log_files[$proj_name]="$log_file"
            result_files[$proj_name]="$result_file"
            
            run_project_translation "$proj_name" "$proj_path" "$log_file" "$result_file"
        done
    fi
    
    # 收集结果
    local success_count=0
    local fail_count=0
    
    echo ""
    echo "============================================================"
    echo "收集结果..."
    echo "============================================================"
    echo ""
    
    for proj_name in "${projects_to_run[@]}"; do
        local result_file="${result_files[$proj_name]}"
        if [[ -f "$result_file" ]]; then
            local result_line=$(cat "$result_file")
            local status=$(echo "$result_line" | cut -d'|' -f1)
            if [[ "$status" == "SUCCESS" ]]; then
                success_count=$((success_count + 1))
                local duration=$(echo "$result_line" | cut -d'|' -f5)
                local minutes=$((duration / 60))
                local seconds=$((duration % 60))
                echo "✓ $proj_name: 成功 (${minutes}分${seconds}秒)"
            else
                fail_count=$((fail_count + 1))
                local duration=$(echo "$result_line" | cut -d'|' -f5)
                local minutes=$((duration / 60))
                local seconds=$((duration % 60))
                echo "✗ $proj_name: 失败 (${minutes}分${seconds}秒)"
            fi
        else
            fail_count=$((fail_count + 1))
            echo "✗ $proj_name: 结果文件未找到"
        fi
    done
    
    # 计算总耗时
    local overall_end_time=$(date +%s)
    local overall_duration=$((overall_end_time - overall_start_time))
    local overall_minutes=$((overall_duration / 60))
    local overall_seconds=$((overall_duration % 60))
    
    # 输出总结
    echo ""
    echo "============================================================"
    echo "批量测试完成！"
    echo "============================================================"
    echo "总项目数: ${#RECOMMENDED_PROJECTS[@]}"
    echo "运行项目: $total_projects"
    echo "成功: $success_count"
    echo "失败: $fail_count"
    echo "跳过: $skip_count"
    echo "总耗时: ${overall_minutes}分${overall_seconds}秒"
    echo ""
    echo "详细日志："
    for proj_name in "${projects_to_run[@]}"; do
        echo "  - $proj_name: ${log_files[$proj_name]}"
    done
    echo "============================================================"
    
    # 如果使用并行模式，关闭 vLLM
    if [[ "$PARALLEL" == true ]]; then
        echo ""
        stop_vllm
    fi
    
    # 返回适当的退出码
    if [[ $fail_count -gt 0 ]]; then
        exit 1
    else
        exit 0
    fi
}

# 运行主函数
main "$@"
