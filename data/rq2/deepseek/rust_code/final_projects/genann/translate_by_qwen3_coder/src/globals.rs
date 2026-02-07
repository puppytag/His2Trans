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
use crate::types::*;

// ==========================================
// Global Variables (top-level)
// ==========================================

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: genann_copy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: genann_init

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: genann_read

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: genann_run

// Source: bindgen on preprocessed TU
pub static mut stderr: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };

// Source: bindgen on preprocessed TU
pub static mut stdin: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };

// Source: bindgen on preprocessed TU
pub static mut stdout: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };

// ==========================================
// Lifted Static Variables (from functions)
// ==========================================

// From function: genann_act_sigmoid_cached()
/// Originally: static int initialized
pub static mut genann_act_sigmoid_cached_initialized: i32 = unsafe { MaybeUninit::<i32>::zeroed().assume_init() };
/// Originally: static double lookup
pub static mut genann_act_sigmoid_cached_lookup: f64 = unsafe { MaybeUninit::<f64>::zeroed().assume_init() };
