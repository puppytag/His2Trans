// Rust translation of rgba test.c
// Copyright (c) 2024
// Translated from C to Rust

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_short};

// rgba_t type (32-bit color value)
pub type rgba_t = i32;

// External declarations for rgba library functions
extern "C" {
    fn rgba_from_string(str: *const c_char, ok: *mut c_short) -> i32;
    fn rgba_new(val: i32) -> rgba_t;
    fn rgba_to_string(color: rgba_t, buf: *mut c_char, len: usize);
}

fn test_named() {
    unsafe {
        let mut ok: c_short = 0;
        let color_name = CString::new("olive").unwrap();
        let val = rgba_from_string(color_name.as_ptr(), &mut ok);
        assert!(ok != 0, "Failed to parse 'olive'");
        assert_eq!(val as u32, 0x808000FF, "Expected olive color value");
    }
}

fn test_rgb() {
    unsafe {
        let mut ok: c_short = 0;

        let rgb1 = CString::new("rgb(255, 30   , 0)").unwrap();
        let val = rgba_from_string(rgb1.as_ptr(), &mut ok);
        assert!(ok != 0, "Failed to parse rgb(255, 30, 0)");
        assert_eq!(val as u32, 0xff1e00ff);

        let rgb2 = CString::new("rgb(0,0,0)").unwrap();
        let val = rgba_from_string(rgb2.as_ptr(), &mut ok);
        assert!(ok != 0, "Failed to parse rgb(0,0,0)");
        assert_eq!(val as u32, 0x000000ff);
    }
}

fn test_rgba() {
    unsafe {
        let mut ok: c_short = 0;

        let rgba1 = CString::new("rgba(255, 30   , 0, .5)").unwrap();
        let val = rgba_from_string(rgba1.as_ptr(), &mut ok);
        assert!(ok != 0, "Failed to parse rgba(255, 30, 0, .5)");
        assert_eq!(val as u32, 0xff1e007f);

        let rgba2 = CString::new("rgba(0,0,0, 1)").unwrap();
        let val = rgba_from_string(rgba2.as_ptr(), &mut ok);
        assert!(ok != 0, "Failed to parse rgba(0,0,0,1)");
        assert_eq!(val as u32, 0x000000ff);
    }
}

fn test_hex() {
    unsafe {
        let mut ok: c_short = 0;

        let hex1 = CString::new("#ff1e00").unwrap();
        let val = rgba_from_string(hex1.as_ptr(), &mut ok);
        assert!(ok != 0);
        assert_eq!(val as u32, 0xff1e00ff);

        let hex2 = CString::new("#ffffff").unwrap();
        let val = rgba_from_string(hex2.as_ptr(), &mut ok);
        assert!(ok != 0);
        assert_eq!(val as u32, 0xffffffff);

        let hex3 = CString::new("#ffcc00").unwrap();
        let val = rgba_from_string(hex3.as_ptr(), &mut ok);
        assert!(ok != 0);
        assert_eq!(val as u32, 0xffcc00ff);

        let hex4 = CString::new("#fco").unwrap();
        let val = rgba_from_string(hex4.as_ptr(), &mut ok);
        assert!(ok != 0);
        assert_eq!(val as u32, 0xffcc00ff);
    }
}

fn test_to_string() {
    unsafe {
        let mut buf: [c_char; 256] = [0; 256];

        let color = rgba_new(0xffcc00ffu32 as i32);
        rgba_to_string(color, buf.as_mut_ptr(), 256);
        let result = CStr::from_ptr(buf.as_ptr()).to_str().unwrap();
        assert_eq!(result, "#ffcc00");

        let color = rgba_new(0xffcc0050u32 as i32);
        rgba_to_string(color, buf.as_mut_ptr(), 256);
        let result = CStr::from_ptr(buf.as_ptr()).to_str().unwrap();
        assert_eq!(result, "rgba(255, 204, 0, 0.31)");
    }
}

fn main() {
    test_named();
    test_rgb();
    test_rgba();
    test_hex();
    test_to_string();
    println!("\n  \x1b[32m✓ \x1b[90mok\x1b[0m\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_colors() {
        test_named();
    }

    #[test]
    fn test_rgb_format() {
        test_rgb();
    }

    #[test]
    fn test_rgba_format() {
        test_rgba();
    }

    #[test]
    fn test_hex_format() {
        test_hex();
    }

    #[test]
    fn test_color_to_string() {
        test_to_string();
    }
}
