//! Module: src_audio_codec_base
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
// Replaced with safe lookup functions that match the original C string literal arrays.

fn get_codec_control_name(index: usize) -> *const ::core::ffi::c_char {
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
        10 => b"Headphone Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        11 => b"PCM Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        12 => b"PCM Capture Volume\0".as_ptr() as *const ::core::ffi::c_char,
        13 => b"Mono Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        14 => b"Phone Capture Volume\0".as_ptr() as *const ::core::ffi::c_char,
        15 => b"Mic Volume\0".as_ptr() as *const ::core::ffi::c_char,
        16 => b"Surround Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        17 => b"Center/LFE Playback Volume\0".as_ptr() as *const ::core::ffi::c_char,
        18 => b"DAC1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        19 => b"DAC2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        20 => b"DAC3 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        21 => b"DAC4 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        22 => b"ADC1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        23 => b"ADC2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        24 => b"ADC3 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        25 => b"ADC4 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        26 => b"Speaker1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        27 => b"Speaker2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        28 => b"Speaker3 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        29 => b"Speaker4 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        30 => b"MIC1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        31 => b"MIC2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        32 => b"MIC3 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        33 => b"MIC4 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        34 => b"MIC1 Boost Volume\0".as_ptr() as *const ::core::ffi::c_char,
        35 => b"MIC2 Boost Volume\0".as_ptr() as *const ::core::ffi::c_char,
        36 => b"INA1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        37 => b"INB1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        38 => b"INA2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        39 => b"INB2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        40 => b"Lineout1 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        41 => b"Lineout2 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        42 => b"Lineout3 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        43 => b"Lineout4 Volume\0".as_ptr() as *const ::core::ffi::c_char,
        44 => b"Headphone Volume\0".as_ptr() as *const ::core::ffi::c_char,
        45 => b"Receiver Volume\0".as_ptr() as *const ::core::ffi::c_char,
        46 => b"EQ1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        47 => b"EQ2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        48 => b"DAI1 Filter Mode\0".as_ptr() as *const ::core::ffi::c_char,
        49 => b"DAI2 Filter Mode\0".as_ptr() as *const ::core::ffi::c_char,
        50 => b"ADC High Pass Filter Switch\0".as_ptr() as *const ::core::ffi::c_char,
        51 => b"Playback Deemphasis\0".as_ptr() as *const ::core::ffi::c_char,
        52 => b"PGA1 Setting\0".as_ptr() as *const ::core::ffi::c_char,
        53 => b"PGA2 Setting\0".as_ptr() as *const ::core::ffi::c_char,
        54 => b"PGA3 Setting\0".as_ptr() as *const ::core::ffi::c_char,
        55 => b"PGA3 Setting\0".as_ptr() as *const ::core::ffi::c_char,
        56 => b"ADC1 Mute\0".as_ptr() as *const ::core::ffi::c_char,
        57 => b"ADC2 Mute\0".as_ptr() as *const ::core::ffi::c_char,
        58 => b"ADC3 Mute\0".as_ptr() as *const ::core::ffi::c_char,
        59 => b"ADC4 Mute\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}

fn get_sapm_cfg_name(index: usize) -> *const ::core::ffi::c_char {
    match index {
        0 => b"LPGA MIC Switch\0".as_ptr() as *const ::core::ffi::c_char,
        1 => b"RPGA MIC Switch\0".as_ptr() as *const ::core::ffi::c_char,
        2 => b"Dacl enable\0".as_ptr() as *const ::core::ffi::c_char,
        3 => b"Dacr enable\0".as_ptr() as *const ::core::ffi::c_char,
        4 => b"Headphone Playback Switch\0".as_ptr() as *const ::core::ffi::c_char,
        5 => b"PCM Playback Switch\0".as_ptr() as *const ::core::ffi::c_char,
        6 => b"PCM Capture Switch\0".as_ptr() as *const ::core::ffi::c_char,
        7 => b"Mono Playback Switch\0".as_ptr() as *const ::core::ffi::c_char,
        8 => b"Phone Capture Switch\0".as_ptr() as *const ::core::ffi::c_char,
        9 => b"Mic Switch\0".as_ptr() as *const ::core::ffi::c_char,
        10 => b"Stereo Mic Switch\0".as_ptr() as *const ::core::ffi::c_char,
        11 => b"Line HP Swap Switch\0".as_ptr() as *const ::core::ffi::c_char,
        12 => b"Surround Playback Switch\0".as_ptr() as *const ::core::ffi::c_char,
        13 => b"Center/LFE Playback Switch\0".as_ptr() as *const ::core::ffi::c_char,
        14 => b"Capture Source\0".as_ptr() as *const ::core::ffi::c_char,
        15 => b"Mic Boost Switch\0".as_ptr() as *const ::core::ffi::c_char,
        16 => b"DAC1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        17 => b"DAC2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        18 => b"DAC3 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        19 => b"DAC4 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        20 => b"ADC1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        21 => b"ADC2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        22 => b"ADC3 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        23 => b"ADC4 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        24 => b"Speaker1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        25 => b"Speaker2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        26 => b"Speaker3 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        27 => b"Speaker4 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        28 => b"Headphone1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        29 => b"Headphone2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        30 => b"Lineout1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        31 => b"Lineout2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        32 => b"Lineout3 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        33 => b"Lineout4 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        34 => b"Mixer1 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        35 => b"Mixer2 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        36 => b"Mixer3 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        37 => b"Mixer4 Switch\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}

fn get_sapm_comp_name(index: usize) -> *const ::core::ffi::c_char {
    match index {
        0 => b"ADCL\0".as_ptr() as *const ::core::ffi::c_char,
        1 => b"ADCR\0".as_ptr() as *const ::core::ffi::c_char,
        2 => b"DACL\0".as_ptr() as *const ::core::ffi::c_char,
        3 => b"DACR\0".as_ptr() as *const ::core::ffi::c_char,
        4 => b"LPGA\0".as_ptr() as *const ::core::ffi::c_char,
        5 => b"RPGA\0".as_ptr() as *const ::core::ffi::c_char,
        6 => b"SPKL\0".as_ptr() as *const ::core::ffi::c_char,
        7 => b"SPKR\0".as_ptr() as *const ::core::ffi::c_char,
        8 => b"MIC\0".as_ptr() as *const ::core::ffi::c_char,
        9 => b"LOUT\0".as_ptr() as *const ::core::ffi::c_char,
        10 => b"HPL\0".as_ptr() as *const ::core::ffi::c_char,
        11 => b"HPR\0".as_ptr() as *const ::core::ffi::c_char,
        12 => b"Stereo Mixer\0".as_ptr() as *const ::core::ffi::c_char,
        13 => b"Line Mix\0".as_ptr() as *const ::core::ffi::c_char,
        14 => b"Input Mixer\0".as_ptr() as *const ::core::ffi::c_char,
        15 => b"Speaker Mix\0".as_ptr() as *const ::core::ffi::c_char,
        16 => b"Input Mux\0".as_ptr() as *const ::core::ffi::c_char,
        17 => b"AuxOut Mux\0".as_ptr() as *const ::core::ffi::c_char,
        18 => b"SPKL Mux\0".as_ptr() as *const ::core::ffi::c_char,
        19 => b"SPKR Mux\0".as_ptr() as *const ::core::ffi::c_char,
        20 => b"AUXOUTL\0".as_ptr() as *const ::core::ffi::c_char,
        21 => b"AUXOUTR\0".as_ptr() as *const ::core::ffi::c_char,
        22 => b"LINEINL\0".as_ptr() as *const ::core::ffi::c_char,
        23 => b"LINEINR\0".as_ptr() as *const ::core::ffi::c_char,
        24 => b"AUXINL\0".as_ptr() as *const ::core::ffi::c_char,
        25 => b"AUXINR\0".as_ptr() as *const ::core::ffi::c_char,
        26 => b"I2S Mix\0".as_ptr() as *const ::core::ffi::c_char,
        27 => b"AuxI Mix\0".as_ptr() as *const ::core::ffi::c_char,
        28 => b"CaptureL Mix\0".as_ptr() as *const ::core::ffi::c_char,
        29 => b"CaptureR Mix\0".as_ptr() as *const ::core::ffi::c_char,
        30 => b"Mono1 Mixer\0".as_ptr() as *const ::core::ffi::c_char,
        31 => b"Mono2 Mixer\0".as_ptr() as *const ::core::ffi::c_char,
        32 => b"DAC1\0".as_ptr() as *const ::core::ffi::c_char,
        33 => b"DAC2\0".as_ptr() as *const ::core::ffi::c_char,
        34 => b"DAC3\0".as_ptr() as *const ::core::ffi::c_char,
        35 => b"DAC4\0".as_ptr() as *const ::core::ffi::c_char,
        36 => b"ADC1\0".as_ptr() as *const ::core::ffi::c_char,
        37 => b"ADC2\0".as_ptr() as *const ::core::ffi::c_char,
        38 => b"ADC3\0".as_ptr() as *const ::core::ffi::c_char,
        39 => b"ADC4\0".as_ptr() as *const ::core::ffi::c_char,
        40 => b"MIC1\0".as_ptr() as *const ::core::ffi::c_char,
        41 => b"MIC2\0".as_ptr() as *const ::core::ffi::c_char,
        42 => b"MIC3\0".as_ptr() as *const ::core::ffi::c_char,
        43 => b"MIC4\0".as_ptr() as *const ::core::ffi::c_char,
        44 => b"SPK1\0".as_ptr() as *const ::core::ffi::c_char,
        45 => b"SPK2\0".as_ptr() as *const ::core::ffi::c_char,
        46 => b"SPK3\0".as_ptr() as *const ::core::ffi::c_char,
        47 => b"SPK4\0".as_ptr() as *const ::core::ffi::c_char,
        48 => b"DAC Mix\0".as_ptr() as *const ::core::ffi::c_char,
        49 => b"DAC Mux\0".as_ptr() as *const ::core::ffi::c_char,
        50 => b"ADC Mix\0".as_ptr() as *const ::core::ffi::c_char,
        51 => b"ADC Mux\0".as_ptr() as *const ::core::ffi::c_char,
        52 => b"SPKL PGA\0".as_ptr() as *const ::core::ffi::c_char,
        53 => b"SPKR PGA\0".as_ptr() as *const ::core::ffi::c_char,
        54 => b"HPL PGA\0".as_ptr() as *const ::core::ffi::c_char,
        55 => b"HPR PGA\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}

// === C2R_FILE_STATICS_END ===

pub extern "C" fn CodecGetServiceName(device: *const crate::types::HdfDeviceObject, drvCodecName: *mut *const ::core::ffi::c_char) -> i32 {
    if device.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input device para is nullptr.\0" as *const u8 as *const ::core::ffi::c_char,
                b"CodecGetServiceName\0" as *const u8 as *const ::core::ffi::c_char,
                107i32,
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
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: node instance is nullptr.\0" as *const u8 as *const ::core::ffi::c_char,
                b"CodecGetServiceName\0" as *const u8 as *const ::core::ffi::c_char,
                113i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let drsOps = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) as *mut crate::types::DeviceResourceIface };
    if drsOps.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: from resource get drsOps fail!\0" as *const u8 as *const ::core::ffi::c_char,
                b"CodecGetServiceName\0" as *const u8 as *const ::core::ffi::c_char,
                118i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let get_string = unsafe {
        match (*drsOps).GetString {
            Some(func) => func,
            None => {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                    b"[%s][line:%d]: from resource get drsOps fail!\0" as *const u8 as *const ::core::ffi::c_char,
                    b"CodecGetServiceName\0" as *const u8 as *const ::core::ffi::c_char,
                    118i32,
                );
                return crate::types::HDF_FAILURE;
            }
        }
    };
    let ret = unsafe {
        get_string(
            node,
            b"serviceName\0" as *const u8 as *const ::core::ffi::c_char,
            drvCodecName,
            std::ptr::null(),
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: read codecServiceName fail!\0" as *const u8 as *const ::core::ffi::c_char,
                b"CodecGetServiceName\0" as *const u8 as *const ::core::ffi::c_char,
                124i32,
            );
        }
        return ret;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecGetDaiName(device: *const crate::types::HdfDeviceObject, drvDaiName: *mut *const ::core::ffi::c_char) -> i32 {
    if device.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let node = unsafe { (*device).property };
    if node.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let drs_ops = unsafe { DeviceResourceGetIfaceInstance(crate::types::HDF_CONFIG_SOURCE) as *mut crate::types::DeviceResourceIface };
    if drs_ops.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let get_string_fn = unsafe {
        match (*drs_ops).GetString {
            Some(f) => f,
            None => return crate::types::HDF_FAILURE,
        }
    };
    let ret = unsafe {
        get_string_fn(
            node,
            b"codecDaiName\0".as_ptr() as *const ::core::ffi::c_char,
            drvDaiName,
            ::core::ptr::null(),
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecGetConfigInfo(device: *const crate::types::HdfDeviceObject, codecData: *mut crate::types::CodecData) -> i32 {
    if device.is_null() || codecData.is_null() {
        unsafe {
            let _ = HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: param is null!\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"CodecGetConfigInfo\0").unwrap().as_ptr(),
                165i32);
        }
        return crate::types::HDF_FAILURE;
    }

    let reg_config_ptr = unsafe { (*codecData).regConfig };
    if !reg_config_ptr.is_null() {
        unsafe {
            let _ = HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: g_codecData regConfig has been parsed!\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"CodecGetConfigInfo\0").unwrap().as_ptr(),
                170i32);
        }
        return crate::types::HDF_SUCCESS;
    }

    let reg_config = unsafe {
        OsalMemCalloc((std::mem::size_of::<crate::types::AudioRegCfgData>() as usize).try_into().unwrap()) as *mut crate::types::AudioRegCfgData
    };
    unsafe { (*codecData).regConfig = reg_config; }
    if reg_config.is_null() {
        unsafe {
            let _ = HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: malloc AudioRegCfgData fail!\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"CodecGetConfigInfo\0").unwrap().as_ptr(),
                176i32);
        }
        return crate::types::HDF_FAILURE;
    }

    if unsafe { AudioGetRegConfig(device, reg_config) } != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: AudioGetRegConfig fail!\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"CodecGetConfigInfo\0").unwrap().as_ptr(),
                181i32);
            OsalMemFree(reg_config as *mut ::core::ffi::c_void);
            (*codecData).regConfig = std::ptr::null_mut();
        }
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDaiGetPortConfigInfo(device: *const crate::types::HdfDeviceObject, codecData: *mut crate::types::DaiData) -> i32 {
    if device.is_null() || codecData.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: <device> or <codecData> is a null pointer!\0".as_ptr() as *const i8,
                b"CodecDaiGetPortConfigInfo\0".as_ptr() as *const i8,
                193i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe { AudioGetPortConfig(device, &mut (*codecData).portInfo as *mut crate::types::AudioPortInfo) }
}

fn SapmCtrlToSapmComp(sapmComponents: *mut crate::types::AudioSapmComponent, sapmCompItem: *const crate::types::AudioSapmCtrlConfig, index: u16)-> i32 {
    if sapmComponents.is_null() || sapmCompItem.is_null() {
        // AUDIO_DRIVER_LOG_ERR not available; original log call omitted
        return crate::types::HDF_FAILURE;
    }

    let sapm_comp = unsafe { &mut *sapmComponents.add(index as usize) };
    let sapm_item = unsafe { &*sapmCompItem.add(index as usize) };

    sapm_comp.componentName =
        get_sapm_comp_name(sapm_item.compNameIndex as usize) as *mut i8;
    sapm_comp.reg = sapm_item.reg;
    sapm_comp.sapmType = sapm_item.sapmType as crate::types::AudioSapmType;
    sapm_comp.mask = sapm_item.mask;
    sapm_comp.shift = sapm_item.shift;
    sapm_comp.invert = sapm_item.invert;
    sapm_comp.kcontrolsNum = sapm_item.kcontrolsNum as i32;

    crate::types::HDF_SUCCESS
}

fn CodecSetSapmKcontrolInfo(audioSapmControls: *mut crate::types::AudioKcontrol, regCfgGroup: *mut *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if audioSapmControls.is_null() || regCfgGroup.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecSetSapmKcontrolInfo\0".as_ptr() as *const ::core::ffi::c_char,
                229i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let group_sapm_patam = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_SAPM_PATAM_GROUP as usize) };
    let group_sapm_cfg = unsafe { *regCfgGroup.add(crate::types::AUDIO_SAPM_CFG_GROUP as usize) };

    if group_sapm_patam.is_null() || group_sapm_cfg.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: codec config hcs configuration file is no configuration information for sapm\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecSetSapmKcontrolInfo\0".as_ptr() as *const ::core::ffi::c_char,
                233i32,
            );
        }
        return crate::types::HDF_SUCCESS;
    }

    let sapm_ctrl_item: *mut crate::types::AudioControlConfig = unsafe { (*group_sapm_cfg).ctrlCfgItem };
    let ctl_sapm_reg_cfg_item: *mut crate::types::AudioMixerControl = unsafe { (*group_sapm_patam).regCfgItem };

    if sapm_ctrl_item.is_null() || ctl_sapm_reg_cfg_item.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: sapmCtrlItem, ctlSapmRegCfgItem is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecSetSapmKcontrolInfo\0".as_ptr() as *const ::core::ffi::c_char,
                240i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let mut ctl_reg_enum_cfg_item: *mut crate::types::AudioEnumCtrlConfig = std::ptr::null_mut();
    let group_sapm_mux = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_SAPM_PATAM_MUX_GROUP as usize) };

    if !group_sapm_mux.is_null() {
        ctl_reg_enum_cfg_item = unsafe { (*group_sapm_mux).regEnumCfgItem };
        if ctl_reg_enum_cfg_item.is_null() {
            unsafe {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510_u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s][line:%d]: ctlRegEnumCfgItem is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                    b"CodecSetSapmKcontrolInfo\0".as_ptr() as *const ::core::ffi::c_char,
                    247i32,
                );
            }
            return crate::types::HDF_FAILURE;
        }
    }

    let item_num = unsafe { (*group_sapm_cfg).itemNum };
    if item_num == 0 {
        return crate::types::HDF_SUCCESS;
    }
    let ctrl_slice = unsafe { std::slice::from_raw_parts(sapm_ctrl_item, item_num as usize) };
    let kctrl_slice = unsafe { std::slice::from_raw_parts_mut(audioSapmControls, item_num as usize) };
    for index in 0..item_num as usize {
        let sapm_type = ctrl_slice[index].type_;

        if sapm_type == crate::types::AUDIO_CONTROL_MIXER as u16 {
            let ctrl_item = &ctrl_slice[index];
            let audio_ctrl = &mut kctrl_slice[index];
            audio_ctrl.iface = ctrl_item.iface as i32;
            let arr_index = ctrl_item.arrayIndex as usize;
            audio_ctrl.name = get_sapm_cfg_name(arr_index) as *mut ::core::ffi::c_char;
            audio_ctrl.privateValue =
                ctl_sapm_reg_cfg_item.wrapping_add(index) as usize as ::core::ffi::c_ulong;
            audio_ctrl.Info = Some(AudioInfoCtrlOps);
            audio_ctrl.Get = Some(AudioCodecSapmGetCtrlOps);
            audio_ctrl.Set = Some(AudioCodecSapmSetCtrlOps);
        } else if sapm_type == crate::types::AUDIO_CONTROL_MUX as u16 {
            let ctrl_item = &ctrl_slice[index];
            let audio_ctrl = &mut kctrl_slice[index];
            audio_ctrl.iface = ctrl_item.iface as i32;
            let arr_index = ctrl_item.arrayIndex as usize;
            audio_ctrl.name = get_sapm_cfg_name(arr_index) as *mut ::core::ffi::c_char;
            audio_ctrl.privateValue =
                ctl_reg_enum_cfg_item.wrapping_add(index) as usize as ::core::ffi::c_ulong;
            audio_ctrl.Info = Some(AudioInfoEnumCtrlOps);
            audio_ctrl.Get = Some(AudioCodecSapmGetEnumCtrlOps);
            audio_ctrl.Set = Some(AudioCodecSapmSetEnumCtrlOps);
        }
    }

    crate::types::HDF_SUCCESS
}

fn CodecSetSapmConfigInfo(codeData: *mut crate::types::CodecData, regCfgGroup: *mut *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if codeData.is_null() || regCfgGroup.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let node_comp = unsafe { *regCfgGroup.offset(crate::types::AUDIO_SAPM_COMP_GROUP as isize) };
    let node_cfg = unsafe { *regCfgGroup.offset(crate::types::AUDIO_SAPM_CFG_GROUP as isize) };
    if node_comp.is_null() || node_cfg.is_null() {
        let func = b"CodecSetSapmConfigInfo\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: codec config hcs configuration file is no configuration information for sapm\0".as_ptr() as *const ::core::ffi::c_char,
                func,
                282i32,
            );
        }
        return crate::types::HDF_SUCCESS;
    }
    let node_comp_ref = unsafe { &*node_comp };
    let sapmCompItem = node_comp_ref.sapmCompItem;
    if sapmCompItem.is_null() {
        let func = b"CodecSetSapmConfigInfo\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: sapmCompItem is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                func,
                287i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let cfg_item_count = unsafe {
        let node_cfg_ref = &*node_cfg;
        node_cfg_ref.itemNum as usize
    };
    let audioSapmControls = unsafe {
        OsalMemCalloc(
            (cfg_item_count * ::core::mem::size_of::<crate::types::AudioKcontrol>()) as crate::types::size_t,
        ) as *mut crate::types::AudioKcontrol
    };
    if audioSapmControls.is_null() {
        let func = b"CodecSetSapmConfigInfo\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: OsalMemCalloc failed.\0".as_ptr() as *const ::core::ffi::c_char,
                func,
                293i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let audio_sapm_slice = unsafe { std::slice::from_raw_parts_mut(audioSapmControls, cfg_item_count) };
    if crate::src_audio_codec_base::CodecSetSapmKcontrolInfo(audioSapmControls, regCfgGroup) != crate::types::HDF_SUCCESS {
        unsafe { OsalMemFree(audioSapmControls as *mut ::core::ffi::c_void); }
        return crate::types::HDF_FAILURE;
    }
    let code_data = unsafe { &mut *codeData };
    code_data.numSapmComponent = node_comp_ref.itemNum as ::core::ffi::c_int;
    let num_comps = code_data.numSapmComponent;
    let comps_ptr = unsafe {
        OsalMemCalloc(
            (num_comps as usize * ::core::mem::size_of::<crate::types::AudioSapmComponent>()) as crate::types::size_t,
        ) as *mut crate::types::AudioSapmComponent
    };
    code_data.sapmComponents = comps_ptr;
    if comps_ptr.is_null() {
        unsafe { OsalMemFree(audioSapmControls as *mut ::core::ffi::c_void); }
        let func = b"CodecSetSapmConfigInfo\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: OsalMemCalloc failed.\0".as_ptr() as *const ::core::ffi::c_char,
                func,
                305i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let sapm_comp_slice = unsafe { std::slice::from_raw_parts(sapmCompItem, num_comps as usize) };
    let comps_slice = unsafe { std::slice::from_raw_parts_mut(code_data.sapmComponents, num_comps as usize) };
    for i in 0..(num_comps as usize) {
        let index = i as u16;
        if crate::src_audio_codec_base::SapmCtrlToSapmComp(
            code_data.sapmComponents,
            sapmCompItem,
            index,
        ) != crate::types::HDF_SUCCESS
        {
            unsafe {
                OsalMemFree(audioSapmControls as *mut ::core::ffi::c_void);
                OsalMemFree(code_data.sapmComponents as *mut ::core::ffi::c_void);
                code_data.sapmComponents = ::core::ptr::null_mut();
            }
            return crate::types::HDF_FAILURE;
        }
        let comp_item = &sapm_comp_slice[i];
        if comp_item.kcontrolsNum != 0 {
            let news = comp_item.kcontrolNews;
            let src = &mut audio_sapm_slice[(news - 1) as usize] as *mut crate::types::AudioKcontrol;
            comps_slice[i].kcontrolNews = src;
        }
    }
    crate::types::HDF_SUCCESS
}

fn CodecSetKcontrolInfo(codeData: *mut crate::types::CodecData, regCfgGroup: *mut *mut crate::types::AudioRegCfgGroupNode)-> i32 {
    if codeData.is_null() || regCfgGroup.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let cfg_group_ptr = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_CFG_GROUP as usize) };
    let param_group_ptr = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_PATAM_GROUP as usize) };
    if cfg_group_ptr.is_null() || param_group_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let cfg_group_ref = unsafe { &*cfg_group_ptr };
    let param_group_ref = unsafe { &*param_group_ptr };
    let comp_item = cfg_group_ref.ctrlCfgItem;
    let ctl_reg_cfg_item = param_group_ref.regCfgItem;
    if comp_item.is_null() || ctl_reg_cfg_item.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let mux_group_ptr = unsafe { *regCfgGroup.add(crate::types::AUDIO_CTRL_PATAM_MUX_GROUP as usize) };
    let mut enum_ctl_reg_cfg_item: *mut crate::types::AudioEnumCtrlConfig = std::ptr::null_mut();
    if !mux_group_ptr.is_null() {
        let mux_group_ref = unsafe { &*mux_group_ptr };
        enum_ctl_reg_cfg_item = mux_group_ref.regEnumCfgItem;
        if enum_ctl_reg_cfg_item.is_null() {
            return crate::types::HDF_FAILURE;
        }
    }

    let code_data = unsafe { &mut *codeData };
    let num_controls = code_data.numControls as usize;
    let controls_slice = unsafe { std::slice::from_raw_parts_mut(code_data.controls, num_controls) };
    let comp_slice = unsafe { std::slice::from_raw_parts(comp_item, num_controls) };
    let mut enum_index: usize = 0;

    for i in 0..num_controls {
        let ctrl = &mut controls_slice[i];
        let item = &comp_slice[i];
        if item.type_ == crate::types::AUDIO_CONTROL_MIXER as u16 {
            ctrl.iface = item.iface as i32;
            ctrl.name = get_codec_control_name(item.arrayIndex as usize) as *mut ::core::ffi::c_char;
            ctrl.Info = Some(AudioInfoCtrlOps);
            ctrl.privateValue = unsafe { ctl_reg_cfg_item.add(i) as usize as ::core::ffi::c_ulong };
            if item.enable != 0 {
                ctrl.Get = Some(AudioCodecGetCtrlOps);
                ctrl.Set = Some(AudioCodecSetCtrlOps);
            }
        } else if item.type_ == crate::types::AUDIO_CONTROL_MUX as u16 {
            ctrl.iface = item.iface as i32;
            ctrl.name = get_codec_control_name(item.arrayIndex as usize) as *mut ::core::ffi::c_char;
            ctrl.Info = Some(AudioInfoEnumCtrlOps);
            ctrl.privateValue = unsafe { enum_ctl_reg_cfg_item.add(enum_index) as usize as ::core::ffi::c_ulong };
            if item.enable != 0 {
                ctrl.Get = Some(AudioCodecGetEnumCtrlOps);
                ctrl.Set = Some(AudioCodecSetEnumCtrlOps);
            }
            enum_index += 1;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecSetConfigInfoOfControls(codeData: *mut crate::types::CodecData, daiData: *mut crate::types::DaiData) -> i32 {
    if codeData.is_null() || daiData.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let code_data = unsafe { &mut *codeData };
    let reg_config_ptr = code_data.regConfig;
    if reg_config_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let reg_config = unsafe { &mut *reg_config_ptr };
    let audio_id_info = &reg_config.audioIdInfo;
    let reg_cfg_group = reg_config.audioRegParams.as_mut_ptr();
    if reg_cfg_group.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dai_data = unsafe { &mut *daiData };
    dai_data.regCfgGroup = reg_cfg_group;
    code_data.regCfgGroup = reg_cfg_group;
    let ctrl_cfg_group_ptr = unsafe { *reg_cfg_group.add(crate::types::AUDIO_CTRL_CFG_GROUP as usize) };
    if ctrl_cfg_group_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let num_controls = unsafe { (*ctrl_cfg_group_ptr).itemNum as i32 };
    code_data.numControls = num_controls;
    let layout = ::std::alloc::Layout::from_size_align(
        (num_controls as usize) * ::std::mem::size_of::<crate::types::AudioKcontrol>(),
        ::std::mem::align_of::<crate::types::AudioKcontrol>(),
    )
    .unwrap();
    let controls_ptr = unsafe { ::std::alloc::alloc_zeroed(layout) as *mut crate::types::AudioKcontrol };
    if controls_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    code_data.controls = controls_ptr;
    if crate::src_audio_codec_base::CodecSetKcontrolInfo(codeData, reg_cfg_group) != crate::types::HDF_SUCCESS {
        unsafe {
            ::std::alloc::dealloc(controls_ptr as *mut u8, layout);
            code_data.controls = ::core::ptr::null_mut();
        }
        return crate::types::HDF_FAILURE;
    }
    let chip_id_reg = audio_id_info.chipIdRegister;
    let _chip_id_size = audio_id_info.chipIdSize;
    code_data.virtualAddress = chip_id_reg as crate::types::c_ulong;
    if crate::src_audio_codec_base::CodecSetSapmConfigInfo(codeData, reg_cfg_group) != crate::types::HDF_SUCCESS {
        unsafe {
            ::std::alloc::dealloc(controls_ptr as *mut u8, layout);
            code_data.controls = ::core::ptr::null_mut();
        }
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecSetCtlFunc(codeData: *mut crate::types::CodecData, controlType: crate::types::AudioControlType, getCtrl: *const ::core::ffi::c_void, setCtrl: *const ::core::ffi::c_void) -> i32 {
    if codeData.is_null() || getCtrl.is_null() || setCtrl.is_null() {
        unsafe {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"[%s][line:%d]: input para is NULL.\0" as *const u8 as *const i8,
                b"CodecSetCtlFunc\0" as *const u8 as *const i8,
                433i32,
            );
        }
        return HDF_FAILURE;
    }
    let code_data = unsafe { &*codeData };
    if code_data.regConfig.is_null() {
        unsafe {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"[%s][line:%d]: regConfig is NULL.\0" as *const u8 as *const i8,
                b"CodecSetCtlFunc\0" as *const u8 as *const i8,
                435i32,
            );
        }
        return HDF_FAILURE;
    }
    let reg_config = unsafe { &mut *code_data.regConfig };
    let reg_cfg_group = reg_config.audioRegParams.as_mut_ptr();
    if reg_cfg_group.is_null() {
        unsafe {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"[%s][line:%d]: regCfgGroup is NULL.\0" as *const u8 as *const i8,
                b"CodecSetCtlFunc\0" as *const u8 as *const i8,
                438i32,
            );
        }
        return HDF_FAILURE;
    }
    let group_ptr = unsafe { *reg_cfg_group.offset(AUDIO_CTRL_CFG_GROUP as isize) };
    if group_ptr.is_null() {
        unsafe {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"[%s][line:%d]: ctrlCfgItem group is NULL.\0" as *const u8 as *const i8,
                b"CodecSetCtlFunc\0" as *const u8 as *const i8,
                440i32,
            );
        }
        return HDF_FAILURE;
    }
    let comp_item = unsafe { (*group_ptr).ctrlCfgItem };
    if comp_item.is_null() {
        unsafe {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"[%s][line:%d]: compItem is NULL.\0" as *const u8 as *const i8,
                b"CodecSetCtlFunc\0" as *const u8 as *const i8,
                444i32,
            );
        }
        return HDF_FAILURE;
    }
    let num_controls = code_data.numControls as usize;
    let controls = code_data.controls;
    for index in 0..num_controls {
        let item = unsafe { &*comp_item.offset(index as isize) };
        if item.type_ as crate::types::AudioControlType == controlType && item.enable == 0 {
            let control = unsafe { &mut *controls.offset(index as isize) };
            control.Get = unsafe { Some(std::mem::transmute(getCtrl)) };
            control.Set = unsafe { Some(std::mem::transmute(setCtrl)) };
        }
    }
    HDF_SUCCESS
}

fn CodecI2cRelease(msgs: *mut crate::types::I2cMsg, msgSize: i16, i2cHandle: crate::types::DevHandle) {
    if msgs.is_null() {
        return;
    }
    if msgSize == 0 {
        if !unsafe { (*msgs).buf.is_null() } {
            unsafe { OsalMemFree((*msgs).buf as *mut ::core::ffi::c_void); }
            unsafe { (*msgs).buf = std::ptr::null_mut(); }
        }
    } else if msgSize == 1 {
        if !unsafe { (*msgs).buf.is_null() } {
            unsafe { OsalMemFree((*msgs).buf as *mut ::core::ffi::c_void); }
            unsafe { (*msgs).buf = std::ptr::null_mut(); }
        }
    } else {
        if !unsafe { (*msgs).buf.is_null() } {
            unsafe { OsalMemFree((*msgs).buf as *mut ::core::ffi::c_void); }
            unsafe { (*msgs).buf = std::ptr::null_mut(); }
        }
        if !unsafe { (*msgs.offset(1)).buf.is_null() } {
            unsafe { OsalMemFree((*msgs.offset(1)).buf as *mut ::core::ffi::c_void); }
            unsafe { (*msgs.offset(1)).buf = std::ptr::null_mut(); }
        }
    }

    if !i2cHandle.is_null() {
        unsafe { I2cClose(i2cHandle); }
    }
}

fn CodecI2cMsgFill(i2cTransferParam: *mut crate::types::I2cTransferParam, regAttr: *const crate::types::AudioAddrConfig, rwFlag: u16, regs: *mut u8, msgs: *mut crate::types::I2cMsg)-> i32 {
    let mut msg_buf: *mut u8 = std::ptr::null_mut();

    if i2cTransferParam.is_null() || regAttr.is_null() || regs.is_null() || msgs.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    if rwFlag != 0 && rwFlag != (crate::types::I2C_FLAG_READ as u16) {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let i2c_param = unsafe { &*i2cTransferParam };
    let reg_attr = unsafe { &*regAttr };

    unsafe {
        *regs = reg_attr.addr as u8;
        (*msgs).addr = i2c_param.i2cDevAddr;
        (*msgs).flags = 0;
        (*msgs).len = i2c_param.i2cRegDataLen.wrapping_add(1);
    }

    if rwFlag == 0 {
        // write
        let size = (i2c_param.i2cRegDataLen as usize) + 1;
        msg_buf = unsafe { OsalMemCalloc(size.try_into().unwrap()) as *mut u8 };
        if msg_buf.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }

        unsafe { *msg_buf = *regs; }
        {
            let buf_slice = unsafe { std::slice::from_raw_parts_mut(msg_buf, size) };
            let len = i2c_param.i2cRegDataLen;
            if len == 1 {
                buf_slice[1] = reg_attr.value as u8;
            } else if len == 2 {
                buf_slice[1] = (reg_attr.value >> 8) as u8;
                buf_slice[2] = (reg_attr.value & 0xFF) as u8;
            } else {
                return crate::types::HDF_FAILURE;
            }
        }
        unsafe { (*msgs).buf = msg_buf; }
    } else {
        // read
        let size = i2c_param.i2cRegDataLen as usize;
        msg_buf = unsafe { OsalMemCalloc(size.try_into().unwrap()) as *mut u8 };
        if msg_buf.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }

        unsafe {
            (*msgs.offset(0)).len = 1;
            (*msgs.offset(0)).buf = regs;
            (*msgs.offset(1)).addr = i2c_param.i2cDevAddr;
            (*msgs.offset(1)).flags = crate::types::I2C_FLAG_READ as u16;
            (*msgs.offset(1)).len = i2c_param.i2cRegDataLen;
            (*msgs.offset(1)).buf = msg_buf;
        }
    }

    crate::types::HDF_SUCCESS
}

fn CodecI2cTransfer(i2cTransferParam: *mut crate::types::I2cTransferParam, regAttr: *mut crate::types::AudioAddrConfig, rwFlag: u16)-> i32 {
    use crate::types::{I2cMsg, I2C_FLAG_READ, HDF_SUCCESS, HDF_FAILURE, HDF_ERR_INVALID_PARAM};

    if i2cTransferParam.is_null() || regAttr.is_null() || rwFlag > 1 {
        return HDF_ERR_INVALID_PARAM;
    }
    let i2c_param = unsafe { &*i2cTransferParam };
    let mut msgs: [I2cMsg; 2] = [
        I2cMsg { addr: 0, buf: std::ptr::null_mut(), len: 0, flags: 0 },
        I2cMsg { addr: 0, buf: std::ptr::null_mut(), len: 0, flags: 0 },
    ];
    let mut regs: [u8; 1] = [0; 1];
    let mut transferMsgCount: i16 = 1;

    let i2cBusNumber = i2c_param.i2cBusNumber;
    let i2cHandle = unsafe { I2cOpen(i2cBusNumber as i16) };
    if i2cHandle.is_null() {
        return HDF_FAILURE;
    }
    if rwFlag == (I2C_FLAG_READ as u16) {
        transferMsgCount = 2;
    }
    let ret = unsafe {
        crate::src_audio_codec_base::CodecI2cMsgFill(
            i2cTransferParam,
            regAttr as *const crate::types::AudioAddrConfig,
            rwFlag,
            regs.as_mut_ptr(),
            msgs.as_mut_ptr(),
        )
    };
    if ret != HDF_SUCCESS {
        unsafe { I2cClose(i2cHandle); }
        return HDF_FAILURE;
    }
    let ret = unsafe { I2cTransfer(i2cHandle, msgs.as_mut_ptr(), transferMsgCount) };
    if ret != transferMsgCount as i32 {
        unsafe {
            crate::src_audio_codec_base::CodecI2cRelease(
                msgs.as_mut_ptr(),
                transferMsgCount,
                i2cHandle,
            );
        }
        return HDF_FAILURE;
    }
    if rwFlag == (I2C_FLAG_READ as u16) {
        let regDataLen = i2c_param.i2cRegDataLen;
        let reg_attr = unsafe { &mut *regAttr };
        if regDataLen == 1 {
            reg_attr.value = unsafe { (*msgs[1].buf) as u32 };
        } else if regDataLen == 2 {
            unsafe {
                let b0 = *msgs[1].buf.offset(0);
                let b1 = *msgs[1].buf.offset(1);
                reg_attr.value = ((b0 as u32) << 8) | (b1 as u32);
            }
        } else {
            unsafe { I2cClose(i2cHandle); }
            return HDF_FAILURE;
        }
    }
    unsafe {
        crate::src_audio_codec_base::CodecI2cRelease(
            msgs.as_mut_ptr(),
            transferMsgCount,
            i2cHandle,
        );
    }
    HDF_SUCCESS
}

pub extern "C" fn CodecDeviceRegI2cRead(codec: *const crate::types::CodecDevice, reg: u32, value: *mut u32) -> i32 {
    if codec.is_null() || value.is_null() {
        eprintln!("[CodecDeviceRegI2cRead][line:604]: input para is NULL.");
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_data = unsafe { (*codec).devData };
    if dev_data.is_null() {
        eprintln!("[CodecDeviceRegI2cRead][line:604]: input para is NULL.");
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_data_ref = unsafe { &*dev_data };
    let i2c_transfer_param = dev_data_ref.privateParam as *mut crate::types::I2cTransferParam;
    if i2c_transfer_param.is_null() {
        eprintln!("[CodecDeviceRegI2cRead][line:610]: codec i2cTransferParam is NULL.");
        return crate::types::HDF_FAILURE;
    }
    let mut reg_attr = crate::types::AudioAddrConfig {
        addr: (reg & 0xFF) as u32,
        value: 0,
    };
    let ret = crate::src_audio_codec_base::CodecI2cTransfer(
        i2c_transfer_param,
        &mut reg_attr as *mut crate::types::AudioAddrConfig,
        crate::types::I2C_FLAG_READ as u16,
    );
    if ret != crate::types::HDF_SUCCESS {
        eprintln!("[CodecDeviceRegI2cRead][line:618]: failed.");
        return crate::types::HDF_FAILURE;
    }
    unsafe { *value = reg_attr.value; }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDeviceRegI2cWrite(codec: *const crate::types::CodecDevice, reg: u32, value: u32) -> i32 {
    if codec.is_null() || unsafe { (*codec).devData.is_null() } {
        return -1;
    }
    let devData = unsafe { (*codec).devData };
    let privateParam = unsafe { (*devData).privateParam };
    if privateParam.is_null() {
        return -1;
    }
    let i2cTransferParam = privateParam as *mut crate::types::I2cTransferParam;
    let mut regAttr = crate::types::AudioAddrConfig {
        addr: (reg & 0xFF) as u32,
        value: (value & 0xFFFF) as u32,
    };
    let ret = crate::src_audio_codec_base::CodecI2cTransfer(
        i2cTransferParam,
        &mut regAttr,
        0,
    );
    if ret != 0 {
        return -1;
    }
    0
}

pub extern "C" fn CodecDaiRegI2cRead(dai: *const crate::types::DaiDevice, reg: u32, value: *mut u32) -> i32 {
    if dai.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecDaiRegI2cRead\0".as_ptr() as *const ::core::ffi::c_char,
                660i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if value.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecDaiRegI2cRead\0".as_ptr() as *const ::core::ffi::c_char,
                660i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let dev_data = unsafe { (*dai).devData };
    if dev_data.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecDaiRegI2cRead\0".as_ptr() as *const ::core::ffi::c_char,
                660i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let dev_data_ref = unsafe { &*dev_data };
    let i2c_transfer_param = dev_data_ref.privateParam as *mut crate::types::I2cTransferParam;
    if i2c_transfer_param.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: codec dai i2cTransferParam is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecDaiRegI2cRead\0".as_ptr() as *const ::core::ffi::c_char,
                666i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let mut reg_attr = crate::types::AudioAddrConfig { addr: reg, value: 0 };
    let ret = crate::src_audio_codec_base::CodecI2cTransfer(
        i2c_transfer_param,
        &mut reg_attr,
        crate::types::I2C_FLAG_READ as u16,
    );
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: CodecI2cTransfer failed.\0".as_ptr() as *const ::core::ffi::c_char,
                b"CodecDaiRegI2cRead\0".as_ptr() as *const ::core::ffi::c_char,
                674i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        *value = reg_attr.value;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDaiRegI2cWrite(dai: *const crate::types::DaiDevice, reg: u32, value: u32) -> i32 {
    if dai.is_null() || unsafe { (*dai).devData.is_null() } {
        return crate::types::HDF_FAILURE;
    }

    let dev_data = unsafe { (*dai).devData };
    let i2c_transfer_param = unsafe { (*dev_data).privateParam as *mut crate::types::I2cTransferParam };
    if i2c_transfer_param.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let mut reg_attr = crate::types::AudioAddrConfig {
        addr: (reg as u8) as u32,
        value: (value as u16) as u32,
    };

    let ret = crate::src_audio_codec_base::CodecI2cTransfer(
        i2c_transfer_param,
        &mut reg_attr as *mut crate::types::AudioAddrConfig,
        0u16,
    );
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDeviceReadReg(codec: *const crate::types::CodecDevice, reg: u32, value: *mut u32) -> i32 {
    if codec.is_null() || value.is_null() {
        unsafe {
            let fmt = b"HDF_AUDIO_KADM[%s][line:%d]: param val is null.\0";
            let func = b"CodecDeviceReadReg\0";
            let _ = libc::printf(fmt.as_ptr() as *const i8, func.as_ptr() as *const i8, 713i32);
        }
        return crate::types::HDF_FAILURE;
    }
    let dev_data = unsafe { (*codec).devData };
    if dev_data.is_null() {
        unsafe {
            let fmt = b"HDF_AUDIO_KADM[%s][line:%d]: param val is null.\0";
            let func = b"CodecDeviceReadReg\0";
            let _ = libc::printf(fmt.as_ptr() as *const i8, func.as_ptr() as *const i8, 713i32);
        }
        return crate::types::HDF_FAILURE;
    }
    let virtual_address: usize = unsafe { (*dev_data).virtualAddress as usize };
    let read_ptr = (virtual_address + reg as usize) as *mut u32;
    let read_val = unsafe { core::ptr::read_volatile(read_ptr) };
    unsafe {
        core::arch::asm!("dsb", options(nostack));
        *value = read_val;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDeviceWriteReg(codec: *const crate::types::CodecDevice, reg: u32, value: u32) -> i32 {
    if codec.is_null() || unsafe { (*codec).devData.is_null() } {
        return HDF_FAILURE;
    }
    let virtual_address = unsafe { (*(*codec).devData).virtualAddress };
    let addr = virtual_address + reg as ::core::ffi::c_ulong;
    unsafe {
        crate::src_audio_platform_base::SysWritel(addr, value);
    }
    HDF_SUCCESS
}

pub extern "C" fn CodecDeviceInitRegConfig(device: *const crate::types::CodecDevice) -> i32 {
    if device.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dev_data = unsafe { (*device).devData };
    if dev_data.is_null() { return crate::types::HDF_FAILURE; }
    let write_fn = unsafe { (*dev_data).Write };
    if write_fn.is_none() { return crate::types::HDF_FAILURE; }
    let reg_cfg_group = unsafe { (*dev_data).regCfgGroup };
    if reg_cfg_group.is_null() { return crate::types::HDF_FAILURE; }
    let init_group_ptr = unsafe { *reg_cfg_group.offset(crate::types::AUDIO_INIT_GROUP as isize) };
    if init_group_ptr.is_null() { return crate::types::HDF_FAILURE; }
    let init_group = unsafe { &*init_group_ptr };
    let init_cfg = init_group.addrCfgItem;
    if init_cfg.is_null() { return crate::types::HDF_FAILURE; }
    let item_num = init_group.itemNum as usize;
    let write = write_fn.unwrap();
    for index in 0..item_num {
        let cfg = unsafe { &*init_cfg.add(index) };
        let ret = unsafe { write(device, cfg.addr, cfg.value) };
        if ret != crate::types::HDF_SUCCESS { return crate::types::HDF_FAILURE; }
        unsafe { OsalMSleep(10u32); }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn CodecDaiDeviceStartupRegConfig(device: *const crate::types::DaiDevice) -> i32 {
    extern "C" {
        fn AudioDaiRegUpdate(
            dai: *const crate::types::DaiDevice,
            mixerCtrl: *mut crate::types::AudioMixerControl,
        ) -> i32;
    }

    if device.is_null() || unsafe { (*device).devData.is_null() } {
        return crate::types::HDF_FAILURE;
    }
    let dev_data = unsafe { (*device).devData };
    let reg_cfg_group = unsafe { (*dev_data).regCfgGroup };
    if reg_cfg_group.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let group_index = crate::types::AUDIO_DAI_STARTUP_PATAM_GROUP as usize;
    let node_ptr = unsafe { *reg_cfg_group.add(group_index) };
    if node_ptr.is_null() { return crate::types::HDF_FAILURE; }
    let startup_reg_cfg_item = unsafe { (*node_ptr).regCfgItem };
    let reg_cfg_item_count = unsafe { (*node_ptr).itemNum as u16 };
    if startup_reg_cfg_item.is_null() { return crate::types::HDF_FAILURE; }
    let mut index: u16 = 0;
    while index < reg_cfg_item_count {
        let ret = unsafe { AudioDaiRegUpdate(device, startup_reg_cfg_item.offset(index as isize)) };
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
        index += 1;
    }
    crate::types::HDF_SUCCESS
}
