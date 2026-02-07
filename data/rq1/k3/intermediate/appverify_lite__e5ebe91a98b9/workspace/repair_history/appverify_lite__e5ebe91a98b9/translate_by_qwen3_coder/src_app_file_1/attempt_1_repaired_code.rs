pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    use crate::types::{V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, V_OK, PATH_MAX, SEEK_END};
    use crate::globals::g_memoryPageSize;
    use crate::src_app_verify_hal::RegistHalFunc;
    if handle.is_null() || file.is_null() || filePath.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file open error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            32,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        RegistHalFunc();
    }
    let path = unsafe { libc::malloc((PATH_MAX + 1) as usize) } as *mut ::core::ffi::c_char;
    if path.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: path malloc error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            38,
        ) };
        return V_ERR_MALLOC as i32;
    }
    let file_path_len = unsafe { libc::strlen(filePath) };
    if file_path_len > PATH_MAX as usize || unsafe { crate::compat::realpath(filePath, path) }.is_null() {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file path error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            43,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe { *handle = crate::compat::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file open error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            49,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        if g_memoryPageSize == 0 {
            g_memoryPageSize = crate::compat::sysconf(crate::types::_SC_PAGESIZE!() as i32);
        }
        if g_memoryPageSize <= 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _,
                b"InitVerify\0".as_ptr() as *const _,
                56,
                g_memoryPageSize,
            );
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = crate::compat::lseek(*handle, 0, SEEK_END as i32);
        (*file).fp = *handle;
        if !path.is_null() {
            libc::free(path as *mut ::core::ffi::c_void);
        }
    }
    V_OK as i32
}