#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// === C2R_C2RUST_EXTERN_TYPES_BEGIN ===
// Auto-generated: downgraded c2rust `extern type` to stable-safe opaque structs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdfSBuf {
    _unused: [u8; 0],
}

// === C2R_C2RUST_EXTERN_TYPES_END ===

extern "C" {
    pub fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut core::ffi::c_void);
    pub fn HdfSbufWriteUint16(sbuf: *mut HdfSBuf, value: uint16_t) -> bool;
    pub fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    pub fn HdfIoServiceBind(serviceName: *const core::ffi::c_char) -> *mut HdfIoService;
    pub fn HdfIoServiceRecycle(service: *mut HdfIoService);
    pub fn HdfDeviceRegisterEventListener(
        target: *mut HdfIoService,
        listener: *mut HdfDevEventlistener,
    ) -> core::ffi::c_int;
    pub fn HdfDeviceUnregisterEventListener(
        target: *mut HdfIoService,
        listener: *mut HdfDevEventlistener,
    ) -> core::ffi::c_int;
    pub fn HdfIoserviceGetListenerCount(service: *const HdfIoService) -> core::ffi::c_int;
    pub fn HdfSbufObtainDefaultSize() -> *mut HdfSBuf;
    pub fn HdfSbufRecycle(sbuf: *mut HdfSBuf);
}
pub type size_t = core::ffi::c_uint;
pub type int32_t = core::ffi::c_int;
pub type uint16_t = core::ffi::c_ushort;
pub type uint32_t = core::ffi::c_uint;
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
pub type SvcMgrIoCmd = core::ffi::c_uint;
pub const SVCMGR_UNREGISTER_LISTENER: SvcMgrIoCmd = 1;
pub const SVCMGR_REGISTER_LISTENER: SvcMgrIoCmd = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISvcMgrIoservice {
    pub RegisterServiceStatusListener: Option<
        unsafe extern "C" fn(
            *mut ISvcMgrIoservice,
            *mut ServiceStatusListener,
            uint16_t,
        ) -> int32_t,
    >,
    pub UnregisterServiceStatusListener: Option<
        unsafe extern "C" fn(
            *mut ISvcMgrIoservice,
            *mut ServiceStatusListener,
        ) -> int32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvcMgrIoservice {
    pub svcmgr: ISvcMgrIoservice,
    pub iosvc: *mut HdfIoService,
    pub listeners: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
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
pub type OnEventReceived = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        uint32_t,
        *mut HdfSBuf,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IoServiceStatusListener {
    pub svcstatListener: ServiceStatusListener,
    pub ioservListener: HdfDevEventlistener,
    pub node: DListHead,
    pub deviceClass: uint16_t,
}
pub const HDF_ERR_INVALID_OBJECT: C2RustUnnamed = -4;
pub const HDF_ERR_MALLOC_FAIL: C2RustUnnamed = -6;
pub const HDF_SUCCESS: C2RustUnnamed = 0;
pub const HDF_ERR_INVALID_PARAM: C2RustUnnamed = -3;
pub type LogLevel = core::ffi::c_uint;
pub const LOG_FATAL: LogLevel = 7;
pub const LOG_ERROR: LogLevel = 6;
pub const LOG_WARN: LogLevel = 5;
pub const LOG_INFO: LogLevel = 4;
pub const LOG_DEBUG: LogLevel = 3;
pub type LogType = core::ffi::c_uint;
pub const LOG_TYPE_MAX: LogType = 4;
pub const LOG_CORE: LogType = 3;
pub const LOG_INIT: LogType = 1;
pub const LOG_TYPE_MIN: LogType = 0;
pub const DEVICE_CLASS_MAX: C2RustUnnamed_0 = 1024;
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
pub const HDF_ERR_NOT_SUPPORT: C2RustUnnamed = -2;
pub const HDF_FAILURE: C2RustUnnamed = -1;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const DEVICE_CLASS_HIMEDIACOMM: C2RustUnnamed_0 = 512;
pub const DEVICE_CLASS_USERAUTH: C2RustUnnamed_0 = 256;
pub const DEVICE_CLASS_USB: C2RustUnnamed_0 = 128;
pub const DEVICE_CLASS_CAMERA: C2RustUnnamed_0 = 64;
pub const DEVICE_CLASS_AUDIO: C2RustUnnamed_0 = 32;
pub const DEVICE_CLASS_DISPLAY: C2RustUnnamed_0 = 16;
pub const DEVICE_CLASS_INPUT: C2RustUnnamed_0 = 8;
pub const DEVICE_CLASS_SENSOR: C2RustUnnamed_0 = 4;
pub const DEVICE_CLASS_PLAT: C2RustUnnamed_0 = 2;
pub const DEVICE_CLASS_DEFAULT: C2RustUnnamed_0 = 1;
pub unsafe extern "C" fn ProcessListenClass(
    mut svcmgrInst: *mut SvcMgrIoservice,
    mut devClass: uint16_t,
    mut cmdId: core::ffi::c_int,
) -> int32_t {
    let mut data: *mut HdfSBuf = HdfSbufObtainDefaultSize();
    if data.is_null() {
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    HdfSbufWriteUint16(data, devClass);
    if ((*svcmgrInst).iosvc).is_null() || ((*(*svcmgrInst).iosvc).dispatcher).is_null()
        || ((*(*(*svcmgrInst).iosvc).dispatcher).Dispatch).is_none()
    {
        HdfSbufRecycle(data);
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    let mut ret: int32_t = ((*(*(*svcmgrInst).iosvc).dispatcher).Dispatch)
        .expect(
            "non-null function pointer",
        )((*svcmgrInst).iosvc as *mut HdfObject, cmdId, data, 0 as *mut HdfSBuf)
        as int32_t;
    HdfSbufRecycle(data);
    return ret;
}
pub unsafe extern "C" fn SetListenClass(
    mut svcmgrInst: *mut SvcMgrIoservice,
    mut devClass: uint16_t,
) -> int32_t {
    return ProcessListenClass(
        svcmgrInst,
        devClass,
        SVCMGR_REGISTER_LISTENER as core::ffi::c_int,
    );
}
pub unsafe extern "C" fn UnSetListenClass(
    mut svcmgrInst: *mut SvcMgrIoservice,
    mut devClass: uint16_t,
) -> int32_t {
    return ProcessListenClass(
        svcmgrInst,
        devClass,
        SVCMGR_UNREGISTER_LISTENER as core::ffi::c_int,
    );
}
pub unsafe extern "C" fn SvcMgrIoserviceRegSvcStatListener(
    mut self_0: *mut ISvcMgrIoservice,
    mut listener: *mut ServiceStatusListener,
    mut deviceClass: uint16_t,
) -> int32_t {
    if self_0.is_null() || listener.is_null()
        || deviceClass as core::ffi::c_int >= DEVICE_CLASS_MAX as core::ffi::c_int
    {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    let mut svcmgrInst: *mut SvcMgrIoservice = (self_0 as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut SvcMgrIoservice)).svcmgr as *mut ISvcMgrIoservice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut SvcMgrIoservice;
    let mut listenerInst: *mut IoServiceStatusListener = (listener
        as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut IoServiceStatusListener)).svcstatListener
                as *mut ServiceStatusListener as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut IoServiceStatusListener;
    (*listenerInst).deviceClass = deviceClass;
    let mut ret: core::ffi::c_int = SetListenClass(svcmgrInst, deviceClass)
        as core::ffi::c_int;
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"HDF_LOG_TAG\0" as *const u8 as *const core::ffi::c_char,
            b"failed to set listen class\0" as *const u8 as *const core::ffi::c_char,
        );
        return ret as int32_t;
    }
    return HdfDeviceRegisterEventListener(
        (*svcmgrInst).iosvc,
        &mut (*listenerInst).ioservListener,
    ) as int32_t;
}
pub unsafe extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(
    mut self_0: *mut ISvcMgrIoservice,
    mut listener: *mut ServiceStatusListener,
) -> int32_t {
    if self_0.is_null() || listener.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    let mut svcmgrInst: *mut SvcMgrIoservice = (self_0 as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut SvcMgrIoservice)).svcmgr as *mut ISvcMgrIoservice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut SvcMgrIoservice;
    let mut listenerInst: *mut IoServiceStatusListener = (listener
        as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut IoServiceStatusListener)).svcstatListener
                as *mut ServiceStatusListener as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut IoServiceStatusListener;
    let mut ret: core::ffi::c_int = HdfDeviceUnregisterEventListener(
        (*svcmgrInst).iosvc,
        &mut (*listenerInst).ioservListener,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        return ret as int32_t;
    }
    if HdfIoserviceGetListenerCount((*svcmgrInst).iosvc) == 0 as core::ffi::c_int {
        ret = UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass)
            as core::ffi::c_int;
    }
    return ret as int32_t;
}
pub unsafe extern "C" fn SvcMgrIoserviceConstruct(mut svcmgrInst: *mut ISvcMgrIoservice) {
    (*svcmgrInst).RegisterServiceStatusListener = Some(
        SvcMgrIoserviceRegSvcStatListener
            as unsafe extern "C" fn(
                *mut ISvcMgrIoservice,
                *mut ServiceStatusListener,
                uint16_t,
            ) -> int32_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ISvcMgrIoservice,
                *mut ServiceStatusListener,
                uint16_t,
            ) -> int32_t,
        >;
    (*svcmgrInst).UnregisterServiceStatusListener = Some(
        SvcMgrIoserviceUnRegSvcStatListener
            as unsafe extern "C" fn(
                *mut ISvcMgrIoservice,
                *mut ServiceStatusListener,
            ) -> int32_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ISvcMgrIoservice,
                *mut ServiceStatusListener,
            ) -> int32_t,
        >;
}
pub unsafe extern "C" fn SvcMgrIoserviceGet() -> *mut ISvcMgrIoservice {
    let mut svcmgrInst: *mut SvcMgrIoservice = OsalMemCalloc(
        ::core::mem::size_of::<SvcMgrIoservice>() as size_t,
    ) as *mut SvcMgrIoservice;
    if svcmgrInst.is_null() {
        return 0 as *mut ISvcMgrIoservice;
    }
    (*svcmgrInst).iosvc = HdfIoServiceBind(
        b"devsvc_mgr\0" as *const u8 as *const core::ffi::c_char,
    );
    if ((*svcmgrInst).iosvc).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"HDF_LOG_TAG\0" as *const u8 as *const core::ffi::c_char,
            b"ioserivce %{public}s not exist\0" as *const u8 as *const core::ffi::c_char,
            b"devsvc_mgr\0" as *const u8 as *const core::ffi::c_char,
        );
        OsalMemFree(svcmgrInst as *mut core::ffi::c_void);
        return 0 as *mut ISvcMgrIoservice;
    }
    SvcMgrIoserviceConstruct(&mut (*svcmgrInst).svcmgr);
    return &mut (*svcmgrInst).svcmgr;
}
pub unsafe extern "C" fn SvcMgrIoserviceRelease(mut svcmgr: *mut ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    let mut svcmgrInst: *mut SvcMgrIoservice = (svcmgr as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut SvcMgrIoservice)).svcmgr as *mut ISvcMgrIoservice
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut SvcMgrIoservice;
    HdfIoServiceRecycle((*svcmgrInst).iosvc);
    OsalMemFree(svcmgrInst as *mut core::ffi::c_void);
}
