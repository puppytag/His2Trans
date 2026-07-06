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

/// container_of helper: given a pointer to HdfDeviceObject, returns pointer to HdfDeviceNode
fn container_of_dev_node(device_object: *mut crate::types::HdfDeviceObject) -> *mut crate::types::HdfDeviceNode {
    // SAFETY: offset is computed from field layout; pointer arithmetic is valid for the containing struct
    unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::HdfDeviceNode>()).deviceObject) as usize;
        (device_object as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    }
}

pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return -1;
    }

    let devNode = container_of_dev_node(deviceObject);

    let hostService = unsafe { (*devNode).hostService };
    if hostService.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return -1;
    }

    let devId = unsafe { (*devNode).devId };
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(
            core::ptr::addr_of_mut!((*hostService).observer),
            serviceName,
            devId,
            callback,
        ) as i32
    }
}

pub extern "C" fn HdfDeviceGetServiceName(deviceObject: *const crate::types::HdfDeviceObject) -> *const ::core::ffi::c_char {
    if deviceObject.is_null() {
        return ::core::ptr::null();
    }
    let dev_node = unsafe { container_of_dev_node(deviceObject as *mut _) };
    unsafe { (*dev_node).servName as *const ::core::ffi::c_char }
}

pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    if deviceObject.is_null() {
        return HDF_ERR_INVALID_PARAM as ::core::ffi::c_int;
    }
    let devNode = container_of_dev_node(deviceObject);
    crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener)
}

pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    if deviceObject.is_null() {
        return;
    }
    let devNode = container_of_dev_node(deviceObject);
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener);
}

pub extern "C" fn HdfPmAcquireDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        // HiLogPrint call omitted (missing compat symbol)
        return;
    }
    // container_of: get containing HdfDeviceNode from embedded deviceObject field
    let devNode = container_of_dev_node(deviceObject);
    if devNode.is_null() {
        return;
    }
    let tokenIf = unsafe { (*devNode).powerToken as *mut crate::types::IPowerStateToken };
    if !tokenIf.is_null() {
        unsafe {
            if let Some(acquire_wake_lock) = (*tokenIf).AcquireWakeLock {
                acquire_wake_lock(tokenIf);
            }
        }
    }
}

pub extern "C" fn HdfPmReleaseDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        return;
    }
    let devNode = container_of_dev_node(deviceObject);
    if devNode.is_null() {
        return;
    }
    let powerToken = unsafe { (*devNode).powerToken };
    let tokenIf = powerToken as *mut crate::types::IPowerStateToken;
    if !tokenIf.is_null() {
        if let Some(release_fn) = unsafe { (*tokenIf).ReleaseWakeLock } {
            unsafe { release_fn(tokenIf) };
        }
    }
}

pub extern "C" fn HdfPmAcquireDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        return;
    }
    let devnode_ptr = container_of_dev_node(deviceObject);
    if !devnode_ptr.is_null() {
        let power_token = unsafe { (*devnode_ptr).powerToken };
        unsafe {
            crate::src_hdf_power_manager::HdfPmTaskPut(
                power_token,
                crate::types::HDF_PM_REQUEST_ACQUIRE,
            );
        }
    }
}

pub extern "C" fn HdfPmReleaseDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        return;
    }

    let devNode = container_of_dev_node(deviceObject);

    if !devNode.is_null() {
        let powerToken = unsafe { (*devNode).powerToken };
        if !powerToken.is_null() {
            unsafe {
                crate::src_hdf_power_manager::HdfPmTaskPut(
                    powerToken,
                    crate::types::HDF_PM_REQUEST_RELEASE,
                );
            }
        }
    }
}

pub extern "C" fn HdfPmSetMode(deviceObject: *mut crate::types::HdfDeviceObject, mode: u32) {
    if deviceObject.is_null() || mode > crate::types::HDF_POWER_MODE_MAX as u32 {
        return;
    }

    let devNode = container_of_dev_node(deviceObject);
    let token = unsafe { (*devNode).powerToken };
    if !token.is_null() {
        let _ = unsafe { crate::src_power_state_token::PowerStateChange(token, mode) };
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
            pthread_rwlock_init(&mut (*deviceObject).mutex, std::ptr::null());
            (*deviceObject).property = std::ptr::null();
            (*deviceObject).service = std::ptr::null_mut();
            (*deviceObject).deviceClass = DEVICE_CLASS_DEFAULT;
        }
    }
}

pub extern "C" fn HdfDeviceObjectAlloc(parent: *mut crate::types::HdfDeviceObject, driverName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    // compute container_of offset (unsafe only for raw pointer deref in addr_of!)
    let offset = unsafe {
        core::ptr::addr_of!((*::core::ptr::null::<crate::types::HdfDeviceNode>()).deviceObject) as usize
    };
    // raw pointer arithmetic to obtain parent_dev_node
    let parent_dev_node = unsafe {
        (parent as *mut u8).offset(-(offset as isize)) as *mut crate::types::HdfDeviceNode
    };

    if parent.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return ::core::ptr::null_mut();
    }

    // read devStatus once
    let parent_status = unsafe { u32::from((*parent_dev_node).devStatus) };
    if parent_status != crate::types::DEVNODE_LAUNCHED {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent status invalid %{public}u\0".as_ptr() as *const ::core::ffi::c_char,
                parent_status as u32,
            );
        }
        return ::core::ptr::null_mut();
    }

    let new_node = unsafe {
        HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE.try_into().unwrap())
            as *mut crate::types::HdfDeviceNode
    };
    if new_node.is_null() {
        return ::core::ptr::null_mut();
    }

    let driver_name_copy = unsafe { HdfStringCopy(driverName) };
    unsafe { (*new_node).driverName = driver_name_copy; }
    if driver_name_copy.is_null() {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(new_node);
        return ::core::ptr::null_mut();
    }

    unsafe {
        (*new_node).hostService = (*parent_dev_node).hostService;
        (*new_node).device = (*parent_dev_node).device;
    }

    unsafe { core::ptr::addr_of_mut!((*new_node).deviceObject) }
}

pub extern "C" fn HdfDeviceObjectRelease(dev: *mut crate::types::HdfDeviceObject) {
    if dev.is_null() {
        return;
    }
    let devNode = container_of_dev_node(dev);
    unsafe {
        let device_ptr = (*devNode).device;
        if !device_ptr.is_null() {
            if let Some(detach_fn) = (*device_ptr).super_.Detach {
                let super_ptr: *mut crate::types::IHdfDevice = &mut (*device_ptr).super_;
                detach_fn(super_ptr, devNode);
            }
        }
    }
    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
}

pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::types::HDF_FAILURE;

    if dev.is_null() {
        eprintln!("failed to add device, param invalid");
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let devNode = container_of_dev_node(dev);

    let driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();

    if unsafe { (*devNode).driverName.is_null() }
        || unsafe { (*devNode).device.is_null() }
        || driverLoader.is_null()
        || unsafe { (*driverLoader).GetDriver.is_none() }
    {
        eprintln!("failed to add device, param invalid");
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let driver = unsafe {
        let get_driver_fn = (*driverLoader).GetDriver.unwrap();
        get_driver_fn((*devNode).driverName as *const ::core::ffi::c_char)
    };
    unsafe {
        (*devNode).driver = driver;
    }

    if driver.is_null() {
        let driver_name =
            unsafe { ::std::ffi::CStr::from_ptr((*devNode).driverName as *const ::core::ffi::c_char) };
        eprintln!("can not found driver {}", driver_name.to_string_lossy());
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    let device_ptr = unsafe { (*devNode).device };
    let super_ptr: *mut crate::types::IHdfDevice = device_ptr as *mut crate::types::IHdfDevice;
    let attach_fn = unsafe { (*super_ptr).Attach.unwrap() };
    ret = unsafe { attach_fn(super_ptr, devNode) };

    if ret != crate::types::HDF_SUCCESS {
        let driver_name =
            unsafe { ::std::ffi::CStr::from_ptr((*devNode).driverName as *const ::core::ffi::c_char) };
        eprintln!("failed to attach device {}", driver_name.to_string_lossy());
        return crate::types::HDF_DEV_ERR_ATTACHDEV_FAIL;
    }

    ret
}

pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    let dev_node = container_of_dev_node(dev);
    if dev_node.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let device = unsafe { (*dev_node).device };
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let detach = unsafe { (*device).super_.Detach.unwrap() };
    unsafe { detach(device as *mut crate::types::IHdfDevice, dev_node) }
}

pub extern "C" fn HdfDeviceObjectPublishService(dev: *mut crate::types::HdfDeviceObject, servName: *const ::core::ffi::c_char, policy: u8, perm: u32) -> ::core::ffi::c_int {
    if dev.is_null() || servName.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    if policy as u32 <= crate::types::SERVICE_POLICY_NONE as u32 || policy as u32 >= crate::types::SERVICE_POLICY_INVALID as u32 {
        return crate::types::HDF_DEV_ERR_NO_DEVICE_SERVICE;
    }

    let dev_node = container_of_dev_node(dev);

    if unsafe { (*dev_node).servStatus } {
        return crate::types::HDF_FAILURE;
    }

    // Duplicate servName string via HdfStringCopy
    let copy = unsafe { HdfStringCopy(servName) };
    if copy.is_null() {
        return crate::types::HDF_DEV_ERR_NO_MEMORY;
    }
    unsafe {
        (*dev_node).servName = copy;
        (*dev_node).policy = policy as u16;
        (*dev_node).permission = perm as u16;
    }

    let ret = crate::src_hdf_device_node::DeviceDriverBind(dev_node);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let publish = unsafe { (*dev_node).super_.PublishService };
    if let Some(func) = publish {
        unsafe { func(dev_node) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn HdfDeviceObjectRemoveService(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node = container_of_dev_node(dev);
    unsafe { ((*dev_node).super_.RemoveService.unwrap())(dev_node) }
}

pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || info.is_null() || unsafe { libc::strlen(info) > 128 } {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let dev_node = container_of_dev_node(dev);

    let serv_info_ptr = unsafe { (*dev_node).servInfo };
    if !serv_info_ptr.is_null() {
        unsafe {
            OsalMemFree(serv_info_ptr as *mut ::core::ffi::c_void);
        }
    }
    let new_serv = unsafe { HdfStringCopy(info) };
    unsafe {
        (*dev_node).servInfo = new_serv as *const ::core::ffi::c_char;
    }
    if new_serv.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dummy = unsafe { std::mem::MaybeUninit::<crate::types::HdfDeviceNode>::zeroed().assume_init() };
    let offset = (std::ptr::addr_of!(dummy.deviceObject) as usize) - (std::ptr::addr_of!(dummy) as usize);
    let devNode = unsafe { (dev as *mut u8).offset(-(offset as isize)) as *mut crate::types::HdfDeviceNode };
    let mut servInfo: crate::types::HdfServiceInfo = crate::types::HdfServiceInfo {
        servName: std::ptr::null(),
        servInfo: std::ptr::null(),
        devClass: 0,
        devId: 0,
        interfaceDesc: std::ptr::null(),
    };
    unsafe {
        servInfo.servName = (*devNode).servName;
        servInfo.servInfo = (*devNode).servInfo;
        servInfo.devClass = (*devNode).deviceObject.deviceClass as u16;
        servInfo.devId = (*devNode).devId;
        servInfo.interfaceDesc = (*devNode).interfaceDesc;
    }
    unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntUpdateService(
            std::ptr::addr_of_mut!((*devNode).deviceObject),
            &servInfo as *const crate::types::HdfServiceInfo,
        )
    }
}

pub extern "C" fn HdfDeviceObjectSetInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, interfaceDesc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || interfaceDesc.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let offset = unsafe {
        let null_node = std::ptr::null::<crate::types::HdfDeviceNode>();
        std::ptr::addr_of!((*null_node).deviceObject) as usize
    };
    let dev_node = unsafe { (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode };

    let len = unsafe { libc::strlen(interfaceDesc) };
    let copied = unsafe { libc::malloc((len + 1) as usize) as *mut ::core::ffi::c_char };
    if !copied.is_null() {
        unsafe {
            std::ptr::copy_nonoverlapping(interfaceDesc, copied, len as usize);
            *copied.add(len as usize) = 0;
        }
    }

    unsafe {
        (*dev_node).interfaceDesc = copied;
    }
    if copied.is_null() {
        crate::types::HDF_DEV_ERR_NO_DEVICE_SERVICE
    } else {
        crate::types::HDF_SUCCESS
    }
}
