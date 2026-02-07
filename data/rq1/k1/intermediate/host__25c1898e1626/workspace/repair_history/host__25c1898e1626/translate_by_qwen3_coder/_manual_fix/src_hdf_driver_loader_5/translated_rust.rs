static mut IS_DRIVER_LOADER_INIT!(): bool = false;
static mut DRIVER_LOADER: crate::types::HdfDriverLoader = crate::types::HdfDriverLoader {
    super_: crate::types::IDriverLoader {
        object: crate::types::HdfObject { objectId: 0 },
        GetDriver: None,
        ReclaimDriver: None,
    },
};

pub extern "C" fn HdfDriverLoaderCreate() -> *mut crate::types::HdfObject {
    unsafe {
        if !IS_DRIVER_LOADER_INIT!() {
            if crate::src_hdf_driver_loader::HdfDriverEntryConstruct() != crate::types::HDF_SUCCESS {
                return std::ptr::null_mut();
            }
            crate::src_hdf_driver_loader::HdfDriverLoaderConstruct(&mut DRIVER_LOADER);
            IS_DRIVER_LOADER_INIT!() = true;
        }
        &mut DRIVER_LOADER.super_.object as *mut crate::types::HdfObject
    }
}