fn VerifyProfileSignGetRaw(buf: *const std::ffi::c_char, len: i32, profileContent: *mut *mut std::ffi::c_char, contentLen: *mut i32) -> i32 {
    use crate::types::*;
    
    let mut profileData: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let mut input: *mut u8 = std::ptr::null_mut();
    let mut inputLen: size_t = 0;
    
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<Pkcs7>()) as *mut Pkcs7 };
    if pkcs7.is_null() {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"pkcs7\" is null\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 496) };
        return V_ERR as i32;
    }
    
    let mut ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(buf as *const u8, len as size_t, pkcs7) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 499) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"pkcs7 parse message success\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 501) };
    
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7 as *const Pkcs7) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 505) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"Verify certs success\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 507) };
    
    ret = crate::src_app_verify::GetProfileSingerCertType(pkcs7, &mut certType);
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 510) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    if certType == CERT_TYPE_OTHER as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"cert type invalid\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 513) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    let calc_digest_fn: PKCS7_CalcDigest = Some(CalcDigest);
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7 as *const Pkcs7, calc_digest_fn) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 518) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"verify profile ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 519) };
    
    ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7 as *const Pkcs7, &mut input, &mut inputLen) };
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 523) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"get profile sign content ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 525) };
    
    if inputLen > MAX_PROFILE_SIZE || inputLen == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    profileData = unsafe { libc::malloc((inputLen as usize) + 1) as *mut std::ffi::c_char };
    if profileData.is_null() {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"profileData\" is null\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 532) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
        return V_ERR as i32;
    }
    
    ret = unsafe { memcpy_s(profileData as *mut _, inputLen as usize, input as *const _, inputLen as usize) };
    unsafe { *profileData.add(inputLen as usize) = 0; }
    if ret != V_OK as i32 {
        unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 536) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); libc::free(profileData as *mut _); }
        return V_ERR as i32;
    }
    
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7); libc::free(pkcs7 as *mut _); }
    unsafe { *profileContent = profileData; *contentLen = inputLen as i32; }
    unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: \"verify profile get raw data ok\0".as_ptr() as *const i8, b"VerifyProfileSignGetRaw\0".as_ptr() as *const i8, 542) };
    V_OK as i32
}