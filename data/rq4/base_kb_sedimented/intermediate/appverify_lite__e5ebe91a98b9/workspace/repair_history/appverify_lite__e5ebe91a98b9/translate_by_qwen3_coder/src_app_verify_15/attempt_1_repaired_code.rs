fn GetAppCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::{V_OK, V_ERR, LOG_CORE, LOG_ERROR, TrustAppCert};
    use crate::globals::*;
    
    let trust_list_len = unsafe {
        std::mem::size_of_val(&g_trustAppList) / std::mem::size_of::<TrustAppCert>()
    };
    
    let mut trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
        signer as *const crate::types::SignerResovledInfo,
        unsafe { (g_trustAppList.as_ptr)() },
        trust_list_len as i32
    );
    
    if unsafe { g_isDebugMode } && trustCert.is_null() {
        let test_list_len = unsafe {
            std::mem::size_of_val(&g_trustAppListTest) / std::mem::size_of::<TrustAppCert>()
        };
        trustCert = crate::src_app_verify::GetAppSourceBySigningCert(
            signer as *const crate::types::SignerResovledInfo,
            unsafe { (g_trustAppListTest.as_ptr)() },
            test_list_len as i32
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