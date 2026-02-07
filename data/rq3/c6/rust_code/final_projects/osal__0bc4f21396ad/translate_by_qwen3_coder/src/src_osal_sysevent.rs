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
        
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let ret = crate::compat::OsalMutexInit(mutex_ptr);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // DListHeadInit inline implementation
        // DListHead is opaque, so we use pointer arithmetic
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C6/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `LOG_CORE` in this scope
//     --> src/src_osal_sysevent.rs:81:17
//      |
//      |                 ^^^^^^^^
//   error[E0425]: cannot find value `LOG_ERROR` in this scope
//     --> src/src_osal_sysevent.rs:82:17
//      |
//      |                 ^^^^^^^^^ not found in this scope
// =================================
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::FinishEvent(service as _, event as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C6/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
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
        unsafe { f(object_ptr, 1, sbuf, std::ptr::null_mut()) }
    } else {
        HDF_FAILURE as i32
    };
    
    if ret != HDF_SUCCESS as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE!(),
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


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_3
// c_function: OnKEventReceived
// rust_file: src_osal_sysevent.rs
// rust_signature: fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32
// c_first_line: static int OnKEventReceived(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C6/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `LOG_CORE` in this scope
//      --> src/src_osal_sysevent.rs:147:17
//       |
//       |                 ^^^^^^^^
//   error[E0425]: cannot find value `LOG_ERROR` in this scope
//      --> src/src_osal_sysevent.rs:148:17
//       |
//       |                 ^^^^^^^^^ not found in this scope
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C6/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    const HDF_ERR_INVALID_OBJECT: i32 = -4;
    const HDF_FAILURE: i32 = -1;
    
    unsafe {
        let notifier = (*listener).priv_ as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM as i32;
        }

        if id != 0xFADE {
            return HDF_ERR_INVALID_OBJECT;
        }

        let mut receivedEvent: *const ::core::ffi::c_void = std::ptr::null();
        let mut receivedEventLen: u32 = 0;

        if crate::compat::HdfSbufReadBuffer(data, &mut receivedEvent, &mut receivedEventLen) == false
            || receivedEventLen as usize != std::mem::size_of::<crate::types::HdfSysEvent>()
        {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE!(),
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8,
            );
            return HDF_FAILURE;
        }

        let receivedEvent = receivedEvent as *const crate::types::HdfSysEvent;

        let eventContent = crate::compat::HdfSbufReadString(data);
        let eventContent = if eventContent.is_null() {
            b"\0".as_ptr() as *const i8
        } else {
            eventContent
        };

        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        crate::compat::OsalMutexLock(mutex_ptr);

        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        let mut cur = (*head).next;
        while cur != head as *mut crate::types::DListHead {
            let offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
            let notifyNode = (cur as *mut u8).sub(offset) as *mut crate::types::HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(cb) = (*notifyNode).callback {
                    let _ = cb(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent,
                    );
                }
            }
            cur = (*cur).next;
        }

        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }

        crate::compat::OsalMutexUnlock(mutex_ptr);

        HDF_SUCCESS
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


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
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return HDF_DEV_ERR_NO_DEVICE;
        }

        extern "C" fn on_kevent_wrapper(
            listener: *mut crate::types::HdfDevEventlistener,
            service: *mut crate::types::HdfIoService,
            id: u32,
            data: *mut crate::types::HdfSBuf,
        ) -> i32 {
            OnKEventReceived(listener, service, id, data)
        }
        
        (*notifier).ioServiceListener.onReceive = Some(on_kevent_wrapper);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        
        if ret != HDF_SUCCESS {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
            (*notifier).keventIoService = std::ptr::null_mut();
        }

        ret
    }
}

fn DeInitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) {
    if notifier.is_null() {
        return;
    }
    
    unsafe {
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
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return -3i32;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline implementation
        let entry = &mut (*notifierNode).listNode;
        let head = &mut (*notifier).notifyNodeList;
        (*entry).next = head as *mut _;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry as *mut _;
        (*head).prev = entry as *mut _;
        
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != crate::types::HDF_SUCCESS {
            // DListRemove inline implementation
            let entry = &mut (*notifierNode).listNode;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
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
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);

        // DListRemove inline implementation
        let entry = &mut (*notifierNode).listNode;
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        // DListIsEmpty inline implementation
        let head = &(*notifier).notifyNodeList;
        let is_empty = (*head).next == head as *const _ as *mut _;
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(mutex_ptr);
    }
}
