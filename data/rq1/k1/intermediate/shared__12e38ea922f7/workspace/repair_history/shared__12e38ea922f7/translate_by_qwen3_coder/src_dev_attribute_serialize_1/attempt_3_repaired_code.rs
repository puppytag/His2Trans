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
                return false;
            }
        } else {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 0) {
                return false;
            }
        }
    }
    true
}