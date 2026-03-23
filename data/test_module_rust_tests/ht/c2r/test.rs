// Unified RQ2 test suite for ht in His2Trans/C2R mode.
//
// 说明：
// - 当前产物只暴露 `main` 和私有 `hash_key`，没有可测试的 hash table API。
// - 统一测试集需要 3 个测试，因此保留 1 个真实 smoke test，
//   其余 2 个测试显式失败，暴露“接口缺失”这一真实事实。

#[test]
fn test_main_smoke() {
    crate::src_ht::main();
}

#[test]
fn test_hash_api_missing_create_destroy() {
    panic!(
        "unified RQ2 ht suite expects hash-table construction API, but current artifact only exposes `main`/private `hash_key`"
    );
}

#[test]
fn test_hash_api_missing_mutation_and_iteration() {
    panic!(
        "unified RQ2 ht suite expects insert/get/iterate API, but current artifact does not expose any public hash-table operations"
    );
}
