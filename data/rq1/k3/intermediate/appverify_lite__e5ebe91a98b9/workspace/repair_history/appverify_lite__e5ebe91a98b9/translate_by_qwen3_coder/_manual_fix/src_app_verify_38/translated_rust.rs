fn GetFileRead(fp: i32, offset: i32, size: i32) -> *mut crate::types::FileRead {
    let fileRead = unsafe { libc::malloc(std::mem::size_of::<crate::types::FileRead>()) } as *mut crate::types::FileRead;
    if fileRead.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: malloc error\0".as_ptr() as *const i8, __FUNCTION__!(), 1084) };
        return std::ptr::null_mut();
    }
    unsafe {
        (*fileRead).fp = fp;
        (*fileRead).offset = offset;
        (*fileRead).len = size;
    }
    fileRead
}