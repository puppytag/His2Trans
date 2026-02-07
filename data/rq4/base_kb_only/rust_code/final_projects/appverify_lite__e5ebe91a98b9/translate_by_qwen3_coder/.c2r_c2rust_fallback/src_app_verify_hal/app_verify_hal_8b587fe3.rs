#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn RegistBaseDefaultFunc(productFunc: *mut ProductDiff);
    fn RegistProductFunc(productFunc: *mut ProductDiff);
}
pub type int32_t = core::ffi::c_int;
pub type GetDeviceUdid = Option<
    unsafe extern "C" fn(*mut core::ffi::c_uchar, int32_t) -> int32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProductDiff {
    pub devUdidFunc: GetDeviceUdid,
}
static mut g_productDiffFunc: ProductDiff = ProductDiff { devUdidFunc: None };
#[no_mangle]
pub unsafe extern "C" fn RegistHalFunc() {
    RegistBaseDefaultFunc(&mut g_productDiffFunc);
    RegistProductFunc(&mut g_productDiffFunc);
}
#[no_mangle]
pub unsafe extern "C" fn InquiryDeviceUdid(
    mut udid: *mut core::ffi::c_uchar,
    mut size: int32_t,
) -> int32_t {
    if (g_productDiffFunc.devUdidFunc).is_none() {
        return -(1 as int32_t);
    }
    return (g_productDiffFunc.devUdidFunc)
        .expect("non-null function pointer")(udid, size);
}
