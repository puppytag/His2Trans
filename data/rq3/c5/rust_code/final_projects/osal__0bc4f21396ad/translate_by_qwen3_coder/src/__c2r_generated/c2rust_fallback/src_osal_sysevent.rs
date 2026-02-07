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
    pub fn OsalMutexInit(mutex: *mut OsalMutex) -> int32_t;
    pub fn OsalMemFree(mem: *mut core::ffi::c_void);
    pub fn HdfSbufWriteUint64(sbuf: *mut HdfSBuf, value: uint64_t) -> bool;
    pub fn OsalMutexLock(mutex: *mut OsalMutex) -> int32_t;
    pub fn OsalMutexUnlock(mutex: *mut OsalMutex) -> int32_t;
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
    pub fn HdfSbufReadBuffer(
        sbuf: *mut HdfSBuf,
        data: *mut *const core::ffi::c_void,
        readSize: *mut uint32_t,
    ) -> bool;
    pub fn HdfSbufReadString(sbuf: *mut HdfSBuf) -> *const core::ffi::c_char;
    pub fn HdfSbufObtain(capacity: size_t) -> *mut HdfSBuf;
    pub fn HdfSbufRecycle(sbuf: *mut HdfSBuf);
}
pub type size_t = core::ffi::c_uint;
pub type int32_t = core::ffi::c_int;
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
pub struct HdfSysEvent {
    pub eventClass: uint64_t,
    pub syncToken: uint64_t,
    pub eventid: uint32_t,
    pub reserved: uint32_t,
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
pub struct HdfSysEventNotifier {
    pub mutex: OsalMutex,
    pub notifyNodeList: DListHead,
    pub waitList: DListHead,
    pub ioServiceListener: HdfDevEventlistener,
    pub keventIoService: *mut HdfIoService,
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
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
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
pub unsafe extern "C" fn HdfSysEventNotifierGetInstance() -> *mut HdfSysEventNotifier {
    static mut hdfSysEventNotifier: *mut HdfSysEventNotifier = 0
        as *const HdfSysEventNotifier as *mut HdfSysEventNotifier;
    if !hdfSysEventNotifier.is_null() {
        return hdfSysEventNotifier;
    }
    let mut notifier: *mut HdfSysEventNotifier = OsalMemCalloc(
        ::core::mem::size_of::<HdfSysEventNotifier>() as size_t,
    ) as *mut HdfSysEventNotifier;
    if notifier.is_null() {
        return 0 as *mut HdfSysEventNotifier;
    }
    let mut ret: core::ffi::c_int = OsalMutexInit(&mut (*notifier).mutex)
        as core::ffi::c_int;
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMemFree(notifier as *mut core::ffi::c_void);
        return 0 as *mut HdfSysEventNotifier;
    }
    DListHeadInit(&mut (*notifier).notifyNodeList);
    hdfSysEventNotifier = notifier;
    return notifier;
}
#[inline]
pub unsafe extern "C" fn DListHeadInit(mut head: *mut DListHead) {
    (*head).next = head;
    (*head).prev = head;
}
pub unsafe extern "C" fn FinishEvent(
    mut service: *mut HdfIoService,
    mut event: *const HdfSysEvent,
) -> core::ffi::c_int {
    let mut sbuf: *mut HdfSBuf = HdfSbufObtain(
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    if sbuf.is_null() {
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int;
    }
    if !HdfSbufWriteUint64(sbuf, (*event).syncToken) {
        HdfSbufRecycle(sbuf);
        return HDF_FAILURE as core::ffi::c_int;
    }
    let mut ret: core::ffi::c_int = ((*(*service).dispatcher).Dispatch)
        .expect(
            "non-null function pointer",
        )(&mut (*service).object, 1 as core::ffi::c_int, sbuf, 0 as *mut HdfSBuf);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"usysevent\0" as *const u8 as *const core::ffi::c_char,
            b"failed to finish sysevent, %{public}d\0" as *const u8
                as *const core::ffi::c_char,
            ret,
        );
    }
    HdfSbufRecycle(sbuf);
    return ret;
}
#[inline]
pub unsafe extern "C" fn DListIsEmpty(mut head: *const DListHead) -> bool {
    return if (*head).next == head as *mut DListHead {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    } != 0;
}
#[inline]
pub unsafe extern "C" fn DListRemove(mut entry: *mut DListHead) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
    (*entry).prev = 0 as *mut DListHead;
    (*entry).next = 0 as *mut DListHead;
}
pub unsafe extern "C" fn OnKEventReceived(
    mut listener: *mut HdfDevEventlistener,
    mut service: *mut HdfIoService,
    mut id: uint32_t,
    mut data: *mut HdfSBuf,
) -> core::ffi::c_int {
    let mut notifier: *mut HdfSysEventNotifier = (*listener).priv_0
        as *mut HdfSysEventNotifier;
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int;
    }
    if id != 0xfade as core::ffi::c_uint {
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int;
    }
    let mut receivedEvent: *mut HdfSysEvent = 0 as *mut HdfSysEvent;
    let mut receivedEventLen: uint32_t = 0 as uint32_t;
    if !HdfSbufReadBuffer(
        data,
        &mut receivedEvent as *mut *mut HdfSysEvent as *mut *const core::ffi::c_void,
        &mut receivedEventLen,
    ) || receivedEventLen as usize != ::core::mem::size_of::<HdfSysEvent>() as usize
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"usysevent\0" as *const u8 as *const core::ffi::c_char,
            b"failed to read kevent object\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_FAILURE as core::ffi::c_int;
    }
    let mut eventContent: *const core::ffi::c_char = HdfSbufReadString(data);
    eventContent = if eventContent.is_null() {
        b"\0" as *const u8 as *const core::ffi::c_char
    } else {
        eventContent
    };
    OsalMutexLock(&mut (*notifier).mutex);
    let mut notifyNode: *mut HdfSysEventNotifyNode = 0 as *mut HdfSysEventNotifyNode;
    notifyNode = ((*notifier).notifyNodeList.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut HdfSysEventNotifyNode)).listNode as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_long as *mut HdfSysEventNotifyNode;
    while &mut (*notifyNode).listNode as *mut DListHead
        != &mut (*notifier).notifyNodeList as *mut DListHead
    {
        if (*receivedEvent).eventClass & (*notifyNode).classFilter != 0 {
            ((*notifyNode).callback)
                .expect(
                    "non-null function pointer",
                )(
                notifyNode,
                (*receivedEvent).eventClass,
                (*receivedEvent).eventid,
                eventContent,
            );
        }
        notifyNode = ((*notifyNode).listNode.next as *mut core::ffi::c_char)
            .offset_from(
                &mut (*(0 as *mut HdfSysEventNotifyNode)).listNode as *mut DListHead
                    as *mut core::ffi::c_char,
            ) as core::ffi::c_long as *mut HdfSysEventNotifyNode;
    }
    if (*receivedEvent).syncToken != 0 as core::ffi::c_ulonglong {
        FinishEvent(service, receivedEvent);
    }
    OsalMutexUnlock(&mut (*notifier).mutex);
    return HDF_SUCCESS as core::ffi::c_int;
}
#[inline]
pub unsafe extern "C" fn DListInsertTail(
    mut entry: *mut DListHead,
    mut head: *mut DListHead,
) {
    (*entry).next = head;
    (*entry).prev = (*head).prev;
    (*(*head).prev).next = entry;
    (*head).prev = entry;
}
pub unsafe extern "C" fn InitKeventIoServiceListenerLocked(
    mut notifier: *mut HdfSysEventNotifier,
) -> core::ffi::c_int {
    if ((*notifier).keventIoService).is_null() {
        (*notifier).keventIoService = HdfIoServiceBind(
            b"hdf_kevent\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if ((*notifier).keventIoService).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"usysevent\0" as *const u8 as *const core::ffi::c_char,
            b" ioservice %{public}s is invalid\0" as *const u8
                as *const core::ffi::c_char,
            b"hdf_kevent\0" as *const u8 as *const core::ffi::c_char,
        );
        return HDF_DEV_ERR_NO_DEVICE as core::ffi::c_int;
    }
    (*notifier).ioServiceListener.onReceive = Some(
        OnKEventReceived
            as unsafe extern "C" fn(
                *mut HdfDevEventlistener,
                *mut HdfIoService,
                uint32_t,
                *mut HdfSBuf,
            ) -> core::ffi::c_int,
    ) as OnDevEventReceive;
    (*notifier).ioServiceListener.priv_0 = notifier as *mut core::ffi::c_void;
    let mut ret: core::ffi::c_int = HdfDeviceRegisterEventListener(
        (*notifier).keventIoService,
        &mut (*notifier).ioServiceListener,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xd002510 as core::ffi::c_uint,
            b"usysevent\0" as *const u8 as *const core::ffi::c_char,
            b" ioservice %{public}s is invalid\0" as *const u8
                as *const core::ffi::c_char,
            b"hdf_kevent\0" as *const u8 as *const core::ffi::c_char,
        );
        HdfIoServiceRecycle((*notifier).keventIoService);
        (*notifier).keventIoService = 0 as *mut HdfIoService;
    }
    return ret;
}
pub unsafe extern "C" fn DeInitKeventIoServiceListenerLocked(
    mut notifier: *mut HdfSysEventNotifier,
) {
    if ((*notifier).keventIoService).is_null() {
        return;
    }
    HdfDeviceUnregisterEventListener(
        (*notifier).keventIoService,
        &mut (*notifier).ioServiceListener,
    );
    HdfIoServiceRecycle((*notifier).keventIoService);
    (*notifier).keventIoService = 0 as *mut HdfIoService;
}
pub unsafe extern "C" fn HdfSysEventNotifyRegister(
    mut notifierNode: *mut HdfSysEventNotifyNode,
    mut classSet: uint64_t,
) -> int32_t {
    if notifierNode.is_null() {
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    let mut notifier: *mut HdfSysEventNotifier = HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return HDF_DEV_ERR_NO_MEMORY as core::ffi::c_int as int32_t;
    }
    OsalMutexLock(&mut (*notifier).mutex);
    DListInsertTail(&mut (*notifierNode).listNode, &mut (*notifier).notifyNodeList);
    (*notifierNode).classFilter = classSet;
    let mut ret: int32_t = InitKeventIoServiceListenerLocked(notifier) as int32_t;
    if ret != HDF_SUCCESS as core::ffi::c_int {
        DListRemove(&mut (*notifierNode).listNode);
    }
    OsalMutexUnlock(&mut (*notifier).mutex);
    return ret;
}
pub unsafe extern "C" fn HdfSysEventNotifyUnregister(
    mut notifierNode: *mut HdfSysEventNotifyNode,
) {
    if notifierNode.is_null() {
        return;
    }
    let mut notifier: *mut HdfSysEventNotifier = HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return;
    }
    OsalMutexLock(&mut (*notifier).mutex);
    DListRemove(&mut (*notifierNode).listNode);
    if DListIsEmpty(&mut (*notifier).notifyNodeList) {
        DeInitKeventIoServiceListenerLocked(notifier);
    }
    OsalMutexUnlock(&mut (*notifier).mutex);
}
