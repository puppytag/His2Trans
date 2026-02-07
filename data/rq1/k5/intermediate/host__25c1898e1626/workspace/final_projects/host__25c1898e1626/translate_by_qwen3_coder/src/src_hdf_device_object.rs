//! Module: src_hdf_device_object
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

pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    let mut hostService: *mut crate::types::DevHostService = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "device_object\0".as_ptr() as *const ::core::ffi::c_char,
                "failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        devNode = (deviceObject as *mut ::core::ffi::c_char).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
        hostService = (*devNode).hostService;
    }
    if hostService.is_null() {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        let devId = (*devNode).devId;
        return crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(
            &mut (*hostService).observer as *mut crate::types::HdfServiceObserver,
            serviceName,
            devId,
            callback,
        );
    }
}

pub extern "C" fn HdfDeviceGetServiceName(deviceObject: *const crate::types::HdfDeviceObject) -> *const ::core::ffi::c_char {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to get service name, deviceObject is invalid\0".as_ptr() as *const _) };
        return std::ptr::null();
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *const u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
        if !devNode.is_null() {
            return (*devNode).servName;
        }
    }
    std::ptr::null()
}

pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        devNode = (__mptr as *const u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
    }
    return crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener);
}

pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *const u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener);
}

pub extern "C" fn HdfPmAcquireDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    if deviceObject.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const _,
                b"HdfPmAcquireDevice input param is invalid\0".as_ptr() as *const _,
            )
        };
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *mut u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    if !devNode.is_null() {
        unsafe {
            tokenIf = (*devNode).powerToken as *mut crate::types::IPowerStateToken;
        }
    }
    if !tokenIf.is_null() {
        unsafe {
            if let Some(f) = (*tokenIf).AcquireWakeLock {
                f(tokenIf);
            }
        }
    }
}

pub extern "C" fn HdfPmReleaseDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    if deviceObject.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"HdfPmReleaseDevice input param is invalid\0".as_ptr() as *const _) };
        return;
    }
    unsafe {
        devNode = (deviceObject as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
        tokenIf = (*devNode).powerToken as *mut crate::types::IPowerStateToken;
        if !tokenIf.is_null() && (*tokenIf).ReleaseWakeLock.is_some() {
            if let Some(f) = (*tokenIf).ReleaseWakeLock {
                f(tokenIf);
            }
        }
    }
}

pub extern "C" fn HdfPmAcquireDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();

    if deviceObject.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_object\0".as_ptr() as *const i8,
                b"%{public}s: input param is invalid\0".as_ptr() as *const i8,
                b"HdfPmAcquireDeviceAsync\0".as_ptr() as *const i8,
            )
        };
        return;
    }

    unsafe {
        devNode = (deviceObject as *mut u8).offset(
            -(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)
        ) as *mut crate::types::HdfDeviceNode;
    }

    if !devNode.is_null() {
        let powerToken = unsafe { (*devNode).powerToken };
        crate::src_hdf_power_manager::HdfPmTaskPut(powerToken, crate::types::HDF_PM_REQUEST_ACQUIRE);
    }
}

pub extern "C" fn HdfPmReleaseDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();

    if deviceObject.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"%{public}s: input param is invalid\0".as_ptr() as *const _, b"HdfPmReleaseDeviceAsync\0".as_ptr() as *const _) };
        return;
    }

    unsafe {
        devNode = (deviceObject as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
    }

    if !devNode.is_null() {
        let powerToken = unsafe { (*devNode).powerToken };
        if !powerToken.is_null() {
            crate::src_hdf_power_manager::HdfPmTaskPut(powerToken, crate::types::HDF_PM_REQUEST_RELEASE);
        }
    }
}

pub extern "C" fn HdfPmSetMode(deviceObject: *mut crate::types::HdfDeviceObject, mode: u32) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut token: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    if deviceObject.is_null() || mode > crate::types::HDF_POWER_MODE_MAX {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"%{public}s: input param is invalid\0".as_ptr() as *const _, b"HdfPmSetMode\0".as_ptr() as *const _) };
        return;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (__mptr as *const u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    if !devNode.is_null() {
        unsafe {
            token = (*devNode).powerToken;
        }
    }
    if !token.is_null() {
        unsafe {
            (*token).mode = mode;
        }
    }
}

pub extern "C" fn HdfDeviceSetClass(deviceObject: *mut crate::types::HdfDeviceObject, deviceClass: crate::types::DeviceClass) -> bool {
    if deviceObject.is_null() || deviceClass >= crate::types::DEVICE_CLASS_MAX {
        return false;
    }
    unsafe {
        (*deviceObject).deviceClass = deviceClass;
    }
    true
}

pub extern "C" fn HdfDeviceObjectConstruct(deviceObject: *mut crate::types::HdfDeviceObject) {
    if !deviceObject.is_null() {
        unsafe {
            (*deviceObject).property = std::ptr::null();
            (*deviceObject).service = std::ptr::null_mut();
            (*deviceObject).deviceClass = crate::types::DEVICE_CLASS_DEFAULT;
        }
    }
}

pub extern "C" fn HdfDeviceObjectAlloc(parent: *mut crate::types::HdfDeviceObject, driverName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    let mut newNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let parentDevNode: *mut crate::types::HdfDeviceNode = if !parent.is_null() {
        unsafe {
            (parent as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode
        }
    } else {
        std::ptr::null_mut()
    };
    if parent.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to alloc device, parent invalid\0".as_ptr() as *const _) };
        return std::ptr::null_mut();
    }
    unsafe {
        if (*parentDevNode).devStatus as u32 != crate::types::DEVNODE_LAUNCHED {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to alloc device, parent status invalid %{public}u\0".as_ptr() as *const _, (*parentDevNode).devStatus as u32);
            return std::ptr::null_mut();
        }
    }
    newNode = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut crate::types::HdfDeviceNode };
    if newNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*newNode).driverName = crate::compat::HdfStringCopy(driverName);
        if (*newNode).driverName.is_null() {
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(newNode);
            return std::ptr::null_mut();
        }
        (*newNode).hostService = (*parentDevNode).hostService;
        (*newNode).device = (*parentDevNode).device;
        &mut (*newNode).deviceObject
    }
}

pub extern "C" fn HdfDeviceObjectRelease(dev: *mut crate::types::HdfDeviceObject) {
    if dev.is_null() {
        return;
    }
    let dev_node = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    unsafe {
        if !(*dev_node).device.is_null() {
            let device = (*dev_node).device;
            if let Some(detach) = (*device).super_.Detach {
                detach(device as *mut crate::types::IHdfDevice, dev_node);
            }
        }
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(dev_node);
    }
}

pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    let mut ret = crate::types::HDF_FAILURE;
    if dev.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"device_object\0".as_ptr() as *const _,
            b"failed to add device, param invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode };
    let driver_loader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    if unsafe { (*dev_node).driverName.is_null() || (*dev_node).device.is_null() } || driver_loader.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"device_object\0".as_ptr() as *const _,
            b"failed to add device, param invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let get_driver = unsafe { (*driver_loader).GetDriver };
    if get_driver.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"device_object\0".as_ptr() as *const _,
            b"failed to add device, param invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        (*dev_node).driver = get_driver.unwrap()((*dev_node).driverName);
    }
    if unsafe { (*dev_node).driver.is_null() } {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"device_object\0".as_ptr() as *const _,
            b"can not found driver %{public}s\0".as_ptr() as *const _,
            (*dev_node).driverName,
        ) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    let device = unsafe { (*dev_node).device };
    if !device.is_null() {
        let attach = unsafe { (*device).super_.Attach };
        if let Some(f) = attach {
            ret = unsafe { f(device as *mut crate::types::IHdfDevice, dev_node) };
        }
    }
    if ret != crate::types::HDF_SUCCESS {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"device_object\0".as_ptr() as *const _,
            b"failed to attach device %{public}s\0".as_ptr() as *const _,
            (*dev_node).driverName,
        ) };
        return crate::types::HDF_DEV_ERR_ATTACHDEV_FAIL;
    }
    ret
}

pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_node = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    if dev_node.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let device = unsafe { (*dev_node).device };
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let detach_fn = unsafe { (*device).super_.Detach };
    if let Some(detach) = detach_fn {
        unsafe { detach(device as *mut crate::types::IHdfDevice, dev_node) }
    } else {
        crate::types::HDF_ERR_INVALID_OBJECT
    }
}

pub extern "C" fn HdfDeviceObjectPublishService(dev: *mut crate::types::HdfDeviceObject, servName: *const ::core::ffi::c_char, policy: u8, perm: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    if dev.is_null() || servName.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    if policy <= SERVICE_POLICY_NONE as u8 || policy >= SERVICE_POLICY_INVALID as u8 {
        return HDF_DEV_ERR_NO_DEVICE_SERVICE;
    }
    let dev_node_ptr = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(HdfDeviceNode, deviceObject) as isize)) as *mut HdfDeviceNode };
    let dev_node = unsafe { &mut *dev_node_ptr };
    if dev_node.servStatus {
        unsafe {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "device_object\0".as_ptr() as *const i8, "failed to publish public service, repeat publish\0".as_ptr() as *const i8);
        }
        return HDF_FAILURE;
    }
    unsafe {
        dev_node.servName = HdfStringCopy(servName);
    }
    if dev_node.servName.is_null() {
        return HDF_DEV_ERR_NO_MEMORY;
    }
    dev_node.policy = policy as u16;
    dev_node.permission = perm as u16;
    let ret = crate::src_hdf_device_node::DeviceDriverBind(dev_node_ptr);
    if ret != HDF_SUCCESS {
        return ret;
    }
    if let Some(f) = dev_node.super_.PublishService {
        unsafe { f(dev_node_ptr) }
    } else {
        HDF_FAILURE
    }
}

pub extern "C" fn HdfDeviceObjectRemoveService(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    let cb = unsafe { (*dev_node).super_.RemoveService };
    if let Some(f) = cb {
        unsafe { f(dev_node) }
    } else {
        0
    }
}

pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || info.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let len = unsafe { libc::strlen(info) };
    if len > 128 {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    unsafe {
        if !(*dev_node).servInfo.is_null() {
            OsalMemFree((*dev_node).servInfo as *mut ::core::ffi::c_void);
        }
        (*dev_node).servInfo = HdfStringCopy(info);
        if (*dev_node).servInfo.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node_ptr = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    let mut serv_info = crate::types::HdfServiceInfo {
        servName: std::ptr::null(),
        servInfo: std::ptr::null(),
        devClass: 0,
        devId: 0,
        interfaceDesc: std::ptr::null(),
    };
    unsafe {
        serv_info.servName = (*dev_node_ptr).servName;
        serv_info.servInfo = (*dev_node_ptr).servInfo;
        serv_info.devClass = (*dev_node_ptr).deviceObject.deviceClass as u16;
        serv_info.devId = (*dev_node_ptr).devId;
        serv_info.interfaceDesc = (*dev_node_ptr).interfaceDesc;
    }
    unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntUpdateService(&mut (*dev_node_ptr).deviceObject, &serv_info)
    }
}

pub extern "C" fn HdfDeviceObjectSetInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, interfaceDesc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || interfaceDesc.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode };
    let copy = unsafe { crate::compat::HdfStringCopy(interfaceDesc) };
    unsafe {
        (*dev_node).interfaceDesc = copy;
    }
    if copy.is_null() {
        crate::types::HDF_ERR_MALLOC_FAIL
    } else {
        crate::types::HDF_SUCCESS
    }
}

pub extern "C" fn HdfDeviceObjectCheckInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, data: *mut crate::types::HdfSBuf) -> bool {
    let _ = dev;
    let _ = data;
    true
}
