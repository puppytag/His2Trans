pub extern "C" fn HdfCloseVdi(vdiObj: *mut crate::types::HdfVdiObject) {
    unsafe {
        if vdiObj.is_null() {
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
        if (*vdiObj).dlHandler == 0 || (*vdiObj).vdiBase.is_null() {
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
        let vdiBase = (*vdiObj).vdiBase;
        if let Some(f) = (*vdiBase).DestoryVdiInstance {
            f(vdiBase);
        }
        crate::compat::dlclose((*vdiObj).dlHandler as *mut ::core::ffi::c_void);
        (*vdiObj).dlHandler = 0;
        (*vdiObj).vdiBase = std::ptr::null_mut();
        crate::compat::OsalMemFree(vdiObj as *mut ::core::ffi::c_void);
    }
}