pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    if signInfo.is_null() || blockHead.is_null() {
        return std::ptr::null_mut();
    }
    
    let ret = FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != crate::types::V_OK as i32 {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let block_length: u32 = (*blockHead).length;
        let block_offset: u32 = (*blockHead).offset;
        let full_sign_block_offset: i32 = (*signInfo).fullSignBlockOffset;
        let hap_core_dir_offset: i32 = (*signInfo).hapCoreDirOffset;
        let file_size: i32 = (*signInfo).fileSize;
        
        if block_length == 0 || block_length > ((hap_core_dir_offset - full_sign_block_offset) as u32) {
            return std::ptr::null_mut();
        }
        
        if (block_length + 1) >= (file_size as u32) {
            return std::ptr::null_mut();
        }
        
        let alloc_size: usize = (block_length as usize) + 1;
        let buf: *mut ::core::ffi::c_char = libc::malloc(alloc_size) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return std::ptr::null_mut();
        }
        
        *(buf.add(block_length as usize)) = 0i8;
        
        let mut file_st: libc::stat = std::mem::zeroed();
        let fstat_ret: i32 = libc::fstat(fp, &mut file_st);
        let required_size: i64 = (full_sign_block_offset as i64) + (block_offset as i64) + (block_length as i64);
        if fstat_ret != 0 || file_st.st_size < required_size {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        let seek_offset: libc::off_t = (full_sign_block_offset as libc::off_t) + (block_offset as libc::off_t);
        let _ = libc::lseek(fp, seek_offset, 0);
        let read_len: isize = libc::read(fp, buf as *mut ::core::ffi::c_void, block_length as usize);
        if read_len != (block_length as isize) {
            libc::free(buf as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        *len = read_len as i32;
        buf
    }
}