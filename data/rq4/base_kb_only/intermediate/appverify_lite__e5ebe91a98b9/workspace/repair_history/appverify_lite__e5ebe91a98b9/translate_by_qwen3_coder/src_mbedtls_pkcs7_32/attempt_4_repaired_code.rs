fn FreeSignersInfo(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = (*pkcs7).signedData.signers.next;
        let mut next: *mut crate::types::SignerInfo = std::ptr::null_mut();

        while !signer.is_null() {
            next = (*signer).next;
            FreeSignerCerts(signer);
            FreeSignerIssuer(signer);
            Pkcs7Free(signer as *mut ::core::ffi::c_void);
            signer = next;
        }
        (*pkcs7).signedData.signers.next = std::ptr::null_mut();
        FreeSignerCerts(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
        FreeSignerIssuer(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
    }
}