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
// MISSING: quadtree_bounds_new

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_new

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_node_new

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_node_with_bounds

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_point_new

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_search

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: quadtree_walk
