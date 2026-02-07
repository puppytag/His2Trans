//! Module: src_zlib_container
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

fn adler32(data: *const u8, size: usize) -> u32 {
    const SUMS_OVERFLOW: u32 = 5550;
    let mut s1: u32 = 1;
    let mut s2: u32 = 1 >> 16;
    let mut data_ptr = data;
    let mut remaining = size;
    while remaining > 0 {
        let mut amount = remaining;
        if amount > SUMS_OVERFLOW as usize {
            amount = SUMS_OVERFLOW as usize;
        }
        remaining -= amount;
        while amount > 0 {
            unsafe {
                s1 = s1.wrapping_add(*data_ptr as u32);
                data_ptr = data_ptr.add(1);
            }
            s2 = s2.wrapping_add(s1);
            amount -= 1;
        }
        s1 %= 65521;
        s2 %= 65521;
    }
    (s2 << 16) | s1
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_zlib_container_2
// c_function: ZopfliZlibCompress
// rust_file: src_zlib_container.rs
// rust_signature: pub extern "C" fn ZopfliZlibCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t)
// c_first_line: void ZopfliZlibCompress(const ZopfliOptions* options,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zlib_container_2/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_zlib_container.rs:127:21
//       |
//       |                         ------------- arguments to this function are incorrect
//       |                     ^^^^^^^^^^^^^^^^^^^^^ expected `libc::FILE`, found `types::FILE`
//       |
//      --> src/types.rs:69:1
//       |
// =================================
pub extern "C" fn ZopfliZlibCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_zlib_container::ZopfliZlibCompress(options as _, in_ as _, insize as _, out as _, outsize as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_zlib_container_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_zlib_container_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliZlibCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let mut bitpointer: u8 = 0;
    let checksum: u32 = crate::src_zlib_container::adler32(in_, insize as usize);
    let cmf: u32 = 120;
    let flevel: u32 = 3;
    let fdict: u32 = 0;
    let mut cmfflg: u32 = 256 * cmf + fdict * 32 + flevel * 64;
    let fcheck: u32 = 31 - cmfflg % 31;
    cmfflg += fcheck;

    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write((cmfflg / 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }
    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write((cmfflg % 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }

    crate::src_deflate::ZopfliDeflate(options, 2, 1, in_, insize, &mut bitpointer, out, outsize);

    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write(((checksum >> 24) % 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }
    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write(((checksum >> 16) % 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }
    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write(((checksum >> 8) % 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }
    unsafe {
        if ((*outsize) & ((*outsize).wrapping_sub(1))) == 0 {
            *out = if *outsize == 0 {
                libc::malloc(std::mem::size_of::<::core::ffi::c_uchar>()) as *mut ::core::ffi::c_uchar
            } else {
                libc::realloc(*out as *mut libc::c_void, (*outsize as usize).wrapping_mul(2).wrapping_mul(std::mem::size_of::<::core::ffi::c_uchar>())) as *mut ::core::ffi::c_uchar
            };
        }
        (*out).offset(*outsize as isize).write((checksum % 256) as u8);
        *outsize = (*outsize).wrapping_add(1);
    }

    unsafe {
        if !options.is_null() {
            let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void);
            let verbose = *(verbose_ptr as *const ::core::ffi::c_int);
            if verbose != 0 {
                let _ = libc::fprintf(
                    crate::compat::stderr,
                    b"Original Size: %d, Zlib: %d, Compression: %f%% Removed\n\0" as *const u8 as *const libc::c_char,
                    insize as libc::c_int,
                    *outsize as libc::c_int,
                    100.0f64 * (insize as f64 - *outsize as f64) / insize as f64,
                );
            }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_zlib_container_2
 * === C2R_LLM_FAILED_OUTPUT_END === */

