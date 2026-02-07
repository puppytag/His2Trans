//! Auto-generated stub type definitions
//!
//! This is a STUB file generated as a fallback when bindgen failed.
//! All custom types are declared as opaque structs to ensure compilation.
//!
//! Generation mode: Tier C (guaranteed compilation)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused)]

// ============================================================
// Core C Type Mappings (guaranteed correct)
// ============================================================

pub type c_void = core::ffi::c_void;
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

// Fixed-width integer types
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// Size types
pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

// POSIX types
pub type off_t = i64;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type mode_t = u32;
pub type time_t = i64;

// Boolean type
pub type BOOL = i32;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

// ============================================================
// Common System Types (opaque definitions)
// ============================================================

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILE { _opaque: [u8; 0] }

#[repr(C)]
pub struct pthread_mutex_t { _opaque: [u8; 40] }

#[repr(C)]
pub struct pthread_cond_t { _opaque: [u8; 48] }

#[repr(C)]
pub struct pthread_attr_t { _opaque: [u8; 56] }

pub type pthread_t = usize;

// PTHREAD initializers
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = unsafe { ::core::mem::zeroed() };
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = unsafe { ::core::mem::zeroed() };

// ============================================================
// Common Error Codes
// ============================================================

pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;
pub const EAGAIN: i32 = 11;
pub const ETIMEDOUT: i32 = 110;
pub const EBUSY: i32 = 16;
pub const EPERM: i32 = 1;
pub const EFAULT: i32 = 14;

// ============================================================
// Framework-specific Constants (OpenHarmony/HDF/LiteOS)
// ============================================================

// SoftBus
pub const SOFTBUS_OK: i32 = 0;
pub const SOFTBUS_ERR: i32 = -1;
pub const SOFTBUS_INVALID_PARAM: i32 = -3;

// HDF
pub const HDF_SUCCESS: i32 = 0;
pub const HDF_FAILURE: i32 = -1;
pub const HDF_ERR_INVALID_PARAM: i32 = -3;

// LiteOS
pub const LOS_OK: u32 = 0;
pub const LOS_NOK: u32 = 1;
pub const LOS_ERRNO_TSK_ID_INVALID: u32 = 0x02000207;


// ============================================================
// Project-specific Types (scanned from headers, opaque)
// ============================================================

/// Opaque placeholder for external type `ZopfliBlockState`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZopfliBlockState { _opaque: [u8; 0] }

/// Opaque placeholder for external type `ZopfliHash`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZopfliHash { _opaque: [u8; 0] }

/// Opaque placeholder for external type `ZopfliLZ77Store`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZopfliLZ77Store { _opaque: [u8; 0] }

/// Opaque placeholder for external type `ZopfliLongestMatchCache`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZopfliLongestMatchCache { _opaque: [u8; 0] }

/// Opaque placeholder for external type `ZopfliOptions`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZopfliOptions { _opaque: [u8; 0] }

// ============================================================
// C2R: TU-pinned type supplements (from stage1 `.i` truth)
// ============================================================
pub mod __c2r_tu_types_src_blocksplitter {
    include!("__c2r_generated/tu_types_src_blocksplitter.rs");
}
pub use __c2r_tu_types_src_blocksplitter::{FindMinimumFun};

pub mod __c2r_tu_types_src_katajainen {
    include!("__c2r_generated/tu_types_src_katajainen.rs");
}
pub use __c2r_tu_types_src_katajainen::{Node, NodePool};

pub mod __c2r_tu_types_src_squeeze {
    include!("__c2r_generated/tu_types_src_squeeze.rs");
}
pub use __c2r_tu_types_src_squeeze::{CostModelFun, RanState, SymbolStats};

pub mod __c2r_tu_types_src_zopfli_bin {
    include!("__c2r_generated/tu_types_src_zopfli_bin.rs");
}
pub use __c2r_tu_types_src_zopfli_bin::{ZopfliFormat};
// === C2R_AUTO_MISSING_TYPES_BEGIN ===
/// C2R_AUTO_TYPE: placeholder for external type `SplitCostContext`
// C2R_STRUCT_RECOVERY: `SplitCostContext`
// C2R_FROM_C: /data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/blocksplitter.c:113
#[repr(C)]
pub struct SplitCostContext {
    pub end: usize,
    pub lz77: *const ZopfliLZ77Store,
    pub start: usize,
    pub _c2r_private: [u8; 0],
}
// === C2R_AUTO_MISSING_TYPES_END ===
