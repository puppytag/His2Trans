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
    static mut HDF_SYS_EVENT_NOTIFIER: *mut crate::types::HdfSysEventNotifier = std::ptr::null_mut();
    unsafe {
        if !HDF_SYS_EVENT_NOTIFIER.is_null() {
            return HDF_SYS_EVENT_NOTIFIER;
        }
        let notifier = libc::calloc(1, std::mem::size_of::<crate::types::HdfSysEventNotifier>()) as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return std::ptr::null_mut();
        }
        let ret = crate::compat::OsalMutexInit(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        if ret != 0 {
            libc::free(notifier as *mut libc::c_void);
            return std::ptr::null_mut();
        }
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*head).next = head;
        (*head).prev = head;
        HDF_SYS_EVENT_NOTIFIER = notifier;
        notifier
    }
}

fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
    if sbuf.is_null() {
        return -1;
    }
    let sync_token = unsafe { (*event).syncToken };
    if !unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return -1;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = if let Some(func) = unsafe { (*dispatcher).Dispatch } {
        unsafe { func(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) }
    } else {
        -1
    };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(
            3,
            3,
            0xD002510,
            b"usysevent\0".as_ptr() as *const _,
            b"failed to finish sysevent, %{public}d\0".as_ptr() as *const _,
            ret,
        ) };
    }
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
    ret
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_3
// c_function: OnKEventReceived
// rust_file: src_osal_sysevent.rs
// rust_signature: fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32
// c_first_line: static int OnKEventReceived(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error: expected identifier, found reserved keyword `priv`
//     --> src/src_osal_sysevent.rs:80:41
//      |
//      |                                         ^^^^ expected identifier, found reserved keyword
//   error: cannot find macro `LOG_CORE` in this scope
//     --> src/src_osal_sysevent.rs:97:37
//      |
//      |                                     ^^^^^^^^
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    use ::libc;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -3;
    const HDF_SUCCESS: i32 = 0;
    let notifier = unsafe { (*listener).priv as *mut HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
    }
    let mut received_event_ptr: *const c_void = std::ptr::null();
    let mut received_event_len: u32 = 0;
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut received_event_ptr as *mut *const c_void as *mut *const c_void,
            &mut received_event_len,
        )
    };
    if read_ok == 0 || received_event_len != std::mem::size_of::<HdfSysEvent>() as u32 {
        let _ = unsafe { HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8) };
        return HDF_FAILURE;
    }
    let received_event = received_event_ptr as *const HdfSysEvent;
    let event_content_ptr = unsafe { HdfSbufReadString(data) };
    let event_content = if event_content_ptr.is_null() {
        b"\0".as_ptr() as *const i8
    } else {
        event_content_ptr
    };
    unsafe { OsalMutexLock(&mut (*notifier).mutex) };
    let notify_node_list = unsafe { &(*notifier).notifyNodeList };
    let mut notify_node = unsafe {
        (notify_node_list.next as *mut u8).offset(-(std::mem::size_of::<HdfSysEventNotifyNode>() as isize)) as *mut HdfSysEventNotifyNode
    };
    while unsafe { &(*notify_node).listNode } != notify_node_list {
        let next = unsafe { (*notify_node).listNode.next };
        if unsafe { (*received_event).eventClass & (*notify_node).classFilter } != 0 {
            let cb = unsafe { (*notify_node).callback };
            if let Some(f) = cb {
                unsafe { f(notify_node, (*received_event).eventClass, (*received_event).eventid, event_content) };
            }
        }
        notify_node = unsafe {
            (next as *mut u8).offset(-(std::mem::size_of::<HdfSysEventNotifyNode>() as isize)) as *mut HdfSysEventNotifyNode
        };
    }
    if unsafe { (*received_event).syncToken } != 0 {
        let _ = crate::src_osal_sysevent::FinishEvent(service, received_event);
    }
    unsafe { OsalMutexUnlock(&mut (*notifier).mutex) };
    HDF_SUCCESS
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
// last_error_truncated:
//   error: expected identifier, found reserved keyword `priv`
//     --> src/src_osal_sysevent.rs:91:39
//      |
//      |                                       ^^^^ expected identifier, found reserved keyword
//      |
//   help: escape `priv` to use it as an identifier
//      |
//      |                                       ++
// =================================
fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_4
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
 * ------------------------------------------------------------
extern "C" fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return crate::compat::HDF_DEV_ERR_NO_DEVICE!();
        }

        (*notifier).ioServiceListener.onReceive = Some(crate::src_osal_sysevent::OnKEventReceived);
        (*notifier).ioServiceListener.priv = notifier as *mut ::core::ffi::c_void;
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener,
        );
        if ret != crate::compat::HDF_SUCCESS!() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR!(),
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
        if (*notifier).keventIoService.is_null() {
            return;
        }
        let _ = crate::compat::HdfDeviceUnregisterEventListener((*notifier).keventIoService, &mut (*notifier).ioServiceListener);
        crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
        (*notifier).keventIoService = std::ptr::null_mut();
    }
}

pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return -1;
    }
    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let _ = crate::compat::OsalMutexLock(mutex_ptr);
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut crate::types::DListHead;
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut crate::types::DListHead;
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != crate::types::HDF_SUCCESS {
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut crate::types::DListHead;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        let _ = crate::compat::OsalMutexUnlock(mutex_ptr);
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
        let _ = crate::compat::OsalMutexLock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        let list_node_ptr = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut crate::types::DListHead;
        (*list_node_ptr).prev.as_mut().unwrap().next = (*list_node_ptr).next;
        (*list_node_ptr).next.as_mut().unwrap().prev = (*list_node_ptr).prev;
        (*list_node_ptr).prev = std::ptr::null_mut();
        (*list_node_ptr).next = std::ptr::null_mut();
        let notify_node_list_ptr = std::ptr::addr_of!((*notifier).notifyNodeList) as *const crate::types::DListHead;
        if (*notify_node_list_ptr).next == notify_node_list_ptr as *mut crate::types::DListHead {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }
        let _ = crate::compat::OsalMutexUnlock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
    }
}
