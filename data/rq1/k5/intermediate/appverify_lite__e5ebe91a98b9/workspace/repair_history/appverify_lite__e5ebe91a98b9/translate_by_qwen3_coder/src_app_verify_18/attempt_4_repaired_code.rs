extern "C" fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    let mut profile_data: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut cert_type: i32 = 0;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut input_len: crate::types::size_t = 0;
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 496) };
        return crate::types::V_ERR as i32;
    }
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 499) };
        goto_exit;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 parse message success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 501) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const crate::types::Pkcs7);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 505) };
        goto_exit;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Verify certs success\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 507) };
    ret = crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut cert_type as *mut i32);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 510) };
        goto_exit;
    }
    if cert_type == crate::types::CERT_TYPE_OTHER as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert type invalid\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 513) };
        ret = crate::types::V_ERR as i32;
        goto_exit;
    }
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const crate::types::Pkcs7, Some(crate::src_app_verify::CalcDigest));
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 518) };
        goto_exit;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 519) };
    ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const crate::types::Pkcs7, &mut input as *mut *mut ::core::ffi::c_uchar, &mut input_len as *mut crate::types::size_t);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 523) };
        goto_exit;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get profile sign content ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 525) };
    if input_len > crate::types::MAX_PROFILE_SIZE as crate::types::size_t || input_len == 0 {
        ret = crate::types::V_ERR as i32;
        goto_exit;
    }
    profile_data = unsafe { libc::malloc((input_len + 1) as usize) } as *mut std::ffi::c_char;
    if profile_data.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profileData is null\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 532) };
        goto_exit;
    }
    ret = unsafe { crate::compat::memcpy_s(profile_data as *mut std::ffi::c_void, input_len as u32, input as *const std::ffi::c_void, input_len as u32) };
    unsafe { *profile_data.offset(input_len as isize) = 0 };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 536) };
        goto_exit;
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
    }
    unsafe { *profileContent = profile_data };
    unsafe { *contentLen = input_len as i32 };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify profile get raw data ok\0".as_ptr() as *const _, b"VerifyProfileSignGetRaw\0".as_ptr() as *const _, 542) };
    return crate::types::V_OK as i32;
    macro_rules! goto_exit {
        () => {
            {
                crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
                if !pkcs7.is_null() {
                    unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
                }
                if !profile_data.is_null() {
                    unsafe { libc::free(profile_data as *mut std::ffi::c_void) };
                }
                return crate::types::V_ERR as i32;
            }
        };
    }
    goto_exit;
}