pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }
    unsafe {
        if !crate::compat::HdfSbufWriteUint32(sbuf, (*attribute).deviceId) ||
            !crate::compat::HdfSbufWriteUint16(sbuf, (*attribute).policy) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).svcName) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).moduleName) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).deviceName)
        {
            return false;
        }
        if !(*attribute).deviceMatchAttr.is_null() {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
                !crate::compat::HdfSbufWriteString(sbuf, (*attribute).deviceMatchAttr)
            {
                let _ = crate::compat::HiLogPrint(
                    crate::compat::LOG_CORE!(),
                    crate::compat::LOG_ERROR!(),
                    0xD002510,
                    "dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    "failed to serialize device attribute\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        } else {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 0) {
                let _ = crate::compat::HiLogPrint(
                    crate::compat::LOG_CORE!(),
                    crate::compat::LOG_ERROR!(),
                    0xD002510,
                    "dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    "failed to serialize device attribute\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        }
    }
    true
}