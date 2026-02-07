fn DLogCrtVerifyInfo(flags: u32) {
    let mut vrfyBuf: [std::ffi::c_char; 512] = [0; 512];
    unsafe {
        memset_s(
            vrfyBuf.as_mut_ptr() as *mut std::ffi::c_void,
            512,
            0,
            512,
        );
        crate::compat::mbedtls_x509_crt_verify_info(
            vrfyBuf.as_mut_ptr(),
            std::mem::size_of::<[std::ffi::c_char; 512]>() as u32,
            b" ! \0".as_ptr() as *const std::ffi::c_char,
            flags,
        );
        HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: %s\0".as_ptr() as *const std::ffi::c_char,
            b"DLogCrtVerifyInfo\0".as_ptr() as *const std::ffi::c_char,
            981i32,
            vrfyBuf.as_ptr(),
        );
    }
}