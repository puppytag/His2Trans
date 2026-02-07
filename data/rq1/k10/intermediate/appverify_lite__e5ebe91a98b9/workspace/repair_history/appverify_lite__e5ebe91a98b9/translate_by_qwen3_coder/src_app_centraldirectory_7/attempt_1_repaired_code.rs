pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    let mut eocd_offset: i32 = 0;
    let mut hap_eocd = crate::types::HapEocd {
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
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hap_eocd, &mut eocd_offset) {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find Eocd fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                154,
            );
        }
        return false;
    }
    unsafe {
        (*signInfo).hapEocdOffset = eocd_offset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocd_offset;
        let core_dir_offset_bytes = std::ptr::addr_of!(hap_eocd.eocdHead.coreDirOffset) as *const ::core::ffi::c_uchar;
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            core_dir_offset_bytes,
            std::mem::size_of::<i32>() as i32,
        );
        if (*signInfo).hapCoreDirOffset <= 0
            || (*signInfo).hapCoreDirOffset >= eocd_offset
            || (*signInfo).hapEocdSize <= 0
            || (*signInfo).hapEocdOffset <= 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: core dir error\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                162,
            );
            return false;
        }
    }
    true
}