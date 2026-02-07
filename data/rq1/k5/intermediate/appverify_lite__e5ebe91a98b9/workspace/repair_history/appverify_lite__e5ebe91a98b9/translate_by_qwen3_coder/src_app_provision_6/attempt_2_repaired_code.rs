fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    use std::ffi::CString;
    let jsonObj = unsafe { cJSON_GetObjectItem(root, CString::new("bundle-info").unwrap().as_ptr()) };
    if jsonObj.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: failed to get bundle-info").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 156) };
        return V_ERR;
    }
    unsafe {
        (*profVal).developerId = crate::src_app_provision::GetStringTag(jsonObj, CString::new("developer-id").unwrap().as_ptr());
        if (*profVal).developerId.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->developerId is null").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 161);
            return V_ERR;
        }
        (*profVal).devCert = crate::src_app_provision::GetStringTag(jsonObj, CString::new("development-certificate").unwrap().as_ptr()) as *mut ::core::ffi::c_uchar;
        if (*profVal).devCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: get development-certificat failed").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 165);
            (*profVal).devCert = libc::malloc(1) as *mut ::core::ffi::c_uchar;
            if (*profVal).devCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->devCert is null").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 167);
                return V_ERR;
            }
            *(*profVal).devCert = 0;
        }
        (*profVal).releaseCert = crate::src_app_provision::GetStringTag(jsonObj, CString::new("distribution-certificate").unwrap().as_ptr()) as *mut ::core::ffi::c_uchar;
        if (*profVal).releaseCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: get distribution-certificat failed").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 173);
            (*profVal).releaseCert = libc::malloc(1) as *mut ::core::ffi::c_uchar;
            if (*profVal).releaseCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->releaseCert is null").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 175);
                return V_ERR;
            }
            *(*profVal).releaseCert = 0;
        }
        (*profVal).bundleName = crate::src_app_provision::GetStringTag(jsonObj, CString::new("bundle-name").unwrap().as_ptr());
        if (*profVal).bundleName.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->bundleName is null").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 180);
            return V_ERR;
        }
        (*profVal).appFeature = crate::src_app_provision::GetStringTag(jsonObj, CString::new("app-feature").unwrap().as_ptr());
        if (*profVal).appFeature.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->appFeature is null").unwrap().as_ptr(), CString::new(__FUNCTION__).unwrap().as_ptr(), 183);
            return V_ERR;
        }
    }
    V_OK
}