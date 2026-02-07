pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute = HdfDeviceInfoNewInstance();
    if attribute.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "OsalMemCalloc failed, attribute is null\0".as_ptr() as *const _);
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_WARN, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const _);
        }
        if !HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "invalid deviceId or policy\0".as_ptr() as *const _);
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
        if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
            return attribute;
        }
        crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    }
    std::ptr::null_mut()
}