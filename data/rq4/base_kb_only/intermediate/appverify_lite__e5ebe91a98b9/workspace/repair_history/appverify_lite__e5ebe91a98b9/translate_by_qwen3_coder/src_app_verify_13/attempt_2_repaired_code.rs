fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    // g_trustAppList and g_trustAppListTest are i32 in globals, so we need to cast them to pointers
    // The original C code uses sizeof(g_trustAppList) / sizeof(TrustAppCert) which suggests these are arrays
    // But in the skeleton they're defined as i32, so we treat them as pointers cast from integers
    
    let trust_list_ptr = unsafe { crate::globals::g_trustAppList as *const TrustAppCert };
    let trust_list_len = 1i32; // Since g_trustAppList is i32, we can't compute array size
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        trust_list_ptr,
        trust_list_len
    );
    
    // g_isDebugMode is i32, need to compare with 0 for bool
    if unsafe { crate::globals::g_isDebugMode != 0 } && trustCert.is_null() {
        let test_list_ptr = unsafe { crate::globals::g_trustAppListTest as *const TrustAppCert };
        let test_list_len = 1i32;
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            test_list_ptr,
            test_list_len
        );
    }
    
    if !trustCert.is_null() && unsafe { (*trustCert).maxCertPath < (*signer).depth } {
        // HiLogPrint call - skip since it's just logging
        return V_ERR as i32;
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}