#!/bin/bash
# ============================================
# C2Rust 批量翻译 - 性能优化版启动脚本
# ============================================
#
# 优化内容：
# 1. 降低 vLLM 显存占用（0.7），让 Jina Reranker 可以共存
# 2. 启用 Rust 并行编译
# 3. 避免阶段间 vLLM 重启（节省 15-20 分钟）
#
# 预期效果：
# - 从 103 分钟降到 65-70 分钟（约 35% 提升）
# ============================================

# 1. vLLM 显存优化
# 从 0.92 降到 0.7，为 Jina Reranker 留 30% 显存（约14GB/GPU）
export VLLM_GPU_MEMORY_UTILIZATION=0.7

# 2. Rust 并行编译优化
# 根据你的 CPU 核心数调整（建议设置为核心数的 80%）
export CARGO_BUILD_JOBS=32

# 3. Jina Reranker 最小内存要求（降低到6GB，更激进地利用GPU）
export JINA_MIN_MEMORY_GB=6.0

# 4. 启用 PyTorch 内存优化
export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True

# 5. 显示配置
echo "=================================="
echo "性能优化配置"
echo "=================================="
echo "vLLM 显存占用: $VLLM_GPU_MEMORY_UTILIZATION (70%)"
echo "Rust 并行编译: $CARGO_BUILD_JOBS 个任务"
echo "Jina 最小内存: $JINA_MIN_MEMORY_GB GB"
echo "=================================="
echo ""

# 运行优化版 batch_test_staged.sh
exec bash "$(dirname "$0")/batch_test_staged.sh" "$@"
