pub extern "C" fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject) {
    let targetCreator: *const crate::types::HdfObjectCreator;
    if object.is_null() {
        return;
    }
    targetCreator = unsafe { crate::compat::HdfObjectManagerGetCreators((*object).objectId) };
    if targetCreator.is_null() {
        return;
    }
    let release_fn = unsafe { (*targetCreator).Release };
    if release_fn.is_none() {
        return;
    }
    unsafe {
        (release_fn.unwrap())(object);
    }
}