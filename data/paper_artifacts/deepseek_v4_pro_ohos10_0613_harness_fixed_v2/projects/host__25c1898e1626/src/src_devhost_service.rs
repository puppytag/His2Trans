//! Module: src_devhost_service
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

fn DevHostServiceFindDevice(hostService: *mut crate::types::DevHostService, deviceId: u16)-> *mut crate::types::HdfDevice {
    if hostService.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: null pointer deref is used to compute offset, which is safe.
    let node_offset = unsafe {
        core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).node) as usize
    };
    // SAFETY: hostService is non-null, get mutable pointer to devices field.
    let devices: *mut crate::types::DListHead = unsafe { core::ptr::addr_of_mut!((*hostService).devices) };
    let mut next = unsafe { (*devices).next };

    while !core::ptr::eq(next, devices) {
        // SAFETY: next is a valid pointer, container_of is pointer arithmetic.
        let device = unsafe { (next as *mut u8).sub(node_offset) as *mut crate::types::HdfDevice };
        // SAFETY: device points to a valid HdfDevice.
        let dev_id = unsafe { (*device).deviceId };
        let id = ((dev_id >> 8) & 0xFFFF) as u16;

        if id == deviceId {
            return device;
        }
        // SAFETY: device->node.next is valid.
        next = unsafe { (*device).node.next };
    }

    std::ptr::null_mut()
}

fn DevHostServiceFreeDevice(hostService: *mut crate::types::DevHostService, device: *mut crate::types::HdfDevice) {
    let _ = hostService;
    if device.is_null() {
        return;
    }
    unsafe {
        let entry = core::ptr::addr_of_mut!((*device).node);
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
        (*entry).prev = core::ptr::null_mut();
        (*entry).next = core::ptr::null_mut();
    }
    crate::src_hdf_device::HdfDeviceFreeInstance(device);
}

fn DevHostServiceQueryOrAddDevice(inst: *mut crate::types::DevHostService, deviceId: u16)-> *mut crate::types::HdfDevice {
    let mut device = crate::src_devhost_service::DevHostServiceFindDevice(inst, deviceId);
    if device.is_null() {
        // HDF_LOGD("%{public}s can't find device, try to create", __func__);
        device = crate::src_hdf_device::HdfDeviceNewInstance();
        if device.is_null() {
            // HDF_LOGE("Dev host service failed to create driver instance");
            return std::ptr::null_mut();
        }
        let host_id = unsafe { (*inst).hostId } as u32;
        let dev_id = deviceId as u32;
        let new_devid = (host_id << 24) | (dev_id << 8) | 0;
        unsafe {
            (*device).deviceId = new_devid;

            let entry_ptr: *mut crate::types::DListHead = &mut (*device).node;
            let head_ptr: *mut crate::types::DListHead = &mut (*inst).devices;
            (*entry_ptr).next = (*head_ptr).next;
            (*entry_ptr).prev = head_ptr;
            (*(*head_ptr).next).prev = entry_ptr;
            (*head_ptr).next = entry_ptr;
        }
        // HDF_LOGD("%{public}s add device complete", __func__);
    }
    device
}

pub extern "C" fn DevHostServiceAddDevice(inst: *mut crate::types::IDevHostService, deviceInfo: *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::types::HDF_FAILURE;
    let hostService = inst as *mut crate::types::DevHostService;

    let driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();

    // Early safe null checks
    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null() {
        return ret;
    }

    // Fetch get_driver_fn and deviceIdPart (only raw pointer reads need unsafe)
    let get_driver_fn = {
        // SAFETY: driverLoader is non-null (checked above)
        let loader = unsafe { &*driverLoader };
        if loader.GetDriver.is_none() {
            return ret;
        }
        loader.GetDriver
    };
    let deviceIdPart = unsafe { (*deviceInfo).deviceId >> 8 & 0xFFFF } as u16;

    let device = crate::src_devhost_service::DevHostServiceQueryOrAddDevice(hostService, deviceIdPart);

    // Obtain device super_ function pointers
    let (attach_fn, get_device_node_fn) = {
        // SAFETY: device is non-null (checked above)
        let dev = unsafe { &*device };
        if dev.super_.Attach.is_none() {
            return crate::types::HDF_DEV_ERR_NO_DEVICE;
        }
        (dev.super_.Attach, dev.super_.GetDeviceNode)
    };

    let devNode = if let Some(get_dev_node) = get_device_node_fn {
        unsafe { get_dev_node(&mut (*device).super_, (*deviceInfo).deviceId) }
    } else {
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    };

    if !devNode.is_null() {
        return crate::types::HDF_ERR_DEVICE_BUSY;
    }

    let driver = unsafe {
        get_driver_fn.expect("GetDriver not set")((*deviceInfo).moduleName)
    };

    if driver.is_null() {
        ret = crate::types::HDF_DEV_ERR_NODATA;
        let device_list_empty = unsafe {
            let head_ptr: *const crate::types::DListHead = core::ptr::addr_of!((*device).devNodes);
            (*head_ptr).next as *const crate::types::DListHead == head_ptr
        };
        if device_list_empty {
            crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
        }
        return ret;
    }

    let devNode = crate::src_hdf_device_node::HdfDeviceNodeNewInstance(deviceInfo, driver);
    if devNode.is_null() {
        unsafe {
            if let Some(reclaim) = (*driverLoader).ReclaimDriver {
                reclaim(driver);
            }
        }
        return crate::types::HDF_DEV_ERR_NO_MEMORY;
    }

    // Assign devNode fields and call attach
    unsafe {
        (*devNode).hostService = hostService;
        (*devNode).device = device;
        (*devNode).driver = driver;
    }
    ret = unsafe { attach_fn.expect("Attach not set")(&mut (*device).super_, devNode) };

    if ret != crate::types::HDF_SUCCESS {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        let device_list_empty = unsafe {
            let head_ptr: *const crate::types::DListHead = core::ptr::addr_of!((*device).devNodes);
            (*head_ptr).next as *const crate::types::DListHead == head_ptr
        };
        if device_list_empty {
            crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
        }
        return ret;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn DevHostServiceDelDevice(inst: *mut crate::types::IDevHostService, devId: crate::types::devid_t) -> ::core::ffi::c_int {
    let host_service = inst as *mut crate::types::DevHostService;
    let device_id = ((devId >> 8) & 0xffff) as u16;

    let device = crate::src_devhost_service::DevHostServiceFindDevice(host_service, device_id);
    if device.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_WARN,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to del device, device is not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_SUCCESS;
    }

    let super_ptr: *mut crate::types::IHdfDevice = unsafe { &mut (*device).super_ };

    let dev_node = unsafe {
        match (*super_ptr).GetDeviceNode {
            Some(f) => f(super_ptr, devId),
            None => return crate::types::HDF_ERR_INVALID_OBJECT,
        }
    };
    if dev_node.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to del device, not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    let detach = unsafe {
        match (*super_ptr).Detach {
            Some(f) => f,
            None => {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to del device, invalid device\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return crate::types::HDF_ERR_INVALID_OBJECT;
            }
        }
    };
    if unsafe { detach(super_ptr, dev_node) } != crate::types::HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to detach device\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(dev_node);

    let device_list_empty = unsafe {
        let dev_nodes_ptr: *const crate::types::DListHead = &(*device).devNodes;
        (*dev_nodes_ptr).next as *const crate::types::DListHead == dev_nodes_ptr
    };
    if device_list_empty {
        crate::src_devhost_service::DevHostServiceFreeDevice(host_service, device);
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
            b"%{public}s add device success\0".as_ptr() as *const ::core::ffi::c_char,
            b"DevHostServiceDelDevice\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }

    crate::types::HDF_SUCCESS
}

fn DevHostServiceStartService(service: *mut crate::types::IDevHostService)-> i32 {
    let host_service = service as *mut crate::types::DevHostService;
    if host_service.is_null() {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        let host_id = (*host_service).hostId;
        crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDeviceHost(host_id, service)
    }
}

fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32)-> i32 {
    const POWER_STATE_RESUME: u32 = 1;
    const POWER_STATE_DOZE_RESUME: u32 = 3;
    let is_wake = state == POWER_STATE_RESUME || state == POWER_STATE_DOZE_RESUME;
    let mut ret: i32 = 0;
    let entry_offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDeviceNode>()).entry) as usize };

    // SAFETY: we only dereference device pointer, which is checked before calling this function
    let head = unsafe { core::ptr::addr_of_mut!((*device).devNodes) as *const crate::types::DListHead };
    let iter = unsafe { crate::src_hdf_device::DlListIter::<crate::types::HdfDeviceNode>::new(head, entry_offset) };
    if is_wake {
        for device_node in iter {
            let power_token = unsafe { (*device_node).powerToken };
            if !power_token.is_null() {
                ret = crate::src_power_state_token::PowerStateChange(power_token, state);
                if ret != crate::types::HDF_SUCCESS {
                    // logging omitted: HiLogPrint not available
                }
            }
        }
    } else {
        // Reverse traversal: collect into Vec and iterate reversed
        let nodes: Vec<*mut crate::types::HdfDeviceNode> = iter.collect();
        for device_node in nodes.into_iter().rev() {
            let power_token = unsafe { (*device_node).powerToken };
            if !power_token.is_null() {
                ret = crate::src_power_state_token::PowerStateChange(power_token, state);
                if ret != crate::types::HDF_SUCCESS {
                    // logging omitted
                }
            }
        }
    }

    crate::types::HDF_SUCCESS
}

fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32)-> i32 {
    let mut ret = crate::types::HDF_SUCCESS;
    let host_service: *mut crate::types::DevHostService = unsafe {
        let base = service as *mut u8;
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::DevHostService>()).super_) as isize;
        base.offset(-offset) as *mut crate::types::DevHostService
    };
    if host_service.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let is_wake = state == crate::types::POWER_STATE_RESUME || state == crate::types::POWER_STATE_DOZE_RESUME;
    let devices_ptr: *const crate::types::DListHead = unsafe { std::ptr::addr_of!((*host_service).devices) };
    let device_offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).node) as isize };
    if is_wake {
        // Reverse traversal — must preserve order; keep narrowed unsafe loop.
        let mut node_ptr = unsafe { (*devices_ptr).prev };
        while node_ptr as *const _ != devices_ptr {
            let device_ptr = unsafe { (node_ptr as *mut u8).offset(-device_offset) as *mut crate::types::HdfDevice };
            if crate::src_devhost_service::ApplyDevicesPowerState(device_ptr, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node_ptr = unsafe { (*device_ptr).node.prev };
        }
    } else {
        // Forward traversal — use DlListIter to reduce unsafe surface.
        let iter = unsafe { crate::src_hdf_device::DlListIter::<crate::types::HdfDevice>::new(devices_ptr, device_offset as usize) };
        for device_ptr in iter {
            if crate::src_devhost_service::ApplyDevicesPowerState(device_ptr, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
        }
    }
    ret
}

pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let host_service_if = core::ptr::addr_of_mut!((*service).super_);
        (*host_service_if).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice);
        (*host_service_if).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice);
        let start_fn: unsafe extern "C" fn(*mut crate::types::IDevHostService) -> i32 =
            std::mem::transmute(crate::src_devhost_service::DevHostServiceStartService as *const ());
        (*host_service_if).StartService = Some(start_fn);
        let pm_fn: unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> i32 =
            std::mem::transmute(crate::src_devhost_service::DevHostServicePmNotify as *const ());
        (*host_service_if).PmNotify = Some(pm_fn);

        let devices = core::ptr::addr_of_mut!((*service).devices);
        (*devices).next = devices;
        (*devices).prev = devices;

        let observer = core::ptr::addr_of_mut!((*service).observer);
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(observer);
    }
}

pub extern "C" fn DevHostServiceDestruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }

    // Offset of `node` field in HdfDevice
    let offset = unsafe { core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDevice>()).node) as usize };

    // Head of the device list
    let head: *const crate::types::DListHead = unsafe { std::ptr::addr_of!((*service).devices) };

    // Use DlListIter to free each device
    let iter = unsafe { crate::src_hdf_device::DlListIter::<crate::types::HdfDevice>::new(head, offset) };
    for device in iter {
        crate::src_hdf_device::HdfDeviceFreeInstance(device);
    }

    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverDestruct(
            core::ptr::addr_of_mut!((*service).observer),
        );
    }
}

pub extern "C" fn DevHostServiceCreate() -> *mut crate::types::HdfObject {
    let size = std::mem::size_of::<crate::types::DevHostService>();
    let dev_host_service = unsafe { libc::calloc(1, size) as *mut crate::types::DevHostService };
    if !dev_host_service.is_null() {
        crate::src_devhost_service::DevHostServiceConstruct(dev_host_service);
    }
    dev_host_service as *mut crate::types::HdfObject
}

pub extern "C" fn DevHostServiceRelease(object: *mut crate::types::HdfObject) {
    let devHostService = object as *mut crate::types::DevHostService;
    if !devHostService.is_null() {
        unsafe {
            crate::src_devhost_service::DevHostServiceDestruct(devHostService);
            OsalMemFree(devHostService as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn DevHostServiceNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::IDevHostService {
    let host_service = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVHOST_SERVICE as i32) as *mut crate::types::DevHostService };
    if !host_service.is_null() && !hostName.is_null() {
        unsafe {
            (*host_service).hostId = hostId;
            (*host_service).hostName = hostName;
        }
    }
    host_service.cast::<crate::types::IDevHostService>()
}

pub extern "C" fn DevHostServiceFreeInstance(service: *mut crate::types::IDevHostService) {
    if !service.is_null() {
        unsafe {
            crate::compat::HdfObjectManagerFreeObject(
                &mut (*service).object as *mut crate::types::HdfObject,
            );
        }
    }
}
