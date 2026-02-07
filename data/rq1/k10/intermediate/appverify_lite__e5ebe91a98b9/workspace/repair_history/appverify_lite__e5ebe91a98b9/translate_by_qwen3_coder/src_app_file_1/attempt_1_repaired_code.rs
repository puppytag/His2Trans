pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    use crate::types::{PATH_MAX, V_OK, V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_END};
    use crate::src_app_verify_hal::RegistHalFunc;
    unsafe {
        if handle.is_null() || file.is_null() || filePath.is_null() {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 32);
            return V_ERR_FILE_OPEN as i32;
        }
        RegistHalFunc();
        let path = libc::malloc(256 + 1) as *mut ::core::ffi::c_char;
        if path.is_null() {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: path malloc error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 38);
            return V_ERR_MALLOC as i32;
        }
        if (libc::strlen(filePath) > 256) || libc::realpath(filePath, path).is_null() {
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file path error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 43);
            return V_ERR_FILE_OPEN as i32;
        }
        *handle = libc::open(path, 0, 0);
        if *handle < 0 {
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 49);
            return V_ERR_FILE_OPEN as i32;
        }
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = libc::sysconf(30) as i32;
        }
        if crate::globals::g_memoryPageSize <= 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 56, crate::globals::g_memoryPageSize);
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = libc::lseek(*handle, 0, SEEK_END as i32) as i32;
        (*file).fp = *handle;
        if !path.is_null() {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        V_OK as i32
    }
}