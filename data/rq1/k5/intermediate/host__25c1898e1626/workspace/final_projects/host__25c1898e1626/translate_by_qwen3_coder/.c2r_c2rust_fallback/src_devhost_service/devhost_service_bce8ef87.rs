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
    pub type DeviceResourceNode;
    pub type HdfSBuf;
    fn HdfServiceObserverConstruct(observer: *mut HdfServiceObserver) -> bool;
    fn HdfServiceObserverDestruct(observer: *mut HdfServiceObserver);
    fn PowerStateChange(
        stateToken: *mut PowerStateToken,
        pEvent: uint32_t,
    ) -> core::ffi::c_int;
    fn DevmgrServiceClntAttachDeviceHost(
        hostId: uint16_t,
        hostService: *mut IDevHostService,
    ) -> core::ffi::c_int;
    fn HdfDriverLoaderGetInstance() -> *mut IDriverLoader;
    fn HdfObjectManagerGetObject(objectId: core::ffi::c_int) -> *mut HdfObject;
    fn HdfObjectManagerFreeObject(object: *mut HdfObject);
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn HdfDeviceNewInstance() -> *mut HdfDevice;
    fn HdfDeviceFreeInstance(device: *mut HdfDevice);
    fn HdfDeviceNodeNewInstance(
        deviceInfo: *const HdfDeviceInfo,
        driver: *mut HdfDriver,
    ) -> *mut HdfDeviceNode;
    fn HdfDeviceNodeFreeInstance(devNode: *mut HdfDeviceNode);
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
pub const POWER_STATE_RESUME: HdfPowerState = 2;
pub const POWER_STATE_DOZE_RESUME: HdfPowerState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IDriverLoader {
    pub object: HdfObject,
    pub GetDriver: Option<
        unsafe extern "C" fn(*const core::ffi::c_char) -> *mut HdfDriver,
    >,
    pub ReclaimDriver: Option<unsafe extern "C" fn(*mut HdfDriver) -> ()>,
}
pub const HDF_OBJECT_ID_DEVHOST_SERVICE: C2RustUnnamed_0 = 2;
pub type HdfPowerState = core::ffi::c_uint;
pub const POWER_STATE_MAX: HdfPowerState = 4;
pub const POWER_STATE_SUSPEND: HdfPowerState = 3;
pub const POWER_STATE_DOZE_SUSPEND: HdfPowerState = 1;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HDF_OBJECT_ID_MAX: C2RustUnnamed_0 = 9;
pub const HDF_OBJECT_ID_REMOTE_SERVICE: C2RustUnnamed_0 = 8;
pub const HDF_OBJECT_ID_DEVICE_SERVICE: C2RustUnnamed_0 = 7;
pub const HDF_OBJECT_ID_DEVICE_TOKEN: C2RustUnnamed_0 = 6;
pub const HDF_OBJECT_ID_DEVICE: C2RustUnnamed_0 = 5;
pub const HDF_OBJECT_ID_DRIVER_LOADER: C2RustUnnamed_0 = 4;
pub const HDF_OBJECT_ID_DRIVER_INSTALLER: C2RustUnnamed_0 = 3;
pub const HDF_OBJECT_ID_DEVSVC_MANAGER: C2RustUnnamed_0 = 1;
pub const HDF_OBJECT_ID_DEVMGR_SERVICE: C2RustUnnamed_0 = 0;
#[inline]
unsafe extern "C" fn IsPowerWakeState(mut state: uint32_t) -> bool {
    return state == POWER_STATE_DOZE_RESUME as core::ffi::c_int as core::ffi::c_uint
        || state == POWER_STATE_RESUME as core::ffi::c_int as core::ffi::c_uint;
}
unsafe extern "C" fn DevHostServiceFindDevice(
    mut hostService: *mut DevHostService,
    mut deviceId: uint16_t,
) -> *mut HdfDevice {
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    if hostService.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to find driver, hostService is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut HdfDevice;
    }
    device = ((*hostService).devices.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    while &mut (*device).node as *mut DListHead
        != &mut (*hostService).devices as *mut DListHead
    {
        if (*device).deviceId >> 8 as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint
            == deviceId as core::ffi::c_uint
        {
            return device;
        }
        device = ((*device).node.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDevice;
    }
    return 0 as *mut HdfDevice;
}
unsafe extern "C" fn DevHostServiceFreeDevice(
    mut hostService: *mut DevHostService,
    mut device: *mut HdfDevice,
) {
    if !device.is_null() {
        DListRemove(&mut (*device).node);
        HdfDeviceFreeInstance(device);
    }
}
unsafe extern "C" fn DevHostServiceQueryOrAddDevice(
    mut inst: *mut DevHostService,
    mut deviceId: uint16_t,
) -> *mut HdfDevice {
    let mut device: *mut HdfDevice = DevHostServiceFindDevice(inst, deviceId);
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"%{public}s can't find device, try to create\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"DevHostServiceQueryOrAddDevice\0"))
                .as_ptr(),
        );
        device = HdfDeviceNewInstance();
        if device.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                b"Dev host service failed to create driver instance\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return 0 as *mut HdfDevice;
        }
        (*device).deviceId = (((*inst).hostId as core::ffi::c_int)
            << 16 as core::ffi::c_int + 8 as core::ffi::c_int
            | (deviceId as core::ffi::c_int) << 8 as core::ffi::c_int
            | 0 as core::ffi::c_int) as devid_t;
        DListInsertHead(&mut (*device).node, &mut (*inst).devices);
        HiLogPrint(
            LOG_CORE,
            LOG_DEBUG,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"%{public}s add device complete\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"DevHostServiceQueryOrAddDevice\0"))
                .as_ptr(),
        );
    }
    return device;
}
#[inline]
unsafe extern "C" fn DListHeadInit(mut head: *mut DListHead) {
    (*head).next = head;
    (*head).prev = head;
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceAddDevice(
    mut inst: *mut IDevHostService,
    mut deviceInfo: *const HdfDeviceInfo,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = HDF_FAILURE as core::ffi::c_int;
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    let mut driver: *mut HdfDriver = 0 as *mut HdfDriver;
    let mut hostService: *mut DevHostService = (inst as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut DevHostService)).super_0 as *mut IDevHostService
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut DevHostService;
    let mut driverLoader: *mut IDriverLoader = HdfDriverLoaderGetInstance();
    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null()
        || ((*driverLoader).GetDriver).is_none()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to add device, input param is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return ret;
    }
    device = DevHostServiceQueryOrAddDevice(
        hostService,
        ((*deviceInfo).deviceId >> 8 as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint) as uint16_t,
    );
    if device.is_null() || ((*device).super_0.Attach).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to add device, device or Attach func is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int;
    }
    devNode = ((*device).super_0.GetDeviceNode)
        .expect(
            "non-null function pointer",
        )(&mut (*device).super_0, (*deviceInfo).deviceId as devid_t);
    if !devNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to add device, device already exist\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_DEVICE_BUSY as core::ffi::c_int;
    }
    driver = ((*driverLoader).GetDriver)
        .expect("non-null function pointer")((*deviceInfo).moduleName);
    if driver.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to add device %{public}s, get driver failed\0" as *const u8
                as *const core::ffi::c_char,
            (*deviceInfo).moduleName,
        );
        ret = HDF_DEV_ERR_NODATA as core::ffi::c_int;
    } else {
        devNode = HdfDeviceNodeNewInstance(deviceInfo, driver);
        if devNode.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                b"failed to add device, create devNode failed\0" as *const u8
                    as *const core::ffi::c_char,
            );
            ((*driverLoader).ReclaimDriver).expect("non-null function pointer")(driver);
            return HDF_DEV_ERR_NO_MEMORY as core::ffi::c_int;
        }
        (*devNode).hostService = hostService;
        (*devNode).device = device;
        (*devNode).driver = driver;
        ret = ((*device).super_0.Attach)
            .expect("non-null function pointer")(&mut (*device).super_0, devNode);
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xd002510 as core::ffi::c_uint,
                b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                b"failed to add device, attach devNode failed\0" as *const u8
                    as *const core::ffi::c_char,
            );
            HdfDeviceNodeFreeInstance(devNode);
        } else {
            HiLogPrint(
                LOG_CORE,
                LOG_DEBUG,
                0xd002510 as core::ffi::c_uint,
                b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                b"%{public}s add device success\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"DevHostServiceAddDevice\0"))
                    .as_ptr(),
            );
            return HDF_SUCCESS as core::ffi::c_int;
        }
    }
    if DListIsEmpty(&mut (*device).devNodes) {
        DevHostServiceFreeDevice(hostService, device);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn DListIsEmpty(mut head: *const DListHead) -> bool {
    return if (*head).next == head as *mut DListHead {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn DListRemove(mut entry: *mut DListHead) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
    (*entry).prev = 0 as *mut DListHead;
    (*entry).next = 0 as *mut DListHead;
}
#[inline]
unsafe extern "C" fn DListInsertHead(
    mut entry: *mut DListHead,
    mut head: *mut DListHead,
) {
    (*entry).next = (*head).next;
    (*entry).prev = head;
    (*(*head).next).prev = entry;
    (*head).next = entry;
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceDelDevice(
    mut inst: *mut IDevHostService,
    mut devId: devid_t,
) -> core::ffi::c_int {
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    let mut hostService: *mut DevHostService = inst as *mut DevHostService;
    let mut devNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    device = DevHostServiceFindDevice(
        hostService,
        (devId >> 8 as core::ffi::c_int
            & (((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint) as uint16_t,
    );
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_WARN,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to del device, device is not exist\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_SUCCESS as core::ffi::c_int;
    }
    devNode = ((*device).super_0.GetDeviceNode)
        .expect("non-null function pointer")(&mut (*device).super_0, devId);
    if devNode.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to del device, not exist\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int;
    }
    if ((*device).super_0.Detach).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to del device, invalid device\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int;
    }
    if ((*device).super_0.Detach)
        .expect("non-null function pointer")(&mut (*device).super_0, devNode)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to detach device\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    HdfDeviceNodeFreeInstance(devNode);
    if DListIsEmpty(&mut (*device).devNodes) {
        DevHostServiceFreeDevice(hostService, device);
    }
    HiLogPrint(
        LOG_CORE,
        LOG_DEBUG,
        0xd002510 as core::ffi::c_uint,
        b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
        b"%{public}s add device success\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 24],
            [core::ffi::c_char; 24],
        >(*b"DevHostServiceDelDevice\0"))
            .as_ptr(),
    );
    return HDF_SUCCESS as core::ffi::c_int;
}
unsafe extern "C" fn DevHostServiceStartService(
    mut service: *mut IDevHostService,
) -> core::ffi::c_int {
    let mut hostService: *mut DevHostService = service as *mut DevHostService;
    if hostService.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to start device service, hostService is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    return DevmgrServiceClntAttachDeviceHost((*hostService).hostId, service);
}
unsafe extern "C" fn ApplyDevicesPowerState(
    mut device: *mut HdfDevice,
    mut state: uint32_t,
) -> core::ffi::c_int {
    let mut deviceNode: *mut HdfDeviceNode = 0 as *mut HdfDeviceNode;
    let mut ret: core::ffi::c_int = 0;
    if IsPowerWakeState(state) {
        deviceNode = ((*device).devNodes.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDeviceNode;
        while &mut (*deviceNode).entry as *mut DListHead
            != &mut (*device).devNodes as *mut DListHead
        {
            if !((*deviceNode).powerToken).is_null() {
                ret = PowerStateChange(
                    (*deviceNode).powerToken as *mut PowerStateToken,
                    state,
                );
                if ret != HDF_SUCCESS as core::ffi::c_int {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd002510 as core::ffi::c_uint,
                        b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                        b"device %{public}s failed to resume(%{public}u)\0" as *const u8
                            as *const core::ffi::c_char,
                        (*(*(*deviceNode).driver).entry).moduleName,
                        state,
                    );
                }
            }
            deviceNode = ((*deviceNode).entry.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_long as *mut HdfDeviceNode;
        }
    } else {
        deviceNode = ((*device).devNodes.prev as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDeviceNode;
        while &mut (*deviceNode).entry as *mut DListHead
            != &mut (*device).devNodes as *mut DListHead
        {
            if !((*deviceNode).powerToken).is_null() {
                ret = PowerStateChange(
                    (*deviceNode).powerToken as *mut PowerStateToken,
                    state,
                );
                if ret != HDF_SUCCESS as core::ffi::c_int {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xd002510 as core::ffi::c_uint,
                        b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
                        b"device %{public}s failed to suspend(%{public}u)\0" as *const u8
                            as *const core::ffi::c_char,
                        (*(*(*deviceNode).driver).entry).moduleName,
                        state,
                    );
                }
            }
            deviceNode = ((*deviceNode).entry.prev as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut HdfDeviceNode)).entry as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_long as *mut HdfDeviceNode;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int;
}
unsafe extern "C" fn DevHostServicePmNotify(
    mut service: *mut IDevHostService,
    mut state: uint32_t,
) -> core::ffi::c_int {
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    let mut ret: core::ffi::c_int = HDF_SUCCESS as core::ffi::c_int;
    let mut hostService: *mut DevHostService = (service as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut DevHostService)).super_0 as *mut IDevHostService
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut DevHostService;
    if hostService.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
            b"failed to start device service, hostService is null\0" as *const u8
                as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    HiLogPrint(
        LOG_CORE,
        LOG_DEBUG,
        0xd002510 as core::ffi::c_uint,
        b"devhost_service\0" as *const u8 as *const core::ffi::c_char,
        b"host(%{public}s) set power state=%{public}u\0" as *const u8
            as *const core::ffi::c_char,
        (*hostService).hostName,
        state,
    );
    if IsPowerWakeState(state) {
        device = ((*hostService).devices.prev as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDevice;
        while &mut (*device).node as *mut DListHead
            != &mut (*hostService).devices as *mut DListHead
        {
            if ApplyDevicesPowerState(device, state) != HDF_SUCCESS as core::ffi::c_int {
                ret = HDF_FAILURE as core::ffi::c_int;
            }
            device = ((*device).node.prev as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_long as *mut HdfDevice;
        }
    } else {
        device = ((*hostService).devices.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDevice;
        while &mut (*device).node as *mut DListHead
            != &mut (*hostService).devices as *mut DListHead
        {
            if ApplyDevicesPowerState(device, state) != HDF_SUCCESS as core::ffi::c_int {
                ret = HDF_FAILURE as core::ffi::c_int;
            }
            device = ((*device).node.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_long as *mut HdfDevice;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceConstruct(mut service: *mut DevHostService) {
    if service.is_null() {
        return;
    }
    let mut hostServiceIf: *mut IDevHostService = &mut (*service).super_0;
    (*hostServiceIf).AddDevice = Some(
        DevHostServiceAddDevice
            as unsafe extern "C" fn(
                *mut IDevHostService,
                *const HdfDeviceInfo,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut IDevHostService,
                *const HdfDeviceInfo,
            ) -> core::ffi::c_int,
        >;
    (*hostServiceIf).DelDevice = Some(
        DevHostServiceDelDevice
            as unsafe extern "C" fn(*mut IDevHostService, devid_t) -> core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut IDevHostService, devid_t) -> core::ffi::c_int>;
    (*hostServiceIf).StartService = Some(
        DevHostServiceStartService
            as unsafe extern "C" fn(*mut IDevHostService) -> core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut IDevHostService) -> core::ffi::c_int>;
    (*hostServiceIf).PmNotify = Some(
        DevHostServicePmNotify
            as unsafe extern "C" fn(*mut IDevHostService, uint32_t) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut IDevHostService, uint32_t) -> core::ffi::c_int,
        >;
    DListHeadInit(&mut (*service).devices);
    HdfServiceObserverConstruct(&mut (*service).observer);
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceDestruct(mut service: *mut DevHostService) {
    let mut device: *mut HdfDevice = 0 as *mut HdfDevice;
    let mut tmp: *mut HdfDevice = 0 as *mut HdfDevice;
    if service.is_null() {
        return;
    }
    device = ((*service).devices.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    tmp = ((*device).node.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfDevice;
    while &mut (*device).node as *mut DListHead
        != &mut (*service).devices as *mut DListHead
    {
        HdfDeviceFreeInstance(device);
        device = tmp;
        tmp = ((*device).node.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfDevice)).node as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfDevice;
    }
    HdfServiceObserverDestruct(&mut (*service).observer);
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceCreate() -> *mut HdfObject {
    let mut devHostService: *mut DevHostService = OsalMemCalloc(
        ::core::mem::size_of::<DevHostService>() as size_t,
    ) as *mut DevHostService;
    if !devHostService.is_null() {
        DevHostServiceConstruct(devHostService);
    }
    return devHostService as *mut HdfObject;
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceRelease(mut object: *mut HdfObject) {
    let mut devHostService: *mut DevHostService = object as *mut DevHostService;
    if !devHostService.is_null() {
        DevHostServiceDestruct(devHostService);
        OsalMemFree(devHostService as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceNewInstance(
    mut hostId: uint16_t,
    mut hostName: *const core::ffi::c_char,
) -> *mut IDevHostService {
    let mut hostService: *mut DevHostService = HdfObjectManagerGetObject(
        HDF_OBJECT_ID_DEVHOST_SERVICE as core::ffi::c_int,
    ) as *mut DevHostService;
    if !hostService.is_null() && !hostName.is_null() {
        (*hostService).hostId = hostId;
        (*hostService).hostName = hostName;
    }
    return hostService as *mut IDevHostService;
}
#[no_mangle]
pub unsafe extern "C" fn DevHostServiceFreeInstance(mut service: *mut IDevHostService) {
    if !service.is_null() {
        HdfObjectManagerFreeObject(&mut (*service).object);
    }
}
