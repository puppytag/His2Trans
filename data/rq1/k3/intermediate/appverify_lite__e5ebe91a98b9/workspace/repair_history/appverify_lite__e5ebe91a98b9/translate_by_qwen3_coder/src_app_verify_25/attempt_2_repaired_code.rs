pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::{FreeAppSignPublicKey, GetAppSignPublicKey};
    use crate::types::{AppSignPk, ProfileProf};
    use ::core::ffi::{c_char, c_uchar};
    use libc::{free, malloc, strlen};
    use std::ptr::null_mut;

    if profile.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const c_char,
                "[%s:%d]: profile is null\0".as_ptr() as *const c_char,
                "GetAppid\0".as_ptr() as *const c_char,
                708,
            );
        }
        return crate::types::V_ERR as i32;
    }
    let mut pk = AppSignPk {
        pk: null_mut(),
        len: 0,
    };
    let ret = GetAppSignPublicKey(profile as *const ProfileProf, &mut pk as *mut AppSignPk);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const c_char,
                "[%s:%d]: get sign pk failed\0".as_ptr() as *const c_char,
                "GetAppid\0".as_ptr() as *const c_char,
                712,
            );
        }
        return ret;
    }
    let mut use_len: crate::types::size_t = 0;
    unsafe {
        crate::compat::mbedtls_base64_encode(
            null_mut(),
            0,
            &mut use_len as *mut crate::types::size_t,
            pk.pk as *mut c_uchar,
            pk.len as crate::types::size_t,
        );
    }
    let bundle_name_len = unsafe { strlen((*profile).bundleInfo.bundleName as *const c_char) } as i32;
    let appid_len = bundle_name_len + use_len as i32 + 1 + 1;
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const c_char,
            "[%s:%d]: GetAppid %d\0".as_ptr() as *const c_char,
            "GetAppid\0".as_ptr() as *const c_char,
            721,
            appid_len,
        );
    }
    if use_len > 4096 {
        FreeAppSignPublicKey(&mut pk as *mut AppSignPk);
        return crate::types::V_ERR as i32;
    }
    let appid = unsafe { malloc(appid_len as usize) as *mut c_char };
    if appid.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const c_char,
                "[%s:%d]: malloc failed\0".as_ptr() as *const c_char,
                "GetAppid\0".as_ptr() as *const c_char,
                727,
            );
        }
        FreeAppSignPublicKey(&mut pk as *mut AppSignPk);
        return crate::types::V_ERR_MALLOC as i32;
    }
    unsafe {
        *appid.offset((appid_len - 1) as isize) = 0;
    }
    let ret = unsafe {
        libc::snprintf(
            appid,
            appid_len as usize,
            "%s_\0".as_ptr() as *const c_char,
            (*profile).bundleInfo.bundleName,
        )
    };
    if ret < 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const c_char,
                "[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const c_char,
                "GetAppid\0".as_ptr() as *const c_char,
                734,
                ret,
            );
        }
        if !appid.is_null() {
            unsafe { free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk as *mut AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    let ret = unsafe {
        crate::compat::mbedtls_base64_encode(
            appid.offset((bundle_name_len + 1) as isize) as *mut c_uchar,
            (appid_len - bundle_name_len - 1) as crate::types::size_t,
            &mut use_len as *mut crate::types::size_t,
            pk.pk as *mut c_uchar,
            pk.len as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const c_char,
                "[%s:%d]: base 64 encode error\0".as_ptr() as *const c_char,
                "GetAppid\0".as_ptr() as *const c_char,
                742,
            );
        }
        if !appid.is_null() {
            unsafe { free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk as *mut AppSignPk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    unsafe {
        (*profile).appid = appid;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const c_char,
            "[%s:%d]: appid len: %d, bL len: %d, base64: %d\0".as_ptr() as *const c_char,
            "GetAppid\0".as_ptr() as *const c_char,
            748,
            appid_len,
            bundle_name_len,
            use_len as i32,
        );
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const c_char,
            "[%s:%d]: %s\0".as_ptr() as *const c_char,
            "GetAppid\0".as_ptr() as *const c_char,
            749,
            appid,
        );
    }
    FreeAppSignPublicKey(&mut pk as *mut AppSignPk);
    crate::types::V_OK as i32
}