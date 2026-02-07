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

fn HdfDeviceNodePublishLocalService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, device is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    let host_service = unsafe { (*devNode).hostService };
    if host_service.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, host service is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverPublishService(
            &mut (*host_service).observer,
            (*devNode).servName,
            (*devNode).devId,
            (*devNode).policy,
            (*devNode).deviceObject.service as *mut crate::types::HdfObject,
        )
    }
}

fn HdfDeviceNodePublishService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut status = crate::types::HDF_SUCCESS;
    if devNode.is_null() {
        return status;
    }
    let policy = unsafe { (*devNode).policy };
    let serv_name = unsafe { (*devNode).servName };
    if policy as u32 == crate::types::SERVICE_POLICY_NONE ||
        (!serv_name.is_null() && unsafe { libc::strlen(serv_name as *const i8) } == 0) {
        return status;
    }
    let node_if = unsafe { &(*devNode).super_ };
    if policy as u32 == crate::types::SERVICE_POLICY_PUBLIC || policy as u32 == crate::types::SERVICE_POLICY_CAPACITY {
        if let Some(f) = node_if.PublishService {
            status = unsafe { f(devNode) };
        }
    }
    if status == crate::types::HDF_SUCCESS {
        status = crate::src_hdf_device_node::HdfDeviceNodePublishLocalService(devNode);
    }
    status
}

pub extern "C" fn DeviceDriverBind(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut driverEntry: *const crate::types::HdfDriverEntry = std::ptr::null();
    if devNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        driverEntry = (*devNode).driver.as_ref().unwrap().entry;
        if (*devNode).policy == crate::types::SERVICE_POLICY_PUBLIC as u16
            || (*devNode).policy == crate::types::SERVICE_POLICY_CAPACITY as u16
        {
            if let Some(bind_func) = (*driverEntry).Bind {
                ret = bind_func(&mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject);
                if ret != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
                }
            } else {
                (*devNode).devStatus = crate::types::DEVNODE_NONE as u8;
                return crate::types::HDF_ERR_INVALID_OBJECT;
            }
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDeviceLaunchNode(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to launch service, device or service is null\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let serv_name_ptr = unsafe { (*devNode).servName };
    let serv_name_str = if serv_name_ptr.is_null() { b"\0".as_ptr() } else { serv_name_ptr as *const _ };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD002510, b"device_node\0".as_ptr() as *const _, b"launch devnode %{public}s\0".as_ptr() as *const _, serv_name_str) };
    let driver = unsafe { (*devNode).driver };
    if driver.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let driver_entry = unsafe { (*driver).entry };
    if driver_entry.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let init_fn = unsafe { (*driver_entry).Init };
    if init_fn.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe { (*devNode).devStatus = crate::types::DEVNODE_LAUNCHED as u8 };
    ret = crate::src_hdf_device_node::DeviceDriverBind(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }
    let device_object_ptr = unsafe { &mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject };
    ret = unsafe { init_fn.unwrap()(device_object_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
    }
    ret = crate::src_hdf_device_node::HdfDeviceNodePublishService(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_PUBLISH_FAIL;
    }
    let token_ptr = unsafe { (*devNode).token };
    ret = crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDevice(token_ptr);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_ATTACHDEV_FAIL;
    }
    ret
}

pub extern "C" fn HdfDeviceNodeAddPowerStateListener(devNode: *mut crate::types::HdfDeviceNode, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    unsafe {
        if (*devNode).powerToken != std::ptr::null_mut() {
            return crate::types::HDF_FAILURE;
        }
        let token = crate::src_power_state_token::PowerStateTokenNewInstance(&mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject, listener);
        (*devNode).powerToken = token;
        if token.is_null() {
            crate::types::HDF_FAILURE
        } else {
            crate::types::HDF_SUCCESS
        }
    }
}

pub extern "C" fn HdfDeviceNodeRemovePowerStateListener(devNode: *mut crate::types::HdfDeviceNode, listener: *const crate::types::IPowerEventListener) {
    let _ = listener;
    if devNode.is_null() {
        return;
    }
    unsafe {
        if (*devNode).powerToken.is_null() {
            return;
        }
        crate::src_power_state_token::PowerStateTokenFreeInstance((*devNode).powerToken);
        (*devNode).powerToken = std::ptr::null_mut();
    }
}

pub extern "C" fn HdfDeviceNodePublishPublicService(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish public service, devNode is NULL\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if (*devNode).deviceObject.service.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish public service, devNode is NULL\0".as_ptr() as *const _);
            return crate::types::HDF_FAILURE;
        }
    }
    let mut servInfo: crate::types::HdfServiceInfo = unsafe { ::core::mem::zeroed() };
    unsafe {
        servInfo.servName = (*devNode).servName;
        servInfo.servInfo = (*devNode).servInfo;
        servInfo.devClass = (*devNode).deviceObject.deviceClass as u16;
        servInfo.devId = (*devNode).devId;
        servInfo.interfaceDesc = (*devNode).interfaceDesc;
    }
    ret = unsafe { crate::src_devsvc_manager_clnt::DevSvcManagerClntAddService(&mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject, &servInfo as *const crate::types::HdfServiceInfo) };
    if ret == crate::types::HDF_SUCCESS {
        unsafe {
            (*devNode).servStatus = true;
        }
    }
    ret
}

pub extern "C" fn HdfDeviceNodeRemoveService(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    if !devNode.is_null() && unsafe { (*devNode).servStatus } {
        unsafe {
            crate::src_devsvc_manager_clnt::DevSvcManagerClntRemoveService((*devNode).servName as *const ::core::ffi::c_char);
            (*devNode).servStatus = false;
        }
    }
    crate::types::HDF_SUCCESS
}

fn HdfDeviceUnlaunchNode(devNode: *mut crate::types::HdfDeviceNode) {
    let mut driverEntry: *const crate::types::HdfDriverEntry = std::ptr::null();
    let mut driverLoader: *mut crate::types::IDriverLoader = std::ptr::null_mut();
    if devNode.is_null() {
        return;
    }
    unsafe {
        if (*devNode).devStatus != crate::types::DEVNODE_LAUNCHED as u8 {
            return;
        }
        if !(*devNode).driver.is_null() {
            driverEntry = (*(*devNode).driver).entry;
        }
        if !driverEntry.is_null() && !(*driverEntry).Release.is_none() {
            if let Some(f) = (*driverEntry).Release {
                f(&mut (*devNode).deviceObject);
            }
        }
        if (*devNode).servStatus {
            if let Some(f) = (*devNode).super_.RemoveService {
                let _ = f(devNode);
            }
        }
        crate::src_devmgr_service_clnt::DevmgrServiceClntDetachDevice((*devNode).devId);
        driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
        if !driverLoader.is_null() {
            if let Some(f) = (*driverLoader).ReclaimDriver {
                f((*devNode).driver);
            }
            (*devNode).driver = std::ptr::null_mut();
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const _,
                b"failed to get driver loader\0".as_ptr() as *const _,
            );
        }
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_device_node_10
// c_function: HdfDeviceNodeConstruct
// rust_file: src_hdf_device_node.rs
// rust_signature: pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode)
// c_first_line: void HdfDeviceNodeConstruct(struct HdfDeviceNode *devNode)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_node_10/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `HdfDeviceLaunchNode` in module `crate::compat`
//     --> src/src_hdf_device_node.rs:88:53
//      |
//      |                                                     ^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
//   error[E0425]: cannot find value `HdfDeviceNodePublishPublicService` in module `crate::compat`
//     --> src/src_hdf_device_node.rs:89:57
//      |
//      |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
// =================================
pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_device_node::HdfDeviceNodeConstruct(devNode as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_device_node_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_device_node_10/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            nodeIf.LaunchNode = Some(crate::compat::HdfDeviceLaunchNode);
            nodeIf.PublishService = Some(crate::compat::HdfDeviceNodePublishPublicService);
            nodeIf.RemoveService = Some(crate::compat::HdfDeviceNodeRemoveService);
            nodeIf.UnlaunchNode = Some(crate::compat::HdfDeviceUnlaunchNode);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_device_node_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfDeviceNodeDestruct(devNode: *mut crate::types::HdfDeviceNode) {
    if devNode.is_null() {
        return;
    }
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD002510,
            b"device_node\0".as_ptr() as *const _,
            b"release devnode %{public}s\0".as_ptr() as *const _,
            (*devNode).servName,
        )
    };
    unsafe {
        match (*devNode).devStatus {
            2 => {
                crate::src_hdf_device_node::HdfDeviceUnlaunchNode(devNode);
            }
            1 => {
                crate::src_hdf_device_token::HdfDeviceTokenFreeInstance((*devNode).token);
                (*devNode).token = std::ptr::null_mut();
                crate::src_power_state_token::PowerStateTokenFreeInstance((*devNode).powerToken);
                (*devNode).powerToken = std::ptr::null_mut();
                crate::compat::OsalMemFree((*devNode).servName as *mut _);
                crate::compat::OsalMemFree((*devNode).servInfo as *mut _);
                crate::compat::OsalMemFree((*devNode).driverName as *mut _);
                (*devNode).servName = std::ptr::null_mut();
                (*devNode).servInfo = std::ptr::null_mut();
            }
            0 => {}
            _ => {}
        }
    }
}

pub extern "C" fn HdfDeviceNodeNewInstance(deviceInfo: *const crate::types::HdfDeviceInfo, driver: *mut crate::types::HdfDriver) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceInfo.is_null() {
        return std::ptr::null_mut();
    }
    devNode = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut crate::types::HdfDeviceNode };
    if devNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*devNode).driver = driver;
        (*devNode).devId = (*deviceInfo).deviceId;
        (*devNode).permission = (*deviceInfo).permission;
        (*devNode).policy = (*deviceInfo).policy;
        if !(*devNode).token.is_null() {
            (*(*devNode).token).devid = (*deviceInfo).deviceId;
        }
        (*devNode).servName = crate::compat::HdfStringCopy((*deviceInfo).svcName);
        if !(*devNode).token.is_null() {
            (*(*devNode).token).servName = crate::compat::HdfStringCopy((*deviceInfo).svcName);
            (*(*devNode).token).deviceName = crate::compat::HdfStringCopy((*deviceInfo).deviceName);
        }
    }
    unsafe {
        if (*devNode).servName.is_null() {
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
            return std::ptr::null_mut();
        }
    }
    unsafe {
        (*devNode).deviceObject.property = crate::compat::HcsGetNodeByMatchAttr(crate::compat::HdfGetHcsRootNode(), (*deviceInfo).deviceMatchAttr);
        if (*devNode).deviceObject.property.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_DEBUG as u32, 0xD002510, b"device_node\0".as_ptr() as *const _, b"node %{public}s property empty, match attr: %{public}s\0".as_ptr() as *const _, (*deviceInfo).moduleName, (*deviceInfo).deviceMatchAttr);
        }
    }
    unsafe {
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
    devNode
}

pub extern "C" fn HdfDeviceNodeFreeInstance(devNode: *mut crate::types::HdfDeviceNode) {
    unsafe {
        crate::compat::HdfObjectManagerFreeObject(devNode as *mut crate::types::HdfObject);
    }
}
