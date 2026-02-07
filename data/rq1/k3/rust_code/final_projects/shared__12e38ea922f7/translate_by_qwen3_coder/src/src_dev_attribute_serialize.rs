//! Module: src_dev_attribute_serialize
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

pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }
    unsafe {
        let device_id = (*attribute).deviceId;
        let policy = (*attribute).policy;
        let svc_name = (*attribute).svcName;
        let module_name = (*attribute).moduleName;
        let device_name = (*attribute).deviceName;
        if !crate::compat::HdfSbufWriteUint32(sbuf, device_id) ||
            !crate::compat::HdfSbufWriteUint16(sbuf, policy) ||
            !crate::compat::HdfSbufWriteString(sbuf, svc_name) ||
            !crate::compat::HdfSbufWriteString(sbuf, module_name) ||
            !crate::compat::HdfSbufWriteString(sbuf, device_name) {
            return false;
        }
        let device_match_attr = (*attribute).deviceMatchAttr;
        if !device_match_attr.is_null() {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
                !crate::compat::HdfSbufWriteString(sbuf, device_match_attr) {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"failed to serialize device attribute\0".as_ptr() as *const i8,
                );
                return false;
            }
        } else {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 0) {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"failed to serialize device attribute\0".as_ptr() as *const i8,
                );
                return false;
            }
        }
    }
    true
}

fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    unsafe {
        let svcName = crate::compat::HdfSbufReadString(sbuf);
        if svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from sbuf failed, svcName is null\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup svcName fail\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }

        let moduleName = crate::compat::HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from parcel failed, moduleName is null\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup moduleName fail\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }

        let deviceName = crate::compat::HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from sbuf failed, deviceName is null\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup deviceName fail\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }

        let mut length: u32 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut length) {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                b"Device attribute readDeviceMatchAttr length failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false;
        }
        if length == 1 {
            let deviceMatchAttr = crate::compat::HdfSbufReadString(sbuf);
            if deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                    b"DeviceAttributeSet: Read from sbuf failed, deviceMatchAttr is null\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0" as *const u8 as *const ::core::ffi::c_char,
                    b"Read from sbuf failed, strdup deviceMatchAttr fail\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return false;
            }
        }

        true
    }
}

pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute: *mut crate::types::HdfDeviceInfo = unsafe { crate::compat::HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let mut deviceId: u32 = 0;
        let mut policy: u16 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut deviceId) || !crate::compat::HdfSbufReadUint16(sbuf, &mut policy) {
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
        (*attribute).deviceId = deviceId;
        (*attribute).policy = policy;
    }
    if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
        return attribute;
    }
    crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    std::ptr::null_mut()
}

pub extern "C" fn DeviceSerializedAttributeRelease(attribute: *mut HdfDeviceInfo) {
    if attribute.is_null() {
        return;
    }
    unsafe {
        if !(*attribute).moduleName.is_null() {
            OsalMemFree((*attribute).moduleName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).svcName.is_null() {
            OsalMemFree((*attribute).svcName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).deviceName.is_null() {
            OsalMemFree((*attribute).deviceName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).deviceMatchAttr.is_null() {
            OsalMemFree((*attribute).deviceMatchAttr as *mut ::core::ffi::c_void);
        }
        OsalMemFree(attribute as *mut ::core::ffi::c_void);
    }
}
