pub extern "C" fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject) {
    let mut targetCreator: *const crate::types::HdfObjectCreator = std::ptr::null();
    if object.is_null() {
        return;
    }
    let object_id = unsafe { (*object).objectId };
    targetCreator = crate::compat::HdfObjectManagerGetCreators(object_id);
    if targetCreator.is_null() {
        return;
    }
    let release_fn = unsafe { (*targetCreator).Release };
    if release_fn.is_none() {
        return;
    }
    if let Some(release) = release_fn {
        unsafe { release(object) };
    }
}