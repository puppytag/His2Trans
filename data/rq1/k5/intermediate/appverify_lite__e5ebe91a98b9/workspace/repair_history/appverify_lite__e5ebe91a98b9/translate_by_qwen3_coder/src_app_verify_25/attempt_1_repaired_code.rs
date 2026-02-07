pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_verify::{FreeAppSignPublicKey, GetAppSignPublicKey};
    if profile.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: profile is null\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                708,
            );
        }
        return crate::types::V_ERR as i32;
    }
    let mut pk = crate::types::AppSignPk {
        pk: std::ptr::null_mut(),
        len: 0,
    };
    let ret = GetAppSignPublicKey(profile as *const _, &mut pk);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get sign pk failed\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                712,
            );
        }
        return ret;
    }
    let mut use_len: crate::types::size_t = 0;
    unsafe {
        let _ = crate::compat::mbedtls_base64_encode(
            std::ptr::null_mut(),
            0,
            &mut use_len,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        );
    }
    let bundle_name_len = unsafe { libc::strlen((*profile).bundleInfo.bundleName) } as i32;
    let appid_len = bundle_name_len + use_len as i32 + 1 + 1;
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: GetAppid %d\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            721,
            appid_len,
        );
    }
    if use_len > 4096 {
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR as i32;
    }
    let appid = unsafe { libc::malloc(appid_len as usize) } as *mut i8;
    if appid.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc failed\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                727,
            );
        }
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_MALLOC as i32;
    }
    unsafe {
        *appid.offset((appid_len - 1) as isize) = 0;
    }
    let ret = unsafe {
        libc::snprintf(
            appid,
            appid_len as usize,
            b"%s_\0".as_ptr() as *const i8,
            (*profile).bundleInfo.bundleName,
        )
    };
    if ret < 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: snprintf error ret: %d\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                734,
                ret,
            );
        }
        if !appid.is_null() {
            unsafe { libc::free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    let ret = unsafe {
        crate::compat::mbedtls_base64_encode(
            appid.offset(bundle_name_len as isize + 1) as *mut u8,
            (appid_len - bundle_name_len - 1) as crate::types::size_t,
            &mut use_len,
            pk.pk as *mut u8,
            pk.len as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: base 64 encode error\0".as_ptr() as *const i8,
                b"GetAppid\0".as_ptr() as *const i8,
                742,
            );
        }
        if !appid.is_null() {
            unsafe { libc::free(appid as *mut std::ffi::c_void) };
        }
        FreeAppSignPublicKey(&mut pk);
        return crate::types::V_ERR_GET_APPID as i32;
    }
    unsafe {
        (*profile).appid = appid;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: appid len: %d, bL len: %d, base64: %d\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            748,
            appid_len,
            bundle_name_len,
            use_len as i32,
        );
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: %s\0".as_ptr() as *const i8,
            b"GetAppid\0".as_ptr() as *const i8,
            749,
            appid,
        );
    }
    FreeAppSignPublicKey(&mut pk);
    crate::types::V_OK as i32
}