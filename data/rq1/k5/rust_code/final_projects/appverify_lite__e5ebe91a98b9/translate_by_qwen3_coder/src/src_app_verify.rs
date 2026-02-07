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
        let block_num = std::ptr::read_unaligned((*signHead).blockNum as *const u32);
        (*signHead).blockNum = crate::src_app_common::HapGetInt(
            &block_num as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        let size_val = std::ptr::read_unaligned((*signHead).size as *const ::core::ffi::c_ulonglong);
        (*signHead).size = crate::src_app_common::HapGetInt64(
            &size_val as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let magic_low = std::ptr::read_unaligned((*signHead).magicLow as *const ::core::ffi::c_ulonglong);
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(
            &magic_low as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let magic_high = std::ptr::read_unaligned((*signHead).magicHigh as *const ::core::ffi::c_ulonglong);
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(
            &magic_high as *const ::core::ffi::c_ulonglong as *const ::core::ffi::c_uchar,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        let version_val = std::ptr::read_unaligned((*signHead).version as *const u32);
        (*signHead).version = crate::src_app_common::HapGetInt(
            &version_val as *const u32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
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
    if content.is_null() {
        return;
    }
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

fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    let mut file_st: libc::stat = unsafe { ::core::mem::zeroed() };
    let ret = unsafe { libc::fstat((*file).fp, &mut file_st as *mut libc::stat) };
    if ret != 0 || (file_st.st_size as i64) < (::core::mem::size_of::<HwSignHead>() as i64) {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 121, ret, file_st.st_size as i32); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: find signature error\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 125); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let hap_core_dir_offset = unsafe { (*signInfo).hapCoreDirOffset };
    if hap_core_dir_offset < ::core::mem::size_of::<HwSignHead>() as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 129, hap_core_dir_offset); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let seek_ret = unsafe { libc::lseek((*file).fp, (hap_core_dir_offset as i64) - (::core::mem::size_of::<HwSignHead>() as i64), 0) };
    if seek_ret < 0 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 134, seek_ret as i32); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    let sign_head = unsafe { libc::malloc(::core::mem::size_of::<HwSignHead>()) as *mut HwSignHead };
    if sign_head.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: signHead is null\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 138); }
        return V_ERR as i32;
    }
    let read_len = unsafe { libc::read((*file).fp, sign_head as *mut c_void, ::core::mem::size_of::<HwSignHead>()) };
    if read_len != ::core::mem::size_of::<HwSignHead>() as isize {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 141, read_len as i32, ::core::mem::size_of::<HwSignHead>() as i32); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    crate::src_app_verify::SignHeadN2H(sign_head);
    let mut magic_low: u64 = HAP_SIG_BLOCK_MAGIC_LO;
    let mut magic_high: u64 = HAP_SIG_BLOCK_MAGIC_HI;
    if unsafe { (*sign_head).version } < VERSION_FOR_NEW_MAGIC_NUM {
        magic_low = HAP_SIG_BLOCK_MAGIC_LO_OLD;
        magic_high = HAP_SIG_BLOCK_MAGIC_HI_OLD;
    }
    if unsafe { (*sign_head).magicLow } != magic_low || unsafe { (*sign_head).magicHigh } != magic_high {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 153); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    unsafe { let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 157, (*sign_head).size, (*sign_head).blockNum); }
    unsafe {
        (*signInfo).signHead = sign_head;
        (*signInfo).fullSignBlockOffset = hap_core_dir_offset - (*sign_head).size as i32;
        (*signInfo).fileSize = file_st.st_size as i32;
    }
    let full_sign_block_offset = unsafe { (*signInfo).fullSignBlockOffset };
    if full_sign_block_offset <= 0 || full_sign_block_offset >= hap_core_dir_offset {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8, b"GetSignHead\0".as_ptr() as *const i8, 162); }
        unsafe { libc::free(sign_head as *mut c_void); }
        return V_ERR_GET_SIGNHEAD as i32;
    }
    V_OK as i32
}

fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    use crate::src_app_common::HapGetInt;
    use crate::src_app_verify::BlockHeadN2H;
    unsafe {
        let signH = (*signInfo).signHead;
        if signH.is_null() {
            return crate::types::V_ERR as i32;
        }
        let _ = libc::lseek(fp, (*signInfo).fullSignBlockOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let mut num = (*signH).blockNum as i32;
        if num > crate::types::MAX_BLOCK_NUM as i32 {
            return crate::types::V_ERR as i32;
        }
        while num > 0 {
            num -= 1;
            let read_len = libc::read(fp, block as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::BlockHead>() as libc::size_t);
            if read_len != std::mem::size_of::<crate::types::BlockHead>() as isize {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: find block head , read err %d, %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char,
                    181,
                    read_len,
                    std::mem::size_of::<crate::types::BlockHead>() as i32,
                );
                return crate::types::V_ERR as i32;
            }
            let type_val = HapGetInt((&(*block).type_ as *const u32) as *const ::core::ffi::c_uchar, std::mem::size_of::<u32>() as i32);
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find block type: %0x\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char,
                185,
                type_val,
            );
            if type_val == blockType {
                BlockHeadN2H(block);
                return crate::types::V_OK as i32;
            }
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: get sign block by type failed, type: %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char,
            191,
            blockType,
        );
        crate::types::V_ERR as i32
    }
}

pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    if signInfo.is_null() || blockHead.is_null() {
        return std::ptr::null_mut();
    }
    let ret = crate::src_app_verify::FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: find block head error\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 203);
        }
        return std::ptr::null_mut();
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: type: %u, len: %u, offset: %u signoffset: %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 207, (*blockHead).type_, (*blockHead).length, (*blockHead).offset, (*signInfo).fullSignBlockOffset);
        if (*blockHead).length == 0 || (*blockHead).length > ((*signInfo).hapCoreDirOffset - (*signInfo).fullSignBlockOffset) as u32 {
            return std::ptr::null_mut();
        }
        if (*blockHead).length + 1 >= (*signInfo).fileSize as u32 {
            return std::ptr::null_mut();
        }
    }
    let buf_len = unsafe { (*blockHead).length as usize };
    let buf = unsafe { libc::malloc((buf_len + 1) as usize) as *mut ::core::ffi::c_char };
    if buf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 222);
        }
        return std::ptr::null_mut();
    }
    unsafe {
        *buf.offset(buf_len as isize) = 0;
    }
    let mut fileSt = std::mem::MaybeUninit::<libc::stat>::uninit();
    let ret = unsafe { libc::fstat(fp, fileSt.as_mut_ptr()) };
    unsafe {
        if ret != 0 || fileSt.assume_init().st_size < (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64 + (*blockHead).length as i64 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 229, ret, fileSt.assume_init().st_size as i32);
            libc::free(buf as *mut _);
            return std::ptr::null_mut();
        }
        libc::lseek(fp, (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64, 0);
        let readLen = libc::read(fp, buf as *mut _, (*blockHead).length as usize) as i32;
        if readLen != (*blockHead).length as i32 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: read error: %d, %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 236, readLen, (*blockHead).length);
            libc::free(buf as *mut _);
            return std::ptr::null_mut();
        }
        if !len.is_null() {
            *len = readLen;
        }
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: buf begin\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 241);
    }
    buf
}

pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: algId: %d\0".as_ptr() as *const i8,
            "GetHashUnitLen\0".as_ptr() as *const i8,
            247,
            hashAlg,
        );
        let info = crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t);
        crate::compat::mbedtls_md_get_size(info) as i32
    }
}

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
    return crate::types::V_OK as i32;
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
    let ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut inputLen);
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
    let ret = unsafe { crate::compat::memcpy_s(content as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::ContentInfo>() as u32, input as *const ::core::ffi::c_void, inputLen as u32) };
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
    let alg_id = unsafe { (*content).algId };
    if alg_id != crate::types::MBEDTLS_MD_SHA256 as i32 && alg_id != crate::types::MBEDTLS_MD_SHA384 as i32 && alg_id != crate::types::MBEDTLS_MD_SHA512 as i32 {
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
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(alg_id);
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actualDigest, rootHashLen) {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: create buf fail\0".as_ptr() as *const i8, "VerifyRawHash\0".as_ptr() as *const i8, 348);
            libc::free(content as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR as i32;
    }
    let fp = unsafe { (*fileRead).fp };
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(alg_id, fp, signInfo, &actualDigest) {
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
    return crate::types::V_OK as i32;
}

fn GetCertTypeBySourceName(cert: *const crate::types::TrustAppCert) -> i32 {
    if cert.is_null() {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
    unsafe {
        let name = (*cert).name;
        if name.is_null() {
            return crate::types::CERT_TYPE_OTHER as i32;
        }
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
        let trust = unsafe { &*trustList.offset(i as isize) };
        let sign = unsafe { &*signer };
        if unsafe { libc::strcmp(trust.issueCA, sign.issuer.as_ptr()) } == 0 {
            if unsafe { libc::strcmp(trust.profileSignCert, sign.subject.as_ptr()) } == 0 ||
               unsafe { libc::strcmp(trust.profileDebugSignCert, sign.subject.as_ptr()) } == 0 {
                let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profile source name : %s\0".as_ptr() as *const _, b"GetProfSourceBySigningCert\0".as_ptr() as *const _, 393, (*trustList.offset(i as isize)).name) };
                return trust;
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
    unsafe {
        if g_isDebugMode && trust_cert.is_null() {
            trust_cert = crate::src_app_verify::GetProfSourceBySigningCert(
                signer as *const _,
                g_trustAppListTest.as_ptr(),
                (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32,
            );
        }
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const _,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const _,
                    412,
                    (*signer).depth,
                );
                return crate::types::V_ERR as i32;
            }
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
        if unsafe { libc::strcmp(trust.appSignCert, signer_ref.subject.as_ptr()) } == 0 &&
           unsafe { libc::strcmp(trust.issueCA, signer_ref.issuer.as_ptr()) } == 0 {
            return trust;
        }
    }
    std::ptr::null()
}

fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::src_app_verify::GetAppSourceBySigningCert;
    use crate::src_app_verify::GetCertTypeBySourceName;
    let g_trust_app_list_len = unsafe {
        (std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
    };
    let mut trust_cert = unsafe {
        GetAppSourceBySigningCert(signer, g_trustAppList.as_ptr(), g_trust_app_list_len)
    };
    if unsafe { g_isDebugMode } && trust_cert.is_null() {
        let g_trust_app_list_test_len = unsafe {
            (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
        };
        trust_cert = unsafe {
            GetAppSourceBySigningCert(signer, g_trustAppListTest.as_ptr(), g_trust_app_list_test_len)
        };
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const _,
                    b"GetAppCertTypeBySignInfo\0".as_ptr() as *const _,
                    443,
                    (*trust_cert).maxCertPath,
                    (*signer).depth,
                );
                return crate::types::V_ERR as i32;
            }
        }
    }
    unsafe {
        *certType = GetCertTypeBySourceName(trust_cert);
    }
    crate::types::V_OK as i32
}

fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const _) };
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe {
        crate::src_app_verify::GetAppCertTypeBySignInfo(
            (*sri).signers as *const crate::types::SignerResovledInfo,
            certType,
        )
    };
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
    crate::types::V_OK as i32
}

fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7) };
    if sri.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const i8,
            b"GetProfileSingerCertType\0".as_ptr() as *const i8,
            474,
        ) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe {
        crate::src_app_verify::GetProfileCertTypeBySignInfo(
            (*sri).signers,
            certType,
        )
    };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const i8,
            b"GetProfileSingerCertType\0".as_ptr() as *const i8,
            479,
            ret,
        ) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
    crate::types::V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_18
// c_function: VerifyProfileSignGetRaw
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32
// c_first_line: static int32_t VerifyProfileSignGetRaw(const char *buf, int32_t len, char **profileContent, int32_t *contentLen)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_18/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `goto_exit` in this scope
//      --> src/src_app_verify.rs:662:9
//       |
//       |         ^^^^^^^^^
//       |
//       = help: have you added the `#[macro_use]` on the module/import?
//   error: cannot find macro `goto_exit` in this scope
//      --> src/src_app_verify.rs:668:9
// =================================
fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_18
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_18/translated_rust.rs
 * ------------------------------------------------------------
fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    let mut profile_data: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut cert_type: i32 = 0;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut input_len: crate::types::size_t = 0;
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 496) };
        return crate::types::V_ERR as i32;
    }
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 499) };
        goto_exit!();
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 parse message success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 501) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const crate::types::Pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 505) };
        goto_exit!();
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Verify certs success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 507) };
    ret = crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut cert_type as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 510) };
        goto_exit!();
    }
    if cert_type == crate::types::CERT_TYPE_OTHER as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert type invalid\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 513) };
        ret = crate::types::V_ERR as i32;
        goto_exit!();
    }
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const crate::types::Pkcs7, crate::src_app_verify::CalcDigest);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 518) };
        goto_exit!();
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 519) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const crate::types::Pkcs7, &mut input as *mut *mut ::core::ffi::c_uchar, &mut input_len as *mut crate::types::size_t);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 523) };
        goto_exit!();
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get profile sign content ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 525) };
    if input_len > crate::types::MAX_PROFILE_SIZE as crate::types::size_t || input_len == 0 {
        ret = crate::types::V_ERR as i32;
        goto_exit!();
    }
    profile_data = unsafe { libc::malloc((input_len + 1) as usize) } as *mut std::ffi::c_char;
    if profile_data.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profileData is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 532) };
        goto_exit!();
    }
    ret = unsafe { crate::compat::memcpy_s(profile_data as *mut std::ffi::c_void, input_len as u32, input as *const std::ffi::c_void, input_len as u32) };
    unsafe { *profile_data.offset(input_len as isize) = 0 };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 536) };
        goto_exit!();
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    }
    unsafe { *profileContent = profile_data };
    unsafe { *contentLen = input_len as i32 };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile get raw data ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 542) };
    return crate::types::V_OK as i32;
    macro_rules! goto_exit {
        () => {
            {
                crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
                if !pkcs7.is_null() {
                    unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
                }
                if !profile_data.is_null() {
                    unsafe { libc::free(profile_data as *mut std::ffi::c_void) };
                }
                return crate::types::V_ERR as i32;
            }
        };
    }
    goto_exit!();
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_18
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetRsaPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *const u8 {
    let buf_size = (1024 * 2 + 20) as usize;
    let buf = unsafe { libc::malloc(buf_size) } as *mut u8;
    if buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 554) };
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 559) };
        unsafe { libc::free(buf as *mut std::ffi::c_void) };
        return std::ptr::null();
    }
    let mut c = unsafe { buf.add(buf_size) };
    let pk_len = unsafe { crate::compat::mbedtls_pk_write_pubkey(&mut c as *mut *mut u8, buf, pk) };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetRsaPk pkLen %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 565, pk_len) };
    if pk_len < 0 || pk_len > buf_size as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get pk buf error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 567) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void) };
        return std::ptr::null();
    }
    let pk_buf = unsafe { libc::malloc(pk_len as usize) } as *mut u8;
    if pk_buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 574) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void) };
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memcpy_s(pk_buf as *mut std::ffi::c_void, pk_len as crate::types::size_t, c as *const std::ffi::c_void, pk_len as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 581, ret) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void) };
        unsafe { libc::free(pk_buf as *mut std::ffi::c_void) };
        return std::ptr::null();
    }
    if !len.is_null() {
        unsafe { *len = pk_len };
    }
    let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    unsafe { libc::free(buf as *mut std::ffi::c_void) };
    pk_buf as *const u8
}

fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    use crate::compat::*;
    use crate::globals::*;
    use libc::{free, malloc};
    let ecCtx = unsafe {
        match crate::compat::mbedtls_pk_get_type(pk) {
            1 | 2 | 3 => (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair,
            _ => std::ptr::null_mut(),
        }
    };
    if ecCtx.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ec pk error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 597) };
        return std::ptr::null_mut();
    }
    let buf_size = (2 * ((521 + 7) / 8) + 1) as usize;
    let buf = unsafe { malloc(buf_size) as *mut u8 };
    if buf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 602) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 607) };
        unsafe { free(buf as *mut std::ffi::c_void); }
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
    if ret != V_OK as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ecc pk key error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 614) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let len_val = unsafe { *len };
    let _ = unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetEcPk *len %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 619, len_val) };
    if len_val <= 0 || len_val > buf_size as i32 {
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let pkBuf = unsafe { malloc(len_val as usize) as *mut u8 };
    if pkBuf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 626) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let ret = unsafe { memcpy_s(pkBuf as *mut std::ffi::c_void, len_val as crate::types::size_t, buf as *const std::ffi::c_void, len_val as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 633, ret) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        unsafe { free(pkBuf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    unsafe { free(buf as *mut std::ffi::c_void); }
    pkBuf
}

fn GetPkBuf(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let mut bufA: *mut u8 = std::ptr::null_mut();
    let pk_type = unsafe { crate::compat::mbedtls_pk_get_type(pk) };
    if pk_type == crate::types::MBEDTLS_PK_RSA || pk_type == crate::types::MBEDTLS_PK_RSASSA_PSS {
        let rsa_buf = crate::src_app_verify::GetRsaPk(pk, len);
        bufA = rsa_buf as *mut u8;
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
    crate::types::V_OK as i32
}

fn FreeAppSignPublicKey(pk: *mut crate::types::AppSignPk) {
    if pk.is_null() {
        return;
    }
    unsafe {
        if (*pk).pk != std::ptr::null_mut() {
            libc::free((*pk).pk as *mut std::ffi::c_void);
            (*pk).pk = std::ptr::null_mut();
        }
    }
}

pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::{FreeAppSignPublicKey, GetAppSignPublicKey};
    if profile.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
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
    let ret = GetAppSignPublicKey(profile as *const _, &mut pk);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
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
        let _ = crate::compat::mbedtls_base64_encode(
            std::ptr::null_mut(),
            0,
            &mut use_len,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        );
    }
    let bundle_name_len = unsafe { libc::strlen((*profile).bundleInfo.bundleName) } as i32;
    let appid_len = bundle_name_len + use_len as i32 + 1 + 1;
    unsafe {
        let _ = crate::compat::HiLogPrint(
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
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR as i32;
    }
    let appid = unsafe { libc::malloc(appid_len as usize) } as *mut i8;
    if appid.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc failed\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                727,
            );
        }
        FreeAppSignPublicKey(&mut pk);
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
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                734,
                ret,
            );
        }
        if !appid.is_null() {
            unsafe { libc::free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    let ret = unsafe {
        crate::compat::mbedtls_base64_encode(
            appid.offset(bundle_name_len as isize + 1) as *mut u8,
            (appid_len - bundle_name_len - 1) as crate::types::size_t,
            &mut use_len,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: base 64 encode error\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                742,
            );
        }
        if !appid.is_null() {
            unsafe { libc::free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    unsafe {
        (*profile).appid = appid;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
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
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
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
    FreeAppSignPublicKey(&mut pk);
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
    let rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
    if rawBuf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"rawBuf\" is null\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 762);
        }
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"certType %d\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 763, certType);
    }
    if certType == 0 {
        profBuf = rawBuf;
        len = rawLen;
    } else {
        let mut contentLen: i32 = 0;
        let ret = crate::src_app_verify::VerifyProfileSignGetRaw(rawBuf as *const std::ffi::c_char, rawLen, &mut profBuf, &mut contentLen);
        if !rawBuf.is_null() {
            unsafe { libc::free(rawBuf as *mut std::ffi::c_void); }
        }
        if ret != crate::types::V_OK as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 772);
            }
            return ret;
        }
        len = contentLen;
    }
    let ret = crate::src_app_provision::ParseProfile(profBuf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"GetSignBlock error\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 777);
        }
        if !profBuf.is_null() {
            unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    if !profBuf.is_null() {
        unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
    }
    let ret = crate::src_app_provision::VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 784);
        }
        crate::src_app_provision::ProfFreeData(pf);
        return ret;
    }
    let ret = crate::src_app_verify::GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const std::ffi::c_char, b"VerifyProfGetContent\0".as_ptr() as *const std::ffi::c_char, 787);
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

        let pk_type = crate::compat::mbedtls_pk_get_type(&cert.pk);
        if pk_type != bin.pkType {
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
    use crate::types::{V_ERR, V_OK};
    if certBase64.is_null() || binSignCert.is_null() {
        return V_ERR as i32;
    }
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { ::core::mem::zeroed() };
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut cert);
    }
    let len = unsafe { libc::strlen(certBase64 as *const ::core::ffi::c_char) } as crate::types::size_t;
    let ret = unsafe {
        crate::compat::mbedtls_x509_crt_parse(&mut cert, certBase64, len.wrapping_add(1))
    };
    if ret != V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                846,
            )
        };
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_INFO as u32,
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
    let cmp = crate::src_app_verify::CmpCert(&cert as *const _, binSignCert);
    if cmp == V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_INFO as u32,
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
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_30
// c_function: CheckDebugAppSign
// rust_file: src_app_verify.rs
// rust_signature: fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32
// c_first_line: static int32_t CheckDebugAppSign(CertInfo *binSignCert, const ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_30/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//       --> src/src_app_verify.rs:1226:13
//        |
//        |             ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//       --> src/src_app_verify.rs:1214:21
//        |
//        |                     ^^^^^^^^^^^^
// =================================
fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_30
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_30/translated_rust.rs
 * ------------------------------------------------------------
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
                __FUNCTION__!(),
                885,
            );
            return crate::types::V_ERR as i32;
        }
        let mut ret = LoadCertAndCmpDest((*pf).bundleInfo.devCert, binSignCert);
        if ret == crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: dev cert consistent\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                890,
            );
            return crate::types::V_OK as i32;
        }
        if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) != 0 {
            ret = LoadCertAndCmpDest((*pf).bundleInfo.releaseCert, binSignCert);
            if ret == crate::types::V_OK as i32 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: release cert consistent\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
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
            __FUNCTION__!(),
            900,
        );
        crate::types::V_ERR as i32
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_30
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    if appCertType == 0 || appCertType == 1 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: app type : %d, return OK\0".as_ptr() as *const i8, b"CheckAppSignCertWithProfile\0".as_ptr() as *const i8, 908, appCertType) };
        return crate::types::V_OK as i32;
    }
    let mut ret = crate::types::V_ERR as i32;
    unsafe {
        if crate::compat::strcmp(b"debug\0".as_ptr() as *const i8, (*pf).type_) == 0 {
            ret = crate::src_app_verify::CheckDebugAppSign(binSignCert as *mut crate::types::CertInfo, pf);
        } else if crate::compat::strcmp(b"release\0".as_ptr() as *const i8, (*pf).type_) == 0 {
            ret = crate::src_app_verify::CheckReleaseAppSign(binSignCert, pf);
        }
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: check app sign cert ret : %d\0".as_ptr() as *const i8, b"CheckAppSignCertWithProfile\0".as_ptr() as *const i8, 920, ret) };
    ret
}

fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32 {
    if certInfo.is_null() {
        return 0;
    }
    let size = std::mem::size_of::<crate::types::CertInfo>();
    let ret = unsafe { crate::compat::memset_s(certInfo as *mut std::ffi::c_void, size as crate::types::size_t, 0, size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: memset error\0".as_ptr() as *const std::ffi::c_char, b"CertInfoInit\0".as_ptr() as *const std::ffi::c_char, 928) };
    }
    ret
}

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

fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    let cert_info_size = std::mem::size_of::<crate::types::CertInfo>();
    let cert_info = unsafe { libc::malloc(cert_info_size) } as *mut crate::types::CertInfo;
    if cert_info.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: certInfo is null\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            958,
        ) };
        return crate::types::V_ERR_MALLOC as i32;
    }
    let mut ret = crate::src_app_verify::CertInfoInit(cert_info);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: cert info init\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            962,
        ) };
        ret = crate::types::V_ERR_MEMSET as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        (*cert_info).issuerLen = (*ctr).issuer_raw.len as i32;
        (*cert_info).subjectLen = (*ctr).subject_raw.len as i32;
    }
    if unsafe { (*cert_info).issuerLen == 0 || (*cert_info).issuerLen > (1024 * 1024) ||
        (*cert_info).subjectLen == 0 || (*cert_info).subjectLen > (1024 * 1024) } {
        ret = crate::types::V_ERR_MALLOC as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        (*cert_info).issuer = libc::malloc(((*cert_info).issuerLen + 1) as usize) as *mut i8;
    }
    if unsafe { (*cert_info).issuer.is_null() } {
        ret = crate::types::V_ERR_MALLOC as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        *((*cert_info).issuer.offset((*cert_info).issuerLen as isize)) = 0;
        ret = crate::compat::memcpy_s(
            (*cert_info).issuer as *mut core::ffi::c_void,
            (*cert_info).issuerLen as crate::types::size_t,
            (*ctr).issuer_raw.p as *const core::ffi::c_void,
            (*ctr).issuer_raw.len,
        ) as i32;
    }
    if ret != 0 {
        ret = crate::types::V_ERR_MEMCPY as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        (*cert_info).subject = libc::malloc(((*cert_info).subjectLen + 1) as usize) as *mut i8;
    }
    if unsafe { (*cert_info).subject.is_null() } {
        ret = crate::types::V_ERR_MALLOC as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        *((*cert_info).subject.offset((*cert_info).subjectLen as isize)) = 0;
        ret = crate::compat::memcpy_s(
            (*cert_info).subject as *mut core::ffi::c_void,
            (*cert_info).subjectLen as crate::types::size_t,
            (*ctr).subject_raw.p as *const core::ffi::c_void,
            (*ctr).subject_raw.len,
        ) as i32;
    }
    if ret != 0 {
        ret = crate::types::V_ERR_MEMCPY as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        (*cert_info).pkType = crate::compat::mbedtls_pk_get_type(&(*ctr).pk);
        (*cert_info).pkBuf = crate::src_app_verify::GetPkBuf(&(*ctr).pk, &mut (*cert_info).pkLen) as *mut i8;
    }
    if unsafe { (*cert_info).pkBuf.is_null() } {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: get pk error\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            998,
        ) };
        ret = crate::types::V_ERR as i32;
        crate::src_app_verify::FreeCertInfo(cert_info);
        unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
        return ret;
    }
    unsafe {
        *binSignCert = cert_info;
    }
    return crate::types::V_OK as i32;
}

fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: VerifyProfGetContent error: %d\0".as_ptr() as *const _, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const _, 1015, ret) };
        return ret;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify prof get content success\0".as_ptr() as *const _, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const _, 1018) };

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: CheckAppSignCertWithProfile error: %d\0".as_ptr() as *const _, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const _, 1023, ret) };
        unsafe { crate::src_app_provision::ProfFreeData(pf) };
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

    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verfiy app source success\0".as_ptr() as *const _, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const _, 1032) };
    return crate::types::V_OK as i32;
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_36
// c_function: VerifyAppSignPkcsData
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32
// c_first_line: static int32_t VerifyAppSignPkcsData(const FileRead *fileRead, const SignatureInfo *signInfo, const Pkcs7 *pkcs7Handle)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_app_verify.rs:1717:83
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
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
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
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"GetBinSignPkcs\0".as_ptr() as *const _, 1066) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(signBuf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const _, b"GetBinSignPkcs\0".as_ptr() as *const _, 1071, ret) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        if !pkcs7.is_null() {
            unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        }
        return std::ptr::null_mut();
    }
    pkcs7
}

fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead {
    let fileRead = unsafe { libc::malloc(std::mem::size_of::<crate::types::FileRead>()) } as *mut crate::types::FileRead;
    if fileRead.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"GetFileRead\0".as_ptr() as *const _, 1084) };
        return std::ptr::null_mut();
    }
    unsafe {
        (*fileRead).fp = fp;
        (*fileRead).offset = offset;
        (*fileRead).len = size;
    }
    fileRead
}

fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32;

    let signBuf = crate::src_app_verify::GetSignBlockByType(signInfo as *const _, fp, crate::types::SIGNATURE_BLOCK_TYPE as i32, &mut blockLen, &mut blockHead);
    if signBuf.is_null() {
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = crate::src_app_verify::GetBinSignPkcs(signBuf, blockLen);
    if pkcs7.is_null() {
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    fileRead = crate::src_app_verify::GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        ret = crate::types::V_ERR_MALLOC as i32;
        unsafe {
            if !signBuf.is_null() {
                libc::free(signBuf as *mut std::ffi::c_void);
            }
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut std::ffi::c_void);
        }
        return ret;
    }

    ret = crate::src_app_verify::GetAppSingerCertType(pkcs7, certType);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        unsafe {
            if !signBuf.is_null() {
                libc::free(signBuf as *mut std::ffi::c_void);
            }
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut std::ffi::c_void);
            libc::free(fileRead as *mut std::ffi::c_void);
        }
        return ret;
    }

    unsafe { (*signInfo).certType = *certType; }

    ret = crate::src_app_verify::VerifyAppSignPkcsData(fileRead as *const _, signInfo as *const _, pkcs7 as *const _);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
        unsafe {
            if !signBuf.is_null() {
                libc::free(signBuf as *mut std::ffi::c_void);
            }
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut std::ffi::c_void);
            libc::free(fileRead as *mut std::ffi::c_void);
        }
        return ret;
    }

    unsafe {
        let crt_ptr = (*pkcs7).signedData.signers.certPath.crt;
        ret = crate::src_app_verify::GetCertInfo(crt_ptr as *const _, signCert);
    }
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_INFO as i32;
        unsafe {
            if !signBuf.is_null() {
                libc::free(signBuf as *mut std::ffi::c_void);
            }
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut std::ffi::c_void);
            libc::free(fileRead as *mut std::ffi::c_void);
        }
        return ret;
    }

    unsafe {
        if !signBuf.is_null() {
            libc::free(signBuf as *mut std::ffi::c_void);
        }
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        libc::free(pkcs7 as *mut std::ffi::c_void);
        libc::free(fileRead as *mut std::ffi::c_void);
    }
    ret
}

fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: verify bin sign error\0".as_ptr() as *const _,
                b"VerifyIntegrity\0".as_ptr() as *const _,
                1158,
            );
        }
        return ret;
    }
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const _, certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const _,
                b"VerifyIntegrity\0".as_ptr() as *const _,
                1164,
                ret,
            );
        }
        crate::src_app_verify::FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe {
                libc::free(binSignCert as *mut _);
            }
            binSignCert = std::ptr::null_mut();
        }
        return ret;
    }
    crate::src_app_verify::FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe {
            libc::free(binSignCert as *mut _);
        }
        binSignCert = std::ptr::null_mut();
    }
    crate::types::V_OK as i32
}

pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    if filePath.is_null() || verifyRst.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut handle: i32 = 0;
    let mut file = crate::types::FileRead {
        fp: 0,
        offset: 0,
        len: 0,
    };
    if crate::src_app_file::InitVerify(&mut file as *mut crate::types::FileRead, filePath, &mut handle as *mut i32) != crate::types::V_OK as i32 {
        unsafe {
            libc::close(handle);
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut signInfo = crate::types::SignatureInfo {
        signHead: std::ptr::null_mut(),
        fullSignBlockOffset: 0,
        hapCoreDirOffset: 0,
        hapEocdOffset: 0,
        hapEocdSize: 0,
        fileSize: 0,
        version: 0,
        certType: 0,
    };
    let mut ret = crate::src_app_verify::GetSignHead(&file as *const crate::types::FileRead, &mut signInfo as *mut crate::types::SignatureInfo);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: get sign head error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1188);
            libc::close(handle);
        }
        return ret;
    }
    let signHead = signInfo.signHead;
    ret = crate::src_app_verify::VerifyIntegrity(&mut signInfo as *mut crate::types::SignatureInfo, handle, unsafe { &mut (*verifyRst).profile } as *mut crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: verify integrity failed\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1195);
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
            }
        }
        return ret;
    }
    let fileSt = unsafe { libc::malloc(std::mem::size_of::<crate::compat::stat>()) } as *mut crate::compat::stat;
    if fileSt.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: malloc error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1202);
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
            }
            crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
        }
        return crate::types::V_ERR_MALLOC as i32;
    }
    ret = unsafe { libc::fstat(handle, fileSt as *mut libc::stat) };
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: fstat error\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1210);
            libc::close(handle);
            if !signHead.is_null() {
                libc::free(signHead as *mut std::ffi::c_void);
            }
            crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
            libc::free(fileSt as *mut std::ffi::c_void);
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: file len: %d\0".as_ptr() as *const ::core::ffi::c_char, b"APPVERI_AppVerify\0".as_ptr() as *const ::core::ffi::c_char, 1217, (*fileSt).st_size as i32);
        libc::close(handle);
        if !signHead.is_null() {
            libc::free(signHead as *mut std::ffi::c_void);
        }
        libc::free(fileSt as *mut std::ffi::c_void);
    }
    ret
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_45
// c_function: APPVERI_FreeVerifyRst
// rust_file: src_app_verify.rs
// rust_signature: pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult)
// c_first_line: void APPVERI_FreeVerifyRst(VerifyResult *verifyRst)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_45/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//       --> src/src_app_verify.rs:1187:9
//        |
//        |         ^^^^^^^^^^^^
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 16 warnings emitted
// =================================
pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_45
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_45/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: free verify rst data\0".as_ptr() as *const i8,
        __FUNCTION__!(),
        1256,
    );
    unsafe {
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_45
 * === C2R_LLM_FAILED_OUTPUT_END === */

