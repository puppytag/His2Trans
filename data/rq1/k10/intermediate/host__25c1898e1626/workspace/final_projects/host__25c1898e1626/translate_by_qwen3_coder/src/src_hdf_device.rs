//! Module: src_hdf_device
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

fn UpdateDeivceNodeIdIndex(device: *mut crate::types::HdfDevice, nodeDevid: crate::types::devid_t) {
    if device.is_null() {
        return;
    }
    let dev_node_id = (nodeDevid as u32) & ((1 << 8) - 1);
    unsafe {
        if (*device).devidIndex < dev_node_id as u16 {
            (*device).devidIndex = dev_node_id as u16;
        }
    }
}

fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    let mut node_id: u16 = 129;
    let mut find: bool = false;
    unsafe {
        if device.is_null() {
            return node_id as crate::types::devid_t;
        }
        while node_id <= (*device).devidIndex {
            find = false;
            let mut dev_node: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
            let mut entry_ptr: *mut crate::types::DListHead = &mut (*device).devNodes;
            loop {
                entry_ptr = (*entry_ptr).next;
                if entry_ptr.is_null() || entry_ptr == &mut (*device).devNodes {
                    break;
                }
                dev_node = (entry_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
                if ((*dev_node).devId & ((1 << 8) - 1)) as u16 == node_id {
                    find = true;
                    break;
                }
            }
            if !find {
                return node_id as crate::types::devid_t;
            }
            node_id += 1;
        }
    }
    node_id as crate::types::devid_t
}

fn AcquireNodeDeivceId(device: *mut crate::types::HdfDevice, devid: *mut crate::types::devid_t) -> i32 {
    let mut node_id: crate::types::devid_t;
    let usable_id: crate::types::devid_t;
    unsafe {
        if (*device).devidIndex >= ((1 << 8) - 1) {
            return crate::types::HDF_FAILURE;
        }
        if (*device).devidIndex < 129 {
            (*device).devidIndex = 129;
            node_id = (*device).devidIndex as crate::types::devid_t;
        } else {
            usable_id = crate::src_hdf_device::FindUsableDevNodeId(device);
            if usable_id <= (*device).devidIndex as crate::types::devid_t {
                node_id = usable_id;
            } else {
                (*device).devidIndex += 1;
                node_id = (*device).devidIndex as crate::types::devid_t;
            }
        }
        if devid.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"params invalid *devid\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        let device_id = (*device).deviceId;
        *devid = ((((device_id >> (16 + 8)) as u16) << (16 + 8))
            | ((((device_id >> 8) & ((1 << 16) - 1)) as u16) << 8)
            | (node_id as u16)) as crate::types::devid_t;
    }
    crate::types::HDF_SUCCESS
}

fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut ret: i32;
    let device = devInst as *mut crate::types::HdfDevice;
    let node_if = devNode as *mut crate::types::IDeviceNode;
    unsafe {
        if device.is_null() || node_if.is_null() || (*node_if).LaunchNode.is_none() {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, input params invalid\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        if (*devNode).devId == 0
            && crate::src_hdf_device::AcquireNodeDeivceId(
                device,
                &mut (*devNode).devId as *mut crate::types::devid_t,
            ) != crate::types::HDF_SUCCESS
        {
            let _ = HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, invalid device id\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        if !(*devNode).token.is_null() {
            (*((*devNode).token)).devid = (*devNode).devId;
        }
        ret = if let Some(f) = (*node_if).LaunchNode {
            f(devNode)
        } else {
            crate::types::HDF_ERR_INVALID_PARAM
        };
        if ret == crate::types::HDF_SUCCESS {
            let entry = &mut (*devNode).entry as *mut crate::types::DListHead;
            let head = &mut (*device).devNodes as *mut crate::types::DListHead;
            unsafe {
                (*entry).next = head;
                (*entry).prev = (*head).prev;
                (*(*head).prev).next = entry;
                (*head).prev = entry;
            }
            crate::src_hdf_device::UpdateDeivceNodeIdIndex(device, (*devNode).devId);
        }
    }
    ret
}

pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let device = unsafe {
        (devInst as *mut u8).offset(-(std::mem::offset_of!(HdfDevice, super_) as isize)) as *mut HdfDevice
    };
    let device_id_mask = ((1u32 << 16) - 1);
    let device_id_part = unsafe { ((*device).deviceId >> 8) & device_id_mask };
    let devnode_id_part = unsafe { ((*devNode).devId >> 8) & device_id_mask };
    if device_id_part != devnode_id_part {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"hdf_device\0".as_ptr() as *const _, b"%{public}s: device detach unknown devnode \0".as_ptr() as *const _, b"HdfDeviceDetach\0".as_ptr() as *const _) };
        return HDF_DEV_ERR_NO_DEVICE;
    }
    unsafe {
        if !(*devNode).entry.next.is_null() {
            let entry = &mut (*devNode).entry;
            if !entry.prev.is_null() {
                (*entry.prev).next = entry.next;
            }
            if !entry.next.is_null() {
                (*entry.next).prev = entry.prev;
            }
            entry.prev = std::ptr::null_mut();
            entry.next = std::ptr::null_mut();
        }
        if let Some(f) = (*devNode).super_.UnlaunchNode {
            f(devNode);
        }
    }
    HDF_SUCCESS
}

fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let dev: *mut crate::types::HdfDevice = unsafe {
        if device.is_null() {
            return std::ptr::null_mut();
        }
        (device as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, super_) as isize)) as *mut crate::types::HdfDevice
    };
    unsafe {
        if (*dev).devNodes.next.is_null() {
            return std::ptr::null_mut();
        }
        devNode = ((*dev).devNodes.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
        while !std::ptr::eq(&(*devNode).entry, &(*dev).devNodes) {
            if (*devNode).devId == devid {
                return devNode;
            }
            if (*devNode).entry.next.is_null() {
                break;
            }
            devNode = ((*devNode).entry.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
        }
    }
    std::ptr::null_mut()
}

fn HdfDeviceDetachWithDevid(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> i32 {
    let dev = unsafe { (device as *mut u8).offset(-(std::mem::size_of::<crate::types::HdfDevice>() as isize)) } as *mut crate::types::HdfDevice;
    let _ = dev;
    let devNode = crate::src_hdf_device::HdfDeviceGetDeviceNode(device, devid);
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"hdf_device\0".as_ptr() as *const _, b"devNode is NULL\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    crate::src_hdf_device::HdfDeviceDetach(device, devNode)
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_device_8
// c_function: HdfDeviceConstruct
// rust_file: src_hdf_device.rs
// rust_signature: pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice)
// c_first_line: void HdfDeviceConstruct(struct HdfDevice *device)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_8/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `HdfDeviceDetach` in module `crate::compat`
//      --> src/src_hdf_device.rs:136:55
//       |
//       |                                                       ^^^^^^^^^^^^^^^ not found in `crate::compat`
//   error[E0605]: non-primitive cast: `fn(*mut IHdfDevice, *mut HdfDeviceNode) -> i32 {HdfDeviceAttach}` as `unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> i32`
//      --> src/src_hdf_device.rs:135:40
//       |
//       |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
// =================================
pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_device::HdfDeviceConstruct(device as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_device_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_8/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::src_hdf_device::HdfDeviceAttach as unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32);
        (*device).super_.Detach = Some(crate::compat::HdfDeviceDetach);
        (*device).super_.DetachWithDevid = Some(crate::src_hdf_device::HdfDeviceDetachWithDevid as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> i32);
        (*device).super_.GetDeviceNode = Some(crate::src_hdf_device::HdfDeviceGetDeviceNode as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode);
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_device_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfDeviceDestruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        let mut dev_node = ((*device).devNodes.next as *mut u8).offset(-(0 as isize)) as *mut crate::types::HdfDeviceNode;
        while !dev_node.is_null() && &(*dev_node).entry as *const _ != &(*device).devNodes as *const _ {
            let tmp = ((*dev_node).entry.next as *mut u8).offset(-(0 as isize)) as *mut crate::types::HdfDeviceNode;
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(dev_node);
            dev_node = tmp;
        }
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}

pub extern "C" fn HdfDeviceCreate() -> *mut crate::types::HdfObject {
    let device = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::HdfDevice>()) } as *mut crate::types::HdfDevice;
    if !device.is_null() {
        crate::src_hdf_device::HdfDeviceConstruct(device);
    }
    device as *mut crate::types::HdfObject
}

pub extern "C" fn HdfDeviceRelease(object: *mut crate::types::HdfObject) {
    let device = object as *mut crate::types::HdfDevice;
    if !device.is_null() {
        crate::src_hdf_device::HdfDeviceDestruct(device);
        unsafe {
            libc::free(device as *mut core::ffi::c_void);
        }
    }
}

pub extern "C" fn HdfDeviceNewInstance() -> *mut crate::types::HdfDevice {
    unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE as i32) as *mut crate::types::HdfDevice
    }
}

pub extern "C" fn HdfDeviceFreeInstance(device: *mut crate::types::HdfDevice) {
    if !device.is_null() {
        unsafe {
            crate::compat::HdfObjectManagerFreeObject(&mut (*device).super_.object);
        }
    }
}
