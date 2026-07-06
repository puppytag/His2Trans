//! Module: src_hdf_driver_loader
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

fn DListHeadInit(head: *mut crate::types::DListHead) {
    unimplemented!()
}

fn DListIsEmpty(head: *const crate::types::DListHead)-> bool {
    unimplemented!()
}

fn DListRemove(entry: *mut crate::types::DListHead) {
    unimplemented!()
}

fn DListInsertHead(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unimplemented!()
}

fn DListInsertTail(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unimplemented!()
}

fn DListMerge(list: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unimplemented!()
}

fn DListGetCount(head: *const crate::types::DListHead)-> i32 {
    unimplemented!()
}

fn __CPU_AND_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

fn __CPU_OR_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

fn __CPU_XOR_S(__size: usize, __dest: *mut crate::types::cpu_set_t, __src1: *const crate::types::cpu_set_t, __src2: *const crate::types::cpu_set_t) {
    unimplemented!()
}

pub extern "C" fn HdfDriverEntryConstruct()-> i32 {
    let begin: *const u8 = core::ptr::null();
    let end: *const u8 = core::ptr::null();
    let count = ((end as usize).wrapping_sub(begin as usize)
        / core::mem::size_of::<crate::types::size_t>()) as i32;
    if count <= 0 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"driver_loader\0" as *const u8 as *const i8,
                b"%{public}s: no hdf driver exist\0" as *const u8 as *const i8,
                b"HdfDriverEntryConstruct\0" as *const u8 as *const i8,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let mut addr_begin = begin as *const crate::types::size_t;
    for _ in 0..count {
        let driver_entry_value = unsafe { *addr_begin as *mut crate::types::HdfDriverEntry };
        let ret = unsafe { HdfRegisterDriverEntry(driver_entry_value as *const _) };
        if ret != crate::types::HDF_SUCCESS {
            let name: *const i8 = if driver_entry_value.is_null() {
                b"\0" as *const u8 as *const i8
            } else {
                unsafe { (*driver_entry_value).moduleName as *const i8 }
            };
            unsafe {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"driver_loader\0" as *const u8 as *const i8,
                    b"failed to register driver %{public}s, skip and try another\0" as *const u8
                        as *const i8,
                    name,
                );
            }
        }
        addr_begin = unsafe { addr_begin.add(1) };
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfDriverLoaderGetDriver(moduleName: *const std::ffi::c_char)-> *mut crate::types::HdfDriver {
    if moduleName.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { HdfDriverManagerGetDriver(moduleName) }
}

pub extern "C" fn HdfDriverLoaderReclaimDriver(driver: *mut crate::types::HdfDriver) {
    let _ = driver;
}

pub extern "C" fn HdfDriverLoaderConstruct(inst: *mut crate::types::HdfDriverLoader) {
    if inst.is_null() {
        return;
    }
    unsafe {
        let inst = &mut *inst;
        inst.super_.GetDriver = Some(crate::src_hdf_driver_loader::HdfDriverLoaderGetDriver);
        inst.super_.ReclaimDriver = Some(crate::src_hdf_driver_loader::HdfDriverLoaderReclaimDriver);
    }
}

pub extern "C" fn HdfDriverLoaderCreate()-> *mut crate::types::HdfObject {
    use core::sync::atomic::{AtomicBool, Ordering};
    static INIT: AtomicBool = AtomicBool::new(false);
    static mut DRIVER_LOADER: core::mem::MaybeUninit<crate::types::HdfDriverLoader> = core::mem::MaybeUninit::uninit();
    if !INIT.load(Ordering::Acquire) {
        let ptr = unsafe { DRIVER_LOADER.as_mut_ptr() };
        unsafe {
            core::ptr::write_bytes(ptr, 0u8, core::mem::size_of::<crate::types::HdfDriverLoader>());
        }
        if crate::src_hdf_driver_loader::HdfDriverEntryConstruct() != crate::types::HDF_SUCCESS {
            return core::ptr::null_mut::<crate::types::HdfObject>();
        }
        crate::src_hdf_driver_loader::HdfDriverLoaderConstruct(ptr);
        INIT.store(true, Ordering::Release);
    }
    unsafe { DRIVER_LOADER.as_mut_ptr() as *mut crate::types::HdfObject }
}

pub extern "C" fn HdfDriverLoaderGetInstance()-> *mut crate::types::IDriverLoader {
    unsafe {
        HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DRIVER_LOADER.try_into().unwrap()) as *mut crate::types::IDriverLoader
    }
}
