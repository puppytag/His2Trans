pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf: *mut crate::types::IDevHostService = &mut (*service).super_;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(crate::src_devhost_service::DevHostServiceStartService);
        (*hostServiceIf).PmNotify = Some(crate::src_devhost_service::DevHostServicePmNotify);
        
        // Inline DListHeadInit
        let head: *mut crate::types::DListHead = &mut (*service).devices;
        (*head).next = head;
        (*head).prev = head;
        
        crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}