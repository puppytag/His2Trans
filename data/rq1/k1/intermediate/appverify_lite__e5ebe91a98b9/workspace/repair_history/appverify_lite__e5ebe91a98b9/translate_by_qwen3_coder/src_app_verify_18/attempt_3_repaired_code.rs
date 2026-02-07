extern "C" fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    let mut profileData: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: pkcs7 is null\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 496) };
        return crate::types::V_ERR as i32;
    }
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 499) };
        goto_exit(pkcs7, profileData);
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: pkcs7 parse message success\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 501) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const crate::types::Pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 505) };
        goto_exit(pkcs7, profileData);
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: Verify certs success\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 507) };
    ret = crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut certType as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 510) };
        goto_exit(pkcs7, profileData);
    }
    if certType == crate::types::CERT_TYPE_OTHER as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: cert type invalid\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 513) };
        ret = crate::types::V_ERR as i32;
        goto_exit(pkcs7, profileData);
    }
    let calc_digest: crate::types::PKCS7_CalcDigest = Some(crate::src_app_verify::CalcDigest);
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const crate::types::Pkcs7, calc_digest);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 518) };
        goto_exit(pkcs7, profileData);
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: verify profile ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 519) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const crate::types::Pkcs7, &mut input as *mut *mut ::core::ffi::c_uchar, &mut inputLen as *mut crate::types::size_t);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 523) };
        goto_exit(pkcs7, profileData);
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: get profile sign content ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 525) };
    if inputLen > (1024 * 1024) as crate::types::size_t || inputLen == 0 {
        ret = crate::types::V_ERR as i32;
        goto_exit(pkcs7, profileData);
    }
    profileData = unsafe { libc::malloc((inputLen + 1) as usize) } as *mut std::ffi::c_char;
    if profileData.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: profileData is null\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 532) };
        goto_exit(pkcs7, profileData);
    }
    ret = unsafe { crate::compat::memcpy_s(profileData as *mut std::ffi::c_void, inputLen as u64, input as *const std::ffi::c_void, inputLen as u64) };
    unsafe { *profileData.offset(inputLen as isize) = 0 };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 536) };
        goto_exit(pkcs7, profileData);
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    }
    unsafe { *profileContent = profileData };
    unsafe { *contentLen = inputLen as i32 };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: verify profile get raw data ok\0".as_ptr() as *const std::ffi::c_char, "VerifyProfileSignGetRaw\0".as_ptr() as *const std::ffi::c_char, 542) };
    return crate::types::V_OK as i32;
}

extern "C" fn goto_exit(pkcs7: *mut crate::types::Pkcs7, profileData: *mut std::ffi::c_char) -> i32 {
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    }
    if !profileData.is_null() {
        unsafe { libc::free(profileData as *mut std::ffi::c_void) };
    }
    return crate::types::V_ERR as i32;
}