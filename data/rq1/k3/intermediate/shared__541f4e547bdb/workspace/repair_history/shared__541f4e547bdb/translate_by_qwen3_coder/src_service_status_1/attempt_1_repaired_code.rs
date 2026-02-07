pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let status_ref = unsafe { &*status };
    if status_ref.serviceName.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let info_ptr = if !status_ref.info.is_null() {
        status_ref.info
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    unsafe {
        if !crate::compat::HdfSbufWriteString(buf, status_ref.serviceName)
            || !crate::compat::HdfSbufWriteUint16(buf, status_ref.deviceClass)
            || !crate::compat::HdfSbufWriteUint16(buf, status_ref.status)
            || !crate::compat::HdfSbufWriteString(buf, info_ptr)
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