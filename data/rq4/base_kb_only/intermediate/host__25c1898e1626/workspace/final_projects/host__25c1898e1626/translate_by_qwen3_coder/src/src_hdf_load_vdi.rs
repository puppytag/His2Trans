//! Module: src_hdf_load_vdi
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

pub extern "C" fn HdfLoadVdi(libName: *const ::core::ffi::c_char) -> *mut crate::types::HdfVdiObject {
    use crate::compat::*;
    use crate::types::*;
    
    let mut path: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut resolvedPath: [::core::ffi::c_char; 4097] = [0; 4097];
    
    if libName.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s libName is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    let vendor_lib = b"/vendor/lib/\0".as_ptr() as *const ::core::ffi::c_char;
    
    let ret = unsafe {
        snprintf_s(
            path.as_mut_ptr(),
            std::mem::size_of_val(&path) as u32,
            (std::mem::size_of_val(&path) - 1) as u32,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            vendor_lib,
            libName,
        )
    };
    
    if ret < 0 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s snprintf_s failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                libName,
            );
        }
        return std::ptr::null_mut();
    }
    
    let realpath_result = unsafe { realpath(path.as_ptr(), resolvedPath.as_mut_ptr()) };
    let vendor_lib_len = unsafe { strlen(vendor_lib) };
    
    if realpath_result.is_null() || unsafe { strncmp(resolvedPath.as_ptr(), vendor_lib, vendor_lib_len) } != 0 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s %{public}s realpath file name failed %{public}d\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                path.as_ptr(),
                resolvedPath.as_ptr(),
                *__errno_location(),
            );
        }
        return std::ptr::null_mut();
    }
    
    let vdiObj = unsafe { OsalMemCalloc(std::mem::size_of::<HdfVdiObject>() as u32) } as *mut HdfVdiObject;
    if vdiObj.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    let handler = unsafe { dlopen(resolvedPath.as_ptr(), 1) };
    if handler.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlopen failed %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                dlerror(),
            );
            OsalMemFree(vdiObj as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    let vdiBase = unsafe { dlsym(handler, b"hdfVdiDesc\0".as_ptr() as *const ::core::ffi::c_char) } as *mut *mut HdfVdiBase;
    if vdiBase.is_null() || unsafe { (*vdiBase).is_null() } {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlsym hdfVdiDesc failed %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                dlerror(),
            );
            dlclose(handler);
            OsalMemFree(vdiObj as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    
    unsafe {
        let base_ptr = *vdiBase;
        if let Some(create_fn) = (*base_ptr).CreateVdiInstance {
            create_fn(base_ptr);
        }
        
        (*vdiObj).dlHandler = handler as usize;
        (*vdiObj).vdiBase = base_ptr;
    }
    
    vdiObj
}

pub extern "C" fn HdfGetVdiVersion(vdiObj: *const crate::types::HdfVdiObject) -> u32 {
    if vdiObj.is_null() {
        return 0xFFFFFFFF;
    }
    
    unsafe {
        let vdi_base = (*vdiObj).vdiBase;
        if vdi_base.is_null() {
            return 0xFFFFFFFF;
        }
        
        (*vdi_base).moduleVersion
    }
}

pub extern "C" fn HdfCloseVdi(vdiObj: *mut crate::types::HdfVdiObject) {
    unsafe {
        if vdiObj.is_null() || (*vdiObj).dlHandler == 0 || (*vdiObj).vdiBase.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s para invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfCloseVdi\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }

        let vdiBase: *mut crate::types::HdfVdiBase = (*vdiObj).vdiBase;
        if let Some(destroy_fn) = (*vdiBase).DestoryVdiInstance {
            destroy_fn(vdiBase);
        }

        crate::compat::dlclose((*vdiObj).dlHandler as *mut ::core::ffi::c_void);
        (*vdiObj).dlHandler = 0;
        (*vdiObj).vdiBase = std::ptr::null_mut();
        crate::compat::OsalMemFree(vdiObj as *mut ::core::ffi::c_void);
    }
}
