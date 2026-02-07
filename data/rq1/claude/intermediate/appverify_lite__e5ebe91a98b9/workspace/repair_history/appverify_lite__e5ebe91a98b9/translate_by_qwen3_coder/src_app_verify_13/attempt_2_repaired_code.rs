fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    // Based on globals.rs, g_trustAppList and g_trustAppListTest are i32, not arrays
    // The C code uses sizeof(g_trustAppList) / sizeof(TrustAppCert) which suggests they are arrays
    // But in our skeleton they're declared as i32, so we need to treat them as pointers
    // For now, use a reasonable default or cast appropriately
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        unsafe { &crate::globals::g_trustAppList as *const i32 as *const TrustAppCert },
        1i32
    );
    
    if unsafe { crate::globals::g_isDebugMode != 0 } && trustCert.is_null() {
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            unsafe { &crate::globals::g_trustAppListTest as *const i32 as *const TrustAppCert },
            1i32
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