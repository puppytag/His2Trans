pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    if productFunc.is_null() {
        return;
    }
    unsafe {
        (*productFunc).devUdidFunc = Some(crate::compat::GetUdid as unsafe extern "C" fn(*mut ::core::ffi::c_uchar, i32) -> i32);
    }
}