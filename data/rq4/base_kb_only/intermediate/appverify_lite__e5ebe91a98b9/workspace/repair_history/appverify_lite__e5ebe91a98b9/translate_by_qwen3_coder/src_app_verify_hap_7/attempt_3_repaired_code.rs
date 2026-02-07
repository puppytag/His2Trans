fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp, 0, crate::types::SEEK_SET as i32);
        let mut pos: i32 = 0;
        let mut rawBufLen: i32 = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: crate::types::size_t = 0;
            let readLen: i32 = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            let mdInfo = crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            let mut ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, fp, mdCtx);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            rawBufLen -= readLen;
            pos += readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut u8;
            if outbuf.is_null() {
                crate::compat::mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            ret = crate::compat::mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(chunkDigest, *offset, outbuf, hlen as i32);
            let _ = crate::compat::memset_s(outbuf as *mut ::core::ffi::c_void, hlen, 0, hlen);
            *offset += hlen as i32;
            if !outbuf.is_null() {
                libc::free(outbuf as *mut ::core::ffi::c_void);
            }
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                if !mdCtx.is_null() {
                    libc::free(mdCtx as *mut ::core::ffi::c_void);
                }
                return crate::types::V_ERR as i32;
            }
            crate::compat::mbedtls_md_free(mdCtx);
        }
        if !mdCtx.is_null() {
            libc::free(mdCtx as *mut ::core::ffi::c_void);
        }
        let _ = pos;
        crate::types::V_OK as i32
    }
}