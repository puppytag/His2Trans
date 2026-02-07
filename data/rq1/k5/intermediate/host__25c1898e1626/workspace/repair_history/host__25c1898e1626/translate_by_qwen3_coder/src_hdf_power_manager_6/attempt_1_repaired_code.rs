pub extern "C" fn HdfPowerManagerInit() -> i32 {
    unsafe {
        DevMgrPmRegister();
        let _ = crate::src_hdf_power_manager::HdfPmTaskQueueInit(None);
    }
    crate::types::HDF_SUCCESS as i32
}