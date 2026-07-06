//! Module: src_devsvc_manager_ext
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

fn DListHeadInit(head: *mut crate::types::DListHead) {
    unsafe {
        (*head).next = head;
        (*head).prev = head;
    }
}

fn DListIsEmpty(head: *const crate::types::DListHead)-> bool {
    unsafe { (*(head as *mut crate::types::DListHead)).next == head as *mut crate::types::DListHead }
}

fn DListRemove(entry: *mut crate::types::DListHead) {
    unsafe {
        let prev = (*entry).prev;
        let next = (*entry).next;
        if !prev.is_null() {
            (*prev).next = next;
        }
        if !next.is_null() {
            (*next).prev = prev;
        }
        (*entry).next = entry;
        (*entry).prev = entry;
    }
}

fn DListInsertHead(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        (*entry).next = (*head).next;
        (*entry).prev = head;
        (*(*head).next).prev = entry;
        (*head).next = entry;
    }
}

fn DListInsertTail(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
    }
}

fn DListMerge(list: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        if (*list).next == list {
            return;
        }
        let list_first = (*list).next;
        let list_last = (*list).prev;
        let head_last = (*head).prev;

        (*head_last).next = list_first;
        (*list_first).prev = head_last;
        (*head).prev = list_last;
        (*list_last).next = head;

        DListHeadInit(list);
    }
}

fn DListGetCount(head: *const crate::types::DListHead)-> i32 {
    let mut count: i32 = 0;
    unsafe {
        let mut node = (*(head as *mut crate::types::DListHead)).next;
        while node != head as *mut crate::types::DListHead {
            count += 1;
            node = (*node).next;
        }
    }
    count
}

fn __CPU_AND_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

fn __CPU_OR_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

fn __CPU_XOR_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

fn HdfServiceInfoInit(info: *mut crate::types::HdfServiceInfo, devNode: *const crate::types::HdfDeviceNode) {
    unimplemented!()
}

pub extern "C" fn DeviceSvcMgrOpen(client: *mut crate::types::HdfDeviceIoClient)-> i32 {
    let _ = client;
    crate::types::HDF_SUCCESS
}

fn DevSvcManagerExtRegisterListener(client: *mut crate::types::HdfDeviceIoClient, data: *mut crate::types::HdfSBuf)-> i32 {
    let mut devClass: u16 = crate::types::DEVICE_CLASS_MAX as u16;
    if client.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        if (*client).device.is_null() || (*(*client).device).priv_.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
    }
    let svc_mgr = crate::src_devsvc_manager::DevSvcManagerGetInstance();
    if !unsafe { HdfSbufReadUint16(data, &mut devClass) } {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let holder = crate::src_servstat_listener_holder::ServStatListenerHolderGet(client as usize as u64);
    if !holder.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devsvc_mger\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s:register listener exist, update and return\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevSvcManagerExtRegisterListener\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*holder).listenClass = devClass;
        }
        return crate::types::HDF_SUCCESS;
    }
    let holder_new = crate::src_servstat_listener_holder::ServStatListenerHolderCreate(client as usize, devClass);
    if holder_new.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    crate::src_devsvc_manager::DevSvcManagerRegsterServListener(svc_mgr, holder_new)
}

fn DevSvcManagerExtUnRegisterListener(client: *mut crate::types::HdfDeviceIoClient)-> i32 {
    if client.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let device = unsafe { (*client).device };
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let priv_ptr = unsafe { (*device).priv_ };
    if priv_ptr.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let svcmgr_inst = priv_ptr as *mut crate::types::DevSvcManagerExt;
    let mutex_ptr = unsafe { core::ptr::addr_of_mut!((*svcmgr_inst).mutex) as *mut crate::types::OsalMutex };
    unsafe { crate::compat::OsalMutexLock(mutex_ptr); }

    let holder = crate::src_servstat_listener_holder::ServStatListenerHolderGet(client as usize as u64);
    if holder.is_null() {
        unsafe { crate::compat::OsalMutexUnlock(mutex_ptr); }
        return crate::types::HDF_DEV_ERR_NO_DEVICE_SERVICE;
    }

    let holder_in_list = unsafe { !(*holder).node.next.is_null() };
    if holder_in_list {
        let inst = svcmgr_inst as *mut crate::types::IDevSvcManager;
        crate::src_devsvc_manager::DevSvcManagerUnregsterServListener(inst, holder);
    }

    crate::src_servstat_listener_holder::ServStatListenerHolderRelease(holder);
    unsafe { crate::compat::OsalMutexUnlock(mutex_ptr); }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn DeviceSvcMgrDispatch(service: *mut crate::types::HdfObject, cmdId: i32, data: *mut crate::types::HdfSBuf, reply: *mut crate::types::HdfSBuf)-> i32 {
    if service.is_null() || data.is_null() || reply.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let mut ioClientPtr: u64 = 0;
    unsafe {
        if !HdfSbufReadUint64(reply, &mut ioClientPtr as *mut u64) || ioClientPtr == 0 {
            return crate::types::HDF_FAILURE;
        }
        HdfSbufFlush(reply);
    }
    let client = ioClientPtr as *mut crate::types::HdfDeviceIoClient;

    if cmdId == crate::types::SVCMGR_REGISTER_LISTENER as i32 {
        return DevSvcManagerExtRegisterListener(client, data);
    } else if cmdId == crate::types::SVCMGR_UNREGISTER_LISTENER as i32 {
        return DevSvcManagerExtUnRegisterListener(client);
    }

    return crate::types::HDF_ERR_NOT_SUPPORT;
}

pub extern "C" fn DeviceSvcMgrRelease(client: *mut crate::types::HdfDeviceIoClient) {
    if client.is_null() {
        return;
    }
    let _ = crate::src_devsvc_manager_ext::DevSvcManagerExtUnRegisterListener(client);
}

/// Unsafe helper: setup IDeviceIoService and HdfDeviceObject static fields with raw pointer dereference.
/// # Safety
/// Caller must ensure inst is a valid pointer to DevSvcManagerExt.
unsafe fn setup_io_service(inst: *mut crate::types::DevSvcManagerExt) -> *mut crate::types::HdfIoService {
    let svcmgr_dev_obj_ptr = core::ptr::addr_of_mut!(DevSvcManagerExtStart_svcmgrDevObj);
    let svcmgr_io_service_ptr = core::ptr::addr_of_mut!(DevSvcManagerExtStart_svcmgrIoService);

    (*svcmgr_io_service_ptr).Open = Some(crate::src_devsvc_manager_ext::DeviceSvcMgrOpen);
    (*svcmgr_io_service_ptr).Dispatch = None;
    (*svcmgr_io_service_ptr).Release = Some(crate::src_devsvc_manager_ext::DeviceSvcMgrRelease);

    (*svcmgr_dev_obj_ptr).service = svcmgr_io_service_ptr;
    (*svcmgr_dev_obj_ptr).property = ::core::ptr::null_mut();
    (*svcmgr_dev_obj_ptr).deviceClass = DEVICE_CLASS_DEFAULT;
    (*svcmgr_dev_obj_ptr).priv_ = inst as *mut ::core::ffi::c_void;

    let serv_raw = HdfIoServicePublish(
        DEV_SVCMGR_NODE.as_ptr() as *const ::core::ffi::c_char,
        0o660,
    );
    (*inst).serv = serv_raw as *mut _;

    if serv_raw.is_null() {
        let _ = HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xD002510u32,
            b"devsvc_mger\0".as_ptr() as *const ::core::ffi::c_char,
            b"failed to pushlish svcmgr ioservice\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        static mut DISPATCHER: crate::types::HdfIoDispatcher = crate::types::HdfIoDispatcher {
            Dispatch: None,
        };
        DISPATCHER.Dispatch = Some(crate::src_devsvc_manager_ext::DeviceSvcMgrDispatch);
        let dispatcher_ptr = core::ptr::addr_of_mut!(DISPATCHER) as *mut crate::types::HdfIoDispatcher;
        (*serv_raw).dispatcher = dispatcher_ptr as *mut _;
        (*serv_raw).target = svcmgr_dev_obj_ptr as *mut _;
    }

    serv_raw
}

pub extern "C" fn DevSvcManagerExtStart(svcmgr: *mut crate::types::IDevSvcManager)-> i32 {
    let inst = svcmgr as *mut crate::types::DevSvcManagerExt;
    if inst.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }

    let started = unsafe { (*inst).started };
    if started {
        return HDF_SUCCESS;
    }

    let serv_raw = unsafe { setup_io_service(inst) };
    // Note: logging for failure is inside setup_io_service; we still call Init regardless.
    crate::src_servstat_listener_holder::ServStatListenerHolderinit();
    unsafe { (*inst).started = true; }
    HDF_SUCCESS
}

fn DevSvcManagerExtConstruct(inst: *mut crate::types::DevSvcManagerExt)-> bool {
    if inst.is_null() {
        return false;
    }
    // DevSvcManager super is at offset 0, so cast directly (safe ptr cast)
    let super_ptr = inst as *mut crate::types::DevSvcManager;
    if !crate::src_devsvc_manager::DevSvcManagerConstruct(super_ptr) {
        return false;
    }

    // mutex field follows super, offset = size_of::<DevSvcManager>()
    let mutex_offset = std::mem::size_of::<crate::types::DevSvcManager>();
    let mutex_ptr = unsafe { (inst as *mut u8).add(mutex_offset) as *mut crate::types::OsalMutex };
    if unsafe { OsalMutexInit(mutex_ptr) } != crate::types::HDF_SUCCESS {
        return false;
    }

    // The IDevSvcManager base is also at offset 0 (DevSvcManager starts with it)
    let svc_iface_ptr = inst as *mut crate::types::IDevSvcManager;
    unsafe {
        // Set the StartService callback
        (*svc_iface_ptr).StartService = Some(DevSvcManagerExtStart);
    }
    true
}

pub extern "C" fn DevSvcManagerExtCreate()-> *mut crate::types::HdfObject {
    use std::sync::OnceLock;

    struct SingletonPtr(*mut crate::types::DevSvcManagerExt);
    unsafe impl Send for SingletonPtr {}
    unsafe impl Sync for SingletonPtr {}

    static INSTANCE: OnceLock<SingletonPtr> = OnceLock::new();

    let SingletonPtr(ptr) = *INSTANCE.get_or_init(|| {
        let size = std::mem::size_of::<crate::types::DevSvcManagerExt>();
        let instance = unsafe { OsalMemCalloc(size as u32) } as *mut crate::types::DevSvcManagerExt;
        if !crate::src_devsvc_manager_ext::DevSvcManagerExtConstruct(instance) {
            unsafe { OsalMemFree(instance as *mut ::core::ffi::c_void); }
            return SingletonPtr(std::ptr::null_mut());
        }
        SingletonPtr(instance)
    });

    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { core::ptr::addr_of_mut!((*ptr).super_.super_.object) as *mut crate::types::HdfObject }
}

pub extern "C" fn DevSvcManagerExtRelease(inst: *mut crate::types::IDevSvcManager) {
    let instance = inst as *mut crate::types::DevSvcManagerExt;
    if instance.is_null() {
        return;
    }
    unsafe {
        let serv_ptr = (*instance).serv;
        if !serv_ptr.is_null() {
            crate::compat::HdfIoServiceRemove(serv_ptr as *mut crate::types::HdfIoService);
            (*instance).serv = std::ptr::null_mut();
        }
        crate::src_devsvc_manager::DevSvcManagerRelease(inst);
        (*instance).started = false;
    }
}
