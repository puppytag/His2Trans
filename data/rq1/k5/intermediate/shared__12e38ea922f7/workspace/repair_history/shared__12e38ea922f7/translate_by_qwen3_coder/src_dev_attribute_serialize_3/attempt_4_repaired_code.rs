pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute = unsafe { crate::compat::HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(0, 3, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute is null\0".as_ptr() as *const _) };
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = crate::compat::HiLogPrint(0, 4, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const _);
        }
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !crate::compat::HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = crate::compat::HiLogPrint(0, 3, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"invalid deviceId or policy\0".as_ptr() as *const _);
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
    }
    if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
        return attribute;
    }
    crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    std::ptr::null_mut()
}