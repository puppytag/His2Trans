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
// MISSING: buffer_new

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: buffer_new_with_copy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: buffer_new_with_size

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: buffer_new_with_string

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: buffer_new_with_string_length

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: buffer_slice

// Source: bindgen on preprocessed TU
pub static mut stderr: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };

// Source: bindgen on preprocessed TU
pub static mut stdin: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };

// Source: bindgen on preprocessed TU
pub static mut stdout: *mut crate::types::FILE = unsafe { MaybeUninit::<*mut crate::types::FILE>::zeroed().assume_init() };
