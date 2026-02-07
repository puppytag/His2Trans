fn ComputeDigestsWithOptionalBlock(digestAlgorithm: i32, fp: i32, signInfo: *const crate::types::SignatureInfo, chunkDigest: *const crate::types::HapBuf, fianlDigest: *const crate::types::HapBuf) -> i32 {
    let mut rst = crate::types::V_ERR as i32;
    let mut rawBuf: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let mut outbuf: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let rootHashLen = crate::src_app_verify::GetHashUnitLen(digestAlgorithm);
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rootHashLen %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 117, rootHashLen);
    if rootHashLen <= 0 || rootHashLen > 64 {
        return rst;
    }
    let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mdInfo is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 122);
        return rst;
    }
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t };
    if mdCtx.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mdCtx is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 124);
        return rst;
    }
    unsafe { crate::compat::mbedtls_md_init(mdCtx) };
    let mut ret = unsafe { crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0) };
    let mut rawLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead { type_: 0, length: 0, offset: 0 };
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 130);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_starts(mdCtx) };
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 132);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        return rst;
    }
    let mut readLen = unsafe { (*chunkDigest).len };
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: readLen %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 134, readLen);
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, (*chunkDigest).buffer as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 136);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        return rst;
    }
    rawBuf = crate::src_app_verify::GetSignBlockByType(signInfo, fp, crate::types::PROFILE_BLOCK_WITHSIGN_TYPE as i32, &mut rawLen, &mut blockHead);
    if rawBuf.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rawBuf is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 139);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        return rst;
    }
    readLen = rawLen;
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: signBuf %0x %d\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 141, unsafe { *rawBuf } as i32, readLen);
    ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, rawBuf as *const ::core::ffi::c_uchar, readLen as crate::types::size_t) };
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 143);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
        return rst;
    }
    outbuf = unsafe { libc::malloc(rootHashLen as usize) as *mut ::core::ffi::c_uchar };
    if outbuf.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: outbuf is null\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 145);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
        return rst;
    }
    ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputeDigestsWithOptionalBlock\0".as_ptr() as *const _, 147);
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
        unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void) };
        return rst;
    }
    crate::src_app_centraldirectory::HapPutData(fianlDigest, 0, outbuf, rootHashLen);
    let _ = unsafe { crate::compat::memset_s(outbuf as *mut ::core::ffi::c_void, rootHashLen as crate::types::size_t, 0, rootHashLen as crate::types::size_t) };
    rst = crate::types::V_OK as i32;
    unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    unsafe { libc::free(mdCtx as *mut ::core::ffi::c_void) };
    if !rawBuf.is_null() {
        unsafe { libc::free(rawBuf as *mut ::core::ffi::c_void) };
    }
    if !outbuf.is_null() {
        unsafe { libc::free(outbuf as *mut ::core::ffi::c_void) };
    }
    return rst;
}