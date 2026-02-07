fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    if appCertType == crate::types::CERT_TYPE_APPGALLARY as i32 || appCertType == crate::types::CERT_TYPE_SYETEM as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: app type : %d, return OK\0".as_ptr() as *const ::core::ffi::c_char,
                b"CheckAppSignCertWithProfile\0".as_ptr() as *const ::core::ffi::c_char,
                908i32,
                appCertType,
            );
        }
        return crate::types::V_OK as i32;
    }

    let mut ret: i32 = crate::types::V_ERR as i32;

    unsafe {
        let pf_type = (*pf).type_;
        if libc::strcmp(b"debug\0".as_ptr() as *const ::core::ffi::c_char, pf_type) == 0 {
            ret = crate::src_app_verify::CheckDebugAppSign(binSignCert, pf as *const crate::types::ProfileProf);
        } else if libc::strcmp(b"release\0".as_ptr() as *const ::core::ffi::c_char, pf_type) == 0 {
            ret = crate::src_app_verify::CheckReleaseAppSign(binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf);
        }

        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: check app sign cert ret : %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"CheckAppSignCertWithProfile\0".as_ptr() as *const ::core::ffi::c_char,
            920i32,
            ret,
        );
    }

    ret
}