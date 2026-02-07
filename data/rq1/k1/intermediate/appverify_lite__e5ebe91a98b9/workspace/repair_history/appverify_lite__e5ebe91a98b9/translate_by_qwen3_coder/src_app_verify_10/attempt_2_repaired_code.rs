fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    let ret = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut inputLen);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get content info error: %d\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 324, ret);
        }
        return ret;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: content: len: %d\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 327, inputLen as i32);
    }
    let content = unsafe { libc::malloc(std::mem::size_of::<crate::types::ContentInfo>()) } as *mut crate::types::ContentInfo;
    if content.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: content is null\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 330);
        }
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe { crate::compat::memcpy_s(content as *mut _, std::mem::size_of::<crate::types::ContentInfo>() as u32, input as *const _, inputLen as u32) };
    if ret != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mem cpy error, ret: %d\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 334, ret);
            libc::free(content as *mut _);
        }
        return ret;
    }
    crate::src_app_verify::ContentN2H(content);
    unsafe {
        (*content).algId = crate::src_app_verify_hap::GetDigestAlgorithmId((*content).algId as u32);
    }
    unsafe {
        if (*content).algId != crate::types::MBEDTLS_MD_SHA256 as i32 &&
           (*content).algId != crate::types::MBEDTLS_MD_SHA384 as i32 &&
           (*content).algId != crate::types::MBEDTLS_MD_SHA512 as i32 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: hash alg invalid\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 341);
            libc::free(content as *mut _);
            return crate::types::V_ERR as i32;
        }
    }
    let mut actualDigest = crate::types::HapBuf { buffer: std::ptr::null_mut(), len: 0 };
    let rootHashLen = unsafe { crate::src_app_verify::GetHashUnitLen((*content).algId) };
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actualDigest, rootHashLen) {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: create buf fail\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 348);
            libc::free(content as *mut _);
        }
        return crate::types::V_ERR as i32;
    }
    let fp = unsafe { (*fileRead).fp };
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(unsafe { (*content).algId }, fp, signInfo, &actualDigest) {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get raw hash failed\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 353);
        }
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        unsafe { libc::free(content as *mut _); }
        return crate::types::V_ERR as i32;
    }
    let cmp = unsafe { libc::memcmp(actualDigest.buffer as *const _, (*content).hash.as_ptr() as *const _, actualDigest.len as usize) };
    if actualDigest.len != unsafe { (*content).length } || cmp != 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: hash diff\0".as_ptr() as *const _, b"VerifyRawHash\0".as_ptr() as *const _, 359);
            libc::free(content as *mut _);
        }
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        return crate::types::V_ERR_GET_HASH_DIFF as i32;
    }
    unsafe { libc::free(content as *mut _); }
    crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
    crate::types::V_OK as i32
}