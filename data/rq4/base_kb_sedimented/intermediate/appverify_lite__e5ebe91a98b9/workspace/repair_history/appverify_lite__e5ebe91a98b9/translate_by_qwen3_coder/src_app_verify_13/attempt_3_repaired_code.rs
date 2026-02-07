fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    use crate::globals::*;
    
    // g_trustAppList and g_trustAppListTest are i32 in globals, not arrays
    // The C code uses sizeof(g_trustAppList) / sizeof(TrustAppCert) to get count
    // Since these are declared as i32 = 0 in globals, we need to treat them as pointers
    // and the count would be computed differently. Looking at the C code pattern,
    // we pass the address and a count.
    
    let trust_list_ptr = unsafe { &g_trustAppList as *const i32 as *const TrustAppCert };
    let trust_list_count = (std::mem::size_of_val(unsafe { &g_trustAppList }) / std::mem::size_of::<TrustAppCert>()) as i32;
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        trust_list_ptr,
        trust_list_count
    );
    
    if (unsafe { g_isDebugMode } != 0) && trustCert.is_null() {
        let trust_list_test_ptr = unsafe { &g_trustAppListTest as *const i32 as *const TrustAppCert };
        let trust_list_test_count = (std::mem::size_of_val(unsafe { &g_trustAppListTest }) / std::mem::size_of::<TrustAppCert>()) as i32;
        
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            trust_list_test_ptr,
            trust_list_test_count
        );
    }
    
    if !trustCert.is_null() {
        let max_cert_path = unsafe { (*trustCert).maxCertPath };
        let depth = unsafe { (*signer).depth };
        if max_cert_path < depth {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    412i32,
                    depth
                )
            };
            return V_ERR as i32;
        }
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}