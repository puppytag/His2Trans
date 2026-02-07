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
    fn HdfSRefCount(sref: *const HdfSRef) -> core::ffi::c_int;
    fn HdfSRefConstruct(sref: *mut HdfSRef, listener: *mut IHdfSRefListener);
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn OsalMemFree(mem: *mut core::ffi::c_void);
}
pub type size_t = core::ffi::c_uint;
pub type int32_t = core::ffi::c_int;
pub type uint32_t = core::ffi::c_uint;
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
pub type PowerManagementMode = core::ffi::c_uint;
pub const HDF_POWER_MODE_MAX: PowerManagementMode = 2;
pub const HDF_POWER_DYNAMIC_CTRL: PowerManagementMode = 1;
pub const HDF_POWER_SYS_CTRL: PowerManagementMode = 0;
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
pub const POWER_STATE_DOZE_RESUME: HdfPowerState = 0;
pub const POWER_STATE_DOZE_SUSPEND: HdfPowerState = 1;
pub const POWER_STATE_RESUME: HdfPowerState = 2;
pub const POWER_STATE_SUSPEND: HdfPowerState = 3;
pub type HdfPowerState = core::ffi::c_uint;
pub const POWER_STATE_MAX: HdfPowerState = 4;
unsafe extern "C" fn PowerStateTokenOnFirstAcquire(mut sref: *mut HdfSRef) {
    let mut stateToken: *mut PowerStateToken = 0 as *mut PowerStateToken;
    if sref.is_null() {
        return;
    }
    stateToken = ({
        let mut __mptr: *const HdfSRef = sref;
        (__mptr as *mut core::ffi::c_char)
            .offset(
                -(&mut (*(0 as *mut PowerStateToken)).wakeRef as *mut HdfSRef as size_t
                    as isize),
            ) as *mut PowerStateToken
    });
    if (*stateToken).psmState as core::ffi::c_uint
        == PSM_STATE_ACTIVE as core::ffi::c_int as core::ffi::c_uint
    {
        return;
    }
    if (*stateToken).psmState as core::ffi::c_uint
        == PSM_STATE_INACTIVE as core::ffi::c_int as core::ffi::c_uint
        || (*stateToken).psmState as core::ffi::c_uint
            == PSM_STATE_IDLE as core::ffi::c_int as core::ffi::c_uint
    {
        let mut listener: *const IPowerEventListener = (*stateToken).listener;
        if !listener.is_null() && ((*listener).Resume).is_some() {
            ((*listener).Resume)
                .expect("non-null function pointer")((*stateToken).deviceObject);
        }
    }
    (*stateToken).psmState = PSM_STATE_ACTIVE;
}
unsafe extern "C" fn PowerStateTokenOnLastRelease(mut sref: *mut HdfSRef) {
    let mut stateToken: *mut PowerStateToken = 0 as *mut PowerStateToken;
    let mut listener: *const IPowerEventListener = 0 as *const IPowerEventListener;
    if sref.is_null() {
        return;
    }
    stateToken = ({
        let mut __mptr: *const HdfSRef = sref;
        (__mptr as *mut core::ffi::c_char)
            .offset(
                -(&mut (*(0 as *mut PowerStateToken)).wakeRef as *mut HdfSRef as size_t
                    as isize),
            ) as *mut PowerStateToken
    });
    if (*stateToken).psmState as core::ffi::c_uint
        != PSM_STATE_ACTIVE as core::ffi::c_int as core::ffi::c_uint
        && (*stateToken).psmState as core::ffi::c_uint
            != PSM_STATE_IDLE as core::ffi::c_int as core::ffi::c_uint
    {
        return;
    }
    listener = (*stateToken).listener;
    if !listener.is_null() && ((*listener).Suspend).is_some() {
        ((*listener).Suspend)
            .expect("non-null function pointer")((*stateToken).deviceObject);
    }
    (*stateToken).psmState = PSM_STATE_INACTIVE;
}
#[no_mangle]
pub unsafe extern "C" fn PowerStateChange(
    mut stateToken: *mut PowerStateToken,
    mut pEvent: uint32_t,
) -> core::ffi::c_int {
    if stateToken.is_null() || ((*stateToken).listener).is_null()
        || (*stateToken).mode
            != HDF_POWER_SYS_CTRL as core::ffi::c_int as core::ffi::c_uint
    {
        return HDF_SUCCESS as core::ffi::c_int;
    }
    match pEvent {
        3 => {
            if ((*(*stateToken).listener).Suspend).is_some() {
                return ((*(*stateToken).listener).Suspend)
                    .expect("non-null function pointer")((*stateToken).deviceObject);
            }
        }
        2 => {
            if ((*(*stateToken).listener).Resume).is_some() {
                return ((*(*stateToken).listener).Resume)
                    .expect("non-null function pointer")((*stateToken).deviceObject);
            }
        }
        1 => {
            if ((*(*stateToken).listener).DozeSuspend).is_some() {
                return ((*(*stateToken).listener).DozeSuspend)
                    .expect("non-null function pointer")((*stateToken).deviceObject);
            }
        }
        0 => {
            if ((*(*stateToken).listener).DozeResume).is_some() {
                return ((*(*stateToken).listener).DozeResume)
                    .expect("non-null function pointer")((*stateToken).deviceObject);
            }
        }
        _ => {}
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
unsafe extern "C" fn PowerStateTokenAcquireWakeLock(mut token: *mut IPowerStateToken) {
    let mut sref: *mut HdfSRef = 0 as *mut HdfSRef;
    let mut stateToken: *mut PowerStateToken = token as *mut PowerStateToken;
    if stateToken.is_null()
        || (*stateToken).mode
            != HDF_POWER_DYNAMIC_CTRL as core::ffi::c_int as core::ffi::c_uint
    {
        return;
    }
    sref = &mut (*stateToken).wakeRef as *mut HdfSRef;
    if !sref.is_null() && ((*sref).Acquire).is_some() {
        ((*sref).Acquire).expect("non-null function pointer")(sref);
    }
}
unsafe extern "C" fn PowerStateTokenReleaseWakeLock(mut token: *mut IPowerStateToken) {
    let mut sref: *mut HdfSRef = 0 as *mut HdfSRef;
    let mut stateToken: *mut PowerStateToken = token as *mut PowerStateToken;
    if stateToken.is_null()
        || (*stateToken).mode
            != HDF_POWER_DYNAMIC_CTRL as core::ffi::c_int as core::ffi::c_uint
    {
        return;
    }
    sref = &mut (*stateToken).wakeRef as *mut HdfSRef;
    if sref.is_null() || ((*sref).Release).is_none() {
        return;
    }
    if HdfSRefCount(sref) == 0 as core::ffi::c_int {
        PowerStateTokenOnLastRelease(sref);
    } else {
        ((*sref).Release).expect("non-null function pointer")(sref);
    };
}
unsafe extern "C" fn PowerStateTokenConstruct(
    mut powerStateToken: *mut PowerStateToken,
    mut deviceObject: *mut HdfDeviceObject,
    mut listener: *const IPowerEventListener,
) -> int32_t {
    let mut tokenIf: *mut IPowerStateToken = &mut (*powerStateToken).super_0;
    let mut srefListener: *mut IHdfSRefListener = OsalMemCalloc(
        ::core::mem::size_of::<IHdfSRefListener>() as size_t,
    ) as *mut IHdfSRefListener;
    if srefListener.is_null() {
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*tokenIf).AcquireWakeLock = Some(
        PowerStateTokenAcquireWakeLock
            as unsafe extern "C" fn(*mut IPowerStateToken) -> (),
    ) as Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>;
    (*tokenIf).ReleaseWakeLock = Some(
        PowerStateTokenReleaseWakeLock
            as unsafe extern "C" fn(*mut IPowerStateToken) -> (),
    ) as Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>;
    (*srefListener).OnFirstAcquire = Some(
        PowerStateTokenOnFirstAcquire as unsafe extern "C" fn(*mut HdfSRef) -> (),
    ) as Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>;
    (*srefListener).OnLastRelease = Some(
        PowerStateTokenOnLastRelease as unsafe extern "C" fn(*mut HdfSRef) -> (),
    ) as Option<unsafe extern "C" fn(*mut HdfSRef) -> ()>;
    (*powerStateToken).psmState = PSM_STATE_IDLE;
    (*powerStateToken).listener = listener;
    (*powerStateToken).deviceObject = deviceObject;
    HdfSRefConstruct(&mut (*powerStateToken).wakeRef, srefListener);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn PowerStateTokenNewInstance(
    mut deviceObject: *mut HdfDeviceObject,
    mut listener: *const IPowerEventListener,
) -> *mut PowerStateToken {
    let mut stateToken: *mut PowerStateToken = OsalMemCalloc(
        ::core::mem::size_of::<PowerStateToken>() as size_t,
    ) as *mut PowerStateToken;
    if stateToken.is_null() {
        return 0 as *mut PowerStateToken;
    }
    if PowerStateTokenConstruct(stateToken, deviceObject, listener)
        != HDF_SUCCESS as core::ffi::c_int
    {
        OsalMemFree(stateToken as *mut core::ffi::c_void);
        return 0 as *mut PowerStateToken;
    }
    return stateToken;
}
#[no_mangle]
pub unsafe extern "C" fn PowerStateTokenFreeInstance(
    mut stateToken: *mut PowerStateToken,
) {
    if !stateToken.is_null() {
        if !((*stateToken).wakeRef.listener).is_null() {
            OsalMemFree((*stateToken).wakeRef.listener as *mut core::ffi::c_void);
            (*stateToken).wakeRef.listener = 0 as *mut IHdfSRefListener;
        }
        OsalMemFree(stateToken as *mut core::ffi::c_void);
    }
}
