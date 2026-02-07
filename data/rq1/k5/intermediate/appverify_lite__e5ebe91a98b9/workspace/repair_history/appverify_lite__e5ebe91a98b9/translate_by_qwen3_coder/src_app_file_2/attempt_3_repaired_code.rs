pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, MMAP_FAILED, READ_OFFSET_OUT_OF_RANGE, V_OK};
    use crate::compat::{lseek, mmap};
    use libc::{PROT_READ, MAP_SHARED, SEEK_SET};
    use std::ptr;

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
        let _ = lseek((*file).fp, offset as libc::off_t, SEEK_SET as i32);
        let g_memory_page_size = crate::compat::sysconf(libc::_SC_PAGESIZE) as i32;
        if g_memory_page_size == 0 {
            return MMAP_FAILED;
        }
        (*mmapInfo).mmapPosition = (offset / g_memory_page_size) * g_memory_page_size;
        (*mmapInfo).readMoreLen = offset - (*mmapInfo).mmapPosition;
        (*mmapInfo).mmapSize = bufCapacity + (*mmapInfo).readMoreLen;
        (*mmapInfo).mapAddr = mmap(
            ptr::null_mut(),
            (*mmapInfo).mmapSize as u64,
            PROT_READ as i32,
            MAP_SHARED as i32,
            (*file).fp,
            (*mmapInfo).mmapPosition as libc::off_t,
        ) as *mut ::core::ffi::c_char;
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            return MMAP_FAILED;
        }
    }
    V_OK as i32
}