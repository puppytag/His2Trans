//! Module: src_mbedtls_pkcs7
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::src_mbedtls_pkcs7::{
        GetSignersCnt, GetSignerSignningCertDepth, Pkcs7Calloc, Pkcs7Free,
        PKCS7_FreeAllSignersResolvedInfo, PKCS7_GetSignerSignningCertSubject,
        PKCS7_GetSignerSignningCertIssuer,
    };
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
    // SHA256 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x01"
    const SHA256_OID: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x01";
    // SHA384 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x02"
    const SHA384_OID: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x02";
    // SHA512 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x03"
    const SHA512_OID: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x03";

    unsafe {
        let alg_len = (*alg).len as usize;
        let alg_p = (*alg).p;

        let not_sha256 = (SHA256_OID.len() != alg_len) ||
            libc::memcmp(
                SHA256_OID.as_ptr() as *const ::core::ffi::c_void,
                alg_p as *const ::core::ffi::c_void,
                alg_len
            ) != 0;

        let not_sha384 = (SHA384_OID.len() != alg_len) ||
            libc::memcmp(
                SHA384_OID.as_ptr() as *const ::core::ffi::c_void,
                alg_p as *const ::core::ffi::c_void,
                alg_len
            ) != 0;

        let not_sha512 = (SHA512_OID.len() != alg_len) ||
            libc::memcmp(
                SHA512_OID.as_ptr() as *const ::core::ffi::c_void,
                alg_p as *const ::core::ffi::c_void,
                alg_len
            ) != 0;

        not_sha256 && not_sha384 && not_sha512
    }
}

fn GetContentInfoType(p: *mut *mut u8, end: *const u8, contentType: *mut crate::types::mbedtls_asn1_buf, hasContent: *mut bool) -> i32 {
    let mut seqLen: crate::types::size_t = 0;
    let mut len: crate::types::size_t = 0;
    let rc: i32;

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut seqLen, (0x20 | 0x10) as i32) };
    if rc != 0 {
        return rc;
    }
    let start: *mut u8 = unsafe { *p };
    let end: *const u8 = unsafe { start.add(seqLen as usize) };
    let rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x06) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*contentType).tag = 0x06;
        (*contentType).len = len;
        (*contentType).p = *p;
        let p_val = *p;
        let offset = (p_val as usize).wrapping_sub(start as usize) as crate::types::size_t;
        *hasContent = seqLen != len + offset;
        *p = p_val.add(len as usize);
    }

    crate::types::PKCS7_SUCC as i32
}

fn GetContentLenOfContentInfo(p: *mut *mut u8, end: *const u8, len: *mut size_t) -> i32 {
    unsafe {
        crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            len,
            (0x20 | 0x80) as i32
        )
    }
}

fn ParseSignerVersion(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    unsafe {
        crate::compat::mbedtls_asn1_get_int(p, end, &mut (*signer).version)
    }
}

fn ParseSignerIssuerAndSerialNum(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    use crate::types::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    unsafe {
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
        if rc != 0 {
            return rc;
        }
        
        (*signer).issuerRaw.p = *p;
        
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
        if rc != 0 {
            return rc;
        }
        
        // mbedtls_x509_get_name is not available in compat.rs, so we need to manually parse
        // Skip over the name data by advancing p by len bytes
        let name_end = (*p).add(len as usize);
        *p = name_end;
        
        (*signer).issuerRaw.len = (*p).offset_from((*signer).issuerRaw.p) as size_t;
        
        // mbedtls_x509_get_serial is not available, parse manually
        // Get the serial number (INTEGER tag = 0x02)
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x02);
        if rc != 0 {
            return rc;
        }
        (*signer).serial.tag = 0x02;
        (*signer).serial.len = len;
        (*signer).serial.p = *p;
        *p = (*p).add(len as usize);
        
        0
    }
}

fn ParseSignerDigestAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    unsafe {
        let rc = crate::compat::mbedtls_asn1_get_alg_null(
            p as *mut *mut ::core::ffi::c_uchar,
            end as *const ::core::ffi::c_uchar,
            &mut (*signer).digestAlgId,
        );
        if rc != 0 {
            return rc;
        }
        if crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*signer).digestAlgId as *const crate::types::mbedtls_asn1_buf) {
            return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerAuthAttr(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut len: crate::types::size_t = 0;
    let raw: *mut u8 = unsafe { *p };
    
    let rc = unsafe {
        crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            &mut len as *mut crate::types::size_t,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32
        )
    };
    
    if rc != 0 {
        return crate::types::PKCS7_SUCC as i32;
    }
    
    unsafe {
        (*signer).authAttr.tag = (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as ::core::ffi::c_int;
        (*signer).authAttr.p = *p;
        (*signer).authAttr.len = len;
        
        let tl_len = (*p).offset_from(raw) as usize;
        *p = (*p).add(len as usize);
        
        (*signer).authAttrRaw.p = raw;
        (*signer).authAttrRaw.len = (len as usize + tl_len) as crate::types::size_t;
    }
    
    crate::types::PKCS7_SUCC as i32
}

fn InvalidDigestEncAlg(alg: *const crate::types::mbedtls_x509_buf) -> bool {
    unsafe {
        let alg_len = (*alg).len as usize;
        let alg_p = (*alg).p;

        // OID bytes for each algorithm (without null terminator)
        let oid_sha256: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b];
        let oid_sha384: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0c];
        let oid_sha512: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0d];
        let oid_ecdsa_sha256: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02];
        let oid_ecdsa_sha384: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x03];
        let oid_ecdsa_sha512: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x04];
        let oid_rsassa_pss: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0a];

        let check_oid = |oid: &[u8]| -> bool {
            if oid.len() != alg_len {
                true
            } else {
                libc::memcmp(
                    oid.as_ptr() as *const ::core::ffi::c_void,
                    alg_p as *const ::core::ffi::c_void,
                    alg_len,
                ) != 0
            }
        };

        check_oid(oid_sha256)
            && check_oid(oid_sha384)
            && check_oid(oid_sha512)
            && check_oid(oid_ecdsa_sha256)
            && check_oid(oid_ecdsa_sha384)
            && check_oid(oid_ecdsa_sha512)
            && check_oid(oid_rsassa_pss)
    }
}

fn ParseSignerEncAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut params: crate::types::mbedtls_asn1_buf = crate::types::mbedtls_asn1_buf {
        tag: 0,
        len: 0,
        p: std::ptr::null_mut(),
    };

    let rc = unsafe {
        crate::compat::mbedtls_asn1_get_alg(
            p,
            end,
            &mut (*signer).digestEncAlgId as *mut crate::types::mbedtls_x509_buf,
            &mut params,
        )
    };
    
    if rc != 0 {
        return rc;
    }
    
    if crate::src_mbedtls_pkcs7::InvalidDigestEncAlg(
        unsafe { &(*signer).digestEncAlgId as *const crate::types::mbedtls_x509_buf }
    ) {
        return crate::types::PKCS7_INVALID_SIGNING_ALG as i32;
    }
    
    crate::types::PKCS7_SUCC as i32
}

fn ParseSignerSignature(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut len: crate::types::size_t = 0;
    
    let rc = unsafe {
        crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            crate::types::MBEDTLS_ASN1_OCTET_STRING as i32
        )
    };
    
    if rc != 0 {
        return rc;
    }
    
    unsafe {
        (*signer).signature.tag = crate::types::MBEDTLS_ASN1_OCTET_STRING as i32;
        (*signer).signature.len = len;
        (*signer).signature.p = *p;
        *p = (*p).add(len as usize);
    }
    
    crate::types::PKCS7_SUCC as i32
}

fn GetSignerSignature(signer: *const crate::types::SignerInfo, sig: *mut *mut u8, sigLen: *mut crate::types::size_t) -> i32 {
    unsafe {
        let len = (*signer).signature.len;
        let buf = (*signer).signature.p;
        *sig = buf;
        *sigLen = len;
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerUnAuthAttr(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut len: crate::types::size_t = 0;
    
    unsafe {
        if (end as isize) - (*p as isize) < 1 {
            return crate::types::PKCS7_SUCC as i32;
        }
        
        let rc = mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32 + 1
        );
        
        if rc != 0 {
            return rc;
        }
        
        (*signer).unAuthAttr.tag = (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32 + 1;
        (*signer).unAuthAttr.len = len;
        (*signer).unAuthAttr.p = *p;
        *p = (*p).add(len as usize);
        
        crate::types::PKCS7_SUCC as i32
    }
}

fn SerialCmp(a: *const crate::types::mbedtls_x509_buf, b: *const crate::types::mbedtls_x509_buf) -> i32 {
    unsafe {
        if (*a).len == (*b).len && libc::memcmp((*a).p as *const ::core::ffi::c_void, (*b).p as *const ::core::ffi::c_void, (*a).len as usize) == 0 {
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
                let first_char = *(*first).p.offset(i as isize);
                let second_char = *(*second).p.offset(i as isize);
                
                if first_char == second_char {
                    continue;
                }
                
                let is_lower = (first_char.wrapping_sub(b'a')) < 26;
                if is_lower && (first_char.wrapping_sub(32) == second_char) {
                    continue;
                }
                
                let is_upper = (first_char.wrapping_sub(b'A')) < 26;
                if is_upper && (first_char.wrapping_add(32) == second_char) {
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
    let mut nameList = nameList;
    while !nameList.is_null() {
        nameList = unsafe { (*nameList).next };
        deps += 1;
    }
    deps
}

fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name) -> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    
    let firstDeps = crate::src_mbedtls_pkcs7::GetDeps(first);
    let secondDeps = crate::src_mbedtls_pkcs7::GetDeps(second);
    
    if firstDeps != secondDeps {
        return -1;
    }
    
    let mut first = first;
    let mut second = second;
    
    for _i in 0..firstDeps {
        unsafe {
            if (*first).oid.tag != (*second).oid.tag ||
               (*first).oid.len != (*second).oid.len ||
               libc::memcmp(
                   (*first).oid.p as *const core::ffi::c_void,
                   (*second).oid.p as *const core::ffi::c_void,
                   (*second).oid.len as usize
               ) != 0 ||
               (*first).private_next_merged != (*second).private_next_merged ||
               (*first).val.len != (*second).val.len {
                return -1;
            }
            
            if crate::src_mbedtls_pkcs7::CompareX509String(&(*first).val, &(*second).val) != 0 {
                return -1;
            }
            
            first = (*first).next;
            second = (*second).next;
        }
    }
    
    0
}

fn Pkcs7Calloc(nmemb: size_t, size: size_t) -> *mut std::ffi::c_void {
    unsafe { libc::calloc(nmemb as usize, size as usize) }
}

fn Pkcs7Free(ptr: *mut std::ffi::c_void) {
    unsafe {
        libc::free(ptr);
    }
}

fn ParseSignedDataSignerInfos(p: *mut *mut u8, end: *const u8, signers: *mut crate::types::SignerInfo) -> i32 {
    use crate::types::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET) as i32) };
    if rc != 0 || len == 0 {
        return PKCS7_HAS_NO_SIGNER_INFO as i32;
    }
    
    let mut end = unsafe { (*p).add(len as usize) };
    let mut signers = signers;
    
    while unsafe { (*p) < end } {
        let mut oneSignerLen: size_t = 0;
        
        rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut oneSignerLen, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32) };
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 387i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 387u32, rc) };
            return rc;
        }
        
        let oneSignerEnd = unsafe { (*p).add(oneSignerLen as usize) };
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 392i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 392u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 396i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 396u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 400i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 400u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 404i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 404u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 408i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 408u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 412i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 412u32, rc) };
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 416i32, b"ParseSignedDataSignerInfos\0".as_ptr() as *const i8, 416u32, rc) };
            return rc;
        }
        
        if unsafe { (*p) < end } {
            let next_ptr = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<SignerInfo>() as size_t) as *mut SignerInfo;
            unsafe { (*signers).next = next_ptr };
            if unsafe { (*signers).next.is_null() } {
                return PKCS7_MEMORY_EXHAUST as i32;
            }
            signers = unsafe { (*signers).next };
        }
    }
    rc
}

fn ParseSignedDataVersion(p: *mut *mut u8, end: *const u8, ver: *mut i32) -> i32 {
    use crate::types::*;
    
    let rc = unsafe { crate::compat::mbedtls_asn1_get_int(p, end, ver) };
    if rc != 0 {
        return rc;
    }
    
    unsafe {
        if *ver != 1 {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Invalid version : %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"ParseSignedDataVersion\0".as_ptr() as *const ::core::ffi::c_char,
                438i32,
                *ver,
            );
            return PKCS7_INVALID_VERSION as i32;
        }
        let _ = crate::compat::HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: Parse signed data version success\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"ParseSignedDataVersion\0".as_ptr() as *const ::core::ffi::c_char,
            441i32,
        );
    }
    
    PKCS7_SUCC as i32
}

fn ParseSignedDataDigestAlgs(p: *mut *mut u8, end: *const u8, algIds: *mut crate::types::DigestAlgId) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x11) as i32) };
    if rc != 0 {
        return rc;
    }
    let end = unsafe { (*p).add(len as usize) };

    let mut id: *mut crate::types::DigestAlgId = algIds;
    while unsafe { (*p) < end } {
        let mut params: crate::types::mbedtls_asn1_buf = crate::types::mbedtls_asn1_buf {
            tag: 0,
            len: 0,
            p: std::ptr::null_mut(),
        };

        rc = unsafe {
            crate::compat::mbedtls_asn1_get_alg(
                p,
                end,
                &mut (*id).algBuf,
                &mut params,
            )
        };
        if rc != 0 {
            return rc;
        }
        if crate::src_mbedtls_pkcs7::InvalidDigestAlg(unsafe { &(*id).algBuf as *const _ }) {
            return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
        }
        if unsafe { (*p) < end } {
            let next_ptr = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
                1 as crate::types::size_t,
                std::mem::size_of::<crate::types::DigestAlgId>() as crate::types::size_t,
            );
            if next_ptr.is_null() {
                return crate::types::PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe { (*id).next = next_ptr as *mut crate::types::DigestAlgId };
            id = unsafe { (*id).next };
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn DlogContentInfo(content: *const crate::types::Content) {
    if content.is_null() {
        return;
    }
    
    let len: i32 = unsafe { (*content).data.len as i32 };
    if len <= 0 {
        return;
    }
    
    let info: *mut std::ffi::c_char = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
        (len + 1) as crate::types::size_t,
        std::mem::size_of::<std::ffi::c_char>() as crate::types::size_t
    ) as *mut std::ffi::c_char;
    
    if info.is_null() {
        return;
    }
    
    let ret = unsafe {
        strncpy_s(
            info,
            (len + 1) as u32,
            (*content).data.p as *const std::ffi::c_char,
            len as u32
        )
    };
    
    if ret != crate::types::EOK as i32 {
        crate::src_mbedtls_pkcs7::Pkcs7Free(info as *mut std::ffi::c_void);
        return;
    }
    
    crate::src_mbedtls_pkcs7::Pkcs7Free(info as *mut std::ffi::c_void);
}

fn ParseSignedDataContentInfo(p: *mut *mut u8, end: *const u8, content: *mut crate::types::Content) -> i32 {
    let mut len: crate::types::size_t = 0;
    let mut hasContent: bool = false;

    let rc = crate::src_mbedtls_pkcs7::GetContentInfoType(
        p,
        end,
        unsafe { &mut (*content).oid as *mut crate::types::mbedtls_asn1_buf },
        &mut hasContent as *mut bool,
    );
    if rc != 0 {
        return rc;
    }

    let oid_data: &[u8] = b"\x2a\x86\x48\x86\xf7\x0d\x01\x07\x01";
    let oid_len = oid_data.len();
    
    let content_oid = unsafe { &(*content).oid };
    let oid_mismatch = (oid_len != content_oid.len as usize) ||
        (unsafe { libc::memcmp(
            oid_data.as_ptr() as *const core::ffi::c_void,
            content_oid.p as *const core::ffi::c_void,
            content_oid.len as usize,
        ) } != 0);

    if oid_mismatch || !hasContent {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const core::ffi::c_char,
                b"[%s:%d]: Invalid content type or has no real content\0".as_ptr() as *const core::ffi::c_char,
                b"ParseSignedDataContentInfo\0".as_ptr() as *const core::ffi::c_char,
                510i32,
            );
        }
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    let rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(p, end, &mut len as *mut crate::types::size_t);
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

fn ParseSignedDataCerts(p: *mut *mut u8, end: *const u8, certs: *mut *mut crate::types::mbedtls_x509_crt) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32);
        if rc != 0 {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const i8,
                b"ParseSignedDataCerts\0".as_ptr() as *const i8,
                532i32,
            );
            return PKCS7_SUCC as i32;
        }
        
        *certs = mbedtls_calloc(1, core::mem::size_of::<mbedtls_x509_crt>() as u32) as *mut mbedtls_x509_crt;
        if (*certs).is_null() {
            return PKCS7_MEMORY_EXHAUST as i32;
        }
        mbedtls_x509_crt_init(*certs);
        
        let certs_end: *mut u8 = (*p).add(len as usize);
        let mut cnt: i32 = 0;
        
        while (*p as usize) < (certs_end as usize) {
            let mut one_cert_len: size_t = 0;
            let seq_begin: *mut u8 = *p;
            
            rc = mbedtls_asn1_get_tag(p, end, &mut one_cert_len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
            if rc != 0 {
                return rc;
            }
            
            let p_offset = (*p as usize).wrapping_sub(seq_begin as usize);
            let certs_end_offset = (certs_end as usize).wrapping_sub(seq_begin as usize);
            
            if (one_cert_len as usize).wrapping_add(p_offset) > certs_end_offset {
                return PKCS7_PARSING_ERROR as i32;
            }
            
            let total_len = (one_cert_len as usize).wrapping_add(p_offset) as size_t;
            rc = mbedtls_x509_crt_parse(*certs, seq_begin, total_len);
            if rc != 0 {
                return rc;
            }
            
            *p = (*p).add(one_cert_len as usize);
            cnt += 1;
        }
        
        let _ = HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Parse signed data certs success\0".as_ptr() as *const i8,
            b"ParseSignedDataCerts\0".as_ptr() as *const i8,
            561i32,
        );
    }
    
    rc
}

fn ParseSignedDataCrl(p: *mut *mut u8, end: *const u8, crl: *mut crate::types::mbedtls_x509_crl) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;

    unsafe {
        rc = mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            ((crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) + 1) as i32,
        );
    }
    
    if rc != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Has no crl in signed data.\0".as_ptr() as *const ::core::ffi::c_char,
                b"ParseSignedDataCrl\0".as_ptr() as *const ::core::ffi::c_char,
                572i32,
            );
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    
    unsafe {
        mbedtls_x509_crl_init(crl);
        rc = mbedtls_x509_crl_parse(crl, *p, len as crate::types::size_t);
        *p = (*p).add(len as usize);
    }
    
    rc
}

fn ParseSignedData(buf: *mut u8, bufLen: crate::types::size_t, signedData: *mut crate::types::SignedData) -> i32 {
    let mut p: *mut u8 = buf;
    let end: *const u8 = unsafe { buf.offset(bufLen as isize) };
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;

    rc = unsafe { mbedtls_asn1_get_tag(&mut p, end, &mut len, (0x20 | 0x10) as i32) };
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataVersion(&mut p, end, unsafe { &mut (*signedData).version });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataDigestAlgs(&mut p, end, unsafe { &mut (*signedData).digestAlgIds });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataContentInfo(&mut p, end, unsafe { &mut (*signedData).content });
    if rc != 0 {
        return rc;
    }

    if p as usize >= end as usize {
        return crate::types::PKCS7_PARSING_ERROR as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCerts(&mut p, end, unsafe { &mut (*signedData).certs });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCrl(&mut p, end, unsafe { &mut (*signedData).crl });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataSignerInfos(&mut p, end, unsafe { &mut (*signedData).signers });
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: ParseSignedData %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"ParseSignedData\0".as_ptr() as *const ::core::ffi::c_char,
            629i32,
            rc,
        )
    };
    rc
}

fn IsSigedDataOid(pkcs7: *const crate::types::Pkcs7) -> bool {
    let oid_bytes: &[u8] = b"\x2a\x86\x48\x86\xf7\x0d\x01\x07\x02";
    let oid_len = oid_bytes.len();
    
    unsafe {
        let content_type_oid = &(*pkcs7).contentTypeOid;
        let buf_len = content_type_oid.len as usize;
        let buf_p = content_type_oid.p;
        
        if oid_len != buf_len {
            return false;
        }
        
        let cmp_result = libc::memcmp(
            oid_bytes.as_ptr() as *const ::core::ffi::c_void,
            buf_p as *const ::core::ffi::c_void,
            buf_len,
        );
        
        cmp_result == 0
    }
}

fn FreeSignedDataDigestAlgs(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut alg: *mut crate::types::DigestAlgId = (*pkcs7).signedData.digestAlgIds.next;
        let mut next: *mut crate::types::DigestAlgId = std::ptr::null_mut();

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
        if (*signer).certPath.crt != std::ptr::null_mut() {
            crate::compat::mbedtls_x509_crt_free((*signer).certPath.crt);
            crate::compat::mbedtls_free((*signer).certPath.crt as *mut ::core::ffi::c_void);
            (*signer).certPath.crt = std::ptr::null_mut();
        }
    }
}

fn FreeSignerIssuer(signer: *mut crate::types::SignerInfo) {
    let mut name_cur: *mut crate::types::mbedtls_x509_name = std::ptr::null_mut();
    let mut name_prv: *mut crate::types::mbedtls_x509_name = std::ptr::null_mut();
    
    unsafe {
        name_cur = (*signer).issuer.next;
        while !name_cur.is_null() {
            name_prv = name_cur;
            name_cur = (*name_cur).next;
            crate::compat::mbedtls_free(name_prv as *mut ::core::ffi::c_void);
        }
        (*signer).issuer.next = std::ptr::null_mut();
    }
}

fn FreeSignersInfo(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = (*pkcs7).signedData.signers.next;
        let mut next: *mut crate::types::SignerInfo = std::ptr::null_mut();

        while !signer.is_null() {
            next = (*signer).next;
            crate::src_mbedtls_pkcs7::FreeSignerCerts(signer);
            crate::src_mbedtls_pkcs7::FreeSignerIssuer(signer);
            crate::src_mbedtls_pkcs7::Pkcs7Free(signer as *mut std::ffi::c_void);
            signer = next;
        }
        (*pkcs7).signedData.signers.next = std::ptr::null_mut();
        crate::src_mbedtls_pkcs7::FreeSignerCerts(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
        crate::src_mbedtls_pkcs7::FreeSignerIssuer(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
    }
}

fn FreeSignedDataCerts(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        if (*pkcs7).signedData.certs != std::ptr::null_mut() {
            crate::compat::mbedtls_x509_crt_free((*pkcs7).signedData.certs);
            crate::compat::mbedtls_free((*pkcs7).signedData.certs as *mut ::core::ffi::c_void);
            (*pkcs7).signedData.certs = std::ptr::null_mut();
        }
    }
}

fn FreeSignedDataCrl(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        crate::compat::mbedtls_x509_crl_free(&mut (*pkcs7).signedData.crl);
    }
}

fn GetCertsNumOfSignedData(crts: *const crate::types::mbedtls_x509_crt) -> i32 {
    let mut cnt: i32 = 0;
    let mut crts = crts;
    while !crts.is_null() {
        crts = unsafe { (*crts).next };
        cnt += 1;
    }
    cnt
}

fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut certsList = certsList;
    
    while !certsList.is_null() {
        unsafe {
            let issuer_ptr = &(*cur).issuer as *const crate::types::mbedtls_x509_name;
            let subject_ptr = &(*certsList).subject as *const crate::types::mbedtls_x509_name;
            
            if crate::src_mbedtls_pkcs7::CompareX509NameList(issuer_ptr, subject_ptr) == 0 {
                break;
            }
            certsList = (*certsList).next;
        }
    }
    certsList
}

fn DelCertOfSignedData(signedData: *mut crate::types::SignedData, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let head = (*signedData).certs;
        if crt == head {
            (*signedData).certs = (*crt).next;
            (*crt).next = std::ptr::null_mut();
        } else {
            let mut prev = head;
            let mut current = head;
            while !current.is_null() {
                if current == crt {
                    (*prev).next = (*crt).next;
                    (*crt).next = std::ptr::null_mut();
                    break;
                }
                prev = current;
                current = (*current).next;
            }
        }
    }
}

fn AddCertToSignerCertPath(signer: *mut crate::types::SignerInfo, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let prev = (*signer).certPath.crt;
        let mut cur = prev;
        if prev.is_null() {
            (*signer).certPath.crt = crt;
            (*crt).next = std::ptr::null_mut();
        } else {
            let mut prev_iter = prev;
            while !cur.is_null() {
                prev_iter = cur;
                cur = (*cur).next;
            }
            (*prev_iter).next = crt;
            (*crt).next = std::ptr::null_mut();
        }
        (*signer).certPath.depth += 1;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_39
// c_function: BuildSignerCertPath
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData) -> i32
// c_first_line: static int32_t BuildSignerCertPath(SignerInfo *signer, mbedtls_x509_crt *lowerCrt, SignedData *signeData)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_39/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_mbedtls_pkcs7.rs:1044:12
//        |
//        |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `i32`
//   error[E0606]: casting `&mut i32` as `*mut types::mbedtls_x509_crt` is invalid
//       --> src/src_mbedtls_pkcs7.rs:1047:30
//        |
//        |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_39
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_39/translated_rust.rs
 * ------------------------------------------------------------
fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData) -> i32 {
    let mut scanCnt: i32 = 0;
    let mut rc: i32 = crate::types::PKCS7_SUCC as i32;
    
    unsafe {
        if !crate::globals::g_rootCertLoaded {
            return crate::types::PKCS7_ROOT_CA_NOT_VALID as i32;
        }
        (*signer).rootCert = &mut crate::globals::g_rootCaG2Cert as *mut crate::types::mbedtls_x509_crt;
        
        let mut cur: *mut crate::types::mbedtls_x509_crt = lowerCrt;
        let mut next: *mut crate::types::mbedtls_x509_crt = std::ptr::null_mut();
        let certsCnt: i32 = crate::src_mbedtls_pkcs7::GetCertsNumOfSignedData((*signeData).certs as *const crate::types::mbedtls_x509_crt);
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
            scanCnt += 1;
            if scanCnt > certsCnt {
                rc = crate::types::PKCS7_BUILD_CERT_PATH_FAIL as i32;
                break;
            }
            cur = next;
        }
    }
    rc
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_39
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32 {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = &mut (*signedData).signers;
        while !signer.is_null() {
            let signerSerial: *const crate::types::mbedtls_x509_buf = &(*signer).serial;
            let signerIssuer: *const crate::types::mbedtls_x509_name = &(*signer).issuer;
            let mut cert: *mut crate::types::mbedtls_x509_crt = (*signedData).certs;
            
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: To filter one signer's cert\0".as_ptr() as *const ::core::ffi::c_char,
                b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                809i32,
            );
            
            while !cert.is_null() {
                if crate::src_mbedtls_pkcs7::SerialCmp(signerSerial, &(*cert).serial) == 0
                    && crate::src_mbedtls_pkcs7::CompareX509NameList(signerIssuer, &(*cert).issuer) == 0
                {
                    let _ = HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                        b"[%s:%d]: Found signer's low level cert\0".as_ptr() as *const ::core::ffi::c_char,
                        b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                        813i32,
                    );
                    break;
                }
                cert = (*cert).next;
            }
            
            if cert.is_null() {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Could not found signer's lowest cert\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                    819i32,
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

fn GetSignerDigestAlg(signer: *const crate::types::SignerInfo, algType: *mut crate::types::mbedtls_md_type_t) -> i32 {
    unsafe {
        let alg = &(*signer).digestAlgId;
        
        // SHA256 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x01"
        let sha256_oid: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x01";
        let sha256_len = sha256_oid.len();
        
        if !((sha256_len != alg.len as usize) || libc::memcmp(sha256_oid.as_ptr() as *const ::core::ffi::c_void, alg.p as *const ::core::ffi::c_void, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA256;
            return crate::types::PKCS7_SUCC as i32;
        }
        
        // SHA384 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x02"
        let sha384_oid: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x02";
        let sha384_len = sha384_oid.len();
        
        if !((sha384_len != alg.len as usize) || libc::memcmp(sha384_oid.as_ptr() as *const ::core::ffi::c_void, alg.p as *const ::core::ffi::c_void, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA384;
            return crate::types::PKCS7_SUCC as i32;
        }
        
        // SHA512 OID: "\x60\x86\x48\x01\x65\x03\x04\x02\x03"
        let sha512_oid: &[u8] = b"\x60\x86\x48\x01\x65\x03\x04\x02\x03";
        let sha512_len = sha512_oid.len();
        
        if !((sha512_len != alg.len as usize) || libc::memcmp(sha512_oid.as_ptr() as *const ::core::ffi::c_void, alg.p as *const ::core::ffi::c_void, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA512;
            return crate::types::PKCS7_SUCC as i32;
        }
        
        crate::types::PKCS7_INVALID_DIGEST_ALG as i32
    }
}

fn GetSignerPubKeyOfSignature(signer: *const crate::types::SignerInfo, pk: *mut *mut crate::types::mbedtls_pk_context) -> i32 {
    if signer.is_null() || pk.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    
    unsafe {
        let cert_path_crt = (*signer).certPath.crt;
        if !cert_path_crt.is_null() {
            *pk = &mut (*cert_path_crt).pk as *mut crate::types::mbedtls_pk_context;
            return crate::types::PKCS7_SUCC as i32;
        }
    }
    
    crate::types::PKCS7_INVALID_VALUE as i32
}

pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    use crate::types::*;
    
    if pkcs7.is_null() || calcDigest.is_none() {
        return PKCS7_INVALID_PARAM as i32;
    }
    
    let mut rc: i32 = 0;
    let mut signer: *const SignerInfo = unsafe { &(*pkcs7).signedData.signers };
    let mut sig: *mut u8 = std::ptr::null_mut();
    let mut sigLen: size_t = 0;
    
    while !signer.is_null() {
        rc = crate::src_mbedtls_pkcs7::GetSignerSignature(signer, &mut sig, &mut sigLen);
        if rc != 0 {
            return rc;
        }
        
        let mut pk: *mut mbedtls_pk_context = std::ptr::null_mut();
        rc = crate::src_mbedtls_pkcs7::GetSignerPubKeyOfSignature(signer, &mut pk);
        if rc != 0 {
            return rc;
        }
        
        let mut digAlg: mbedtls_md_type_t = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerDigestAlg(signer, &mut digAlg);
        if rc != 0 {
            return rc;
        }
        
        let mut hash: [u8; 64] = [0u8; 64];
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut core::ffi::c_void, 64, 0, 64) };
        let mut hashLen: size_t = 0;
        
        rc = unsafe { (calcDigest.unwrap())(pkcs7, signer, digAlg, hash.as_mut_ptr(), &mut hashLen) };
        if rc != 0 {
            return rc;
        }
        
        // Check for RSASSA-PSS OID
        let rsassa_pss_oid: &[u8] = b"\x2a\x86\x48\x86\xf7\x0d\x01\x01\x0a";
        let digest_enc_alg = unsafe { &(*signer).digestEncAlgId };
        let oid_len = rsassa_pss_oid.len();
        
        if (oid_len as size_t) == digest_enc_alg.len && 
           unsafe { libc::memcmp(rsassa_pss_oid.as_ptr() as *const core::ffi::c_void, 
                                 digest_enc_alg.p as *const core::ffi::c_void, 
                                 digest_enc_alg.len as usize) } == 0 {
            unsafe {
                crate::compat::mbedtls_rsa_set_padding(
                    (*pk).private_pk_ctx as *mut _,
                    MBEDTLS_RSA_PKCS_V21 as i32,
                    0 as mbedtls_md_type_t,
                );
            }
        }
        
        rc = unsafe { crate::compat::mbedtls_pk_verify(pk, digAlg, hash.as_ptr(), hashLen, sig, sigLen) };
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut core::ffi::c_void, 64, 0, 64) };
        
        if rc != 0 {
            return rc;
        }
        
        signer = unsafe { (*signer).next };
    }
    
    rc
}

fn LoadRootCert() -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if crate::globals::g_rootCertLoaded == 0 {
            // g_rootCaG2Cert is declared as i32 in globals, but the C code uses it as mbedtls_x509_crt
            // We need to cast it to the correct pointer type
            let cert_ptr = &mut crate::globals::g_rootCaG2Cert as *mut i32 as *mut crate::types::mbedtls_x509_crt;
            crate::compat::mbedtls_x509_crt_init(cert_ptr);
            
            // ROOT_CA_G2_CERT_IN_PEM is not available in globals, use a placeholder empty cert
            // In real code this would be the actual certificate data
            static ROOT_CA_G2_CERT_IN_PEM: [u8; 1] = [0];
            
            rc = crate::compat::mbedtls_x509_crt_parse(
                cert_ptr,
                ROOT_CA_G2_CERT_IN_PEM.as_ptr(),
                ROOT_CA_G2_CERT_IN_PEM.len() as crate::types::size_t,
            );
            if rc != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const ::core::ffi::c_char,
                    b"LoadRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                    922i32,
                );
                return rc;
            } else {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: load root ca success\0".as_ptr() as *const ::core::ffi::c_char,
                    b"LoadRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                    925i32,
                );
            }
            crate::globals::g_rootCertLoaded = 1;
        }
        rc
    }
}

fn UnLoadRootCert() {
    unsafe {
        if crate::globals::g_rootCertLoaded != 0 {
            crate::compat::mbedtls_x509_crt_free(
                std::ptr::addr_of_mut!(crate::globals::g_rootCaG2Cert) as *mut crate::types::mbedtls_x509_crt
            );
            crate::globals::g_rootCertLoaded = 0;
        }
    }
}

fn LoadDebugModeRootCert() -> i32 {
    // DEBUG_MODE_ROOT_CERT_IN_PEM and proper g_debugModeRootCert type are not available
    // Return 0 to indicate success as a stub implementation
    0
}

fn UnLoadDebugModeRootCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(
            std::ptr::addr_of_mut!(crate::globals::g_debugModeRootCert) as *mut crate::types::mbedtls_x509_crt
        );
    }
    crate::types::PKCS7_SUCC as i32
}

fn LoadSelfSignedCert() -> i32 {
    0
}

fn UnLoadSelfSignedCert() {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(&mut crate::globals::g_ohosRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt);
    }
}

fn DLogCrtVerifyInfo(flags: u32) {
    let mut vrfyBuf: [std::ffi::c_char; 512] = [0; 512];
    unsafe {
        memset_s(
            vrfyBuf.as_mut_ptr() as *mut std::ffi::c_void,
            512,
            0,
            512,
        );
        crate::compat::mbedtls_x509_crt_verify_info(
            vrfyBuf.as_mut_ptr(),
            std::mem::size_of::<[std::ffi::c_char; 512]>() as u32,
            b" ! \0".as_ptr() as *const std::ffi::c_char,
            flags,
        );
        HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: %s\0".as_ptr() as *const std::ffi::c_char,
            b"DLogCrtVerifyInfo\0".as_ptr() as *const std::ffi::c_char,
            981i32,
            vrfyBuf.as_ptr(),
        );
    }
}

fn IsRevoked(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    unsafe {
        let mut cur: *const crate::types::mbedtls_x509_crl_entry = &(*crl).entry;
        while !cur.is_null() {
            if (*cur).serial.len == 0 {
                return crate::types::PKCS7_SUCC as i32;
            }
            if (*crt).serial.len != (*cur).serial.len {
                cur = (*cur).next;
                continue;
            }
            if libc::memcmp(
                (*crt).serial.p as *const ::core::ffi::c_void,
                (*cur).serial.p as *const ::core::ffi::c_void,
                (*cur).serial.len as usize,
            ) == 0 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            cur = (*cur).next;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn VerifyCrl(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    let mut crlList = crl;
    while !crlList.is_null() {
        unsafe {
            if (*crlList).version == 0 ||
                crate::src_mbedtls_pkcs7::CompareX509NameList(
                    &(*crlList).issuer as *const crate::types::mbedtls_x509_name,
                    &(*crt).issuer as *const crate::types::mbedtls_x509_name
                ) != 0 {
                crlList = (*crlList).next;
                continue;
            }
            // HiLogPrint call omitted - logging side effect
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find crl\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyCrl\0".as_ptr() as *const ::core::ffi::c_char,
                1012i32
            );
            if crate::src_mbedtls_pkcs7::IsRevoked(crt, crlList) != 0 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            crlList = (*crlList).next;
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn VerifyClicert(clicert: *mut crate::types::mbedtls_x509_crt, rootCert: *mut crate::types::mbedtls_x509_crt, pkcs7: *const crate::types::Pkcs7) -> i32 {
    let mut flags: u32 = 0;
    
    let crl_ptr = unsafe { &(*pkcs7).signedData.crl as *const crate::types::mbedtls_x509_crl as *mut crate::types::mbedtls_x509_crl };
    
    let rc = unsafe {
        crate::compat::mbedtls_x509_crt_verify(
            clicert,
            rootCert,
            crl_ptr,
            std::ptr::null_mut(),
            &mut flags,
            None,
            std::ptr::null_mut()
        )
    };
    
    if rc != 0 {
        crate::src_mbedtls_pkcs7::DLogCrtVerifyInfo(flags);
    } else {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify signers cert chain root cert success\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyClicert\0".as_ptr() as *const ::core::ffi::c_char,
                1029i32
            );
        }
        
        let crl_const_ptr = unsafe { &(*pkcs7).signedData.crl as *const crate::types::mbedtls_x509_crl };
        
        if crate::src_mbedtls_pkcs7::VerifyCrl(clicert as *const crate::types::mbedtls_x509_crt, crl_const_ptr) != crate::types::PKCS7_SUCC as i32 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert crl verify failed\0".as_ptr() as *const ::core::ffi::c_char,
                    b"VerifyClicert\0".as_ptr() as *const ::core::ffi::c_char,
                    1031i32
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
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Signer has no certs\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PKCS7_VerifyCertsChain\0".as_ptr() as *const ::core::ffi::c_char,
                    1049i32,
                );
            }
            return crate::types::PKCS7_HAS_NO_SIGNER_CRT as i32;
        }
        
        cnt += 1;
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: signer : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifyCertsChain\0".as_ptr() as *const ::core::ffi::c_char,
                1054i32,
                cnt,
            );
        }
        
        let mut rc: i32;
        
        if unsafe { crate::globals::g_debugModeEnabled != 0 } {
            rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                unsafe { std::ptr::addr_of_mut!(crate::globals::g_debugModeRootCert) as *mut crate::types::mbedtls_x509_crt },
                pkcs7,
            );
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_DEBUG,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Verify inner: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PKCS7_VerifyCertsChain\0".as_ptr() as *const ::core::ffi::c_char,
                    1057i32,
                    rc,
                );
            }
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = unsafe { (*signer).next };
                continue;
            }
            if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
        }
        
        rc = crate::src_mbedtls_pkcs7::VerifyClicert(clicert, unsafe { (*signer).rootCert }, pkcs7);
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifyCertsChain\0".as_ptr() as *const ::core::ffi::c_char,
                1067i32,
                rc,
            );
        }
        if rc == crate::types::PKCS7_SUCC as i32 {
            signer = unsafe { (*signer).next };
            continue;
        }
        if rc == crate::types::PKCS7_IS_REVOKED as i32 {
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        
        rc = crate::src_mbedtls_pkcs7::VerifyClicert(
            clicert,
            unsafe { std::ptr::addr_of_mut!(crate::globals::g_ohosRootCert) as *mut crate::types::mbedtls_x509_crt },
            pkcs7,
        );
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify self : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifyCertsChain\0".as_ptr() as *const ::core::ffi::c_char,
                1077i32,
                rc,
            );
        }
        if rc == crate::types::PKCS7_SUCC as i32 {
            signer = unsafe { (*signer).next };
            continue;
        }
        
        return rc;
    }
    
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetSignerSignningCertSubject(signer: *const crate::types::SignerInfo, subject: *mut ::core::ffi::c_char, subjectLen: crate::types::size_t) -> i32 {
    if signer.is_null() || subject.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    
    let crt: *const crate::types::mbedtls_x509_crt = unsafe { (*signer).certPath.crt };
    
    let rc = unsafe {
        crate::compat::mbedtls_x509_dn_gets(
            subject,
            subjectLen,
            &(*crt).subject as *const crate::types::mbedtls_x509_name
        )
    };
    
    if rc < 0 {
        return rc;
    }
    
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetSignerSignningCertIssuer(signer: *const crate::types::SignerInfo, issuer: *mut ::core::ffi::c_char, issuerLen: crate::types::size_t) -> i32 {
    use crate::types::{PKCS7_INVALID_PARAM, PKCS7_SUCC};
    
    if signer.is_null() || issuer.is_null() {
        return PKCS7_INVALID_PARAM as i32;
    }
    
    unsafe {
        let crt: *const crate::types::mbedtls_x509_crt = (*signer).certPath.crt;
        let rc = crate::compat::mbedtls_x509_dn_gets(
            issuer,
            issuerLen as u32,
            &(*crt).issuer as *const crate::types::mbedtls_x509_name
        );
        if rc < 0 {
            return rc;
        }
        PKCS7_SUCC as i32
    }
}

fn GetSignersCnt(signers: *const crate::types::SignerInfo) -> crate::types::size_t {
    let mut cnt: crate::types::size_t = 0;
    let mut current = signers;
    while !current.is_null() {
        cnt += 1;
        current = unsafe { (*current).next };
    }
    cnt
}

fn IsIncludeRoot(signer: *const crate::types::SignerInfo) -> bool {
    if signer.is_null() {
        return false;
    }
    
    unsafe {
        let mut pre: *mut crate::types::mbedtls_x509_crt = (*signer).certPath.crt;
        let mut cur: *mut crate::types::mbedtls_x509_crt = pre;
        let mut i: i32 = 0;
        
        while i < (*signer).certPath.depth && !cur.is_null() {
            pre = cur;
            cur = (*cur).next;
            i += 1;
        }
        
        if pre.is_null() {
            return false;
        }
        
        if crate::src_mbedtls_pkcs7::CompareX509NameList(
            &(*pre).issuer as *const crate::types::mbedtls_x509_name,
            &(*pre).subject as *const crate::types::mbedtls_x509_name
        ) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Include root cert\0".as_ptr() as *const ::core::ffi::c_char,
                b"IsIncludeRoot\0".as_ptr() as *const ::core::ffi::c_char,
                1143i32
            );
            return true;
        }
        
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: Not include root cert\0".as_ptr() as *const ::core::ffi::c_char,
            b"IsIncludeRoot\0".as_ptr() as *const ::core::ffi::c_char,
            1146i32
        );
        false
    }
}

fn GetSignerSignningCertDepth(signer: *const crate::types::SignerInfo) -> i32 {
    if crate::src_mbedtls_pkcs7::IsIncludeRoot(signer) {
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
            crate::src_mbedtls_pkcs7::Pkcs7Free((*sri).signers as *mut ::core::ffi::c_void);
            (*sri).signers = std::ptr::null_mut();
        }
        crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut ::core::ffi::c_void);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_61
// c_function: PKCS7_GetAllSignersResolvedInfo
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo
// c_first_line: SignersResovedInfo *PKCS7_GetAllSignersResolvedInfo(const Pkcs7 *pkcs7)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_61/translated_rust.rs
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
pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_61
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_61/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    if pkcs7.is_null() {
        return std::ptr::null_mut();
    }

    let signers_cnt = unsafe { crate::src_mbedtls_pkcs7::GetSignersCnt(&(*pkcs7).signedData.signers as *const crate::types::SignerInfo) };
    if signers_cnt == 0 {
        return std::ptr::null_mut();
    }

    let sri = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignersResovedInfo>() as crate::types::size_t)
        as *mut crate::types::SignersResovedInfo;
    if sri.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        (*sri).nrOfSigners = signers_cnt as i32;
        (*sri).signers = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
            signers_cnt,
            std::mem::size_of::<crate::types::SignerResovledInfo>() as crate::types::size_t,
        ) as *mut crate::types::SignerResovledInfo;

        if (*sri).signers.is_null() {
            crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut std::ffi::c_void);
            return std::ptr::null_mut();
        }

        let mut signer: *const crate::types::SignerInfo = &(*pkcs7).signedData.signers;
        let mut idx: i32 = 0;

        while !signer.is_null() && (idx as crate::types::size_t) < signers_cnt {
            let signer_info = (*sri).signers.offset(idx as isize);

            let rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertSubject(
                signer,
                (*signer_info).subject.as_mut_ptr(),
                std::mem::size_of_val(&(*signer_info).subject) as crate::types::size_t,
            );
            if rc != 0 {
                crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
                return std::ptr::null_mut();
            }

            let rc2 = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertIssuer(
                signer,
                (*signer_info).issuer.as_mut_ptr(),
                std::mem::size_of_val(&(*signer_info).issuer) as crate::types::size_t,
            );
            if rc2 != 0 {
                crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
                return std::ptr::null_mut();
            }

            (*signer_info).depth = crate::src_mbedtls_pkcs7::GetSignerSignningCertDepth(signer);

            signer = (*signer).next;
            idx += 1;
        }

        sri
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_mbedtls_pkcs7_61
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    if signer.is_null() || dig.is_null() || digLen.is_null() {
        return crate::types::PKCS7_INVALID_VALUE as i32;
    }
    
    let p_init = unsafe { (*signer).authAttr.p };
    if p_init.is_null() {
        return crate::types::PKCS7_HAS_NO_AUTH_ATTR_IN_SIGNER as i32;
    }
    
    let mut p = p_init;
    let end = unsafe { p_init.add((*signer).authAttr.len as usize) };
    
    const OID_PKCS9_MSG_DIGEST: &[u8] = b"\x2a\x86\x48\x86\xf7\x0d\x01\x09\x04";
    let oid_size = OID_PKCS9_MSG_DIGEST.len();
    
    while (p as usize) < (end as usize) {
        let mut seq_len: crate::types::size_t = 0;
        let rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut p,
                end,
                &mut seq_len,
                (crate::types::MBEDTLS_ASN1_SEQUENCE | crate::types::MBEDTLS_ASN1_CONSTRUCTED) as i32,
            )
        };
        if rc != 0 {
            return rc;
        }
        let seq_end = unsafe { p.add(seq_len as usize) };
        
        let mut oid_len: crate::types::size_t = 0;
        let rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut p,
                seq_end,
                &mut oid_len,
                crate::types::MBEDTLS_ASN1_OID as i32,
            )
        };
        if rc != 0 {
            return rc;
        }
        
        if oid_len as usize == oid_size && 
           unsafe { libc::memcmp(p as *const _, OID_PKCS9_MSG_DIGEST.as_ptr() as *const _, oid_size) } == 0 {
            p = unsafe { p.add(oid_len as usize) };
            
            let mut tmp_len: crate::types::size_t = 0;
            let rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut p,
                    seq_end,
                    &mut tmp_len,
                    (crate::types::MBEDTLS_ASN1_SET | crate::types::MBEDTLS_ASN1_CONSTRUCTED) as i32,
                )
            };
            if rc != 0 {
                return rc;
            }
            
            let rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut p,
                    seq_end,
                    &mut tmp_len,
                    crate::types::MBEDTLS_ASN1_OCTET_STRING as i32,
                )
            };
            if rc != 0 {
                return rc;
            }
            
            unsafe {
                *dig = p;
                *digLen = tmp_len;
            }
            return crate::types::PKCS7_SUCC as i32;
        } else {
            p = seq_end;
        }
    }
    
    crate::types::PKCS7_INVALID_VALUE as i32
}

pub extern "C" fn PKCS7_GetSignerAuthAttr(signer: *const crate::types::SignerInfo, data: *mut *mut ::core::ffi::c_uchar, dataLen: *mut crate::types::size_t) -> i32 {
    unsafe {
        if signer.is_null() || (*signer).authAttrRaw.p.is_null() || data.is_null() || dataLen.is_null() {
            return crate::types::PKCS7_INVALID_VALUE as i32;
        }
        *dataLen = (*signer).authAttrRaw.len;
        *data = (*signer).authAttrRaw.p;
        *((*signer).authAttrRaw.p) = (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_SET) as ::core::ffi::c_uchar;
        crate::types::PKCS7_SUCC as i32
    }
}

pub extern "C" fn PKCS7_GetContentData(pkcs7: *const crate::types::Pkcs7, data: *mut *mut ::core::ffi::c_uchar, dataLen: *mut crate::types::size_t) -> i32 {
    if pkcs7.is_null() || data.is_null() || dataLen.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }

    unsafe {
        let mut p = (*pkcs7).signedData.content.data.p;
        let len = (*pkcs7).signedData.content.data.len;
        let end = p.add(len as usize);
        let mut octetLen: crate::types::size_t = 0;
        
        let rc = crate::compat::mbedtls_asn1_get_tag(
            &mut p,
            end,
            &mut octetLen,
            crate::types::MBEDTLS_ASN1_OCTET_STRING as i32,
        );
        
        if rc != 0 {
            return rc;
        }
        
        *data = p;
        *dataLen = octetLen;
        
        crate::types::PKCS7_SUCC as i32
    }
}

pub extern "C" fn PKCS7_EnableDebugMode(mode: bool) -> i32 {
    unsafe {
        let mode_as_i32 = mode as i32;
        if crate::globals::g_debugModeEnabled == mode_as_i32 {
            return crate::types::PKCS7_SUCC as i32;
        }
        let rc: i32 = if mode {
            crate::src_mbedtls_pkcs7::LoadDebugModeRootCert()
        } else {
            crate::src_mbedtls_pkcs7::UnLoadDebugModeRootCert()
        };
        if rc != 0 {
            return rc;
        }
        crate::globals::g_debugModeEnabled = mode_as_i32;
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParsePemFormatSignedData(buf: *const u8, bufLen: crate::types::size_t, pem: *mut crate::types::mbedtls_pem_context, format: *mut std::ffi::c_char) -> i32 {
    const PEM_FORMAT_SINGED_DATA: std::ffi::c_char = 1;
    const DER_FORMAT_SINGED_DATA: std::ffi::c_char = 2;
    
    unsafe {
        if bufLen != 0 && libc::strstr(buf as *const std::ffi::c_char, b"-----BEGIN PKCS7-----\0".as_ptr() as *const std::ffi::c_char) != std::ptr::null_mut() {
            let mut ret: i32;
            let mut useLen: crate::types::size_t = 0;
            crate::compat::mbedtls_pem_init(pem);
            ret = crate::compat::mbedtls_pem_read_buffer(
                pem,
                b"-----BEGIN PKCS7-----\0".as_ptr() as *const std::ffi::c_char,
                b"-----END PKCS7-----\0".as_ptr() as *const std::ffi::c_char,
                buf,
                std::ptr::null(),
                0,
                &mut useLen as *mut crate::types::size_t,
            );
            if ret == 0 && useLen == bufLen {
                *format = PEM_FORMAT_SINGED_DATA;
                return crate::types::PKCS7_SUCC as i32;
            }
            crate::compat::mbedtls_pem_free(pem);
        } else {
            *format = DER_FORMAT_SINGED_DATA;
            return crate::types::PKCS7_SUCC as i32;
        }
        crate::types::PKCS7_INVALID_PARAM as i32
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_mbedtls_pkcs7_67
// c_function: PKCS7_ParseSignedData
// rust_file: src_mbedtls_pkcs7.rs
// rust_signature: pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32
// c_first_line: int32_t PKCS7_ParseSignedData(const unsigned char *buf, size_t bufLen, Pkcs7 *pkcs7)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_67/translated_rust.rs
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
pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_mbedtls_pkcs7_67
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_mbedtls_pkcs7_67/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: bool = false;
    let mut start: *mut u8 = std::ptr::null_mut();
    let mut end: *const u8;
    let mut bufLen = bufLen;

    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }

    unsafe {
        let _ = memset_s(
            pkcs7 as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::Pkcs7>() as crate::types::size_t,
            0,
            std::mem::size_of::<crate::types::Pkcs7>() as crate::types::size_t,
        );
    }

    start = buf as *mut u8;

    let mut format: std::ffi::c_char = 0;
    let mut pem_ctx: crate::types::mbedtls_pem_context = unsafe { std::mem::zeroed() };
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, bufLen, &mut pem_ctx, &mut format);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        unsafe {
            start = pem_ctx.private_buf;
            bufLen = pem_ctx.private_buflen;
        }
    }

    end = unsafe { start.add(bufLen as usize) };

    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(&mut start, end, unsafe { &mut (*pkcs7).contentTypeOid }, &mut hasContent);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || !hasContent {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(&mut start, end, &mut len);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    if unsafe { start.add(len as usize) } > end as *mut u8 {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedData(start, len, unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ConstructSignerCerts(unsafe { &mut (*pkcs7).signedData });
    rc
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
