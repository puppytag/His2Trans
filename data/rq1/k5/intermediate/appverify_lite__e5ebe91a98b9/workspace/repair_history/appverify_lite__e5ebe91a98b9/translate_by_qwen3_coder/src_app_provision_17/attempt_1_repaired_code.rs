fn VerifyUdid(pf: *const crate::types::ProfileProf) -> i32 {
    let size: u32 = 64 + 1;
    unsafe {
        if (*pf).debugInfo.devidNum > 100 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: udid num exceed maximum\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyUdid\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: udid is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyUdid\0".as_ptr() as *const ::core::ffi::c_char,
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
        unsafe { libc::free(udid as *mut ::core::ffi::c_void) };
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: get udid fail, ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyUdid\0".as_ptr() as *const ::core::ffi::c_char,
                395,
                result,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        for i in 0..(*pf).debugInfo.devidNum {
            let device_id_ptr = *((*pf).debugInfo.deviceId.offset(i as isize));
            if libc::strcmp(device_id_ptr as *const ::core::ffi::c_char, udid as *const ::core::ffi::c_char) == 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: find right udid\0".as_ptr() as *const ::core::ffi::c_char,
                    b"VerifyUdid\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: udid invalid\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerifyUdid\0".as_ptr() as *const ::core::ffi::c_char,
            406,
        );
    }
    unsafe { libc::free(udid as *mut ::core::ffi::c_void) };
    crate::types::V_ERR as i32
}