//! Module: src_audio_dsp_base
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

pub extern "C" fn DspGetServiceName(device: *const crate::types::HdfDeviceObject, drvDspName: *mut *const ::core::ffi::c_char) -> i32 {
    if device.is_null() || drvDspName.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: device is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetServiceName\0".as_ptr() as *const ::core::ffi::c_char,
                21i32,
            );
        }
        return HDF_FAILURE;
    }

    let node = unsafe { (*device).property };
    if node.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: device property is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetServiceName\0".as_ptr() as *const ::core::ffi::c_char,
                27i32,
            );
        }
        return HDF_FAILURE;
    }

    let drsOps = unsafe { DeviceResourceGetIfaceInstance(HDF_CONFIG_SOURCE) };
    if drsOps.is_null() || unsafe { (*drsOps).GetString.is_none() } {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: from resource get drsops failed!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetServiceName\0".as_ptr() as *const ::core::ffi::c_char,
                32i32,
            );
        }
        return HDF_FAILURE;
    }

    let ret = unsafe {
        (*drsOps).GetString.unwrap()(
            node,
            b"serviceName\0".as_ptr() as *const ::core::ffi::c_char,
            drvDspName,
            ::core::ptr::null(),
        )
    };

    if ret != HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: read DspServiceName fail!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetServiceName\0".as_ptr() as *const ::core::ffi::c_char,
                38i32,
            );
        }
        return ret;
    }

    HDF_SUCCESS
}

pub extern "C" fn DspGetDaiName(device: *const crate::types::HdfDeviceObject, drvDaiName: *mut *const ::core::ffi::c_char) -> i32 {
    if device.is_null() || drvDaiName.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is null pointer.\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                52i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let node = unsafe { (*device).property };
    if node.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: drs node is null pointer.\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                58i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let drs_ops = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) };
    if drs_ops.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: drs ops fail!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DspGetDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                63i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let get_string_fn = unsafe { (*drs_ops).GetString };
    match get_string_fn {
        Some(get_string) => {
            let ret = unsafe {
                get_string(
                    node,
                    b"dspDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                    drvDaiName,
                    std::ptr::null(),
                )
            };
            if ret != crate::types::HDF_SUCCESS {
                unsafe {
                    HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_ERROR,
                        0xD002510u32,
                        b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                        b"[%s][line:%d]: read dspDaiName fail!\0".as_ptr() as *const ::core::ffi::c_char,
                        b"DspGetDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                        69i32,
                    );
                }
                return ret;
            }
            crate::types::HDF_SUCCESS
        }
        None => {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s][line:%d]: drs ops fail!\0".as_ptr() as *const ::core::ffi::c_char,
                    b"DspGetDaiName\0".as_ptr() as *const ::core::ffi::c_char,
                    63i32,
                );
            }
            crate::types::HDF_FAILURE
        }
    }
}
