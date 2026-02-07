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

fn DevHostServiceFindDevice(hostService: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    use crate::types::*;
    
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to find driver, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    unsafe {
        let devices_head = &(*hostService).devices as *const DListHead;
        let mut node_ptr = (*devices_head).next;
        
        while node_ptr != devices_head as *mut DListHead {
            // Calculate offset of 'node' field in HdfDevice
            let node_offset = std::mem::offset_of!(HdfDevice, node);
            let device = (node_ptr as *mut u8).sub(node_offset) as *mut HdfDevice;
            
            // Extract deviceId field and check
            let dev_id = (*device).deviceId;
            let extracted_id = ((dev_id >> 8) & ((1 << 16) - 1)) as u16;
            
            if extracted_id == deviceId {
                return device;
            }
            
            node_ptr = (*node_ptr).next;
        }
    }
    
    std::ptr::null_mut()
}

fn DevHostServiceFreeDevice(hostService: *mut crate::types::DevHostService, device: *mut crate::types::HdfDevice) {
    let _ = hostService;
    if !device.is_null() {
        unsafe {
            let entry = &mut (*device).node;
            let prev = (*entry).prev;
            let next = (*entry).next;
            (*prev).next = next;
            (*next).prev = prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        crate::src_hdf_device::HdfDeviceFreeInstance(device);
    }
}

fn DevHostServiceQueryOrAddDevice(inst: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    let mut device = crate::src_devhost_service::DevHostServiceFindDevice(inst, deviceId);
    if device.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s can't find device, try to create\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        device = crate::src_hdf_device::HdfDeviceNewInstance();
        if device.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Dev host service failed to create driver instance\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return std::ptr::null_mut();
        }
        unsafe {
            let host_id = (*inst).hostId as u32;
            let device_id_shifted = (deviceId as u32) << 8;
            (*device).deviceId = ((host_id << (16 + 8)) | device_id_shifted | 0) as crate::types::devid_t;
            
            // DListInsertHead inline implementation
            let entry = &mut (*device).node as *mut crate::types::DListHead;
            let head = &mut (*inst).devices as *mut crate::types::DListHead;
            (*entry).next = (*head).next;
            (*entry).prev = head;
            (*(*head).next).prev = entry;
            (*head).next = entry;
            
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s add device complete\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    device
}

pub extern "C" fn DevHostServiceAddDevice(inst: *mut crate::types::IDevHostService, deviceInfo: *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    let mut device: *mut HdfDevice = std::ptr::null_mut();
    let mut devNode: *mut HdfDeviceNode = std::ptr::null_mut();
    let mut driver: *mut HdfDriver = std::ptr::null_mut();
    
    // CONTAINER_OF: hostService = (DevHostService*)((char*)inst - offsetof(DevHostService, super_))
    let hostService: *mut DevHostService = unsafe {
        (inst as *mut u8).offset(-(std::mem::offset_of!(DevHostService, super_) as isize)) as *mut DevHostService
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, input param is null\0".as_ptr() as *const ::core::ffi::c_char) };
        return ret;
    }
    
    let getDriver = unsafe { (*driverLoader).GetDriver };
    if getDriver.is_none() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, input param is null\0".as_ptr() as *const ::core::ffi::c_char) };
        return ret;
    }
    
    // DEVICEID macro: ((deviceId >> 8) & ((1 << 16) - 1))
    let deviceId = unsafe { (*deviceInfo).deviceId };
    let deviceIdExtracted: u16 = (((deviceId as u32) >> 8) & ((1u32 << 16) - 1)) as u16;
    
    device = crate::src_devhost_service::DevHostServiceQueryOrAddDevice(hostService, deviceIdExtracted);
    if device.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, device or Attach func is null\0".as_ptr() as *const ::core::ffi::c_char) };
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    let attachFn = unsafe { (*device).super_.Attach };
    if attachFn.is_none() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, device or Attach func is null\0".as_ptr() as *const ::core::ffi::c_char) };
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    let getDeviceNodeFn = unsafe { (*device).super_.GetDeviceNode };
    if let Some(gdn) = getDeviceNodeFn {
        devNode = unsafe { gdn(&mut (*device).super_ as *mut IHdfDevice, deviceId) };
    }
    if !devNode.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, device already exist\0".as_ptr() as *const ::core::ffi::c_char) };
        return HDF_ERR_DEVICE_BUSY;
    }
    
    let moduleName = unsafe { (*deviceInfo).moduleName };
    driver = unsafe { getDriver.unwrap()(moduleName) };
    if driver.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, get driver failed\0".as_ptr() as *const ::core::ffi::c_char) };
        ret = HDF_DEV_ERR_NODATA;
    } else {
        devNode = crate::src_hdf_device_node::HdfDeviceNodeNewInstance(deviceInfo, driver);
        if devNode.is_null() {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, create devNode failed\0".as_ptr() as *const ::core::ffi::c_char) };
            if let Some(reclaim) = unsafe { (*driverLoader).ReclaimDriver } {
                unsafe { reclaim(driver) };
            }
            return HDF_DEV_ERR_NO_MEMORY;
        }
        
        unsafe {
            (*devNode).hostService = hostService;
            (*devNode).device = device;
            (*devNode).driver = driver;
        }
        
        ret = unsafe { attachFn.unwrap()(&mut (*device).super_ as *mut IHdfDevice, devNode) };
        if ret != HDF_SUCCESS {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, attach devNode failed\0".as_ptr() as *const ::core::ffi::c_char) };
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        } else {
            unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_DEBUG, 0xD002510, b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char, b"add device success\0".as_ptr() as *const ::core::ffi::c_char) };
            return HDF_SUCCESS;
        }
    }
    
    // ERROR label logic: DListIsEmpty inline
    let devNodes_head = unsafe { &(*device).devNodes };
    let is_empty = unsafe { (*devNodes_head).next == devNodes_head as *const DListHead as *mut DListHead };
    if is_empty {
        crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
    }
    
    ret
}

pub extern "C" fn DevHostServiceDelDevice(inst: *mut crate::types::IDevHostService, devId: crate::types::devid_t) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let hostService = inst as *mut DevHostService;
    
    let deviceId: u16 = ((devId as u32 >> 8) & ((1 << 16) - 1)) as u16;
    let device = crate::src_devhost_service::DevHostServiceFindDevice(hostService, deviceId);
    
    if device.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_WARN,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to del device, device is not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_SUCCESS;
    }
    
    let get_device_node_fn = unsafe { (*device).super_.GetDeviceNode };
    let devNode = if let Some(f) = get_device_node_fn {
        unsafe { f(&mut (*device).super_ as *mut IHdfDevice, devId) }
    } else {
        std::ptr::null_mut()
    };
    
    if devNode.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to del device, not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    let detach_fn = unsafe { (*device).super_.Detach };
    if detach_fn.is_none() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to del device, invalid device\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let detach_result = unsafe { detach_fn.unwrap()(&mut (*device).super_ as *mut IHdfDevice, devNode) };
    if detach_result != HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to detach device\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
    
    let is_empty = unsafe {
        let head = &(*device).devNodes as *const DListHead;
        (*head).next == head as *mut DListHead
    };
    
    if is_empty {
        crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
    }
    
    unsafe {
        crate::compat::HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
            b"%s add device success\0".as_ptr() as *const ::core::ffi::c_char,
            b"DevHostServiceDelDevice\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    
    HDF_SUCCESS
}

fn DevHostServiceStartService(service: *mut crate::types::IDevHostService) -> ::core::ffi::c_int {
    let hostService = service as *mut crate::types::DevHostService;
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to start device service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let host_id = unsafe { (*hostService).hostId };
    crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDeviceHost(host_id, service)
}

fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut deviceNode: *mut HdfDeviceNode;
    let mut ret: ::core::ffi::c_int;
    
    // Inline IsPowerWakeState logic - need to define these constants or use values
    const POWER_STATE_DOZE_RESUME: u32 = 1;
    const POWER_STATE_RESUME: u32 = 3;
    let is_wake_state = state == POWER_STATE_DOZE_RESUME || state == POWER_STATE_RESUME;
    
    // Calculate offset of entry field in HdfDeviceNode using offset_of pattern
    let entry_offset = {
        let dummy = std::mem::MaybeUninit::<HdfDeviceNode>::uninit();
        let base = dummy.as_ptr();
        unsafe { std::ptr::addr_of!((*base).entry) as usize - base as usize }
    };
    
    unsafe {
        if is_wake_state {
            // Forward iteration: DLIST_FOR_EACH_ENTRY
            let head = std::ptr::addr_of_mut!((*device).devNodes);
            let mut current = (*head).next;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS {
                        let _ = ret;
                    }
                }
                
                current = (*current).next;
            }
        } else {
            // Reverse iteration: DLIST_FOR_EACH_ENTRY_REVERSE
            let head = std::ptr::addr_of_mut!((*device).devNodes);
            let mut current = (*head).prev;
            
            while current != head {
                deviceNode = (current as *mut u8).sub(entry_offset) as *mut HdfDeviceNode;
                
                if !(*deviceNode).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*deviceNode).powerToken, state);
                    if ret != HDF_SUCCESS {
                        let _ = ret;
                    }
                }
                
                current = (*current).prev;
            }
        }
    }
    
    HDF_SUCCESS
}

fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut device: *mut HdfDevice = std::ptr::null_mut();
    let mut ret: ::core::ffi::c_int = HDF_SUCCESS;
    
    // CONTAINER_OF: hostService = (DevHostService*)((char*)service - offsetof(DevHostService, super_))
    let super_offset = std::mem::offset_of!(DevHostService, super_) as isize;
    let hostService: *mut DevHostService = unsafe {
        (service as *mut u8).offset(-super_offset) as *mut DevHostService
    };
    
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to start device service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    unsafe {
        crate::compat::HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
            b"host(%s) set power state=%u\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    
    // IsPowerWakeState inline helper
    let is_power_wake_state = state == POWER_STATE_DOZE_RESUME || state == POWER_STATE_RESUME;
    
    let node_offset = std::mem::offset_of!(HdfDevice, node) as isize;
    
    if is_power_wake_state {
        // DLIST_FOR_EACH_ENTRY_REVERSE: iterate from prev
        unsafe {
            let devices_head = &(*hostService).devices as *const DListHead;
            let mut current = (*devices_head).prev;
            
            while current != devices_head as *mut DListHead {
                device = (current as *mut u8).offset(-node_offset) as *mut HdfDevice;
                
                if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != HDF_SUCCESS {
                    ret = HDF_FAILURE;
                }
                
                current = (*current).prev;
            }
        }
    } else {
        // DLIST_FOR_EACH_ENTRY: iterate from next
        unsafe {
            let devices_head = &(*hostService).devices as *const DListHead;
            let mut current = (*devices_head).next;
            
            while current != devices_head as *mut DListHead {
                device = (current as *mut u8).offset(-node_offset) as *mut HdfDevice;
                
                if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != HDF_SUCCESS {
                    ret = HDF_FAILURE;
                }
                
                current = (*current).next;
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
        let hostServiceIf: *mut crate::types::IDevHostService = &mut (*service).super_;
        (*hostServiceIf).AddDevice = Some(DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(std::mem::transmute::<fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int>(DevHostServiceStartService));
        (*hostServiceIf).PmNotify = Some(std::mem::transmute::<fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int>(DevHostServicePmNotify));
        
        // Inline DListHeadInit
        let head: *mut crate::types::DListHead = &mut (*service).devices;
        (*head).next = head;
        (*head).prev = head;
        
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}

pub extern "C" fn DevHostServiceDestruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }

    unsafe {
        // Get the address of service->devices (the list head)
        let devices_head: *mut crate::types::DListHead = &mut (*service).devices;
        
        // Calculate the offset of 'node' field within HdfDevice
        let node_offset = std::mem::offset_of!(crate::types::HdfDevice, node);
        
        // Get first entry: container_of(devices->next, HdfDevice, node)
        let mut current_node: *mut crate::types::DListHead = (*devices_head).next;
        
        while current_node != devices_head {
            // container_of: get HdfDevice* from node*
            let device: *mut crate::types::HdfDevice = 
                (current_node as *mut u8).sub(node_offset) as *mut crate::types::HdfDevice;
            
            // Save next before freeing
            let next_node: *mut crate::types::DListHead = (*current_node).next;
            
            crate::src_hdf_device::HdfDeviceFreeInstance(device);
            
            current_node = next_node;
        }
        
        crate::src_hdf_service_observer::HdfServiceObserverDestruct(&mut (*service).observer);
    }
}

pub extern "C" fn DevHostServiceCreate() -> *mut crate::types::HdfObject {
    let devHostService = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::DevHostService>() as u32)
    } as *mut crate::types::DevHostService;
    
    if !devHostService.is_null() {
        crate::src_devhost_service::DevHostServiceConstruct(devHostService);
    }
    
    devHostService as *mut crate::types::HdfObject
}

pub extern "C" fn DevHostServiceRelease(object: *mut crate::types::HdfObject) {
    let devHostService = object as *mut crate::types::DevHostService;
    if !devHostService.is_null() {
        crate::src_devhost_service::DevHostServiceDestruct(devHostService);
        unsafe {
            crate::compat::OsalMemFree(devHostService as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn DevHostServiceNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::IDevHostService {
    let hostService = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVHOST_SERVICE as i32) } as *mut crate::types::DevHostService;
    if !hostService.is_null() && !hostName.is_null() {
        unsafe {
            (*hostService).hostId = hostId;
            (*hostService).hostName = hostName;
        }
    }
    hostService as *mut crate::types::IDevHostService
}

pub extern "C" fn DevHostServiceFreeInstance(service: *mut crate::types::IDevHostService) {
    if !service.is_null() {
        unsafe {
            crate::compat::HdfObjectManagerFreeObject(&mut (*service).object as *mut crate::types::HdfObject);
        }
    }
}
