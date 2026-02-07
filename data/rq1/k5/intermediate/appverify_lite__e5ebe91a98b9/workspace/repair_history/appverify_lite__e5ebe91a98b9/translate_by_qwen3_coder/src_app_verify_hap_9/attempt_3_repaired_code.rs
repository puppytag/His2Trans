fn ComputerEocdHash(signInfo: *const crate::types::SignatureInfo, digestAlgorithm: i32, fp: i32, chunkDigest: *const crate::types::HapBuf, offset: *mut i32) -> i32 {
    unsafe {
        if (*signInfo).hapEocdSize <= 0 {
            return crate::types::V_ERR as i32;
        }
        let eocdBuf = libc::malloc((*signInfo).hapEocdSize as usize) as *mut crate::types::HapEocd;
        if eocdBuf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: eocdBuf is null\0".as_ptr() as *const _,
                b"ComputerEocdHash\0".as_ptr() as *const _,
                279,
            );
            return crate::types::V_ERR as i32;
        }
        let _ = libc::lseek(fp as i32, (*signInfo).hapEocdOffset as libc::off_t, libc::SEEK_SET as i32);
        let len = libc::read(
            fp as i32,
            eocdBuf as *mut _,
            (*signInfo).hapEocdSize as usize,
        ) as i32;
        if len != (*signInfo).hapEocdSize {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: fread err: %d, %d\0".as_ptr() as *const _,
                b"ComputerEocdHash\0".as_ptr() as *const _,
                283,
                len,
                (*signInfo).hapEocdSize,
            );
            libc::free(eocdBuf as *mut _);
            return crate::types::V_ERR as i32;
        }
        let core_dir_offset_ptr = &mut (*eocdBuf).eocdHead.coreDirOffset as *mut i32;
        crate::src_app_common::HapPutInt32(
            core_dir_offset_ptr as *mut _,
            std::mem::size_of::<i32>() as i32,
            (*signInfo).fullSignBlockOffset,
        );
        let ret = crate::src_app_verify_hap::ComputeBlockHash(
            eocdBuf as *const _,
            len,
            digestAlgorithm,
            chunkDigest,
            offset,
        );
        let _ = crate::compat::memset_s(
            eocdBuf as *mut _,
            (*signInfo).hapEocdSize as u32,
            0,
            (*signInfo).hapEocdSize as u32,
        );
        libc::free(eocdBuf as *mut _);
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: ret not ok\0".as_ptr() as *const _,
                b"ComputerEocdHash\0".as_ptr() as *const _,
                291,
            );
            return ret;
        }
        crate::types::V_OK as i32
    }
}