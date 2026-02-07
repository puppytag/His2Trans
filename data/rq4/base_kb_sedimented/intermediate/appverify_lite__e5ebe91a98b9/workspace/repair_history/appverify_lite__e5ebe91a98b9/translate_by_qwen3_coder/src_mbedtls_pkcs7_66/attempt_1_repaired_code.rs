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