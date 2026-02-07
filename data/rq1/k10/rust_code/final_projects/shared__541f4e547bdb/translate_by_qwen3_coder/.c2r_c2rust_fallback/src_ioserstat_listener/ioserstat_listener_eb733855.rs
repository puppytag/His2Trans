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
    fn ServiceStatusUnMarshalling(
        status: *mut ServiceStatus,
        buf: *mut HdfSBuf,
    ) -> core::ffi::c_int;
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn OsalMemFree(mem: *mut core::ffi::c_void);
}
pub type size_t = core::ffi::c_uint;
pub type int32_t = core::ffi::c_int;
pub type uint16_t = core::ffi::c_ushort;
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
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDevEventlistener {
    pub callBack: OnEventReceived,
    pub onReceive: OnDevEventReceive,
    pub listNode: DListHead,
    pub priv_0: *mut core::ffi::c_void,
}
pub type OnDevEventReceive = Option<
    unsafe extern "C" fn(
        *mut HdfDevEventlistener,
        *mut HdfIoService,
        uint32_t,
        *mut HdfSBuf,
    ) -> core::ffi::c_int,
>;
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
pub type OnEventReceived = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        uint32_t,
        *mut HdfSBuf,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ServiceStatusListener {
    pub callback: OnServiceStatusReceived,
    pub priv_0: *mut core::ffi::c_void,
}
pub type OnServiceStatusReceived = Option<
    unsafe extern "C" fn(*mut ServiceStatusListener, *mut ServiceStatus) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ServiceStatus {
    pub serviceName: *const core::ffi::c_char,
    pub deviceClass: uint16_t,
    pub status: uint16_t,
    pub info: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IoServiceStatusListener {
    pub svcstatListener: ServiceStatusListener,
    pub ioservListener: HdfDevEventlistener,
    pub node: DListHead,
    pub deviceClass: uint16_t,
}
unsafe extern "C" fn OnIoServiceEventReceive(
    mut listener: *mut HdfDevEventlistener,
    mut service: *mut HdfIoService,
    mut id: uint32_t,
    mut data: *mut HdfSBuf,
) -> core::ffi::c_int {
    if listener.is_null() || service.is_null() || data.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    let mut status: ServiceStatus = {
        let mut init = ServiceStatus {
            serviceName: 0 as *const core::ffi::c_char,
            deviceClass: 0,
            status: 0,
            info: 0 as *const core::ffi::c_char,
        };
        init
    };
    if ServiceStatusUnMarshalling(&mut status, data) != HDF_SUCCESS as core::ffi::c_int {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    let mut statusListener: *mut IoServiceStatusListener = (*listener).priv_0
        as *mut IoServiceStatusListener;
    if ((*statusListener).svcstatListener.callback).is_some()
        && (*statusListener).deviceClass as core::ffi::c_int
            & status.deviceClass as core::ffi::c_int != 0
    {
        ((*statusListener).svcstatListener.callback)
            .expect(
                "non-null function pointer",
            )(&mut (*statusListener).svcstatListener, &mut status);
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn IoServiceStatusListenerNewInstance() -> *mut ServiceStatusListener {
    let mut listener: *mut IoServiceStatusListener = OsalMemCalloc(
        ::core::mem::size_of::<IoServiceStatusListener>() as size_t,
    ) as *mut IoServiceStatusListener;
    if listener.is_null() {
        return 0 as *mut ServiceStatusListener;
    }
    (*listener).ioservListener.onReceive = Some(
        OnIoServiceEventReceive
            as unsafe extern "C" fn(
                *mut HdfDevEventlistener,
                *mut HdfIoService,
                uint32_t,
                *mut HdfSBuf,
            ) -> core::ffi::c_int,
    ) as OnDevEventReceive;
    (*listener).ioservListener.priv_0 = listener as *mut core::ffi::c_void;
    return &mut (*listener).svcstatListener;
}
#[no_mangle]
pub unsafe extern "C" fn IoServiceStatusListenerFree(
    mut listener: *mut ServiceStatusListener,
) {
    if listener.is_null() {
        return;
    }
    let mut ioservListener: *mut IoServiceStatusListener = (listener
        as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut IoServiceStatusListener)).svcstatListener
                as *mut ServiceStatusListener as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut IoServiceStatusListener;
    OsalMemFree(ioservListener as *mut core::ffi::c_void);
}
