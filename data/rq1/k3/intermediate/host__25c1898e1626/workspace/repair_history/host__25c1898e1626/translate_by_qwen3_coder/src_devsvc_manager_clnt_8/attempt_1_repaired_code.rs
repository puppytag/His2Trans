unsafe {
    (*inst).devSvcMgrIf = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVSVC_MANAGER as i32) as *mut crate::types::IDevSvcManager;
}