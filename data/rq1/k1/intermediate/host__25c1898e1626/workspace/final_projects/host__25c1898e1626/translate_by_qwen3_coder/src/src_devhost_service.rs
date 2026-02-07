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
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    if hostService.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devhost_service\0".as_ptr() as *const _,
            "failed to find driver, hostService is null\0".as_ptr() as *const _,
        ) };
        return std::ptr::null_mut();
    }
    unsafe {
        let devices_ptr = &(*hostService).devices as *const crate::types::DListHead;
        let mut node_ptr = (*devices_ptr).next;
        while node_ptr != devices_ptr as *mut crate::types::DListHead {
            device = (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice;
            if ((((*device).deviceId >> 8) & ((1 << 16) - 1)) as u16) == deviceId {
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
            let node = &mut (*device).node;
            (*node).prev.as_mut().unwrap().next = (*node).next;
            (*node).next.as_mut().unwrap().prev = (*node).prev;
            (*node).prev = std::ptr::null_mut();
            (*node).next = std::ptr::null_mut();
        }
        crate::src_hdf_device::HdfDeviceFreeInstance(device);
    }
}

fn DevHostServiceQueryOrAddDevice(inst: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    let device = crate::src_devhost_service::DevHostServiceFindDevice(inst, deviceId);
    if device.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"%{public}s can't find device, try to create\0".as_ptr() as *const _,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const _,
            );
        }
        let device = crate::src_hdf_device::HdfDeviceNewInstance();
        if device.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"devhost_service\0".as_ptr() as *const _,
                    b"Dev host service failed to create driver instance\0".as_ptr() as *const _,
                );
            }
            return std::ptr::null_mut();
        }
        unsafe {
            (*device).deviceId = (((*inst).hostId as u32) << (16 + 8)) | ((deviceId as u32) << 8) | (0);
            let entry = &mut (*device).node as *mut crate::types::DListHead;
            let head = &mut (*inst).devices as *mut crate::types::DListHead;
            (*entry).next = (*head).next;
            (*entry).prev = head;
            (*(*head).next).prev = entry;
            (*head).next = entry;
        }
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"%{public}s add device complete\0".as_ptr() as *const _,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const _,
            );
        }
        device
    } else {
        device
    }
}

pub extern "C" fn DevHostServiceAddDevice(inst: *mut crate::types::IDevHostService, deviceInfo: *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int {
    let mut ret = crate::types::HDF_FAILURE;
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut driver: *mut crate::types::HdfDriver = std::ptr::null_mut();
    let hostService = if !inst.is_null() {
        unsafe { (inst as *mut u8).offset(-(std::mem::offset_of!(crate::types::DevHostService, super_) as isize)) as *mut crate::types::DevHostService }
    } else {
        std::ptr::null_mut()
    };
    let driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();

    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }
    let get_driver = unsafe { (*driverLoader).GetDriver };
    if get_driver.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }

    let device_id = unsafe { ((*deviceInfo).deviceId >> 8) & 0xFFFF };
    device = crate::src_devhost_service::DevHostServiceQueryOrAddDevice(hostService, device_id as u16);
    if device.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device or Attach func is null\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    let attach = unsafe { (*device).super_.Attach };
    if attach.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device or Attach func is null\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    let get_device_node = unsafe { (*device).super_.GetDeviceNode };
    if let Some(f) = get_device_node {
        devNode = unsafe { f(&mut (*device).super_ as *mut crate::types::IHdfDevice, unsafe { (*deviceInfo).deviceId }) };
    }
    if !devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device already exist\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_DEVICE_BUSY;
    }
    let module_name = unsafe { (*deviceInfo).moduleName };
    if let Some(f) = get_driver {
        driver = unsafe { f(module_name) };
    }
    if driver.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device %{public}s, get driver failed\0".as_ptr() as *const _, module_name) };
        ret = crate::types::HDF_DEV_ERR_NODATA;
        unsafe {
            if !device.is_null() {
                let dev_nodes = &(*device).devNodes;
                let is_empty = (*dev_nodes).next == dev_nodes as *const _ as *mut _;
                if is_empty {
                    crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
                }
            }
            return ret;
        }
    }

    devNode = crate::src_hdf_device_node::HdfDeviceNodeNewInstance(deviceInfo, driver);
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, create devNode failed\0".as_ptr() as *const _) };
        let reclaim_driver = unsafe { (*driverLoader).ReclaimDriver };
        if let Some(f) = reclaim_driver {
            unsafe { f(driver) };
        }
        return crate::types::HDF_DEV_ERR_NO_MEMORY;
    }

    unsafe {
        (*devNode).hostService = hostService;
        (*devNode).device = device;
        (*devNode).driver = driver;
    }
    if let Some(f) = attach {
        ret = unsafe { f(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode) };
    }
    if ret != crate::types::HDF_SUCCESS {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, attach devNode failed\0".as_ptr() as *const _) };
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        unsafe {
            if !device.is_null() {
                let dev_nodes = &(*device).devNodes;
                let is_empty = (*dev_nodes).next == dev_nodes as *const _ as *mut _;
                if is_empty {
                    crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
                }
            }
            return ret;
        }
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_DEBUG, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"%{public}s add device success\0".as_ptr() as *const _, b"DevHostServiceAddDevice\0".as_ptr() as *const _) };
    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn DevHostServiceDelDevice(inst: *mut crate::types::IDevHostService, devId: crate::types::devid_t) -> ::core::ffi::c_int {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let hostService = inst as *mut crate::types::DevHostService;
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();

    let deviceId = ((devId >> 8) & ((1 << 16) - 1)) as u16;
    device = crate::src_devhost_service::DevHostServiceFindDevice(hostService, deviceId);
    if device.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_WARN,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const _,
            b"failed to del device, device is not exist\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_SUCCESS;
    }

    unsafe {
        if let Some(get_device_node) = (*device).super_.GetDeviceNode {
            devNode = get_device_node(&mut (*device).super_ as *mut crate::types::IHdfDevice, devId);
        }
    }
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const _,
            b"failed to del device, not exist\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    unsafe {
        if (*device).super_.Detach.is_none() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to del device, invalid device\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
    }

    unsafe {
        if let Some(detach) = (*device).super_.Detach {
            if detach(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode) != crate::types::HDF_SUCCESS {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"devhost_service\0".as_ptr() as *const _,
                    b"failed to detach device\0".as_ptr() as *const _,
                );
                return crate::types::HDF_FAILURE;
            }
        }
    }
    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);

    unsafe {
        let head = &(*device).devNodes;
        if (head.next == head as *const crate::types::DListHead as *mut crate::types::DListHead) {
            crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
        }
    }
    let _ = unsafe { crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_DEBUG,
        0xD002510,
        b"devhost_service\0".as_ptr() as *const _,
        b"%{public}s add device success\0".as_ptr() as *const _,
    ) };
    return crate::types::HDF_SUCCESS;
}

fn DevHostServiceStartService(service: *mut crate::types::IDevHostService) -> i32 {
    let hostService = service as *mut crate::types::DevHostService;
    if hostService.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to start device service, hostService is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let hostId = unsafe { (*hostService).hostId };
    crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDeviceHost(hostId, service)
}

fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32) -> i32 {
    let mut device_node: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut ret: i32;
    const POWER_STATE_DOZE_RESUME: u32 = 0;
    const POWER_STATE_RESUME: u32 = 0;
    let is_power_wake_state = state == POWER_STATE_DOZE_RESUME || state == POWER_STATE_RESUME;
    if is_power_wake_state {
        unsafe {
            let mut entry_ptr = (*device).devNodes.next;
            while entry_ptr != &mut (*device).devNodes as *mut crate::types::DListHead {
                device_node = (entry_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
                if !(*device_node).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*device_node).powerToken, state);
                    if ret != crate::types::HDF_SUCCESS {
                        let _ = crate::compat::HiLogPrint(
                            crate::types::LOG_CORE,
                            crate::types::LOG_ERROR,
                            0xD002510,
                            "devhost_service\0".as_ptr() as *const i8,
                            "device %{public}s failed to resume(%{public}u)\0".as_ptr() as *const i8,
                            if !(*device_node).driver.is_null() && !(*(*device_node).driver).entry.is_null() {
                                (*(*(*device_node).driver).entry).moduleName
                            } else {
                                std::ptr::null()
                            },
                            state,
                        );
                    }
                }
                entry_ptr = (*entry_ptr).next;
            }
        }
    } else {
        unsafe {
            let mut entry_ptr = (*device).devNodes.prev;
            while entry_ptr != &mut (*device).devNodes as *mut crate::types::DListHead {
                device_node = (entry_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
                if !(*device_node).powerToken.is_null() {
                    ret = crate::src_power_state_token::PowerStateChange((*device_node).powerToken, state);
                    if ret != crate::types::HDF_SUCCESS {
                        let _ = crate::compat::HiLogPrint(
                            crate::types::LOG_CORE,
                            crate::types::LOG_ERROR,
                            0xD002510,
                            "devhost_service\0".as_ptr() as *const i8,
                            "device %{public}s failed to suspend(%{public}u)\0".as_ptr() as *const i8,
                            if !(*device_node).driver.is_null() && !(*(*device_node).driver).entry.is_null() {
                                (*(*(*device_node).driver).entry).moduleName
                            } else {
                                std::ptr::null()
                            },
                            state,
                        );
                    }
                }
                entry_ptr = (*entry_ptr).prev;
            }
        }
    }
    crate::types::HDF_SUCCESS
}

fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32) -> i32 {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut ret: i32 = crate::types::HDF_SUCCESS;
    let host_service_ptr = if !service.is_null() {
        unsafe {
            (service as *mut u8).offset(-(std::mem::offset_of!(crate::types::DevHostService, super_) as isize))
                as *mut crate::types::DevHostService
        }
    } else {
        std::ptr::null_mut()
    };
    if host_service_ptr.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to start device service, hostService is null\0".as_ptr() as *const _,
            )
        };
        return crate::types::HDF_FAILURE;
    }
    let host_service = unsafe { &*host_service_ptr };
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const _,
            b"host(%{public}s) set power state=%{public}u\0".as_ptr() as *const _,
            host_service.hostName,
            state,
        )
    };
    let is_wake = state == crate::types::POWER_STATE_DOZE_RESUME || state == crate::types::POWER_STATE_RESUME;
    if is_wake {
        let mut node_ptr = unsafe { (*host_service_ptr).devices.prev };
        while node_ptr != &(unsafe { (*host_service_ptr).devices }) as *const crate::types::DListHead as *mut crate::types::DListHead {
            device = unsafe {
                (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node_ptr = unsafe { (*device).node.prev };
        }
    } else {
        let mut node_ptr = unsafe { (*host_service_ptr).devices.next };
        while node_ptr != &(unsafe { (*host_service_ptr).devices }) as *const crate::types::DListHead as *mut crate::types::DListHead {
            device = unsafe {
                (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node_ptr = unsafe { (*device).node.next };
        }
    }
    ret
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_devhost_service_9
// c_function: DevHostServiceConstruct
// rust_file: src_devhost_service.rs
// rust_signature: pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService)
// c_first_line: void DevHostServiceConstruct(struct DevHostService *service)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_devhost_service_9/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_devhost_service.rs:192:46
//       |
//       |                                         ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected "C" fn, found "Rust" fn
//       |                                         |
//       |                                         arguments to this enum variant are incorrect
//       |
//   help: the type constructed contains `fn(*mut IDevHostService) -> i32 {DevHostServiceStartService}` due to the type of the argument passed
// =================================
pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_devhost_service::DevHostServiceConstruct(service as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_devhost_service_9
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_devhost_service_9/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf = &mut (*service).super_ as *mut crate::types::IDevHostService;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(crate::src_devhost_service::DevHostServiceStartService);
        (*hostServiceIf).PmNotify = Some(crate::src_devhost_service::DevHostServicePmNotify);
        let head = &mut (*service).devices;
        (*head).next = head as *mut crate::types::DListHead;
        (*head).prev = head as *mut crate::types::DListHead;
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_devhost_service_9
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn DevHostServiceDestruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let mut device = ((*service).devices.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice;
        while &(*device).node as *const crate::types::DListHead != &(*service).devices as *const crate::types::DListHead {
            let tmp = ((*device).node.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice;
            crate::src_hdf_device::HdfDeviceFreeInstance(device);
            device = tmp;
        }
        crate::src_hdf_service_observer::HdfServiceObserverDestruct(&mut (*service).observer as *mut crate::types::HdfServiceObserver);
    }
}

pub extern "C" fn DevHostServiceCreate() -> *mut crate::types::HdfObject {
    let devHostService = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::DevHostService>()) } as *mut crate::types::DevHostService;
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
            libc::free(devHostService as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn DevHostServiceNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::IDevHostService {
    let hostService = unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVHOST_SERVICE as i32)
            as *mut crate::types::DevHostService
    };
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
