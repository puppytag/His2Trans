pub extern "C" fn HdfObjectManagerGetObject(objectId: ::core::ffi::c_int) -> *mut crate::types::HdfObject {
    let mut object: *mut crate::types::HdfObject = std::ptr::null_mut();
    let targetCreator: *const crate::types::HdfObjectCreator = crate::src_hdf_object_manager::HdfObjectManagerGetCreators(objectId);
    if !targetCreator.is_null() {
        unsafe {
            if let Some(create_fn) = (*targetCreator).Create {
                object = create_fn();
                if !object.is_null() {
                    (*object).objectId = objectId;
                }
            }
        }
    }
    object
}