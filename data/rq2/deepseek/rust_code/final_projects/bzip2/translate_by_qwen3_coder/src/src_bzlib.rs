//! Module: src_bzlib
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
/// C: static const char *[16] bzerrorstrings
static mut bzerrorstrings: [*const ::core::ffi::c_char; 16usize] = unsafe { core::mem::MaybeUninit::<[*const ::core::ffi::c_char; 16usize]>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

pub extern "C" fn BZ2_bz__AssertH__fail(errcode: ::core::ffi::c_int) {
    unsafe {
        libc::fprintf(
            crate::compat::stderr as *mut libc::FILE,
            b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\0".as_ptr() as *const _,
            errcode,
            crate::src_bzlib::BZ2_bzlibVersion(),
        );
        if errcode == 1007 {
            libc::fprintf(
                crate::compat::stderr as *mut libc::FILE,
                b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn't.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\0".as_ptr() as *const _,
            );
        }
        libc::exit(3);
    }
}

fn bz_config_ok() -> i32 {
    if std::mem::size_of::<i32>() != 4 {
        return 0;
    }
    if std::mem::size_of::<i16>() != 2 {
        return 0;
    }
    if std::mem::size_of::<u8>() != 1 {
        return 0;
    }
    1
}

fn default_bzalloc(opaque: *mut std::ffi::c_void, items: crate::types::Int32, size: crate::types::Int32) -> *mut std::ffi::c_void {
    let _ = opaque;
    unsafe { libc::malloc((items as usize).wrapping_mul(size as usize)) as *mut std::ffi::c_void }
}

fn default_bzfree(opaque: *mut std::ffi::c_void, addr: *mut std::ffi::c_void) {
    if !addr.is_null() {
        unsafe {
            libc::free(addr);
        }
    }
}

fn prepare_new_block(s: *mut crate::types::EState) {
    unsafe {
        let s_ptr = s as *mut u8;
        let nblock_ptr = s_ptr.offset(0) as *mut crate::types::Int32;
        *nblock_ptr = 0;
        let num_z_ptr = s_ptr.offset(4) as *mut crate::types::Int32;
        *num_z_ptr = 0;
        let state_out_pos_ptr = s_ptr.offset(8) as *mut crate::types::Int32;
        *state_out_pos_ptr = 0;
        let block_crc_ptr = s_ptr.offset(12) as *mut crate::types::UInt32;
        *block_crc_ptr = 0xffffffff;
        let in_use_ptr = s_ptr.offset(16) as *mut crate::types::Bool;
        for i in 0..256 {
            *in_use_ptr.add(i) = 0;
        }
        let block_no_ptr = s_ptr.offset(272) as *mut crate::types::Int32;
        *block_no_ptr += 1;
    }
}

fn init_RL(s: *mut crate::types::EState) {
    unsafe {
        *(crate::compat::c2r_field_ptr_EState__state_in_ch(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32) = 256;
        *(crate::compat::c2r_field_ptr_EState__state_in_len(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32) = 0;
    }
}

fn isempty_RL(s: *mut crate::types::EState) -> crate::types::Bool {
    if s.is_null() {
        return 1;
    }
    unsafe {
        let state_in_ch = *((s as *mut u8).add(0) as *const crate::types::UInt32);
        let state_in_len = *((s as *mut u8).add(4) as *const crate::types::Int32);
        if state_in_ch < 256 && state_in_len > 0 {
            0
        } else {
            1
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_8
// c_function: BZ2_bzCompressInit
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzCompressInit(strm: *mut crate::types::bz_stream, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> ::core::ffi::c_int
// c_first_line: int BZ2_bzCompressInit
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_8/translated_rust.rs
// last_error_truncated:
//   error[E0605]: non-primitive cast: `fn(*mut libc::c_void, i32, i32) -> *mut libc::c_void {default_bzalloc}` as `unsafe extern "C" fn(*mut libc::c_void, i32, i32) -> *mut libc::c_void`
//      --> src/src_bzlib.rs:130:33
//       |
//       |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
//   error[E0605]: non-primitive cast: `fn(*mut libc::c_void, *mut libc::c_void) {default_bzfree}` as `unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void)`
//      --> src/src_bzlib.rs:133:32
//       |
//       |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
// =================================
pub extern "C" fn BZ2_bzCompressInit(strm: *mut crate::types::bz_stream, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_8/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzCompressInit(strm: *mut crate::types::bz_stream, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if crate::src_bzlib::bz_config_ok() == 0 {
        return -9;
    }
    if strm.is_null()
        || blockSize100k < 1
        || blockSize100k > 9
        || workFactor < 0
        || workFactor > 250
    {
        return -2;
    }
    let mut workFactor = workFactor;
    if workFactor == 0 {
        workFactor = 30;
    }
    unsafe {
        let bzalloc_ptr = (strm as *mut u8).add(0) as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void>;
        let bzfree_ptr = (strm as *mut u8).add(8) as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>;
        let opaque_ptr = (strm as *mut u8).add(16) as *mut *mut ::core::ffi::c_void;
        if (*bzalloc_ptr).is_none() {
            *bzalloc_ptr = Some(crate::src_bzlib::default_bzalloc as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void);
        }
        if (*bzfree_ptr).is_none() {
            *bzfree_ptr = Some(crate::src_bzlib::default_bzfree as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void));
        }
        let bzalloc_fn = (*bzalloc_ptr).unwrap();
        let bzfree_fn = (*bzfree_ptr).unwrap();
        let opaque = *opaque_ptr;
        let s = bzalloc_fn(
            opaque,
            std::mem::size_of::<crate::types::EState>() as crate::types::Int32,
            1,
        ) as *mut crate::types::EState;
        if s.is_null() {
            return -3;
        }
        let s_strm_ptr = (s as *mut u8).add(0) as *mut *mut crate::types::bz_stream;
        *s_strm_ptr = strm;
        let arr1_ptr = (s as *mut u8).add(8) as *mut *mut ::core::ffi::c_void;
        let arr2_ptr = (s as *mut u8).add(16) as *mut *mut ::core::ffi::c_void;
        let ftab_ptr = (s as *mut u8).add(24) as *mut *mut ::core::ffi::c_void;
        *arr1_ptr = std::ptr::null_mut();
        *arr2_ptr = std::ptr::null_mut();
        *ftab_ptr = std::ptr::null_mut();
        let n = 100000 * blockSize100k as crate::types::Int32;
        let arr1 = bzalloc_fn(
            opaque,
            n,
            std::mem::size_of::<crate::types::UInt32>() as crate::types::Int32,
        );
        let arr2 = bzalloc_fn(
            opaque,
            n + (2 + 12 + 18 + 2),
            std::mem::size_of::<crate::types::UInt32>() as crate::types::Int32,
        );
        let ftab = bzalloc_fn(
            opaque,
            65537,
            std::mem::size_of::<crate::types::UInt32>() as crate::types::Int32,
        );
        *arr1_ptr = arr1;
        *arr2_ptr = arr2;
        *ftab_ptr = ftab;
        if arr1.is_null() || arr2.is_null() || ftab.is_null() {
            if !arr1.is_null() {
                bzfree_fn(opaque, arr1);
            }
            if !arr2.is_null() {
                bzfree_fn(opaque, arr2);
            }
            if !ftab.is_null() {
                bzfree_fn(opaque, ftab);
            }
            if !s.is_null() {
                bzfree_fn(opaque, s as *mut ::core::ffi::c_void);
            }
            return -3;
        }
        let block_no_ptr = (s as *mut u8).add(32) as *mut crate::types::Int32;
        let state_ptr = (s as *mut u8).add(36) as *mut crate::types::Int32;
        let mode_ptr = (s as *mut u8).add(40) as *mut crate::types::Int32;
        let combined_crc_ptr = (s as *mut u8).add(44) as *mut crate::types::UInt32;
        let block_size_100k_ptr = (s as *mut u8).add(48) as *mut crate::types::Int32;
        let nblock_max_ptr = (s as *mut u8).add(52) as *mut crate::types::Int32;
        let verbosity_ptr = (s as *mut u8).add(56) as *mut crate::types::Int32;
        let work_factor_ptr = (s as *mut u8).add(60) as *mut crate::types::Int32;
        *block_no_ptr = 0;
        *state_ptr = 2;
        *mode_ptr = 2;
        *combined_crc_ptr = 0;
        *block_size_100k_ptr = blockSize100k;
        *nblock_max_ptr = 100000 * blockSize100k - 19;
        *verbosity_ptr = verbosity;
        *work_factor_ptr = workFactor;
        let block_ptr = (s as *mut u8).add(64) as *mut *mut crate::types::UChar;
        let mtfv_ptr = (s as *mut u8).add(72) as *mut *mut crate::types::UInt16;
        let zbits_ptr = (s as *mut u8).add(80) as *mut *mut ::core::ffi::c_void;
        let ptr_ptr = (s as *mut u8).add(88) as *mut *mut crate::types::UInt32;
        *block_ptr = arr2 as *mut crate::types::UChar;
        *mtfv_ptr = arr1 as *mut crate::types::UInt16;
        *zbits_ptr = std::ptr::null_mut();
        *ptr_ptr = arr1 as *mut crate::types::UInt32;
        let strm_state_ptr = (strm as *mut u8).add(24) as *mut *mut crate::types::EState;
        let total_in_lo32_ptr = (strm as *mut u8).add(32) as *mut crate::types::UInt32;
        let total_in_hi32_ptr = (strm as *mut u8).add(36) as *mut crate::types::UInt32;
        let total_out_lo32_ptr = (strm as *mut u8).add(40) as *mut crate::types::UInt32;
        let total_out_hi32_ptr = (strm as *mut u8).add(44) as *mut crate::types::UInt32;
        *strm_state_ptr = s;
        *total_in_lo32_ptr = 0;
        *total_in_hi32_ptr = 0;
        *total_out_lo32_ptr = 0;
        *total_out_hi32_ptr = 0;
        crate::src_bzlib::init_RL(s);
        crate::src_bzlib::prepare_new_block(s);
    }
    0
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_9
// c_function: add_pair_to_block
// rust_file: src_bzlib.rs
// rust_signature: fn add_pair_to_block(s: *mut crate::types::EState)
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_9/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `c2r_field_ptr_EState__nblock` in module `crate::compat`
//      --> src/src_bzlib.rs:115:41
//       |
//       |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     ----------------------------------------------------------------------------------------------- similarly named function `c2r_field_ptr_EState__block` defined here
//       |
// =================================
fn add_pair_to_block(s: *mut crate::types::EState) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_9/translated_rust.rs
 * ------------------------------------------------------------
fn add_pair_to_block(s: *mut crate::types::EState) {
    unsafe {
        let state_in_ch_ptr = crate::compat::c2r_field_ptr_EState__state_in_ch(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        let ch = *state_in_ch_ptr as crate::types::UChar;
        let state_in_len_ptr = crate::compat::c2r_field_ptr_EState__state_in_len(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let state_in_len = *state_in_len_ptr;
        let blockcrc_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let mut blockcrc = *blockcrc_ptr;
        for _i in 0..state_in_len {
            blockcrc = (blockcrc << 8) ^ BZ2_crc32Table[((blockcrc >> 24) ^ (ch as crate::types::UInt32)) as usize];
        }
        *blockcrc_ptr = blockcrc;
        let inuse_ptr = crate::compat::c2r_field_ptr_EState__inUse(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
        let inuse = std::slice::from_raw_parts_mut(inuse_ptr, 256);
        inuse[ch as usize] = 1;
        let block_ptr = crate::compat::c2r_field_ptr_EState__block(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let mut nblock = *nblock_ptr;
        match state_in_len {
            1 => {
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
            }
            2 => {
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
            }
            3 => {
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
            }
            _ => {
                inuse[(state_in_len - 4) as usize] = 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = ch;
                nblock += 1;
                *block_ptr.add(nblock as usize) = (state_in_len - 4) as crate::types::UChar;
                nblock += 1;
            }
        }
        *nblock_ptr = nblock;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn flush_RL(s: *mut crate::types::EState) {
    unsafe {
        let state_in_ch_ptr = crate::compat::c2r_field_ptr_EState__state_in_ch(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *state_in_ch_ptr < 256 {
            crate::src_bzlib::add_pair_to_block(s);
        }
        crate::src_bzlib::init_RL(s);
    }
}

fn ADD_CHAR_TO_BLOCK(zs: *mut crate::types::EState, zchh0: crate::types::UInt32) {
    let zchh = zchh0;
    let zs_void = zs as *mut ::core::ffi::c_void;
    unsafe {
        let state_in_ch_ptr = crate::compat::c2r_field_ptr_EState__state_in_ch(zs_void) as *mut crate::types::UInt32;
        let state_in_len_ptr = crate::compat::c2r_field_ptr_EState__state_in_len(zs_void) as *mut crate::types::UInt32;
        let state_in_ch = *state_in_ch_ptr;
        let state_in_len = *state_in_len_ptr;
        if zchh != state_in_ch && state_in_len == 1 {
            let ch = state_in_ch as crate::types::UChar;
            let block_crc_ptr = crate::compat::c2r_field_ptr_EState__blockCRC(zs_void) as *mut crate::types::UInt32;
            let block_crc = *block_crc_ptr;
            let new_crc = (block_crc << 8) ^ crate::globals::BZ2_crc32Table[((block_crc >> 24) ^ (ch as crate::types::UInt32)) as usize];
            *block_crc_ptr = new_crc;
            let in_use_ptr = crate::compat::c2r_field_ptr_EState__inUse(zs_void) as *mut crate::types::Bool;
            let in_use_idx = state_in_ch as usize;
            *in_use_ptr.add(in_use_idx) = 1;
            let block_ptr = crate::compat::c2r_field_ptr_EState__block(zs_void) as *mut crate::types::UChar;
            let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(zs_void) as *mut crate::types::Int32;
            let nblock = *nblock_ptr;
            *block_ptr.add(nblock as usize) = ch;
            *nblock_ptr = nblock + 1;
            *state_in_ch_ptr = zchh;
        } else if zchh != state_in_ch || state_in_len == 255 {
            if state_in_ch < 256 {
                crate::src_bzlib::add_pair_to_block(zs);
            }
            *state_in_ch_ptr = zchh;
            *state_in_len_ptr = 1;
        } else {
            *state_in_len_ptr = state_in_len + 1;
        }
    }
}

fn copy_input_until_stop(s: *mut crate::types::EState) -> crate::types::Bool {
    let mut progress_in: crate::types::Bool = 0;
    unsafe {
        let mode_ptr = crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let mode = *mode_ptr;
        if mode == 2 {
            loop {
                let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let nblock = *nblock_ptr;
                let nblockMAX_ptr = crate::compat::c2r_field_ptr_EState__nblockMAX(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let nblockMAX = *nblockMAX_ptr;
                if nblock >= nblockMAX {
                    break;
                }
                let strm_ptr = crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void) as *mut crate::types::bz_stream;
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if *avail_in_ptr == 0 {
                    break;
                }
                progress_in = 1;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                let ch = **next_in_ptr as crate::types::UInt32;
                crate::src_bzlib::ADD_CHAR_TO_BLOCK(s, ch);
                *next_in_ptr = (*next_in_ptr).offset(1);
                *avail_in_ptr -= 1;
                let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                *total_in_lo32_ptr = (*total_in_lo32_ptr).wrapping_add(1);
                if *total_in_lo32_ptr == 0 {
                    let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_hi32_ptr = (*total_in_hi32_ptr).wrapping_add(1);
                }
            }
        } else {
            loop {
                let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let nblock = *nblock_ptr;
                let nblockMAX_ptr = crate::compat::c2r_field_ptr_EState__nblockMAX(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let nblockMAX = *nblockMAX_ptr;
                if nblock >= nblockMAX {
                    break;
                }
                let strm_ptr = crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void) as *mut crate::types::bz_stream;
                let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if *avail_in_ptr == 0 {
                    break;
                }
                let avail_in_expect_ptr = crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if *avail_in_expect_ptr == 0 {
                    break;
                }
                progress_in = 1;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                let ch = **next_in_ptr as crate::types::UInt32;
                crate::src_bzlib::ADD_CHAR_TO_BLOCK(s, ch);
                *next_in_ptr = (*next_in_ptr).offset(1);
                *avail_in_ptr -= 1;
                let total_in_lo32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_lo32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                *total_in_lo32_ptr = (*total_in_lo32_ptr).wrapping_add(1);
                if *total_in_lo32_ptr == 0 {
                    let total_in_hi32_ptr = crate::compat::c2r_field_ptr_bz_stream__total_in_hi32(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *total_in_hi32_ptr = (*total_in_hi32_ptr).wrapping_add(1);
                }
                *avail_in_expect_ptr -= 1;
            }
        }
    }
    progress_in
}

fn copy_output_until_stop(s: *mut crate::types::EState) -> crate::types::Bool {
    let mut progress_out = 0u8;
    unsafe {
        let s_ptr = s as *mut u8;
        let state_out_pos = (s_ptr.offset(0x1c) as *mut i32).read();
        let numZ = (s_ptr.offset(0x20) as *mut i32).read();
        let zbits = s_ptr.offset(0x28) as *mut u8;
        let strm = s_ptr.offset(0x30) as *mut u8;
        let mut state_out_pos_local = state_out_pos;
        loop {
            let avail_out = (strm.offset(0x18) as *mut i32).read();
            if avail_out == 0 {
                break;
            }
            if state_out_pos_local >= numZ {
                break;
            }
            progress_out = 1u8;
            let next_out = (strm.offset(0x10) as *mut *mut u8).read();
            let byte = *zbits.offset(state_out_pos_local as isize);
            next_out.write(byte);
            state_out_pos_local += 1;
            (strm.offset(0x18) as *mut i32).write(avail_out - 1);
            (strm.offset(0x10) as *mut *mut u8).write(next_out.offset(1));
            let total_out_lo32_ptr = strm.offset(0x20) as *mut u32;
            let total_out_lo32 = total_out_lo32_ptr.read().wrapping_add(1);
            total_out_lo32_ptr.write(total_out_lo32);
            if total_out_lo32 == 0 {
                let total_out_hi32_ptr = strm.offset(0x24) as *mut u32;
                total_out_hi32_ptr.write(total_out_hi32_ptr.read() + 1);
            }
        }
        (s_ptr.offset(0x1c) as *mut i32).write(state_out_pos_local);
    }
    progress_out
}

fn handle_compress(strm: *mut crate::types::bz_stream) -> crate::types::Bool {
    let mut progress_in: crate::types::Bool = 0;
    let mut progress_out: crate::types::Bool = 0;
    let s_ptr = unsafe {
        crate::compat::c2r_field_ptr_bz_stream__state(strm as *mut ::core::ffi::c_void)
    };
    let s = s_ptr as *mut crate::types::EState;
    loop {
        let state_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__state(s as *mut ::core::ffi::c_void)
        };
        let state = unsafe { *(state_ptr as *const i32) };
        if state == 1 {
            progress_out |= crate::src_bzlib::copy_output_until_stop(s);
            let state_out_pos_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__state_out_pos(s as *mut ::core::ffi::c_void)
            };
            let state_out_pos = unsafe { *(state_out_pos_ptr as *const i32) };
            let numz_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
            };
            let numz = unsafe { *(numz_ptr as *const i32) };
            if state_out_pos < numz {
                break;
            }
            let mode_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
            };
            let mode = unsafe { *(mode_ptr as *const i32) };
            let avail_in_expect_ptr = unsafe {
                crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
            };
            let avail_in_expect = unsafe { *(avail_in_expect_ptr as *const i32) };
            if mode == 4 && avail_in_expect == 0 && crate::src_bzlib::isempty_RL(s) != 0 {
                break;
            }
            crate::src_bzlib::prepare_new_block(s);
            unsafe {
                *(crate::compat::c2r_field_ptr_EState__state(s as *mut ::core::ffi::c_void) as *mut i32) = 2;
            }
            if mode == 3 && avail_in_expect == 0 && crate::src_bzlib::isempty_RL(s) != 0 {
                break;
            }
        }
        let state_ptr2 = unsafe {
            crate::compat::c2r_field_ptr_EState__state(s as *mut ::core::ffi::c_void)
        };
        let state2 = unsafe { *(state_ptr2 as *const i32) };
        if state2 == 2 {
            progress_in |= crate::src_bzlib::copy_input_until_stop(s);
            let mode_ptr2 = unsafe {
                crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
            };
            let mode2 = unsafe { *(mode_ptr2 as *const i32) };
            let avail_in_expect_ptr2 = unsafe {
                crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
            };
            let avail_in_expect2 = unsafe { *(avail_in_expect_ptr2 as *const i32) };
            if mode2 != 2 && avail_in_expect2 == 0 {
                crate::src_bzlib::flush_RL(s);
                crate::src_compress::BZ2_compressBlock(s, if mode2 == 4 { 1 } else { 0 });
                unsafe {
                    *(crate::compat::c2r_field_ptr_EState__state(s as *mut ::core::ffi::c_void) as *mut i32) = 1;
                }
            } else {
                let nblock_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void)
                };
                let nblock = unsafe { *(nblock_ptr as *const i32) };
                let nblockmax_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__nblockMAX(s as *mut ::core::ffi::c_void)
                };
                let nblockmax = unsafe { *(nblockmax_ptr as *const i32) };
                if nblock >= nblockmax {
                    crate::src_compress::BZ2_compressBlock(s, 0);
                    unsafe {
                        *(crate::compat::c2r_field_ptr_EState__state(s as *mut ::core::ffi::c_void) as *mut i32) = 1;
                    }
                } else {
                    let strm_ptr = unsafe {
                        crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void)
                    };
                    let strm2 = strm_ptr as *mut crate::types::bz_stream;
                    let avail_in = unsafe { (*strm2).avail_in };
                    if avail_in == 0 {
                        break;
                    }
                }
            }
        }
    }
    if progress_in != 0 || progress_out != 0 { 1 } else { 0 }
}

pub extern "C" fn BZ2_bzCompress(strm: *mut crate::types::bz_stream, action: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut progress: crate::types::Bool;
    let s: *mut crate::types::EState;
    if strm.is_null() {
        return -2;
    }
    let s_ptr = unsafe {
        crate::compat::c2r_field_ptr_bz_stream__state(strm as *mut ::core::ffi::c_void)
    };
    s = unsafe { *(s_ptr as *mut *mut crate::types::EState) };
    if s.is_null() {
        return -2;
    }
    let s_strm_ptr = unsafe {
        crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void)
    };
    let s_strm = unsafe { *(s_strm_ptr as *mut *mut crate::types::bz_stream) };
    if s_strm != strm {
        return -2;
    }

    'preswitch: loop {
        let mode_ptr = unsafe {
            crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
        };
        let mode = unsafe { *(mode_ptr as *mut ::core::ffi::c_int) };
        match mode {
            1 => {
                return -1;
            }
            2 => {
                if action == 0 {
                    progress = crate::src_bzlib::handle_compress(strm);
                    return if progress != 0 { 1 } else { -2 };
                } else if action == 1 {
                    let avail_in_ptr = unsafe {
                        crate::compat::c2r_field_ptr_bz_stream__avail_in(strm as *mut ::core::ffi::c_void)
                    };
                    let avail_in = unsafe { *(avail_in_ptr as *mut u32) };
                    let avail_in_expect_ptr = unsafe {
                        crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
                    };
                    unsafe {
                        *(avail_in_expect_ptr as *mut u32) = avail_in;
                    }
                    let mode_ptr = unsafe {
                        crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
                    };
                    unsafe {
                        *(mode_ptr as *mut ::core::ffi::c_int) = 3;
                    }
                    continue 'preswitch;
                } else if action == 2 {
                    let avail_in_ptr = unsafe {
                        crate::compat::c2r_field_ptr_bz_stream__avail_in(strm as *mut ::core::ffi::c_void)
                    };
                    let avail_in = unsafe { *(avail_in_ptr as *mut u32) };
                    let avail_in_expect_ptr = unsafe {
                        crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
                    };
                    unsafe {
                        *(avail_in_expect_ptr as *mut u32) = avail_in;
                    }
                    let mode_ptr = unsafe {
                        crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
                    };
                    unsafe {
                        *(mode_ptr as *mut ::core::ffi::c_int) = 4;
                    }
                    continue 'preswitch;
                } else {
                    return -2;
                }
            }
            3 => {
                if action != 1 {
                    return -1;
                }
                let avail_in_expect_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
                };
                let avail_in_expect = unsafe { *(avail_in_expect_ptr as *mut u32) };
                let s_strm_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void)
                };
                let s_strm = unsafe { *(s_strm_ptr as *mut *mut crate::types::bz_stream) };
                let avail_in_ptr = unsafe {
                    crate::compat::c2r_field_ptr_bz_stream__avail_in(s_strm as *mut ::core::ffi::c_void)
                };
                let avail_in = unsafe { *(avail_in_ptr as *mut u32) };
                if avail_in_expect != avail_in {
                    return -1;
                }
                progress = crate::src_bzlib::handle_compress(strm);
                let numz_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
                };
                let numz = unsafe { *(numz_ptr as *mut u32) };
                let state_out_pos_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__state_out_pos(s as *mut ::core::ffi::c_void)
                };
                let state_out_pos = unsafe { *(state_out_pos_ptr as *mut u32) };
                if avail_in_expect > 0 || crate::src_bzlib::isempty_RL(s) == 0 || state_out_pos < numz {
                    return 2;
                }
                let mode_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
                };
                unsafe {
                    *(mode_ptr as *mut ::core::ffi::c_int) = 2;
                }
                return 1;
            }
            4 => {
                if action != 2 {
                    return -1;
                }
                let avail_in_expect_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__avail_in_expect(s as *mut ::core::ffi::c_void)
                };
                let avail_in_expect = unsafe { *(avail_in_expect_ptr as *mut u32) };
                let s_strm_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__strm(s as *mut ::core::ffi::c_void)
                };
                let s_strm = unsafe { *(s_strm_ptr as *mut *mut crate::types::bz_stream) };
                let avail_in_ptr = unsafe {
                    crate::compat::c2r_field_ptr_bz_stream__avail_in(s_strm as *mut ::core::ffi::c_void)
                };
                let avail_in = unsafe { *(avail_in_ptr as *mut u32) };
                if avail_in_expect != avail_in {
                    return -1;
                }
                progress = crate::src_bzlib::handle_compress(strm);
                if progress == 0 {
                    return -1;
                }
                let numz_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__numZ(s as *mut ::core::ffi::c_void)
                };
                let numz = unsafe { *(numz_ptr as *mut u32) };
                let state_out_pos_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__state_out_pos(s as *mut ::core::ffi::c_void)
                };
                let state_out_pos = unsafe { *(state_out_pos_ptr as *mut u32) };
                if avail_in_expect > 0 || crate::src_bzlib::isempty_RL(s) == 0 || state_out_pos < numz {
                    return 3;
                }
                let mode_ptr = unsafe {
                    crate::compat::c2r_field_ptr_EState__mode(s as *mut ::core::ffi::c_void)
                };
                unsafe {
                    *(mode_ptr as *mut ::core::ffi::c_int) = 1;
                }
                return 4;
            }
            _ => {
                return 0;
            }
        }
    }
}

pub extern "C" fn BZ2_bzCompressEnd(strm: *mut crate::types::bz_stream) -> ::core::ffi::c_int {
    if strm.is_null() {
        return -2;
    }
    let s = unsafe { *((strm as *mut u8).offset(0) as *mut *mut crate::types::EState) };
    if s.is_null() {
        return -2;
    }
    let s_strm = unsafe { *((s as *mut u8).offset(0) as *mut *mut crate::types::bz_stream) };
    if s_strm != strm {
        return -2;
    }
    let arr1 = unsafe { *((s as *mut u8).offset(8) as *mut *mut ::core::ffi::c_void) };
    let arr2 = unsafe { *((s as *mut u8).offset(16) as *mut *mut ::core::ffi::c_void) };
    let ftab = unsafe { *((s as *mut u8).offset(24) as *mut *mut ::core::ffi::c_void) };
    let bzfree = unsafe { *((strm as *mut u8).offset(8) as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>) };
    let opaque = unsafe { *((strm as *mut u8).offset(16) as *mut *mut ::core::ffi::c_void) };
    if !arr1.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, arr1) };
        }
    }
    if !arr2.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, arr2) };
        }
    }
    if !ftab.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, ftab) };
        }
    }
    if let Some(f) = bzfree {
        unsafe { f(opaque, s as *mut ::core::ffi::c_void) };
    }
    unsafe {
        *((strm as *mut u8).offset(0) as *mut *mut crate::types::EState) = std::ptr::null_mut();
    }
    0
}

pub extern "C" fn BZ2_bzDecompressInit(strm: *mut crate::types::bz_stream, verbosity: ::core::ffi::c_int, small: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if crate::src_bzlib::bz_config_ok() == 0 {
        return -9;
    }
    if strm.is_null() {
        return -2;
    }
    if small != 0 && small != 1 {
        return -2;
    }
    if verbosity < 0 || verbosity > 4 {
        return -2;
    }
    unsafe {
        let bzalloc_ptr = strm as *mut u8;
        let bzfree_ptr = bzalloc_ptr;
        let opaque_ptr = bzalloc_ptr;
        let mut bzalloc_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void> = None;
        let mut bzfree_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)> = None;
        bzalloc_fn = *(bzalloc_ptr as *const Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void>);
        bzfree_fn = *(bzfree_ptr as *const Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>);
        if bzalloc_fn.is_none() {
            let default_bzalloc_c: unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void = std::mem::transmute(crate::src_bzlib::default_bzalloc as *const ());
            *(bzalloc_ptr as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::types::Int32, crate::types::Int32) -> *mut ::core::ffi::c_void>) = Some(default_bzalloc_c);
            bzalloc_fn = Some(default_bzalloc_c);
        }
        if bzfree_fn.is_none() {
            let default_bzfree_c: unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) = std::mem::transmute(crate::src_bzlib::default_bzfree as *const ());
            *(bzfree_ptr as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>) = Some(default_bzfree_c);
            bzfree_fn = Some(default_bzfree_c);
        }
        let opaque_val = *(opaque_ptr as *const *mut ::core::ffi::c_void);
        let s = if let Some(f) = bzalloc_fn {
            f(opaque_val, 1, std::mem::size_of::<crate::types::DState>() as crate::types::Int32)
        } else {
            std::ptr::null_mut()
        };
        if s.is_null() {
            return -3;
        }
        let s = s as *mut crate::types::DState;
        let strm_field_ptr = s as *mut u8;
        *(strm_field_ptr as *mut *mut crate::types::bz_stream) = strm;
        let state_field_ptr = strm as *mut u8;
        *(state_field_ptr as *mut *mut crate::types::DState) = s;
        let dstate_state_ptr = s as *mut u8;
        *(dstate_state_ptr as *mut crate::types::Int32) = 10;
        let bsLive_ptr = s as *mut u8;
        *(bsLive_ptr as *mut crate::types::Int32) = 0;
        let bsBuff_ptr = s as *mut u8;
        *(bsBuff_ptr as *mut crate::types::UInt32) = 0;
        let calculatedCombinedCRC_ptr = s as *mut u8;
        *(calculatedCombinedCRC_ptr as *mut crate::types::UInt32) = 0;
        let total_in_lo32_ptr = strm as *mut u8;
        *(total_in_lo32_ptr as *mut crate::types::UInt32) = 0;
        let total_in_hi32_ptr = strm as *mut u8;
        *(total_in_hi32_ptr as *mut crate::types::UInt32) = 0;
        let total_out_lo32_ptr = strm as *mut u8;
        *(total_out_lo32_ptr as *mut crate::types::UInt32) = 0;
        let total_out_hi32_ptr = strm as *mut u8;
        *(total_out_hi32_ptr as *mut crate::types::UInt32) = 0;
        let smallDecompress_ptr = s as *mut u8;
        *(smallDecompress_ptr as *mut crate::types::Bool) = small as crate::types::Bool;
        let ll4_ptr = s as *mut u8;
        *(ll4_ptr as *mut *mut crate::types::UChar) = std::ptr::null_mut();
        let ll16_ptr = s as *mut u8;
        *(ll16_ptr as *mut *mut crate::types::UInt16) = std::ptr::null_mut();
        let tt_ptr = s as *mut u8;
        *(tt_ptr as *mut *mut crate::types::UInt32) = std::ptr::null_mut();
        let currBlockNo_ptr = s as *mut u8;
        *(currBlockNo_ptr as *mut crate::types::Int32) = 0;
        let verbosity_ptr = s as *mut u8;
        *(verbosity_ptr as *mut crate::types::Int32) = verbosity;
    }
    0
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_18
// c_function: unRLE_obuf_to_output_FAST
// rust_file: src_bzlib.rs
// rust_signature: fn unRLE_obuf_to_output_FAST(s: *mut crate::types::DState) -> crate::types::Bool
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_18/translated_rust.rs
// last_error_truncated:
//   error[E0609]: no field `blockRandomised` on type `DState`
//      --> src/src_bzlib.rs:289:17
//       |
//       |                 ^^^^^^^^^^^^^^^ unknown field
//   error[E0609]: no field `strm` on type `DState`
//      --> src/src_bzlib.rs:292:31
//       |
//       |                               ^^^^ unknown field
// =================================
fn unRLE_obuf_to_output_FAST(s: *mut crate::types::DState) -> crate::types::Bool {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_18
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_18/translated_rust.rs
 * ------------------------------------------------------------
fn unRLE_obuf_to_output_FAST(s: *mut crate::types::DState) -> crate::types::Bool {
    unsafe {
        let mut k1: crate::types::UChar = 0;
        if (*s).blockRandomised != 0 {
            loop {
                loop {
                    if (*(*s).strm).avail_out == 0 {
                        return 0;
                    }
                    if (*s).state_out_len == 0 {
                        break;
                    }
                    *((*(*s).strm).next_out as *mut crate::types::UChar) = (*s).state_out_ch;
                    (*s).calculatedBlockCRC = ((*s).calculatedBlockCRC << 8)
                        ^ crate::globals::BZ2_crc32Table[((((*s).calculatedBlockCRC >> 24) as crate::types::UChar
                            ^ (*s).state_out_ch) as usize)];
                    (*s).state_out_len -= 1;
                    (*(*s).strm).next_out = (*(*s).strm).next_out.add(1);
                    (*(*s).strm).avail_out -= 1;
                    (*(*s).strm).total_out_lo32 += 1;
                    if (*(*s).strm).total_out_lo32 == 0 {
                        (*(*s).strm).total_out_hi32 += 1;
                    }
                }
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    return 0;
                }
                if (*s).nblock_used > (*s).save_nblock + 1 {
                    return 1;
                }
                (*s).state_out_len = 1;
                (*s).state_out_ch = (*s).k0;
                if (*s).tPos >= 100000 * (*s).blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                (*s).tPos = (*s).tt[(*s).tPos as usize];
                k1 = ((*s).tPos & 0xff) as crate::types::UChar;
                (*s).tPos >>= 8;
                if (*s).rNToGo == 0 {
                    (*s).rNToGo = crate::globals::BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 {
                        (*s).rTPos = 0;
                    }
                }
                (*s).rNToGo -= 1;
                k1 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 != (*s).k0 {
                    (*s).k0 = k1;
                    continue;
                }
                (*s).state_out_len = 2;
                if (*s).tPos >= 100000 * (*s).blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                (*s).tPos = (*s).tt[(*s).tPos as usize];
                k1 = ((*s).tPos & 0xff) as crate::types::UChar;
                (*s).tPos >>= 8;
                if (*s).rNToGo == 0 {
                    (*s).rNToGo = crate::globals::BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 {
                        (*s).rTPos = 0;
                    }
                }
                (*s).rNToGo -= 1;
                k1 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 != (*s).k0 {
                    (*s).k0 = k1;
                    continue;
                }
                (*s).state_out_len = 3;
                if (*s).tPos >= 100000 * (*s).blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                (*s).tPos = (*s).tt[(*s).tPos as usize];
                k1 = ((*s).tPos & 0xff) as crate::types::UChar;
                (*s).tPos >>= 8;
                if (*s).rNToGo == 0 {
                    (*s).rNToGo = crate::globals::BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 {
                        (*s).rTPos = 0;
                    }
                }
                (*s).rNToGo -= 1;
                k1 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 {
                    continue;
                }
                if k1 != (*s).k0 {
                    (*s).k0 = k1;
                    continue;
                }
                if (*s).tPos >= 100000 * (*s).blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                (*s).tPos = (*s).tt[(*s).tPos as usize];
                k1 = ((*s).tPos & 0xff) as crate::types::UChar;
                (*s).tPos >>= 8;
                if (*s).rNToGo == 0 {
                    (*s).rNToGo = crate::globals::BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 {
                        (*s).rTPos = 0;
                    }
                }
                (*s).rNToGo -= 1;
                k1 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                (*s).nblock_used += 1;
                (*s).state_out_len = (k1 as crate::types::Int32) + 4;
                if (*s).tPos >= 100000 * (*s).blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                (*s).tPos = (*s).tt[(*s).tPos as usize];
                (*s).k0 = ((*s).tPos & 0xff) as crate::types::UChar;
                (*s).tPos >>= 8;
                if (*s).rNToGo == 0 {
                    (*s).rNToGo = crate::globals::BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 {
                        (*s).rTPos = 0;
                    }
                }
                (*s).rNToGo -= 1;
                (*s).k0 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
                (*s).nblock_used += 1;
            }
        } else {
            let mut c_calculatedBlockCRC = (*s).calculatedBlockCRC;
            let mut c_state_out_ch = (*s).state_out_ch;
            let mut c_state_out_len = (*s).state_out_len;
            let mut c_nblock_used = (*s).nblock_used;
            let mut c_k0 = (*s).k0;
            let c_tt = (*s).tt;
            let mut c_tPos = (*s).tPos;
            let mut cs_next_out = (*(*s).strm).next_out;
            let mut cs_avail_out = (*(*s).strm).avail_out;
            let ro_blockSize100k = (*s).blockSize100k;
            let avail_out_INIT = cs_avail_out;
            let s_save_nblockPP = (*s).save_nblock + 1;
            let mut total_out_lo32_old: crate::types::Int32;
            'outer: loop {
                if c_state_out_len > 0 {
                    loop {
                        if cs_avail_out == 0 {
                            break 'outer;
                        }
                        if c_state_out_len == 1 {
                            break;
                        }
                        *(cs_next_out as *mut crate::types::UChar) = c_state_out_ch;
                        c_calculatedBlockCRC = (c_calculatedBlockCRC << 8)
                            ^ crate::globals::BZ2_crc32Table[(((c_calculatedBlockCRC >> 24) as crate::types::UChar
                                ^ c_state_out_ch) as usize)];
                        c_state_out_len -= 1;
                        cs_next_out = cs_next_out.add(1);
                        cs_avail_out -= 1;
                    }
                    if cs_avail_out == 0 {
                        c_state_out_len = 1;
                        break 'outer;
                    }
                    *(cs_next_out as *mut crate::types::UChar) = c_state_out_ch;
                    c_calculatedBlockCRC = (c_calculatedBlockCRC << 8)
                        ^ crate::globals::BZ2_crc32Table[(((c_calculatedBlockCRC >> 24) as crate::types::UChar
                            ^ c_state_out_ch) as usize)];
                    cs_next_out = cs_next_out.add(1);
                    cs_avail_out -= 1;
                }
                if c_nblock_used > s_save_nblockPP {
                    return 1;
                }
                if c_nblock_used == s_save_nblockPP {
                    c_state_out_len = 0;
                    break 'outer;
                }
                c_state_out_ch = c_k0;
                if c_tPos >= 100000 * ro_blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                c_tPos = c_tt[c_tPos as usize];
                k1 = (c_tPos & 0xff) as crate::types::UChar;
                c_tPos >>= 8;
                c_nblock_used += 1;
                if k1 != c_k0 {
                    c_k0 = k1;
                    continue;
                }
                if c_nblock_used == s_save_nblockPP {
                    continue;
                }
                c_state_out_len = 2;
                if c_tPos >= 100000 * ro_blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                c_tPos = c_tt[c_tPos as usize];
                k1 = (c_tPos & 0xff) as crate::types::UChar;
                c_tPos >>= 8;
                c_nblock_used += 1;
                if c_nblock_used == s_save_nblockPP {
                    continue;
                }
                if k1 != c_k0 {
                    c_k0 = k1;
                    continue;
                }
                c_state_out_len = 3;
                if c_tPos >= 100000 * ro_blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                c_tPos = c_tt[c_tPos as usize];
                k1 = (c_tPos & 0xff) as crate::types::UChar;
                c_tPos >>= 8;
                c_nblock_used += 1;
                if c_nblock_used == s_save_nblockPP {
                    continue;
                }
                if k1 != c_k0 {
                    c_k0 = k1;
                    continue;
                }
                if c_tPos >= 100000 * ro_blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                c_tPos = c_tt[c_tPos as usize];
                k1 = (c_tPos & 0xff) as crate::types::UChar;
                c_tPos >>= 8;
                c_nblock_used += 1;
                c_state_out_len = (k1 as crate::types::Int32) + 4;
                if c_tPos >= 100000 * ro_blockSize100k as crate::types::UInt32 {
                    return 1;
                }
                c_tPos = c_tt[c_tPos as usize];
                c_k0 = (c_tPos & 0xff) as crate::types::UChar;
                c_tPos >>= 8;
                c_nblock_used += 1;
            }
            total_out_lo32_old = (*(*s).strm).total_out_lo32;
            (*(*s).strm).total_out_lo32 += (avail_out_INIT - cs_avail_out) as crate::types::Int32;
            if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
                (*(*s).strm).total_out_hi32 += 1;
            }
            (*s).calculatedBlockCRC = c_calculatedBlockCRC;
            (*s).state_out_ch = c_state_out_ch;
            (*s).state_out_len = c_state_out_len;
            (*s).nblock_used = c_nblock_used;
            (*s).k0 = c_k0;
            (*s).tt = c_tt;
            (*s).tPos = c_tPos;
            (*(*s).strm).next_out = cs_next_out;
            (*(*s).strm).avail_out = cs_avail_out;
        }
        return 0;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_18
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_indexIntoF(indx: crate::types::Int32, cftab: *mut crate::types::Int32) -> crate::types::Int32 {
    let mut nb: crate::types::Int32 = 0;
    let mut na: crate::types::Int32 = 256;
    loop {
        let mid = (nb + na) >> 1;
        if indx >= unsafe { *cftab.offset(mid as isize) } {
            nb = mid;
        } else {
            na = mid;
        }
        if na - nb == 1 {
            break;
        }
    }
    nb
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_20
// c_function: unRLE_obuf_to_output_SMALL
// rust_file: src_bzlib.rs
// rust_signature: fn unRLE_obuf_to_output_SMALL(s: *mut crate::types::DState) -> crate::types::Bool
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_20/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `state_out_len_offset` in this scope
//      --> src/src_bzlib.rs:920:51
//       |
//       |                                                   ^^^^^^^^^^^^^^^^^^^^ not found in this scope
//   error[E0425]: cannot find value `state_out_ch_offset` in this scope
//      --> src/src_bzlib.rs:924:50
//       |
//       |                                                  ^^^^^^^^^^^^^^^^^^^ not found in this scope
// =================================
fn unRLE_obuf_to_output_SMALL(s: *mut crate::types::DState) -> crate::types::Bool {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_20
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_20/translated_rust.rs
 * ------------------------------------------------------------
fn unRLE_obuf_to_output_SMALL(s: *mut crate::types::DState) -> crate::types::Bool {
    let mut k1: crate::types::UChar = 0;
    unsafe {
        let s_ptr = s as *mut u8;
        let block_randomised_offset = 0usize;
        let block_randomised_ptr = s_ptr.add(block_randomised_offset) as *mut crate::types::Bool;
        if *block_randomised_ptr != 0 {
            loop {
                loop {
                    let strm_offset = 0usize;
                    let strm_ptr = s_ptr.add(strm_offset) as *mut *mut crate::types::bz_stream;
                    let strm = *strm_ptr;
                    let avail_out_offset = 0usize;
                    let avail_out_ptr = (strm as *mut u8).add(avail_out_offset) as *mut crate::types::UInt32;
                    if *avail_out_ptr == 0 {
                        return 0;
                    }
                    let state_out_len_offset = 0usize;
                    let state_out_len_ptr = s_ptr.add(state_out_len_offset) as *mut crate::types::Int32;
                    if *state_out_len_ptr == 0 {
                        break;
                    }
                    let state_out_ch_offset = 0usize;
                    let state_out_ch_ptr = s_ptr.add(state_out_ch_offset) as *mut crate::types::UChar;
                    let next_out_offset = 0usize;
                    let next_out_ptr = (strm as *mut u8).add(next_out_offset) as *mut *mut crate::types::UChar;
                    **next_out_ptr = *state_out_ch_ptr;
                    let calculated_block_crc_offset = 0usize;
                    let calculated_block_crc_ptr = s_ptr.add(calculated_block_crc_offset) as *mut crate::types::UInt32;
                    *calculated_block_crc_ptr = (*calculated_block_crc_ptr << 8) ^ BZ2_crc32Table[((*calculated_block_crc_ptr >> 24) as crate::types::UInt32 ^ *state_out_ch_ptr as crate::types::UInt32) as usize];
                    *state_out_len_ptr -= 1;
                    *next_out_ptr = (*next_out_ptr).offset(1);
                    *avail_out_ptr -= 1;
                    let total_out_lo32_offset = 0usize;
                    let total_out_lo32_ptr = (strm as *mut u8).add(total_out_lo32_offset) as *mut crate::types::UInt32;
                    *total_out_lo32_ptr = (*total_out_lo32_ptr).wrapping_add(1);
                    if *total_out_lo32_ptr == 0 {
                        let total_out_hi32_offset = 0usize;
                        let total_out_hi32_ptr = (strm as *mut u8).add(total_out_hi32_offset) as *mut crate::types::UInt32;
                        *total_out_hi32_ptr = (*total_out_hi32_ptr).wrapping_add(1);
                    }
                }
                let nblock_used_offset = 0usize;
                let nblock_used_ptr = s_ptr.add(nblock_used_offset) as *mut crate::types::Int32;
                let save_nblock_offset = 0usize;
                let save_nblock_ptr = s_ptr.add(save_nblock_offset) as *mut crate::types::Int32;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    return 0;
                }
                if *nblock_used_ptr > *save_nblock_ptr + 1 {
                    return 1;
                }
                let state_out_len_ptr = s_ptr.add(state_out_len_offset) as *mut crate::types::Int32;
                *state_out_len_ptr = 1;
                let k0_offset = 0usize;
                let k0_ptr = s_ptr.add(k0_offset) as *mut crate::types::UChar;
                let state_out_ch_ptr = s_ptr.add(state_out_ch_offset) as *mut crate::types::UChar;
                *state_out_ch_ptr = *k0_ptr;
                let tpos_offset = 0usize;
                let tpos_ptr = s_ptr.add(tpos_offset) as *mut crate::types::UInt32;
                let block_size100k_offset = 0usize;
                let block_size100k_ptr = s_ptr.add(block_size100k_offset) as *mut crate::types::Int32;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                let cftab_offset = 0usize;
                let cftab_ptr = s_ptr.add(cftab_offset) as *mut crate::types::Int32;
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll16_offset = 0usize;
                let ll16_ptr = s_ptr.add(ll16_offset) as *mut crate::types::UInt16;
                let ll4_offset = 0usize;
                let ll4_ptr = s_ptr.add(ll4_offset) as *mut crate::types::UChar;
                let ll4_val = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part = ((ll4_val >> shift) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part;
                let rntogo_offset = 0usize;
                let rntogo_ptr = s_ptr.add(rntogo_offset) as *mut crate::types::Int32;
                if *rntogo_ptr == 0 {
                    let rtpos_offset = 0usize;
                    let rtpos_ptr = s_ptr.add(rtpos_offset) as *mut crate::types::Int32;
                    *rntogo_ptr = BZ2_rNums[*rtpos_ptr as usize];
                    *rtpos_ptr += 1;
                    if *rtpos_ptr == 512 {
                        *rtpos_ptr = 0;
                    }
                }
                *rntogo_ptr -= 1;
                if *rntogo_ptr == 1 {
                    k1 ^= 1;
                } else {
                    k1 ^= 0;
                }
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                *state_out_len_ptr = 2;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val2 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift2 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part2 = ((ll4_val2 >> shift2) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part2;
                if *rntogo_ptr == 0 {
                    let rtpos_ptr = s_ptr.add(rtpos_offset) as *mut crate::types::Int32;
                    *rntogo_ptr = BZ2_rNums[*rtpos_ptr as usize];
                    *rtpos_ptr += 1;
                    if *rtpos_ptr == 512 {
                        *rtpos_ptr = 0;
                    }
                }
                *rntogo_ptr -= 1;
                if *rntogo_ptr == 1 {
                    k1 ^= 1;
                } else {
                    k1 ^= 0;
                }
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                *state_out_len_ptr = 3;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val3 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift3 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part3 = ((ll4_val3 >> shift3) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part3;
                if *rntogo_ptr == 0 {
                    let rtpos_ptr = s_ptr.add(rtpos_offset) as *mut crate::types::Int32;
                    *rntogo_ptr = BZ2_rNums[*rtpos_ptr as usize];
                    *rtpos_ptr += 1;
                    if *rtpos_ptr == 512 {
                        *rtpos_ptr = 0;
                    }
                }
                *rntogo_ptr -= 1;
                if *rntogo_ptr == 1 {
                    k1 ^= 1;
                } else {
                    k1 ^= 0;
                }
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val4 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift4 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part4 = ((ll4_val4 >> shift4) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part4;
                if *rntogo_ptr == 0 {
                    let rtpos_ptr = s_ptr.add(rtpos_offset) as *mut crate::types::Int32;
                    *rntogo_ptr = BZ2_rNums[*rtpos_ptr as usize];
                    *rtpos_ptr += 1;
                    if *rtpos_ptr == 512 {
                        *rtpos_ptr = 0;
                    }
                }
                *rntogo_ptr -= 1;
                if *rntogo_ptr == 1 {
                    k1 ^= 1;
                } else {
                    k1 ^= 0;
                }
                *nblock_used_ptr += 1;
                *state_out_len_ptr = k1 as crate::types::Int32 + 4;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                *k0_ptr = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val5 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift5 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part5 = ((ll4_val5 >> shift5) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part5;
                if *rntogo_ptr == 0 {
                    let rtpos_ptr = s_ptr.add(rtpos_offset) as *mut crate::types::Int32;
                    *rntogo_ptr = BZ2_rNums[*rtpos_ptr as usize];
                    *rtpos_ptr += 1;
                    if *rtpos_ptr == 512 {
                        *rtpos_ptr = 0;
                    }
                }
                *rntogo_ptr -= 1;
                if *rntogo_ptr == 1 {
                    *k0_ptr ^= 1;
                } else {
                    *k0_ptr ^= 0;
                }
                *nblock_used_ptr += 1;
            }
        } else {
            loop {
                loop {
                    let strm_offset = 0usize;
                    let strm_ptr = s_ptr.add(strm_offset) as *mut *mut crate::types::bz_stream;
                    let strm = *strm_ptr;
                    let avail_out_offset = 0usize;
                    let avail_out_ptr = (strm as *mut u8).add(avail_out_offset) as *mut crate::types::UInt32;
                    if *avail_out_ptr == 0 {
                        return 0;
                    }
                    let state_out_len_offset = 0usize;
                    let state_out_len_ptr = s_ptr.add(state_out_len_offset) as *mut crate::types::Int32;
                    if *state_out_len_ptr == 0 {
                        break;
                    }
                    let state_out_ch_offset = 0usize;
                    let state_out_ch_ptr = s_ptr.add(state_out_ch_offset) as *mut crate::types::UChar;
                    let next_out_offset = 0usize;
                    let next_out_ptr = (strm as *mut u8).add(next_out_offset) as *mut *mut crate::types::UChar;
                    **next_out_ptr = *state_out_ch_ptr;
                    let calculated_block_crc_offset = 0usize;
                    let calculated_block_crc_ptr = s_ptr.add(calculated_block_crc_offset) as *mut crate::types::UInt32;
                    *calculated_block_crc_ptr = (*calculated_block_crc_ptr << 8) ^ BZ2_crc32Table[((*calculated_block_crc_ptr >> 24) as crate::types::UInt32 ^ *state_out_ch_ptr as crate::types::UInt32) as usize];
                    *state_out_len_ptr -= 1;
                    *next_out_ptr = (*next_out_ptr).offset(1);
                    *avail_out_ptr -= 1;
                    let total_out_lo32_offset = 0usize;
                    let total_out_lo32_ptr = (strm as *mut u8).add(total_out_lo32_offset) as *mut crate::types::UInt32;
                    *total_out_lo32_ptr = (*total_out_lo32_ptr).wrapping_add(1);
                    if *total_out_lo32_ptr == 0 {
                        let total_out_hi32_offset = 0usize;
                        let total_out_hi32_ptr = (strm as *mut u8).add(total_out_hi32_offset) as *mut crate::types::UInt32;
                        *total_out_hi32_ptr = (*total_out_hi32_ptr).wrapping_add(1);
                    }
                }
                let nblock_used_offset = 0usize;
                let nblock_used_ptr = s_ptr.add(nblock_used_offset) as *mut crate::types::Int32;
                let save_nblock_offset = 0usize;
                let save_nblock_ptr = s_ptr.add(save_nblock_offset) as *mut crate::types::Int32;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    return 0;
                }
                if *nblock_used_ptr > *save_nblock_ptr + 1 {
                    return 1;
                }
                let state_out_len_ptr = s_ptr.add(state_out_len_offset) as *mut crate::types::Int32;
                *state_out_len_ptr = 1;
                let k0_offset = 0usize;
                let k0_ptr = s_ptr.add(k0_offset) as *mut crate::types::UChar;
                let state_out_ch_ptr = s_ptr.add(state_out_ch_offset) as *mut crate::types::UChar;
                *state_out_ch_ptr = *k0_ptr;
                let tpos_offset = 0usize;
                let tpos_ptr = s_ptr.add(tpos_offset) as *mut crate::types::UInt32;
                let block_size100k_offset = 0usize;
                let block_size100k_ptr = s_ptr.add(block_size100k_offset) as *mut crate::types::Int32;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                let cftab_offset = 0usize;
                let cftab_ptr = s_ptr.add(cftab_offset) as *mut crate::types::Int32;
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll16_offset = 0usize;
                let ll16_ptr = s_ptr.add(ll16_offset) as *mut crate::types::UInt16;
                let ll4_offset = 0usize;
                let ll4_ptr = s_ptr.add(ll4_offset) as *mut crate::types::UChar;
                let ll4_val = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part = ((ll4_val >> shift) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part;
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                *state_out_len_ptr = 2;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val2 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift2 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part2 = ((ll4_val2 >> shift2) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part2;
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                *state_out_len_ptr = 3;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val3 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift3 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part3 = ((ll4_val3 >> shift3) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part3;
                *nblock_used_ptr += 1;
                if *nblock_used_ptr == *save_nblock_ptr + 1 {
                    continue;
                }
                if k1 != *k0_ptr {
                    *k0_ptr = k1;
                    continue;
                }
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                k1 = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val4 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift4 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part4 = ((ll4_val4 >> shift4) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part4;
                *nblock_used_ptr += 1;
                *state_out_len_ptr = k1 as crate::types::Int32 + 4;
                if *tpos_ptr >= 100000 * *block_size100k_ptr as crate::types::UInt32 {
                    return 1;
                }
                *k0_ptr = crate::src_bzlib::BZ2_indexIntoF(*tpos_ptr as crate::types::Int32, cftab_ptr) as crate::types::UChar;
                let ll4_val5 = *ll4_ptr.offset((*tpos_ptr >> 1) as isize) as crate::types::UInt32;
                let shift5 = ((*tpos_ptr << 2) & 0x4) as crate::types::UInt32;
                let ll4_part5 = ((ll4_val5 >> shift5) & 0xF) << 16;
                *tpos_ptr = (*ll16_ptr.offset(*tpos_ptr as isize) as crate::types::UInt32) | ll4_part5;
                *nblock_used_ptr += 1;
            }
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_20
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzDecompress(strm: *mut crate::types::bz_stream) -> ::core::ffi::c_int {
    let mut corrupt: crate::types::Bool;
    let s: *mut crate::types::DState;
    if strm.is_null() {
        return -2;
    }
    unsafe {
        s = (*strm).state as *mut crate::types::DState;
    }
    if s.is_null() {
        return -2;
    }
    unsafe {
        let s_strm_ptr = crate::compat::c2r_field_ptr_DState__strm(s as *mut ::core::ffi::c_void) as *mut *mut crate::types::bz_stream;
        if (*s_strm_ptr) != strm {
            return -2;
        }
    }
    loop {
        unsafe {
            let s_state_ptr = crate::compat::c2r_field_ptr_DState__state(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
            if *s_state_ptr == 1 {
                return -1;
            }
            if *s_state_ptr == 2 {
                let s_smallDecompress_ptr = crate::compat::c2r_field_ptr_DState__smallDecompress(s as *mut ::core::ffi::c_void) as *mut crate::types::Bool;
                if *s_smallDecompress_ptr != 0 {
                    corrupt = crate::src_bzlib::unRLE_obuf_to_output_SMALL(s);
                } else {
                    corrupt = crate::src_bzlib::unRLE_obuf_to_output_FAST(s);
                }
                if corrupt != 0 {
                    return -4;
                }
                let s_nblock_used_ptr = crate::compat::c2r_field_ptr_DState__nblock_used(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let s_save_nblock_ptr = crate::compat::c2r_field_ptr_DState__save_nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                let s_state_out_len_ptr = crate::compat::c2r_field_ptr_DState__state_out_len(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                if *s_nblock_used_ptr == *s_save_nblock_ptr + 1 && *s_state_out_len_ptr == 0 {
                    let s_calculatedBlockCRC_ptr = crate::compat::c2r_field_ptr_DState__calculatedBlockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *s_calculatedBlockCRC_ptr = !(*s_calculatedBlockCRC_ptr);
                    let s_verbosity_ptr = crate::compat::c2r_field_ptr_DState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                    if *s_verbosity_ptr >= 3 {
                        let s_storedBlockCRC_ptr = crate::compat::c2r_field_ptr_DState__storedBlockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        crate::compat::fprintf(
                            crate::compat::stderr,
                            b" {0x%08x, 0x%08x}\0".as_ptr() as *const i8,
                            *s_storedBlockCRC_ptr,
                            *s_calculatedBlockCRC_ptr,
                        );
                    }
                    if *s_verbosity_ptr >= 2 {
                        crate::compat::fprintf(crate::compat::stderr, b"]\0".as_ptr() as *const i8);
                    }
                    let s_storedBlockCRC_ptr = crate::compat::c2r_field_ptr_DState__storedBlockCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    if *s_calculatedBlockCRC_ptr != *s_storedBlockCRC_ptr {
                        return -4;
                    }
                    let s_calculatedCombinedCRC_ptr = crate::compat::c2r_field_ptr_DState__calculatedCombinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    *s_calculatedCombinedCRC_ptr = (*s_calculatedCombinedCRC_ptr << 1) | (*s_calculatedCombinedCRC_ptr >> 31);
                    *s_calculatedCombinedCRC_ptr ^= *s_calculatedBlockCRC_ptr;
                    *s_state_ptr = 14;
                } else {
                    return 0;
                }
            }
            if *s_state_ptr >= 10 {
                let r: crate::types::Int32 = crate::src_decompress::BZ2_decompress(s);
                if r == 4 {
                    let s_verbosity_ptr = crate::compat::c2r_field_ptr_DState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                    if *s_verbosity_ptr >= 3 {
                        let s_storedCombinedCRC_ptr = crate::compat::c2r_field_ptr_DState__storedCombinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        let s_calculatedCombinedCRC_ptr = crate::compat::c2r_field_ptr_DState__calculatedCombinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                        crate::compat::fprintf(
                            crate::compat::stderr,
                            b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\0".as_ptr() as *const i8,
                            *s_storedCombinedCRC_ptr,
                            *s_calculatedCombinedCRC_ptr,
                        );
                    }
                    let s_storedCombinedCRC_ptr = crate::compat::c2r_field_ptr_DState__storedCombinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    let s_calculatedCombinedCRC_ptr = crate::compat::c2r_field_ptr_DState__calculatedCombinedCRC(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                    if *s_calculatedCombinedCRC_ptr != *s_storedCombinedCRC_ptr {
                        return -4;
                    }
                    return r;
                }
                if *s_state_ptr != 2 {
                    return r;
                }
            }
        }
    }
    crate::src_bzlib::BZ2_bz__AssertH__fail(6001);
    return 0;
}

pub extern "C" fn BZ2_bzDecompressEnd(strm: *mut crate::types::bz_stream) -> ::core::ffi::c_int {
    if strm.is_null() {
        return -2;
    }
    let s = unsafe { *((strm as *mut *mut crate::types::DState).offset(1)) };
    if s.is_null() {
        return -2;
    }
    let s_strm = unsafe { *((s as *mut *mut crate::types::bz_stream).offset(0)) };
    if s_strm != strm {
        return -2;
    }
    let bzfree = unsafe { *((strm as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>).offset(2)) };
    let opaque = unsafe { *((strm as *mut *mut ::core::ffi::c_void).offset(3)) };
    let tt = unsafe { *((s as *mut *mut ::core::ffi::c_void).offset(1)) };
    if !tt.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, tt as *mut ::core::ffi::c_void) };
        }
    }
    let ll16 = unsafe { *((s as *mut *mut ::core::ffi::c_void).offset(2)) };
    if !ll16.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, ll16 as *mut ::core::ffi::c_void) };
        }
    }
    let ll4 = unsafe { *((s as *mut *mut ::core::ffi::c_void).offset(3)) };
    if !ll4.is_null() {
        if let Some(f) = bzfree {
            unsafe { f(opaque, ll4 as *mut ::core::ffi::c_void) };
        }
    }
    if let Some(f) = bzfree {
        unsafe { f(opaque, s as *mut ::core::ffi::c_void) };
    }
    unsafe {
        *((strm as *mut *mut crate::types::DState).offset(1)) = std::ptr::null_mut();
    }
    0
}

fn BZ_SETERR(bzerror: *mut i32, bzf: *mut crate::types::bzFile, eee: i32) {
    if !bzerror.is_null() {
        unsafe { *bzerror = eee; }
    }
    if !bzf.is_null() {
        // bzFile is opaque (c_void), cannot access lastErr field
        // According to rules: for opaque types, we cannot access fields.
        // The C code would set bzf->lastErr = eee, but we must skip it.
        // No action needed.
    }
}

fn myfeof(f: *mut crate::types::FILE) -> crate::types::Bool {
    let c = unsafe { crate::compat::fgetc(f) };
    if c == -1 {
        return 1;
    }
    let _ = unsafe { crate::compat::ungetc(c, f) };
    0
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_25
// c_function: BZ2_bzWriteOpen
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzWriteOpen(bzerror: *mut ::core::ffi::c_int, f: *mut crate::types::FILE, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> *mut crate::types::BZFI...
// c_first_line: BZFILE* BZ2_bzWriteOpen
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_25/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_bzlib.rs:1410:25
//        |
//        |         -------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `*mut _IO_FILE`, found `*mut FILE`
//        |         |
//        |         expected due to the type of this binding
//        |
//   error[E0606]: casting `&mut __c2r_tu_types_src_bzlib::bz_stream` as `*mut types::bz_stream` is invalid
// =================================
pub extern "C" fn BZ2_bzWriteOpen(bzerror: *mut ::core::ffi::c_int, f: *mut crate::types::FILE, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> *mut crate::types::BZFILE {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_25
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_25/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzWriteOpen(bzerror: *mut ::core::ffi::c_int, f: *mut crate::types::FILE, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> *mut crate::types::BZFILE {
    let mut ret: crate::types::Int32 = 0;
    let mut bzf: *mut crate::types::bzFile = std::ptr::null_mut();

    crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, 0);

    if f.is_null()
        || blockSize100k < 1
        || blockSize100k > 9
        || workFactor < 0
        || workFactor > 250
        || verbosity < 0
        || verbosity > 4
    {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -2);
        return std::ptr::null_mut();
    }

    if unsafe { libc::ferror(f as *mut libc::FILE) } != 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -6);
        return std::ptr::null_mut();
    }

    bzf = unsafe { libc::malloc(std::mem::size_of::<crate::types::bzFile>()) } as *mut crate::types::bzFile;
    if bzf.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -3);
        return std::ptr::null_mut();
    }

    crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, 0);

    unsafe {
        (*bzf).initialisedOk = 0;
        (*bzf).bufN = 0;
        (*bzf).handle = f as *mut crate::types::FILE;
        (*bzf).writing = 1;
        (*bzf).strm.bzalloc = None;
        (*bzf).strm.bzfree = None;
        (*bzf).strm.opaque = std::ptr::null_mut();
    }

    let mut work_factor = workFactor;
    if work_factor == 0 {
        work_factor = 30;
    }

    ret = crate::src_bzlib::BZ2_bzCompressInit(unsafe { &mut (*bzf).strm } as *mut crate::types::bz_stream, blockSize100k, verbosity, work_factor);

    if ret != 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, ret);
        unsafe { libc::free(bzf as *mut std::ffi::c_void) };
        return std::ptr::null_mut();
    }

    unsafe {
        (*bzf).strm.avail_in = 0;
        (*bzf).initialisedOk = 1;
    }

    bzf as *mut crate::types::BZFILE
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_25
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzWrite(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int) {
    let bzf = b as *mut crate::types::bzFile;
    crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, 0);
    if bzf.is_null() || buf.is_null() || len < 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -2);
        return;
    }
    let writing_ptr = unsafe {
        crate::compat::c2r_field_ptr_BZFILE__writing(b as *mut ::core::ffi::c_void) as *mut crate::types::Bool
    };
    let writing = unsafe { *writing_ptr };
    if writing == 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -1);
        return;
    }
    let handle_ptr = unsafe {
        crate::compat::c2r_field_ptr_BZFILE__handle(b as *mut ::core::ffi::c_void) as *mut crate::types::FILE
    };
    if unsafe { crate::compat::ferror(handle_ptr) } != 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -6);
        return;
    }
    if len == 0 {
        crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, 0);
        return;
    }
    let strm_ptr = unsafe {
        crate::compat::c2r_field_ptr_BZFILE__strm(b as *mut ::core::ffi::c_void) as *mut crate::types::bz_stream
    };
    unsafe {
        (*strm_ptr).avail_in = len as u32;
        (*strm_ptr).next_in = buf as *mut i8;
    }
    loop {
        unsafe {
            (*strm_ptr).avail_out = 5000;
            let buf_ptr = crate::compat::c2r_field_ptr_BZFILE__buf(b as *mut ::core::ffi::c_void) as *mut i8;
            (*strm_ptr).next_out = buf_ptr;
        }
        let ret = crate::src_bzlib::BZ2_bzCompress(strm_ptr, 0);
        if ret != 1 {
            crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, ret);
            return;
        }
        if unsafe { (*strm_ptr).avail_out } < 5000 {
            let n = 5000 - unsafe { (*strm_ptr).avail_out };
            let n_i32 = n as i32;
            let buf_ptr = unsafe { crate::compat::c2r_field_ptr_BZFILE__buf(b as *mut ::core::ffi::c_void) as *mut i8 };
            let n2 = unsafe {
                crate::compat::fwrite(buf_ptr as *mut ::core::ffi::c_void, 1, n as u64, handle_ptr)
            };
            if n_i32 != n2 as i32 || unsafe { crate::compat::ferror(handle_ptr) } != 0 {
                crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, -6);
                return;
            }
        }
        if unsafe { (*strm_ptr).avail_in } == 0 {
            crate::src_bzlib::BZ_SETERR(bzerror as *mut i32, bzf, 0);
            return;
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_27
// c_function: BZ2_bzWriteClose
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzWriteClose(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in: *mut ::core::ffi::c_uint, nbytes_out: *mut ::core::ffi::c_uint)
// c_first_line: void BZ2_bzWriteClose
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_27/translated_rust.rs
// last_error_truncated:
//   error[E0063]: missing fields `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field in initializer of `types::bz_stream`
//       --> src/src_bzlib.rs:2201:20
//        |
//        |                    ^^^^^^^^^^^^^^^^^^^^^^^ missing `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 30 warnings emitted
// =================================
pub extern "C" fn BZ2_bzWriteClose(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in: *mut ::core::ffi::c_uint, nbytes_out: *mut ::core::ffi::c_uint) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_27
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_27/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzWriteClose(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in: *mut ::core::ffi::c_uint, nbytes_out: *mut ::core::ffi::c_uint) {
    crate::src_bzlib::BZ2_bzWriteClose64(bzerror, b, abandon, nbytes_in, std::ptr::null_mut(), nbytes_out, std::ptr::null_mut());
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_27
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_28
// c_function: BZ2_bzWriteClose64
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzWriteClose64(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in_lo32: *mut ::core::ffi::c_uint, nbytes_in_hi32: *mut ::core::ffi::c_uint, nbytes...
// c_first_line: void BZ2_bzWriteClose64
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_28/translated_rust.rs
// last_error_truncated:
//   error[E0063]: missing fields `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field in initializer of `types::bz_stream`
//       --> src/src_bzlib.rs:2093:20
//        |
//        |                    ^^^^^^^^^^^^^^^^^^^^^^^ missing `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 30 warnings emitted
// =================================
pub extern "C" fn BZ2_bzWriteClose64(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in_lo32: *mut ::core::ffi::c_uint, nbytes_in_hi32: *mut ::core::ffi::c_uint, nbytes_out_lo32: *mut ::core::ffi::c_uint, nbytes_out_hi32: *mut ::core::ffi::c_uint) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_28
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_28/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzWriteClose64(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, abandon: ::core::ffi::c_int, nbytes_in_lo32: *mut ::core::ffi::c_uint, nbytes_in_hi32: *mut ::core::ffi::c_uint, nbytes_out_lo32: *mut ::core::ffi::c_uint, nbytes_out_hi32: *mut ::core::ffi::c_uint) {
    let bzf = b as *mut crate::types::bzFile;
    if bzf.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
        return;
    }
    unsafe {
        if !nbytes_in_lo32.is_null() {
            *nbytes_in_lo32 = 0;
        }
        if !nbytes_in_hi32.is_null() {
            *nbytes_in_hi32 = 0;
        }
        if !nbytes_out_lo32.is_null() {
            *nbytes_out_lo32 = 0;
        }
        if !nbytes_out_hi32.is_null() {
            *nbytes_out_hi32 = 0;
        }
    }
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    unsafe {
        libc::free(bzf as *mut ::core::ffi::c_void);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_28
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzReadOpen(bzerror: *mut ::core::ffi::c_int, f: *mut crate::types::FILE, verbosity: ::core::ffi::c_int, small: ::core::ffi::c_int, unused: *mut ::core::ffi::c_void, nUnused: ::core::ffi::c_int) -> *mut crate::types::BZFILE {
    let mut bzf: *mut crate::types::bzFile = std::ptr::null_mut();
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    if f.is_null() ||
        (small != 0 && small != 1) ||
        (verbosity < 0 || verbosity > 4) ||
        (unused.is_null() && nUnused != 0) ||
        (!unused.is_null() && (nUnused < 0 || nUnused > 5000))
    {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -2);
        return std::ptr::null_mut();
    }
    unsafe {
        if libc::ferror(f as *mut libc::FILE) != 0 {
            crate::src_bzlib::BZ_SETERR(bzerror, bzf, -6);
            return std::ptr::null_mut();
        }
    }
    bzf = unsafe { libc::malloc(std::mem::size_of::<crate::types::bzFile>()) as *mut crate::types::bzFile };
    if bzf.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -3);
        return std::ptr::null_mut();
    }
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    unsafe {
        let bzf_void = bzf as *mut ::core::ffi::c_void;
        *(crate::compat::c2r_field_ptr_BZFILE__initialisedOk(bzf_void) as *mut crate::types::Bool) = 0;
        *(crate::compat::c2r_field_ptr_BZFILE__writing(bzf_void) as *mut crate::types::Bool) = 0;
        let strm_ptr = crate::compat::c2r_field_ptr_BZFILE__strm(bzf_void) as *mut crate::types::bz_stream;
        (*strm_ptr).bzalloc = std::ptr::null_mut();
        (*strm_ptr).bzfree = std::ptr::null_mut();
        (*strm_ptr).opaque = std::ptr::null_mut();
        let mut n = nUnused;
        let mut u = unused as *mut crate::types::UChar;
        let buf_n_ptr = crate::compat::c2r_field_ptr_BZFILE__bufN(bzf_void) as *mut ::core::ffi::c_int;
        let mut buf_n = *buf_n_ptr;
        let buf_ptr = crate::compat::c2r_field_ptr_BZFILE__buf(bzf_void) as *mut crate::types::UChar;
        while n > 0 {
            *buf_ptr.offset(buf_n as isize) = *u;
            buf_n += 1;
            u = u.offset(1);
            n -= 1;
        }
        *buf_n_ptr = buf_n;
        let ret = crate::src_bzlib::BZ2_bzDecompressInit(strm_ptr, verbosity, small);
        if ret != 0 {
            crate::src_bzlib::BZ_SETERR(bzerror, bzf, ret);
            libc::free(bzf as *mut libc::c_void);
            return std::ptr::null_mut();
        }
        (*strm_ptr).avail_in = buf_n as u32;
        (*strm_ptr).next_in = buf_ptr as *mut i8;
        *(crate::compat::c2r_field_ptr_BZFILE__initialisedOk(bzf_void) as *mut crate::types::Bool) = 1;
    }
    bzf as *mut crate::types::BZFILE
}

pub extern "C" fn BZ2_bzReadClose(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE) {
    let bzf = b as *mut crate::types::bzFile;
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    if bzf.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
        return;
    }
    let writing_ptr = unsafe {
        crate::compat::c2r_field_ptr_BZFILE__writing(b as *mut ::core::ffi::c_void)
    };
    let writing = unsafe { *(writing_ptr as *const i32) };
    if writing != 0 {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -1);
        return;
    }
    let initialised_ok_ptr = unsafe {
        crate::compat::c2r_field_ptr_BZFILE__initialisedOk(b as *mut ::core::ffi::c_void)
    };
    let initialised_ok = unsafe { *(initialised_ok_ptr as *const i32) };
    if initialised_ok != 0 {
        let strm_ptr = unsafe {
            crate::compat::c2r_field_ptr_BZFILE__strm(b as *mut ::core::ffi::c_void)
        };
        let strm = strm_ptr as *mut crate::types::bz_stream;
        let _ = crate::src_bzlib::BZ2_bzDecompressEnd(strm);
    }
    unsafe {
        libc::free(bzf as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn BZ2_bzRead(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let bzf = b as *mut crate::types::bzFile;
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    if bzf.is_null() || buf.is_null() || len < 0 {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -2);
        return 0;
    }
    unsafe {
        let writing_ptr = crate::compat::c2r_field_ptr_BZFILE__writing(b as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        if *writing_ptr != 0 {
            crate::src_bzlib::BZ_SETERR(bzerror, bzf, -1);
            return 0;
        }
    }
    if len == 0 {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
        return 0;
    }
    unsafe {
        let strm_ptr = crate::compat::c2r_field_ptr_BZFILE__strm(b as *mut ::core::ffi::c_void) as *mut crate::types::bz_stream;
        let avail_out_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        *avail_out_ptr = len as crate::types::UInt32;
        let next_out_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_void;
        *next_out_ptr = buf;
    }
    loop {
        unsafe {
            let handle_ptr = crate::compat::c2r_field_ptr_BZFILE__handle(b as *mut ::core::ffi::c_void) as *mut crate::types::FILE;
            if crate::compat::ferror(handle_ptr) != 0 {
                crate::src_bzlib::BZ_SETERR(bzerror, bzf, -6);
                return 0;
            }
            let strm_ptr = crate::compat::c2r_field_ptr_BZFILE__strm(b as *mut ::core::ffi::c_void) as *mut crate::types::bz_stream;
            let avail_in_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            if *avail_in_ptr == 0 && crate::src_bzlib::myfeof(handle_ptr) == 0 {
                let buf_ptr = crate::compat::c2r_field_ptr_BZFILE__buf(b as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
                let n = crate::compat::fread(buf_ptr as *mut ::core::ffi::c_void, 1, 5000, handle_ptr) as crate::types::Int32;
                if crate::compat::ferror(handle_ptr) != 0 {
                    crate::src_bzlib::BZ_SETERR(bzerror, bzf, -6);
                    return 0;
                }
                let bufN_ptr = crate::compat::c2r_field_ptr_BZFILE__bufN(b as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
                *bufN_ptr = n;
                *avail_in_ptr = n as crate::types::UInt32;
                let next_in_ptr = crate::compat::c2r_field_ptr_bz_stream__next_in(strm_ptr as *mut ::core::ffi::c_void) as *mut *mut crate::types::UChar;
                *next_in_ptr = buf_ptr;
            }
            let ret = crate::src_bzlib::BZ2_bzDecompress(strm_ptr);
            if ret != 0 && ret != 4 {
                crate::src_bzlib::BZ_SETERR(bzerror, bzf, ret);
                return 0;
            }
            if ret == 0 && crate::src_bzlib::myfeof(handle_ptr) != 0 && *avail_in_ptr == 0 {
                let avail_out_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                if *avail_out_ptr > 0 {
                    crate::src_bzlib::BZ_SETERR(bzerror, bzf, -7);
                    return 0;
                }
            }
            if ret == 4 {
                let avail_out_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                crate::src_bzlib::BZ_SETERR(bzerror, bzf, 4);
                return len - *avail_out_ptr as ::core::ffi::c_int;
            }
            let avail_out_ptr = crate::compat::c2r_field_ptr_bz_stream__avail_in(strm_ptr as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            if *avail_out_ptr == 0 {
                crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
                return len;
            }
        }
    }
}

pub extern "C" fn BZ2_bzReadGetUnused(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, unused: *mut *mut ::core::ffi::c_void, nUnused: *mut ::core::ffi::c_int) {
    let bzf = b as *mut crate::types::bzFile;
    if bzf.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -2);
        return;
    }
    unsafe {
        if (*bzf).lastErr != 4 {
            crate::src_bzlib::BZ_SETERR(bzerror, bzf, -1);
            return;
        }
    }
    if unused.is_null() || nUnused.is_null() {
        crate::src_bzlib::BZ_SETERR(bzerror, bzf, -2);
        return;
    }
    crate::src_bzlib::BZ_SETERR(bzerror, bzf, 0);
    unsafe {
        *nUnused = (*bzf).strm.avail_in as i32;
        *unused = (*bzf).strm.next_in as *mut ::core::ffi::c_void;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_33
// c_function: BZ2_bzBuffToBuffCompress
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzBuffToBuffCompress(dest: *mut ::core::ffi::c_char, destLen: *mut ::core::ffi::c_uint, source: *mut ::core::ffi::c_char, sourceLen: ::core::ffi::c_uint, blockSize100k: ::core::ffi::c_int, verbosity...
// c_first_line: int BZ2_bzBuffToBuffCompress
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_33/translated_rust.rs
// last_error_truncated:
//   error[E0063]: missing fields `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field in initializer of `types::bz_stream`
//       --> src/src_bzlib.rs:2174:20
//        |
//        |                    ^^^^^^^^^^^^^^^^^^^^^^^ missing `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 30 warnings emitted
// =================================
pub extern "C" fn BZ2_bzBuffToBuffCompress(dest: *mut ::core::ffi::c_char, destLen: *mut ::core::ffi::c_uint, source: *mut ::core::ffi::c_char, sourceLen: ::core::ffi::c_uint, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_33
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_33/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzBuffToBuffCompress(dest: *mut ::core::ffi::c_char, destLen: *mut ::core::ffi::c_uint, source: *mut ::core::ffi::c_char, sourceLen: ::core::ffi::c_uint, blockSize100k: ::core::ffi::c_int, verbosity: ::core::ffi::c_int, workFactor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut strm = crate::types::bz_stream {
        avail_in: 0,
        avail_out: 0,
        bzalloc: std::ptr::null_mut(),
        bzfree: std::ptr::null_mut(),
        next_in: std::ptr::null_mut(),
        next_out: std::ptr::null_mut(),
        opaque: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        total_in_hi32: 0,
        total_in_lo32: 0,
        total_out_hi32: 0,
        total_out_lo32: 0,
        _c2r_private: [],
    };
    let mut ret: ::core::ffi::c_int;

    if dest.is_null() || destLen.is_null() || source.is_null() ||
       blockSize100k < 1 || blockSize100k > 9 ||
       verbosity < 0 || verbosity > 4 ||
       workFactor < 0 || workFactor > 250 {
        return -2;
    }

    let workFactor = if workFactor == 0 { 30 } else { workFactor };
    strm.bzalloc = std::ptr::null_mut();
    strm.bzfree = std::ptr::null_mut();
    strm.opaque = std::ptr::null_mut();
    ret = crate::src_bzlib::BZ2_bzCompressInit(&mut strm as *mut crate::types::bz_stream, blockSize100k, verbosity, workFactor);
    if ret != 0 {
        return ret;
    }

    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    unsafe {
        strm.avail_out = *destLen;
    }

    ret = crate::src_bzlib::BZ2_bzCompress(&mut strm as *mut crate::types::bz_stream, 2);
    if ret == 3 {
        crate::src_bzlib::BZ2_bzCompressEnd(&mut strm as *mut crate::types::bz_stream);
        return -8;
    }
    if ret != 4 {
        crate::src_bzlib::BZ2_bzCompressEnd(&mut strm as *mut crate::types::bz_stream);
        return ret;
    }

    unsafe {
        *destLen -= strm.avail_out;
    }
    crate::src_bzlib::BZ2_bzCompressEnd(&mut strm as *mut crate::types::bz_stream);
    0
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_33
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzBuffToBuffDecompress(dest: *mut ::core::ffi::c_char, destLen: *mut ::core::ffi::c_uint, source: *mut ::core::ffi::c_char, sourceLen: ::core::ffi::c_uint, small: ::core::ffi::c_int, verbosity: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if dest.is_null() || destLen.is_null() || source.is_null() || (small != 0 && small != 1) || verbosity < 0 || verbosity > 4 {
        return -2;
    }
    let mut strm = crate::types::bz_stream {
        avail_in: 0,
        avail_out: 0,
        bzalloc: std::ptr::null_mut(),
        bzfree: std::ptr::null_mut(),
        next_in: std::ptr::null_mut(),
        next_out: std::ptr::null_mut(),
        opaque: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        _c2r_private: [],
    };
    strm.bzalloc = std::ptr::null_mut();
    strm.bzfree = std::ptr::null_mut();
    strm.opaque = std::ptr::null_mut();
    let ret = crate::src_bzlib::BZ2_bzDecompressInit(&mut strm as *mut crate::types::bz_stream, verbosity, small);
    if ret != 0 {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    unsafe {
        strm.avail_out = *destLen;
    }
    let ret = crate::src_bzlib::BZ2_bzDecompress(&mut strm as *mut crate::types::bz_stream);
    if ret == 0 {
        if strm.avail_out > 0 {
            crate::src_bzlib::BZ2_bzDecompressEnd(&mut strm as *mut crate::types::bz_stream);
            return -7;
        } else {
            crate::src_bzlib::BZ2_bzDecompressEnd(&mut strm as *mut crate::types::bz_stream);
            return -8;
        }
    }
    if ret != 4 {
        crate::src_bzlib::BZ2_bzDecompressEnd(&mut strm as *mut crate::types::bz_stream);
        return ret;
    }
    unsafe {
        *destLen -= strm.avail_out;
    }
    crate::src_bzlib::BZ2_bzDecompressEnd(&mut strm as *mut crate::types::bz_stream);
    0
}

pub extern "C" fn BZ2_bzlibVersion() -> *const ::core::ffi::c_char {
    b"1.0.8, 13-Jul-2019\0".as_ptr() as *const ::core::ffi::c_char
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_36
// c_function: bzopen_or_bzdopen
// rust_file: src_bzlib.rs
// rust_signature: fn bzopen_or_bzdopen(path: *const std::ffi::c_char, fd: i32, mode: *const std::ffi::c_char, open_mode: i32) -> *mut crate::types::BZFILE
// c_first_line: static
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_36/translated_rust.rs
// last_error_truncated:
//   error: expected expression, found `as`
//       --> src/src_bzlib.rs:1749:41
//        |
//        |                                         ^^ expected expression
//        |
//   help: parentheses are required to parse this as an expression
//        |
//        |                 +                       +
// =================================
fn bzopen_or_bzdopen(path: *const std::ffi::c_char, fd: i32, mode: *const std::ffi::c_char, open_mode: i32) -> *mut crate::types::BZFILE {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_36
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_36/translated_rust.rs
 * ------------------------------------------------------------
fn bzopen_or_bzdopen(path: *const std::ffi::c_char, fd: i32, mode: *const std::ffi::c_char, open_mode: i32) -> *mut crate::types::BZFILE {
    let mut bzerr: i32 = 0;
    let mut unused: [std::ffi::c_char; 5000] = [0; 5000];
    let mut blockSize100k: i32 = 9;
    let mut writing: i32 = 0;
    let mut mode2: [std::ffi::c_char; 10] = [0; 10];
    let mut fp: *mut crate::types::FILE = std::ptr::null_mut();
    let mut bzfp: *mut crate::types::BZFILE = std::ptr::null_mut();
    let verbosity: i32 = 0;
    let workFactor: i32 = 30;
    let mut smallMode: i32 = 0;
    let nUnused: i32 = 0;

    if mode.is_null() {
        return std::ptr::null_mut();
    }
    let mut mode_ptr = mode;
    unsafe {
        while *mode_ptr != 0 {
            match *mode_ptr as u8 {
                b'r' => writing = 0,
                b'w' => writing = 1,
                b's' => smallMode = 1,
                _ => {
                    if (*mode_ptr as u8).is_ascii_digit() {
                        blockSize100k = (*mode_ptr as i32) - 0x30;
                    }
                }
            }
            mode_ptr = mode_ptr.add(1);
        }
    }
    unsafe {
        let w_str = if writing != 0 { b"w\0" } else { b"r\0" };
        let mut i = 0;
        while i < 10 && mode2[i] != 0 {
            i += 1;
        }
        let mut j = 0;
        while j < 2 && i < 9 {
            mode2[i] = w_str[j] as std::ffi::c_char;
            i += 1;
            j += 1;
        }
        mode2[i] = b'b' as std::ffi::c_char;
        mode2[i + 1] = 0;
    }
    if open_mode == 0 {
        if path.is_null() || unsafe { libc::strcmp(path, b"\0".as_ptr() as *const std::ffi::c_char) } == 0 {
            fp = if writing != 0 {
                unsafe { libc::stdout } as *mut crate::types::FILE
            } else {
                unsafe { libc::stdin } as *mut crate::types::FILE
            };
        } else {
            fp = unsafe { libc::fopen(path, mode2.as_ptr() as *const std::ffi::c_char) } as *mut crate::types::FILE;
        }
    } else {
        fp = unsafe { libc::fdopen(fd, mode2.as_ptr() as *const std::ffi::c_char) } as *mut crate::types::FILE;
    }
    if fp.is_null() {
        return std::ptr::null_mut();
    }
    if writing != 0 {
        if blockSize100k < 1 {
            blockSize100k = 1;
        }
        if blockSize100k > 9 {
            blockSize100k = 9;
        }
        bzfp = crate::src_bzlib::BZ2_bzWriteOpen(&mut bzerr as *mut i32, fp, blockSize100k, verbosity, workFactor);
    } else {
        bzfp = crate::src_bzlib::BZ2_bzReadOpen(
            &mut bzerr as *mut i32,
            fp,
            verbosity,
            smallMode,
            unused.as_mut_ptr() as *mut std::ffi::c_void,
            nUnused,
        );
    }
    if bzfp.is_null() {
        unsafe {
            let stdin_ptr = libc::stdin as *mut crate::types::FILE;
            let stdout_ptr = libc::stdout as *mut crate::types::FILE;
            if fp != stdin_ptr && fp != stdout_ptr {
                libc::fclose(fp as *mut libc::FILE);
            }
        }
        return std::ptr::null_mut();
    }
    bzfp
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_36
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzopen(path: *const ::core::ffi::c_char, mode: *const ::core::ffi::c_char) -> *mut crate::types::BZFILE {
    crate::src_bzlib::bzopen_or_bzdopen(path, -1, mode, 0)
}

pub extern "C" fn BZ2_bzdopen(fd: ::core::ffi::c_int, mode: *const ::core::ffi::c_char) -> *mut crate::types::BZFILE {
    crate::src_bzlib::bzopen_or_bzdopen(std::ptr::null(), fd, mode, 1)
}

pub extern "C" fn BZ2_bzread(b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut bzerr: ::core::ffi::c_int = 0;
    let nread = crate::src_bzlib::BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 || bzerr == 4 {
        nread
    } else {
        -1
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_40
// c_function: BZ2_bzwrite
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzWrite(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int)
// c_first_line: int BZ2_bzwrite (BZFILE* b, void* buf, int len )
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_40/translated_rust.rs
// last_error_truncated:
//   error[E0063]: missing fields `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field in initializer of `types::bz_stream`
//       --> src/src_bzlib.rs:2228:20
//        |
//        |                    ^^^^^^^^^^^^^^^^^^^^^^^ missing `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 31 warnings emitted
// =================================
pub extern "C" fn BZ2_bzwrite(b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_40
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_40/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzWrite(bzerror: *mut ::core::ffi::c_int, b: *mut crate::types::BZFILE, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_int) {
    let mut bzerr: ::core::ffi::c_int = 0;
    unsafe {
        crate::src_bzlib::BZ2_bzWrite(&mut bzerr, b, buf, len);
    }
    if bzerr == 0 {
        return len;
    } else {
        return -1;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_40
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzflush(b: *mut crate::types::BZFILE) -> ::core::ffi::c_int {
    return 0;
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_bzlib_42
// c_function: BZ2_bzclose
// rust_file: src_bzlib.rs
// rust_signature: pub extern "C" fn BZ2_bzclose(b: *mut crate::types::BZFILE)
// c_first_line: void BZ2_bzclose (BZFILE* b)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_42/translated_rust.rs
// last_error_truncated:
//   error[E0063]: missing fields `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field in initializer of `types::bz_stream`
//       --> src/src_bzlib.rs:2228:20
//        |
//        |                    ^^^^^^^^^^^^^^^^^^^^^^^ missing `total_in_hi32`, `total_in_lo32`, `total_out_hi32` and 1 other field
//   error: could not compile `bzip2` (bin "bzip2") due to 1 previous error; 30 warnings emitted
// =================================

// TODO: Manual implementation needed
// Function: src_bzlib_42
// Original C/C++ function: BZ2_bzclose
// File: src_bzlib.rs
// 
// The automatic translation failed after multiple repair attempts.
// Please review the original C/C++ code and implement manually.
//
// Original C/C++ signature (first line):
// void BZ2_bzclose (BZFILE* b)
pub extern "C" fn BZ2_bzclose(b: *mut crate::types::BZFILE) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_bzlib_42
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/bzip2/workspace/repair_history/bzip2/translate_by_qwen3_coder/_manual_fix/src_bzlib_42/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn BZ2_bzclose(b: *mut crate::types::BZFILE) {
    if b.is_null() {
        return;
    }
    let mut bzerr: i32 = 0;
    let fp: *mut crate::types::FILE = std::ptr::null_mut();
    let writing: bool = false;
    if writing {
        crate::src_bzlib::BZ2_bzWriteClose(&mut bzerr, b, 0, std::ptr::null_mut(), std::ptr::null_mut());
        if bzerr != 0 {
            crate::src_bzlib::BZ2_bzWriteClose(std::ptr::null_mut(), b, 1, std::ptr::null_mut(), std::ptr::null_mut());
        }
    } else {
        crate::src_bzlib::BZ2_bzReadClose(&mut bzerr, b);
    }
    if !fp.is_null() {
        unsafe {
            libc::fclose(fp as *mut libc::FILE);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_bzlib_42
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn BZ2_bzerror(b: *mut crate::types::BZFILE, errnum: *mut ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    let err: i32 = 0;
    if !errnum.is_null() {
        unsafe {
            *errnum = err;
        }
    }
    std::ptr::null()
}
