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
    let hap = unsafe { &*hapBuffer };
    if hap.buffer.is_null() {
        return;
    }
    if offset >= 0 && hap.len - offset >= std::mem::size_of::<::core::ffi::c_char>() as i32 {
        unsafe {
            let ptr = (hap.buffer as *mut ::core::ffi::c_char).offset(offset as isize);
            *ptr = value;
        }
    }
}

pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    let hap = unsafe { &*hapBuffer };
    if hap.buffer.is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 && len > 0 && (hap.len - offset) >= len {
        let dest = unsafe { (hap.buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) as *mut ::core::ffi::c_void };
        let dest_max = (hap.len - offset) as crate::types::size_t;
        let src = data as *const ::core::ffi::c_void;
        let count = len as crate::types::size_t;
        if unsafe { crate::compat::memcpy_s(dest, dest_max, src, count) } != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const _, b"HapPutData\0".as_ptr() as *const _, 50) };
        }
    }
}

pub extern "C" fn HapSetInt32(buffer: *const crate::types::HapBuf, offset: i32, value: i32) {
    if value < 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const ::core::ffi::c_char,
                "[%s:%d]: int32 value of out range: %d\0".as_ptr() as *const ::core::ffi::c_char,
                "HapSetInt32\0".as_ptr() as *const ::core::ffi::c_char,
                58,
                value,
            );
        }
        return;
    }
    if buffer.is_null() {
        return;
    }
    let buf = unsafe { &*buffer };
    if buf.buffer.is_null() {
        return;
    }
    if offset >= 0 && (buf.len - offset) >= (::core::mem::size_of::<i32>() as i32) {
        let ptr = unsafe { buf.buffer.offset(offset as isize) as *mut ::core::ffi::c_uchar };
        crate::src_app_common::HapPutInt32(ptr, buf.len - offset, value);
    }
}

pub extern "C" fn CreateHapBuffer(hapBuffer: *mut crate::types::HapBuf, len: i32) -> bool {
    if hapBuffer.is_null() || len <= 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: create buf fail, buf is null\0".as_ptr() as *const _,
                b"CreateHapBuffer\0".as_ptr() as *const _,
                73,
            );
        }
        return false;
    }
    unsafe {
        (*hapBuffer).buffer = libc::malloc(len as usize) as *mut ::core::ffi::c_void;
        if (*hapBuffer).buffer.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: create buf fail\0".as_ptr() as *const _,
                b"CreateHapBuffer\0".as_ptr() as *const _,
                78,
            );
            return false;
        }
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
        let len = (*hapBuffer).len as usize;
        let _ = crate::compat::memset_s((*hapBuffer).buffer, len as crate::types::size_t, 0, len as crate::types::size_t);
        libc::free((*hapBuffer).buffer);
        (*hapBuffer).buffer = std::ptr::null_mut();
        (*hapBuffer).len = 0;
    }
}

fn GetEocd(hapFile: *const crate::types::FileRead, hapEocd: *mut crate::types::HapEocd, eocdOffset: *mut i32) -> bool {
    use crate::src_app_common::{HapGetInt, HapGetShort};
    use crate::src_app_file::{HapMMap, HapMUnMap};
    let mut mmapInfo = crate::types::MmapInfo {
        mmapPosition: 0,
        readMoreLen: 0,
        mmapSize: 0,
        mapAddr: std::ptr::null_mut(),
    };
    unsafe {
        if (*hapFile).len <= std::mem::size_of::<crate::types::MinEocd>() as i32 {
            return false;
        }
        let ret = HapMMap((*hapFile).len, 0, &mut mmapInfo as *mut _, hapFile);
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: mmap not ok\0".as_ptr() as *const _,
                b"GetEocd\0".as_ptr() as *const _,
                104,
            );
            return false;
        }
        let fileStart = (mmapInfo.mapAddr as *mut u8).offset(mmapInfo.readMoreLen as isize) as *mut i8;
        if HapGetShort(
            fileStart.offset((*hapFile).len as isize - std::mem::size_of::<i16>() as isize) as *const _,
            std::mem::size_of::<i16>() as i32,
        ) == 0
            && HapGetInt(
                fileStart.offset((*hapFile).len as isize - std::mem::size_of::<crate::types::MinEocd>() as isize) as *const _,
                std::mem::size_of::<i32>() as i32,
            ) == crate::types::HAP_EOCD_MAGIC as i32
        {
            if crate::compat::memcpy_s(
                &mut (*hapEocd).eocdHead as *mut _ as *mut _,
                std::mem::size_of::<crate::types::MinEocd>() as u32,
                fileStart.offset((*hapFile).len as isize - std::mem::size_of::<crate::types::MinEocd>() as isize) as *const _,
                std::mem::size_of::<crate::types::MinEocd>() as u32,
            ) != crate::types::EOK as i32
            {
                HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: copy error\0".as_ptr() as *const _,
                    b"GetEocd\0".as_ptr() as *const _,
                    113,
                );
                return false;
            }
            HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
            *eocdOffset = (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32;
            return true;
        }
        let maxReadLen = if ((*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32) as u32 > crate::types::UINT16_MAX_VALUE {
            crate::types::UINT16_MAX_VALUE as i32
        } else {
            (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32
        };
        let fileStart = fileStart.offset(((*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32 - maxReadLen) as isize);
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: maxReadLen %d\0".as_ptr() as *const _,
            b"GetEocd\0".as_ptr() as *const _,
            124,
            maxReadLen,
        );
        for i in 0..maxReadLen {
            if HapGetShort(
                fileStart.offset((i + std::mem::size_of::<crate::types::MinEocd>() as i32 - std::mem::size_of::<i16>() as i32) as isize) as *const _,
                std::mem::size_of::<i16>() as i32,
            ) == (maxReadLen - i) as i16
                && HapGetInt(
                    fileStart.offset(i as isize) as *const _,
                    std::mem::size_of::<i32>() as i32,
                ) == crate::types::HAP_EOCD_MAGIC as i32
            {
                if crate::compat::memcpy_s(
                    &mut (*hapEocd).eocdHead as *mut _ as *mut _,
                    std::mem::size_of::<crate::types::MinEocd>() as u32,
                    fileStart.offset(i as isize) as *const _,
                    std::mem::size_of::<crate::types::MinEocd>() as u32,
                ) != crate::types::EOK as i32
                {
                    HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                    let _ = crate::compat::HiLogPrint(
                        crate::types::LOG_CORE as u32,
                        crate::types::LOG_ERROR as u32,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const _,
                        b"[%s:%d]: copy error\0".as_ptr() as *const _,
                        b"GetEocd\0".as_ptr() as *const _,
                        132,
                    );
                    return false;
                }
                HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_INFO as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: comment num %d\0".as_ptr() as *const _,
                    b"GetEocd\0".as_ptr() as *const _,
                    136,
                    maxReadLen - i,
                );
                *eocdOffset = (*hapFile).len - std::mem::size_of::<crate::types::MinEocd>() as i32 - (maxReadLen - i);
                return true;
            }
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: can not find eocd\0".as_ptr() as *const _,
            b"GetEocd\0".as_ptr() as *const _,
            141,
        );
        HapMUnMap(mmapInfo.mapAddr, mmapInfo.mmapSize);
        false
    }
}

pub extern "C" fn FindSignature(hapFile: *const crate::types::FileRead, signInfo: *mut crate::types::SignatureInfo) -> bool {
    if hapFile.is_null() || signInfo.is_null() {
        return false;
    }
    let mut eocd_offset: i32 = 0;
    let mut hap_eocd = crate::types::HapEocd {
        eocdHead: crate::types::MinEocd {
            magic: 0,
            diskNum: 0,
            startNum: 0,
            coreDirNumOnDisk: 0,
            coreDirNum: 0,
            coreDirSize: 0,
            coreDirOffset: 0,
            commentLen: 0,
        },
        comment: std::ptr::null_mut(),
    };
    if !crate::src_app_centraldirectory::GetEocd(hapFile, &mut hap_eocd, &mut eocd_offset) {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: find Eocd fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                154,
            );
        }
        return false;
    }
    unsafe {
        (*signInfo).hapEocdOffset = eocd_offset;
        (*signInfo).hapEocdSize = (*hapFile).len - eocd_offset;
        let core_dir_offset_bytes = std::ptr::addr_of!(hap_eocd.eocdHead.coreDirOffset) as *const ::core::ffi::c_uchar;
        (*signInfo).hapCoreDirOffset = crate::src_app_common::HapGetInt(
            core_dir_offset_bytes,
            std::mem::size_of::<i32>() as i32,
        );
        if (*signInfo).hapCoreDirOffset <= 0
            || (*signInfo).hapCoreDirOffset >= eocd_offset
            || (*signInfo).hapEocdSize <= 0
            || (*signInfo).hapEocdOffset <= 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: core dir error\0".as_ptr() as *const ::core::ffi::c_char,
                b"FindSignature\0".as_ptr() as *const ::core::ffi::c_char,
                162,
            );
            return false;
        }
    }
    true
}

pub extern "C" fn ReadFileFullyFromOffset(buffer: *const crate::types::HapBuf, offset: i32, file: *const crate::types::FileRead) -> i32 {
    if buffer.is_null() || unsafe { (*buffer).buffer.is_null() } || file.is_null() {
        return DEST_BUFFER_IS_NULL;
    }
    let file_ref = unsafe { &*file };
    if offset < 0 || offset > file_ref.len {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    let seek_ret = unsafe { libc::lseek(file_ref.fp, offset as libc::off_t, SEEK_SET as i32) };
    if seek_ret == -1 {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    let buffer_ref = unsafe { &*buffer };
    let read_len = unsafe {
        libc::read(
            file_ref.fp,
            buffer_ref.buffer as *mut libc::c_void,
            buffer_ref.len as libc::size_t,
        ) as i32
    };
    if read_len != buffer_ref.len {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const libc::c_char,
                "[%s:%d]: file read error %d --- %d\0".as_ptr() as *const libc::c_char,
                "ReadFileFullyFromOffset\0".as_ptr() as *const libc::c_char,
                179,
                read_len,
                buffer_ref.len,
            )
        };
        return READ_OFFSET_OUT_OF_RANGE;
    }
    buffer_ref.len
}
