pub extern "C" fn PKCS7_GetDigestInSignerAuthAttr(signer: *const crate::types::SignerInfo, dig: *mut *mut ::core::ffi::c_uchar, digLen: *mut crate::types::size_t) -> i32 {
    if signer.is_null() || dig.is_null() || digLen.is_null() {
        return crate::types::PKCS7_INVALID_VALUE as i32;
    }
    let p = unsafe { (*signer).authAttr.p };
    if p.is_null() {
        return crate::types::PKCS7_HAS_NO_AUTH_ATTR_IN_SIGNER as i32;
    }
    let end = unsafe { p.offset((*signer).authAttr.len as isize) };
    let mut tmpLen: crate::types::size_t = 0;
    let mut current = p;
    while current < end {
        let mut seqLen: crate::types::size_t = 0;
        let mut seqEnd: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
        let rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut current as *mut *mut ::core::ffi::c_uchar,
                end,
                &mut seqLen as *mut crate::types::size_t,
                (crate::types::MBEDTLS_ASN1_SEQUENCE | crate::types::MBEDTLS_ASN1_CONSTRUCTED) as i32,
            )
        };
        if rc != 0 {
            return rc;
        }
        seqEnd = unsafe { current.offset(seqLen as isize) };
        let mut oidLen: crate::types::size_t = 0;
        let rc = unsafe {
            crate::compat::mbedtls_asn1_get_tag(
                &mut current as *mut *mut ::core::ffi::c_uchar,
                seqEnd,
                &mut oidLen as *mut crate::types::size_t,
                crate::types::MBEDTLS_ASN1_OID as i32,
            )
        };
        if rc != 0 {
            return rc;
        }
        if oidLen == (13 as crate::types::size_t) &&
            unsafe {
                libc::memcmp(
                    current as *const ::core::ffi::c_void,
                    b"\x2a\x86\x48\x86\xf7\x0d\x01\x09\x04" as *const _ as *const ::core::ffi::c_void,
                    oidLen as usize,
                ) == 0
            } {
            current = unsafe { current.offset(oidLen as isize) };
            let rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut current as *mut *mut ::core::ffi::c_uchar,
                    seqEnd,
                    &mut tmpLen as *mut crate::types::size_t,
                    (crate::types::MBEDTLS_ASN1_SET | crate::types::MBEDTLS_ASN1_CONSTRUCTED) as i32,
                )
            };
            if rc != 0 {
                return rc;
            }
            let rc = unsafe {
                crate::compat::mbedtls_asn1_get_tag(
                    &mut current as *mut *mut ::core::ffi::c_uchar,
                    seqEnd,
                    &mut tmpLen as *mut crate::types::size_t,
                    crate::types::MBEDTLS_ASN1_OCTET_STRING as i32,
                )
            };
            if rc != 0 {
                return rc;
            }
            unsafe {
                *dig = current;
                *digLen = tmpLen;
            }
            return crate::types::PKCS7_SUCC as i32;
        } else {
            current = seqEnd;
        }
    }
    crate::types::PKCS7_INVALID_VALUE as i32
}