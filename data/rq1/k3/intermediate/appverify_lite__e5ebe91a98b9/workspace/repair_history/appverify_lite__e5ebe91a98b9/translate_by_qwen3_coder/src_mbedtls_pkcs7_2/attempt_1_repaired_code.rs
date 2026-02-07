fn GetContentInfoType(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, contentType: *mut crate::types::mbedtls_asn1_buf, hasContent: *mut crate::types::c_bool) -> i32 {
    let mut seqLen: crate::types::size_t = 0;
    let mut len: crate::types::size_t = 0;
    let rc: i32;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut seqLen, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }
    let start: *mut ::core::ffi::c_uchar = unsafe { *p };
    let end = unsafe { start.offset(seqLen as isize) };
    let rc: i32 = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x06) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*contentType).tag = 0x06;
        (*contentType).len = len;
        (*contentType).p = *p;
        let diff = (*p).offset_from(start) as usize;
        *hasContent = ((seqLen as usize) != (len as usize).wrapping_add(diff)) as i32;
        *p = (*p).offset(len as isize);
    }
    return crate::types::PKCS7_SUCC as i32;
}