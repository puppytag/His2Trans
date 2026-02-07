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
    if device.is_null() {
        return node_id as crate::types::devid_t;
    }
    unsafe {
        let devid_index = (*device).devidIndex;
        while node_id <= devid_index {
            find = false;
            let dev_nodes_head = &(*device).devNodes as *const crate::types::DListHead;
            let mut entry_ptr = (*dev_nodes_head).next;
            while entry_ptr != dev_nodes_head as *mut crate::types::DListHead {
                let dev_node = (entry_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
                let dev_id = (*dev_node).devId;
                if (dev_id & ((1 << 8) - 1)) as u16 == node_id {
                    find = true;
                    break;
                }
                entry_ptr = (*entry_ptr).next;
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
    let mut node_id: crate::types::devid_t = 0;
    let usable_id: crate::types::devid_t;
    unsafe {
        if (*device).devidIndex >= ((1 << 8) - 1) as u16 {
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
        let high_part = ((device_id >> (16 + 8)) as u32) << (16 + 8);
        let mid_part = (((device_id >> 8) & ((1 << 16) - 1)) as u32) << 8;
        *devid = high_part | mid_part | node_id;
    }
    crate::types::HDF_SUCCESS
}

fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let device = devInst as *mut crate::types::HdfDevice;
    let node_if = devNode as *mut crate::types::IDeviceNode;
    if device.is_null() || node_if.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"hdf_device\0".as_ptr() as *const _,
            b"failed to attach device, input params invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let launch_node = unsafe { (*node_if).LaunchNode };
    if launch_node.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"hdf_device\0".as_ptr() as *const _,
            b"failed to attach device, input params invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_id = unsafe { (*devNode).devId };
    if dev_id == 0 {
        let ret = unsafe { crate::src_hdf_device::AcquireNodeDeivceId(device, &mut (*devNode).devId as *mut crate::types::devid_t) };
        if ret != crate::types::HDF_SUCCESS {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, invalid device id\0".as_ptr() as *const _,
            ) };
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
    }
    let token = unsafe { (*devNode).token };
    if !token.is_null() {
        unsafe { (*token).devid = (*devNode).devId };
    }
    let ret = unsafe { launch_node.unwrap()(devNode) };
    if ret == crate::types::HDF_SUCCESS {
        let entry = unsafe { &mut (*devNode).entry as *mut crate::types::DListHead };
        let head = unsafe { &mut (*device).devNodes as *mut crate::types::DListHead };
        unsafe {
            (*entry).next = head;
            (*entry).prev = (*head).prev;
            (*(*head).prev).next = entry;
            (*head).prev = entry;
        }
        unsafe { crate::src_hdf_device::UpdateDeivceNodeIdIndex(device, (*devNode).devId) };
    }
    ret
}

pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let device = unsafe {
        let offset = std::mem::offset_of!(HdfDevice, super_) as usize;
        (devInst as *mut u8).sub(offset) as *mut HdfDevice
    };
    let device_id_mask = unsafe { ((*device).deviceId >> 8) & 0xFFFF };
    let dev_node_id_mask = unsafe { ((*devNode).devId >> 8) & 0xFFFF };
    if device_id_mask != dev_node_id_mask {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"%{public}s: device detach unknown devnode \0".as_ptr() as *const _,
                b"HdfDeviceDetach\0".as_ptr() as *const _,
            )
        };
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
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let dev = unsafe { (device as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, super_) as isize)) as *mut crate::types::HdfDevice };
    let dev_nodes_ptr = unsafe { std::ptr::addr_of_mut!((*dev).devNodes) };
    let mut entry = unsafe { (*dev_nodes_ptr).next };
    while entry != dev_nodes_ptr {
        devNode = unsafe { (entry as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode };
        if unsafe { (*devNode).devId } == devid {
            return devNode;
        }
        entry = unsafe { (*entry).next };
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_8/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_hdf_device.rs:135:40
//       |
//       |                                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected "C" fn, found "Rust" fn
//       |                                   |
//       |                                   arguments to this enum variant are incorrect
//       |
//   help: the type constructed contains `fn(*mut IHdfDevice, *mut HdfDeviceNode) -> i32 {HdfDeviceAttach}` due to the type of the argument passed
// =================================
pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_device::HdfDeviceConstruct(device as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_device_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_8/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::src_hdf_device::HdfDeviceAttach);
        (*device).super_.Detach = Some(crate::src_hdf_device::HdfDeviceDetach);
        (*device).super_.DetachWithDevid = Some(crate::src_hdf_device::HdfDeviceDetachWithDevid);
        (*device).super_.GetDeviceNode = Some(crate::src_hdf_device::HdfDeviceGetDeviceNode);
        (*device).devNodes.next = device as *mut crate::types::DListHead;
        (*device).devNodes.prev = device as *mut crate::types::DListHead;
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
        let mut dev_node = (*device).devNodes.next;
        while dev_node != &mut (*device).devNodes as *mut crate::types::DListHead {
            let tmp = (*dev_node).next;
            let dev_node_ptr = (dev_node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(dev_node_ptr);
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
            libc::free(device as *mut libc::c_void);
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
