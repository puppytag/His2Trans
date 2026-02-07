pub extern "C" fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject) {
    if object.is_null() {
        return;
    }
    let object_id = unsafe { (*object).objectId };
    let target_creator = unsafe { crate::compat::HdfObjectManagerGetCreators(object_id) };
    if target_creator.is_null() {
        return;
    }
    let release = unsafe { (*target_creator).Release };
    if let Some(f) = release {
        unsafe { f(object) };
    }
}