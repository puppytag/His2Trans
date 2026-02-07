pub extern "C" fn GetAppid(profile: *mut crate::types::ProfileProf) -> i32 {
    use crate::types::*;
    
    if profile.is_null() {
        return V_ERR as i32;
    }
    
    let mut pk = AppSignPk {
        pk: std::ptr::null_mut(),
        len: 0,
    };
    
    let ret = crate::src_app_verify::GetAppSignPublicKey(profile as *const ProfileProf, &mut pk);
    if ret != V_OK as i32 {
        return ret;
    }
    
    let mut use_len: size_t = 0;
    unsafe {
        crate::compat::mbedtls_base64_encode(
            std::ptr::null_mut(),
            0,
            &mut use_len,
            pk.pk as *mut u8,
            pk.len as size_t,
        );
    }
    
    let bundle_name_len = unsafe { libc::strlen((*profile).bundleInfo.bundleName) } as i32;
    let appid_len = bundle_name_len + use_len as i32 + 1 + 1;
    
    if use_len > MAX_KEY_PAIR_SIZE {
        return V_ERR as i32;
    }
    
    let appid = unsafe { libc::malloc(appid_len as usize) as *mut ::core::ffi::c_char };
    if appid.is_null() {
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return V_ERR_MALLOC as i32;
    }
    
    unsafe {
        *appid.offset((appid_len - 1) as isize) = 0;
    }
    
    let fmt_str: &[u8] = b"%s_\0";
    let ret = unsafe {
        crate::compat::snprintf_s(
            appid,
            appid_len as size_t,
            (bundle_name_len + 1) as size_t,
            fmt_str.as_ptr() as *const ::core::ffi::c_char,
            (*profile).bundleInfo.bundleName,
        )
    };
    
    if ret < 0 {
        unsafe {
            libc::free(appid as *mut ::core::ffi::c_void);
        }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return V_ERR_GET_APPID as i32;
    }
    
    let mut use_len_out: size_t = 0;
    let ret = unsafe {
        crate::compat::mbedtls_base64_encode(
            appid.offset((bundle_name_len + 1) as isize) as *mut u8,
            (appid_len - bundle_name_len - 1) as size_t,
            &mut use_len_out,
            pk.pk as *mut u8,
            pk.len as size_t,
        )
    };
    
    if ret != V_OK as i32 {
        unsafe {
            libc::free(appid as *mut ::core::ffi::c_void);
        }
        crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
        return V_ERR_GET_APPID as i32;
    }
    
    unsafe {
        (*profile).appid = appid;
    }
    
    crate::src_app_verify::FreeAppSignPublicKey(&mut pk);
    V_OK as i32
}