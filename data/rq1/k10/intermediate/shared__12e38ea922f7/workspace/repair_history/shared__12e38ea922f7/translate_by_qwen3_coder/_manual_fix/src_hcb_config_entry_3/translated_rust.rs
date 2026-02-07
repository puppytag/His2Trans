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