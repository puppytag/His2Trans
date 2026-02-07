pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf = &mut (*service).super_ as *mut crate::types::IDevHostService;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(crate::src_devhost_service::DevHostServiceStartService);
        (*hostServiceIf).PmNotify = Some(crate::src_devhost_service::DevHostServicePmNotify);
        let devices = &mut (*service).devices;
        (*devices).next = devices;
        (*devices).prev = devices;
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}