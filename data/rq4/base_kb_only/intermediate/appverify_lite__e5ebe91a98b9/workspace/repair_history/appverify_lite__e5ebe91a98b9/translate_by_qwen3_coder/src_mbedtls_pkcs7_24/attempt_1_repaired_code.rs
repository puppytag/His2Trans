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
            (oid_data.as_ptr)() as *const core::ffi::c_void,
            content_oid.p as *const core::ffi::c_void,
            content_oid.len as usize,
        ) } != 0);

    if oid_mismatch || !hasContent {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Invalid content type or has no real content\0".as_ptr() as *const i8,
                b"ParseSignedDataContentInfo\0".as_ptr() as *const i8,
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