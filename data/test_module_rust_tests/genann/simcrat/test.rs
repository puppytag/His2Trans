// simcrat 测试文件 for genann (神经网络库)
//
// 目标：与 evolc2rust/c2saferrust 的外部测试语义保持一致（12 个 #[test]）。

use crate::*;

#[test]
fn test_genann_act_sigmoid() {
    // sigmoid(0) = 0.5
    let result = genann_act_sigmoid(0.0);
    assert!((result - 0.5).abs() < 0.001);
}

#[test]
fn test_genann_act_sigmoid_positive() {
    let result = genann_act_sigmoid(10.0);
    assert!(result > 0.99);
}

#[test]
fn test_genann_act_sigmoid_negative() {
    let result = genann_act_sigmoid(-10.0);
    assert!(result < 0.01);
}

#[test]
fn test_genann_act_threshold() {
    assert_eq!(genann_act_threshold(1.0), true);
    assert_eq!(genann_act_threshold(-1.0), false);
    assert_eq!(genann_act_threshold(0.0), false);
}

#[test]
fn test_genann_act_linear() {
    assert_eq!(genann_act_linear(5.0), 5.0);
    assert_eq!(genann_act_linear(-3.0), -3.0);
    assert_eq!(genann_act_linear(0.0), 0.0);
}

#[test]
fn test_genann_init() {
    let ann = genann_init(1, 1, 2, 1).expect("genann_init failed");
    drop(ann);
}

#[test]
fn test_genann_init_no_hidden() {
    let ann = genann_init(2, 0, 0, 1).expect("genann_init (no hidden) failed");
    drop(ann);
}

#[test]
fn test_genann_copy() {
    let ann = genann_init(2, 1, 3, 1).expect("genann_init failed");
    let copy = genann_copy(&ann);
    drop(copy);
    drop(ann);
}

#[test]
fn test_genann_randomize() {
    let mut ann = genann_init(2, 1, 3, 1).expect("genann_init failed");
    genann_randomize(&mut ann);
    drop(ann);
}

#[test]
fn test_genann_run() {
    let mut ann = genann_init(2, 1, 2, 1).expect("genann_init failed");
    let inputs = [0.5f64, 0.5f64];
    let out = genann_run(&mut ann, &inputs).expect("genann_run failed");
    assert!(!out.is_empty());
    let y = out[0];
    assert!(y > 0.0 && y < 1.0);
}

#[test]
fn test_genann_sigmoid_cached() {
    let x = 0.5;
    let cached = genann_act_sigmoid_cached(x);
    let normal = genann_act_sigmoid(x);
    assert!((cached - normal).abs() < 0.1);
}

#[test]
fn test_genann_multilayer() {
    let ann = genann_init(4, 3, 5, 2).expect("genann_init (multilayer) failed");
    drop(ann);
}

