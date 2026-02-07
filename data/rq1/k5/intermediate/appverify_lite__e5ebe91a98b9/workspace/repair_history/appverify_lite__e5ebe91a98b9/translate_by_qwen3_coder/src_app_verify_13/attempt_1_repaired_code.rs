fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::globals::*;
    let trust_cert = unsafe {
        crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const _,
            g_trustAppList.as_ptr(),
            (std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32,
        )
    };
    let mut trust_cert = trust_cert;
    unsafe {
        if g_isDebugMode && trust_cert.is_null() {
            trust_cert = crate::src_app_verify::GetProfSourceBySigningCert(
                signer as *const _,
                g_trustAppListTest.as_ptr(),
                (std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<crate::types::TrustAppCert>()) as i32,
            );
        }
    }
    if !trust_cert.is_null() {
        unsafe {
            if (*trust_cert).maxCertPath < (*signer).depth {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const _,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const _,
                    412,
                    (*signer).depth,
                );
                return crate::types::V_ERR as i32;
            }
        }
    }
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trust_cert);
    }
    crate::types::V_OK as i32
}