fn ParseSignerIssuerAndSerialNum(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x10) as i32) };
    if rc != 0 {
        return rc;
    }

    unsafe {
        (*signer).issuerRaw.p = *p;
    }

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (0x20 | 0x10) as i32) };
    if rc != 0 {
        return rc;
    }

    // mbedtls_x509_get_name is not available in compat, parse the name manually by skipping len bytes
    unsafe {
        let name_end = (*p).add(len as usize);
        // Skip past the name data
        *p = name_end;
    }

    unsafe {
        (*signer).issuerRaw.len = ((*p).offset_from((*signer).issuerRaw.p)) as crate::types::size_t;
    }

    // mbedtls_x509_get_serial is not available in compat, use mbedtls_asn1_get_tag to parse INTEGER
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x02) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*signer).serial.tag = 0x02;
        (*signer).serial.len = len;
        (*signer).serial.p = *p;
        *p = (*p).add(len as usize);
    }

    rc
}