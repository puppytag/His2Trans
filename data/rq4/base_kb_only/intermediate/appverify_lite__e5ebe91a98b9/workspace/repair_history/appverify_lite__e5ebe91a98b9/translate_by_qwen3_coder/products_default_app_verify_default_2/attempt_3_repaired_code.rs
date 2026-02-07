pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    if productFunc.is_null() {
        return;
    }
    unsafe extern "C" fn get_udid_wrapper(udid: *mut ::core::ffi::c_uchar, size: i32) -> i32 {
        GetUdid(udid, size)
    }
    unsafe {
        (*productFunc).devUdidFunc = Some(get_udid_wrapper);
    }
}