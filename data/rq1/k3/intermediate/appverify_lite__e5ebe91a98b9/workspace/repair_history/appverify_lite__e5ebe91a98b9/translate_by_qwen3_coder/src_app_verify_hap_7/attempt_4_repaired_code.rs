fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp as i32, 0, crate::types::SEEK_SET as i32);
        let mut rawBufLen = (*signInfo).fullSignBlockOffset;
        while rawBufLen > 0 {
            let mut hlen: crate::types::size_t = 0;
            let readLen = if rawBufLen > crate::types::HASH_BLOB_LEN as i32 {
                crate::types::HASH_BLOB_LEN as i32
            } else {
                rawBufLen
            };
            let mdInfo = crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t);
            if mdInfo.is_null() {
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut crate::types::size_t);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            rawBufLen -= readLen;
            let outbuf = libc::malloc(hlen as usize) as *mut libc::c_uchar;
            if outbuf.is_null() {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            let ret = crate::compat::mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(chunkDigest, *offset, outbuf, hlen as i32);
            crate::compat::memset_s(outbuf as *mut libc::c_void, hlen as u64, 0, hlen as u64);
            *offset += hlen as i32;
            libc::free(outbuf as *mut libc::c_void);
            if ret != 0 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            crate::compat::mbedtls_md_free(mdCtx);
        }
        libc::free(mdCtx as *mut libc::c_void);
        return crate::types::V_OK as i32;
    }
}