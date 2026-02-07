pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf = &mut (*service).super_ as *mut crate::types::IDevHostService;
        (*hostServiceIf).AddDevice = Some(crate::compat::DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(crate::compat::DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(crate::compat::DevHostServiceStartService);
        (*hostServiceIf).PmNotify = Some(crate::compat::DevHostServicePmNotify);
        let head = &mut (*service).devices as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
        let _ = crate::compat::HdfServiceObserverConstruct(&mut (*service).observer as *mut crate::types::HdfServiceObserver);
    }
}