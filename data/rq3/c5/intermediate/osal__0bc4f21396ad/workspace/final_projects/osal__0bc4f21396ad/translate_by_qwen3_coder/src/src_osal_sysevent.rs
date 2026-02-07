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
        
        // DListHeadInit inline implementation
        let list_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*list_ptr).next = list_ptr as *mut _;
        (*list_ptr).prev = list_ptr as *mut _;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}

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
            let log_core: crate::compat::LogType = 3;
            let log_error: crate::compat::LogLevel = 6;
            crate::compat::HiLogPrint(
                log_core,
                log_error,
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_3
// c_function: OnKEventReceived
// rust_file: src_osal_sysevent.rs
// rust_signature: fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32
// c_first_line: static int OnKEventReceived(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error[E0606]: casting `&mut __c2r_tu_types_src_osal_sysevent::OsalMutex` as `*mut compat::OsalMutex` is invalid
//      --> src/src_osal_sysevent.rs:129:23
//       |
//       |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   error[E0606]: casting `&mut __c2r_tu_types_src_osal_sysevent::OsalMutex` as `*mut compat::OsalMutex` is invalid
//      --> src/src_osal_sysevent.rs:156:25
//       |
//       |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    unsafe {
        let notifier = (*listener).priv_ as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM as i32;
        }

        if id != 0xFADE {
            return -3i32;
        }

        let mut receivedEvent: *const crate::types::HdfSysEvent = std::ptr::null();
        let mut receivedEventLen: u32 = 0;

        let read_ok = HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *const crate::types::HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen,
        );

        if !read_ok || receivedEventLen as usize != std::mem::size_of::<crate::types::HdfSysEvent>() {
            let _ = HiLogPrint(
                3u32,
                6u32,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_FAILURE as i32;
        }

        let mut eventContent = HdfSbufReadString(data);
        if eventContent.is_null() {
            eventContent = b"\0".as_ptr() as *const i8;
        }

        OsalMutexLock(&mut (*notifier).mutex as *mut crate::compat::OsalMutex);

        let head = std::ptr::addr_of!((*notifier).notifyNodeList) as *const crate::types::DListHead;
        let listnode_offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
        
        let mut cur = (*head).next;
        while cur != head as *mut crate::types::DListHead {
            let notifyNode = (cur as *mut u8).sub(listnode_offset) as *mut crate::types::HdfSysEventNotifyNode;
            
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

        OsalMutexUnlock(&mut (*notifier).mutex as *mut crate::compat::OsalMutex);

        crate::types::HDF_SUCCESS as i32
    }
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
// last_error_truncated:
//   error[E0433]: failed to resolve: could not find `LOG_ERROR` in `types`
//      --> src/src_osal_sysevent.rs:125:31
//       |
//       |                               ^^^^^^^^^ could not find `LOG_ERROR` in `types`
//   error[E0433]: failed to resolve: could not find `LOG_CORE` in `types`
//      --> src/src_osal_sysevent.rs:124:31
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
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
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
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut ::core::ffi::c_void as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline: insert notifierNode->listNode at tail of notifier->notifyNodeList
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut u8;
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut u8;
        
        // DListHead layout: next, prev (both pointers)
        let ptr_size = std::mem::size_of::<*mut u8>();
        
        let entry_next_ptr = entry as *mut *mut u8;
        let entry_prev_ptr = entry.add(ptr_size) as *mut *mut u8;
        let head_prev_ptr = head.add(ptr_size) as *mut *mut u8;
        
        let head_prev = *head_prev_ptr;
        let head_prev_next_ptr = head_prev as *mut *mut u8;
        
        *entry_next_ptr = head;
        *entry_prev_ptr = head_prev;
        *head_prev_next_ptr = entry;
        *head_prev_ptr = entry;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline: remove notifierNode->listNode from list
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut u8;
            let entry_next_ptr = entry as *mut *mut u8;
            let entry_prev_ptr = entry.add(ptr_size) as *mut *mut u8;
            
            let entry_prev = *entry_prev_ptr;
            let entry_next = *entry_next_ptr;
            
            let entry_prev_next_ptr = entry_prev as *mut *mut u8;
            let entry_next_prev_ptr = (entry_next).add(ptr_size) as *mut *mut u8;
            
            *entry_prev_next_ptr = entry_next;
            *entry_next_prev_ptr = entry_prev;
            *entry_prev_ptr = std::ptr::null_mut();
            *entry_next_ptr = std::ptr::null_mut();
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
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let prev = (*entry).prev;
        let next = (*entry).next;
        (*prev).next = next;
        (*next).prev = prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        // DListIsEmpty check
        let head = std::ptr::addr_of!((*notifier).notifyNodeList);
        let is_empty = (*head).next == head as *mut _;
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(mutex_ptr);
    }
}
