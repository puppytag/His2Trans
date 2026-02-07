fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    unsafe {
        let mut profileData: *mut std::ffi::c_char = std::ptr::null_mut();
        let mut certType: i32 = 0;
        let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
        let mut inputLen: crate::types::size_t = 0;
        
        let pkcs7: *mut crate::types::Pkcs7 = libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) as *mut crate::types::Pkcs7;
        if pkcs7.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        let mut ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(
            buf as *const ::core::ffi::c_uchar,
            len as crate::types::size_t,
            pkcs7
        );
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const crate::types::Pkcs7);
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        ret = GetProfileSingerCertType(pkcs7, &mut certType);
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        if certType == crate::types::CERT_TYPE_OTHER as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        let calc_digest_fn: crate::types::PKCS7_CalcDigest = std::mem::transmute(CalcDigest as usize);
        ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(
            pkcs7 as *const crate::types::Pkcs7,
            calc_digest_fn
        );
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(
            pkcs7 as *const crate::types::Pkcs7,
            &mut input,
            &mut inputLen
        );
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        if inputLen > crate::types::MAX_PROFILE_SIZE || inputLen == 0 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        profileData = libc::malloc((inputLen as usize) + 1) as *mut std::ffi::c_char;
        if profileData.is_null() {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        ret = memcpy_s(
            profileData as *mut ::core::ffi::c_void,
            inputLen,
            input as *const ::core::ffi::c_void,
            inputLen
        ) as i32;
        *profileData.add(inputLen as usize) = 0;
        if ret != crate::types::V_OK as i32 {
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            libc::free(pkcs7 as *mut ::core::ffi::c_void);
            libc::free(profileData as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        libc::free(pkcs7 as *mut ::core::ffi::c_void);
        *profileContent = profileData;
        *contentLen = inputLen as i32;
        
        crate::types::V_OK as i32
    }
}