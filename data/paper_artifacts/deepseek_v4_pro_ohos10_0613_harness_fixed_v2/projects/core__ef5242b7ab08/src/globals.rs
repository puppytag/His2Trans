//! Global and Static Variable Declarations (Scheme A: bindgen-truth static storage)
//!
//! Controller lists use UnsafeCell-based safe statics; every access returns a *mut DListHead
//! that must be dereferenced inside unsafe blocks.
//! Types are derived from bindgen on the exact preprocessed `.i` TU.
//! NOTE: file-scope `static` (internal linkage) variables are emitted in each module file (Scheme B).

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use core::cell::UnsafeCell;
use core::ptr::{null_mut};
use crate::types::*;

// ==========================================
// Safe wrapper for global controller lists
// ==========================================

pub struct ControllerList(UnsafeCell<DListHead>);

// SAFETY: ControllerList is only accessed behind raw pointers obtained via get_ptr();
// the caller is responsible for synchronization (matching C's free-threading semantics).
unsafe impl Send for ControllerList {}
unsafe impl Sync for ControllerList {}

impl ControllerList {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(DListHead { next: null_mut(), prev: null_mut() }))
    }

    /// Returns a mutable raw pointer to the inner DListHead.
    /// The caller must ensure proper synchronization and must not create `&mut`
    /// references that alias with other accesses to the same static.
    pub fn get_ptr(&self) -> *mut DListHead {
        self.0.get()
    }
}

// ==========================================
// Global Variables (top-level)
// ==========================================

// Source: bindgen on preprocessed TU
pub static codecController: ControllerList = ControllerList::new();

// Source: bindgen on preprocessed TU
pub static daiController: ControllerList = ControllerList::new();

// Source: bindgen on preprocessed TU
pub static dspController: ControllerList = ControllerList::new();

// Source: C production src/audio_host.c:448-456
#[no_mangle]
#[used]
#[link_section = ".hdf.drivers"]
pub static mut g_audioDriverEntry: crate::types::HdfDriverEntry = crate::types::HdfDriverEntry {
    moduleVersion: 1,
    moduleName: b"HDF_AUDIO\0" as *const u8 as *const ::core::ffi::c_char,
    Bind: Some(crate::src_audio_host::AudioDriverBind as unsafe extern "C" fn(*mut crate::types::HdfDeviceObject) -> i32),
    Init: Some(crate::src_audio_host::AudioDriverInit as unsafe extern "C" fn(*mut crate::types::HdfDeviceObject) -> i32),
    Release: Some(crate::src_audio_host::AudioDriverRelease as unsafe extern "C" fn(*mut crate::types::HdfDeviceObject)),
};

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: g_audioDriverEntryHdfEntry (HDF_INIT pointer in driver section; not implemented in Rust)

// Source: C production src/audio_host.c:18 (AUDIO_LIST_HEAD macro)
pub static g_cardManager: ControllerList = ControllerList::new();

use core::sync::atomic::{AtomicBool, Ordering};

// Source: bindgen on preprocessed TU (g_cardManager_initialized)
pub static g_cardManager_initialized: AtomicBool = AtomicBool::new(false);

// Source: bindgen on preprocessed TU
pub static platformController: ControllerList = ControllerList::new();