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
use std::marker::PhantomData;

pub(crate) struct DlListIter<T> {
    head: *const crate::types::DListHead,
    entry: *const crate::types::DListHead,
    offset: usize,
    _marker: PhantomData<*const T>,
}

impl<T> DlListIter<T> {
    /// # Safety
    /// `head` must be a valid pointer to a DListHead sentinel; the list must be stable.
    pub(crate) unsafe fn new(head: *const crate::types::DListHead, offset: usize) -> Self {
        let entry = (*head).next;
        DlListIter {
            head,
            entry,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<T> Iterator for DlListIter<T> {
    type Item = *mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.entry == self.head {
            return None;
        }
        let current = self.entry;
        // container_of: subtract offset to get pointer to the container struct
        let node = unsafe { (current as *mut u8).sub(self.offset) as *mut T };
        // SAFETY: current is a valid DListHead pointer; next is valid
        self.entry = unsafe { (*current).next };
        Some(node)
    }
}

fn UpdateDeivceNodeIdIndex(device: *mut crate::types::HdfDevice, nodeDevid: crate::types::devid_t) {
    let masked = (nodeDevid & 0xFF) as u16;
    let devid_index = unsafe { (*device).devidIndex };
    if devid_index < masked {
        unsafe { (*device).devidIndex = masked; }
    }
}

fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    // SAFETY: caller ensures device is valid
    let device_ref = unsafe { &*device };
    let mut node_id: u16 = 129;
    let head = core::ptr::addr_of!(device_ref.devNodes);
    // SAFETY: offset between HdfDeviceNode and its DListHead entry field
    let entry_offset = unsafe {
        let null_node: *const crate::types::HdfDeviceNode = core::ptr::null();
        core::ptr::addr_of!((*null_node).entry) as usize
    };
    while node_id <= device_ref.devidIndex {
        let mut find = false;
        let iter = unsafe { DlListIter::<crate::types::HdfDeviceNode>::new(head, entry_offset) };
        for dev_node in iter {
            // SAFETY: dev_node is a valid pointer from the list
            let devid = unsafe { (*dev_node).devId };
            if (devid as u32 & 0xFF) == node_id as u32 {
                find = true;
                break;
            }
        }
        if !find {
            return node_id as crate::types::devid_t;
        }
        node_id = node_id.wrapping_add(1);
    }
    node_id as crate::types::devid_t
}

fn AcquireNodeDeivceId(device: *mut crate::types::HdfDevice, devid: *mut crate::types::devid_t)-> i32 {
    let nodeId: crate::types::devid_t;

    // Early checks: raw deref required
    let devid_index = unsafe { (*device).devidIndex };
    if devid_index >= 255u16 {
        return crate::types::HDF_FAILURE;
    }

    if devid_index < 129u16 {
        unsafe { (*device).devidIndex = 129u16; }
        nodeId = 129;
    } else {
        let usable_id = FindUsableDevNodeId(device);
        if usable_id <= devid_index as crate::types::devid_t {
            nodeId = usable_id;
        } else {
            unsafe { (*device).devidIndex += 1; }
            nodeId = unsafe { (*device).devidIndex as crate::types::devid_t };
        }
    }

    if devid.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // Build new devid
    let device_id = unsafe { (*device).deviceId };
    let part1: u32 = ((device_id >> 24) as u16 as u32) << 24;
    let part2: u32 = ((device_id >> 8) & 0xFFFF) << 8;
    let new_devid = part1 | part2 | (nodeId as u32);
    unsafe { *devid = new_devid; }

    crate::types::HDF_SUCCESS
}

fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode)-> i32 {
    // Null pointer checks are safe comparisons
    if devInst.is_null() || devNode.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device, input params invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let device = devInst as *mut crate::types::HdfDevice;

    // Check LaunchNode presence (raw pointer deref)
    let launch_node_exists = unsafe { (*devNode).super_.LaunchNode.is_some() };
    if !launch_node_exists {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device, input params invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // Acquire device id if needed
    let dev_id = unsafe { (*devNode).devId };
    if dev_id == 0 {
        let ret = unsafe {
            AcquireNodeDeivceId(
                device,
                core::ptr::addr_of_mut!((*devNode).devId) as *mut crate::types::devid_t,
            )
        };
        if ret != crate::types::HDF_SUCCESS {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to attach device, invalid device id\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
    }

    // Set token devid
    unsafe {
        (*(*devNode).token).devid = (*devNode).devId;
    }

    // Launch node callback
    let launch = unsafe { (*devNode).super_.LaunchNode.unwrap() };
    let ret = unsafe { launch(devNode) };

    // DList insert on success
    if ret == crate::types::HDF_SUCCESS {
        unsafe {
            let entry = core::ptr::addr_of_mut!((*devNode).entry);
            let head = core::ptr::addr_of_mut!((*device).devNodes);
            (*entry).next = head;
            (*entry).prev = (*head).prev;
            (*(*head).prev).next = entry;
            (*head).prev = entry;
        }
        UpdateDeivceNodeIdIndex(device, unsafe { (*devNode).devId });
    }

    ret
}

pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if devInst.is_null() || devNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // container_of: compute device pointer from devInst using super_ field offset
    let offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).super_) as usize };
    let device = unsafe { (devInst as *mut u8).sub(offset) as *mut crate::types::HdfDevice };

    // Read deviceId and node devId
    let device_id = unsafe { (*device).deviceId };
    let node_id = unsafe { (*devNode).devId };

    if ((device_id >> 8) & 0xFFFF) != ((node_id >> 8) & 0xFFFF) {
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    // DList remove if entry is linked
    let entry_linked = unsafe { !(*devNode).entry.next.is_null() };
    if entry_linked {
        unsafe {
            let entry: *mut crate::types::DListHead = &mut (*devNode).entry as *mut crate::types::DListHead;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
    }

    // Unlaunch callback if present
    let unlaunch_opt = unsafe { (*devNode).super_.UnlaunchNode };
    if let Some(unlaunch) = unlaunch_opt {
        unsafe { unlaunch(devNode) };
    }

    crate::types::HDF_SUCCESS
}

fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t)-> *mut crate::types::HdfDeviceNode {
    let super_offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).super_) as usize };
    let entry_offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDeviceNode>()).entry) as usize };
    let dev = unsafe { (device as *mut u8).sub(super_offset) as *mut crate::types::HdfDevice };
    let head = unsafe { &mut (*dev).devNodes as *mut crate::types::DListHead };
    let iter = unsafe { DlListIter::<crate::types::HdfDeviceNode>::new(head, entry_offset) };
    for dev_node_ptr in iter {
        let dev_id = unsafe { (*dev_node_ptr).devId };
        if dev_id == devid {
            return dev_node_ptr;
        }
    }
    core::ptr::null_mut()
}

fn HdfDeviceDetachWithDevid(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t)-> i32 {
    let dev = unsafe {
        (device as *mut u8).offset(-(unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).super_) as isize })) as *mut crate::types::HdfDevice
    };
    let _ = dev;
    let devNode = crate::src_hdf_device::HdfDeviceGetDeviceNode(device, devid);
    if devNode.is_null() {
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    crate::src_hdf_device::HdfDeviceDetach(device, devNode) as i32
}

fn hdf_device_attach_transmute() -> unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32 {
    unsafe {
        std::mem::transmute(
            crate::src_hdf_device::HdfDeviceAttach
                as fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32,
        )
    }
}

fn hdf_device_detach_transmute() -> unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32 {
    // HdfDeviceDetach is already pub extern "C" fn; coerce via double as
    crate::src_hdf_device::HdfDeviceDetach
        as extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32
        as unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32
}

fn hdf_device_detach_with_devid_transmute() -> unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> i32 {
    unsafe {
        std::mem::transmute(
            crate::src_hdf_device::HdfDeviceDetachWithDevid
                as fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> i32,
        )
    }
}

fn hdf_device_get_device_node_transmute() -> unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    unsafe {
        std::mem::transmute(
            crate::src_hdf_device::HdfDeviceGetDeviceNode
                as fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode,
        )
    }
}

fn hdf_device_dlist_head_init(head: &mut crate::types::DListHead) {
    let ptr: *mut crate::types::DListHead = head;
    head.next = ptr;
    head.prev = ptr;
}

pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    let dev = unsafe { &mut *device };
    // Assign vtable entries using transmute wrappers
    unsafe {
        dev.super_.Attach = Some(hdf_device_attach_transmute());
        dev.super_.Detach = Some(hdf_device_detach_transmute());
        dev.super_.DetachWithDevid = Some(hdf_device_detach_with_devid_transmute());
        dev.super_.GetDeviceNode = Some(hdf_device_get_device_node_transmute());
    }
    // Safe DListHead initialization
    hdf_device_dlist_head_init(&mut dev.devNodes);
}

pub extern "C" fn HdfDeviceDestruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }

    // Compute offset of `entry` field in HdfDeviceNode
    let entry_offset = unsafe {
        core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDeviceNode>()).entry) as usize
    };

    let head = unsafe { core::ptr::addr_of!((*device).devNodes) };

    // Iterate DList via safe iterator; the iterator internally uses unsafe for container_of.
    let iter = unsafe { DlListIter::<crate::types::HdfDeviceNode>::new(head, entry_offset) };
    for dev_node in iter {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(dev_node);
    }

    // Safe DListHead reinitialization
    let dev = unsafe { &mut *device };
    let ptr: *mut crate::types::DListHead = &mut dev.devNodes;
    dev.devNodes.next = ptr;
    dev.devNodes.prev = ptr;
}

pub extern "C" fn HdfDeviceCreate() -> *mut crate::types::HdfObject {
    let device = unsafe {
        let layout = std::alloc::Layout::new::<crate::types::HdfDevice>();
        std::alloc::alloc_zeroed(layout) as *mut crate::types::HdfDevice
    };
    if !device.is_null() {
        crate::src_hdf_device::HdfDeviceConstruct(device);
    }
    device as *mut crate::types::HdfObject
}

pub extern "C" fn HdfDeviceRelease(object: *mut crate::types::HdfObject) {
    let device = object as *mut crate::types::HdfDevice;
    if device.is_null() {
        return;
    }
    crate::src_hdf_device::HdfDeviceDestruct(device);
    unsafe {
        crate::compat::OsalMemFree(device as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn HdfDeviceNewInstance() -> *mut crate::types::HdfDevice {
    unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE as i32) as *mut crate::types::HdfDevice }
}

pub extern "C" fn HdfDeviceFreeInstance(device: *mut crate::types::HdfDevice) {
    if !device.is_null() {
        unsafe {
            let obj = &mut (*device).super_.object as *mut crate::types::HdfObject;
            crate::compat::HdfObjectManagerFreeObject(obj);
        }
    }
}
