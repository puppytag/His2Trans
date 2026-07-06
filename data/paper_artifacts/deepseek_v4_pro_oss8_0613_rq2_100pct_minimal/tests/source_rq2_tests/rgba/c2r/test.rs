// C2R 框架测试文件 for rgba
//
// 说明：C2R 输出的 rgba 项目对外暴露的 API 主要是：
// - rgba_new(u32) -> rgba_t
// - rgba_to_string(rgba_t, buf, len)
// - rgba_from_string(str, ok) -> u32
// - rgba_inspect(u32)
//
// 目标：用这些公开 API 做真实语义测试（10 个 #[test]），并与其他方法的 suite 数量对齐。

use crate::src_rgba::*;
use crate::types::*;
use std::ffi::CString;

fn pack_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (u32::from(r) << 24) | (u32::from(g) << 16) | (u32::from(b) << 8) | u32::from(a)
}

fn c_buf_to_string(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).to_string()
}

#[test]
fn test_rgba_from_string_hex6() {
    let s = CString::new("#ff8040").unwrap();
    let mut ok: i16 = 0;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    assert_eq!(color, pack_rgba(255, 128, 64, 255));
}

#[test]
fn test_rgba_from_string_hex3() {
    let s = CString::new("#f84").unwrap();
    let mut ok: i16 = 0;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    // #f84 == #ff8844
    assert_eq!(color, pack_rgba(255, 136, 68, 255));
}

#[test]
fn test_rgba_from_string_rgb() {
    let s = CString::new("rgb(255, 128, 64)").unwrap();
    let mut ok: i16 = 0;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    assert_eq!(color, pack_rgba(255, 128, 64, 255));
}

#[test]
fn test_rgba_from_string_rgba_alpha_zero() {
    let s = CString::new("rgba(255, 128, 64, 0.0)").unwrap();
    let mut ok: i16 = 0;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    assert_eq!(color, pack_rgba(255, 128, 64, 0));
}

#[test]
fn test_rgba_to_string_rgba_format_for_transparent() {
    let rgba_val: u32 = 0xFF804020;
    let color: rgba_t = rgba_new(rgba_val);
    let mut buf: [u8; 64] = [0u8; 64];
    rgba_to_string(color, buf.as_mut_ptr() as *mut i8, 64 as size_t);
    let out = c_buf_to_string(&buf);
    assert!(
        out.starts_with("rgba(255, 128, 64,"),
        "rgba_to_string 输出格式不正确: {out}"
    );
    assert!(out.contains("0.13"), "alpha 应四舍五入到两位小数: {out}");
}

#[test]
fn test_rgba_to_string_hex_for_opaque() {
    let rgba_val: u32 = 0xFF8040FF;
    let color: rgba_t = rgba_new(rgba_val);
    let mut buf: [u8; 64] = [0u8; 64];
    rgba_to_string(color, buf.as_mut_ptr() as *mut i8, 64 as size_t);
    let out = c_buf_to_string(&buf);
    assert!(out.starts_with("#"), "opaque color should use hex: {out}");
    assert!(out.to_lowercase().contains("ff8040"), "hex content mismatch: {out}");
}

#[test]
fn test_rgba_from_string_named_color_red() {
    let s = CString::new("red").unwrap();
    let mut ok: i16 = 0;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    assert_eq!(color, pack_rgba(255, 0, 0, 255));
}

#[test]
fn test_rgba_consistency() {
    let s = CString::new("#6496c8").unwrap();
    let mut ok1: i16 = 0;
    let mut ok2: i16 = 0;
    let c1 = rgba_from_string(s.as_ptr(), &mut ok1 as *mut i16);
    let c2 = rgba_from_string(s.as_ptr(), &mut ok2 as *mut i16);
    assert_eq!(ok1, 1);
    assert_eq!(ok2, 1);
    assert_eq!(c1, c2);
}

#[test]
fn test_rgba_different_colors() {
    let mut ok: i16 = 0;
    let r = rgba_from_string(CString::new("#ff0000").unwrap().as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    let g = rgba_from_string(CString::new("#00ff00").unwrap().as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    let b = rgba_from_string(CString::new("#0000ff").unwrap().as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 1);
    assert_ne!(r, g);
    assert_ne!(g, b);
    assert_ne!(r, b);
}

#[test]
fn test_rgba_invalid_string_sets_ok_0() {
    let s = CString::new("not-a-color").unwrap();
    let mut ok: i16 = 1;
    let color = rgba_from_string(s.as_ptr(), &mut ok as *mut i16);
    assert_eq!(ok, 0);
    assert_eq!(color, 0);
}

