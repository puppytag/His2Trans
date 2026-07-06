//! Module: src_audio_dai_base
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

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
// Replaced with a safe lookup function that returns the original C string literals.

fn get_dai_control_name(index: usize) -> *const ::core::ffi::c_char {
    match index {
        0 => b"Main Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        1 => b"Main Capture Volume\0".as_ptr() as *const ::core::ffi::c_char,
        2 => b"Playback Mute\0".as_ptr() as *const ::core::ffi::c_char,
        3 => b"Capture Mute\0".as_ptr() as *const ::core::ffi::c_char,
        4 => b"Mic Left Gain\0".as_ptr() as *const ::core::ffi::c_char,
        5 => b"Mic Right Gain\0".as_ptr() as *const ::core::ffi::c_char,
        6 => b"External Codec Enable\0".as_ptr() as *const ::core::ffi::c_char,
        7 => b"Internally Codec Enable\0".as_ptr() as *const ::core::ffi::c_char,
        8 => b"Render Channel Mode\0".as_ptr() as *const ::core::ffi::c_char,
        9 => b"Captrue Channel Mode\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}

// === C2R_FILE_STATICS_END ===

pub extern "C" fn DaiDataFromCard(card: *const crate::types::AudioCard) -> *mut crate::types::DaiData {
    if card.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8,
                b"DaiDataFromCard\0".as_ptr() as *const i8,
                25_i32,
            );
        }
        return std::ptr::null_mut();
    }
    let rtd = unsafe { (*card).rtd };
    if rtd.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8,
                b"DaiDataFromCard\0".as_ptr() as *const i8,
                25_i32,
            );
        }
        return std::ptr::null_mut();
    }
    let cpuDai = unsafe { (*rtd).cpuDai };
    if cpuDai.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8,
                b"DaiDataFromCard\0".as_ptr() as *const i8,
                25_i32,
            );
        }
        return std::ptr::null_mut();
    }
    return unsafe { (*cpuDai).devData };
}

pub extern "C" fn DaiGetConfigInfo(device: *const crate::types::HdfDeviceObject, data: *mut crate::types::DaiData) -> i32 {
    if device.is_null() || data.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: param is null!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DaiGetConfigInfo\0".as_ptr() as *const ::core::ffi::c_char,
                35i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let reg_config = unsafe { (*data).regConfig };
    if !reg_config.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: g_codecData regConfig has been parsed!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DaiGetConfigInfo\0".as_ptr() as *const ::core::ffi::c_char,
                40i32,
            );
        }
        return crate::types::HDF_SUCCESS;
    }
    let size = std::mem::size_of::<crate::types::AudioRegCfgData>() as crate::types::size_t;
    let new_reg_config = unsafe { OsalMemCalloc(size) as *mut crate::types::AudioRegCfgData };
    unsafe { (*data).regConfig = new_reg_config; }
    if new_reg_config.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: malloc AudioRegCfgData fail!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DaiGetConfigInfo\0".as_ptr() as *const ::core::ffi::c_char,
                46i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    if unsafe { AudioGetRegConfig(device, new_reg_config) } != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: dai GetRegConfig fail!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DaiGetConfigInfo\0".as_ptr() as *const ::core::ffi::c_char,
                51i32,
            );
            OsalMemFree(new_reg_config as *mut ::core::ffi::c_void);
        }
        unsafe { (*data).regConfig = std::ptr::null_mut(); }
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn DaiCheckSampleRate(sampleRates: u32) -> i32 {
    let check = sampleRates == crate::types::AUDIO_SAMPLE_RATE_8000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_12000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_11025
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_16000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_22050
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_24000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_32000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_44100
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_48000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_64000
        || sampleRates == crate::types::AUDIO_SAMPLE_RATE_96000;
    if check {
        crate::types::HDF_SUCCESS
    } else {
        crate::types::HDF_ERR_NOT_SUPPORT
    }
}

pub extern "C" fn DaiSetConfigInfoOfControls(data: *mut crate::types::DaiData) -> i32 {
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dai_data = unsafe { &mut *data };
    let regConfig = dai_data.regConfig;
    if regConfig.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let reg_config = unsafe { &mut *regConfig };
    let arr = &mut reg_config.audioRegParams;
    let regCfgGroup = arr.as_mut_ptr();
    if regCfgGroup.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    let group_patam = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_PATAM_GROUP as usize) };
    let group_cfg = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_CFG_GROUP as usize) };
    if group_patam.is_null() || group_cfg.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    let group_patam_ref = unsafe { &*group_patam };
    let group_cfg_ref = unsafe { &*group_cfg };
    let patRegCfgItemTmp = group_patam_ref.regCfgItem;
    let item = group_cfg_ref.ctrlCfgItem;
    if patRegCfgItemTmp.is_null() || item.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let num_controls = group_cfg_ref.itemNum as i32;
    dai_data.numControls = num_controls;

    let controls = unsafe {
        libc::calloc(
            num_controls as usize,
            std::mem::size_of::<crate::types::AudioKcontrol>(),
        ) as *mut crate::types::AudioKcontrol
    };
    if controls.is_null() {
        return crate::types::HDF_FAILURE;
    }
    dai_data.controls = controls;

    let num = num_controls as usize;
    let item_slice = unsafe { std::slice::from_raw_parts(item, num) };
    let controls_slice = unsafe { std::slice::from_raw_parts_mut(controls, num) };

    for index in 0..num {
        let item_entry = &item_slice[index];
        let iface = item_entry.iface as i32;
        let array_idx = item_entry.arrayIndex as u32;
        if array_idx >= crate::types::AUDIO_CTRL_LIST_MAX {
            unsafe { libc::free(controls as *mut ::core::ffi::c_void); }
            dai_data.controls = std::ptr::null_mut();
            return crate::types::HDF_FAILURE;
        }
        let ctrl = &mut controls_slice[index];
        ctrl.iface = iface;
        ctrl.name = get_dai_control_name(array_idx as usize) as *mut ::core::ffi::c_char;
        ctrl.Info = Some(AudioInfoCtrlOps);
        ctrl.Get = Some(AudioCpuDaiGetCtrlOps);
        ctrl.Set = Some(AudioCpuDaiSetCtrlOps);
        ctrl.privateValue = unsafe { (patRegCfgItemTmp.add(index) as usize) as ::core::ffi::c_ulong };
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn DaiDeviceReadReg(dai: *const crate::types::DaiDevice, reg: u32, val: *mut u32) -> i32 {
    if dai.is_null() || val.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dev_data = unsafe { (*dai).devData };
    if dev_data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let virtual_address = unsafe { (*dev_data).regVirtualAddr };
    let addr = virtual_address + reg as ::core::ffi::c_ulong;
    let value = unsafe { crate::src_audio_platform_base::SysReadl(addr) };
    unsafe { *val = value; }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn DaiDeviceWriteReg(dai: *const crate::types::DaiDevice, reg: u32, value: u32) -> i32 {
    if dai.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dev_data_ptr = unsafe { (*dai).devData };
    if dev_data_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let reg_virtual_addr = unsafe { (*dev_data_ptr).regVirtualAddr };
    let addr = reg_virtual_addr.wrapping_add(reg as ::core::ffi::c_ulong);
    unsafe {
        crate::src_audio_platform_base::SysWritel(addr, value);
    }
    crate::types::HDF_SUCCESS
}
