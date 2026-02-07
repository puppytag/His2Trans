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
    unsafe {
        if (*hapBuffer).buffer.is_null() {
            return;
        }
        if offset >= 0 && ((*hapBuffer).len - offset >= std::mem::size_of::<::core::ffi::c_char>() as i32) {
            let ptr = ((*hapBuffer).buffer as *mut ::core::ffi::c_char).offset(offset as isize);
            *ptr = value;
        }
    }
}

pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    
    let buffer = unsafe { (*hapBuffer).buffer };
    let buf_len = unsafe { (*hapBuffer).len };
    
    if buffer.is_null() {
        return;
    }
    
    if !data.is_null() && offset >= 0 && len > 0 && (buf_len - offset) >= len {
        let dest = unsafe { (buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) };
        let dest_max = (buf_len - offset) as crate::types::size_t;
        let ret = unsafe { crate::compat::memcpy_s(dest as *mut ::core::ffi::c_void, dest_max, data as *const ::core::ffi::c_void, len as crate::types::size_t) };
        if ret != crate::types::EOK as i32 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const ::core::ffi::c_char,
                    b"HapPutData\0".as_ptr() as *const ::core::ffi::c_char,
                    50i32,
                );
            }
        }
    }
}

pub extern "C" fn HapSetInt32(buffer: *const crate::types::HapBuf, offset: i32, value: i32) {
    if value < 0 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: int32 value of out range: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"HapSetInt32\0".as_ptr() as *const ::core::ffi::c_char,
                58i32,
                value,
            );
        }
        return;
    }
    if buffer.is_null() {
        return;
    }
    unsafe {
        if (*buffer).buffer.is_null() {
            return;
        }
        if (offset >= 0) && (((*buffer).len - offset) >= (std::mem::size_of::<i32>() as i32)) {
            crate::src_app_common::HapPutInt32(
                ((*buffer).buffer as *mut ::core::ffi::c_uchar).offset(offset as isize),
                (*buffer).len - offset,
                value,
            );
        }
    }
}

pub extern "C" fn CreateHapBuffer(hapBuffer: *mut crate::types::HapBuf, len: i32) -> bool {
    if hapBuffer.is_null() || len <= 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: create buf fail, buf is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"CreateHapBuffer\0".as_ptr() as *const ::core::ffi::c_char,
                73i32,
            );
        }
        return false;
    }
    let buffer = unsafe { libc::malloc(len as usize) };
    if buffer.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: create buf fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"CreateHapBuffer\0".as_ptr() as *const ::core::ffi::c_char,
                78i32,
            );
        }
        return false;
    }
    unsafe {
        (*hapBuffer).buffer = buffer;
        (*hapBuffer).len = len;
    }
    true
}

pub extern "C" fn ClearHapBuffer(hapBuffer: *mut crate::types::HapBuf) {
    if hapBuffer.is_null() {
        return;
    }
    unsafe {
        if (*hapBuffer).buffer.is_null() {
            return;
        }
        let _ = crate::compat::memset_s(
            (*hapBuffer).buffer,
            (*hapBuffer).len as crate::types::size_t,
            0,
            (*hapBuffer).len as crate::types::size_t,
        );
        libc::free((*hapBuffer).buffer);
        (*hapBuffer).buffer = std::ptr::null_mut();
        (*hapBuffer).len = 0;
    }
}

fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    unsafe {
        let mut mmapInfo: crate::types::MmapInfo = std::mem::zeroed();
        
        let min_eocd_size = std::mem::size_of::<crate::types::MinEocd>() as i32;
        
        if (*hapFile).len <= min_eocd_size {
            return false;
        }
        
        let ret = crate::src_app_file::HapMMap((*hapFile).len, 0, &mut mmapInfo, hapFile);
        if ret != crate::types::V_OK as i32 {
            return false;
        }
        
        let file_start = mmapInfo.mapAddr.offset(mmapInfo.readMoreLen as isize);
        let file_len = (*hapFile).len;
        
        let short_size = std::mem::size_of::<i16>() as i32;
        let int_size = std::mem::size_of::<i32>() as i32;
        
        let short_check = crate::src_app_common::HapGetShort(
            file_start.offset((file_len - short_size) as isize) as *const u8,
            short_size
        );
        let magic_check = crate::src_app_common::HapGetInt(
            file_start.offset((file_len - min_eocd_size) as isize) as *const u8,
            int_size
        );
        
        if short_check == 0 && magic_check == crate::types::HAP_EOCD_MAGIC as i32 {
            let src = file_start.offset((file_len - min_eocd_size) as isize);
            let dest = &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut ::core::ffi::c_void;
            let ret = memcpy_s(dest, min_eocd_size as u32, src as *const ::core::ffi::c_void, min_eocd_size as u32);
            if ret != 0 {
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                return false;
            }
            crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            *eocdOffset = file_len - min_eocd_size;
            return true;
        }
        
        let max_read_len: i32 = if (file_len - min_eocd_size) as u32 > crate::types::UINT16_MAX_VALUE {
            crate::types::UINT16_MAX_VALUE as i32
        } else {
            file_len - min_eocd_size
        };
        
        let search_start = file_start.offset((file_len - min_eocd_size - max_read_len) as isize);
        
        for i in 0..max_read_len {
            let comment_len_check = crate::src_app_common::HapGetShort(
                search_start.offset((i + min_eocd_size - short_size) as isize) as *const u8,
                short_size
            );
            let magic_check = crate::src_app_common::HapGetInt(
                search_start.offset(i as isize) as *const u8,
                int_size
            );
            
            if comment_len_check as i32 == (max_read_len - i) && magic_check == crate::types::HAP_EOCD_MAGIC as i32 {
                let src = search_start.offset(i as isize);
                let dest = &mut (*hapEocd).eocdHead as *mut crate::types::MinEocd as *mut ::core::ffi::c_void;
                let ret = memcpy_s(dest, min_eocd_size as u32, src as *const ::core::ffi::c_void, min_eocd_size as u32);
                if ret != 0 {
                    crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                    return false;
                }
                crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                *eocdOffset = file_len - min_eocd_size - (max_read_len - i);
                return true;
            }
        }
        
        crate::src_app_file::HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        false
    }
}

pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    
    let mut eocdOffset: i32 = 0;
    let mut hapEocd: crate::types::HapEocd = unsafe { std::mem::zeroed() };
    
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hapEocd, &mut eocdOffset) {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find Eocd fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                154i32,
            );
        }
        return false;
    }
    
    unsafe {
        (*signInfo).hapEocdOffset = eocdOffset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocdOffset;
        
        // Copy the field to a local variable to avoid unaligned reference
        let core_dir_offset_ptr = std::ptr::addr_of!(hapEocd.eocdHead.coreDirOffset);
        let core_dir_offset_val = std::ptr::read_unaligned(core_dir_offset_ptr);
        
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            (&core_dir_offset_val) as *const i32 as *const ::core::ffi::c_uchar,
            std::mem::size_of::<i32>() as i32,
        );
        
        if (*signInfo).hapCoreDirOffset <= 0 
            || (*signInfo).hapCoreDirOffset >= eocdOffset
            || (*signInfo).hapEocdSize <= 0 
            || (*signInfo).hapEocdOffset <= 0 
        {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: core dir error\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                162i32,
            );
            return false;
        }
    }
    
    true
}

pub extern "C" fn ReadFileFullyFromOffset(buffer: *const crate::types::HapBuf, offset: i32, file: *const crate::types::FileRead) -> i32 {
    unsafe {
        if buffer.is_null() || (*buffer).buffer.is_null() || file.is_null() {
            return crate::types::DEST_BUFFER_IS_NULL;
        }
        if offset < 0 || offset > (*file).len {
            return crate::types::READ_OFFSET_OUT_OF_RANGE;
        }
        libc::lseek((*file).fp, offset as libc::off_t, crate::types::SEEK_SET as i32);
        let read_len = libc::read((*file).fp, (*buffer).buffer, (*buffer).len as usize) as i32;
        if read_len != (*buffer).len {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file read error %d --- %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"ReadFileFullyFromOffset\0".as_ptr() as *const ::core::ffi::c_char,
                179i32,
                read_len,
                (*buffer).len,
            );
            return crate::types::READ_OFFSET_OUT_OF_RANGE;
        }
        (*buffer).len
    }
}
