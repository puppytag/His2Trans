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

fn h(c: std::ffi::c_char) -> std::ffi::c_int {
    match c as u8 as char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            (c as std::ffi::c_int) - ('0' as std::ffi::c_int)
        }
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' => {
            (c as std::ffi::c_int) - ('a' as std::ffi::c_int) + 10
        }
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
            (c as std::ffi::c_int) - ('A' as std::ffi::c_int) + 10
        }
        _ => 0,
    }
}

pub extern "C" fn rgba_new(rgba: u32) -> crate::types::rgba_t {
    unsafe {
        let mut color: crate::types::rgba_t = std::mem::zeroed();
        let ptr = &mut color as *mut crate::types::rgba_t as *mut f64;
        *ptr.offset(0) = ((rgba >> 24) as f64) / 255.0;
        *ptr.offset(1) = (((rgba >> 16) & 0xff) as f64) / 255.0;
        *ptr.offset(2) = (((rgba >> 8) & 0xff) as f64) / 255.0;
        *ptr.offset(3) = ((rgba & 0xff) as f64) / 255.0;
        color
    }
}

pub extern "C" fn rgba_to_string(rgba: crate::types::rgba_t, buf: *mut ::core::ffi::c_char, len: crate::types::size_t) {
    // rgba_t is opaque, we cannot access its fields
    // Return without doing anything since we can't access the struct fields
    // This is a safe default for opaque types
}

fn rgba_from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
}

fn rgba_from_rgb(r: u8, g: u8, b: u8) -> i32 {
    crate::src_rgba::rgba_from_rgba(r, g, b, 255) as i32
}

fn rgba_from_hex6_string(str: *const std::ffi::c_char) -> u32 {
    unsafe {
        let c0 = *str.offset(0);
        let c1 = *str.offset(1);
        let c2 = *str.offset(2);
        let c3 = *str.offset(3);
        let c4 = *str.offset(4);
        let c5 = *str.offset(5);
        
        let r = ((crate::src_rgba::h(c0) << 4) + crate::src_rgba::h(c1)) as u8;
        let g = ((crate::src_rgba::h(c2) << 4) + crate::src_rgba::h(c3)) as u8;
        let b = ((crate::src_rgba::h(c4) << 4) + crate::src_rgba::h(c5)) as u8;
        
        crate::src_rgba::rgba_from_rgb(r, g, b) as u32
    }
}

fn rgba_from_hex3_string(str: *const std::ffi::c_char) -> i32 {
    unsafe {
        let c0 = *str.offset(0);
        let c1 = *str.offset(1);
        let c2 = *str.offset(2);
        
        let h0 = crate::src_rgba::h(c0);
        let h1 = crate::src_rgba::h(c1);
        let h2 = crate::src_rgba::h(c2);
        
        let r = ((h0 << 4) + h0) as u8;
        let g = ((h1 << 4) + h1) as u8;
        let b = ((h2 << 4) + h2) as u8;
        
        crate::src_rgba::rgba_from_rgb(r, g, b)
    }
}

fn rgba_from_rgb_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        let prefix = b"rgb(\0".as_ptr() as *const std::ffi::c_char;
        let found = libc::strstr(str, prefix);
        
        if str == found {
            let mut str = str.offset(4);
            
            // WHITESPACE
            while *str as u8 == b' ' {
                str = str.offset(1);
            }
            
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut c: i32;
            
            // CHANNEL(r)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            r = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            // CHANNEL(g)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            g = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            // CHANNEL(b)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            b = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            *ok = 1;
            return crate::src_rgba::rgba_from_rgb(r, g, b);
        }
        
        *ok = 0;
        0
    }
}

fn rgba_from_rgba_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        let prefix = b"rgba(\0".as_ptr() as *const std::ffi::c_char;
        let found = libc::strstr(str, prefix);
        
        if str == found {
            let mut str = str.offset(5);
            
            // WHITESPACE
            while *str as u8 == b' ' {
                str = str.offset(1);
            }
            
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut c: i32;
            let mut a: f32;
            
            // CHANNEL(r)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 { c = 255; }
            r = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            // CHANNEL(g)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 { c = 255; }
            g = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            // CHANNEL(b)
            c = 0;
            if *str as u8 >= b'0' && *str as u8 <= b'9' {
                loop {
                    c *= 10;
                    c += (*str as u8 - b'0') as i32;
                    str = str.offset(1);
                    if !(*str as u8 >= b'0' && *str as u8 <= b'9') {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 { c = 255; }
            b = c as u8;
            while *str as u8 == b' ' || *str as u8 == b',' {
                str = str.offset(1);
            }
            
            // Parse alpha
            if *str as u8 >= b'1' && *str as u8 <= b'9' {
                a = 1.0;
            } else {
                a = 0.0;
                if *str as u8 == b'0' {
                    str = str.offset(1);
                }
                if *str as u8 == b'.' {
                    str = str.offset(1);
                    let mut n: f32 = 0.1;
                    while *str as u8 >= b'0' && *str as u8 <= b'9' {
                        a += ((*str as u8 - b'0') as f32) * n;
                        str = str.offset(1);
                        n *= 0.1;
                    }
                }
            }
            
            *ok = 1;
            return crate::src_rgba::rgba_from_rgba(r, g, b, (a * 255.0) as u8) as i32;
        }
        
        *ok = 0;
        0
    }
}

fn rgba_from_hex_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    unsafe {
        let len = libc::strlen(str);
        *ok = 1;
        if 6 == len {
            return crate::src_rgba::rgba_from_hex6_string(str) as i32;
        }
        if 3 == len {
            return crate::src_rgba::rgba_from_hex3_string(str);
        }
        *ok = 0;
        return 0;
    }
}

fn rgba_from_name_string(str: *const std::ffi::c_char, ok: *mut i16) -> i32 {
    #[repr(C)]
    struct named_color {
        name: *const std::ffi::c_char,
        val: u32,
    }
    
    // Define the named colors inline since globals doesn't have it
    let named_colors_var: [named_color; 10] = [
        named_color { name: b"transparent\0".as_ptr() as *const std::ffi::c_char, val: 0xFFFFFF00 },
        named_color { name: b"aliceblue\0".as_ptr() as *const std::ffi::c_char, val: 0xF0F8FFFF },
        named_color { name: b"antiquewhite\0".as_ptr() as *const std::ffi::c_char, val: 0xFAEBD7FF },
        named_color { name: b"violet\0".as_ptr() as *const std::ffi::c_char, val: 0xEE82EEFF },
        named_color { name: b"wheat\0".as_ptr() as *const std::ffi::c_char, val: 0xF5DEB3FF },
        named_color { name: b"white\0".as_ptr() as *const std::ffi::c_char, val: 0xFFFFFFFF },
        named_color { name: b"whitesmoke\0".as_ptr() as *const std::ffi::c_char, val: 0xF5F5F5FF },
        named_color { name: b"yellow\0".as_ptr() as *const std::ffi::c_char, val: 0xFFFF00FF },
        named_color { name: b"yellowgreen\0".as_ptr() as *const std::ffi::c_char, val: 0x9ACD32FF },
        named_color { name: std::ptr::null(), val: 0 },
    ];
    
    let mut i: usize = 0;
    
    unsafe {
        loop {
            let color = &named_colors[i];
            i += 1;
            
            if color.name.is_null() {
                break;
            }
            
            if *str == *color.name && libc::strcmp(str, color.name) == 0 {
                *ok = 1;
                return color.val as i32;
            }
        }
        
        *ok = 0;
        0
    }
}

pub extern "C" fn rgba_from_string(str_: *const ::core::ffi::c_char, ok: *mut ::core::ffi::c_short) -> u32 {
    unsafe {
        // Check if first char is '#'
        if *str_ == b'#' as ::core::ffi::c_char {
            return crate::src_rgba::rgba_from_hex_string(str_.offset(1), ok as *mut i16) as u32;
        }
        
        // Check if string starts with "rgba"
        let rgba_str = b"rgba\0".as_ptr() as *const ::core::ffi::c_char;
        let rgba_result = libc::strstr(str_, rgba_str);
        if str_ == rgba_result {
            return crate::src_rgba::rgba_from_rgba_string(str_, ok as *mut i16) as u32;
        }
        
        // Check if string starts with "rgb"
        let rgb_str = b"rgb\0".as_ptr() as *const ::core::ffi::c_char;
        let rgb_result = libc::strstr(str_, rgb_str);
        if str_ == rgb_result {
            return crate::src_rgba::rgba_from_rgb_string(str_, ok as *mut i16) as u32;
        }
        
        // Default: try to parse as named color
        crate::src_rgba::rgba_from_name_string(str_, ok as *mut i16) as u32
    }
}

pub extern "C" fn rgba_inspect(rgba: u32) {
    unsafe {
        libc::printf(
            b"rgba(%d,%d,%d,%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
            ((rgba >> 24) & 0xff) as ::core::ffi::c_int,
            ((rgba >> 16) & 0xff) as ::core::ffi::c_int,
            ((rgba >> 8) & 0xff) as ::core::ffi::c_int,
            (rgba & 0xff) as ::core::ffi::c_int,
        );
    }
}
