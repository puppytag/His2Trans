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

/// Activation function pointer type: matches C `typedef double (*genann_actfun)(double a);`
/// Must be `extern "C"` to match the C ABI when stored in the struct and called.
pub type genann_actfun = extern "C" fn(f64) -> f64;

/// Rust-native struct for genann. Uses Vec<f64> for safe memory management
/// instead of raw pointers. Not repr(C) since this is a Rust-native project.
#[derive(Debug, Clone)]
pub struct genann {
    pub inputs: i32,
    pub hidden_layers: i32,
    pub hidden: i32,
    pub outputs: i32,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: i32,
    pub total_neurons: i32,
    pub weight: Vec<f64>,
    pub output: Vec<f64>,
    pub delta: Vec<f64>,
}


pub const ENODEV: i32 = 19;
pub const ENOSYS: i32 = 38;
pub const ERANGE: i32 = 34;
pub const ENOTCONN: i32 = 107;
pub const SOFTBUS_NOT_IMPLEMENT: i32 = -2;
pub const SOFTBUS_MEM_ERR: i32 = -4;
pub const SOFTBUS_MALLOC_ERR: i32 = -5;
pub const SOFTBUS_PERMISSION_DENIED: i32 = -6;
pub const SOFTBUS_NETWORK_ERR: i32 = -7;
pub const HDF_ERR_MALLOC_FAIL: i32 = -3;
pub const HDF_ERR_TIMEOUT: i32 = -4;
pub const HDF_ERR_NOT_SUPPORT: i32 = -10;
pub const HDF_ERR_IO: i32 = -8;
pub const HDF_ERR_DEVICE_BUSY: i32 = -22;
pub const LOS_ERRNO_BASE: u32 = 0x02000000;
pub const AUDIO_FORMAT_TYPE_PCM_16_BIT: i32 = 1;
pub const AUDIO_FORMAT_TYPE_PCM_8_BIT: i32 = 0;
pub const AUDIO_FORMAT_TYPE_PCM_24_BIT: i32 = 2;
pub const AUDIO_FORMAT_TYPE_PCM_32_BIT: i32 = 3;
