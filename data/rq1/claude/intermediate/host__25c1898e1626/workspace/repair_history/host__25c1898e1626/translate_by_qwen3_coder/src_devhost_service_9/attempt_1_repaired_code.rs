pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    
    unsafe {
        let hostServiceIf: *mut crate::types::IDevHostService = &mut (*service).super_;
        (*hostServiceIf).AddDevice = Some(DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(DevHostServiceStartService as unsafe extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int);
        (*hostServiceIf).PmNotify = Some(DevHostServicePmNotify as unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int);
        
        // Inline DListHeadInit
        let head: *mut crate::types::DListHead = &mut (*service).devices;
        (*head).next = head;
        (*head).prev = head;
        
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}