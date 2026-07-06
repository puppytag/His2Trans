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

pub extern "C" fn DeviceAttributeSerialize(attribute: *const crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }

    let attribute_ref = unsafe { &*attribute };
    let svc_name = attribute_ref.svcName;
    let module_name = attribute_ref.moduleName;
    let device_name = attribute_ref.deviceName;
    let device_match_attr = attribute_ref.deviceMatchAttr;

    if !(unsafe { HdfSbufWriteUint32(sbuf, attribute_ref.deviceId) }
        && unsafe { HdfSbufWriteUint16(sbuf, attribute_ref.policy) }
        && unsafe { HdfSbufWriteString(sbuf, svc_name) }
        && unsafe { HdfSbufWriteString(sbuf, module_name) }
        && unsafe { HdfSbufWriteString(sbuf, device_name) }) {
        return false;
    }

    let tag_ptr: *const ::core::ffi::c_char = b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char;
    let msg_ptr: *const ::core::ffi::c_char = b"failed to serialize device attribute\0".as_ptr() as *const ::core::ffi::c_char;
    let domain: u32 = 0xD002510u32;

    if !device_match_attr.is_null() {
        let ok = unsafe { HdfSbufWriteUint32(sbuf, 1) }
              && unsafe { HdfSbufWriteString(sbuf, device_match_attr) };
        if !ok {
            unsafe {
                HiLogPrint(crate::types::LOG_CORE,
                           crate::types::LOG_ERROR,
                           domain,
                           tag_ptr,
                           msg_ptr);
            }
            return false;
        }
    } else {
        if !unsafe { HdfSbufWriteUint32(sbuf, 0) } {
            unsafe {
                HiLogPrint(crate::types::LOG_CORE,
                           crate::types::LOG_ERROR,
                           domain,
                           tag_ptr,
                           msg_ptr);
            }
            return false;
        }
    }

    true
}

// Private helper to read a string from sbuf, duplicate it, assign to field, with error logging.
fn read_and_strdup_field(
    sbuf: *mut crate::types::HdfSBuf,
    field_ptr: &mut *const ::core::ffi::c_char,
    null_msg: *const ::core::ffi::c_char,
    dup_fail_msg: *const ::core::ffi::c_char,
    tag_ptr: *const ::core::ffi::c_char,
    domain: ::core::ffi::c_uint,
    extra_arg: *const ::core::ffi::c_char, // non-null for formatted null_msg
) -> bool {
    let value = unsafe { HdfSbufReadString(sbuf) as *const ::core::ffi::c_char };
    if value.is_null() {
        unsafe {
            if extra_arg.is_null() {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, domain, tag_ptr, null_msg);
            } else {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, domain, tag_ptr, null_msg, extra_arg);
            }
        }
        return false;
    }
    let dup = unsafe { strdup(value) as *const ::core::ffi::c_char };
    if dup.is_null() {
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, domain, tag_ptr, dup_fail_msg);
        }
        return false;
    }
    *field_ptr = dup;
    true
}

fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf)-> bool {
    let tag_ptr: *const ::core::ffi::c_char = b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char;
    let domain: ::core::ffi::c_uint = 0xD002510;

    if attribute.is_null() {
        return false;
    }
    let attr_mut = unsafe { &mut *attribute };

    // svcName
    let svc_null_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, svcName is null\0".as_ptr() as *const ::core::ffi::c_char;
    let svc_dup_fail_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, strdup svcName fail\0".as_ptr() as *const ::core::ffi::c_char;
    if !read_and_strdup_field(sbuf, &mut attr_mut.svcName, svc_null_msg, svc_dup_fail_msg, tag_ptr, domain, std::ptr::null()) {
        return false;
    }

    // moduleName
    let mod_null_msg: *const ::core::ffi::c_char = b"Read from parcel failed, moduleName is null\0".as_ptr() as *const ::core::ffi::c_char;
    let mod_dup_fail_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, strdup moduleName fail\0".as_ptr() as *const ::core::ffi::c_char;
    if !read_and_strdup_field(sbuf, &mut attr_mut.moduleName, mod_null_msg, mod_dup_fail_msg, tag_ptr, domain, std::ptr::null()) {
        return false;
    }

    // deviceName
    let dev_null_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, deviceName is null\0".as_ptr() as *const ::core::ffi::c_char;
    let dev_dup_fail_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, strdup deviceName fail\0".as_ptr() as *const ::core::ffi::c_char;
    if !read_and_strdup_field(sbuf, &mut attr_mut.deviceName, dev_null_msg, dev_dup_fail_msg, tag_ptr, domain, std::ptr::null()) {
        return false;
    }

    // deviceMatchAttr
    let mut length: u32 = 0;
    if !unsafe { HdfSbufReadUint32(sbuf, &mut length) } {
        let msg: *const ::core::ffi::c_char = b"Device attribute readDeviceMatchAttr length failed\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, domain, tag_ptr, msg);
        }
        return false;
    }
    if length == 1 {
        let match_null_msg: *const ::core::ffi::c_char = b"%s: Read from sbuf failed, deviceMatchAttr is null\0".as_ptr() as *const ::core::ffi::c_char;
        let match_dup_fail_msg: *const ::core::ffi::c_char = b"Read from sbuf failed, strdup deviceMatchAttr fail\0".as_ptr() as *const ::core::ffi::c_char;
        let match_extra_arg: *const ::core::ffi::c_char = b"DeviceAttributeSet\0".as_ptr() as *const ::core::ffi::c_char;
        if !read_and_strdup_field(sbuf, &mut attr_mut.deviceMatchAttr, match_null_msg, match_dup_fail_msg, tag_ptr, domain, match_extra_arg) {
            return false;
        }
    }

    true
}

pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut crate::types::HdfSBuf) -> *mut crate::types::HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }

    let attribute = unsafe { HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        // HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "dev_attr_serialze", "OsalMemCalloc failed, attribute is null");
        return std::ptr::null_mut();
    }

    let attr_mut = unsafe { &mut *attribute };
    // Check deviceMatchAttr (expected NULL after zeroing; original logs a warning)
    let match_attr = attr_mut.deviceMatchAttr;
    if match_attr.is_null() {
        // HiLogPrint(LOG_CORE, LOG_WARN, 0xD002510, "dev_attr_serialze", "OsalMemCalloc failed, attribute->deviceMatchAttr is null");
    }

    // Read deviceId and policy from sbuf
    let mut device_id: u32 = 0;
    let mut policy: u16 = 0;
    if !unsafe { HdfSbufReadUint32(sbuf, &mut device_id) }
        || !unsafe { HdfSbufReadUint16(sbuf, &mut policy) }
    {
        DeviceSerializedAttributeRelease(attribute);
        return std::ptr::null_mut();
    }
    attr_mut.deviceId = device_id;
    attr_mut.policy = policy;

    let success = crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf);
    if success {
        return attribute;
    }

    crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    std::ptr::null_mut()
}

pub extern "C" fn DeviceSerializedAttributeRelease(attribute: *mut crate::types::HdfDeviceInfo) {
    if attribute.is_null() {
        return;
    }
    let attr_mut = unsafe { &mut *attribute };
    let module_name = attr_mut.moduleName;
    if !module_name.is_null() {
        unsafe { OsalMemFree(module_name as *mut ::core::ffi::c_void); }
    }
    let svc_name = attr_mut.svcName;
    if !svc_name.is_null() {
        unsafe { OsalMemFree(svc_name as *mut ::core::ffi::c_void); }
    }
    let device_name = attr_mut.deviceName;
    if !device_name.is_null() {
        unsafe { OsalMemFree(device_name as *mut ::core::ffi::c_void); }
    }
    let device_match_attr = attr_mut.deviceMatchAttr;
    if !device_match_attr.is_null() {
        unsafe { OsalMemFree(device_match_attr as *mut ::core::ffi::c_void); }
    }
    unsafe { OsalMemFree(attribute as *mut ::core::ffi::c_void); }
}
