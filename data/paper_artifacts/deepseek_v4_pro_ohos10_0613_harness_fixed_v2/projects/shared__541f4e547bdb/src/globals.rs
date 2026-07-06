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
// MISSING: DevSvcManagerClntGetService

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcRecordNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: GetLastFatalMessage

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceGetServiceName

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceInfoNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfIoServiceAdapterObtain

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfIoServiceAdapterPublish

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfIoServiceBind

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfIoServiceGroupObtain

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfIoServicePublish

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfObjectManagerGetCreators

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfObjectManagerGetObject

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListGetLast

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListIteratorNext

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListNext

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListPeek

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListPop

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSListSearch

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufBind

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufCopy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufGetData

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufGetImpl

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufMove

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufObtain

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufObtainDefaultSize

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufReadRemoteService

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufReadString

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufReadString16

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufReadUnpadBuffer

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufTypedBind

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufTypedObtain

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufTypedObtainCapacity

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfSbufTypedObtainInplace

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: IoServiceStatusListenerNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemAlloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemAllocAlign

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemCalloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: SvcMgrIoserviceGet

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __asm__

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __memchr_chk

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __memrchr_chk

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __sched_cpualloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __strchr_chk

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: __strrchr_chk

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: asctime

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: asctime_r

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: basename

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: calloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: ctime

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: ctime_r

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: getdate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: gmtime

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: gmtime_r

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: index

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: localtime

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: localtime_noenv_r

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: localtime_r

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memccpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memchr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memcpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memmem

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memmove

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: mempcpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memrchr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: memset

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: pthread_getspecific

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: rindex

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: stpcpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: stpncpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strcasestr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strcat

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strchr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strchrnul

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strcpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strdup

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strerror

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strerror_l

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strncat

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strncpy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strndup

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strpbrk

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strptime

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strrchr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strsep

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strsignal

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strstr

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strtok

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: strtok_r

// Source: bindgen on preprocessed TU
pub static mut tzname: [*mut ::core::ffi::c_char; 2usize] = [std::ptr::null_mut(); 2usize];
