fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let centralDirSize: i32 = (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset;
        if centralDirSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        let dirBuf: *mut std::ffi::c_char = libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char;
        if dirBuf.is_null() {
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp, (*signInfo).hapCoreDirOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let len: i32 = libc::read(fp, dirBuf as *mut std::ffi::c_void, (std::mem::size_of::<std::ffi::c_char>() * centralDirSize as usize) as usize) as i32;
        if len != centralDirSize {
            if !dirBuf.is_null() {
                libc::free(dirBuf as *mut std::ffi::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret: i32 = crate::src_app_verify_hap::ComputeBlockHash(dirBuf as *const std::ffi::c_char, centralDirSize, digestAlgorithm, chunkDigest, offset);
        let _ = crate::compat::memset_s(dirBuf as *mut std::ffi::c_void, centralDirSize as crate::types::size_t, 0, centralDirSize as crate::types::size_t);
        if !dirBuf.is_null() {
            libc::free(dirBuf as *mut std::ffi::c_void);
        }
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        crate::types::V_OK as i32
    }
}