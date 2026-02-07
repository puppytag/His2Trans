fn HapUpdateDigistHead(digestAlgorithm: i32, mdCtx: *mut crate::types::mbedtls_md_context_t, mdInfo: *const crate::types::mbedtls_md_info_t, readLen: i32, hlen: *mut crate::types::size_t) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    unsafe {
        mbedtls_md_init(mdCtx);
        let ret = mbedtls_md_setup(mdCtx, mdInfo, 0);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        *hlen = mbedtls_md_get_size(mdInfo) as size_t;
        if *hlen == 0 || *hlen > MAX_HASH_SIZE as size_t {
            return V_ERR as i32;
        }
        
        let ret = mbedtls_md_starts(mdCtx);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        let mut chunkContentPrefix: [u8; 5] = [HAP_SECOND_LEVEL_CHUNK_PREFIX as u8, 0, 0, 0, 0];
        
        let result = memcpy_s(
            chunkContentPrefix.as_mut_ptr().add(1) as *mut ::core::ffi::c_void,
            (HAP_DIGEST_PRIFIX_LEN - 1) as u32,
            &readLen as *const i32 as *const ::core::ffi::c_void,
            std::mem::size_of::<i32>() as u32
        );
        if result != EOK as i32 {
            return V_ERR as i32;
        }
        
        let ret = mbedtls_md_update(mdCtx, chunkContentPrefix.as_ptr(), HAP_DIGEST_PRIFIX_LEN as u32);
        if ret != 0 {
            return V_ERR as i32;
        }
        
        V_OK as i32
    }
}