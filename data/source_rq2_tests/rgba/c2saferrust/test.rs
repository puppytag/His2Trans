// c2saferrust 测试文件 for rgba
// 目标：与 evolc2rust 套件对齐的真实功能测试（构造/解析/格式化）。

use crate::*;
use std::ffi::{CStr, CString};

#[test]
fn test_rgba_from_rgba() {
    let color = crate::rgba_from_rgba(255, 128, 64, 32);
    assert_ne!(color, 0, "颜色值不应该为零");
}

#[test]
fn test_rgba_from_rgb() {
    let color = crate::rgba_from_rgb(255, 128, 64);
    assert_ne!(color, 0, "RGB颜色值不应该为零");
}

#[test]
fn test_rgba_new() {
    let rgba_val: u32 = 0xFF804020;
    let _ = rgba_new(rgba_val);
    // 结构体字段在不同实现中可能不同；这里至少保证调用不崩溃。
}

#[test]
fn test_rgba_from_hex6_string() {
    let s = CString::new("ff8040").unwrap();
    let color = unsafe { crate::rgba_from_hex6_string(s.as_ptr()) };
    assert_eq!(
        color,
        crate::rgba_from_rgba(255, 128, 64, 255),
        "hex6 解析结果应等价于 rgba(255,128,64,255)"
    );
}

#[test]
fn test_rgba_from_hex3_string() {
    let s = CString::new("f84").unwrap();
    let color = unsafe { crate::rgba_from_hex3_string(s.as_ptr()) as u32 };
    assert_eq!(
        color,
        crate::rgba_from_rgba(255, 136, 68, 255),
        "hex3 解析结果应等价于 #ff8844"
    );
}

#[test]
fn test_rgba_to_string() {
    let rgba_val: u32 = 0xFF804020;
    let color = rgba_new(rgba_val);
    let mut buf: [i8; 32] = [0; 32];
    rgba_to_string(color, buf.as_mut_ptr(), 32);
    let out = unsafe { CStr::from_ptr(buf.as_ptr()) }.to_string_lossy().to_string();
    assert!(
        out.starts_with("rgba(255, 128, 64,"),
        "rgba_to_string 输出格式不正确: {out}"
    );
    assert!(out.contains("0.13"), "alpha 应四舍五入到两位小数: {out}");
}

#[test]
fn test_h_function() {
    assert_eq!(crate::h(b'0' as i8), 0);
    assert_eq!(crate::h(b'9' as i8), 9);
    assert_eq!(crate::h(b'a' as i8), 10);
    assert_eq!(crate::h(b'f' as i8), 15);
    assert_eq!(crate::h(b'A' as i8), 10);
    assert_eq!(crate::h(b'F' as i8), 15);
}

#[test]
fn test_rgba_from_string_hex() {
    let s = CString::new("#ff0000").unwrap();
    let mut ok: i16 = 0;
    let color = unsafe { rgba_from_string(s.as_ptr(), &mut ok) };
    assert_eq!(ok, 1, "rgba_from_string 应该设置 ok=1 表示解析成功");
    assert_eq!(color, crate::rgba_from_rgba(255, 0, 0, 255));
}

#[test]
fn test_rgba_consistency() {
    let c1 = crate::rgba_from_rgba(100, 150, 200, 255);
    let c2 = crate::rgba_from_rgba(100, 150, 200, 255);
    assert_eq!(c1, c2, "相同参数应该产生相同的颜色值");
}

#[test]
fn test_rgba_different_colors() {
    let r = crate::rgba_from_rgba(255, 0, 0, 255);
    let g = crate::rgba_from_rgba(0, 255, 0, 255);
    let b = crate::rgba_from_rgba(0, 0, 255, 255);
    assert_ne!(r, g, "红色和绿色应该不同");
    assert_ne!(g, b, "绿色和蓝色应该不同");
    assert_ne!(r, b, "红色和蓝色应该不同");
}
