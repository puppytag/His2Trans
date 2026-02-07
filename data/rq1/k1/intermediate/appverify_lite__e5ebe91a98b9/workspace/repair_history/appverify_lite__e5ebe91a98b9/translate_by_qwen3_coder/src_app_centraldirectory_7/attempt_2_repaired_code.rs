pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    let mut eocdOffset: i32 = 0;
    let mut hapEocd = crate::types::HapEocd {
        eocdHead: crate::types::MinEocd {
            magic: 0,
            diskNum: 0,
            startNum: 0,
            coreDirNumOnDisk: 0,
            coreDirNum: 0,
            coreDirSize: 0,
            coreDirOffset: 0,
            commentLen: 0,
        },
        comment: std::ptr::null_mut(),
    };
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: find Eocd fail\0".as_ptr() as *const _,
                b"FindSignature\0".as_ptr() as *const _,
                154,
            );
        }
        return false;
    }
    unsafe {
        (*signInfo).hapEocdOffset = eocdOffset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
        let core_dir_offset = std::ptr::addr_of!(hapEocd.eocdHead.coreDirOffset) as *const ::core::ffi::c_uchar;
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            core_dir_offset,
            std::mem::size_of::<i32>() as i32,
        );
        if (*signInfo).hapCoreDirOffset <= 0
            || (*signInfo).hapCoreDirOffset >= eocdOffset
            || (*signInfo).hapEocdSize <= 0
            || (*signInfo).hapEocdOffset <= 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: core dir error\0".as_ptr() as *const _,
                b"FindSignature\0".as_ptr() as *const _,
                162,
            );
            return false;
        }
    }
    true
}