fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    let trust_list_ptr = unsafe { crate::globals::g_trustAppList as usize as *const TrustAppCert };
    let trust_list_len = 1i32;
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        trust_list_ptr,
        trust_list_len
    );
    
    let is_debug: bool = unsafe { crate::globals::g_isDebugMode != 0 };
    if is_debug && trustCert.is_null() {
        let test_list_ptr = unsafe { crate::globals::g_trustAppListTest as usize as *const TrustAppCert };
        let test_list_len = 1i32;
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            test_list_ptr,
            test_list_len
        );
    }
    
    if !trustCert.is_null() && unsafe { (*trustCert).maxCertPath < (*signer).depth } {
        return V_ERR as i32;
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}