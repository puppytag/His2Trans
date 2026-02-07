fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__!(), 212);
            return V_ERR;
        }
        libc::lseek(fp as i32, 0, 0);
        let mut pos: i32 = 0;
        let mut rawBufLen = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: crate::types::size_t = 0;
            let readLen = if rawBufLen > HASH_BLOB_LEN as i32 { HASH_BLOB_LEN as i32 } else { rawBufLen };
            let mdInfo = mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut libc::c_void);
                }
                return V_ERR;
            }
            let ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut crate::types::size_t);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 225);
                goto EXIT;
            }
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 226, rawBufLen, pos);
            let ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 228);
                goto EXIT;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut libc::c_uchar;
            if outbuf.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 232);
                goto EXIT;
            }
            let ret = mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(chunkDigest, *offset, outbuf, hlen as i32);
            memset_s(outbuf as *mut libc::c_void, hlen as usize, 0, hlen as usize);
            *offset += hlen as i32;
            if !outbuf.is_null() {
                libc::free(outbuf as *mut libc::c_void);
            }
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 238);
                goto EXIT;
            }
            mbedtls_md_free(mdCtx);
        }
        if !mdCtx.is_null() {
            libc::free(mdCtx as *mut libc::c_void);
        }
        return V_OK;
        EXIT:
        mbedtls_md_free(mdCtx);
        if !mdCtx.is_null() {
            libc::free(mdCtx as *mut libc::c_void);
        }
        return V_ERR;
    }
}