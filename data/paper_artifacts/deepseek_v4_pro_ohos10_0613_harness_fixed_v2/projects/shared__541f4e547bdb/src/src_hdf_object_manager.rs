//! Module: src_hdf_object_manager
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn HdfObjectManagerGetObject(objectId: ::core::ffi::c_int) -> *mut crate::types::HdfObject {
    let mut object: *mut crate::types::HdfObject = std::ptr::null_mut();
    let targetCreator = unsafe { HdfObjectManagerGetCreators(objectId) };
    if targetCreator.is_null() {
        return object;
    }
    let create_fn = unsafe { (*targetCreator).Create };
    if let Some(create_fn) = create_fn {
        object = unsafe { create_fn() };
        if !object.is_null() {
            unsafe { (*object).objectId = objectId; }
        }
    }
    object
}

pub extern "C" fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject) {
    let mut target_creator: *const crate::types::HdfObjectCreator = std::ptr::null();
    if object.is_null() {
        return;
    }
    let object_id = unsafe { (*object).objectId };
    target_creator = unsafe { HdfObjectManagerGetCreators(object_id) };
    if target_creator.is_null() {
        return;
    }
    let release_fn = unsafe { (*target_creator).Release };
    if release_fn.is_none() {
        return;
    }
    let release_fn = release_fn.unwrap();
    unsafe {
        release_fn(object);
    }
}
