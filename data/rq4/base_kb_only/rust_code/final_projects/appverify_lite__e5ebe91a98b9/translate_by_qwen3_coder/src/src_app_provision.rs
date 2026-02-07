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
    let ret: crate::types::errno_t = unsafe {
        crate::compat::memset_s(
            pf as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t,
            0,
            std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t,
        )
    };
    if ret != crate::types::EOK as crate::types::errno_t {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: memset failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"ProfInit\0".as_ptr() as *const ::core::ffi::c_char,
                35i32,
            );
        }
        return;
    }
    return;
}

fn GetStringTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    unsafe {
        let jsonObj = crate::compat::cJSON_GetObjectItem(root, tag);
        if jsonObj.is_null() || (*jsonObj).valuestring.is_null() {
            return std::ptr::null_mut();
        }
        let objLen = libc::strlen((*jsonObj).valuestring) as i32;
        if objLen < 0 {
            return std::ptr::null_mut();
        }
        let value = libc::malloc((objLen + 1) as usize) as *mut std::ffi::c_char;
        if value.is_null() {
            return std::ptr::null_mut();
        }
        let ret = crate::compat::strcpy_s(value, (objLen + 1) as crate::types::size_t, (*jsonObj).valuestring);
        if ret != 0 {
            if !value.is_null() {
                libc::free(value as *mut std::ffi::c_void);
            }
            return std::ptr::null_mut();
        }
        value
    }
}

fn FreeStringAttay(array: *mut *mut std::ffi::c_char, num: i32) {
    if array.is_null() {
        return;
    }
    for i in 0..num {
        unsafe {
            let elem = *array.offset(i as isize);
            if !elem.is_null() {
                libc::free(elem as *mut std::ffi::c_void);
                *array.offset(i as isize) = std::ptr::null_mut();
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
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get %s\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                85i32,
                tag,
            );
            return std::ptr::null_mut();
        }
        
        let num = crate::compat::cJSON_GetArraySize(jsonObj);
        if num == 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: array num 0\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                90i32,
            );
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        
        let alloc_size = (std::mem::size_of::<*mut std::ffi::c_char>() * num as usize) as usize;
        let value = libc::malloc(alloc_size) as *mut *mut std::ffi::c_char;
        if value.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: value is null\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                96i32,
            );
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        
        let _ = crate::compat::memset_s(value as *mut std::ffi::c_void, alloc_size as crate::types::size_t, 0, alloc_size as crate::types::size_t);
        
        for i in 0..num {
            let item = crate::compat::cJSON_GetArrayItem(jsonObj, i);
            if item.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: item is null\0".as_ptr() as *const std::ffi::c_char,
                    b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                    104i32,
                );
                FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            
            if (*item).valuestring.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: valuestring is NULL\0".as_ptr() as *const std::ffi::c_char,
                    b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                    106i32,
                );
                FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            
            let len = libc::strlen((*item).valuestring) as i32;
            *value.offset(i as isize) = libc::malloc((len + 1) as usize) as *mut std::ffi::c_char;
            if (*value.offset(i as isize)).is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: value[i] is null\0".as_ptr() as *const std::ffi::c_char,
                    b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                    112i32,
                );
                FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            
            let ret = crate::compat::strcpy_s(*value.offset(i as isize), (len + 1) as crate::types::size_t, (*item).valuestring);
            if ret != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: str cpy error : %d\0".as_ptr() as *const std::ffi::c_char,
                    b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char,
                    116i32,
                    ret,
                );
                FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
        }
        
        *numReturn = num;
        value
    }
}

fn GetProfValidity(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfValidity) -> i32 {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, b"validity\0".as_ptr() as *const ::core::ffi::c_char) };
    if jsonObj.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: failed to get validity\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetProfValidity\0".as_ptr() as *const ::core::ffi::c_char,
                132i32,
            );
        }
        return crate::types::V_ERR as i32;
    }

    let notBefore = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, b"not-before\0".as_ptr() as *const ::core::ffi::c_char) };
    if notBefore.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: failed to get not-before\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetProfValidity\0".as_ptr() as *const ::core::ffi::c_char,
                138i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).notBefore = (*notBefore).valueint;
    }

    let notAfter = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, b"not-after\0".as_ptr() as *const ::core::ffi::c_char) };
    if notAfter.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: failed to get not-after\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetProfValidity\0".as_ptr() as *const ::core::ffi::c_char,
                145i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).notAfter = (*notAfter).valueint;
    }
    
    crate::types::V_OK as i32
}

fn GetProfBundleInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfBundleInfo) -> i32 {
    unsafe {
        let jsonObj = crate::compat::cJSON_GetObjectItem(root, b"bundle-info\0".as_ptr() as *const std::ffi::c_char);
        if jsonObj.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get bundle-info\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                156 as i32,
            );
            return crate::types::V_ERR as i32;
        }

        (*profVal).developerId = crate::src_app_provision::GetStringTag(jsonObj, b"developer-id\0".as_ptr() as *const std::ffi::c_char);
        if (*profVal).developerId.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: profVal->developerId is null\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                161 as i32,
            );
            return crate::types::V_ERR as i32;
        }

        (*profVal).devCert = crate::src_app_provision::GetStringTag(jsonObj, b"development-certificate\0".as_ptr() as *const std::ffi::c_char) as *mut u8;
        if (*profVal).devCert.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: get development-certificat failed\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                165 as i32,
            );
            (*profVal).devCert = libc::malloc(core::mem::size_of::<std::ffi::c_char>() as usize) as *mut u8;
            if (*profVal).devCert.is_null() {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: profVal->devCert is null\0".as_ptr() as *const std::ffi::c_char,
                    b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                    167 as i32,
                );
                return crate::types::V_ERR as i32;
            }
            *(*profVal).devCert = 0;
        }

        (*profVal).releaseCert = crate::src_app_provision::GetStringTag(jsonObj, b"distribution-certificate\0".as_ptr() as *const std::ffi::c_char) as *mut u8;
        if (*profVal).releaseCert.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: get distribution-certificat failed\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                173 as i32,
            );
            (*profVal).releaseCert = libc::malloc(core::mem::size_of::<std::ffi::c_char>() as usize) as *mut u8;
            if (*profVal).releaseCert.is_null() {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: profVal->releaseCert is null\0".as_ptr() as *const std::ffi::c_char,
                    b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                    175 as i32,
                );
                return crate::types::V_ERR as i32;
            }
            *(*profVal).releaseCert = 0;
        }

        (*profVal).bundleName = crate::src_app_provision::GetStringTag(jsonObj, b"bundle-name\0".as_ptr() as *const std::ffi::c_char);
        if (*profVal).bundleName.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: profVal->bundleName is null\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                180 as i32,
            );
            return crate::types::V_ERR as i32;
        }

        (*profVal).appFeature = crate::src_app_provision::GetStringTag(jsonObj, b"app-feature\0".as_ptr() as *const std::ffi::c_char);
        if (*profVal).appFeature.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: profVal->appFeature is null\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfBundleInfo\0".as_ptr() as *const std::ffi::c_char,
                183 as i32,
            );
            return crate::types::V_ERR as i32;
        }

        crate::types::V_OK as i32
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_7
// c_function: GetProfPermission
// rust_file: src_app_provision.rs
// rust_signature: fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32
// c_first_line: static int32_t GetProfPermission(const cJSON *root, ProfPermission *profVal)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_7/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:216:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_7
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_7/translated_rust.rs
 * ------------------------------------------------------------
fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"permissions\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).permission = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"feature-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).permissionNum,
        );
        (*profVal).restricPermission = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"restricted-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).restricNum,
        );
    }
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_8
// c_function: GetProfDebugInfo
// rust_file: src_app_provision.rs
// rust_signature: fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32
// c_first_line: static int32_t GetProfDebugInfo(const cJSON *root, ProfDebugInfo *profVal)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_8/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:216:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_8
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_8/translated_rust.rs
 * ------------------------------------------------------------
fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"debug-info\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        return crate::types::V_OK as i32;
    }
    
    let devIdType = crate::src_app_provision::GetStringTag(jsonObj as *const crate::types::cJSON, b"device-id-type\0".as_ptr() as *const std::ffi::c_char);
    unsafe { (*profVal).devIdType = devIdType; }
    
    if devIdType.is_null() {
        return crate::types::V_OK as i32;
    }
    
    unsafe {
        (*profVal).deviceId = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"device-ids\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).devidNum,
        );
    }
    
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_8
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn GetProfIssuerInfo(root: *const crate::types::cJSON, pf: *mut crate::types::ProfileProf) -> i32 {
    unsafe {
        (*pf).issuer = crate::src_app_provision::GetStringTag(root, b"issuer\0".as_ptr() as *const std::ffi::c_char);
        if (*pf).issuer.is_null() {
            let len: i32 = libc::strlen(b"Huawei App Store\0".as_ptr() as *const std::ffi::c_char) as i32;
            (*pf).issuer = libc::malloc((len + 1) as usize) as *mut std::ffi::c_char;
            if (*pf).issuer.is_null() {
                return crate::types::V_ERR as i32;
            }
            let ret: crate::types::errno_t = crate::compat::strcpy_s(
                (*pf).issuer,
                (len + 1) as crate::types::size_t,
                b"Huawei App Store\0".as_ptr() as *const std::ffi::c_char,
            );
            if ret != 0 {
                if !(*pf).issuer.is_null() {
                    libc::free((*pf).issuer as *mut std::ffi::c_void);
                    (*pf).issuer = std::ptr::null_mut();
                }
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const std::ffi::c_char,
                    b"[%s:%d]: str cpy error: %d\0".as_ptr() as *const std::ffi::c_char,
                    b"GetProfIssuerInfo\0".as_ptr() as *const std::ffi::c_char,
                    228i32,
                    ret,
                );
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
            libc::free((*pfval).appFeature as *mut ::core::ffi::c_void);
            (*pfval).appFeature = std::ptr::null_mut();
        }
        if !(*pfval).bundleName.is_null() {
            libc::free((*pfval).bundleName as *mut ::core::ffi::c_void);
            (*pfval).bundleName = std::ptr::null_mut();
        }
        if !(*pfval).devCert.is_null() {
            libc::free((*pfval).devCert as *mut ::core::ffi::c_void);
            (*pfval).devCert = std::ptr::null_mut();
        }
        if !(*pfval).developerId.is_null() {
            libc::free((*pfval).developerId as *mut ::core::ffi::c_void);
            (*pfval).developerId = std::ptr::null_mut();
        }
        if !(*pfval).releaseCert.is_null() {
            libc::free((*pfval).releaseCert as *mut ::core::ffi::c_void);
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
    unsafe {
        if !(*pfval).devIdType.is_null() {
            libc::free((*pfval).devIdType as *mut std::ffi::c_void);
            (*pfval).devIdType = std::ptr::null_mut();
        }

        FreeStringAttay((*pfval).deviceId, (*pfval).devidNum);
        (*pfval).devidNum = 0;
        (*pfval).deviceId = std::ptr::null_mut();
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_13
// c_function: ProfFreeData
// rust_file: src_app_provision.rs
// rust_signature: pub extern "C" fn ProfFreeData(pf: *mut crate::types::ProfileProf)
// c_first_line: void ProfFreeData(ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_13/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:216:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
pub extern "C" fn ProfFreeData(pf: *mut crate::types::ProfileProf) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_13
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_13/translated_rust.rs
 * ------------------------------------------------------------
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
        
        FreeProfBundle(&mut (*pf).bundleInfo as *mut crate::types::ProfBundleInfo);
        
        FreeProfPerssion(&mut (*pf).permission as *mut crate::types::ProfPermission);
        
        FreeProfDebuginfo(&mut (*pf).debugInfo as *mut crate::types::ProfDebugInfo);
        
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
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_13
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_14
// c_function: ParseProfile
// rust_file: src_app_provision.rs
// rust_signature: pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32
// c_first_line: int32_t ParseProfile(const char *buf, int32_t len, ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_14/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:373:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_14
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_14/translated_rust.rs
 * ------------------------------------------------------------
Looking at the error, it's occurring at `src/src_app_verify.rs:373:55` - the error says `no method named 'as_ptr' found for type 'i32'`. This suggests there's code elsewhere in the file that has this issue, not in the `ParseProfile` function I'm translating.

However, looking at the previous attempts, the issue seems to be with the syntax `(version_code_str.as_ptr)()` which is incorrect. The correct syntax is `version_code_str.as_ptr()`.

Let me provide a clean translation:

pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let _ = len;
    
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    if buf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    ProfInit(pf);
    
    let pf_str = unsafe { libc::strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let json_obj = unsafe { cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const ::core::ffi::c_char) };
    if json_obj.is_null() {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*json_obj).valueint; }
    
    unsafe { (*pf).versionName = GetStringTag(root, b"version-name\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).versionName.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).uuid = GetStringTag(root, b"uuid\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).uuid.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).type_ = GetStringTag(root, b"type\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).type_.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).appDistType = GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).appDistType.is_null() } {
        unsafe { (*pf).appDistType = libc::malloc(core::mem::size_of::<i8>()) as *mut i8; }
        if unsafe { (*pf).appDistType.is_null() } {
            unsafe { cJSON_Delete(root); ProfFreeData(pf); }
            return crate::types::V_ERR as i32;
        }
        unsafe { *(*pf).appDistType = 0; }
    }
    
    let mut ret = GetProfValidity(root, unsafe { &mut (*pf).validity as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfBundleInfo(root, unsafe { &mut (*pf).bundleInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfPermission(root, unsafe { &mut (*pf).permission as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfDebugInfo(root, unsafe { &mut (*pf).debugInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfIssuerInfo(root, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { cJSON_Delete(root); }
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_14
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn VerifyAppTypeAndDistribution(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let debug_str = b"debug\0".as_ptr() as *const i8;
        let release_str = b"release\0".as_ptr() as *const i8;
        
        let pf_type = (*pf).type_;
        
        if libc::strcmp(pf_type, debug_str) != 0 && libc::strcmp(pf_type, release_str) != 0 {
            return crate::types::V_ERR as i32;
        }
        
        if libc::strcmp(pf_type, release_str) == 0 {
            let app_dist_type = (*pf).appDistType;
            
            if libc::strcmp(app_dist_type, crate::globals::APP_GALLERY.as_ptr()) != 0 &&
               libc::strcmp(app_dist_type, crate::globals::ENTERPRISE.as_ptr()) != 0 &&
               libc::strcmp(app_dist_type, crate::globals::ENTERPRISE_NORMAL.as_ptr()) != 0 &&
               libc::strcmp(app_dist_type, crate::globals::ENTERPRISE_MDM.as_ptr()) != 0 &&
               libc::strcmp(app_dist_type, crate::globals::INTERNALTESTING.as_ptr()) != 0 &&
               libc::strcmp(app_dist_type, crate::globals::OS_INTEGRATION.as_ptr()) != 0 {
                return crate::types::V_ERR as i32;
            }
        }
        
        crate::types::V_OK as i32
    }
}

fn VerifyAppBundleInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let debug_type = b"debug\0".as_ptr() as *const i8;
        let release_type = b"release\0".as_ptr() as *const i8;
        
        if libc::strcmp((*pf).type_, debug_type) == 0 {
            if libc::strlen((*pf).bundleInfo.devCert as *const i8) == 0 {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: debug app, dev cert null\0".as_ptr() as *const i8,
                    b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                    364i32,
                );
                return crate::types::V_ERR as i32;
            }
        } else if libc::strcmp((*pf).type_, release_type) == 0 {
            if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) == 0 {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: debug app, dev cert null\0".as_ptr() as *const i8,
                    b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                    369i32,
                );
                return crate::types::V_ERR as i32;
            }
        } else {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: invalid app type: %s\0".as_ptr() as *const i8,
                b"VerifyAppBundleInfo\0".as_ptr() as *const i8,
                373i32,
                (*pf).type_,
            );
            return crate::types::V_ERR as i32;
        }
        
        crate::types::V_OK as i32
    }
}

fn VerifyUdid(pf: *const crate::types::ProfileProf) -> i32 {
    let size: u32 = crate::types::UDID_VERIFY_BYTES + 1;
    
    unsafe {
        if (*pf).debugInfo.devidNum > crate::types::MAX_UDID_NUM as i32 {
            return crate::types::V_ERR as i32;
        }
        
        let udid = libc::malloc(size as usize) as *mut ::core::ffi::c_uchar;
        if udid.is_null() {
            return crate::types::V_ERR as i32;
        }
        
        let _ = memset_s(
            udid as *mut ::core::ffi::c_void,
            size as crate::types::size_t,
            0,
            size as crate::types::size_t,
        );
        
        let result = crate::src_app_verify_hal::InquiryDeviceUdid(udid, size as i32);
        if result != crate::types::INQUIRY_UDID_OK as i32 {
            libc::free(udid as *mut ::core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        let devid_num = (*pf).debugInfo.devidNum;
        for i in 0..devid_num {
            let device_id = *(*pf).debugInfo.deviceId.offset(i as isize);
            if libc::strcmp(
                device_id as *const ::core::ffi::c_char,
                udid as *const ::core::ffi::c_char,
            ) == 0
            {
                libc::free(udid as *mut ::core::ffi::c_void);
                return crate::types::V_OK as i32;
            }
        }
        
        libc::free(udid as *mut ::core::ffi::c_void);
        crate::types::V_ERR as i32
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_18
// c_function: VerifyDebugInfo
// rust_file: src_app_provision.rs
// rust_signature: fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32
// c_first_line: static int32_t VerifyDebugInfo(const ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_18/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:216:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_18
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_18/translated_rust.rs
 * ------------------------------------------------------------
fn VerifyDebugInfo(pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let debug_str: *const i8 = b"debug\0".as_ptr() as *const i8;
        if libc::strcmp((*pf).type_, debug_str) != 0 {
            return crate::types::V_OK as i32;
        }
        let ret: i32;
        let udid_str: *const i8 = b"udid\0".as_ptr() as *const i8;
        if libc::strcmp((*pf).debugInfo.devIdType, udid_str) == 0 {
            ret = crate::src_app_provision::VerifyUdid(pf);
        } else {
            ret = crate::types::V_ERR as i32;
        }
        ret
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_18
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_provision_19
// c_function: VerifyProfileContent
// rust_file: src_app_provision.rs
// rust_signature: pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32
// c_first_line: int32_t VerifyProfileContent(const ProfileProf *pf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_19/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:373:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_provision_19
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_provision_19/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let mut ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_DISP_TYPE as i32;
    }
    
    ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_APP_BUNDLE as i32;
    }
    
    ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_DEVID as i32;
    }
    
    let _ = ret;
    crate::types::V_OK as i32
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_provision_19
 * === C2R_LLM_FAILED_OUTPUT_END === */

