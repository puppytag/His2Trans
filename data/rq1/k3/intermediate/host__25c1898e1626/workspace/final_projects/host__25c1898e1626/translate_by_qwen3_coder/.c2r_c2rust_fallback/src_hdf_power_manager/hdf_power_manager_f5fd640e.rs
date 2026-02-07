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
    pub type DeviceResourceNode;
    fn DevMgrPmRegister() -> core::ffi::c_int;
    fn HdfTaskEnqueue(queue: *mut HdfTaskQueue, task: *mut HdfTaskType);
    fn HdfTaskQueueCreate(
        func: HdfTaskFunc,
        name: *const core::ffi::c_char,
    ) -> *mut HdfTaskQueue;
    fn HdfTaskQueueDestroy(queue: *mut HdfTaskQueue);
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn OsalMemFree(mem: *mut core::ffi::c_void);
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type size_t = core::ffi::c_uint;
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
pub struct DListHead {
    pub next: *mut DListHead,
    pub prev: *mut DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalSem {
    pub realSemaphore: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalThread {
    pub realThread: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfTaskType {
    pub node: DListHead,
    pub func: HdfTaskFunc,
}
pub type HdfTaskFunc = Option<unsafe extern "C" fn(*mut HdfTaskType) -> int32_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfTaskQueue {
    pub sem: OsalSem,
    pub mutex: OsalMutex,
    pub head: DListHead,
    pub thread: OsalThread,
    pub threadRunFlag: bool,
    pub queueFunc: HdfTaskFunc,
    pub queueName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalAtomic {
    pub counter: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSRef {
    pub refs: OsalAtomic,
    pub listener: *mut IHdfSRefListener,
    pub Acquire: Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>,
    pub Release: Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>,
    pub Count: Option<unsafe extern "C" fn(*const HdfSRef) -> core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IHdfSRefListener {
    pub OnFirstAcquire: Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>,
    pub OnLastRelease: Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>,
}
pub type HdfPsmState = core::ffi::c_uint;
pub const PSM_STATE_INACTIVE: HdfPsmState = 2;
pub const PSM_STATE_ACTIVE: HdfPsmState = 1;
pub const PSM_STATE_IDLE: HdfPsmState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPowerStateToken {
    pub AcquireWakeLock: Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>,
    pub ReleaseWakeLock: Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>,
}
pub type DeviceClass = core::ffi::c_uint;
pub const DEVICE_CLASS_MAX: DeviceClass = 1024;
pub const DEVICE_CLASS_HIMEDIACOMM: DeviceClass = 512;
pub const DEVICE_CLASS_USERAUTH: DeviceClass = 256;
pub const DEVICE_CLASS_USB: DeviceClass = 128;
pub const DEVICE_CLASS_CAMERA: DeviceClass = 64;
pub const DEVICE_CLASS_AUDIO: DeviceClass = 32;
pub const DEVICE_CLASS_DISPLAY: DeviceClass = 16;
pub const DEVICE_CLASS_INPUT: DeviceClass = 8;
pub const DEVICE_CLASS_SENSOR: DeviceClass = 4;
pub const DEVICE_CLASS_PLAT: DeviceClass = 2;
pub const DEVICE_CLASS_DEFAULT: DeviceClass = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfObject {
    pub objectId: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceObject {
    pub service: *mut IDeviceIoService,
    pub property: *const DeviceResourceNode,
    pub deviceClass: DeviceClass,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDeviceIoService {
    pub object: HdfObject,
    pub Open: Option<unsafe extern "C" fn(*mut HdfDeviceIoClient) -> int32_t>,
    pub Dispatch: Option<
        unsafe extern "C" fn(
            *mut HdfDeviceIoClient,
            core::ffi::c_int,
            *mut HdfSBuf,
            *mut HdfSBuf,
        ) -> int32_t,
    >,
    pub Release: Option<unsafe extern "C" fn(*mut HdfDeviceIoClient) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceIoClient {
    pub device: *mut HdfDeviceObject,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPowerEventListener {
    pub DozeResume: Option<
        unsafe extern "C" fn(*mut HdfDeviceObject) -> core::ffi::c_int,
    >,
    pub DozeSuspend: Option<
        unsafe extern "C" fn(*mut HdfDeviceObject) -> core::ffi::c_int,
    >,
    pub Resume: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> core::ffi::c_int>,
    pub Suspend: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PowerStateToken {
    pub super_0: IPowerStateToken,
    pub listener: *const IPowerEventListener,
    pub deviceObject: *mut HdfDeviceObject,
    pub wakeRef: HdfSRef,
    pub psmState: HdfPsmState,
    pub mode: uint32_t,
}
pub type HDF_PM_REQUEST_TYPE = core::ffi::c_uint;
pub const HDF_PM_REQUEST_RELEASE: HDF_PM_REQUEST_TYPE = 1;
pub const HDF_PM_REQUEST_ACQUIRE: HDF_PM_REQUEST_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfPmRequest {
    pub token: *mut PowerStateToken,
    pub pmType: HDF_PM_REQUEST_TYPE,
    pub task: HdfTaskType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmTaskQueue {
    pub taskQueue: *mut HdfTaskQueue,
}
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
unsafe extern "C" fn HdfPmTaskQueueInstance() -> *mut PmTaskQueue {
    static mut pmTaskQueue: PmTaskQueue = {
        let mut init = PmTaskQueue {
            taskQueue: 0 as *const HdfTaskQueue as *mut HdfTaskQueue,
        };
        init
    };
    return &mut pmTaskQueue;
}
#[no_mangle]
pub unsafe extern "C" fn HdfPmTaskQueueInit(mut func: HdfTaskFunc) -> *mut PmTaskQueue {
    let mut pmTaskQueue: *mut PmTaskQueue = HdfPmTaskQueueInstance();
    if ((*pmTaskQueue).taskQueue).is_null() {
        (*pmTaskQueue).taskQueue = HdfTaskQueueCreate(
            func,
            b"pm_queue\0" as *const u8 as *const core::ffi::c_char,
        );
        if !((*pmTaskQueue).taskQueue).is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xd002510 as core::ffi::c_uint,
                b"hdf_power_manager\0" as *const u8 as *const core::ffi::c_char,
                b"%{public}s HdfTaskQueueCreate success\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"HdfPmTaskQueueInit\0"))
                    .as_ptr(),
            );
        }
    }
    return pmTaskQueue as *mut PmTaskQueue;
}
#[no_mangle]
pub unsafe extern "C" fn HdfPmTaskQueueDestroy() {
    let mut pmTaskQueue: *mut PmTaskQueue = HdfPmTaskQueueInstance();
    HdfTaskQueueDestroy((*pmTaskQueue).taskQueue);
    (*pmTaskQueue).taskQueue = 0 as *mut HdfTaskQueue;
}
unsafe extern "C" fn PmTaskFunc(mut para: *mut HdfTaskType) -> int32_t {
    let mut pmRequest: *mut HdfPmRequest = 0 as *mut HdfPmRequest;
    let mut tokenIf: *mut IPowerStateToken = 0 as *mut IPowerStateToken;
    if para.is_null() {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    pmRequest = (para as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfPmRequest)).task as *mut HdfTaskType
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfPmRequest;
    tokenIf = (*pmRequest).token as *mut IPowerStateToken;
    if (*pmRequest).pmType as core::ffi::c_uint
        == HDF_PM_REQUEST_ACQUIRE as core::ffi::c_int as core::ffi::c_uint
    {
        if !tokenIf.is_null() && ((*tokenIf).AcquireWakeLock).is_some() {
            ((*tokenIf).AcquireWakeLock).expect("non-null function pointer")(tokenIf);
        }
    } else if (*pmRequest).pmType as core::ffi::c_uint
        == HDF_PM_REQUEST_RELEASE as core::ffi::c_int as core::ffi::c_uint
    {
        if !tokenIf.is_null() && ((*tokenIf).ReleaseWakeLock).is_some() {
            ((*tokenIf).ReleaseWakeLock).expect("non-null function pointer")(tokenIf);
        }
    }
    OsalMemFree(pmRequest as *mut core::ffi::c_void);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn HdfPmTaskPut(
    mut powerToken: *mut PowerStateToken,
    mut type_0: HDF_PM_REQUEST_TYPE,
) {
    let mut pmRequest: *mut HdfPmRequest = 0 as *mut HdfPmRequest;
    let mut pmTaskQueue: *mut PmTaskQueue = 0 as *mut PmTaskQueue;
    if powerToken.is_null() {
        return;
    }
    pmTaskQueue = HdfPmTaskQueueInstance();
    pmRequest = OsalMemCalloc(::core::mem::size_of::<HdfPmRequest>() as size_t)
        as *mut HdfPmRequest;
    if pmRequest.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd002510 as core::ffi::c_uint,
            b"hdf_power_manager\0" as *const u8 as *const core::ffi::c_char,
            b"%{public}s OsalMemCalloc fail\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"HdfPmTaskPut\0"))
                .as_ptr(),
        );
        return;
    }
    (*pmRequest).token = powerToken;
    (*pmRequest).pmType = type_0;
    (*pmRequest).task.func = Some(
        PmTaskFunc as unsafe extern "C" fn(*mut HdfTaskType) -> int32_t,
    ) as HdfTaskFunc;
    HdfTaskEnqueue((*pmTaskQueue).taskQueue, &mut (*pmRequest).task);
}
#[no_mangle]
pub unsafe extern "C" fn HdfPowerManagerInit() -> int32_t {
    DevMgrPmRegister();
    HdfPmTaskQueueInit(None);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn HdfPowerManagerExit() {
    HdfPmTaskQueueDestroy();
}
