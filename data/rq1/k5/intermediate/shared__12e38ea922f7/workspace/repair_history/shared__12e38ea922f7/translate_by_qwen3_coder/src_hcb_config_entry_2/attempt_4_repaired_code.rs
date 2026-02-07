fn GetConfigFilePath(productName: *const std::ffi::c_char, configPath: *mut std::ffi::c_char, configPathLen: usize) -> bool {
    let adapterConfigPath: [*const std::ffi::c_char; 2] = [
        b"/vendor/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
        b"/chip_prod/etc/hdfconfig\0".as_ptr() as *const std::ffi::c_char,
    ];
    let pathNum = adapterConfigPath.len();
    for i in 0..pathNum {
        let ret = unsafe {
            libc::snprintf(
                configPath,
                configPathLen,
                b"%s/hdf_%s.hcb\0".as_ptr() as *const std::ffi::c_char,
                adapterConfigPath[i],
                productName,
            )
        };
        if ret < 0 {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                    b"failed to generate file path\0".as_ptr() as *const std::ffi::c_char,
                )
            };
            continue;
        }
        if unsafe { libc::access(configPath, libc::F_OK | libc::R_OK) } == 0 {
            return true;
        }
        let _ = unsafe {
            crate::compat::HiLogPrint(
                3u32,
                1u32,
                0xD002510,
                b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
                b"invalid config file path or permission:%{public}s\0".as_ptr() as *const std::ffi::c_char,
                configPath,
            )
        };
    }
    false
}