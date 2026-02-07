fn VerifyRawHash(signInfo: *const crate::types::SignatureInfo, fileRead: *const crate::types::FileRead, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    use crate::types::*;
    
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: size_t = 0;
    
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7Handle, &mut input, &mut inputLen) };
    if ret != V_OK as i32 {
        return ret;
    }
    
    let content: *mut ContentInfo = unsafe { libc::malloc(std::mem::size_of::<ContentInfo>()) as *mut ContentInfo };
    if content.is_null() {
        return V_ERR as i32;
    }
    
    let ret = unsafe { memcpy_s(content as *mut ::core::ffi::c_void, std::mem::size_of::<ContentInfo>() as u32, input as *const ::core::ffi::c_void, inputLen as u32) };
    if ret != 0 {
        unsafe { libc::free(content as *mut ::core::ffi::c_void) };
        return ret;
    }
    
    crate::src_app_verify::ContentN2H(content);
    
    unsafe {
        (*content).algId = crate::src_app_verify_hap::GetDigestAlgorithmId((*content).algId as u32);
        
        if (*content).algId != MBEDTLS_MD_SHA256 as i32 && 
           (*content).algId != MBEDTLS_MD_SHA384 as i32 && 
           (*content).algId != MBEDTLS_MD_SHA512 as i32 {
            libc::free(content as *mut ::core::ffi::c_void);
            return V_ERR as i32;
        }
    }
    
    let mut actualDigest = HapBuf {
        buffer: std::ptr::null_mut(),
        len: 0,
    };
    
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(unsafe { (*content).algId });
    
    if !crate::src_app_centraldirectory::CreateHapBuffer(&mut actualDigest, rootHashLen) {
        unsafe { libc::free(content as *mut ::core::ffi::c_void) };
        return V_ERR as i32;
    }
    
    let fp = unsafe { (*fileRead).fp };
    
    if !crate::src_app_verify_hap::VerifyIntegrityChunk(unsafe { (*content).algId }, fp, signInfo, &actualDigest) {
        crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
        unsafe { libc::free(content as *mut ::core::ffi::c_void) };
        return V_ERR as i32;
    }
    
    unsafe {
        let hash_ptr = (*content).hash.as_ptr() as *const ::core::ffi::c_void;
        if (actualDigest.len != (*content).length) || 
           (libc::memcmp(actualDigest.buffer, hash_ptr, actualDigest.len as usize) != 0) {
            libc::free(content as *mut ::core::ffi::c_void);
            crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
            return V_ERR_GET_HASH_DIFF as i32;
        }
    }
    
    unsafe { libc::free(content as *mut ::core::ffi::c_void) };
    crate::src_app_centraldirectory::ClearHapBuffer(&mut actualDigest);
    
    V_OK as i32
}