pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, MMAP_FAILED, READ_OFFSET_OUT_OF_RANGE, V_OK};
    use crate::compat::{lseek, mmap};
    use libc::{PROT_READ, MAP_SHARED, MAP_FAILED};
    use core::ffi::c_void;

    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = MAP_FAILED as *mut ::core::ffi::c_char;
    }
    let fp = unsafe { (*file).fp };
    if fp == -1 {
        return FILE_IS_CLOSE;
    }
    let len = unsafe { (*file).len };
    if offset < 0 || offset > len - bufCapacity {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    unsafe {
        lseek(fp, offset as i64, 0);
    }
    let page_size = unsafe { crate::globals::g_memoryPageSize };
    if page_size == 0 {
        return MMAP_FAILED;
    }
    let mmap_position = (offset / page_size) * page_size;
    let read_more_len = offset - mmap_position;
    let mmap_size = bufCapacity + read_more_len;
    let map_addr = unsafe {
        mmap(
            core::ptr::null_mut(),
            mmap_size as u32,
            PROT_READ as i32,
            MAP_SHARED as i32,
            fp,
            mmap_position as i64,
        ) as *mut ::core::ffi::c_char
    };
    unsafe {
        (*mmapInfo).mmapPosition = mmap_position;
        (*mmapInfo).readMoreLen = read_more_len;
        (*mmapInfo).mmapSize = mmap_size;
        (*mmapInfo).mapAddr = map_addr;
    }
    if map_addr as *mut c_void == MAP_FAILED {
        return MMAP_FAILED;
    }
    V_OK as i32
}