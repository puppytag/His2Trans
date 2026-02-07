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
    let mut tmpLen: size_t = 0;
    let mut current = p;
    while current < end {
        let mut seqLen: size_t = 0;
        let mut seqEnd: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
        let mut rc = unsafe { mbedtls_asn1_get_tag(&mut current, end, &mut seqLen, (MBEDTLS_ASN1_SEQUENCE | MBEDTLS_ASN1_CONSTRUCTED) as u8) };
        if rc != 0 {
            return rc;
        }
        seqEnd = unsafe { current.offset(seqLen as isize) };
        let mut oidLen: size_t = 0;
        rc = unsafe { mbedtls_asn1_get_tag(&mut current, seqEnd, &mut oidLen, MBEDTLS_ASN1_OID as u8) };
        if rc != 0 {
            return rc;
        }
        if oidLen == (MBEDTLS_OID_SIZE(MBEDTLS_OID_PKCS9_MSG_DIGEST) as size_t) &&
            unsafe { libc::memcmp(current as *const _, MBEDTLS_OID_PKCS9_MSG_DIGEST as *const _, oidLen as usize) } == 0 {
            current = unsafe { current.offset(oidLen as isize) };
            rc = unsafe { mbedtls_asn1_get_tag(&mut current, seqEnd, &mut tmpLen, (MBEDTLS_ASN1_SET | MBEDTLS_ASN1_CONSTRUCTED) as u8) };
            if rc != 0 {
                return rc;
            }
            rc = unsafe { mbedtls_asn1_get_tag(&mut current, seqEnd, &mut tmpLen, MBEDTLS_ASN1_OCTET_STRING as u8) };
            if rc != 0 {
                return rc;
            }
            unsafe {
                *dig = current;
                *digLen = tmpLen;
            }
            return PKCS7_SUCC as i32;
        } else {
            current = seqEnd;
        }
    }
    PKCS7_INVALID_VALUE as i32
}