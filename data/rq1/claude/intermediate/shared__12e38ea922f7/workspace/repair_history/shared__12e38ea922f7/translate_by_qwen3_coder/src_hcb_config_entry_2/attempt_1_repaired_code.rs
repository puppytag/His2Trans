fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    let adapterConfigPath: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];

    let pathNum: usize = 2;
    
    for i in 0..pathNum {
        let format = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
        let ret = unsafe {
            crate::compat::sprintf_s(
                configPath,
                configPathLen - 1,
                format,
                adapterConfigPath[i],
                productName
            )
        };
        
        if ret < 0 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::compat::LOG_CORE!(),
                    crate::compat::LOG_ERROR,
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
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_DEBUG,
                0xD002510,
                b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                b"invalid config file path or permission:%s\0".as_ptr() as *const std::ffi::c_char,
                configPath
            );
        }
    }
    
    false
}