fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    // g_trustAppList and g_trustAppListTest are i32 in globals
    // Cast them to pointers for use with GetProfSourceBySigningCert
    let trust_list_ptr = unsafe { crate::globals::g_trustAppList as usize as *const TrustAppCert };
    let trust_list_len = 1i32;
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        trust_list_ptr,
        trust_list_len
    );
    
    // g_isDebugMode is i32, compare with 0 for bool check
    let is_debug = unsafe { crate::globals::g_isDebugMode != 0 };
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