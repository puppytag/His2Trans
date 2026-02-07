pub extern "C" fn HdfDriverEntryConstruct() -> i32 {
    let mut i: i32;
    let mut driverEntry: *mut crate::types::HdfDriverEntry = std::ptr::null_mut();
    let mut addrBegin: *mut crate::types::size_t = std::ptr::null_mut();
    extern "C" {
        static _hdf_drivers_start: crate::types::size_t;
        static _hdf_drivers_end: crate::types::size_t;
    }
    let count: i32 = unsafe {
        ((&_hdf_drivers_end as *const crate::types::size_t as *const u8 as usize)
            .wrapping_sub(&_hdf_drivers_start as *const crate::types::size_t as *const u8 as usize))
            / std::mem::size_of::<crate::types::size_t>()
    } as i32;
    if count <= 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"driver_loader\0".as_ptr() as *const i8,
                b"%{public}s: no hdf driver exist\0".as_ptr() as *const i8,
                b"HdfDriverEntryConstruct\0".as_ptr() as *const i8,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    addrBegin = unsafe { &_hdf_drivers_start as *const crate::types::size_t as *mut crate::types::size_t };
    i = 0;
    while i < count {
        driverEntry = unsafe { *addrBegin as *mut crate::types::HdfDriverEntry };
        if unsafe {
            crate::compat::HdfRegisterDriverEntry(driverEntry as *const crate::types::HdfDriverEntry)
        } != crate::types::HDF_SUCCESS
        {
            let module_name = if !driverEntry.is_null() {
                unsafe { (*driverEntry).moduleName }
            } else {
                b"\0".as_ptr() as *const i8
            };
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"driver_loader\0".as_ptr() as *const i8,
                    b"failed to register driver %{public}s, skip and try another\0".as_ptr()
                        as *const i8,
                    module_name,
                );
            }
        }
        addrBegin = unsafe { addrBegin.offset(1) };
        i += 1;
    }
    crate::types::HDF_SUCCESS
}