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
    use crate::types::*;
    
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    // Container-of macro: get HdfDeviceNode from embedded deviceObject field
    // Use offset_of pattern without dereferencing null pointer
    let devnode_offset = core::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (deviceObject as *mut u8).sub(devnode_offset) as *mut HdfDeviceNode
    };
    
    let hostService: *mut DevHostService = unsafe { (*devNode).hostService };
    
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    let observer = unsafe { &mut (*hostService).observer as *mut HdfServiceObserver };
    let devId = unsafe { (*devNode).devId };
    
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(observer, serviceName, devId, callback)
    }
}

pub extern "C" fn HdfDeviceGetServiceName(deviceObject: *const crate::types::HdfDeviceObject) -> *const ::core::ffi::c_char {
    use crate::types::*;
    
    if deviceObject.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to get service name, deviceObject is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null();
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode
    // Using container_of pattern: devNode = (HdfDeviceNode*)((char*)deviceObject - offsetof(HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        ((deviceObject as *const u8).sub(offset)) as *const HdfDeviceNode
    };
    
    // Return the servName field from the HdfDeviceNode
    unsafe { (*devNode).servName as *const ::core::ffi::c_char }
}

pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    if deviceObject.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion:
    // Get the HdfDeviceNode from the embedded deviceObject field
    // devNode = (struct HdfDeviceNode *)((char *)deviceObject - offsetof(struct HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener)
}

pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    if deviceObject.is_null() {
        return;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion:
    // devNode = (struct HdfDeviceNode *)((char *)deviceObject - offsetof(struct HdfDeviceNode, deviceObject))
    // Calculate offset of deviceObject field within HdfDeviceNode
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener);
}

pub extern "C" fn HdfPmAcquireDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmAcquireDevice input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using memoffset-style approach
    unsafe {
        // Use a dummy aligned allocation to compute offset safely
        let dummy: std::mem::MaybeUninit<crate::types::HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as usize) - (base_ptr as usize);
        
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if devNode.is_null() {
            return;
        }
        
        let powerToken = (*devNode).powerToken;
        if powerToken.is_null() {
            return;
        }
        
        tokenIf = powerToken as *mut crate::types::IPowerStateToken;
        
        if !tokenIf.is_null() {
            if let Some(acquire_fn) = (*tokenIf).AcquireWakeLock {
                acquire_fn(tokenIf);
            }
        }
    }
}

pub extern "C" fn HdfPmReleaseDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmReleaseDevice input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion: get HdfDeviceNode from deviceObject field
    // Use offset_of pattern without dereferencing null pointer
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if !devNode.is_null() {
            let powerToken = (*devNode).powerToken;
            if !powerToken.is_null() {
                tokenIf = powerToken as *mut crate::types::IPowerStateToken;
                if !tokenIf.is_null() {
                    if let Some(release_fn) = (*tokenIf).ReleaseWakeLock {
                        release_fn(tokenIf);
                    }
                }
            }
        }
    }
}

pub extern "C" fn HdfPmAcquireDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();

    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmAcquireDeviceAsync\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    // HDF_SLIST_CONTAINER_OF macro expansion:
    // devNode = (struct HdfDeviceNode *)((char *)deviceObject - offsetof(struct HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };

    unsafe {
        crate::src_hdf_power_manager::HdfPmTaskPut(
            (*devNode).powerToken,
            crate::types::HDF_PM_REQUEST_ACQUIRE,
        );
    }
}

pub extern "C" fn HdfPmReleaseDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmReleaseDeviceAsync\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    // container_of macro: get HdfDeviceNode from embedded deviceObject field
    // Use offset_of pattern without null pointer dereference
    let offset = core::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode: *mut crate::types::HdfDeviceNode = 
        ((deviceObject as *mut u8).wrapping_sub(offset)) as *mut crate::types::HdfDeviceNode;

    unsafe {
        crate::src_hdf_power_manager::HdfPmTaskPut(
            (*devNode).powerToken,
            crate::types::HDF_PM_REQUEST_RELEASE,
        );
    }
}

pub extern "C" fn HdfPmSetMode(deviceObject: *mut crate::types::HdfDeviceObject, mode: u32) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut token: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() || mode > crate::types::HDF_POWER_MODE_MAX {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmSetMode\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    unsafe {
        // Calculate offset using a dummy aligned allocation approach
        // offset_of!(HdfDeviceNode, deviceObject)
        let dummy: std::mem::MaybeUninit<crate::types::HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as usize) - (base_ptr as usize);
        
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if devNode.is_null() {
            return;
        }
        
        token = (*devNode).powerToken;
        if token.is_null() {
            return;
        }
        
        (*token).mode = mode;
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
    use crate::types::*;
    use crate::compat::*;
    
    if parent.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    // Calculate offset using MaybeUninit to avoid null pointer dereference
    let offset = {
        let dummy = core::mem::MaybeUninit::<HdfDeviceNode>::uninit();
        let base_ptr = dummy.as_ptr();
        unsafe {
            (core::ptr::addr_of!((*base_ptr).deviceObject) as usize) - (base_ptr as usize)
        }
    };
    
    let parentDevNode = unsafe {
        (parent as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    if unsafe { (*parentDevNode).devStatus } != DEVNODE_LAUNCHED as u8 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent status invalid %u\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    let newNode = unsafe {
        HdfObjectManagerGetObject(HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut HdfDeviceNode
    };
    
    if newNode.is_null() {
        return std::ptr::null_mut();
    }
    
    let driver_name_copy = unsafe { HdfStringCopy(driverName) };
    if driver_name_copy.is_null() {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(newNode);
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*newNode).driverName = driver_name_copy;
        (*newNode).hostService = (*parentDevNode).hostService;
        (*newNode).device = (*parentDevNode).device;
        
        core::ptr::addr_of_mut!((*newNode).deviceObject)
    }
}

pub extern "C" fn HdfDeviceObjectRelease(dev: *mut crate::types::HdfDeviceObject) {
    if dev.is_null() {
        return;
    }

    // CONTAINER_OF macro: devNode = (HdfDeviceNode*)((char*)dev - offsetof(HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe { (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode };

    unsafe {
        let device = (*devNode).device;
        if !device.is_null() {
            let detach_fn = (*device).super_.Detach;
            if let Some(detach) = detach_fn {
                detach(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode);
            }
        }
    }

    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
}

pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    
    // CONTAINER_OF: compute offset using memoffset-style calculation
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    // Null checks
    if dev.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        if (*devNode).driverName.is_null() || (*devNode).device.is_null() || driverLoader.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        
        if (*driverLoader).GetDriver.is_none() {
            return HDF_ERR_INVALID_PARAM;
        }
        
        // devNode->driver = driverLoader->GetDriver(devNode->driverName)
        let get_driver_fn = (*driverLoader).GetDriver.unwrap();
        (*devNode).driver = get_driver_fn((*devNode).driverName);
        
        if (*devNode).driver.is_null() {
            return HDF_DEV_ERR_NO_DEVICE;
        }
        
        // ret = devNode->device->super.Attach(&devNode->device->super, devNode)
        let device_ptr = (*devNode).device;
        if let Some(attach_fn) = (*device_ptr).super_.Attach {
            ret = attach_fn(&mut (*device_ptr).super_ as *mut IHdfDevice, devNode);
        } else {
            return HDF_DEV_ERR_ATTACHDEV_FAIL;
        }
        
        if ret != HDF_SUCCESS {
            return HDF_DEV_ERR_ATTACHDEV_FAIL;
        }
    }
    
    ret
}

pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::{HdfDeviceNode, HdfDeviceObject, HDF_ERR_INVALID_OBJECT};
    
    // Calculate offset of deviceObject field within HdfDeviceNode using offset_of approach
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    if devNode.is_null() {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let device = unsafe { (*devNode).device };
    if device.is_null() {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let detach_fn = unsafe { (*device).super_.Detach };
    if let Some(detach) = detach_fn {
        unsafe {
            detach(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode)
        }
    } else {
        HDF_ERR_INVALID_OBJECT
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
    
    // CONTAINER_OF: devNode = (HdfDeviceNode*)((char*)dev - offsetof(HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    unsafe {
        if (*devNode).servStatus {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to publish public service, repeat publish\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_FAILURE;
        }
        
        (*devNode).servName = crate::compat::HdfStringCopy(servName);
        if (*devNode).servName.is_null() {
            return HDF_DEV_ERR_NO_MEMORY;
        }
        
        (*devNode).policy = policy as u16;
        (*devNode).permission = perm as u16;
        
        let ret = crate::src_hdf_device_node::DeviceDriverBind(devNode);
        if ret != HDF_SUCCESS {
            return ret;
        }
        
        if let Some(publish_fn) = (*devNode).super_.PublishService {
            publish_fn(devNode)
        } else {
            HDF_FAILURE
        }
    }
}

pub extern "C" fn HdfDeviceObjectRemoveService(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::{HdfDeviceNode, HdfDeviceObject, HDF_ERR_INVALID_PARAM};
    
    if dev.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF macro: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Calculate offset of deviceObject field within HdfDeviceNode
    let offset = unsafe {
        let dummy: *const HdfDeviceNode = std::ptr::null();
        &(*dummy).deviceObject as *const HdfDeviceObject as usize
    };
    
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    // Call devNode->super.RemoveService(devNode)
    unsafe {
        if let Some(remove_service) = (*devNode).super_.RemoveService {
            remove_service(devNode)
        } else {
            HDF_ERR_INVALID_PARAM
        }
    }
}

pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    use crate::types::*;
    
    const SERVICE_INFO_LEN_MAX: usize = 128;
    
    if dev.is_null() || info.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    let info_len = unsafe { libc::strlen(info) };
    if info_len > SERVICE_INFO_LEN_MAX {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Use offset_of pattern without dereferencing null pointer
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode = unsafe { (dev as *mut u8).sub(offset) as *mut HdfDeviceNode };
    
    unsafe {
        if !(*devNode).servInfo.is_null() {
            crate::compat::OsalMemFree((*devNode).servInfo as *mut ::core::ffi::c_void);
        }
        (*devNode).servInfo = crate::compat::HdfStringCopy(info);
        if (*devNode).servInfo.is_null() {
            return HDF_ERR_MALLOC_FAIL;
        }
    }
    
    HDF_SUCCESS
}

pub extern "C" fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF macro: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Use offset_of pattern without dereferencing null pointer
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    // Initialize HdfServiceInfo from devNode (inline HdfServiceInfoInit)
    let mut servInfo: crate::types::HdfServiceInfo = unsafe { std::mem::zeroed() };
    unsafe {
        servInfo.servName = (*devNode).servName as *const ::core::ffi::c_char;
        servInfo.servInfo = (*devNode).servInfo;
        servInfo.devClass = (*devNode).deviceObject.deviceClass as u16;
        servInfo.devId = (*devNode).devId;
        servInfo.interfaceDesc = (*devNode).interfaceDesc as *const ::core::ffi::c_char;
    }
    
    unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntUpdateService(
            &mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject,
            &servInfo as *const crate::types::HdfServiceInfo
        )
    }
}

pub extern "C" fn HdfDeviceObjectSetInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, interfaceDesc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    use crate::types::*;
    
    if dev.is_null() || interfaceDesc.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // Calculate offset of deviceObject within HdfDeviceNode using container_of pattern
    // devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    let offset = unsafe {
        let dummy: *const HdfDeviceNode = std::ptr::null();
        &(*dummy).deviceObject as *const _ as usize
    };
    
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    // Call HdfStringCopy to copy the interface description
    let copied = unsafe { crate::compat::HdfStringCopy(interfaceDesc) };
    
    unsafe {
        (*devNode).interfaceDesc = copied;
    }
    
    if copied.is_null() {
        HDF_ERR_MALLOC_FAIL
    } else {
        HDF_SUCCESS
    }
}

pub extern "C" fn HdfDeviceObjectCheckInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, data: *mut crate::types::HdfSBuf) -> bool {
    let _ = dev;
    let _ = data;
    true
}
