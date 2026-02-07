//! Module: src_app_verify
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
/// C: static _Bool g_isActsMode
static mut g_isActsMode: bool = false;

/// C: static _Bool g_isDebugMode
static mut g_isDebugMode: bool = false;

/// C: static const TrustAppCert[3] g_trustAppList
static mut g_trustAppList: [crate::types::TrustAppCert; 3usize] = unsafe { core::mem::MaybeUninit::<[crate::types::TrustAppCert; 3usize]>::zeroed().assume_init() };

/// C: static const TrustAppCert[2] g_trustAppListTest
static mut g_trustAppListTest: [crate::types::TrustAppCert; 2usize] = unsafe { core::mem::MaybeUninit::<[crate::types::TrustAppCert; 2usize]>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    unsafe {
        let block_num_ptr = (signHead as *mut u8).offset(std::mem::offset_of!(crate::types::HwSignHead, blockNum) as isize) as *mut ::core::ffi::c_uchar;
        (*signHead).blockNum = crate::src_app_common::HapGetInt(block_num_ptr, std::mem::size_of::<u32>() as i32) as u32;
        let size_ptr = (signHead as *mut u8).offset(std::mem::offset_of!(crate::types::HwSignHead, size) as isize) as *mut ::core::ffi::c_uchar;
        (*signHead).size = crate::src_app_common::HapGetInt64(size_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let magic_low_ptr = (signHead as *mut u8).offset(std::mem::offset_of!(crate::types::HwSignHead, magicLow) as isize) as *mut ::core::ffi::c_uchar;
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(magic_low_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let magic_high_ptr = (signHead as *mut u8).offset(std::mem::offset_of!(crate::types::HwSignHead, magicHigh) as isize) as *mut ::core::ffi::c_uchar;
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(magic_high_ptr, std::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
        let version_ptr = (signHead as *mut u8).offset(std::mem::offset_of!(crate::types::HwSignHead, version) as isize) as *mut ::core::ffi::c_uchar;
        (*signHead).version = crate::src_app_common::HapGetInt(version_ptr, std::mem::size_of::<u32>() as i32) as u32;
    }
}

fn BlockHeadN2H(blockHead: *mut crate::types::BlockHead) {
    unsafe {
        (*blockHead).type_ = crate::src_app_common::HapGetUnsignedInt(
            &(*blockHead).type_ as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        );
        (*blockHead).length = crate::src_app_common::HapGetUnsignedInt(
            &(*blockHead).length as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        );
        (*blockHead).offset = crate::src_app_common::HapGetUnsignedInt(
            &(*blockHead).offset as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        );
    }
}

fn ContentN2H(content: *mut crate::types::ContentInfo) {
    unsafe {
        (*content).blockNum = crate::src_app_common::HapGetInt(
            &(*content).blockNum as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).size = crate::src_app_common::HapGetInt(
            &(*content).size as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).algId = crate::src_app_common::HapGetInt(
            &(*content).algId as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
        (*content).length = crate::src_app_common::HapGetInt(
            &(*content).length as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_4
// c_function: GetSignHead
// rust_file: src_app_verify.rs
// rust_signature: fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32
// c_first_line: static int32_t GetSignHead(const FileRead *file, SignatureInfo *signInfo)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_4/translated_rust.rs
// last_error_truncated:
//   error[E0603]: struct import `timespec` is private
//       --> src/src_app_verify.rs:108:33
//        |
//        |         -------                 ^^^^^^^^ private struct import
//        |         |
//        |         while setting this field
//        |
//       --> src/compat.rs:36:27
// =================================
fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_4
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_4/translated_rust.rs
 * ------------------------------------------------------------
fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::{HAP_SIG_BLOCK_MAGIC_HI, HAP_SIG_BLOCK_MAGIC_HI_OLD, HAP_SIG_BLOCK_MAGIC_LO, HAP_SIG_BLOCK_MAGIC_LO_OLD, VERSION_FOR_NEW_MAGIC_NUM, V_OK, V_ERR_GET_SIGNHEAD, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_SET};
    use crate::compat::*;
    use crate::globals::*;
    use ::libc::{fstat, lseek, read, free, malloc};
    use ::core::mem::size_of;
    let mut file_st = crate::compat::stat {
        st_dev: 0,
        __st_dev_padding: 0,
        __st_ino_truncated: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __st_rdev_padding: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        __st_atim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        __st_mtim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        __st_ctim32: crate::compat::stat__bindgen_ty_1 { tv_sec: 0, tv_nsec: 0 },
        st_ino: 0,
        st_atim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: crate::compat::timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let fp = unsafe { (*file).fp };
    let mut ret = unsafe { fstat(fp, &mut file_st as *mut _ as *mut ::libc::stat) };
    if ret != 0 || file_st.st_size < size_of::<crate::types::HwSignHead>() as i64 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, file_st.st_size as i32) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find signature error\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 125) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let hap_core_dir_offset = unsafe { (*signInfo).hapCoreDirOffset };
    if hap_core_dir_offset < size_of::<crate::types::HwSignHead>() as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 129, hap_core_dir_offset) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    ret = unsafe { lseek(fp, hap_core_dir_offset as i64 - size_of::<crate::types::HwSignHead>() as i64, SEEK_SET as i32) } as i32;
    if ret < 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 134, ret) };
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let sign_head = unsafe { malloc(size_of::<crate::types::HwSignHead>() as usize) as *mut crate::types::HwSignHead };
    if sign_head.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: signHead is null\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 138) };
        return V_ERR as i32;
    }
    let read_len = unsafe { read(fp, sign_head as *mut ::core::ffi::c_void, size_of::<crate::types::HwSignHead>() as usize) } as i32;
    if read_len != size_of::<crate::types::HwSignHead>() as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 141, read_len, size_of::<crate::types::HwSignHead>() as i32) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    crate::src_app_verify::SignHeadN2H(sign_head);
    let mut magic_low = HAP_SIG_BLOCK_MAGIC_LO;
    let mut magic_high = HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*sign_head).version } < VERSION_FOR_NEW_MAGIC_NUM {
        magic_low = HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magic_high = HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*sign_head).magicLow } != magic_low || unsafe { (*sign_head).magicHigh } != magic_high {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 153) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let _ = unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 157, unsafe { (*sign_head).size }, unsafe { (*sign_head).blockNum }) };
    unsafe { (*signInfo).signHead = sign_head; }
    unsafe { (*signInfo).fullSignBlockOffset = hap_core_dir_offset - (*sign_head).size as i32; }
    unsafe { (*signInfo).fileSize = file_st.st_size as i32; }
    let full_sign_block_offset = unsafe { (*signInfo).fullSignBlockOffset };
    if full_sign_block_offset <= 0 || full_sign_block_offset >= hap_core_dir_offset {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 162) };
        unsafe { free(sign_head as *mut ::core::ffi::c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_4
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    use crate::types::{MAX_BLOCK_NUM, V_ERR, V_OK, SEEK_SET};
    unsafe {
        let signH = (*signInfo).signHead;
        if signH.is_null() {
            return V_ERR as i32;
        }
        let _ = libc::lseek(fp, (*signInfo).fullSignBlockOffset as libc::off_t, SEEK_SET as i32);
        let mut num = (*signH).blockNum as i32;
        if num > MAX_BLOCK_NUM as i32 {
            return V_ERR as i32;
        }
        while num > 0 {
            num -= 1;
            let read_len = libc::read(fp, block as *mut libc::c_void, std::mem::size_of::<crate::types::BlockHead>() as libc::size_t) as i32;
            if read_len != std::mem::size_of::<crate::types::BlockHead>() as i32 {
                return V_ERR as i32;
            }
            let type_val = crate::src_app_common::HapGetInt((&(*block).type_) as *const u32 as *const libc::c_uchar, std::mem::size_of::<u32>() as i32);
            if type_val == blockType {
                crate::src_app_verify::BlockHeadN2H(block);
                return V_OK as i32;
            }
        }
        V_ERR as i32
    }
}

pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    if signInfo.is_null() || blockHead.is_null() {
        return std::ptr::null_mut();
    }
    let ret = crate::src_app_verify::FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find block head error\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 203);
        }
        return std::ptr::null_mut();
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: type: %u, len: %u, offset: %u signoffset: %d\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 207, (*blockHead).type_, (*blockHead).length, (*blockHead).offset, (*signInfo).fullSignBlockOffset);
        if (*blockHead).length == 0 || (*blockHead).length > ((*signInfo).hapCoreDirOffset - (*signInfo).fullSignBlockOffset) as u32 {
            return std::ptr::null_mut();
        }
        if (*blockHead).length as i32 + 1 >= (*signInfo).fileSize {
            return std::ptr::null_mut();
        }
        let buf = libc::malloc(((*blockHead).length + 1) as usize) as *mut i8;
        if buf.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 222);
            return std::ptr::null_mut();
        }
        *buf.offset((*blockHead).length as isize) = 0;
        let mut fileSt: libc::stat = std::mem::zeroed();
        let ret = libc::fstat(fp, &mut fileSt);
        if ret != 0 || fileSt.st_size < (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64 + (*blockHead).length as i64 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 229, ret, fileSt.st_size as i32);
            libc::free(buf as *mut std::ffi::c_void);
            return std::ptr::null_mut();
        }
        libc::lseek(fp, (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64, 0);
        let readLen = libc::read(fp, buf as *mut std::ffi::c_void, (*blockHead).length as usize) as i32;
        if readLen != (*blockHead).length as i32 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: read error: %d, %d\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 236, readLen, (*blockHead).length);
            libc::free(buf as *mut std::ffi::c_void);
            return std::ptr::null_mut();
        }
        if !len.is_null() {
            *len = readLen;
        }
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: buf begin\0".as_ptr() as *const i8, b"GetSignBlockByType\0".as_ptr() as *const i8, 241);
        buf
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_7
// c_function: GetHashUnitLen
// rust_file: src_app_verify.rs
// rust_signature: pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32
// c_first_line: int32_t GetHashUnitLen(int32_t hashAlg)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_7/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//     --> src/src_app_verify.rs:63:9
//      |
//      |         ^^^^^^^^^^^^
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error
// =================================
pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_7
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: algId: %d\0".as_ptr() as *const i8,
        __FUNCTION__!(),
        247,
        hashAlg,
    );
    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t) };
    unsafe { crate::compat::mbedtls_md_get_size(md_info) as i32 }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn CalcCmpContHash(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut ::core::ffi::c_uchar, hashLen: *mut ::core::ffi::c_uint) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;

    rc = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rc not ok\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 260);
        }
        return rc;
    }

    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(algType) };
    rc = unsafe { crate::compat::mbedtls_md(md_info, input, inputLen as u32, hash) };
    if rc != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: calc digest failed\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 264);
        }
        return rc;
    }
    unsafe {
        *hashLen = crate::compat::mbedtls_md_get_size(md_info) as ::core::ffi::c_uint;
    }

    let mut digInAttr: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut digInAttrLen: crate::types::size_t = 0;
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetDigestInSignerAuthAttr(signer, &mut digInAttr, &mut digInAttrLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: PKCS7_GetDigestInSignerAuthAttr error: %d\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 274, rc);
        }
        return rc;
    }
    unsafe {
        if digInAttrLen != *hashLen as crate::types::size_t {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: content hash len is not equal with attr's hash len\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 278);
            return crate::types::V_ERR as i32;
        }
        if crate::compat::memcmp(hash as *const _, digInAttr as *const _, digInAttrLen as u32) != 0 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 282);
            return crate::types::V_ERR as i32;
        }
    }
    crate::types::V_OK as i32
}

fn CalcDigest(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut ::core::ffi::c_uchar, hashLen: *mut ::core::ffi::c_uint) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    rc = crate::src_app_verify::CalcCmpContHash(pkcs7, signer, algType, hash, hashLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const _, b"CalcDigest\0".as_ptr() as *const _, 296);
        }
        return rc;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: signer context hash equal with attr hash\0".as_ptr() as *const _, b"CalcDigest\0".as_ptr() as *const _, 299);
    }
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerAuthAttr(signer, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: PKCS7_GetSignerAuthAttr failed ret: %d\0".as_ptr() as *const _, b"CalcDigest\0".as_ptr() as *const _, 304, rc);
        }
        return rc;
    }
    unsafe {
        rc = crate::compat::mbedtls_md(crate::compat::mbedtls_md_info_from_type(algType), input, inputLen as u32, hash);
    }
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: calc digest failed ret: %d\0".as_ptr() as *const _, b"CalcDigest\0".as_ptr() as *const _, 309, rc);
        }
        return rc;
    }
    unsafe {
        *hashLen = crate::compat::mbedtls_md_get_size(crate::compat::mbedtls_md_info_from_type(algType)) as u32;
    }
    crate::types::V_OK as i32
}

fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut inputLen);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: get content info error: %d\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 324, ret);
        }
        return ret;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: len: %d\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 327, inputLen as i32);
    }
    let content = unsafe { libc::malloc(std::mem::size_of::<crate::types::ContentInfo>()) } as *mut crate::types::ContentInfo;
    if content.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content is null\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 330);
        }
        return crate::types::V_ERR as i32;
    }
    ret = unsafe { crate::compat::memcpy_s(content as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::ContentInfo>() as u32, input as *const ::core::ffi::c_void, inputLen as u32) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mem cpy error, ret: %d\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 334, ret);
            libc::free(content as *mut ::core::ffi::c_void);
        }
        return ret;
    }
    crate::src_app_verify::ContentN2H(content);
    unsafe {
        (*content).algId = crate::src_app_verify_hap::GetDigestAlgorithmId((*content).algId as u32);
    }
    let algId = unsafe { (*content).algId };
    if algId != crate::types::MBEDTLS_MD_SHA256 as i32 && algId != crate::types::MBEDTLS_MD_SHA384 as i32 && algId != crate::types::MBEDTLS_MD_SHA512 as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: hash alg invalid\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 341);
            libc::free(content as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR as i32;
    }
    let mut actualDigest = crate::types::HapBuf {
        buffer: std::ptr::null_mut(),
        len: 0,
    };
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(algId);
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actualDigest, rootHashLen) {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: create buf fail\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 348);
            libc::free(content as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR as i32;
    }
    let fp = unsafe { (*fileRead).fp };
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(algId, fp, signInfo, &actualDigest) {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: get raw hash failed\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 353);
        }
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        unsafe {
            libc::free(content as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR as i32;
    }
    let content_len = unsafe { (*content).length };
    let content_hash_ptr = unsafe { (*content).hash.as_ptr() as *const ::core::ffi::c_void };
    if actualDigest.len != content_len || unsafe { libc::memcmp(actualDigest.buffer, content_hash_ptr, actualDigest.len as usize) } != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: hash diff\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 359);
            libc::free(content as *mut ::core::ffi::c_void);
        }
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        return crate::types::V_ERR_GET_HASH_DIFF as i32;
    }
    unsafe {
        libc::free(content as *mut ::core::ffi::c_void);
    }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
    crate::types::V_OK as i32
}

fn GetCertTypeBySourceName(cert: *const crate::types::TrustAppCert) -> i32 {
    if cert.is_null() {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
    let name = unsafe { (*cert).name };
    if name.is_null() {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
    unsafe {
        if libc::strcmp(name, b"huawei app gallary\0".as_ptr() as *const _) == 0 {
            return crate::types::CERT_TYPE_APPGALLARY as i32;
        } else if libc::strcmp(name, b"huawei system apps\0".as_ptr() as *const _) == 0 {
            return crate::types::CERT_TYPE_SYETEM as i32;
        } else if libc::strcmp(name, b"OpenHarmony apps\0".as_ptr() as *const _) == 0 {
            return crate::types::CERT_TYPE_SYETEM as i32;
        } else {
            return crate::types::CERT_TYPE_OTHER as i32;
        }
    }
}

fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    for i in 0..num {
        let trust_entry = unsafe { &*trustList.offset(i as isize) };
        let issue_ca = trust_entry.issueCA;
        let issuer = unsafe { &(*signer).issuer };
        if unsafe { libc::strcmp(issue_ca, issuer.as_ptr()) } == 0 {
            let profile_sign_cert = trust_entry.profileSignCert;
            let profile_debug_sign_cert = trust_entry.profileDebugSignCert;
            let subject = unsafe { &(*signer).subject };
            if unsafe { libc::strcmp(profile_sign_cert, subject.as_ptr()) } == 0 ||
                unsafe { libc::strcmp(profile_debug_sign_cert, subject.as_ptr()) } == 0 {
                let _ = unsafe { crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const libc::c_char,
                    b"[%s:%d]: profile source name : %s\0".as_ptr() as *const libc::c_char,
                    b"GetProfSourceBySigningCert\0".as_ptr() as *const libc::c_char,
                    393,
                    trust_entry.name,
                ) };
                return trust_entry as *const crate::types::TrustAppCert;
            }
        }
    }
    std::ptr::null()
}

fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::globals::*;
    let trust_cert = unsafe {
        crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const _,
            g_trustAppList.as_ptr(),
            (std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32,
        )
    };
    let mut trust_cert = trust_cert;
    if unsafe { g_isDebugMode } && trust_cert.is_null() {
        trust_cert = unsafe {
            crate::src_app_verify::GetProfSourceBySigningCert(
                signer as *const _,
                g_trustAppListTest.as_ptr(),
                (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32,
            )
        };
    }
    if !trust_cert.is_null() {
        let max_cert_path = unsafe { (*trust_cert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const _,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const _,
                    412,
                    depth,
                )
            };
            return crate::types::V_ERR as i32;
        }
    }
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trust_cert);
    }
    crate::types::V_OK as i32
}

fn GetAppSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    for i in 0..num {
        let trust = unsafe { &*trustList.offset(i as isize) };
        let signer_ref = unsafe { &*signer };
        if unsafe { libc::strcmp(trust.appSignCert, signer_ref.subject.as_ptr() as *const i8) } == 0 &&
           unsafe { libc::strcmp(trust.issueCA, signer_ref.issuer.as_ptr() as *const i8) } == 0 {
            return trust as *const crate::types::TrustAppCert;
        }
    }
    std::ptr::null()
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_15
// c_function: GetAppCertTypeBySignInfo
// rust_file: src_app_verify.rs
// rust_signature: fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32
// c_first_line: static int32_t GetAppCertTypeBySignInfo(SignerResovledInfo *signer, int32_t *certType)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_15/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify.rs:282:168
//       |
//   error[E0308]: mismatched types
//      --> src/src_app_verify.rs:274:60
//       |
//       |                          -------------------------         ^^^^^^^^^^^^^^ expected `*const TrustAppCert`, found `[TrustAppCert; 3]`
//       |                          |
// =================================
fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_15
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_15/translated_rust.rs
 * ------------------------------------------------------------
fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::src_app_verify::*;
    let g_trust_app_list_len = (std::mem::size_of::<[crate::types::TrustAppCert; 1]>() / std::mem::size_of::<crate::types::TrustAppCert>()) as i32;
    let mut trust_cert = GetAppSourceBySigningCert(signer, g_trustAppList, g_trust_app_list_len);
    if g_isDebugMode && trust_cert.is_null() {
        let g_trust_app_list_test_len = (std::mem::size_of::<[crate::types::TrustAppCert; 1]>() / std::mem::size_of::<crate::types::TrustAppCert>()) as i32;
        trust_cert = GetAppSourceBySigningCert(signer, g_trustAppListTest, g_trust_app_list_test_len);
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const i8, __FUNCTION__!(), 443, (*trust_cert).maxCertPath, (*signer).depth);
                return V_ERR;
            }
        }
    }
    unsafe {
        *certType = GetCertTypeBySourceName(trust_cert);
    }
    V_OK
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_15
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const _) };
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const i8,
            b"GetAppSingerCertType\0".as_ptr() as *const i8,
            456,
        ) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe {
        crate::src_app_verify::GetAppCertTypeBySignInfo(
            (*sri).signers as *const crate::types::SignerResovledInfo,
            certType,
        )
    };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const i8,
            b"GetAppSingerCertType\0".as_ptr() as *const i8,
            461,
            ret,
        ) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
    crate::types::V_OK as i32
}

fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7) };
    if sri.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const i8, b"GetProfileSingerCertType\0".as_ptr() as *const i8, 474) };
        return crate::types::V_ERR as i32;
    }
    let signer_ptr = unsafe { (*sri).signers };
    let ret = crate::src_app_verify::GetProfileCertTypeBySignInfo(signer_ptr, certType);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const i8, b"GetProfileSingerCertType\0".as_ptr() as *const i8, 479, ret) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
    crate::types::V_OK as i32
}

fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    let mut profile_data: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut cert_type: i32 = 0;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut input_len: crate::types::size_t = 0;
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) as *mut crate::types::Pkcs7 };
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 496) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 499) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 parse message success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 501) };
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const crate::types::Pkcs7) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 505) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Verify certs success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 507) };
    let ret = unsafe { crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut cert_type) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 510) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    if cert_type == crate::types::CERT_TYPE_OTHER as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert type invalid\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 513) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let calc_digest: crate::types::PKCS7_CalcDigest = Some(unsafe { std::mem::transmute(crate::src_app_verify::CalcDigest as *const ()) });
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const crate::types::Pkcs7, calc_digest) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 518) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 519) };
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const crate::types::Pkcs7, &mut input, &mut input_len) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 523) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get profile sign content ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 525) };
    if input_len > crate::types::MAX_PROFILE_SIZE as crate::types::size_t || input_len == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    profile_data = unsafe { libc::malloc((input_len + 1) as usize) as *mut std::ffi::c_char };
    if profile_data.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profileData is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 532) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe { crate::compat::memcpy_s(profile_data as *mut std::ffi::c_void, input_len as u32, input as *const std::ffi::c_void, input_len as u32) };
    unsafe { *profile_data.offset(input_len as isize) = 0 };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 536) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        unsafe { libc::free(profile_data as *mut std::ffi::c_void) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
    unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    unsafe { *profileContent = profile_data };
    unsafe { *contentLen = input_len as i32 };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile get raw data ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 542) };
    crate::types::V_OK as i32
}

fn GetRsaPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *const u8 {
    let buf_size = 1024 * 2 + 20;
    let buf = unsafe { libc::malloc(buf_size as usize) as *mut u8 };
    if buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 554) };
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 559) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let c = unsafe { buf.add(buf_size as usize) };
    let pk_len = unsafe { crate::compat::mbedtls_pk_write_pubkey(&c as *const *mut u8 as *mut *mut u8, buf, pk) };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetRsaPk pkLen %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 565, pk_len) };
    if pk_len < 0 || pk_len > buf_size {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get pk buf error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 567) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let pk_buf = unsafe { libc::malloc(pk_len as usize) as *mut u8 };
    if pk_buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 574) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memcpy_s(pk_buf as *mut std::ffi::c_void, pk_len as crate::types::size_t, c as *const std::ffi::c_void, pk_len as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 581, ret) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        unsafe { libc::free(pk_buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    unsafe { *len = pk_len; }
    let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    unsafe { libc::free(buf as *mut std::ffi::c_void); }
    pk_buf as *const u8
}

fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let ecCtx = unsafe {
        match crate::compat::mbedtls_pk_get_type(pk) {
            crate::types::MBEDTLS_PK_ECKEY |
            crate::types::MBEDTLS_PK_ECKEY_DH |
            crate::types::MBEDTLS_PK_ECDSA => (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair,
            _ => std::ptr::null_mut(),
        }
    };
    if ecCtx.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ec pk error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 597) };
        return std::ptr::null_mut();
    }
    let buf_size = (2 * ((521 + 7) / 8) + 1) as usize;
    let buf = unsafe { libc::malloc(buf_size) } as *mut u8;
    if buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 602) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 607) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let ret = unsafe {
        crate::compat::mbedtls_ecp_point_write_binary(
            &(*ecCtx).private_grp,
            &(*ecCtx).private_Q,
            0,
            len as *mut crate::types::size_t,
            buf,
            buf_size as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ecc pk key error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 614) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let len_val = unsafe { *len };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetEcPk *len %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 619, len_val) };
    if len_val <= 0 || len_val > buf_size as i32 {
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let pkBuf = unsafe { libc::malloc(len_val as usize) } as *mut u8;
    if pkBuf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 626) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::compat::memcpy_s(pkBuf as *mut core::ffi::c_void, len_val as crate::types::size_t, buf as *const core::ffi::c_void, len_val as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 633, ret) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        unsafe { libc::free(pkBuf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    unsafe { libc::free(buf as *mut core::ffi::c_void) };
    pkBuf
}

fn GetPkBuf(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let mut bufA: *mut u8 = std::ptr::null_mut();
    let pk_type = unsafe { crate::compat::mbedtls_pk_get_type(pk) };
    if pk_type == crate::types::MBEDTLS_PK_RSA || pk_type == crate::types::MBEDTLS_PK_RSASSA_PSS {
        bufA = crate::src_app_verify::GetRsaPk(pk, len) as *mut u8;
    } else if pk_type == crate::types::MBEDTLS_PK_ECDSA || pk_type == crate::types::MBEDTLS_PK_ECKEY {
        bufA = crate::src_app_verify::GetEcPk(pk, len);
    }
    bufA
}

fn ParseCertGetPk(certEncoded: *const std::ffi::c_char, pk: *mut crate::types::AppSignPk) -> i32 {
    unsafe {
        let cert = libc::malloc(std::mem::size_of::<crate::types::mbedtls_x509_crt>()) as *mut crate::types::mbedtls_x509_crt;
        if cert.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: cert is null\0".as_ptr() as *const std::ffi::c_char,
                b"ParseCertGetPk\0".as_ptr() as *const std::ffi::c_char,
                657,
            );
            return crate::types::V_ERR as i32;
        }
        crate::compat::mbedtls_x509_crt_init(cert);
        let cert_len = libc::strlen(certEncoded) as usize + 1;
        let ret = crate::compat::mbedtls_x509_crt_parse(
            cert,
            certEncoded as *const u8,
            cert_len as crate::types::size_t,
        );
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: load cert failed, ret: %d\0".as_ptr() as *const std::ffi::c_char,
                b"ParseCertGetPk\0".as_ptr() as *const std::ffi::c_char,
                662,
                ret,
            );
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut std::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        let mut len: i32 = 0;
        let pk_buf = crate::src_app_verify::GetPkBuf(&(*cert).pk, &mut len);
        if pk_buf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: get pk error\0".as_ptr() as *const std::ffi::c_char,
                b"ParseCertGetPk\0".as_ptr() as *const std::ffi::c_char,
                669,
            );
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut std::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        (*pk).pk = pk_buf as *mut std::ffi::c_char;
        (*pk).len = len;
        crate::compat::mbedtls_x509_crt_free(cert);
        libc::free(cert as *mut std::ffi::c_void);
        crate::types::V_OK as i32
    }
}

fn GetAppSignPublicKey(profile: *const crate::types::ProfileProf, pk: *mut crate::types::AppSignPk) -> i32 {
    let mut ret: i32;
    unsafe {
        if !(*profile).bundleInfo.releaseCert.is_null() &&
            libc::strlen((*profile).bundleInfo.releaseCert as *const std::ffi::c_char) != 0 {
            ret = crate::src_app_verify::ParseCertGetPk(
                (*profile).bundleInfo.releaseCert as *const std::ffi::c_char,
                pk,
            );
        } else {
            ret = crate::src_app_verify::ParseCertGetPk(
                (*profile).bundleInfo.devCert as *const std::ffi::c_char,
                pk,
            );
        }
    }
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                "appverify\0".as_ptr() as *const std::ffi::c_char,
                "[%s:%d]: GetSignCertpk failed, ret: %d\0".as_ptr() as *const std::ffi::c_char,
                "GetAppSignPublicKey\0".as_ptr() as *const std::ffi::c_char,
                692,
                ret,
            );
        }
        return crate::types::V_ERR_GET_CERT_PK as i32;
    }
    return crate::types::V_OK as i32;
}

fn FreeAppSignPublicKey(pk: *mut crate::types::AppSignPk) {
    if pk.is_null() {
        return;
    }
    unsafe {
        if !(*pk).pk.is_null() {
            libc::free((*pk).pk as *mut std::ffi::c_void);
            (*pk).pk = std::ptr::null_mut();
        }
    }
}

pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::{GetAppSignPublicKey, FreeAppSignPublicKey};
    use crate::compat::*;
    use libc::{free, malloc, strlen};

    if profile.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: profile is null\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                708,
            );
        }
        return crate::types::V_ERR as i32;
    }

    let mut pk = crate::types::AppSignPk {
        pk: std::ptr::null_mut(),
        len: 0,
    };
    let ret = GetAppSignPublicKey(profile as *const crate::types::ProfileProf, &mut pk as *mut crate::types::AppSignPk);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get sign pk failed\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                712,
            );
        }
        return ret;
    }

    let mut use_len: crate::types::size_t = 0;
    unsafe {
        let _ = mbedtls_base64_encode(
            std::ptr::null_mut(),
            0,
            &mut use_len as *mut crate::types::size_t,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        );
    }

    let bundle_name_len: i32 = unsafe {
        let bundle_name = (*profile).bundleInfo.bundleName;
        if bundle_name.is_null() {
            0
        } else {
            strlen(bundle_name) as i32
        }
    };
    let appid_len: i32 = bundle_name_len + use_len as i32 + 1 + 1;

    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: GetAppid %d\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            721,
            appid_len,
        );
    }

    if use_len > 4096 {
        FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR as i32;
    }

    let appid = unsafe { malloc(appid_len as usize) as *mut i8 };
    if appid.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc failed\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                727,
            );
        }
        FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_MALLOC as i32;
    }

    unsafe {
        *appid.offset((appid_len - 1) as isize) = 0;
    }

    let ret = unsafe {
        libc::snprintf(
            appid,
            appid_len as usize,
            b"%s_\0".as_ptr() as *const i8,
            (*profile).bundleInfo.bundleName,
        )
    };
    if ret < 0 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                734,
                ret,
            );
            if !appid.is_null() {
                free(appid as *mut std::ffi::c_void);
            }
        }
        FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    let ret = unsafe {
        mbedtls_base64_encode(
            appid.offset((bundle_name_len + 1) as isize) as *mut u8,
            (appid_len - bundle_name_len - 1) as crate::types::size_t,
            &mut use_len as *mut crate::types::size_t,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: base 64 encode error\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                742,
            );
            if !appid.is_null() {
                free(appid as *mut std::ffi::c_void);
            }
        }
        FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    unsafe {
        (*profile).appid = appid;
        let _ = HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: appid len: %d, bL len: %d, base64: %d\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            748,
            appid_len,
            bundle_name_len,
            use_len as i32,
        );
        let _ = HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: %s\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            749,
            appid,
        );
    }

    FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
    crate::types::V_OK as i32
}

fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut profBuf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut blockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut rawLen: i32 = 0;
    let rawBuf = crate::src_app_verify::GetSignBlockByType(
        signInfo,
        fp,
        crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
        &mut rawLen,
        &mut blockHead,
    );
    if rawBuf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rawBuf is null\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                762,
            );
        }
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: certType %d\0".as_ptr() as *const std::ffi::c_char,
            b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
            763,
            certType,
        );
    }
    let mut ret: i32;
    if certType == 0 {
        profBuf = rawBuf;
        len = rawLen;
        ret = 0;
    } else {
        let mut contentLen: i32 = 0;
        ret = crate::src_app_verify::VerifyProfileSignGetRaw(
            rawBuf as *const std::ffi::c_char,
            rawLen,
            &mut profBuf,
            &mut contentLen,
        );
        len = contentLen;
        if !rawBuf.is_null() {
            unsafe {
                libc::free(rawBuf as *mut std::ffi::c_void);
            }
        }
        if ret != crate::types::V_OK as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                    b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                    772,
                );
            }
            return ret;
        }
    }
    ret = crate::src_app_provision::ParseProfile(profBuf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: GetSignBlock error\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                777,
            );
        }
        if !profBuf.is_null() {
            unsafe {
                libc::free(profBuf as *mut std::ffi::c_void);
            }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    if !profBuf.is_null() {
        unsafe {
            libc::free(profBuf as *mut std::ffi::c_void);
        }
    }
    ret = crate::src_app_provision::VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                784,
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    ret = crate::src_app_verify::GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char,
                787,
            );
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    crate::types::V_OK as i32
}

fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo) -> i32 {
    unsafe {
        if certA.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: certA is null\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                797,
            );
            return crate::types::V_ERR as i32;
        }
        if binSignCert.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: binSignCert is null\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                798,
            );
            return crate::types::V_ERR as i32;
        }

        let cert = &*certA;
        let bin = &*binSignCert;

        if cert.subject_raw.len != bin.subjectLen as crate::types::size_t
            || libc::memcmp(
                cert.subject_raw.p as *const _,
                bin.subject as *const _,
                cert.subject_raw.len as usize,
            ) != 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: cert subject diff\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                802,
            );
            return crate::types::V_ERR as i32;
        }

        if cert.issuer_raw.len != bin.issuerLen as crate::types::size_t
            || libc::memcmp(
                cert.issuer_raw.p as *const _,
                bin.issuer as *const _,
                cert.issuer_raw.len as usize,
            ) != 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: cert issuer diff\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                808,
            );
            return crate::types::V_ERR as i32;
        }

        if crate::compat::mbedtls_pk_get_type(&cert.pk) != bin.pkType {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: pk type diff\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                814,
            );
            return crate::types::V_ERR as i32;
        }

        let mut lenA: i32 = 0;
        let bufA = crate::src_app_verify::GetPkBuf(&cert.pk, &mut lenA);
        if bufA.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: bufA is null\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                819,
            );
            return crate::types::V_ERR as i32;
        }

        if lenA != bin.pkLen {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: pkA len diff %d, %d\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                822,
                lenA,
                bin.pkLen,
            );
            libc::free(bufA as *mut _);
            return crate::types::V_ERR as i32;
        }

        if libc::memcmp(
            bufA as *const _,
            bin.pkBuf as *const _,
            lenA as usize,
        ) != 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: pk content different\0".as_ptr() as *const _,
                b"CmpCert\0".as_ptr() as *const _,
                828,
            );
            libc::free(bufA as *mut _);
            return crate::types::V_ERR as i32;
        }

        libc::free(bufA as *mut _);
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: compare cert consistent\0".as_ptr() as *const _,
            b"CmpCert\0".as_ptr() as *const _,
            833,
        );
        crate::types::V_OK as i32
    }
}

pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::types::{V_ERR, V_OK, LOG_CORE, LOG_ERROR, LOG_INFO};
    if certBase64.is_null() || binSignCert.is_null() {
        return V_ERR as i32;
    }
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { ::core::mem::zeroed() };
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut cert);
    }
    let cert_cstr = certBase64 as *const ::core::ffi::c_char;
    let len = unsafe { libc::strlen(cert_cstr) } as crate::types::size_t;
    let ret = unsafe {
        crate::compat::mbedtls_x509_crt_parse(&mut cert, certBase64, len.wrapping_add(1))
    };
    if ret != V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                846,
            )
        };
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                847,
                certBase64,
            )
        };
        unsafe {
            crate::compat::mbedtls_x509_crt_free(&mut cert);
        }
        return V_ERR as i32;
    }
    if crate::src_app_verify::CmpCert(&cert, binSignCert) == V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: cert consistent\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                852,
            )
        };
        unsafe {
            crate::compat::mbedtls_x509_crt_free(&mut cert);
        }
        return V_OK as i32;
    }
    let _ = unsafe {
        crate::compat::HiLogPrint(
            LOG_CORE as u32,
            LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: cert inconsistent\0".as_ptr() as *const ::core::ffi::c_char,
            b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
            856,
        )
    };
    unsafe {
        crate::compat::mbedtls_x509_crt_free(&mut cert);
    }
    V_ERR as i32
}

fn CheckReleaseAppSign(binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    use crate::types::{V_ERR, V_OK, LOG_CORE, LOG_ERROR, LOG_INFO};
    unsafe {
        let app_dist_type = (*pf).appDistType;
        if !app_dist_type.is_null() && libc::strcmp(app_dist_type, b"app_gallery\0".as_ptr() as *const _) == 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: app release, distribution type is app_gallery, return error\0".as_ptr() as *const _, b"CheckReleaseAppSign\0".as_ptr() as *const _, 865);
            return V_ERR as i32;
        }
        let release_cert = (*pf).bundleInfo.releaseCert;
        if release_cert.is_null() || libc::strlen(release_cert as *const _) == 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: release app, release Cert null\0".as_ptr() as *const _, b"CheckReleaseAppSign\0".as_ptr() as *const _, 870);
            return V_ERR as i32;
        }
        let ret = crate::src_app_verify::LoadCertAndCmpDest(release_cert, binSignCert);
        if ret == V_OK as i32 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: dev cert consistent\0".as_ptr() as *const _, b"CheckReleaseAppSign\0".as_ptr() as *const _, 875);
            return V_OK as i32;
        }
        let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: app sign cert not consistent with profile cert\0".as_ptr() as *const _, b"CheckReleaseAppSign\0".as_ptr() as *const _, 878);
        V_ERR as i32
    }
}

fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::LoadCertAndCmpDest;
    unsafe {
        if libc::strlen((*pf).bundleInfo.devCert as *const i8) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: debug app, devCert null\0".as_ptr() as *const i8,
                "CheckDebugAppSign\0".as_ptr() as *const i8,
                885,
            );
            return crate::types::V_ERR as i32;
        }
        let mut ret = LoadCertAndCmpDest((*pf).bundleInfo.devCert, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: dev cert consistent\0".as_ptr() as *const i8,
                "CheckDebugAppSign\0".as_ptr() as *const i8,
                890,
            );
            return crate::types::V_OK as i32;
        }
        if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) != 0 {
            ret = LoadCertAndCmpDest((*pf).bundleInfo.releaseCert, binSignCert as *const crate::types::CertInfo);
            if ret == crate::types::V_OK as i32 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: release cert consistent\0".as_ptr() as *const i8,
                    "CheckDebugAppSign\0".as_ptr() as *const i8,
                    896,
                );
                return crate::types::V_OK as i32;
            }
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: app sign cert not consistent with profile cert\0".as_ptr() as *const i8,
            "CheckDebugAppSign\0".as_ptr() as *const i8,
            900,
        );
        crate::types::V_ERR as i32
    }
}

fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    if appCertType == 0 || appCertType == 1 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: app type : %d, return OK\0".as_ptr() as *const i8, b"CheckAppSignCertWithProfile\0".as_ptr() as *const i8, 908, appCertType) };
        return crate::types::V_OK as i32;
    }
    let mut ret = crate::types::V_ERR as i32;
    unsafe {
        let type_ptr = (*pf).type_;
        if !type_ptr.is_null() && libc::strcmp(type_ptr, b"debug\0".as_ptr() as *const i8) == 0 {
            ret = crate::src_app_verify::CheckDebugAppSign(binSignCert as *mut crate::types::CertInfo, pf);
        } else if !type_ptr.is_null() && libc::strcmp(type_ptr, b"release\0".as_ptr() as *const i8) == 0 {
            ret = crate::src_app_verify::CheckReleaseAppSign(binSignCert, pf);
        }
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: check app sign cert ret : %d\0".as_ptr() as *const i8, b"CheckAppSignCertWithProfile\0".as_ptr() as *const i8, 920, ret) };
    ret
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_32
// c_function: CertInfoInit
// rust_file: src_app_verify.rs
// rust_signature: fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32
// c_first_line: static int32_t CertInfoInit(CertInfo *certInfo)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_32/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify.rs:359:226
//       |
//       |                                                                                              ^^^^^^^^^^^^
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error
// =================================
fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_32
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_32/translated_rust.rs
 * ------------------------------------------------------------
fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32 {
    if certInfo.is_null() {
        return 0;
    }
    let size = std::mem::size_of::<crate::types::CertInfo>();
    let ret = unsafe { crate::compat::memset_s(certInfo as *mut std::ffi::c_void, size as crate::types::size_t, 0, size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: memset error\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 928) };
    }
    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_32
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn FreeCertInfo(certInfo: *mut crate::types::CertInfo) {
    if certInfo.is_null() {
        return;
    }
    unsafe {
        if !(*certInfo).issuer.is_null() {
            libc::free((*certInfo).issuer as *mut libc::c_void);
            (*certInfo).issuer = std::ptr::null_mut();
            (*certInfo).issuerLen = 0;
        }
        if !(*certInfo).subject.is_null() {
            libc::free((*certInfo).subject as *mut libc::c_void);
            (*certInfo).subject = std::ptr::null_mut();
            (*certInfo).subjectLen = 0;
        }
        if !(*certInfo).pkBuf.is_null() {
            libc::free((*certInfo).pkBuf as *mut libc::c_void);
            (*certInfo).pkBuf = std::ptr::null_mut();
            (*certInfo).pkLen = 0;
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_34
// c_function: GetCertInfo
// rust_file: src_app_verify.rs
// rust_signature: fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32
// c_first_line: static int32_t GetCertInfo(const mbedtls_x509_crt *ctr, CertInfo **binSignCert)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_34/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify.rs:933:17
//       |
//       |                 ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify.rs:870:17
//       |
//       |                 ^^^^^^^^^^^^
// =================================
fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_34
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_34/translated_rust.rs
 * ------------------------------------------------------------
fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    unsafe {
        let certInfo = libc::malloc(std::mem::size_of::<crate::types::CertInfo>()) as *mut crate::types::CertInfo;
        if certInfo.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: certInfo is null\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                958,
            );
            return crate::types::V_ERR_MALLOC as i32;
        }
        let mut ret = crate::src_app_verify::CertInfoInit(certInfo);
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: cert info init\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                962,
            );
            ret = crate::types::V_ERR_MEMSET as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        (*certInfo).issuerLen = (*ctr).issuer_raw.len as i32;
        (*certInfo).subjectLen = (*ctr).subject_raw.len as i32;
        if (*certInfo).issuerLen == 0
            || (*certInfo).issuerLen > (1024 * 1024)
            || (*certInfo).subjectLen == 0
            || (*certInfo).subjectLen > (1024 * 1024)
        {
            ret = crate::types::V_ERR_MALLOC as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        (*certInfo).issuer = libc::malloc(((*certInfo).issuerLen + 1) as usize) as *mut i8;
        if (*certInfo).issuer.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        *(*certInfo).issuer.offset((*certInfo).issuerLen as isize) = 0;
        ret = crate::compat::memcpy_s(
            (*certInfo).issuer as *mut core::ffi::c_void,
            (*certInfo).issuerLen as crate::types::size_t,
            (*ctr).issuer_raw.p as *const core::ffi::c_void,
            (*ctr).issuer_raw.len,
        ) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        (*certInfo).subject = libc::malloc(((*certInfo).subjectLen + 1) as usize) as *mut i8;
        if (*certInfo).subject.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        *(*certInfo).subject.offset((*certInfo).subjectLen as isize) = 0;
        ret = crate::compat::memcpy_s(
            (*certInfo).subject as *mut core::ffi::c_void,
            (*certInfo).subjectLen as crate::types::size_t,
            (*ctr).subject_raw.p as *const core::ffi::c_void,
            (*ctr).subject_raw.len,
        ) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        (*certInfo).pkType = crate::compat::mbedtls_pk_get_type(&(*ctr).pk);
        (*certInfo).pkBuf = crate::src_app_verify::GetPkBuf(&(*ctr).pk, &mut (*certInfo).pkLen) as *mut i8;
        if (*certInfo).pkBuf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: get pk error\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                998,
            );
            ret = crate::types::V_ERR as i32;
            goto_exit(certInfo, ret);
            return ret;
        }
        *binSignCert = certInfo;
        return crate::types::V_OK as i32;
    }

    fn goto_exit(certInfo: *mut crate::types::CertInfo, ret: i32) -> i32 {
        unsafe {
            crate::src_app_verify::FreeCertInfo(certInfo);
            if !certInfo.is_null() {
                libc::free(certInfo as *mut core::ffi::c_void);
            }
        }
        ret
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_34
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: VerifyProfGetContent error: %d\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1015, ret);
        }
        return ret;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: verify prof get content success\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1018);
    }

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: CheckAppSignCertWithProfile error: %d\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1023, ret);
        }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR_VERFIY_PROF_CERT as i32;
    }

    unsafe {
        if !(*pf).bundleInfo.devCert.is_null() {
            libc::free((*pf).bundleInfo.devCert as *mut libc::c_void);
            (*pf).bundleInfo.devCert = std::ptr::null_mut();
        }
        if !(*pf).bundleInfo.releaseCert.is_null() {
            libc::free((*pf).bundleInfo.releaseCert as *mut libc::c_void);
            (*pf).bundleInfo.releaseCert = std::ptr::null_mut();
        }
    }

    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: verfiy app source success\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1032);
    }
    return crate::types::V_OK as i32;
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_36
// c_function: VerifyAppSignPkcsData
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32
// c_first_line: static int32_t VerifyAppSignPkcsData(const FileRead *fileRead, const SignatureInfo *signInfo, const Pkcs7 *pkcs7Handle)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_app_verify.rs:1763:83
//        |
//        |                                                                              ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected "C" fn, found "Rust" fn
//        |                                                                              |
//        |                                                                              arguments to this enum variant are incorrect
//        |
//   help: the type constructed contains `fn(*const Pkcs7, *const tagSignerInfo, u32, *mut u8, *mut u32) -> i32 {CalcDigest}` due to the type of the argument passed
// =================================
fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_36
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
 * ------------------------------------------------------------
extern "C" fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: Verify certs failed, ret: %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1041,
            ret,
        );
        return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE as u32,
        crate::types::LOG_INFO as u32,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: Verify certs success\0".as_ptr() as *const i8,
        "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
        1044,
    );
    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: VerifyRawHash failed : %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1048,
            ret,
        );
        return ret;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE as u32,
        crate::types::LOG_INFO as u32,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: VerifyRawHash success\0".as_ptr() as *const i8,
        "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
        1051,
    );
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, Some(crate::src_app_verify::CalcDigest));
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: pkcs7 verify signer signature failed : %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1055,
            ret,
        );
        return crate::types::V_ERR_VERIFY_SIGNATURE as i32;
    }
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_36
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetBinSignPkcs(signBuf: *const std::ffi::c_char, len: i32) -> *mut crate::types::Pkcs7 {
    unsafe {
        let pkcs7 = libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) as *mut crate::types::Pkcs7;
        if pkcs7.is_null() {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: malloc error\0".as_ptr() as *const std::ffi::c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const std::ffi::c_char,
                1066,
            );
            return std::ptr::null_mut();
        }
        let ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(
            signBuf as *const ::core::ffi::c_uchar,
            len as crate::types::size_t,
            pkcs7,
        );
        if ret != crate::types::V_OK as i32 {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const std::ffi::c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const std::ffi::c_char,
                1071,
                ret,
            );
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            if !pkcs7.is_null() {
                libc::free(pkcs7 as *mut std::ffi::c_void);
            }
            return std::ptr::null_mut();
        }
        pkcs7
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_38
// c_function: GetFileRead
// rust_file: src_app_verify.rs
// rust_signature: fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead
// c_first_line: static FileRead *GetFileRead(int32_t fp, int32_t offset, int32_t size)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_38/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_verify.rs:433:174
//       |
//       |                                                                                                                                                                              ^^^^^^^^^^^^
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error
// =================================
fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_38
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_38/translated_rust.rs
 * ------------------------------------------------------------
fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead {
    let fileRead = unsafe { libc::malloc(std::mem::size_of::<crate::types::FileRead>()) } as *mut crate::types::FileRead;
    if fileRead.is_null() {
        let _ = HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: malloc error\0".as_ptr() as *const i8, __FUNCTION__!(), 1084);
        return std::ptr::null_mut();
    }
    unsafe {
        (*fileRead).fp = fp;
        (*fileRead).offset = offset;
        (*fileRead).len = size;
    }
    fileRead
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_38
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32 = 0;

    let signBuf = crate::src_app_verify::GetSignBlockByType(signInfo as *const _, fp, crate::types::SIGNATURE_BLOCK_TYPE as i32, &mut blockLen, &mut blockHead);
    if signBuf.is_null() {
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = crate::src_app_verify::GetBinSignPkcs(signBuf as *const std::ffi::c_char, blockLen);
    if pkcs7.is_null() {
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    fileRead = crate::src_app_verify::GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        ret = crate::types::V_ERR_MALLOC as i32;
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return ret;
    }

    ret = crate::src_app_verify::GetAppSingerCertType(pkcs7, certType);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        unsafe { libc::free(fileRead as *mut std::ffi::c_void) };
        return ret;
    }

    unsafe { (*signInfo).certType = *certType; }

    ret = crate::src_app_verify::VerifyAppSignPkcsData(fileRead as *const _, signInfo as *const _, pkcs7 as *const _);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        unsafe { libc::free(fileRead as *mut std::ffi::c_void) };
        return ret;
    }

    let crt_ptr = unsafe { (*pkcs7).signedData.signers.certPath.crt };
    ret = crate::src_app_verify::GetCertInfo(crt_ptr as *const _, signCert);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_INFO as i32;
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        unsafe { libc::free(fileRead as *mut std::ffi::c_void) };
        return ret;
    }

    ret = crate::types::V_OK as i32;
    unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    unsafe { libc::free(fileRead as *mut std::ffi::c_void) };
    ret
}

fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify bin sign error\0".as_ptr() as *const _, b"VerifyIntegrity\0".as_ptr() as *const _, 1158);
        }
        return ret;
    }
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const _, certType, binSignCert, pf);
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const _, b"VerifyIntegrity\0".as_ptr() as *const _, 1164, ret);
        }
        crate::src_app_verify::FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe { libc::free(binSignCert as *mut _) };
            binSignCert = std::ptr::null_mut();
        }
        return ret;
    }
    crate::src_app_verify::FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe { libc::free(binSignCert as *mut _) };
        binSignCert = std::ptr::null_mut();
    }
    0
}

pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    use crate::types::{FileRead, SignatureInfo, VerifyResult, V_ERR_FILE_OPEN, V_ERR_FILE_STAT, V_ERR_MALLOC, V_OK};
    use crate::src_app_file::InitVerify;
    use crate::src_app_provision::ProfFreeData;
    use crate::src_app_verify::{GetSignHead, VerifyIntegrity};
    use ::core::ffi::c_void;
    use libc::{close, fstat, free, malloc};

    if filePath.is_null() || verifyRst.is_null() {
        return V_ERR_FILE_OPEN as i32;
    }
    let mut handle: i32 = 0;
    let mut file = FileRead {
        fp: 0,
        offset: 0,
        len: 0,
    };
    if unsafe { InitVerify(&mut file as *mut FileRead, filePath, &mut handle as *mut i32) } != V_OK as i32 {
        unsafe { close(handle); }
        return V_ERR_FILE_OPEN as i32;
    }
    let mut signInfo = SignatureInfo {
        signHead: std::ptr::null_mut(),
        fullSignBlockOffset: 0,
        hapCoreDirOffset: 0,
        hapEocdOffset: 0,
        hapEocdSize: 0,
        fileSize: 0,
        version: 0,
        certType: 0,
    };
    let ret = unsafe { GetSignHead(&file as *const FileRead, &mut signInfo as *mut SignatureInfo) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        return ret;
    }
    let mut signHead = signInfo.signHead;
    let ret = unsafe { VerifyIntegrity(&mut signInfo as *mut SignatureInfo, handle, &mut (*verifyRst).profile as *mut crate::types::ProfileProf) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        return ret;
    }
    let fileSt = unsafe { malloc(std::mem::size_of::<libc::stat>()) } as *mut libc::stat;
    if fileSt.is_null() {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        unsafe { ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf); }
        return V_ERR_MALLOC as i32;
    }
    let ret = unsafe { fstat(handle, fileSt) };
    if ret != V_OK as i32 {
        unsafe { close(handle); }
        if !signHead.is_null() {
            unsafe { free(signHead as *mut c_void); }
            signHead = std::ptr::null_mut();
        }
        unsafe { ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf); }
        if !fileSt.is_null() {
            unsafe { free(fileSt as *mut c_void); }
        }
        return V_ERR_FILE_STAT as i32;
    }
    unsafe { close(handle); }
    if !signHead.is_null() {
        unsafe { free(signHead as *mut c_void); }
    }
    if !fileSt.is_null() {
        unsafe { free(fileSt as *mut c_void); }
    }
    return ret;
}

pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: set debug mode: %d\0".as_ptr() as *const _,
            b"APPVERI_SetDebugMode\0".as_ptr() as *const _,
            1227,
            mode as i32,
        );
        if g_isDebugMode == mode {
            return crate::types::V_OK as i32;
        }
    }
    let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: enable pcks7 debug mode failed\0".as_ptr() as *const _,
                b"APPVERI_SetDebugMode\0".as_ptr() as *const _,
                1233,
            );
        }
        return ret;
    }
    unsafe {
        g_isDebugMode = mode;
    }
    crate::types::V_OK as i32
}

pub extern "C" fn APPVERI_SetActsMode(mode: bool) {
    unsafe {
        g_isActsMode = mode;
    }
}

pub extern "C" fn APPVERI_IsActsMode() -> i32 {
    unsafe { g_isActsMode as i32 }
}

pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: free verify rst data\0".as_ptr() as *const _, b"APPVERI_FreeVerifyRst\0".as_ptr() as *const _, 1256) };
    unsafe {
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile);
    }
}
