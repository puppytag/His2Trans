//! Auto-generated types module (TU truth entry)
//!
//! - Source of truth: stage1 pinned preprocessed `.i` from tu_context_map.json
//! - Shared/global types are re-exported here (crate::types::*).
//! - Per-module private types live in each module's `local_types` submodule.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused)]

// ============================================================
// C2R: TU-pinned type supplements (from stage1 `.i` truth)
// ============================================================
pub mod __c2r_tu_types_src_osal_mem {
    include!("__c2r_generated/tu_types_src_osal_mem.rs");
}
pub use __c2r_tu_types_src_osal_mem::{size_t};

pub mod __c2r_tu_types_src_osal_mutex {
    include!("__c2r_generated/tu_types_src_osal_mutex.rs");
}
pub use __c2r_tu_types_src_osal_mutex::{OsalMutex};

pub mod __c2r_tu_types_src_osal_sem {
    include!("__c2r_generated/tu_types_src_osal_sem.rs");
}
pub use __c2r_tu_types_src_osal_sem::{OsalSem};

pub mod __c2r_tu_types_src_osal_spinlock {
    include!("__c2r_generated/tu_types_src_osal_spinlock.rs");
}
pub use __c2r_tu_types_src_osal_spinlock::{OsalSpinlock};

pub mod __c2r_tu_types_src_osal_thread {
    include!("__c2r_generated/tu_types_src_osal_thread.rs");
}
pub use __c2r_tu_types_src_osal_thread::{OSAL_THREAD_PRIORITY, OSAL_THREAD_PRI_LOW, OSAL_THREAD_PRI_DEFAULT, OSAL_THREAD_PRI_HIGH, OSAL_THREAD_PRI_HIGHEST, OsalThread, OsalThreadEntry, OsalThreadParam, ThreadWrapper, __pthread, pthread_attr_t, pthread_attr_t__bindgen_ty_1, pthread_t, sched_param};

pub mod __c2r_tu_types_src_osal_time {
    include!("__c2r_generated/tu_types_src_osal_time.rs");
}
pub use __c2r_tu_types_src_osal_time::{OsalTimespec};


// ============================================================
// TU Truth Constants (from preprocessed translation units)
// ============================================================

pub const HDF_BSP_ERR_OP: i32 = -101;
pub const HDF_DEV_ERR_ATTACHDEV_FAIL: i32 = -206;
pub const HDF_DEV_ERR_DEV_INIT_FAIL: i32 = -204;
pub const HDF_DEV_ERR_NETDOWN: i32 = -211;
pub const HDF_DEV_ERR_NODATA: i32 = -207;
pub const HDF_DEV_ERR_NORANGE: i32 = -208;
pub const HDF_DEV_ERR_NO_DEVICE: i32 = -202;
pub const HDF_DEV_ERR_NO_DEVICE_SERVICE: i32 = -203;
pub const HDF_DEV_ERR_NO_MEMORY: i32 = -201;
pub const HDF_DEV_ERR_OP: i32 = -210;
pub const HDF_DEV_ERR_PUBLISH_FAIL: i32 = -205;
pub const HDF_ERR_BAD_FD: i32 = -18;
pub const HDF_ERR_BSP_PLT_API_ERR: i32 = -102;
pub const HDF_ERR_DEVICE_BUSY: i32 = -16;
pub const HDF_ERR_INVALID_OBJECT: i32 = -4;
pub const HDF_ERR_INVALID_PARAM: i32 = -3;
pub const HDF_ERR_IO: i32 = -17;
pub const HDF_ERR_MALLOC_FAIL: i32 = -6;
pub const HDF_ERR_NOPERM: i32 = -19;
pub const HDF_ERR_NOT_SUPPORT: i32 = -2;
pub const HDF_ERR_OUT_OF_RANGE: i32 = -20;
pub const HDF_ERR_QUEUE_FULL: i32 = -15;
pub const HDF_ERR_THREAD_CREATE_FAIL: i32 = -10;
pub const HDF_ERR_TIMEOUT: i32 = -7;
pub const HDF_FAILURE: i32 = -1;
pub const HDF_PAL_ERR_DEV_CREATE: i32 = -103;
pub const HDF_PAL_ERR_INNER: i32 = -104;
pub const HDF_SUCCESS: i32 = 0;
pub const LOG_APP: i32 = 0;
pub const LOG_CORE: i32 = 3;
pub const LOG_DEBUG: i32 = 3;
pub const LOG_ERROR: i32 = 6;
pub const LOG_FATAL: i32 = 7;
pub const LOG_INFO: i32 = 4;
pub const LOG_INIT: i32 = 1;
pub const LOG_KMSG: i32 = 4;
pub const LOG_LEVEL_MAX: i32 = 8;
pub const LOG_LEVEL_MIN: i32 = 0;
pub const LOG_ONLY_PRERELEASE: i32 = 5;
pub const LOG_TYPE_MAX: i32 = 6;
pub const LOG_TYPE_MIN: i32 = 0;
pub const LOG_WARN: i32 = 5;

// --- POSIX Thread Types ---
#[repr(C)]
pub struct pthread_mutex_t { _opaque: [u8; 40] }
#[repr(C)]
pub struct pthread_cond_t { _opaque: [u8; 48] }
#[repr(C)]
pub struct pthread_rwlock_t { _opaque: [u8; 56] }
pub type pthread_once_t = i32;


// ============================================================
// Common Constants (自动追加)
// ============================================================

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { _opaque: [0u8; 40] };
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { _opaque: [0u8; 48] };
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t { _opaque: [0u8; 56] };
pub const PTHREAD_ONCE_INIT: pthread_once_t = 0_i32;
pub const __PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { _opaque: [0u8; 40] };
pub const __PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { _opaque: [0u8; 48] };
pub const __PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t { _opaque: [0u8; 56] };
pub const __PTHREAD_ONCE_INIT: pthread_once_t = 0_i32;
pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;
pub const EBUSY: i32 = 16;
pub const EAGAIN: i32 = 11;
pub const ETIMEDOUT: i32 = 110;
pub const ENODEV: i32 = 19;
pub const EFAULT: i32 = 14;
pub const ENOSYS: i32 = 38;
pub const ERANGE: i32 = 34;
pub const ENOTCONN: i32 = 107;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub const SOFTBUS_OK: i32 = 0;
pub const SOFTBUS_ERR: i32 = -1;
pub const SOFTBUS_NOT_IMPLEMENT: i32 = -2;
pub const SOFTBUS_INVALID_PARAM: i32 = -3;
pub const SOFTBUS_MEM_ERR: i32 = -4;
pub const SOFTBUS_MALLOC_ERR: i32 = -5;
pub const SOFTBUS_PERMISSION_DENIED: i32 = -6;
pub const SOFTBUS_NETWORK_ERR: i32 = -7;
pub const LOS_OK: i32 = 0;
pub const LOS_NOK: i32 = -1;
pub const LOS_ERRNO_BASE: u32 = 0x02000000;
pub const AUDIO_FORMAT_TYPE_PCM_16_BIT: i32 = 1;
pub const AUDIO_FORMAT_TYPE_PCM_8_BIT: i32 = 0;
pub const AUDIO_FORMAT_TYPE_PCM_24_BIT: i32 = 2;
pub const AUDIO_FORMAT_TYPE_PCM_32_BIT: i32 = 3;
// === C2R_AUTO_MISSING_TYPES_BEGIN ===
/// C2R_AUTO_TYPE: placeholder for external type `pthread_spinlock_t`
pub type pthread_spinlock_t = usize;

// === C2R_AUTO_MISSING_TYPES_END ===
