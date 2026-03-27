#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: core::ffi::c_int,
    pub verbose_more: core::ffi::c_int,
    pub numiterations: core::ffi::c_int,
    pub blocksplitting: core::ffi::c_int,
    pub blocksplittinglast: core::ffi::c_int,
    pub blocksplittingmax: core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliInitOptions(mut options: *mut ZopfliOptions) {
    (*options).verbose = 0 as core::ffi::c_int;
    (*options).verbose_more = 0 as core::ffi::c_int;
    (*options).numiterations = 15 as core::ffi::c_int;
    (*options).blocksplitting = 1 as core::ffi::c_int;
    (*options).blocksplittinglast = 0 as core::ffi::c_int;
    (*options).blocksplittingmax = 15 as core::ffi::c_int;
}
