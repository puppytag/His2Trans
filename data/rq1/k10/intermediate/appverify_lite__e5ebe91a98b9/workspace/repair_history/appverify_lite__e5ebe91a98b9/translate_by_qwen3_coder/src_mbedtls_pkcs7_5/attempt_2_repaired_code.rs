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