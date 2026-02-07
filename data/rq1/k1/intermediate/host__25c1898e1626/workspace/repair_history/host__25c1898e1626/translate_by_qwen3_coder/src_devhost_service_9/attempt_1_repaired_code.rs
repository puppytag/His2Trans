pub extern "C" fn DevHostServiceConstruct(service: *mut crate::types::DevHostService) {
    if service.is_null() {
        return;
    }
    unsafe {
        let hostServiceIf = &mut (*service).super_ as *mut crate::types::IDevHostService;
        (*hostServiceIf).AddDevice = Some(crate::src_devhost_service::DevHostServiceAddDevice as unsafe extern "C" fn(*mut crate::types::IDevHostService, *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int);
        (*hostServiceIf).DelDevice = Some(crate::src_devhost_service::DevHostServiceDelDevice as unsafe extern "C" fn(*mut crate::types::IDevHostService, crate::types::devid_t) -> ::core::ffi::c_int);
        (*hostServiceIf).StartService = Some(crate::src_devhost_service::DevHostServiceStartService as unsafe extern "C" fn(*mut crate::types::IDevHostService) -> ::core::ffi::c_int);
        (*hostServiceIf).PmNotify = Some(crate::src_devhost_service::DevHostServicePmNotify as unsafe extern "C" fn(*mut crate::types::IDevHostService, u32) -> ::core::ffi::c_int);
        let head = &mut (*service).devices as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
        let _ = crate::src_hdf_service_observer::HdfServiceObserverConstruct(&mut (*service).observer as *mut crate::types::HdfServiceObserver);
    }
}