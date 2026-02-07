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
    let attr = unsafe { &*attribute };
    unsafe {
        if !crate::compat::HdfSbufWriteUint32(sbuf, attr.deviceId) ||
            !crate::compat::HdfSbufWriteUint16(sbuf, attr.policy) ||
            !crate::compat::HdfSbufWriteString(sbuf, attr.svcName) ||
            !crate::compat::HdfSbufWriteString(sbuf, attr.moduleName) ||
            !crate::compat::HdfSbufWriteString(sbuf, attr.deviceName) {
            return false;
        }
    }
    if !attr.deviceMatchAttr.is_null() {
        unsafe {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
                !crate::compat::HdfSbufWriteString(sbuf, attr.deviceMatchAttr) {
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
    } else {
        unsafe {
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
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, svcName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup svcName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let moduleName = crate::compat::HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from parcel failed, moduleName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup moduleName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let deviceName = crate::compat::HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, deviceName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup deviceName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let mut length: u32 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut length) {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Device attribute readDeviceMatchAttr length failed\0".as_ptr() as *const i8,
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
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"DeviceAttributeSet: Read from sbuf failed, deviceMatchAttr is null\0".as_ptr() as *const i8,
                );
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"Read from sbuf failed, strdup deviceMatchAttr fail\0".as_ptr() as *const i8,
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
    let attribute = unsafe { crate::compat::HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(0, 3, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute is null\0".as_ptr() as *const _) };
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = crate::compat::HiLogPrint(0, 4, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const _);
        }
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !crate::compat::HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = crate::compat::HiLogPrint(0, 3, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"invalid deviceId or policy\0".as_ptr() as *const _);
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
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
