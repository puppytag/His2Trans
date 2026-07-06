//! Module: src_audio_dma_base
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

pub extern "C" fn AudioDmaBufAlloc(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    match unsafe { (*ops).DmaBufAlloc } {
        Some(callback) => unsafe { callback(data, streamType) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn AudioDmaBufFree(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dma_buf_free = unsafe { (*ops).DmaBufFree };
    if let Some(f) = dma_buf_free {
        unsafe { f(data, streamType) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn AudioDmaRequestChannel(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    if let Some(func) = unsafe { (*ops).DmaRequestChannel } {
        return unsafe { func(data as *const crate::types::PlatformData, streamType) };
    }
    crate::types::HDF_FAILURE as i32
}

pub extern "C" fn AudioDmaConfigChannel(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dma_config_channel = unsafe { (*ops).DmaConfigChannel };
    if let Some(func) = dma_config_channel {
        return unsafe { func(data as *const crate::types::PlatformData, streamType) };
    }
    crate::types::HDF_FAILURE
}

pub extern "C" fn AudioDmaPrep(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if let Some(prep) = unsafe { (*ops).DmaPrep } {
        return unsafe { prep(data as *const crate::types::PlatformData, streamType) };
    }
    crate::types::HDF_FAILURE
}

pub extern "C" fn AudioDmaSubmit(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return HDF_FAILURE;
    }
    match unsafe { (*ops).DmaSubmit } {
        Some(f) => unsafe { f(data as *const crate::types::PlatformData, streamType) },
        None => HDF_FAILURE,
    }
}

pub extern "C" fn AudioDmaPending(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops: *mut crate::types::AudioDmaOps = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let pending_fn = unsafe { (*ops).DmaPending };
    match pending_fn {
        Some(f) => unsafe { f(data, streamType) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn AudioDmaPause(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    match unsafe { (*ops).DmaPause } {
        Some(pause_fn) => unsafe { pause_fn(data, streamType) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn AudioDmaResume(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops = unsafe { (*data).ops };
    if ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    match unsafe { (*ops).DmaResume } {
        Some(resume_fn) => unsafe { resume_fn(data as *const crate::types::PlatformData, streamType) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn AudioDmaPointer(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType, pointer: *mut u32) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ops_ptr = unsafe { (*data).ops };
    if ops_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dma_pointer_fn = unsafe { (*ops_ptr).DmaPointer };
    if let Some(func) = dma_pointer_fn {
        unsafe { func(data, streamType, pointer) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn AudioDmaGetConfigInfo(device: *const crate::types::HdfDeviceObject, data: *mut crate::types::PlatformData) -> i32 {
    if device.is_null() || data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let reg_config = unsafe { (*data).regConfig };
    if !reg_config.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    let size = core::mem::size_of::<crate::types::AudioRegCfgData>();
    let mem = unsafe { libc::calloc(1, size) as *mut crate::types::AudioRegCfgData };
    unsafe { (*data).regConfig = mem; }
    if mem.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let ret = unsafe { crate::compat::AudioGetRegConfig(device, mem) };
    if ret != crate::types::HDF_SUCCESS {
        unsafe { libc::free(mem as *mut core::ffi::c_void); }
        unsafe { (*data).regConfig = core::ptr::null_mut(); }
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioDmaTransferStatusIsNormal(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType) -> bool {
    if data.is_null() {
        return false;
    }
    let data = unsafe { &mut *data };
    if streamType == crate::types::AUDIO_RENDER_STREAM {
        if data.renderBufInfo.rbufOffSet == data.renderBufInfo.wbufOffSet {
            data.renderBufInfo.trafCompCount += 1;
            if data.renderBufInfo.trafCompCount > 12 {
                return false;
            }
        } else {
            data.renderBufInfo.rbufOffSet = data.renderBufInfo.wbufOffSet;
            data.renderBufInfo.trafCompCount = 0;
        }
    } else {
        if data.captureBufInfo.wbufOffSet == data.captureBufInfo.rbufOffSet {
            data.captureBufInfo.trafCompCount += 1;
            if data.captureBufInfo.trafCompCount > 12 {
                return false;
            }
        } else {
            data.captureBufInfo.wbufOffSet = data.captureBufInfo.rbufOffSet;
            data.captureBufInfo.trafCompCount = 0;
        }
    }
    true
}
