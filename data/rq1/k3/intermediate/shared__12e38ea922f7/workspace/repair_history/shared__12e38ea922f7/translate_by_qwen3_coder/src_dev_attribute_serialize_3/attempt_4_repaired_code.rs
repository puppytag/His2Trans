pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute: *mut crate::types::HdfDeviceInfo = unsafe { crate::compat::HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let mut deviceId: u32 = 0;
        let mut policy: u16 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut deviceId) || !crate::compat::HdfSbufReadUint16(sbuf, &mut policy) {
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