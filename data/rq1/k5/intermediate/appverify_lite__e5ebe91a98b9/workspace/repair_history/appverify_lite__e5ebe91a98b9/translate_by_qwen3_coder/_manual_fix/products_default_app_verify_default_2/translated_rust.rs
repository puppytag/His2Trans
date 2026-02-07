pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    if !productFunc.is_null() {
        unsafe {
            (*productFunc).devUdidFunc = Some(crate::compat::GetUdid);
        }
    }
}