pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }

    unsafe {
        if !HdfSbufWriteUint32(sbuf, (*attribute).deviceId) ||
           !HdfSbufWriteUint16(sbuf, (*attribute).policy) ||
           !HdfSbufWriteString(sbuf, (*attribute).svcName) ||
           !HdfSbufWriteString(sbuf, (*attribute).moduleName) ||
           !HdfSbufWriteString(sbuf, (*attribute).deviceName) {
            return false;
        }

        if !(*attribute).deviceMatchAttr.is_null() {
            if !HdfSbufWriteUint32(sbuf, 1) ||
               !HdfSbufWriteString(sbuf, (*attribute).deviceMatchAttr) {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to serialize device attribute\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        } else {
            if !HdfSbufWriteUint32(sbuf, 0) {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to serialize device attribute\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        }
    }

    true
}