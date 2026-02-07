fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    unsafe {
        (*svcmgrInst).RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
        (*svcmgrInst).UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
    }
}