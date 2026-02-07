//! Module: src_gzip_container
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

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
/// C: static const unsigned long[256] crc32_table
static mut crc32_table: [::core::ffi::c_ulong; 256usize] = unsafe { core::mem::MaybeUninit::<[::core::ffi::c_ulong; 256usize]>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

fn CRC(data: *const u8, size: usize) -> libc::c_ulong {
    let mut result: libc::c_ulong = 0xffffffffu32 as libc::c_ulong;
    let mut data_ptr = data;
    let mut sz = size;
    while sz > 0 {
        unsafe {
            let byte = *data_ptr;
            data_ptr = data_ptr.offset(1);
            result = crate::compat::crc32_table[((result ^ (byte as libc::c_ulong)) & 0xff) as usize] ^ (result >> 8);
        }
        sz -= 1;
    }
    result ^ 0xffffffffu32 as libc::c_ulong
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_gzip_container_2
// c_function: ZopfliGzipCompress
// rust_file: src_gzip_container.rs
// rust_signature: pub extern "C" fn ZopfliGzipCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t)
// c_first_line: void ZopfliGzipCompress(const ZopfliOptions* options,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_gzip_container_2/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ZopfliOptions__verbose` in module `crate::compat`
//      --> src/src_gzip_container.rs:77:38
//       |
//       |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     -------------------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_ZopfliOptions__verbose` defined here
//       |
// =================================
pub extern "C" fn ZopfliGzipCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_gzip_container::ZopfliGzipCompress(options as _, in_ as _, insize as _, out as _, outsize as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_gzip_container_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_gzip_container_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliGzipCompress(options: *const crate::types::ZopfliOptions, in_: *const ::core::ffi::c_uchar, insize: crate::types::size_t, out: *mut *mut ::core::ffi::c_uchar, outsize: *mut crate::types::size_t) {
    let crcvalue = crate::src_gzip_container::CRC(in_, insize as usize);
    let mut bp: u8 = 0;
    unsafe {
        let mut append = |value: u8| {
            let cur = *outsize;
            if cur & (cur - 1) == 0 {
                let new_size = if cur == 0 { 1 } else { cur * 2 };
                *out = if cur == 0 {
                    libc::malloc(new_size as usize) as *mut ::core::ffi::c_uchar
                } else {
                    libc::realloc(*out as *mut libc::c_void, new_size as usize) as *mut ::core::ffi::c_uchar
                };
            }
            let out_ptr = *out;
            if !out_ptr.is_null() {
                *out_ptr.offset(*outsize as isize) = value;
            }
            *outsize += 1;
        };
        append(31);
        append(139);
        append(8);
        append(0);
        append(0);
        append(0);
        append(0);
        append(0);
        append(2);
        append(3);
        crate::src_deflate::ZopfliDeflate(options, 2, 1, in_, insize, &mut bp, out, outsize);
        append((crcvalue % 256) as u8);
        append(((crcvalue >> 8) % 256) as u8);
        append(((crcvalue >> 16) % 256) as u8);
        append(((crcvalue >> 24) % 256) as u8);
        append((insize % 256) as u8);
        append(((insize >> 8) % 256) as u8);
        append(((insize >> 16) % 256) as u8);
        append(((insize >> 24) % 256) as u8);
        let verbose = crate::compat::c2r_field_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void);
        if verbose != 0 {
            let insize_i32 = insize as i32;
            let outsize_i32 = *outsize as i32;
            let ratio = 100.0 * (insize_i32 - outsize_i32) as f64 / insize_i32 as f64;
            libc::fprintf(
                crate::compat::stderr,
                b"Original Size: %d, Gzip: %d, Compression: %f%% Removed\n\0".as_ptr() as *const i8,
                insize_i32,
                outsize_i32,
                ratio,
            );
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_gzip_container_2
 * === C2R_LLM_FAILED_OUTPUT_END === */

