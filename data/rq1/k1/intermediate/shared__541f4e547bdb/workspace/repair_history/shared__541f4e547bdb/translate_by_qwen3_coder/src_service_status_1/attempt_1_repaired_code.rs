pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let service_name = unsafe { (*status).serviceName };
    if service_name.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let device_class = unsafe { (*status).deviceClass };
    let status_val = unsafe { (*status).status };
    let info = unsafe { (*status).info };
    let info_str = if info.is_null() {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        info
    };
    unsafe {
        if !crate::compat::HdfSbufWriteString(buf, service_name)
            || !crate::compat::HdfSbufWriteUint16(buf, device_class)
            || !crate::compat::HdfSbufWriteUint16(buf, status_val)
            || !crate::compat::HdfSbufWriteString(buf, info_str)
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to marshalling service status\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}