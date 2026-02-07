pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut crate::types::HdfSBuf) -> *mut crate::types::HdfDeviceInfo {
    use crate::compat::*;
    use crate::globals::*;
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute: *mut crate::types::HdfDeviceInfo = unsafe { HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = HiLogPrint(LOG_CORE as LogType, LOG_ERROR as LogLevel, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const i8, "OsalMemCalloc failed, attribute is null\0".as_ptr() as *const i8);
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = HiLogPrint(LOG_CORE as LogType, LOG_WARN as LogLevel, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const i8, "OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const i8);
        }
        if !HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = HiLogPrint(LOG_CORE as LogType, LOG_ERROR as LogLevel, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const i8, "invalid deviceId or policy\0".as_ptr() as *const i8);
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