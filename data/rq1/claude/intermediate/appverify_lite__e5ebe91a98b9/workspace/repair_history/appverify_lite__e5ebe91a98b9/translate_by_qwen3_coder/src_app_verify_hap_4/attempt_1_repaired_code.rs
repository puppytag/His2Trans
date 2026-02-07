fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let mut rst: i32 = V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    if rootHashLen <= 0 || rootHashLen > MAX_HASH_SIZE as i32 {
        return rst;
    }
    
    let mdInfo = unsafe { mbedtls_md_info_from_type(digestAlgorithm as mbedtls_md_type_t) };
    if mdInfo.is_null() {
        return V_ERR as i32;
    }
    
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t };
    if mdCtx.is_null() {
        return V_ERR as i32;
    }
    
    unsafe { mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead: BlockHead = unsafe { std::mem::zeroed() };
    
    'exit: loop {
        if ret != V_OK as i32 {
            break 'exit;
        }
        
        ret = unsafe { mbedtls_md_starts(mdCtx) };
        if ret != V_OK as i32 {
            break 'exit;
        }
        
        let readLen = unsafe { (*chunkDigest).len };
        let buffer = unsafe { (*chunkDigest).buffer as *const ::core::ffi::c_uchar };
        ret = unsafe { mbedtls_md_update(mdCtx, buffer, readLen as u32) };
        if ret != V_OK as i32 {
            break 'exit;
        }
        
        rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
        if rawBuf.is_null() {
            break 'exit;
        }
        
        let readLen2 = rawLen;
        ret = unsafe { mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, readLen2 as u32) };
        if ret != V_OK as i32 {
            break 'exit;
        }
        
        outbuf = unsafe { libc::malloc(rootHashLen as usize) as *mut ::core::ffi::c_uchar };
        if outbuf.is_null() {
            break 'exit;
        }
        
        ret = unsafe { mbedtls_md_finish(mdCtx, outbuf) };
        if ret != V_OK as i32 {
            break 'exit;
        }
        
        crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf, rootHashLen);
        let _ = unsafe { memset_s(outbuf as *mut ::core::ffi::c_void, rootHashLen as u32, 0, rootHashLen as u32) };
        rst = V_OK as i32;
        break 'exit;
    }
    
    unsafe { mbedtls_md_free(mdCtx) };
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
    }
    if !rawBuf.is_null() {
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
    }
    if !outbuf.is_null() {
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void) };
    }
    
    rst
}