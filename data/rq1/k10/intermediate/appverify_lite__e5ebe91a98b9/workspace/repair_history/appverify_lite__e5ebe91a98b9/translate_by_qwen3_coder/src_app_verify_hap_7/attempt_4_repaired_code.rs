fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::{mbedtls_md_context_t, mbedtls_md_info_t, size_t};
    use crate::compat::*;
    use crate::src_app_verify_hap::{HapUpdateDigistHead, UpdateSmallBlock};
    use crate::src_app_centraldirectory::HapPutData;
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t;
        if mdCtx.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, __FUNCTION__, 212);
            return V_ERR;
        }
        libc::lseek(fp as i32, 0, SEEK_SET as i32);
        let mut pos = 0;
        let mut rawBufLen = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: size_t = 0;
            let readLen = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mdInfo = mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                libc::free(mdCtx as *mut libc::c_void);
                return V_ERR;
            }
            let mut ret = HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut size_t);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 225);
                goto EXIT;
            }
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, __FUNCTION__, 226, rawBufLen, pos);
            ret = UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 228);
                goto EXIT;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut libc::c_uchar;
            if outbuf.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: outbuf is null\0".as_ptr() as *const i8, __FUNCTION__, 232);
                goto EXIT;
            }
            ret = mbedtls_md_finish(mdCtx, outbuf);
            HapPutData(chunkDigest, *offset, outbuf as *const libc::c_uchar, hlen as i32);
            memset_s(outbuf as *mut libc::c_void, hlen as usize, 0, hlen as usize);
            *offset += hlen as i32;
            libc::free(outbuf as *mut libc::c_void);
            if ret != V_OK {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__, 238);
                goto EXIT;
            }
            mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut libc::c_void);
        return V_OK;
        EXIT:
        mbedtls_md_free(mdCtx);
        libc::free(mdCtx as *mut libc::c_void);
        return V_ERR;
    }
}