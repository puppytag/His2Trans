fn HapUpdateDigistHead(digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut crate::types::size_t) -> i32 {
    unsafe {
        crate::compat::mbedtls_md_init(mdCtx);
        let mut ret = crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        *hlen = crate::compat::mbedtls_md_get_size(mdInfo) as crate::types::size_t;
        if *hlen == 0 || *hlen > 64 {
            return crate::types::V_ERR as i32;
        }
        ret = crate::compat::mbedtls_md_starts(mdCtx);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        let mut chunk_content_prefix: [u8; 5] = [0xa5, 0, 0, 0, 0];
        if crate::compat::memcpy_s(
            (chunk_content_prefix.as_mut_ptr()).offset(1) as *mut core::ffi::c_void,
            (5 - 1) as crate::types::size_t,
            &readLen as *const i32 as *const core::ffi::c_void,
            std::mem::size_of::<i32>() as crate::types::size_t,
        ) != 0
        {
            return crate::types::V_ERR as i32;
        }
        ret = crate::compat::mbedtls_md_update(mdCtx, chunk_content_prefix.as_ptr(), 5 as crate::types::size_t);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        crate::types::V_OK as i32
    }
}