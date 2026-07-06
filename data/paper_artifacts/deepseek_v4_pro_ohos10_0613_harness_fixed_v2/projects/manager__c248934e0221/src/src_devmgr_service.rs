//! Module: src_devmgr_service
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

fn DevmgrServiceDynamicDevInfoFound(svcName: *const std::ffi::c_char, targetHostClnt: *mut *mut crate::types::DevHostServiceClnt, targetDeviceInfo: *mut *mut crate::types::HdfDeviceInfo)-> bool {
    let dev_mgr_svc_ptr = crate::src_devmgr_service::DevmgrServiceGetInstance();
    if dev_mgr_svc_ptr.is_null() {
        return false;
    }
    let dev_mgr_svc = dev_mgr_svc_ptr as *mut crate::types::DevmgrService;
    let hosts_head = unsafe { ::core::ptr::addr_of!((*dev_mgr_svc).hosts) };
    let mut host_clnt: *mut crate::types::DevHostServiceClnt =
        unsafe { (*hosts_head).next as *mut crate::types::DevHostServiceClnt };
    let svc_name_cstr = unsafe { ::std::ffi::CStr::from_ptr(svcName as *const i8) };
    while host_clnt as *const crate::types::DListHead != hosts_head {
        let mut curr_node: *mut crate::types::HdfSListNode =
            unsafe { (*host_clnt).dynamicDevInfos.root };
        while !curr_node.is_null() {
            let device_info = curr_node as *mut crate::types::HdfDeviceInfo;
            let device_svc_name =
                unsafe { ::std::ffi::CStr::from_ptr((*device_info).svcName as *const i8) };
            if svc_name_cstr == device_svc_name {
                unsafe {
                    *targetDeviceInfo = device_info;
                    *targetHostClnt = host_clnt;
                }
                return true;
            }
            curr_node = unsafe { (*curr_node).next };
        }
        host_clnt = unsafe { (*host_clnt).node.next as *mut crate::types::DevHostServiceClnt };
    }
    false
}

fn DevmgrServiceStartHostProcess(hostClnt: *mut crate::types::DevHostServiceClnt, sync: bool, dynamic: bool)-> i32 {
    let mut wait_count: i32 = 1000;
    let installer = crate::src_hdf_driver_installer::DriverInstallerGetInstance();
    if installer.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let start_device_host = unsafe { (*installer).StartDeviceHost };
    if start_device_host.is_none() {
        return crate::types::HDF_FAILURE;
    }
    let start_fn = start_device_host.unwrap();
    let host_pid = unsafe { start_fn((*hostClnt).hostId as u32, (*hostClnt).hostName, dynamic) };
    unsafe { (*hostClnt).hostPid = host_pid; }

    if host_pid == crate::types::HDF_FAILURE {
        return crate::types::HDF_FAILURE;
    }

    unsafe { (*hostClnt).stopFlag = false; }

    if !sync {
        return crate::types::HDF_SUCCESS;
    }

    while unsafe { (*hostClnt).hostService.is_null() } && wait_count > 0 {
        unsafe { libc::usleep(1000u32); }
        wait_count -= 1;
    }

    if wait_count <= 0 {
        unsafe { (*hostClnt).hostPid = -1; }
        return crate::types::HDF_ERR_TIMEOUT;
    }

    crate::types::HDF_SUCCESS
}

fn DevmgrServiceLoadDevice(devMgrSvc: *mut crate::types::IDevmgrService, serviceName: *const std::ffi::c_char)-> i32 {
    let _ = devMgrSvc;
    if serviceName.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    fn load_device_safe(hostClnt: *mut crate::types::DevHostServiceClnt, deviceInfo: *mut crate::types::HdfDeviceInfo, serviceName: *const std::ffi::c_char)-> i32 {
        // Check preload
        if unsafe { (*deviceInfo).preload } != crate::types::DEVICE_PRELOAD_DISABLE as u16 {
            unsafe { HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const i8,
                b"device %{public}s not an dynamic load device\0".as_ptr() as *const i8,
                serviceName,
            ); }
            return crate::types::HDF_DEV_ERR_NORANGE;
        }

        let dynamic = unsafe { (*hostClnt).unloadDevInfos.root.is_null() && !(*hostClnt).dynamicDevInfos.root.is_null() };

        unsafe { OsalMutexLock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex); }

        if unsafe { (*hostClnt).hostPid } < 0 {
            unsafe { OsalMutexUnlock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex); }
            if crate::src_devmgr_service::DevmgrServiceStartHostProcess(hostClnt, true, dynamic) != crate::types::HDF_SUCCESS {
                unsafe { HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_WARN,
                    0xD002510u32,
                    b"devmgr_service\0".as_ptr() as *const i8,
                    b"failed to start device host(%{public}s, %{public}u)\0".as_ptr() as *const i8,
                    (*hostClnt).hostName,
                    (*hostClnt).hostId as u32,
                ); }
                return crate::types::HDF_FAILURE;
            }
            unsafe { OsalMutexLock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex); }
        }

        let host_service = unsafe { (*hostClnt).hostService };
        if host_service.is_null() || unsafe { (*host_service).AddDevice.is_none() } {
            unsafe { OsalMutexUnlock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex); }
            unsafe { HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const i8,
                b"%{public}s load %{public}s failed, hostService is null\0".as_ptr() as *const i8,
                b"DevmgrServiceLoadDevice\0".as_ptr() as *const i8,
                serviceName,
            ); }
            return crate::types::HDF_FAILURE;
        }

        let ret = unsafe { (*host_service).AddDevice.unwrap_unchecked()(host_service, deviceInfo as *const crate::types::HdfDeviceInfo) };
        unsafe { OsalMutexUnlock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex); }

        if ret == crate::types::HDF_SUCCESS {
            unsafe { (*deviceInfo).status = crate::types::HDF_SERVICE_USABLE as u16; }
        }
        ret
    }

    let mut hostClnt: *mut crate::types::DevHostServiceClnt = std::ptr::null_mut();
    let mut deviceInfo: *mut crate::types::HdfDeviceInfo = std::ptr::null_mut();

    if !crate::src_devmgr_service::DevmgrServiceDynamicDevInfoFound(serviceName, &mut hostClnt, &mut deviceInfo) {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const i8,
                b"device %{public}s not in configed device list\0".as_ptr() as *const i8,
                serviceName,
            );
        }
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    load_device_safe(hostClnt, deviceInfo, serviceName)
}

fn DevmgrServiceStopHost(hostClnt: *mut crate::types::DevHostServiceClnt)-> i32 {
    let installer = crate::src_hdf_driver_installer::DriverInstallerGetInstance();
    if installer.is_null() || unsafe { (*installer).StopDeviceHost.is_none() } {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        let stop_fn = (*installer).StopDeviceHost.unwrap();
        let _ = stop_fn((*hostClnt).hostId as u32, (*hostClnt).hostName);
        (*hostClnt).stopFlag = true;
    }
    return crate::types::HDF_SUCCESS;
}

/// Safe wrapper around narrow unsafe helpers for unloading a device.
fn unload_device_safe(hostClnt: *mut crate::types::DevHostServiceClnt, deviceInfo: *mut crate::types::HdfDeviceInfo) -> i32 {
    // Safety: hostClnt and deviceInfo are non-null (validated by caller).
    let host_service = unsafe { (*hostClnt).hostService };
    if unsafe { host_service_unavailable(host_service) } {
        return crate::types::HDF_FAILURE;
    }
    let ret = unsafe { call_del_device(host_service, (*deviceInfo).deviceId) };
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }
    unsafe { update_status_and_clean(hostClnt, deviceInfo) }
}

/// Checks if hostService is null or lacks DelDevice callback.
#[inline]
#[allow(unsafe_code)]
unsafe fn host_service_unavailable(host_service: *mut crate::types::IDevHostService) -> bool {
    host_service.is_null() || (*host_service).DelDevice.is_none()
}

/// Calls the DelDevice callback.
#[inline]
#[allow(unsafe_code)]
unsafe fn call_del_device(host_service: *mut crate::types::IDevHostService, device_id: u32) -> i32 {
    (*host_service).DelDevice.unwrap()(host_service, device_id)
}

/// Sets status to UNUSABLE, and if device list is empty, resets hostStatus and frees device list.
#[inline]
#[allow(unsafe_code)]
unsafe fn update_status_and_clean(hostClnt: *mut crate::types::DevHostServiceClnt, deviceInfo: *mut crate::types::HdfDeviceInfo) -> i32 {
    (*deviceInfo).status = crate::types::HDF_SERVICE_UNUSABLE as u16;
    if !(*hostClnt).devices.root.is_null() || !(*hostClnt).unloadDevInfos.root.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    (*hostClnt).hostPid = -1;
    (*hostClnt).hostService = std::ptr::null_mut();
    free_device_list(hostClnt);
    crate::types::HDF_SUCCESS
}

unsafe fn free_device_list(hostClnt: *mut crate::types::DevHostServiceClnt) {
    let mut node = (*hostClnt).devices.root;
    while !node.is_null() {
        let next = (*node).next;
        crate::src_device_token_clnt::DeviceTokenClntDelete(node);
        node = next;
    }
    (*hostClnt).devices.root = std::ptr::null_mut();
}

fn DevmgrServiceUnloadDevice(devMgrSvc: *mut crate::types::IDevmgrService, serviceName: *const std::ffi::c_char)-> i32 {
    let mut hostClnt: *mut crate::types::DevHostServiceClnt = std::ptr::null_mut();
    let mut deviceInfo: *mut crate::types::HdfDeviceInfo = std::ptr::null_mut();
    let _ = devMgrSvc;

    if serviceName.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    if !crate::src_devmgr_service::DevmgrServiceDynamicDevInfoFound(
        serviceName,
        std::ptr::addr_of_mut!(hostClnt),
        std::ptr::addr_of_mut!(deviceInfo),
    ) || unsafe { (*deviceInfo).preload as crate::types::DevicePreload != crate::types::DEVICE_PRELOAD_DISABLE }
    {
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    unsafe { OsalMutexLock(std::ptr::addr_of_mut!((*hostClnt).hostLock)); }
    let inner_ret = unload_device_safe(hostClnt, deviceInfo);
    unsafe { OsalMutexUnlock(std::ptr::addr_of_mut!((*hostClnt).hostLock)); }

    if inner_ret == crate::types::HDF_SUCCESS {
        return crate::src_devmgr_service::DevmgrServiceStopHost(hostClnt);
    }
    inner_ret
}

unsafe fn process_unload_devices(host_clnt: *mut crate::types::DevHostServiceClnt) {
    use crate::types::*;
    let unload_list = core::ptr::addr_of_mut!((*host_clnt).unloadDevInfos);
    let mut prev_node: *mut HdfSListNode = std::ptr::null_mut();
    let mut curr_node = (*unload_list).root;
    while !curr_node.is_null() {
        let device_info = curr_node as *mut HdfDeviceInfo;
        if (*device_info).preload == DEVICE_PRELOAD_ENABLE_STEP2 as u16 {
            let host_service = (*host_clnt).hostService;
            if let Some(add_dev) = (*host_service).AddDevice {
                let ret = add_dev(host_service, device_info as *const HdfDeviceInfo);
                if ret != HDF_SUCCESS {
                    prev_node = curr_node;
                    curr_node = (*curr_node).next;
                    continue;
                }
                (*device_info).status = HDF_SERVICE_USABLE as u16;
                if prev_node.is_null() {
                    (*unload_list).root = (*curr_node).next;
                } else {
                    (*prev_node).next = (*curr_node).next;
                }
                let next_node = (*curr_node).next;
                curr_node = next_node;
                continue;
            } else {
                prev_node = curr_node;
                curr_node = (*curr_node).next;
                continue;
            }
        }
        prev_node = curr_node;
        curr_node = (*curr_node).next;
    }
}

pub extern "C" fn DevmgrServiceLoadLeftDriver(devMgrSvc: *mut crate::types::DevmgrService) -> i32 {
    use crate::types::*;

    if devMgrSvc.is_null() {
        return HDF_FAILURE;
    }

    let offset = crate::compat::offset_of!(DevHostServiceClnt, node);

    unsafe {
        let hosts = core::ptr::addr_of!((*devMgrSvc).hosts) as *mut DListHead;
        let mut node_ptr = (*hosts).next;
        while node_ptr != hosts {
            let host_clnt = ((node_ptr as *const u8).offset(-(offset as isize))) as *mut DevHostServiceClnt;
            process_unload_devices(host_clnt);
            node_ptr = (*node_ptr).next;
        }
    }

    HDF_SUCCESS
}

fn DevmgrServiceFindDeviceHost(inst: *mut crate::types::IDevmgrService, hostId: u16)-> *mut crate::types::DevHostServiceClnt {
    let dm_service = inst as *mut crate::types::DevmgrService;
    if dm_service.is_null() {
        return std::ptr::null_mut();
    }

    let offset = crate::compat::offset_of!(crate::types::DevHostServiceClnt, node);

    unsafe {
        let mut host_clnt = {
            let node_ptr = (*dm_service).hosts.next;
            (node_ptr as *mut u8).sub(offset) as *mut crate::types::DevHostServiceClnt
        };

        while core::ptr::addr_of!((*host_clnt).node) != core::ptr::addr_of!((*dm_service).hosts) {
            if (*host_clnt).hostId == hostId {
                return host_clnt;
            }
            let next_node = (*host_clnt).node.next;
            host_clnt = (next_node as *mut u8).sub(offset) as *mut crate::types::DevHostServiceClnt;
        }
    }

    std::ptr::null_mut()
}

fn DevmgrServiceAttachDevice(inst: *mut crate::types::IDevmgrService, token: *mut crate::types::IHdfDeviceToken)-> i32 {
    let mut host_clnt: *mut crate::types::DevHostServiceClnt = std::ptr::null_mut();
    let mut token_clnt: *mut crate::types::DeviceTokenClnt = std::ptr::null_mut();

    if token.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let devid = unsafe { (*token).devid };
    let host_id = (devid >> 24) as u16;
    host_clnt = crate::src_devmgr_service::DevmgrServiceFindDeviceHost(inst, host_id);
    if host_clnt.is_null() {
        return crate::types::HDF_FAILURE;
    }

    token_clnt = crate::src_device_token_clnt::DeviceTokenClntNewInstance(token);
    if token_clnt.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        (*token_clnt).node.next = (*host_clnt).devices.root;
        (*host_clnt).devices.root = &mut (*token_clnt).node as *mut crate::types::HdfSListNode;
    }

    crate::types::HDF_SUCCESS
}

fn HdfSListHostSearchDeviceTokenComparer(tokenNode: *mut crate::types::HdfSListNode, devid: u32)-> bool {
    let tokenClnt = tokenNode as *mut crate::types::DeviceTokenClnt;
    unsafe { (*(*tokenClnt).tokenIf).devid == devid }
}

fn DevmgrServiceDetachDevice(inst: *mut crate::types::IDevmgrService, devid: crate::types::devid_t)-> i32 {
    let host_id = ((devid as u32) >> 24) as u16;
    let host_clnt = unsafe {
        crate::src_devmgr_service::DevmgrServiceFindDeviceHost(inst, host_id)
    };
    if host_clnt.is_null() {
        // HiLogPrint not available; skip logging
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        let mut prev_ptr: *mut *mut crate::types::HdfSListNode = &mut (*host_clnt).devices.root as *mut _;
        let mut current_node = (*host_clnt).devices.root;
        while !current_node.is_null() {
            let token_clnt = current_node as *mut crate::types::DeviceTokenClnt;
            if (*(*token_clnt).tokenIf).devid == devid {
                *prev_ptr = (*current_node).next;
                return crate::types::HDF_SUCCESS;
            }
            prev_ptr = &mut (*current_node).next as *mut _;
            current_node = (*current_node).next;
        }
    }
    // HiLogPrint not available; skip logging
    crate::types::HDF_DEV_ERR_NO_DEVICE
}

fn DevmgrServiceAttachDeviceHost(inst: *mut crate::types::IDevmgrService, hostId: u16, hostService: *mut crate::types::IDevHostService)-> i32 {
    let hostClnt = crate::src_devmgr_service::DevmgrServiceFindDeviceHost(inst, hostId);
    if hostClnt.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    if hostService.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }

    unsafe {
        let _ = crate::compat::OsalMutexLock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex);
        (*hostClnt).hostService = hostService;
        let _ = crate::compat::OsalMutexUnlock(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex);
    }

    unsafe { crate::src_devhost_service_clnt::DevHostServiceClntInstallDriver(hostClnt) as i32 }
}

fn DevmgrServiceStartDeviceHost(devmgr: *mut crate::types::DevmgrService, hostAttr: *mut crate::types::HdfHostInfo)-> i32 {
    // (1) Safe outer: parameter validation
    if devmgr.is_null() || hostAttr.is_null() {
        return crate::types::HDF_FAILURE;
    }

    // (2) Narrow unsafe: read hostAttr fields
    let (host_id, host_name) = unsafe {
        ((*hostAttr).hostId, (*hostAttr).hostName)
    };

    // (3) Safe outer: allocate host client instance
    let host_clnt = unsafe {
        crate::src_devhost_service_clnt::DevHostServiceClntNewInstance(host_id, host_name)
    };
    if host_clnt.is_null() {
        return crate::types::HDF_FAILURE;
    }

    // (4) Safe: check attribute list
    if unsafe { crate::compat::HdfAttributeManagerGetDeviceList(host_clnt) } != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    // (5) Narrow unsafe: DListInsertTail(&hostClnt->node, &devmgr->hosts)
    unsafe {
        let host_node = &mut (*host_clnt).node as *mut crate::types::DListHead;
        let host_head = &mut (*devmgr).hosts as *mut crate::types::DListHead;
        (*host_node).next = host_head;
        let head_prev = (*host_head).prev;
        (*host_node).prev = head_prev;
        (*head_prev).next = host_node;
        (*host_head).prev = host_node;
    }

    // (6) Safe: check unloadDevInfos
    let has_unload_devices = unsafe { !(*host_clnt).unloadDevInfos.root.is_null() };
    if !has_unload_devices {
        return crate::types::HDF_SUCCESS;
    }

    // (7) Safe: try to start host process
    let start_result = crate::src_devmgr_service::DevmgrServiceStartHostProcess(host_clnt, false, false);
    if start_result == crate::types::HDF_SUCCESS {
        return crate::types::HDF_SUCCESS;
    }

    // (8) Narrow unsafe: error rollback (DListRemove + FreeInstance)
    unsafe {
        let entry = &mut (*host_clnt).node as *mut crate::types::DListHead;
        let entry_prev = (*entry).prev;
        let entry_next = (*entry).next;
        (*entry_prev).next = entry_next;
        (*entry_next).prev = entry_prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        crate::src_devhost_service_clnt::DevHostServiceClntFreeInstance(host_clnt);
    }
    crate::types::HDF_FAILURE
}

fn DevmgrServiceStartDeviceHosts(inst: *mut crate::types::DevmgrService)-> i32 {
    let mut hostList: crate::types::HdfSList = unsafe { std::mem::zeroed() };
    let mut it: crate::types::HdfSListIterator = unsafe { std::mem::zeroed() };
    let mut hostAttr: *mut crate::types::HdfHostInfo;
    let mut ret: i32;

    unsafe {
        HdfSListInit(&mut hostList);
    }
    if !unsafe { HdfAttributeManagerGetHostList(&mut hostList) } {
        // Warning log omitted because HiLogPrint is unavailable
        return crate::types::HDF_SUCCESS;
    }
    unsafe {
        HdfSListIteratorInit(&mut it, &hostList);
    }
    while unsafe { HdfSListIteratorHasNext(core::ptr::addr_of!(it)) } {
        // Inline replacement for HdfSListIteratorNext (unresolved symbol)
        hostAttr = unsafe {
            let iter = &mut *(core::ptr::addr_of_mut!(it) as *mut crate::types::HdfSListIterator);
            let cur = iter.curr; // guaranteed non-null by HasNext
            iter.prev = cur;
            iter.curr = (*cur).next;
            cur
        } as *mut crate::types::HdfHostInfo;

        ret = crate::src_devmgr_service::DevmgrServiceStartDeviceHost(inst, hostAttr);
        if ret != crate::types::HDF_SUCCESS {
            // Error log omitted
        }
    }
    unsafe {
        HdfSListFlush(
            &mut hostList,
            Some(crate::src_hdf_host_info::HdfHostInfoDelete),
        );
    }
    crate::types::HDF_SUCCESS
}

fn DevmgrServiceListAllDevice(inst: *mut crate::types::IDevmgrService, reply: *mut crate::types::HdfSBuf)-> i32 {
    let dev_mgr_svc = inst as *mut crate::types::DevmgrService;
    if dev_mgr_svc.is_null() || reply.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s failed, parameter is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevmgrServiceListAllDevice\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    // Compute offset of `node` field within DevHostServiceClnt
    let offset: isize = crate::compat::offset_of!(crate::types::DevHostServiceClnt, node) as isize;

    let hosts_ptr: *mut crate::types::DListHead = unsafe { &mut (*dev_mgr_svc).hosts as *mut crate::types::DListHead };

    // Initialize host_clnt from hosts_ptr->next
    let mut host_clnt: *mut crate::types::DevHostServiceClnt = unsafe {
        let next = (*hosts_ptr).next;
        ((next as *mut u8).offset(-offset)) as *mut crate::types::DevHostServiceClnt
    };

    while unsafe { core::ptr::addr_of_mut!((*host_clnt).node) } as *mut crate::types::DListHead != hosts_ptr {
        let host_name = unsafe { (*host_clnt).hostName };
        let host_id = unsafe { (*host_clnt).hostId };
        let devices_ptr: *const crate::types::HdfSList = unsafe { &(*host_clnt).devices as *const crate::types::HdfSList };

        unsafe {
            HdfSbufWriteString(reply, host_name);
            HdfSbufWriteUint32(reply, host_id as u32);
            HdfSbufWriteUint32(reply, HdfSListCount(devices_ptr).try_into().unwrap());
        }

        let mut iterator: crate::types::HdfSListIterator = unsafe { std::mem::zeroed() };
        unsafe { HdfSListIteratorInit(&mut iterator as *mut crate::types::HdfSListIterator, devices_ptr) };

        while unsafe { HdfSListIteratorHasNext(&iterator as *const crate::types::HdfSListIterator) } {
            let node = unsafe { HdfSListIteratorNext(&mut iterator as *mut crate::types::HdfSListIterator) };
            let token_clnt = node as *mut crate::types::DeviceTokenClnt;

            let (valid, token_if) = unsafe {
                if token_clnt.is_null() {
                    (false, std::ptr::null_mut())
                } else {
                    let tif = (*token_clnt).tokenIf;
                    (!tif.is_null(), tif)
                }
            };

            if valid {
                let dev_name = unsafe { (*token_if).deviceName };
                let name = if dev_name.is_null() {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    dev_name
                };
                let devid = unsafe { (*token_if).devid };
                let serv_name = unsafe { (*token_if).servName };
                let name2 = if serv_name.is_null() {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    serv_name
                };

                unsafe {
                    HdfSbufWriteString(reply, name);
                    HdfSbufWriteUint32(reply, devid);
                    HdfSbufWriteString(reply, name2);
                }
            } else {
                let host_name_log = unsafe { (*host_clnt).hostName };
                unsafe {
                    HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD002510u32,
                        b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char,
                        b"%{public}s host:%{public}s token null\0".as_ptr() as *const ::core::ffi::c_char,
                        b"DevmgrServiceListAllDevice\0".as_ptr() as *const ::core::ffi::c_char,
                        host_name_log,
                    );
                }
            }
        }

        // Advance host_clnt to next entry
        host_clnt = unsafe {
            let next = (*host_clnt).node.next;
            ((next as *mut u8).offset(-offset)) as *mut crate::types::DevHostServiceClnt
        };
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn DevmgrServiceStartService(inst: *mut crate::types::IDevmgrService) -> ::core::ffi::c_int {
    let dm_service = inst as *mut crate::types::DevmgrService;
    if dm_service.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to start device manager service, dmService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let ret = crate::src_devmgr_service::DevmgrServiceStartDeviceHosts(dm_service);
    let start_service_ret = unsafe { crate::src_devsvc_manager::DevSvcManagerStartService() };
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char,
            b"start svcmgr result %{public}d. Init DeviceHosts info result: %{public}d\0".as_ptr() as *const ::core::ffi::c_char,
            start_service_ret,
            ret,
        );
    }
    ret
}

pub extern "C" fn DevmgrServicePowerStateChange(devmgrService: *mut crate::types::IDevmgrService, powerState: crate::types::HdfPowerState) -> ::core::ffi::c_int {
    if devmgrService.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    // Inline IsValidPowerState: state < POWER_STATE_MAX (4)
    if powerState >= 4 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s:invalid power event %{public}u\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevmgrServicePowerStateChange\0".as_ptr() as *const ::core::ffi::c_char,
                powerState,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let devmgr = devmgrService as *mut crate::types::DevmgrService;
    let mut result: i32 = crate::types::HDF_SUCCESS;

    // Inline IsPowerWakeState: state == POWER_STATE_DOZE_RESUME (1) || state == POWER_STATE_RESUME (3)
    let is_wake: bool = powerState == 1 || powerState == 3;

    // Prepare log messages outside unsafe
    let wake_msg = unsafe { ::std::ffi::CStr::from_ptr(b"%{public}s:wake state %{public}u\0".as_ptr() as *const ::core::ffi::c_char) };
    let suspend_msg = unsafe { ::std::ffi::CStr::from_ptr(b"%{public}s:suspend state %{public}u\0".as_ptr() as *const ::core::ffi::c_char) };
    let func_name_cstr = unsafe { ::std::ffi::CStr::from_ptr(b"DevmgrServicePowerStateChange\0".as_ptr() as *const ::core::ffi::c_char) };
    let tag_cstr = unsafe { ::std::ffi::CStr::from_ptr(b"devmgr_service\0".as_ptr() as *const ::core::ffi::c_char) };

    let hosts_ptr = unsafe { core::ptr::addr_of_mut!((*devmgr).hosts) };

    if is_wake {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510u32,
                tag_cstr.as_ptr(),
                wake_msg.as_ptr(),
                func_name_cstr.as_ptr(),
                powerState,
            );
        }
        let mut host_client_node = unsafe { (*devmgr).hosts.next };
        while host_client_node != hosts_ptr {
            unsafe {
                let host_client = host_client_node as *mut crate::types::DevHostServiceClnt;
                let host_service = (*host_client).hostService;
                if !host_service.is_null() {
                    if let Some(pm_fn) = (*host_service).PmNotify {
                        if pm_fn(host_service, powerState) != crate::types::HDF_SUCCESS {
                            result = crate::types::HDF_FAILURE;
                        }
                    }
                }
                host_client_node = (*host_client).node.next;
            }
        }
    } else {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510u32,
                tag_cstr.as_ptr(),
                suspend_msg.as_ptr(),
                func_name_cstr.as_ptr(),
                powerState,
            );
        }
        let mut host_client_node = unsafe { (*devmgr).hosts.prev };
        while host_client_node != hosts_ptr {
            unsafe {
                let host_client = host_client_node as *mut crate::types::DevHostServiceClnt;
                let host_service = (*host_client).hostService;
                if !host_service.is_null() {
                    if let Some(pm_fn) = (*host_service).PmNotify {
                        if pm_fn(host_service, powerState) != crate::types::HDF_SUCCESS {
                            result = crate::types::HDF_FAILURE;
                        }
                    }
                }
                host_client_node = (*host_client).node.prev;
            }
        }
    }

    result
}

pub extern "C" fn DevmgrServiceConstruct(inst: *mut crate::types::DevmgrService) -> bool {
    if inst.is_null() {
        return false;
    }
    let mutex_ptr = unsafe { &mut (*inst).devMgrMutex as *mut crate::types::OsalMutex };
    if unsafe { OsalMutexInit(mutex_ptr) } != crate::types::HDF_SUCCESS {
        return false;
    }
    let dev_mgr_svc_if = unsafe { &mut (*inst).super_ as *mut crate::types::IDevmgrService };
    if dev_mgr_svc_if.is_null() {
        return false;
    }
    unsafe {
        (*dev_mgr_svc_if).AttachDevice = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, *mut crate::types::IHdfDeviceToken) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, *mut crate::types::IHdfDeviceToken) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceAttachDevice as fn(*mut crate::types::IDevmgrService, *mut crate::types::IHdfDeviceToken) -> i32)
        );
        (*dev_mgr_svc_if).DetachDevice = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, crate::types::devid_t) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, crate::types::devid_t) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceDetachDevice as fn(*mut crate::types::IDevmgrService, crate::types::devid_t) -> i32)
        );
        (*dev_mgr_svc_if).LoadDevice = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceLoadDevice as fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> i32)
        );
        (*dev_mgr_svc_if).UnloadDevice = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceUnloadDevice as fn(*mut crate::types::IDevmgrService, *const ::core::ffi::c_char) -> i32)
        );
        (*dev_mgr_svc_if).AttachDeviceHost = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, u16, *mut crate::types::IDevHostService) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, u16, *mut crate::types::IDevHostService) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceAttachDeviceHost as fn(*mut crate::types::IDevmgrService, u16, *mut crate::types::IDevHostService) -> i32)
        );
        (*dev_mgr_svc_if).StartService = Some(
            crate::src_devmgr_service::DevmgrServiceStartService as unsafe extern "C" fn(*mut crate::types::IDevmgrService) -> ::core::ffi::c_int
        );
        (*dev_mgr_svc_if).PowerStateChange = Some(
            crate::src_devmgr_service::DevmgrServicePowerStateChange as unsafe extern "C" fn(*mut crate::types::IDevmgrService, crate::types::HdfPowerState) -> ::core::ffi::c_int
        );
        (*dev_mgr_svc_if).ListAllDevice = Some(
            std::mem::transmute::<
                fn(*mut crate::types::IDevmgrService, *mut crate::types::HdfSBuf) -> i32,
                unsafe extern "C" fn(*mut crate::types::IDevmgrService, *mut crate::types::HdfSBuf) -> ::core::ffi::c_int,
            >(crate::src_devmgr_service::DevmgrServiceListAllDevice as fn(*mut crate::types::IDevmgrService, *mut crate::types::HdfSBuf) -> i32)
        );
    }
    let hosts_ptr = unsafe { &mut (*inst).hosts as *mut crate::types::DListHead };
    unsafe {
        (*hosts_ptr).next = hosts_ptr;
        (*hosts_ptr).prev = hosts_ptr;
    }
    true
}

pub extern "C" fn DevmgrServiceCreate() -> *mut crate::types::HdfObject {
    use std::sync::OnceLock;

    struct SingletonPtr(*mut crate::types::DevmgrService);
    unsafe impl Send for SingletonPtr {}
    unsafe impl Sync for SingletonPtr {}

    static INSTANCE: OnceLock<SingletonPtr> = OnceLock::new();

    let SingletonPtr(mgr_ptr) = *INSTANCE.get_or_init(|| {
        let size = std::mem::size_of::<crate::types::DevmgrService>();
        let ptr = unsafe { OsalMemCalloc(size as u32) } as *mut crate::types::DevmgrService;
        if ptr.is_null() {
            return SingletonPtr(std::ptr::null_mut());
        }
        if !crate::src_devmgr_service::DevmgrServiceConstruct(ptr) {
            unsafe { OsalMemFree(ptr as *mut ::core::ffi::c_void); }
            return SingletonPtr(std::ptr::null_mut());
        }
        SingletonPtr(ptr)
    });

    if mgr_ptr.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { core::ptr::addr_of_mut!((*mgr_ptr).super_.object) as *mut crate::types::HdfObject }
}

pub extern "C" fn DevmgrServiceGetInstance() -> *mut crate::types::IDevmgrService {
    use std::sync::OnceLock;

    struct SingletonPtr(*mut crate::types::IDevmgrService);
    unsafe impl Send for SingletonPtr {}
    unsafe impl Sync for SingletonPtr {}

    static INSTANCE: OnceLock<SingletonPtr> = OnceLock::new();

    let SingletonPtr(ptr) = *INSTANCE.get_or_init(|| {
        let obj = unsafe { crate::compat::HdfObjectManagerGetObject(
            crate::types::HDF_OBJECT_ID_DEVMGR_SERVICE as i32,
        ) };
        SingletonPtr(obj as *mut crate::types::IDevmgrService)
    });

    ptr
}

pub extern "C" fn DevmgrServiceRelease(object: *mut crate::types::HdfObject) {
    let devmgr_service = object as *mut crate::types::DevmgrService;
    if devmgr_service.is_null() {
        return;
    }

    // Offset of `node` in DevHostServiceClnt; used for container_of.
    let node_offset = crate::compat::offset_of!(crate::types::DevHostServiceClnt, node);

    unsafe { release_host_list(devmgr_service, node_offset); }

    unsafe {
        OsalMutexDestroy(core::ptr::addr_of_mut!((*devmgr_service).devMgrMutex));
    }
}

/// Releases all hosts in the provided hosts DList.
/// # Safety
/// `devmgr_service` must point to a valid DevmgrService.
unsafe fn release_host_list(devmgr_service: *mut crate::types::DevmgrService, node_offset: usize) {
    let hosts = core::ptr::addr_of!((*devmgr_service).hosts);
    let first_node = (*hosts).next;

    let mut host_clnt = (first_node as *const u8).offset(-(node_offset as isize))
        as *mut crate::types::DevHostServiceClnt;
    let mut host_clnt_tmp = {
        let next = (*host_clnt).node.next;
        (next as *const u8).offset(-(node_offset as isize))
            as *mut crate::types::DevHostServiceClnt
    };

    while core::ptr::addr_of!((*host_clnt).node) != hosts {
        // DListRemove inline
        let entry: *mut crate::types::DListHead = core::ptr::addr_of_mut!((*host_clnt).node);
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        crate::src_devhost_service_clnt::DevHostServiceClntDelete(host_clnt);

        host_clnt = host_clnt_tmp;

        let next_node = (*host_clnt).node.next;
        host_clnt_tmp = (next_node as *const u8).offset(-(node_offset as isize))
            as *mut crate::types::DevHostServiceClnt;
    }
}
