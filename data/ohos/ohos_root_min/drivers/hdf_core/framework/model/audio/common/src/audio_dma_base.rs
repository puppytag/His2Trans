extern "C" {
    pub type HdfSBuf;
    pub type AudioSapmRoute;
    pub type AudioSapmComponent;
    pub type DaiDevice;
    pub type DspDevice;
    pub type CodecDevice;
    fn HiLogPrint(
        type_0: LogType,
        level: LogLevel,
        domain: core::ffi::c_uint,
        tag: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn OsalMemCalloc(size: size_t) -> *mut core::ffi::c_void;
    fn OsalMemFree(mem: *mut core::ffi::c_void);
    fn AudioGetRegConfig(
        device: *const HdfDeviceObject,
        configData: *mut AudioRegCfgData,
    ) -> int32_t;
}
pub type size_t = core::ffi::c_uint;
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
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioConfigData {
    pub cardServiceName: *const core::ffi::c_char,
    pub codecName: *const core::ffi::c_char,
    pub platformName: *const core::ffi::c_char,
    pub cpuDaiName: *const core::ffi::c_char,
    pub codecDaiName: *const core::ffi::c_char,
    pub dspName: *const core::ffi::c_char,
    pub dspDaiName: *const core::ffi::c_char,
}
pub type AudioSapmTurnStandbyMode = core::ffi::c_uint;
pub const AUDIO_SAPM_TURN_STANDBY_BUTT: AudioSapmTurnStandbyMode = 2;
pub const AUDIO_SAPM_TURN_STANDBY_NOW: AudioSapmTurnStandbyMode = 1;
pub const AUDIO_SAPM_TURN_STANDBY_LATER: AudioSapmTurnStandbyMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCard {
    pub device: *mut HdfDeviceObject,
    pub rtd: *mut AudioRuntimeDeivces,
    pub configData: AudioConfigData,
    pub sapmComponents: *const AudioSapmComponent,
    pub sapmComponentsNum: int32_t,
    pub sapmRoutes: *const AudioSapmRoute,
    pub sapmRoutesNum: int32_t,
    pub list: DListHead,
    pub controls: DListHead,
    pub components: DListHead,
    pub paths: DListHead,
    pub sapmDirty: DListHead,
    pub standbyMode: AudioSapmTurnStandbyMode,
    pub time: uint64_t,
    pub sapmSleepState: bool,
    pub sapmStandbyState: bool,
    pub sapmMonitorState: bool,
    pub sapmStandbyStartTimeFlag: bool,
    pub sapmSleepStartTimeFlag: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRuntimeDeivces {
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub codecDai: *mut DaiDevice,
    pub cpuDai: *mut DaiDevice,
    pub dsp: *mut DspDevice,
    pub dspDai: *mut DaiDevice,
    pub complete: uint8_t,
    pub frameBits: uint32_t,
    pub dmaArea: *mut uint8_t,
    pub bufferSize: core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlatformDevice {
    pub devPlatformName: *const core::ffi::c_char,
    pub devData: *mut PlatformData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlatformData {
    pub drvPlatformName: *const core::ffi::c_char,
    pub PlatformInit: Option<
        unsafe extern "C" fn(*const AudioCard, *const PlatformDevice) -> int32_t,
    >,
    pub ops: *mut AudioDmaOps,
    pub renderBufInfo: CircleBufInfo,
    pub captureBufInfo: CircleBufInfo,
    pub renderPcmInfo: PcmInfo,
    pub capturePcmInfo: PcmInfo,
    pub platformInitFlag: bool,
    pub mmapData: AudioMmapData,
    pub mmapLoopCount: uint32_t,
    pub dmaPrv: *mut core::ffi::c_void,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRegCfgGroupNode {
    pub itemNum: uint8_t,
    pub groupIndex: AudioRegOpsType,
    pub addrCfgItem: *mut AudioAddrConfig,
    pub regCfgItem: *mut AudioMixerControl,
    pub regEnumCfgItem: *mut AudioEnumCtrlConfig,
    pub ctrlCfgItem: *mut AudioControlConfig,
    pub sapmCompItem: *mut AudioSapmCtrlConfig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmCtrlConfig {
    pub sapmType: uint8_t,
    pub compNameIndex: uint16_t,
    pub reg: uint32_t,
    pub mask: uint32_t,
    pub shift: uint8_t,
    pub invert: uint8_t,
    pub kcontrolNews: uint32_t,
    pub kcontrolsNum: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioControlConfig {
    pub arrayIndex: uint16_t,
    pub iface: uint16_t,
    pub type_0: uint16_t,
    pub enable: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioEnumCtrlConfig {
    pub reg: uint32_t,
    pub reg2: uint32_t,
    pub shiftLeft: uint8_t,
    pub shiftRight: uint8_t,
    pub max: uint32_t,
    pub mask: uint32_t,
    pub texts: uint32_t,
    pub values: uint32_t,
    pub sapm: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioMixerControl {
    pub min: uint32_t,
    pub max: uint32_t,
    pub platformMax: int32_t,
    pub mask: uint32_t,
    pub reg: uint32_t,
    pub rreg: uint32_t,
    pub shift: uint32_t,
    pub rshift: uint32_t,
    pub invert: uint32_t,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioAddrConfig {
    pub addr: uint32_t,
    pub value: uint32_t,
}
pub type AudioRegOpsType = core::ffi::c_uint;
pub const AUDIO_GROUP_MAX: AudioRegOpsType = 12;
pub const AUDIO_SAPM_CFG_GROUP: AudioRegOpsType = 11;
pub const AUDIO_SAPM_COMP_GROUP: AudioRegOpsType = 10;
pub const AUDIO_CTRL_CFG_GROUP: AudioRegOpsType = 9;
pub const AUDIO_DAI_TRIGGER_GROUP: AudioRegOpsType = 8;
pub const AUDIO_DAI_PATAM_GROUP: AudioRegOpsType = 7;
pub const AUDIO_DAI_STARTUP_PATAM_GROUP: AudioRegOpsType = 6;
pub const AUDIO_CTRL_SAPM_PATAM_MUX_GROUP: AudioRegOpsType = 5;
pub const AUDIO_CTRL_SAPM_PATAM_GROUP: AudioRegOpsType = 4;
pub const AUDIO_CTRL_PATAM_MUX_GROUP: AudioRegOpsType = 3;
pub const AUDIO_CTRL_PATAM_GROUP: AudioRegOpsType = 2;
pub const AUDIO_INIT_GROUP: AudioRegOpsType = 1;
pub const AUDIO_RSET_GROUP: AudioRegOpsType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioRegCfgData {
    pub audioIdInfo: AudioIdInfo,
    pub audioRegParams: [*mut AudioRegCfgGroupNode; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioIdInfo {
    pub chipName: *const core::ffi::c_char,
    pub chipIdRegister: uint32_t,
    pub chipIdSize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioMmapData {
    pub memoryAddress: *mut core::ffi::c_void,
    pub memoryFd: int32_t,
    pub totalBufferFrames: int32_t,
    pub transferFrameSize: int32_t,
    pub isShareable: int32_t,
    pub offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PcmInfo {
    pub streamType: AudioStreamType,
    pub channels: uint32_t,
    pub rate: uint32_t,
    pub bitWidth: uint32_t,
    pub frameSize: uint32_t,
    pub isBigEndian: bool,
    pub isSignedData: bool,
    pub startThreshold: uint32_t,
    pub stopThreshold: uint32_t,
    pub silenceThreshold: uint32_t,
    pub totalStreamSize: uint32_t,
    pub interleaved: uint32_t,
}
pub type AudioStreamType = core::ffi::c_uint;
pub const AUDIO_RENDER_STREAM: AudioStreamType = 1;
pub const AUDIO_CAPTURE_STREAM: AudioStreamType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CircleBufInfo {
    pub cirBufSize: uint32_t,
    pub trafBufSize: uint32_t,
    pub period: uint32_t,
    pub periodSize: uint32_t,
    pub periodCount: uint32_t,
    pub phyAddr: core::ffi::c_ulong,
    pub virtAddr: *mut uint32_t,
    pub wbufOffSet: uint32_t,
    pub wptrOffSet: uint32_t,
    pub rbufOffSet: uint32_t,
    pub rptrOffSet: uint32_t,
    pub runStatus: PcmStatus,
    pub chnId: uint32_t,
    pub enable: uint32_t,
    pub buffMutex: OsalMutex,
    pub framesPosition: uint32_t,
    pub pointer: uint32_t,
    pub periodsMax: uint32_t,
    pub periodsMin: uint32_t,
    pub cirBufMax: uint32_t,
    pub curTrafSize: uint32_t,
    pub oneMsBytes: uint32_t,
    pub trafCompCount: uint32_t,
}
pub type PcmStatus = core::ffi::c_uint;
pub const PCM_START: PcmStatus = 2;
pub const PCM_PAUSE: PcmStatus = 1;
pub const PCM_STOP: PcmStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioDmaOps {
    pub DmaBufAlloc: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaBufFree: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaRequestChannel: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaConfigChannel: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPrep: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaSubmit: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPending: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPause: Option<
        unsafe extern "C" fn(*mut PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaResume: Option<
        unsafe extern "C" fn(*const PlatformData, AudioStreamType) -> int32_t,
    >,
    pub DmaPointer: Option<
        unsafe extern "C" fn(
            *mut PlatformData,
            AudioStreamType,
            *mut uint32_t,
        ) -> int32_t,
    >,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const DMA_TRANSFER_MAX_COUNT: core::ffi::c_int = 12 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn AudioDmaBufAlloc(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaBufAlloc).is_some()
    {
        return ((*(*data).ops).DmaBufAlloc)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaBufFree(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaBufFree).is_some()
    {
        return ((*(*data).ops).DmaBufFree)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaRequestChannel(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaRequestChannel).is_some()
    {
        return ((*(*data).ops).DmaRequestChannel)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaConfigChannel(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaConfigChannel).is_some()
    {
        return ((*(*data).ops).DmaConfigChannel)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaPrep(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null() && ((*(*data).ops).DmaPrep).is_some()
    {
        return ((*(*data).ops).DmaPrep)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaSubmit(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaSubmit).is_some()
    {
        return ((*(*data).ops).DmaSubmit)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaPending(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaPending).is_some()
    {
        return ((*(*data).ops).DmaPending)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaPause(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null() && ((*(*data).ops).DmaPause).is_some()
    {
        return ((*(*data).ops).DmaPause)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaResume(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaResume).is_some()
    {
        return ((*(*data).ops).DmaResume)
            .expect("non-null function pointer")(data, streamType);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaPointer(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
    mut pointer: *mut uint32_t,
) -> int32_t {
    if !data.is_null() && !((*data).ops).is_null()
        && ((*(*data).ops).DmaPointer).is_some()
    {
        return ((*(*data).ops).DmaPointer)
            .expect("non-null function pointer")(data, streamType, pointer);
    }
    return HDF_FAILURE as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaGetConfigInfo(
    mut device: *const HdfDeviceObject,
    mut data: *mut PlatformData,
) -> int32_t {
    if device.is_null() || data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioDmaGetConfigInfo\0"))
                .as_ptr(),
            99 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if !((*data).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: regConfig has been parsed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioDmaGetConfigInfo\0"))
                .as_ptr(),
            104 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    (*data).regConfig = OsalMemCalloc(
        ::core::mem::size_of::<AudioRegCfgData>() as size_t,
    ) as *mut AudioRegCfgData;
    if ((*data).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc AudioRegCfgData fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioDmaGetConfigInfo\0"))
                .as_ptr(),
            110 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioGetRegConfig(device, (*data).regConfig) != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: dai GetRegConfig fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioDmaGetConfigInfo\0"))
                .as_ptr(),
            115 as core::ffi::c_int,
        );
        OsalMemFree((*data).regConfig as *mut core::ffi::c_void);
        (*data).regConfig = 0 as *mut AudioRegCfgData;
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDmaTransferStatusIsNormal(
    mut data: *mut PlatformData,
    mut streamType: AudioStreamType,
) -> bool {
    if data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"AudioDmaTransferStatusIsNormal\0"))
                .as_ptr(),
            127 as core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if streamType as core::ffi::c_uint
        == AUDIO_RENDER_STREAM as core::ffi::c_int as core::ffi::c_uint
    {
        if (*data).renderBufInfo.rbufOffSet == (*data).renderBufInfo.wbufOffSet {
            (*data).renderBufInfo.trafCompCount = ((*data).renderBufInfo.trafCompCount)
                .wrapping_add(1);
            if (*data).renderBufInfo.trafCompCount
                > DMA_TRANSFER_MAX_COUNT as core::ffi::c_uint
            {
                HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    LOG_DOMAIN as core::ffi::c_uint,
                    b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                    b"[%s][line:%d]: audio render send data to DMA too slow DMA will stop!\0"
                        as *const u8 as *const core::ffi::c_char,
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"AudioDmaTransferStatusIsNormal\0"))
                        .as_ptr(),
                    135 as core::ffi::c_int,
                );
                return false_0 != 0;
            }
        } else {
            (*data).renderBufInfo.rbufOffSet = (*data).renderBufInfo.wbufOffSet;
            (*data).renderBufInfo.trafCompCount = 0 as uint32_t;
        }
    } else if (*data).captureBufInfo.wbufOffSet == (*data).captureBufInfo.rbufOffSet {
        (*data).captureBufInfo.trafCompCount = ((*data).captureBufInfo.trafCompCount)
            .wrapping_add(1);
        if (*data).captureBufInfo.trafCompCount
            > DMA_TRANSFER_MAX_COUNT as core::ffi::c_uint
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: audio capture retrieve data from DMA too slow DMA will stop!\0"
                    as *const u8 as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 31],
                    [core::ffi::c_char; 31],
                >(*b"AudioDmaTransferStatusIsNormal\0"))
                    .as_ptr(),
                146 as core::ffi::c_int,
            );
            return false_0 != 0;
        }
    } else {
        (*data).captureBufInfo.wbufOffSet = (*data).captureBufInfo.rbufOffSet;
        (*data).captureBufInfo.trafCompCount = 0 as uint32_t;
    }
    return true_0 != 0;
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
