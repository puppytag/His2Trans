//! Global and Static Variable Declarations (Scheme A: bindgen-truth static storage)
//!
//! - No safe wrappers (Mutex/RwLock).
//! - Types are derived from bindgen on the exact preprocessed `.i` TU.
//! - Storage is real Rust `static mut`, zero-initialized (C-like).
//! - NOTE: file-scope `static` (internal linkage) variables are emitted in each module file (Scheme B).

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::types::*;

// ==========================================
// Global Variables (top-level)
// ==========================================

// Source: bindgen on preprocessed TU
pub static mut APP_GALLERY: [::core::ffi::c_char; 12usize] = [0 as ::core::ffi::c_char; 12usize];

// Source: bindgen on preprocessed TU
pub static mut ENTERPRISE: [::core::ffi::c_char; 11usize] = [0 as ::core::ffi::c_char; 11usize];

// Source: bindgen on preprocessed TU
pub static mut ENTERPRISE_MDM: [::core::ffi::c_char; 15usize] = [0 as ::core::ffi::c_char; 15usize];

// Source: bindgen on preprocessed TU
pub static mut ENTERPRISE_NORMAL: [::core::ffi::c_char; 18usize] = [0 as ::core::ffi::c_char; 18usize];

// Source: bindgen on preprocessed TU
pub static mut INTERNALTESTING: [::core::ffi::c_char; 16usize] = [0 as ::core::ffi::c_char; 16usize];

// Source: bindgen on preprocessed TU
pub static mut OS_INTEGRATION: [::core::ffi::c_char; 15usize] = [0 as ::core::ffi::c_char; 15usize];

// ==========================================
// Extern bindings for globals defined in native/globals.c
// ==========================================

extern "C" {
    // Originally `const TrustAppCert g_trustAppList[3]` in app_verify.c
    static g_trustAppList: [TrustAppCert; 3usize];
}

extern "C" {
    // Originally `const TrustAppCert g_trustAppListTest[2]` in app_verify.c
    static g_trustAppListTest: [TrustAppCert; 2usize];
}

/// Safe accessor for g_trustAppList
pub fn trust_app_list() -> &'static [TrustAppCert; 3] {
    unsafe { &g_trustAppList }
}

/// Safe accessor for g_trustAppListTest
pub fn trust_app_list_test() -> &'static [TrustAppCert; 2] {
    unsafe { &g_trustAppListTest }
}

// ==========================================
// Mutable globals originally static in app_verify.c
// ==========================================

pub static g_isDebugMode: AtomicBool = AtomicBool::new(false);


pub static g_isActsMode: AtomicBool = AtomicBool::new(false);
pub static mut g_message_func: crate::types::MessageFunc = None;

pub fn cstr_from_static_mut_array(arr: &[::core::ffi::c_char]) -> &'static ::core::ffi::CStr {
    let ptr = arr.as_ptr();
    unsafe { ::core::ffi::CStr::from_ptr(ptr) }
}

pub fn app_gallery_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &APP_GALLERY })
}
pub fn enterprise_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &ENTERPRISE })
}
pub fn enterprise_mdm_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &ENTERPRISE_MDM })
}
pub fn enterprise_normal_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &ENTERPRISE_NORMAL })
}
pub fn internaltesting_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &INTERNALTESTING })
}
pub fn os_integration_cstr() -> &'static ::core::ffi::CStr {
    cstr_from_static_mut_array(unsafe { &OS_INTEGRATION })
}
