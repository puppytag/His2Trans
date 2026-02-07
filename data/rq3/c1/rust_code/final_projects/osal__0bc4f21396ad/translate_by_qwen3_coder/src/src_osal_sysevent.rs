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

fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier {
    static mut hdfSysEventNotifier: *mut crate::types::HdfSysEventNotifier = std::ptr::null_mut();
    
    unsafe {
        if !hdfSysEventNotifier.is_null() {
            return hdfSysEventNotifier;
        }
        
        let notifier = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::HdfSysEventNotifier>() as u32
        ) as *mut crate::types::HdfSysEventNotifier;
        
        if notifier.is_null() {
            return std::ptr::null_mut();
        }
        
        let ret = crate::compat::OsalMutexInit(&mut (*notifier).mutex as *mut _ as *mut crate::compat::OsalMutex);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // Inline DListHeadInit - DListHead is opaque, use pointer arithmetic
        let head_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut *mut ::core::ffi::c_void;
        let head_as_void = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut ::core::ffi::c_void;
        // next is at offset 0, prev is at offset sizeof(pointer)
        *head_ptr = head_as_void;
        *(head_ptr.add(1)) = head_as_void;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_2
// c_function: FinishEvent
// rust_file: src_osal_sysevent.rs
// rust_signature: fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32
// c_first_line: static int FinishEvent(struct HdfIoService *service, const struct HdfSysEvent *event)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C1/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `KEVENT_COMPLETE_EVENT` in this scope
//     --> src/src_osal_sysevent.rs:71:32
//      |
//      |                                ^^^^^^^^^^^^^^^^^^^^^
//   error[E0425]: cannot find value `LOG_CORE` in this scope
//     --> src/src_osal_sysevent.rs:79:17
//      |
//      |                 ^^^^^^^^ not found in this scope
// =================================
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::FinishEvent(service as _, event as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C1/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
 * ------------------------------------------------------------
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let sbuf = unsafe { HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
    
    if sbuf.is_null() {
        return HDF_ERR_MALLOC_FAIL as i32;
    }
    
    let sync_token = unsafe { (*event).syncToken };
    if unsafe { HdfSbufWriteUint64(sbuf, sync_token) } == false {
        unsafe { HdfSbufRecycle(sbuf) };
        return HDF_FAILURE as i32;
    }
    
    let dispatcher = unsafe { (*service).dispatcher };
    let dispatch_fn = unsafe { (*dispatcher).Dispatch };
    let object_ptr = unsafe { &mut (*service).object as *mut HdfObject };
    
    let ret = if let Some(f) = dispatch_fn {
        unsafe { f(object_ptr, KEVENT_COMPLETE_EVENT!() as i32, sbuf, std::ptr::null_mut()) }
    } else {
        HDF_FAILURE as i32
    };
    
    if ret != HDF_SUCCESS as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            );
        }
    }
    
    unsafe { HdfSbufRecycle(sbuf) };
    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    
    const HDF_ERR_INVALID_PARAM: i32 = 0x80000003u32 as i32;
    const HDF_ERR_INVALID_OBJECT: i32 = 0x80000004u32 as i32;
    const HDF_FAILURE: i32 = 0x80000001u32 as i32;
    const HDF_SUCCESS: i32 = 0;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const LOG_CORE: crate::compat::LogType = 3;
    const LOG_ERROR: crate::compat::LogLevel = 6;
    
    let notifier: *mut crate::types::HdfSysEventNotifier = unsafe { (*listener).priv_ as *mut crate::types::HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let mut receivedEvent: *mut crate::types::HdfSysEvent = std::ptr::null_mut();
    let mut receivedEventLen: u32 = 0;
    
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *mut crate::types::HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen
        )
    };
    
    if !read_ok || receivedEventLen != std::mem::size_of::<crate::types::HdfSysEvent>() as u32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8
            );
        }
        return HDF_FAILURE;
    }
    
    let mut eventContent: *const i8 = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    
    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
    }
    
    // DListHead is opaque - we need to use raw pointer arithmetic
    // The list structure: each node has a DListHead at offset listNode
    // We iterate by treating the memory as raw pointers
    unsafe {
        let list_head_ptr = std::ptr::addr_of!((*notifier).notifyNodeList) as *mut ::core::ffi::c_void;
        // First pointer in DListHead is 'next'
        let mut current = *(list_head_ptr as *mut *mut ::core::ffi::c_void);
        
        while current != list_head_ptr {
            let offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
            let notifyNode = (current as *mut u8).wrapping_sub(offset) as *mut crate::types::HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(callback) = (*notifyNode).callback {
                    let _ = callback(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent
                    );
                }
            }
            // Move to next: first field of current DListHead is 'next'
            current = *(current as *mut *mut ::core::ffi::c_void);
        }
        
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent as *const crate::types::HdfSysEvent);
        }
        
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexUnlock(mutex_ptr);
    }
    
    HDF_SUCCESS
}

fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    const LOG_CORE: u32 = 3;
    const LOG_ERROR: u32 = 6;
    const HDF_DEV_ERR_NO_DEVICE: i32 = -207;
    const HDF_SUCCESS: i32 = 0;
    
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return HDF_DEV_ERR_NO_DEVICE;
        }

        extern "C" fn on_kevent_received_wrapper(
            listener: *mut crate::types::HdfDevEventlistener,
            service: *mut crate::types::HdfIoService,
            id: u32,
            data: *mut crate::types::HdfSBuf,
        ) -> i32 {
            OnKEventReceived(listener, service, id, data)
        }

        (*notifier).ioServiceListener.onReceive = Some(on_kevent_received_wrapper);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        
        if ret != HDF_SUCCESS {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
            (*notifier).keventIoService = std::ptr::null_mut();
        }

        ret
    }
}

fn DeInitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) {
    unsafe {
        if notifier.is_null() {
            return;
        }
        
        let kevent_io_service = (*notifier).keventIoService;
        if kevent_io_service.is_null() {
            return;
        }
        
        let _ = crate::compat::HdfDeviceUnregisterEventListener(
            kevent_io_service,
            &mut (*notifier).ioServiceListener as *mut _
        );
        crate::compat::HdfIoServiceRecycle(kevent_io_service);
        (*notifier).keventIoService = std::ptr::null_mut();
    }
}

pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return -6i32;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline implementation
        let entry_ptr = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let head_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*entry_ptr).next = head_ptr;
        (*entry_ptr).prev = (*head_ptr).prev;
        (*(*head_ptr).prev).next = entry_ptr;
        (*head_ptr).prev = entry_ptr;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline implementation
            let entry_ptr2 = std::ptr::addr_of_mut!((*notifierNode).listNode);
            (*(*entry_ptr2).prev).next = (*entry_ptr2).next;
            (*(*entry_ptr2).next).prev = (*entry_ptr2).prev;
            (*entry_ptr2).prev = std::ptr::null_mut();
            (*entry_ptr2).next = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(mutex_ptr);
        
        ret
    }
}

pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return;
    }

    unsafe {
        OsalMutexLock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut _);

        // DListRemove inline implementation
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        // DListIsEmpty inline implementation
        let head = std::ptr::addr_of!((*notifier).notifyNodeList);
        let is_empty = (*head).next == head as *mut _;
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut _);
    }
}
