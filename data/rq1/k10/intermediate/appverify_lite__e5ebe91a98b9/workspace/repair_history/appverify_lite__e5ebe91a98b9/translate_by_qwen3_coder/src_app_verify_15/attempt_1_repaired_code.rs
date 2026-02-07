fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::src_app_verify::*;
    let g_trust_app_list_len = (std::mem::size_of::<[crate::types::TrustAppCert; 1]>() / std::mem::size_of::<crate::types::TrustAppCert>()) as i32;
    let mut trust_cert = GetAppSourceBySigningCert(signer, g_trustAppList, g_trust_app_list_len);
    if g_isDebugMode && trust_cert.is_null() {
        let g_trust_app_list_test_len = (std::mem::size_of::<[crate::types::TrustAppCert; 1]>() / std::mem::size_of::<crate::types::TrustAppCert>()) as i32;
        trust_cert = GetAppSourceBySigningCert(signer, g_trustAppListTest, g_trust_app_list_test_len);
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const i8, __FUNCTION__!(), 443, (*trust_cert).maxCertPath, (*signer).depth);
                return V_ERR;
            }
        }
    }
    unsafe {
        *certType = GetCertTypeBySourceName(trust_cert);
    }
    V_OK
}