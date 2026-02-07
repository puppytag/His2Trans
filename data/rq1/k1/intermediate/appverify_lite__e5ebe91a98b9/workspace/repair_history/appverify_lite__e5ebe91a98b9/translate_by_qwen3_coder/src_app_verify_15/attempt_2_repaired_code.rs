fn GetAppCertTypeBySignInfo(signer: *const crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::src_app_verify::GetAppSourceBySigningCert;
    use crate::src_app_verify::GetCertTypeBySourceName;
    let g_trust_app_list_size = unsafe {
        (std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
    };
    let mut trust_cert = GetAppSourceBySigningCert(signer, unsafe { g_trustAppList.as_ptr() }, g_trust_app_list_size);
    if unsafe { g_isDebugMode } && trust_cert.is_null() {
        let g_trust_app_list_test_size = unsafe {
            (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32
        };
        trust_cert = GetAppSourceBySigningCert(signer, unsafe { g_trustAppListTest.as_ptr() }, g_trust_app_list_test_size);
    }
    if !trust_cert.is_null() {
        let max_cert_path = unsafe { (*trust_cert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const _, b"GetAppCertTypeBySignInfo\0".as_ptr() as *const _, 443, max_cert_path, depth) };
            return V_ERR as i32;
        }
    }
    unsafe { *certType = GetCertTypeBySourceName(trust_cert) };
    V_OK as i32
}