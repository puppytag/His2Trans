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
    const PATH_SIZE: usize = (PATH_MAX + 1) as usize;
    let mut path: [::core::ffi::c_char; PATH_SIZE] = [0; PATH_SIZE];
    let mut resolved_path: [::core::ffi::c_char; PATH_SIZE] = [0; PATH_SIZE];

    if libName.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s libName is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return ::core::ptr::null_mut();
    }

    // Safely compute lengths; only CStr::from_ptr is unsafe.
    let prefix = b"/vendor/lib/";
    let prefix_len = prefix.len() - 1; // exclude null terminator
    let cstr_lib = unsafe { ::core::ffi::CStr::from_ptr(libName) };
    let lib_bytes = cstr_lib.to_bytes();
    let lib_len = lib_bytes.len();

    let total = prefix_len + 1 + lib_len; // +1 for the extra '/'
    if total + 1 > PATH_SIZE {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s snprintf_s failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                libName,
            );
        }
        return ::core::ptr::null_mut();
    }

    // Build path using safe slice operations; only slice creation is unsafe.
    {
        let path_slice: &mut [u8] = unsafe {
            ::core::slice::from_raw_parts_mut(path.as_mut_ptr() as *mut u8, PATH_SIZE)
        };
        path_slice[..prefix_len].copy_from_slice(&prefix[..prefix_len]);
        path_slice[prefix_len] = b'/';
        path_slice[prefix_len + 1..prefix_len + 1 + lib_len].copy_from_slice(lib_bytes);
        path_slice[total] = 0u8;
    }

    // realpath & prefix check (FFI calls remain unsafe)
    let rp = unsafe { realpath(path.as_mut_ptr(), resolved_path.as_mut_ptr()) };
    let cmp_result = unsafe {
        libc::strncmp(
            resolved_path.as_ptr() as *const ::core::ffi::c_char,
            prefix.as_ptr() as *const ::core::ffi::c_char,
            prefix_len,
        )
    };
    if rp.is_null() || cmp_result != 0 {
        let err = unsafe { *libc::__errno_location() };
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s %{public}s realpath file name failed %{public}d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                path.as_ptr(),
                resolved_path.as_ptr(),
                err,
            );
        }
        return ::core::ptr::null_mut();
    }

    // Allocate HdfVdiObject (FFI unsafe)
    let vdi_obj_size = ::core::mem::size_of::<crate::types::HdfVdiObject>();
    let vdi_obj = unsafe { OsalMemCalloc(vdi_obj_size as u32) as *mut crate::types::HdfVdiObject };
    if vdi_obj.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return ::core::ptr::null_mut();
    }

    // dlopen
    let handler = unsafe {
        dlopen(
            resolved_path.as_ptr() as *const ::core::ffi::c_char,
            RTLD_LAZY as i32,
        )
    };
    if handler.is_null() {
        let err = unsafe { dlerror() };
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlopen failed %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                err,
            );
            OsalMemFree(vdi_obj as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut();
    }

    // dlsym for hdfVdiDesc
    let vdi_base = unsafe {
        dlsym(
            handler,
            b"hdfVdiDesc\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut *mut crate::types::HdfVdiBase
    };
    if vdi_base.is_null() {
        let err = unsafe { dlerror() };
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlsym hdfVdiDesc failed %{public}s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                err,
            );
            dlclose(handler);
            OsalMemFree(vdi_obj as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut();
    }

    let base_ptr = unsafe { *vdi_base };
    if base_ptr.is_null() {
        let err = unsafe { dlerror() };
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlsym hdfVdiDesc failed %{public}s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                err,
            );
            dlclose(handler);
            OsalMemFree(vdi_obj as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut();
    }

    // Call CreateVdiInstance if present
    let create_fn = unsafe { (*base_ptr).CreateVdiInstance };
    if let Some(create_fn) = create_fn {
        unsafe {
            create_fn(base_ptr);
        }
    }

    // Set fields
    unsafe {
        (*vdi_obj).dlHandler = handler as usize;
        (*vdi_obj).vdiBase = base_ptr;
    }

    vdi_obj
}

pub extern "C" fn HdfGetVdiVersion(vdiObj: *const crate::types::HdfVdiObject) -> u32 {
    if vdiObj.is_null() {
        return crate::types::HDF_INVALID_VERSION;
    }
    let vdi_base = unsafe { (*vdiObj).vdiBase };
    if vdi_base.is_null() {
        return crate::types::HDF_INVALID_VERSION;
    }
    unsafe { (*vdi_base).moduleVersion }
}

pub extern "C" fn HdfCloseVdi(vdiObj: *mut crate::types::HdfVdiObject) {
    if vdiObj.is_null() {
        return;
    }
    let dlHandler: usize;
    let vdiBase: *mut crate::types::HdfVdiBase;
    unsafe {
        dlHandler = (*vdiObj).dlHandler;
        vdiBase = (*vdiObj).vdiBase;
    }
    if dlHandler == 0 || vdiBase.is_null() {
        return;
    }
    let vdiObj_ref = unsafe { &mut *vdiObj };
    let destroy_fn = unsafe { (*vdiBase).DestoryVdiInstance };
    if let Some(destroy_fn) = destroy_fn {
        unsafe {
            destroy_fn(vdiBase);
        }
    }
    unsafe {
        dlclose(dlHandler as *mut ::core::ffi::c_void);
    }
    vdiObj_ref.dlHandler = 0;
    vdiObj_ref.vdiBase = ::core::ptr::null_mut();
    unsafe { OsalMemFree(vdiObj as *mut ::core::ffi::c_void); }
}
