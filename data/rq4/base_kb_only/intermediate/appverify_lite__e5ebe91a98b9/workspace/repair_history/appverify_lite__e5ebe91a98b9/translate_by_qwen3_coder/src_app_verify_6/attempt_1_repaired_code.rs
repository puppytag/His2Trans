pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    if signInfo.is_null() || blockHead.is_null() {
        return std::ptr::null_mut();
    }
    
    let ret = FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != crate::types::V_OK as i32 {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let block_length = (*blockHead).length;
        let block_offset = (*blockHead).offset;
        let full_sign_block_offset = (*signInfo).fullSignBlockOffset;
        let hap_core_dir_offset = (*signInfo).hapCoreDirOffset;
        let file_size = (*signInfo).fileSize;
        
        if block_length == 0 || block_length > ((hap_core_dir_offset - full_sign_block_offset) as u32) {
            return std::ptr::null_mut();
        }
        
        if (block_length + 1) >= (file_size as u32) {
            return std::ptr::null_mut();
        }
        
        let buf = libc::malloc((block_length + 1) as usize) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return std::ptr::null_mut();
        }
        
        *buf.offset(block_length as isize) = 0;
        
        let mut fileSt: libc::stat = std::mem::zeroed();
        let fstat_ret = libc::fstat(fp, &mut fileSt);
        if fstat_ret != 0 || fileSt.st_size < (full_sign_block_offset as i64 + block_offset as i64 + block_length as i64) {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        libc::lseek(fp, (full_sign_block_offset + block_offset as i32) as libc::off_t, 0);
        let readLen = libc::read(fp, buf as *mut ::core::ffi::c_void, block_length as usize) as i32;
        if readLen != block_length as i32 {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        *len = readLen;
        buf
    }
}