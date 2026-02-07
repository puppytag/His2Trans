//! Module: src_app_provision
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

fn ProfInit(pf: *mut crate::types::ProfileProf) {
    if pf.is_null() {
        return;
    }
    let ret = unsafe { crate::compat::memset_s(pf as *mut std::ffi::c_void, std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t, 0, std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t) };
    if ret != crate::types::EOK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: memset failed\0".as_ptr() as *const std::ffi::c_char, b"ProfInit\0".as_ptr() as *const std::ffi::c_char, 35) };
        return;
    }
}

fn GetStringTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, tag) };
    if jsonObj.is_null() || unsafe { (*jsonObj).valuestring.is_null() } {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get %s\0".as_ptr() as *const _, b"GetStringTag\0".as_ptr() as *const _, 45, tag) };
        return std::ptr::null_mut();
    }
    let objLen = unsafe { libc::strlen((*jsonObj).valuestring) };
    if objLen < 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: len error\0".as_ptr() as *const _, b"GetStringTag\0".as_ptr() as *const _, 50) };
        return std::ptr::null_mut();
    }
    let value = unsafe { libc::malloc((objLen + 1) as usize) as *mut std::ffi::c_char };
    if value.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error: %d\0".as_ptr() as *const _, b"GetStringTag\0".as_ptr() as *const _, 55, objLen + 1) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::compat::strcpy_s(value, (objLen + 1) as crate::types::size_t, (*jsonObj).valuestring) };
    if ret != 0 {
        if !value.is_null() {
            unsafe { libc::free(value as *mut std::ffi::c_void) };
        }
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: strcpy error: %d\0".as_ptr() as *const _, b"GetStringTag\0".as_ptr() as *const _, 61, ret) };
        return std::ptr::null_mut();
    }
    value
}

fn FreeStringAttay(array: *mut *mut std::ffi::c_char, num: i32) {
    if array.is_null() {
        return;
    }
    for i in 0..num {
        unsafe {
            let elem = array.offset(i as isize);
            if !(*elem).is_null() {
                libc::free((*elem) as *mut std::ffi::c_void);
                *elem = std::ptr::null_mut();
            }
        }
    }
    unsafe {
        libc::free(array as *mut std::ffi::c_void);
    }
}

fn GetStringArrayTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char, numReturn: *mut i32) -> *mut *mut std::ffi::c_char {
    unsafe {
        let jsonObj = crate::compat::cJSON_GetObjectItem(root, tag);
        if jsonObj.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: failed to get %s\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 85, tag);
            return std::ptr::null_mut();
        }
        let num = crate::compat::cJSON_GetArraySize(jsonObj);
        if num == 0 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: array num 0\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 90);
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        let value = libc::malloc((num as usize).wrapping_mul(std::mem::size_of::<*mut std::ffi::c_char>())) as *mut *mut std::ffi::c_char;
        if value.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: value is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 96);
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        let dest_max = (num as usize).wrapping_mul(std::mem::size_of::<*mut std::ffi::c_char>()) as crate::types::size_t;
        let count = dest_max;
        let _ = crate::compat::memset_s(value as *mut std::ffi::c_void, dest_max, 0, count);
        for i in 0..num {
            let item = crate::compat::cJSON_GetArrayItem(jsonObj, i);
            if item.is_null() {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: item is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 104);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            if (*item).valuestring.is_null() {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: valuestring is NULL\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 106);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            let len = libc::strlen((*item).valuestring);
            *value.offset(i as isize) = libc::malloc(len.wrapping_add(1)) as *mut std::ffi::c_char;
            if (*value.offset(i as isize)).is_null() {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: value[i] is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 112);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            let ret = crate::compat::strcpy_s(*value.offset(i as isize), len.wrapping_add(1) as crate::types::size_t, (*item).valuestring);
            if ret != 0 {
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: str cpy error : %d\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 116, ret);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
        }
        *numReturn = num;
        value
    }
}

fn GetProfValidity(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfValidity) -> i32 {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, b"validity\0".as_ptr() as *const _) };
    if jsonObj.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get validity\0".as_ptr() as *const _, b"GetProfValidity\0".as_ptr() as *const _, 132) };
        return crate::types::V_ERR as i32;
    }
    let notBefore = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, b"not-before\0".as_ptr() as *const _) };
    if notBefore.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get not-before\0".as_ptr() as *const _, b"GetProfValidity\0".as_ptr() as *const _, 138) };
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).notBefore = (*notBefore).valueint;
    }
    let notAfter = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, b"not-after\0".as_ptr() as *const _) };
    if notAfter.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get not-after\0".as_ptr() as *const _, b"GetProfValidity\0".as_ptr() as *const _, 145) };
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).notAfter = (*notAfter).valueint;
    }
    crate::types::V_OK as i32
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_6
// c_function: GetProfBundleInfo
// rust_file: src_app_provision.rs
// rust_signature: fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32
// c_first_line: static int32_t GetProfBundleInfo(const cJSON *root, ProfBundleInfo *profVal)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_6/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_provision.rs:191:194
//       |
//       |                                                            ^^^^^^^^^^^^
//   error: cannot find macro `__FUNCTION__` in this scope
//      --> src/src_app_provision.rs:186:194
//       |
//       |                                                            ^^^^^^^^^^^^
// =================================
fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_app_provision::GetProfBundleInfo(root as _, profVal as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_6
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk5/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_6/translated_rust.rs
 * ------------------------------------------------------------
fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    use std::ffi::CString;
    let jsonObj = unsafe { cJSON_GetObjectItem(root, CString::new("bundle-info").unwrap().as_ptr()) };
    if jsonObj.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: failed to get bundle-info").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 156) };
        return V_ERR;
    }
    unsafe {
        (*profVal).developerId = crate::src_app_provision::GetStringTag(jsonObj, CString::new("developer-id").unwrap().as_ptr());
        if (*profVal).developerId.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->developerId is null").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 161);
            return V_ERR;
        }
        (*profVal).devCert = crate::src_app_provision::GetStringTag(jsonObj, CString::new("development-certificate").unwrap().as_ptr()) as *mut ::core::ffi::c_uchar;
        if (*profVal).devCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: get development-certificat failed").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 165);
            (*profVal).devCert = libc::malloc(1) as *mut ::core::ffi::c_uchar;
            if (*profVal).devCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->devCert is null").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 167);
                return V_ERR;
            }
            *(*profVal).devCert = 0;
        }
        (*profVal).releaseCert = crate::src_app_provision::GetStringTag(jsonObj, CString::new("distribution-certificate").unwrap().as_ptr()) as *mut ::core::ffi::c_uchar;
        if (*profVal).releaseCert.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: get distribution-certificat failed").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 173);
            (*profVal).releaseCert = libc::malloc(1) as *mut ::core::ffi::c_uchar;
            if (*profVal).releaseCert.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->releaseCert is null").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 175);
                return V_ERR;
            }
            *(*profVal).releaseCert = 0;
        }
        (*profVal).bundleName = crate::src_app_provision::GetStringTag(jsonObj, CString::new("bundle-name").unwrap().as_ptr());
        if (*profVal).bundleName.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->bundleName is null").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 180);
            return V_ERR;
        }
        (*profVal).appFeature = crate::src_app_provision::GetStringTag(jsonObj, CString::new("app-feature").unwrap().as_ptr());
        if (*profVal).appFeature.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, CString::new("appverify").unwrap().as_ptr(), CString::new("[%s:%d]: profVal->appFeature is null").unwrap().as_ptr(), CString::new(__FUNCTION__!()).unwrap().as_ptr(), 183);
            return V_ERR;
        }
    }
    V_OK
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_6
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32 {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, b"permissions\0".as_ptr() as *const _) };
    if jsonObj.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get permissions\0".as_ptr() as *const _, b"GetProfPermission\0".as_ptr() as *const _, 192) };
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).permission = crate::src_app_provision::GetStringArrayTag(jsonObj, b"feature-permissions\0".as_ptr() as *const _, &mut (*profVal).permissionNum);
        (*profVal).restricPermission = crate::src_app_provision::GetStringArrayTag(jsonObj, b"restricted-permissions\0".as_ptr() as *const _, &mut (*profVal).restricNum);
    }
    crate::types::V_OK as i32
}

fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32 {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, b"debug-info\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: failed to get debug-info\0".as_ptr() as *const std::ffi::c_char, b"GetProfDebugInfo\0".as_ptr() as *const std::ffi::c_char, 204) };
        return crate::types::V_OK as i32;
    }
    unsafe {
        (*profVal).devIdType = crate::src_app_provision::GetStringTag(jsonObj, b"device-id-type\0".as_ptr() as *const std::ffi::c_char);
    }
    if unsafe { (*profVal).devIdType.is_null() } {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: failed to get device-id-type\0".as_ptr() as *const std::ffi::c_char, b"GetProfDebugInfo\0".as_ptr() as *const std::ffi::c_char, 209) };
        return crate::types::V_OK as i32;
    }
    unsafe {
        (*profVal).deviceId = crate::src_app_provision::GetStringArrayTag(jsonObj, b"device-ids\0".as_ptr() as *const std::ffi::c_char, &mut (*profVal).devidNum);
    }
    crate::types::V_OK as i32
}

fn GetProfIssuerInfo(root: *const crate::types::cJSON, pf: *mut crate::types::ProfileProf) -> i32 {
    unsafe {
        (*pf).issuer = crate::src_app_provision::GetStringTag(root, b"issuer\0".as_ptr() as *const std::ffi::c_char);
        if (*pf).issuer.is_null() {
            let len = libc::strlen(b"Huawei App Store\0".as_ptr() as *const std::ffi::c_char);
            (*pf).issuer = libc::malloc((len + 1) as usize) as *mut std::ffi::c_char;
            if (*pf).issuer.is_null() {
                return crate::types::V_ERR as i32;
            }
            let ret = crate::compat::strcpy_s((*pf).issuer, (len + 1) as crate::types::size_t, b"Huawei App Store\0".as_ptr() as *const std::ffi::c_char);
            if ret != 0 {
                if !(*pf).issuer.is_null() {
                    libc::free((*pf).issuer as *mut std::ffi::c_void);
                    (*pf).issuer = std::ptr::null_mut();
                }
                let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: str cpy error: %d\0".as_ptr() as *const std::ffi::c_char, b"GetProfIssuerInfo\0".as_ptr() as *const std::ffi::c_char, 228, ret);
            }
            return ret;
        }
        crate::types::V_OK as i32
    }
}

fn FreeProfBundle(pfval: *mut crate::types::ProfBundleInfo) {
    if pfval.is_null() {
        return;
    }
    unsafe {
        if !(*pfval).appFeature.is_null() {
            libc::free((*pfval).appFeature as *mut std::ffi::c_void);
            (*pfval).appFeature = std::ptr::null_mut();
        }
        if !(*pfval).bundleName.is_null() {
            libc::free((*pfval).bundleName as *mut std::ffi::c_void);
            (*pfval).bundleName = std::ptr::null_mut();
        }
        if !(*pfval).devCert.is_null() {
            libc::free((*pfval).devCert as *mut std::ffi::c_void);
            (*pfval).devCert = std::ptr::null_mut();
        }
        if !(*pfval).developerId.is_null() {
            libc::free((*pfval).developerId as *mut std::ffi::c_void);
            (*pfval).developerId = std::ptr::null_mut();
        }
        if !(*pfval).releaseCert.is_null() {
            libc::free((*pfval).releaseCert as *mut std::ffi::c_void);
            (*pfval).releaseCert = std::ptr::null_mut();
        }
    }
}

fn FreeProfPerssion(pfval: *mut crate::types::ProfPermission) {
    unsafe {
        crate::src_app_provision::FreeStringAttay((*pfval).permission, (*pfval).permissionNum);
        (*pfval).permissionNum = 0;
        (*pfval).permission = std::ptr::null_mut();
        crate::src_app_provision::FreeStringAttay((*pfval).restricPermission, (*pfval).restricNum);
        (*pfval).restricNum = 0;
        (*pfval).restricPermission = std::ptr::null_mut();
    }
}

fn FreeProfDebuginfo(pfval: *mut crate::types::ProfDebugInfo) {
    if pfval.is_null() {
        return;
    }
    unsafe {
        if !(*pfval).devIdType.is_null() {
            libc::free((*pfval).devIdType as *mut std::ffi::c_void);
            (*pfval).devIdType = std::ptr::null_mut();
        }
        crate::src_app_provision::FreeStringAttay((*pfval).deviceId, (*pfval).devidNum);
        (*pfval).devidNum = 0;
        (*pfval).deviceId = std::ptr::null_mut();
    }
}

pub extern "C" fn ProfFreeData(pf: *mut crate::types::ProfileProf) {
    if pf.is_null() {
        return;
    }
    unsafe {
        if !(*pf).versionName.is_null() {
            libc::free((*pf).versionName as *mut ::core::ffi::c_void);
            (*pf).versionName = std::ptr::null_mut();
        }
        if !(*pf).uuid.is_null() {
            libc::free((*pf).uuid as *mut ::core::ffi::c_void);
            (*pf).uuid = std::ptr::null_mut();
        }
        if !(*pf).type_.is_null() {
            libc::free((*pf).type_ as *mut ::core::ffi::c_void);
            (*pf).type_ = std::ptr::null_mut();
        }
        if !(*pf).appDistType.is_null() {
            libc::free((*pf).appDistType as *mut ::core::ffi::c_void);
            (*pf).appDistType = std::ptr::null_mut();
        }
        crate::src_app_provision::FreeProfBundle(&mut (*pf).bundleInfo);
        crate::src_app_provision::FreeProfPerssion(&mut (*pf).permission);
        crate::src_app_provision::FreeProfDebuginfo(&mut (*pf).debugInfo);
        if !(*pf).issuer.is_null() {
            libc::free((*pf).issuer as *mut ::core::ffi::c_void);
            (*pf).issuer = std::ptr::null_mut();
        }
        if !(*pf).appid.is_null() {
            libc::free((*pf).appid as *mut ::core::ffi::c_void);
            (*pf).appid = std::ptr::null_mut();
        }
    }
}

pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::types::*;
    use ::core::ffi::c_void;
    use ::libc;
    if pf.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 288); }
        return V_ERR as i32;
    }
    if buf.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"buf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 289); }
        return V_ERR as i32;
    }
    unsafe { ProfInit(pf); }
    let pf_str = unsafe { libc::strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pfStr\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 293); }
        return V_ERR as i32;
    }
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"root\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 296); }
        return V_ERR as i32;
    }
    let json_obj = unsafe { cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const _) };
    if json_obj.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"jsonObj\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 299); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*json_obj).valueint; }
    unsafe { (*pf).versionName = GetStringTag(root, b"version-name\0".as_ptr() as *const _); }
    if unsafe { (*pf).versionName }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->versionName\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 303); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe { (*pf).uuid = GetStringTag(root, b"uuid\0".as_ptr() as *const _); }
    if unsafe { (*pf).uuid }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->uuid\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 306); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe { (*pf).type_ = GetStringTag(root, b"type\0".as_ptr() as *const _); }
    if unsafe { (*pf).type_ }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->type\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 309); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe { (*pf).appDistType = GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const _); }
    if unsafe { (*pf).appDistType }.is_null() {
        unsafe { (*pf).appDistType = libc::malloc(1) as *mut ::core::ffi::c_char; }
        if unsafe { (*pf).appDistType }.is_null() {
            unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->appDistType\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 314); }
            unsafe { cJSON_Delete(root); }
            unsafe { ProfFreeData(pf); }
            return V_ERR as i32;
        }
        unsafe { *(*pf).appDistType = 0; }
    }
    let mut ret = unsafe { GetProfValidity(root, &mut (*pf).validity) };
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 319); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfBundleInfo(root, &mut (*pf).bundleInfo) };
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 322); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfPermission(root, &mut (*pf).permission) };
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 325); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfDebugInfo(root, &mut (*pf).debugInfo) };
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 328); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfIssuerInfo(root, pf) };
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 331); }
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe { let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"parse profile json success\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 333); }
    unsafe { cJSON_Delete(root); }
    V_OK as i32
}

fn VerifyAppTypeAndDistribution(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let type_ptr = (*pf).type_;
        if libc::strcmp(type_ptr, b"debug\0".as_ptr() as *const i8) != 0 &&
           libc::strcmp(type_ptr, b"release\0".as_ptr() as *const i8) != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: invalid app type: %s\0".as_ptr() as *const i8,
                b"VerifyAppTypeAndDistribution\0".as_ptr() as *const i8,
                346,
                type_ptr,
            );
            return crate::types::V_ERR as i32;
        }
        if libc::strcmp(type_ptr, b"release\0".as_ptr() as *const i8) == 0 {
            let dist_ptr = (*pf).appDistType;
            if libc::strcmp(dist_ptr, crate::globals::APP_GALLERY.as_ptr() as *const i8) != 0 &&
               libc::strcmp(dist_ptr, crate::globals::ENTERPRISE.as_ptr() as *const i8) != 0 &&
               libc::strcmp(dist_ptr, crate::globals::ENTERPRISE_NORMAL.as_ptr() as *const i8) != 0 &&
               libc::strcmp(dist_ptr, crate::globals::ENTERPRISE_MDM.as_ptr() as *const i8) != 0 &&
               libc::strcmp(dist_ptr, crate::globals::INTERNALTESTING.as_ptr() as *const i8) != 0 &&
               libc::strcmp(dist_ptr, crate::globals::OS_INTEGRATION.as_ptr() as *const i8) != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: invalid app dis type: %s\0".as_ptr() as *const i8,
                    b"VerifyAppTypeAndDistribution\0".as_ptr() as *const i8,
                    353,
                    dist_ptr,
                );
                return crate::types::V_ERR as i32;
            }
        }
        crate::types::V_OK as i32
    }
}

fn VerifyAppBundleInfo(pf: *const crate::types::ProfileProf) -> i32 {
    use crate::types::{DEBUG_TYPE, RELEASE_TYPE, V_ERR, V_OK};
    unsafe {
        let type_ptr = (*pf).type_;
        if libc::strcmp(type_ptr, DEBUG_TYPE.as_ptr() as *const i8) == 0 {
            let dev_cert_ptr = (*pf).bundleInfo.devCert;
            if libc::strlen(dev_cert_ptr as *const i8) == 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: debug app, dev cert null\0".as_ptr() as *const i8,
                    b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                    364,
                );
                return V_ERR as i32;
            }
        } else if libc::strcmp(type_ptr, RELEASE_TYPE.as_ptr() as *const i8) == 0 {
            let release_cert_ptr = (*pf).bundleInfo.releaseCert;
            if libc::strlen(release_cert_ptr as *const i8) == 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: debug app, dev cert null\0".as_ptr() as *const i8,
                    b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                    369,
                );
                return V_ERR as i32;
            }
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: invalid app type: %s\0".as_ptr() as *const i8,
                b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                373,
                type_ptr,
            );
            return V_ERR as i32;
        }
        V_OK as i32
    }
}

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

fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let type_ptr = (*pf).type_;
        if libc::strcmp(type_ptr, b"debug\0".as_ptr() as *const i8) != 0 {
            let _ = HiLogPrint(
                LOG_CORE as u32,
                LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: not debug app, return ok\0".as_ptr() as *const i8,
                b"VerifyDebugInfo\0".as_ptr() as *const i8,
                415,
            );
            return V_OK as i32;
        }
        let dev_id_type_ptr = (*pf).debugInfo.devIdType;
        let _ = HiLogPrint(
            LOG_CORE as u32,
            LOG_INFO as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: devid type: %s\0".as_ptr() as *const i8,
            b"VerifyDebugInfo\0".as_ptr() as *const i8,
            418,
            dev_id_type_ptr,
        );
        let mut ret: i32;
        if libc::strcmp(dev_id_type_ptr, b"udid\0".as_ptr() as *const i8) == 0 {
            ret = crate::src_app_provision::VerifyUdid(pf);
        } else {
            let _ = HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: devid type invalid\0".as_ptr() as *const i8,
                b"VerifyDebugInfo\0".as_ptr() as *const i8,
                423,
            );
            ret = V_ERR as i32;
        }
        ret
    }
}

pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    if pf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: pf is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                431,
            );
        }
        return crate::types::V_ERR as i32;
    }
    let mut ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let app_dist_type = (*pf).appDistType;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile distribution type : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                434,
                app_dist_type,
            );
        }
        return crate::types::V_ERR_INVALID_DISP_TYPE as i32;
    }
    ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile app bundle info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                439,
            );
        }
        return crate::types::V_ERR_INVALID_APP_BUNDLE as i32;
    }
    ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: validate debug info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                445,
            );
        }
        return crate::types::V_ERR_INVALID_DEVID as i32;
    }
    crate::types::V_OK as i32
}
