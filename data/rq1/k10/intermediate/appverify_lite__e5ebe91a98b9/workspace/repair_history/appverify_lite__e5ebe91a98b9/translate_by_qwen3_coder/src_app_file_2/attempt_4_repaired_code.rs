pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, READ_OFFSET_OUT_OF_RANGE, MMAP_FAILED, V_OK, SEEK_SET};
    use crate::types::{LOG_CORE, LOG_ERROR};
    use crate::compat::lseek;
    use libc::{mmap, PROT_READ, MAP_SHARED};

    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = (-1isize) as *mut ::core::ffi::c_char;
    }
    let file_fp = unsafe { (*file).fp };
    if file_fp == -1 {
        return FILE_IS_CLOSE;
    }
    let file_len = unsafe { (*file).len };
    if offset < 0 || offset > file_len - bufCapacity {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    unsafe {
        lseek(file_fp, offset as i64, SEEK_SET as i32);
    }
    let page_size = 4096;
    if page_size == 0 {
        return MMAP_FAILED;
    }
    let mmap_pos = (offset / page_size) * page_size;
    let read_more = offset - mmap_pos;
    let mmap_size = bufCapacity + read_more;
    unsafe {
        (*mmapInfo).mmapPosition = mmap_pos;
        (*mmapInfo).readMoreLen = read_more;
        (*mmapInfo).mmapSize = mmap_size;
        (*mmapInfo).mapAddr = mmap(
            std::ptr::null_mut(),
            mmap_size as usize,
            PROT_READ,
            MAP_SHARED,
            file_fp,
            mmap_pos as i64,
        ) as *mut ::core::ffi::c_char;
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: MAP_FAILED\0".as_ptr() as *const ::core::ffi::c_char,
                b"HapMMap\0".as_ptr() as *const ::core::ffi::c_char,
                88,
            );
            return MMAP_FAILED;
        }
    }
    V_OK as i32
}