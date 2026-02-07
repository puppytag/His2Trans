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