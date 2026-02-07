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
        
        let alloc_size = (block_length as usize) + 1;
        let buf = libc::malloc(alloc_size) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return std::ptr::null_mut();
        }
        
        *buf.add(block_length as usize) = 0;
        
        let mut fileSt: libc::stat = std::mem::zeroed();
        let fstat_ret = libc::fstat(fp, &mut fileSt);
        let required_size = (full_sign_block_offset as i64) + (block_offset as i64) + (block_length as i64);
        if fstat_ret != 0 || fileSt.st_size < required_size {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        let seek_offset = (full_sign_block_offset as i64) + (block_offset as i64);
        libc::lseek(fp, seek_offset as libc::off_t, libc::SEEK_SET);
        let read_len = libc::read(fp, buf as *mut ::core::ffi::c_void, block_length as usize);
        if read_len != (block_length as isize) {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        *len = read_len as i32;
        buf
    }
}