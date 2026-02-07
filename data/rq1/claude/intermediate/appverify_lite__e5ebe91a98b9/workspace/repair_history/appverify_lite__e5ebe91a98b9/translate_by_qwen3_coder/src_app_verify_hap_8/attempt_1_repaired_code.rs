fn ComputerCoreDirHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        let centralDirSize = (*signInfo).hapEocdOffset - (*signInfo).hapCoreDirOffset;
        if centralDirSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        let dirBuf = libc::malloc(centralDirSize as usize) as *mut std::ffi::c_char;
        if dirBuf.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: dirBuf is null\0".as_ptr() as *const std::ffi::c_char,
                b"ComputerCoreDirHash\0".as_ptr() as *const std::ffi::c_char,
                257i32,
            );
            return crate::types::V_ERR as i32;
        }
        libc::lseek(fp, (*signInfo).hapCoreDirOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let len = libc::read(fp, dirBuf as *mut std::ffi::c_void, centralDirSize as usize) as i32;
        if len != centralDirSize {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: fread err: %d, %d\0".as_ptr() as *const std::ffi::c_char,
                b"ComputerCoreDirHash\0".as_ptr() as *const std::ffi::c_char,
                261i32,
                len,
                centralDirSize,
            );
            libc::free(dirBuf as *mut std::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        let ret = crate::src_app_verify_hap::ComputeBlockHash(dirBuf, centralDirSize, digestAlgorithm, chunkDigest, offset);
        let _ = crate::compat::memset_s(dirBuf as *mut std::ffi::c_void, centralDirSize as u32, 0, centralDirSize as u32);
        libc::free(dirBuf as *mut std::ffi::c_void);
        if ret != crate::types::V_OK as i32 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const std::ffi::c_char,
                b"ComputerCoreDirHash\0".as_ptr() as *const std::ffi::c_char,
                268i32,
            );
            return ret;
        }
        crate::types::V_OK as i32
    }
}