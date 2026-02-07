//! Module: src_rgba
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

mod local_types {
    include!("__c2r_generated/local_types_src_rgba.rs");
}
use local_types::{named_color};



// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
/// C: static struct named_color[149] named_colors
static mut named_colors: [named_color; 149usize] = unsafe { core::mem::MaybeUninit::<[named_color; 149usize]>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

fn h(c: std::os::raw::c_char) -> std::os::raw::c_int {
    match c as u8 as char {
        '0'..='9' => (c as u8 - b'0') as i32,
        'a'..='f' => (c as u8 - b'a' + 10) as i32,
        'A'..='F' => (c as u8 - b'A' + 10) as i32,
        _ => 0,
    }
}

pub extern "C" fn rgba_new(rgba: u32) -> crate::types::rgba_t {
    let r = (rgba >> 24) as f64 / 255.0;
    let g = ((rgba >> 16) & 0xff) as f64 / 255.0;
    let b = ((rgba >> 8) & 0xff) as f64 / 255.0;
    let a = (rgba & 0xff) as f64 / 255.0;
    unsafe {
        let mut color: crate::types::rgba_t = std::mem::zeroed();
        *(&mut color as *mut _ as *mut f64).offset(0) = r;
        *(&mut color as *mut _ as *mut f64).offset(1) = g;
        *(&mut color as *mut _ as *mut f64).offset(2) = b;
        *(&mut color as *mut _ as *mut f64).offset(3) = a;
        color
    }
}

pub extern "C" fn rgba_to_string(rgba: crate::types::rgba_t, buf: *mut ::core::ffi::c_char, len: crate::types::size_t) {
    let _ = rgba;
    let _ = buf;
    let _ = len;
}

fn rgba_from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
}

fn rgba_from_rgb(r: u8, g: u8, b: u8) -> i32 {
    crate::src_rgba::rgba_from_rgba(r, g, b, 255) as i32
}

fn rgba_from_hex6_string(str: *const std::ffi::c_char) -> u32 {
    unsafe {
        let s = str;
        let r = (crate::src_rgba::h(*s.offset(0)) << 4) + crate::src_rgba::h(*s.offset(1));
        let g = (crate::src_rgba::h(*s.offset(2)) << 4) + crate::src_rgba::h(*s.offset(3));
        let b = (crate::src_rgba::h(*s.offset(4)) << 4) + crate::src_rgba::h(*s.offset(5));
        crate::src_rgba::rgba_from_rgb(r as u8, g as u8, b as u8) as u32
    }
}

fn rgba_from_hex3_string(str: *const std::ffi::c_char) -> i32 {
    unsafe {
        let r = (crate::src_rgba::h(*str.offset(0)) << 4) + crate::src_rgba::h(*str.offset(0));
        let g = (crate::src_rgba::h(*str.offset(1)) << 4) + crate::src_rgba::h(*str.offset(1));
        let b = (crate::src_rgba::h(*str.offset(2)) << 4) + crate::src_rgba::h(*str.offset(2));
        crate::src_rgba::rgba_from_rgb(r as u8, g as u8, b as u8)
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_rgba_8
// c_function: rgba_from_rgb_string
// rust_file: src_rgba.rs
// rust_signature: fn rgba_from_rgb_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32
// c_first_line: static int32_t
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/repair_history/rgba/translate_by_qwen3_coder/_manual_fix/src_rgba_8/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//     --> src/src_rgba.rs:97:21
//      |
//      |                     ^^^ types differ in mutability
//      |
//   error: could not compile `rgba` (bin "rgba") due to 1 previous error
// =================================
fn rgba_from_rgb_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_rgba::rgba_from_rgb_string(str as _, ok as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_rgba_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/repair_history/rgba/translate_by_qwen3_coder/_manual_fix/src_rgba_8/translated_rust.rs
 * ------------------------------------------------------------
fn rgba_from_rgb_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        if str.is_null() {
            if !ok.is_null() {
                *ok = 0;
            }
            return 0;
        }
        let prefix = b"rgb(\0" as *const u8 as *const std::ffi::c_char;
        let found = libc::strstr(str, prefix);
        if found == str {
            let mut s = str.offset(4);
            while *s == b' ' as i8 {
                s = s.offset(1);
            }
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut c: i32;
            c = 0;
            if *s >= b'0' as i8 && *s <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*s as i32) - (b'0' as i32);
                    s = s.offset(1);
                    if !(*s >= b'0' as i8 && *s <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            r = c as u8;
            while *s == b' ' as i8 || *s == b',' as i8 {
                s = s.offset(1);
            }
            c = 0;
            if *s >= b'0' as i8 && *s <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*s as i32) - (b'0' as i32);
                    s = s.offset(1);
                    if !(*s >= b'0' as i8 && *s <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            g = c as u8;
            while *s == b' ' as i8 || *s == b',' as i8 {
                s = s.offset(1);
            }
            c = 0;
            if *s >= b'0' as i8 && *s <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*s as i32) - (b'0' as i32);
                    s = s.offset(1);
                    if !(*s >= b'0' as i8 && *s <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            b = c as u8;
            while *s == b' ' as i8 || *s == b',' as i8 {
                s = s.offset(1);
            }
            if !ok.is_null() {
                *ok = 1;
            }
            return crate::src_rgba::rgba_from_rgb(r, g, b);
        }
        if !ok.is_null() {
            *ok = 0;
        }
        0
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_rgba_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_rgba_9
// c_function: rgba_from_rgba_string
// rust_file: src_rgba.rs
// rust_signature: fn rgba_from_rgba_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32
// c_first_line: static int32_t
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/repair_history/rgba/translate_by_qwen3_coder/_manual_fix/src_rgba_9/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//     --> src/src_rgba.rs:89:17
//      |
//      |                 ^^^ types differ in mutability
//      |
//   error: could not compile `rgba` (bin "rgba") due to 1 previous error
// =================================
fn rgba_from_rgba_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_rgba::rgba_from_rgba_string(str as _, ok as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_rgba_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/repair_history/rgba/translate_by_qwen3_coder/_manual_fix/src_rgba_9/translated_rust.rs
 * ------------------------------------------------------------
fn rgba_from_rgba_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        if str.is_null() {
            if !ok.is_null() {
                *ok = 0;
            }
            return 0;
        }
        let s = libc::strstr(str, b"rgba(\0".as_ptr() as *const _);
        if s == str {
            let mut p = str.offset(5);
            while *p == b' ' as i8 {
                p = p.offset(1);
            }
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut c: i32;
            let mut a: f32 = 0.0;
            c = 0;
            if *p >= b'0' as i8 && *p <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*p - b'0' as i8) as i32;
                    p = p.offset(1);
                    if !(*p >= b'0' as i8 && *p <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            r = c as u8;
            while *p == b' ' as i8 || *p == b',' as i8 {
                p = p.offset(1);
            }
            c = 0;
            if *p >= b'0' as i8 && *p <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*p - b'0' as i8) as i32;
                    p = p.offset(1);
                    if !(*p >= b'0' as i8 && *p <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            g = c as u8;
            while *p == b' ' as i8 || *p == b',' as i8 {
                p = p.offset(1);
            }
            c = 0;
            if *p >= b'0' as i8 && *p <= b'9' as i8 {
                loop {
                    c *= 10;
                    c += (*p - b'0' as i8) as i32;
                    p = p.offset(1);
                    if !(*p >= b'0' as i8 && *p <= b'9' as i8) {
                        break;
                    }
                }
            } else {
                if !ok.is_null() {
                    *ok = 0;
                }
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            b = c as u8;
            while *p == b' ' as i8 || *p == b',' as i8 {
                p = p.offset(1);
            }
            if *p >= b'1' as i8 && *p <= b'9' as i8 {
                a = 1.0;
            } else {
                if *p == b'0' as i8 {
                    p = p.offset(1);
                }
                if *p == b'.' as i8 {
                    p = p.offset(1);
                    let mut n: f32 = 0.1;
                    while *p >= b'0' as i8 && *p <= b'9' as i8 {
                        a += (*p - b'0' as i8) as f32 * n;
                        n *= 0.1;
                        p = p.offset(1);
                    }
                }
            }
            if !ok.is_null() {
                *ok = 1;
            }
            let a_byte = (a * 255.0) as u8;
            return crate::src_rgba::rgba_from_rgba(r, g, b, a_byte) as i32;
        }
        if !ok.is_null() {
            *ok = 0;
        }
        0
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_rgba_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn rgba_from_hex_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        let len = libc::strlen(str);
        if !ok.is_null() {
            *ok = 1;
        }
        if len == 6 {
            return crate::src_rgba::rgba_from_hex6_string(str) as i32;
        }
        if len == 3 {
            return crate::src_rgba::rgba_from_hex3_string(str);
        }
        if !ok.is_null() {
            *ok = 0;
        }
        0
    }
}

fn rgba_from_name_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    let mut i: usize = 0;
    unsafe {
        while !named_colors[i].name.is_null() {
            let color = named_colors[i];
            i += 1;
            if *str == *color.name && libc::strcmp(str, color.name) == 0 {
                *ok = 1;
                return color.val as i32;
            }
        }
        *ok = 0;
    }
    0
}

pub extern "C" fn rgba_from_string(str_: *const ::core::ffi::c_char, ok: *mut ::core::ffi::c_short) -> u32 {
    unsafe {
        if !str_.is_null() && *str_ as u8 == b'#' {
            let next_str = str_.offset(1);
            return crate::src_rgba::rgba_from_hex_string(next_str, ok) as u32;
        }
        let strstr_result = libc::strstr(str_, b"rgba\0".as_ptr() as *const _);
        if !str_.is_null() && str_ == strstr_result {
            return crate::src_rgba::rgba_from_rgba_string(str_, ok) as u32;
        }
        let strstr_result2 = libc::strstr(str_, b"rgb\0".as_ptr() as *const _);
        if !str_.is_null() && str_ == strstr_result2 {
            return crate::src_rgba::rgba_from_rgb_string(str_, ok) as u32;
        }
        crate::src_rgba::rgba_from_name_string(str_, ok) as u32
    }
}

pub extern "C" fn rgba_inspect(rgba: u32) {
    unsafe {
        crate::compat::printf(
            b"rgba(%d,%d,%d,%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
            (rgba >> 24 & 0xff) as ::core::ffi::c_int,
            (rgba >> 16 & 0xff) as ::core::ffi::c_int,
            (rgba >> 8 & 0xff) as ::core::ffi::c_int,
            (rgba & 0xff) as ::core::ffi::c_int,
        );
    }
}
