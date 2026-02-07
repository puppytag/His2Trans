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
    unsafe {
        let node_id_index = (nodeDevid as u32) & ((1u32 << 8) - 1);
        if (*device).devidIndex < node_id_index as u16 {
            (*device).devidIndex = node_id_index as u16;
        }
    }
}

fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    let mut nodeId: u16 = 129;
    let mut find: bool;
    let mut devNode: *mut crate::types::HdfDeviceNode;
    
    // Calculate offset of entry field in HdfDeviceNode using MaybeUninit
    let entry_offset = {
        let dummy = std::mem::MaybeUninit::<crate::types::HdfDeviceNode>::uninit();
        let base_ptr = dummy.as_ptr();
        unsafe { std::ptr::addr_of!((*base_ptr).entry) as usize - base_ptr as usize }
    };
    
    unsafe {
        while nodeId <= (*device).devidIndex {
            find = false;
            
            // DLIST_FOR_EACH_ENTRY expansion:
            let list_head = &(*device).devNodes as *const crate::types::DListHead;
            let mut current = (*list_head).next;
            
            while current != list_head as *mut crate::types::DListHead {
                devNode = (current as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
                
                // Check if DEVNODEID(devNode->devId) == nodeId
                // DEVNODEID extracts lower 8 bits: (devId & ((1 << 8) - 1))
                let dev_node_id = ((*devNode).devId as u32) & ((1u32 << 8) - 1);
                if dev_node_id == nodeId as u32 {
                    find = true;
                    break;
                }
                
                current = (*current).next;
            }
            
            if !find {
                return nodeId as crate::types::devid_t;
            }
            
            nodeId += 1;
        }
    }
    
    nodeId as crate::types::devid_t
}

fn AcquireNodeDeivceId(device: *mut crate::types::HdfDevice, devid: *mut crate::types::devid_t) -> ::core::ffi::c_int {
    unsafe {
        let mut nodeId: crate::types::devid_t;
        let usableId: crate::types::devid_t;
        
        if (*device).devidIndex >= ((1u16 << 8) - 1) {
            return crate::types::HDF_FAILURE;
        }
        
        if (*device).devidIndex < 129 {
            (*device).devidIndex = 129;
            nodeId = (*device).devidIndex as crate::types::devid_t;
        } else {
            usableId = crate::src_hdf_device::FindUsableDevNodeId(device);
            if usableId <= (*device).devidIndex as crate::types::devid_t {
                nodeId = usableId;
            } else {
                (*device).devidIndex += 1;
                nodeId = (*device).devidIndex as crate::types::devid_t;
            }
        }
        
        if devid.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"params invalid *devid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        
        let device_id = (*device).deviceId;
        let host_id = ((device_id as u32) >> (16 + 8)) as u16;
        let dev_id = (((device_id as u32) >> 8) & ((1u32 << 16) - 1)) as u16;
        
        *devid = ((host_id as u32) << (16 + 8)) | ((dev_id as u32) << 8) | nodeId;
        
        crate::types::HDF_SUCCESS
    }
}

fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let device = devInst as *mut crate::types::HdfDevice;
    let nodeIf = devNode as *mut crate::types::IDeviceNode;

    if device.is_null() || nodeIf.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device, input params invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let launch_node = unsafe { (*nodeIf).LaunchNode };
    if launch_node.is_none() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device, input params invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    unsafe {
        if (*devNode).devId == 0 && crate::src_hdf_device::AcquireNodeDeivceId(device, &mut (*devNode).devId) != crate::types::HDF_SUCCESS {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device, invalid device id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }

        (*(*devNode).token).devid = (*devNode).devId;

        let ret = launch_node.unwrap()(devNode);

        if ret == crate::types::HDF_SUCCESS {
            let entry = &mut (*devNode).entry;
            let head = &mut (*device).devNodes;
            (*entry).next = head;
            (*entry).prev = (*head).prev;
            (*(*head).prev).next = entry;
            (*head).prev = entry;

            crate::src_hdf_device::UpdateDeivceNodeIdIndex(device, (*devNode).devId);
        }

        ret
    }
}

pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF: device = (struct HdfDevice *)((char *)(devInst) - (char *)&((struct HdfDevice *)0)->super)
    // Calculate offset using memoffset-style calculation without null deref
    let device: *mut HdfDevice = unsafe {
        let offset = core::mem::offset_of!(HdfDevice, super_);
        (devInst as *mut u8).sub(offset) as *mut HdfDevice
    };
    
    // Extract device IDs and compare: ((deviceId >> 8) & ((1 << 16) - 1))
    let device_id = unsafe { (*device).deviceId };
    let dev_node_id = unsafe { (*devNode).devId };
    
    let mask: u32 = (1 << 16) - 1;
    let device_id_masked = (device_id >> 8) & mask;
    let dev_node_id_masked = (dev_node_id >> 8) & mask;
    
    if device_id_masked != dev_node_id_masked {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: device detach unknown devnode \0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfDeviceDetach\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    // DListRemove inline implementation
    unsafe {
        if !(*devNode).entry.next.is_null() {
            let entry = &mut (*devNode).entry;
            (*entry.prev).next = entry.next;
            (*entry.next).prev = entry.prev;
            entry.prev = std::ptr::null_mut();
            entry.next = std::ptr::null_mut();
        }
    }
    
    // Call UnlaunchNode if present
    unsafe {
        if let Some(unlaunch_fn) = (*devNode).super_.UnlaunchNode {
            unlaunch_fn(devNode);
        }
    }
    
    HDF_SUCCESS
}

fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    unsafe {
        let super_offset = std::mem::offset_of!(crate::types::HdfDevice, super_);
        let dev = (device as *mut u8).sub(super_offset) as *mut crate::types::HdfDevice;
        
        let entry_offset = std::mem::offset_of!(crate::types::HdfDeviceNode, entry);
        let dev_nodes_head = &(*dev).devNodes as *const crate::types::DListHead;
        
        let mut current = (*dev_nodes_head).next;
        
        while current != dev_nodes_head as *mut crate::types::DListHead {
            let dev_node = (current as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
            
            if (*dev_node).devId == devid {
                return dev_node;
            }
            
            current = (*current).next;
        }
        
        std::ptr::null_mut()
    }
}

fn HdfDeviceDetachWithDevid(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> ::core::ffi::c_int {
    let _dev: *mut crate::types::HdfDevice = unsafe {
        (device as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, super_) as isize)) as *mut crate::types::HdfDevice
    };
    let _ = _dev;
    
    let devNode = crate::src_hdf_device::HdfDeviceGetDeviceNode(device, devid);
    if devNode.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"devNode is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    
    crate::src_hdf_device::HdfDeviceDetach(device, devNode)
}

pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    unsafe {
        (*device).super_.Attach = Some(std::mem::transmute::<usize, unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceAttach as usize));
        (*device).super_.Detach = Some(std::mem::transmute::<usize, unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceDetach as usize));
        (*device).super_.DetachWithDevid = Some(std::mem::transmute::<usize, unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> ::core::ffi::c_int>(HdfDeviceDetachWithDevid as usize));
        (*device).super_.GetDeviceNode = Some(std::mem::transmute::<usize, unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode>(HdfDeviceGetDeviceNode as usize));

        let head = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
    }
}

pub extern "C" fn HdfDeviceDestruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    
    unsafe {
        // Get the offset of 'entry' field within HdfDeviceNode
        let entry_offset = std::mem::offset_of!(crate::types::HdfDeviceNode, entry);
        
        // Get pointer to devNodes list head
        let head = &mut (*device).devNodes as *mut crate::types::DListHead;
        
        // Initialize iteration: devNode = container_of(head->next, HdfDeviceNode, entry)
        let mut current_entry = (*head).next;
        
        while current_entry != head {
            // Get the HdfDeviceNode from the entry pointer
            let devNode = (current_entry as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
            
            // Save next before freeing current node
            let next_entry = (*current_entry).next;
            
            // Free the device node
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
            
            // Move to next
            current_entry = next_entry;
        }
        
        // DListHeadInit(&device->devNodes)
        (*head).next = head;
        (*head).prev = head;
    }
}

pub extern "C" fn HdfDeviceCreate() -> *mut crate::types::HdfObject {
    let device: *mut crate::types::HdfDevice = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::HdfDevice>() as u32) as *mut crate::types::HdfDevice
    };
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
            crate::compat::OsalMemFree(device as *mut ::core::ffi::c_void);
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
            crate::compat::HdfObjectManagerFreeObject(&mut (*device).super_.object as *mut crate::types::HdfObject);
        }
    }
}
