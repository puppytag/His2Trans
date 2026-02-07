fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    for i in 0..num {
        let trust = unsafe { &*(trustList.offset)(i as isize) };
        let signer_ref = unsafe { &*signer };
        if unsafe { libc::strcmp(trust.issueCA, signer_ref.issuer.as_ptr()) } == 0 {
            if unsafe { libc::strcmp(trust.profileSignCert, signer_ref.subject.as_ptr()) } == 0 ||
               unsafe { libc::strcmp(trust.profileDebugSignCert, signer_ref.subject.as_ptr()) } == 0 {
                let _ = unsafe { crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: profile source name : %s\0".as_ptr() as *const _,
                    b"GetProfSourceBySigningCert\0".as_ptr() as *const _,
                    393,
                    (*crate::globals::(g_trustAppList.offset)(i as isize)).name,
                ) };
                return trust;
            }
        }
    }
    std::ptr::null()
}