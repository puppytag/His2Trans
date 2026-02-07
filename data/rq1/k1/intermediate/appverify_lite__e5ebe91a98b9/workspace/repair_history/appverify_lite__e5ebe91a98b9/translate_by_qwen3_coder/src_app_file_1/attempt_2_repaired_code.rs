pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    use crate::types::{PATH_MAX, V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, V_OK, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_END};
    use crate::src_app_verify_hal::RegistHalFunc;
    if handle.is_null() || file.is_null() || filePath.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 32) };
        return V_ERR_FILE_OPEN as i32;
    }
    RegistHalFunc();
    let path = unsafe { libc::malloc((PATH_MAX as usize) + 1) as *mut ::core::ffi::c_char };
    if path.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: path malloc error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 38) };
        return V_ERR_MALLOC as i32;
    }
    if unsafe { libc::strlen(filePath) as u64 } > PATH_MAX as u64 || unsafe { crate::compat::realpath(filePath, path) }.is_null() {
        unsafe { libc::free(path as *mut _) };
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file path error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 43) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe { *handle = crate::compat::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        unsafe { libc::free(path as *mut _) };
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 49) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = crate::compat::sysconf(30);
        }
        if crate::globals::g_memoryPageSize <= 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 56, crate::globals::g_memoryPageSize);
            libc::free(path as *mut _);
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = crate::compat::lseek(*handle, 0, SEEK_END as i32) as i32;
        (*file).fp = *handle;
        libc::free(path as *mut _);
    }
    V_OK as i32
}