fn GetBinSignPkcs(signBuf: *const c_char, len: i32) -> *mut crate::types::Pkcs7 {
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const c_char,
                b"[%s:%d]: malloc error\0".as_ptr() as *const c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const c_char,
                1066i32,
            );
        }
        return std::ptr::null_mut();
    }
    
    let ret = unsafe {
        PKCS7_ParseSignedData(
            signBuf as *const ::core::ffi::c_uchar,
            len as crate::types::size_t,
            pkcs7,
        )
    };
    
    if ret != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const c_char,
                b"[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const c_char,
                b"GetBinSignPkcs\0".as_ptr() as *const c_char,
                1071i32,
                ret,
            );
            PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    pkcs7
}