#!/bin/bash
# ============================================
# C2Rust 批量翻译 - 流水线模式
# ============================================
#
# 流水线策略：
# - 项目完成阶段1后立即进入阶段2（不等待其他项目）
# - 项目完成阶段2后立即进入阶段3（不等待其他项目）
# - vLLM 保持运行，避免重启开销
# - 使用智能资源调度，避免冲突
#
# 预期效果：
# - 从 65 分钟降到 40-45 分钟（约 35% 提升）
# - 单个项目更快完成
# - GPU 利用率从 60% 提升到 90%+
# ============================================

set -euo pipefail

# 获取脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 加载批处理脚本的函数（但不执行）
source "$SCRIPT_DIR/batch_test_staged.sh" --source-only 2>/dev/null || true

# 如果无法 source，手动定义关键变量
if [[ -z "${VIRTUAL_ENV_NAME:-}" ]]; then
    VIRTUAL_ENV_NAME="${VIRTUAL_ENV_NAME:-c2r_frame}"
    LLM_NAME="${LLM_NAME:-qwen3_coder}"
    VLLM_URL="${VLLM_URL:-http://localhost:8000/v1}"
    MAX_REPAIR="${MAX_REPAIR:-5}"
    MAX_PARALLEL="${MAX_PARALLEL:-10}"
fi

# ============================================
# 流水线配置
# ============================================
PIPELINE_BUFFER=10  # 最多10个项目同时在不同阶段
PIPELINE_CHECK_INTERVAL=5  # 每5秒检查一次队列
PIPELINE_LOG_DIR="$SCRIPT_DIR/translation_outputs/Test/intermediate/logs/pipeline"

mkdir -p "$PIPELINE_LOG_DIR"

# ============================================
# 流水线状态文件
# ============================================
PIPELINE_STATE_DIR="$SCRIPT_DIR/.pipeline_state"
mkdir -p "$PIPELINE_STATE_DIR"

STAGE1_QUEUE="$PIPELINE_STATE_DIR/stage1_queue.txt"
STAGE2_QUEUE="$PIPELINE_STATE_DIR/stage2_queue.txt"
STAGE3_QUEUE="$PIPELINE_STATE_DIR/stage3_queue.txt"
COMPLETED="$PIPELINE_STATE_DIR/completed.txt"

# 初始化队列文件
: > "$STAGE1_QUEUE"
: > "$STAGE2_QUEUE"
: > "$STAGE3_QUEUE"
: > "$COMPLETED"

# ============================================
# 队列管理函数
# ============================================

pipeline_add_to_stage1() {
    local proj_name="$1"
    echo "$proj_name" >> "$STAGE1_QUEUE"
}

pipeline_move_to_stage2() {
    local proj_name="$1"
    # 从 stage1 移除
    sed -i "/^${proj_name}$/d" "$STAGE1_QUEUE" 2>/dev/null || true
    # 添加到 stage2
    echo "$proj_name" >> "$STAGE2_QUEUE"
    echo "[Pipeline] $proj_name: 阶段1完成 → 进入阶段2"
}

pipeline_move_to_stage3() {
    local proj_name="$1"
    # 从 stage2 移除
    sed -i "/^${proj_name}$/d" "$STAGE2_QUEUE" 2>/dev/null || true
    # 添加到 stage3
    echo "$proj_name" >> "$STAGE3_QUEUE"
    echo "[Pipeline] $proj_name: 阶段2完成 → 进入阶段3"
}

pipeline_mark_completed() {
    local proj_name="$1"
    # 从 stage3 移除
    sed -i "/^${proj_name}$/d" "$STAGE3_QUEUE" 2>/dev/null || true
    # 标记为完成
    echo "$proj_name" >> "$COMPLETED"
    echo "[Pipeline] $proj_name: ✅ 全部阶段完成"
}

pipeline_count_running() {
    local stage="$1"
    local count_file="$PIPELINE_STATE_DIR/${stage}_running_count.txt"
    if [[ -f "$count_file" ]]; then
        cat "$count_file"
    else
        echo "0"
    fi
}

pipeline_increment_running() {
    local stage="$1"
    local count_file="$PIPELINE_STATE_DIR/${stage}_running_count.txt"
    local count=$(pipeline_count_running "$stage")
    echo $((count + 1)) > "$count_file"
}

pipeline_decrement_running() {
    local stage="$1"
    local count_file="$PIPELINE_STATE_DIR/${stage}_running_count.txt"
    local count=$(pipeline_count_running "$stage")
    echo $((count - 1)) > "$count_file"
}

# ============================================
# 阶段执行函数
# ============================================

pipeline_run_stage1() {
    local proj_name="$1"
    local proj_path="$2"

    echo "[Pipeline] 启动阶段1: $proj_name"
    pipeline_increment_running "stage1"

    # 异步执行阶段1
    (
        # 调用原有的 run_stage1 函数
        local result_file="$PIPELINE_STATE_DIR/${proj_name}_stage1_result.txt"
        local log_file="$PIPELINE_LOG_DIR/${proj_name}_stage1.log"

        if run_stage1 "$proj_name" "$proj_path" "$log_file" "$result_file" 2>&1 | tee -a "$log_file"; then
            pipeline_move_to_stage2 "$proj_name"
        else
            echo "[Pipeline] ❌ $proj_name 阶段1失败"
        fi

        pipeline_decrement_running "stage1"
    ) &
}

pipeline_run_stage2() {
    local proj_name="$1"
    local proj_path="$2"

    echo "[Pipeline] 启动阶段2: $proj_name"
    pipeline_increment_running "stage2"

    # 异步执行阶段2
    (
        local result_file="$PIPELINE_STATE_DIR/${proj_name}_stage2_result.txt"
        local log_file="$PIPELINE_LOG_DIR/${proj_name}_stage2.log"

        if run_stage2 "$proj_name" "$proj_path" "$log_file" "$result_file" 2>&1 | tee -a "$log_file"; then
            pipeline_move_to_stage3 "$proj_name"
        else
            echo "[Pipeline] ❌ $proj_name 阶段2失败"
        fi

        pipeline_decrement_running "stage2"
    ) &
}

pipeline_run_stage3() {
    local proj_name="$1"
    local proj_path="$2"

    echo "[Pipeline] 启动阶段3: $proj_name"
    pipeline_increment_running "stage3"

    # 异步执行阶段3
    (
        local result_file="$PIPELINE_STATE_DIR/${proj_name}_stage3_result.txt"
        local log_file="$PIPELINE_LOG_DIR/${proj_name}_stage3.log"

        if run_stage3 "$proj_name" "$proj_path" "$log_file" "$result_file" 2>&1 | tee -a "$log_file"; then
            pipeline_mark_completed "$proj_name"
        else
            echo "[Pipeline] ❌ $proj_name 阶段3失败"
        fi

        pipeline_decrement_running "stage3"
    ) &
}

# ============================================
# 流水线调度器
# ============================================

pipeline_scheduler() {
    echo "============================================"
    echo "流水线调度器启动"
    echo "============================================"
    echo "缓冲区大小: $PIPELINE_BUFFER"
    echo "检查间隔: ${PIPELINE_CHECK_INTERVAL}秒"
    echo "============================================"

    while true; do
        # 检查是否所有项目都完成
        local total_projects=$(wc -l < "$COMPLETED" 2>/dev/null || echo "0")
        local expected_projects="$1"

        if [[ "$total_projects" -eq "$expected_projects" ]]; then
            echo "[Pipeline] ✅ 所有 $total_projects 个项目完成！"
            break
        fi

        # 获取当前运行数
        local stage1_running=$(pipeline_count_running "stage1")
        local stage2_running=$(pipeline_count_running "stage2")
        local stage3_running=$(pipeline_count_running "stage3")

        echo "[Pipeline] 状态: 阶段1运行=${stage1_running}, 阶段2运行=${stage2_running}, 阶段3运行=${stage3_running}, 完成=${total_projects}/${expected_projects}"

        # 启动阶段1任务（如果有空闲槽位）
        if [[ "$stage1_running" -lt "$MAX_PARALLEL" ]]; then
            local proj_name=$(head -n 1 "$STAGE1_QUEUE" 2>/dev/null)
            if [[ -n "$proj_name" ]]; then
                local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
                pipeline_run_stage1 "$proj_name" "$proj_path"
            fi
        fi

        # 启动阶段2任务（如果有空闲槽位）
        if [[ "$stage2_running" -lt "$MAX_PARALLEL" ]]; then
            local proj_name=$(head -n 1 "$STAGE2_QUEUE" 2>/dev/null)
            if [[ -n "$proj_name" ]]; then
                local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
                pipeline_run_stage2 "$proj_name" "$proj_path"
            fi
        fi

        # 启动阶段3任务（如果有空闲槽位）
        if [[ "$stage3_running" -lt "$MAX_PARALLEL" ]]; then
            local proj_name=$(head -n 1 "$STAGE3_QUEUE" 2>/dev/null)
            if [[ -n "$proj_name" ]]; then
                local proj_path="${RECOMMENDED_PROJECTS[$proj_name]}"
                pipeline_run_stage3 "$proj_name" "$proj_path"
            fi
        fi

        sleep "$PIPELINE_CHECK_INTERVAL"
    done

    echo "[Pipeline] 流水线调度器结束"
}

# ============================================
# 主函数
# ============================================

main() {
    echo "============================================"
    echo "C2Rust 批量翻译 - 流水线模式"
    echo "============================================"

    # 解析命令行参数（调用原脚本的参数解析）
    # TODO: 这里需要从 batch_test_staged.sh 获取项目列表

    # 示例：加载项目列表
    local projects=("$@")
    if [[ ${#projects[@]} -eq 0 ]]; then
        echo "错误：未指定项目"
        echo "用法: $0 proj1 proj2 proj3 ..."
        exit 1
    fi

    # 添加所有项目到阶段1队列
    for proj in "${projects[@]}"; do
        pipeline_add_to_stage1 "$proj"
    done

    # 启动 vLLM（保持运行整个流程）
    echo "[Pipeline] 启动 vLLM 服务..."
    start_vllm || {
        echo "[Pipeline] 无法启动 vLLM，退出"
        exit 1
    }

    # 启动流水线调度器
    pipeline_scheduler "${#projects[@]}"

    # 清理
    echo "[Pipeline] 清理状态文件..."
    rm -rf "$PIPELINE_STATE_DIR"

    echo "[Pipeline] ✅ 流水线处理完成！"
}

# 如果直接执行（不是被 source）
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
