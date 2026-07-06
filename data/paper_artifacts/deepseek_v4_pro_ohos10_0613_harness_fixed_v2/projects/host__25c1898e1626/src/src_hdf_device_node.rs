//! Module: src_hdf_device_node
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

fn HdfDeviceNodePublishLocalService(devNode: *mut crate::types::HdfDeviceNode)-> i32 {
    if devNode.is_null() {
        // original: HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "device_node", "...")
        std::eprintln!("[device_node] failed to publish local service, device is null");
        return crate::types::HDF_FAILURE;
    }
    let hostService = unsafe { (*devNode).hostService };
    if hostService.is_null() {
        std::eprintln!("[device_node] failed to publish local service, host service is null");
        return crate::types::HDF_FAILURE;
    }
    let observer_ptr = unsafe { &mut (*hostService).observer as *mut crate::types::HdfServiceObserver };
    let svc_name = unsafe { (*devNode).servName as *const ::core::ffi::c_char };
    let dev_id = unsafe { (*devNode).devId };
    let policy = unsafe { (*devNode).policy };
    let service_ptr = unsafe { (*devNode).deviceObject.service as *mut crate::types::HdfObject };
    crate::src_hdf_service_observer::HdfServiceObserverPublishService(
        observer_ptr,
        svc_name,
        dev_id,
        policy,
        service_ptr,
    )
}

fn HdfDeviceNodePublishService(devNode: *mut crate::types::HdfDeviceNode)-> i32 {
    let mut status: i32 = crate::types::HDF_SUCCESS;
    let policy = unsafe { (*devNode).policy };
    let serv_name = unsafe { (*devNode).servName };
    if policy == crate::types::SERVICE_POLICY_NONE as u16
        || (!serv_name.is_null()
            && unsafe { ::core::ffi::CStr::from_ptr(serv_name as *const ::core::ffi::c_char) }
                .to_bytes()
                .len()
                == 0)
    {
        return status;
    }
    let node_if: *mut crate::types::IDeviceNode = unsafe { core::ptr::addr_of_mut!((*devNode).super_) };
    if policy == crate::types::SERVICE_POLICY_PUBLIC as u16
        || policy == crate::types::SERVICE_POLICY_CAPACITY as u16
    {
        if let Some(publish_service) = unsafe { (*node_if).PublishService } {
            status = unsafe { publish_service(devNode) };
        }
    }
    if status == crate::types::HDF_SUCCESS {
        status = crate::src_hdf_device_node::HdfDeviceNodePublishLocalService(devNode);
    }
    status
}

pub extern "C" fn DeviceDriverBind(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if devNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let driver = unsafe { (*devNode).driver };
    let driver_entry = unsafe { (*driver).entry };
    let policy = unsafe { (*devNode).policy };
    if policy == crate::types::SERVICE_POLICY_PUBLIC as u16
        || policy == crate::types::SERVICE_POLICY_CAPACITY as u16
    {
        if unsafe { (*driver_entry).Bind.is_none() } {
            // HiLogPrint omitted: bind method not implement
            unsafe {
                (*devNode).devStatus = crate::types::DEVNODE_NONE as u8;
            }
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let bind_fn = unsafe { (*driver_entry).Bind.unwrap() };
        let device_obj_ptr: *mut crate::types::HdfDeviceObject =
            unsafe { &mut (*devNode).deviceObject };
        let ret = unsafe { bind_fn(device_obj_ptr) };
        if ret != crate::types::HDF_SUCCESS {
            // HiLogPrint omitted: bind driver failed
            return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDeviceLaunchNode(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if devNode.is_null() {
        let tag = b"device_node\0".as_ptr() as *const ::core::ffi::c_char;
        let msg = b"failed to launch service, device or service is null\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag, msg); }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // Log launch
    {
        let tag = b"device_node\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt_launch = b"launch devnode %{public}s\0".as_ptr() as *const ::core::ffi::c_char;
        let servname = unsafe { (*devNode).servName };
        let servname_ptr: *const ::core::ffi::c_char = if !servname.is_null() {
            servname as *const _
        } else {
            b"\0".as_ptr() as *const _
        };
        unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD002510u32, tag, fmt_launch, servname_ptr); }
    }

    let driver = unsafe { (*devNode).driver };
    let driver_entry = unsafe { (*driver).entry };

    if driver_entry.is_null() || unsafe { (*driver_entry).Init.is_none() } {
        let tag = b"device_node\0".as_ptr() as *const ::core::ffi::c_char;
        let msg = b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag, msg); }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    unsafe {
        (*devNode).devStatus = crate::types::DEVNODE_LAUNCHED as u8;
    }

    let mut ret: ::core::ffi::c_int = crate::src_hdf_device_node::DeviceDriverBind(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let init_fn = unsafe { (*driver_entry).Init };
    if let Some(init_fn) = init_fn {
        let device_obj_ptr = unsafe { std::ptr::addr_of_mut!((*devNode).deviceObject) };
        ret = unsafe { init_fn(device_obj_ptr) };
    } else {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
    }

    ret = crate::src_hdf_device_node::HdfDeviceNodePublishService(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_PUBLISH_FAIL;
    }

    let token = unsafe { (*devNode).token };
    ret = crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDevice(token);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_ATTACHDEV_FAIL;
    }

    ret
}

pub extern "C" fn HdfDeviceNodeAddPowerStateListener(devNode: *mut crate::types::HdfDeviceNode, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    let power_token_null = unsafe { (*devNode).powerToken.is_null() };
    if !power_token_null {
        return crate::types::HDF_FAILURE;
    }
    let device_obj_ptr: *mut crate::types::HdfDeviceObject = unsafe { std::ptr::addr_of_mut!((*devNode).deviceObject) };
    let token: *mut crate::types::PowerStateToken = crate::src_power_state_token::PowerStateTokenNewInstance(device_obj_ptr, listener);
    unsafe { (*devNode).powerToken = token; }
    if token.is_null() {
        crate::types::HDF_FAILURE
    } else {
        crate::types::HDF_SUCCESS
    }
}

pub extern "C" fn HdfDeviceNodeRemovePowerStateListener(devNode: *mut crate::types::HdfDeviceNode, listener: *const crate::types::IPowerEventListener) {
    let _ = listener;
    if devNode.is_null() || unsafe { (*devNode).powerToken.is_null() } {
        return;
    }
    unsafe {
        crate::src_power_state_token::PowerStateTokenFreeInstance((*devNode).powerToken);
        (*devNode).powerToken = std::ptr::null_mut();
    }
}

pub extern "C" fn HdfDeviceNodePublishPublicService(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if devNode.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let service_null = unsafe { (*devNode).deviceObject.service.is_null() };
    if service_null {
        return crate::types::HDF_FAILURE;
    }
    let mut servInfo: crate::types::HdfServiceInfo = unsafe { ::core::mem::MaybeUninit::zeroed().assume_init() };
    unsafe {
        servInfo.servName = (*devNode).servName;
        servInfo.servInfo = (*devNode).servInfo;
    }
    servInfo.devClass = unsafe { (*devNode).deviceObject.deviceClass as u16 };
    servInfo.devId = unsafe { (*devNode).devId };
    servInfo.interfaceDesc = unsafe { (*devNode).interfaceDesc };
    let ret = unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntAddService(
            &mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject,
            &servInfo as *const crate::types::HdfServiceInfo,
        )
    };
    if ret == crate::types::HDF_SUCCESS {
        unsafe { (*devNode).servStatus = true; }
    }
    ret
}

pub extern "C" fn HdfDeviceNodeRemoveService(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if !devNode.is_null() && unsafe { (*devNode).servStatus } {
        unsafe {
            let svcName = (*devNode).servName;
            crate::src_devsvc_manager_clnt::DevSvcManagerClntRemoveService(svcName);
            (*devNode).servStatus = false;
        }
    }
    crate::types::HDF_SUCCESS
}

fn HdfDeviceUnlaunchNode(devNode: *mut crate::types::HdfDeviceNode) {
    // NULL check and status check (safe)
    if devNode.is_null() {
        return;
    }
    let dev_status = unsafe { (*devNode).devStatus as u32 };
    if dev_status != crate::types::DEVNODE_LAUNCHED {
        return;
    }

    // driverEntry = NULL initially
    let mut driver_entry: *const crate::types::HdfDriverEntry = std::ptr::null();

    // If driver exists, get its entry point
    let driver_ptr = unsafe { (*devNode).driver };
    if !driver_ptr.is_null() {
        let driver = unsafe { &*driver_ptr };
        driver_entry = driver.entry;
    }

    // If driver entry has a Release callback, lock, call it, and cleanup
    if !driver_entry.is_null() && unsafe { (*driver_entry).Release.is_some() } {
        // SAFETY: rwlock is a valid, initialized pthread_rwlock_t.
        let rwlock: *mut crate::types::pthread_rwlock_t =
            unsafe { &mut (*devNode).deviceObject.mutex };
        let _guard = unsafe { crate::compat::RwlockGuard::new(rwlock) };
        let release_fn = unsafe { (*driver_entry).Release.unwrap() };
        let device_obj_ptr: *mut crate::types::HdfDeviceObject =
            unsafe { &mut (*devNode).deviceObject };
        unsafe {
            release_fn(device_obj_ptr);
        }
        unsafe {
            (*devNode).deviceObject.service = std::ptr::null_mut();
        }
        // RwlockGuard drops here, releasing the lock.
    }

    // If local service was published, remove it
    let serv_status = unsafe { (*devNode).servStatus };
    if serv_status {
        let remove_service = unsafe { (*devNode).super_.RemoveService };
        if let Some(rm_fn) = remove_service {
            unsafe { rm_fn(devNode); }
        }
    }

    // Detach device from device manager
    unsafe {
        crate::src_devmgr_service_clnt::DevmgrServiceClntDetachDevice((*devNode).devId);
    }

    // Reclaim driver via driver loader
    let driver_loader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    if !driver_loader.is_null() {
        let reclaim_fn = unsafe { (*driver_loader).ReclaimDriver };
        if let Some(f) = reclaim_fn {
            unsafe {
                f((*devNode).driver);
            }
        }
        unsafe { (*devNode).driver = std::ptr::null_mut(); }
    }

    // Mark the node as "initialized"
    unsafe {
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
}

pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if devNode.is_null() {
        return;
    }
    // Safe operation: construct device object
    let device_obj = unsafe { &mut (*devNode).deviceObject };
    crate::src_hdf_device_object::HdfDeviceObjectConstruct(device_obj);
    // Obtain token
    let token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
    unsafe { (*devNode).token = token; }

    // Assign vtable entries (transmutes must remain unsafe)
    unsafe {
        let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
        (*nodeIf).LaunchNode = Some(crate::src_hdf_device_node::HdfDeviceLaunchNode);
        (*nodeIf).PublishService = Some(crate::src_hdf_device_node::HdfDeviceNodePublishPublicService);
        (*nodeIf).RemoveService = Some(crate::src_hdf_device_node::HdfDeviceNodeRemoveService);
        (*nodeIf).UnlaunchNode = Some(::core::mem::transmute::<
            fn(*mut crate::types::HdfDeviceNode),
            unsafe extern "C" fn(*mut crate::types::HdfDeviceNode)
        >(crate::src_hdf_device_node::HdfDeviceUnlaunchNode));
    }
}

pub extern "C" fn HdfDeviceNodeDestruct(devNode: *mut crate::types::HdfDeviceNode) {
    if devNode.is_null() {
        return;
    }
    let dev_status = unsafe { (*devNode).devStatus };
    if u32::from(dev_status) == crate::types::DEVNODE_LAUNCHED {
        crate::src_hdf_device_node::HdfDeviceUnlaunchNode(devNode);
    }
    if u32::from(dev_status) == crate::types::DEVNODE_LAUNCHED || u32::from(dev_status) == crate::types::DEVNODE_INITED {
        let devNode_ref = unsafe { &mut *devNode };
        unsafe {
            crate::src_hdf_device_token::HdfDeviceTokenFreeInstance(devNode_ref.token);
            crate::src_power_state_token::PowerStateTokenFreeInstance(devNode_ref.powerToken);
            libc::free(devNode_ref.servName as *mut ::core::ffi::c_void);
            libc::free(devNode_ref.servInfo as *mut ::core::ffi::c_void);
            libc::free(devNode_ref.driverName as *mut ::core::ffi::c_void);
        }
        devNode_ref.token = std::ptr::null_mut();
        devNode_ref.powerToken = std::ptr::null_mut();
        devNode_ref.servName = std::ptr::null_mut();
        devNode_ref.servInfo = std::ptr::null();
    }
}

pub extern "C" fn HdfDeviceNodeNewInstance(deviceInfo: *const crate::types::HdfDeviceInfo, driver: *mut crate::types::HdfDriver) -> *mut crate::types::HdfDeviceNode {
    if deviceInfo.is_null() {
        return std::ptr::null_mut();
    }

    let devNode = unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE as i32)
            as *mut crate::types::HdfDeviceNode
    };
    if devNode.is_null() {
        return std::ptr::null_mut();
    }

    // Fetch fields from deviceInfo once
    let (dev_id, permission, policy, svc_name, dev_name, device_match_attr) = unsafe {
        (
            (*deviceInfo).deviceId,
            (*deviceInfo).permission,
            (*deviceInfo).policy,
            (*deviceInfo).svcName,
            (*deviceInfo).deviceName,
            (*deviceInfo).deviceMatchAttr,
        )
    };

    // Assign driver, devId, permission, policy, token devid
    unsafe {
        (*devNode).driver = driver;
        (*devNode).devId = dev_id;
        (*devNode).permission = permission;
        (*devNode).policy = policy;
        (*(*devNode).token).devid = dev_id;
    }

    // Copy servName, deviceName, token servName
    let serv_name_copy = unsafe { crate::compat::HdfStringCopy(svc_name) };
    if serv_name_copy.is_null() {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        return std::ptr::null_mut();
    }
    unsafe { (*devNode).servName = serv_name_copy; }

    let dev_name_copy = unsafe { crate::compat::HdfStringCopy(dev_name) };
    unsafe { (*(*devNode).token).deviceName = dev_name_copy as *const ::core::ffi::c_char; }

    let token_serv_name_copy = unsafe { crate::compat::HdfStringCopy(svc_name) };
    unsafe { (*(*devNode).token).servName = token_serv_name_copy as *const ::core::ffi::c_char; }

    // Property and devStatus
    unsafe {
        (*devNode).deviceObject.property = crate::compat::HcsGetNodeByMatchAttr(
            crate::compat::HdfGetHcsRootNode(),
            device_match_attr,
        );
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }

    devNode
}

pub extern "C" fn HdfDeviceNodeFreeInstance(devNode: *mut crate::types::HdfDeviceNode) {
    unsafe {
        HdfObjectManagerFreeObject(devNode as *mut crate::types::HdfObject);
    }
}
