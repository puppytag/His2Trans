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
    fn HdfIoServiceAdapterObtain(
        serviceName: *const core::ffi::c_char,
    ) -> *mut HdfIoService;
    fn HdfIoServiceAdapterRecycle(service: *mut HdfIoService);
    fn HdfIoServiceAdapterPublish(
        serviceName: *const core::ffi::c_char,
        mode: uint32_t,
    ) -> *mut HdfIoService;
    fn HdfIoServiceAdapterRemove(service: *mut HdfIoService);
}
pub type int32_t = core::ffi::c_int;
pub type uint32_t = core::ffi::c_uint;
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
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfIoService {
    pub object: HdfObject,
    pub target: *mut HdfObject,
    pub dispatcher: *mut HdfIoDispatcher,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfIoDispatcher {
    pub Dispatch: Option<
        unsafe extern "C" fn(
            *mut HdfObject,
            core::ffi::c_int,
            *mut HdfSBuf,
            *mut HdfSBuf,
        ) -> core::ffi::c_int,
    >,
}
#[no_mangle]
pub unsafe extern "C" fn HdfIoServiceBind(
    mut serviceName: *const core::ffi::c_char,
) -> *mut HdfIoService {
    return HdfIoServiceAdapterObtain(serviceName);
}
#[no_mangle]
pub unsafe extern "C" fn HdfIoServiceRecycle(mut service: *mut HdfIoService) {
    HdfIoServiceAdapterRecycle(service);
}
#[no_mangle]
pub unsafe extern "C" fn HdfIoServicePublish(
    mut serviceName: *const core::ffi::c_char,
    mut mode: uint32_t,
) -> *mut HdfIoService {
    if (Some(
        HdfIoServiceAdapterPublish
            as unsafe extern "C" fn(
                *const core::ffi::c_char,
                uint32_t,
            ) -> *mut HdfIoService,
    ))
        .is_some()
    {
        return HdfIoServiceAdapterPublish(serviceName, mode);
    }
    return 0 as *mut HdfIoService;
}
#[no_mangle]
pub unsafe extern "C" fn HdfIoServiceRemove(mut service: *mut HdfIoService) {
    if (Some(HdfIoServiceAdapterRemove as unsafe extern "C" fn(*mut HdfIoService) -> ()))
        .is_some()
    {
        HdfIoServiceAdapterRemove(service);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HdfIoServiceDispatch(
    mut ioService: *mut HdfIoService,
    mut cmdId: core::ffi::c_int,
    mut data: *mut HdfSBuf,
    mut reply: *mut HdfSBuf,
) -> int32_t {
    if ioService.is_null() || ((*ioService).dispatcher).is_null()
        || ((*(*ioService).dispatcher).Dispatch).is_none()
    {
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    return ((*(*ioService).dispatcher).Dispatch)
        .expect(
            "non-null function pointer",
        )(&mut (*ioService).object, cmdId, data, reply) as int32_t;
}
