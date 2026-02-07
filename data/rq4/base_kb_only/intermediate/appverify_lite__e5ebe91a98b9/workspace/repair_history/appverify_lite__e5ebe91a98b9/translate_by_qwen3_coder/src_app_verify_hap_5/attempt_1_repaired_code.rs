fn HapUpdateDigistHead(digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut crate::types::size_t) -> i32 {
    unsafe {
        crate::compat::mbedtls_md_init(mdCtx);
        let ret = crate::compat::mbedtls_md_setup(mdCtx, mdInfo, 0);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        *hlen = crate::compat::mbedtls_md_get_size(mdInfo) as crate::types::size_t;
        if *hlen == 0 || *hlen > crate::types::MAX_HASH_SIZE {
            return crate::types::V_ERR as i32;
        }
        let ret = crate::compat::mbedtls_md_starts(mdCtx);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        let mut chunkContentPrefix: [u8; 5] = [crate::types::HAP_SECOND_LEVEL_CHUNK_PREFIX as u8, 0, 0, 0, 0];
        let ret = crate::compat::memcpy_s(
            chunkContentPrefix.as_mut_ptr().add(1) as *mut ::core::ffi::c_void,
            (5 - 1) as u32,
            &readLen as *const i32 as *const ::core::ffi::c_void,
            core::mem::size_of::<i32>() as u32
        );
        if ret != crate::types::EOK as i32 {
            return crate::types::V_ERR as i32;
        }
        let ret = crate::compat::mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), crate::types::HAP_DIGEST_PRIFIX_LEN as crate::types::size_t);
        if ret != 0 {
            return crate::types::V_ERR as i32;
        }
        crate::types::V_OK as i32
    }
}