#!/bin/bash

# 安全模式
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "即将删除旧的可重算输出（保留 logs 与 RAG/知识库等）:"
echo " - workspace/test_results/*"
echo " - workspace/translated/*"
echo " - workspace/repair_results/*"
echo " - workspace/signature_matches/*"
echo " - workspace/skeletons/*   (保留 src/ 结构？默认清理全部骨架，谨慎)"
echo " - workspace/source_skeletons/*"
echo " - workspace/extracted/*"
echo " - 顶层日志 auto_test_rust.log translate_throughLLM.log build_final.log (保留 logs/ 目录)"
echo

read -r -p "确认执行清理? (yes/no): " ans
if [[ "$ans" != "yes" ]]; then
  echo "已取消。"
  exit 0
fi

rm -rf workspace/test_results/* || true
rm -rf workspace/translated/* || true
rm -rf workspace/repair_results/* || true
rm -rf workspace/signature_matches/* || true
rm -rf workspace/source_skeletons/* || true
rm -rf workspace/skeletons/* || true
rm -rf workspace/extracted/* || true
# 保留 logs 目录内容
rm -f auto_test_rust.log translate_throughLLM.log build_final.log || true

echo "清理完成。"


