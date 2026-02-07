fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::LoadCertAndCmpDest;
    unsafe {
        if libc::strlen((*pf).bundleInfo.devCert as *const i8) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: debug app, devCert null\0".as_ptr() as *const i8,
                __FUNCTION__,
                885,
            );
            return crate::types::V_ERR as i32;
        }
        let mut ret = LoadCertAndCmpDest((*pf).bundleInfo.devCert, binSignCert);
        if ret == crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: dev cert consistent\0".as_ptr() as *const i8,
                __FUNCTION__,
                890,
            );
            return crate::types::V_OK as i32;
        }
        if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) != 0 {
            ret = LoadCertAndCmpDest((*pf).bundleInfo.releaseCert, binSignCert);
            if ret == crate::types::V_OK as i32 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: release cert consistent\0".as_ptr() as *const i8,
                    __FUNCTION__,
                    896,
                );
                return crate::types::V_OK as i32;
            }
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: app sign cert not consistent with profile cert\0".as_ptr() as *const i8,
            __FUNCTION__,
            900,
        );
        crate::types::V_ERR as i32
    }
}