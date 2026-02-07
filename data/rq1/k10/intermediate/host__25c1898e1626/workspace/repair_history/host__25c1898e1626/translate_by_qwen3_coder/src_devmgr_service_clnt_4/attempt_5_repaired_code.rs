static mut INSTANCE: crate::types::DevmgrServiceClnt = crate::types::DevmgrServiceClnt {
    devMgrSvcIf: std::ptr::null_mut(),
};

pub extern "C" fn DevmgrServiceClntGetInstance() -> *mut crate::types::DevmgrServiceClnt {
    unsafe {
        if INSTANCE.devMgrSvcIf.is_null() {
            INSTANCE.devMgrSvcIf = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVMGR_SERVICE as i32) as *mut crate::types::IDevmgrService;
        }
        &mut INSTANCE
    }
}