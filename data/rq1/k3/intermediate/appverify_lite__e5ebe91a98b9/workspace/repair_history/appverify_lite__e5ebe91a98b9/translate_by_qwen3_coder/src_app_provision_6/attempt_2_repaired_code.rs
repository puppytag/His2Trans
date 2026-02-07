fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"bundle-info\0".as_ptr() as *const _) };
    if jsonObj.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get bundle-info\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 156) };
        return V_ERR as i32;
    }
    unsafe {
        (*profVal).developerId = crate::src_app_provision::GetStringTag(jsonObj, b"developer-id\0".as_ptr() as *const _);
        if (*profVal).developerId.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profVal->developerId is null\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 161);
            return V_ERR as i32;
        }
        (*profVal).devCert = crate::src_app_provision::GetStringTag(jsonObj, b"development-certificate\0".as_ptr() as *const _) as *mut _;
        if (*profVal).devCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get development-certificat failed\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 165);
            (*profVal).devCert = libc::malloc(1) as *mut _;
            if (*profVal).devCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profVal->devCert is null\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 167);
                return V_ERR as i32;
            }
            *(*profVal).devCert = 0;
        }
        (*profVal).releaseCert = crate::src_app_provision::GetStringTag(jsonObj, b"distribution-certificate\0".as_ptr() as *const _) as *mut _;
        if (*profVal).releaseCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get distribution-certificat failed\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 173);
            (*profVal).releaseCert = libc::malloc(1) as *mut _;
            if (*profVal).releaseCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profVal->releaseCert is null\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 175);
                return V_ERR as i32;
            }
            *(*profVal).releaseCert = 0;
        }
        (*profVal).bundleName = crate::src_app_provision::GetStringTag(jsonObj, b"bundle-name\0".as_ptr() as *const _);
        if (*profVal).bundleName.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profVal->bundleName is null\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 180);
            return V_ERR as i32;
        }
        (*profVal).appFeature = crate::src_app_provision::GetStringTag(jsonObj, b"app-feature\0".as_ptr() as *const _);
        if (*profVal).appFeature.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: profVal->appFeature is null\0".as_ptr() as *const _, b"GetProfBundleInfo\0".as_ptr() as *const _, 183);
            return V_ERR as i32;
        }
    }
    V_OK as i32
}