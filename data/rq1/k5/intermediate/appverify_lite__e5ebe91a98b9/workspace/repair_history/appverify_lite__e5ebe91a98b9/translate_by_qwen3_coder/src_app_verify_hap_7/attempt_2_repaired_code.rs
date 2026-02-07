fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<crate::types::mbedtls_md_context_t>()) } as *mut crate::types::mbedtls_md_context_t;
    if mdCtx.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: mdCtx is null\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 212) };
        return crate::types::V_ERR as i32;
    }
    unsafe { libc::lseek(fp as i32, 0, crate::types::SEEK_SET as i32) };
    let mut pos: i32 = 0;
    let mut rawBufLen = unsafe { (*signInfo).fullSignBlockOffset };
    while rawBufLen > 0 {
        let mut hlen: crate::types::size_t = 0;
        let readLen = if rawBufLen > crate::types::HASH_BLOB_LEN as i32 { crate::types::HASH_BLOB_LEN as i32 } else { rawBufLen };
        let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as crate::types::mbedtls_md_type_t) };
        if mdInfo.is_null() {
            unsafe { libc::free(mdCtx as *mut _) };
            return crate::types::V_ERR as i32;
        }
        let ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen as *mut crate::types::size_t);
        if ret != crate::types::V_OK as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 225) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut _) };
            return crate::types::V_ERR as i32;
        }
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: content: %d, %d\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 226, rawBufLen, pos) };
        let ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
        if ret != crate::types::V_OK as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 228) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut _) };
            return crate::types::V_ERR as i32;
        }
        rawBufLen -= readLen;
        pos += readLen;
        let outbuf = unsafe { libc::malloc(hlen as usize) } as *mut ::core::ffi::c_uchar;
        if outbuf.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: outbuf is null\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 232) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut _) };
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
        let offset_val = unsafe { *offset };
        crate::src_app_centraldirectory::HapPutData(chunkDigest, offset_val, outbuf, hlen as i32);
        let _ = unsafe { crate::compat::memset_s(outbuf as *mut _, hlen as u32, 0, hlen as u32) };
        unsafe { *offset += hlen as i32 };
        unsafe { libc::free(outbuf as *mut _) };
        if ret != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: ret not ok\0".as_ptr() as *const _, b"ComputerFileHash\0".as_ptr() as *const _, 238) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut _) };
            return crate::types::V_ERR as i32;
        }
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    }
    unsafe { libc::free(mdCtx as *mut _) };
    return crate::types::V_OK as i32;
}