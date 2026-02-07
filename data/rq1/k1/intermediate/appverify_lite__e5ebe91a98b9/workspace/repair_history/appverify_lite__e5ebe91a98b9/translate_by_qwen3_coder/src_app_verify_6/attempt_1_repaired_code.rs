pub extern "C" fn GetSignBlockByType(signInfo: *const crate::types::SignatureInfo, fp: i32, blockType: i32, len: *mut i32, blockHead: *mut crate::types::BlockHead) -> *mut ::core::ffi::c_char {
    use crate::src_app_verify::FindBlockHead;
    if signInfo.is_null() || blockHead.is_null() {
        return std::ptr::null_mut();
    }
    let ret = FindBlockHead(signInfo, fp, blockType, blockHead);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: find block head error\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 203);
        return std::ptr::null_mut();
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: type: %u, len: %u, offset: %u signoffset: %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 207, (*blockHead).type_, (*blockHead).length, (*blockHead).offset, (*signInfo).fullSignBlockOffset);
        if (*blockHead).length == 0 || (*blockHead).length > ((*signInfo).hapCoreDirOffset - (*signInfo).fullSignBlockOffset) as u32 {
            return std::ptr::null_mut();
        }
        if (*blockHead).length as i32 + 1 >= (*signInfo).fileSize {
            return std::ptr::null_mut();
        }
        let buf = libc::malloc(((*blockHead).length + 1) as usize) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 222);
            return std::ptr::null_mut();
        }
        *buf.offset((*blockHead).length as isize) = 0;
        let mut fileSt: crate::compat::stat = std::mem::zeroed();
        let ret = libc::fstat(fp, &mut fileSt as *mut libc::stat);
        if ret != 0 || fileSt.st_size < (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64 + (*blockHead).length as i64 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: fstat error, %d, filelen: %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 229, ret, fileSt.st_size as i32);
            libc::free(buf as *mut libc::c_void);
            return std::ptr::null_mut();
        }
        libc::lseek(fp, (*signInfo).fullSignBlockOffset as i64 + (*blockHead).offset as i64, 0);
        let readLen = libc::read(fp, buf as *mut libc::c_void, (*blockHead).length as usize) as i32;
        if readLen != (*blockHead).length as i32 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: read error: %d, %d\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 236, readLen, (*blockHead).length);
            libc::free(buf as *mut libc::c_void);
            return std::ptr::null_mut();
        }
        if !len.is_null() {
            *len = readLen;
        }
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_INFO as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: buf begin\0".as_ptr() as *const _, b"GetSignBlockByType\0".as_ptr() as *const _, 241);
        buf
    }
}