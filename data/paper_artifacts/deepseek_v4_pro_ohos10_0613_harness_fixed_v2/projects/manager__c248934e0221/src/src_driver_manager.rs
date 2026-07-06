//! Module: src_driver_manager
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
    unsafe {
        (*head).next = head;
        (*head).prev = head;
    }
}

fn DListIsEmpty(head: *const crate::types::DListHead)-> bool {
    unsafe { (*(head as *mut crate::types::DListHead)).next == head as *mut crate::types::DListHead }
}

fn DListRemove(entry: *mut crate::types::DListHead) {
    unsafe {
        let prev = (*entry).prev;
        let next = (*entry).next;
        if !prev.is_null() {
            (*prev).next = next;
        }
        if !next.is_null() {
            (*next).prev = prev;
        }
        (*entry).next = entry;
        (*entry).prev = entry;
    }
}

fn DListInsertHead(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        (*entry).next = (*head).next;
        (*entry).prev = head;
        (*(*head).next).prev = entry;
        (*head).next = entry;
    }
}

fn DListInsertTail(entry: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
    }
}

fn DListMerge(list: *mut crate::types::DListHead, head: *mut crate::types::DListHead) {
    unsafe {
        if (*list).next == list {
            return;
        }
        let list_first = (*list).next;
        let list_last = (*list).prev;
        let head_last = (*head).prev;

        (*head_last).next = list_first;
        (*list_first).prev = head_last;
        (*head).prev = list_last;
        (*list_last).next = head;

        DListHeadInit(list);
    }
}

fn DListGetCount(head: *const crate::types::DListHead)-> i32 {
    let mut count: i32 = 0;
    unsafe {
        let mut node = (*(head as *mut crate::types::DListHead)).next;
        while node != head as *mut crate::types::DListHead {
            count += 1;
            node = (*node).next;
        }
    }
    count
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

fn HdfDriverHead()-> *mut crate::types::DListHead {
    static mut DRIVER_HEAD: crate::types::DListHead = crate::types::DListHead {
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    unsafe {
        if DRIVER_HEAD.next.is_null() {
            DRIVER_HEAD.next = &mut DRIVER_HEAD as *mut crate::types::DListHead;
            DRIVER_HEAD.prev = &mut DRIVER_HEAD as *mut crate::types::DListHead;
        }
        &mut DRIVER_HEAD
    }
}

pub extern "C" fn HdfRegisterDriverEntry(entry: *const crate::types::HdfDriverEntry)-> i32 {
    if entry.is_null() || unsafe { (*entry).moduleName.is_null() } {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let layout = std::alloc::Layout::new::<crate::types::HdfDriver>();
    let new_driver = unsafe { std::alloc::alloc(layout) as *mut crate::types::HdfDriver };
    if new_driver.is_null() {
        return crate::types::HDF_DEV_ERR_NO_MEMORY;
    }

    unsafe {
        std::ptr::write_bytes(new_driver as *mut u8, 0, std::mem::size_of::<crate::types::HdfDriver>());
        (*new_driver).entry = entry;
    }

    let node_ptr: *mut crate::types::DListHead = unsafe { std::ptr::addr_of_mut!((*new_driver).node) };
    DListInsertTail(node_ptr, HdfDriverHead());

    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfUnregisterDriverEntry(entry: *const crate::types::HdfDriverEntry)-> i32 {
    if entry.is_null() || unsafe { (*entry).moduleName.is_null() } {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }

    let driver_head = unsafe { crate::src_driver_manager::HdfDriverHead() };

    let offset: usize = unsafe {
        let null: *const crate::types::HdfDriver = std::ptr::null();
        let node_ptr = std::ptr::addr_of!((*null).node);
        node_ptr as usize - null as usize
    };

    let container_of = |ptr: *mut crate::types::DListHead| -> *mut crate::types::HdfDriver {
        unsafe { (ptr as *mut u8).offset(-(offset as isize)) as *mut crate::types::HdfDriver }
    };

    unsafe {
        let mut node = (*driver_head).next;
        while node != driver_head {
            let driver = container_of(node);
            let next_node = (*node).next;
            if (*driver).entry == entry {
                crate::src_driver_manager::DListRemove(node);
                let layout = std::alloc::Layout::new::<crate::types::HdfDriver>();
                std::alloc::dealloc(driver as *mut u8, layout);
                break;
            }
            node = next_node;
        }
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfRegisterDriver(driver: *mut crate::types::HdfDriver)-> i32 {
    if driver.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let entry = unsafe { (*driver).entry };
    if entry.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if unsafe { (*entry).moduleName.is_null() } {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    unsafe {
        let node_ptr: *mut crate::types::DListHead = &mut (*driver).node;
        crate::src_driver_manager::DListInsertTail(node_ptr, crate::src_driver_manager::HdfDriverHead());
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfUnregisterDriver(driver: *mut crate::types::HdfDriver)-> i32 {
    if driver.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let driver_head = unsafe { crate::src_driver_manager::HdfDriverHead() };
    let offset = crate::compat::offset_of!(crate::types::HdfDriver, node);
    unsafe {
        let mut node_ptr = (*driver_head).next;
        while node_ptr != driver_head {
            let it = (node_ptr as *const u8).wrapping_sub(offset) as *mut crate::types::HdfDriver;
            if it == driver {
                crate::src_driver_manager::DListRemove(node_ptr as *mut crate::types::DListHead);
                break;
            }
            node_ptr = (*node_ptr).next;
        }
    }
    crate::types::HDF_SUCCESS
}

fn HdfDriverManagerFoundDriver(driverName: *const std::ffi::c_char) -> *mut crate::types::HdfDriver {
    let offset = crate::compat::offset_of!(crate::types::HdfDriver, node);
    let driver_head = unsafe { crate::src_driver_manager::HdfDriverHead() };
    if driver_head.is_null() {
        return std::ptr::null_mut();
    }
    let mut node_ptr = unsafe { (*driver_head).next };
    while node_ptr != driver_head {
        let driver = (node_ptr as *const u8).wrapping_sub(offset) as *mut crate::types::HdfDriver;
        let entry = unsafe { (*driver).entry };
        if !entry.is_null() {
            let module_name = unsafe { (*entry).moduleName };
            if !module_name.is_null() {
                let match_found = unsafe {
                    core::ffi::CStr::from_ptr(module_name) == core::ffi::CStr::from_ptr(driverName)
                };
                if match_found {
                    return driver;
                }
            }
        }
        node_ptr = unsafe { (*node_ptr).next };
    }
    std::ptr::null_mut()
}

pub extern "C" fn HdfDriverManagerGetDriver(driverName: *const std::ffi::c_char)-> *mut crate::types::HdfDriver {
    if driverName.is_null() {
        return std::ptr::null_mut();
    }
    let mut driver = crate::src_driver_manager::HdfDriverManagerFoundDriver(driverName);
    if !driver.is_null() {
        return driver;
    }
    let hdf_sys_event_send: Option<unsafe extern "C" fn(u32, u32, *const std::ffi::c_char, i32) -> i32> = None;
    if let Some(sys_event_send) = hdf_sys_event_send {
        let ret = unsafe {
            sys_event_send(
                crate::types::HDF_SYSEVENT_CLASS_MODULE,
                crate::types::KEVENT_MODULE_INSTALL as u32,
                driverName,
                1,
            )
        };
        if ret != crate::types::HDF_SUCCESS {
            return std::ptr::null_mut();
        }
        driver = crate::src_driver_manager::HdfDriverManagerFoundDriver(driverName);
    }
    if driver.is_null() {
    }
    driver
}

pub extern "C" fn HdfDriverManagerGetDriverList()-> *mut crate::types::DListHead {
    HdfDriverHead()
}
