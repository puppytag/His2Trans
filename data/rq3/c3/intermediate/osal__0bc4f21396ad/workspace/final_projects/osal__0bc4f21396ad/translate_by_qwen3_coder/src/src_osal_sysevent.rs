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
        
        let ret = crate::compat::OsalMutexInit(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // DListHeadInit inline - use raw pointer without type annotation
        let list_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*notifier).notifyNodeList.next = list_ptr;
        (*notifier).notifyNodeList.prev = list_ptr;
        
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
// last_error_truncated:
//   error[E0433]: failed to resolve: could not find `LOG_CORE` in `compat`
//     --> src/src_osal_sysevent.rs:75:32
//      |
//      |                                ^^^^^^^^ could not find `LOG_CORE` in `compat`
//   error[E0433]: failed to resolve: could not find `LOG_ERROR` in `compat`
//     --> src/src_osal_sysevent.rs:76:32
//      |
//      |                                ^^^^^^^^^ could not find `LOG_ERROR` in `compat`
// =================================
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::FinishEvent(service as _, event as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
 * ------------------------------------------------------------
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
    
    if sbuf.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    
    let sync_token = unsafe { (*event).syncToken };
    
    if unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } == false {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return crate::types::HDF_FAILURE as i32;
    }
    
    let dispatcher = unsafe { (*service).dispatcher };
    let dispatch_fn = unsafe { (*dispatcher).Dispatch };
    let object_ptr = unsafe { std::ptr::addr_of_mut!((*service).object) };
    
    let ret = if let Some(f) = dispatch_fn {
        unsafe { f(object_ptr as *mut _, 1, sbuf, std::ptr::null_mut()) }
    } else {
        crate::types::HDF_FAILURE as i32
    };
    
    if ret != crate::types::HDF_SUCCESS as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            )
        };
    }
    
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `HDF_ERR_INVALID_OBJECT` in this scope
//      --> src/src_osal_sysevent.rs:130:16
//       |
//       |                ^^^^^^^^^^^^^^^^^^^^^^
//   error[E0425]: cannot find value `LOG_CORE` in this scope
//      --> src/src_osal_sysevent.rs:147:17
//       |
//       |                 ^^^^^^^^ not found in this scope
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let notifier: *mut HdfSysEventNotifier = unsafe { (*listener).priv_ as *mut HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM as i32;
    }
    
    if id != 0xFADE {
        return HDF_ERR_INVALID_OBJECT!() as i32;
    }
    
    let mut receivedEvent: *const HdfSysEvent = std::ptr::null();
    let mut receivedEventLen: u32 = 0;
    
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *const HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen
        )
    };
    
    if !read_ok || receivedEventLen != std::mem::size_of::<HdfSysEvent>() as u32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8
            );
        }
        return HDF_FAILURE as i32;
    }
    
    let mut eventContent = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    
    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);
    }
    
    // Iterate through the notify node list
    unsafe {
        let list_head = &(*notifier).notifyNodeList;
        let offset = &(*(std::ptr::null::<HdfSysEventNotifyNode>())).listNode as *const _ as usize;
        
        let mut current = (*list_head).next;
        while current != list_head as *const DListHead as *mut DListHead {
            let notifyNode = (current as *mut u8).sub(offset) as *mut HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(cb) = (*notifyNode).callback {
                    let _ = cb(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent
                    );
                }
            }
            
            current = (*current).next;
        }
        
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }
        
        OsalMutexUnlock(&mut (*notifier).mutex);
    }
    
    HDF_SUCCESS as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_4
// c_function: InitKeventIoServiceListenerLocked
// rust_file: src_osal_sysevent.rs
// rust_signature: fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32
// c_first_line: static int InitKeventIoServiceListenerLocked(struct HdfSysEventNotifier *notifier)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
// last_error_truncated:
//   error[E0433]: failed to resolve: could not find `LOG_ERROR` in `types`
//      --> src/src_osal_sysevent.rs:152:31
//       |
//       |                               ^^^^^^^^^ could not find `LOG_ERROR` in `types`
//   error[E0433]: failed to resolve: could not find `LOG_CORE` in `types`
//      --> src/src_osal_sysevent.rs:151:31
//       |
//       |                               ^^^^^^^^ could not find `LOG_CORE` in `types`
// =================================
fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_4
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C3/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
 * ------------------------------------------------------------
extern "C" fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_DEV_ERR_NO_DEVICE!() as i32;
        }

        (*notifier).ioServiceListener.onReceive = Some(OnKEventReceived);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR!(),
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
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_4
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn DeInitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) {
    if notifier.is_null() {
        return;
    }
    
    unsafe {
        if (*notifier).keventIoService.is_null() {
            return;
        }
        
        let _ = crate::compat::HdfDeviceUnregisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut _
        );
        
        crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
        
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
            let entry_ptr = std::ptr::addr_of_mut!((*notifierNode).listNode);
            (*(*entry_ptr).prev).next = (*entry_ptr).next;
            (*(*entry_ptr).next).prev = (*entry_ptr).prev;
            (*entry_ptr).prev = std::ptr::null_mut();
            (*entry_ptr).next = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(mutex_ptr);
        
        ret
    }
}

pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    unimplemented!()
}
