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

fn GetProductName(name: &mut [::core::ffi::c_char], maxLen: i32) -> i32 {
    const SRC: &[::core::ffi::c_char] = &[100, 101, 102, 97, 117, 108, 116, 0]; // b"default\0"
    let src_len = SRC.len();
    let dest_len = maxLen as usize;
    if maxLen <= 0 || dest_len < src_len {
        if !name.is_empty() {
            name[0] = 0;
        }
        return 1;
    }
    name[..src_len].copy_from_slice(SRC);
    0
}

fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize)-> bool {
    if productName.is_null() || configPath.is_null() {
        return false;
    }
    let adapter_paths: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];
    let format_str = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
    let dest_max = configPathLen.saturating_sub(1);
    if dest_max == 0 {
        // HiLogPrint(LOG_CORE, LOG_ERROR, ...) - unavailable
        return false;
    }
    for &host in &adapter_paths {
        let written = unsafe {
            ::libc::snprintf(
                configPath,
                dest_max as ::libc::size_t,
                format_str,
                host,
                productName,
            )
        };
        if written < 0 || (written as usize) >= dest_max {
            // HiLogPrint(LOG_CORE, LOG_ERROR, ...) - unavailable
            continue;
        }
        if unsafe { ::libc::access(configPath as *const std::ffi::c_char, 4) } == 0 {
            return true;
        }
        // HiLogPrint(LOG_CORE, LOG_DEBUG, ...) - unavailable
    }
    false
}

pub extern "C" fn HdfGetHcsRootNode() -> *const crate::types::DeviceResourceNode {
    let mut productName: [::core::ffi::c_char; 128] = [0; 128];
    let mut configPath: [::core::ffi::c_char; 4096] = [0; 4096];

    let ret = crate::src_hcb_config_entry::GetProductName(&mut productName, 128i32);
    if ret != HDF_SUCCESS {
        return std::ptr::null();
    }

    let get_config_ok = crate::src_hcb_config_entry::GetConfigFilePath(
        productName.as_ptr(),
        configPath.as_mut_ptr(),
        4096usize,
    );
    if !get_config_ok {
        // HiLogPrint unavailable: skip logging and return null.
        return std::ptr::null();
    }

    // SetHcsBlobPath and get root node
    unsafe {
        SetHcsBlobPath(configPath.as_ptr());
        HcsGetRootNode()
    }
}
