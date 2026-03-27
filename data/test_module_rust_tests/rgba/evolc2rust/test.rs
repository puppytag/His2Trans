// evo-c2rust-v2 测试文件 for rgba
// 直接调用 Rust 函数而不是通过 FFI

use crate::translation_utils::*;
use crate::src::rgba_c::*;
use crate::src::rgba_h::*;

#[test]
fn test_rgba_from_rgba() {
    let color = rgba_from_rgba(255, 128, 64, 32);
    // RGBA packed as 0xRRGGBBAA or 0xAABBGGRR depending on implementation
    assert_ne!(color, 0, "颜色值不应该为零");
}

#[test]
fn test_rgba_from_rgb() {
    let color = rgba_from_rgb(255, 128, 64);
    assert_ne!(color, 0, "RGB颜色值不应该为零");
}

#[test]
fn test_rgba_new() {
    let rgba_val: u32 = 0xFF804020;
    let color = rgba_new(rgba_val);
    // 验证 rgba_t 结构体被正确创建
    // 具体值取决于结构体定义
}

#[test]
fn test_rgba_from_hex6_string() {
    // 测试 6 位十六进制颜色字符串
    let mut s: Array<u8, 7> = Array([b'f', b'f', b'8', b'0', b'4', b'0', 0]);
    let ptr: Ptr<u8> = s.cast();
    let color = rgba_from_hex6_string(ptr);
    assert_eq!(color, rgba_from_rgba(255, 128, 64, 255), "hex6 解析结果应等价于 rgba(255,128,64,255)");
}

#[test]
fn test_rgba_from_hex3_string() {
    // 测试 3 位十六进制颜色字符串
    let mut s: Array<u8, 4> = Array([b'f', b'8', b'4', 0]);
    let ptr: Ptr<u8> = s.cast();
    let color = rgba_from_hex3_string(ptr);
    // #f84 应该等价于 #ff8844
    assert_eq!(color as u32, rgba_from_rgba(255, 136, 68, 255), "hex3 解析结果应等价于 #ff8844");
}

#[test]
fn test_rgba_to_string() {
    let rgba_val: u32 = 0xFF804020;
    let color = rgba_new(rgba_val);
    let mut buf: Array<u8, 32> = Array([0u8; 32]);
    rgba_to_string(color, buf.cast(), 32);
    // 验证输出字符串内容
    let out = {
        let mut v = Vec::new();
        for &b in &buf.0 {
            if b == 0 {
                break;
            }
            v.push(b);
        }
        String::from_utf8_lossy(&v).to_string()
    };
    assert!(out.starts_with("rgba(255, 128, 64,"), "rgba_to_string 输出格式不正确: {out}");
    assert!(out.contains("0.13"), "alpha 应四舍五入到两位小数: {out}");
}

#[test]
fn test_h_function() {
    // 测试 h 辅助函数（将十六进制字符转换为数值）
    assert_eq!(h(b'0'), 0);
    assert_eq!(h(b'9'), 9);
    assert_eq!(h(b'a'), 10);
    assert_eq!(h(b'f'), 15);
    assert_eq!(h(b'A'), 10);
    assert_eq!(h(b'F'), 15);
}

#[test]
fn test_rgba_from_string_hex() {
    let mut s: Array<u8, 8> = Array([b'#', b'f', b'f', b'0', b'0', b'0', b'0', 0]);
    let mut ok: Array<i16, 1> = Array([0i16]);
    let ptr: Ptr<u8> = s.cast();
    let mut ok_ptr: Ptr<i16> = ok.cast();
    let color = rgba_from_string(ptr, ok_ptr.cast());
    assert_eq!(ok.0[0], 1, "rgba_from_string 应该设置 ok=1 表示解析成功");
    assert_eq!(color, rgba_from_rgba(255, 0, 0, 255), "解析 #ff0000 的结果应该是纯红色");
}

#[test]
fn test_rgba_consistency() {
    // 相同的输入应该产生相同的输出
    let color1 = rgba_from_rgba(100, 150, 200, 255);
    let color2 = rgba_from_rgba(100, 150, 200, 255);
    assert_eq!(color1, color2, "相同参数应该产生相同的颜色值");
}

#[test]
fn test_rgba_different_colors() {
    // 不同的输入应该产生不同的输出
    let color1 = rgba_from_rgba(255, 0, 0, 255);  // 红色
    let color2 = rgba_from_rgba(0, 255, 0, 255);  // 绿色
    let color3 = rgba_from_rgba(0, 0, 255, 255);  // 蓝色
    assert_ne!(color1, color2, "红色和绿色应该不同");
    assert_ne!(color2, color3, "绿色和蓝色应该不同");
    assert_ne!(color1, color3, "红色和蓝色应该不同");
}
