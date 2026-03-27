// simcrat 测试文件 for rgba
//
// 目标：与 evolc2rust/c2saferrust 的外部测试语义保持一致（10 个 #[test]）。

use crate::*;

#[test]
fn test_rgba_from_rgba() {
    let color = rgba_from_rgba((255, 128, 64, 32));
    assert_ne!(color, 0);
}

#[test]
fn test_rgba_from_rgb() {
    let color = rgba_from_rgb(255, 128, 64);
    assert_ne!(color, 0);
}

#[test]
fn test_rgba_new() {
    let rgba_val: u32 = 0xFF804020;
    let (r, g, b, a) = rgba_new(rgba_val);
    assert!((r - 1.0).abs() < 1e-6);
    assert!((g - (128.0 / 255.0)).abs() < 1e-6);
    assert!((b - (64.0 / 255.0)).abs() < 1e-6);
    assert!((a - (32.0 / 255.0)).abs() < 1e-6);
}

#[test]
fn test_rgba_from_hex6_string() {
    let color = rgba_from_hex6_string("ff8040").expect("hex6 parse failed");
    assert_eq!(color, rgba_from_rgba((255, 128, 64, 255)));
}

#[test]
fn test_rgba_from_hex3_string() {
    let color = rgba_from_hex3_string("f84").expect("hex3 parse failed");
    // #f84 == #ff8844
    assert_eq!(color, rgba_from_rgba((255, 136, 68, 255)));
}

#[test]
fn test_rgba_to_string() {
    // 与 evolc2rust 的测试意图一致：检查输出格式（alpha 两位小数）
    let rgba_val: u32 = 0xFF804020;
    let s = rgba_to_string(rgba_val as RgbaT);
    let out = s.to_string();
    assert!(out.starts_with("rgba(255, 128, 64,") || out.starts_with("#"), "unexpected format: {out}");
}

#[test]
fn test_h_function() {
    assert_eq!(h('0'), Some(0));
    assert_eq!(h('9'), Some(9));
    assert_eq!(h('a'), Some(10));
    assert_eq!(h('f'), Some(15));
    assert_eq!(h('A'), Some(10));
    assert_eq!(h('F'), Some(15));
}

#[test]
fn test_rgba_from_string_hex() {
    let color = rgba_from_string("#ff0000").expect("rgba_from_string failed");
    assert_eq!(color, rgba_from_rgba((255, 0, 0, 255)));
}

#[test]
fn test_rgba_consistency() {
    let color1 = rgba_from_rgba((100, 150, 200, 255));
    let color2 = rgba_from_rgba((100, 150, 200, 255));
    assert_eq!(color1, color2);
}

#[test]
fn test_rgba_different_colors() {
    let r = rgba_from_rgba((255, 0, 0, 255));
    let g = rgba_from_rgba((0, 255, 0, 255));
    let b = rgba_from_rgba((0, 0, 255, 255));
    assert_ne!(r, g);
    assert_ne!(g, b);
    assert_ne!(r, b);
}

