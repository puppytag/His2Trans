fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::src_app_verify::GetAppSourceBySigningCert;
    use crate::src_app_verify::GetCertTypeBySourceName;
    let g_trust_app_list_len = unsafe {
        (std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
    };
    let mut trust_cert = unsafe {
        GetAppSourceBySigningCert(signer, g_trustAppList.as_ptr(), g_trust_app_list_len)
    };
    if unsafe { g_isDebugMode } && trust_cert.is_null() {
        let g_trust_app_list_test_len = unsafe {
            (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
        };
        trust_cert = unsafe {
            GetAppSourceBySigningCert(signer, g_trustAppListTest.as_ptr(), g_trust_app_list_test_len)
        };
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const _,
                    b"GetAppCertTypeBySignInfo\0".as_ptr() as *const _,
                    443,
                    (*trust_cert).maxCertPath,
                    (*signer).depth,
                );
                return crate::types::V_ERR as i32;
            }
        }
    }
    unsafe {
        *certType = GetCertTypeBySourceName(trust_cert);
    }
    crate::types::V_OK as i32
}