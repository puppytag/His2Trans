//! Module: src_app_file
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    if handle.is_null() || file.is_null() || filePath.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    crate::src_app_verify_hal::RegistHalFunc();
    let path_size = (crate::types::PATH_MAX + 1) as usize;
    let path = unsafe { libc::malloc(path_size) as *mut ::core::ffi::c_char };
    if path.is_null() {
        return crate::types::V_ERR_MALLOC as i32;
    }
    let file_path_len = unsafe { libc::strlen(filePath) };
    if file_path_len > crate::types::PATH_MAX as usize || unsafe { libc::realpath(filePath, path) }.is_null() {
        unsafe { libc::free(path as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let fd = unsafe { libc::open(path, libc::O_RDONLY, 0 as libc::mode_t) };
    unsafe { *handle = fd; }
    if fd < 0 {
        unsafe { libc::free(path as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size <= 0 {
        unsafe { libc::free(path as *mut ::core::ffi::c_void); }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    let len = unsafe { libc::lseek(fd, 0, crate::types::SEEK_END as i32) as i32 };
    unsafe {
        (*file).len = len;
        (*file).fp = fd;
    }
    unsafe { libc::free(path as *mut ::core::ffi::c_void); }
    crate::types::V_OK as i32
}

pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return crate::types::MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = libc::MAP_FAILED as *mut i8;
        if (*file).fp == -1 {
            return crate::types::FILE_IS_CLOSE;
        }
        if offset < 0 || offset > (*file).len - bufCapacity {
            return crate::types::READ_OFFSET_OUT_OF_RANGE;
        }
        libc::lseek((*file).fp, offset as libc::off_t, 0);
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as i32;
        if page_size <= 0 {
            return crate::types::MMAP_FAILED;
        }
        let mmapPosition = (offset / page_size) * page_size;
        let readMoreLen = offset - mmapPosition;
        let mmapSize = bufCapacity + readMoreLen;
        (*mmapInfo).mmapPosition = mmapPosition;
        (*mmapInfo).readMoreLen = readMoreLen;
        (*mmapInfo).mmapSize = mmapSize;
        let map_addr = libc::mmap(
            std::ptr::null_mut(),
            mmapSize as usize,
            libc::PROT_READ,
            libc::MAP_SHARED,
            (*file).fp,
            mmapPosition as libc::off_t,
        );
        (*mmapInfo).mapAddr = map_addr as *mut i8;
        if map_addr == libc::MAP_FAILED {
            return crate::types::MMAP_FAILED;
        }
        crate::types::V_OK.try_into().unwrap()
    }
}

pub extern "C" fn HapMUnMap(mapAddr: *mut ::core::ffi::c_char, mmapSize: i32) {
    if mapAddr.is_null() || mmapSize <= 0 {
        return;
    }
    unsafe {
        libc::munmap(mapAddr as *mut libc::c_void, mmapSize as libc::size_t);
    }
}
