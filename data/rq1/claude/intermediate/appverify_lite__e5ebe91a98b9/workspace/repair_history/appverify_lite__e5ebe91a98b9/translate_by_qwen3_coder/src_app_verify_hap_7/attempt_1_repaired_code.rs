fn ComputerFileHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::types::*;
    
    let mdCtx = unsafe { libc::malloc(std::mem::size_of::<mbedtls_md_context_t>()) as *mut mbedtls_md_context_t };
    if mdCtx.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mdCtx is null\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 212i32) };
        return V_ERR as i32;
    }
    
    unsafe { libc::lseek(fp, 0, SEEK_SET as i32) };
    let mut pos: i32 = 0;
    let mut rawBufLen: i32 = unsafe { (*signInfo).fullSignBlockOffset };
    
    while rawBufLen > 0 {
        let mut hlen: size_t = 0;
        let readLen: i32 = if rawBufLen > (1024 * 1024) { 1024 * 1024 } else { rawBufLen };
        let mdInfo = unsafe { crate::compat::mbedtls_md_info_from_type(digestAlgorithm as mbedtls_md_type_t) };
        if mdInfo.is_null() {
            unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
            return V_ERR as i32;
        }
        
        let ret = crate::src_app_verify_hap::HapUpdateDigistHead(digestAlgorithm, mdCtx, mdInfo, readLen, &mut hlen);
        if ret != V_OK as i32 {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 225i32) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
            return V_ERR as i32;
        }
        
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: content: %d, %d\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 226i32, rawBufLen, pos) };
        
        let ret = crate::src_app_verify_hap::UpdateSmallBlock(readLen, &fp as *const i32, mdCtx);
        if ret != V_OK as i32 {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 228i32) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
            return V_ERR as i32;
        }
        
        rawBufLen -= readLen;
        pos += readLen;
        
        let outbuf = unsafe { libc::malloc(hlen as usize) as *mut u8 };
        if outbuf.is_null() {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: outbuf is null\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 232i32) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
            return V_ERR as i32;
        }
        
        let ret = unsafe { crate::compat::mbedtls_md_finish(mdCtx, outbuf) };
        crate::src_app_centraldirectory::HapPutData(chunkDigest, unsafe { *offset }, outbuf, hlen as i32);
        let _ = unsafe { crate::compat::memset_s(outbuf as *mut core::ffi::c_void, hlen as size_t, 0, hlen as size_t) };
        unsafe { *offset += hlen as i32 };
        unsafe { libc::free(outbuf as *mut core::ffi::c_void) };
        
        if ret != V_OK as i32 {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: ret not ok\0".as_ptr() as *const i8, b"ComputerFileHash\0".as_ptr() as *const i8, 238i32) };
            unsafe { crate::compat::mbedtls_md_free(mdCtx) };
            unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
            return V_ERR as i32;
        }
        unsafe { crate::compat::mbedtls_md_free(mdCtx) };
    }
    
    unsafe { libc::free(mdCtx as *mut core::ffi::c_void) };
    V_OK as i32
}