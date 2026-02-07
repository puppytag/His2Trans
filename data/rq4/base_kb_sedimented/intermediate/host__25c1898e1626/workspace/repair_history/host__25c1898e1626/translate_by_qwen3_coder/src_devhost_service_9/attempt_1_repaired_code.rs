pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf: *mut crate::types::IDevHostService = &mut (*service).super_;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice as unsafe extern "C" fn(*mut crate::types::IDevHostService, *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice as unsafe extern "C" fn(*mut crate::types::IDevHostService, crate::types::devid_t) -> ::core::ffi::c_int);
        (*hostServiceIf).StartService = Some(DevHostServiceStartService as unsafe extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int);
        (*hostServiceIf).PmNotify = Some(DevHostServicePmNotify as unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int);
        
        let head: *mut crate::types::DListHead = &mut (*service).devices;
        (*head).next = head;
        (*head).prev = head;
        
        crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer);
    }
}