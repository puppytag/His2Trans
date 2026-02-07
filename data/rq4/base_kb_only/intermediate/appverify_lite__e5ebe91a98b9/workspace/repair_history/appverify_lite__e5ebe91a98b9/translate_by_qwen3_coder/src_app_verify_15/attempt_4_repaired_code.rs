fn GetAppCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::{V_OK, V_ERR, LOG_CORE, LOG_ERROR, TrustAppCert};
    
    // g_trustAppList and g_trustAppListTest are arrays of TrustAppCert in globals
    // We need to access them as pointers and get their sizes
    let g_trust_app_list_ptr: *const TrustAppCert = unsafe { crate::globals::g_trustAppList.as_ptr() };
    let g_trust_app_list_len = std::mem::size_of_val(unsafe { &crate::globals::g_trustAppList }) / std::mem::size_of::<TrustAppCert>();
    
    let mut trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
        signer as *const crate::types::SignerResovledInfo,
        g_trust_app_list_ptr,
        g_trust_app_list_len as i32
    );
    
    if unsafe { crate::globals::g_isDebugMode } && trustCert.is_null() {
        let g_trust_app_list_test_ptr: *const TrustAppCert = unsafe { crate::globals::g_trustAppListTest.as_ptr() };
        let g_trust_app_list_test_len = std::mem::size_of_val(unsafe { &crate::globals::g_trustAppListTest }) / std::mem::size_of::<TrustAppCert>();
        
        trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
            signer as *const crate::types::SignerResovledInfo,
            g_trust_app_list_test_ptr,
            g_trust_app_list_test_len as i32
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
                    b"[%s:%d]: cert maxdepth error: %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GetAppCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    443i32,
                    max_cert_path,
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