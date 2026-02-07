pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }
    let attr = unsafe { &*attribute };
    if !crate::compat::HdfSbufWriteUint32(sbuf, attr.deviceId) ||
        !crate::compat::HdfSbufWriteUint16(sbuf, attr.policy) ||
        !crate::compat::HdfSbufWriteString(sbuf, attr.svcName) ||
        !crate::compat::HdfSbufWriteString(sbuf, attr.moduleName) ||
        !crate::compat::HdfSbufWriteString(sbuf, attr.deviceName) {
        return false;
    }
    if !attr.deviceMatchAttr.is_null() {
        if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
            !crate::compat::HdfSbufWriteString(sbuf, attr.deviceMatchAttr) {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"failed to serialize device attribute\0".as_ptr() as *const i8,
            );
            return false;
        }
    } else {
        if !crate::compat::HdfSbufWriteUint32(sbuf, 0) {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"failed to serialize device attribute\0".as_ptr() as *const i8,
            );
            return false;
        }
    }
    true
}