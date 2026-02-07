fn ComputeBlockHash(block: *const std::ffi::c_char, blockLen: i32, alg: i32, result: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let mdInfo = crate::compat::mbedtls_md_info_from_type(alg as crate::types::mbedtls_md_type_t);
        if mdInfo.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        let mut pos: i32 = 0;
        let mut rawBufLen: i32 = blockLen;
        
        let mdCtx = libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) as *mut crate::types::mbedtls_md_context_t;
        if mdCtx.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        while rawBufLen > 0 {
            crate::compat::mbedtls_md_init(mdCtx);
            let readLen: i32 = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
            
            let mut ret = crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            let hlen: crate::types::size_t = crate::compat::mbedtls_md_get_size(mdInfo) as crate::types::size_t;
            if hlen == 0 || hlen > 64 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_starts(mdCtx);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            let mut chunkContentPrefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
            if crate::compat::memcpy_s(
                chunkContentPrefix.as_mut_ptr().offset(1) as *mut libc::c_void,
                4,
                &readLen as *const i32 as *const libc::c_void,
                std::mem::size_of::<i32>() as crate::types::size_t
            ) != 0 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_update(mdCtx, (chunkContentPrefix.as_ptr)(), 5);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_update(mdCtx, (block as *const u8).offset(pos as isize), readLen as crate::types::size_t);
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            rawBufLen -= readLen;
            pos += readLen;
            
            let outbuf = libc::malloc(hlen as usize) as *mut u8;
            if outbuf.is_null() {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            ret = crate::compat::mbedtls_md_finish(mdCtx, outbuf);
            crate::src_app_centraldirectory::HapPutData(result, *offset, outbuf, hlen as i32);
            *offset += hlen as i32;
            let _ = crate::compat::memset_s(outbuf as *mut libc::c_void, hlen, 0, hlen);
            libc::free(outbuf as *mut libc::c_void);
            
            if ret != crate::types::V_OK as i32 {
                crate::compat::mbedtls_md_free(mdCtx);
                libc::free(mdCtx as *mut libc::c_void);
                return crate::types::V_ERR as i32;
            }
            
            crate::compat::mbedtls_md_free(mdCtx);
        }
        
        libc::free(mdCtx as *mut libc::c_void);
        crate::types::V_OK as i32
    }
}