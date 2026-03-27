// c2rust 测试文件 for genann

use crate::src_genann::*;

#[test]
fn test_genann_init() {
    unsafe {
        let ann = genann_init(2, 1, 3, 1);
        assert!(!ann.is_null());
        if !ann.is_null() {
            genann_free(ann);
        }
    }
}

#[test]
fn test_genann_act_sigmoid() {
    unsafe {
        let result = genann_act_sigmoid(std::ptr::null(), 0.0);
        assert!((result - 0.5).abs() < 0.01);
    }
}

#[test]
#[ignore]
fn test_compile_check() {
}
