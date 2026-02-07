fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        if (*signInfo).hapEocdSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        
        let eocdBuf = libc::malloc((*signInfo).hapEocdSize as usize) as *mut crate::types::HapEocd;
        if eocdBuf.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        libc::lseek(fp, (*signInfo).hapEocdOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let len = libc::read(fp, eocdBuf as *mut libc::c_void, (*signInfo).hapEocdSize as usize) as i32;
        
        if len != (*signInfo).hapEocdSize {
            libc::free(eocdBuf as *mut libc::c_void);
            return crate::types::V_ERR as i32;
        }
        
        let core_dir_offset_ptr = &mut (*eocdBuf).eocdHead.coreDirOffset as *mut i32 as *mut u8;
        crate::src_app_common::HapPutInt32(core_dir_offset_ptr, std::mem::size_of::<i32>() as i32, (*signInfo).fullSignBlockOffset);
        
        let ret = crate::src_app_verify_hap::ComputeBlockHash(
            eocdBuf as *const std::ffi::c_char,
            len,
            digestAlgorithm,
            chunkDigest,
            offset
        );
        
        let _ = memset_s(
            eocdBuf as *mut libc::c_void,
            (*signInfo).hapEocdSize as u32,
            0,
            (*signInfo).hapEocdSize as u32
        );
        
        libc::free(eocdBuf as *mut libc::c_void);
        
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        
        crate::types::V_OK as i32
    }
}