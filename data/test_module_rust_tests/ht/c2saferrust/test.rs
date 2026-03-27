// c2saferrust 测试文件 for ht (hash table)
//
// 重要：在 deepseek_batch_20260107_145427 这批结果中，ht 的输出只有最小的 `main`，
// 没有翻译出可测试的 hash table API（hash_new/hash_get/...）。
//
// 为了让“单元测试正确率”统计有意义，这里保留与 evolc2rust 相同数量的测试用例，
// 并在运行时显式失败，提示该项目缺少可测试 API。

fn not_translated() -> ! {
    panic!("c2saferrust(ht): 输出仅包含 main，未包含可测试的 hash table API；该项目无法进行真实单元测试。");
}

#[test]
fn test_hash_new_and_free() {
    not_translated();
}

#[test]
fn test_hash_set_get_has_del_size_clear() {
    not_translated();
}

#[test]
fn test_kh_iteration_api() {
    not_translated();
}

