pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }
    unsafe {
        let device_id = (*attribute).deviceId;
        let policy = (*attribute).policy;
        let svc_name = (*attribute).svcName;
        let module_name = (*attribute).moduleName;
        let device_name = (*attribute).deviceName;
        if !crate::compat::HdfSbufWriteUint32(sbuf, device_id) ||
            !crate::compat::HdfSbufWriteUint16(sbuf, policy) ||
            !crate::compat::HdfSbufWriteString(sbuf, svc_name) ||
            !crate::compat::HdfSbufWriteString(sbuf, module_name) ||
            !crate::compat::HdfSbufWriteString(sbuf, device_name) {
            return false;
        }
        let device_match_attr = (*attribute).deviceMatchAttr;
        if !device_match_attr.is_null() {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
                !crate::compat::HdfSbufWriteString(sbuf, device_match_attr) {
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
    }
    true
}