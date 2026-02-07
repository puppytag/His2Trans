//! Module: src_cache
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_cache_1
// c_function: ZopfliInitCache
// rust_file: src_cache.rs
// rust_signature: pub extern "C" fn ZopfliInitCache(blocksize: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache)
// c_first_line: void ZopfliInitCache(size_t blocksize, ZopfliLongestMatchCache* lmc) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_cache_1/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_ZopfliLongestMatchCache__length` in module `crate::compat`
//     --> src/src_cache.rs:31:43
//      |
//      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//      |
//      |
//      |     ----------------------------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_ZopfliLongestMatchCache__sublen` defined here
//      |
// =================================
pub extern "C" fn ZopfliInitCache(blocksize: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_cache::ZopfliInitCache(blocksize as _, lmc as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_cache_1
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/zopfli/workspace/repair_history/zopfli/translate_by_qwen3_coder/_manual_fix/src_cache_1/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ZopfliInitCache(blocksize: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache) {
    unsafe {
        let blocksize_usize = blocksize as usize;
        let length_ptr = libc::malloc((std::mem::size_of::<libc::c_ushort>() as usize).wrapping_mul(blocksize_usize)) as *mut libc::c_ushort;
        let dist_ptr = libc::malloc((std::mem::size_of::<libc::c_ushort>() as usize).wrapping_mul(blocksize_usize)) as *mut libc::c_ushort;
        let sublen_size = 8usize.wrapping_mul(3).wrapping_mul(blocksize_usize);
        let sublen_ptr = libc::malloc(sublen_size) as *mut libc::c_uchar;
        if sublen_ptr.is_null() {
            let _ = libc::fprintf(
                crate::compat::stderr,
                b"Error: Out of memory. Tried allocating %lu bytes of memory.\n\0" as *const u8 as *const libc::c_char,
                (8u64).wrapping_mul(3).wrapping_mul(blocksize) as libc::c_ulong,
            );
            libc::exit(1);
        }
        let lmc_void = lmc as *mut ::core::ffi::c_void;
        let length_field = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__length(lmc_void) as *mut *mut libc::c_ushort;
        let dist_field = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__dist(lmc_void) as *mut *mut libc::c_ushort;
        let sublen_field = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc_void) as *mut *mut libc::c_uchar;
        *length_field = length_ptr;
        *dist_field = dist_ptr;
        *sublen_field = sublen_ptr;
        let mut i: usize = 0;
        while i < blocksize_usize {
            (*length_field).add(i).write(1);
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < blocksize_usize {
            (*dist_field).add(i).write(0);
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < 8usize.wrapping_mul(blocksize_usize).wrapping_mul(3) {
            (*sublen_field).add(i).write(0);
            i = i.wrapping_add(1);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_cache_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ZopfliCleanCache(lmc: *mut crate::types::ZopfliLongestMatchCache) {
    if lmc.is_null() {
        return;
    }
    unsafe {
        let lmc_void = lmc as *mut ::core::ffi::c_void;
        let sublen_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc_void) as *mut *mut ::core::ffi::c_void;
        if !(*sublen_ptr).is_null() {
            libc::free(*sublen_ptr as *mut libc::c_void);
        }
        let dist_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc_void) as *mut *mut ::core::ffi::c_void;
        if !(*dist_ptr).is_null() {
            libc::free(*dist_ptr as *mut libc::c_void);
        }
        let length_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc_void) as *mut *mut ::core::ffi::c_void;
        if !(*length_ptr).is_null() {
            libc::free(*length_ptr as *mut libc::c_void);
        }
    }
}

pub extern "C" fn ZopfliSublenToCache(sublen: *const ::core::ffi::c_ushort, pos: crate::types::size_t, length: crate::types::size_t, lmc: *mut crate::types::ZopfliLongestMatchCache) {
    let mut i: crate::types::size_t;
    let mut j: crate::types::size_t = 0;
    let mut bestlength: ::core::ffi::c_uint = 0;
    let mut cache: *mut ::core::ffi::c_uchar;

    unsafe {
        let sublen_ptr = crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc as *mut ::core::ffi::c_void);
        cache = sublen_ptr as *mut ::core::ffi::c_uchar;
        cache = cache.offset((8 * pos * 3) as isize);
    }
    if length < 3 {
        return;
    }
    i = 3;
    while i <= length {
        if i == length || unsafe { *sublen.offset(i as isize) } != unsafe { *sublen.offset((i + 1) as isize) } {
            unsafe {
                *cache.offset((j * 3) as isize) = (i - 3) as ::core::ffi::c_uchar;
                let sublen_val = *sublen.offset(i as isize);
                *cache.offset((j * 3 + 1) as isize) = (sublen_val % 256) as ::core::ffi::c_uchar;
                *cache.offset((j * 3 + 2) as isize) = ((sublen_val >> 8) % 256) as ::core::ffi::c_uchar;
            }
            bestlength = i as ::core::ffi::c_uint;
            j += 1;
            if j >= 8 {
                break;
            }
        }
        i += 1;
    }
    if j < 8 {
        let _ = (bestlength == length as ::core::ffi::c_uint);
        unsafe {
            *cache.offset(((8 - 1) * 3) as isize) = (bestlength - 3) as ::core::ffi::c_uchar;
        }
    } else {
        let _ = (bestlength <= length as ::core::ffi::c_uint);
    }
    let max_cached = crate::src_cache::ZopfliMaxCachedSublen(lmc as *const crate::types::ZopfliLongestMatchCache, pos, length);
    let _ = (bestlength == max_cached);
}

pub extern "C" fn ZopfliCacheToSublen(lmc: *const crate::types::ZopfliLongestMatchCache, pos: crate::types::size_t, length: crate::types::size_t, sublen: *mut ::core::ffi::c_ushort) {
    if length < 3 {
        return;
    }
    let maxlength = crate::src_cache::ZopfliMaxCachedSublen(lmc, pos, length);
    let mut prevlength: ::core::ffi::c_uint = 0;
    let cache_ptr = unsafe {
        crate::compat::c2r_field_ptr_ZopfliLongestMatchCache__sublen(lmc as *mut ::core::ffi::c_void)
    };
    let cache = unsafe { (cache_ptr as *mut ::core::ffi::c_uchar).add((8 * pos * 3) as usize) };
    for j in 0..8 {
        let length_j = unsafe { *cache.add((j * 3) as usize) as ::core::ffi::c_uint } + 3;
        let dist_lo = unsafe { *cache.add((j * 3 + 1) as usize) as ::core::ffi::c_uint };
        let dist_hi = unsafe { *cache.add((j * 3 + 2) as usize) as ::core::ffi::c_uint };
        let dist = dist_lo + 256 * dist_hi;
        let mut i = prevlength;
        while i <= length_j {
            unsafe {
                *sublen.add(i as usize) = dist as ::core::ffi::c_ushort;
            }
            i += 1;
        }
        if length_j == maxlength {
            break;
        }
        prevlength = length_j + 1;
    }
}

pub extern "C" fn ZopfliMaxCachedSublen(lmc: *const crate::types::ZopfliLongestMatchCache, pos: crate::types::size_t, length: crate::types::size_t) -> ::core::ffi::c_uint {
    let _ = length;
    if lmc.is_null() {
        return 0;
    }
    unsafe {
        let cache = (lmc as *const u8).add((8 * pos * 3) as usize);
        if *cache.add(1) == 0 && *cache.add(2) == 0 {
            return 0;
        }
        return (*cache.add((8 - 1) * 3)) as ::core::ffi::c_uint + 3;
    }
}
