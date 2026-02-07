fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let dev_cert = (*pf).bundleInfo.devCert;
        if libc::strlen(dev_cert as *const i8) == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: debug app, devCert null\0".as_ptr() as *const i8,
                b"CheckDebugAppSign\0".as_ptr() as *const i8,
                885i32,
            );
            return crate::types::V_ERR as i32;
        }
        
        let ret = crate::src_app_verify::LoadCertAndCmpDest(dev_cert, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: dev cert consistent\0".as_ptr() as *const i8,
                b"CheckDebugAppSign\0".as_ptr() as *const i8,
                890i32,
            );
            return crate::types::V_OK as i32;
        }
        
        let release_cert = (*pf).bundleInfo.releaseCert;
        if libc::strlen(release_cert as *const i8) != 0 {
            let ret2 = crate::src_app_verify::LoadCertAndCmpDest(release_cert, binSignCert as *const crate::types::CertInfo);
            if ret2 == crate::types::V_OK as i32 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: release cert consistent\0".as_ptr() as *const i8,
                    b"CheckDebugAppSign\0".as_ptr() as *const i8,
                    896i32,
                );
                return crate::types::V_OK as i32;
            }
        }
        
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: app sign cert not consistent with profile cert\0".as_ptr() as *const i8,
            b"CheckDebugAppSign\0".as_ptr() as *const i8,
            900i32,
        );
        crate::types::V_ERR as i32
    }
}