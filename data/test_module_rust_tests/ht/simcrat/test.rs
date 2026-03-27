// simcrat 测试文件 for ht (hash table)
//
// 目标：与 evolc2rust/c2saferrust 的外部测试语义保持一致（3 个 #[test]，覆盖 hash_new/free、
// set/get/has/del/size/clear、以及 kh_* 迭代 API）。
//
// 但当前 simcrat 输出的 ht.rs 仅包含 `hash_key(&str) -> u64`，缺失完整哈希表 API。
// 为了避免“空跑=100% 通过”的假象，这里显式 fail。

#[test]
fn test_hash_new_and_free() {
    panic!("simcrat ht: missing hash_new/hash_free (only hash_key is present)");
}

#[test]
fn test_hash_set_get_has_del_size_clear() {
    panic!("simcrat ht: missing hash_set/get/has/del/size/clear API");
}

#[test]
fn test_kh_iteration_api() {
    panic!("simcrat ht: missing kh_* iteration API");
}

