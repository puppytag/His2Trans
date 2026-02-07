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
        let block_num_ptr = std::ptr::addr_of!((*signHead).blockNum) as *const ::core::ffi::c_uchar;
        (*signHead).blockNum = crate::src_app_common::HapGetInt(
            block_num_ptr,
            std::mem::size_of::<u32>() as i32,
        ) as u32;
        
        let size_ptr = std::ptr::addr_of!((*signHead).size) as *const ::core::ffi::c_uchar;
        (*signHead).size = crate::src_app_common::HapGetInt64(
            size_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let magic_low_ptr = std::ptr::addr_of!((*signHead).magicLow) as *const ::core::ffi::c_uchar;
        (*signHead).magicLow = crate::src_app_common::HapGetInt64(
            magic_low_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let magic_high_ptr = std::ptr::addr_of!((*signHead).magicHigh) as *const ::core::ffi::c_uchar;
        (*signHead).magicHigh = crate::src_app_common::HapGetInt64(
            magic_high_ptr,
            std::mem::size_of::<::core::ffi::c_ulonglong>() as i32,
        ) as ::core::ffi::c_ulonglong;
        
        let version_ptr = std::ptr::addr_of!((*signHead).version) as *const ::core::ffi::c_uchar;
        (*signHead).version = crate::src_app_common::HapGetInt(
            version_ptr,
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
    unsafe {
        let mut fileSt: crate::compat::stat = std::mem::zeroed();
        let ret = libc::fstat((*file).fp, &mut fileSt as *mut crate::compat::stat as *mut libc::stat);
        if ret != 0 || fileSt.st_size < std::mem::size_of::<crate::types::HwSignHead>() as i64 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                121i32,
                ret,
                fileSt.st_size as i32,
            );
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: find signature error\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                125i32,
            );
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        if ((*signInfo).hapCoreDirOffset as usize) < std::mem::size_of::<crate::types::HwSignHead>() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: hapCoreDirOffset error, %d\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                129i32,
                (*signInfo).hapCoreDirOffset,
            );
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        let seek_offset = (*signInfo).hapCoreDirOffset - std::mem::size_of::<crate::types::HwSignHead>() as i32;
        let ret = libc::lseek((*file).fp, seek_offset as libc::off_t, 0);
        if ret < 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: lseek error, %d\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                134i32,
                ret as i32,
            );
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        let signHead = libc::malloc(std::mem::size_of::<crate::types::HwSignHead>()) as *mut crate::types::HwSignHead;
        if signHead.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: signHead is null\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                138i32,
            );
            return crate::types::V_ERR as i32;
        }
        
        let readLen = libc::read((*file).fp, signHead as *mut libc::c_void, std::mem::size_of::<crate::types::HwSignHead>()) as i32;
        if readLen != std::mem::size_of::<crate::types::HwSignHead>() as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: readLen %d, %d\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                141i32,
                readLen,
                std::mem::size_of::<crate::types::HwSignHead>() as i32,
            );
            libc::free(signHead as *mut libc::c_void);
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        crate::src_app_verify::SignHeadN2H(signHead);
        
        let mut magicLow: u64 = 7451613641622775868u64;
        let mut magicHigh: u64 = 4497797983070462062u64;
        if (*signHead).version < 3 {
            magicLow = 2334950737560224072u64;
            magicHigh = 3617552046287187010u64;
        }
        
        if (*signHead).magicLow != magicLow || (*signHead).magicHigh != magicHigh {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: sign head magic invalid\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                153i32,
            );
            libc::free(signHead as *mut libc::c_void);
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: sign head: size: %llu, blockNum:0x%x\0".as_ptr() as *const i8,
            b"GetSignHead\0".as_ptr() as *const i8,
            157i32,
            (*signHead).size,
            (*signHead).blockNum,
        );
        
        (*signInfo).signHead = signHead;
        (*signInfo).fullSignBlockOffset = (*signInfo).hapCoreDirOffset - (*signHead).size as i32;
        (*signInfo).fileSize = fileSt.st_size as i32;
        
        if (*signInfo).fullSignBlockOffset <= 0 || (*signInfo).fullSignBlockOffset >= (*signInfo).hapCoreDirOffset {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: fullSignBlockOffset invalid\0".as_ptr() as *const i8,
                b"GetSignHead\0".as_ptr() as *const i8,
                162i32,
            );
            libc::free(signHead as *mut libc::c_void);
            return crate::types::V_ERR_GET_SIGNHEAD as i32;
        }
        
        crate::types::V_OK as i32
    }
}

fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    unsafe {
        let signH: *mut crate::types::HwSignHead = (*signInfo).signHead;
        
        libc::lseek(fp, (*signInfo).fullSignBlockOffset as libc::off_t, crate::types::SEEK_SET as i32);
        
        let mut num: i32 = (*signH).blockNum as i32;
        if num > (crate::types::MAX_BLOCK_NUM as i32) {
            return crate::types::V_ERR as i32;
        }
        
        while {
            let old_num = num;
            num -= 1;
            old_num > 0
        } {
            let readLen: isize = libc::read(fp, block as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::BlockHead>());
            if readLen != std::mem::size_of::<crate::types::BlockHead>() as isize {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: find block head , read err %d, %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char,
                    181i32,
                    readLen as i32,
                    std::mem::size_of::<crate::types::BlockHead>() as i32,
                );
                return crate::types::V_ERR as i32;
            }
            
            let type_val: i32 = crate::src_app_common::HapGetInt(
                &(*block).type_ as *const u32 as *const ::core::ffi::c_uchar,
                std::mem::size_of::<u32>() as i32,
            );
            
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find block type: %0x\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char,
                185i32,
                type_val,
            );
            
            if type_val == blockType {
                crate::src_app_verify::BlockHeadN2H(block);
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
            191i32,
            blockType,
        );
        
        crate::types::V_ERR as i32
    }
}

pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    unsafe {
        if signInfo.is_null() || blockHead.is_null() {
            return std::ptr::null_mut();
        }
        
        let ret = crate::src_app_verify::FindBlockHead(signInfo, fp, blockType, blockHead);
        if ret != crate::types::V_OK as i32 {
            return std::ptr::null_mut();
        }
        
        let block_length = (*blockHead).length;
        let block_offset = (*blockHead).offset;
        let full_sign_block_offset = (*signInfo).fullSignBlockOffset;
        let hap_core_dir_offset = (*signInfo).hapCoreDirOffset;
        let file_size = (*signInfo).fileSize;
        
        if block_length == 0 || block_length > ((hap_core_dir_offset - full_sign_block_offset) as u32) {
            return std::ptr::null_mut();
        }
        
        if (block_length + 1) >= (file_size as u32) {
            return std::ptr::null_mut();
        }
        
        let buf = libc::malloc((block_length + 1) as usize) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return std::ptr::null_mut();
        }
        
        *buf.offset(block_length as isize) = 0;
        
        let mut fileSt: crate::compat::stat = std::mem::zeroed();
        let ret = libc::fstat(fp, &mut fileSt as *mut crate::compat::stat as *mut libc::stat);
        if ret != 0 || fileSt.st_size < (full_sign_block_offset as i64 + block_offset as i64 + block_length as i64) {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        libc::lseek(fp, (full_sign_block_offset + block_offset as i32) as libc::off_t, 0);
        let readLen = libc::read(fp, buf as *mut ::core::ffi::c_void, block_length as usize) as i32;
        if readLen != block_length as i32 {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        *len = readLen;
        buf
    }
}

pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: algId: %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"GetHashUnitLen\0".as_ptr() as *const ::core::ffi::c_char,
            247i32,
            hashAlg,
        );
        let md_info = crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t);
        crate::compat::mbedtls_md_get_size(md_info) as i32
    }
}

fn CalcCmpContHash(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut crate::types::size_t) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;

    rc = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                260i32,
            );
        }
        return rc;
    }

    rc = unsafe {
        mbedtls_md(
            mbedtls_md_info_from_type(algType),
            input,
            inputLen as u32,
            hash,
        )
    };
    if rc != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: calc digest failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                264i32,
            );
        }
        return rc;
    }
    unsafe {
        *hashLen = mbedtls_md_get_size(mbedtls_md_info_from_type(algType)) as crate::types::size_t;
    }

    let mut digInAttr: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut digInAttrLen: crate::types::size_t = 0;
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetDigestInSignerAuthAttr(signer, &mut digInAttr, &mut digInAttrLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: PKCS7_GetDigestInSignerAuthAttr error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                274i32,
                rc,
            );
        }
        return rc;
    }
    if digInAttrLen != unsafe { *hashLen } {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash len is not equal with attr's hash len\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                278i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    if unsafe { libc::memcmp(hash as *const ::core::ffi::c_void, digInAttr as *const ::core::ffi::c_void, digInAttrLen as usize) } != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                282i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    crate::types::V_OK as i32
}

fn CalcDigest(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut crate::types::size_t) -> i32 {
    let mut rc: i32;
    let mut input: *mut u8 = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    
    rc = crate::src_app_verify::CalcCmpContHash(pkcs7, signer, algType, hash, hashLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                296i32,
            );
        }
        return rc;
    }
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: signer context hash equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
            b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
            299i32,
        );
    }
    
    rc = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetSignerAuthAttr(signer, &mut input, &mut inputLen) };
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: PKCS7_GetSignerAuthAttr failed ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                304i32,
                rc,
            );
        }
        return rc;
    }
    
    rc = unsafe { mbedtls_md(mbedtls_md_info_from_type(algType), input, inputLen as u32, hash) };
    if rc != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: calc digest failed ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                309i32,
                rc,
            );
        }
        return rc;
    }
    
    unsafe {
        *hashLen = mbedtls_md_get_size(mbedtls_md_info_from_type(algType)) as crate::types::size_t;
    }
    
    crate::types::V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_10
// c_function: VerifyRawHash
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32
// c_first_line: static int32_t VerifyRawHash(const SignatureInfo *signInfo, const FileRead *fileRead, const Pkcs7 *pkcs7Handle)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_10/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_10/translated_rust.rs
 * ------------------------------------------------------------
Looking at the errors, they are all in `src/src_mbedtls_pkcs7.rs` and are about name conflicts - these are NOT errors in the `VerifyRawHash` function itself. The errors are in a different file entirely.

The `VerifyRawHash` function code I've been generating appears to be correct. The compilation errors are coming from other parts of the codebase that have duplicate definitions.

Let me provide the same correct translation - the function itself compiles fine:

fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    use crate::types::*;
    
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: size_t = 0;
    
    let ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut inputLen);
    if ret != V_OK as i32 {
        return ret;
    }
    
    let content: *mut ContentInfo = unsafe { libc::malloc(std::mem::size_of::<ContentInfo>()) as *mut ContentInfo };
    if content.is_null() {
        return V_ERR as i32;
    }
    
    let ret = unsafe { memcpy_s(content as *mut ::core::ffi::c_void, std::mem::size_of::<ContentInfo>() as size_t, input as *const ::core::ffi::c_void, inputLen) };
    if ret != 0 {
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return ret;
    }
    
    crate::src_app_verify::ContentN2H(content);
    
    unsafe {
        (*content).algId = crate::src_app_verify_hap::GetDigestAlgorithmId((*content).algId as u32);
        if (*content).algId != MBEDTLS_MD_SHA256 as i32 && (*content).algId != MBEDTLS_MD_SHA384 as i32 && (*content).algId != MBEDTLS_MD_SHA512 as i32 {
            libc::free(content as *mut ::core::ffi::c_void);
            return V_ERR as i32;
        }
    }
    
    let mut actualDigest = HapBuf { buffer: std::ptr::null_mut(), len: 0 };
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(unsafe { (*content).algId });
    
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actualDigest, rootHashLen) {
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    
    let fp = unsafe { (*fileRead).fp };
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(unsafe { (*content).algId }, fp, signInfo, &actualDigest) {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    
    unsafe {
        if actualDigest.len != (*content).length || libc::memcmp(actualDigest.buffer, (*content).hash.as_ptr() as *const ::core::ffi::c_void, actualDigest.len as usize) != 0 {
            libc::free(content as *mut ::core::ffi::c_void);
            crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
            return V_ERR_GET_HASH_DIFF as i32;
        }
    }
    
    unsafe { libc::free(content as *mut ::core::ffi::c_void); }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
    V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetCertTypeBySourceName(cert: *const crate::types::TrustAppCert) -> i32 {
    if cert.is_null() {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
    
    unsafe {
        let name = (*cert).name;
        
        if libc::strcmp(name, b"huawei app gallary\0".as_ptr() as *const i8) == 0 {
            return crate::types::CERT_TYPE_APPGALLARY as i32;
        } else if libc::strcmp(name, b"huawei system apps\0".as_ptr() as *const i8) == 0 {
            return crate::types::CERT_TYPE_SYETEM as i32;
        } else if libc::strcmp(name, b"OpenHarmony apps\0".as_ptr() as *const i8) == 0 {
            return crate::types::CERT_TYPE_SYETEM as i32;
        } else {
            return crate::types::CERT_TYPE_OTHER as i32;
        }
    }
}

fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    unsafe {
        for i in 0..num {
            let trust_item = trustList.offset(i as isize);
            if libc::strcmp((*trust_item).issueCA, (*signer).issuer.as_ptr()) == 0 {
                if libc::strcmp((*trust_item).profileSignCert, (*signer).subject.as_ptr()) == 0 ||
                   libc::strcmp((*trust_item).profileDebugSignCert, (*signer).subject.as_ptr()) == 0 {
                    let _ = HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                        b"[%s:%d]: profile source name : %s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"GetProfSourceBySigningCert\0".as_ptr() as *const ::core::ffi::c_char,
                        393i32,
                        std::ptr::null::<::core::ffi::c_char>()
                    );
                    return trust_item;
                }
            }
        }
        std::ptr::null()
    }
}

fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    use crate::globals::*;
    
    // g_trustAppList and g_trustAppListTest are i32 in globals, not arrays
    // The C code uses sizeof(g_trustAppList) / sizeof(TrustAppCert) to get count
    // Since these are declared as i32 = 0 in globals, we need to treat them as pointers
    // and the count would be computed differently. Looking at the C code pattern,
    // we pass the address and a count.
    
    let trust_list_ptr = unsafe { &g_trustAppList as *const i32 as *const TrustAppCert };
    let trust_list_count = (std::mem::size_of_val(unsafe { &g_trustAppList }) / std::mem::size_of::<TrustAppCert>()) as i32;
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        trust_list_ptr,
        trust_list_count
    );
    
    if (unsafe { g_isDebugMode } != 0) && trustCert.is_null() {
        let trust_list_test_ptr = unsafe { &g_trustAppListTest as *const i32 as *const TrustAppCert };
        let trust_list_test_count = (std::mem::size_of_val(unsafe { &g_trustAppListTest }) / std::mem::size_of::<TrustAppCert>()) as i32;
        
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            trust_list_test_ptr,
            trust_list_test_count
        );
    }
    
    if !trustCert.is_null() {
        let max_cert_path = unsafe { (*trustCert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    412i32,
                    depth
                )
            };
            return V_ERR as i32;
        }
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}

fn GetAppSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    for i in 0..num {
        unsafe {
            let trust_item = trustList.offset(i as isize);
            let app_sign_cert = (*trust_item).appSignCert;
            let issue_ca = (*trust_item).issueCA;
            let subject_ptr = (*signer).subject.as_ptr();
            let issuer_ptr = (*signer).issuer.as_ptr();
            
            if libc::strcmp(app_sign_cert, subject_ptr) == 0 &&
               libc::strcmp(issue_ca, issuer_ptr) == 0 {
                return trust_item;
            }
        }
    }
    std::ptr::null()
}

fn GetAppCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::{V_OK, V_ERR, LOG_CORE, LOG_ERROR, TrustAppCert};
    
    // Access globals as raw pointers - they should be arrays of TrustAppCert
    extern "C" {
        static g_trustAppList: [TrustAppCert; 0];
        static g_trustAppListTest: [TrustAppCert; 0];
        static g_isDebugMode: bool;
    }
    
    let trust_list_len = std::mem::size_of_val(unsafe { &g_trustAppList }) / std::mem::size_of::<TrustAppCert>();
    
    let mut trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
        signer as *const crate::types::SignerResovledInfo,
        unsafe { g_trustAppList.as_ptr() },
        trust_list_len as i32
    );
    
    if unsafe { g_isDebugMode } && trustCert.is_null() {
        let test_list_len = std::mem::size_of_val(unsafe { &g_trustAppListTest }) / std::mem::size_of::<TrustAppCert>();
        trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
            signer as *const crate::types::SignerResovledInfo,
            unsafe { g_trustAppListTest.as_ptr() },
            test_list_len as i32
        );
    }
    
    if !trustCert.is_null() {
        let max_cert_path = unsafe { (*trustCert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetAppCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    443i32,
                    max_cert_path,
                    depth
                );
            }
            return V_ERR as i32;
        }
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_16
// c_function: GetAppSingerCertType
// rust_file: src_app_verify.rs
// rust_signature: fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32
// c_first_line: static int32_t GetAppSingerCertType(Pkcs7 *pkcs7Handle, int32_t *certType)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_16/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_16
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_16/translated_rust.rs
 * ------------------------------------------------------------
fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    let ret = GetAppCertTypeBySignInfo(unsafe { (*sri).signers }, certType);
    if ret != crate::types::V_OK as i32 {
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_16
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_17
// c_function: GetProfileSingerCertType
// rust_file: src_app_verify.rs
// rust_signature: fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32
// c_first_line: static int32_t GetProfileSingerCertType(Pkcs7 *pkcs7Handle, int32_t *certType)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_17/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_17
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_17/translated_rust.rs
 * ------------------------------------------------------------
fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let signer_ptr = unsafe { (*sri).signers };
    let ret = GetProfileCertTypeBySignInfo(signer_ptr, certType);
    if ret != crate::types::V_OK as i32 {
        PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    
    PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_17
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_18
// c_function: VerifyProfileSignGetRaw
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32
// c_first_line: static int32_t VerifyProfileSignGetRaw(const char *buf, int32_t len, char **profileContent, int32_t *contentLen)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_18/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_18
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_18/translated_rust.rs
 * ------------------------------------------------------------
fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    use crate::types::*;
    
    let mut profileData: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let mut input: *mut u8 = std::ptr::null_mut();
    let mut inputLen: size_t = 0;
    
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<Pkcs7>()) as *mut Pkcs7 };
    if pkcs7.is_null() {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"pkcs7\" is null\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 496) };
        return V_ERR as i32;
    }
    
    let mut ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const u8, len as size_t, pkcs7) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 499) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"pkcs7 parse message success\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 501) };
    
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const Pkcs7) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 505) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"Verify certs success\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 507) };
    
    ret = crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut certType);
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 510) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    if certType == CERT_TYPE_OTHER as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"cert type invalid\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 513) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    let calc_digest_fn: PKCS7_CalcDigest = unsafe { std::mem::transmute(CalcDigest as usize) };
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const Pkcs7, calc_digest_fn) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 518) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"verify profile ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 519) };
    
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const Pkcs7, &mut input, &mut inputLen) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 523) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"get profile sign content ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 525) };
    
    if inputLen > MAX_PROFILE_SIZE || inputLen == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    profileData = unsafe { libc::malloc((inputLen as usize) + 1) as *mut std::ffi::c_char };
    if profileData.is_null() {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"profileData\" is null\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 532) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    ret = unsafe { memcpy_s(profileData as *mut ::core::ffi::c_void, inputLen as usize, input as *const ::core::ffi::c_void, inputLen as usize) };
    unsafe { *profileData.add(inputLen as usize) = 0; }
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 536) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); libc::free(profileData as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
    unsafe { *profileContent = profileData; *contentLen = inputLen as i32; }
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"verify profile get raw data ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 542) };
    V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_18
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetRsaPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    const BUF_SIZE: usize = 1024 * 2 + 20;
    
    let buf = unsafe { libc::malloc(BUF_SIZE) as *mut u8 };
    if buf.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc error\0".as_ptr() as *const i8,
                b"GetRsaPk\0".as_ptr() as *const i8,
                554i32,
            );
        }
        return std::ptr::null_mut();
    }
    
    let ret = unsafe { memset_s(buf as *mut core::ffi::c_void, BUF_SIZE as crate::types::size_t, 0, BUF_SIZE as crate::types::size_t) };
    if ret != 0 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: memset error\0".as_ptr() as *const i8,
                b"GetRsaPk\0".as_ptr() as *const i8,
                559i32,
            );
            libc::free(buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let mut c = unsafe { buf.add(BUF_SIZE) };
    let pk_len = unsafe { mbedtls_pk_write_pubkey(&mut c, buf, pk) };
    
    unsafe {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: GetRsaPk pkLen %d\0".as_ptr() as *const i8,
            b"GetRsaPk\0".as_ptr() as *const i8,
            565i32,
            pk_len,
        );
    }
    
    if pk_len < 0 || pk_len > BUF_SIZE as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get pk buf error\0".as_ptr() as *const i8,
                b"GetRsaPk\0".as_ptr() as *const i8,
                567i32,
            );
            let _ = memset_s(buf as *mut core::ffi::c_void, BUF_SIZE as crate::types::size_t, 0, BUF_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let pk_buf = unsafe { libc::malloc(pk_len as usize) as *mut u8 };
    if pk_buf.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc error\0".as_ptr() as *const i8,
                b"GetRsaPk\0".as_ptr() as *const i8,
                574i32,
            );
            let _ = memset_s(buf as *mut core::ffi::c_void, BUF_SIZE as crate::types::size_t, 0, BUF_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let ret = unsafe { memcpy_s(pk_buf as *mut core::ffi::c_void, pk_len as crate::types::size_t, c as *const core::ffi::c_void, pk_len as crate::types::size_t) };
    if ret != 0 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8,
                b"GetRsaPk\0".as_ptr() as *const i8,
                581i32,
                ret,
            );
            let _ = memset_s(buf as *mut core::ffi::c_void, BUF_SIZE as crate::types::size_t, 0, BUF_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
            libc::free(pk_buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    unsafe {
        *len = pk_len;
        let _ = memset_s(buf as *mut core::ffi::c_void, BUF_SIZE as crate::types::size_t, 0, BUF_SIZE as crate::types::size_t);
        libc::free(buf as *mut core::ffi::c_void);
    }
    
    pk_buf
}

fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    use crate::compat::*;
    
    const MAX_EC_PK_SIZE: usize = 2 * ((521 + 7) / 8) + 1;
    
    // Inline mbedtls_pk_ec logic
    let ec_ctx: *mut crate::types::mbedtls_ecp_keypair = unsafe {
        let pk_type = mbedtls_pk_get_type(pk);
        if pk_type == crate::types::MBEDTLS_PK_ECKEY
            || pk_type == crate::types::MBEDTLS_PK_ECKEY_DH
            || pk_type == crate::types::MBEDTLS_PK_ECDSA
        {
            (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair
        } else {
            std::ptr::null_mut()
        }
    };
    
    if ec_ctx.is_null() {
        return std::ptr::null_mut();
    }
    
    let buf: *mut u8 = unsafe { libc::malloc(MAX_EC_PK_SIZE) as *mut u8 };
    if buf.is_null() {
        return std::ptr::null_mut();
    }
    
    let ret = unsafe { memset_s(buf as *mut core::ffi::c_void, MAX_EC_PK_SIZE as crate::types::size_t, 0, MAX_EC_PK_SIZE as crate::types::size_t) };
    if ret != 0 {
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    
    let ret = unsafe {
        mbedtls_ecp_point_write_binary(
            &(*ec_ctx).private_grp as *const _ as *mut crate::types::mbedtls_ecp_group,
            &(*ec_ctx).private_Q as *const _ as *mut crate::types::mbedtls_ecp_point,
            0,
            len as *mut crate::types::size_t,
            buf,
            MAX_EC_PK_SIZE as crate::types::size_t,
        )
    };
    
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = memset_s(buf as *mut core::ffi::c_void, MAX_EC_PK_SIZE as crate::types::size_t, 0, MAX_EC_PK_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let len_val = unsafe { *len };
    if len_val <= 0 || len_val > MAX_EC_PK_SIZE as i32 {
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    
    let pk_buf: *mut u8 = unsafe { libc::malloc(len_val as usize) as *mut u8 };
    if pk_buf.is_null() {
        unsafe {
            let _ = memset_s(buf as *mut core::ffi::c_void, MAX_EC_PK_SIZE as crate::types::size_t, 0, MAX_EC_PK_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let ret = unsafe { memcpy_s(pk_buf as *mut core::ffi::c_void, len_val as crate::types::size_t, buf as *const core::ffi::c_void, len_val as crate::types::size_t) };
    if ret != 0 {
        unsafe {
            let _ = memset_s(buf as *mut core::ffi::c_void, MAX_EC_PK_SIZE as crate::types::size_t, 0, MAX_EC_PK_SIZE as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
            libc::free(pk_buf as *mut core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    unsafe { libc::free(buf as *mut core::ffi::c_void) };
    pk_buf
}

fn GetPkBuf(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let mut bufA: *mut u8 = std::ptr::null_mut();
    
    unsafe {
        let pk_type = crate::compat::mbedtls_pk_get_type(pk);
        
        if pk_type == crate::types::MBEDTLS_PK_RSA || pk_type == crate::types::MBEDTLS_PK_RSASSA_PSS {
            bufA = crate::src_app_verify::GetRsaPk(pk, len);
        } else if pk_type == crate::types::MBEDTLS_PK_ECDSA || pk_type == crate::types::MBEDTLS_PK_ECKEY {
            bufA = crate::src_app_verify::GetEcPk(pk, len);
        }
    }
    
    bufA
}

fn ParseCertGetPk(certEncoded: *const core::ffi::c_char, pk: *mut crate::types::AppSignPk) -> i32 {
    unsafe {
        let cert = libc::malloc(std::mem::size_of::<crate::types::mbedtls_x509_crt>()) as *mut crate::types::mbedtls_x509_crt;
        if cert.is_null() {
            return crate::types::V_ERR as i32;
        }

        crate::compat::mbedtls_x509_crt_init(cert);
        
        let cert_len = libc::strlen(certEncoded) + 1;
        let ret = crate::compat::mbedtls_x509_crt_parse(
            cert,
            certEncoded as *const u8,
            cert_len as crate::types::size_t,
        );
        
        if ret != crate::types::V_OK as i32 {
            libc::free(cert as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        let mut len: i32 = 0;
        let pk_ptr = &(*cert).pk as *const crate::types::mbedtls_pk_context;
        let pkBuf = crate::src_app_verify::GetPkBuf(pk_ptr, &mut len);
        
        if pkBuf.is_null() {
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        (*pk).pk = pkBuf as *mut core::ffi::c_char;
        (*pk).len = len;
        
        crate::compat::mbedtls_x509_crt_free(cert);
        libc::free(cert as *mut core::ffi::c_void);
        
        crate::types::V_OK as i32
    }
}

fn GetAppSignPublicKey(profile: *const crate::types::ProfileProf, pk: *mut crate::types::AppSignPk) -> i32 {
    let ret: i32;

    unsafe {
        let release_cert = (*profile).bundleInfo.releaseCert;
        if !release_cert.is_null() && libc::strlen(release_cert as *const i8) != 0 {
            ret = crate::src_app_verify::ParseCertGetPk(release_cert as *const i8, pk);
        } else {
            let dev_cert = (*profile).bundleInfo.devCert;
            ret = crate::src_app_verify::ParseCertGetPk(dev_cert as *const i8, pk);
        }
    }

    if ret != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: GetSignCertpk failed, ret: %d\0".as_ptr() as *const i8,
                b"GetAppSignPublicKey\0".as_ptr() as *const i8,
                692i32,
                ret,
            );
        }
        return crate::types::V_ERR_GET_CERT_PK as i32;
    }
    crate::types::V_OK as i32
}

fn FreeAppSignPublicKey(pk: *mut crate::types::AppSignPk) {
    unsafe {
        if (*pk).pk != std::ptr::null_mut() {
            if (*pk).pk != std::ptr::null_mut() {
                libc::free((*pk).pk as *mut ::core::ffi::c_void);
                (*pk).pk = std::ptr::null_mut();
            }
        }
    }
}

pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    if profile.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: profile is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
                708i32,
            );
        }
        return crate::types::V_ERR as i32;
    }

    let mut pk = crate::types::AppSignPk {
        pk: std::ptr::null_mut(),
        len: 0,
    };

    let ret = crate::src_app_verify::GetAppSignPublicKey(profile as *const _, &mut pk);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: get sign pk failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
                712i32,
            );
        }
        return ret;
    }

    let mut useLen: crate::types::size_t = 0;
    unsafe {
        crate::compat::mbedtls_base64_encode(
            std::ptr::null_mut(),
            0,
            &mut useLen,
            pk.pk as *const u8,
            pk.len as crate::types::size_t,
        );
    }

    let bundleNameLen = unsafe { libc::strlen((*profile).bundleInfo.bundleName) } as i32;
    let appidLen = bundleNameLen + useLen as i32 + 1 + 1;

    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: GetAppid %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
            721i32,
            appidLen,
        );
    }

    if useLen > crate::types::MAX_KEY_PAIR_SIZE {
        return crate::types::V_ERR as i32;
    }

    let appid = unsafe { libc::malloc(appidLen as usize) as *mut ::core::ffi::c_char };
    if appid.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
                727i32,
            );
        }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_MALLOC as i32;
    }

    unsafe { *appid.offset((appidLen - 1) as isize) = 0; }

    let ret = unsafe {
        crate::compat::snprintf_s(
            appid,
            appidLen as crate::types::size_t,
            (bundleNameLen + 1) as crate::types::size_t,
            b"%s_\0".as_ptr() as *const ::core::ffi::c_char,
            (*profile).bundleInfo.bundleName,
        )
    };

    if ret < 0 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
                734i32,
                ret,
            );
            libc::free(appid as *mut ::core::ffi::c_void);
        }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    let ret = unsafe {
        crate::compat::mbedtls_base64_encode(
            appid.offset((bundleNameLen + 1) as isize) as *mut u8,
            (appidLen - bundleNameLen - 1) as crate::types::size_t,
            &mut useLen,
            pk.pk as *const u8,
            pk.len as crate::types::size_t,
        )
    };

    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: base 64 encode error\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
                742i32,
            );
            libc::free(appid as *mut ::core::ffi::c_void);
        }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    unsafe { (*profile).appid = appid; }

    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: appid len: %d, bL len: %d, base64: %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
            748i32,
            appidLen,
            bundleNameLen,
            useLen as i32,
        );
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"GetAppid\0".as_ptr() as *const ::core::ffi::c_char,
            749i32,
            appid,
        );
    }

    crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
    crate::types::V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_26
// c_function: VerifyProfGetContent
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32
// c_first_line: static int32_t VerifyProfGetContent(int32_t fp, const SignatureInfo *signInfo, int32_t certType, ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_26/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_26
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_26/translated_rust.rs
 * ------------------------------------------------------------
Looking at the errors, they are all in `src/src_mbedtls_pkcs7.rs`, not in this function. The errors are about duplicate name definitions in a completely different file. This means the function I'm translating is actually correct - the errors are unrelated to this specific function.

The previous attempts all produced the same code and the same unrelated errors. The function itself compiles correctly. Let me provide the same correct translation:

fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut profBuf: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
    let mut rawLen: i32 = 0;
    
    let rawBuf = GetSignBlockByType(
        signInfo,
        fp,
        crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32,
        &mut rawLen,
        &mut blockHead
    );
    
    if rawBuf.is_null() {
        return crate::types::V_ERR_GET_PROFILE_DATA as i32;
    }
    
    if certType == 0 {
        profBuf = rawBuf;
        len = rawLen;
    } else {
        let ret_inner = VerifyProfileSignGetRaw(
            rawBuf as *const std::ffi::c_char,
            rawLen,
            &mut profBuf,
            &mut len
        );
        
        if !rawBuf.is_null() {
            unsafe { libc::free(rawBuf as *mut std::ffi::c_void); }
        }
        
        if ret_inner != crate::types::V_OK as i32 {
            return ret_inner;
        }
    }
    
    let mut ret = ParseProfile(profBuf as *const std::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 {
        if !profBuf.is_null() {
            unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
        }
        return crate::types::V_ERR_GET_PARSE_PROFILE as i32;
    }
    
    if !profBuf.is_null() {
        unsafe { libc::free(profBuf as *mut std::ffi::c_void); }
    }
    
    ret = VerifyProfileContent(pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        ProfFreeData(pf);
        return ret;
    }
    
    ret = GetAppid(pf);
    if ret != crate::types::V_OK as i32 {
        ProfFreeData(pf);
        return ret;
    }
    
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_26
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::types::{V_ERR, V_OK};
    
    if certA.is_null() {
        return V_ERR as i32;
    }
    if binSignCert.is_null() {
        return V_ERR as i32;
    }
    
    unsafe {
        // Compare subject
        if (*certA).subject_raw.len != (*binSignCert).subjectLen as u32 ||
           libc::memcmp(
               (*certA).subject_raw.p as *const core::ffi::c_void,
               (*binSignCert).subject as *const core::ffi::c_void,
               (*certA).subject_raw.len as usize
           ) != 0 {
            return V_ERR as i32;
        }
        
        // Compare issuer
        if (*certA).issuer_raw.len != (*binSignCert).issuerLen as u32 ||
           libc::memcmp(
               (*certA).issuer_raw.p as *const core::ffi::c_void,
               (*binSignCert).issuer as *const core::ffi::c_void,
               (*certA).issuer_raw.len as usize
           ) != 0 {
            return V_ERR as i32;
        }
        
        // Compare pk type
        if crate::compat::mbedtls_pk_get_type(&(*certA).pk) != (*binSignCert).pkType {
            return V_ERR as i32;
        }
        
        // Get pk buffer
        let mut lenA: i32 = 0;
        let bufA = crate::src_app_verify::GetPkBuf(&(*certA).pk, &mut lenA);
        if bufA.is_null() {
            return V_ERR as i32;
        }
        
        // Compare pk length
        if lenA != (*binSignCert).pkLen {
            if !bufA.is_null() {
                libc::free(bufA as *mut core::ffi::c_void);
            }
            return V_ERR as i32;
        }
        
        // Compare pk content
        if libc::memcmp(
            bufA as *const core::ffi::c_void,
            (*binSignCert).pkBuf as *const core::ffi::c_void,
            lenA as usize
        ) != 0 {
            if !bufA.is_null() {
                libc::free(bufA as *mut core::ffi::c_void);
            }
            return V_ERR as i32;
        }
        
        if !bufA.is_null() {
            libc::free(bufA as *mut core::ffi::c_void);
        }
        
        V_OK as i32
    }
}

pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    if certBase64.is_null() || binSignCert.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { std::mem::zeroed() };
    unsafe { mbedtls_x509_crt_init(&mut cert) };
    
    let cert_len = unsafe { libc::strlen(certBase64 as *const i8) + 1 };
    let ret = unsafe { mbedtls_x509_crt_parse(&mut cert, certBase64, cert_len as crate::types::size_t) };
    
    if ret != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                846i32,
            );
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: %s\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                847i32,
                certBase64,
            );
        }
        return crate::types::V_ERR as i32;
    }
    
    if crate::src_app_verify::CmpCert(&cert as *const _, binSignCert) == crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: cert consistent\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                852i32,
            );
            mbedtls_x509_crt_free(&mut cert);
        }
        return crate::types::V_OK as i32;
    }
    
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: cert inconsistent\0".as_ptr() as *const i8,
            b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
            856i32,
        );
        mbedtls_x509_crt_free(&mut cert);
    }
    crate::types::V_ERR as i32
}

fn CheckReleaseAppSign(binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let app_dist_type = (*pf).appDistType;
        let app_gallery = b"app_gallery\0".as_ptr() as *const ::core::ffi::c_char;
        
        if libc::strcmp(app_dist_type, app_gallery) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        let release_cert = (*pf).bundleInfo.releaseCert;
        if libc::strlen(release_cert as *const ::core::ffi::c_char) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        let ret = crate::src_app_verify::LoadCertAndCmpDest(release_cert, binSignCert);
        if ret == crate::types::V_OK as i32 {
            return crate::types::V_OK as i32;
        }
        
        crate::types::V_ERR as i32
    }
}

fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let dev_cert = (*pf).bundleInfo.devCert;
        if libc::strlen(dev_cert as *const i8) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: debug app, devCert null\0".as_ptr() as *const i8,
                b"CheckDebugAppSign\0".as_ptr() as *const i8,
                885i32,
            );
            return crate::types::V_ERR as i32;
        }
        
        let ret = crate::src_app_verify::LoadCertAndCmpDest(dev_cert, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: dev cert consistent\0".as_ptr() as *const i8,
                b"CheckDebugAppSign\0".as_ptr() as *const i8,
                890i32,
            );
            return crate::types::V_OK as i32;
        }
        
        let release_cert = (*pf).bundleInfo.releaseCert;
        if libc::strlen(release_cert as *const i8) != 0 {
            let ret = crate::src_app_verify::LoadCertAndCmpDest(release_cert, binSignCert as *const crate::types::CertInfo);
            if ret == crate::types::V_OK as i32 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: release cert consistent\0".as_ptr() as *const i8,
                    b"CheckDebugAppSign\0".as_ptr() as *const i8,
                    896i32,
                );
                return crate::types::V_OK as i32;
            }
        }
        
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: app sign cert not consistent with profile cert\0".as_ptr() as *const i8,
            b"CheckDebugAppSign\0".as_ptr() as *const i8,
            900i32,
        );
        crate::types::V_ERR as i32
    }
}

fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    if appCertType == crate::types::CERT_TYPE_APPGALLARY as i32 || appCertType == crate::types::CERT_TYPE_SYETEM as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: app type : %d, return OK\0".as_ptr() as *const ::core::ffi::c_char,
                b"CheckAppSignCertWithProfile\0".as_ptr() as *const ::core::ffi::c_char,
                908i32,
                appCertType,
            );
        }
        return crate::types::V_OK as i32;
    }

    let mut ret: i32 = crate::types::V_ERR as i32;

    unsafe {
        let pf_type = (*pf).type_;
        if libc::strcmp(b"debug\0".as_ptr() as *const ::core::ffi::c_char, pf_type as *const ::core::ffi::c_char) == 0 {
            ret = crate::src_app_verify::CheckDebugAppSign(binSignCert, pf as *const crate::types::ProfileProf);
        } else if libc::strcmp(b"release\0".as_ptr() as *const ::core::ffi::c_char, pf_type as *const ::core::ffi::c_char) == 0 {
            ret = crate::src_app_verify::CheckReleaseAppSign(binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf);
        }

        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: check app sign cert ret : %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"CheckAppSignCertWithProfile\0".as_ptr() as *const ::core::ffi::c_char,
            920i32,
            ret,
        );
    }

    ret
}

fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32 {
    let ret = unsafe {
        crate::compat::memset_s(
            certInfo as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::CertInfo>() as crate::types::size_t,
            0,
            std::mem::size_of::<crate::types::CertInfo>() as crate::types::size_t,
        )
    };
    if ret != 0 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: memset error\0".as_ptr() as *const ::core::ffi::c_char,
                b"CertInfoInit\0".as_ptr() as *const ::core::ffi::c_char,
                928i32,
            );
        }
    }
    ret as i32
}

pub extern "C" fn FreeCertInfo(certInfo: *mut crate::types::CertInfo) {
    if certInfo.is_null() {
        return;
    }
    
    unsafe {
        if !(*certInfo).issuer.is_null() {
            libc::free((*certInfo).issuer as *mut ::core::ffi::c_void);
            (*certInfo).issuer = std::ptr::null_mut();
            (*certInfo).issuerLen = 0;
        }
        
        if !(*certInfo).subject.is_null() {
            libc::free((*certInfo).subject as *mut ::core::ffi::c_void);
            (*certInfo).subject = std::ptr::null_mut();
            (*certInfo).subjectLen = 0;
        }
        
        if !(*certInfo).pkBuf.is_null() {
            libc::free((*certInfo).pkBuf as *mut ::core::ffi::c_void);
            (*certInfo).pkBuf = std::ptr::null_mut();
            (*certInfo).pkLen = 0;
        }
    }
}

fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    use crate::types::*;
    
    let certInfo: *mut CertInfo = unsafe { libc::malloc(core::mem::size_of::<CertInfo>()) as *mut CertInfo };
    if certInfo.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: \"certInfo\" is null\0".as_ptr() as *const i8,
                b"GetCertInfo\0".as_ptr() as *const i8,
                958,
            );
        }
        return V_ERR_MALLOC as i32;
    }
    
    let mut ret = crate::src_app_verify::CertInfoInit(certInfo);
    if ret != V_OK as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: \"cert info init\"\0".as_ptr() as *const i8,
                b"GetCertInfo\0".as_ptr() as *const i8,
                962,
            );
        }
        ret = V_ERR_MEMSET as i32;
        unsafe {
            crate::src_app_verify::FreeCertInfo(certInfo);
            if !certInfo.is_null() {
                libc::free(certInfo as *mut core::ffi::c_void);
            }
        }
        return ret;
    }
    
    unsafe {
        (*certInfo).issuerLen = (*ctr).issuer_raw.len as i32;
        (*certInfo).subjectLen = (*ctr).subject_raw.len as i32;
        
        if (*certInfo).issuerLen == 0 || (*certInfo).issuerLen > (1024 * 1024) ||
           (*certInfo).subjectLen == 0 || (*certInfo).subjectLen > (1024 * 1024) {
            ret = V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        
        (*certInfo).issuer = libc::malloc(((*certInfo).issuerLen + 1) as usize) as *mut i8;
        if (*certInfo).issuer.is_null() {
            ret = V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        *(*certInfo).issuer.offset((*certInfo).issuerLen as isize) = 0;
        ret = memcpy_s((*certInfo).issuer as *mut core::ffi::c_void, (*certInfo).issuerLen as size_t, (*ctr).issuer_raw.p as *const core::ffi::c_void, (*ctr).issuer_raw.len as size_t);
        if ret != 0 {
            ret = V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        
        (*certInfo).subject = libc::malloc(((*certInfo).subjectLen + 1) as usize) as *mut i8;
        if (*certInfo).subject.is_null() {
            ret = V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        *(*certInfo).subject.offset((*certInfo).subjectLen as isize) = 0;
        ret = memcpy_s((*certInfo).subject as *mut core::ffi::c_void, (*certInfo).subjectLen as size_t, (*ctr).subject_raw.p as *const core::ffi::c_void, (*ctr).subject_raw.len as size_t);
        if ret != 0 {
            ret = V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        
        (*certInfo).pkType = mbedtls_pk_get_type(&(*ctr).pk as *const mbedtls_pk_context);
        (*certInfo).pkBuf = crate::src_app_verify::GetPkBuf(&(*ctr).pk as *const mbedtls_pk_context, &mut (*certInfo).pkLen as *mut i32) as *mut i8;
        if (*certInfo).pkBuf.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: \"get pk error\"\0".as_ptr() as *const i8,
                b"GetCertInfo\0".as_ptr() as *const i8,
                998,
            );
            ret = V_ERR as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        
        *binSignCert = certInfo;
    }
    V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_35
// c_function: VerfiyAppSourceGetProfile
// rust_file: src_app_verify.rs
// rust_signature: fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32
// c_first_line: static int32_t VerfiyAppSourceGetProfile(int32_t fp, const SignatureInfo *signInfo,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_35/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================

// TODO: Manual implementation needed
// Function: src_app_verify_35
// Original C/C++ function: VerfiyAppSourceGetProfile
// File: src_app_verify.rs
// 
// The automatic translation failed after multiple repair attempts.
// Please review the original C/C++ code and implement manually.
//
// Original C/C++ signature (first line):
// static int32_t VerfiyAppSourceGetProfile(int32_t fp, const SignatureInfo *signInfo,
fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_35
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_35/translated_rust.rs
 * ------------------------------------------------------------
fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK as i32 {
        return ret;
    }

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR_VERFIY_PROF_CERT as i32;
    }

    unsafe {
        if !(*pf).bundleInfo.devCert.is_null() {
            libc::free((*pf).bundleInfo.devCert as *mut core::ffi::c_void);
            (*pf).bundleInfo.devCert = std::ptr::null_mut();
        }
        if !(*pf).bundleInfo.releaseCert.is_null() {
            libc::free((*pf).bundleInfo.releaseCert as *mut core::ffi::c_void);
            (*pf).bundleInfo.releaseCert = std::ptr::null_mut();
        }
    }

    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_35
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_36
// c_function: VerifyAppSignPkcsData
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32
// c_first_line: static int32_t VerifyAppSignPkcsData(const FileRead *fileRead, const SignatureInfo *signInfo, const Pkcs7 *pkcs7Handle)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_36
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_36/translated_rust.rs
 * ------------------------------------------------------------
fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify certs failed, ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1041i32,
                ret,
            );
        }
        return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
    }
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: Verify certs success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
            1044i32,
        );
    }

    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: VerifyRawHash failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1048i32,
                ret,
            );
        }
        return ret;
    }
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: VerifyRawHash success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
            1051i32,
        );
    }

    let calc_digest_fn: crate::types::PKCS7_CalcDigest = unsafe {
        std::mem::transmute(CalcDigest as usize)
    };
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, calc_digest_fn);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: pkcs7 verify signer signature failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1055i32,
                ret,
            );
        }
        return crate::types::V_ERR_VERIFY_SIGNATURE as i32;
    }

    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_36
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_37
// c_function: GetBinSignPkcs
// rust_file: src_app_verify.rs
// rust_signature: fn GetBinSignPkcs(signBuf: *const c_char, len: i32) -> *mut crate::types::Pkcs7
// c_first_line: static Pkcs7 *GetBinSignPkcs(const char *signBuf, int32_t len)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_37/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn GetBinSignPkcs(signBuf: *const c_char, len: i32) -> *mut crate::types::Pkcs7 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_37
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_37/translated_rust.rs
 * ------------------------------------------------------------
fn GetBinSignPkcs(signBuf: *const c_char, len: i32) -> *mut crate::types::Pkcs7 {
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const c_char,
                b"[%s:%d]: malloc error\0".as_ptr() as *const c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const c_char,
                1066i32,
            );
        }
        return std::ptr::null_mut();
    }
    
    let ret = unsafe {
        PKCS7_ParseSignedData(
            signBuf as *const ::core::ffi::c_uchar,
            len as crate::types::size_t,
            pkcs7,
        )
    };
    
    if ret != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const c_char,
                b"[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const c_char,
                1071i32,
                ret,
            );
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    pkcs7
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_37
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead {
    let fileRead = unsafe { libc::malloc(std::mem::size_of::<crate::types::FileRead>()) } as *mut crate::types::FileRead;
    if fileRead.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: malloc error\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetFileRead\0".as_ptr() as *const ::core::ffi::c_char,
                1084i32,
            );
        }
        return std::ptr::null_mut();
    }
    unsafe {
        (*fileRead).fp = fp;
        (*fileRead).offset = offset;
        (*fileRead).len = size;
    }
    fileRead
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_39
// c_function: VerifyBinSign
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32
// c_first_line: static int32_t VerifyBinSign(SignatureInfo *signInfo, int32_t fp, CertInfo **signCert, int32_t *certType)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_39/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================
fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_39
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_39/translated_rust.rs
 * ------------------------------------------------------------
Looking at the errors, they are all in `src/src_mbedtls_pkcs7.rs` and are about duplicate name definitions - these are NOT errors in the `VerifyBinSign` function itself. The errors are in a completely different file and are unrelated to this function's code.

The `VerifyBinSign` function code itself appears correct. The compilation errors are module-level import conflicts in another file. Since I can only output the function body and cannot fix module-level issues in other files, I'll provide the correct function implementation:

fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead: crate::types::BlockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32;

    let signBuf = GetSignBlockByType(
        signInfo as *const crate::types::SignatureInfo,
        fp,
        crate::types::SIGNATURE_BLOCK_TYPE as i32,
        &mut blockLen,
        &mut blockHead,
    );

    if signBuf.is_null() {
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = GetBinSignPkcs(signBuf as *const i8, blockLen);
    if pkcs7.is_null() {
        unsafe { libc::free(signBuf as *mut core::ffi::c_void) };
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    fileRead = GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        ret = crate::types::V_ERR_MALLOC as i32;
    } else {
        ret = GetAppSingerCertType(pkcs7, certType);
        if ret != crate::types::V_OK as i32 {
            ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        } else {
            unsafe { (*signInfo).certType = *certType };
            ret = VerifyAppSignPkcsData(
                fileRead as *const crate::types::FileRead,
                signInfo as *const crate::types::SignatureInfo,
                pkcs7 as *const crate::types::Pkcs7,
            );
            if ret != crate::types::V_OK as i32 {
                ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
            } else {
                let crt = unsafe { (*pkcs7).signedData.signers.certPath.crt };
                ret = GetCertInfo(
                    crt as *const crate::types::mbedtls_x509_crt,
                    signCert,
                );
                if ret != crate::types::V_OK as i32 {
                    ret = crate::types::V_ERR_GET_CERT_INFO as i32;
                }
            }
        }
    }

    unsafe { libc::free(signBuf as *mut core::ffi::c_void) };
    PKCS7_FreeRes(pkcs7);
    unsafe { libc::free(pkcs7 as *mut core::ffi::c_void) };
    if !fileRead.is_null() {
        unsafe { libc::free(fileRead as *mut core::ffi::c_void) };
    }

    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_39
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_40
// c_function: VerifyIntegrity
// rust_file: src_app_verify.rs
// rust_signature: fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32
// c_first_line: static int32_t VerifyIntegrity(SignatureInfo *signInfo, int32_t fp, ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_40/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================

// TODO: Manual implementation needed
// Function: src_app_verify_40
// Original C/C++ function: VerifyIntegrity
// File: src_app_verify.rs
// 
// The automatic translation failed after multiple repair attempts.
// Please review the original C/C++ code and implement manually.
//
// Original C/C++ signature (first line):
// static int32_t VerifyIntegrity(SignatureInfo *signInfo, int32_t fp, ProfileProf *pf)
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_40
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_40/translated_rust.rs
 * ------------------------------------------------------------
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    
    let ret = VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify bin sign error\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1158 as i32,
            );
        }
        return ret;
    }
    
    let ret = VerfiyAppSourceGetProfile(fp, signInfo as *const crate::types::SignatureInfo, certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1164 as i32,
                ret,
            );
        }
        FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe {
                libc::free(binSignCert as *mut ::core::ffi::c_void);
            }
        }
        return ret;
    }
    
    FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe {
            libc::free(binSignCert as *mut ::core::ffi::c_void);
        }
    }
    
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_40
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_verify_41
// c_function: APPVERI_AppVerify
// rust_file: src_app_verify.rs
// rust_signature: pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32
// c_first_line: int32_t APPVERI_AppVerify(const char *filePath, VerifyResult *verifyRst)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_41/translated_rust.rs
// last_error_truncated:
//   error[E0255]: the name `GetSignersCnt` is defined multiple times
//       --> src/src_mbedtls_pkcs7.rs:1669:1
//        |
//        |         ------------- previous import of the value `GetSignersCnt` here
//        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `GetSignersCnt` redefined here
//        |
//   help: you can use `as` to change the binding name of the import
//        |
// =================================

// TODO: Manual implementation needed
// Function: src_app_verify_41
// Original C/C++ function: APPVERI_AppVerify
// File: src_app_verify.rs
// 
// The automatic translation failed after multiple repair attempts.
// Please review the original C/C++ code and implement manually.
//
// Original C/C++ signature (first line):
// int32_t APPVERI_AppVerify(const char *filePath, VerifyResult *verifyRst)
pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_verify_41
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_verify_41/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    if filePath.is_null() || verifyRst.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    let mut handle: i32 = 0;
    let mut file: crate::types::FileRead = unsafe { std::mem::zeroed() };
    
    if crate::src_app_file::InitVerify(&mut file, filePath, &mut handle) != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    let mut signInfo: crate::types::SignatureInfo = unsafe { std::mem::zeroed() };
    let mut ret = crate::src_app_verify::GetSignHead(&file as *const _, &mut signInfo);
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        return ret;
    }
    
    let mut signHead = unsafe { signInfo.signHead };
    ret = crate::src_app_verify::VerifyIntegrity(&mut signInfo, handle, unsafe { &mut (*verifyRst).profile });
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
            signHead = std::ptr::null_mut();
        }
        let _ = signHead;
        return ret;
    }
    
    let fileSt: *mut libc::stat = unsafe { libc::malloc(std::mem::size_of::<libc::stat>()) as *mut libc::stat };
    if fileSt.is_null() {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
        }
        crate::src_app_provision::ProfFreeData(unsafe { &mut (*verifyRst).profile });
        return crate::types::V_ERR_MALLOC as i32;
    }
    
    ret = unsafe { libc::fstat(handle, fileSt) };
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle) };
        if !signHead.is_null() {
            unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
        }
        crate::src_app_provision::ProfFreeData(unsafe { &mut (*verifyRst).profile });
        if !fileSt.is_null() {
            unsafe { libc::free(fileSt as *mut ::core::ffi::c_void) };
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    
    unsafe { libc::close(handle) };
    if !signHead.is_null() {
        unsafe { libc::free(signHead as *mut ::core::ffi::c_void) };
    }
    if !fileSt.is_null() {
        unsafe { libc::free(fileSt as *mut ::core::ffi::c_void) };
    }
    
    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_verify_41
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: set debug mode: %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"APPVERI_SetDebugMode\0".as_ptr() as *const ::core::ffi::c_char,
            1227i32,
            mode as i32,
        );
    }
    
    let g_debug_mode_val = unsafe { crate::globals::g_isDebugMode };
    if (g_debug_mode_val != 0) == mode {
        return crate::types::V_OK as i32;
    }
    
    let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: enable pcks7 debug mode failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"APPVERI_SetDebugMode\0".as_ptr() as *const ::core::ffi::c_char,
                1233i32,
            );
        }
        return ret;
    }
    
    unsafe {
        crate::globals::g_isDebugMode = mode as i32;
    }
    
    crate::types::V_OK as i32
}

pub extern "C" fn APPVERI_SetActsMode(mode: bool) {
    unsafe {
        g_isActsMode = mode;
    }
}

pub extern "C" fn APPVERI_IsActsMode() -> i32 {
    unsafe { crate::globals::g_isActsMode }
}

pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: free verify rst data\0".as_ptr() as *const ::core::ffi::c_char,
            b"APPVERI_FreeVerifyRst\0".as_ptr() as *const ::core::ffi::c_char,
            1256i32,
        );
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
    }
}
