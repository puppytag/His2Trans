fn VerifyUdid(pf: *const crate::types::ProfileProf) -> i32 {
    let size: u32 = 64 + 1;
    unsafe {
        if (*pf).debugInfo.devidNum > 100 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: udid num exceed maximum\0".as_ptr() as *const i8,
                "VerifyUdid\0".as_ptr() as *const i8,
                383,
            );
            return crate::types::V_ERR as i32;
        }
    }
    let udid = unsafe { libc::malloc(size as usize) as *mut ::core::ffi::c_uchar };
    if udid.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: udid is null\0".as_ptr() as *const i8,
                "VerifyUdid\0".as_ptr() as *const i8,
                388,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        crate::compat::memset_s(
            udid as *mut ::core::ffi::c_void,
            size as crate::types::size_t,
            0,
            size as crate::types::size_t,
        );
    }
    let result = unsafe { crate::src_app_verify_hal::InquiryDeviceUdid(udid, size as i32) };
    if result != 0 {
        unsafe {
            libc::free(udid as *mut ::core::ffi::c_void);
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: get udid fail, ret: %d\0".as_ptr() as *const i8,
                "VerifyUdid\0".as_ptr() as *const i8,
                395,
                result,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        for i in 0..(*pf).debugInfo.devidNum {
            let dev_id_ptr = *((*pf).debugInfo.deviceId.offset(i as isize));
            if libc::strcmp(dev_id_ptr as *const i8, udid as *const i8) == 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: find right udid\0".as_ptr() as *const i8,
                    "VerifyUdid\0".as_ptr() as *const i8,
                    400,
                );
                libc::free(udid as *mut ::core::ffi::c_void);
                return crate::types::V_OK as i32;
            }
        }
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: udid invalid\0".as_ptr() as *const i8,
            "VerifyUdid\0".as_ptr() as *const i8,
            406,
        );
        libc::free(udid as *mut ::core::ffi::c_void);
    }
    crate::types::V_ERR as i32
}