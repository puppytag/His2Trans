fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    unsafe {
        for i in 0..num {
            let trust_item = trustList.offset(i as isize);
            if libc::strcmp((*trust_item).issueCA, (*signer).issuer.as_ptr()) == 0 {
                if libc::strcmp((*trust_item).profileSignCert, (*signer).subject.as_ptr()) == 0 ||
                   libc::strcmp((*trust_item).profileDebugSignCert, (*signer).subject.as_ptr()) == 0 {
                    let _ = HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                        b"[%s:%d]: profile source name : %s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"GetProfSourceBySigningCert\0".as_ptr() as *const ::core::ffi::c_char,
                        393i32,
                        std::ptr::null::<::core::ffi::c_char>()
                    );
                    return trust_item;
                }
            }
        }
        std::ptr::null()
    }
}