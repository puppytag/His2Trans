//! Module: src_hcb_config_entry
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

fn GetProductName(name: *mut std::ffi::c_char, maxLen: i32) -> i32 {
    unsafe {
        strcpy_s(name, maxLen as crate::types::size_t, b"default\0".as_ptr() as *const std::ffi::c_char)
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hcb_config_entry_2
// c_function: GetConfigFilePath
// rust_file: src_hcb_config_entry.rs
// rust_signature: fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool
// c_first_line: static _Bool GetConfigFilePath(const char *productName, char *configPath, size_t configPathLen)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_hcb_config_entry_2/translated_rust.rs
// last_error_truncated:
//   error[E0433]: failed to resolve: could not find `LOG_CORE` in `compat`
//     --> src/src_hcb_config_entry.rs:36:32
//      |
//      |                                ^^^^^^^^ could not find `LOG_CORE` in `compat`
//   error[E0433]: failed to resolve: could not find `LOG_ERROR` in `compat`
//     --> src/src_hcb_config_entry.rs:37:32
//      |
//      |                                ^^^^^^^^^ could not find `LOG_ERROR` in `compat`
// =================================
fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hcb_config_entry::GetConfigFilePath(productName as _, configPath as _, configPathLen as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hcb_config_entry_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_hcb_config_entry_2/translated_rust.rs
 * ------------------------------------------------------------
fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    let adapterConfigPath: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];
    let pathNum = adapterConfigPath.len();
    for i in 0..pathNum {
        if unsafe { crate::compat::sprintf_s(
            configPath,
            configPathLen - 1,
            b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char,
            adapterConfigPath[i],
            productName,
        ) } < 0 {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!() as crate::types::LogType,
                crate::compat::LOG_ERROR!() as crate::types::LogLevel,
                0xD002510,
                b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                b"failed to generate file path\0".as_ptr() as *const std::ffi::c_char,
            ) };
            continue;
        }
        if unsafe { crate::compat::access(configPath, crate::compat::F_OK!() | crate::compat::R_OK!()) } == 0 {
            return true;
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::compat::LOG_CORE!() as crate::types::LogType,
            crate::compat::LOG_DEBUG!() as crate::types::LogLevel,
            0xD002510,
            b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
            b"invalid config file path or permission:%{public}s\0".as_ptr() as *const std::ffi::c_char,
            configPath,
        ) };
    }
    false
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hcb_config_entry_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hcb_config_entry_3
// c_function: HdfGetHcsRootNode
// rust_file: src_hcb_config_entry.rs
// rust_signature: pub extern "C" fn HdfGetHcsRootNode() -> *const DeviceResourceNode
// c_first_line: const struct DeviceResourceNode *HdfGetHcsRootNode(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_hcb_config_entry_3/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `LOG_CORE` in this scope
//      --> src/src_hcb_config_entry.rs:100:28
//       |
//       |                            ^^^^^^^^
//   error[E0425]: cannot find value `LOG_ERROR` in this scope
//      --> src/src_hcb_config_entry.rs:100:41
//       |
//       |                                         ^^^^^^^^^ not found in this scope
// =================================
pub extern "C" fn HdfGetHcsRootNode() -> *const DeviceResourceNode {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hcb_config_entry::HdfGetHcsRootNode() as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hcb_config_entry_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_hcb_config_entry_3/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfGetHcsRootNode() -> *const DeviceResourceNode {
    let mut productName: [std::ffi::c_char; 128] = [0; 128];
    let mut configPath: [std::ffi::c_char; 256] = [0; 256];

    let ret = crate::src_hcb_config_entry::GetProductName(productName.as_mut_ptr(), 128);
    if ret != HDF_SUCCESS {
        return std::ptr::null();
    }

    if !crate::src_hcb_config_entry::GetConfigFilePath(productName.as_ptr(), configPath.as_mut_ptr(), 256) {
        let _ = HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, b"attribute_manager\0".as_ptr() as *const std::ffi::c_char, b"failed to get config file path\0".as_ptr() as *const std::ffi::c_char);
        return std::ptr::null();
    }

    unsafe {
        SetHcsBlobPath(configPath.as_ptr());
    }
    let mgrRoot = unsafe {
        HcsGetRootNode()
    };
    mgrRoot
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hcb_config_entry_3
 * === C2R_LLM_FAILED_OUTPUT_END === */

