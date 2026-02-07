//! Module: src_hdf_driver_loader
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

pub extern "C" fn HdfDriverEntryConstruct() -> i32 {
    let mut i: i32;
    let mut driverEntry: *mut crate::types::HdfDriverEntry = std::ptr::null_mut();
    let mut addrBegin: *mut crate::types::size_t = std::ptr::null_mut();
    let count: i32 = 0;
    if count <= 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"driver_loader\0".as_ptr() as *const i8,
                b"%{public}s: no hdf driver exist\0".as_ptr() as *const i8,
                b"HdfDriverEntryConstruct\0".as_ptr() as *const i8,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    addrBegin = std::ptr::null_mut();
    i = 0;
    while i < count {
        driverEntry = unsafe { *addrBegin as *mut crate::types::HdfDriverEntry };
        if unsafe {
            crate::compat::HdfRegisterDriverEntry(driverEntry as *const crate::types::HdfDriverEntry)
        } != crate::types::HDF_SUCCESS
        {
            let module_name = if !driverEntry.is_null() {
                unsafe { (*driverEntry).moduleName }
            } else {
                std::ptr::null()
            };
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"driver_loader\0".as_ptr() as *const i8,
                    b"failed to register driver %{public}s, skip and try another\0".as_ptr() as *const i8,
                    module_name,
                );
            }
        }
        addrBegin = unsafe { addrBegin.offset(1) };
        i += 1;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDriverLoaderGetDriver(moduleName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDriver {
    if moduleName.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"driver_loader\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: failed to get device entry, moduleName is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfDriverLoaderGetDriver\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        return std::ptr::null_mut();
    }
    unsafe { crate::compat::HdfDriverManagerGetDriver(moduleName) }
}

pub extern "C" fn HdfDriverLoaderReclaimDriver(driver: *mut crate::types::HdfDriver) {
    let _ = driver;
}

pub extern "C" fn HdfDriverLoaderConstruct(inst: *mut crate::types::HdfDriverLoader) {
    if !inst.is_null() {
        unsafe {
            (*inst).super_.GetDriver = Some(HdfDriverLoaderGetDriver);
            (*inst).super_.ReclaimDriver = Some(HdfDriverLoaderReclaimDriver);
        }
    }
}

pub extern "C" fn HdfDriverLoaderCreate() -> *mut crate::types::HdfObject {
    static mut IS_DRIVER_LOADER_INIT: bool = false;
    static mut DRIVER_LOADER: crate::types::HdfDriverLoader = unsafe {
        crate::types::HdfDriverLoader {
            super_: crate::types::IDriverLoader {
                object: crate::types::HdfObject { objectId: 0 },
                GetDriver: None,
                ReclaimDriver: None,
            },
        }
    };
    unsafe {
        if !IS_DRIVER_LOADER_INIT {
            if crate::src_hdf_driver_loader::HdfDriverEntryConstruct() != crate::types::HDF_SUCCESS {
                return std::ptr::null_mut();
            }
            crate::src_hdf_driver_loader::HdfDriverLoaderConstruct(&mut DRIVER_LOADER);
            IS_DRIVER_LOADER_INIT = true;
        }
        &mut DRIVER_LOADER as *mut crate::types::HdfDriverLoader as *mut crate::types::HdfObject
    }
}

pub extern "C" fn HdfDriverLoaderGetInstance() -> *mut crate::types::IDriverLoader {
    static mut INSTANCE: *mut crate::types::IDriverLoader = std::ptr::null_mut();
    unsafe {
        if INSTANCE.is_null() {
            INSTANCE = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DRIVER_LOADER as i32) as *mut crate::types::IDriverLoader;
        }
        INSTANCE
    }
}
