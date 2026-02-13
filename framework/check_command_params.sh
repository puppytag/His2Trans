#!/bin/bash
# 检查命令参数的有效性

echo "============================================================"
echo "命令参数分析"
echo "============================================================"
echo ""

CMD="bash batch_test_staged.sh --layered --incremental --max-repair 3 --max-parallel 50 --run-rag true --skeleton-only"

echo "您的命令："
echo "$CMD"
echo ""

echo "参数生效情况："
echo "  ✓ --layered: 生效（使用分层骨架构建 + TypeMapper）"
echo "  ✓ --max-parallel 50: 生效（阶段1并行数）"
echo "  ✓ --skeleton-only: 生效（只运行阶段1）"
echo ""
echo "  ✗ --incremental: 不生效（阶段3参数，被 --skeleton-only 跳过）"
echo "  ✗ --max-repair 3: 不生效（阶段3参数，被 --skeleton-only 跳过）"
echo "  ✗ --run-rag true: 不生效（阶段2参数，被 --skeleton-only 跳过）"
echo ""

echo "会使用的更新："
echo "  ✓ TypeMapper（确定性规则引擎）"
echo "  ✓ compile_commands.json（硬编码路径）"
echo "  ✓ 分层骨架构建（bindgen + tree-sitter）"
echo ""

echo "推荐命令（如果只想测试骨架）："
echo "  bash batch_test_staged.sh --layered --max-parallel 50 --skeleton-only"
echo ""

echo "推荐命令（如果要运行完整流程）："
echo "  bash batch_test_staged.sh --layered --incremental --max-repair 3 --max-parallel 50 --run-rag true"
echo "  （去掉 --skeleton-only）"
