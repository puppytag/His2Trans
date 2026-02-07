// C2R 框架测试文件 for ht
//
// 说明：
// - Our/projects/ht/src/ht.c 里只有一个 `static` 的 FNV-1a 64-bit 哈希函数（hash_key）和空 main。
// - 由于 `hash_key` 是 `static`，翻译后的 Rust 会保持它为私有（模块内可见），
//   测试 harness 以 crate root 的 `test_c2r` 模块注入测试文件时无法直接访问该私有函数。
// - 因此这里提供一个最小 smoke test：确保翻译出来的 `crate::src_ht::main()` 可被调用。

#[test]
fn test_main_smoke() {
    // translated C `main` (an empty function in this project)
    crate::src_ht::main();
}

