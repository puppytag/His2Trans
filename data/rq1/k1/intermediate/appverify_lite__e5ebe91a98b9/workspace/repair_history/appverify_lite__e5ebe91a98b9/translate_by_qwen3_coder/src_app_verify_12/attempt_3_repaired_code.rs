fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    for i in 0..num {
        unsafe {
            let trust = &*trustList.offset(i as isize);
            let sign = &*signer;
            if libc::strcmp(trust.issueCA, sign.issuer.as_ptr() as *const i8) == 0 {
                if libc::strcmp(trust.profileSignCert, sign.subject.as_ptr() as *const i8) == 0 ||
                   libc::strcmp(trust.profileDebugSignCert, sign.subject.as_ptr() as *const i8) == 0 {
                    let _ = crate::compat::HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        "appverify\0".as_ptr() as *const i8,
                        "[%s:%d]: profile source name : %s\0".as_ptr() as *const i8,
                        "GetProfSourceBySigningCert\0".as_ptr() as *const i8,
                        393,
                        std::ptr::null_mut(),
                    );
                    return trust;
                }
            }
        }
    }
    std::ptr::null()
}