pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    if handle.is_null() || file.is_null() || filePath.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file open error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                32i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    crate::src_app_verify_hal::RegistHalFunc();
    
    let path: *mut ::core::ffi::c_char = unsafe { libc::malloc((crate::types::PATH_MAX + 1) as usize) } as *mut ::core::ffi::c_char;
    if path.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: path malloc error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                38i32,
            );
        }
        return crate::types::V_ERR_MALLOC as i32;
    }
    
    let path_len = unsafe { libc::strlen(filePath) };
    if path_len > crate::types::PATH_MAX as usize || unsafe { libc::realpath(filePath, path) }.is_null() {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file path error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                43i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe { *handle = libc::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file open error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                49i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe {
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = libc::sysconf(30) as i32;
        }
    }
    
    if unsafe { crate::globals::g_memoryPageSize } <= 0 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                56i32,
                crate::globals::g_memoryPageSize as i32,
            );
        }
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    
    unsafe {
        (*file).len = libc::lseek(*handle, 0, crate::types::SEEK_END as i32) as i32;
        (*file).fp = *handle;
    }
    
    if !path.is_null() {
        unsafe { libc::free(path as *mut ::core::ffi::c_void) };
    }
    
    crate::types::V_OK as i32
}