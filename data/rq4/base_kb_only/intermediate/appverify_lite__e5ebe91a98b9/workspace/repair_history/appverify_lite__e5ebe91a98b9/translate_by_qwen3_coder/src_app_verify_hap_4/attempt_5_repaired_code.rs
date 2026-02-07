fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst: i32 = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let rootHashLen = GetHashUnitLen(digestAlgorithm);
    if rootHashLen <= 0 || rootHashLen > 64 {
        return rst;
    }
    let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        return crate::types::V_ERR as i32;
    }
    let mdCtx: *mut crate::types::mbedtls_md_context_t = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) } as *mut crate::types::mbedtls_md_context_t;
    if mdCtx.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::compat::mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead: crate::types::BlockHead = unsafe { std::mem::zeroed() };

    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_starts(mdCtx) };
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    let readLen = unsafe { (*chunkDigest).len };
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, (*chunkDigest).buffer as *const ::core::ffi::c_uchar, readLen as usize) };
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    rawBuf = GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
    if rawBuf.is_null() {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, rawLen as usize) };
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); }
        return rst;
    }
    outbuf = unsafe { libc::malloc(rootHashLen as usize) } as *mut ::core::ffi::c_uchar;
    if outbuf.is_null() {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); }
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); libc::free(outbuf as *mut _); }
        return rst;
    }
    HapPutData(fianlDigest, 0, outbuf, rootHashLen);
    let _ = unsafe { crate::compat::memset_s(outbuf as *mut ::core::ffi::c_void, rootHashLen as usize, 0, rootHashLen as usize) };
    rst = crate::types::V_OK as i32;
    unsafe { crate::compat::mbedtls_md_free(mdCtx); libc::free(mdCtx as *mut _); libc::free(rawBuf as *mut _); libc::free(outbuf as *mut _); }
    rst
}