fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        if (*signInfo).hapEocdSize <= 0 {
            return V_ERR as i32;
        }
        let eocdBuf = libc::malloc((*signInfo).hapEocdSize as usize) as *mut crate::types::HapEocd;
        if eocdBuf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: eocdBuf is null\0".as_ptr() as *const i8, __FUNCTION__!(), 279);
            return V_ERR as i32;
        }
        let _ = libc::lseek(fp as i32, (*signInfo).hapEocdOffset as libc::off_t, SEEK_SET as i32);
        let len = libc::read(fp as i32, eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize) as i32;
        if len != (*signInfo).hapEocdSize {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8, __FUNCTION__!(), 283, len, (*signInfo).hapEocdSize);
            if !eocdBuf.is_null() {
                libc::free(eocdBuf as *mut libc::c_void);
            }
            return V_ERR as i32;
        }
        crate::src_app_common::HapPutInt32((&mut (*eocdBuf).eocdHead.coreDirOffset) as *mut i32 as *mut u8, std::mem::size_of::<i32>() as i32, (*signInfo).fullSignBlockOffset);
        let ret = crate::src_app_verify_hap::ComputeBlockHash(eocdBuf as *const i8, len, digestAlgorithm, chunkDigest, offset);
        let _ = memset_s(eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize, 0, (*signInfo).hapEocdSize as usize);
        if !eocdBuf.is_null() {
            libc::free(eocdBuf as *mut libc::c_void);
        }
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: ret not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 291);
            return ret;
        }
        return V_OK as i32;
    }
}