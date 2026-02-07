//! Module: src_svcmgr_ioservice
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

fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = unsafe { crate::compat::HdfSbufObtainDefaultSize() };
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let _ = unsafe { crate::compat::HdfSbufWriteUint16(data, devClass) };

    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        if iosvc.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let dispatcher = (*iosvc).dispatcher;
        if dispatcher.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        let ret = crate::src_hdf_io_service::HdfIoServiceDispatch(
            iosvc as *mut crate::types::HdfIoService,
            cmdId as ::core::ffi::c_int,
            data as *mut crate::types::HdfSBuf,
            std::ptr::null_mut(),
        );
        crate::compat::HdfSbufRecycle(data);
        ret
    }
}

fn SetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16) -> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_REGISTER_LISTENER as i32)
}

fn UnSetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16) -> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_UNREGISTER_LISTENER as i32)
}

pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = core::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = core::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }

    let ret = SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    unsafe {
        let iosvc_ptr = (*svcmgrInst).iosvc as *mut crate::types::HdfIoService;
        let listener_ptr = &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener;
        crate::compat::HdfDeviceRegisterEventListener(iosvc_ptr, listener_ptr)
    }
}

pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr) using offset_of! macro approach
    let svcmgr_offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (self_ as *mut u8).sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice
    };

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = unsafe {
        (listener as *mut u8).sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener
    };

    let iosvc = unsafe { (*svcmgrInst).iosvc };
    let ioservListener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };

    let mut ret = unsafe { crate::compat::HdfDeviceUnregisterEventListener(iosvc as *mut _, ioservListener_ptr as *mut _) };
    if ret != crate::types::HDF_SUCCESS as i32 {
        return ret;
    }

    if unsafe { crate::compat::HdfIoserviceGetListenerCount(iosvc as *const _) } == 0 {
        let deviceClass = unsafe { (*listenerInst).deviceClass };
        ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, deviceClass);
    }

    ret
}

fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    unsafe {
        (*svcmgrInst).RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
        (*svcmgrInst).UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
    }
}

pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    unsafe {
        let svcmgr_inst = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::SvcMgrIoservice>() as u32
        ) as *mut crate::types::SvcMgrIoservice;
        
        if svcmgr_inst.is_null() {
            return std::ptr::null_mut();
        }
        
        let service_name = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
        let iosvc = crate::src_hdf_io_service::HdfIoServiceBind(service_name);
        
        if iosvc.is_null() {
            let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char;
            let svc_name_arg = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                tag,
                fmt,
                svc_name_arg,
            );
            crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        let iosvc_field_ptr = std::ptr::addr_of_mut!((*svcmgr_inst).iosvc);
        std::ptr::write(iosvc_field_ptr as *mut *mut crate::types::HdfIoService, iosvc);
        
        let svcmgr_ptr = std::ptr::addr_of_mut!((*svcmgr_inst).svcmgr) as *mut crate::types::ISvcMgrIoservice;
        SvcMgrIoserviceConstruct(svcmgr_ptr);
        
        svcmgr_ptr
    }
}

pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get SvcMgrIoservice from ISvcMgrIoservice pointer
    // offset = &((struct SvcMgrIoservice *)0)->svcmgr
    let offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgr_inst = unsafe {
        (svcmgr as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        // Cast iosvc to the expected type for HdfIoServiceRecycle
        let iosvc_ptr = (*svcmgr_inst).iosvc as *mut crate::types::HdfIoService;
        crate::src_hdf_io_service::HdfIoServiceRecycle(iosvc_ptr);
        crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
    }
}
