#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn HdfObjectManagerGetCreators(
        objectId: core::ffi::c_int,
    ) -> *const HdfObjectCreator;
}
pub type int32_t = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObjectCreator {
    pub Create: Option<unsafe extern "C" fn() -> *mut HdfObject>,
    pub Release: Option<unsafe extern "C" fn(*mut HdfObject) -> ()>,
}
#[no_mangle]
pub unsafe extern "C" fn HdfObjectManagerGetObject(
    mut objectId: core::ffi::c_int,
) -> *mut HdfObject {
    let mut object: *mut HdfObject = 0 as *mut HdfObject;
    let mut targetCreator: *const HdfObjectCreator = HdfObjectManagerGetCreators(
        objectId,
    );
    if !targetCreator.is_null() && ((*targetCreator).Create).is_some() {
        object = ((*targetCreator).Create).expect("non-null function pointer")();
        if !object.is_null() {
            (*object).objectId = objectId as int32_t;
        }
    }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn HdfObjectManagerFreeObject(mut object: *mut HdfObject) {
    let mut targetCreator: *const HdfObjectCreator = 0 as *const HdfObjectCreator;
    if object.is_null() {
        return;
    }
    targetCreator = HdfObjectManagerGetCreators((*object).objectId as core::ffi::c_int);
    if targetCreator.is_null() || ((*targetCreator).Release).is_none() {
        return;
    }
    ((*targetCreator).Release).expect("non-null function pointer")(object);
}
