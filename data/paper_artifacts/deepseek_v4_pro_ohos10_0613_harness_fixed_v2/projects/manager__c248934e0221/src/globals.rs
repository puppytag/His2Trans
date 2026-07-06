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
// MISSING: DevHostServiceClntNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevHostServiceCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevHostServiceNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcManagerClntGetService

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcManagerCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcManagerGetInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcManagerGetService

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevSvcRecordNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DeviceTokenClntNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevmgrServiceCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DevmgrServiceGetInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DriverInstallerCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: DriverInstallerGetInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: GetLastFatalMessage

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfCStringObtain

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceGetServiceName

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceInfoNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceNodeNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceTokenCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDeviceTokenNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDriverManagerGetDriver

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfDriverManagerGetDriverList

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfGetHcsRootNode

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: HdfHostInfoNewInstance

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
// MISSING: HdfStringCopy

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: MapGet

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemAlloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemAllocAlign

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: OsalMemCalloc

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: PowerStateTokenClntNewInstance

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: ServStatListenerHolderCreate

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: ServStatListenerHolderGet

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
/// # Safety
/// This static is zero-initialized with null pointers. All-null is a valid state for `[*mut c_char; 2]`.
/// This variable is not referenced elsewhere in the crate; kept for FFI compatibility.
pub static mut tzname: [*mut ::core::ffi::c_char; 2usize] = [::core::ptr::null_mut(); 2];

// ==========================================
// Lifted Static Variables (from functions)
// ==========================================

// From function: DevSvcManagerExtStart()
/// Originally: static HdfDeviceObject svcmgrDevObj
/// # Safety
/// This static mut is zero-initialized via MaybeUninit::zeroed().
/// It contains an opaque pthread_rwlock_t that cannot be safely field-initialized.
/// `setup_io_service` writes all fields before any FFI callback reads them.
/// This static is only ever dereferenced inside `unsafe fn setup_io_service`.
pub static mut DevSvcManagerExtStart_svcmgrDevObj: crate::types::HdfDeviceObject = unsafe { MaybeUninit::<crate::types::HdfDeviceObject>::zeroed().assume_init() };
/// Originally: static IDeviceIoService svcmgrIoService
/// # Safety
/// This static mut is zero-initialized via MaybeUninit::zeroed().
/// It contains an HdfObject (opaque) and function pointers whose zero representation
/// may be invalid. `setup_io_service` writes valid callback pointers (Open, Release)
/// and a valid object before any external code invokes the IDeviceIoService vtable.
/// Dispatch intentionally stays None. This static is only ever dereferenced inside
/// `unsafe fn setup_io_service`.
pub static mut DevSvcManagerExtStart_svcmgrIoService: crate::types::IDeviceIoService = unsafe { MaybeUninit::<crate::types::IDeviceIoService>::zeroed().assume_init() };

