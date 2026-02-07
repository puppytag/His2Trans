//! Module: src_app_verify_hap
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

pub extern "C" fn GetDigestAlgorithmId(signAlgorithm: u32) -> i32 {
    match signAlgorithm & 0x000f {
        0x01 | 0x04 => crate::types::MBEDTLS_MD_SHA256 as i32,
        0x02 | 0x05 => crate::types::MBEDTLS_MD_SHA384 as i32,
        0x03 | 0x06 => crate::types::MBEDTLS_MD_SHA512 as i32,
        _ => {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: signAlgorithm: %u error\0".as_ptr() as *const _,
                    b"GetDigestAlgorithmId\0".as_ptr() as *const _,
                    38,
                    signAlgorithm,
                )
            };
            crate::types::V_ERR as i32
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_2
// c_function: ComputeBlockHash
// rust_file: src_app_verify_hap.rs
// rust_signature: fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32
// c_first_line: static int32_t ComputeBlockHash(const char *block, int32_t blockLen, int32_t alg, const HapBuf *result, int32_t *offset)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_2/translated_rust.rs
// last_error_truncated:
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//     --> src/src_app_verify_hap.rs:62:18
//      |
//      |                  ^^^^ expected one of 8 possible tokens
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//     --> src/src_app_verify_hap.rs:66:18
//      |
//      |                  ^^^^ expected one of 8 possible tokens
// =================================
fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_2/translated_rust.rs
 * ------------------------------------------------------------
fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    use ::core::ffi::c_void;
    use ::libc;
    let mdInfo = mbedtls_md_info_from_type(alg as mbedtls_md_type_t);
    if mdInfo.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdInfo is null\0".as_ptr() as *const i8, __FUNCTION__!(), 46);
        return V_ERR as i32;
    }
    let mut pos: i32 = 0;
    let mut rawBufLen: i32 = blockLen;
    let mdCtx = libc::malloc(::core::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t;
    if mdCtx.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__!(), 50);
        return V_ERR as i32;
    }
    let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg: %d wholelen: %d\0".as_ptr() as *const i8, __FUNCTION__!(), 51, alg, rawBufLen);
    while rawBufLen > 0 {
        unsafe { mbedtls_md_init(mdCtx); }
        let readLen = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
        let ret = unsafe { mbedtls_md_setup(mdCtx, mdInfo, 0) };
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 56);
            goto EXIT;
        }
        let hlen = unsafe { mbedtls_md_get_size(mdInfo) } as usize;
        if hlen == 0 || hlen > 64 {
            goto EXIT;
        }
        let ret = unsafe { mbedtls_md_starts(mdCtx) };
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 62);
            goto EXIT;
        }
        let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        if unsafe { memcpy_s(chunkContentPrefix.as_mut_ptr().offset(1) as *mut c_void, 4, &readLen as *const i32 as *const c_void, ::core::mem::size_of::<i32>()) } != 0 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: memcpy_s fail\0".as_ptr() as *const i8, __FUNCTION__!(), 65);
            goto EXIT;
        }
        let ret = unsafe { mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5) };
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 69);
            goto EXIT;
        }
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 70, rawBufLen, pos);
        let ret = unsafe { mbedtls_md_update(mdCtx, block.offset(pos as isize) as *const u8, readLen as size_t) };
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 72);
            goto EXIT;
        }
        rawBufLen -= readLen;
        pos += readLen;
        let outbuf = libc::malloc(hlen) as *mut u8;
        if outbuf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 76);
            goto EXIT;
        }
        let ret = unsafe { mbedtls_md_finish(mdCtx, outbuf) };
        unsafe { HapPutData(result, *offset, outbuf, hlen as i32); }
        unsafe { *offset += hlen as i32; }
        let _ = unsafe { memset_s(outbuf as *mut c_void, hlen, 0, hlen) };
        if !outbuf.is_null() {
            libc::free(outbuf as *mut c_void);
        }
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 82);
            goto EXIT;
        }
        unsafe { mbedtls_md_free(mdCtx); }
    }
    if !mdCtx.is_null() {
        libc::free(mdCtx as *mut c_void);
    }
    return V_OK as i32;
    EXIT:
    unsafe { mbedtls_md_free(mdCtx); }
    if !mdCtx.is_null() {
        libc::free(mdCtx as *mut c_void);
    }
    return V_ERR as i32;
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_hap_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetChunkSumCount(fileSize: i32, coreDirectorySize: i32, eocdSize: i32, rootHashLen: i32) -> i32 {
    let chunkSize: i32 = 1024 * 1024;
    let maxSize: i32 = 0x7fffffff - chunkSize;
    if fileSize > maxSize || coreDirectorySize > maxSize || eocdSize > maxSize {
        return 0;
    }
    let count: i32 = ((fileSize - 1 + chunkSize) / chunkSize) + ((coreDirectorySize - 1 + chunkSize) / chunkSize) +
        ((eocdSize - 1 + chunkSize) / chunkSize);
    if rootHashLen < 0 || (((0x7fffffff - 5) / count) < rootHashLen) {
        unsafe {
            let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: overflow count: %d, chunkDigestLen: %d\0".as_ptr() as *const i8, b"GetChunkSumCount\0".as_ptr() as *const i8, 103, count, rootHashLen);
        }
        return 0;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(3, 4, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get sum count %d\0".as_ptr() as *const i8, b"GetChunkSumCount\0".as_ptr() as *const i8, 106, count);
    }
    return count;
}

fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rootHashLen %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 117, rootHashLen);
    }
    if rootHashLen <= 0 || rootHashLen > 64 {
        return rst;
    }
    let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mdInfo is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 122);
        }
        return rst;
    }
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t };
    if mdCtx.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mdCtx is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 124);
        }
        return rst;
    }
    unsafe { crate::compat::mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead { type_: 0, length: 0, offset: 0 };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 130);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_starts(mdCtx) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 132);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    let mut readLen = unsafe { (*chunkDigest).len };
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: readLen %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 134, readLen);
    }
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, (*chunkDigest).buffer as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 136);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
    if rawBuf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rawBuf is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 139);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    readLen = rawLen;
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: signBuf %0x %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 141, *rawBuf as i32, readLen);
    }
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 143);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
            libc::free(rawBuf as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    outbuf = unsafe { libc::malloc(rootHashLen as usize) as *mut ::core::ffi::c_uchar };
    if outbuf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: outbuf is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 145);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
            libc::free(rawBuf as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 147);
            crate::compat::mbedtls_md_free(mdCtx);
            libc::free(mdCtx as *mut ::core::ffi::c_void);
            libc::free(rawBuf as *mut ::core::ffi::c_void);
            libc::free(outbuf as *mut ::core::ffi::c_void);
        }
        return rst;
    }
    crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf, rootHashLen);
    unsafe {
        let _ = crate::compat::memset_s(outbuf as *mut ::core::ffi::c_void, rootHashLen as crate::types::size_t, 0, rootHashLen as crate::types::size_t);
        crate::compat::mbedtls_md_free(mdCtx);
        libc::free(mdCtx as *mut ::core::ffi::c_void);
        if !rawBuf.is_null() {
            libc::free(rawBuf as *mut ::core::ffi::c_void);
        }
        if !outbuf.is_null() {
            libc::free(outbuf as *mut ::core::ffi::c_void);
        }
    }
    rst = 0;
    return rst;
}

fn HapUpdateDigistHead(digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut crate::types::size_t) -> i32 {
    unsafe {
        crate::compat::mbedtls_md_init(mdCtx);
        let mut ret = crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        *hlen = crate::compat::mbedtls_md_get_size(mdInfo) as crate::types::size_t;
        if *hlen == 0 || *hlen > 64 {
            return crate::types::V_ERR as i32;
        }
        ret = crate::compat::mbedtls_md_starts(mdCtx);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        if crate::compat::memcpy_s(
            (chunkContentPrefix.as_mut_ptr()).offset(1) as *mut core::ffi::c_void,
            (5 - 1) as crate::types::size_t,
            &readLen as *const i32 as *const core::ffi::c_void,
            std::mem::size_of::<i32>() as crate::types::size_t,
        ) != 0
        {
            return crate::types::V_ERR as i32;
        }
        ret = crate::compat::mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5 as crate::types::size_t);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        crate::types::V_OK as i32
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_6
// c_function: UpdateSmallBlock
// rust_file: src_app_verify_hap.rs
// rust_signature: fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32
// c_first_line: static int32_t UpdateSmallBlock(int32_t readLen, const int32_t fp, mbedtls_md_context_t *mdCtx)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_6/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:149:17
//       |
//       |                 ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:128:17
//       |
//       |                 ^^^^^^^^^^^^
// =================================
fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_6
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_6/translated_rust.rs
 * ------------------------------------------------------------
fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead = if readLenLeft > crate::types::ONCE_READ_LEN as i32 {
            crate::types::ONCE_READ_LEN as i32
        } else {
            readLenLeft
        };
        let onceBuf = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: onceBuf is null\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                193,
            );
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(*fp as i32, onceBuf as *mut core::ffi::c_void, onceRead as usize) };
        if len != onceRead as isize {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                196,
                len as i32,
                onceRead,
            );
            if !onceBuf.is_null() {
                unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, onceBuf as *const u8, onceRead as crate::types::size_t) };
        if !onceBuf.is_null() {
            unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
        }
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: ret not ok\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                202,
            );
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_hap_6
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_7
// c_function: ComputerFileHash
// rust_file: src_app_verify_hap.rs
// rust_signature: fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32
// c_first_line: static int32_t ComputerFileHash(const SignatureInfo *signInfo, int32_t digestAlgorithm, const int32_t fp,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_7/translated_rust.rs
// last_error_truncated:
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//      --> src/src_app_verify_hap.rs:325:22
//       |
//       |                      ^^^^ expected one of 8 possible tokens
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//      --> src/src_app_verify_hap.rs:333:22
//       |
//       |                      ^^^^ expected one of 8 possible tokens
// =================================
fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_7
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_7/translated_rust.rs
 * ------------------------------------------------------------
fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::{mbedtls_md_context_t, mbedtls_md_info_t, size_t};
    use crate::compat::*;
    use crate::src_app_verify_hap::{HapUpdateDigistHead, UpdateSmallBlock};
    use crate::src_app_centraldirectory::HapPutData;
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t;
        if mdCtx.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__!(), 212);
            return V_ERR;
        }
        libc::lseek(fp as i32, 0, SEEK_SET as i32);
        let mut pos = 0;
        let mut rawBufLen = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: size_t = 0;
            let readLen = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mdInfo = mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                libc::free(mdCtx as *mut libc::c_void);
                return V_ERR;
            }
            let mut ret = HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut size_t);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 225);
                goto EXIT;
            }
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 226, rawBufLen, pos);
            ret = UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 228);
                goto EXIT;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut libc::c_uchar;
            if outbuf.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 232);
                goto EXIT;
            }
            ret = mbedtls_md_finish(mdCtx, outbuf);
            HapPutData(chunkDigest, *offset, outbuf as *const libc::c_uchar, hlen as i32);
            memset_s(outbuf as *mut libc::c_void, hlen as usize, 0, hlen as usize);
            *offset += hlen as i32;
            libc::free(outbuf as *mut libc::c_void);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 238);
                goto EXIT;
            }
            mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut libc::c_void);
        return V_OK;
        EXIT:
        mbedtls_md_free(mdCtx);
        libc::free(mdCtx as *mut libc::c_void);
        return V_ERR;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_hap_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_8
// c_function: ComputerCoreDirHash
// rust_file: src_app_verify_hap.rs
// rust_signature: fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32
// c_first_line: static int32_t ComputerCoreDirHash(const SignatureInfo *signInfo, int32_t digestAlgorithm, const int32_t fp,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_8/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:422:180
//       |
//       |                                                                                                                                                                              ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:410:187
//       |
//       |                                                                                                                                                                                     ^^^^^^^^^^^^
// =================================
fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_8/translated_rust.rs
 * ------------------------------------------------------------
fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        let centralDirSize = (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset;
        if centralDirSize <= 0 {
            return V_ERR as i32;
        }
        let dirBuf = libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char;
        if dirBuf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: \"dirBuf\" is null\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 257);
            return V_ERR as i32;
        }
        let _ = libc::lseek(fp as libc::c_int, (*signInfo).hapCoreDirOffset as libc::off_t, SEEK_SET as libc::c_int);
        let len = libc::read(fp as libc::c_int, dirBuf as *mut libc::c_void, centralDirSize as usize) as i32;
        if len != centralDirSize {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: \"fread err: %d, %d\"\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 261, len, centralDirSize);
            if !dirBuf.is_null() {
                libc::free(dirBuf as *mut libc::c_void);
            }
            return V_ERR as i32;
        }
        let ret = crate::src_app_verify_hap::ComputeBlockHash(dirBuf as *const std::ffi::c_char, centralDirSize, digestAlgorithm, chunkDigest, offset);
        let _ = memset_s(dirBuf as *mut libc::c_void, centralDirSize as usize, 0, centralDirSize as usize);
        if !dirBuf.is_null() {
            libc::free(dirBuf as *mut libc::c_void);
        }
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 268);
            return ret;
        }
        return V_OK as i32;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_hap_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_9
// c_function: ComputerEocdHash
// rust_file: src_app_verify_hap.rs
// rust_signature: fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32
// c_first_line: static int32_t ComputerEocdHash(const SignatureInfo *signInfo, int32_t digestAlgorithm, const int32_t fp,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_9/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:487:148
//       |
//       |                                                                                                                                                    ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:474:155
//       |
//       |                                                                                                                                                     ^^^^^^^^^^^^
// =================================
fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_9/translated_rust.rs
 * ------------------------------------------------------------
fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        if (*signInfo).hapEocdSize <= 0 {
            return V_ERR as i32;
        }
        let eocdBuf = libc::malloc((*signInfo).hapEocdSize as usize) as *mut crate::types::HapEocd;
        if eocdBuf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: eocdBuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 279);
            return V_ERR as i32;
        }
        let _ = libc::lseek(fp as i32, (*signInfo).hapEocdOffset as libc::off_t, SEEK_SET as i32);
        let len = libc::read(fp as i32, eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize) as i32;
        if len != (*signInfo).hapEocdSize {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 283, len, (*signInfo).hapEocdSize);
            if !eocdBuf.is_null() {
                libc::free(eocdBuf as *mut libc::c_void);
            }
            return V_ERR as i32;
        }
        HapPutInt32((&mut (*eocdBuf).eocdHead.coreDirOffset) as *mut i32 as *mut u8, std::mem::size_of::<i32>() as i32, (*signInfo).fullSignBlockOffset);
        let ret = crate::src_app_verify_hap::ComputeBlockHash(eocdBuf as *const i8, len, digestAlgorithm, chunkDigest, offset);
        let _ = memset_s(eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize, 0, (*signInfo).hapEocdSize as usize);
        if !eocdBuf.is_null() {
            libc::free(eocdBuf as *mut libc::c_void);
        }
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 291);
            return ret;
        }
        return V_OK as i32;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_hap_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn VerifyIntegrityChunk(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, actualDigest: *const crate::types::HapBuf) -> bool {
    if signInfo.is_null() || actualDigest.is_null() {
        return false;
    }
    let actual_digest = unsafe { &*actualDigest };
    if actual_digest.buffer.is_null() {
        return false;
    }
    let sign_info = unsafe { &*signInfo };
    let central_dir_size = sign_info.hapEocdOffset - sign_info.hapCoreDirOffset;
    let root_hash_len = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if root_hash_len < 0 {
        return false;
    }
    let sum_count = crate::src_app_verify_hap::GetChunkSumCount(
        sign_info.fullSignBlockOffset,
        central_dir_size,
        sign_info.hapEocdSize,
        root_hash_len,
    );
    if sum_count == 0 {
        return false;
    }
    let sum_of_chunks_len = 5 + sum_count * root_hash_len;
    let mut chunk_digest = crate::types::HapBuf {
        buffer: std::ptr::null_mut(),
        len: 0,
    };
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut chunk_digest, sum_of_chunks_len) {
        return false;
    }
    crate::src_app_centraldirectory::HapPutByte(&chunk_digest, 0, 0x5a as ::core::ffi::c_char);
    crate::src_app_centraldirectory::HapSetInt32(&chunk_digest, 1, sum_count);
    let mut offset = 5;
    let mut ret = crate::src_app_verify_hap::ComputerFileHash(signInfo, digestAlgorithm, fp, &chunk_digest, &mut offset);
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputerCoreDirHash(signInfo, digestAlgorithm, fp, &chunk_digest, &mut offset);
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputerEocdHash(signInfo, digestAlgorithm, fp, &chunk_digest, &mut offset);
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest);
        return false;
    }
    ret = crate::src_app_verify_hap::ComputeDigestsWithOptionalBlock(digestAlgorithm, fp, signInfo, &chunk_digest, actualDigest);
    if ret != 0 {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest);
        return false;
    }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut chunk_digest);
    true
}
