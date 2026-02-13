#!/bin/bash
# 测试 vLLM 跳过逻辑

echo "============================================================"
echo "vLLM 跳过逻辑测试"
echo "============================================================"
echo ""

# 模拟您的命令参数
USE_LAYERED_SKELETON=true
USE_BINDGEN=true
USE_LLM_SIGNATURES=false  # 默认值（已修改）
USE_TYPE_MAPPER=true
SKELETON_ONLY=true

echo "配置："
echo "  USE_LAYERED_SKELETON=$USE_LAYERED_SKELETON"
echo "  USE_BINDGEN=$USE_BINDGEN"
echo "  USE_LLM_SIGNATURES=$USE_LLM_SIGNATURES"
echo "  USE_TYPE_MAPPER=$USE_TYPE_MAPPER"
echo "  SKELETON_ONLY=$SKELETON_ONLY"
echo ""

# 检查是否需要 vLLM
NEED_VLLM_FOR_STAGE1=false
if [[ "$USE_LAYERED_SKELETON" == "true" ]]; then
    if [[ "$USE_TYPE_MAPPER" == "true" ]] && [[ "$USE_LLM_SIGNATURES" != "true" ]]; then
        NEED_VLLM_FOR_STAGE1=false
        echo "✓ 结果：跳过 vLLM 启动（使用 TypeMapper，无需 LLM）"
    elif [[ "$USE_LLM_SIGNATURES" == "true" ]]; then
        NEED_VLLM_FOR_STAGE1=true
        echo "✗ 结果：需要启动 vLLM（使用 LLM）"
    else
        NEED_VLLM_FOR_STAGE1=false
        echo "✓ 结果：跳过 vLLM 启动（默认）"
    fi
else
    NEED_VLLM_FOR_STAGE1=true
    echo "✗ 结果：需要启动 vLLM（传统模式）"
fi

echo ""
echo "结论："
if [[ "$NEED_VLLM_FOR_STAGE1" == "false" ]]; then
    echo "  ✅ 您的命令不会启动 vLLM"
    echo "  ✅ 使用 TypeMapper（确定性规则引擎）"
    echo "  ✅ 更快、更稳定、零成本"
else
    echo "  ⚠️  您的命令会启动 vLLM"
fi
