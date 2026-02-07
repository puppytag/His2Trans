fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    let adapterConfigPath: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];

    let pathNum: usize = 2;
    for i in 0..pathNum {
        let format_str = b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char;
        let ret = unsafe {
            crate::compat::sprintf_s(
                configPath,
                configPathLen - 1,
                format_str,
                adapterConfigPath[i],
                productName,
            )
        };
        if ret < 0 {
            let log_core: crate::compat::LogType = 3;
            let log_error: crate::compat::LogLevel = 6;
            unsafe {
                crate::compat::HiLogPrint(
                    log_core,
                    log_error,
                    0xD002510,
                    b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                    b"failed to generate file path\0".as_ptr() as *const std::ffi::c_char,
                );
            }
            continue;
        }

        let f_ok: i32 = 0;
        let r_ok: i32 = 4;
        if unsafe { crate::compat::access(configPath, f_ok | r_ok) } == 0 {
            return true;
        }
        let log_core: crate::compat::LogType = 3;
        let log_debug: crate::compat::LogLevel = 3;
        unsafe {
            crate::compat::HiLogPrint(
                log_core,
                log_debug,
                0xD002510,
                b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                b"invalid config file path or permission:%{public}s\0".as_ptr() as *const std::ffi::c_char,
                configPath,
            );
        }
    }
    false
}