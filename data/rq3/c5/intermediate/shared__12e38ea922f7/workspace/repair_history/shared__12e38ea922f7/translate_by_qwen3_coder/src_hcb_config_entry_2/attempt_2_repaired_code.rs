fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    static ADAPTER_CONFIG_PATH: [&[u8]; 2] = [
        b"/vendor/etc/hdfconfig\0",
        b"/chip_prod/etc/hdfconfig\0",
    ];
    
    let path_num: usize = 2;
    
    const LOG_CORE: crate::compat::LogType = 3;
    const LOG_ERROR: crate::compat::LogLevel = 6;
    const LOG_DEBUG: crate::compat::LogLevel = 3;
    
    for i in 0..path_num {
        let format_str = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
        let adapter_path = ADAPTER_CONFIG_PATH[i].as_ptr() as *const std::ffi::c_char;
        
        let result = unsafe {
            crate::compat::sprintf_s(
                configPath,
                configPathLen - 1,
                format_str,
                adapter_path,
                productName
            )
        };
        
        if result < 0 {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD002510,
                    b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                    b"failed to generate file path\0".as_ptr() as *const std::ffi::c_char
                );
            }
            continue;
        }
        
        let f_ok: i32 = 0;
        let r_ok: i32 = 4;
        
        if unsafe { crate::compat::access(configPath as *const std::ffi::c_char, f_ok | r_ok) } == 0 {
            return true;
        }
        
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_DEBUG,
                0xD002510,
                b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                b"invalid config file path or permission:%s\0".as_ptr() as *const std::ffi::c_char,
                configPath
            );
        }
    }
    
    false
}