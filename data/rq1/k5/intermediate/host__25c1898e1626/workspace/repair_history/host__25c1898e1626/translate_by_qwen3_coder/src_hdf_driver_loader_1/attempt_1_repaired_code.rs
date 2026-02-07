pub extern "C" fn HdfDriverEntryConstruct() -> i32 {
    let mut i: i32;
    let mut driverEntry: *mut crate::types::HdfDriverEntry = std::ptr::null_mut();
    let mut addrBegin: *mut crate::types::size_t = std::ptr::null_mut();
    let count: i32 = 0;
    if count <= 0 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"driver_loader\0".as_ptr() as *const i8,
            b"%{public}s: no hdf driver exist\0".as_ptr() as *const i8,
            b"HdfDriverEntryConstruct\0".as_ptr() as *const i8,
        );
        return crate::types::HDF_FAILURE;
    }
    addrBegin = std::ptr::null_mut();
    i = 0;
    while i < count {
        driverEntry = unsafe { *addrBegin as *mut crate::types::HdfDriverEntry };
        if crate::compat::HdfRegisterDriverEntry(driverEntry as *const crate::types::HdfDriverEntry) != crate::types::HDF_SUCCESS {
            let module_name = if !driverEntry.is_null() {
                unsafe { (*driverEntry).moduleName }
            } else {
                std::ptr::null()
            };
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"driver_loader\0".as_ptr() as *const i8,
                b"failed to register driver %{public}s, skip and try another\0".as_ptr() as *const i8,
                module_name,
            );
        }
        addrBegin = unsafe { addrBegin.offset(1) };
        i += 1;
    }
    crate::types::HDF_SUCCESS
}