//! Module: src_app_verify
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::sync::atomic::Ordering;
use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.


// === C2R_FILE_STATICS_END ===

fn SignHeadN2H(signHead: *mut crate::types::HwSignHead) {
    // Read each field via HapGet (safe) and write back (unsafe – raw pointer deref).
    // Narrow unsafe to just the five pointer writes.

    let block_num_ptr = unsafe { ::core::ptr::addr_of!((*signHead).blockNum) as *const ::core::ffi::c_uchar };
    let block_num = crate::src_app_common::HapGetInt(block_num_ptr, ::core::mem::size_of::<u32>() as i32) as u32;
    unsafe { *::core::ptr::addr_of_mut!((*signHead).blockNum) = block_num; }

    let size_ptr = unsafe { ::core::ptr::addr_of!((*signHead).size) as *const ::core::ffi::c_uchar };
    let size = crate::src_app_common::HapGetInt64(size_ptr, ::core::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
    unsafe { *::core::ptr::addr_of_mut!((*signHead).size) = size; }

    let magic_low_ptr = unsafe { ::core::ptr::addr_of!((*signHead).magicLow) as *const ::core::ffi::c_uchar };
    let magic_low = crate::src_app_common::HapGetInt64(magic_low_ptr, ::core::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
    unsafe { *::core::ptr::addr_of_mut!((*signHead).magicLow) = magic_low; }

    let magic_high_ptr = unsafe { ::core::ptr::addr_of!((*signHead).magicHigh) as *const ::core::ffi::c_uchar };
    let magic_high = crate::src_app_common::HapGetInt64(magic_high_ptr, ::core::mem::size_of::<::core::ffi::c_ulonglong>() as i32) as ::core::ffi::c_ulonglong;
    unsafe { *::core::ptr::addr_of_mut!((*signHead).magicHigh) = magic_high; }

    let version_ptr = unsafe { ::core::ptr::addr_of!((*signHead).version) as *const ::core::ffi::c_uchar };
    let version = crate::src_app_common::HapGetInt(version_ptr, ::core::mem::size_of::<u32>() as i32) as u32;
    unsafe { *::core::ptr::addr_of_mut!((*signHead).version) = version; }
}

fn BlockHeadN2H(blockHead: *mut crate::types::BlockHead) {
    // Narrow unsafe to the three field writes; HapGetUnsignedInt is safe.
    let type_ptr = unsafe { ::core::ptr::addr_of!((*blockHead).type_) as *const u32 as *const ::core::ffi::c_uchar };
    let type_val = crate::src_app_common::HapGetUnsignedInt(type_ptr, 4);
    unsafe { (*blockHead).type_ = type_val; }

    let len_ptr = unsafe { ::core::ptr::addr_of!((*blockHead).length) as *const u32 as *const ::core::ffi::c_uchar };
    let len_val = crate::src_app_common::HapGetUnsignedInt(len_ptr, 4);
    unsafe { (*blockHead).length = len_val; }

    let off_ptr = unsafe { ::core::ptr::addr_of!((*blockHead).offset) as *const u32 as *const ::core::ffi::c_uchar };
    let off_val = crate::src_app_common::HapGetUnsignedInt(off_ptr, 4);
    unsafe { (*blockHead).offset = off_val; }
}

fn ContentN2H(content: *mut crate::types::ContentInfo) {
    // Narrow unsafe to deref writes; compute values safely via HapGetInt.
    let block_num_ptr = unsafe { core::ptr::addr_of!((*content).blockNum) as *const u8 };
    let block_num = crate::src_app_common::HapGetInt(block_num_ptr, core::mem::size_of::<i32>() as i32);
    unsafe { (*content).blockNum = block_num; }

    let size_ptr = unsafe { core::ptr::addr_of!((*content).size) as *const u8 };
    let size = crate::src_app_common::HapGetInt(size_ptr, core::mem::size_of::<i32>() as i32);
    unsafe { (*content).size = size; }

    let alg_id_ptr = unsafe { core::ptr::addr_of!((*content).algId) as *const u8 };
    let alg_id = crate::src_app_common::HapGetInt(alg_id_ptr, core::mem::size_of::<i32>() as i32);
    unsafe { (*content).algId = alg_id; }

    let length_ptr = unsafe { core::ptr::addr_of!((*content).length) as *const u8 };
    let length = crate::src_app_common::HapGetInt(length_ptr, core::mem::size_of::<i32>() as i32);
    unsafe { (*content).length = length; }
}

fn GetSignHead(file: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> i32 {
    let file_fp = unsafe { (*file).fp };

    let mut st: crate::compat::stat = unsafe { std::mem::zeroed() };
    let fstat_ret = unsafe { crate::compat::fstat(file_fp, &mut st) };
    if fstat_ret != 0 || st.st_size < (std::mem::size_of::<crate::types::HwSignHead>() as crate::types::off_t) {
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    if !crate::src_app_centraldirectory::FindSignature(file, signInfo) {
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    let sign_info_ref = unsafe { &mut *signInfo };
    if sign_info_ref.hapCoreDirOffset < (std::mem::size_of::<crate::types::HwSignHead>() as i32) {
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    let seek_offset = (sign_info_ref.hapCoreDirOffset as crate::types::off_t)
        - (std::mem::size_of::<crate::types::HwSignHead>() as crate::types::off_t);
    let lseek_ret = unsafe { crate::compat::lseek(file_fp, seek_offset, crate::types::SEEK_SET as i32) };
    if lseek_ret < 0 {
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    let sign_head_size = std::mem::size_of::<crate::types::HwSignHead>();
    let sign_head_ptr: *mut crate::types::HwSignHead = unsafe {
        crate::compat::malloc(sign_head_size as crate::types::size_t)
    } as *mut crate::types::HwSignHead;
    if sign_head_ptr.is_null() {
        return crate::types::V_ERR as i32;
    }

    let read_len: crate::types::ssize_t = unsafe {
        crate::compat::read(file_fp, sign_head_ptr as *mut ::core::ffi::c_void, sign_head_size as crate::types::size_t)
    };
    if read_len != (sign_head_size as crate::types::ssize_t) {
        unsafe { crate::compat::free(sign_head_ptr as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    crate::src_app_verify::SignHeadN2H(sign_head_ptr);

    let (version, magic_low_val, magic_high_val, sign_head_size_i32) = {
        let head_ref = unsafe { &*sign_head_ptr };
        (head_ref.version, head_ref.magicLow, head_ref.magicHigh, head_ref.size as i32)
    };

    let magic_low: u64 = if version < crate::types::VERSION_FOR_NEW_MAGIC_NUM {
        crate::types::HAP_SIG_BLOCK_MAGIC_LO_OLD
    } else {
        crate::types::HAP_SIG_BLOCK_MAGIC_LO
    };
    let magic_high: u64 = if version < crate::types::VERSION_FOR_NEW_MAGIC_NUM {
        crate::types::HAP_SIG_BLOCK_MAGIC_HI_OLD
    } else {
        crate::types::HAP_SIG_BLOCK_MAGIC_HI
    };
    if magic_low_val != magic_low || magic_high_val != magic_high {
        unsafe { crate::compat::free(sign_head_ptr as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    let full_sign_block_offset = sign_info_ref.hapCoreDirOffset - sign_head_size_i32;
    if full_sign_block_offset <= 0 || full_sign_block_offset >= sign_info_ref.hapCoreDirOffset {
        unsafe { crate::compat::free(sign_head_ptr as *mut ::core::ffi::c_void); }
        sign_info_ref.signHead = std::ptr::null_mut();
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }

    sign_info_ref.signHead = sign_head_ptr;
    sign_info_ref.fullSignBlockOffset = full_sign_block_offset;
    sign_info_ref.fileSize = st.st_size as i32;
    crate::types::V_OK as i32
}

fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    let si = unsafe { &*signInfo };
    let sh = si.signHead;
    let fso = si.fullSignBlockOffset;
    let num = unsafe { (*sh).blockNum };
    let _lseek_ret = unsafe { lseek(fp as libc::c_int, fso as libc::off_t, libc::SEEK_SET) };
    if num > crate::types::MAX_BLOCK_NUM {
        return crate::types::V_ERR as i32;
    }
    let block_size = std::mem::size_of::<crate::types::BlockHead>();
    for _ in 0..num {
        let read_ret = unsafe { libc::read(fp as libc::c_int, block as *mut std::ffi::c_void, block_size) };
        if read_ret as usize != block_size {
            return crate::types::V_ERR as i32;
        }
        let type_val = {
            let type_ptr = unsafe { std::ptr::addr_of!((*block).type_) as *const ::core::ffi::c_uchar };
            crate::src_app_common::HapGetInt(type_ptr, std::mem::size_of::<u32>() as i32)
        };
        if type_val == blockType {
            crate::src_app_verify::BlockHeadN2H(block);
            return crate::types::V_OK as i32;
        }
    }
    crate::types::V_ERR as i32
}

pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    if signInfo.is_null() || blockHead.is_null() {
        return ::core::ptr::null_mut();
    }
    let ret = FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != 0 {
        return ::core::ptr::null_mut();
    }
    // unsafe: deref raw pointers from FFI
    let sign_info_ref = unsafe { &*signInfo };
    let block_head_ref = unsafe { &*blockHead };
    let full_sign_offset = sign_info_ref.fullSignBlockOffset;
    let hap_core_dir_offset = sign_info_ref.hapCoreDirOffset;
    let file_size = sign_info_ref.fileSize;
    let length = block_head_ref.length;
    let block_offset = block_head_ref.offset;

    if length == 0 || (length as i64) > (hap_core_dir_offset as i64 - full_sign_offset as i64) {
        return ::core::ptr::null_mut();
    }
    if (length as u64 + 1) >= file_size as u64 {
        return ::core::ptr::null_mut();
    }
    // unsafe: POSIX malloc call
    let buf: *mut ::core::ffi::c_char = unsafe { ::libc::malloc((length + 1) as usize) as *mut ::core::ffi::c_char };
    if buf.is_null() {
        return ::core::ptr::null_mut();
    }
    // unsafe: raw pointer write for null terminator
    unsafe { *buf.add(length as usize) = 0; }
    let fstat_ret: i32;
    let file_st_size: i64;
    {
        // unsafe: zeroed and POSIX fstat call
        let mut file_st: libc::stat = unsafe { std::mem::zeroed() };
        let ret = unsafe { ::libc::fstat(fp as ::libc::c_int, &mut file_st as *mut libc::stat) };
        fstat_ret = ret;
        file_st_size = file_st.st_size;
    }
    let needed_size = full_sign_offset as i64 + block_offset as i64 + length as i64;
    if fstat_ret != 0 || file_st_size < needed_size {
        if !buf.is_null() {
            unsafe { ::libc::free(buf as *mut ::core::ffi::c_void); }
        }
        return ::core::ptr::null_mut();
    }
    let seek_pos = (full_sign_offset as i64 + block_offset as i64) as ::libc::off_t;
    // unsafe: POSIX lseek, read, write to out-param
    unsafe { ::libc::lseek(fp as ::libc::c_int, seek_pos, ::libc::SEEK_SET) };
    let read_len = unsafe { ::libc::read(fp as ::libc::c_int, buf as *mut ::core::ffi::c_void, length as usize) };
    if read_len as i64 != length as i64 {
        if !buf.is_null() {
            unsafe { ::libc::free(buf as *mut ::core::ffi::c_void); }
        }
        return ::core::ptr::null_mut();
    }
    unsafe { *len = read_len as i32; }
    buf
}

pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t) };
    if md_info.is_null() {
        return 0;
    }
    unsafe { crate::compat::mbedtls_md_get_size(md_info) as i32 }
}

fn CalcCmpContHash(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut usize) -> i32 {
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let rc = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
    if rc != V_OK as i32 {
        return rc;
    }
    // unsafe: FFI call
    let md_info = unsafe { mbedtls_md_info_from_type(algType) };
    if md_info.is_null() {
        return V_ERR as i32;
    }
    // unsafe: FFI call
    let md_rc = unsafe { mbedtls_md(md_info, input as *const u8, (inputLen as usize).try_into().unwrap(), hash) };
    if md_rc != 0 {
        return md_rc;
    }
    // unsafe: FFI call
    let hash_len = unsafe { mbedtls_md_get_size(md_info) as usize };
    // unsafe: raw pointer deref for out-param
    unsafe { *hashLen = hash_len; }
    let mut digInAttr: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut digInAttrLen: crate::types::size_t = 0;
    let rc = crate::src_mbedtls_pkcs7::PKCS7_GetDigestInSignerAuthAttr(signer, &mut digInAttr, &mut digInAttrLen);
    if rc != V_OK as i32 {
        return rc;
    }
    if digInAttrLen as usize != hash_len {
        return V_ERR as i32;
    }
    // unsafe: FFI call
    let cmp = unsafe { libc::memcmp(hash as *const libc::c_void, digInAttr as *const libc::c_void, digInAttrLen as usize) };
    if cmp != 0 {
        return V_ERR as i32;
    }
    V_OK as i32
}

fn CalcDigest(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut usize)-> i32 {
    let rc = crate::src_app_verify::CalcCmpContHash(pkcs7, signer, algType, hash, hashLen);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerAuthAttr(signer, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    // unsafe: FFI call
    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(algType) };
    if md_info.is_null() {
        return crate::types::V_ERR as i32;
    }
    // unsafe: FFI call
    let md_rc = unsafe { crate::compat::mbedtls_md(md_info, input as *const u8, inputLen, hash as *mut u8) };
    if md_rc != crate::types::V_OK as i32 {
        return md_rc;
    }
    // unsafe: FFI call
    let hash_len = unsafe { crate::compat::mbedtls_md_get_size(md_info) as usize };
    // unsafe: raw pointer deref for out-param
    unsafe { *hashLen = hash_len; }
    crate::types::V_OK as i32
}

unsafe extern "C" fn calc_digest_for_verify_callback(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut crate::types::size_t) -> i32 {
    let mut local_hash_len: usize = 0;
    let ret = crate::src_app_verify::CalcDigest(pkcs7, signer, algType, hash, &mut local_hash_len);
    unsafe { *hashLen = local_hash_len as crate::types::size_t; }
    ret
}

fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut input_len: crate::types::size_t = 0;
    let ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut input_len);
    if ret as u32 != V_OK {
        return ret;
    }
    let content_size = std::mem::size_of::<crate::types::ContentInfo>();
    let content: *mut crate::types::ContentInfo = unsafe { libc::malloc(content_size) as *mut crate::types::ContentInfo };
    if content.is_null() {
        return V_ERR as i32;
    }
    if (input_len as usize) > content_size {
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    unsafe { std::ptr::copy_nonoverlapping(input as *const u8, content as *mut u8, input_len as usize); }
    crate::src_app_verify::ContentN2H(content);
    // Extract alg_id and compute new_alg_id in a narrow unsafe block.
    let (alg_id, new_alg_id) = {
        let content_ref = unsafe { &mut *content };
        let raw_alg = content_ref.algId;
        let new = crate::src_app_verify_hap::GetDigestAlgorithmId(raw_alg as u32);
        content_ref.algId = new;
        (raw_alg, new)
    };
    if new_alg_id as u32 != MBEDTLS_MD_SHA256
        && new_alg_id as u32 != MBEDTLS_MD_SHA384
        && new_alg_id as u32 != MBEDTLS_MD_SHA512
    {
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    let root_hash_len = crate::src_app_verify::GetHashUnitLen(alg_id as i32);
    let mut actual_digest: crate::types::HapBuf = unsafe { std::mem::zeroed() };
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actual_digest as *mut crate::types::HapBuf, root_hash_len) {
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    let fp = unsafe { (*fileRead).fp };
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(new_alg_id, fp, signInfo, &actual_digest as *const crate::types::HapBuf) {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actual_digest as *mut crate::types::HapBuf);
        unsafe { libc::free(content as *mut ::core::ffi::c_void); }
        return V_ERR as i32;
    }
    let hash_diff = {
        let content_ref = unsafe { &*content };
        let content_len = content_ref.length;
        if actual_digest.len != content_len {
            true
        } else {
            unsafe { libc::memcmp(actual_digest.buffer, content_ref.hash.as_ptr() as *const ::core::ffi::c_void, actual_digest.len as usize) != 0 }
        }
    };
    unsafe { libc::free(content as *mut ::core::ffi::c_void); }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut actual_digest as *mut crate::types::HapBuf);
    if hash_diff {
        return V_ERR_GET_HASH_DIFF as i32;
    }
    V_OK as i32
}

fn cstr_from_nullable_i8(ptr: *const i8) -> Option<&'static ::core::ffi::CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { ::core::ffi::CStr::from_ptr(ptr as *const ::core::ffi::c_char) })
    }
}

fn GetCertTypeBySourceName(cert: *const crate::types::TrustAppCert)-> i32 {
    if cert.is_null() {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
    let cert_ref = unsafe { &*cert };
    let name = match cstr_from_nullable_i8(cert_ref.name as *const i8) {
        Some(cs) => cs,
        None => return crate::types::CERT_TYPE_OTHER as i32,
    };
    let name_bytes = name.to_bytes();
    if name_bytes == b"huawei app gallary" {
        return crate::types::CERT_TYPE_APPGALLARY as i32;
    } else if name_bytes == b"huawei system apps" {
        return crate::types::CERT_TYPE_SYETEM as i32;
    } else if name_bytes == b"OpenHarmony apps" {
        return crate::types::CERT_TYPE_SYETEM as i32;
    } else {
        return crate::types::CERT_TYPE_OTHER as i32;
    }
}

fn trust_app_cert_issue_ca(cert: &crate::types::TrustAppCert) -> Option<&::core::ffi::CStr> {
    cstr_from_nullable_i8(cert.issueCA as *const i8)
}
fn trust_app_cert_profile_sign_cert(cert: &crate::types::TrustAppCert) -> Option<&::core::ffi::CStr> {
    cstr_from_nullable_i8(cert.profileSignCert as *const i8)
}
fn trust_app_cert_profile_debug_sign_cert(cert: &crate::types::TrustAppCert) -> Option<&::core::ffi::CStr> {
    cstr_from_nullable_i8(cert.profileDebugSignCert as *const i8)
}

fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    if num <= 0 || signer.is_null() {
        return core::ptr::null();
    }
    let signer_ref = unsafe { &*signer };
    let issuer_cstr = match cstr_from_nullable_i8(signer_ref.issuer.as_ptr() as *const i8) {
        Some(cs) => cs,
        None => return core::ptr::null(),
    };
    let subject_cstr = match cstr_from_nullable_i8(signer_ref.subject.as_ptr() as *const i8) {
        Some(cs) => cs,
        None => return core::ptr::null(),
    };
    let list = unsafe { core::slice::from_raw_parts(trustList, num as usize) };
    for item in list.iter() {
        if trust_app_cert_issue_ca(item) == Some(issuer_cstr)
            && (trust_app_cert_profile_sign_cert(item) == Some(subject_cstr) || trust_app_cert_profile_debug_sign_cert(item) == Some(subject_cstr))
        {
            return item as *const crate::types::TrustAppCert;
        }
    }
    core::ptr::null()
}

fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo) -> (i32, i32) {
    if signer.is_null() {
        return (crate::types::V_ERR as i32, 0);
    }
    let trust_list = crate::globals::trust_app_list();
    let mut trust_cert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer,
        trust_list.as_ptr(),
        trust_list.len() as i32,
    );
    if g_isDebugMode.load(Ordering::Relaxed) && trust_cert.is_null() {
        let trust_list_test = crate::globals::trust_app_list_test();
        trust_cert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer,
            trust_list_test.as_ptr(),
            trust_list_test.len() as i32,
        );
    }
    let trust_cert_ref = unsafe { trust_cert.as_ref() };
    if let Some(tc) = trust_cert_ref {
        let signer_ref = unsafe { &*signer };
        if tc.maxCertPath < signer_ref.depth {
            return (V_ERR as i32, 0);
        }
    }
    let cert_val = crate::src_app_verify::GetCertTypeBySourceName(trust_cert);
    (V_OK as i32, cert_val)
}

fn GetAppSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    if signer.is_null() || trustList.is_null() || num <= 0 {
        return std::ptr::null();
    }
    let signer_ref = unsafe { &*signer };
    let subject_cstr = match cstr_from_nullable_i8(signer_ref.subject.as_ptr() as *const i8) {
        Some(cs) => cs,
        None => return std::ptr::null(),
    };
    let issuer_cstr = match cstr_from_nullable_i8(signer_ref.issuer.as_ptr() as *const i8) {
        Some(cs) => cs,
        None => return std::ptr::null(),
    };
    let trust_slice = unsafe { core::slice::from_raw_parts(trustList, num as usize) };
    for trust in trust_slice.iter() {
        let trust_sub = match cstr_from_nullable_i8(trust.appSignCert as *const i8) {
            Some(cs) => cs,
            None => continue,
        };
        let trust_iss = match cstr_from_nullable_i8(trust.issueCA as *const i8) {
            Some(cs) => cs,
            None => continue,
        };
        if trust_sub == subject_cstr && trust_iss == issuer_cstr {
            return trust as *const crate::types::TrustAppCert;
        }
    }
    std::ptr::null()
}

fn GetAppCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo) -> (i32, i32) {
    if signer.is_null() {
        return (crate::types::V_ERR as i32, 0);
    }
    let trust_list = crate::globals::trust_app_list();
    let mut trust_cert = crate::src_app_verify::GetAppSourceBySigningCert(
        signer as *const crate::types::SignerResovledInfo,
        trust_list.as_ptr(),
        trust_list.len() as i32,
    );
    if g_isDebugMode.load(Ordering::Relaxed) && trust_cert.is_null() {
        let trust_list_test = crate::globals::trust_app_list_test();
        trust_cert = crate::src_app_verify::GetAppSourceBySigningCert(
            signer as *const crate::types::SignerResovledInfo,
            trust_list_test.as_ptr(),
            trust_list_test.len() as i32,
        );
    }
    let trust_cert_ref = unsafe { trust_cert.as_ref() };
    if let Some(trust_ref) = trust_cert_ref {
        let signer_ref = unsafe { &*signer };
        if trust_ref.maxCertPath < signer_ref.depth {
            return (crate::types::V_ERR as i32, 0);
        }
    }
    let cert_val = crate::src_app_verify::GetCertTypeBySourceName(trust_cert);
    (crate::types::V_OK as i32, cert_val)
}

fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7) -> (i32, i32) {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    let sri_ref = unsafe { sri.as_ref() };
    if let Some(sr) = sri_ref {
        if sr.nrOfSigners == 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return (crate::types::V_ERR as i32, 0);
        }
        let (ret, cert_val) = crate::src_app_verify::GetAppCertTypeBySignInfo(sr.signers);
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return (crate::types::V_ERR as i32, 0);
        }
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return (crate::types::V_OK as i32, cert_val);
    }
    (crate::types::V_ERR as i32, 0)
}

fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7) -> (i32, i32) {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    let sri_ref = unsafe { sri.as_ref() };
    if let Some(sr) = sri_ref {
        let (ret, cert_val) = crate::src_app_verify::GetProfileCertTypeBySignInfo(sr.signers);
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return (crate::types::V_ERR as i32, 0);
        }
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return (crate::types::V_OK as i32, cert_val);
    }
    (crate::types::V_ERR as i32, 0)
}

fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32)-> i32 {
    let mut profileData: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let mut ret: i32;

    let pkcs7: *mut crate::types::Pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>() as usize) as *mut crate::types::Pkcs7 };
    if pkcs7.is_null() {
        return crate::types::V_ERR as i32;
    }

    let mut result = crate::types::V_OK;
    loop {
        ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7) };
        if ret != crate::types::V_OK as i32 { result = crate::types::V_ERR; break; }

        ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7);
        if ret != crate::types::V_OK as i32 { result = crate::types::V_ERR; break; }

        let (singer_ret, singer_cert_type) = crate::src_app_verify::GetProfileSingerCertType(pkcs7);
        ret = singer_ret;
        certType = singer_cert_type;
        if ret != crate::types::V_OK as i32 { result = crate::types::V_ERR; break; }
        if certType == 2 { result = crate::types::V_ERR; break; }

        ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7, Some(calc_digest_for_verify_callback));
        if ret != crate::types::V_OK as i32 { result = crate::types::V_ERR; break; }

        ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
        if ret != crate::types::V_OK as i32 { result = crate::types::V_ERR; break; }

         if inputLen > crate::types::MAX_PROFILE_SIZE as crate::types::size_t || inputLen == 0 { result = crate::types::V_ERR; break; }

        profileData = unsafe { libc::malloc((inputLen as usize) + 1) as *mut std::ffi::c_char };
        if profileData.is_null() { result = crate::types::V_ERR; break; }

        unsafe {
            std::ptr::copy_nonoverlapping(input as *const u8, profileData as *mut u8, inputLen as usize);
            *profileData.add(inputLen as usize) = b'\0' as std::ffi::c_char;
        }

        // Narrow unsafe: output parameter writes
        unsafe {
            *profileContent = profileData;
            *contentLen = inputLen as i32;
        }
        return crate::types::V_OK as i32;
    }

    // Narrow unsafe: resource cleanup
    unsafe {
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        libc::free(pkcs7 as *mut libc::c_void);
    }
    if !profileData.is_null() {
        unsafe { libc::free(profileData as *mut libc::c_void); }
    }
    result as i32
}

fn GetRsaPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32)-> *mut u8 {
    if pk.is_null() || len.is_null() { return std::ptr::null_mut(); }
    let max_buf = crate::types::MAX_PK_BUF as usize;
    let buf = unsafe { libc::malloc(max_buf) as *mut u8 };
    if buf.is_null() { return std::ptr::null_mut(); }
    unsafe { std::ptr::write_bytes(buf, 0, max_buf); }
    let mut c = buf.wrapping_add(max_buf);
    let pk_len = unsafe { mbedtls_pk_write_pubkey(&mut c as *mut *mut u8, buf, pk) };
    if pk_len < 0 || pk_len > crate::types::MAX_PK_BUF as i32 {
        unsafe { libc::free(buf as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    let pk_buf = unsafe { libc::malloc(pk_len as usize) as *mut u8 };
    if pk_buf.is_null() {
        unsafe { libc::free(buf as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    unsafe { std::ptr::copy_nonoverlapping(c, pk_buf, pk_len as usize); }
    unsafe { *len = pk_len; }
    unsafe { libc::free(buf as *mut libc::c_void); }
    pk_buf
}

fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32)-> *mut u8 {
    let buf_size: usize = (2 * ((521 + 7) / 8) + 1) as usize;
    if pk.is_null() || len.is_null() { return std::ptr::null_mut(); }
    let ec_ctx = unsafe { (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair };
    if ec_ctx.is_null() { return std::ptr::null_mut(); }
    let buf = unsafe { libc::malloc(buf_size) as *mut u8 };
    if buf.is_null() { return std::ptr::null_mut(); }
    unsafe { std::ptr::write_bytes(buf, 0u8, buf_size); }
    let mut al: crate::types::size_t = 0;
    let grp_ptr = unsafe { &(*ec_ctx).private_grp as *const crate::types::mbedtls_ecp_group };
    let q_ptr = unsafe { &(*ec_ctx).private_Q as *const crate::types::mbedtls_ecp_point };
    let ret = unsafe {
        crate::compat::mbedtls_ecp_point_write_binary(
            grp_ptr,
            q_ptr,
            crate::types::MBEDTLS_ECP_PF_UNCOMPRESSED as i32,
            &mut al,
            buf,
            buf_size as crate::types::size_t,
        )
    };
    let actual_len = al;
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::free(buf as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    let len_val = actual_len as i32;
    if len_val <= 0 || len_val > buf_size as i32 {
        unsafe { libc::free(buf as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    let pk_buf = unsafe { libc::malloc(len_val as usize) as *mut u8 };
    if pk_buf.is_null() {
        unsafe { libc::free(buf as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    unsafe { std::ptr::copy_nonoverlapping(buf as *const u8, pk_buf, len_val as usize); }
    unsafe { *len = len_val; }
    unsafe { libc::free(buf as *mut libc::c_void); }
    pk_buf
}

fn GetPkBuf(pk: *const crate::types::mbedtls_pk_context, len: *mut i32)-> *mut u8 {
    let mut buf_a: *mut u8 = std::ptr::null_mut();
    let pk_type = unsafe { mbedtls_pk_get_type(pk) };
    if pk_type == crate::types::MBEDTLS_PK_RSA || pk_type == crate::types::MBEDTLS_PK_RSASSA_PSS {
        buf_a = crate::src_app_verify::GetRsaPk(pk, len);
    } else if pk_type == crate::types::MBEDTLS_PK_ECDSA || pk_type == crate::types::MBEDTLS_PK_ECKEY {
        buf_a = crate::src_app_verify::GetEcPk(pk, len);
        } buf_a
}

fn ParseCertGetPk(certEncoded: *const std::ffi::c_char, pk: *mut crate::types::AppSignPk)-> i32 {
    let cert_size = std::mem::size_of::<crate::types::mbedtls_x509_crt>();
    let cert = unsafe { libc::malloc(cert_size) as *mut crate::types::mbedtls_x509_crt };
    if cert.is_null() { return crate::types::V_ERR as i32; }
    unsafe { mbedtls_x509_crt_init(cert); }
    let cert_len = unsafe { libc::strlen(certEncoded) + 1 };
    let parse_ret = unsafe { mbedtls_x509_crt_parse(cert, certEncoded as *const ::core::ffi::c_uchar, cert_len.try_into().unwrap()) };
    if parse_ret != 0 {
        unsafe { libc::free(cert as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR as i32;
    }
    let mut len: i32 = 0;
    let pk_ptr = unsafe { &(*cert).pk as *const crate::types::mbedtls_pk_context };
    let pk_buf = GetPkBuf(pk_ptr, &mut len);
    if pk_buf.is_null() {
        unsafe {
            mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*pk).pk = pk_buf as *mut ::core::ffi::c_char;
        (*pk).len = len;
    }
    unsafe {
        mbedtls_x509_crt_free(cert);
        libc::free(cert as *mut ::core::ffi::c_void);
    }
    crate::types::V_OK as i32
}

fn GetAppSignPublicKey(profile: *const crate::types::ProfileProf, pk: *mut crate::types::AppSignPk)-> i32 {
    let prof_ref = unsafe { &*profile };
    let release_cert = prof_ref.bundleInfo.releaseCert;
    let dev_cert = prof_ref.bundleInfo.devCert;
    let cert_ptr = if !release_cert.is_null() && unsafe { libc::strlen(release_cert as *const libc::c_char) } != 0 {
        release_cert as *const std::ffi::c_char
    } else {
        dev_cert as *const std::ffi::c_char
    };
    let ret = crate::src_app_verify::ParseCertGetPk(cert_ptr, pk);
    if ret != crate::types::V_OK as i32 { return crate::types::V_ERR_GET_CERT_PK as i32; }
    crate::types::V_OK as i32
}

fn FreeAppSignPublicKey(pk: *mut crate::types::AppSignPk) {
    let pk_ref = unsafe { &mut *pk };
    if !pk_ref.pk.is_null() {
        unsafe { libc::free(pk_ref.pk as *mut libc::c_void); }
        pk_ref.pk = std::ptr::null_mut();
    }
}

pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    if profile.is_null() {
        let tag = b"appverify\0".as_ptr() as *const i8;
        let fmt = b"[%s:%d]: \"profile\" is null\0".as_ptr() as *const i8;
        let func = b"GetAppid\0".as_ptr() as *const i8;
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, tag, fmt, func, 708i32); }
        return crate::types::V_ERR as i32;
    }

    let mut pk = crate::types::AppSignPk { pk: core::ptr::null_mut(), len: 0 };
    let mut ret = crate::src_app_verify::GetAppSignPublicKey(profile as *const crate::types::ProfileProf, &mut pk as *mut crate::types::AppSignPk);
    if ret != crate::types::V_OK as i32 {
        let tag = b"appverify\0".as_ptr() as *const i8;
        let fmt = b"[%s:%d]: get sign pk failed\0".as_ptr() as *const i8;
        let func = b"GetAppid\0".as_ptr() as *const i8;
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, tag, fmt, func, 712i32); }
        return ret;
    }

    let mut useLen: crate::types::size_t = 0;
    ret = unsafe { mbedtls_base64_encode(core::ptr::null_mut(), 0 as crate::types::size_t, &mut useLen, pk.pk as *const u8, pk.len as crate::types::size_t) };

    let pf_ref = unsafe { &mut *profile };
    let bundle_name = pf_ref.bundleInfo.bundleName;
    let bundle_name_len = unsafe { libc::strlen(bundle_name) as i32 };
    let appid_len = bundle_name_len + useLen as i32 + 2;

    {
        let tag = b"appverify\0".as_ptr() as *const i8;
        let fmt = b"[%s:%d]: GetAppid %d\0".as_ptr() as *const i8;
        let func = b"GetAppid\0".as_ptr() as *const i8;
        unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100u32, tag, fmt, func, 721i32, appid_len); }
    }

    if useLen > 4096 as crate::types::size_t {
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR as i32;
    }

    let appid = unsafe { libc::malloc(appid_len as usize) as *mut i8 };
    if appid.is_null() {
        let tag = b"appverify\0".as_ptr() as *const i8;
        let fmt = b"[%s:%d]: malloc failed\0".as_ptr() as *const i8;
        let func = b"GetAppid\0".as_ptr() as *const i8;
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, tag, fmt, func, 727i32); }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_MALLOC as i32;
    }
    unsafe { *appid.add(appid_len as usize - 1) = 0; }

    ret = unsafe { snprintf_s(appid, appid_len as crate::types::size_t, (bundle_name_len + 1) as crate::types::size_t, b"%s_\0".as_ptr() as *const i8, bundle_name) };
    if ret < 0 {
        {
            let tag = b"appverify\0".as_ptr() as *const i8;
            let fmt = b"[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const i8;
            let func = b"GetAppid\0".as_ptr() as *const i8;
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, tag, fmt, func, 734i32, ret); }
        }
        if !appid.is_null() { unsafe { libc::free(appid as *mut libc::c_void); } }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    ret = unsafe { mbedtls_base64_encode(appid.add(bundle_name_len as usize + 1) as *mut u8, (appid_len - bundle_name_len - 1) as crate::types::size_t, &mut useLen, pk.pk as *const u8, pk.len as crate::types::size_t) };
    if ret != crate::types::V_OK as i32 {
        {
            let tag = b"appverify\0".as_ptr() as *const i8;
            let fmt = b"[%s:%d]: base 64 encode error\0".as_ptr() as *const i8;
            let func = b"GetAppid\0".as_ptr() as *const i8;
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, tag, fmt, func, 742i32); }
        }
        if !appid.is_null() { unsafe { libc::free(appid as *mut libc::c_void); } }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }

    pf_ref.appid = appid;
    {
        let tag = b"appverify\0".as_ptr() as *const i8;
        let fmt1 = b"[%s:%d]: appid len: %d, bL len: %d, base64: %d\0".as_ptr() as *const i8;
        let fmt2 = b"[%s:%d]: %s\0".as_ptr() as *const i8;
        let func = b"GetAppid\0".as_ptr() as *const i8;
        unsafe {
            HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100u32, tag, fmt1, func, 748i32, appid_len, bundle_name_len, useLen as i32);
            HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100u32, tag, fmt2, func, 749i32, pf_ref.appid as *const i8);
        }
    }
    crate::src_app_verify::FreeAppSignPublicKey(&mut pk as *mut crate::types::AppSignPk);
    crate::types::V_OK as i32
}

fn VerifyProfGetContent(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, pf: *mut crate::types::ProfileProf)-> i32 {
    let mut profBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let rawBuf: *mut ::core::ffi::c_char = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROPERTY_BLOCK_TYPE as i32, &mut len, std::ptr::null_mut());
    if rawBuf.is_null() { return crate::types::V_ERR_GET_PROFILE_DATA as i32; }
    let ret: i32 = crate::src_app_verify::VerifyProfileSignGetRaw(rawBuf, len, &mut profBuf, &mut len);
    unsafe { libc::free(rawBuf as *mut libc::c_void); }
    if ret != crate::types::V_OK as i32 { return ret; }
    let mut ret: i32 = crate::src_app_provision::ParseProfile(profBuf as *const ::core::ffi::c_char, len, pf);
    if ret != crate::types::V_OK as i32 { unsafe { libc::free(profBuf as *mut libc::c_void); } return ret; }
    ret = crate::src_app_provision::VerifyProfileContent(pf);
    if ret != crate::types::V_OK as i32 { crate::src_app_provision::ProfFreeData(pf); return ret; }
    ret = crate::src_app_verify::GetAppid(pf);
    if ret != crate::types::V_OK as i32 { crate::src_app_provision::ProfFreeData(pf); return ret; }
    unsafe { libc::free(profBuf as *mut libc::c_void); }
    crate::types::V_OK as i32
}

fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo)-> i32 {
    if certA.is_null() || binSignCert.is_null() { return V_ERR as i32; }
    let certA_ref = unsafe { &*certA };
    let binSignCert_ref = unsafe { &*binSignCert };
    if certA_ref.subject_raw.len as i32 != binSignCert_ref.subjectLen { return V_ERR as i32; }
    let cert_subject = unsafe { std::slice::from_raw_parts(certA_ref.subject_raw.p as *const u8, certA_ref.subject_raw.len as usize) };
    let bin_subject = unsafe { std::slice::from_raw_parts(binSignCert_ref.subject as *const u8, binSignCert_ref.subjectLen as usize) };
    if cert_subject != bin_subject { return V_ERR as i32; }
    if certA_ref.issuer_raw.len as i32 != binSignCert_ref.issuerLen { return V_ERR as i32; }
    let cert_issuer = unsafe { std::slice::from_raw_parts(certA_ref.issuer_raw.p as *const u8, certA_ref.issuer_raw.len as usize) };
    let bin_issuer = unsafe { std::slice::from_raw_parts(binSignCert_ref.issuer as *const u8, binSignCert_ref.issuerLen as usize) };
    if cert_issuer != bin_issuer { return V_ERR as i32; }
    let mut lenA: i32 = 0;
    let bufA = crate::src_app_verify::GetPkBuf(&certA_ref.pk as *const _, &mut lenA);
    if bufA.is_null() { return V_ERR as i32; }
    if lenA != binSignCert_ref.pkLen { unsafe { libc::free(bufA as *mut libc::c_void); } return V_ERR as i32; }
    let buf_slice = unsafe { std::slice::from_raw_parts(bufA as *const u8, lenA as usize) };
    let bin_pk_slice = unsafe { std::slice::from_raw_parts(binSignCert_ref.pkBuf as *const u8, binSignCert_ref.pkLen as usize) };
    let pk_matches = buf_slice == bin_pk_slice;
    unsafe { libc::free(bufA as *mut libc::c_void); }
    if pk_matches { V_OK as i32 } else { V_ERR as i32 }
}

pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    if certBase64.is_null() || binSignCert.is_null() { return V_ERR as i32; }
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { std::mem::zeroed() };
    let len = unsafe { libc::strlen(certBase64 as *const ::core::ffi::c_char) + 1 };
    let ret = unsafe { mbedtls_x509_crt_parse(&mut cert, certBase64, len as crate::types::size_t) };
    if ret != 0 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: load release cert failed\0".as_ptr() as *const ::core::ffi::c_char, b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char, 846 as ::core::ffi::c_int); }
        unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100u32, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char, b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char, 847 as ::core::ffi::c_int, certBase64 as *const ::core::ffi::c_char); }
        return V_ERR as i32;
    }
    if crate::src_app_verify::CmpCert(&cert, binSignCert) == 0 {
        unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100u32, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: cert consistent\0".as_ptr() as *const ::core::ffi::c_char, b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char, 852 as ::core::ffi::c_int); }
        unsafe { mbedtls_x509_crt_free(&mut cert); }
        return V_OK as i32;
    }
    unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100u32, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: cert inconsistent\0".as_ptr() as *const ::core::ffi::c_char, b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char, 856 as ::core::ffi::c_int); }
    unsafe { mbedtls_x509_crt_free(&mut cert); }
    V_ERR as i32
}

fn CheckReleaseAppSign(binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf)-> i32 {
    let pf_ref = unsafe { &*pf };
    let app_dist_cmp = unsafe { libc::strcmp(pf_ref.appDistType as *const ::core::ffi::c_char, b"app_gallery\0".as_ptr() as *const ::core::ffi::c_char) };
    if app_dist_cmp == 0 { return crate::types::V_ERR as i32; }
    let release_cert_ptr = pf_ref.bundleInfo.releaseCert;
    if release_cert_ptr.is_null() { return crate::types::V_ERR as i32; }
    let release_cert_len = unsafe { libc::strlen(release_cert_ptr as *const ::core::ffi::c_char) };
    if release_cert_len == 0 { return crate::types::V_ERR as i32; }
    let ret = crate::src_app_verify::LoadCertAndCmpDest(release_cert_ptr, binSignCert);
    if ret == crate::types::V_OK as i32 { return crate::types::V_OK as i32; }
    crate::types::V_ERR as i32
}

fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf)-> i32 {
    let pf_ref = unsafe { &*pf };
    if !pf_ref.bundleInfo.devCert.is_null() && unsafe { libc::strlen(pf_ref.bundleInfo.devCert as *const ::core::ffi::c_char) } != 0 {
        let ret = crate::src_app_verify::LoadCertAndCmpDest(pf_ref.bundleInfo.devCert as *const ::core::ffi::c_uchar, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 { return crate::types::V_OK as i32; }
    }
    if !pf_ref.bundleInfo.releaseCert.is_null() && unsafe { libc::strlen(pf_ref.bundleInfo.releaseCert as *const ::core::ffi::c_char) } != 0 {
        let ret = crate::src_app_verify::LoadCertAndCmpDest(pf_ref.bundleInfo.releaseCert as *const ::core::ffi::c_uchar, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 { return crate::types::V_OK as i32; }
    }
    crate::types::V_ERR as i32
}

fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf)-> i32 {
    if appCertType == 0 || appCertType == 1 { return crate::types::V_OK as i32; }
    let pf_ref = unsafe { &*pf };
    let profile_type = unsafe { std::ffi::CStr::from_ptr(pf_ref.type_ as *const ::core::ffi::c_char) };
    if profile_type.to_bytes() == b"debug" {
        crate::src_app_verify::CheckDebugAppSign(binSignCert, pf as *const crate::types::ProfileProf)
    } else if profile_type.to_bytes() == b"release" {
        crate::src_app_verify::CheckReleaseAppSign(binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf)
    } else {
        crate::types::V_ERR as i32
    }
}

fn CertInfoInit(certInfo: *mut crate::types::CertInfo)-> i32 {
    // Only write_bytes on raw pointer requires unsafe; null-check is already ensured by caller.
    unsafe { std::ptr::write_bytes(certInfo as *mut u8, 0u8, std::mem::size_of::<crate::types::CertInfo>()); }
    0
}

pub extern "C" fn FreeCertInfo(certInfo: *mut crate::types::CertInfo) {
    if certInfo.is_null() { return; }
    let ci = unsafe { &mut *certInfo };
    if !ci.issuer.is_null() { unsafe { libc::free(ci.issuer as *mut libc::c_void); } }
    ci.issuer = std::ptr::null_mut();
    ci.issuerLen = 0;
    if !ci.subject.is_null() { unsafe { libc::free(ci.subject as *mut libc::c_void); } }
    ci.subject = std::ptr::null_mut();
    ci.subjectLen = 0;
    if !ci.pkBuf.is_null() { unsafe { libc::free(ci.pkBuf as *mut libc::c_void); } }
    ci.pkBuf = std::ptr::null_mut();
    ci.pkLen = 0;
}

fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo)-> i32 {
    let cert_info = unsafe { libc::malloc(std::mem::size_of::<crate::types::CertInfo>() as usize) as *mut crate::types::CertInfo };
    if cert_info.is_null() { return crate::types::V_ERR_MALLOC as i32; }
    let ret = crate::src_app_verify::CertInfoInit(cert_info);
    if ret != crate::types::V_OK as i32 { crate::src_app_verify::FreeCertInfo(cert_info); unsafe { libc::free(cert_info as *mut ::core::ffi::c_void); } return ret; }
    let ctr_ref = unsafe { &*ctr };
    let issuer_len_raw = ctr_ref.issuer_raw.len;
    let subject_len_raw = ctr_ref.subject_raw.len;
    unsafe { (*cert_info).issuerLen = issuer_len_raw as i32; (*cert_info).subjectLen = subject_len_raw as i32; }
    if issuer_len_raw == 0 || issuer_len_raw > (crate::types::MAX_PROFILE_SIZE as crate::types::size_t) || subject_len_raw == 0 || subject_len_raw > (crate::types::MAX_PROFILE_SIZE as crate::types::size_t) {
        crate::src_app_verify::FreeCertInfo(cert_info); unsafe { libc::free(cert_info as *mut ::core::ffi::c_void); } return crate::types::V_ERR_MALLOC as i32;
    }
    let issuer_len = issuer_len_raw as usize;
    let issuer_buf = unsafe { libc::malloc(issuer_len + 1) as *mut ::core::ffi::c_char };
    unsafe { (*cert_info).issuer = issuer_buf; }
    if issuer_buf.is_null() { crate::src_app_verify::FreeCertInfo(cert_info); unsafe { libc::free(cert_info as *mut ::core::ffi::c_void); } return crate::types::V_ERR_MALLOC as i32; }
    unsafe { *issuer_buf.add(issuer_len) = 0; }
    unsafe { std::ptr::copy_nonoverlapping(ctr_ref.issuer_raw.p as *const u8, issuer_buf as *mut u8, issuer_len); }
    let subject_len = subject_len_raw as usize;
    let subject_buf = unsafe { libc::malloc(subject_len + 1) as *mut ::core::ffi::c_char };
    unsafe { (*cert_info).subject = subject_buf; }
    if subject_buf.is_null() { crate::src_app_verify::FreeCertInfo(cert_info); unsafe { libc::free(cert_info as *mut ::core::ffi::c_void); } return crate::types::V_ERR_MALLOC as i32; }
    unsafe { *subject_buf.add(subject_len) = 0; }
    unsafe { std::ptr::copy_nonoverlapping(ctr_ref.subject_raw.p as *const u8, subject_buf as *mut u8, subject_len); }
    let pk_ptr: *const crate::types::mbedtls_pk_context = &ctr_ref.pk as *const _;
    let mut pk_len: i32 = 0;
    let (pk_buf, pk_type) = {
        let rsa_buf = crate::src_app_verify::GetRsaPk(pk_ptr, &mut pk_len);
        if !rsa_buf.is_null() { (rsa_buf, crate::types::MBEDTLS_PK_RSA) }
        else {
            let ec_buf = crate::src_app_verify::GetEcPk(pk_ptr, &mut pk_len);
            if !ec_buf.is_null() { (ec_buf, crate::types::MBEDTLS_PK_ECDSA) }
            else { (std::ptr::null_mut(), 0u32) }
        }
    };
    unsafe { (*cert_info).pkBuf = pk_buf as *mut ::core::ffi::c_char; (*cert_info).pkLen = pk_len; (*cert_info).pkType = pk_type; }
    if pk_buf.is_null() { crate::src_app_verify::FreeCertInfo(cert_info); unsafe { libc::free(cert_info as *mut ::core::ffi::c_void); } return crate::types::V_ERR as i32; }
    unsafe { *binSignCert = cert_info; }
    crate::types::V_OK as i32
}

fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf)-> i32 {
    if pf.is_null() { return crate::types::V_ERR as i32; }
    let ret = if certType == crate::types::CERT_TYPE_APPGALLARY as i32 {
        crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf)
    } else {
        crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert, pf)
    };
    ret
}

fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7)-> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 { return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32; }
    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 { return ret; }
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, Some(calc_digest_for_verify_callback));
    if ret != crate::types::V_OK as i32 { return crate::types::V_ERR_VERIFY_SIGNATURE as i32; }
    crate::types::V_OK as i32
}

fn GetBinSignPkcs(signBuf: *const std::ffi::c_char, len: i32)-> *mut crate::types::Pkcs7 {
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) as *mut crate::types::Pkcs7 };
    if pkcs7.is_null() { return std::ptr::null_mut(); }
    let ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(signBuf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7);
    if ret != crate::types::V_OK as i32 {
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
    pkcs7
}

fn GetFileRead(fp: i32, offset: i32, size: i32)-> *mut crate::types::FileRead {
    let file_read = unsafe { libc::malloc(core::mem::size_of::<crate::types::FileRead>()) as *mut crate::types::FileRead };
    if file_read.is_null() { return core::ptr::null_mut(); }
    let fr = unsafe { &mut *file_read };
    fr.fp = fp;
    fr.offset = offset;
    fr.len = size;
    file_read
}

fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32)-> i32 {
    let mut ret: i32 = crate::types::V_OK as i32;
    let mut signBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut pkcs7: *mut crate::types::Pkcs7 = std::ptr::null_mut();
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    'cleanup: {
        let mut blockLen: i32 = 0;
        let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };
        signBuf = crate::src_app_verify::GetSignBlockByType(signInfo as *const crate::types::SignatureInfo, fp, crate::types::SIGNATURE_BLOCK_TYPE as i32, &mut blockLen, &mut blockHead);
        if signBuf.is_null() { ret = crate::types::V_ERR_GET_SIGN_BLOCK as i32; break 'cleanup; }
        pkcs7 = crate::src_app_verify::GetBinSignPkcs(signBuf as *const ::core::ffi::c_char, blockLen);
        if pkcs7.is_null() { ret = crate::types::V_ERR_PARSE_PKC7_DATA as i32; break 'cleanup; }
        fileRead = crate::src_app_verify::GetFileRead(fp, 0, blockHead.offset as i32);
        if fileRead.is_null() { ret = crate::types::V_ERR_MALLOC as i32; break 'cleanup; }
        let (singer_ret, singer_cert_val) = crate::src_app_verify::GetAppSingerCertType(pkcs7);
        let singer_ret = singer_ret;
        ret = singer_ret;
        if ret != crate::types::V_OK as i32 { ret = crate::types::V_ERR_GET_CERT_TYPE as i32; break 'cleanup; }
        unsafe { (*signInfo).certType = singer_cert_val; }
        ret = crate::src_app_verify::VerifyAppSignPkcsData(fileRead as *const crate::types::FileRead, signInfo as *const crate::types::SignatureInfo, pkcs7 as *const crate::types::Pkcs7);
        if ret != crate::types::V_OK as i32 { ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32; break 'cleanup; }
        let pkcs7_ref = unsafe { &*pkcs7 };
        ret = crate::src_app_verify::GetCertInfo(pkcs7_ref.signedData.signers.certPath.crt as *const crate::types::mbedtls_x509_crt, signCert);
        if ret != crate::types::V_OK as i32 { ret = crate::types::V_ERR_GET_CERT_INFO as i32; break 'cleanup; }
    }
    if !signBuf.is_null() { unsafe { libc::free(signBuf as *mut ::core::ffi::c_void); } }
    if !pkcs7.is_null() { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); unsafe { libc::free(pkcs7 as *mut ::core::ffi::c_void); } }
    if !fileRead.is_null() { unsafe { libc::free(fileRead as *mut ::core::ffi::c_void); } }
    ret
}

fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf)-> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert as *mut *mut crate::types::CertInfo, &mut certType as *mut i32);
    if ret != 0 { eprintln!("[VerifyIntegrity] verify bin sign error"); return ret; }
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const crate::types::SignatureInfo, certType, binSignCert, pf);
    if ret != 0 {
        eprintln!("[VerifyIntegrity] verify app source failed: {}", ret);
        crate::src_app_verify::FreeCertInfo(binSignCert);
        unsafe { libc::free(binSignCert as *mut ::core::ffi::c_void); }
        return ret;
    }
    crate::src_app_verify::FreeCertInfo(binSignCert);
    unsafe { libc::free(binSignCert as *mut ::core::ffi::c_void); }
    0
}

pub extern "C" fn APPVERI_AppVerify(filePath: *const ::core::ffi::c_char, verifyRst: *mut crate::types::VerifyResult) -> i32 {
    if filePath.is_null() || verifyRst.is_null() { return crate::types::V_ERR_FILE_OPEN as i32; }
    let mut handle: i32 = 0;
    let mut file: crate::types::FileRead = crate::types::FileRead { fp: 0, offset: 0, len: 0 };
    let init_ret = crate::src_app_file::InitVerify(&mut file as *mut crate::types::FileRead, filePath, &mut handle as *mut i32);
    if init_ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle); }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut signInfo: crate::types::SignatureInfo = unsafe { std::mem::zeroed() };
    let gsh_ret = crate::src_app_verify::GetSignHead(&file as *const crate::types::FileRead, &mut signInfo as *mut crate::types::SignatureInfo);
    if gsh_ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle); if !signInfo.signHead.is_null() { libc::free(signInfo.signHead as *mut ::core::ffi::c_void); } }
        return crate::types::V_ERR_GET_SIGNHEAD as i32;
    }
    let mut pf: crate::types::ProfileProf = unsafe { std::mem::zeroed() };
    let ret = crate::src_app_verify::VerifyIntegrity(&mut signInfo as *mut crate::types::SignatureInfo, file.fp, &mut pf as *mut crate::types::ProfileProf);
    if ret != crate::types::V_OK as i32 {
        unsafe { libc::close(handle); if !signInfo.signHead.is_null() { libc::free(signInfo.signHead as *mut ::core::ffi::c_void); } }
        return ret;
    }
    let fileSt: *mut crate::compat::stat = unsafe { crate::compat::malloc(::core::mem::size_of::<crate::compat::stat>() as crate::types::size_t) as *mut crate::compat::stat };
    if fileSt.is_null() {
        unsafe { libc::close(handle); if !signInfo.signHead.is_null() { libc::free(signInfo.signHead as *mut ::core::ffi::c_void); } }
        crate::src_app_provision::ProfFreeData(&mut pf as *mut crate::types::ProfileProf);
        return crate::types::V_ERR_MALLOC as i32;
    }
    let fstat_ret = unsafe { crate::compat::fstat(handle, fileSt) };
    if fstat_ret != 0 {
        unsafe { libc::close(handle); if !signInfo.signHead.is_null() { libc::free(signInfo.signHead as *mut ::core::ffi::c_void); } }
        crate::src_app_provision::ProfFreeData(&mut pf as *mut crate::types::ProfileProf);
        unsafe { crate::compat::free(fileSt as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    {
        let vr = unsafe { &mut *verifyRst };
        vr.profile = pf;
    }
    unsafe { crate::compat::free(fileSt as *mut ::core::ffi::c_void); }
    unsafe { libc::close(handle); if !signInfo.signHead.is_null() { libc::free(signInfo.signHead as *mut ::core::ffi::c_void); } }
    crate::types::V_OK as i32
}

pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    // LOG_INFO omitted (logging side-effect not critical for semantics, no HiLogPrint in this module context)
    if g_isDebugMode.load(Ordering::Relaxed) == mode {
        return crate::types::V_OK as i32;
    }
    let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
    if ret != crate::types::V_OK as i32 {
        // LOG_ERROR omitted (logging side-effect not critical for semantics)
        return ret;
    }
    g_isDebugMode.store(mode, Ordering::Relaxed);
    crate::types::V_OK as i32
}

pub extern "C" fn APPVERI_SetActsMode(mode: bool) { g_isActsMode.store(mode, Ordering::Relaxed); }

pub extern "C" fn APPVERI_GetUnsignedFileLength(filePath: *const ::core::ffi::c_char) -> i32 {
    let mut file: crate::types::FileRead = unsafe { std::mem::zeroed() };
    let mut handle: i32 = 0;
    let ret = crate::src_app_file::InitVerify(&mut file as *mut crate::types::FileRead, filePath, &mut handle as *mut i32);
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::close(handle); }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let mut sign_info: crate::types::SignatureInfo = unsafe { std::mem::zeroed() };
    let gsh_ret = crate::src_app_verify::GetSignHead(&file as *const crate::types::FileRead, &mut sign_info as *mut crate::types::SignatureInfo);
    if gsh_ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::close(handle); if !sign_info.signHead.is_null() { crate::compat::free(sign_info.signHead as *mut ::core::ffi::c_void); } }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let unsigned_len = sign_info.fullSignBlockOffset;
    unsafe { crate::compat::close(handle); if !sign_info.signHead.is_null() { crate::compat::free(sign_info.signHead as *mut ::core::ffi::c_void); } }
    unsigned_len
}

pub extern "C" fn APPVERI_RegisterMsgFunc(messageFunc: crate::types::MessageFunc) { unsafe { crate::globals::g_message_func = messageFunc; } }

pub extern "C" fn CalculateHash(input: *const ::core::ffi::c_uchar, len: i32, hashAlg: i32, output: *mut ::core::ffi::c_uchar) -> i32 {
    if input.is_null() || output.is_null() || len <= 0 { return crate::types::V_ERR as i32; }
    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t) };
    if md_info.is_null() { return crate::types::V_ERR as i32; }
    let ret = unsafe { crate::compat::mbedtls_md(md_info, input, len as crate::types::size_t, output) };
    if ret != 0 { crate::types::V_ERR as i32 } else { crate::types::V_OK as i32 }
}

pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() { return; }
    unsafe { crate::src_app_provision::ProfFreeData(::core::ptr::addr_of_mut!((*verifyRst).profile)); }
}

pub extern "C" fn APPVERI_IsActsMode() -> i32 { crate::globals::g_isActsMode.load(Ordering::Relaxed) as i32 }
