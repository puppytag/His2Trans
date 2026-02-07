fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    static ADAPTER_CONFIG_PATH: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];

    let path_num: usize = 2;
    
    for i in 0..path_num {
        let format_str = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
        let result = unsafe {
            crate::compat::sprintf_s(
                configPath,
                configPathLen - 1,
                format_str,
                ADAPTER_CONFIG_PATH[i],
                productName,
            )
        };
        
        if result < 0 {
            continue;
        }
        
        const F_OK: i32 = 0;
        const R_OK: i32 = 4;
        
        if unsafe { crate::compat::access(configPath, F_OK | R_OK) } == 0 {
            return true;
        }
    }
    
    false
}