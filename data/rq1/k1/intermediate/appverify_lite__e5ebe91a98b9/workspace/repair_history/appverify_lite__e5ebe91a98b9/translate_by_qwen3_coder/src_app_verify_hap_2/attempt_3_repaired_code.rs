fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::{V_OK, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO};
    let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(alg as crate::types::mbedtls_md_type_t) };
    if mdInfo.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdInfo is null\0".as_ptr() as *const i8, __FUNCTION__!(), 46) };
        return V_ERR;
    }
    let mut pos: i32 = 0;
    let mut rawBufLen = blockLen;
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t };
    if mdCtx.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__!(), 50) };
        return V_ERR;
    }
    let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg: %d wholelen: %d\0".as_ptr() as *const i8, __FUNCTION__!(), 51, alg, rawBufLen) };
    while rawBufLen > 0 {
        unsafe { crate::compat::mbedtls_md_init(mdCtx) };
        let readLen = if rawBufLen > 1024 * 1024 { 1024 * 1024 } else { rawBufLen };
        let mut ret = unsafe { crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0) };
        if ret != V_OK {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 56) };
            goto EXIT;
        }
        let hlen = unsafe { crate::compat::mbedtls_md_get_size(mdInfo) };
        if hlen == 0 || hlen > 64 {
            goto EXIT;
        }
        ret = unsafe { crate::compat::mbedtls_md_starts(mdCtx) };
        if ret != V_OK {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 62) };
            goto EXIT;
        }
        let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        if unsafe { crate::compat::memcpy_s((chunkContentPrefix.as_mut_ptr() as *mut std::ffi::c_void).offset(1), 4, &readLen as *const i32 as *const std::ffi::c_void, std::mem::size_of::<i32>()) } != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: memcpy_s fail\0".as_ptr() as *const i8, __FUNCTION__!(), 65) };
            goto EXIT;
        }
        ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr() as *const u8, 5) };
        if ret != V_OK {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 69) };
            goto EXIT;
        }
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 70, rawBufLen, pos) };
        ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, block.offset(pos as isize) as *const u8, readLen as crate::types::size_t) };
        if ret != V_OK {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 72) };
            goto EXIT;
        }
        rawBufLen -= readLen;
        pos += readLen;
        let outbuf = unsafe { libc::malloc(hlen as usize) as *mut u8 };
        if outbuf.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 76) };
            goto EXIT;
        }
        ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
        unsafe { crate::compat::HapPutData(result, *offset, outbuf, hlen as i32) };
        unsafe { *offset += hlen as i32 };
        let _ = unsafe { crate::compat::memset_s(outbuf as *mut std::ffi::c_void, hlen as usize, 0, hlen as usize) };
        if !outbuf.is_null() {
            unsafe { libc::free(outbuf as *mut std::ffi::c_void) };
        }
        if ret != V_OK {
            let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 82) };
            goto EXIT;
        }
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    }
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut std::ffi::c_void) };
    }
    return V_OK;
    EXIT:
    unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    if !mdCtx.is_null() {
        unsafe { libc::free(mdCtx as *mut std::ffi::c_void) };
    }
    return V_ERR;
}