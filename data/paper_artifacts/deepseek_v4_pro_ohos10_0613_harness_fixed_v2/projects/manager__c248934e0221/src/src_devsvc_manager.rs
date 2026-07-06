//! Module: src_devsvc_manager
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

fn DevSvcManagerSearchServiceLocked(inst: *mut crate::types::IDevSvcManager, serviceKey: u32)-> *mut crate::types::DevSvcRecord {
    let mut search_result: *mut crate::types::DevSvcRecord = std::ptr::null_mut();

    let dev_svc_manager = inst as *mut crate::types::DevSvcManager;
    if dev_svc_manager.is_null() {
        // HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "devsvc_manager", "failed to search service, devSvcManager is null");
        return std::ptr::null_mut();
    }

    unsafe {
        let head = &mut (*dev_svc_manager).services as *mut crate::types::DListHead;
        let entry_offset = crate::compat::offset_of!(crate::types::DevSvcRecord, entry);

        let mut record = {
            let next_entry = (*head).next;
            (next_entry as *mut u8).wrapping_offset(-(entry_offset as isize)) as *mut crate::types::DevSvcRecord
        };

        while &(*record).entry as *const crate::types::DListHead != head as *const crate::types::DListHead {
            if (*record).key == serviceKey {
                search_result = record;
                break;
            }
            record = {
                let next_entry = (*record).entry.next;
                (next_entry as *mut u8).wrapping_offset(-(entry_offset as isize)) as *mut crate::types::DevSvcRecord
            };
        }
    }

    search_result
}

fn NotifyServiceStatusLocked(devSvcManager: *mut crate::types::DevSvcManager, record: *const crate::types::DevSvcRecord, status: u32) {
    unsafe {
        let mut svcstat = crate::types::ServiceStatus {
            deviceClass: (*record).devClass,
            serviceName: (*record).servName,
            status: status as u16,
            info: (*record).servInfo,
        };
        let offset = crate::compat::offset_of!(crate::types::ServStatListenerHolder, node);
        let head = core::ptr::addr_of!((*devSvcManager).svcstatListeners);
        let mut node = (*head).next;
        while node != head as *mut crate::types::DListHead {
            let holder =
                (node as *mut u8).offset(-(offset as isize)) as *mut crate::types::ServStatListenerHolder;
            let next_node = (*node).next;
            if ((*holder).listenClass & (*record).devClass) != 0 && (*holder).NotifyStatus.is_some() {
                let notify_fn = (*holder).NotifyStatus.unwrap();
                if notify_fn(holder, &mut svcstat as *mut crate::types::ServiceStatus)
                    == crate::types::HDF_FAILURE
                {
                    let prev = (*node).prev;
                    let next = (*node).next;
                    (*prev).next = next;
                    (*next).prev = prev;
                    (*node).prev = std::ptr::null_mut();
                    (*node).next = std::ptr::null_mut();
                    if let Some(recycle_fn) = (*holder).Recycle {
                        recycle_fn(holder);
                    }
                }
            }
            node = next_node;
        }
    }
}

fn NotifyServiceStatusOnRegisterLocked(devSvcManager: *const crate::types::DevSvcManager, listenerHolder: *mut crate::types::ServStatListenerHolder) {
    let entry_offset = {
        let null: *const crate::types::DevSvcRecord = std::ptr::null();
        let entry_ptr: *const crate::types::DListHead =
            unsafe { core::ptr::addr_of!((*null).entry) };
        entry_ptr as usize
    };

    unsafe {
        let services_ptr: *const crate::types::DListHead =
            core::ptr::addr_of!((*devSvcManager).services);
        let first_next: *mut crate::types::DListHead = (*devSvcManager).services.next;
        let mut record = ((first_next as *mut u8).offset(-(entry_offset as isize)))
            as *mut crate::types::DevSvcRecord;

        while core::ptr::addr_of!((*record).entry) as *const crate::types::DListHead != services_ptr {
            let listen_class = (*listenerHolder).listenClass;
            let dev_class = (*record).devClass;

            if (listen_class & dev_class) != 0 {
                let mut svcstat = crate::types::ServiceStatus {
                    serviceName: (*record).servName,
                    deviceClass: dev_class,
                    status: crate::types::SERVIE_STATUS_REGISTER as u16,
                    info: (*record).servInfo,
                };
                if let Some(notify) = (*listenerHolder).NotifyStatus {
                    notify(listenerHolder, &mut svcstat as *mut crate::types::ServiceStatus);
                }
            }

            let next_next: *mut crate::types::DListHead = (*record).entry.next;
            record = ((next_next as *mut u8).offset(-(entry_offset as isize)))
                as *mut crate::types::DevSvcRecord;
        }
    }
}

// Private unsafe helpers for DevSvcManagerAddService
unsafe fn fill_record(record: *mut crate::types::DevSvcRecord, servInfo: *const crate::types::HdfServiceInfo, service: *mut crate::types::HdfDeviceObject) -> bool {
    (*record).key = HdfStringMakeHashKey((*servInfo).servName, 0);
    (*record).value = service;
    (*record).devClass = (*servInfo).devClass;
    (*record).devId = (*servInfo).devId;
    (*record).servName = HdfStringCopy((*servInfo).servName) as *const ::core::ffi::c_char;
    (*record).servInfo = HdfStringCopy((*servInfo).servInfo) as *const ::core::ffi::c_char;

    if !(*servInfo).interfaceDesc.is_null() && libc::strcmp((*servInfo).interfaceDesc, b"\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
        (*record).interfaceDesc = HdfStringCopy((*servInfo).interfaceDesc) as *const ::core::ffi::c_char;
    }

    !(*record).servName.is_null()
}

unsafe fn insert_service_locked(devSvcManager: *mut crate::types::DevSvcManager, record: *mut crate::types::DevSvcRecord) {
    let entry_ptr: *mut crate::types::DListHead = &mut (*record).entry;
    let head_ptr: *mut crate::types::DListHead = &mut (*devSvcManager).services;
    (*entry_ptr).next = head_ptr;
    (*entry_ptr).prev = (*head_ptr).prev;
    (*(*head_ptr).prev).next = entry_ptr;
    (*head_ptr).prev = entry_ptr;
}

pub extern "C" fn DevSvcManagerAddService(inst: *mut crate::types::IDevSvcManager, service: *mut crate::types::HdfDeviceObject, servInfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let devSvcManager = inst as *mut crate::types::DevSvcManager;
    if devSvcManager.is_null() || service.is_null() || servInfo.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if unsafe { (*servInfo).servName.is_null() } {
        return crate::types::HDF_FAILURE;
    }

    unsafe { OsalMutexLock(&mut (*devSvcManager).mutex); }

    let serviceKey = unsafe { HdfStringMakeHashKey((*servInfo).servName, 0) };
    let existing = unsafe { crate::src_devsvc_manager::DevSvcManagerSearchServiceLocked(inst, serviceKey) };

    if !existing.is_null() {
        unsafe { (*existing).value = service; }
        unsafe { OsalMutexUnlock(&mut (*devSvcManager).mutex); }
        return crate::types::HDF_SUCCESS;
    }

    unsafe { OsalMutexUnlock(&mut (*devSvcManager).mutex); }

    let record = unsafe { DevSvcRecordNewInstance() };
    if record.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let ok = unsafe { fill_record(record, servInfo, service) };
    if !ok {
        unsafe { DevSvcRecordFreeInstance(record); }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    unsafe {
        OsalMutexLock(&mut (*devSvcManager).mutex);
        insert_service_locked(devSvcManager, record);
        crate::src_devsvc_manager::NotifyServiceStatusLocked(devSvcManager, record, crate::types::SERVIE_STATUS_START);
        OsalMutexUnlock(&mut (*devSvcManager).mutex);
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn DevSvcManagerUpdateService(inst: *mut crate::types::IDevSvcManager, service: *mut crate::types::HdfDeviceObject, servInfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let devSvcManager = inst as *mut crate::types::DevSvcManager;
    if devSvcManager.is_null() || service.is_null() || servInfo.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if unsafe { (*servInfo).servName.is_null() } {
        return crate::types::HDF_FAILURE;
    }

    unsafe { OsalMutexLock(&mut (*devSvcManager).mutex); }
    let serviceKey = unsafe { HdfStringMakeHashKey((*servInfo).servName, 0) };
    let record = crate::src_devsvc_manager::DevSvcManagerSearchServiceLocked(inst, serviceKey);
    if record.is_null() {
        unsafe { OsalMutexUnlock(&mut (*devSvcManager).mutex); }
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }

    if !unsafe { (*servInfo).servInfo.is_null() } {
        let servInfoStr = unsafe { HdfStringCopy((*servInfo).servInfo) };
        if servInfoStr.is_null() {
            unsafe { OsalMutexUnlock(&mut (*devSvcManager).mutex); }
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        unsafe {
            crate::compat::OsalMemFree((*record).servInfo as *mut ::core::ffi::c_void);
            (*record).servInfo = servInfoStr as *const ::core::ffi::c_char;
        }
    }

    unsafe {
        (*record).value = service;
        (*record).devClass = (*servInfo).devClass;
        (*record).devId = (*servInfo).devId;
        crate::src_devsvc_manager::NotifyServiceStatusLocked(devSvcManager, record, crate::types::SERVIE_STATUS_CHANGE);
        OsalMutexUnlock(&mut (*devSvcManager).mutex);
    }

    crate::types::HDF_SUCCESS
}



pub extern "C" fn DevSvcManagerSubscribeService(inst: *mut crate::types::IDevSvcManager, svcName: *const ::core::ffi::c_char, callBack: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    let dev_mgr_svc = crate::src_devmgr_service::DevmgrServiceGetInstance();
    if inst.is_null() || svcName.is_null() || dev_mgr_svc.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let device_service = crate::src_devsvc_manager::DevSvcManagerGetService(inst, svcName);
    if !device_service.is_null() {
        if let Some(on_connected) = callBack.OnServiceConnected {
            unsafe {
                on_connected(callBack.deviceObject, device_service as *const crate::types::HdfObject);
            }
        }
        return crate::types::HDF_SUCCESS;
    }

    let load_device = unsafe { (*dev_mgr_svc).LoadDevice };
    match load_device {
        Some(f) => unsafe { f(dev_mgr_svc, svcName) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn DevSvcManagerRemoveService(inst: *mut crate::types::IDevSvcManager, svcName: *const ::core::ffi::c_char, devObj: *const crate::types::HdfDeviceObject) {
    let devSvcManager = inst as *mut crate::types::DevSvcManager;
    if svcName.is_null() || devSvcManager.is_null() {
        return;
    }
    let serviceKey = unsafe { HdfStringMakeHashKey(svcName, 0) };
    let mut removeFlag = false;

    unsafe {
        let _ = OsalMutexLock(&mut (*devSvcManager).mutex as *mut crate::types::OsalMutex);
    }
    let serviceRecord =
        crate::src_devsvc_manager::DevSvcManagerSearchServiceLocked(inst, serviceKey);
    if serviceRecord.is_null() {
        unsafe {
            let _ = OsalMutexUnlock(&mut (*devSvcManager).mutex as *mut crate::types::OsalMutex);
        }
        return;
    }
    if devObj.is_null() || (devObj as usize == unsafe { (*serviceRecord).value } as usize) {
        crate::src_devsvc_manager::NotifyServiceStatusLocked(
            devSvcManager,
            serviceRecord as *const crate::types::DevSvcRecord,
            crate::types::SERVIE_STATUS_STOP,
        );
        // Inline DListRemove for &serviceRecord->entry
        unsafe {
            let entry: *mut crate::types::DListHead = &mut (*serviceRecord).entry;
            let prev = (*entry).prev;
            let next = (*entry).next;
            if !prev.is_null() {
                (*prev).next = next;
            }
            if !next.is_null() {
                (*next).prev = prev;
            }
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        removeFlag = true;
    }
    unsafe {
        let _ = OsalMutexUnlock(&mut (*devSvcManager).mutex as *mut crate::types::OsalMutex);
    }

    if removeFlag {
        unsafe {
            DevSvcRecordFreeInstance(serviceRecord);
        }
    } else {
        // HiLogPrint not available, logging side effect is omitted
    }
}

pub extern "C" fn DevSvcManagerGetObject(inst: *mut crate::types::IDevSvcManager, svcName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    let service_key = unsafe { HdfStringMakeHashKey(svcName, 0) };
    let dev_svc_manager = inst as *mut crate::types::DevSvcManager;
    let mut service_record: *mut crate::types::DevSvcRecord = core::ptr::null_mut();

    if svcName.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char,
                b"Get service failed, svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return core::ptr::null_mut();
    }

    unsafe {
        let _ = OsalMutexLock(core::ptr::addr_of_mut!((*dev_svc_manager).mutex));
    }

    service_record = crate::src_devsvc_manager::DevSvcManagerSearchServiceLocked(inst, service_key);
    if !service_record.is_null() {
        unsafe {
            let device_object = (*service_record).value;
            let _ = OsalMutexUnlock(core::ptr::addr_of_mut!((*dev_svc_manager).mutex));
            return device_object;
        }
    }

    unsafe {
        let _ = OsalMutexUnlock(core::ptr::addr_of_mut!((*dev_svc_manager).mutex));
    }

    core::ptr::null_mut()
}

pub extern "C" fn DevSvcManagerListService(serviceNameSet: *mut crate::types::HdfSBuf, deviceClass: crate::types::DeviceClass) {
    let instance = crate::src_devsvc_manager::DevSvcManagerGetInstance();
    let dev_svc_manager = instance as *mut crate::types::DevSvcManager;
    if dev_svc_manager.is_null() {
        let tag = b"devsvc_manager\0";
        let msg = b"failed to list service, devSvcManager is null\0";
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag.as_ptr() as *const ::core::ffi::c_char,
                msg.as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    let services_ptr = unsafe { std::ptr::addr_of_mut!((*dev_svc_manager).services) };
    let mut node = unsafe { (*dev_svc_manager).services.next };
    unsafe {
        let _ = OsalMutexLock(&mut (*dev_svc_manager).mutex);
    }
    while node != services_ptr {
        let record = node as *mut crate::types::DevSvcRecord;
        unsafe {
            if (*record).devClass as crate::types::DeviceClass == deviceClass {
                let _ = HdfSbufWriteString(serviceNameSet, (*record).servName);
            }
            node = (*node).next;
        }
    }
    unsafe {
        let _ = OsalMutexUnlock(&mut (*dev_svc_manager).mutex);
    }
}

pub extern "C" fn DevSvcManagerGetService(inst: *mut crate::types::IDevSvcManager, svcName: *const ::core::ffi::c_char) -> *mut crate::types::HdfObject {
    let deviceObject = crate::src_devsvc_manager::DevSvcManagerGetObject(inst, svcName);
    if deviceObject.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*deviceObject).service as *mut crate::types::HdfObject }
}

unsafe fn write_record_data(reply: *mut crate::types::HdfSBuf, record: *mut crate::types::DevSvcRecord) {
    HdfSbufWriteString(reply, (*record).servName);
    HdfSbufWriteUint16(reply, (*record).devClass);
    HdfSbufWriteUint32(reply, (*record).devId);
}

pub extern "C" fn DevSvcManagerListAllService(inst: *mut crate::types::IDevSvcManager, reply: *mut crate::types::HdfSBuf) {
    let dev_svc_manager = inst as *mut crate::types::DevSvcManager;
    if dev_svc_manager.is_null() || reply.is_null() {
        return;
    }

    unsafe {
        OsalMutexLock(&mut (*dev_svc_manager).mutex);
    }

    let entry_offset = crate::compat::offset_of!(crate::types::DevSvcRecord, entry);

    // narrow unsafe block for DList traversal
    unsafe {
        let mut next = (*dev_svc_manager).services.next;
        let mut record =
            ((next as *const u8).sub(entry_offset)) as *mut crate::types::DevSvcRecord;

        while !core::ptr::eq(
            core::ptr::addr_of!((*record).entry),
            core::ptr::addr_of!((*dev_svc_manager).services),
        ) {
            write_record_data(reply, record);

            next = (*record).entry.next;
            record = ((next as *const u8).sub(entry_offset)) as *mut crate::types::DevSvcRecord;
        }
    }

    unsafe {
        OsalMutexUnlock(&mut (*dev_svc_manager).mutex);
    }
}

pub extern "C" fn DevSvcManagerListServiceByInterfaceDesc(inst: *mut crate::types::IDevSvcManager, interfaceDesc: *const ::core::ffi::c_char, reply: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    let dev_svc_manager = inst as *mut crate::types::DevSvcManager;
    if dev_svc_manager.is_null() || reply.is_null() {
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char, b"failed to list service collection info, parameter is null\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let mut service_names: [*const ::core::ffi::c_char; 16] = [::core::ptr::null(); 16];
    let mut service_num: u32 = 0;
    let mut status: i32 = crate::types::HDF_SUCCESS;

    unsafe {
        OsalMutexLock(core::ptr::addr_of_mut!((*dev_svc_manager).mutex));
    }

    let head: *mut crate::types::DListHead = unsafe { ::core::ptr::addr_of_mut!((*dev_svc_manager).services) };
    let mut entry: *mut crate::types::DListHead = unsafe { (*head).next };

    while entry != head {
        let record = entry as *mut crate::types::DevSvcRecord;
        let (ifc_desc, serv_name) = unsafe {
            ((*record).interfaceDesc, (*record).servName)
        };
        if ifc_desc.is_null() {
            unsafe {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_DEBUG, 0xD002510u32, b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char, b"%{public}s interfacedesc is null\0".as_ptr() as *const ::core::ffi::c_char, serv_name);
            }
            entry = unsafe { (*entry).next };
            continue;
        }

        if service_num >= 16 {
            status = crate::types::HDF_ERR_OUT_OF_RANGE;
            unsafe {
                HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char, b"%{public}s: More than %{public}d services are found, but up to %{public}d services can be returned\0".as_ptr() as *const ::core::ffi::c_char, interfaceDesc, 16i32, 16i32);
            }
            break;
        }

        let cstr1 = unsafe { ::core::ffi::CStr::from_ptr(ifc_desc) };
        let cstr2 = unsafe { ::core::ffi::CStr::from_ptr(interfaceDesc) };
        if cstr1 == cstr2 {
            service_names[service_num as usize] = serv_name;
            service_num += 1;
        }

        entry = unsafe { (*entry).next };
    }

    unsafe {
        OsalMutexUnlock(core::ptr::addr_of_mut!((*dev_svc_manager).mutex));
    }

    unsafe {
        HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_DEBUG, 0xD002510u32, b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char, b"find %{public}u services interfacedesc is %{public}s\0".as_ptr() as *const ::core::ffi::c_char, service_num, interfaceDesc);
    }
    if !unsafe { HdfSbufWriteUint32(reply, service_num) } {
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char, b"failed to write serviceNum to buffer, interfacedesc is %{public}s, serviceNum is %{public}d\0".as_ptr() as *const ::core::ffi::c_char, interfaceDesc, service_num as i32);
        }
        return crate::types::HDF_FAILURE;
    }
    for i in 0..service_num {
        unsafe {
            HdfSbufWriteString(reply, service_names[i as usize]);
        }
    }

    status
}

pub extern "C" fn DevSvcManagerRegsterServListener(inst: *mut crate::types::IDevSvcManager, listenerHolder: *mut crate::types::ServStatListenerHolder) -> ::core::ffi::c_int {
    let devSvcManager = inst as *mut crate::types::DevSvcManager;
    if devSvcManager.is_null() || listenerHolder.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        let _ = crate::compat::OsalMutexLock(&mut (*devSvcManager).mutex as *mut _);
        let entry = &mut (*listenerHolder).node as *mut crate::types::DListHead;
        let head = &mut (*devSvcManager).svcstatListeners as *mut crate::types::DListHead;
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
        crate::src_devsvc_manager::NotifyServiceStatusOnRegisterLocked(
            devSvcManager as *const crate::types::DevSvcManager,
            listenerHolder,
        );
        let _ = crate::compat::OsalMutexUnlock(&mut (*devSvcManager).mutex as *mut _);
    }
    HDF_SUCCESS
}

pub extern "C" fn DevSvcManagerUnregsterServListener(inst: *mut crate::types::IDevSvcManager, listenerHolder: *mut crate::types::ServStatListenerHolder) {
    let dev_svc_manager = inst as *mut crate::types::DevSvcManager;
    if dev_svc_manager.is_null() || listenerHolder.is_null() {
        return;
    }
    unsafe {
        let _ = OsalMutexLock(&mut (*dev_svc_manager).mutex);
        let node = &mut (*listenerHolder).node;
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
        (*node).prev = std::ptr::null_mut();
        (*node).next = std::ptr::null_mut();
        let _ = OsalMutexUnlock(&mut (*dev_svc_manager).mutex);
    }
}

pub extern "C" fn DevSvcManagerConstruct(inst: *mut crate::types::DevSvcManager) -> bool {
    if inst.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510_u32,
                b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: inst is null!\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevSvcManagerConstruct\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return false;
    }
    let dev_svc_mgr_if: *mut crate::types::IDevSvcManager = unsafe { core::ptr::addr_of_mut!((*inst).super_) };
    unsafe {
        (*dev_svc_mgr_if).AddService = Some(crate::src_devsvc_manager::DevSvcManagerAddService);
        (*dev_svc_mgr_if).UpdateService = Some(crate::src_devsvc_manager::DevSvcManagerUpdateService);
        (*dev_svc_mgr_if).SubscribeService = Some(crate::src_devsvc_manager::DevSvcManagerSubscribeService);
        (*dev_svc_mgr_if).UnsubscribeService = None;
        (*dev_svc_mgr_if).RemoveService = Some(crate::src_devsvc_manager::DevSvcManagerRemoveService);
        (*dev_svc_mgr_if).GetService = Some(crate::src_devsvc_manager::DevSvcManagerGetService);
        (*dev_svc_mgr_if).ListAllService = Some(crate::src_devsvc_manager::DevSvcManagerListAllService);
        (*dev_svc_mgr_if).GetObject = Some(crate::src_devsvc_manager::DevSvcManagerGetObject);
        (*dev_svc_mgr_if).RegsterServListener = Some(crate::src_devsvc_manager::DevSvcManagerRegsterServListener);
        (*dev_svc_mgr_if).UnregsterServListener = Some(crate::src_devsvc_manager::DevSvcManagerUnregsterServListener);
        (*dev_svc_mgr_if).ListServiceByInterfaceDesc = Some(crate::src_devsvc_manager::DevSvcManagerListServiceByInterfaceDesc);
    }

    let ret = unsafe { OsalMutexInit(core::ptr::addr_of_mut!((*inst).mutex)) };
    if ret != HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510_u32,
                b"devsvc_manager\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to create device service manager mutex\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return false;
    }

    unsafe {
        let services_ptr = core::ptr::addr_of_mut!((*inst).services);
        (*services_ptr).next = services_ptr;
        (*services_ptr).prev = services_ptr;

        let svcstat_listeners_ptr = core::ptr::addr_of_mut!((*inst).svcstatListeners);
        (*svcstat_listeners_ptr).next = svcstat_listeners_ptr;
        (*svcstat_listeners_ptr).prev = svcstat_listeners_ptr;
    }
    true
}

pub extern "C" fn DevSvcManagerStartService() -> ::core::ffi::c_int {
    let svcmgr = crate::src_devsvc_manager::DevSvcManagerGetInstance();
    if svcmgr.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let ret = unsafe {
        match (*svcmgr).StartService {
            Some(f) => f(svcmgr),
            None => return crate::types::HDF_SUCCESS,
        }
    };
    ret
}

pub extern "C" fn DevSvcManagerCreate() -> *mut crate::types::HdfObject {
    use std::sync::OnceLock;

    struct SingletonPtr(*mut crate::types::DevSvcManager);
    unsafe impl Send for SingletonPtr {}
    unsafe impl Sync for SingletonPtr {}

    static INSTANCE: OnceLock<SingletonPtr> = OnceLock::new();

    let SingletonPtr(mgr_ptr) = *INSTANCE.get_or_init(|| {
        let layout = std::alloc::Layout::new::<crate::types::DevSvcManager>();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) } as *mut crate::types::DevSvcManager;
        if ptr.is_null() {
            return SingletonPtr(std::ptr::null_mut());
        }
        if !crate::src_devsvc_manager::DevSvcManagerConstruct(ptr) {
            unsafe { std::alloc::dealloc(ptr as *mut u8, layout); }
            return SingletonPtr(std::ptr::null_mut());
        }
        SingletonPtr(ptr)
    });

    if mgr_ptr.is_null() {
        return core::ptr::null_mut();
    }
    unsafe { core::ptr::addr_of_mut!((*mgr_ptr).super_.object) as *mut crate::types::HdfObject }
}

pub extern "C" fn DevSvcManagerRelease(inst: *mut crate::types::IDevSvcManager) {
    if inst.is_null() {
        return;
    }
    let super_offset = crate::compat::offset_of!(crate::types::DevSvcManager, super_);
    let devSvcManager = {
        let offset = super_offset;
        unsafe { (inst as *mut u8).sub(offset) as *mut crate::types::DevSvcManager }
    };
    let record_entry_offset = crate::compat::offset_of!(crate::types::DevSvcRecord, entry);
    unsafe {
        release_all_records(devSvcManager, record_entry_offset);
        crate::compat::OsalMutexDestroy(
            core::ptr::addr_of_mut!((*devSvcManager).mutex) as *mut crate::types::OsalMutex,
        );
    }
}

unsafe fn release_all_records(devSvcManager: *mut crate::types::DevSvcManager, record_entry_offset: usize) {
    let services_ptr: *const crate::types::DListHead =
        core::ptr::addr_of!((*devSvcManager).services);
    let mut record = {
        let next = (*devSvcManager).services.next;
        (next as *mut u8).sub(record_entry_offset) as *mut crate::types::DevSvcRecord
    };
    while core::ptr::addr_of!((*record).entry) != services_ptr {
        let next = {
            let next_entry = (*record).entry.next;
            (next_entry as *mut u8).sub(record_entry_offset) as *mut crate::types::DevSvcRecord
        };
        crate::compat::DevSvcRecordFreeInstance(record);
        record = next;
    }
}

pub extern "C" fn DevSvcManagerGetInstance() -> *mut crate::types::IDevSvcManager {
    unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVSVC_MANAGER.try_into().unwrap())
            as *mut crate::types::IDevSvcManager
    }
}

pub extern "C" fn DevSvcManagerClntSubscribeService(svcName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    // Client-side stub: subscribe to service status changes.
    // The C implementation resides in a client library not present in the translated sources.
    // Returning failure to indicate unsupported operation.
    let _ = svcName;
    let _ = callback;
    crate::types::HDF_FAILURE
}

pub extern "C" fn DevSvcManagerClntUnsubscribeService(svcName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    // Client-side stub: unsubscribe from service status changes.
    let _ = svcName;
    crate::types::HDF_FAILURE
}
