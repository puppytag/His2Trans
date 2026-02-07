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
    match signAlgorithm & crate::types::ALGORITHM_MASK {
        crate::types::ALGORITHM_SHA256 | crate::types::ALGORITHM_PKCS1_SHA256 => crate::types::MBEDTLS_MD_SHA256 as i32,
        crate::types::ALGORITHM_SHA384 | crate::types::ALGORITHM_PKCS1_SHA384 => crate::types::MBEDTLS_MD_SHA384 as i32,
        crate::types::ALGORITHM_SHA512 | crate::types::ALGORITHM_PKCS1_SHA512 => crate::types::MBEDTLS_MD_SHA512 as i32,
        _ => {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: signAlgorithm: %u error\0".as_ptr() as *const _,
                    b"GetDigestAlgorithmId\0".as_ptr() as *const _,
                    38,
                    signAlgorithm,
                );
            }
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_2/translated_rust.rs
// last_error_truncated:
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//     --> src/src_app_verify_hap.rs:63:22
//      |
//      |                      ^^^^ expected one of 8 possible tokens
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `EXIT`
//     --> src/src_app_verify_hap.rs:67:22
//      |
//      |                      ^^^^ expected one of 8 possible tokens
// =================================
fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_2/translated_rust.rs
 * ------------------------------------------------------------
fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::{mbedtls_md_context_t, mbedtls_md_info_t, mbedtls_md_type_t, size_t, V_OK, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO};
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    unsafe {
        let mdInfo = mbedtls_md_info_from_type(alg as mbedtls_md_type_t);
        if mdInfo.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdInfo is null\0".as_ptr() as *const i8, __FUNCTION__, 46);
            return V_ERR;
        }
        let mut pos = 0;
        let mut rawBufLen = blockLen;
        let mdCtx = libc::malloc(::core::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t;
        if mdCtx.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__, 50);
            return V_ERR;
        }
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg: %d wholelen: %d\0".as_ptr() as *const i8, __FUNCTION__, 51, alg, rawBufLen);
        while rawBufLen > 0 {
            mbedtls_md_init(mdCtx);
            let readLen = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mut ret = mbedtls_md_setup(mdCtx, mdInfo, 0);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 56);
                goto EXIT;
            }
            let hlen = mbedtls_md_get_size(mdInfo) as usize;
            if hlen == 0 || hlen > 64 {
                goto EXIT;
            }
            ret = mbedtls_md_starts(mdCtx);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 62);
                goto EXIT;
            }
            let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
            if memcpy_s((chunkContentPrefix.as_mut_ptr() as *mut c_void).offset(1) as *mut c_void, 4, (&readLen as *const i32) as *const c_void, ::core::mem::size_of::<i32>()) != 0 {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: memcpy_s fail\0".as_ptr() as *const i8, __FUNCTION__, 65);
                goto EXIT;
            }
            ret = mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 69);
                goto EXIT;
            }
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__, 70, rawBufLen, pos);
            ret = mbedtls_md_update(mdCtx, block.offset(pos as isize) as *const u8, readLen as size_t);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 72);
                goto EXIT;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen) as *mut u8;
            if outbuf.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__, 76);
                goto EXIT;
            }
            ret = mbedtls_md_finish(mdCtx, outbuf);
            HapPutData(result, *offset, outbuf, hlen as i32);
            *offset += hlen as i32;
            let _ = memset_s(outbuf as *mut c_void, hlen, 0, hlen) as i32 as i32 as i32 as i32 as i32;
            libc::free(outbuf as *mut c_void);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 82);
                goto EXIT;
            }
            mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut c_void);
        return V_OK;
        EXIT:
        mbedtls_md_free(mdCtx);
        libc::free(mdCtx as *mut c_void);
        return V_ERR;
    }
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
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: overflow count: %d, chunkDigestLen: %d\0".as_ptr() as *const i8,
                "GetChunkSumCount\0".as_ptr() as *const i8,
                103,
                count,
                rootHashLen,
            );
        }
        return 0;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: get sum count %d\0".as_ptr() as *const i8,
            "GetChunkSumCount\0".as_ptr() as *const i8,
            106,
            count,
        );
    }
    count
}

fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if rootHashLen <= 0 || rootHashLen > 64 {
        return rst;
    }
    let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        return rst;
    }
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t };
    if mdCtx.is_null() {
        return rst;
    }
    unsafe { crate::compat::mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead { type_: 0, length: 0, offset: 0 };
    if ret != 0 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_starts(mdCtx) };
    if ret != 0 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    let readLen = unsafe { (*chunkDigest).len };
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, (*chunkDigest).buffer as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != 0 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
    if rawBuf.is_null() {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    let readLen = rawLen;
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != 0 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); }
        return rst;
    }
    outbuf = unsafe { libc::malloc(rootHashLen as usize) as *mut ::core::ffi::c_uchar };
    if outbuf.is_null() {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
    if ret != 0 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); libc::free(outbuf as *mut _); }
        return rst;
    }
    unsafe { crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf, rootHashLen) };
    unsafe { crate::compat::memset_s(outbuf as *mut _, rootHashLen as crate::types::size_t, 0, rootHashLen as crate::types::size_t) };
    rst = crate::types::V_OK as i32;
    unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut _); }
    }
    if !rawBuf.is_null() {
        unsafe { libc::free(rawBuf as *mut _); }
    }
    if !outbuf.is_null() {
        unsafe { libc::free(outbuf as *mut _); }
    }
    rst
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
        let mut chunk_content_prefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        if crate::compat::memcpy_s(
            (chunk_content_prefix.as_mut_ptr()).offset(1) as *mut core::ffi::c_void,
            (5 - 1) as crate::types::size_t,
            &readLen as *const i32 as *const core::ffi::c_void,
            std::mem::size_of::<i32>() as crate::types::size_t,
        ) != 0
        {
            return crate::types::V_ERR as i32;
        }
        ret = crate::compat::mbedtls_md_update(mdCtx, chunk_content_prefix.as_ptr(), 5 as crate::types::size_t);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        crate::types::V_OK as i32
    }
}

fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead = if readLenLeft > 65536 { 65536 } else { readLenLeft };
        let onceBuf = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: onceBuf is null\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    193,
                );
            }
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(*fp as i32, onceBuf as *mut core::ffi::c_void, onceRead as usize) };
        if len != onceRead as isize {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    196,
                    len as i32,
                    onceRead,
                );
                libc::free(onceBuf as *mut core::ffi::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe {
            crate::compat::mbedtls_md_update(
                mdCtx,
                onceBuf as *const u8,
                onceRead as crate::types::size_t,
            )
        };
        unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
        if ret != crate::types::V_OK as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: ret not ok\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    202,
                );
            }
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}

fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp as i32, 0, crate::types::SEEK_SET as i32);
        let mut rawBufLen = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: crate::types::size_t = 0;
            let readLen = if rawBufLen > crate::types::HASH_BLOB_LEN as i32 {
                crate::types::HASH_BLOB_LEN as i32
            } else {
                rawBufLen
            };
            let mdInfo = crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut crate::types::size_t);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            rawBufLen -= readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut libc::c_uchar;
            if outbuf.is_null() {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::compat::mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(chunkDigest, *offset, outbuf, hlen as i32);
            crate::compat::memset_s(outbuf as *mut libc::c_void, hlen as u32, 0, hlen as u32);
            *offset += hlen as i32;
            libc::free(outbuf as *mut libc::c_void);
            if ret != 0 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            crate::compat::mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut libc::c_void);
        return crate::types::V_OK as i32;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_hap_8
// c_function: ComputerCoreDirHash
// rust_file: src_app_verify_hap.rs
// rust_signature: fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32
// c_first_line: static int32_t ComputerCoreDirHash(const SignatureInfo *signInfo, int32_t digestAlgorithm, const int32_t fp,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_8/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:370:181
//       |
//       |                                                            ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:358:188
//       |
//       |                                                            ^^^^^^^^^^^^
// =================================
fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_8/translated_rust.rs
 * ------------------------------------------------------------
fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    let centralDirSize = unsafe { (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset };
    if centralDirSize <= 0 {
        return V_ERR as i32;
    }
    let dirBuf = unsafe { libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char };
    if dirBuf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: dirBuf is null\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 257) };
        return V_ERR as i32;
    }
    let _ = unsafe { libc::lseek(fp as libc::c_int, (*signInfo).hapCoreDirOffset as libc::off_t, SEEK_SET as libc::c_int) };
    let len = unsafe { libc::read(fp as libc::c_int, dirBuf as *mut libc::c_void, centralDirSize as usize) };
    if len != centralDirSize as isize {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: fread err: %d, %d\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 261, len as i32, centralDirSize) };
        if !dirBuf.is_null() {
            unsafe { libc::free(dirBuf as *mut libc::c_void); }
        }
        return V_ERR as i32;
    }
    let ret = crate::src_app_verify_hap::ComputeBlockHash(dirBuf as *const std::ffi::c_char, centralDirSize, digestAlgorithm, chunkDigest, offset);
    let _ = unsafe { memset_s(dirBuf as *mut libc::c_void, centralDirSize as usize, 0, centralDirSize as usize) };
    if !dirBuf.is_null() {
        unsafe { libc::free(dirBuf as *mut libc::c_void); }
    }
    if ret != V_OK as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 268) };
        return ret;
    }
    V_OK as i32
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_9/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:434:148
//       |
//       |                                                            ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify_hap.rs:421:155
//       |
//       |                                                            ^^^^^^^^^^^^
// =================================
fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_hap_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_hap_9/translated_rust.rs
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
        crate::src_app_common::HapPutInt32((&mut (*eocdBuf).eocdHead.coreDirOffset) as *mut i32 as *mut u8, std::mem::size_of::<i32>() as i32, (*signInfo).fullSignBlockOffset);
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
    let mut ret;
    ret = crate::src_app_verify_hap::ComputerFileHash(signInfo, digestAlgorithm, fp, &chunk_digest, &mut offset);
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
