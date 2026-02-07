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
    let sbuf = unsafe { crate::compat::HdfSbufObtain(8) };
    if sbuf.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let sync_token = unsafe { (*event).syncToken };
    if !unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return crate::types::HDF_FAILURE as i32;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = if let Some(func) = unsafe { (*dispatcher).Dispatch } {
        unsafe { func(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) }
    } else {
        crate::types::HDF_FAILURE as i32
    };
    if ret != crate::types::HDF_SUCCESS as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                3u32,
                3u32,
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_osal_sysevent_3
// c_function: OnKEventReceived
// rust_file: src_osal_sysevent.rs
// rust_signature: fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32
// c_first_line: static int OnKEventReceived(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
// last_error_truncated:
//   error: expected identifier, found reserved keyword `priv`
//     --> src/src_osal_sysevent.rs:82:36
//      |
//      |                                    ^^^^ expected identifier, found reserved keyword
//   error: cannot find macro `LOG_CORE` in this scope
//     --> src/src_osal_sysevent.rs:94:32
//      |
//      |                          ^^^^^^^^
// =================================
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_osal_sysevent::OnKEventReceived(listener as _, service as _, id as _, data as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_osal_sysevent_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_3/translated_rust.rs
 * ------------------------------------------------------------
fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    use crate::src_osal_sysevent::FinishEvent;
    use libc::c_void;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -3;
    const HDF_SUCCESS: i32 = 0;
    const HDF_SYSEVENT: u32 = 0xFADE;
    unsafe {
        let notifier = (*listener).priv as *mut c_void;
        if notifier.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        if id != HDF_SYSEVENT {
            return HDF_ERR_INVALID_OBJECT;
        }
        let mut receivedEvent: *const c_void = std::ptr::null();
        let mut receivedEventLen: u32 = 0;
        if HdfSbufReadBuffer(data, &mut receivedEvent as *mut *const c_void, &mut receivedEventLen) == 0
            || receivedEventLen != std::mem::size_of::<c_void>() as u32
        {
            let _ = HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8);
            return HDF_FAILURE;
        }
        let mut eventContent = HdfSbufReadString(data);
        if eventContent.is_null() {
            eventContent = b"\0".as_ptr() as *const i8;
        }
        let notifier_mutex = notifier as *mut OsalMutex;
        let _ = OsalMutexLock(notifier_mutex);
        let notify_node_list = notifier as *mut c_void;
        let mut notifyNode = (*notify_node_list).offset(0) as *mut c_void;
        while !notifyNode.is_null() && notifyNode != notify_node_list {
            let class_filter = *(notifyNode as *const u64);
            let event_class = *(receivedEvent as *const u64);
            if (event_class & class_filter) != 0 {
                let callback = *(notifyNode.offset(8) as *const Option<unsafe extern "C" fn(*mut c_void, u64, u64, *const i8)>);
                if let Some(cb) = callback {
                    cb(notifyNode, event_class, *(receivedEvent.offset(8) as *const u64), eventContent);
                }
            }
            notifyNode = *(notifyNode as *const *mut c_void);
        }
        let sync_token = *(receivedEvent.offset(16) as *const u64);
        if sync_token != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent as *const c_void);
        }
        let _ = OsalMutexUnlock(notifier_mutex);
        HDF_SUCCESS
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
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
// last_error_truncated:
//   error: expected identifier, found reserved keyword `priv`
//     --> src/src_osal_sysevent.rs:93:39
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
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/osal__0bc4f21396ad/workspace/repair_history/osal__0bc4f21396ad/translate_by_qwen3_coder/_manual_fix/src_osal_sysevent_4/translated_rust.rs
 * ------------------------------------------------------------
extern "C" fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind("hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR!(),
                0xD002510,
                "usysevent\0".as_ptr() as *const i8,
                " ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                "hdf_kevent\0".as_ptr() as *const i8,
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
                "usysevent\0".as_ptr() as *const i8,
                " ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                "hdf_kevent\0".as_ptr() as *const i8,
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
        let _ = crate::compat::HdfDeviceUnregisterEventListener((*notifier).keventIoService, &mut (*notifier).ioServiceListener as *mut _);
        crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
        (*notifier).keventIoService = std::ptr::null_mut();
    }
}

pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return -1;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return -2;
    }
    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let _ = crate::compat::OsalMutexLock(mutex_ptr);
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        let head_prev = (*head).prev;
        (*entry).next = head;
        (*entry).prev = head_prev;
        if !head_prev.is_null() {
            (*head_prev).next = entry;
        }
        (*head).prev = entry;
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != 0 {
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
            let entry_prev = (*entry).prev;
            let entry_next = (*entry).next;
            if !entry_prev.is_null() {
                (*entry_prev).next = entry_next;
            }
            if !entry_next.is_null() {
                (*entry_next).prev = entry_prev;
            }
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
        let list_node_ptr = &mut (*notifierNode).listNode;
        (*list_node_ptr).prev.as_mut().unwrap().next = (*list_node_ptr).next;
        (*list_node_ptr).next.as_mut().unwrap().prev = (*list_node_ptr).prev;
        (*list_node_ptr).prev = std::ptr::null_mut();
        (*list_node_ptr).next = std::ptr::null_mut();
        if (*notifier).notifyNodeList.next == &mut (*notifier).notifyNodeList {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }
        let _ = crate::compat::OsalMutexUnlock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
    }
}
