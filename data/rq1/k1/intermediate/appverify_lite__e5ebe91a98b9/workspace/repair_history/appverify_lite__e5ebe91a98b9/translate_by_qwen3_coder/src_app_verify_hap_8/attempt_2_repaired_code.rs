fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    let centralDirSize = unsafe { (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset };
    if centralDirSize <= 0 {
        return V_ERR as i32;
    }
    let dirBuf = unsafe { libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char };
    if dirBuf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"dirBuf\" is null\0".as_ptr() as *const _, b"ComputerCoreDirHash\0".as_ptr() as *const _, 257) };
        return V_ERR as i32;
    }
    let _ = unsafe { libc::lseek(fp as libc::c_int, (*signInfo).hapCoreDirOffset as libc::off_t, 0) };
    let len = unsafe { libc::read(fp as libc::c_int, dirBuf as *mut libc::c_void, centralDirSize as usize) as i32 };
    if len != centralDirSize {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"fread err: %d, %d\"\0".as_ptr() as *const _, b"ComputerCoreDirHash\0".as_ptr() as *const _, 261, len, centralDirSize) };
        if !dirBuf.is_null() {
            unsafe { libc::free(dirBuf as *mut libc::c_void); }
        }
        return V_ERR as i32;
    }
    let ret = crate::src_app_verify_hap::ComputeBlockHash(dirBuf as *const std::ffi::c_char, centralDirSize, digestAlgorithm, chunkDigest, offset);
    let _ = unsafe { memset_s(dirBuf as *mut libc::c_void, centralDirSize as u32, 0, centralDirSize as u32) };
    if !dirBuf.is_null() {
        unsafe { libc::free(dirBuf as *mut libc::c_void); }
    }
    if ret != V_OK as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ComputerCoreDirHash\0".as_ptr() as *const _, 268) };
        return ret;
    }
    return V_OK as i32;
}