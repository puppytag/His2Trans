pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }

    let attribute = unsafe { HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"OsalMemCalloc failed, attribute is null\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        return std::ptr::null_mut();
    }

    if unsafe { (*attribute).deviceMatchAttr.is_null() } {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_WARN,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
    }

    let read_id_ok = unsafe { HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) };
    let read_policy_ok = unsafe { HdfSbufReadUint16(sbuf, &mut (*attribute).policy) };
    
    if !read_id_ok || !read_policy_ok {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"invalid deviceId or policy\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
        return std::ptr::null_mut();
    }

    if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
        return attribute;
    }

    crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    std::ptr::null_mut()
}