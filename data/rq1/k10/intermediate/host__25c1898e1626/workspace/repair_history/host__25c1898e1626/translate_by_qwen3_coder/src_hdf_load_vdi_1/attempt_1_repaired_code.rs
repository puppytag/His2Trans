pub extern "C" fn HdfLoadVdi(libName: *const ::core::ffi::c_char) -> *mut crate::types::HdfVdiObject {
    let mut path: [::core::ffi::c_char; 4096 + 1] = [0; 4096 + 1];
    let mut resolvedPath: [::core::ffi::c_char; 4096 + 1] = [0; 4096 + 1];
    if libName.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s libName is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        return std::ptr::null_mut();
    }
    if unsafe {
        crate::compat::snprintf_s(
            path.as_mut_ptr(),
            std::mem::size_of_val(&path) as crate::types::size_t,
            (std::mem::size_of_val(&path) - 1) as crate::types::size_t,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/vendor/lib/\0".as_ptr() as *const ::core::ffi::c_char,
            libName,
        )
    } < 0
    {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s snprintf_s failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                libName,
            )
        };
        return std::ptr::null_mut();
    }
    if unsafe { crate::compat::realpath(path.as_ptr(), resolvedPath.as_mut_ptr()) }.is_null()
        || unsafe {
            crate::compat::strncmp(
                resolvedPath.as_ptr(),
                b"/vendor/lib/\0".as_ptr() as *const ::core::ffi::c_char,
                unsafe { crate::compat::strlen(b"/vendor/lib/\0".as_ptr() as *const ::core::ffi::c_char) } as crate::types::size_t,
            )
        } != 0
    {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s %{public}s %{public}s realpath file name failed %{public}d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                path.as_ptr(),
                resolvedPath.as_ptr(),
                unsafe { *crate::compat::__errno_location() },
            )
        };
        return std::ptr::null_mut();
    }
    let vdiObj = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::HdfVdiObject>() as crate::types::size_t) as *mut crate::types::HdfVdiObject
    };
    if vdiObj.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        return std::ptr::null_mut();
    }
    let handler = unsafe { crate::compat::dlopen(resolvedPath.as_ptr(), 1) };
    if handler.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlopen failed %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                unsafe { crate::compat::dlerror() },
            )
        };
        unsafe { crate::compat::OsalMemFree(vdiObj as *mut ::core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let vdiBase = unsafe {
        crate::compat::dlsym(
            handler,
            b"hdfVdiDesc\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut *mut crate::types::HdfVdiBase
    };
    if vdiBase.is_null() || unsafe { *vdiBase }.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s dlsym hdfVdiDesc failed %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfLoadVdi\0".as_ptr() as *const ::core::ffi::c_char,
                unsafe { crate::compat::dlerror() },
            )
        };
        unsafe {
            crate::compat::dlclose(handler);
            crate::compat::OsalMemFree(vdiObj as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }
    let base_ptr = unsafe { *vdiBase };
    if let Some(func) = unsafe { (*base_ptr).CreateVdiInstance } {
        let _ = unsafe { func(base_ptr) };
    }
    unsafe {
        (*vdiObj).dlHandler = handler as usize;
        (*vdiObj).vdiBase = base_ptr;
    }
    vdiObj
}