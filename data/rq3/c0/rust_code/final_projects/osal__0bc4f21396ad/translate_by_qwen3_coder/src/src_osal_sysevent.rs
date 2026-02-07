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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_1
// c_function: HdfSysEventNotifierGetInstance
// rust_file: src_osal_sysevent.rs
// rust_signature: fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier
// c_first_line: static struct HdfSysEventNotifier *HdfSysEventNotifierGetInstance(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_1/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//     --> src/src_osal_sysevent.rs:31:48
//      |
//      |                   ---------------------------- ^^^^^^^^^^^^^^^^^^^^^^ expected `*mut OsalMutex`, found `&mut OsalMutex`
//      |                   |
//      |                   arguments to this function are incorrect
//      |
//     --> src/__c2r_generated/tu_types_src_osal_sysevent.rs:93:1
// =================================
fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::HdfSysEventNotifierGetInstance() as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_1
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_1/translated_rust.rs
 * ------------------------------------------------------------
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
        
        let ret = crate::compat::OsalMutexInit(&mut (*notifier).mutex);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // Inline DListHeadInit
        (*notifier).notifyNodeList.next = &mut (*notifier).notifyNodeList as *mut crate::types::DListHead;
        (*notifier).notifyNodeList.prev = &mut (*notifier).notifyNodeList as *mut crate::types::DListHead;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_2
// c_function: FinishEvent
// rust_file: src_osal_sysevent.rs
// rust_signature: fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32
// c_first_line: static int FinishEvent(struct HdfIoService *service, const struct HdfSysEvent *event)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `LOG_CORE` in module `crate::compat`
//      --> src/src_osal_sysevent.rs:107:32
//       |
//       |                                ^^^^^^^^ not found in `crate::compat`
//       |
//   help: consider importing this constant
//       |
//       |
// =================================
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::FinishEvent(service as _, event as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_2
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_2/translated_rust.rs
 * ------------------------------------------------------------
fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    unsafe {
        let sbuf = crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32);
        
        if sbuf.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL as i32;
        }
        
        let sync_token = (*event).syncToken;
        if crate::compat::HdfSbufWriteUint64(sbuf, sync_token) == false {
            crate::compat::HdfSbufRecycle(sbuf);
            return crate::types::HDF_FAILURE as i32;
        }
        
        let dispatcher = (*service).dispatcher;
        if dispatcher.is_null() {
            crate::compat::HdfSbufRecycle(sbuf);
            return crate::types::HDF_FAILURE as i32;
        }
        
        let dispatch_fn = (*dispatcher).Dispatch;
        let ret = if let Some(f) = dispatch_fn {
            f(&mut (*service).object as *mut _, 1, sbuf, std::ptr::null_mut())
        } else {
            crate::types::HDF_FAILURE as i32
        };
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE,
                crate::compat::LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            );
        }
        
        crate::compat::HdfSbufRecycle(sbuf);
        ret
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_3
// c_function: OnKEventReceived
// rust_file: src_osal_sysevent.rs
// rust_signature: fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32
// c_first_line: static int OnKEventReceived(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error[E0606]: casting `&mut __c2r_tu_types_src_osal_sysevent::OsalMutex` as `*mut compat::OsalMutex` is invalid
//      --> src/src_osal_sysevent.rs:207:23
//       |
//       |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   error[E0606]: casting `&__c2r_tu_types_src_osal_sysevent::DListHead` as `*const types::DListHead` is invalid
//      --> src/src_osal_sysevent.rs:211:30
//       |
//       |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    const HDF_ERR_INVALID_PARAM: i32 = 3;
    const HDF_ERR_INVALID_OBJECT: i32 = 4;
    const HDF_FAILURE: i32 = 1;
    const HDF_SUCCESS: i32 = 0;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const LOG_CORE: LogType = 3;
    const LOG_ERROR: LogLevel = 6;
    
    if listener.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    let notifier: *mut HdfSysEventNotifier = unsafe { (*listener).priv_ as *mut HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
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
        return HDF_FAILURE;
    }
    
    let mut eventContent: *const i8 = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    
    unsafe {
        OsalMutexLock(&mut (*notifier).mutex as *mut OsalMutex);
    }
    
    // Iterate through the notify node list
    let list_head = unsafe { &(*notifier).notifyNodeList as *const DListHead };
    let mut current = unsafe { (*list_head).next };
    
    while current != list_head as *mut DListHead {
        let offset = std::mem::offset_of!(HdfSysEventNotifyNode, listNode);
        let notifyNode = (current as *mut u8).wrapping_sub(offset) as *mut HdfSysEventNotifyNode;
        
        unsafe {
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
    }
    
    unsafe {
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }
        
        OsalMutexUnlock(&mut (*notifier).mutex as *mut OsalMutex);
    }
    
    HDF_SUCCESS
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_4
// c_function: InitKeventIoServiceListenerLocked
// rust_file: src_osal_sysevent.rs
// rust_signature: fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32
// c_first_line: static int InitKeventIoServiceListenerLocked(struct HdfSysEventNotifier *notifier)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `LOG_CORE` in module `crate::types`
//      --> src/src_osal_sysevent.rs:163:31
//       |
//       |                               ^^^^^^^^ not found in `crate::types`
//       |
//   help: consider importing this constant
//       |
//       |
// =================================
fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_4
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
 * ------------------------------------------------------------
fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_DEV_ERR_NO_DEVICE as i32;
        }

        (*notifier).ioServiceListener.onReceive = Some(OnKEventReceived);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_6
// c_function: HdfSysEventNotifyRegister
// rust_file: src_osal_sysevent.rs
// rust_signature: pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32
// c_first_line: int32_t HdfSysEventNotifyRegister(struct HdfSysEventNotifyNode *notifierNode, uint64_t classSet)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_6/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `HDF_DEV_ERR_NO_MEMORY` in module `crate::types`
//      --> src/src_osal_sysevent.rs:376:30
//       |
//       |                              ^^^^^^^^^^^^^^^^^^^^^ not found in `crate::types`
//       |
//   help: consider importing this constant
//       |
//       |
// =================================
pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::HdfSysEventNotifyRegister(notifierNode as _, classSet as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_6
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_6/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return crate::types::HDF_DEV_ERR_NO_MEMORY as i32;
    }

    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);
        
        // DListInsertTail inline implementation
        let entry = &mut (*notifierNode).listNode;
        let head = &mut (*notifier).notifyNodeList;
        (*entry).next = head as *mut _;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry as *mut _;
        (*head).prev = entry as *mut _;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline implementation
            let entry = &mut (*notifierNode).listNode;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(&mut (*notifier).mutex);
        
        ret
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_6
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_osal_sysevent_7
// c_function: HdfSysEventNotifyUnregister
// rust_file: src_osal_sysevent.rs
// rust_signature: pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode)
// c_first_line: void HdfSysEventNotifyUnregister(struct HdfSysEventNotifyNode *notifierNode)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_7/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_osal_sysevent.rs:452:23
//       |
//       |         ------------- ^^^^^^^^^^^^^^^^^^^^^^ expected `*mut OsalMutex`, found `&mut OsalMutex`
//       |         |
//       |         arguments to this function are incorrect
//       |
//      --> src/__c2r_generated/tu_types_src_osal_sysevent.rs:93:1
// =================================
pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::HdfSysEventNotifyUnregister(notifierNode as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_7
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return;
    }

    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);

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

        OsalMutexUnlock(&mut (*notifier).mutex);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_osal_sysevent_7
 * === C2R_LLM_FAILED_OUTPUT_END === */

