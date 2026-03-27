// c2saferrust 测试文件 for genann (神经网络)
// 目标：与 evolc2rust 套件对齐的真实语义测试。

use crate::*;

#[test]
fn test_genann_act_sigmoid() {
    let result = genann_act_sigmoid(0.0);
    assert!((result - 0.5).abs() < 0.001, "sigmoid(0) 应该约等于 0.5");
}

#[test]
fn test_genann_act_sigmoid_positive() {
    let result = genann_act_sigmoid(10.0);
    assert!(result > 0.99, "sigmoid(10) 应该接近 1");
}

#[test]
fn test_genann_act_sigmoid_negative() {
    let result = genann_act_sigmoid(-10.0);
    assert!(result < 0.01, "sigmoid(-10) 应该接近 0");
}

#[test]
fn test_genann_act_threshold() {
    assert_eq!(genann_act_threshold(1.0), 1.0, "threshold(1) 应该是 1");
    assert_eq!(genann_act_threshold(-1.0), 0.0, "threshold(-1) 应该是 0");
    assert_eq!(genann_act_threshold(0.0), 0.0, "threshold(0) 应该是 0");
}

#[test]
fn test_genann_act_linear() {
    unsafe {
        assert_eq!(genann_act_linear(5.0), 5.0, "linear(5) 应该是 5");
        assert_eq!(genann_act_linear(-3.0), -3.0, "linear(-3) 应该是 -3");
        assert_eq!(genann_act_linear(0.0), 0.0, "linear(0) 应该是 0");
    }
}

#[test]
fn test_genann_init() {
    let ann = genann_init(1, 1, 2, 1);
    assert!(ann.is_some(), "genann_init 应该返回 Some");
    if let Some(ann_ptr) = ann {
        genann_free(ann_ptr.as_ptr());
    }
}

#[test]
fn test_genann_init_no_hidden() {
    let ann = genann_init(2, 0, 0, 1);
    assert!(ann.is_some(), "没有隐藏层的网络也应该创建成功");
    if let Some(ann_ptr) = ann {
        genann_free(ann_ptr.as_ptr());
    }
}

#[test]
fn test_genann_copy() {
    let ann = genann_init(2, 1, 3, 1);
    assert!(ann.is_some());
    if let Some(ann_ptr) = ann {
        let ann_ref = unsafe { ann_ptr.as_ref() };
        let copy = genann_copy(ann_ref);
        assert!(copy.is_some(), "genann_copy 应该返回 Some");
        genann_free(ann_ptr.as_ptr());
        // copy: Box<genann> 自动释放
    }
}

#[test]
fn test_genann_randomize() {
    let ann = genann_init(2, 1, 3, 1);
    assert!(ann.is_some());
    if let Some(ann_ptr) = ann {
        let ann_ref = unsafe { &mut *ann_ptr.as_ptr() };
        genann_randomize(ann_ref);
        genann_free(ann_ptr.as_ptr());
    }
}

#[test]
fn test_genann_run() {
    let ann = genann_init(2, 1, 2, 1);
    assert!(ann.is_some());
    if let Some(ann_ptr) = ann {
        let ann_ref = unsafe { ann_ptr.as_ref() };
        let inputs = [0.5, 0.5];
        let output = genann_run(ann_ref, &inputs);
        assert_eq!(output.len(), 1, "输出长度应该是 1");
        let y = output[0];
        assert!(y > 0.0 && y < 1.0, "sigmoid 输出应在 (0, 1) 范围内");
        genann_free(ann_ptr.as_ptr());
    }
}

#[test]
fn test_genann_sigmoid_cached() {
    let x = 0.5;
    let cached = unsafe { genann_act_sigmoid_cached(x) };
    let normal = genann_act_sigmoid(x);
    assert!((cached - normal).abs() < 0.1, "缓存和非缓存 sigmoid 结果应相近");
}

#[test]
fn test_genann_multilayer() {
    let ann = genann_init(4, 3, 5, 2);
    assert!(ann.is_some(), "多隐藏层网络应该创建成功");
    if let Some(ann_ptr) = ann {
        genann_free(ann_ptr.as_ptr());
    }
}

