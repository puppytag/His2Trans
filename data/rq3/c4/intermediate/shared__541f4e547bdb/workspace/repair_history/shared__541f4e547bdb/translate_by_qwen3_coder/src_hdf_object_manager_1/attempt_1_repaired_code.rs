pub extern "C" fn HdfObjectManagerGetObject(objectId: ::core::ffi::c_int) -> *mut crate::types::HdfObject {
    let mut object: *mut crate::types::HdfObject = std::ptr::null_mut();
    let targetCreator = unsafe { crate::compat::HdfObjectManagerGetCreators(objectId) };
    if !targetCreator.is_null() {
        let create_fn = unsafe { (*targetCreator).Create };
        if let Some(create) = create_fn {
            object = unsafe { create() };
            if !object.is_null() {
                unsafe {
                    (*object).objectId = objectId;
                }
            }
        }
    }
    object
}