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
        
        let ret = dispatch_fn.unwrap()(
            iosvc as *mut _,
            cmdId,
            data as *mut _,
            std::ptr::null_mut()
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

    let svcmgrInst = self_ as *mut crate::types::SvcMgrIoservice;
    let listenerInst = listener as *mut crate::types::IoServiceStatusListener;

    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }

    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const i8,
                b"failed to set listen class\0".as_ptr() as *const i8,
            );
        }
        return ret;
    }

    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        let ioserv_listener = &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener;
        crate::compat::HdfDeviceRegisterEventListener(iosvc as *mut _, ioserv_listener as *mut _)
    }
}

pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(svcmgr_offset as isize)) as *mut crate::types::SvcMgrIoservice
    };

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstat_offset = std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(svcstat_offset as isize)) as *mut crate::types::IoServiceStatusListener
    };

    let iosvc_ptr = unsafe { (*svcmgrInst).iosvc };
    let listener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };
    
    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            iosvc_ptr as *mut crate::types::HdfIoService,
            listener_ptr as *mut crate::types::HdfDevEventlistener
        )
    };

    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    unsafe {
        if crate::compat::HdfIoserviceGetListenerCount(iosvc_ptr as *const crate::types::HdfIoService) == 0 {
            ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass);
        }
    }

    ret
}

fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    unimplemented!()
}

pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    let svcmgr_inst: *mut crate::types::SvcMgrIoservice = unsafe {
        crate::compat::OsalMemCalloc(core::mem::size_of::<crate::types::SvcMgrIoservice>() as u32) as *mut crate::types::SvcMgrIoservice
    };
    if svcmgr_inst.is_null() {
        return core::ptr::null_mut();
    }

    let service_name = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
    let iosvc = crate::src_hdf_io_service::HdfIoServiceBind(service_name);
    if iosvc.is_null() {
        let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char;
        let svc_name_arg = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag,
                fmt,
                svc_name_arg,
            );
            crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
        }
        return core::ptr::null_mut();
    }

    unsafe {
        (*svcmgr_inst).iosvc = iosvc as *mut _;
        let svcmgr_ptr = core::ptr::addr_of_mut!((*svcmgr_inst).svcmgr) as *mut crate::types::ISvcMgrIoservice;
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(svcmgr_ptr);
        svcmgr_ptr
    }
}

pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF: svcmgrInst = (SvcMgrIoservice*)((char*)svcmgr - offsetof(SvcMgrIoservice, svcmgr))
    // SvcMgrIoservice has field 'svcmgr' of type ISvcMgrIoservice
    // We need to compute the offset of 'svcmgr' field within SvcMgrIoservice
    let offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (svcmgr as *mut u8).offset(-(offset as isize)) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        // Cast iosvc to the expected type for HdfIoServiceRecycle
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgrInst).iosvc as *mut crate::types::HdfIoService);
        crate::compat::OsalMemFree(svcmgrInst as *mut ::core::ffi::c_void);
    }
}
