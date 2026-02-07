fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    if svcmgrInst.is_null() {
        return;
    }
    unsafe {
        (*svcmgrInst).RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
        (*svcmgrInst).UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
    }
}