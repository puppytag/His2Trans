fn GetProfileCertTypeBySignInfo(signer: *mut crate::types::SignerResovledInfo, certType: *mut i32) -> i32 {
    use crate::types::*;
    
    let trust_list_len = (std::mem::size_of_val(unsafe { &crate::globals::g_trustAppList }) 
        / std::mem::size_of::<TrustAppCert>()) as i32;
    
    let mut trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
        signer as *const SignerResovledInfo,
        unsafe { crate::globals::(g_trustAppList.as_ptr)() },
        trust_list_len
    );
    
    if unsafe { crate::globals::g_isDebugMode } && trustCert.is_null() {
        let test_list_len = (std::mem::size_of_val(unsafe { &crate::globals::g_trustAppListTest })
            / std::mem::size_of::<TrustAppCert>()) as i32;
        trustCert = crate::src_app_verify::GetProfSourceBySigningCert(
            signer as *const SignerResovledInfo,
            unsafe { crate::globals::(g_trustAppListTest.as_ptr)() },
            test_list_len
        );
    }
    
    if !trustCert.is_null() && unsafe { (*trustCert).maxCertPath < (*signer).depth } {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: cert maxdepth error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetProfileCertTypeBySignInfo\0".as_ptr() as *const ::core::ffi::c_char,
                412i32,
                (*signer).depth
            );
        }
        return V_ERR as i32;
    }
    
    unsafe {
        *certType = crate::src_app_verify::GetCertTypeBySourceName(trustCert);
    }
    
    V_OK as i32
}