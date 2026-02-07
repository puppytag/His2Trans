pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    
    let mut eocdOffset: i32 = 0;
    let mut hapEocd: crate::types::HapEocd = unsafe { std::mem::zeroed() };
    
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find Eocd fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                154i32,
            );
        }
        return false;
    }
    
    unsafe {
        (*signInfo).hapEocdOffset = eocdOffset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
        
        // Copy the field to a local variable to avoid unaligned reference
        let core_dir_offset_ptr = std::ptr::addr_of!(hapEocd.eocdHead.coreDirOffset);
        let core_dir_offset_val = std::ptr::read_unaligned(core_dir_offset_ptr);
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            (&core_dir_offset_val) as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
        
        if (*signInfo).hapCoreDirOffset <= 0 || (*signInfo).hapCoreDirOffset >= eocdOffset ||
           (*signInfo).hapEocdSize <= 0 || (*signInfo).hapEocdOffset <= 0 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: core dir error\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                162i32,
            );
            return false;
        }
    }
    
    true
}