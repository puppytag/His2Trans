fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    let trust_app_list_len = unsafe {
        std::mem::size_of_val(&crate::globals::g_trustAppList) / std::mem::size_of::<TrustAppCert>()
    };
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        unsafe { crate::globals::(g_trustAppList.as_ptr)() },
        trust_app_list_len as i32
    );
    
    if unsafe { crate::globals::g_isDebugMode } && trustCert.is_null() {
        let trust_app_list_test_len = unsafe {
            std::mem::size_of_val(&crate::globals::g_trustAppListTest) / std::mem::size_of::<TrustAppCert>()
        };
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            unsafe { crate::globals::(g_trustAppListTest.as_ptr)() },
            trust_app_list_test_len as i32
        );
    }
    
    if !trustCert.is_null() {
        let max_cert_path = unsafe { (*trustCert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    412i32,
                    depth
                );
            }
            return V_ERR as i32;
        }
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}