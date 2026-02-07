fn FreeSignedDataDigestAlgs(pkcs7: *mut crate::types::Pkcs7) {
    unsafe {
        let mut alg: *mut crate::types::DigestAlgId = (*pkcs7).signedData.digestAlgIds.next;
        let mut next: *mut crate::types::DigestAlgId = std::ptr::null_mut();

        while !alg.is_null() {
            next = (*alg).next;
            crate::src_mbedtls_pkcs7::Pkcs7Free(alg as *mut ::core::ffi::c_void);
            alg = next;
        }
        (*pkcs7).signedData.digestAlgIds.next = std::ptr::null_mut();
    }
}