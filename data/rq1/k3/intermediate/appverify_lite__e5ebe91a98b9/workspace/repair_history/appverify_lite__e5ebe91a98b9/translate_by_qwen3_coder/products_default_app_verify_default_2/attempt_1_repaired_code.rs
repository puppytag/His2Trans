pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    if !productFunc.is_null() {
        unsafe {
            (*productFunc).devUdidFunc = Some(GetUdid as unsafe extern "C" fn(*mut u8, i32) -> i32);
        }
    }
}