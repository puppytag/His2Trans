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
    pub fn HdfGetHcsRootNode() -> *const DeviceResourceNode;
    pub fn HdfDeviceObjectConstruct(deviceObject: *mut HdfDeviceObject);
    pub fn HdfDeviceTokenNewInstance() -> *mut IHdfDeviceToken;
    pub fn DevSvcManagerClntAddService(
        service: *mut HdfDeviceObject,
        servinfo: *const HdfServiceInfo,
    ) -> core::ffi::c_int;
    pub fn HdfServiceObserverPublishService(
        observer: *mut HdfServiceObserver,
        svcName: *const core::ffi::c_char,
        deviceId: devid_t,
        policy: uint16_t,
        service: *mut HdfObject,
    ) -> core::ffi::c_int;
    pub fn HdfDeviceTokenFreeInstance(token: *mut IHdfDeviceToken);
    pub fn PowerStateTokenNewInstance(
        deviceObject: *mut HdfDeviceObject,
        listener: *const IPowerEventListener,
    ) -> *mut PowerStateToken;
    pub fn HdfStringCopy(src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn DevmgrServiceClntAttachDevice(
        deviceToken: *mut IHdfDeviceToken,
    ) -> core::ffi::c_int;
    pub fn DevSvcManagerClntRemoveService(svcName: *const core::ffi::c_char);
    pub fn DevmgrServiceClntDetachDevice(devid: devid_t) -> core::ffi::c_int;
    pub fn PowerStateTokenFreeInstance(stateToken: *mut PowerStateToken);
    pub fn HdfDriverLoaderGetInstance() -> *mut IDriverLoader;
    pub fn HdfObjectManagerGetObject(objectId: core::ffi::c_int) -> *mut HdfObject;
    pub fn HdfObjectManagerFreeObject(object: *mut HdfObject);
    pub fn HcsGetNodeByMatchAttr(
        node: *const DeviceResourceNode,
        attrValue: *const core::ffi::c_char,
    ) -> *const DeviceResourceNode;
    pub fn strlen(_: *const core::ffi::c_char) -> size_t;
    pub fn OsalMemFree(mem: *mut core::ffi::c_void);
    pub fn HiLogPrint(
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
pub type uint8_t = core::ffi::c_uchar;
pub type uint16_t = core::ffi::c_ushort;
pub type uint32_t = core::ffi::c_uint;
pub type uint64_t = core::ffi::c_ulonglong;
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
pub type devid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceNode {
    pub super_0: IDeviceNode,
    pub entry: DListHead,
    pub powerToken: *mut PowerStateToken,
    pub hostService: *mut DevHostService,
    pub deviceObject: HdfDeviceObject,
    pub token: *mut IHdfDeviceToken,
    pub driver: *mut HdfDriver,
    pub device: *mut HdfDevice,
    pub servName: *mut core::ffi::c_char,
    pub servInfo: *const core::ffi::c_char,
    pub driverName: *mut core::ffi::c_char,
    pub devId: devid_t,
    pub policy: uint16_t,
    pub permission: uint16_t,
    pub devStatus: uint8_t,
    pub servStatus: bool,
    pub interfaceDesc: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDevice {
    pub super_0: IHdfDevice,
    pub node: DListHead,
    pub devNodes: DListHead,
    pub deviceId: devid_t,
    pub devidIndex: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IHdfDevice {
    pub object: HdfObject,
    pub Attach: Option<
        unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub Detach: Option<
        unsafe extern "C" fn(*mut IHdfDevice, *mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub GetDeviceNode: Option<
        unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> *mut HdfDeviceNode,
    >,
    pub DetachWithDevid: Option<
        unsafe extern "C" fn(*mut IHdfDevice, devid_t) -> core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriver {
    pub entry: *const HdfDriverEntry,
    pub type_0: uint16_t,
    pub bus: uint16_t,
    pub node: DListHead,
    pub priv_0: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDriverEntry {
    pub moduleVersion: int32_t,
    pub moduleName: *const core::ffi::c_char,
    pub Bind: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Init: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> int32_t>,
    pub Release: Option<unsafe extern "C" fn(*mut HdfDeviceObject) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceObject {
    pub service: *mut IDeviceIoService,
    pub property: *const DeviceResourceNode,
    pub deviceClass: DeviceClass,
    pub priv_0: *mut core::ffi::c_void,
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
pub struct DeviceResourceNode {
    pub name: *const core::ffi::c_char,
    pub hashValue: uint32_t,
    pub attrData: *mut DeviceResourceAttr,
    pub parent: *mut DeviceResourceNode,
    pub child: *mut DeviceResourceNode,
    pub sibling: *mut DeviceResourceNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceResourceAttr {
    pub name: *const core::ffi::c_char,
    pub value: *const core::ffi::c_char,
    pub next: *mut DeviceResourceAttr,
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
pub struct IHdfDeviceToken {
    pub object: HdfObject,
    pub devid: devid_t,
    pub servName: *const core::ffi::c_char,
    pub deviceName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DevHostService {
    pub super_0: IDevHostService,
    pub hostId: uint16_t,
    pub hostName: *const core::ffi::c_char,
    pub devices: DListHead,
    pub observer: HdfServiceObserver,
    pub sysEventNotifyNode: HdfSysEventNotifyNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSysEventNotifyNode {
    pub callback: HdfSysEventNotifierFn,
    pub listNode: DListHead,
    pub classFilter: uint64_t,
}
pub type HdfSysEventNotifierFn = Option<
    unsafe extern "C" fn(
        *mut HdfSysEventNotifyNode,
        uint64_t,
        uint32_t,
        *const core::ffi::c_char,
    ) -> int32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfServiceObserver {
    pub services: HdfSList,
    pub observerMutex: OsalMutex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSList {
    pub root: *mut HdfSListNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfSListNode {
    pub next: *mut HdfSListNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDevHostService {
    pub object: HdfObject,
    pub AddDevice: Option<
        unsafe extern "C" fn(
            *mut IDevHostService,
            *const HdfDeviceInfo,
        ) -> core::ffi::c_int,
    >,
    pub DelDevice: Option<
        unsafe extern "C" fn(*mut IDevHostService, devid_t) -> core::ffi::c_int,
    >,
    pub StartService: Option<
        unsafe extern "C" fn(*mut IDevHostService) -> core::ffi::c_int,
    >,
    pub PmNotify: Option<
        unsafe extern "C" fn(*mut IDevHostService, uint32_t) -> core::ffi::c_int,
    >,
    pub Dump: Option<
        unsafe extern "C" fn(
            *mut IDevHostService,
            *mut HdfSBuf,
            *mut HdfSBuf,
        ) -> core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfDeviceInfo {
    pub node: HdfSListNode,
    pub isDynamic: bool,
    pub status: uint16_t,
    pub deviceType: uint16_t,
    pub deviceId: uint32_t,
    pub policy: uint16_t,
    pub priority: uint16_t,
    pub preload: uint16_t,
    pub permission: uint16_t,
    pub moduleName: *const core::ffi::c_char,
    pub svcName: *const core::ffi::c_char,
    pub deviceMatchAttr: *const core::ffi::c_char,
    pub deviceName: *const core::ffi::c_char,
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
pub type HdfPsmState = core::ffi::c_uint;
pub const PSM_STATE_INACTIVE: HdfPsmState = 2;
pub const PSM_STATE_ACTIVE: HdfPsmState = 1;
pub const PSM_STATE_IDLE: HdfPsmState = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalAtomic {
    pub counter: int32_t,
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
pub struct IPowerStateToken {
    pub AcquireWakeLock: Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>,
    pub ReleaseWakeLock: Option<unsafe extern "C" fn(*mut IPowerStateToken) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDeviceNode {
    pub object: HdfObject,
    pub PublishService: Option<
        unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub RemoveService: Option<
        unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
    >,
    pub LaunchNode: Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int>,
    pub UnlaunchNode: Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> ()>,
}
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const SERVICE_POLICY_INVALID: C2RustUnnamed_0 = 5;
pub const SERVICE_POLICY_PRIVATE: C2RustUnnamed_0 = 4;
pub const SERVICE_POLICY_FRIENDLY: C2RustUnnamed_0 = 3;
pub const SERVICE_POLICY_CAPACITY: C2RustUnnamed_0 = 2;
pub const SERVICE_POLICY_PUBLIC: C2RustUnnamed_0 = 1;
pub const SERVICE_POLICY_NONE: C2RustUnnamed_0 = 0;
pub type DevNodeStaus = core::ffi::c_uint;
pub const DEVNODE_LAUNCHED: DevNodeStaus = 2;
pub const DEVNODE_INITED: DevNodeStaus = 1;
pub const DEVNODE_NONE: DevNodeStaus = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDriverLoader {
    pub object: HdfObject,
    pub GetDriver: Option<
        unsafe extern "C" fn(*const core::ffi::c_char) -> *mut HdfDriver,
    >,
    pub ReclaimDriver: Option<unsafe extern "C" fn(*mut HdfDriver) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdfServiceInfo {
    pub servName: *const core::ffi::c_char,
    pub servInfo: *const core::ffi::c_char,
    pub devClass: uint16_t,
    pub devId: devid_t,
    pub interfaceDesc: *const core::ffi::c_char,
}
pub const HDF_OBJECT_ID_DEVICE_SERVICE: C2RustUnnamed_1 = 7;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const HDF_OBJECT_ID_MAX: C2RustUnnamed_1 = 9;
pub const HDF_OBJECT_ID_REMOTE_SERVICE: C2RustUnnamed_1 = 8;
pub const HDF_OBJECT_ID_DEVICE_TOKEN: C2RustUnnamed_1 = 6;
pub const HDF_OBJECT_ID_DEVICE: C2RustUnnamed_1 = 5;
pub const HDF_OBJECT_ID_DRIVER_LOADER: C2RustUnnamed_1 = 4;
pub const HDF_OBJECT_ID_DRIVER_INSTALLER: C2RustUnnamed_1 = 3;
pub const HDF_OBJECT_ID_DEVHOST_SERVICE: C2RustUnnamed_1 = 2;
pub const HDF_OBJECT_ID_DEVSVC_MANAGER: C2RustUnnamed_1 = 1;
pub const HDF_OBJECT_ID_DEVMGR_SERVICE: C2RustUnnamed_1 = 0;
#[inline]
pub unsafe extern "C" fn HdfServiceInfoInit(
    mut info: *mut HdfServiceInfo,
    mut devNode: *const HdfDeviceNode,
) {
    (*info).servName = (*devNode).servName;
    (*info).servInfo = (*devNode).servInfo;
    (*info).devClass = (*devNode).deviceObject.deviceClass as uint16_t;
    (*info).devId = (*devNode).devId;
    (*info).interfaceDesc = (*devNode).interfaceDesc;
}
pub unsafe extern "C" fn HdfDeviceNodePublishLocalService(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    if devNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to publish local service, device is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    if ((*devNode).hostService).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to publish local service, host service is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    return HdfServiceObserverPublishService(
        &mut (*(*devNode).hostService).observer,
        (*devNode).servName,
        (*devNode).devId,
        (*devNode).policy,
        (*devNode).deviceObject.service as *mut HdfObject,
    );
}
pub unsafe extern "C" fn HdfDeviceNodePublishService(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut status: core::ffi::c_int = HDF_SUCCESS as core::ffi::c_int;
    let mut nodeIf: *mut IDeviceNode = 0 as *mut IDeviceNode;
    if (*devNode).policy as core::ffi::c_int == SERVICE_POLICY_NONE as core::ffi::c_int
        || !((*devNode).servName).is_null()
            && strlen((*devNode).servName) == 0 as core::ffi::c_uint
    {
        return status;
    }
    nodeIf = &mut (*devNode).super_0;
    if (*devNode).policy as core::ffi::c_int == SERVICE_POLICY_PUBLIC as core::ffi::c_int
        || (*devNode).policy as core::ffi::c_int
            == SERVICE_POLICY_CAPACITY as core::ffi::c_int
    {
        if ((*nodeIf).PublishService).is_some() {
            status = ((*nodeIf).PublishService)
                .expect("non-null function pointer")(devNode);
        }
    }
    if status == HDF_SUCCESS as core::ffi::c_int {
        status = HdfDeviceNodePublishLocalService(devNode);
    }
    return status;
}
pub unsafe extern "C" fn DeviceDriverBind(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    let mut driverEntry: *const HdfDriverEntry = 0 as *const HdfDriverEntry;
    if devNode.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    driverEntry = (*(*devNode).driver).entry;
    if (*devNode).policy as core::ffi::c_int == SERVICE_POLICY_PUBLIC as core::ffi::c_int
        || (*devNode).policy as core::ffi::c_int
            == SERVICE_POLICY_CAPACITY as core::ffi::c_int
    {
        if ((*driverEntry).Bind).is_none() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"device_node\0" as *const u8 as *const core::ffi::c_char,
                b"driver %{public}s bind method not implement\0" as *const u8
                    as *const core::ffi::c_char,
                (*driverEntry).moduleName,
            );
            (*devNode).devStatus = DEVNODE_NONE as core::ffi::c_int as uint8_t;
            return HDF_ERR_INVALID_OBJECT as core::ffi::c_int;
        }
        ret = ((*driverEntry).Bind)
            .expect("non-null function pointer")(&mut (*devNode).deviceObject)
            as core::ffi::c_int;
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"device_node\0" as *const u8 as *const core::ffi::c_char,
                b"bind driver %{public}s failed\0" as *const u8
                    as *const core::ffi::c_char,
                (*driverEntry).moduleName,
            );
            return HDF_DEV_ERR_DEV_INIT_FAIL as core::ffi::c_int;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
pub unsafe extern "C" fn HdfDeviceLaunchNode(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut driverEntry: *const HdfDriverEntry = 0 as *const HdfDriverEntry;
    let mut ret: core::ffi::c_int = 0;
    if devNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to launch service, device or service is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd002510 as core::ffi::c_uint,
        b"device_node\0" as *const u8 as *const core::ffi::c_char,
        b"launch devnode %{public}s\0" as *const u8 as *const core::ffi::c_char,
        if !((*devNode).servName).is_null() {
            (*devNode).servName as *const core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const core::ffi::c_char
        },
    );
    driverEntry = (*(*devNode).driver).entry;
    if driverEntry.is_null() || ((*driverEntry).Init).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to launch service, deviceEntry invalid\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    (*devNode).devStatus = DEVNODE_LAUNCHED as core::ffi::c_int as uint8_t;
    ret = DeviceDriverBind(devNode);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        return ret;
    }
    ret = ((*driverEntry).Init)
        .expect("non-null function pointer")(&mut (*devNode).deviceObject)
        as core::ffi::c_int;
    if ret != HDF_SUCCESS as core::ffi::c_int {
        return HDF_DEV_ERR_DEV_INIT_FAIL as core::ffi::c_int;
    }
    ret = HdfDeviceNodePublishService(devNode);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        return HDF_DEV_ERR_PUBLISH_FAIL as core::ffi::c_int;
    }
    ret = DevmgrServiceClntAttachDevice((*devNode).token as *mut IHdfDeviceToken);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        return HDF_DEV_ERR_ATTACHDEV_FAIL as core::ffi::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn HdfDeviceNodeAddPowerStateListener(
    mut devNode: *mut HdfDeviceNode,
    mut listener: *const IPowerEventListener,
) -> core::ffi::c_int {
    if !((*devNode).powerToken).is_null() {
        return HDF_FAILURE as core::ffi::c_int;
    }
    (*devNode).powerToken = PowerStateTokenNewInstance(
        &mut (*devNode).deviceObject,
        listener,
    ) as *mut PowerStateToken;
    return if !((*devNode).powerToken).is_null() {
        HDF_SUCCESS as core::ffi::c_int
    } else {
        HDF_FAILURE as core::ffi::c_int
    };
}
pub unsafe extern "C" fn HdfDeviceNodeRemovePowerStateListener(
    mut devNode: *mut HdfDeviceNode,
    mut listener: *const IPowerEventListener,
) {
    if devNode.is_null() || ((*devNode).powerToken).is_null() {
        return;
    }
    PowerStateTokenFreeInstance((*devNode).powerToken as *mut PowerStateToken);
    (*devNode).powerToken = 0 as *mut PowerStateToken;
}
pub unsafe extern "C" fn HdfDeviceNodePublishPublicService(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    if devNode.is_null() || ((*devNode).deviceObject.service).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to publish public service, devNode is NULL\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    let mut servInfo: HdfServiceInfo = HdfServiceInfo {
        servName: 0 as *const core::ffi::c_char,
        servInfo: 0 as *const core::ffi::c_char,
        devClass: 0,
        devId: 0,
        interfaceDesc: 0 as *const core::ffi::c_char,
    };
    HdfServiceInfoInit(&mut servInfo, devNode);
    ret = DevSvcManagerClntAddService(&mut (*devNode).deviceObject, &mut servInfo);
    if ret == HDF_SUCCESS as core::ffi::c_int {
        (*devNode).servStatus = 1 as core::ffi::c_int != 0;
    }
    return ret;
}
pub unsafe extern "C" fn HdfDeviceNodeRemoveService(
    mut devNode: *mut HdfDeviceNode,
) -> core::ffi::c_int {
    if !devNode.is_null() && (*devNode).servStatus as core::ffi::c_int != 0 {
        DevSvcManagerClntRemoveService((*devNode).servName);
        (*devNode).servStatus = 0 as core::ffi::c_int != 0;
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
pub unsafe extern "C" fn HdfDeviceUnlaunchNode(mut devNode: *mut HdfDeviceNode) {
    let mut driverEntry: *const HdfDriverEntry = 0 as *const HdfDriverEntry;
    let mut driverLoader: *mut IDriverLoader = 0 as *mut IDriverLoader;
    if devNode.is_null()
        || (*devNode).devStatus as core::ffi::c_int
            != DEVNODE_LAUNCHED as core::ffi::c_int
    {
        return;
    }
    if !((*devNode).driver).is_null() {
        driverEntry = (*(*devNode).driver).entry;
    }
    if !driverEntry.is_null() && ((*driverEntry).Release).is_some() {
        ((*driverEntry).Release)
            .expect("non-null function pointer")(&mut (*devNode).deviceObject);
    }
    if (*devNode).servStatus {
        ((*devNode).super_0.RemoveService).expect("non-null function pointer")(devNode);
    }
    DevmgrServiceClntDetachDevice((*devNode).devId);
    driverLoader = HdfDriverLoaderGetInstance();
    if !driverLoader.is_null() {
        ((*driverLoader).ReclaimDriver)
            .expect("non-null function pointer")((*devNode).driver);
        (*devNode).driver = 0 as *mut HdfDriver;
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"failed to get driver loader\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    (*devNode).devStatus = DEVNODE_INITED as core::ffi::c_int as uint8_t;
}
pub unsafe extern "C" fn HdfDeviceNodeConstruct(mut devNode: *mut HdfDeviceNode) {
    if !devNode.is_null() {
        let mut nodeIf: *mut IDeviceNode = &mut (*devNode).super_0;
        HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
        (*devNode).token = HdfDeviceTokenNewInstance() as *mut IHdfDeviceToken;
        (*nodeIf).LaunchNode = Some(
            HdfDeviceLaunchNode
                as unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int>;
        (*nodeIf).PublishService = Some(
            HdfDeviceNodePublishPublicService
                as unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int>;
        (*nodeIf).RemoveService = Some(
            HdfDeviceNodeRemoveService
                as unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> core::ffi::c_int>;
        (*nodeIf).UnlaunchNode = Some(
            HdfDeviceUnlaunchNode as unsafe extern "C" fn(*mut HdfDeviceNode) -> (),
        ) as Option<unsafe extern "C" fn(*mut HdfDeviceNode) -> ()>;
    }
}
pub unsafe extern "C" fn HdfDeviceNodeDestruct(mut devNode: *mut HdfDeviceNode) {
    if devNode.is_null() {
        return;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xd002510 as core::ffi::c_uint,
        b"device_node\0" as *const u8 as *const core::ffi::c_char,
        b"release devnode %{public}s\0" as *const u8 as *const core::ffi::c_char,
        (*devNode).servName,
    );
    let mut current_block_13: u64;
    match (*devNode).devStatus as core::ffi::c_int {
        2 => {
            HdfDeviceUnlaunchNode(devNode);
            current_block_13 = 11390969533560125515;
        }
        1 => {
            current_block_13 = 11390969533560125515;
        }
        0 | _ => {
            current_block_13 = 2968425633554183086;
        }
    }
    match current_block_13 {
        11390969533560125515 => {
            HdfDeviceTokenFreeInstance((*devNode).token as *mut IHdfDeviceToken);
            (*devNode).token = 0 as *mut IHdfDeviceToken;
            PowerStateTokenFreeInstance((*devNode).powerToken as *mut PowerStateToken);
            (*devNode).powerToken = 0 as *mut PowerStateToken;
            OsalMemFree((*devNode).servName as *mut core::ffi::c_void);
            OsalMemFree(
                (*devNode).servInfo as *mut core::ffi::c_char as *mut core::ffi::c_void,
            );
            OsalMemFree((*devNode).driverName as *mut core::ffi::c_void);
            (*devNode).servName = 0 as *mut core::ffi::c_char;
            (*devNode).servInfo = 0 as *const core::ffi::c_char;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn HdfDeviceNodeNewInstance(
    mut deviceInfo: *const HdfDeviceInfo,
    mut driver: *mut HdfDriver,
) -> *mut HdfDeviceNode {
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    if deviceInfo.is_null() {
        return 0 as *mut HdfDeviceNode;
    }
    devNode = HdfObjectManagerGetObject(HDF_OBJECT_ID_DEVICE_SERVICE as core::ffi::c_int)
        as *mut HdfDeviceNode;
    if devNode.is_null() {
        return 0 as *mut HdfDeviceNode;
    }
    (*devNode).driver = driver;
    (*devNode).devId = (*deviceInfo).deviceId as devid_t;
    (*devNode).permission = (*deviceInfo).permission;
    (*devNode).policy = (*deviceInfo).policy;
    (*(*devNode).token).devid = (*deviceInfo).deviceId as devid_t;
    (*devNode).servName = HdfStringCopy((*deviceInfo).svcName);
    (*(*devNode).token).servName = HdfStringCopy((*deviceInfo).svcName);
    (*(*devNode).token).deviceName = HdfStringCopy((*deviceInfo).deviceName);
    if ((*devNode).servName).is_null() {
        HdfDeviceNodeFreeInstance(devNode);
        return 0 as *mut HdfDeviceNode;
    }
    (*devNode).deviceObject.property = HcsGetNodeByMatchAttr(
        HdfGetHcsRootNode(),
        (*deviceInfo).deviceMatchAttr,
    ) as *const DeviceResourceNode;
    if ((*devNode).deviceObject.property).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xd002510 as core::ffi::c_uint,
            b"device_node\0" as *const u8 as *const core::ffi::c_char,
            b"node %{public}s property empty, match attr: %{public}s\0" as *const u8
                as *const core::ffi::c_char,
            (*deviceInfo).moduleName,
            (*deviceInfo).deviceMatchAttr,
        );
    }
    (*devNode).devStatus = DEVNODE_INITED as core::ffi::c_int as uint8_t;
    return devNode;
}
pub unsafe extern "C" fn HdfDeviceNodeFreeInstance(mut devNode: *mut HdfDeviceNode) {
    HdfObjectManagerFreeObject(devNode as *mut HdfObject);
}
