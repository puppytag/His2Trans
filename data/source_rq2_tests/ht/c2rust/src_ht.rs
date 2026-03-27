extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht {
    pub entries: *mut ht_entry,
    pub capacity: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *const core::ffi::c_char,
    pub value: *mut core::ffi::c_void,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INITIAL_CAPACITY: core::ffi::c_int = 16 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ht_create() -> *mut ht {
    let mut table: *mut ht = malloc(::core::mem::size_of::<ht>() as size_t) as *mut ht;
    if table.is_null() {
        return 0 as *mut ht;
    }
    (*table).length = 0 as size_t;
    (*table).capacity = INITIAL_CAPACITY as size_t;
    (*table).entries = calloc(
        (*table).capacity,
        ::core::mem::size_of::<ht_entry>() as size_t,
    ) as *mut ht_entry;
    if ((*table).entries).is_null() {
        free(table as *mut core::ffi::c_void);
        return 0 as *mut ht;
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn ht_destroy(mut table: *mut ht) {
    let mut i: size_t = 0 as size_t;
    while i < (*table).capacity {
        free((*((*table).entries).offset(i as isize)).key as *mut core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free((*table).entries as *mut core::ffi::c_void);
    free(table as *mut core::ffi::c_void);
}
unsafe fn main_0() {}
pub fn main() {
    unsafe { main_0() }
    ::std::process::exit(0i32);
}
