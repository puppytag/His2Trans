pub extern "C" fn DevHostServiceNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::IDevHostService {
    let hostService = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVHOST_SERVICE as i32) } as *mut crate::types::DevHostService;
    if !hostService.is_null() && !hostName.is_null() {
        unsafe {
            (*hostService).hostId = hostId;
            (*hostService).hostName = hostName;
        }
    }
    hostService as *mut crate::types::IDevHostService
}