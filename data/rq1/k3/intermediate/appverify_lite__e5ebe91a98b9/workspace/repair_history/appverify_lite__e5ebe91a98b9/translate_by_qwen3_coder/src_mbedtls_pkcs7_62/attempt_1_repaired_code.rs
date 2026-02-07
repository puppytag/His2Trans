pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    if signer.is_null() || dig.is_null() || digLen.is_null() {
        return crate::types::PKCS7_INVALID_VALUE as i32;
    }
    let p = unsafe { (*signer).authAttr.p };
    if p.is_null() {
        return crate::types::PKCS7_HAS_NO_AUTH_ATTR_IN_SIGNER as i32;
    }
    let end = unsafe { p.add((*signer).authAttr.len as usize) };
    let mut p = p;
    let mut tmpLen: crate::types::size_t = 0;
    while p < end {
        let mut seqLen: crate::types::size_t = 0;
        let mut seqEnd: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
        let mut rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut p as *mut *mut ::core::ffi::c_uchar,
                end,
                &mut seqLen as *mut crate::types::size_t,
                crate::types::MBEDTLS_ASN1_SEQUENCE | crate::types::MBEDTLS_ASN1_CONSTRUCTED,
            )
        };
        if rc != 0 {
            return rc;
        }
        seqEnd = unsafe { p.add(seqLen as usize) };
        let mut oidLen: crate::types::size_t = 0;
        rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut p as *mut *mut ::core::ffi::c_uchar,
                seqEnd,
                &mut oidLen as *mut crate::types::size_t,
                crate::types::MBEDTLS_ASN1_OID,
            )
        };
        if rc != 0 {
            return rc;
        }
        if oidLen == (crate::types::MBEDTLS_OID_SIZE(crate::types::MBEDTLS_OID_PKCS9_MSG_DIGEST!()) as crate::types::size_t)
            && unsafe {
                libc::memcmp(
                    p as *const ::core::ffi::c_void,
                    crate::types::MBEDTLS_OID_PKCS9_MSG_DIGEST!() as *const ::core::ffi::c_void,
                    crate::types::MBEDTLS_OID_SIZE(crate::types::MBEDTLS_OID_PKCS9_MSG_DIGEST!()) as usize,
                ) == 0
            }
        {
            p = unsafe { p.add(oidLen as usize) };
            rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut p as *mut *mut ::core::ffi::c_uchar,
                    seqEnd,
                    &mut tmpLen as *mut crate::types::size_t,
                    crate::types::MBEDTLS_ASN1_SET | crate::types::MBEDTLS_ASN1_CONSTRUCTED,
                )
            };
            if rc != 0 {
                return rc;
            }
            rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut p as *mut *mut ::core::ffi::c_uchar,
                    seqEnd,
                    &mut tmpLen as *mut crate::types::size_t,
                    crate::types::MBEDTLS_ASN1_OCTET_STRING,
                )
            };
            if rc != 0 {
                return rc;
            }
            unsafe {
                *dig = p;
                *digLen = tmpLen;
            }
            return crate::types::PKCS7_SUCC as i32;
        } else {
            p = seqEnd;
        }
    }
    crate::types::PKCS7_INVALID_VALUE as i32
}