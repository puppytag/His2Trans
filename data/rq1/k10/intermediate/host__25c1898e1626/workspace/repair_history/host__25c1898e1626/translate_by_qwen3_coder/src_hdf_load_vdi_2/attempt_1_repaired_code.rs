pub extern "C" fn HdfGetVdiVersion(vdiObj: *const crate::types::HdfVdiObject) -> u32 {
    if vdiObj.is_null() {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
            b"%{public}s para is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            b"HdfGetVdiVersion\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0xFFFFFFFF;
    }
    unsafe {
        if (*vdiObj).vdiBase.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"dev_load_vdi\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s para is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfGetVdiVersion\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0xFFFFFFFF;
        }
        (*(*vdiObj).vdiBase).moduleVersion
    }
}