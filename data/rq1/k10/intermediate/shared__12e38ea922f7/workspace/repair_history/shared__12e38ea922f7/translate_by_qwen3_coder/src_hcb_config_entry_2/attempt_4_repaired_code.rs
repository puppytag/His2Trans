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
            crate::compat::LOG_DEBUG as crate::types::LogLevel,
            0xD002510,
            b"attribute_manager\0".as_ptr() as *const std::ffi::c_char,
            b"invalid config file path or permission:%{public}s\0".as_ptr() as *const std::ffi::c_char,
            configPath,
        ) };
    }
    false
}