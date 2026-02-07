fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    unsafe {
        let signH = (*signInfo).signHead;
        
        libc::lseek(fp, (*signInfo).fullSignBlockOffset as libc::off_t, crate::types::SEEK_SET as i32);
        let mut num = (*signH).blockNum as i32;
        if num > crate::types::MAX_BLOCK_NUM as i32 {
            return crate::types::V_ERR as i32;
        }
        
        while num > 0 {
            num -= 1;
            let readLen = libc::read(fp, block as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::BlockHead>()) as i32;
            if readLen != std::mem::size_of::<crate::types::BlockHead>() as i32 {
                return crate::types::V_ERR as i32;
            }
            
            let block_type_ptr = &(*block).type_ as *const u32 as *const ::core::ffi::c_uchar;
            let type_val = crate::src_app_common::HapGetInt(block_type_ptr, std::mem::size_of::<u32>() as i32);
            
            if type_val == blockType {
                crate::src_app_verify::BlockHeadN2H(block);
                return crate::types::V_OK as i32;
            }
        }
        
        return crate::types::V_ERR as i32;
    }
}