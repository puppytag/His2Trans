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

// Source: bindgen on preprocessed TU
pub static mut MAX_PERIOD_SILENCE_THRESHOLD: i32 = 16384;

// Source: bindgen on preprocessed TU
pub static mut MIN_PERIOD_SILENCE_THRESHOLD: i32 = 4096;

// Source: bindgen on preprocessed TU
pub static mut PERIOD_COUNT: i32 = 4;

// Source: bindgen on preprocessed TU
pub static mut RENDER_TRAF_BUF_SIZE: i32 = 1024;

// Source: bindgen on preprocessed TU
pub static mut SLEEP_TIME: i32 = 5;

// Source: bindgen on preprocessed TU
pub static mut TIME_OUT_CONST: i32 = 50;
