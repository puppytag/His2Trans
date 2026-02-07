pub extern "C" fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node_ptr = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    let mut serv_info = crate::types::HdfServiceInfo {
        servName: std::ptr::null(),
        servInfo: std::ptr::null(),
        devClass: 0,
        devId: 0,
        interfaceDesc: std::ptr::null(),
    };
    unsafe {
        let dev_node = &*dev_node_ptr;
        serv_info.servName = dev_node.servName;
        serv_info.servInfo = dev_node.servInfo;
        serv_info.devClass = dev_node.deviceObject.deviceClass as u16;
        serv_info.devId = dev_node.devId;
        serv_info.interfaceDesc = dev_node.interfaceDesc;
    }
    unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntUpdateService(&mut (*dev_node_ptr).deviceObject, &serv_info)
    }
}