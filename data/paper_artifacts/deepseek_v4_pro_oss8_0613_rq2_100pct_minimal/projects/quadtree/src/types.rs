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

/// Opaque placeholder for external type `quadtree`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct quadtree { _opaque: [u8; 0] }

/// Opaque placeholder for external type `quadtree_bounds`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct quadtree_bounds { _opaque: [u8; 0] }

/// quadtree_bounds_t: bounding box with corner points and dimensions
#[derive(Debug)]
pub struct quadtree_bounds_t {
    pub nw: Box<quadtree_point_t>,
    pub se: Box<quadtree_point_t>,
    pub width: f64,
    pub height: f64,
}

/// Opaque placeholder for external type `quadtree_node`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct quadtree_node { _opaque: [u8; 0] }

/// quadtree_node_t: quadtree node with child pointers, bounds, point, and key
#[derive(Debug)]
pub struct quadtree_node_t {
    pub ne: Option<Box<quadtree_node_t>>,
    pub nw: Option<Box<quadtree_node_t>>,
    pub se: Option<Box<quadtree_node_t>>,
    pub sw: Option<Box<quadtree_node_t>>,
    pub bounds: Option<Box<quadtree_bounds_t>>,
    pub point: Option<Box<quadtree_point_t>>,
    pub key: *mut c_void,
}

/// Opaque placeholder for external type `quadtree_point`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct quadtree_point { _opaque: [u8; 0] }

/// quadtree_point_t: a 2D point with f64 coordinates
#[repr(C)]
#[derive(Debug)]
pub struct quadtree_point_t {
    pub x: f64,
    pub y: f64,
}
/// quadtree_t: root node, key_free callback, and element count
#[derive(Debug)]
pub struct quadtree_t {
    pub root: Option<Box<quadtree_node_t>>,
    pub key_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub length: c_uint,
}




// ============================================================
// Safe accessor methods for project structs
// ============================================================

impl quadtree_node_t {
    /// Safe immutable access to child node references. Returns None if null.
    pub fn ne_as_ref(&self) -> Option<&quadtree_node_t> { self.ne.as_deref() }
    pub fn nw_as_ref(&self) -> Option<&quadtree_node_t> { self.nw.as_deref() }
    pub fn se_as_ref(&self) -> Option<&quadtree_node_t> { self.se.as_deref() }
    pub fn sw_as_ref(&self) -> Option<&quadtree_node_t> { self.sw.as_deref() }

    /// Safe mutable access to child node references. Returns None if null.
    pub fn ne_as_mut(&mut self) -> Option<&mut quadtree_node_t> { self.ne.as_deref_mut() }
    pub fn nw_as_mut(&mut self) -> Option<&mut quadtree_node_t> { self.nw.as_deref_mut() }
    pub fn se_as_mut(&mut self) -> Option<&mut quadtree_node_t> { self.se.as_deref_mut() }
    pub fn sw_as_mut(&mut self) -> Option<&mut quadtree_node_t> { self.sw.as_deref_mut() }

    /// Safe immutable access to bounds. Returns None if null.
    pub fn bounds_as_ref(&self) -> Option<&quadtree_bounds_t> { self.bounds.as_deref() }
    pub fn bounds_as_mut(&mut self) -> Option<&mut quadtree_bounds_t> { self.bounds.as_deref_mut() }

    /// Safe immutable access to point. Returns None if null.
    pub fn point_as_ref(&self) -> Option<&quadtree_point_t> { self.point.as_deref() }
    pub fn point_as_mut(&mut self) -> Option<&mut quadtree_point_t> { self.point.as_deref_mut() }

    /// Check if node is an internal pointer (has children, no point).
    pub fn is_pointer(&self) -> bool {
        self.nw.is_some() && self.ne.is_some() && self.sw.is_some() && self.se.is_some()
            && self.point.is_none()
    }
    /// Check if node is empty (no children, no point).
    pub fn is_empty(&self) -> bool {
        self.nw.is_none() && self.ne.is_none() && self.sw.is_none() && self.se.is_none()
            && self.point.is_none()
    }
    /// Check if node is a leaf (has a point).
    pub fn is_leaf(&self) -> bool {
        self.point.is_some()
    }
}

impl quadtree_bounds_t {
    pub fn nw_as_ref(&self) -> &quadtree_point_t { &self.nw }
    pub fn nw_as_mut(&mut self) -> &mut quadtree_point_t { &mut self.nw }
    pub fn se_as_ref(&self) -> &quadtree_point_t { &self.se }
    pub fn se_as_mut(&mut self) -> &mut quadtree_point_t { &mut self.se }
}

// ============================================================
// Common Constants (自动追加)
// ============================================================


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
