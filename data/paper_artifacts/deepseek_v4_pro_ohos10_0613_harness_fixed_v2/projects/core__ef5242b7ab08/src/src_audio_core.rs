//! Module: src_audio_core
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

// -- Narrow unsafe helpers for DList insertion and pointer deref -----------------

/// Remove `entry` from a circular doubly-linked list.
/// SAFETY: both pointers must be valid, entry must be currently in a list.
#[inline]
pub(crate) unsafe fn dlist_remove(entry: *mut DListHead) {
    if !(*entry).prev.is_null() {
        (*(*entry).prev).next = (*entry).next;
    }
    if !(*entry).next.is_null() {
        (*(*entry).next).prev = (*entry).prev;
    }
    (*entry).prev = std::ptr::null_mut();
    (*entry).next = std::ptr::null_mut();
}

/// Insert `entry` after `head` in a circular doubly-linked list.
/// SAFETY: both pointers must be valid and point to properly initialized DListHead fields.
#[inline]
unsafe fn dlist_insert_after(head: *mut DListHead, entry: *mut DListHead) {
    (*entry).next = (*head).next;
    (*entry).prev = head;
    (*(*head).next).prev = entry;
    (*head).next = entry;
}

/// Create a &mut PlatformDevice and write the mandatory fields; return list entry pointer.
/// SAFETY: caller must ensure `dev` is a valid, aligned, non-null pointer to allocated PlatformDevice;
/// `platformData` must be a valid pointer to PlatformData.
#[inline]
unsafe fn platform_device_set_fields(
    dev: *mut PlatformDevice,
    platformData: *mut PlatformData,
    device: *mut HdfDeviceObject,
) -> *mut DListHead {
    let dev_ref = &mut *dev;
    dev_ref.devPlatformName = (*platformData).drvPlatformName;
    dev_ref.devData = platformData;
    dev_ref.device = device;
    core::ptr::addr_of_mut!(dev_ref.list)
}

/// Create a &mut DaiDevice and write mandatory fields; return list entry pointer.
#[inline]
unsafe fn dai_device_set_fields(
    dev: *mut DaiDevice,
    daiData: *mut DaiData,
    device: *mut HdfDeviceObject,
) -> *mut DListHead {
    let dev_ref = &mut *dev;
    dev_ref.devDaiName = (*daiData).drvDaiName;
    dev_ref.devData = daiData;
    dev_ref.device = device;
    core::ptr::addr_of_mut!(dev_ref.list)
}

/// Create a &mut CodecDevice and write mandatory fields; return list entry pointer.
#[inline]
unsafe fn codec_device_set_fields(
    dev: *mut CodecDevice,
    codecData: *mut CodecData,
    device: *mut HdfDeviceObject,
) -> *mut DListHead {
    let dev_ref = &mut *dev;
    dev_ref.devCodecName = (*codecData).drvCodecName;
    dev_ref.devData = codecData;
    dev_ref.device = device;
    core::ptr::addr_of_mut!(dev_ref.list)
}

/// Create a &mut DspDevice and write mandatory fields; return list entry pointer.
#[inline]
unsafe fn dsp_device_set_fields(
    dev: *mut DspDevice,
    dspData: *mut DspData,
    device: *mut HdfDeviceObject,
) -> *mut DListHead {
    let dev_ref = &mut *dev;
    dev_ref.devDspName = (*dspData).drvDspName;
    dev_ref.devData = dspData;
    dev_ref.device = device;
    core::ptr::addr_of_mut!(dev_ref.list)
}

// -- Iterator helpers for intrusive DList containers -------------------------------

/// Walk the platformController list, calling `f` for each PlatformDevice.
/// The closure receives a mutable pointer; if it returns `true`, iteration stops early.
/// SAFETY: `head` must point to a valid, initialized DListHead circular list.
#[inline]
unsafe fn for_each_platform_device<F: FnMut(*mut PlatformDevice) -> bool>(head: *mut DListHead, mut f: F) {
    let offset = unsafe {
        let null = std::ptr::null::<PlatformDevice>();
        core::ptr::addr_of!((*null).list) as usize
    };
    let mut node = unsafe { (*head).next };
    while node != head {
        let dev = unsafe { (node as *mut u8).sub(offset) as *mut PlatformDevice };
        if f(dev) {
            break;
        }
        node = unsafe { (*node).next };
    }
}

/// Walk the daiController list, calling `f` for each DaiDevice.
#[inline]
unsafe fn for_each_dai_device<F: FnMut(*mut DaiDevice) -> bool>(head: *mut DListHead, mut f: F) {
    let offset = unsafe {
        let null = std::ptr::null::<DaiDevice>();
        core::ptr::addr_of!((*null).list) as usize
    };
    let mut node = unsafe { (*head).next };
    while node != head {
        let dev = unsafe { (node as *mut u8).sub(offset) as *mut DaiDevice };
        if f(dev) {
            break;
        }
        node = unsafe { (*node).next };
    }
}

/// Walk the codecController list, calling `f` for each CodecDevice.
#[inline]
unsafe fn for_each_codec_device<F: FnMut(*mut CodecDevice) -> bool>(head: *mut DListHead, mut f: F) {
    let offset = unsafe {
        let null = std::ptr::null::<CodecDevice>();
        core::ptr::addr_of!((*null).list) as usize
    };
    let mut node = unsafe { (*head).next };
    while node != head {
        let dev = unsafe { (node as *mut u8).sub(offset) as *mut CodecDevice };
        if f(dev) {
            break;
        }
        node = unsafe { (*node).next };
    }
}

/// Walk the dspController list, calling `f` for each DspDevice.
#[inline]
unsafe fn for_each_dsp_device<F: FnMut(*mut DspDevice) -> bool>(head: *mut DListHead, mut f: F) {
    let offset = unsafe {
        let null = std::ptr::null::<DspDevice>();
        core::ptr::addr_of!((*null).list) as usize
    };
    let mut node = unsafe { (*head).next };
    while node != head {
        let dev = unsafe { (node as *mut u8).sub(offset) as *mut DspDevice };
        if f(dev) {
            break;
        }
        node = unsafe { (*node).next };
    }
}

pub extern "C" fn AudioSocRegisterPlatform(device: *mut crate::types::HdfDeviceObject, platformData: *mut crate::types::PlatformData) -> i32 {
    let func_name = b"AudioSocRegisterPlatform\0".as_ptr() as *const ::core::ffi::c_char;
    let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char;

    if device.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag,
                b"[%s][line:%d]: Input params check error: device is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                func_name,
                41i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    if platformData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag,
                b"[%s][line:%d]: Input params check error: platformData is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                func_name,
                45i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let size = std::mem::size_of::<crate::types::PlatformDevice>() as u32;
    let platformDevice = unsafe { OsalMemCalloc(size) as *mut crate::types::PlatformDevice };
    if platformDevice.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag,
                b"[%s][line:%d]: Malloc platformDevice device fail!\0".as_ptr() as *const ::core::ffi::c_char,
                func_name,
                51i32,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    // Narrow unsafe: use helper for field writes + list insertion
    let entry: *mut crate::types::DListHead = unsafe {
        platform_device_set_fields(platformDevice, platformData, device)
    };
    unsafe {
        dlist_insert_after(
            crate::globals::platformController.get_ptr(),
            entry,
        );
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            tag,
            b"[%s][line:%d]: Register [%s] success.\0".as_ptr() as *const ::core::ffi::c_char,
            func_name,
            60i32,
            (*platformDevice).devPlatformName,
        );
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSocRegisterDai(device: *mut crate::types::HdfDeviceObject, daiData: *mut crate::types::DaiData) -> i32 {
    if device.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Input params check error: device is NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"AudioSocRegisterDai\0" as *const u8 as *const ::core::ffi::c_char,
                69i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if daiData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Input params check error: daiData is NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"AudioSocRegisterDai\0" as *const u8 as *const ::core::ffi::c_char,
                73i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dai: *mut crate::types::DaiDevice = unsafe {
        crate::compat::OsalMemCalloc(::core::mem::size_of::<crate::types::DaiDevice>().try_into().unwrap())
            as *mut crate::types::DaiDevice
    };
    if dai.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Malloc dai device fail!\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"AudioSocRegisterDai\0" as *const u8 as *const ::core::ffi::c_char,
                79i32,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    // Narrow unsafe: use helper for field writes + list insertion
    let entry: *mut crate::types::DListHead = unsafe {
        dai_device_set_fields(dai, daiData, device)
    };
    unsafe {
        dlist_insert_after(
            crate::globals::daiController.get_ptr(),
            entry,
        );
    }
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
            b"[%s][line:%d]: Register [%s] success.\0" as *const u8 as *const ::core::ffi::c_char,
            b"AudioSocRegisterDai\0" as *const u8 as *const ::core::ffi::c_char,
            87i32,
            (*dai).devDaiName,
        );
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioRegisterCodec(device: *mut crate::types::HdfDeviceObject, codecData: *mut crate::types::CodecData, daiData: *mut crate::types::DaiData) -> i32 {
    let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
    let func = std::ffi::CStr::from_bytes_with_nul(b"AudioRegisterCodec\0").unwrap();

    if device.is_null() {
        let fmt = std::ffi::CStr::from_bytes_with_nul(
            b"[%s][line:%d]: Input params check error: device is NULL.\0",
        ).unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                98i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if codecData.is_null() {
        let fmt = std::ffi::CStr::from_bytes_with_nul(
            b"[%s][line:%d]: Input params check error: codecData is NULL.\0",
        ).unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                102i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if daiData.is_null() {
        let fmt = std::ffi::CStr::from_bytes_with_nul(
            b"[%s][line:%d]: Input params check error: daiData is NULL.\0",
        ).unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                106i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let codec = unsafe {
        OsalMemCalloc((core::mem::size_of::<crate::types::CodecDevice>() as usize).try_into().unwrap())
            as *mut crate::types::CodecDevice
    };
    if codec.is_null() {
        let fmt = std::ffi::CStr::from_bytes_with_nul(
            b"[%s][line:%d]: Malloc codec device fail!\0",
        ).unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                112i32,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    // Narrow unsafe: field writes via helper
    let _entry: *mut crate::types::DListHead = unsafe {
        codec_device_set_fields(codec, codecData, device)
    };

    let ret = crate::src_audio_core::AudioSocRegisterDai(device, daiData);
    if ret != crate::types::HDF_SUCCESS {
        let fmt = std::ffi::CStr::from_bytes_with_nul(
            b"[%s][line:%d]: Register dai device fail ret=%d\0",
        ).unwrap();
        unsafe {
            OsalIoUnmap((*(*codec).devData).virtualAddress as *mut core::ffi::c_void);
            OsalMemFree(codec as *mut core::ffi::c_void);
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                124i32,
                ret,
            );
        }
        return crate::types::HDF_ERR_IO;
    }

    // DList insertion
    unsafe {
        dlist_insert_after(
            crate::globals::codecController.get_ptr(),
            core::ptr::addr_of_mut!((*codec).list),
        );
    }

    let fmt = std::ffi::CStr::from_bytes_with_nul(
        b"[%s][line:%d]: Register [%s] success.\0",
    ).unwrap();
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD002510u32,
            tag.as_ptr(),
            fmt.as_ptr(),
            func.as_ptr(),
            128i32,
            (*codec).devCodecName,
        );
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioRegisterDsp(device: *mut crate::types::HdfDeviceObject, dspData: *mut crate::types::DspData, DaiData: *mut crate::types::DaiData) -> i32 {
    let mut dsp_dev: *mut crate::types::DspDevice = core::ptr::null_mut();

    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if dspData.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if DaiData.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    dsp_dev = unsafe {
        crate::compat::OsalMemCalloc((core::mem::size_of::<crate::types::DspDevice>() as usize).try_into().unwrap()) as *mut crate::types::DspDevice
    };
    if dsp_dev.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    // Narrow unsafe: field writes via helper
    let _entry: *mut crate::types::DListHead = unsafe {
        dsp_device_set_fields(dsp_dev, dspData, device)
    };

    let ret = crate::src_audio_core::AudioSocRegisterDai(device, DaiData);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            crate::compat::OsalMemFree(dsp_dev as *mut core::ffi::c_void);
        }
        return crate::types::HDF_ERR_IO;
    }

    // DList insertion
    unsafe {
        dlist_insert_after(
            crate::globals::dspController.get_ptr(),
            core::ptr::addr_of_mut!((*dsp_dev).list),
        );
    }

    crate::types::HDF_SUCCESS
}

fn AudioSeekPlatformDevice(rtd: *mut crate::types::AudioRuntimeDeivces, configData: *const crate::types::AudioConfigData)-> i32 {
    if rtd.is_null() || configData.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let platform_name = unsafe { (*configData).platformName };
    if platform_name.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let target_cstr = unsafe { std::ffi::CStr::from_ptr(platform_name) };
    unsafe {
        for_each_platform_device(
            crate::globals::platformController.get_ptr(),
            |platform| {
                let dev_name = (*platform).devPlatformName;
                if !dev_name.is_null() {
                    let dev_cstr = std::ffi::CStr::from_ptr(dev_name);
                    if dev_cstr == target_cstr {
                        (*rtd).platform = platform;
                        return true;
                    }
                }
                false
            },
        );
    }
    crate::types::HDF_SUCCESS
}

fn AudioSeekCpuDaiDevice(rtd: *mut crate::types::AudioRuntimeDeivces, configData: *const crate::types::AudioConfigData)-> i32 {
    if rtd.is_null() || configData.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let cfg_cpu_dai_name = unsafe { (*configData).cpuDaiName };
    if cfg_cpu_dai_name.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let head = crate::globals::daiController.get_ptr();
    if unsafe { (*head).next == head } {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        for_each_dai_device(head, |dai| {
            let dev_dai_name = (*dai).devDaiName;
            if !dev_dai_name.is_null() {
                let dev_cstr = ::core::ffi::CStr::from_ptr(dev_dai_name);
                let cfg_cstr = ::core::ffi::CStr::from_ptr(cfg_cpu_dai_name);
                if dev_cstr == cfg_cstr {
                    (*rtd).cpuDai = dai;
                    return true;
                }
            }
            false
        });
    }
    crate::types::HDF_SUCCESS
}

fn AudioSeekCodecDevice(rtd: *mut crate::types::AudioRuntimeDeivces, configData: *const crate::types::AudioConfigData)-> i32 {
    use std::ffi::CStr;
    if rtd.is_null() || configData.is_null() {
        eprintln!("HDF_AUDIO_KADM [AudioSeekCodecDevice:233]: Input params check error");
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let config = unsafe { &*configData };
    if config.codecName.is_null() {
        eprintln!("HDF_AUDIO_KADM [AudioSeekCodecDevice:237]: Input devicesName check error: configData->codecName is NULL.");
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if config.codecDaiName.is_null() {
        eprintln!("HDF_AUDIO_KADM [AudioSeekCodecDevice:241]: Input devicesName check error: configData->codecDaiName is NULL.");
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let codec_target_name: &CStr = unsafe { CStr::from_ptr(config.codecName) };
    let codec_dai_target_name: &CStr = unsafe { CStr::from_ptr(config.codecDaiName) };

    let codec_controller_ptr = crate::globals::codecController.get_ptr();
    let dai_controller_ptr = crate::globals::daiController.get_ptr();

    unsafe {
        for_each_codec_device(codec_controller_ptr, |codec| {
            let codec_ref = &*codec;
            if !codec_ref.devCodecName.is_null() {
                let dev_name = CStr::from_ptr(codec_ref.devCodecName);
                if dev_name == codec_target_name {
                    (*rtd).codec = codec;
                    for_each_dai_device(dai_controller_ptr, |dai| {
                        let dai_ref = &*dai;
                        if !dai_ref.device.is_null() && dai_ref.device == codec_ref.device
                            && !dai_ref.devDaiName.is_null()
                        {
                            let dai_name = CStr::from_ptr(dai_ref.devDaiName);
                            if dai_name == codec_dai_target_name {
                                (*rtd).codecDai = dai;
                                return true;
                            }
                        }
                        false
                    });
                    return true;
                }
            }
            false
        });
    }
    crate::types::HDF_SUCCESS
}

fn AudioSeekDspDevice(rtd: *mut crate::types::AudioRuntimeDeivces, configData: *const crate::types::AudioConfigData)-> i32 {
    if rtd.is_null() || configData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Input params check error\0" as *const u8 as *const core::ffi::c_char,
                b"AudioSeekDspDevice\0" as *const u8 as *const core::ffi::c_char,
                270i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let dsp_name = unsafe { (*configData).dspName };
    if dsp_name.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Input devicesName check error: configData->dspName is NULL.\0" as *const u8
                    as *const core::ffi::c_char,
                b"AudioSeekDspDevice\0" as *const u8 as *const core::ffi::c_char,
                274i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let dsp_dai_name = unsafe { (*configData).dspDaiName };
    if dsp_dai_name.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Input devicesName check error: configData->dspDaiName is NULL.\0"
                    as *const u8 as *const core::ffi::c_char,
                b"AudioSeekDspDevice\0" as *const u8 as *const core::ffi::c_char,
                278i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let head = crate::globals::dspController.get_ptr();
    let dai_head = crate::globals::daiController.get_ptr();

    unsafe {
        for_each_dsp_device(head, |dsp| {
            let name = core::ffi::CStr::from_ptr((*dsp).devDspName);
            let target = core::ffi::CStr::from_ptr(dsp_name);
            if name == target {
                (*rtd).dsp = dsp;
                for_each_dai_device(dai_head, |dai| {
                    if !(*dai).device.is_null() && (*dai).device == (*dsp).device {
                        let dai_dev_name = core::ffi::CStr::from_ptr((*dai).devDaiName);
                        let target_dai_name = core::ffi::CStr::from_ptr(dsp_dai_name);
                        if dai_dev_name == target_dai_name {
                            (*rtd).dspDai = dai;
                            return true;
                        }
                    }
                    false
                });
                return true;
            }
            false
        });
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioBindDaiLink(audioCard: *mut crate::types::AudioCard, configData: *const crate::types::AudioConfigData) -> i32 {
    if audioCard.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if configData.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let rtd_ptr = unsafe { OsalMemCalloc(std::mem::size_of::<crate::types::AudioRuntimeDeivces>() as u32) as *mut crate::types::AudioRuntimeDeivces };
    if rtd_ptr.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    unsafe {
        (*audioCard).rtd = rtd_ptr;
        (*(*audioCard).rtd).complete = 0;
    }
    if crate::src_audio_core::AudioSeekPlatformDevice(unsafe { (*audioCard).rtd }, configData) == crate::types::HDF_SUCCESS {
        // Log omitted
    }
    if crate::src_audio_core::AudioSeekCpuDaiDevice(unsafe { (*audioCard).rtd }, configData) == crate::types::HDF_SUCCESS {
        // Log omitted
    }
    if crate::src_audio_core::AudioSeekCodecDevice(unsafe { (*audioCard).rtd }, configData) == crate::types::HDF_SUCCESS {
        // Log omitted
    }
    if crate::src_audio_core::AudioSeekDspDevice(unsafe { (*audioCard).rtd }, configData) == crate::types::HDF_SUCCESS {
        // Log omitted
    }
    unsafe {
        (*(*audioCard).rtd).complete = 1;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioDaiReadReg(dai: *const crate::types::DaiDevice, reg: u32, val: *mut u32) -> i32 {
    if dai.is_null() || val.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_data = unsafe { (*dai).devData };
    if dev_data.is_null() || unsafe { (*dev_data).Read.is_none() } {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let read_fn = unsafe { (*dev_data).Read.unwrap() };
    let ret = unsafe { read_fn(dai, reg, val) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn AudioDaiWriteReg(dai: *const crate::types::DaiDevice, reg: u32, val: u32) -> i32 {
    if dai.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_data = unsafe { (*dai).devData };
    if dev_data.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let write_opt = unsafe { (*dev_data).Write };
    if write_opt.is_none() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let write = write_opt.unwrap();
    let ret = unsafe { write(dai, reg, val) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecReadReg(codec: *const crate::types::CodecDevice, reg: u32, val: *mut u32) -> i32 {
    if codec.is_null() || val.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                b"[%s][line:%d]: Input param codec is NULL.\0" as *const u8 as *const i8,
                b"AudioCodecReadReg\0" as *const u8 as *const i8,
                381i32,
            );
        }
        return HDF_ERR_INVALID_OBJECT;
    }

    let dev_data: *mut crate::types::CodecData = unsafe { (*codec).devData };
    if dev_data.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                b"[%s][line:%d]: dev_data is NULL.\0" as *const u8 as *const i8,
                b"AudioCodecReadReg\0" as *const u8 as *const i8,
                385i32,
            );
        }
        return HDF_ERR_INVALID_OBJECT;
    }

    let read_fn = match unsafe { (*dev_data).Read } {
        Some(func) => func,
        None => {
            unsafe {
                HiLogPrint(
                    LOG_CORE as u32,
                    LOG_ERROR as u32,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                    b"[%s][line:%d]: Read is NULL.\0" as *const u8 as *const i8,
                    b"AudioCodecReadReg\0" as *const u8 as *const i8,
                    389i32,
                );
            }
            return HDF_ERR_INVALID_OBJECT;
        }
    };

    let ret: i32 = unsafe { (read_fn)(codec, reg, val) };
    if ret != HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                b"[%s][line:%d]: Codec device read fail.\0" as *const u8 as *const i8,
                b"AudioCodecReadReg\0" as *const u8 as *const i8,
                387i32,
            );
        }
        return HDF_FAILURE;
    }

    HDF_SUCCESS
}

pub extern "C" fn AudioCodecWriteReg(codec: *const crate::types::CodecDevice, reg: u32, val: u32) -> i32 {
    if codec.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_data = unsafe { (*codec).devData };
    if dev_data.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let write_fn = unsafe { (*dev_data).Write };
    if write_fn.is_none() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let ret = unsafe { write_fn.unwrap()(codec, reg, val) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioUpdateCodecRegBits(codec: *mut crate::types::CodecDevice, reg: u32, mask: u32, shift: u32, value: u32) -> i32 {
    let mut ret: i32;
    let mut cur_value: u32 = 0;
    let control_mask: u32;
    let temp_val: u32;

    if codec.is_null() || unsafe { (*codec).devData.is_null() } {
        return crate::types::HDF_ERR_INVALID_OBJECT as i32;
    }

    let shifted_value = value.wrapping_shl(shift);
    control_mask = mask.wrapping_shl(shift);

    unsafe {
        OsalMutexLock(&mut (*(*codec).devData).mutex);
    }

    ret = crate::src_audio_core::AudioCodecReadReg(codec, reg, &mut cur_value);
    if ret != crate::types::HDF_SUCCESS as i32 {
        unsafe {
            OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        }
        return crate::types::HDF_FAILURE as i32;
    }

    temp_val = cur_value & control_mask;
    if temp_val == shifted_value {
        unsafe {
            OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        }
        return crate::types::HDF_SUCCESS as i32;
    }

    cur_value = (cur_value & !control_mask) | (shifted_value & control_mask);
    ret = crate::src_audio_core::AudioCodecWriteReg(codec, reg, cur_value);
    if ret != crate::types::HDF_SUCCESS as i32 {
        unsafe {
            OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        }
        return crate::types::HDF_FAILURE as i32;
    }

    unsafe {
        OsalMutexUnlock(&mut (*(*codec).devData).mutex);
    }

    crate::types::HDF_SUCCESS as i32
}

pub extern "C" fn AudioUpdateDaiRegBits(dai: *const crate::types::DaiDevice, reg: u32, mask: u32, shift: u32, mut value: u32) -> i32 {
    if dai.is_null() || unsafe { (*dai).devData.is_null() } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Invalid input param.\0".as_ptr() as *const i8,
                b"AudioUpdateDaiRegBits\0".as_ptr() as *const i8,
                462i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let data: *mut crate::types::DaiData = unsafe { (*dai).devData };
    value = value << shift;
    let mixerControlMask: u32 = mask << shift;

    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*data).mutex) };
    unsafe { let _ = OsalMutexLock(mutex_ptr); }

    let mut curValue: u32 = 0;
    let ret = crate::src_audio_core::AudioDaiReadReg(dai, reg, &mut curValue as *mut u32);
    if ret != crate::types::HDF_SUCCESS {
        unsafe { let _ = OsalMutexUnlock(mutex_ptr); }
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Read reg fail ret=%d.\0".as_ptr() as *const i8,
                b"AudioUpdateDaiRegBits\0".as_ptr() as *const i8,
                473i32,
                ret,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let tempVal = curValue & mixerControlMask;
    if tempVal == value {
        unsafe { let _ = OsalMutexUnlock(mutex_ptr); }
        return crate::types::HDF_SUCCESS;
    }

    curValue = (curValue & !mixerControlMask) | (value & mixerControlMask);
    let ret = crate::src_audio_core::AudioDaiWriteReg(dai, reg, curValue);
    if ret != crate::types::HDF_SUCCESS {
        unsafe { let _ = OsalMutexUnlock(mutex_ptr); }
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Write reg fail ret=%d\0".as_ptr() as *const i8,
                b"AudioUpdateDaiRegBits\0".as_ptr() as *const i8,
                487i32,
                ret,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    unsafe { let _ = OsalMutexUnlock(mutex_ptr); }
    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn AudioCodecRegUpdate(codec: *mut crate::types::CodecDevice, mixerCtrl: *mut crate::types::AudioMixerControl) -> i32 {
    if codec.is_null() || mixerCtrl.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let m = unsafe { &*mixerCtrl };
    let mut mixer_value: u32 = m.value;
    let min: u32 = m.min;
    let max: u32 = m.max;
    if mixer_value < min || mixer_value > max {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if m.invert != 0 {
        mixer_value = max - mixer_value;
    }
    let reg = m.reg;
    let mask = m.mask;
    let shift = m.shift;
    if unsafe { crate::src_audio_core::AudioUpdateCodecRegBits(codec, reg, mask, shift, mixer_value) } != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    let rreg = m.rreg;
    let rshift = m.rshift;
    if reg != rreg || shift != rshift {
        if unsafe { crate::src_audio_core::AudioUpdateCodecRegBits(codec, rreg, mask, rshift, mixer_value) } != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecMuxRegUpdate(codec: *mut crate::types::CodecDevice, enumCtrl: *mut crate::types::AudioEnumKcontrol, value: *const u32) -> i32 {
    if codec.is_null() || enumCtrl.is_null() || value.is_null() {
        unsafe {
            let func_name = std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioCodecMuxRegUpdate\0");
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: input para is null.\0".as_ptr() as *const ::core::ffi::c_char,
                func_name.as_ptr(),
                535i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mut val: [u32; 2] = [0; 2];
    let e = unsafe { &*enumCtrl };
    let (max_val, reg, mask, shift_left, shift_right, reg2) =
        (e.max, e.reg, e.mask, e.shiftLeft, e.shiftRight, e.reg2);
    let vals = if !e.values.is_null() {
        let v0 = unsafe { *e.values.add(*value as usize) };
        let v1 = unsafe { *e.values.add(*value.offset(1) as usize) };
        [v0, v1]
    } else {
        let v0 = unsafe { *value };
        let v1 = unsafe { *value.offset(1) };
        [v0, v1]
    };
    val[0] = vals[0];
    val[1] = vals[1];

    if val[0] > max_val {
        unsafe {
            let func_name = std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioCodecMuxRegUpdate\0");
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid value=%u\0".as_ptr() as *const ::core::ffi::c_char,
                func_name.as_ptr(),
                548i32,
                val[0],
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mut ret = unsafe {
        crate::src_audio_core::AudioUpdateCodecRegBits(
            codec,
            reg,
            mask,
            shift_left as u32,
            val[0],
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            let func_name = std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioCodecMuxRegUpdate\0");
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: update left reg bits fail!\0".as_ptr() as *const ::core::ffi::c_char,
                func_name.as_ptr(),
                553i32,
            );
        }
        return ret;
    }

    if reg != reg2 || shift_left != shift_right {
        if val[1] > max_val {
            unsafe {
                let func_name = std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioCodecMuxRegUpdate\0");
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s][line:%d]: Audio invalid value=%u\0".as_ptr() as *const ::core::ffi::c_char,
                    func_name.as_ptr(),
                    559i32,
                    val[1],
                );
            }
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        ret = unsafe {
            crate::src_audio_core::AudioUpdateCodecRegBits(
                codec,
                reg2,
                mask,
                shift_right as u32,
                val[1],
            )
        };
        if ret != crate::types::HDF_SUCCESS {
            unsafe {
                let func_name = std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioCodecMuxRegUpdate\0");
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s][line:%d]: update right reg bits fail!\0".as_ptr() as *const ::core::ffi::c_char,
                    func_name.as_ptr(),
                    564i32,
                );
            }
            return ret;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioDaiRegUpdate(dai: *const crate::types::DaiDevice, mixerCtrl: *mut crate::types::AudioMixerControl) -> i32 {
    if dai.is_null() || mixerCtrl.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let (mut value, min, max, invert, reg, mask, shift, rreg, rshift) = unsafe {
        let m = &*mixerCtrl;
        (m.value, m.min, m.max, m.invert, m.reg, m.mask, m.shift, m.rreg, m.rshift)
    };

    if value < min || value > max {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    if invert != 0 {
        value = max - value;
    }

    let ret = unsafe {
        crate::src_audio_core::AudioUpdateDaiRegBits(
            dai,
            reg,
            mask,
            shift,
            value,
        )
    };

    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    if reg != rreg || shift != rshift {
        let ret2 = unsafe {
            crate::src_audio_core::AudioUpdateDaiRegBits(
                dai,
                rreg,
                mask,
                rshift,
                value,
            )
        };
        if ret2 != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioKcontrolGetCodec(kcontrol: *const crate::types::AudioKcontrol) -> *mut crate::types::CodecDevice {
    if kcontrol.is_null() || unsafe { (*kcontrol).pri.is_null() } {
        // Logging omitted (HiLogPrint unresolved); original call: HiLogPrint(LOG_CORE, LOG_ERROR, ..., __func__, ...)
        return std::ptr::null_mut();
    }
    let audioCard = unsafe { (*kcontrol).pri as *mut crate::types::AudioCard };
    if unsafe { (*audioCard).rtd.is_null() } {
        // Logging omitted (HiLogPrint unresolved)
        return std::ptr::null_mut();
    }
    unsafe { (*(*audioCard).rtd).codec }
}

pub extern "C" fn AudioKcontrolGetCpuDai(kcontrol: *const crate::types::AudioKcontrol) -> *mut crate::types::DaiDevice {
    if kcontrol.is_null() || unsafe { (*kcontrol).pri.is_null() } {
        return core::ptr::null_mut();
    }
    let audioCard = unsafe { (*kcontrol).pri as *mut crate::types::AudioCard };
    if unsafe { (*audioCard).rtd.is_null() } {
        return core::ptr::null_mut();
    }
    unsafe { (*(*audioCard).rtd).cpuDai }
}

pub extern "C" fn AudioAddControl(audioCard: *const crate::types::AudioCard, ctl: *const crate::types::AudioKcontrol) -> *mut crate::types::AudioKcontrol {
    if audioCard.is_null() || ctl.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Input params check error\0" as *const u8 as *const ::core::ffi::c_char,
                b"AudioAddControl\0" as *const u8 as *const ::core::ffi::c_char,
                645i32,
            );
        }
        return std::ptr::null_mut();
    }

    let control = unsafe {
        OsalMemCalloc(std::mem::size_of::<crate::types::AudioKcontrol>().try_into().unwrap()) as *mut crate::types::AudioKcontrol
    };
    if control.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Malloc control fail!\0" as *const u8 as *const ::core::ffi::c_char,
                b"AudioAddControl\0" as *const u8 as *const ::core::ffi::c_char,
                651i32,
            );
        }
        return std::ptr::null_mut();
    }

    unsafe {
        let list_ptr: *mut crate::types::DListHead = &mut (*control).list;
        (*list_ptr).next = list_ptr;
        (*list_ptr).prev = list_ptr;

        (*control).name = (*ctl).name;
        (*control).iface = (*ctl).iface;
        (*control).Info = (*ctl).Info;
        (*control).Get = (*ctl).Get;
        (*control).Set = (*ctl).Set;
        (*control).pri = audioCard as *mut ::core::ffi::c_void;
        (*control).privateValue = (*ctl).privateValue;
    }

    control
}

pub extern "C" fn AudioAddControls(audioCard: *mut crate::types::AudioCard, controls: *const crate::types::AudioKcontrol, controlMaxNum: i32) -> i32 {
    if audioCard.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if controls.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if controlMaxNum <= 0 {
        return crate::types::HDF_FAILURE;
    }

    for i in 0..controlMaxNum {
        let ctrl = unsafe {
            crate::src_audio_core::AudioAddControl(
                audioCard as *const crate::types::AudioCard,
                controls.offset(i as isize),
            )
        };
        if ctrl.is_null() {
            return crate::types::HDF_FAILURE;
        }

        unsafe {
            let head: *mut crate::types::DListHead =
                &mut (*audioCard).controls as *mut crate::types::DListHead;
            let entry: *mut crate::types::DListHead =
                &mut (*ctrl).list as *mut crate::types::DListHead;

            dlist_insert_after(head, entry);
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioInfoCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemInfo: *mut crate::types::AudioCtrlElemInfo) -> i32 {
    if kcontrol.is_null() || elemInfo.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Input param kcontrol is NULL.\0".as_ptr() as *const i8,
                b"AudioInfoCtrlOps\0".as_ptr() as *const i8,
                702i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mixerCtrl: *mut crate::types::AudioMixerControl = unsafe { (*kcontrol).privateValue as *mut crate::types::AudioMixerControl };
    if mixerCtrl.is_null() || unsafe { (*kcontrol).privateValue == 0 } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Input param kcontrol is NULL.\0".as_ptr() as *const i8,
                b"AudioInfoCtrlOps\0".as_ptr() as *const i8,
                702i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let (reg, rreg, shift, rshift, min, max) = unsafe {
        let m = &*mixerCtrl;
        (m.reg, m.rreg, m.shift, m.rshift, m.min, m.max)
    };

    let elem_info = unsafe { &mut *elemInfo };
    if reg != rreg || shift != rshift {
        elem_info.count = 2;
    } else {
        elem_info.count = 1;
    }
    elem_info.type_ = crate::types::AUDIO_CTL_ELEM_TYPE_INTEGER;
    elem_info.min = min as i32;
    elem_info.max = max as i32;

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioInfoEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemInfo: *mut crate::types::AudioCtrlElemInfo) -> i32 {
    if kcontrol.is_null() || elemInfo.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let private_value = unsafe { (*kcontrol).privateValue };
    if private_value <= 0 {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let enumCtrl = private_value as *mut crate::types::AudioEnumKcontrol;

    let (reg, reg2, shiftLeft, shiftRight, max_val) = unsafe {
        let e = &*enumCtrl;
        (e.reg, e.reg2, e.shiftLeft, e.shiftRight, e.max)
    };

    unsafe {
        if reg != reg2 || shiftLeft != shiftRight {
            (*elemInfo).count = 2;
        } else {
            (*elemInfo).count = 1;
        }
        (*elemInfo).type_ = crate::types::AUDIO_CTL_ELEM_TYPE_ENUMERATED;
        (*elemInfo).min = 0;
        (*elemInfo).max = max_val as i32;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioGetCtrlOpsRReg(elemValue: *mut crate::types::AudioCtrlElemValue, mixerCtrl: *const crate::types::AudioMixerControl, rcurValue: u32) -> i32 {
    if elemValue.is_null() || mixerCtrl.is_null() {
        let domain = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr() };
        let fmt = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: Audio input param is NULL.\0").unwrap().as_ptr() };
        let func = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"AudioGetCtrlOpsRReg\0").unwrap().as_ptr() };
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                domain,
                fmt,
                func,
                747i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let m = unsafe { &*mixerCtrl };
    let (reg, rreg, shift, rshift, mask, max, min, invert) =
        (m.reg, m.rreg, m.shift, m.rshift, m.mask, m.max, m.min, m.invert);

    if reg != rreg || shift != rshift {
        let mut rval = rcurValue;
        if reg == rreg {
            rval = (rval >> rshift) & mask;
        } else {
            rval = (rval >> shift) & mask;
        }
        if rval > max || rval < min {
            let domain = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap().as_ptr() };
            let fmt = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: Audio invalid rcurValue=%u\0").unwrap().as_ptr() };
            let func = unsafe { ::std::ffi::CStr::from_bytes_with_nul(b"AudioGetCtrlOpsRReg\0").unwrap().as_ptr() };
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    domain,
                    fmt,
                    func,
                    758i32,
                    rval,
                );
            }
            return crate::types::HDF_FAILURE;
        }
        if invert != 0 {
            rval = max - rval;
        }
        let elem = unsafe { &mut *elemValue };
        elem.value[1] = rval;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioGetCtrlOpsReg(elemValue: *mut crate::types::AudioCtrlElemValue, mixerCtrl: *const crate::types::AudioMixerControl, curValue: u32) -> i32 {
    if elemValue.is_null() || mixerCtrl.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let mixer = unsafe { &*mixerCtrl };
    let mut cur_value = (curValue >> mixer.shift) & mixer.mask;
    if cur_value > mixer.max || cur_value < mixer.min {
        return crate::types::HDF_FAILURE;
    }
    if mixer.invert != 0 {
        cur_value = mixer.max - cur_value;
    }
    unsafe {
        (*elemValue).value[0] = cur_value;
    }
    crate::types::HDF_SUCCESS
}

fn AudioGetEnumCtrlOpsReg(elemValue: *mut crate::types::AudioCtrlElemValue, enumCtrl: *const crate::types::AudioEnumKcontrol, curValue: u32)-> i32 {
    if elemValue.is_null() || enumCtrl.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mut curValue = curValue;

    unsafe {
        curValue = (curValue >> (*enumCtrl).shiftLeft as u32) & (*enumCtrl).mask;
        if curValue > (*enumCtrl).max {
            return crate::types::HDF_FAILURE;
        }
        (*elemValue).value[0] = curValue;
    }

    crate::types::HDF_SUCCESS
}

fn AudioGetEnumCtrlOpsRReg(elemValue: *mut crate::types::AudioCtrlElemValue, enumCtrl: *const crate::types::AudioEnumKcontrol, rcurValue: u32)-> i32 {
    if elemValue.is_null() || enumCtrl.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0").as_ptr(),
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: Audio input param is NULL.\0").as_ptr(),
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioGetEnumCtrlOpsRReg\0").as_ptr(),
                814i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    unsafe {
        if (*enumCtrl).reg != (*enumCtrl).reg2 || (*enumCtrl).shiftLeft != (*enumCtrl).shiftRight {
            let mut rcur: u32 = rcurValue;
            rcur = (rcur >> (*enumCtrl).shiftLeft) & (*enumCtrl).mask;
            if rcur > (*enumCtrl).max {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0").as_ptr(),
                    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: Audio invalid rcurValue=%u\0").as_ptr(),
                    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioGetEnumCtrlOpsRReg\0").as_ptr(),
                    822i32,
                    rcur,
                );
                return crate::types::HDF_FAILURE;
            }
            (*elemValue).value[1] = rcur;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32 {
    let mut curValue: u32 = 0;
    let mut rcurValue: u32 = 0;
    let mixerCtrl: *mut crate::types::AudioMixerControl;
    let codec: *mut crate::types::CodecDevice;

    unsafe {
        if kcontrol.is_null() || (*kcontrol).privateValue == 0 || elemValue.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        mixerCtrl = (*kcontrol).privateValue as *mut crate::types::AudioMixerControl;
    }

    codec = crate::src_audio_core::AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        if crate::src_audio_core::AudioCodecReadReg(codec, (*mixerCtrl).reg, &mut curValue as *mut u32) != crate::types::HDF_SUCCESS
            || crate::src_audio_core::AudioCodecReadReg(codec, (*mixerCtrl).rreg, &mut rcurValue as *mut u32) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_FAILURE;
        }
        if crate::src_audio_core::AudioGetCtrlOpsReg(elemValue, mixerCtrl, curValue) != crate::types::HDF_SUCCESS
            || crate::src_audio_core::AudioGetCtrlOpsRReg(elemValue, mixerCtrl, rcurValue) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_FAILURE;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecGetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32 {
    let mut curValue: u32 = 0;
    let mut rcurValue: u32 = 0;
    if kcontrol.is_null() || unsafe { (*kcontrol).privateValue <= 0 } || elemValue.is_null() {
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: Audio input param is NULL.\0").unwrap();
        let func = std::ffi::CStr::from_bytes_with_nul(b"AudioCodecGetEnumCtrlOps\0").unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                869i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let enumCtrl = unsafe {
        let pv = (*kcontrol).privateValue;
        pv as usize as *mut crate::types::AudioEnumKcontrol
    };
    let codec = crate::src_audio_core::AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: mixerCtrl and codec is NULL.\0").unwrap();
        let func = std::ffi::CStr::from_bytes_with_nul(b"AudioCodecGetEnumCtrlOps\0").unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                875i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let ret_reg = unsafe {
        crate::src_audio_core::AudioCodecReadReg(codec, (*enumCtrl).reg, &mut curValue)
    };
    let ret_reg2 = unsafe {
        crate::src_audio_core::AudioCodecReadReg(codec, (*enumCtrl).reg2, &mut rcurValue)
    };
    if ret_reg != crate::types::HDF_SUCCESS || ret_reg2 != crate::types::HDF_SUCCESS {
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: Read Reg fail.\0").unwrap();
        let func = std::ffi::CStr::from_bytes_with_nul(b"AudioCodecGetEnumCtrlOps\0").unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                880i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let ret1 = crate::src_audio_core::AudioGetEnumCtrlOpsReg(elemValue, enumCtrl, curValue);
    let ret2 = crate::src_audio_core::AudioGetEnumCtrlOpsRReg(elemValue, enumCtrl, rcurValue);
    if ret1 != crate::types::HDF_SUCCESS || ret2 != crate::types::HDF_SUCCESS {
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: Audio codec get kcontrol reg and rreg fail.\0").unwrap();
        let func = std::ffi::CStr::from_bytes_with_nul(b"AudioCodecGetEnumCtrlOps\0").unwrap();
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag.as_ptr(),
                fmt.as_ptr(),
                func.as_ptr(),
                886i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSetCtrlOpsReg(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue, mixerCtrl: *const crate::types::AudioMixerControl, value: *mut u32) -> i32 {
    // Validate pointers and privateValue
    if kcontrol.is_null()
        || (unsafe { (*kcontrol).privateValue } == 0)
        || elemValue.is_null()
        || mixerCtrl.is_null()
        || value.is_null()
    {
        // Logging skipped: HiLogPrint is not available
        return HDF_ERR_INVALID_OBJECT;
    }

    let mixer = unsafe { &*mixerCtrl };
    let elem = unsafe { &*elemValue };
    let val = elem.value[0];

    if val < mixer.min || val > mixer.max {
        // Logging skipped: HiLogPrint is not available
        return HDF_ERR_INVALID_OBJECT;
    }

    let new_val = if mixer.invert == 0 { val } else { mixer.max - val };
    unsafe { *value = new_val; }

    HDF_SUCCESS
}

pub extern "C" fn AudioSetCtrlOpsRReg(elemValue: *const crate::types::AudioCtrlElemValue, mixerCtrl: *mut crate::types::AudioMixerControl, rvalue: *mut u32, updateRReg: *mut bool) -> i32 {
    if elemValue.is_null() || mixerCtrl.is_null() || rvalue.is_null() || updateRReg.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let m = unsafe { &*mixerCtrl };
    if m.reg != m.rreg || m.shift != m.rshift {
        let val = unsafe { (*elemValue).value[1] };
        if val < m.min || val > m.max {
            return crate::types::HDF_FAILURE;
        }
        // original C may assign shift; per unsafe auditor we defer to caller
        let rv = if m.invert == 0 { val } else { m.max - val };
        unsafe { *rvalue = rv; *updateRReg = true; }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCodecSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32 {
    let mut value: u32 = 0;
    let mut rvalue: u32 = 0;
    let mut updateRReg: bool = false;

    if kcontrol.is_null() || (unsafe { (*kcontrol).privateValue } == 0) || elemValue.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let (codec, mixerCtrl) = unsafe {
        let c = crate::src_audio_core::AudioKcontrolGetCodec(kcontrol);
        let m = (*kcontrol).privateValue as *mut crate::types::AudioMixerControl;
        (c, m)
    };

    if codec.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    if crate::src_audio_core::AudioSetCtrlOpsReg(kcontrol, elemValue, mixerCtrl, &mut value)
        != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let mixer = unsafe { &*mixerCtrl };

    if crate::src_audio_core::AudioUpdateCodecRegBits(
        codec,
        mixer.reg,
        mixer.mask,
        mixer.shift,
        value,
    ) != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_FAILURE;
    }

    if crate::src_audio_core::AudioSetCtrlOpsRReg(elemValue, mixerCtrl, &mut rvalue, &mut updateRReg)
        != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    if updateRReg {
        if crate::src_audio_core::AudioUpdateCodecRegBits(
            codec,
            mixer.rreg,
            mixer.mask,
            mixer.rshift,
            rvalue,
        ) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_FAILURE;
        }
    }

    crate::types::HDF_SUCCESS
}

fn AudioCodecSetEnumRegUpdate(codec: *mut crate::types::CodecDevice, enumCtrl: *const crate::types::AudioEnumKcontrol, value: *const u32)-> i32 {
    use crate::types::*;

    if codec.is_null() || enumCtrl.is_null() || value.is_null() {
        return HDF_ERR_INVALID_OBJECT;
    }

    let enum_values = unsafe { (*enumCtrl).values };
    let set_val0: u32;
    let set_val1: u32;
    if !enum_values.is_null() {
        set_val0 = unsafe { *enum_values.offset((*value) as isize) };
        set_val1 = unsafe { *enum_values.offset((*value.offset(1)) as isize) };
    } else {
        set_val0 = unsafe { *value };
        set_val1 = unsafe { *value.offset(1) };
    }

    let max = unsafe { (*enumCtrl).max };
    if set_val0 > max {
        return HDF_ERR_INVALID_OBJECT;
    }

    let reg = unsafe { (*enumCtrl).reg };
    let mask = unsafe { (*enumCtrl).mask };
    let shift_left = unsafe { (*enumCtrl).shiftLeft } as u32;
    let reg2 = unsafe { (*enumCtrl).reg2 };
    let shift_right = unsafe { (*enumCtrl).shiftRight } as u32;

    let ret = crate::src_audio_core::AudioUpdateCodecRegBits(codec, reg, mask, shift_left, set_val0);
    if ret != HDF_SUCCESS {
        return HDF_FAILURE;
    }

    if reg != reg2 || shift_left != shift_right {
        if set_val1 > max {
            return HDF_ERR_INVALID_OBJECT;
        }
        let ret = crate::src_audio_core::AudioUpdateCodecRegBits(codec, reg2, mask, shift_right, set_val1);
        if ret != HDF_SUCCESS {
            return HDF_FAILURE;
        }
    }

    HDF_SUCCESS
}

pub extern "C" fn AudioCodecSetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32 {
    if kcontrol.is_null() || unsafe { (*kcontrol).privateValue == 0 } || elemValue.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let codec = crate::src_audio_core::AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let enumCtrl = unsafe { (*kcontrol).privateValue as *mut crate::types::AudioEnumKcontrol };
    let ret = crate::src_audio_core::AudioCodecSetEnumRegUpdate(
        codec,
        enumCtrl,
        unsafe { &(*elemValue).value as *const u32 },
    );
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCpuDaiSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32 {
    let mut value: u32 = 0;
    let mut rvalue: u32 = 0;
    let mut updateRReg: bool = false;
    let mixerCtrl: *mut crate::types::AudioMixerControl;
    let dai: *mut crate::types::DaiDevice;

    unsafe {
        if kcontrol.is_null()
            || (*kcontrol).privateValue <= 0
            || elemValue.is_null()
        {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        mixerCtrl = (*kcontrol).privateValue as *mut crate::types::AudioMixerControl;

        if crate::src_audio_core::AudioSetCtrlOpsReg(
            kcontrol,
            elemValue,
            mixerCtrl as *const crate::types::AudioMixerControl,
            &mut value as *mut u32,
        ) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        dai = crate::src_audio_core::AudioKcontrolGetCpuDai(kcontrol);
        if dai.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        if crate::src_audio_core::AudioUpdateDaiRegBits(
            dai as *const crate::types::DaiDevice,
            (*mixerCtrl).reg,
            (*mixerCtrl).mask,
            (*mixerCtrl).shift,
            value,
        ) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_FAILURE;
        }

        if crate::src_audio_core::AudioSetCtrlOpsRReg(
            elemValue,
            mixerCtrl,
            &mut rvalue as *mut u32,
            &mut updateRReg as *mut bool,
        ) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        if updateRReg {
            if crate::src_audio_core::AudioUpdateDaiRegBits(
                dai as *const crate::types::DaiDevice,
                (*mixerCtrl).rreg,
                (*mixerCtrl).mask,
                (*mixerCtrl).rshift,
                rvalue,
            ) != crate::types::HDF_SUCCESS
            {
                return crate::types::HDF_FAILURE;
            }
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCpuDaiGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32 {
    let mut curValue: u32 = 0;
    let mut rcurValue: u32 = 0;

    unsafe {
        if kcontrol.is_null() || (*kcontrol).privateValue == 0 || elemValue.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
    }

    let mixerCtrl: *mut crate::types::AudioMixerControl = unsafe {
        (*kcontrol).privateValue as usize as *mut crate::types::AudioMixerControl
    };

    let dai: *mut crate::types::DaiDevice = crate::src_audio_core::AudioKcontrolGetCpuDai(kcontrol);

    if dai.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        let reg_read1 = crate::src_audio_core::AudioDaiReadReg(
            dai as *const crate::types::DaiDevice,
            (*mixerCtrl).reg,
            &mut curValue,
        );
        let reg_read2 = crate::src_audio_core::AudioDaiReadReg(
            dai as *const crate::types::DaiDevice,
            (*mixerCtrl).rreg,
            &mut rcurValue,
        );
        if reg_read1 != crate::types::HDF_SUCCESS || reg_read2 != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }

        let get_reg = crate::src_audio_core::AudioGetCtrlOpsReg(
            elemValue,
            mixerCtrl as *const crate::types::AudioMixerControl,
            curValue,
        );
        let get_rreg = crate::src_audio_core::AudioGetCtrlOpsRReg(
            elemValue,
            mixerCtrl as *const crate::types::AudioMixerControl,
            rcurValue,
        );
        if get_reg != crate::types::HDF_SUCCESS || get_rreg != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
    }

    crate::types::HDF_SUCCESS
}
