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
// File-scope `static` variables from the original C TU.
// Now provided by native/globals.c for correct linkage,
// except g_debugModeEnabled and g_rootCertLoaded which are simple bools.
extern "C" {
    /// C: static const unsigned char[821] DEBUG_MODE_ROOT_CERT_IN_PEM
    static mut DEBUG_MODE_ROOT_CERT_IN_PEM: [::core::ffi::c_uchar; 821usize];
    /// C: static const unsigned char[863] OHOS_ROOT_CERT_IN_PEM
    static mut OHOS_ROOT_CERT_IN_PEM: [::core::ffi::c_uchar; 863usize];
    /// C: static const unsigned char[805] ROOT_CA_G2_CERT_IN_PEM
    static mut ROOT_CA_G2_CERT_IN_PEM: [::core::ffi::c_uchar; 805usize];
    /// C: static mbedtls_x509_crt g_debugModeRootCert
    static mut g_debugModeRootCert: crate::types::mbedtls_x509_crt;
    /// C: static mbedtls_x509_crt g_ohosRootCert
    static mut g_ohosRootCert: crate::types::mbedtls_x509_crt;
    /// C: static mbedtls_x509_crt g_rootCaG2Cert
    static mut g_rootCaG2Cert: crate::types::mbedtls_x509_crt;
}

/// C: static _Bool g_debugModeEnabled
static mut g_debugModeEnabled: bool = false;

/// C: static _Bool g_rootCertLoaded
static mut g_rootCertLoaded: bool = false;

// === C2R_FILE_STATICS_END ===

fn InvalidDigestAlg(alg: *const crate::types::mbedtls_asn1_buf)-> bool {
    let alg_ref = unsafe { &*alg };
    let len = alg_ref.len as usize;
    let p = alg_ref.p as *const u8;
    let sha256_expected_len = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256.len() - 1;
    let sha384_expected_len = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384.len() - 1;
    let sha512_expected_len = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512.len() - 1;
    let sha256_match = len == sha256_expected_len && {
        let expected = &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256[..sha256_expected_len];
        let actual = unsafe { core::slice::from_raw_parts(p, len) };
        actual == expected
    };
    let sha384_match = len == sha384_expected_len && {
        let expected = &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384[..sha384_expected_len];
        let actual = unsafe { core::slice::from_raw_parts(p, len) };
        actual == expected
    };
    let sha512_match = len == sha512_expected_len && {
        let expected = &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512[..sha512_expected_len];
        let actual = unsafe { core::slice::from_raw_parts(p, len) };
        actual == expected
    };
    !sha256_match && !sha384_match && !sha512_match
}

fn GetContentInfoType(p: *mut *mut u8, end: *const u8, contentType: *mut crate::types::mbedtls_asn1_buf, hasContent: *mut bool) -> i32 {
    // Narrow unsafe to initial raw pointer deref and slice construction.
    let (start, slice) = unsafe {
        let start = *p;
        if (end as usize) < (start as usize) {
            return -1;
        }
        let slice_len = (end as usize) - (start as usize);
        let slice = std::slice::from_raw_parts(start as *const u8, slice_len);
        (start, slice)
    };
    let mut idx: usize = 0;
    let mut seq_len: crate::types::size_t = 0;
    let mut oid_len: crate::types::size_t = 0;

    // ---- first ASN.1 get tag: expect SEQUENCE (0x30) ----
    if idx + 1 > slice.len() { return -1; }
    if slice[idx] != 0x30 { return -1; }
    idx += 1;

    // read SEQUENCE length
    if idx + 1 > slice.len() { return -1; }
    let first = slice[idx];
    if first & 0x80 == 0 {
        seq_len = first as crate::types::size_t;
        idx += 1;
    } else {
        let cnt = (first & 0x7F) as isize;
        if cnt == 0 || cnt > 2 { return -1; }
        if idx + 1 + (cnt as usize) > slice.len() { return -1; }
        idx += 1;
        seq_len = 0;
        for _ in 0..cnt {
            seq_len = (seq_len << 8) | (slice[idx] as crate::types::size_t);
            idx += 1;
        }
    }

    let inner_start = idx;
    let inner_end = inner_start + seq_len as usize;
    if inner_end > slice.len() { return -1; }

    // ---- second ASN.1 get tag: expect OID (0x06) ----
    if idx + 1 > inner_end { return -1; }
    if slice[idx] != 0x06 { return -1; }
    idx += 1;

    // read OID length
    if idx + 1 > inner_end { return -1; }
    let first = slice[idx];
    if first & 0x80 == 0 {
        oid_len = first as crate::types::size_t;
        idx += 1;
    } else {
        let cnt = (first & 0x7F) as isize;
        if cnt == 0 || cnt > 2 { return -1; }
        if idx + 1 + (cnt as usize) > inner_end { return -1; }
        idx += 1;
        oid_len = 0;
        for _ in 0..cnt {
            oid_len = (oid_len << 8) | (slice[idx] as crate::types::size_t);
            idx += 1;
        }
    }

    let content_type_p = unsafe { slice.as_ptr().add(idx) };
    let offset = (idx - inner_start) as crate::types::size_t;
    let total_oid = oid_len.wrapping_add(offset);
    let has_content = seq_len != total_oid;
    idx += oid_len as usize;
    // Narrow unsafe: write outputs.
    unsafe {
        (*contentType).tag = 0x06;
        (*contentType).len = oid_len;
        (*contentType).p = content_type_p as *mut ::core::ffi::c_uchar;
        *hasContent = has_content;
        *p = slice.as_ptr().add(idx) as *mut u8;
    }
    0
}

fn GetContentLenOfContentInfo(p: *mut *mut u8, end: *const u8, len: *mut usize)-> i32 {
    unsafe {
        let mut len_u32: crate::types::size_t = 0;
        let ret = mbedtls_asn1_get_tag(
            p,
            end,
            &mut len_u32,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32,
        );
        *len = len_u32 as usize;
        ret
    }
}

fn ParseSignerVersion(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        mbedtls_asn1_get_int(p, end, &mut (*signer).version)
    }
}

fn ParseSignerIssuerAndSerialNum(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        let mut len: crate::types::size_t = 0;
        let mut rc = crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_SEQUENCE) as i32,
        );
        if rc != 0 {
            return rc;
        }

        let issuer_start = *p;
        (*signer).issuerRaw.p = issuer_start as *mut ::core::ffi::c_uchar;

        rc = crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_SEQUENCE) as i32,
        );
        if rc != 0 {
            return rc;
        }

        // Inline replacement for mbedtls_x509_get_name: skip name content, zero issuer struct
        std::ptr::write(&mut (*signer).issuer, std::mem::zeroed());
        *p = (*p).add(len as usize);
        (*signer).issuerRaw.len = ((*p as usize).wrapping_sub(issuer_start as usize)) as crate::types::size_t;

        // Inline replacement for mbedtls_x509_get_serial
        let mut serial_len: crate::types::size_t = 0;
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut serial_len, 0x02 as i32);
        if rc != 0 {
            return rc;
        }
        (*signer).serial.p = *p as *mut ::core::ffi::c_uchar;
        (*signer).serial.len = serial_len;
        *p = (*p).add(serial_len as usize);

        0
    }
}

fn ParseSignerDigestAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        let rc = mbedtls_asn1_get_alg_null(p, end, &mut (*signer).digestAlgId);
        if rc != 0 {
            return rc;
        }
        if crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*signer).digestAlgId) {
            return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerAuthAttr(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        let raw = *p;
        if raw.is_null() || raw as usize >= end as usize {
            return 0;
        }
        if *raw != 0xA0u8 {
            return 0;
        }
        let mut cur = raw.offset(1);
        if cur as usize >= end as usize {
            return 0;
        }
        let first_len = *cur;
        let len: u32;
        if first_len & 0x80 == 0 {
            len = first_len as u32;
            cur = cur.offset(1);
        } else {
            let num_len = (first_len & 0x7F) as usize;
            cur = cur.offset(1);
            if (cur as usize).wrapping_add(num_len) > end as usize {
                return 0;
            }
            let mut tmp: u32 = 0;
            for _ in 0..num_len {
                tmp = (tmp << 8) | (*cur as u32);
                cur = cur.offset(1);
            }
            len = tmp;
        }
        *p = cur;
        (*signer).authAttr.tag = 0xA0i32;
        (*signer).authAttr.p = cur as *mut ::core::ffi::c_uchar;
        (*signer).authAttr.len = len as crate::types::size_t;
        let tl_len = ((*p as usize).wrapping_sub(raw as usize)) as u32;
        *p = (*p).offset(len as isize);
        (*signer).authAttrRaw.p = raw as *mut ::core::ffi::c_uchar;
        (*signer).authAttrRaw.len = (len.wrapping_add(tl_len)) as crate::types::size_t;
        return 0;
    }
}

fn InvalidDigestEncAlg(alg: *const crate::types::mbedtls_x509_buf)-> bool {
    unsafe {
        let alg_ref = &*alg;
        let alg_len = alg_ref.len as usize;
        let alg_slice = std::slice::from_raw_parts(alg_ref.p as *const u8, alg_len);

        // Return true if the OID does not match any valid digest encryption OID.
        !(alg_slice == &crate::types::MBEDTLS_OID_PKCS1_SHA256[..crate::types::MBEDTLS_OID_PKCS1_SHA256.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_PKCS1_SHA384[..crate::types::MBEDTLS_OID_PKCS1_SHA384.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_PKCS1_SHA512[..crate::types::MBEDTLS_OID_PKCS1_SHA512.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_ECDSA_SHA256[..crate::types::MBEDTLS_OID_ECDSA_SHA256.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_ECDSA_SHA384[..crate::types::MBEDTLS_OID_ECDSA_SHA384.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_ECDSA_SHA512[..crate::types::MBEDTLS_OID_ECDSA_SHA512.len() - 1]
            || alg_slice == &crate::types::MBEDTLS_OID_RSASSA_PSS[..crate::types::MBEDTLS_OID_RSASSA_PSS.len() - 1])
    }
}

fn ParseSignerEncAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        let mut params: crate::types::mbedtls_asn1_buf = std::mem::zeroed();
        let signer_ref = &mut *signer;
        let rc = mbedtls_asn1_get_alg(p, end, &mut signer_ref.digestEncAlgId, &mut params);
        if rc != 0 {
            return rc;
        }
        if crate::src_mbedtls_pkcs7::InvalidDigestEncAlg(&signer_ref.digestEncAlgId) {
            return crate::types::PKCS7_INVALID_SIGNING_ALG as i32;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerSignature(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        let mut len: crate::types::size_t = 0;

        // Parse tag (expect MBEDTLS_ASN1_OCTET_STRING = 0x04)
        if (*p as *const u8) >= end {
            return -0x0060; // MBEDTLS_ERR_ASN1_OUT_OF_DATA
        }
        if **p != crate::types::MBEDTLS_ASN1_OCTET_STRING as u8 {
            return -0x0062; // MBEDTLS_ERR_ASN1_UNEXPECTED_TAG
        }
        *p = (*p).add(1);

        // Parse length
        if (*p as *const u8) >= end {
            return -0x0060;
        }
        let byte = **p;
        if byte & 0x80 == 0 {
            // short form
            *p = (*p).add(1);
            len = byte as crate::types::size_t;
        } else {
            let num_bytes = (byte & 0x7F) as usize;
            if num_bytes == 0 || num_bytes > 4 {
                return -0x0064; // MBEDTLS_ERR_ASN1_INVALID_LENGTH
            }
            *p = (*p).add(1);
            if (*p as *const u8).add(num_bytes) > end {
                return -0x0060;
            }
            len = 0;
            for i in 0..num_bytes {
                len = (len << 8) | (*(*p).add(i) as crate::types::size_t);
            }
            *p = (*p).add(num_bytes);
        }

        // Check that content fits within the remaining data
        if (*p as *const u8).add(len as usize) > end {
            return -0x0060;
        }

        // Fill signer->signature fields
        (*signer).signature.tag = crate::types::MBEDTLS_ASN1_OCTET_STRING as i32;
        (*signer).signature.len = len;
        (*signer).signature.p = *p;

        // Advance p past the content
        *p = (*p).add(len as usize);

        crate::types::PKCS7_SUCC as i32
    }
}

fn GetSignerSignature(signer: *const crate::types::SignerInfo, sig: *mut *mut u8, sigLen: *mut usize)-> i32 {
    unsafe {
        let len = (*signer).signature.len;
        let buf = (*signer).signature.p as *mut u8;
        *sig = buf;
        *sigLen = len as usize;
        crate::types::PKCS7_SUCC as i32
    }
}

fn ParseSignerUnAuthAttr(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo)-> i32 {
    unsafe {
        if (end as isize) - (*p as isize) < 1 {
            return 0;
        }
        let tag = **p;
        *p = (*p).add(1);
        if tag != 0xA1 {
            return -1;
        }
        if (end as isize) - (*p as isize) < 1 {
            return -1;
        }
        let len_first = **p;
        *p = (*p).add(1);
        let len: crate::types::size_t;
        if len_first & 0x80 == 0 {
            len = len_first as crate::types::size_t;
        } else {
            let num_bytes = (len_first & 0x7F) as usize;
            if num_bytes == 0 || num_bytes > 4 {
                return -1;
            }
            if (end as isize) - (*p as isize) < num_bytes as isize {
                return -1;
            }
            let mut l: crate::types::size_t = 0;
            for _ in 0..num_bytes {
                l = (l << 8) | (**p as crate::types::size_t);
                *p = (*p).add(1);
            }
            len = l;
        }
        if (end as isize) - (*p as isize) < len as isize {
            return -1;
        }
        let signer_ref = &mut *signer;
        signer_ref.unAuthAttr.tag = 0xA1;
        signer_ref.unAuthAttr.len = len;
        signer_ref.unAuthAttr.p = *p;
        *p = (*p).add(len as usize);
        0
    }
}

fn SerialCmp(a: *const crate::types::mbedtls_x509_buf, b: *const crate::types::mbedtls_x509_buf)-> i32 {
    unsafe {
        if (*a).len == (*b).len && libc::memcmp((*a).p as *const ::core::ffi::c_void, (*b).p as *const ::core::ffi::c_void, (*a).len as usize) == 0 {
            return 0;
        }
        -1
    }
}

fn IsLegitString(tag: i32)-> bool {
    tag == 0x0C || tag == 0x13
}

fn CompareX509String(first: *const crate::types::mbedtls_x509_buf, second: *const crate::types::mbedtls_x509_buf)-> i32 {
    unsafe {
        let first_tag = (*first).tag;
        let second_tag = (*second).tag;
        if !crate::src_mbedtls_pkcs7::IsLegitString(first_tag)
            || !crate::src_mbedtls_pkcs7::IsLegitString(second_tag)
        {
            return -1;
        }

        let first_len = (*first).len as i32;
        let first_p = (*first).p;
        let second_p = (*second).p;

        for i in 0..first_len {
            let fp = *first_p.offset(i as isize);
            let sp = *second_p.offset(i as isize);

            if fp == sp {
                continue;
            }
            // case-insensitive compare: first is lower, second is upper
            if (fp >= b'a' && fp <= b'z') && fp.wrapping_sub(32) == sp {
                continue;
            }
            // case-insensitive compare: first is upper, second is lower
            if (fp >= b'A' && fp <= b'Z') && fp.wrapping_add(32) == sp {
                continue;
            }
            return -1;
        }
        0
    }
}

fn GetDeps(nameList: *const crate::types::mbedtls_x509_name) -> i32 {
    let mut deps: i32 = 0;
    let mut cur = nameList;
    while !cur.is_null() {
        deps += 1;
        cur = unsafe { (*cur).next as *const crate::types::mbedtls_x509_name };
    }
    deps
}

fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name)-> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    let first_deps = crate::src_mbedtls_pkcs7::GetDeps(first);
    let second_deps = crate::src_mbedtls_pkcs7::GetDeps(second);
    if first_deps != second_deps {
        return -1;
    }
    let mut cur_first: *const crate::types::mbedtls_x509_name = first;
    let mut cur_second: *const crate::types::mbedtls_x509_name = second;
    for _ in 0..first_deps {
        let first_ref = unsafe { &*cur_first };
        let second_ref = unsafe { &*cur_second };
        if first_ref.oid.tag != second_ref.oid.tag
            || first_ref.oid.len != second_ref.oid.len
        {
            return -1;
        }
        let first_oid_p = first_ref.oid.p;
        let second_oid_p = second_ref.oid.p;
        let oid_len = second_ref.oid.len as usize;
        if unsafe { std::slice::from_raw_parts(first_oid_p, oid_len) }
            != unsafe { std::slice::from_raw_parts(second_oid_p, oid_len) }
        {
            return -1;
        }
        if first_ref.private_next_merged != second_ref.private_next_merged
            || first_ref.val.len != second_ref.val.len
        {
            return -1;
        }
        if crate::src_mbedtls_pkcs7::CompareX509String(
            &first_ref.val as *const crate::types::mbedtls_x509_buf,
            &second_ref.val as *const crate::types::mbedtls_x509_buf,
        ) != 0
        {
            return -1;
        }
        cur_first = first_ref.next as *const _;
        cur_second = second_ref.next as *const _;
    }
    0
}

fn Pkcs7Calloc(nmemb: usize, size: usize)-> *mut std::ffi::c_void {
    unsafe { libc::calloc(nmemb, size) as *mut std::ffi::c_void }
}

fn Pkcs7Free(ptr: *mut std::ffi::c_void) {
    unsafe {
        libc::free(ptr);
    }
}

fn ParseSignedDataSignerInfos(p: *mut *mut u8, end: *const u8, mut signers: *mut crate::types::SignerInfo)-> i32 {
    let mut len: crate::types::size_t = 0;

    // Parse the outer SET tag and length
    {
        let mut cur = unsafe { *p };
        if (end as usize) < (cur as usize) + 1 {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        if unsafe { *cur } != (0x20 | 0x11) as u8 {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        cur = unsafe { cur.add(1) };
        if (end as usize) <= (cur as usize) {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        if (unsafe { *cur } & 0x80) == 0 {
            len = unsafe { *cur } as crate::types::size_t;
            cur = unsafe { cur.add(1) };
        } else {
            let n: u8 = unsafe { *cur } & 0x7F;
            if n < 1 || n > 4 {
                return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
            }
            if (end as usize) < (cur as usize) + 1 + n as usize {
                return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
            }
            len = 0;
            for i in 0..n {
                len = (len << 8) | unsafe { *cur.add(1 + i as usize) } as crate::types::size_t;
            }
            cur = unsafe { cur.add(1 + n as usize) };
        }
        if len > (end as usize - cur as usize) as crate::types::size_t {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        unsafe { *p = cur; }
    }

    if len == 0 {
        return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
    }

    let loop_end = unsafe { (*p).add(len as usize) as *const u8 };
    if loop_end > end {
        return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
    }

    let mut rc: i32 = 0;

    while unsafe { (*p as *const u8) } < loop_end {
        let mut one_signer_len: crate::types::size_t = 0;

        // Parse the individual SEQUENCE tag and length
        {
            let mut cur = unsafe { *p };
            if (loop_end as usize) < (cur as usize) + 1 {
                rc = -1;
            } else if unsafe { *cur } != (0x20 | 0x10) as u8 {
                rc = -1;
            } else {
                cur = unsafe { cur.add(1) };
                if (loop_end as usize) <= (cur as usize) {
                    rc = -1;
                } else if (unsafe { *cur } & 0x80) == 0 {
                    one_signer_len = unsafe { *cur } as crate::types::size_t;
                    cur = unsafe { cur.add(1) };
                    rc = 0;
                } else {
                    let n: u8 = unsafe { *cur } & 0x7F;
                    if n < 1 || n > 4 {
                        rc = -1;
                    } else if (loop_end as usize) < (cur as usize) + 1 + n as usize {
                        rc = -1;
                    } else {
                        one_signer_len = 0;
                        for i in 0..n {
                            one_signer_len = (one_signer_len << 8) | unsafe { *cur.add(1 + i as usize) } as crate::types::size_t;
                        }
                        cur = unsafe { cur.add(1 + n as usize) };
                        rc = 0;
                    }
                }
                if rc == 0 && one_signer_len > (loop_end as usize - cur as usize) as crate::types::size_t {
                    rc = -1;
                }
            }
            if rc == 0 {
                unsafe { *p = cur; }
            } else {
                return rc;
            }
        }

        let one_signer_end = unsafe { (*p).add(one_signer_len as usize) as *const u8 };
        if one_signer_end > loop_end {
            return -1;
        }

        rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, one_signer_end, signers);
        if rc != crate::types::PKCS7_SUCC as i32 { return rc; }

        if unsafe { *p as *const u8 } < loop_end {
            let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
                1,
                core::mem::size_of::<crate::types::SignerInfo>()
            ) as *mut crate::types::SignerInfo;
            if next.is_null() {
                return crate::types::PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe { (*signers).next = next; }
            signers = next;
        }
    }

    rc
}

fn ParseSignedDataVersion(p: *mut *mut u8, end: *const u8, ver: *mut i32)-> i32 {
    unsafe {
        let rc = mbedtls_asn1_get_int(p, end, ver);
        if rc != 0 {
            return rc;
        }
        if *ver != 1 {
            return crate::types::PKCS7_INVALID_VERSION as i32;
        }
        return crate::types::PKCS7_SUCC as i32;
    }
}

fn ParseSignedDataDigestAlgs(p: *mut *mut u8, end: *const u8, algIds: *mut crate::types::DigestAlgId)-> i32 {
    unsafe {
        let mut rc: i32;
        let mut len: crate::types::size_t = 0;

        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x31);
        if rc != 0 {
            return rc;
        }

        let end = (*p).add(len as usize) as *const u8;

        let mut id: *mut crate::types::DigestAlgId = algIds;
        while (*p as usize) < (end as usize) {
            let mut params: crate::types::mbedtls_asn1_buf = std::mem::zeroed();

            rc = mbedtls_asn1_get_alg(p, end, &mut (*id).algBuf, &mut params);
            if rc != 0 {
                return rc;
            }

            if crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*id).algBuf as *const crate::types::mbedtls_asn1_buf) {
                return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
            }

            if (*p as usize) < (end as usize) {
                let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
                    1usize,
                    std::mem::size_of::<crate::types::DigestAlgId>(),
                ) as *mut crate::types::DigestAlgId;

                if next.is_null() {
                    return crate::types::PKCS7_MEMORY_EXHAUST as i32;
                }

                (*id).next = next;
                id = next;
            }
        }

        crate::types::PKCS7_SUCC as i32
    }
}

fn DlogContentInfo(content: *const crate::types::Content) {
    unsafe {
        let len = (*content).data.len as i32;
        if len <= 0 {
            return;
        }
        let alloc_size = (len + 1) as usize;
        let info_ptr = crate::src_mbedtls_pkcs7::Pkcs7Calloc(alloc_size, 1) as *mut u8;
        if info_ptr.is_null() {
            return;
        }
        let src = (*content).data.p;
        let copy_count = len as usize;
        std::ptr::copy_nonoverlapping(src as *const u8, info_ptr, copy_count);
        *info_ptr.add(copy_count) = 0;
        crate::src_mbedtls_pkcs7::Pkcs7Free(info_ptr as *mut ::core::ffi::c_void);
    }
}

fn ParseSignedDataContentInfo(p: *mut *mut u8, end: *const u8, content: *mut crate::types::Content)-> i32 {
    unsafe {
        let mut has_content: bool = false;
        let rc = crate::src_mbedtls_pkcs7::GetContentInfoType(p, end, &mut (*content).oid, &mut has_content);
        if rc != 0 {
            return rc;
        }

        let oid_len = (*content).oid.len as usize;
        let expected_oid = &crate::types::MBEDTLS_OID_PKCS7_DATA[..9];
        let oid_match = oid_len == 9 && {
            let oid_slice = std::slice::from_raw_parts((*content).oid.p as *const u8, oid_len);
            oid_slice == expected_oid
        };

        if !oid_match || !has_content {
            // HiLogPrint omitted (unavailable symbol)
            return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        }

        let mut len: usize = 0;
        let rc2 = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(p, end, &mut len);
        if rc2 != 0 {
            return rc2;
        }

        (*content).data.tag = (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32;
        (*content).data.p = *p as *mut ::core::ffi::c_uchar;
        (*content).data.len = len as crate::types::size_t;
        crate::src_mbedtls_pkcs7::DlogContentInfo(content);

        *p = (*p).wrapping_add(len);
    }
    crate::types::PKCS7_SUCC as i32
}

fn ParseSignedDataCerts(p: *mut *mut u8, end: *const u8, certs: *mut *mut crate::types::mbedtls_x509_crt)-> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;

    rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x80) as i32) };
    if rc != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100u32,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const i8,
                b"ParseSignedDataCerts\0".as_ptr() as *const i8,
                532i32,
            );
        }
        return crate::types::PKCS7_SUCC as i32;
    }

    unsafe {
        *certs = Pkcs7Calloc(1, std::mem::size_of::<crate::types::mbedtls_x509_crt>())
            as *mut crate::types::mbedtls_x509_crt;
    }
    if unsafe { (*certs).is_null() } {
        return crate::types::PKCS7_MEMORY_EXHAUST as i32;
    }
    unsafe { mbedtls_x509_crt_init(*certs) };

    let certsEnd = unsafe { (*p).wrapping_add(len as usize) };
    let mut _cnt: i32 = 0;

    while (unsafe { *p } as usize) < (certsEnd as usize) {
        let mut oneCertLen: crate::types::size_t = 0;
        let seqBegin = unsafe { *p };
        rc = unsafe { mbedtls_asn1_get_tag(p, end, &mut oneCertLen, (0x20 | 0x10) as i32) };
        if rc != 0 {
            return rc;
        }

        let current_offset = unsafe { (*p as usize) - (seqBegin as usize) };
        let certsEnd_offset = (certsEnd as usize) - (seqBegin as usize);
        if (oneCertLen as usize) + current_offset > certsEnd_offset {
            return crate::types::PKCS7_PARSING_ERROR as i32;
        }

        rc = unsafe {
            mbedtls_x509_crt_parse(
                *certs,
                seqBegin,
                ((oneCertLen as usize) + current_offset) as crate::types::size_t,
            )
        };
        if rc != 0 {
            return rc;
        }

        unsafe { *p = (*p).wrapping_add(oneCertLen as usize) };
        _cnt += 1;
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100u32,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Parse signed data certs success\0".as_ptr() as *const i8,
            b"ParseSignedDataCerts\0".as_ptr() as *const i8,
            561i32,
        );
    }

    rc
}

fn ParseSignedDataCrl(p: *mut *mut u8, end: *const u8, crl: *mut crate::types::mbedtls_x509_crl)-> i32 {
    use crate::types::{size_t, MBEDTLS_ASN1_CONSTRUCTED, MBEDTLS_ASN1_CONTEXT_SPECIFIC, PKCS7_SUCC};
    let mut len: size_t = 0;
    let rc = unsafe {
        mbedtls_asn1_get_tag(
            p,
            end,
            &mut len as *mut size_t,
            (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32 + 1,
        )
    };
    if rc != 0 {
        let _ = std::io::Write::write_all(
            &mut std::io::stderr(),
            b"[ParseSignedDataCrl:572]: Has no crl in signed data.\n",
        );
        return PKCS7_SUCC as i32;
    }
    unsafe {
        mbedtls_x509_crl_init(crl);
    }
    let rc = unsafe { mbedtls_x509_crl_parse(crl, *p, len) };
    unsafe {
        *p = (*p).add(len as usize);
    }
    rc
}

fn ParseSignedData(buf: *mut u8, bufLen: usize, signedData: *mut crate::types::SignedData)-> i32 {
    let mut p = buf;
    let end = unsafe { buf.add(bufLen) };
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;

    unsafe {
        rc = mbedtls_asn1_get_tag(
            &mut p,
            end,
            &mut len,
            (crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_SEQUENCE) as i32,
        );
    }
    if rc != 0 {
        return rc;
    }

    let signed_data = unsafe { &mut *signedData };

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataVersion(&mut p, end, &mut signed_data.version);
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataDigestAlgs(&mut p, end, &mut signed_data.digestAlgIds);
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataContentInfo(&mut p, end, &mut signed_data.content);
    if rc != 0 {
        return rc;
    }

    if p >= end {
        return crate::types::PKCS7_PARSING_ERROR as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCerts(&mut p, end, &mut signed_data.certs);
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCrl(&mut p, end, &mut signed_data.crl);
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataSignerInfos(&mut p, end, &mut signed_data.signers);
    // Log omitted: HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ParseSignedData %d", __FUNCTION__, 629, rc);
    rc
}

fn IsSigedDataOid(pkcs7: *const crate::types::Pkcs7)-> bool {
    let expected_oid: [u8; 9] = [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
    unsafe {
        let oid = &(*pkcs7).contentTypeOid;
        if oid.len as usize == 9 {
            let slice = std::slice::from_raw_parts(oid.p as *const u8, 9);
            slice == &expected_oid
        } else {
            false
        }
    }
}

fn FreeSignedDataDigestAlgs(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut alg: *mut crate::types::DigestAlgId = (*pkcs7).signedData.digestAlgIds.next;
        while !alg.is_null() {
            let next: *mut crate::types::DigestAlgId = (*alg).next;
            crate::src_mbedtls_pkcs7::Pkcs7Free(alg as *mut ::core::ffi::c_void);
            alg = next;
        }
        (*pkcs7).signedData.digestAlgIds.next = std::ptr::null_mut();
    }
}

fn FreeSignerCerts(signer: *mut crate::types::SignerInfo) {
    unsafe {
        if !(*signer).certPath.crt.is_null() {
            crate::compat::mbedtls_x509_crt_free((*signer).certPath.crt);
            crate::compat::mbedtls_free((*signer).certPath.crt as *mut ::core::ffi::c_void);
            (*signer).certPath.crt = ::core::ptr::null_mut();
        }
    }
}

fn FreeSignerIssuer(signer: *mut crate::types::SignerInfo) {
    unsafe {
        let mut name_cur = (*signer).issuer.next;
        while !name_cur.is_null() {
            let name_prv = name_cur;
            name_cur = (*name_cur).next;
            Pkcs7Free(name_prv as *mut ::core::ffi::c_void);
        }
        (*signer).issuer.next = ::core::ptr::null_mut();
    }
}

fn FreeSignersInfo(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let signers = core::ptr::addr_of_mut!((*pkcs7).signedData.signers);
        let mut signer = (*signers).next;
        while !signer.is_null() {
            let next = (*signer).next;
            crate::src_mbedtls_pkcs7::FreeSignerCerts(signer);
            crate::src_mbedtls_pkcs7::FreeSignerIssuer(signer);
            crate::src_mbedtls_pkcs7::Pkcs7Free(signer as *mut ::core::ffi::c_void);
            signer = next;
        }
        (*signers).next = ::core::ptr::null_mut();
        crate::src_mbedtls_pkcs7::FreeSignerCerts(signers as *mut crate::types::SignerInfo);
        crate::src_mbedtls_pkcs7::FreeSignerIssuer(signers as *mut crate::types::SignerInfo);
    }
}

fn FreeSignedDataCerts(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let certs = (*pkcs7).signedData.certs;
        if !certs.is_null() {
            mbedtls_x509_crt_free(certs);
            mbedtls_free(certs as *mut ::core::ffi::c_void);
            (*pkcs7).signedData.certs = std::ptr::null_mut();
        }
    }
}

fn FreeSignedDataCrl(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        mbedtls_x509_crl_free(::core::ptr::addr_of_mut!((*pkcs7).signedData.crl));
    }
}

fn GetCertsNumOfSignedData(crts: *const crate::types::mbedtls_x509_crt)-> i32 {
    let mut cnt: i32 = 0;
    let mut current: *mut crate::types::mbedtls_x509_crt = crts as *mut crate::types::mbedtls_x509_crt;
    while !current.is_null() {
        unsafe {
            current = (*current).next;
        }
        cnt += 1;
    }
    cnt
}

fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt)-> *mut crate::types::mbedtls_x509_crt {
    let mut certsList = certsList;
    unsafe {
        while !certsList.is_null() {
            let first = &(*cur).issuer as *const _;
            let second = &(*certsList).subject as *const _;
            if crate::src_mbedtls_pkcs7::CompareX509NameList(first, second) == 0 {
                break;
            }
            certsList = (*certsList).next;
        }
    }
    certsList
}

fn DelCertOfSignedData(signedData: *mut crate::types::SignedData, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let head_ptr = (*signedData).certs;
        if head_ptr == crt {
            (*signedData).certs = (*crt).next;
            (*crt).next = std::ptr::null_mut();
        } else {
            let mut prev = head_ptr;
            let mut cur = head_ptr;
            while !cur.is_null() {
                if cur == crt {
                    (*prev).next = (*crt).next;
                    (*crt).next = std::ptr::null_mut();
                    break;
                }
                prev = cur;
                cur = (*cur).next;
            }
        }
    }
}

fn AddCertToSignerCertPath(signer: *mut crate::types::SignerInfo, crt: *mut crate::types::mbedtls_x509_crt) {
    unsafe {
        let prev = (*signer).certPath.crt;
        if prev.is_null() {
            (*signer).certPath.crt = crt;
            (*crt).next = ::core::ptr::null_mut::<crate::types::mbedtls_x509_crt>();
        } else {
            let mut cur = prev;
            let mut prev = prev;
            while !cur.is_null() {
                prev = cur;
                cur = (*cur).next;
            }
            (*prev).next = crt;
            (*crt).next = ::core::ptr::null_mut::<crate::types::mbedtls_x509_crt>();
        }
        (*signer).certPath.depth += 1;
    }
}

fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData)-> i32 {
    unsafe {
        if !g_rootCertLoaded {
            return crate::types::PKCS7_ROOT_CA_NOT_VALID as i32;
        }
        (*signer).rootCert = core::ptr::addr_of_mut!(g_rootCaG2Cert);
    }

    let certs: *mut crate::types::mbedtls_x509_crt = unsafe { (*signeData).certs };
    let mut cur: *mut crate::types::mbedtls_x509_crt = lowerCrt;
    let mut next: *mut crate::types::mbedtls_x509_crt = core::ptr::null_mut();
    let certsCnt = crate::src_mbedtls_pkcs7::GetCertsNumOfSignedData(certs as *const _);
    crate::src_mbedtls_pkcs7::DelCertOfSignedData(signeData, cur);
    crate::src_mbedtls_pkcs7::AddCertToSignerCertPath(signer, cur);
    let mut scanCnt: i32 = 0;
    let mut rc: i32 = crate::types::PKCS7_SUCC as i32;

    loop {
        next = crate::src_mbedtls_pkcs7::FindSuperCert(cur, unsafe { (*signeData).certs });
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

    rc
}

fn ConstructSignerCerts(signedData: *mut crate::types::SignedData)-> i32 {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = &mut (*signedData).signers as *mut crate::types::SignerInfo;
        while !signer.is_null() {
            let signerSerial: *const crate::types::mbedtls_x509_buf = &(*signer).serial;
            let signerIssuer: *const crate::types::mbedtls_x509_name = &(*signer).issuer;
            let mut cert: *mut crate::types::mbedtls_x509_crt = (*signedData).certs;
            // HiLogPrint omitted – no available declaration
            while !cert.is_null() {
                if crate::src_mbedtls_pkcs7::SerialCmp(signerSerial, &(*cert).serial) == 0
                    && crate::src_mbedtls_pkcs7::CompareX509NameList(signerIssuer, &(*cert).issuer) == 0
                {
                    break;
                }
                cert = (*cert).next;
            }
            if cert.is_null() {
                // HiLogPrint omitted
                return crate::types::PKCS7_INVALID_VALUE as i32;
            }
            let rc: i32 = crate::src_mbedtls_pkcs7::BuildSignerCertPath(signer, cert, signedData);
            if rc != 0 {
                return rc;
            }
            signer = (*signer).next;
        }
        0
    }
}

fn GetSignerDigestAlg(signer: *const crate::types::SignerInfo, algType: *mut crate::types::mbedtls_md_type_t)-> i32 {
    unsafe {
        let alg: &crate::types::mbedtls_x509_buf = &(*signer).digestAlgId;
        let alg_len = alg.len as usize;
        let alg_p = alg.p as *const u8;
        let alg_slice = std::slice::from_raw_parts(alg_p, alg_len);

        // SHA256
        if alg_slice == &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256[..9] {
            *algType = crate::types::MBEDTLS_MD_SHA256;
            return crate::types::PKCS7_SUCC as i32;
        }
        // SHA384
        if alg_slice == &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384[..9] {
            *algType = crate::types::MBEDTLS_MD_SHA384;
            return crate::types::PKCS7_SUCC as i32;
        }
        // SHA512
        if alg_slice == &crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512[..9] {
            *algType = crate::types::MBEDTLS_MD_SHA512;
            return crate::types::PKCS7_SUCC as i32;
        }
        crate::types::PKCS7_INVALID_DIGEST_ALG as i32
    }
}

fn GetSignerPubKeyOfSignature(signer: *const crate::types::SignerInfo, pk: *mut *mut crate::types::mbedtls_pk_context)-> i32 {
    if signer.is_null() || pk.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        let crt = (*signer).certPath.crt;
        if !crt.is_null() {
            *pk = &mut (*crt).pk as *mut crate::types::mbedtls_pk_context;
            return crate::types::PKCS7_SUCC as i32;
        }
    }
    crate::types::PKCS7_INVALID_VALUE as i32
}

pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    if pkcs7.is_null() || calcDigest.is_none() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let calc_fn = calcDigest.unwrap();
    let mut signer: *const crate::types::SignerInfo =
        unsafe { &(*pkcs7).signedData.signers as *const crate::types::SignerInfo };
    let mut rc: i32 = crate::types::PKCS7_SUCC as i32;
    while !signer.is_null() {
        let mut sig: *mut u8 = std::ptr::null_mut();
        let mut sig_len: usize = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerSignature(signer, &mut sig, &mut sig_len);
        if rc != 0 {
            return rc;
        }

        let mut pk: *mut crate::types::mbedtls_pk_context = std::ptr::null_mut();
        rc = crate::src_mbedtls_pkcs7::GetSignerPubKeyOfSignature(signer, &mut pk);
        if rc != 0 {
            return rc;
        }
        let mut dig_alg: u32 = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerDigestAlg(signer, &mut dig_alg);
        if rc != 0 {
            return rc;
        }
        let mut hash: [u8; 64] = [0u8; 64];
        unsafe { std::ptr::write_bytes(hash.as_mut_ptr(), 0, 64); }
        let mut hash_len: u32 = 0;
        rc = unsafe {
            (calc_fn)(
                pkcs7,
                signer,
                dig_alg,
                hash.as_mut_ptr(),
                &mut hash_len as *mut u32 as *mut crate::types::size_t,
            )
        };
        if rc != 0 {
            return rc;
        }

        let oid_len: usize = unsafe { (*signer).digestEncAlgId.len as usize };
        let oid_ptr: *mut u8 = unsafe { (*signer).digestEncAlgId.p };
        let pss_oid: &[u8; 10] = crate::types::MBEDTLS_OID_RSASSA_PSS;
        if oid_len == 9
            && unsafe { std::slice::from_raw_parts(oid_ptr as *const u8, oid_len) } == &pss_oid[..9]
        {
            unsafe {
                crate::compat::mbedtls_rsa_set_padding((*pk).private_pk_ctx as *mut mbedtls_rsa_context, 1_i32, 0_u32);
            }
        }

        rc = unsafe {
            crate::compat::mbedtls_pk_verify(
                pk,
                dig_alg,
                hash.as_ptr(),
                hash_len,
                sig,
                sig_len as u32,
            )
        };
        unsafe { std::ptr::write_bytes(hash.as_mut_ptr(), 0, 64); }
        if rc != 0 {
            return rc;
        }

        signer = unsafe { (*signer).next };
    }
    rc
}

fn LoadRootCert()-> i32 {
    let mut rc: i32 = 0;
    unsafe {
        if !g_rootCertLoaded {
            mbedtls_x509_crt_init(&mut g_rootCaG2Cert);
            rc = mbedtls_x509_crt_parse(
                &mut g_rootCaG2Cert,
                ROOT_CA_G2_CERT_IN_PEM.as_ptr() as *const u8,
                ROOT_CA_G2_CERT_IN_PEM.len() as crate::types::size_t,
            );
            if rc != 0 {
                return rc;
            }
            g_rootCertLoaded = true;
        }
    }
    rc
}

fn UnLoadRootCert() {
    unsafe {
        if g_rootCertLoaded {
            mbedtls_x509_crt_free(core::ptr::addr_of_mut!(g_rootCaG2Cert));
            g_rootCertLoaded = false;
        }
    }
}

fn LoadDebugModeRootCert()-> i32 {
    unsafe {
        mbedtls_x509_crt_init(core::ptr::addr_of_mut!(g_debugModeRootCert));
    }
    let rc = unsafe {
        mbedtls_x509_crt_parse(
            core::ptr::addr_of_mut!(g_debugModeRootCert),
            DEBUG_MODE_ROOT_CERT_IN_PEM.as_ptr(),
            DEBUG_MODE_ROOT_CERT_IN_PEM.len() as crate::types::size_t,
        )
    };
    rc
}

fn UnLoadDebugModeRootCert()-> i32 {
    unsafe {
        mbedtls_x509_crt_free(&mut g_debugModeRootCert);
    }
    PKCS7_SUCC as i32
}

fn LoadSelfSignedCert()-> i32 {
    unsafe {
        mbedtls_x509_crt_init(core::ptr::addr_of_mut!(g_ohosRootCert));
    }
    let rc = unsafe {
        mbedtls_x509_crt_parse(
            core::ptr::addr_of_mut!(g_ohosRootCert),
            OHOS_ROOT_CERT_IN_PEM.as_ptr() as *const ::core::ffi::c_uchar,
            OHOS_ROOT_CERT_IN_PEM.len() as crate::types::size_t,
        )
    };
    rc
}

fn UnLoadSelfSignedCert() {
    unsafe {
        mbedtls_x509_crt_free(core::ptr::addr_of_mut!(g_ohosRootCert));
    }
}

fn DLogCrtVerifyInfo(flags: i32) {
    let mut vrfyBuf: [u8; 512] = [0; 512];
    unsafe {
        mbedtls_x509_crt_verify_info(
            vrfyBuf.as_mut_ptr() as *mut ::core::ffi::c_char,
            vrfyBuf.len().try_into().unwrap(),
            b" ! \0".as_ptr() as *const ::core::ffi::c_char,
            flags as u32,
        );
        let _ = HiLogPrint(
            LOG_CORE as u32,
            LOG_DEBUG as u32,
            0xD001100u32,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"DLogCrtVerifyInfo\0".as_ptr() as *const ::core::ffi::c_char,
            981i32,
            vrfyBuf.as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

fn IsRevoked(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl)-> i32 {
    unsafe {
        let mut cur: *const crate::types::mbedtls_x509_crl_entry = std::ptr::addr_of!((*crl).entry);
        while !cur.is_null() {
            let cur_serial = &(*cur).serial;
            if cur_serial.len == 0 {
                return crate::types::PKCS7_SUCC as i32;
            }
            let crt_serial = &(*crt).serial;
            if crt_serial.len != cur_serial.len {
                cur = (*cur).next as *const crate::types::mbedtls_x509_crl_entry;
                continue;
            }
            let len = cur_serial.len as usize;
            let crt_slice = std::slice::from_raw_parts(crt_serial.p as *const u8, len);
            let cur_slice = std::slice::from_raw_parts(cur_serial.p as *const u8, len);
            if crt_slice == cur_slice {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            cur = (*cur).next as *const crate::types::mbedtls_x509_crl_entry;
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn VerifyCrl(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl)-> i32 {
    let mut crl_list: *const crate::types::mbedtls_x509_crl = crl;
    while !crl_list.is_null() {
        unsafe {
            if (*crl_list).version == 0
                || crate::src_mbedtls_pkcs7::CompareX509NameList(
                    &(*crl_list).issuer,
                    &(*crt).issuer,
                ) != 0
            {
                crl_list = (*crl_list).next as *const crate::types::mbedtls_x509_crl;
                continue;
            }
        }
        if crate::src_mbedtls_pkcs7::IsRevoked(crt, crl_list) != 0 {
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        unsafe {
            crl_list = (*crl_list).next as *const crate::types::mbedtls_x509_crl;
        }
    }
    crate::types::PKCS7_SUCC as i32
}

fn VerifyClicert(clicert: *mut crate::types::mbedtls_x509_crt, rootCert: *mut crate::types::mbedtls_x509_crt, pkcs7: *const crate::types::Pkcs7)-> i32 {
    let mut flags: u32 = 0;
    let crl_ptr: *mut crate::types::mbedtls_x509_crl = {
        let signed_data_ref = unsafe { &(*pkcs7).signedData };
        &signed_data_ref.crl as *const _ as *mut _
    };
    let rc = unsafe {
        mbedtls_x509_crt_verify(
            clicert,
            rootCert,
            crl_ptr,
            core::ptr::null(),
            &mut flags as *mut u32,
            None,
            core::ptr::null_mut()
        )
    };
    if rc != 0 {
        crate::src_mbedtls_pkcs7::DLogCrtVerifyInfo(flags as i32);
        return rc;
    }
    let verify_result = crate::src_mbedtls_pkcs7::VerifyCrl(clicert as *const _, crl_ptr as *const _);
    if verify_result != crate::types::PKCS7_SUCC as i32 {
        return crate::types::PKCS7_IS_REVOKED as i32;
    }
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_VerifyCertsChain(pkcs7: *const crate::types::Pkcs7) -> i32 {
    if pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let pkcs7_ref = unsafe { &*pkcs7 };
    let mut signer: *const crate::types::SignerInfo = core::ptr::addr_of!(pkcs7_ref.signedData.signers);
    let debug_root = unsafe { core::ptr::addr_of_mut!(g_debugModeRootCert) };
    let ohos_root = unsafe { core::ptr::addr_of_mut!(g_ohosRootCert) };
    while !signer.is_null() {
        let signer_ref = unsafe { &*signer };
        let clicert: *mut crate::types::mbedtls_x509_crt = signer_ref.certPath.crt;
        if clicert.is_null() {
            return crate::types::PKCS7_HAS_NO_SIGNER_CRT as i32;
        }
        let debug_enabled = unsafe { g_debugModeEnabled };
        if debug_enabled {
            let rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                debug_root,
                pkcs7 as *const _,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = signer_ref.next as *const _;
                continue;
            }
            if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
        }
        let rc = crate::src_mbedtls_pkcs7::VerifyClicert(
            clicert,
            signer_ref.rootCert,
            pkcs7 as *const _,
        );
        if rc == crate::types::PKCS7_SUCC as i32 {
            signer = signer_ref.next as *const _;
            continue;
        }
        if rc == crate::types::PKCS7_IS_REVOKED as i32 {
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        let rc = crate::src_mbedtls_pkcs7::VerifyClicert(
            clicert,
            ohos_root,
            pkcs7 as *const _,
        );
        if rc == crate::types::PKCS7_SUCC as i32 {
            signer = signer_ref.next as *const _;
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
    let rc: i32 = unsafe {
        mbedtls_x509_dn_gets(
            subject,
            subjectLen,
            core::ptr::addr_of!((*crt).subject),
        )
    };
    if rc < 0 {
        return rc;
    }
    crate::types::PKCS7_SUCC as i32
}

pub extern "C" fn PKCS7_GetSignerSignningCertIssuer(signer: *const crate::types::SignerInfo, issuer: *mut ::core::ffi::c_char, issuerLen: crate::types::size_t) -> i32 {
    if signer.is_null() || issuer.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        extern "C" {
            fn mbedtls_x509_dn_gets(
                buf: *mut ::core::ffi::c_char,
                size: crate::types::size_t,
                dn: *const crate::types::mbedtls_x509_name,
            ) -> i32;
        }
        let crt = (*signer).certPath.crt as *const crate::types::mbedtls_x509_crt;
        let rc = mbedtls_x509_dn_gets(issuer, issuerLen, &(*crt).issuer as *const _);
        if rc < 0 {
            return rc;
        }
        crate::types::PKCS7_SUCC as i32
    }
}

fn GetSignersCnt(signers: *const crate::types::SignerInfo)-> i32 {
    let mut cnt: i32 = 0;
    let mut cur: *const crate::types::SignerInfo = signers;
    while !cur.is_null() {
        cnt += 1;
        cur = unsafe { (*cur).next };
    }
    cnt
}

fn IsIncludeRoot(signer: *const crate::types::SignerInfo)-> bool {
    if signer.is_null() {
        return false;
    }
    let cert_path = unsafe { &(*signer).certPath };
    let mut pre: *mut crate::types::mbedtls_x509_crt = cert_path.crt;
    let mut cur: *mut crate::types::mbedtls_x509_crt = pre;
    let mut i: i32 = 0;
    while i < cert_path.depth && !cur.is_null() {
        pre = cur;
        cur = unsafe { (*cur).next };
        i += 1;
    }
    if pre.is_null() {
        return false;
    }
    let cmp = unsafe {
        crate::src_mbedtls_pkcs7::CompareX509NameList(
            &(*pre).issuer as *const crate::types::mbedtls_x509_name,
            &(*pre).subject as *const crate::types::mbedtls_x509_name,
        )
    };
    cmp == 0
}

fn GetSignerSignningCertDepth(signer: *const crate::types::SignerInfo)-> i32 {
    let depth = unsafe { (*signer).certPath.depth };
    if crate::src_mbedtls_pkcs7::IsIncludeRoot(signer) {
        return depth;
    }
    depth + 1
}

pub extern "C" fn PKCS7_FreeAllSignersResolvedInfo(sri: *mut crate::types::SignersResovedInfo) {
    if sri.is_null() {
        return;
    }
    let sri_ref = unsafe { &mut *sri };
    if !sri_ref.signers.is_null() {
        Pkcs7Free(sri_ref.signers as *mut ::core::ffi::c_void);
        sri_ref.signers = std::ptr::null_mut();
    }
    Pkcs7Free(sri as *mut ::core::ffi::c_void);
}

pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    if pkcs7.is_null() {
        return std::ptr::null_mut();
    }
    let signers_cnt = unsafe {
        crate::src_mbedtls_pkcs7::GetSignersCnt(
            &(*pkcs7).signedData.signers as *const crate::types::SignerInfo,
        )
    };
    if signers_cnt == 0 {
        return std::ptr::null_mut();
    }
    let sri = unsafe {
        crate::src_mbedtls_pkcs7::Pkcs7Calloc(
            1,
            std::mem::size_of::<crate::types::SignersResovedInfo>(),
        ) as *mut crate::types::SignersResovedInfo
    };
    if sri.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*sri).nrOfSigners = signers_cnt;
    }
    let signers = unsafe {
        crate::src_mbedtls_pkcs7::Pkcs7Calloc(
            signers_cnt as usize,
            std::mem::size_of::<crate::types::SignerResovledInfo>(),
        ) as *mut crate::types::SignerResovledInfo
    };
    unsafe {
        (*sri).signers = signers;
    }
    if signers.is_null() {
        unsafe {
            crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut std::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    let mut signer: *const crate::types::SignerInfo = unsafe {
        &(*pkcs7).signedData.signers as *const crate::types::SignerInfo
    };
    let mut idx = 0i32;
    while !signer.is_null() && idx < signers_cnt {
        let signer_entry = unsafe { (*sri).signers.add(idx as usize) };
        let rc = unsafe {
            crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertSubject(
                signer,
                &mut (*signer_entry).subject as *mut _,
                512 as crate::types::size_t,
            )
        };
        if rc != 0 {
            unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri); }
            return std::ptr::null_mut();
        }
        let rc = unsafe {
            crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertIssuer(
                signer,
                &mut (*signer_entry).issuer as *mut _,
                512 as crate::types::size_t,
            )
        };
        if rc != 0 {
            unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri); }
            return std::ptr::null_mut();
        }
        unsafe {
            (*signer_entry).depth = crate::src_mbedtls_pkcs7::GetSignerSignningCertDepth(signer);
        }
        signer = unsafe { (*signer).next as *const crate::types::SignerInfo };
        idx += 1;
    }
    sri
}

pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    if signer.is_null() || dig.is_null() || digLen.is_null() {
        return crate::types::PKCS7_INVALID_VALUE as i32;
    }
    let signer_ref = unsafe { &*signer };
    let mut p: *mut u8 = signer_ref.authAttr.p;
    if p.is_null() {
        return crate::types::PKCS7_HAS_NO_AUTH_ATTR_IN_SIGNER as i32;
    }
    let len: crate::types::size_t = signer_ref.authAttr.len;
    let end: *mut u8 = unsafe { p.add(len as usize) };

    // ASN.1 TLV tag+length parser (inline closure)
    let mut asn1_get_tag = |pp: &mut *mut u8, end: *mut u8, expected_tag: u32| -> Result<crate::types::size_t, i32> {
        // Check we have at least the tag byte
        if (*pp) >= end {
            return Err(-1);
        }
        let tag = unsafe { **pp };
        if tag != expected_tag as u8 {
            return Err(-1);
        }
        *pp = unsafe { (*pp).add(1) };

        if (*pp) >= end {
            return Err(-1);
        }
        let len_byte = unsafe { **pp };
        if len_byte & 0x80 == 0 {
            // Short form length
            let out_len = len_byte as crate::types::size_t;
            *pp = unsafe { (*pp).add(1) };
            if unsafe { (*pp).add(out_len as usize) } > end {
                return Err(-1);
            }
            Ok(out_len)
        } else if len_byte == 0x80 {
            // Indefinite length not supported
            Err(-1)
        } else {
            // Long form length
            let num_bytes = (len_byte & 0x7f) as usize;
            *pp = unsafe { (*pp).add(1) };
            if num_bytes > std::mem::size_of::<crate::types::size_t>() || (*pp) > end {
                return Err(-1);
            }
            let mut out_len: crate::types::size_t = 0;
            for _ in 0..num_bytes {
                if *pp >= end {
                    return Err(-1);
                }
                out_len = (out_len << 8) | unsafe { **pp as crate::types::size_t };
                *pp = unsafe { (*pp).add(1) };
            }
            if unsafe { (*pp).add(out_len as usize) } > end {
                return Err(-1);
            }
            Ok(out_len)
        }
    };

    while p < end {
        // Parse SEQUENCE (tag 0x30)
        let seq_len = match asn1_get_tag(&mut p, end, 0x10 | 0x20) {
            Ok(l) => l,
            Err(e) => return e,
        };
        let seq_end = unsafe { p.add(seq_len as usize) };

        // Parse OID inside the SEQUENCE
        let oid_len = match asn1_get_tag(&mut p, seq_end, 0x06) {
            Ok(l) => l,
            Err(e) => return e,
        };
        const OID_MSG_DIGEST: [u8; 9] = [
            0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x04,
        ];
        if oid_len == (OID_MSG_DIGEST.len() as crate::types::size_t) {
            let oid_val = unsafe { std::slice::from_raw_parts(p as *const u8, oid_len as usize) };
            if oid_val == &OID_MSG_DIGEST {
                // Advance past the OID value
                p = unsafe { p.add(oid_len as usize) };

                // Skip the SET wrapper (tag 0x31)
                let _set_len = match asn1_get_tag(&mut p, seq_end, 0x11 | 0x20) {
                    Ok(l) => l,
                    Err(e) => return e,
                };

                // Read the OCTET STRING (tag 0x04) containing the digest
                let octet_len = match asn1_get_tag(&mut p, seq_end, 0x04) {
                    Ok(l) => l,
                    Err(e) => return e,
                };
                unsafe {
                    *dig = p;
                    *digLen = octet_len;
                }
                return crate::types::PKCS7_SUCC as i32;
            }
        }
        // Not the target OID – skip to the end of this SEQUENCE
        p = seq_end;
    }

    crate::types::PKCS7_INVALID_VALUE as i32
}

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
        *((*signer).authAttrRaw.p) = (0x20 | 0x11) as u8;
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
        let end = p.add(len as usize);
        let mut cur = p;
        if cur >= end {
            return -0x0062i32;
        }
        if *cur != 0x04 {
            return -0x0060i32;
        }
        cur = cur.add(1);
        if cur >= end {
            return -0x0062i32;
        }
        let mut length: crate::types::size_t = *cur as crate::types::size_t;
        cur = cur.add(1);
        if length & 0x80 != 0 {
            let num_bytes = (length & 0x7F) as usize;
            if num_bytes == 0 || num_bytes > 4 {
                return -0x0064i32;
            }
            if cur as usize + num_bytes > end as usize {
                return -0x0062i32;
            }
            length = 0;
            for _ in 0..num_bytes {
                if cur >= end {
                    return -0x0062i32;
                }
                length = (length << 8) | (*cur as crate::types::size_t);
                cur = cur.add(1);
            }
        }
        if cur as usize + length as usize > end as usize {
            return -0x0062i32;
        }
        *data = cur;
        *dataLen = length;
    }
    return crate::types::PKCS7_SUCC as i32;
}

pub extern "C" fn PKCS7_EnableDebugMode(mode: bool) -> i32 {
    unsafe {
        if g_debugModeEnabled == mode {
            return PKCS7_SUCC as i32;
        }
    }
    let rc = if mode {
        crate::src_mbedtls_pkcs7::LoadDebugModeRootCert()
    } else {
        crate::src_mbedtls_pkcs7::UnLoadDebugModeRootCert()
    };
    if rc != 0 {
        return rc;
    }
    unsafe {
        g_debugModeEnabled = mode;
    }
    PKCS7_SUCC as i32
}

fn ParsePemFormatSignedData(buf: *const u8, bufLen: usize, pem: *mut crate::types::mbedtls_pem_context, format: *mut std::ffi::c_char)-> i32 {
    if bufLen != 0 && {
        let needle = b"-----BEGIN PKCS7-----";
        if bufLen < needle.len() {
            false
        } else {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, bufLen) };
            buf_slice.windows(needle.len()).any(|w| w == needle)
        }
    } {
        let ret: i32;
        let mut use_len: crate::types::size_t = 0;
        unsafe { mbedtls_pem_init(pem); }
        ret = unsafe {
            mbedtls_pem_read_buffer(
                pem,
                b"-----BEGIN PKCS7-----\0".as_ptr() as *const std::ffi::c_char,
                b"-----END PKCS7-----\0".as_ptr() as *const std::ffi::c_char,
                buf as *const ::core::ffi::c_uchar,
                std::ptr::null::<::core::ffi::c_uchar>(),
                0 as crate::types::size_t,
                &mut use_len as *mut crate::types::size_t,
            )
        };
        if ret == 0 && (use_len as usize) == bufLen {
            unsafe { *format = 1 as std::ffi::c_char; }
            return crate::types::PKCS7_SUCC as i32;
        }
        unsafe { mbedtls_pem_free(pem); }
    } else {
        unsafe { *format = 2 as std::ffi::c_char; }
        return crate::types::PKCS7_SUCC as i32;
    }
    return crate::types::PKCS7_INVALID_PARAM as i32;
}

pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }

    unsafe {
        std::ptr::write_bytes(pkcs7 as *mut u8, 0, std::mem::size_of::<crate::types::Pkcs7>());
    }

    let mut start_ptr: *mut u8 = buf as *mut u8;
    let mut buf_len: crate::types::size_t = bufLen;

    let mut format: std::ffi::c_char = 0;
    // Use a local mbedtls_pem_context since Pkcs7 does not have a pem field in the Rust struct.
    let mut pem: crate::types::mbedtls_pem_context = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(
            buf as *const u8,
            buf_len as usize,
            &mut pem as *mut crate::types::mbedtls_pem_context,
            &mut format,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    if format == 1 {
        start_ptr = pem.private_buf as *mut u8;
        buf_len = pem.private_buflen;
    }

    let end: *const u8 = unsafe { start_ptr.add(buf_len as usize) } as *const u8;

    let rc = unsafe { crate::src_mbedtls_pkcs7::LoadRootCert() };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    let rc = unsafe { crate::src_mbedtls_pkcs7::LoadSelfSignedCert() };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    let mut has_content: bool = false;
    let rc = unsafe {
        crate::src_mbedtls_pkcs7::GetContentInfoType(
            &mut start_ptr,
            end,
            &mut (*pkcs7).contentTypeOid,
            &mut has_content,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    if !unsafe { crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) } || !has_content {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    let mut len: usize = 0;
    let rc = unsafe {
        crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(
            &mut start_ptr,
            end,
            &mut len,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    let start_end = unsafe { start_ptr.add(len) } as *const u8;
    if start_end > end {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    let rc = unsafe {
        crate::src_mbedtls_pkcs7::ParseSignedData(
            start_ptr,
            len,
            &mut (*pkcs7).signedData,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    let rc = unsafe {
        crate::src_mbedtls_pkcs7::ConstructSignerCerts(
            &mut (*pkcs7).signedData,
        )
    };
    rc as i32
}

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
