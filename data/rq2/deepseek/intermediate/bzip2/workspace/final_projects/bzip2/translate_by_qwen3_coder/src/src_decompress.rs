//! Module: src_decompress
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
// func_key: src_decompress_1
// c_function: makeMaps_d
// rust_file: src_decompress.rs
// rust_signature: fn makeMaps_d(s: *mut crate::types::DState)
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_decompress_1/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_EState__seqToUnseq` in module `crate::compat`
//     --> src/src_decompress.rs:20:47
//      |
//      |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//      |
//      |
//      |     ---------------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_DState__seqToUnseq` defined here
//      |
// =================================
fn makeMaps_d(s: *mut crate::types::DState) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_decompress::makeMaps_d(s as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_decompress_1
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_decompress_1/translated_rust.rs
 * ------------------------------------------------------------
fn makeMaps_d(s: *mut crate::types::DState) {
    unsafe {
        let n_in_use_ptr = crate::compat::c2r_field_ptr_EState__nInUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        *n_in_use_ptr = 0;
        let in_use_ptr = crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
        let seq_to_unseq_ptr = crate::compat::c2r_field_ptr_EState__seqToUnseq(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        for i in 0..256 {
            if *in_use_ptr.add(i as usize) != 0 {
                let n = *n_in_use_ptr;
                *seq_to_unseq_ptr.add(n as usize) = i as crate::types::UChar;
                *n_in_use_ptr = n + 1;
            }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_decompress_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: injection_failed
// func_key: src_decompress_2
// c_function: BZ2_decompress
// rust_file: src_decompress.rs
// rust_signature: pub extern "C" fn BZ2_decompress(arg1: *mut crate::types::DState) -> crate::types::Int32
// c_first_line: Int32 BZ2_decompress ( DState* s )
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_decompress_2/translated_rust.rs
// =================================
pub extern "C" fn BZ2_decompress(arg1: *mut crate::types::DState) -> crate::types::Int32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_decompress::BZ2_decompress(arg1 as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_decompress_2
 * reason: injection_failed
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_decompress_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_decompress(arg1: *mut crate::types::DState) -> crate::types::Int32 {
    let s = arg1;
    let mut uc: crate::types::UChar = 0;
    let mut retVal: crate::types::Int32 = 0;
    let mut minLen: crate::types::Int32 = 0;
    let mut maxLen: crate::types::Int32 = 0;
    let strm_ptr = unsafe {
        *(crate::compat::c2r_field_ptr_DState__strm(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::bz_stream)
    };

    let mut i: crate::types::Int32 = 0;
    let mut j: crate::types::Int32 = 0;
    let mut t: crate::types::Int32 = 0;
    let mut alphaSize: crate::types::Int32 = 0;
    let mut nGroups: crate::types::Int32 = 0;
    let mut nSelectors: crate::types::Int32 = 0;
    let mut EOB: crate::types::Int32 = 0;
    let mut groupNo: crate::types::Int32 = 0;
    let mut groupPos: crate::types::Int32 = 0;
    let mut nextSym: crate::types::Int32 = 0;
    let mut nblockMAX: crate::types::Int32 = 0;
    let mut nblock: crate::types::Int32 = 0;
    let mut es: crate::types::Int32 = 0;
    let mut N: crate::types::Int32 = 0;
    let mut curr: crate::types::Int32 = 0;
    let mut zt: crate::types::Int32 = 0;
    let mut zn: crate::types::Int32 = 0;
    let mut zvec: crate::types::Int32 = 0;
    let mut zj: crate::types::Int32 = 0;
    let mut gSel: crate::types::Int32 = 0;
    let mut gMinlen: crate::types::Int32 = 0;
    let mut gLimit: *mut crate::types::Int32 = std::ptr::null_mut();
    let mut gBase: *mut crate::types::Int32 = std::ptr::null_mut();
    let mut gPerm: *mut crate::types::Int32 = std::ptr::null_mut();

    let state_ptr = unsafe {
        crate::compat::c2r_field_ptr_DState__state(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32
    };
    if unsafe { *state_ptr } == 10 {
        unsafe {
            *(crate::compat::c2r_field_ptr_DState__save_i(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_j(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_t(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_alphaSize(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_nGroups(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_nSelectors(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_EOB(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_groupNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_groupPos(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_nextSym(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_nblockMAX(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_es(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_N(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_curr(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_zt(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_zn(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_zvec(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_zj(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_gSel(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_gMinlen(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
            *(crate::compat::c2r_field_ptr_DState__save_gLimit(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) = std::ptr::null_mut();
            *(crate::compat::c2r_field_ptr_DState__save_gBase(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) = std::ptr::null_mut();
            *(crate::compat::c2r_field_ptr_DState__save_gPerm(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) = std::ptr::null_mut();
        }
    }

    i = unsafe { *(crate::compat::c2r_field_ptr_DState__save_i(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    j = unsafe { *(crate::compat::c2r_field_ptr_DState__save_j(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    t = unsafe { *(crate::compat::c2r_field_ptr_DState__save_t(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    alphaSize = unsafe { *(crate::compat::c2r_field_ptr_DState__save_alphaSize(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    nGroups = unsafe { *(crate::compat::c2r_field_ptr_DState__save_nGroups(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    nSelectors = unsafe { *(crate::compat::c2r_field_ptr_DState__save_nSelectors(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    EOB = unsafe { *(crate::compat::c2r_field_ptr_DState__save_EOB(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    groupNo = unsafe { *(crate::compat::c2r_field_ptr_DState__save_groupNo(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    groupPos = unsafe { *(crate::compat::c2r_field_ptr_DState__save_groupPos(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    nextSym = unsafe { *(crate::compat::c2r_field_ptr_DState__save_nextSym(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    nblockMAX = unsafe { *(crate::compat::c2r_field_ptr_DState__save_nblockMAX(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    nblock = unsafe { *(crate::compat::c2r_field_ptr_DState__save_nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    es = unsafe { *(crate::compat::c2r_field_ptr_DState__save_es(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    N = unsafe { *(crate::compat::c2r_field_ptr_DState__save_N(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    curr = unsafe { *(crate::compat::c2r_field_ptr_DState__save_curr(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    zt = unsafe { *(crate::compat::c2r_field_ptr_DState__save_zt(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    zn = unsafe { *(crate::compat::c2r_field_ptr_DState__save_zn(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    zvec = unsafe { *(crate::compat::c2r_field_ptr_DState__save_zvec(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    zj = unsafe { *(crate::compat::c2r_field_ptr_DState__save_zj(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    gSel = unsafe { *(crate::compat::c2r_field_ptr_DState__save_gSel(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    gMinlen = unsafe { *(crate::compat::c2r_field_ptr_DState__save_gMinlen(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) };
    gLimit = unsafe { *(crate::compat::c2r_field_ptr_DState__save_gLimit(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) };
    gBase = unsafe { *(crate::compat::c2r_field_ptr_DState__save_gBase(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) };
    gPerm = unsafe { *(crate::compat::c2r_field_ptr_DState__save_gPerm(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32) };

    retVal = 0;

    let state = unsafe { *state_ptr };
    match state {
        10 => {
            unsafe { *state_ptr = 10; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            if uc != 0x42 {
                retVal = -5;
                goto_save_state_and_return!();
            }
        }
        11 => {
            unsafe { *state_ptr = 11; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            if uc != 0x5a {
                retVal = -5;
                goto_save_state_and_return!();
            }
        }
        12 => {
            unsafe { *state_ptr = 12; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            if uc != 0x68 {
                retVal = -5;
                goto_save_state_and_return!();
            }
        }
        13 => {
            unsafe { *state_ptr = 13; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    let blockSize100k_ptr = crate::compat::c2r_field_ptr_DState__blockSize100k(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                    unsafe { *blockSize100k_ptr = v as crate::types::Int32; }
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            let blockSize100k_ptr = crate::compat::c2r_field_ptr_DState__blockSize100k(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if unsafe { *blockSize100k_ptr } < (0x30 + 1) || unsafe { *blockSize100k_ptr } > (0x30 + 9) {
                retVal = -5;
                goto_save_state_and_return!();
            }
            unsafe { *blockSize100k_ptr -= 0x30; }

            let smallDecompress_ptr = crate::compat::c2r_field_ptr_DState__smallDecompress(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
            if unsafe { *smallDecompress_ptr } != 0 {
                let bzalloc_ptr = crate::compat::c2r_field_ptr_bz_stream__bzalloc(strm_ptr as *mut ::core::ffi::c_void) as *mut Option<extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void>;
                let opaque_ptr = crate::compat::c2r_field_ptr_bz_stream__opaque(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_void;
                let ll16_ptr = crate::compat::c2r_field_ptr_DState__ll16(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UInt16;
                let ll4_ptr = crate::compat::c2r_field_ptr_DState__ll4(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    if let Some(f) = *bzalloc_ptr {
                        *ll16_ptr = f(*opaque_ptr, (*blockSize100k_ptr * 100000 * std::mem::size_of::<crate::types::UInt16>() as crate::types::Int32) as crate::types::Int32, 1) as *mut crate::types::UInt16;
                        *ll4_ptr = f(*opaque_ptr, (((1 + *blockSize100k_ptr * 100000) >> 1) * std::mem::size_of::<crate::types::UChar>() as crate::types::Int32) as crate::types::Int32, 1) as *mut crate::types::UChar;
                    }
                    if (*ll16_ptr).is_null() || (*ll4_ptr).is_null() {
                        retVal = -3;
                        goto_save_state_and_return!();
                    }
                }
            } else {
                let bzalloc_ptr = crate::compat::c2r_field_ptr_bz_stream__bzalloc(strm_ptr as *mut ::core::ffi::c_void) as *mut Option<extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void>;
                let opaque_ptr = crate::compat::c2r_field_ptr_bz_stream__opaque(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_void;
                let tt_ptr = crate::compat::c2r_field_ptr_DState__tt(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::Int32;
                unsafe {
                    if let Some(f) = *bzalloc_ptr {
                        *tt_ptr = f(*opaque_ptr, (*blockSize100k_ptr * 100000 * std::mem::size_of::<crate::types::Int32>() as crate::types::Int32) as crate::types::Int32, 1) as *mut crate::types::Int32;
                    }
                    if (*tt_ptr).is_null() {
                        retVal = -3;
                        goto_save_state_and_return!();
                    }
                }
            }
        }
        14 => {
            unsafe { *state_ptr = 14; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            if uc == 0x17 {
                goto_endhdr_2!();
            }
            if uc != 0x31 {
                retVal = -4;
                goto_save_state_and_return!();
            }
        }
        15 => {
            unsafe { *state_ptr = 15; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_lo32_ptr += 1;
                    if *total_in_lo32_ptr == 0 {
                        let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        *total_in_hi32_ptr += 1;
                    }
                }
            }
            if uc != 0x41 {
                retVal = -4;
                goto_save_state_and_return!();
            }
        }
        16 => {
            unsafe { *state_ptr = 16; }
            loop {
                let bsLive_ptr = crate::compat::c2r_field_ptr_DState__bsLive(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if unsafe { *bsLive_ptr } >= 8 {
                    let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let v = (unsafe { *bsBuff_ptr } >> (unsafe { *bsLive_ptr } - 8)) & ((1 << 8) - 1);
                    unsafe { *bsLive_ptr -= 8; }
                    uc = v as crate::types::UChar;
                    break;
                }
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if unsafe { *avail_in_ptr } == 0 {
                    retVal = 0;
                    goto_save_state_and_return!();
                }
                let bsBuff_ptr = crate::compat::c2r_field_ptr_DState__bsBuff(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                unsafe {
                    *bsBuff_ptr = (*bsBuff_ptr << 8) | (*((*next_in_ptr) as *const crate::types::UChar)) as crate::types::UInt32;
                    *bsLive_ptr += 8;
                    *next_in_ptr = (*next_in_ptr).offset(1);
                    *avail_in_ptr -= 1;
                    let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_decompress_2
 * === C2R_LLM_FAILED_OUTPUT_END === */

