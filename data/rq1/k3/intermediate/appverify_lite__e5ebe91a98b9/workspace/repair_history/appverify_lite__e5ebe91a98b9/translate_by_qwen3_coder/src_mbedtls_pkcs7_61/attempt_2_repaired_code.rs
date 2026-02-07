pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    let mut sri: *mut crate::types::SignersResovedInfo = std::ptr::null_mut();
    if pkcs7.is_null() {
        return std::ptr::null_mut();
    }
    let signers_cnt = crate::src_mbedtls_pkcs7::GetSignersCnt(unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo);
    if signers_cnt == 0 {
        return std::ptr::null_mut();
    }
    sri = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignersResovedInfo>() as crate::types::size_t) as *mut crate::types::SignersResovedInfo;
    if sri.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*sri).nrOfSigners = signers_cnt as i32;
    }
    unsafe {
        (*sri).signers = crate::src_mbedtls_pkcs7::Pkcs7Calloc(signers_cnt as crate::types::size_t, std::mem::size_of::<crate::types::SignerResovledInfo>() as crate::types::size_t) as *mut crate::types::SignerResovledInfo;
    }
    if unsafe { (*sri).signers.is_null() } {
        crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut std::ffi::c_void);
        return std::ptr::null_mut();
    }
    let mut rc: i32;
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo;
    let mut idx: i32 = 0;
    while !signer.is_null() && idx < signers_cnt as i32 {
        rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertSubject(
            signer,
            unsafe { &mut (*((*sri).signers.offset(idx as isize))).subject[0] } as *mut ::core::ffi::c_char,
            std::mem::size_of::<[::core::ffi::c_char; 512usize]>() as crate::types::size_t,
        );
        if rc != 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return std::ptr::null_mut();
        }
        rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerSignningCertIssuer(
            signer,
            unsafe { &mut (*((*sri).signers.offset(idx as isize))).issuer[0] } as *mut ::core::ffi::c_char,
            std::mem::size_of::<[::core::ffi::c_char; 512usize]>() as crate::types::size_t,
        );
        if rc != 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
            return std::ptr::null_mut();
        }
        unsafe {
            (*((*sri).signers.offset(idx as isize))).depth = crate::src_mbedtls_pkcs7::GetSignerSignningCertDepth(signer);
        }
        signer = unsafe { (*signer).next } as *const crate::types::SignerInfo;
        idx += 1;
    }
    return sri;
}