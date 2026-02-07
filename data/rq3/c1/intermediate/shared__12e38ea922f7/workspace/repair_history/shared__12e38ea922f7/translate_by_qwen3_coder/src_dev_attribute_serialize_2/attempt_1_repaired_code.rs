fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    unsafe {
        let svcName = crate::compat::HdfSbufReadString(sbuf);
        if svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup svcName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let moduleName = crate::compat::HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from parcel failed, moduleName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup moduleName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let deviceName = crate::compat::HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, deviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup deviceName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let mut length: u32 = 0;
        if crate::compat::HdfSbufReadUint32(sbuf, &mut length) == false {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Device attribute readDeviceMatchAttr length failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        if length == 1 {
            let deviceMatchAttr = crate::compat::HdfSbufReadString(sbuf);
            if deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::compat::LOG_CORE!(),
                    crate::compat::LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"%s: Read from sbuf failed, deviceMatchAttr is null\0".as_ptr() as *const ::core::ffi::c_char,
                    b"DeviceAttributeSet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::compat::LOG_CORE!(),
                    crate::compat::LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Read from sbuf failed, strdup deviceMatchAttr fail\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        }

        true
    }
}