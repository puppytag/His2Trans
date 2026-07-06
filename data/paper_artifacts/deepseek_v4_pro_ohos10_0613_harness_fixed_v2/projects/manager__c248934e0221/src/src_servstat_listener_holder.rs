//! Module: src_servstat_listener_holder
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

/// Private unsafe helper: performs the raw pointer self-referencing assignments.
/// # Safety
/// Caller must ensure `head` is a valid, writable pointer to a DListHead.
unsafe fn DListHeadInit_impl(head: *mut crate::types::DListHead) {
    (*head).next = head;
    (*head).prev = head;
}

fn DListHeadInit(head: *mut crate::types::DListHead) {
    unsafe {
        DListHeadInit_impl(head);
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
    let head_mut = head as *mut crate::types::DListHead;
    let mut node = unsafe { (*head_mut).next };
    let mut count = 0;
    while node != head_mut {
        count += 1;
        node = unsafe { (*node).next };
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

/// Global listener holder list structure matching C's SvcStatListenerHolderList
#[repr(C)]
struct SvcStatListenerHolderList {
    mutex: crate::types::OsalMutex,
    list: crate::types::DListHead,
}

/// Global listener holder list, accessed from multiple C-ABI entry points.
/// Protected by embedded mutex; all read/write operations on list must hold this mutex.
/// # Safety
/// This static mut is used because it must be accessible from multiple `pub extern "C"`
/// entry points at a known address. All accesses are guarded by `g_holoderList.mutex`.
/// Do not replace with `OnceLock` without auditing all C-ABI callers that expect a
/// stable global address for the list head.
static mut g_holoderList: SvcStatListenerHolderList = SvcStatListenerHolderList {
    mutex: crate::types::OsalMutex { realMutex: core::ptr::null_mut() },
    list: crate::types::DListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
};

fn KServStatListenerHolderListInit() {
    unsafe {
        crate::compat::OsalMutexInit(&mut g_holoderList.mutex);
        DListHeadInit(&mut g_holoderList.list);
    }
}

pub extern "C" fn ServStatListenerHolderinit() {
    crate::src_servstat_listener_holder::KServStatListenerHolderListInit();
}

pub extern "C" fn KServStatListenerHolderNotifyStatus(holder: *mut crate::types::ServStatListenerHolder, status: *mut crate::types::ServiceStatus)-> i32 {
    if holder.is_null() || status.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let holder_inst = unsafe {
        let offset = crate::compat::offset_of!(crate::types::KServStatListenerHolder, holder);
        (holder as *mut u8).sub(offset) as *mut crate::types::KServStatListenerHolder
    };

    let client = unsafe { (*holder_inst).listenerClient };
    if client.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to notify service status, invalid holder\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let data = unsafe { crate::compat::HdfSbufObtainDefaultSize() };
    if data.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to notify service status, oom\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let marsh_ret = unsafe { crate::compat::ServiceStatusMarshalling(status, data) };
    if marsh_ret != crate::types::HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to marshalling service status\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        unsafe {
            crate::compat::HdfSbufRecycle(data);
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let send_ret = unsafe {
        crate::compat::HdfDeviceSendEventToClient(client as *const crate::types::HdfDeviceIoClient, 0, data)
    };
    if send_ret != crate::types::HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to notify service status, send error\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        unsafe {
            crate::compat::HdfSbufRecycle(data);
        }
        return crate::types::HDF_FAILURE;
    }

    unsafe { crate::compat::HdfSbufRecycle(data); }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn KServStatListenerHolderRecycle(holder: *mut crate::types::ServStatListenerHolder) {
    if holder.is_null() {
        return;
    }
    crate::src_servstat_listener_holder::ServStatListenerHolderRelease(holder);
    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD002510u32,
            b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
            b"KServStatListenerHolderRecycle success\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

    // Safety: holder is a valid, non-null pointer.
    unsafe fn init_raw_holder_fields(holder: *mut crate::types::KServStatListenerHolder, client: *mut crate::types::HdfDeviceIoClient, holder_member: *mut crate::types::ServStatListenerHolder) {
        (*holder).listenerClient = client;
        (*holder_member).NotifyStatus = Some(KServStatListenerHolderNotifyStatus);
        (*holder_member).Recycle = Some(KServStatListenerHolderRecycle);

        let node: *mut crate::types::DListHead = core::ptr::addr_of_mut!((*holder).node);
        (*node).next = node;
        (*node).prev = node;
    }

pub extern "C" fn ServStatListenerHolderCreate(listener: usize, listenClass: u16)-> *mut crate::types::ServStatListenerHolder {
    if listener == 0 {
        return core::ptr::null_mut();
    }
    let client = listener as *mut crate::types::HdfDeviceIoClient;
    let holder = unsafe {
        crate::compat::OsalMemCalloc(core::mem::size_of::<crate::types::KServStatListenerHolder>().try_into().unwrap()) as *mut crate::types::KServStatListenerHolder
    };
    if holder.is_null() {
        return core::ptr::null_mut();
    }
    let holder_member = unsafe {
        let hm = core::ptr::addr_of_mut!((*holder).holder);
        (*hm).index = client as u64;
        (*hm).listenClass = listenClass;
        hm
    };
    unsafe { init_raw_holder_fields(holder, client, holder_member); }
    unsafe {
        crate::compat::OsalMutexLock(&mut g_holoderList.mutex);
        DListInsertTail(core::ptr::addr_of_mut!((*holder).node), &mut g_holoderList.list);
        crate::compat::OsalMutexUnlock(&mut g_holoderList.mutex);
    }
    holder_member
}

pub extern "C" fn ServStatListenerHolderGet(index: u64)-> *mut crate::types::ServStatListenerHolder {
    let mut result: *mut crate::types::ServStatListenerHolder = core::ptr::null_mut();
    unsafe {
        crate::compat::OsalMutexLock(&mut g_holoderList.mutex);
        let list_head: *mut crate::types::DListHead = &mut g_holoderList.list;
        let mut node: *mut crate::types::DListHead = (*list_head).next;
        while node != list_head {
            let holder = (node as *mut u8).offset(-(crate::compat::offset_of!(crate::types::KServStatListenerHolder, node) as isize)) as *mut crate::types::KServStatListenerHolder;
            if (*holder).holder.index == index {
                result = core::ptr::addr_of_mut!((*holder).holder);
                break;
            }
            node = (*node).next;
        }
        crate::compat::OsalMutexUnlock(&mut g_holoderList.mutex);
    }
    result
}

pub extern "C" fn ServStatListenerHolderRelease(holder: *mut crate::types::ServStatListenerHolder) {
    if holder.is_null() {
        return;
    }
    unsafe {
        let offset = crate::compat::offset_of!(crate::types::KServStatListenerHolder, holder);
        let holderInst = (holder as *mut u8).offset(-(offset as isize)) as *mut crate::types::KServStatListenerHolder;
        crate::compat::OsalMutexLock(&mut g_holoderList.mutex);
        if !(*holderInst).node.next.is_null() {
            crate::src_servstat_listener_holder::DListRemove(core::ptr::addr_of_mut!((*holderInst).node));
        }
        crate::compat::OsalMutexUnlock(&mut g_holoderList.mutex);
        (*holderInst).listenerClient = ::core::ptr::null_mut();
        crate::compat::OsalMemFree(holderInst as *mut ::core::ffi::c_void);
    }
}
