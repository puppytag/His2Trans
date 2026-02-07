fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    let trust_cert = unsafe {
        crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const crate::types::SignerResovledInfo,
            g_trustAppList.as_ptr(),
            (g_trustAppList.len() as i32),
        )
    };
    let mut trust_cert = trust_cert;
    unsafe {
        if g_isDebugMode && trust_cert.is_null() {
            trust_cert = crate::src_app_verify::GetProfSourceBySigningCert(
                signer as *const crate::types::SignerResovledInfo,
                g_trustAppListTest.as_ptr(),
                (g_trustAppListTest.len() as i32),
            );
        }
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = HiLogPrint(
                    LOG_CORE as u32,
                    LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    412,
                    (*signer).depth,
                );
                return V_ERR as i32;
            }
        }
    }
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trust_cert);
    }
    V_OK as i32
}