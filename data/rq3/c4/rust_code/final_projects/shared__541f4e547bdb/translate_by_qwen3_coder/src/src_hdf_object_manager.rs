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
    let targetCreator = unsafe { crate::compat::HdfObjectManagerGetCreators(objectId) };
    if !targetCreator.is_null() {
        let create_fn = unsafe { (*targetCreator).Create };
        if let Some(create) = create_fn {
            object = unsafe { create() };
            if !object.is_null() {
                unsafe {
                    (*object).objectId = objectId;
                }
            }
        }
    }
    object
}

pub extern "C" fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject) {
    if object.is_null() {
        return;
    }
    
    let object_id = unsafe { (*object).objectId };
    let target_creator = unsafe { crate::compat::HdfObjectManagerGetCreators(object_id) };
    
    if target_creator.is_null() {
        return;
    }
    
    let release_fn = unsafe { (*target_creator).Release };
    if let Some(release) = release_fn {
        unsafe { release(object) };
    }
}
