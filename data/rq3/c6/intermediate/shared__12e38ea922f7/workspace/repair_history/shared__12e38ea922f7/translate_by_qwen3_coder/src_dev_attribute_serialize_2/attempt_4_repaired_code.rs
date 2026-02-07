fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    unsafe {
        let svcName = crate::compat::HdfSbufReadString(sbuf);
        if svcName.is_null() {
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            return false;
        }

        let moduleName = crate::compat::HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            return false;
        }

        let deviceName = crate::compat::HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            return false;
        }

        let mut length: u32 = 0;
        if crate::compat::HdfSbufReadUint32(sbuf, &mut length) == false {
            return false;
        }
        if length == 1 {
            let deviceMatchAttr = crate::compat::HdfSbufReadString(sbuf);
            if deviceMatchAttr.is_null() {
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                return false;
            }
        }

        true
    }
}