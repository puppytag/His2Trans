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

fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32)-> i32 {
    let data = unsafe { HdfSbufObtainDefaultSize() };
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let _ = unsafe { HdfSbufWriteUint16(data, devClass) };
    let iosvc = unsafe { (*svcmgrInst).iosvc };
    let dispatcher = if iosvc.is_null() { std::ptr::null_mut() } else { unsafe { (*iosvc).dispatcher } };
    if iosvc.is_null() {
        unsafe { HdfSbufRecycle(data); }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if dispatcher.is_null() {
        unsafe { HdfSbufRecycle(data); }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dispatch_fn = unsafe { (*dispatcher).Dispatch };
    if let Some(_) = dispatch_fn {
        let ret = crate::src_hdf_io_service::HdfIoServiceDispatch(
            iosvc as *mut crate::types::HdfIoService,
            cmdId,
            data,
            ::core::ptr::null_mut::<crate::types::HdfSBuf>(),
        );
        unsafe { HdfSbufRecycle(data); }
        ret
    } else {
        unsafe { HdfSbufRecycle(data); }
        crate::types::HDF_ERR_INVALID_OBJECT
    }
}

fn SetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16)-> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_REGISTER_LISTENER as i32)
}

fn UnSetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16)-> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_UNREGISTER_LISTENER as i32)
}

pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || (deviceClass as u32) >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let svcmgr_inst = unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::SvcMgrIoservice>()).svcmgr) as *const _ as usize;
        (self_ as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };

    let listener_inst = unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::IoServiceStatusListener>()).svcstatListener) as *const _ as usize;
        (listener as *mut u8).sub(offset) as *mut crate::types::IoServiceStatusListener
    };

    unsafe {
        (*listener_inst).deviceClass = deviceClass;
    }

    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgr_inst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let iosvc_ptr = unsafe { (*svcmgr_inst).iosvc } as *mut crate::types::HdfIoService;
    let listener_ptr = unsafe { core::ptr::addr_of_mut!((*listener_inst).ioservListener) };
    unsafe {
        crate::compat::HdfDeviceRegisterEventListener(iosvc_ptr, listener_ptr)
    }
}

pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }

    let svcmgr_inst: *mut crate::types::SvcMgrIoservice = unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::SvcMgrIoservice>()).svcmgr) as *const _ as usize;
        (self_ as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };

    let listener_inst: *mut crate::types::IoServiceStatusListener = unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::IoServiceStatusListener>()).svcstatListener) as *const _ as usize;
        (listener as *mut u8).sub(offset) as *mut crate::types::IoServiceStatusListener
    };

    let iosvc_mut: *mut crate::types::HdfIoService = unsafe { (*(svcmgr_inst)).iosvc } as *mut crate::types::HdfIoService;
    let listener_ioserv_ptr = unsafe { core::ptr::addr_of_mut!((*listener_inst).ioservListener) };
    let mut ret: i32 = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            iosvc_mut,
            listener_ioserv_ptr,
        )
    };

    if ret != HDF_SUCCESS {
        return ret;
    }

    let iosvc_const: *const crate::types::HdfIoService = unsafe { (*(svcmgr_inst)).iosvc } as *const crate::types::HdfIoService;
    let listener_count = unsafe { crate::compat::HdfIoserviceGetListenerCount(iosvc_const) };

    if listener_count == 0 {
        ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgr_inst, unsafe { (*listener_inst).deviceClass });
    }

    ret
}

fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    let r = unsafe { &mut *svcmgrInst };
    r.RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
    r.UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
}

pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    use core::ffi::c_void;
    use crate::types::DEV_SVCMGR_NODE;
    use crate::src_hdf_io_service::HdfIoServiceBind;
    use crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct;

    let size = core::mem::size_of::<SvcMgrIoservice>();
    let svcmgr_inst = {
        let raw = unsafe { crate::compat::OsalMemCalloc(size as crate::types::size_t) };
        raw as *mut SvcMgrIoservice
    };
    if svcmgr_inst.is_null() {
        return core::ptr::null_mut();
    }

    let bind_result = HdfIoServiceBind(
        DEV_SVCMGR_NODE.as_ptr() as *const ::core::ffi::c_char
    ) as *mut ::core::ffi::c_void as *mut _;
    unsafe {
        (*svcmgr_inst).iosvc = bind_result;
    }

    if bind_result.is_null() {
        let ptr = svcmgr_inst as *mut ::core::ffi::c_void;
        unsafe { crate::compat::OsalMemFree(ptr); }
        return core::ptr::null_mut();
    }

    let field_ptr = unsafe { core::ptr::addr_of_mut!((*svcmgr_inst).svcmgr) };
    let svcmgr_mut: *mut crate::types::ISvcMgrIoservice = field_ptr as *mut crate::types::ISvcMgrIoservice;
    SvcMgrIoserviceConstruct(svcmgr_mut);

    svcmgr_mut
}

pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    let svcmgr_inst = unsafe {
        let offset = core::ptr::addr_of!((*std::ptr::null::<crate::types::SvcMgrIoservice>()).svcmgr) as *const _ as usize;
        (svcmgr as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };
    let iosvc = unsafe { (*svcmgr_inst).iosvc } as *mut crate::types::HdfIoService;
    crate::src_hdf_io_service::HdfIoServiceRecycle(iosvc);
    let ptr = svcmgr_inst as *mut ::core::ffi::c_void;
    unsafe {
        OsalMemFree(ptr);
    }
}
