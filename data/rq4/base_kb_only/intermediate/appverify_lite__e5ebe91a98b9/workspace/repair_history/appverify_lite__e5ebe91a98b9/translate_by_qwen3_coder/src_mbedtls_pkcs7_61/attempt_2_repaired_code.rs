pub extern "C" fn PKCS7_GetAllSignersResolvedInfo(pkcs7: *const crate::types::Pkcs7) -> *mut crate::types::SignersResovedInfo {
    if pkcs7.is_null() {
        return std::ptr::null_mut();
    }

    let signers_cnt = unsafe { GetSignersCnt(&(*pkcs7).signedData.signers as *const crate::types::SignerInfo) };
    if signers_cnt == 0 {
        return std::ptr::null_mut();
    }

    let sri = Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignersResovedInfo>() as crate::types::size_t)
        as *mut crate::types::SignersResovedInfo;
    if sri.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        (*sri).nrOfSigners = signers_cnt as i32;
        (*sri).signers = Pkcs7Calloc(
            signers_cnt,
            std::mem::size_of::<crate::types::SignerResovledInfo>() as crate::types::size_t,
        ) as *mut crate::types::SignerResovledInfo;

        if (*sri).signers.is_null() {
            Pkcs7Free(sri as *mut std::ffi::c_void);
            return std::ptr::null_mut();
        }

        let mut signer: *const crate::types::SignerInfo = &(*pkcs7).signedData.signers;
        let mut idx: i32 = 0;

        while !signer.is_null() && (idx as crate::types::size_t) < signers_cnt {
            let signer_info = (*sri).signers.offset(idx as isize);

            let rc = PKCS7_GetSignerSignningCertSubject(
                signer,
                (*signer_info).subject.as_mut_ptr(),
                std::mem::size_of_val(&(*signer_info).subject) as crate::types::size_t,
            );
            if rc != 0 {
                PKCS7_FreeAllSignersResolvedInfo(sri);
                return std::ptr::null_mut();
            }

            let rc = PKCS7_GetSignerSignningCertIssuer(
                signer,
                (*signer_info).issuer.as_mut_ptr(),
                std::mem::size_of_val(&(*signer_info).issuer) as crate::types::size_t,
            );
            if rc != 0 {
                PKCS7_FreeAllSignersResolvedInfo(sri);
                return std::ptr::null_mut();
            }

            (*signer_info).depth = GetSignerSignningCertDepth(signer);

            signer = (*signer).next;
            idx += 1;
        }

        sri
    }
}