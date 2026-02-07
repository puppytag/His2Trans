pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    
    let mut eocdOffset: i32 = 0;
    let mut hapEocd: crate::types::HapEocd = unsafe { std::mem::zeroed() };
    
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        return false;
    }
    
    unsafe {
        (*signInfo).hapEocdOffset = eocdOffset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
        
        // Get pointer to the coreDirOffset field and use read_unaligned
        let core_dir_offset_ptr = std::ptr::addr_of!(hapEocd.eocdHead.coreDirOffset);
        let core_dir_offset_val = std::ptr::read_unaligned(core_dir_offset_ptr);
        let core_dir_offset_bytes: [u8; 4] = core_dir_offset_val.to_ne_bytes();
        let bytes_ptr: *const u8 = core_dir_offset_bytes.as_ptr();
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            bytes_ptr,
            std::mem::size_of::<i32>() as i32
        );
        
        if (*signInfo).hapCoreDirOffset <= 0 || (*signInfo).hapCoreDirOffset >= eocdOffset ||
           (*signInfo).hapEocdSize <= 0 || (*signInfo).hapEocdOffset <= 0 {
            return false;
        }
    }
    
    true
}