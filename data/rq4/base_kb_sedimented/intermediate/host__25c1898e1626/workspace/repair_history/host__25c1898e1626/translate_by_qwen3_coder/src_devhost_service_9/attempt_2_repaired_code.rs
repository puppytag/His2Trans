pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf: *mut crate::types::IDevHostService = &mut (*service).super_;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice);
        (*hostServiceIf).StartService = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int>(DevHostServiceStartService));
        (*hostServiceIf).PmNotify = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int>(DevHostServicePmNotify));
        
        let head: *mut crate::types::DListHead = &mut (*service).devices;
        (*head).next = head;
        (*head).prev = head;
        
        crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}