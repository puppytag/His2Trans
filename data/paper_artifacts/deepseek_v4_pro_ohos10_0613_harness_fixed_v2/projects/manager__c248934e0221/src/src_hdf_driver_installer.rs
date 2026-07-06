//! Module: src_hdf_driver_installer
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

fn DriverInstallerStartDeviceHost(devHostId: u32, devHostName: *const std::ffi::c_char, dynamic: bool)-> i32 {
    let _ = dynamic;
    let hostServiceIf: *mut crate::types::IDevHostService = unsafe { DevHostServiceNewInstance(devHostId.try_into().unwrap(), devHostName) };
    if hostServiceIf.is_null() || unsafe { (*hostServiceIf).StartService.is_none() } {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"driver_installer\0".as_ptr() as *const std::ffi::c_char,
                b"hostServiceIf or hostServiceIf->StartService is null\0".as_ptr() as *const std::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let ret: i32 = unsafe { (*hostServiceIf).StartService.unwrap()(hostServiceIf) };
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510_u32,
                b"driver_installer\0".as_ptr() as *const std::ffi::c_char,
                b"failed to start host service, ret: %{public}d\0".as_ptr() as *const std::ffi::c_char,
                ret,
            );
            DevHostServiceFreeInstance(hostServiceIf);
        }
    }
    ret
}

fn DriverInstallerConstruct(inst: *mut crate::types::DriverInstaller) {
    unsafe {
        let f: unsafe extern "C" fn(u32, *const ::core::ffi::c_char, bool) -> ::core::ffi::c_int =
            ::core::mem::transmute(DriverInstallerStartDeviceHost as *const ());
        (*inst).super_.StartDeviceHost = Some(f);
    }
}

pub extern "C" fn DriverInstallerCreate() -> *mut crate::types::HdfObject {
    static mut IS_DRIVER_INST_INIT: bool = false;
    static mut DRIVER_INSTALLER: core::mem::MaybeUninit<crate::types::DriverInstaller> =
        core::mem::MaybeUninit::zeroed();
    unsafe {
        if !IS_DRIVER_INST_INIT {
            crate::src_hdf_driver_installer::DriverInstallerConstruct(
                DRIVER_INSTALLER.as_mut_ptr(),
            );
            IS_DRIVER_INST_INIT = true;
        }
        DRIVER_INSTALLER.as_mut_ptr() as *mut crate::types::HdfObject
    }
}

pub extern "C" fn DriverInstallerGetInstance() -> *mut crate::types::IDriverInstaller {
    unsafe {
        static mut installer: *mut crate::types::IDriverInstaller = std::ptr::null_mut();
        if installer.is_null() {
            installer = crate::compat::HdfObjectManagerGetObject(
                (crate::types::HDF_OBJECT_ID_DRIVER_INSTALLER as u32).try_into().unwrap(),
            ) as *mut crate::types::IDriverInstaller;
        }
        installer
    }
}
