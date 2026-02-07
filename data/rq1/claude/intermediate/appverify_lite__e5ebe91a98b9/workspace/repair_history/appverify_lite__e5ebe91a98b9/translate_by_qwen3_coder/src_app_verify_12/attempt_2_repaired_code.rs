fn GetProfSourceBySigningCert(signer: *const crate::types::SignerResovledInfo, trustList: *const crate::types::TrustAppCert, num: i32) -> *const crate::types::TrustAppCert {
    unsafe {
        for i in 0..num {
            let trust_item = trustList.offset(i as isize);
            if libc::strcmp((*trust_item).issueCA, (*signer).issuer.as_ptr()) == 0 {
                if libc::strcmp((*trust_item).profileSignCert, (*signer).subject.as_ptr()) == 0 ||
                   libc::strcmp((*trust_item).profileDebugSignCert, (*signer).subject.as_ptr()) == 0 {
                    return trust_item;
                }
            }
        }
        std::ptr::null()
    }
}