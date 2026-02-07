pub extern "C" fn InquiryDeviceUdid(udid: *mut ::core::ffi::c_uchar, size: i32) -> i32 {
    unsafe {
        if g_productDiffFunc.devUdidFunc.is_none() {
            return -1;
        }
        if let Some(f) = g_productDiffFunc.devUdidFunc {
            f(udid, size)
        } else {
            -1
        }
    }
}