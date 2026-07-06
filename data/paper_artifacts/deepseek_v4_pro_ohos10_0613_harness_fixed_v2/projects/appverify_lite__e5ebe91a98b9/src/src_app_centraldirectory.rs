//! Module: src_app_centraldirectory
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

pub extern "C" fn HapPutByte(hapBuffer: *const crate::types::HapBuf, offset: i32, value: ::core::ffi::c_char) {
    if hapBuffer.is_null() {
        return;
    }
    let buf = unsafe { (*hapBuffer).buffer };
    if buf.is_null() {
        return;
    }
    let len = unsafe { (*hapBuffer).len };
    if offset >= 0 && len - offset >= 1 {
        unsafe {
            let dest = (buf as *mut u8).offset(offset as isize);
            *dest = value as u8;
        }
    }
}

pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    let buf = unsafe { &*hapBuffer };
    if buf.buffer.is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 && len > 0 && (buf.len - offset) >= len {
        let dest = unsafe { (buf.buffer as *mut u8).add(offset as usize) };
        unsafe {
            std::ptr::copy_nonoverlapping(data as *const u8, dest, len as usize);
        }
    }
}

pub extern "C" fn HapSetInt32(buffer: *const crate::types::HapBuf, offset: i32, value: i32) {
    if value < 0 {
        return;
    }
    if buffer.is_null() {
        return;
    }
    let buf = unsafe { &*buffer };
    if buf.buffer.is_null() {
        return;
    }
    if offset >= 0 && (buf.len - offset) >= (std::mem::size_of::<i32>() as i32) {
        let ptr = unsafe { (buf.buffer as *mut u8).offset(offset as isize) as *mut ::core::ffi::c_uchar };
        crate::src_app_common::HapPutInt32(ptr, buf.len - offset, value);
    }
}

pub extern "C" fn CreateHapBuffer(hapBuffer: *mut crate::types::HapBuf, len: i32) -> bool {
    if hapBuffer.is_null() || len <= 0 {
        return false;
    }
    let buf = unsafe { libc::malloc(len as usize) };
    if buf.is_null() {
        return false;
    }
    unsafe {
        (*hapBuffer).buffer = buf;
        (*hapBuffer).len = len;
    }
    true
}

pub extern "C" fn ClearHapBuffer(hapBuffer: *mut crate::types::HapBuf) {
    if hapBuffer.is_null() {
        return;
    }
    let buf = unsafe { &mut *hapBuffer };
    if buf.buffer.is_null() {
        return;
    }
    unsafe {
        std::ptr::write_bytes(buf.buffer as *mut u8, 0u8, buf.len as usize);
        libc::free(buf.buffer);
    }
    buf.buffer = std::ptr::null_mut();
    buf.len = 0;
}

fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    let mut mmapInfo = crate::types::MmapInfo {
        mmapPosition: 0,
        readMoreLen: 0,
        mmapSize: 0,
        mapAddr: ::core::ptr::null_mut(),
    };
    let hap_len = unsafe { (*hapFile).len };
    if hap_len <= ::core::mem::size_of::<crate::types::MinEocd>() as i32 {
        return false;
    }
    let ret = crate::src_app_file::HapMMap(hap_len, 0, &mut mmapInfo, hapFile);
    if ret != 0 {
        return false;
    }
    // Construct a safe byte slice for the entire mapped region.
    let file_data = unsafe {
        core::slice::from_raw_parts(mmapInfo.mapAddr.offset(mmapInfo.readMoreLen as isize) as *const u8, hap_len as usize)
    };
    let min_eocd_size = ::core::mem::size_of::<crate::types::MinEocd>() as i32;
    let sizeof_short = ::core::mem::size_of::<::core::ffi::c_short>() as i32;
    let sizeof_int = ::core::mem::size_of::<i32>() as i32;

    // Use safe references to obtain pointers for HapGet calls.
    let short_val = crate::src_app_common::HapGetShort(
        &file_data[(hap_len - sizeof_short) as usize] as *const u8 as *const ::core::ffi::c_uchar,
        sizeof_short,
    );
    let int_val = crate::src_app_common::HapGetInt(
        &file_data[(hap_len - min_eocd_size) as usize] as *const u8 as *const ::core::ffi::c_uchar,
        sizeof_int,
    );
    if short_val == 0 && int_val == crate::types::HAP_EOCD_MAGIC as i32 {
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                file_data.as_ptr().offset((hap_len - min_eocd_size) as isize) as *const crate::types::MinEocd,
                &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd,
                1,
            );
        }
        crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        unsafe { *eocdOffset = hap_len - min_eocd_size; }
        return true;
    }

    let max_read_len_raw = hap_len - min_eocd_size;
    let max_read_len: i32 = if (max_read_len_raw as u32) > crate::types::UINT16_MAX_VALUE {
        crate::types::UINT16_MAX_VALUE as i32
    } else {
        max_read_len_raw
    };
    let search_offset = (hap_len - min_eocd_size - max_read_len) as usize;
    let search_end = search_offset + max_read_len as usize + min_eocd_size as usize;
    let search_region = &file_data[search_offset..search_end];

    for i in 0..max_read_len {
        let off = i as usize;
        let short_val = crate::src_app_common::HapGetShort(
            &search_region[off + min_eocd_size as usize - sizeof_short as usize] as *const u8 as *const ::core::ffi::c_uchar,
            sizeof_short,
        );
        let int_val = crate::src_app_common::HapGetInt(
            &search_region[off] as *const u8 as *const ::core::ffi::c_uchar,
            sizeof_int,
        );
        if (short_val as i32) == (max_read_len - i) && int_val == crate::types::HAP_EOCD_MAGIC as i32 {
            unsafe {
                ::core::ptr::copy_nonoverlapping(
                    search_region.as_ptr().add(off) as *const crate::types::MinEocd,
                    &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd,
                    1,
                );
            }
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            unsafe { *eocdOffset = hap_len - min_eocd_size - (max_read_len - i); }
            return true;
        }
    }

    crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
    false
}

pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    let mut eocdOffset: i32 = 0;
    let mut hapEocd: crate::types::HapEocd = unsafe { std::mem::zeroed() };
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        return false;
    }
    unsafe {
        (*signInfo).hapEocdOffset = eocdOffset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
    }
    let core_dir_offset_ptr: *const i32 = ::core::ptr::addr_of!(hapEocd.eocdHead.coreDirOffset);
    let coreDirOffset = crate::src_app_common::HapGetInt(
        core_dir_offset_ptr as *const ::core::ffi::c_uchar,
        std::mem::size_of::<i32>() as i32,
    );
    unsafe {
        (*signInfo).hapCoreDirOffset = coreDirOffset;
    }
    if coreDirOffset <= 0 || coreDirOffset >= eocdOffset ||
        unsafe { (*signInfo).hapEocdSize <= 0 || (*signInfo).hapEocdOffset <= 0 } {
        return false;
    }
    true
}

pub extern "C" fn ReadFileFullyFromOffset(buffer: *const crate::types::HapBuf, offset: i32, file: *const crate::types::FileRead) -> i32 {
    if buffer.is_null() || file.is_null() {
        return crate::types::DEST_BUFFER_IS_NULL;
    }
    let (buf_ptr, buf_len) = unsafe { ((*buffer).buffer, (*buffer).len) };
    if buf_ptr.is_null() {
        return crate::types::DEST_BUFFER_IS_NULL;
    }
    let (file_len, fp) = unsafe { ((*file).len, (*file).fp) };
    if offset < 0 || offset > file_len {
        return crate::types::READ_OFFSET_OUT_OF_RANGE;
    }
    // Move safe null/range checks before unsafe; keep only lseek and read in unsafe.
    unsafe {
        libc::lseek(fp, offset as i64, libc::SEEK_SET);
        let read_len = libc::read(fp, buf_ptr, buf_len as usize) as i32;
        if read_len != buf_len {
            return crate::types::READ_OFFSET_OUT_OF_RANGE;
        }
        read_len
    }
}
