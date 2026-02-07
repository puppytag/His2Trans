pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute = unsafe { crate::compat::HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::compat::LOG_CORE!(), crate::compat::LOG_ERROR!(), 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute is null\0".as_ptr() as *const _) };
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = crate::compat::HiLogPrint(crate::compat::LOG_CORE!(), crate::compat::LOG_WARN, 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const _);
        }
        let mut deviceId: u32 = 0;
        let mut policy: u16 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut deviceId) || !crate::compat::HdfSbufReadUint16(sbuf, &mut policy) {
            let _ = crate::compat::HiLogPrint(crate::compat::LOG_CORE!(), crate::compat::LOG_ERROR!(), 0xD002510, b"dev_attr_serialze\0".as_ptr() as *const _, b"invalid deviceId or policy\0".as_ptr() as *const _);
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
        (*attribute).deviceId = deviceId;
        (*attribute).policy = policy;
    }
    if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
        return attribute;
    }
    crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    std::ptr::null_mut()
}