fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::{mbedtls_md_context_t, mbedtls_md_info_t, mbedtls_md_type_t, size_t, V_OK, V_ERR, LOG_CORE, LOG_ERROR, LOG_INFO};
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    unsafe {
        let mdInfo = mbedtls_md_info_from_type(alg as mbedtls_md_type_t);
        if mdInfo.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdInfo is null\0".as_ptr() as *const i8, __FUNCTION__, 46);
            return V_ERR;
        }
        let mut pos = 0;
        let mut rawBufLen = blockLen;
        let mdCtx = libc::malloc(::core::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t;
        if mdCtx.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__, 50);
            return V_ERR;
        }
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: alg: %d wholelen: %d\0".as_ptr() as *const i8, __FUNCTION__, 51, alg, rawBufLen);
        while rawBufLen > 0 {
            mbedtls_md_init(mdCtx);
            let readLen = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mut ret = mbedtls_md_setup(mdCtx, mdInfo, 0);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 56);
                goto EXIT;
            }
            let hlen = mbedtls_md_get_size(mdInfo) as usize;
            if hlen == 0 || hlen > 64 {
                goto EXIT;
            }
            ret = mbedtls_md_starts(mdCtx);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 62);
                goto EXIT;
            }
            let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
            if memcpy_s((chunkContentPrefix.as_mut_ptr() as *mut c_void).offset(1) as *mut c_void, 4, (&readLen as *const i32) as *const c_void, ::core::mem::size_of::<i32>()) != 0 {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: memcpy_s fail\0".as_ptr() as *const i8, __FUNCTION__, 65);
                goto EXIT;
            }
            ret = mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), 5);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 69);
                goto EXIT;
            }
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__, 70, rawBufLen, pos);
            ret = mbedtls_md_update(mdCtx, block.offset(pos as isize) as *const u8, readLen as size_t);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 72);
                goto EXIT;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen) as *mut u8;
            if outbuf.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__, 76);
                goto EXIT;
            }
            ret = mbedtls_md_finish(mdCtx, outbuf);
            HapPutData(result, *offset, outbuf, hlen as i32);
            *offset += hlen as i32;
            let _ = memset_s(outbuf as *mut c_void, hlen, 0, hlen) as i32 as i32 as i32;
            libc::free(outbuf as *mut c_void);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 82);
                goto EXIT;
            }
            mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut c_void);
        return V_OK;
        EXIT:
        mbedtls_md_free(mdCtx);
        libc::free(mdCtx as *mut c_void);
        return V_ERR;
    }
}