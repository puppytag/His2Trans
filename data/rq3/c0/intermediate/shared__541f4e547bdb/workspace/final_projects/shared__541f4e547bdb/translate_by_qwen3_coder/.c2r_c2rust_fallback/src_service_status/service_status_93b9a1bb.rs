#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type HdfSBuf;
    fn HdfSbufWriteUint16(sbuf: *mut HdfSBuf, value: uint16_t) -> bool;
    fn HdfSbufWriteString(sbuf: *mut HdfSBuf, value: *const core::ffi::c_char) -> bool;
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn HdfSbufReadUint16(sbuf: *mut HdfSBuf, value: *mut uint16_t) -> bool;
    fn HdfSbufReadString(sbuf: *mut HdfSBuf) -> *const core::ffi::c_char;
}
pub type LogType = core::ffi::c_uint;
pub const LOG_TYPE_MAX: LogType = 4;
pub const LOG_CORE: LogType = 3;
pub const LOG_INIT: LogType = 1;
pub const LOG_TYPE_MIN: LogType = 0;
pub type LogLevel = core::ffi::c_uint;
pub const LOG_FATAL: LogLevel = 7;
pub const LOG_ERROR: LogLevel = 6;
pub const LOG_WARN: LogLevel = 5;
pub const LOG_INFO: LogLevel = 4;
pub const LOG_DEBUG: LogLevel = 3;
pub type uint16_t = core::ffi::c_ushort;
pub type C2RustUnnamed = core::ffi::c_int;
pub const HDF_DEV_ERR_NETDOWN: C2RustUnnamed = -211;
pub const HDF_DEV_ERR_OP: C2RustUnnamed = -210;
pub const HDF_DEV_ERR_NORANGE: C2RustUnnamed = -208;
pub const HDF_DEV_ERR_NODATA: C2RustUnnamed = -207;
pub const HDF_DEV_ERR_ATTACHDEV_FAIL: C2RustUnnamed = -206;
pub const HDF_DEV_ERR_PUBLISH_FAIL: C2RustUnnamed = -205;
pub const HDF_DEV_ERR_DEV_INIT_FAIL: C2RustUnnamed = -204;
pub const HDF_DEV_ERR_NO_DEVICE_SERVICE: C2RustUnnamed = -203;
pub const HDF_DEV_ERR_NO_DEVICE: C2RustUnnamed = -202;
pub const HDF_DEV_ERR_NO_MEMORY: C2RustUnnamed = -201;
pub const HDF_PAL_ERR_INNER: C2RustUnnamed = -104;
pub const HDF_PAL_ERR_DEV_CREATE: C2RustUnnamed = -103;
pub const HDF_ERR_BSP_PLT_API_ERR: C2RustUnnamed = -102;
pub const HDF_BSP_ERR_OP: C2RustUnnamed = -101;
pub const HDF_ERR_OUT_OF_RANGE: C2RustUnnamed = -20;
pub const HDF_ERR_NOPERM: C2RustUnnamed = -19;
pub const HDF_ERR_BAD_FD: C2RustUnnamed = -18;
pub const HDF_ERR_IO: C2RustUnnamed = -17;
pub const HDF_ERR_DEVICE_BUSY: C2RustUnnamed = -16;
pub const HDF_ERR_QUEUE_FULL: C2RustUnnamed = -15;
pub const HDF_ERR_THREAD_CREATE_FAIL: C2RustUnnamed = -10;
pub const HDF_ERR_TIMEOUT: C2RustUnnamed = -7;
pub const HDF_ERR_MALLOC_FAIL: C2RustUnnamed = -6;
pub const HDF_ERR_INVALID_OBJECT: C2RustUnnamed = -4;
pub const HDF_ERR_INVALID_PARAM: C2RustUnnamed = -3;
pub const HDF_ERR_NOT_SUPPORT: C2RustUnnamed = -2;
pub const HDF_FAILURE: C2RustUnnamed = -1;
pub const HDF_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ServiceStatus {
    pub serviceName: *const core::ffi::c_char,
    pub deviceClass: uint16_t,
    pub status: uint16_t,
    pub info: *const core::ffi::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn ServiceStatusMarshalling(
    mut status: *mut ServiceStatus,
    mut buf: *mut HdfSBuf,
) -> core::ffi::c_int {
    if status.is_null() || buf.is_null() || ((*status).serviceName).is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    if !HdfSbufWriteString(buf, (*status).serviceName)
        || !HdfSbufWriteUint16(buf, (*status).deviceClass)
        || !HdfSbufWriteUint16(buf, (*status).status)
        || !HdfSbufWriteString(
            buf,
            (if !((*status).info).is_null() {
                (*status).info
            } else {
                b"\0" as *const u8 as *const core::ffi::c_char
            }),
        )
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd002510 as core::ffi::c_uint,
            b"HDF_LOG_TAG\0" as *const u8 as *const core::ffi::c_char,
            b"failed to marshalling service status\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ServiceStatusUnMarshalling(
    mut status: *mut ServiceStatus,
    mut buf: *mut HdfSBuf,
) -> core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    (*status).serviceName = HdfSbufReadString(buf);
    if ((*status).serviceName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd002510 as core::ffi::c_uint,
            b"HDF_LOG_TAG\0" as *const u8 as *const core::ffi::c_char,
            b"failed to unmarshalling service status, service name is null\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    if !HdfSbufReadUint16(buf, &mut (*status).deviceClass)
        || !HdfSbufReadUint16(buf, &mut (*status).status)
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd002510 as core::ffi::c_uint,
            b"HDF_LOG_TAG\0" as *const u8 as *const core::ffi::c_char,
            b"failed to unmarshalling service status, deviceClass or status invalid\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    (*status).info = HdfSbufReadString(buf);
    return HDF_SUCCESS as core::ffi::c_int;
}
