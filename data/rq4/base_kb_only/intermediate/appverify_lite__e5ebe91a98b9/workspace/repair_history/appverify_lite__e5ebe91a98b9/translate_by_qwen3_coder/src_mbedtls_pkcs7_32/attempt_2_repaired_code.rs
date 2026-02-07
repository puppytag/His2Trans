fn FreeSignersInfo(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = (*pkcs7).signedData.signers.next;
        let mut next: *mut crate::types::SignerInfo = std::ptr::null_mut();

        while !signer.is_null() {
            next = (*signer).next;
            crate::src_mbedtls_pkcs7::FreeSignerCerts(signer);
            crate::src_mbedtls_pkcs7::FreeSignerIssuer(signer);
            crate::src_mbedtls_pkcs7::Pkcs7Free(signer as *mut std::ffi::c_void);
            signer = next;
        }
        (*pkcs7).signedData.signers.next = std::ptr::null_mut();
        crate::src_mbedtls_pkcs7::FreeSignerCerts(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
        crate::src_mbedtls_pkcs7::FreeSignerIssuer(&mut (*pkcs7).signedData.signers as *mut crate::types::SignerInfo);
    }
}