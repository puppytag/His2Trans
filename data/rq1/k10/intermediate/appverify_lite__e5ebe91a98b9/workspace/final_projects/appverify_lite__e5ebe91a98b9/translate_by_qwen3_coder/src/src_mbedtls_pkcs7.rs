//! Module: src_mbedtls_pkcs7
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
/// C: static const unsigned char[821] DEBUG_MODE_ROOT_CERT_IN_PEM
static mut DEBUG_MODE_ROOT_CERT_IN_PEM: [::core::ffi::c_uchar; 821usize] = unsafe { core::mem::MaybeUninit::<[::core::ffi::c_uchar; 821usize]>::zeroed().assume_init() };

/// C: static const unsigned char[863] OHOS_ROOT_CERT_IN_PEM
static mut OHOS_ROOT_CERT_IN_PEM: [::core::ffi::c_uchar; 863usize] = unsafe { core::mem::MaybeUninit::<[::core::ffi::c_uchar; 863usize]>::zeroed().assume_init() };

/// C: static const unsigned char[805] ROOT_CA_G2_CERT_IN_PEM
static mut ROOT_CA_G2_CERT_IN_PEM: [::core::ffi::c_uchar; 805usize] = unsafe { core::mem::MaybeUninit::<[::core::ffi::c_uchar; 805usize]>::zeroed().assume_init() };

/// C: static _Bool g_debugModeEnabled
static mut g_debugModeEnabled: bool = unsafe { core::mem::MaybeUninit::<bool>::zeroed().assume_init() };

/// C: static mbedtls_x509_crt g_debugModeRootCert
static mut g_debugModeRootCert: crate::types::mbedtls_x509_crt = unsafe { core::mem::MaybeUninit::<crate::types::mbedtls_x509_crt>::zeroed().assume_init() };

/// C: static mbedtls_x509_crt g_ohosRootCert
static mut g_ohosRootCert: crate::types::mbedtls_x509_crt = unsafe { core::mem::MaybeUninit::<crate::types::mbedtls_x509_crt>::zeroed().assume_init() };

/// C: static mbedtls_x509_crt g_rootCaG2Cert
static mut g_rootCaG2Cert: crate::types::mbedtls_x509_crt = unsafe { core::mem::MaybeUninit::<crate::types::mbedtls_x509_crt>::zeroed().assume_init() };

/// C: static _Bool g_rootCertLoaded
static mut g_rootCertLoaded: bool = unsafe { core::mem::MaybeUninit::<bool>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

fn InvalidDigestAlg(alg: *const crate::types::mbedtls_asn1_buf) -> bool {
    unsafe {
        let len = (*alg).len as usize;
        let p = (*alg).p;
        let sha256 = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256;
        let sha384 = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384;
        let sha512 = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512;
        (len != sha256.len() - 1 || libc::memcmp(sha256.as_ptr() as *const _, p as *const _, len) != 0) &&
        (len != sha384.len() - 1 || libc::memcmp(sha384.as_ptr() as *const _, p as *const _, len) != 0) &&
        (len != sha512.len() - 1 || libc::memcmp(sha512.as_ptr() as *const _, p as *const _, len) != 0)
    }
}

fn GetContentInfoType(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, contentType: *mut crate::types::mbedtls_asn1_buf, hasContent: *mut crate::types::c_bool) -> i32 {
    let mut seqLen: crate::types::size_t = 0;
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;
    rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut seqLen, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }
    let start: *mut ::core::ffi::c_uchar = unsafe { *p };
    let end: *const ::core::ffi::c_uchar = unsafe { start.offset(seqLen as isize) };
    rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut len, 0x06) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*contentType).tag = 0x06;
        (*contentType).len = len;
        (*contentType).p = *p;
        let diff = (*p).offset_from(start) as crate::types::size_t;
        *hasContent = (seqLen != len.wrapping_add(diff)) as crate::types::c_bool;
        *p = (*p).offset(len as isize);
    }
    return PKCS7_SUCC as i32;
}

fn GetContentLenOfContentInfo(p: *mut *mut u8, end: *const u8, len: *mut size_t) -> i32 {
    unsafe {
        crate::compat::mbedtls_asn1_get_tag(p as *mut *mut ::core::ffi::c_uchar, end as *const ::core::ffi::c_uchar, len, 0x20 | 0x80)
    }
}

fn ParseSignerVersion(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    unsafe {
        crate::compat::mbedtls_asn1_get_int(p, end, &mut (*signer).version)
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_5
// c_function: ParseSignerIssuerAndSerialNum
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: fn ParseSignerIssuerAndSerialNum(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32
// c_first_line: static int32_t ParseSignerIssuerAndSerialNum(unsigned char **p, const unsigned char *end, SignerInfo *signer)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_5/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `mbedtls_x509_get_name` in module `crate::compat`
//      --> src/src_mbedtls_pkcs7.rs:110:34
//       |
//       |                                  ^^^^^^^^^^^^^^^^^^^^^
//       |
//       |
//       |     ----------------------------------------------------------------------- similarly named function `mbedtls_x509_crt_free` defined here
//       |
// =================================
fn ParseSignerIssuerAndSerialNum(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_5
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_5/translated_rust.rs
 * ------------------------------------------------------------
fn ParseSignerIssuerAndSerialNum(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*signer).issuerRaw.p = *p;
    }
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }
    let current_ptr = unsafe { *p };
    let limit_ptr = unsafe { current_ptr.offset(len as isize) };
    rc = unsafe { crate::compat::mbedtls_x509_get_name(p, limit_ptr, &mut (*signer).issuer) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        let issuer_raw_len = (*p).offset_from((*signer).issuerRaw.p) as crate::types::size_t;
        (*signer).issuerRaw.len = issuer_raw_len;
    }
    rc = unsafe { crate::compat::mbedtls_x509_get_serial(p, end, &mut (*signer).serial) };
    return rc;
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_5
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn ParseSignerDigestAlg(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let rc = unsafe { crate::compat::mbedtls_asn1_get_alg_null(p, end, &mut (*signer).digestAlgId) };
    if rc != 0 {
        return rc;
    }
    if unsafe { crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*signer).digestAlgId as *const _) } {
        return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
    }
    crate::types::PKCS7_SUCC as i32
}

fn ParseSignerAuthAttr(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let raw: *mut ::core::ffi::c_uchar;
    unsafe {
        raw = *p;
    }
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x80) };
    if rc != 0 {
        return crate::types::PKCS7_SUCC as i32;
    }
    unsafe {
        (*signer).authAttr.tag = (0x20 | 0x80) as i32;
        (*signer).authAttr.p = *p;
        (*signer).authAttr.len = len;
        let tlLen = (*p).offset_from(raw) as crate::types::size_t;
        *p = (*p).offset(len as isize);
        (*signer).authAttrRaw.p = raw;
        (*signer).authAttrRaw.len = len + tlLen;
    }
    return crate::types::PKCS7_SUCC as i32;
}

fn InvalidDigestEncAlg(alg: *const crate::types::mbedtls_x509_buf) -> bool {
    unsafe {
        let len = (*alg).len as usize;
        let p = (*alg).p;
        !(
            (crate::types::MBEDTLS_OID_PKCS1_SHA256.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_PKCS1_SHA256.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_PKCS1_SHA384.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_PKCS1_SHA384.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_PKCS1_SHA512.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_PKCS1_SHA512.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_ECDSA_SHA256.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_ECDSA_SHA256.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_ECDSA_SHA384.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_ECDSA_SHA384.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_ECDSA_SHA512.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_ECDSA_SHA512.as_ptr() as *const _, p as *const _, len) == 0) ||
            (crate::types::MBEDTLS_OID_RSASSA_PSS.len() - 1 == len && libc::memcmp(crate::types::MBEDTLS_OID_RSASSA_PSS.as_ptr() as *const _, p as *const _, len) == 0)
        )
    }
}

fn ParseSignerEncAlg(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut params = crate::types::mbedtls_asn1_buf {
        tag: 0,
        len: 0,
        p: std::ptr::null_mut(),
    };
    unsafe {
        rc = crate::compat::mbedtls_asn1_get_alg(p, end, &mut (*signer).digestEncAlgId, &mut params);
    }
    if rc != 0 {
        return rc;
    }
    unsafe {
        if crate::src_mbedtls_pkcs7::InvalidDigestEncAlg(&(*signer).digestEncAlgId as *const crate::types::mbedtls_x509_buf) {
            return crate::types::PKCS7_INVALID_SIGNING_ALG as i32;
        }
    }
    return crate::types::PKCS7_SUCC as i32;
}

fn ParseSignerSignature(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x04) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*signer).signature.tag = 0x04;
        (*signer).signature.len = len;
        (*signer).signature.p = *p;
        *p = (*p).offset(len as isize);
    }
    return crate::types::PKCS7_SUCC as i32;
}

fn GetSignerSignature(signer: *const crate::types::SignerInfo, sig: *mut *mut ::core::ffi::c_uchar, sigLen: *mut crate::types::size_t) -> i32 {
    unsafe {
        let len = (*signer).signature.len;
        let buf = (*signer).signature.p;
        *sig = buf;
        *sigLen = len;
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerUnAuthAttr(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        if (end as usize).wrapping_sub(*p as usize) < 1 {
            return crate::types::PKCS7_SUCC as i32;
        }
        rc = mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x80) + 1);
        if rc != 0 {
            return rc;
        }
        (*signer).unAuthAttr.tag = (0x20 | 0x80) + 1;
        (*signer).unAuthAttr.len = len;
        (*signer).unAuthAttr.p = *p;
        *p = (*p).wrapping_add(len as usize);
    }
    return crate::types::PKCS7_SUCC as i32;
}

fn SerialCmp(a: *const crate::types::mbedtls_x509_buf, b: *const crate::types::mbedtls_x509_buf) -> i32 {
    unsafe {
        if (*a).len == (*b).len && libc::memcmp((*a).p as *const _, (*b).p as *const _, (*a).len as usize) == 0 {
            return 0;
        }
        -1
    }
}

fn IsLegitString(tag: i32) -> bool {
    if tag == 0x0C || tag == 0x13 {
        return true;
    }
    false
}

fn CompareX509String(first: *const crate::types::mbedtls_x509_buf, second: *const crate::types::mbedtls_x509_buf) -> i32 {
    unsafe {
        if crate::src_mbedtls_pkcs7::IsLegitString((*first).tag) && crate::src_mbedtls_pkcs7::IsLegitString((*second).tag) {
            let len = (*first).len as i32;
            for i in 0..len {
                let a = *(*first).p.offset(i as isize);
                let b = *(*second).p.offset(i as isize);
                if a == b ||
                    (((0 != 0 && crate::compat::islower(a as i32) != 0) || ((a as u32).wrapping_sub(b'a' as u32) < 26)) && a.wrapping_sub(32) == b) ||
                    (((0 != 0 && crate::compat::isupper(a as i32) != 0) || ((a as u32).wrapping_sub(b'A' as u32) < 26)) && a.wrapping_add(32) == b) {
                    continue;
                }
                return -1;
            }
            return 0;
        }
        -1
    }
}

fn GetDeps(nameList: *const crate::types::mbedtls_x509_name) -> i32 {
    let mut deps: i32 = 0;
    let mut current = nameList;
    while !current.is_null() {
        deps += 1;
        unsafe {
            current = (*current).next as *const crate::types::mbedtls_x509_name;
        }
    }
    deps
}

fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name) -> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    let first_deps = crate::src_mbedtls_pkcs7::GetDeps(first);
    let second_deps = crate::src_mbedtls_pkcs7::GetDeps(second);
    if first_deps != second_deps {
        return -1;
    }
    let mut first_ptr = first;
    let mut second_ptr = second;
    for _ in 0..first_deps {
        unsafe {
            if (*first_ptr).oid.tag != (*second_ptr).oid.tag ||
                (*first_ptr).oid.len != (*second_ptr).oid.len ||
                libc::memcmp(
                    (*first_ptr).oid.p as *const core::ffi::c_void,
                    (*second_ptr).oid.p as *const core::ffi::c_void,
                    (*second_ptr).oid.len as usize,
                ) != 0 ||
                (*first_ptr).private_next_merged != (*second_ptr).private_next_merged ||
                (*first_ptr).val.len != (*second_ptr).val.len
            {
                return -1;
            }
        }
        unsafe {
            if crate::src_mbedtls_pkcs7::CompareX509String(&(*first_ptr).val, &(*second_ptr).val) != 0 {
                return -1;
            }
            first_ptr = (*first_ptr).next;
            second_ptr = (*second_ptr).next;
        }
    }
    0
}

fn Pkcs7Calloc(nmemb: crate::types::size_t, size: crate::types::size_t) -> *mut std::ffi::c_void {
    unsafe { libc::calloc(nmemb as usize, size as usize) }
}

fn Pkcs7Free(ptr: *mut std::ffi::c_void) {
    unsafe {
        libc::free(ptr);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_20
// c_function: ParseSignedDataSignerInfos
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32
// c_first_line: static int32_t ParseSignedDataSignerInfos(unsigned char **p, const unsigned char *end, SignerInfo *signers)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_20/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_mbedtls_pkcs7.rs:497:21
//       |
//       |                     ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_mbedtls_pkcs7.rs:495:21
//       |
//       |                     ^^^^^^^^^^^^
// =================================
fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_20
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_20/translated_rust.rs
 * ------------------------------------------------------------
fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x11);
        if rc != 0 || len == 0 {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        let mut end_local = (*p).wrapping_add(len as usize);
        while *p < end_local {
            let mut one_signer_len: crate::types::size_t = 0;
            rc = mbedtls_asn1_get_tag(p, end_local, &mut one_signer_len, 0x20 | 0x10);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    387,
                    __FUNCTION__!(),
                    387,
                    rc,
                );
                return rc;
            }
            let one_signer_end = (*p).wrapping_add(one_signer_len as usize);
            rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    392,
                    __FUNCTION__!(),
                    392,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    396,
                    __FUNCTION__!(),
                    396,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    400,
                    __FUNCTION__!(),
                    400,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    404,
                    __FUNCTION__!(),
                    404,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    408,
                    __FUNCTION__!(),
                    408,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    412,
                    __FUNCTION__!(),
                    412,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    416,
                    __FUNCTION__!(),
                    416,
                    rc,
                );
                return rc;
            }
            if *p < end_local {
                let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignerInfo>() as crate::types::size_t);
                if next.is_null() {
                    return crate::types::PKCS7_MEMORY_EXHAUST as i32;
                }
                (*signers).next = next as *mut crate::types::SignerInfo;
                signers = (*signers).next;
            }
        }
    }
    rc
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_20
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn ParseSignedDataVersion(p: *mut *mut u8, end: *const u8, ver: *mut i32) -> i32 {
    let rc = unsafe { crate::compat::mbedtls_asn1_get_int(p, end, ver) };
    if rc != 0 {
        return rc;
    }
    let version = unsafe { *ver };
    if version != 1 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Invalid version : %d\n\0".as_ptr() as *const i8, b"ParseSignedDataVersion\0".as_ptr() as *const i8, 438, version) };
        return crate::types::PKCS7_INVALID_VERSION as i32;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Parse signed data version success\n\0".as_ptr() as *const i8, b"ParseSignedDataVersion\0".as_ptr() as *const i8, 441) };
    crate::types::PKCS7_SUCC as i32
}

fn ParseSignedDataDigestAlgs(p: *mut *mut core::ffi::c_uchar, end: *const core::ffi::c_uchar, algIds: *mut crate::types::DigestAlgId) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x11) };
    if rc != 0 {
        return rc;
    }
    let end = unsafe { (*p).add(len as usize) };
    let mut id = algIds;
    while unsafe { *p < end } {
        let mut params = crate::types::mbedtls_asn1_buf {
            tag: 0,
            len: 0,
            p: std::ptr::null_mut(),
        };
        rc = unsafe { crate::compat::mbedtls_asn1_get_alg(p, end, &mut (*id).algBuf, &mut params) };
        if rc != 0 {
            return rc;
        }
        if unsafe { crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*id).algBuf as *const _) } {
            return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
        }
        if unsafe { *p < end } {
            let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::DigestAlgId>() as crate::types::size_t);
            if next.is_null() {
                return crate::types::PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe { (*id).next = next as *mut crate::types::DigestAlgId };
            id = unsafe { (*id).next };
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn DlogContentInfo(content: *const crate::types::Content) {
    if content.is_null() {
        return;
    }
    let len = unsafe { (*content).data.len };
    if len <= 0 {
        return;
    }
    let info = crate::src_mbedtls_pkcs7::Pkcs7Calloc((len + 1) as crate::types::size_t, std::mem::size_of::<std::ffi::c_char>() as crate::types::size_t);
    if info.is_null() {
        return;
    }
    let dest_max = (len + 1) as crate::types::size_t;
    let src = unsafe { (*content).data.p } as *const std::ffi::c_char;
    let ret = unsafe { crate::compat::strncpy_s(info as *mut std::ffi::c_char, dest_max, src, len as crate::types::size_t) };
    if ret != crate::types::EOK as i32 {
        crate::src_mbedtls_pkcs7::Pkcs7Free(info);
        return;
    }
    crate::src_mbedtls_pkcs7::Pkcs7Free(info);
}

fn ParseSignedDataContentInfo(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, content: *mut crate::types::Content) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: crate::types::c_bool = 0;

    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(
        p,
        end,
        unsafe { &mut (*content).oid } as *mut crate::types::mbedtls_asn1_buf,
        &mut hasContent as *mut crate::types::c_bool,
    );
    if rc != 0 {
        return rc;
    }

    let oid_len = unsafe { (*content).oid.len };
    let oid_p = unsafe { (*content).oid.p };
    let expected_oid = b"\x2a\x86\x48\x86\xf7\x0d\x01\x07\x01";
    let expected_len = expected_oid.len() as crate::types::size_t;
    if (expected_len != oid_len
        || unsafe { libc::memcmp(expected_oid.as_ptr() as *const _, oid_p as *const _, oid_len as usize) } != 0)
        || hasContent == 0
    {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: Invalid content type or has no real content\0".as_ptr() as *const _,
                b"ParseSignedDataContentInfo\0".as_ptr() as *const _,
                510,
            )
        };
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(
        p as *mut *mut u8,
        end as *const u8,
        &mut len as *mut crate::types::size_t,
    );
    if rc != 0 {
        return rc;
    }

    unsafe {
        (*content).data.tag = 0x20 | 0x80;
        (*content).data.p = *p;
        (*content).data.len = len;
    }

    crate::src_mbedtls_pkcs7::DlogContentInfo(content as *const crate::types::Content);

    unsafe {
        *p = (*p).add(len as usize);
    }

    crate::types::PKCS7_SUCC as i32
}

fn ParseSignedDataCerts(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, certs: *mut *mut crate::types::mbedtls_x509_crt) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x80) };
    if rc != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const _, b"ParseSignedDataCerts\0".as_ptr() as *const _, 532) };
        return crate::types::PKCS7_SUCC as i32;
    }
    unsafe {
        *certs = libc::calloc(1, std::mem::size_of::<crate::types::mbedtls_x509_crt>()) as *mut crate::types::mbedtls_x509_crt;
    }
    if unsafe { (*certs).is_null() } {
        return crate::types::PKCS7_MEMORY_EXHAUST as i32;
    }
    unsafe {
        crate::compat::mbedtls_x509_crt_init(*certs);
    }
    let certs_end: *mut ::core::ffi::c_uchar = unsafe { (*p).wrapping_add(len as usize) };
    let mut cnt: i32 = 0;
    while unsafe { (*p) < certs_end } {
        let mut one_cert_len: crate::types::size_t = 0;
        let seq_begin: *mut ::core::ffi::c_uchar = unsafe { *p };
        rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut one_cert_len, 0x20 | 0x10) };
        if rc != 0 {
            return rc;
        }
        let seq_len = unsafe { (*p).offset_from(seq_begin) as usize };
        if (one_cert_len as usize + seq_len) > unsafe { certs_end.offset_from(seq_begin) as usize } {
            return crate::types::PKCS7_PARSING_ERROR as i32;
        }
        rc = unsafe { crate::compat::mbedtls_x509_crt_parse(*certs, seq_begin, (one_cert_len as usize + seq_len) as crate::types::size_t) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            *p = (*p).wrapping_add(one_cert_len as usize);
        }
        cnt += 1;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Parse signed data certs success\0".as_ptr() as *const _, b"ParseSignedDataCerts\0".as_ptr() as *const _, 561) };
    rc
}

fn ParseSignedDataCrl(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, crl: *mut crate::types::mbedtls_x509_crl) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x80) + 1) };
    if rc != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Has no crl in signed data.\0".as_ptr() as *const _, b"ParseSignedDataCrl\0".as_ptr() as *const _, 572) };
        return crate::types::PKCS7_SUCC as i32;
    }
    unsafe { crate::compat::mbedtls_x509_crl_init(crl) };
    rc = unsafe { crate::compat::mbedtls_x509_crl_parse(crl, *p, len) };
    unsafe { *p = (*p).offset(len as isize) };
    rc
}

fn ParseSignedData(buf: *mut u8, bufLen: usize, signedData: *mut crate::types::SignedData) -> i32 {
    let mut p = buf;
    let end = unsafe { buf.add(bufLen) };
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(&mut p as *mut *mut u8, end as *const u8, &mut len, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataVersion(
        &mut p as *mut *mut u8,
        end as *const u8,
        unsafe { &mut (*signedData).version } as *mut i32,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataDigestAlgs(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).digestAlgIds } as *mut crate::types::DigestAlgId,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataContentInfo(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).content } as *mut crate::types::Content,
    );
    if rc != 0 {
        return rc;
    }

    if p >= end {
        return crate::types::PKCS7_PARSING_ERROR as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCerts(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).certs } as *mut *mut crate::types::mbedtls_x509_crt,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCrl(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).crl } as *mut crate::types::mbedtls_x509_crl,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataSignerInfos(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).signers } as *mut crate::types::SignerInfo,
    );
    let _ = unsafe { crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const ::core::ffi::c_char,
        "[%s:%d]: ParseSignedData %d\0".as_ptr() as *const ::core::ffi::c_char,
        "ParseSignedData\0".as_ptr() as *const ::core::ffi::c_char,
        629,
        rc,
    ) };
    return rc;
}

fn IsSigedDataOid(pkcs7: *const crate::types::Pkcs7) -> bool {
    if pkcs7.is_null() {
        return false;
    }
    unsafe {
        let oid = &(*pkcs7).contentTypeOid;
        let expected = crate::types::MBEDTLS_OID_PKCS7_SIGNED_DATA;
        if oid.len as usize != expected.len() - 1 {
            return false;
        }
        libc::memcmp(
            expected.as_ptr() as *const _,
            oid.p as *const _,
            oid.len as usize,
        ) == 0
    }
}

fn FreeSignedDataDigestAlgs(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut alg = (*pkcs7).signedData.digestAlgIds.next;
        let mut next = std::ptr::null_mut();
        while !alg.is_null() {
            next = (*alg).next;
            crate::src_mbedtls_pkcs7::Pkcs7Free(alg as *mut std::ffi::c_void);
            alg = next;
        }
        (*pkcs7).signedData.digestAlgIds.next = std::ptr::null_mut();
    }
}

fn FreeSignerCerts(signer: *mut crate::types::SignerInfo) {
    unsafe {
        if !(*signer).certPath.crt.is_null() {
            crate::compat::mbedtls_x509_crt_free((*signer).certPath.crt);
            crate::compat::mbedtls_free((*signer).certPath.crt as *mut std::ffi::c_void);
            (*signer).certPath.crt = std::ptr::null_mut();
        }
    }
}

fn FreeSignerIssuer(signer: *mut crate::types::SignerInfo) {
    unsafe {
        let mut name_cur = (*signer).issuer.next;
        while !name_cur.is_null() {
            let name_prv = name_cur;
            name_cur = (*name_cur).next;
            crate::compat::mbedtls_free(name_prv as *mut ::core::ffi::c_void);
        }
        (*signer).issuer.next = std::ptr::null_mut();
    }
}

fn FreeSignersInfo(pkcs7: *mut crate::types::Pkcs7) {
    if pkcs7.is_null() {
        return;
    }
    unsafe {
        let mut signer = (*pkcs7).signedData.signers.next;
        let mut next = std::ptr::null_mut();
        while !signer.is_null() {
            next = (*signer).next;
            crate::src_mbedtls_pkcs7::FreeSignerCerts(signer);
            crate::src_mbedtls_pkcs7::FreeSignerIssuer(signer);
            crate::src_mbedtls_pkcs7::Pkcs7Free(signer as *mut std::ffi::c_void);
            signer = next;
        }
        (*pkcs7).signedData.signers.next = std::ptr::null_mut();
        crate::src_mbedtls_pkcs7::FreeSignerCerts(&mut (*pkcs7).signedData.signers);
        crate::src_mbedtls_pkcs7::FreeSignerIssuer(&mut (*pkcs7).signedData.signers);
    }
}

fn FreeSignedDataCerts(pkcs7: *mut crate::types::Pkcs7) {
    if pkcs7.is_null() {
        return;
    }
    unsafe {
        let certs = (*pkcs7).signedData.certs;
        if !certs.is_null() {
            crate::compat::mbedtls_x509_crt_free(certs);
            crate::compat::mbedtls_free(certs as *mut std::ffi::c_void);
            (*pkcs7).signedData.certs = std::ptr::null_mut();
        }
    }
}

fn FreeSignedDataCrl(pkcs7: *mut crate::types::Pkcs7) {
    if !pkcs7.is_null() {
        unsafe {
            crate::compat::mbedtls_x509_crl_free(&mut (*pkcs7).signedData.crl);
        }
    }
}

fn GetCertsNumOfSignedData(crts: *const crate::types::mbedtls_x509_crt) -> i32 {
    let mut cnt = 0;
    let mut current = crts;
    while !current.is_null() {
        unsafe {
            current = (*current).next as *const crate::types::mbedtls_x509_crt;
        }
        cnt += 1;
    }
    cnt
}

fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut list = certsList;
    while !list.is_null() {
        let cmp = unsafe {
            crate::src_mbedtls_pkcs7::CompareX509NameList(
                &(*cur).issuer as *const crate::types::mbedtls_x509_name,
                &(*list).subject as *const crate::types::mbedtls_x509_name,
            )
        };
        if cmp == 0 {
            break;
        }
        list = unsafe { (*list).next };
    }
    list
}

fn DelCertOfSignedData(signedData: *mut crate::types::SignedData, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let mut head = (*signedData).certs;
        if crt == head {
            (*signedData).certs = (*crt).next;
            (*crt).next = std::ptr::null_mut();
        } else {
            let mut prev = head;
            while !head.is_null() {
                if head == crt {
                    (*prev).next = (*crt).next;
                    (*crt).next = std::ptr::null_mut();
                    break;
                }
                prev = head;
                head = (*head).next;
            }
        }
    }
}

fn AddCertToSignerCertPath(signer: *mut crate::types::SignerInfo, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let mut prev = (*signer).certPath.crt;
        let mut cur = prev;
        if prev.is_null() {
            (*signer).certPath.crt = crt;
            (*crt).next = std::ptr::null_mut();
        } else {
            while !cur.is_null() {
                prev = cur;
                cur = (*cur).next;
            }
            (*prev).next = crt;
            (*crt).next = std::ptr::null_mut();
        }
        (*signer).certPath.depth += 1;
    }
}

fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData) -> i32 {
    let mut scan_cnt: i32 = 0;
    let mut rc: i32 = crate::types::PKCS7_SUCC as i32;
    unsafe {
        if crate::globals::g_rootCertLoaded == 0 {
            return crate::types::PKCS7_ROOT_CA_NOT_VALID as i32;
        }
        (*signer).rootCert = std::ptr::null_mut();
        let certs = (*signeData).certs;
        let mut cur = lowerCrt;
        let mut next: *mut crate::types::mbedtls_x509_crt = std::ptr::null_mut();
        let certs_cnt = crate::src_mbedtls_pkcs7::GetCertsNumOfSignedData(certs as *const crate::types::mbedtls_x509_crt);
        crate::src_mbedtls_pkcs7::DelCertOfSignedData(signeData, cur);
        crate::src_mbedtls_pkcs7::AddCertToSignerCertPath(signer, cur);
        loop {
            next = crate::src_mbedtls_pkcs7::FindSuperCert(cur, (*signeData).certs);
            if next.is_null() {
                break;
            } else {
                crate::src_mbedtls_pkcs7::DelCertOfSignedData(signeData, next);
                crate::src_mbedtls_pkcs7::AddCertToSignerCertPath(signer, next);
            }
            scan_cnt += 1;
            if scan_cnt > certs_cnt {
                rc = crate::types::PKCS7_BUILD_CERT_PATH_FAIL as i32;
                break;
            }
            cur = next;
        }
    }
    rc
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_40
// c_function: ConstructSignerCerts
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32
// c_first_line: static int32_t ConstructSignerCerts(SignedData *signedData)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_40/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//       --> src/src_mbedtls_pkcs7.rs:1049:21
//        |
//        |                     ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//       --> src/src_mbedtls_pkcs7.rs:1035:25
//        |
//        |                         ^^^^^^^^^^^^
// =================================
fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_40
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_40/translated_rust.rs
 * ------------------------------------------------------------
fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32 {
    unsafe {
        let mut signer = &mut (*signedData).signers as *mut crate::types::SignerInfo;
        while !signer.is_null() {
            let signerSerial = &(*signer).serial as *const crate::types::mbedtls_x509_buf;
            let signerIssuer = &(*signer).issuer as *const crate::types::mbedtls_x509_name;
            let mut cert = (*signedData).certs;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: To filter one signer's cert\0".as_ptr() as *const i8,
                __FUNCTION__!(),
                809,
            );
            while !cert.is_null() {
                if crate::src_mbedtls_pkcs7::SerialCmp(signerSerial, &(*cert).serial as *const crate::types::mbedtls_x509_buf) == 0
                    && crate::src_mbedtls_pkcs7::CompareX509NameList(signerIssuer, &(*cert).issuer as *const crate::types::mbedtls_x509_name) == 0
                {
                    let _ = crate::compat::HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        "appverify\0".as_ptr() as *const i8,
                        "[%s:%d]: Found signer's low level cert\0".as_ptr() as *const i8,
                        __FUNCTION__!(),
                        813,
                    );
                    break;
                }
                cert = (*cert).next;
            }
            if cert.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: Could not found signer's lowest cert\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    819,
                );
                return crate::types::PKCS7_INVALID_VALUE as i32;
            }
            let rc = crate::src_mbedtls_pkcs7::BuildSignerCertPath(signer, cert, signedData);
            if rc != 0 {
                return rc;
            }
            signer = (*signer).next;
        }
        0
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_40
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetSignerDigestAlg(signer: *const crate::types::SignerInfo, algType: *mut crate::types::mbedtls_md_type_t) -> i32 {
    unsafe {
        let alg = &(*signer).digestAlgId;
        let sha256_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256;
        let sha384_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384;
        let sha512_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512;
        if !((sha256_oid.len() - 1) != alg.len as usize || libc::memcmp(sha256_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA256;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((sha384_oid.len() - 1) != alg.len as usize || libc::memcmp(sha384_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA384;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((sha512_oid.len() - 1) != alg.len as usize || libc::memcmp(sha512_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA512;
            return crate::types::PKCS7_SUCC as i32;
        }
        return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
    }
}

fn GetSignerPubKeyOfSignature(signer: *const crate::types::SignerInfo, pk: *mut *mut crate::types::mbedtls_pk_context) -> i32 {
    if signer.is_null() || pk.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        let crt = (*signer).certPath.crt;
        if !crt.is_null() {
            *pk = &mut (*crt).pk;
            return crate::types::PKCS7_SUCC as i32;
        }
    }
    crate::types::PKCS7_INVALID_VALUE as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_43
// c_function: PKCS7_VerifySignerSignature
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32
// c_first_line: int32_t PKCS7_VerifySignerSignature(const Pkcs7 *pkcs7, PKCS7_CalcDigest calcDigest)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_43/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_mbedtls_pkcs7.rs:702:248
//       |
//       |           --------------------- arguments to this function are incorrect                                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `usize`
//       |
//      --> src/compat.rs:157:12
//       |
//       |            ^^^^^^                                                                     ----
// =================================
pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_43
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_43/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    let mut rc: i32 = 0;
    if pkcs7.is_null() || calcDigest.is_none() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo;
    let mut sig: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut sigLen: crate::types::size_t = 0;
    while !signer.is_null() {
        rc = crate::src_mbedtls_pkcs7::GetSignerSignature(signer, &mut sig, &mut sigLen);
        if rc != 0 {
            return rc;
        }
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get signer signature len : %zu\0".as_ptr() as *const i8, b"PKCS7_VerifySignerSignature\0".as_ptr() as *const i8, 878, sigLen as u32) };
        let mut pk: *mut crate::types::mbedtls_pk_context = std::ptr::null_mut();
        rc = crate::src_mbedtls_pkcs7::GetSignerPubKeyOfSignature(signer, &mut pk);
        if rc != 0 {
            return rc;
        }
        let mut digAlg: crate::types::mbedtls_md_type_t = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerDigestAlg(signer, &mut digAlg);
        if rc != 0 {
            return rc;
        }
        let mut hash: [::core::ffi::c_uchar; 64] = [0; 64];
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut ::core::ffi::c_void, 64, 0, 64) };
        let mut hashLen: crate::types::size_t = 0;
        rc = if let Some(f) = calcDigest {
            unsafe { f(pkcs7, signer, digAlg, hash.as_mut_ptr(), &mut hashLen) }
        } else {
            crate::types::PKCS7_INVALID_PARAM as i32
        };
        if rc != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Calculate content hash failed by calling callback\0".as_ptr() as *const i8, b"PKCS7_VerifySignerSignature\0".as_ptr() as *const i8, 895) };
            return rc;
        }
        unsafe {
            let digest_enc_alg = &(*signer).digestEncAlgId;
            if !((crate::types::MBEDTLS_OID_RSASSA_PSS.len() - 1) != digest_enc_alg.len as usize || crate::compat::memcmp(crate::types::MBEDTLS_OID_RSASSA_PSS.as_ptr() as *const ::core::ffi::c_void, digest_enc_alg.p as *const ::core::ffi::c_void, digest_enc_alg.len as usize) != 0) {
                crate::compat::mbedtls_rsa_set_padding((*pk).private_pk_ctx as *mut crate::types::mbedtls_rsa_context, crate::types::MBEDTLS_RSA_PKCS_V21 as i32, 0);
            }
        }
        rc = unsafe { crate::compat::mbedtls_pk_verify(pk, digAlg, hash.as_ptr(), hashLen as u32, sig, sigLen as u32) };
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut ::core::ffi::c_void, 64, 0, 64) };
        if rc != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Verify signature failed, returned -0x%04x\0".as_ptr() as *const i8, b"PKCS7_VerifySignerSignature\0".as_ptr() as *const i8, 905, rc) };
            return rc;
        } else {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: Verify signer signature success\n\0".as_ptr() as *const i8, b"PKCS7_VerifySignerSignature\0".as_ptr() as *const i8, 908) };
        }
        signer = unsafe { (*signer).next } as *const crate::types::SignerInfo;
    }
    rc
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_43
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn LoadRootCert() -> i32 {
    let mut rc = 0;
    unsafe {
        if !g_rootCertLoaded {
            mbedtls_x509_crt_init(&mut g_rootCaG2Cert);
            rc = mbedtls_x509_crt_parse(
                &mut g_rootCaG2Cert,
                ROOT_CA_G2_CERT_IN_PEM.as_ptr(),
                ROOT_CA_G2_CERT_IN_PEM.len() as crate::types::size_t,
            );
            if rc != 0 {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const _,
                    b"LoadRootCert\0".as_ptr() as *const _,
                    922,
                );
                return rc;
            } else {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: load root ca success\0".as_ptr() as *const _,
                    b"LoadRootCert\0".as_ptr() as *const _,
                    925,
                );
            }
            g_rootCertLoaded = true;
        }
    }
    rc
}

fn UnLoadRootCert() {
    unsafe {
        if g_rootCertLoaded {
            mbedtls_x509_crt_free(&mut g_rootCaG2Cert);
            g_rootCertLoaded = false;
        }
    }
}

fn LoadDebugModeRootCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut crate::globals::g_debugModeRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt);
        let rc = crate::compat::mbedtls_x509_crt_parse(
            &mut crate::globals::g_debugModeRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt,
            std::ptr::null(),
            0 as crate::types::size_t,
        );
        if rc != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load debug mode root ca failed %d\0".as_ptr() as *const _,
                b"LoadDebugModeRootCert\0".as_ptr() as *const _,
                946,
                rc,
            );
            return rc;
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load debug mode root ca success\0".as_ptr() as *const _,
                b"LoadDebugModeRootCert\0".as_ptr() as *const _,
                949,
            );
        }
        rc
    }
}

fn UnLoadDebugModeRootCert() -> i32 {
    unsafe {
        mbedtls_x509_crt_free(&mut g_debugModeRootCert);
    }
    PKCS7_SUCC as i32
}

fn LoadSelfSignedCert()-> i32 {
    unsafe {
        mbedtls_x509_crt_init(&mut g_ohosRootCert);
        let rc = mbedtls_x509_crt_parse(
            &mut g_ohosRootCert,
            OHOS_ROOT_CERT_IN_PEM.as_ptr(),
            (OHOS_ROOT_CERT_IN_PEM.len()) as crate::types::size_t,
        );
        if rc != 0 {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load self signed ca failed %d\0".as_ptr() as *const _,
                b"LoadSelfSignedCert\0".as_ptr() as *const _,
                964,
                rc,
            );
            return rc;
        } else {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load self signed root ca success\0".as_ptr() as *const _,
                b"LoadSelfSignedCert\0".as_ptr() as *const _,
                967,
            );
        }
        rc
    }
}

fn UnLoadSelfSignedCert() {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(&mut crate::globals::g_ohosRootCert as *mut i32 as *mut crate::types::mbedtls_x509_crt);
    }
}

fn DLogCrtVerifyInfo(flags: u32) {
    let mut vrfyBuf: [::core::ffi::c_char; 512] = [0; 512];
    let _ = unsafe { crate::compat::memset_s(vrfyBuf.as_mut_ptr() as *mut ::core::ffi::c_void, 512, 0, 512) };
    unsafe { crate::compat::mbedtls_x509_crt_verify_info(vrfyBuf.as_mut_ptr() as *mut ::core::ffi::c_char, 512, b" ! \0".as_ptr() as *const ::core::ffi::c_char, flags) };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_DEBUG, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char, b"DLogCrtVerifyInfo\0".as_ptr() as *const ::core::ffi::c_char, 981, vrfyBuf.as_ptr() as *const ::core::ffi::c_char) };
}

fn IsRevoked(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    unsafe {
        let mut cur = &(*crl).entry as *const crate::types::mbedtls_x509_crl_entry as *mut crate::types::mbedtls_x509_crl_entry;
        while !cur.is_null() {
            if (*cur).serial.len == 0 {
                return crate::types::PKCS7_SUCC as i32;
            }
            if (*crt).serial.len != (*cur).serial.len {
                cur = (*cur).next;
                continue;
            }
            if libc::memcmp(
                (*crt).serial.p as *const libc::c_void,
                (*cur).serial.p as *const libc::c_void,
                (*cur).serial.len as usize,
            ) == 0
            {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            cur = (*cur).next;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn VerifyCrl(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    let mut crl_list = crl;
    while !crl_list.is_null() {
        unsafe {
            if (*crl_list).version == 0 ||
                crate::src_mbedtls_pkcs7::CompareX509NameList(&(*crl_list).issuer as *const _, &(*crt).issuer as *const _) != 0 {
                crl_list = (*crl_list).next;
                continue;
            }
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: find crl\0".as_ptr() as *const _, b"VerifyCrl\0".as_ptr() as *const _, 1012);
            if crate::src_mbedtls_pkcs7::IsRevoked(crt, crl_list) != 0 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            crl_list = (*crl_list).next;
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn VerifyClicert(clicert: *mut crate::types::mbedtls_x509_crt, rootCert: *mut crate::types::mbedtls_x509_crt, pkcs7: *const crate::types::Pkcs7) -> i32 {
    let mut flags: u32 = 0;
    let crl_ptr = unsafe { &(*pkcs7).signedData.crl } as *const crate::types::mbedtls_x509_crl as *mut crate::types::mbedtls_x509_crl;
    let rc = unsafe {
        crate::compat::mbedtls_x509_crt_verify(
            clicert,
            rootCert,
            crl_ptr,
            std::ptr::null_mut(),
            &mut flags as *mut u32,
            None,
            std::ptr::null_mut(),
        )
    };
    if rc != 0 {
        crate::src_mbedtls_pkcs7::DLogCrtVerifyInfo(flags);
    } else {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Verify signers cert chain root cert success\0".as_ptr() as *const i8,
                b"VerifyClicert\0".as_ptr() as *const i8,
                1029,
            );
        }
        if crate::src_mbedtls_pkcs7::VerifyCrl(clicert as *const crate::types::mbedtls_x509_crt, crl_ptr as *const crate::types::mbedtls_x509_crl) != crate::types::PKCS7_SUCC as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: cert crl verify failed\0".as_ptr() as *const i8,
                    b"VerifyClicert\0".as_ptr() as *const i8,
                    1031,
                );
            }
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    rc
}

pub extern "C" fn PKCS7_VerifyCertsChain(pkcs7: *const crate::types::Pkcs7) -> i32 {
    if pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let mut cnt: i32 = 0;
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers };
    while !signer.is_null() {
        let clicert: *mut crate::types::mbedtls_x509_crt = unsafe { (*signer).certPath.crt };
        if clicert.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: Signer has no certs\0".as_ptr() as *const i8,
                    "PKCS7_VerifyCertsChain\0".as_ptr() as *const i8,
                    1049,
                );
            }
            return crate::types::PKCS7_HAS_NO_SIGNER_CRT as i32;
        }
        let mut rc: i32;
        cnt += 1;
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: signer : %d\0".as_ptr() as *const i8,
                "PKCS7_VerifyCertsChain\0".as_ptr() as *const i8,
                1054,
                cnt,
            );
        }
        unsafe {
            if crate::globals::g_debugModeEnabled != 0 {
                rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                    clicert,
                    &crate::globals::g_debugModeRootCert as *const i32 as *mut crate::types::mbedtls_x509_crt,
                    pkcs7,
                );
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_DEBUG,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: Verify inner: %d\0".as_ptr() as *const i8,
                    "PKCS7_VerifyCertsChain\0".as_ptr() as *const i8,
                    1057,
                    rc,
                );
                if rc == crate::types::PKCS7_SUCC as i32 {
                    signer = (*signer).next;
                    continue;
                }
                if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                    return crate::types::PKCS7_IS_REVOKED as i32;
                }
            }
            rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                (*signer).rootCert,
                pkcs7,
            );
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: Verify : %d\0".as_ptr() as *const i8,
                "PKCS7_VerifyCertsChain\0".as_ptr() as *const i8,
                1067,
                rc,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next;
                continue;
            }
            if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                &crate::globals::g_ohosRootCert as *const i32 as *mut crate::types::mbedtls_x509_crt,
                pkcs7,
            );
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: Verify self : %d\0".as_ptr() as *const i8,
                "PKCS7_VerifyCertsChain\0".as_ptr() as *const i8,
                1077,
                rc,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next;
                continue;
            }
        }
        return rc;
    }
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetSignerSignningCertSubject(signer: *const crate::types::SignerInfo, subject: *mut ::core::ffi::c_char, subjectLen: crate::types::size_t) -> i32 {
    if signer.is_null() || subject.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let crt = unsafe { (*signer).certPath.crt };
    if crt.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let rc = unsafe { crate::compat::mbedtls_x509_dn_gets(subject, subjectLen, &(*crt).subject) };
    if rc < 0 {
        return rc;
    }
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetSignerSignningCertIssuer(signer: *const crate::types::SignerInfo, issuer: *mut ::core::ffi::c_char, issuerLen: crate::types::size_t) -> i32 {
    if signer.is_null() || issuer.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let crt = unsafe { (*signer).certPath.crt };
    if crt.is_null() {
        return 0;
    }
    let rc = unsafe { crate::compat::mbedtls_x509_dn_gets(issuer, issuerLen, &(*crt).issuer) };
    if rc < 0 {
        return rc;
    }
    crate::types::PKCS7_SUCC as i32
}

fn GetSignersCnt(signers: *const crate::types::SignerInfo) -> ::core::ffi::c_uint {
    let mut cnt: ::core::ffi::c_uint = 0;
    let mut current = signers;
    while !current.is_null() {
        cnt = cnt.wrapping_add(1);
        unsafe {
            current = (*current).next as *const crate::types::SignerInfo;
        }
    }
    cnt
}

fn IsIncludeRoot(signer: *const crate::types::SignerInfo) -> crate::types::c_bool {
    unsafe {
        if signer.is_null() {
            return 0;
        }
        let mut pre = (*signer).certPath.crt;
        let mut cur = pre;
        let mut i = 0;
        while i < (*signer).certPath.depth && !cur.is_null() {
            pre = cur;
            cur = (*cur).next;
            i += 1;
        }
        if pre.is_null() {
            return 0;
        }
        if crate::src_mbedtls_pkcs7::CompareX509NameList(&(*pre).issuer as *const _, &(*pre).subject as *const _) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: Include root cert\0".as_ptr() as *const _,
                b"__FUNCTION__\0".as_ptr() as *const _,
                1143,
            );
            return 1;
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: Not include root cert\0".as_ptr() as *const _,
            b"__FUNCTION__\0".as_ptr() as *const _,
            1146,
        );
        0
    }
}

fn GetSignerSignningCertDepth(signer: *const crate::types::SignerInfo) -> i32 {
    if unsafe { crate::src_mbedtls_pkcs7::IsIncludeRoot(signer) } != 0 {
        unsafe { (*signer).certPath.depth }
    } else {
        unsafe { (*signer).certPath.depth + 1 }
    }
}

pub extern "C" fn PKCS7_FreeAllSignersResolvedInfo(sri: *mut crate::types::SignersResovedInfo) {
    if sri.is_null() {
        return;
    }
    unsafe {
        if !(*sri).signers.is_null() {
            crate::src_mbedtls_pkcs7::Pkcs7Free((*sri).signers as *mut std::ffi::c_void);
            (*sri).signers = std::ptr::null_mut();
        }
        crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut std::ffi::c_void);
    }
}

pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    let mut sri: *mut crate::types::SignersResovedInfo = std::ptr::null_mut();
    if pkcs7.is_null() {
        return std::ptr::null_mut();
    }
    let signers_cnt = crate::src_mbedtls_pkcs7::GetSignersCnt(unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo);
    if signers_cnt == 0 {
        return std::ptr::null_mut();
    }
    sri = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignersResovedInfo>() as crate::types::size_t) as *mut crate::types::SignersResovedInfo;
    if sri.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*sri).nrOfSigners = signers_cnt as i32;
    }
    unsafe {
        (*sri).signers = crate::src_mbedtls_pkcs7::Pkcs7Calloc(signers_cnt as crate::types::size_t, std::mem::size_of::<crate::types::SignerResovledInfo>() as crate::types::size_t) as *mut crate::types::SignerResovledInfo;
    }
    if unsafe { (*sri).signers.is_null() } {
        crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut std::ffi::c_void);
        return std::ptr::null_mut();
    }
    let mut rc: i32;
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo;
    let mut idx: i32 = 0;
    while !signer.is_null() && idx < signers_cnt as i32 {
        rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertSubject(
            signer,
            unsafe { &mut (*(*sri).signers.offset(idx as isize)).subject as *mut _ as *mut ::core::ffi::c_char },
            std::mem::size_of::<[::core::ffi::c_char; 512usize]>() as crate::types::size_t,
        );
        if rc != 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return std::ptr::null_mut();
        }
        rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertIssuer(
            signer,
            unsafe { &mut (*(*sri).signers.offset(idx as isize)).issuer as *mut _ as *mut ::core::ffi::c_char },
            std::mem::size_of::<[::core::ffi::c_char; 512usize]>() as crate::types::size_t,
        );
        if rc != 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return std::ptr::null_mut();
        }
        unsafe {
            (*(*sri).signers.offset(idx as isize)).depth = crate::src_mbedtls_pkcs7::GetSignerSignningCertDepth(signer);
        }
        signer = unsafe { (*signer).next } as *const crate::types::SignerInfo;
        idx += 1;
    }
    return sri;
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_62
// c_function: PKCS7_GetDigestInSignerAuthAttr
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32
// c_first_line: int32_t PKCS7_GetDigestInSignerAuthAttr(const SignerInfo *signer, unsigned char **dig, size_t *digLen)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_62/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `MBEDTLS_OID_PKCS9_MSG_DIGEST` in this scope
//      --> src/src_mbedtls_pkcs7.rs:834:38
//       |
//       |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   error: cannot find macro `MBEDTLS_OID_PKCS9_MSG_DIGEST` in this scope
//      --> src/src_mbedtls_pkcs7.rs:833:21
//       |
//       |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_62
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_62/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    if signer.is_null() || dig.is_null() || digLen.is_null() {
        return PKCS7_INVALID_VALUE as i32;
    }
    let p = unsafe { (*signer).authAttr.p };
    if p.is_null() {
        return PKCS7_HAS_NO_AUTH_ATTR_IN_SIGNER as i32;
    }
    let end = unsafe { p.offset((*signer).authAttr.len as isize) };
    let mut p = p;
    while p < end {
        let mut seq_len: size_t = 0;
        let mut p_mut = p;
        let rc = unsafe {
            mbedtls_asn1_get_tag(
                &mut p_mut as *mut *mut ::core::ffi::c_uchar,
                end,
                &mut seq_len as *mut size_t,
                (MBEDTLS_ASN1_SEQUENCE | MBEDTLS_ASN1_CONSTRUCTED) as u8,
            )
        };
        if rc != 0 {
            return rc;
        }
        p = p_mut;
        let seq_end = unsafe { p.offset(seq_len as isize) };
        let mut oid_len: size_t = 0;
        let mut p_mut = p;
        let rc = unsafe {
            mbedtls_asn1_get_tag(
                &mut p_mut as *mut *mut ::core::ffi::c_uchar,
                seq_end,
                &mut oid_len as *mut size_t,
                MBEDTLS_ASN1_OID as u8,
            )
        };
        if rc != 0 {
            return rc;
        }
        p = p_mut;
        if oid_len == (MBEDTLS_OID_SIZE(MBEDTLS_OID_PKCS9_MSG_DIGEST!()) as size_t)
            && unsafe {
                libc::memcmp(
                    p as *const ::core::ffi::c_void,
                    MBEDTLS_OID_PKCS9_MSG_DIGEST!() as *const ::core::ffi::c_void,
                    MBEDTLS_OID_SIZE(MBEDTLS_OID_PKCS9_MSG_DIGEST!()) as usize,
                )
            } == 0
        {
            p = unsafe { p.offset(oid_len as isize) };
            let mut tmp_len: size_t = 0;
            let mut p_mut = p;
            let rc = unsafe {
                mbedtls_asn1_get_tag(
                    &mut p_mut as *mut *mut ::core::ffi::c_uchar,
                    seq_end,
                    &mut tmp_len as *mut size_t,
                    (MBEDTLS_ASN1_SET | MBEDTLS_ASN1_CONSTRUCTED) as u8,
                )
            };
            if rc != 0 {
                return rc;
            }
            p = p_mut;
            let mut p_mut = p;
            let rc = unsafe {
                mbedtls_asn1_get_tag(
                    &mut p_mut as *mut *mut ::core::ffi::c_uchar,
                    seq_end,
                    &mut tmp_len as *mut size_t,
                    MBEDTLS_ASN1_OCTET_STRING as u8,
                )
            };
            if rc != 0 {
                return rc;
            }
            p = p_mut;
            unsafe {
                *dig = p;
                *digLen = tmp_len;
            }
            return PKCS7_SUCC as i32;
        } else {
            p = seq_end;
        }
    }
    PKCS7_INVALID_VALUE as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_62
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn PKCS7_GetSignerAuthAttr(signer: *const crate::types::SignerInfo, data: *mut *mut ::core::ffi::c_uchar, dataLen: *mut crate::types::size_t) -> i32 {
    if signer.is_null() || data.is_null() || dataLen.is_null() {
        return crate::types::PKCS7_INVALID_VALUE as i32;
    }
    unsafe {
        if (*signer).authAttrRaw.p.is_null() {
            return crate::types::PKCS7_INVALID_VALUE as i32;
        }
        *dataLen = (*signer).authAttrRaw.len;
        *data = (*signer).authAttrRaw.p;
        *(*data) = 0x20 | 0x11;
    }
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetContentData(pkcs7: *const crate::types::Pkcs7, data: *mut *mut ::core::ffi::c_uchar, dataLen: *mut crate::types::size_t) -> i32 {
    if pkcs7.is_null() || data.is_null() || dataLen.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        let p = (*pkcs7).signedData.content.data.p;
        let len = (*pkcs7).signedData.content.data.len;
        let end = p.offset(len as isize);
        let mut octetLen: crate::types::size_t = 0;
        let mut current = p;
        let rc = crate::compat::mbedtls_asn1_get_tag(&mut current as *mut *mut ::core::ffi::c_uchar, end, &mut octetLen as *mut crate::types::size_t, crate::types::MBEDTLS_ASN1_OCTET_STRING as i32);
        if rc != 0 {
            return rc;
        }
        *data = current;
        *dataLen = octetLen;
        crate::types::PKCS7_SUCC as i32
    }
}

pub extern "C" fn PKCS7_EnableDebugMode(mode: bool) -> i32 {
    unsafe {
        if g_debugModeEnabled == mode {
            return PKCS7_SUCC as i32;
        }
        let rc = if mode {
            crate::src_mbedtls_pkcs7::LoadDebugModeRootCert()
        } else {
            crate::src_mbedtls_pkcs7::UnLoadDebugModeRootCert()
        };
        if rc != 0 {
            return rc;
        }
        g_debugModeEnabled = mode;
        PKCS7_SUCC as i32
    }
}

fn ParsePemFormatSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pem: *mut crate::types::mbedtls_pem_context, format: *mut ::core::ffi::c_char) -> i32 {
    use crate::compat::*;
    if bufLen != 0 && unsafe { !libc::strstr(buf as *const i8, b"-----BEGIN PKCS7-----\0".as_ptr() as *const i8).is_null() } {
        let mut useLen: crate::types::size_t = 0;
        unsafe {
            mbedtls_pem_init(pem);
        }
        let ret = unsafe {
            mbedtls_pem_read_buffer(
                pem,
                b"-----BEGIN PKCS7-----\0".as_ptr() as *const i8,
                b"-----END PKCS7-----\0".as_ptr() as *const i8,
                buf,
                std::ptr::null_mut(),
                0,
                &mut useLen as *mut crate::types::size_t,
            )
        };
        if ret == 0 && useLen == bufLen {
            unsafe {
                *format = 1;
            }
            return crate::types::PKCS7_SUCC as i32;
        }
        unsafe {
            mbedtls_pem_free(pem);
        }
    } else {
        unsafe {
            *format = 2;
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    crate::types::PKCS7_INVALID_PARAM as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_67
// c_function: PKCS7_ParseSignedData
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32
// c_first_line: int32_t PKCS7_ParseSignedData(const unsigned char *buf, size_t bufLen, Pkcs7 *pkcs7)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_67/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_mbedtls_pkcs7.rs:1969:41
//        |
//        |                                         ^^^ types differ in mutability
//        |
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 15 warnings emitted
// =================================
pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_67
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_67/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: crate::types::c_bool = 0;
    let mut start: *mut u8 = std::ptr::null_mut();
    let mut end: *const u8 = std::ptr::null();
    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        std::ptr::write_bytes(pkcs7 as *mut u8, 0, std::mem::size_of::<crate::types::Pkcs7>());
    }
    start = buf as *mut u8;
    let mut buf_len = bufLen;
    let mut format: ::core::ffi::c_char = 0;
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, buf_len, std::ptr::null_mut(), &mut format);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        unsafe {
            start = std::ptr::null_mut();
            buf_len = 0;
        }
    }
    unsafe {
        end = start.offset(buf_len as isize);
    }
    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    let mut p: *mut *mut ::core::ffi::c_uchar = &mut (start as *mut ::core::ffi::c_uchar);
    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(p, end as *const ::core::ffi::c_uchar, unsafe { &mut (*pkcs7).contentTypeOid }, &mut hasContent);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || hasContent == 0 {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }
    let mut p2: *mut *mut u8 = &mut start;
    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(p2, end, &mut len);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    unsafe {
        if start.offset(len as isize) > end {
            return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        }
    }
    rc = crate::src_mbedtls_pkcs7::ParseSignedData(start, len as usize, unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::ConstructSignerCerts(unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    return rc;
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_67
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn PKCS7_FreeRes(pkcs7: *mut crate::types::Pkcs7) {
    if pkcs7.is_null() {
        return;
    }
    crate::src_mbedtls_pkcs7::FreeSignedDataDigestAlgs(pkcs7);
    crate::src_mbedtls_pkcs7::FreeSignersInfo(pkcs7);
    crate::src_mbedtls_pkcs7::FreeSignedDataCerts(pkcs7);
    crate::src_mbedtls_pkcs7::FreeSignedDataCrl(pkcs7);
    crate::src_mbedtls_pkcs7::UnLoadRootCert();
    crate::src_mbedtls_pkcs7::UnLoadSelfSignedCert();
}
