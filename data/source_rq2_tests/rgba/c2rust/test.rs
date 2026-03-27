// c2rust 测试文件 for rgba
// 适配 c2rust 原始指针版本的函数签名

use crate::src_rgba::*;

#[test]
fn test_rgba_new() {
    unsafe {
        let color = rgba_new(0xFF0000FF); // 红色
        // c2rust 版本的 rgba_new 返回的是归一化到 [0,1] 的 double。
        assert!((color.r - 1.0).abs() < 1e-9);
        assert!((color.g - 0.0).abs() < 1e-9);
        assert!((color.b - 0.0).abs() < 1e-9);
        assert!((color.a - 1.0).abs() < 1e-9);
    }
}

#[test]
#[ignore]
fn test_rgba_compile_check() {
    // 验证代码可以编译
}
