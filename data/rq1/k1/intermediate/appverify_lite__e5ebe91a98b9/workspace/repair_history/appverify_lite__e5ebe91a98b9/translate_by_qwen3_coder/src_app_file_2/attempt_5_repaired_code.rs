pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, READ_OFFSET_OUT_OF_RANGE, MMAP_FAILED, V_OK, LOG_CORE, LOG_ERROR};
    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = (-1isize) as *mut ::core::ffi::c_char;
        if (*file).fp == -1 {
            return FILE_IS_CLOSE;
        }
        if offset < 0 || offset > (*file).len - bufCapacity {
            return READ_OFFSET_OUT_OF_RANGE;
        }
        let _ = crate::compat::lseek((*file).fp, offset as crate::types::off_t, 0);
        let g_memoryPageSize_var = crate::compat::sysconf(4) as i32;
        if g_memoryPageSize == 0 {
            return MMAP_FAILED;
        }
        (*mmapInfo).mmapPosition = (offset / g_memoryPageSize) * g_memoryPageSize;
        (*mmapInfo).readMoreLen = offset - (*mmapInfo).mmapPosition;
        (*mmapInfo).mmapSize = bufCapacity + (*mmapInfo).readMoreLen;
        (*mmapInfo).mapAddr = crate::compat::mmap(
            std::ptr::null_mut(),
            (*mmapInfo).mmapSize as crate::types::size_t,
            1,
            0x01,
            (*file).fp,
            (*mmapInfo).mmapPosition as crate::types::off_t,
        ) as *mut ::core::ffi::c_char;
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const ::core::ffi::c_char,
                "[%s:%d]: MAP_FAILED\0".as_ptr() as *const ::core::ffi::c_char,
                "HapMMap\0".as_ptr() as *const ::core::ffi::c_char,
                88,
            );
            return MMAP_FAILED;
        }
        V_OK as i32
    }
}