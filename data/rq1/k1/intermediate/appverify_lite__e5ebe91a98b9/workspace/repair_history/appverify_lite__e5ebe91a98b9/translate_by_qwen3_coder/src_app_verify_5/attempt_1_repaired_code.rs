fn FindBlockHead(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, block: *mut crate::types::BlockHead) -> i32 {
    unsafe {
        let signH = (*signInfo).signHead;
        let offset = (*signInfo).fullSignBlockOffset as i64;
        let _ = libc::lseek(fp, offset, crate::types::SEEK_SET as i32);
        let mut num = (*signH).blockNum as i32;
        if num > crate::types::MAX_BLOCK_NUM as i32 {
            return crate::types::V_ERR as i32;
        }
        while num > 0 {
            num -= 1;
            let read_len = libc::read(fp, block as *mut ::core::ffi::c_void, std::mem::size_of::<crate::types::BlockHead>() as usize) as i32;
            if read_len != std::mem::size_of::<crate::types::BlockHead>() as i32 {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: find block head , read err %d, %d\0".as_ptr() as *const ::core::ffi::c_char, b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char, 181, read_len, std::mem::size_of::<crate::types::BlockHead>() as i32);
                return crate::types::V_ERR as i32;
            }
            let type_val = crate::src_app_common::HapGetInt(&(*block).type_ as *const u32 as *const ::core::ffi::c_uchar, std::mem::size_of::<u32>() as i32);
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: find block type: %0x\0".as_ptr() as *const ::core::ffi::c_char, b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char, 185, type_val);
            if type_val == blockType {
                crate::src_app_verify::BlockHeadN2H(block);
                return crate::types::V_OK as i32;
            }
        }
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: get sign block by type failed, type: %d\0".as_ptr() as *const ::core::ffi::c_char, b"FindBlockHead\0".as_ptr() as *const ::core::ffi::c_char, 191, blockType);
        crate::types::V_ERR as i32
    }
}