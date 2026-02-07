fn CheckAppSignCertWithProfile(appCertType: i32, binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    if appCertType == 0 || appCertType == 1 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as i32,
            crate::types::LOG_INFO as i32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: app type : %d, return OK\0".as_ptr() as *const _,
            b"CheckAppSignCertWithProfile\0".as_ptr() as *const _,
            908,
            appCertType,
        ) };
        return crate::types::V_OK as i32;
    }

    let mut ret = crate::types::V_ERR as i32;

    unsafe {
        let type_ptr = (*pf).type_;
        if !type_ptr.is_null() {
            if libc::strcmp(type_ptr, b"debug\0".as_ptr() as *const _) == 0 {
                ret = crate::src_app_verify::CheckDebugAppSign(binSignCert as *mut crate::types::CertInfo, pf);
            } else if libc::strcmp(type_ptr, b"release\0".as_ptr() as *const _) == 0 {
                ret = crate::src_app_verify::CheckReleaseAppSign(binSignCert, pf);
            }
        }
    }

    let _ = unsafe { crate::compat::HiLogPrint(
        crate::types::LOG_CORE as i32,
        crate::types::LOG_INFO as i32,
        0xD001100,
        b"appverify\0".as_ptr() as *const _,
        b"[%s:%d]: check app sign cert ret : %d\0".as_ptr() as *const _,
        b"CheckAppSignCertWithProfile\0".as_ptr() as *const _,
        920,
        ret,
    ) };
    ret
}