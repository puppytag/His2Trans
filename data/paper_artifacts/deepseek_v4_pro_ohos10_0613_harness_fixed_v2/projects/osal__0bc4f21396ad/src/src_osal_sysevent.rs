//! Module: src_osal_sysevent
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

struct DListHeadRef(*mut DListHead);

impl DListHeadRef {
    fn new(ptr: *mut DListHead) -> Self {
        Self(ptr)
    }

    fn init(&mut self) {
        unsafe {
            (*self.0).next = self.0;
            (*self.0).prev = self.0;
        }
    }

    fn insert_tail(&mut self, entry: *mut DListHead) {
        unsafe {
            let head = self.0;
            (*entry).next = head;
            (*entry).prev = (*head).prev;
            let prev = (*head).prev;
            (*prev).next = entry;
            (*head).prev = entry;
        }
    }

    fn remove(entry: *mut DListHead) {
        unsafe {
            let prev = (*entry).prev;
            let next = (*entry).next;
            (*prev).next = next;
            (*next).prev = prev;
            (*entry).prev = core::ptr::null_mut();
            (*entry).next = core::ptr::null_mut();
        }
    }

    fn is_empty(&self) -> bool {
        unsafe {
            (*self.0).next as *const DListHead == self.0
        }
    }
}

fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier {
    use std::sync::atomic::{AtomicPtr, Ordering};
    static ATOMIC: AtomicPtr<crate::types::HdfSysEventNotifier> = AtomicPtr::new(std::ptr::null_mut());

    let ptr = ATOMIC.load(Ordering::Acquire);
    if !ptr.is_null() {
        return ptr;
    }

    let size = core::mem::size_of::<crate::types::HdfSysEventNotifier>();
    let raw = unsafe { crate::compat::OsalMemCalloc(size) };
    let notifier = raw as *mut crate::types::HdfSysEventNotifier;
    if notifier.is_null() {
        return std::ptr::null_mut();
    }

    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    let ret = unsafe { crate::compat::OsalMutexInit(mutex_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        let notifier_void = notifier as *mut core::ffi::c_void;
        unsafe { crate::compat::OsalMemFree(notifier_void) };
        return std::ptr::null_mut();
    }

    // Initialize DListHead
    let head_ptr = unsafe { core::ptr::addr_of_mut!((*notifier).notifyNodeList) };
    let mut dlist = DListHeadRef::new(head_ptr);
    dlist.init();

    match ATOMIC.compare_exchange(std::ptr::null_mut(), notifier, Ordering::Release, Ordering::Relaxed) {
        Ok(_) => notifier,
        Err(existing) => {
            // Another thread already created the singleton; release ours.
            // Note: OsalMutexDestroy is not available; potential resource leak as in the original C.
            let notifier_void = notifier as *mut core::ffi::c_void;
            unsafe { crate::compat::OsalMemFree(notifier_void) };
            existing
        }
    }
}

fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent)-> i32 {
    let cap = ::std::mem::size_of::<u64>();
    let sbuf: *mut crate::types::HdfSBuf = unsafe { HdfSbufObtain(cap) };
    if sbuf.is_null() {
        return HDF_ERR_MALLOC_FAIL;
    }
    let sync_token = unsafe { (*event).syncToken };
    if !unsafe { HdfSbufWriteUint64(sbuf, sync_token) } {
        unsafe { HdfSbufRecycle(sbuf) };
        return HDF_FAILURE;
    }
    let dispatcher_ptr = unsafe { (*service).dispatcher };
    let dispatch_fn = unsafe { (*dispatcher_ptr).Dispatch };
    let dispatch_fn = dispatch_fn.unwrap();
    let obj_ref = unsafe { &mut (*service).object };
    let object_ptr = obj_ref as *mut _;
    let ret = unsafe { dispatch_fn(object_ptr, 1i32, sbuf, ::std::ptr::null_mut()) };
    if ret != HDF_SUCCESS {
        let log_type = LOG_CORE as crate::compat::LogType;
        let log_level = LOG_ERROR as crate::compat::LogLevel;
        let domain = 0xD002510u32;
        let tag = b"usysevent\0".as_ptr() as *const core::ffi::c_char;
        let fmt = b"failed to finish sysevent, %{public}d\0".as_ptr() as *const core::ffi::c_char;
        let _ = unsafe { HiLogPrint(log_type, log_level, domain, tag, fmt, ret) };
    }
    unsafe { HdfSbufRecycle(sbuf) };
    ret
}

extern "C" fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    let priv_data = unsafe { (*listener).priv_ };
    let notifier = priv_data as *mut crate::types::HdfSysEventNotifier;
    if notifier.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    if id != 0xFADEu32 {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let mut receivedEvent_ptr: *const core::ffi::c_void = core::ptr::null();
    let mut receivedEventLen: u32 = 0;
    if !(unsafe { HdfSbufReadBuffer(data, &mut receivedEvent_ptr, &mut receivedEventLen) }) ||
        receivedEventLen != core::mem::size_of::<crate::types::HdfSysEvent>() as u32
    {
        let log_type = crate::types::LOG_CORE.try_into().unwrap();
        let log_level = crate::types::LOG_ERROR.try_into().unwrap();
        let domain = 0xD002510u32;
        let tag = b"usysevent\0".as_ptr() as *const core::ffi::c_char;
        let fmt = b"failed to read kevent object\0".as_ptr() as *const core::ffi::c_char;
        unsafe { HiLogPrint(log_type, log_level, domain, tag, fmt); }
        return crate::types::HDF_FAILURE;
    }
    let receivedEvent = receivedEvent_ptr as *const crate::types::HdfSysEvent;
    let eventContent_ptr = unsafe { HdfSbufReadString(data) };
    let eventContent: *const core::ffi::c_char = if eventContent_ptr.is_null() {
        b"\0".as_ptr() as *const core::ffi::c_char
    } else {
        eventContent_ptr
    };
    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    unsafe { OsalMutexLock(mutex_ptr) };
    let offset = {
        let uninit = core::mem::MaybeUninit::<crate::types::HdfSysEventNotifyNode>::uninit();
        let base = uninit.as_ptr();
        let field_ptr = unsafe { core::ptr::addr_of!((*base).listNode) };
        (field_ptr as *const u8 as usize) - (base as *const u8 as usize)
    };
    let head = unsafe { std::ptr::addr_of!((*notifier).notifyNodeList) } as *const u8;
    let next = unsafe { (*notifier).notifyNodeList.next };
    let mut current_list_node_ptr = next as *const u8;
    let event_ref = unsafe { &*receivedEvent };
    while current_list_node_ptr != head {
        let notify_node = ((current_list_node_ptr as usize).wrapping_sub(offset)) as *const crate::types::HdfSysEventNotifyNode;
        let class_filter = unsafe { (*notify_node).classFilter };
        if (event_ref.eventClass & class_filter) != 0 {
            let cb = unsafe { (*notify_node).callback };
            if let Some(cb) = cb {
                {
                    let node_ptr = notify_node as *mut crate::types::HdfSysEventNotifyNode;
                    let ec = event_ref.eventClass;
                    let eid = event_ref.eventid;
                    unsafe { cb(node_ptr, ec, eid, eventContent); }
                }
            }
        }
        let next = unsafe { (*notify_node).listNode.next };
        current_list_node_ptr = next as *const u8;
    }
    let sync_token = unsafe { (*receivedEvent).syncToken };
    if sync_token != 0 {
        FinishEvent(service, receivedEvent);
    }
    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    unsafe { OsalMutexUnlock(mutex_ptr) };
    crate::types::HDF_SUCCESS
}

fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    let service = unsafe { (*notifier).keventIoService };
    if service.is_null() {
    let service_name = b"hdf_kevent\0".as_ptr() as *const core::ffi::c_char;
    let service = unsafe { crate::compat::HdfIoServiceBind(service_name) };
        if service.is_null() {
            return crate::types::HDF_DEV_ERR_NO_DEVICE;
        }
        unsafe { (*notifier).keventIoService = service; }
    }
    let on_receive: unsafe extern "C" fn(*mut HdfDevEventlistener, *mut HdfIoService, u32, *mut HdfSBuf) -> i32 = crate::src_osal_sysevent::OnKEventReceived;
    let cb = Some(on_receive);
    unsafe { (*notifier).ioServiceListener.onReceive = cb; }
    let priv_data = notifier as *mut ::core::ffi::c_void;
    unsafe { (*notifier).ioServiceListener.priv_ = priv_data; }
    let listener_ref = unsafe { &mut (*notifier).ioServiceListener };
    let listener_ptr = listener_ref as *mut crate::types::HdfDevEventlistener;
    let ret = unsafe { crate::compat::HdfDeviceRegisterEventListener((*notifier).keventIoService, listener_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        unsafe { crate::compat::HdfIoServiceRecycle((*notifier).keventIoService); }
        unsafe { (*notifier).keventIoService = ::core::ptr::null_mut(); }
    }
    ret
}

fn DeInitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) {
    let service = unsafe { (*notifier).keventIoService };
    if service.is_null() {
        return;
    }
    let listener_ref = unsafe { &mut (*notifier).ioServiceListener };
    let listener_ptr = listener_ref as *mut crate::types::HdfDevEventlistener;
    let _ = unsafe { HdfDeviceUnregisterEventListener(service, listener_ptr) };
    let service = unsafe { (*notifier).keventIoService };
    unsafe { HdfIoServiceRecycle(service); }
    unsafe { (*notifier).keventIoService = core::ptr::null_mut(); }
}

pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return HDF_DEV_ERR_NO_MEMORY;
    }
    let mutex_ptr: *mut OsalMutex = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    let _ = unsafe { OsalMutexLock(mutex_ptr) };

    {
        let entry_ptr: *mut DListHead = unsafe { std::ptr::addr_of_mut!((*notifierNode).listNode) };
        let head_ptr: *mut DListHead = unsafe { std::ptr::addr_of_mut!((*notifier).notifyNodeList) };
        let mut dlist = DListHeadRef::new(head_ptr);
        dlist.insert_tail(entry_ptr);
    }

    unsafe { (*notifierNode).classFilter = classSet; }

    let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
    if ret != HDF_SUCCESS {
        {
            let entry_ptr: *mut DListHead = unsafe { std::ptr::addr_of_mut!((*notifierNode).listNode) };
            DListHeadRef::remove(entry_ptr);
        }
    }

    let _ = unsafe { OsalMutexUnlock(mutex_ptr) };
    ret
}

pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return;
    }
    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    unsafe { OsalMutexLock(mutex_ptr) };

    {
        let list_node = unsafe { std::ptr::addr_of_mut!((*notifierNode).listNode) };
        DListHeadRef::remove(list_node);
    }

    let notify_list = unsafe { std::ptr::addr_of_mut!((*notifier).notifyNodeList) };
    let dlist = DListHeadRef::new(notify_list);
    let is_empty = dlist.is_empty();
    if is_empty {
        crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
    }

    let mutex_ptr = unsafe { std::ptr::addr_of_mut!((*notifier).mutex) };
    unsafe { OsalMutexUnlock(mutex_ptr) };
}
