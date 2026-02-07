pub extern "C" fn HdfIoServiceBind(serviceName: *const ::core::ffi::c_char) -> *mut crate::types::HdfIoService {
    unsafe { crate::compat::HdfIoServiceAdapterObtain(serviceName) }
}