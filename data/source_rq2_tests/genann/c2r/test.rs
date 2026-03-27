// C2R 框架测试文件 for genann (神经网络库)
// 直接调用翻译后的 Rust 函数
//
// 使用方法：
// 1. 复制此文件到项目的 src/test_c2r.rs
// 2. 在 main.rs 中添加: #[cfg(test)] mod test_c2r;
// 3. 运行 cargo test

use crate::src_genann::*;
use crate::types::*;

#[test]
fn test_genann_act_sigmoid() {
    // sigmoid(0) = 0.5
    let result = genann_act_sigmoid(0.0);
    assert!((result - 0.5).abs() < 0.001, "sigmoid(0) 应该约等于 0.5");
}

#[test]
fn test_genann_act_sigmoid_positive() {
    // sigmoid(大正数) 接近 1
    let result = genann_act_sigmoid(10.0);
    assert!(result > 0.99, "sigmoid(10) 应该接近 1");
}

#[test]
fn test_genann_act_sigmoid_negative() {
    // sigmoid(大负数) 接近 0
    let result = genann_act_sigmoid(-10.0);
    assert!(result < 0.01, "sigmoid(-10) 应该接近 0");
}

#[test]
fn test_genann_act_threshold() {
    // 阈值函数：x > 0 返回 1，否则返回 0
    assert_eq!(genann_act_threshold(1.0), 1.0, "threshold(1) 应该是 1");
    assert_eq!(genann_act_threshold(-1.0), 0.0, "threshold(-1) 应该是 0");
    assert_eq!(genann_act_threshold(0.0), 0.0, "threshold(0) 应该是 0");
}

#[test]
fn test_genann_act_linear() {
    // 线性函数：直接返回输入
    assert_eq!(genann_act_linear(5.0), 5.0, "linear(5) 应该是 5");
    assert_eq!(genann_act_linear(-3.0), -3.0, "linear(-3) 应该是 -3");
    assert_eq!(genann_act_linear(0.0), 0.0, "linear(0) 应该是 0");
}

#[test]
fn test_genann_init() {
    // 创建简单的神经网络：1输入，1隐藏层（2神经元），1输出
    unsafe {
        let ann = genann_init(1, 1, 2, 1);
        assert!(!ann.is_null(), "genann_init 应该返回有效指针");
        genann_free(ann);
    }
}

#[test]
fn test_genann_init_no_hidden() {
    // 创建没有隐藏层的网络
    unsafe {
        let ann = genann_init(2, 0, 0, 1);
        assert!(!ann.is_null(), "没有隐藏层的网络也应该创建成功");
        genann_free(ann);
    }
}

#[test]
fn test_genann_copy() {
    unsafe {
        let ann = genann_init(2, 1, 3, 1);
        assert!(!ann.is_null());

        let copy = genann_copy(ann);
        assert!(!copy.is_null(), "genann_copy 应该返回有效指针");

        genann_free(ann);
        genann_free(copy);
    }
}

#[test]
fn test_genann_randomize() {
    unsafe {
        let ann = genann_init(2, 1, 3, 1);
        assert!(!ann.is_null());

        // randomize 不应该崩溃
        genann_randomize(ann);

        genann_free(ann);
    }
}

#[test]
fn test_genann_run() {
    unsafe {
        let ann = genann_init(2, 1, 2, 1);
        assert!(!ann.is_null());

        // 准备输入
        let inputs: [f64; 2] = [0.5, 0.5];

        let output = genann_run(ann, inputs.as_ptr());
        assert!(!output.is_null(), "genann_run 应该返回输出指针");

        genann_free(ann);
    }
}

#[test]
fn test_genann_sigmoid_cached() {
    // 缓存版本应该与非缓存版本结果相近
    let x = 0.5;
    let cached = genann_act_sigmoid_cached(x);
    let normal = genann_act_sigmoid(x);

    // 允许一定的误差（因为缓存可能使用查表）
    assert!((cached - normal).abs() < 0.1,
            "缓存和非缓存的 sigmoid 应该结果相近");
}

#[test]
fn test_genann_multilayer() {
    // 测试多隐藏层网络
    unsafe {
        let ann = genann_init(4, 3, 5, 2);
        assert!(!ann.is_null(), "多隐藏层网络应该创建成功");
        genann_free(ann);
    }
}
