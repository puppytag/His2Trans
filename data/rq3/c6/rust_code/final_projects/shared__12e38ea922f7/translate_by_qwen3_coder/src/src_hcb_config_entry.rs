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
        crate::compat::strcpy_s(
            name,
            maxLen as usize,
            b"default\0".as_ptr() as *const std::ffi::c_char,
        ) as i32
    }
}

fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    static ADAPTER_CONFIG_PATH: [&[u8]; 2] = [
        b"/vendor/etc/hdfconfig\0",
        b"/chip_prod/etc/hdfconfig\0",
    ];

    let path_num: usize = 2;
    
    for i in 0..path_num {
        let format_str = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
        let path_ptr = ADAPTER_CONFIG_PATH[i].as_ptr() as *const std::ffi::c_char;
        
        let ret = unsafe {
            crate::compat::sprintf_s(
                configPath,
                configPathLen - 1,
                format_str,
                path_ptr,
                productName
            )
        };
        
        if ret < 0 {
            continue;
        }
        
        let f_ok: i32 = 0;
        let r_ok: i32 = 4;
        
        if unsafe { crate::compat::access(configPath, f_ok | r_ok) } == 0 {
            return true;
        }
    }
    
    false
}

pub extern "C" fn HdfGetHcsRootNode() -> *const DeviceResourceNode {
    let mut productName: [std::ffi::c_char; 128] = [0; 128];
    let mut configPath: [std::ffi::c_char; 256] = [0; 256];

    let ret = crate::src_hcb_config_entry::GetProductName(productName.as_mut_ptr(), 128);
    if ret != HDF_SUCCESS {
        return std::ptr::null();
    }

    if !crate::src_hcb_config_entry::GetConfigFilePath(
        productName.as_ptr(),
        configPath.as_mut_ptr(),
        256,
    ) {
        // HiLogPrint call omitted as it's a logging macro
        return std::ptr::null();
    }

    unsafe {
        SetHcsBlobPath(configPath.as_ptr());
        let mgrRoot = HcsGetRootNode();
        mgrRoot
    }
}
