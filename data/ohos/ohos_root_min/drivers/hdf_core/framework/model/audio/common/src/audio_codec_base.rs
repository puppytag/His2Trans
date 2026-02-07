use core::arch::asm;
extern "C" {
    pub type HdfSBuf;
    fn DeviceResourceGetIfaceInstance(
        type_0: DeviceResourceType,
    ) -> *mut DeviceResourceIface;
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
    fn memset_s(
        dest: *mut core::ffi::c_void,
        destMax: size_t,
        c: core::ffi::c_int,
        count: size_t,
    ) -> errno_t;
    fn AudioCodecSapmSetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *const AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecSapmGetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecSapmSetEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *const AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecSapmGetEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioGetRegConfig(
        device: *const HdfDeviceObject,
        configData: *mut AudioRegCfgData,
    ) -> int32_t;
    fn AudioGetPortConfig(
        device: *const HdfDeviceObject,
        configData: *mut AudioPortInfo,
    ) -> int32_t;
    fn AudioDaiRegUpdate(
        dai: *const DaiDevice,
        mixerCtrl: *mut AudioMixerControl,
    ) -> int32_t;
    fn AudioInfoCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemInfo: *mut AudioCtrlElemInfo,
    ) -> int32_t;
    fn AudioCodecGetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecSetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *const AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioInfoEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemInfo: *mut AudioCtrlElemInfo,
    ) -> int32_t;
    fn AudioCodecGetEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCodecSetEnumCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *const AudioCtrlElemValue,
    ) -> int32_t;
    fn ioremap(
        phys_addr: core::ffi::c_ulong,
        size: core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
    fn OsalMSleep(ms: uint32_t);
    fn I2cOpen(number: int16_t) -> DevHandle;
    fn I2cClose(handle: DevHandle);
    fn I2cTransfer(handle: DevHandle, msgs: *mut I2cMsg, count: int16_t) -> int32_t;
}
pub type size_t = core::ffi::c_uint;
pub type uintptr_t = core::ffi::c_uint;
pub type int16_t = core::ffi::c_short;
pub type int32_t = core::ffi::c_int;
pub type uint8_t = core::ffi::c_uchar;
pub type uint16_t = core::ffi::c_ushort;
pub type uint32_t = core::ffi::c_uint;
pub type uint64_t = core::ffi::c_ulonglong;
pub type UINT32 = core::ffi::c_uint;
pub type UINTPTR = core::ffi::c_uint;
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
pub type DeviceResourceType = core::ffi::c_uint;
pub const INVALID: DeviceResourceType = 1;
pub const HDF_CONFIG_SOURCE: DeviceResourceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceResourceIface {
    pub GetRootNode: Option<unsafe extern "C" fn() -> *const DeviceResourceNode>,
    pub GetBool: Option<
        unsafe extern "C" fn(*const DeviceResourceNode, *const core::ffi::c_char) -> bool,
    >,
    pub GetUint8: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint8_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint8ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint8_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint8Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint8_t,
            uint32_t,
            uint8_t,
        ) -> int32_t,
    >,
    pub GetUint16: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint16_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint16ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint16_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint16Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint16_t,
            uint32_t,
            uint16_t,
        ) -> int32_t,
    >,
    pub GetUint32: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint32ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint32Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint32_t,
            uint32_t,
            uint32_t,
        ) -> int32_t,
    >,
    pub GetUint64: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint64_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetUint64ArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut uint64_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetUint64Array: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut uint64_t,
            uint32_t,
            uint64_t,
        ) -> int32_t,
    >,
    pub GetString: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            *mut *const core::ffi::c_char,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetStringArrayElem: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
            uint32_t,
            *mut *const core::ffi::c_char,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetElemNum: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> int32_t,
    >,
    pub GetNodeByMatchAttr: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
    pub GetChildNode: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
    pub GetNodeByRefAttr: Option<
        unsafe extern "C" fn(
            *const DeviceResourceNode,
            *const core::ffi::c_char,
        ) -> *const DeviceResourceNode,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
pub type errno_t = core::ffi::c_int;
pub type AudioFormat = core::ffi::c_uint;
pub const AUDIO_FORMAT_TYPE_G726: AudioFormat = 33554435;
pub const AUDIO_FORMAT_TYPE_G711U: AudioFormat = 33554434;
pub const AUDIO_FORMAT_TYPE_G711A: AudioFormat = 33554433;
pub const AUDIO_FORMAT_TYPE_AAC_HE_V2: AudioFormat = 16777222;
pub const AUDIO_FORMAT_TYPE_AAC_HE_V1: AudioFormat = 16777221;
pub const AUDIO_FORMAT_TYPE_AAC_ELD: AudioFormat = 16777220;
pub const AUDIO_FORMAT_TYPE_AAC_LD: AudioFormat = 16777219;
pub const AUDIO_FORMAT_TYPE_AAC_LC: AudioFormat = 16777218;
pub const AUDIO_FORMAT_TYPE_AAC_MAIN: AudioFormat = 16777217;
pub const AUDIO_FORMAT_TYPE_PCM_32_BIT: AudioFormat = 4;
pub const AUDIO_FORMAT_TYPE_PCM_24_BIT: AudioFormat = 3;
pub const AUDIO_FORMAT_TYPE_PCM_16_BIT: AudioFormat = 2;
pub const AUDIO_FORMAT_TYPE_PCM_8_BIT: AudioFormat = 1;
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
pub struct AudioSapmRoute {
    pub sink: *const core::ffi::c_char,
    pub control: *const core::ffi::c_char,
    pub source: *const core::ffi::c_char,
    pub Connected: Option<
        unsafe extern "C" fn(
            *mut AudioSapmComponent,
            *mut AudioSapmComponent,
        ) -> uint32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmComponent {
    pub sapmType: AudioSapmType,
    pub componentName: *mut core::ffi::c_char,
    pub streamName: *mut core::ffi::c_char,
    pub sapm: *mut AudioSapmContext,
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub reg: uint32_t,
    pub shift: uint8_t,
    pub invert: uint8_t,
    pub mask: uint32_t,
    pub connected: uint8_t,
    pub external: uint8_t,
    pub active: uint8_t,
    pub newPower: uint8_t,
    pub power: uint8_t,
    pub newCpt: uint8_t,
    pub eventFlags: uint16_t,
    pub Event: Option<
        unsafe extern "C" fn(
            *mut AudioSapmComponent,
            *mut AudioKcontrol,
            int32_t,
        ) -> int32_t,
    >,
    pub PowerCheck: Option<unsafe extern "C" fn(*const AudioSapmComponent) -> int32_t>,
    pub kcontrolsNum: int32_t,
    pub kcontrolNews: *mut AudioKcontrol,
    pub kcontrols: *mut *mut AudioKcontrol,
    pub list: DListHead,
    pub sources: DListHead,
    pub sinks: DListHead,
    pub powerList: DListHead,
    pub dirty: DListHead,
    pub PowerClockOp: Option<unsafe extern "C" fn(*mut AudioSapmComponent) -> int32_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioKcontrol {
    pub name: *mut core::ffi::c_char,
    pub iface: int32_t,
    pub Info: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *mut AudioCtrlElemInfo) -> int32_t,
    >,
    pub Get: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *mut AudioCtrlElemValue) -> int32_t,
    >,
    pub Set: Option<
        unsafe extern "C" fn(*const AudioKcontrol, *const AudioCtrlElemValue) -> int32_t,
    >,
    pub privateData: *mut core::ffi::c_void,
    pub pri: *mut core::ffi::c_void,
    pub privateValue: core::ffi::c_ulong,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemValue {
    pub id: AudioCtrlElemId,
    pub value: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemId {
    pub cardServiceName: *const core::ffi::c_char,
    pub iface: int32_t,
    pub itemName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCtrlElemInfo {
    pub id: AudioCtrlElemId,
    pub count: uint32_t,
    pub type_0: int32_t,
    pub min: int32_t,
    pub max: int32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodecDevice {
    pub devCodecName: *const core::ffi::c_char,
    pub devData: *mut CodecData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodecData {
    pub drvCodecName: *const core::ffi::c_char,
    pub Init: Option<
        unsafe extern "C" fn(*mut AudioCard, *const CodecDevice) -> int32_t,
    >,
    pub Read: Option<
        unsafe extern "C" fn(*const CodecDevice, uint32_t, *mut uint32_t) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(*const CodecDevice, uint32_t, uint32_t) -> int32_t,
    >,
    pub controls: *mut AudioKcontrol,
    pub numControls: core::ffi::c_int,
    pub sapmComponents: *mut AudioSapmComponent,
    pub numSapmComponent: core::ffi::c_int,
    pub sapmRoutes: *const AudioSapmRoute,
    pub numSapmRoutes: core::ffi::c_int,
    pub virtualAddress: core::ffi::c_ulong,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
    pub mutex: OsalMutex,
    pub privateParam: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSapmContext {
    pub componentNum: int32_t,
    pub biasLevel: AudioBiasLevel,
    pub suspendBiasLevel: AudioBiasLevel,
    pub codec: *mut CodecDevice,
    pub platform: *mut PlatformDevice,
    pub card: *mut AudioCard,
    pub targetBiasLevel: AudioBiasLevel,
    pub list: DListHead,
}
pub type AudioBiasLevel = core::ffi::c_uint;
pub const AUDIO_BIAS_ON: AudioBiasLevel = 3;
pub const AUDIO_BIAS_PREPARE: AudioBiasLevel = 2;
pub const AUDIO_BIAS_STANDBY: AudioBiasLevel = 1;
pub const AUDIO_BIAS_OFF: AudioBiasLevel = 0;
pub type AudioSapmType = core::ffi::c_uint;
pub const AUDIO_SAPM_SINK: AudioSapmType = 27;
pub const AUDIO_SAPM_SIGGEN: AudioSapmType = 26;
pub const AUDIO_SAPM_AIF_OUT: AudioSapmType = 25;
pub const AUDIO_SAPM_AIF_IN: AudioSapmType = 24;
pub const AUDIO_SAPM_CLOCK_SUPPLY: AudioSapmType = 23;
pub const AUDIO_SAPM_REGULATOR_SUPPLY: AudioSapmType = 22;
pub const AUDIO_SAPM_SUPPLY: AudioSapmType = 21;
pub const AUDIO_SAPM_POST: AudioSapmType = 20;
pub const AUDIO_SAPM_PRE: AudioSapmType = 19;
pub const AUDIO_SAPM_VMID: AudioSapmType = 18;
pub const AUDIO_SAPM_ANALOG_SWITCH: AudioSapmType = 17;
pub const AUDIO_SAPM_LINE: AudioSapmType = 16;
pub const AUDIO_SAPM_SPK: AudioSapmType = 15;
pub const AUDIO_SAPM_HP: AudioSapmType = 14;
pub const AUDIO_SAPM_MIC: AudioSapmType = 13;
pub const AUDIO_SAPM_MICBIAS: AudioSapmType = 12;
pub const AUDIO_SAPM_DAC: AudioSapmType = 11;
pub const AUDIO_SAPM_ADC: AudioSapmType = 10;
pub const AUDIO_SAPM_OUT_DRV: AudioSapmType = 9;
pub const AUDIO_SAPM_PGA: AudioSapmType = 8;
pub const AUDIO_SAPM_MIXER_NAMED_CTRL: AudioSapmType = 7;
pub const AUDIO_SAPM_MIXER: AudioSapmType = 6;
pub const AUDIO_SAPM_VALUE_MUX: AudioSapmType = 5;
pub const AUDIO_SAPM_VIRT_MUX: AudioSapmType = 4;
pub const AUDIO_SAPM_DEMUX: AudioSapmType = 3;
pub const AUDIO_SAPM_MUX: AudioSapmType = 2;
pub const AUDIO_SAPM_OUTPUT: AudioSapmType = 1;
pub const AUDIO_SAPM_INPUT: AudioSapmType = 0;
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
pub struct DaiDevice {
    pub devDaiName: *const core::ffi::c_char,
    pub devData: *mut DaiData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DaiData {
    pub drvDaiName: *const core::ffi::c_char,
    pub DaiInit: Option<
        unsafe extern "C" fn(*mut AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub Read: Option<
        unsafe extern "C" fn(*const DaiDevice, uint32_t, *mut uint32_t) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(*const DaiDevice, uint32_t, uint32_t) -> int32_t,
    >,
    pub ops: *const AudioDaiOps,
    pub portInfo: AudioPortInfo,
    pub pcmInfo: PcmInfo,
    pub controls: *mut AudioKcontrol,
    pub numControls: core::ffi::c_int,
    pub daiInitFlag: bool,
    pub regVirtualAddr: core::ffi::c_ulong,
    pub regConfig: *mut AudioRegCfgData,
    pub regCfgGroup: *mut *mut AudioRegCfgGroupNode,
    pub mutex: OsalMutex,
    pub privateParam: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPortInfo {
    pub render: AudioPcmStream,
    pub capture: AudioPcmStream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPcmStream {
    pub portDirection: uint64_t,
    pub formats: uint64_t,
    pub rates: uint64_t,
    pub rateMin: uint64_t,
    pub rateMax: uint64_t,
    pub channelsMin: uint64_t,
    pub channelsMax: uint64_t,
    pub bufferBytesMax: uint64_t,
    pub periodBytesMin: uint64_t,
    pub periodBytesMax: uint64_t,
    pub periodsMin: uint64_t,
    pub periodsMax: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioDaiOps {
    pub Startup: Option<
        unsafe extern "C" fn(*const AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub HwParams: Option<
        unsafe extern "C" fn(*const AudioCard, *const AudioPcmHwParams) -> int32_t,
    >,
    pub Trigger: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            core::ffi::c_int,
            *const DaiDevice,
        ) -> int32_t,
    >,
    pub Shutdown: Option<
        unsafe extern "C" fn(*const AudioCard, *const DaiDevice) -> int32_t,
    >,
    pub MuteStream: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const DaiDevice,
            bool,
            int32_t,
        ) -> int32_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPcmHwParams {
    pub streamType: AudioStreamType,
    pub channels: uint32_t,
    pub rate: uint32_t,
    pub periodSize: uint32_t,
    pub periodCount: uint32_t,
    pub format: AudioFormat,
    pub cardServiceName: *const core::ffi::c_char,
    pub period: uint32_t,
    pub frameSize: uint32_t,
    pub isBigEndian: bool,
    pub isSignedData: bool,
    pub startThreshold: uint32_t,
    pub stopThreshold: uint32_t,
    pub silenceThreshold: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DspDevice {
    pub devDspName: *const core::ffi::c_char,
    pub devData: *mut DspData,
    pub device: *mut HdfDeviceObject,
    pub list: DListHead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DspData {
    pub drvDspName: *const core::ffi::c_char,
    pub DspInit: Option<unsafe extern "C" fn(*const DspDevice) -> int32_t>,
    pub Read: Option<
        unsafe extern "C" fn(
            *const DspDevice,
            *const core::ffi::c_void,
            uint32_t,
        ) -> int32_t,
    >,
    pub Write: Option<
        unsafe extern "C" fn(
            *const DspDevice,
            *const core::ffi::c_void,
            uint32_t,
        ) -> int32_t,
    >,
    pub Decode: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
    pub Encode: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
    pub Equalizer: Option<
        unsafe extern "C" fn(
            *const AudioCard,
            *const uint8_t,
            *const DspDevice,
        ) -> int32_t,
    >,
}
pub type AudioControlType = core::ffi::c_uint;
pub const AUDIO_CONTROL_MUX: AudioControlType = 1;
pub const AUDIO_CONTROL_MIXER: AudioControlType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I2cTransferParam {
    pub i2cDevAddr: uint16_t,
    pub i2cBusNumber: uint16_t,
    pub i2cRegDataLen: uint16_t,
}
pub const I2C_FLAG_READ: I2cFlag = 1;
pub type DevHandle = *mut core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I2cMsg {
    pub addr: uint16_t,
    pub buf: *mut uint8_t,
    pub len: uint16_t,
    pub flags: uint16_t,
}
pub type I2cFlag = core::ffi::c_uint;
pub const I2C_FLAG_STOP: I2cFlag = 32768;
pub const I2C_FLAG_NO_START: I2cFlag = 16384;
pub const I2C_FLAG_IGNORE_NO_ACK: I2cFlag = 4096;
pub const I2C_FLAG_READ_NO_ACK: I2cFlag = 2048;
pub const I2C_FLAG_DMA: I2cFlag = 512;
pub const I2C_FLAG_ADDR_10BIT: I2cFlag = 16;
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn OsalIoRemap(
    mut phys_addr: core::ffi::c_ulong,
    mut size: core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    return ioremap(phys_addr, size);
}
pub const COMM_SHIFT_8BIT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const COMM_MASK_FF: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const COMM_WAIT_TIMES: core::ffi::c_int = 10 as core::ffi::c_int;
pub const I2C_MSG_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const I2C_MSG_BUF_SIZE_1: core::ffi::c_int = 1 as core::ffi::c_int;
pub const I2C_MSG_BUF_SIZE_2: core::ffi::c_int = 2 as core::ffi::c_int;
static mut g_audioSapmCompNameList: [*mut core::ffi::c_char; 100] = [
    b"ADCL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADCR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DACL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DACR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"LPGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"RPGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"LOUT\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"HPL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"HPR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Stereo Mixer\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Line Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Input Mixer\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Speaker Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Input Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AuxOut Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKL Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKR Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AUXOUTL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AUXOUTR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"LINEINL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"LINEINR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AUXINL\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AUXINR\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"I2S Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"AuxI Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"CaptureL Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"CaptureR Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Mono1 Mixer\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Mono2 Mixer\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC1\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC3\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC1\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC3\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC1\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC3\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPK1\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPK2\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPK3\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPK4\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC Mix\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC Mux\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKL PGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"SPKR PGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"HPL PGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"HPR PGA\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
];
static mut g_audioSapmCfgNameList: [*mut core::ffi::c_char; 100] = [
    b"LPGA MIC Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"RPGA MIC Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Dacl enable\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Dacr enable\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Headphone Playback Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"PCM Playback Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"PCM Capture Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mono Playback Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Phone Capture Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mic Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Stereo Mic Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Line HP Swap Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Surround Playback Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Center/LFE Playback Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Capture Source\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mic Boost Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"DAC1 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC2 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC3 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC4 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC1 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC2 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC3 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC4 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Speaker1 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker2 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker3 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker4 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Headphone1 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Headphone2 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout1 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout2 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout3 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout4 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mixer1 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mixer2 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mixer3 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mixer4 Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
];
static mut g_audioCodecControlsList: [*mut core::ffi::c_char; 100] = [
    b"Main Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Main Capture Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Playback Mute\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Capture Mute\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Mic Left Gain\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mic Right Gain\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"External Codec Enable\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Internally Codec Enable\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Render Channel Mode\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Captrue Channel Mode\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Headphone Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"PCM Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"PCM Capture Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mono Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Phone Capture Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Mic Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Surround Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Center/LFE Playback Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"DAC1 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC2 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC3 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAC4 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC1 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC2 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC3 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC4 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Speaker1 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker2 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker3 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Speaker4 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"MIC1 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC2 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC3 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC4 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"MIC1 Boost Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"MIC2 Boost Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"INA1 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"INB1 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"INA2 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"INB2 Volume\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"Lineout1 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout2 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout3 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Lineout4 Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Headphone Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Receiver Volume\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"EQ1 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"EQ2 Switch\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"DAI1 Filter Mode\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"DAI2 Filter Mode\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"ADC High Pass Filter Switch\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"Playback Deemphasis\0" as *const u8 as *const core::ffi::c_char
        as *mut core::ffi::c_char,
    b"PGA1 Setting\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"PGA2 Setting\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"PGA3 Setting\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"PGA3 Setting\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC1 Mute\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC2 Mute\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC3 Mute\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    b"ADC4 Mute\0" as *const u8 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
    0 as *const core::ffi::c_char as *mut core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn CodecGetServiceName(
    mut device: *const HdfDeviceObject,
    mut drvCodecName: *mut *const core::ffi::c_char,
) -> int32_t {
    let mut node: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    let mut ret: int32_t = 0;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input device para is nullptr.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecGetServiceName\0"))
                .as_ptr(),
            107 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    node = (*device).property as *const DeviceResourceNode;
    if node.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: node instance is nullptr.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecGetServiceName\0"))
                .as_ptr(),
            113 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    drsOps = DeviceResourceGetIfaceInstance(HDF_CONFIG_SOURCE);
    if drsOps.is_null() || ((*drsOps).GetString).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: from resource get drsOps fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecGetServiceName\0"))
                .as_ptr(),
            118 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"serviceName\0" as *const u8 as *const core::ffi::c_char,
        drvCodecName,
        0 as *const core::ffi::c_char,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: read codecServiceName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecGetServiceName\0"))
                .as_ptr(),
            124 as core::ffi::c_int,
        );
        return ret;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecGetDaiName(
    mut device: *const HdfDeviceObject,
    mut drvDaiName: *mut *const core::ffi::c_char,
) -> int32_t {
    let mut node: *const DeviceResourceNode = 0 as *const DeviceResourceNode;
    let mut drsOps: *mut DeviceResourceIface = 0 as *mut DeviceResourceIface;
    let mut ret: int32_t = 0;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecGetDaiName\0"))
                .as_ptr(),
            138 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    node = (*device).property as *const DeviceResourceNode;
    if node.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: drs node is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecGetDaiName\0"))
                .as_ptr(),
            144 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    drsOps = DeviceResourceGetIfaceInstance(HDF_CONFIG_SOURCE);
    if drsOps.is_null() || ((*drsOps).GetString).is_none() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: drs ops failed!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecGetDaiName\0"))
                .as_ptr(),
            149 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = ((*drsOps).GetString)
        .expect(
            "non-null function pointer",
        )(
        node,
        b"codecDaiName\0" as *const u8 as *const core::ffi::c_char,
        drvDaiName,
        0 as *const core::ffi::c_char,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: read codecDaiName fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecGetDaiName\0"))
                .as_ptr(),
            155 as core::ffi::c_int,
        );
        return ret;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecGetConfigInfo(
    mut device: *const HdfDeviceObject,
    mut codecData: *mut CodecData,
) -> int32_t {
    if device.is_null() || codecData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecGetConfigInfo\0"))
                .as_ptr(),
            165 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !((*codecData).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: g_codecData regConfig has been parsed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecGetConfigInfo\0"))
                .as_ptr(),
            170 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    (*codecData).regConfig = OsalMemCalloc(
        ::core::mem::size_of::<AudioRegCfgData>() as size_t,
    ) as *mut AudioRegCfgData as *mut AudioRegCfgData;
    if ((*codecData).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: malloc AudioRegCfgData fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecGetConfigInfo\0"))
                .as_ptr(),
            176 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioGetRegConfig(device, (*codecData).regConfig as *mut AudioRegCfgData)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioGetRegConfig fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecGetConfigInfo\0"))
                .as_ptr(),
            181 as core::ffi::c_int,
        );
        OsalMemFree((*codecData).regConfig as *mut core::ffi::c_void);
        (*codecData).regConfig = 0 as *mut AudioRegCfgData;
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDaiGetPortConfigInfo(
    mut device: *const HdfDeviceObject,
    mut codecData: *mut DaiData,
) -> int32_t {
    if device.is_null() || codecData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: <device> or <codecData> is a null pointer!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"CodecDaiGetPortConfigInfo\0"))
                .as_ptr(),
            193 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    return AudioGetPortConfig(device, &mut (*codecData).portInfo);
}
unsafe extern "C" fn SapmCtrlToSapmComp(
    mut sapmComponents: *mut AudioSapmComponent,
    mut sapmCompItem: *const AudioSapmCtrlConfig,
    mut index: uint16_t,
) -> int32_t {
    if sapmComponents.is_null() || sapmCompItem.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"SapmCtrlToSapmComp\0"))
                .as_ptr(),
            204 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    let ref mut fresh1 = (*sapmComponents.offset(index as isize)).componentName;
    *fresh1 = g_audioSapmCompNameList[(*sapmCompItem.offset(index as isize))
        .compNameIndex as usize];
    (*sapmComponents.offset(index as isize)).reg = (*sapmCompItem.offset(index as isize))
        .reg;
    (*sapmComponents.offset(index as isize)).sapmType = (*sapmCompItem
        .offset(index as isize))
        .sapmType as AudioSapmType;
    (*sapmComponents.offset(index as isize)).mask = (*sapmCompItem
        .offset(index as isize))
        .mask;
    (*sapmComponents.offset(index as isize)).shift = (*sapmCompItem
        .offset(index as isize))
        .shift;
    (*sapmComponents.offset(index as isize)).invert = (*sapmCompItem
        .offset(index as isize))
        .invert;
    (*sapmComponents.offset(index as isize)).kcontrolsNum = (*sapmCompItem
        .offset(index as isize))
        .kcontrolsNum as int32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn CodecSetSapmKcontrolInfo(
    mut audioSapmControls: *mut AudioKcontrol,
    mut regCfgGroup: *mut *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut index: uint16_t = 0;
    let mut sapmCtrlItem: *mut AudioControlConfig = 0 as *mut AudioControlConfig;
    let mut ctlSapmRegCfgItem: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut ctlRegEnumCfgItem: *mut AudioEnumCtrlConfig = 0 as *mut AudioEnumCtrlConfig;
    if audioSapmControls.is_null() || regCfgGroup.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecSetSapmKcontrolInfo\0"))
                .as_ptr(),
            229 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*regCfgGroup.offset(AUDIO_CTRL_SAPM_PATAM_GROUP as core::ffi::c_int as isize))
        .is_null()
        || (*regCfgGroup.offset(AUDIO_SAPM_CFG_GROUP as core::ffi::c_int as isize))
            .is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec config hcs configuration file is no configuration information for sapm\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecSetSapmKcontrolInfo\0"))
                .as_ptr(),
            233 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    sapmCtrlItem = (**regCfgGroup
        .offset(AUDIO_SAPM_CFG_GROUP as core::ffi::c_int as isize))
        .ctrlCfgItem;
    ctlSapmRegCfgItem = (**regCfgGroup
        .offset(AUDIO_CTRL_SAPM_PATAM_GROUP as core::ffi::c_int as isize))
        .regCfgItem;
    if sapmCtrlItem.is_null() || ctlSapmRegCfgItem.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: sapmCtrlItem, ctlSapmRegCfgItem is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecSetSapmKcontrolInfo\0"))
                .as_ptr(),
            240 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !(*regCfgGroup
        .offset(AUDIO_CTRL_SAPM_PATAM_MUX_GROUP as core::ffi::c_int as isize))
        .is_null()
    {
        ctlRegEnumCfgItem = (**regCfgGroup
            .offset(AUDIO_CTRL_SAPM_PATAM_MUX_GROUP as core::ffi::c_int as isize))
            .regEnumCfgItem;
        if ctlRegEnumCfgItem.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: ctlRegEnumCfgItem is NULL.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"CodecSetSapmKcontrolInfo\0"))
                    .as_ptr(),
                247 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    index = 0 as uint16_t;
    while (index as core::ffi::c_int)
        < (**regCfgGroup.offset(AUDIO_SAPM_CFG_GROUP as core::ffi::c_int as isize))
            .itemNum as core::ffi::c_int
    {
        if (*sapmCtrlItem.offset(index as isize)).type_0 as core::ffi::c_int
            == AUDIO_CONTROL_MIXER as core::ffi::c_int
        {
            (*audioSapmControls.offset(index as isize)).iface = (*sapmCtrlItem
                .offset(index as isize))
                .iface as int32_t;
            let ref mut fresh2 = (*audioSapmControls.offset(index as isize)).name;
            *fresh2 = g_audioSapmCfgNameList[(*sapmCtrlItem.offset(index as isize))
                .arrayIndex as usize];
            (*audioSapmControls.offset(index as isize)).privateValue = &mut *ctlSapmRegCfgItem
                .offset(index as isize) as *mut AudioMixerControl
                as *mut core::ffi::c_void as uintptr_t as core::ffi::c_ulong;
            let ref mut fresh3 = (*audioSapmControls.offset(index as isize)).Info;
            *fresh3 = Some(
                AudioInfoCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
                >;
            let ref mut fresh4 = (*audioSapmControls.offset(index as isize)).Get;
            *fresh4 = Some(
                AudioCodecSapmGetCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemValue,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemValue,
                    ) -> int32_t,
                >;
            let ref mut fresh5 = (*audioSapmControls.offset(index as isize)).Set;
            *fresh5 = Some(
                AudioCodecSapmSetCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *const AudioCtrlElemValue,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *const AudioCtrlElemValue,
                    ) -> int32_t,
                >;
        } else if (*sapmCtrlItem.offset(index as isize)).type_0 as core::ffi::c_int
            == AUDIO_CONTROL_MUX as core::ffi::c_int
        {
            (*audioSapmControls.offset(index as isize)).iface = (*sapmCtrlItem
                .offset(index as isize))
                .iface as int32_t;
            let ref mut fresh6 = (*audioSapmControls.offset(index as isize)).name;
            *fresh6 = g_audioSapmCfgNameList[(*sapmCtrlItem.offset(index as isize))
                .arrayIndex as usize];
            (*audioSapmControls.offset(index as isize)).privateValue = &mut *ctlRegEnumCfgItem
                .offset(index as isize) as *mut AudioEnumCtrlConfig
                as *mut core::ffi::c_void as uintptr_t as core::ffi::c_ulong;
            let ref mut fresh7 = (*audioSapmControls.offset(index as isize)).Info;
            *fresh7 = Some(
                AudioInfoEnumCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
                >;
            let ref mut fresh8 = (*audioSapmControls.offset(index as isize)).Get;
            *fresh8 = Some(
                AudioCodecSapmGetEnumCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemValue,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemValue,
                    ) -> int32_t,
                >;
            let ref mut fresh9 = (*audioSapmControls.offset(index as isize)).Set;
            *fresh9 = Some(
                AudioCodecSapmSetEnumCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *const AudioCtrlElemValue,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *const AudioCtrlElemValue,
                    ) -> int32_t,
                >;
        }
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn CodecSetSapmConfigInfo(
    mut codeData: *mut CodecData,
    mut regCfgGroup: *mut *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut index: uint16_t = 0;
    let mut sapmCompItem: *mut AudioSapmCtrlConfig = 0 as *mut AudioSapmCtrlConfig;
    let mut audioSapmControls: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    if codeData.is_null() || regCfgGroup.is_null() {
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*regCfgGroup.offset(AUDIO_SAPM_COMP_GROUP as core::ffi::c_int as isize))
        .is_null()
        || (*regCfgGroup.offset(AUDIO_SAPM_CFG_GROUP as core::ffi::c_int as isize))
            .is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec config hcs configuration file is no configuration information for sapm\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecSetSapmConfigInfo\0"))
                .as_ptr(),
            282 as core::ffi::c_int,
        );
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    sapmCompItem = (**regCfgGroup
        .offset(AUDIO_SAPM_COMP_GROUP as core::ffi::c_int as isize))
        .sapmCompItem;
    if sapmCompItem.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: sapmCompItem is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecSetSapmConfigInfo\0"))
                .as_ptr(),
            287 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    audioSapmControls = OsalMemCalloc(
        ((**regCfgGroup.offset(AUDIO_SAPM_CFG_GROUP as core::ffi::c_int as isize))
            .itemNum as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioKcontrol>() as size_t),
    ) as *mut AudioKcontrol;
    if audioSapmControls.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: OsalMemCalloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecSetSapmConfigInfo\0"))
                .as_ptr(),
            293 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if CodecSetSapmKcontrolInfo(audioSapmControls, regCfgGroup)
        != HDF_SUCCESS as core::ffi::c_int
    {
        OsalMemFree(audioSapmControls as *mut core::ffi::c_void);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*codeData).numSapmComponent = (**regCfgGroup
        .offset(AUDIO_SAPM_COMP_GROUP as core::ffi::c_int as isize))
        .itemNum as core::ffi::c_int;
    (*codeData).sapmComponents = OsalMemCalloc(
        ((*codeData).numSapmComponent as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioSapmComponent>() as size_t),
    ) as *mut AudioSapmComponent;
    if ((*codeData).sapmComponents).is_null() {
        OsalMemFree(audioSapmControls as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: OsalMemCalloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecSetSapmConfigInfo\0"))
                .as_ptr(),
            305 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    index = 0 as uint16_t;
    while (index as core::ffi::c_int) < (*codeData).numSapmComponent {
        if SapmCtrlToSapmComp((*codeData).sapmComponents, sapmCompItem, index) != 0 {
            OsalMemFree(audioSapmControls as *mut core::ffi::c_void);
            OsalMemFree((*codeData).sapmComponents as *mut core::ffi::c_void);
            (*codeData).sapmComponents = 0 as *mut AudioSapmComponent;
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        if (*sapmCompItem.offset(index as isize)).kcontrolsNum != 0 {
            let ref mut fresh0 = (*((*codeData).sapmComponents).offset(index as isize))
                .kcontrolNews;
            *fresh0 = &mut *audioSapmControls
                .offset(
                    ((*sapmCompItem.offset(index as isize)).kcontrolNews
                        as core::ffi::c_uint)
                        .wrapping_sub(1 as core::ffi::c_uint) as isize,
                ) as *mut AudioKcontrol;
        }
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn CodecSetKcontrolInfo(
    mut codeData: *mut CodecData,
    mut regCfgGroup: *mut *mut AudioRegCfgGroupNode,
) -> int32_t {
    let mut index: uint16_t = 0 as uint16_t;
    let mut enumIndex: uint16_t = 0 as uint16_t;
    let mut compItem: *mut AudioControlConfig = 0 as *mut AudioControlConfig;
    let mut ctlRegCfgItem: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut enumCtlRegCfgItem: *mut AudioEnumCtrlConfig = 0 as *mut AudioEnumCtrlConfig;
    if codeData.is_null() || regCfgGroup.is_null()
        || (*regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
            .is_null()
        || (*regCfgGroup.offset(AUDIO_CTRL_PATAM_GROUP as core::ffi::c_int as isize))
            .is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"CodecSetKcontrolInfo\0"))
                .as_ptr(),
            334 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    compItem = (**regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
        .ctrlCfgItem;
    ctlRegCfgItem = (**regCfgGroup
        .offset(AUDIO_CTRL_PATAM_GROUP as core::ffi::c_int as isize))
        .regCfgItem;
    if compItem.is_null() || ctlRegCfgItem.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: compItem or ctlRegCfgItem is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"CodecSetKcontrolInfo\0"))
                .as_ptr(),
            341 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !(*regCfgGroup.offset(AUDIO_CTRL_PATAM_MUX_GROUP as core::ffi::c_int as isize))
        .is_null()
    {
        enumCtlRegCfgItem = (**regCfgGroup
            .offset(AUDIO_CTRL_PATAM_MUX_GROUP as core::ffi::c_int as isize))
            .regEnumCfgItem;
        if enumCtlRegCfgItem.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: enumCtlRegCfgItem is NULL.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"CodecSetKcontrolInfo\0"))
                    .as_ptr(),
                348 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    index = 0 as uint16_t;
    while (index as core::ffi::c_int) < (*codeData).numControls {
        if (*compItem.offset(index as isize)).type_0 as core::ffi::c_int
            == AUDIO_CONTROL_MIXER as core::ffi::c_int
        {
            (*((*codeData).controls).offset(index as isize)).iface = (*compItem
                .offset(index as isize))
                .iface as int32_t;
            let ref mut fresh10 = (*((*codeData).controls).offset(index as isize)).name;
            *fresh10 = g_audioCodecControlsList[(*compItem.offset(index as isize))
                .arrayIndex as usize];
            let ref mut fresh11 = (*((*codeData).controls).offset(index as isize)).Info;
            *fresh11 = Some(
                AudioInfoCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
                >;
            (*((*codeData).controls).offset(index as isize)).privateValue = &mut *ctlRegCfgItem
                .offset(index as isize) as *mut AudioMixerControl
                as *mut core::ffi::c_void as uintptr_t as core::ffi::c_ulong;
            if (*compItem.offset(index as isize)).enable != 0 {
                let ref mut fresh12 = (*((*codeData).controls).offset(index as isize))
                    .Get;
                *fresh12 = Some(
                    AudioCodecGetCtrlOps
                        as unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *mut AudioCtrlElemValue,
                        ) -> int32_t,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *mut AudioCtrlElemValue,
                        ) -> int32_t,
                    >;
                let ref mut fresh13 = (*((*codeData).controls).offset(index as isize))
                    .Set;
                *fresh13 = Some(
                    AudioCodecSetCtrlOps
                        as unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *const AudioCtrlElemValue,
                        ) -> int32_t,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *const AudioCtrlElemValue,
                        ) -> int32_t,
                    >;
            }
        } else if (*compItem.offset(index as isize)).type_0 as core::ffi::c_int
            == AUDIO_CONTROL_MUX as core::ffi::c_int
        {
            (*((*codeData).controls).offset(index as isize)).iface = (*compItem
                .offset(index as isize))
                .iface as int32_t;
            let ref mut fresh14 = (*((*codeData).controls).offset(index as isize)).name;
            *fresh14 = g_audioCodecControlsList[(*compItem.offset(index as isize))
                .arrayIndex as usize];
            let ref mut fresh15 = (*((*codeData).controls).offset(index as isize)).Info;
            *fresh15 = Some(
                AudioInfoEnumCtrlOps
                    as unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const AudioKcontrol,
                        *mut AudioCtrlElemInfo,
                    ) -> int32_t,
                >;
            let fresh16 = enumIndex;
            enumIndex = enumIndex.wrapping_add(1);
            (*((*codeData).controls).offset(index as isize)).privateValue = &mut *enumCtlRegCfgItem
                .offset(fresh16 as isize) as *mut AudioEnumCtrlConfig
                as *mut core::ffi::c_void as uintptr_t as core::ffi::c_ulong;
            if (*compItem.offset(index as isize)).enable != 0 {
                let ref mut fresh17 = (*((*codeData).controls).offset(index as isize))
                    .Get;
                *fresh17 = Some(
                    AudioCodecGetEnumCtrlOps
                        as unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *mut AudioCtrlElemValue,
                        ) -> int32_t,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *mut AudioCtrlElemValue,
                        ) -> int32_t,
                    >;
                let ref mut fresh18 = (*((*codeData).controls).offset(index as isize))
                    .Set;
                *fresh18 = Some(
                    AudioCodecSetEnumCtrlOps
                        as unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *const AudioCtrlElemValue,
                        ) -> int32_t,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *const AudioCtrlElemValue,
                        ) -> int32_t,
                    >;
            }
        }
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecSetConfigInfoOfControls(
    mut codeData: *mut CodecData,
    mut daiData: *mut DaiData,
) -> int32_t {
    let mut audioIdInfo: *mut AudioIdInfo = 0 as *mut AudioIdInfo;
    let mut regCfgGroup: *mut *mut AudioRegCfgGroupNode = 0
        as *mut *mut AudioRegCfgGroupNode;
    if codeData.is_null() || daiData.is_null() || ((*codeData).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"CodecSetConfigInfoOfControls\0"))
                .as_ptr(),
            383 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    audioIdInfo = &mut (*(*codeData).regConfig).audioIdInfo;
    regCfgGroup = ((*(*codeData).regConfig).audioRegParams).as_mut_ptr();
    if audioIdInfo.is_null() || regCfgGroup.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: audioIdInfo or regCfgGroup is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"CodecSetConfigInfoOfControls\0"))
                .as_ptr(),
            390 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*daiData).regCfgGroup = regCfgGroup;
    (*codeData).regCfgGroup = regCfgGroup as *mut *mut AudioRegCfgGroupNode;
    if (*regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize)).is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: compItem is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"CodecSetConfigInfoOfControls\0"))
                .as_ptr(),
            397 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*codeData).numControls = (**regCfgGroup
        .offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
        .itemNum as core::ffi::c_int;
    (*codeData).controls = OsalMemCalloc(
        ((*codeData).numControls as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioKcontrol>() as size_t),
    ) as *mut AudioKcontrol;
    if ((*codeData).controls).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: OsalMemCalloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"CodecSetConfigInfoOfControls\0"))
                .as_ptr(),
            405 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if CodecSetKcontrolInfo(codeData, regCfgGroup) != HDF_SUCCESS as core::ffi::c_int {
        OsalMemFree((*codeData).controls as *mut core::ffi::c_void);
        (*codeData).controls = 0 as *mut AudioKcontrol;
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*codeData).virtualAddress = OsalIoRemap(
        (*audioIdInfo).chipIdRegister as core::ffi::c_ulong,
        (*audioIdInfo).chipIdSize as core::ffi::c_ulong,
    ) as uintptr_t as core::ffi::c_ulong;
    if CodecSetSapmConfigInfo(codeData, regCfgGroup) != HDF_SUCCESS as core::ffi::c_int {
        OsalMemFree((*codeData).controls as *mut core::ffi::c_void);
        (*codeData).controls = 0 as *mut AudioKcontrol;
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecSetCtlFunc(
    mut codeData: *mut CodecData,
    mut controlType: AudioControlType,
    mut getCtrl: *const core::ffi::c_void,
    mut setCtrl: *const core::ffi::c_void,
) -> int32_t {
    let mut index: uint32_t = 0;
    let mut regCfgGroup: *mut *mut AudioRegCfgGroupNode = 0
        as *mut *mut AudioRegCfgGroupNode;
    let mut compItem: *mut AudioControlConfig = 0 as *mut AudioControlConfig;
    if codeData.is_null() || ((*codeData).regConfig).is_null() || getCtrl.is_null()
        || setCtrl.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecSetCtlFunc\0"))
                .as_ptr(),
            433 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regCfgGroup = ((*(*codeData).regConfig).audioRegParams).as_mut_ptr();
    if regCfgGroup.is_null()
        || (*regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
            .is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: regCfgGroup or regCfgGroup[AUDIO_CTRL_CFG_GROUP] is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecSetCtlFunc\0"))
                .as_ptr(),
            438 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    compItem = (**regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
        .ctrlCfgItem;
    if compItem.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: compItem is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecSetCtlFunc\0"))
                .as_ptr(),
            444 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    index = 0 as uint32_t;
    while index < (*codeData).numControls as core::ffi::c_uint {
        if (*compItem.offset(index as isize)).type_0 as core::ffi::c_uint
            == controlType as core::ffi::c_uint
        {
            if (*compItem.offset(index as isize)).enable == 0 {
                let ref mut fresh19 = (*((*codeData).controls).offset(index as isize))
                    .Get;
                *fresh19 = ::core::mem::transmute::<
                    *const core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *mut AudioCtrlElemValue,
                        ) -> int32_t,
                    >,
                >(getCtrl);
                let ref mut fresh20 = (*((*codeData).controls).offset(index as isize))
                    .Set;
                *fresh20 = ::core::mem::transmute::<
                    *const core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            *const AudioKcontrol,
                            *const AudioCtrlElemValue,
                        ) -> int32_t,
                    >,
                >(setCtrl);
            }
        }
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn CodecI2cRelease(
    mut msgs: *mut I2cMsg,
    mut msgSize: int16_t,
    mut i2cHandle: DevHandle,
) {
    if !msgs.is_null() {
        if msgSize as core::ffi::c_int == 0 as core::ffi::c_int
            && !((*msgs).buf).is_null()
        {
            OsalMemFree((*msgs).buf as *mut core::ffi::c_void);
            (*msgs).buf = 0 as *mut uint8_t;
        } else if msgSize as core::ffi::c_int == 1 as core::ffi::c_int
            && !((*msgs.offset(0 as core::ffi::c_int as isize)).buf).is_null()
        {
            OsalMemFree(
                (*msgs.offset(0 as core::ffi::c_int as isize)).buf
                    as *mut core::ffi::c_void,
            );
            let ref mut fresh21 = (*msgs.offset(0 as core::ffi::c_int as isize)).buf;
            *fresh21 = 0 as *mut uint8_t;
        } else if msgSize as core::ffi::c_int >= I2C_MSG_NUM {
            if !((*msgs.offset(0 as core::ffi::c_int as isize)).buf).is_null() {
                OsalMemFree(
                    (*msgs.offset(0 as core::ffi::c_int as isize)).buf
                        as *mut core::ffi::c_void,
                );
                let ref mut fresh22 = (*msgs.offset(0 as core::ffi::c_int as isize)).buf;
                *fresh22 = 0 as *mut uint8_t;
            }
            if !((*msgs.offset(1 as core::ffi::c_int as isize)).buf).is_null() {
                OsalMemFree(
                    (*msgs.offset(1 as core::ffi::c_int as isize)).buf
                        as *mut core::ffi::c_void,
                );
                let ref mut fresh23 = (*msgs.offset(1 as core::ffi::c_int as isize)).buf;
                *fresh23 = 0 as *mut uint8_t;
            }
        }
    }
    if !i2cHandle.is_null() {
        I2cClose(i2cHandle);
        i2cHandle = NULL as DevHandle;
    }
}
unsafe extern "C" fn CodecI2cMsgFill(
    mut i2cTransferParam: *mut I2cTransferParam,
    mut regAttr: *const AudioAddrConfig,
    mut rwFlag: uint16_t,
    mut regs: *mut uint8_t,
    mut msgs: *mut I2cMsg,
) -> int32_t {
    let mut msgBuf: *mut uint8_t = 0 as *mut uint8_t;
    if i2cTransferParam.is_null() || regAttr.is_null() || regs.is_null()
        || msgs.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input invalid parameter.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecI2cMsgFill\0"))
                .as_ptr(),
            496 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    if rwFlag as core::ffi::c_int != 0 as core::ffi::c_int
        && rwFlag as core::ffi::c_int != I2C_FLAG_READ as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: invalid rwFlag value: %d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"CodecI2cMsgFill\0"))
                .as_ptr(),
            501 as core::ffi::c_int,
            rwFlag as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    *regs.offset(0 as core::ffi::c_int as isize) = (*regAttr).addr as uint8_t;
    (*msgs.offset(0 as core::ffi::c_int as isize)).addr = (*i2cTransferParam).i2cDevAddr;
    (*msgs.offset(0 as core::ffi::c_int as isize)).flags = 0 as uint16_t;
    (*msgs.offset(0 as core::ffi::c_int as isize)).len = ((*i2cTransferParam)
        .i2cRegDataLen as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t;
    if rwFlag as core::ffi::c_int == 0 as core::ffi::c_int {
        msgBuf = OsalMemCalloc(
            ((*i2cTransferParam).i2cRegDataLen as core::ffi::c_int
                + 1 as core::ffi::c_int) as size_t,
        ) as *mut uint8_t;
        if msgBuf.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: [write]: malloc buf failed!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"CodecI2cMsgFill\0"))
                    .as_ptr(),
                514 as core::ffi::c_int,
            );
            return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
        }
        *msgBuf.offset(0 as core::ffi::c_int as isize) = *regs
            .offset(0 as core::ffi::c_int as isize);
        if (*i2cTransferParam).i2cRegDataLen as core::ffi::c_int == I2C_MSG_BUF_SIZE_1 {
            *msgBuf.offset(1 as core::ffi::c_int as isize) = (*regAttr).value as uint8_t;
        } else if (*i2cTransferParam).i2cRegDataLen as core::ffi::c_int
            == I2C_MSG_BUF_SIZE_2
        {
            *msgBuf.offset(1 as core::ffi::c_int as isize) = ((*regAttr).value
                >> COMM_SHIFT_8BIT) as uint8_t;
            *msgBuf.offset(I2C_MSG_BUF_SIZE_2 as isize) = ((*regAttr).value
                as core::ffi::c_uint & COMM_MASK_FF as core::ffi::c_uint) as uint8_t;
        } else {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: i2cRegDataLen is invalid\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"CodecI2cMsgFill\0"))
                    .as_ptr(),
                524 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        let ref mut fresh24 = (*msgs.offset(0 as core::ffi::c_int as isize)).buf;
        *fresh24 = msgBuf;
    } else {
        msgBuf = OsalMemCalloc((*i2cTransferParam).i2cRegDataLen as size_t)
            as *mut uint8_t;
        if msgBuf.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: [read]: malloc buf failed!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"CodecI2cMsgFill\0"))
                    .as_ptr(),
                532 as core::ffi::c_int,
            );
            return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
        }
        (*msgs.offset(0 as core::ffi::c_int as isize)).len = 1 as uint16_t;
        let ref mut fresh25 = (*msgs.offset(0 as core::ffi::c_int as isize)).buf;
        *fresh25 = regs;
        (*msgs.offset(1 as core::ffi::c_int as isize)).addr = (*i2cTransferParam)
            .i2cDevAddr;
        (*msgs.offset(1 as core::ffi::c_int as isize)).flags = I2C_FLAG_READ
            as core::ffi::c_int as uint16_t;
        (*msgs.offset(1 as core::ffi::c_int as isize)).len = (*i2cTransferParam)
            .i2cRegDataLen;
        let ref mut fresh26 = (*msgs.offset(1 as core::ffi::c_int as isize)).buf;
        *fresh26 = msgBuf;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn CodecI2cTransfer(
    mut i2cTransferParam: *mut I2cTransferParam,
    mut regAttr: *mut AudioAddrConfig,
    mut rwFlag: uint16_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut i2cHandle: DevHandle = 0 as *mut core::ffi::c_void;
    let mut transferMsgCount: int16_t = 1 as int16_t;
    let mut regs: [uint8_t; 1] = [0; 1];
    let mut msgs: [I2cMsg; 2] = [I2cMsg {
        addr: 0,
        buf: 0 as *mut uint8_t,
        len: 0,
        flags: 0,
    }; 2];
    memset_s(
        msgs.as_mut_ptr() as *mut core::ffi::c_void,
        (::core::mem::size_of::<I2cMsg>() as size_t).wrapping_mul(I2C_MSG_NUM as size_t),
        0 as core::ffi::c_int,
        (::core::mem::size_of::<I2cMsg>() as size_t).wrapping_mul(I2C_MSG_NUM as size_t),
    );
    if i2cTransferParam.is_null() || regAttr.is_null()
        || rwFlag as core::ffi::c_int > 1 as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: invalid parameter.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"CodecI2cTransfer\0"))
                .as_ptr(),
            558 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_PARAM as core::ffi::c_int as int32_t;
    }
    i2cHandle = I2cOpen((*i2cTransferParam).i2cBusNumber as int16_t);
    if i2cHandle.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: open i2cBus:%u failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"CodecI2cTransfer\0"))
                .as_ptr(),
            563 as core::ffi::c_int,
            (*i2cTransferParam).i2cBusNumber as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if rwFlag as core::ffi::c_int == I2C_FLAG_READ as core::ffi::c_int {
        transferMsgCount = I2C_MSG_NUM as int16_t;
    }
    ret = CodecI2cMsgFill(
        i2cTransferParam,
        regAttr,
        rwFlag,
        regs.as_mut_ptr(),
        msgs.as_mut_ptr(),
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CodecI2cMsgFill failed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"CodecI2cTransfer\0"))
                .as_ptr(),
            571 as core::ffi::c_int,
        );
        I2cClose(i2cHandle);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    ret = I2cTransfer(i2cHandle, msgs.as_mut_ptr(), transferMsgCount);
    if ret != transferMsgCount as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: I2cTransfer err:%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"CodecI2cTransfer\0"))
                .as_ptr(),
            577 as core::ffi::c_int,
            ret,
        );
        CodecI2cRelease(msgs.as_mut_ptr(), transferMsgCount, i2cHandle);
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if rwFlag as core::ffi::c_int == I2C_FLAG_READ as core::ffi::c_int {
        if (*i2cTransferParam).i2cRegDataLen as core::ffi::c_int == I2C_MSG_BUF_SIZE_1 {
            (*regAttr).value = *(msgs[1 as core::ffi::c_int as usize].buf)
                .offset(0 as core::ffi::c_int as isize) as uint32_t;
        } else if (*i2cTransferParam).i2cRegDataLen as core::ffi::c_int
            == I2C_MSG_BUF_SIZE_2
        {
            (*regAttr).value = ((*(msgs[1 as core::ffi::c_int as usize].buf)
                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                << COMM_SHIFT_8BIT
                | *(msgs[1 as core::ffi::c_int as usize].buf)
                    .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
                as uint32_t;
        } else {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: i2cRegDataLen is invalid\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"CodecI2cTransfer\0"))
                    .as_ptr(),
                587 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    CodecI2cRelease(msgs.as_mut_ptr(), transferMsgCount, i2cHandle);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDeviceRegI2cRead(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut val: *mut uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut regAttr: AudioAddrConfig = AudioAddrConfig {
        addr: 0,
        value: 0,
    };
    let mut i2cTransferParam: *mut I2cTransferParam = 0 as *mut I2cTransferParam;
    if codec.is_null() || ((*codec).devData).is_null() || val.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"CodecDeviceRegI2cRead\0"))
                .as_ptr(),
            604 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    i2cTransferParam = (*(*codec).devData).privateParam as *mut I2cTransferParam;
    if i2cTransferParam.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec i2cTransferParam is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"CodecDeviceRegI2cRead\0"))
                .as_ptr(),
            610 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regAttr.addr = reg as uint8_t as uint32_t;
    regAttr.value = 0 as uint32_t;
    ret = CodecI2cTransfer(
        i2cTransferParam,
        &mut regAttr,
        I2C_FLAG_READ as core::ffi::c_int as uint16_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: failed.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"CodecDeviceRegI2cRead\0"))
                .as_ptr(),
            618 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *val = regAttr.value;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDeviceRegI2cWrite(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut regAttr: AudioAddrConfig = AudioAddrConfig {
        addr: 0,
        value: 0,
    };
    let mut i2cTransferParam: *mut I2cTransferParam = 0 as *mut I2cTransferParam;
    if codec.is_null() || ((*codec).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecDeviceRegI2cWrite\0"))
                .as_ptr(),
            632 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i2cTransferParam = (*(*codec).devData).privateParam as *mut I2cTransferParam;
    if i2cTransferParam.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec i2cTransferParam is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecDeviceRegI2cWrite\0"))
                .as_ptr(),
            638 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regAttr.addr = reg as uint8_t as uint32_t;
    regAttr.value = value as uint16_t as uint32_t;
    ret = CodecI2cTransfer(i2cTransferParam, &mut regAttr, 0 as uint16_t);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: I2c Transfer failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"CodecDeviceRegI2cWrite\0"))
                .as_ptr(),
            646 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDaiRegI2cRead(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut value: *mut uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut regAttr: AudioAddrConfig = AudioAddrConfig {
        addr: 0,
        value: 0,
    };
    let mut i2cTransferParam: *mut I2cTransferParam = 0 as *mut I2cTransferParam;
    if dai.is_null() || ((*dai).devData).is_null() || value.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecDaiRegI2cRead\0"))
                .as_ptr(),
            660 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    i2cTransferParam = (*(*dai).devData).privateParam as *mut I2cTransferParam;
    if i2cTransferParam.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec dai i2cTransferParam is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecDaiRegI2cRead\0"))
                .as_ptr(),
            666 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regAttr.addr = reg as uint8_t as uint32_t;
    regAttr.value = 0 as uint32_t;
    ret = CodecI2cTransfer(
        i2cTransferParam,
        &mut regAttr,
        I2C_FLAG_READ as core::ffi::c_int as uint16_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CodecI2cTransfer failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecDaiRegI2cRead\0"))
                .as_ptr(),
            674 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    *value = regAttr.value;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDaiRegI2cWrite(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut regAttr: AudioAddrConfig = AudioAddrConfig {
        addr: 0,
        value: 0,
    };
    let mut i2cTransferParam: *mut I2cTransferParam = 0 as *mut I2cTransferParam;
    if dai.is_null() || ((*dai).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecDaiRegI2cWrite\0"))
                .as_ptr(),
            688 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i2cTransferParam = (*(*dai).devData).privateParam as *mut I2cTransferParam;
    if i2cTransferParam.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec dai i2cTransferParam is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecDaiRegI2cWrite\0"))
                .as_ptr(),
            694 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regAttr.addr = reg as uint8_t as uint32_t;
    regAttr.value = value as uint16_t as uint32_t;
    ret = CodecI2cTransfer(i2cTransferParam, &mut regAttr, 0 as uint16_t);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: codec I2c Transfer failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecDaiRegI2cWrite\0"))
                .as_ptr(),
            702 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDeviceReadReg(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut val: *mut uint32_t,
) -> int32_t {
    let mut virtualAddress: core::ffi::c_ulong = 0;
    if codec.is_null() || ((*codec).devData).is_null() || val.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"CodecDeviceReadReg\0"))
                .as_ptr(),
            713 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    virtualAddress = (*(*codec).devData).virtualAddress;
    *val = ({
        let mut r: UINT32 = *(virtualAddress.wrapping_add(reg as core::ffi::c_ulong)
            as uintptr_t as *mut core::ffi::c_void as UINTPTR as *mut UINT32);
        asm!("dsb\n", options(preserves_flags));
        r
    }) as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDeviceWriteReg(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut virtualAddress: core::ffi::c_ulong = 0;
    if codec.is_null() || ((*codec).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"CodecDeviceWriteReg\0"))
                .as_ptr(),
            725 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    virtualAddress = (*(*codec).devData).virtualAddress;
    ({
        asm!("dsb\n", options(preserves_flags));
        ::core::ptr::write_volatile(
            (virtualAddress.wrapping_add(reg as core::ffi::c_ulong) as uintptr_t
                as *mut core::ffi::c_void as UINTPTR as *mut UINT32),
            value as UINT32,
        );
        compile_error!("Volatile value is not supposed to be read")
    });
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDeviceInitRegConfig(
    mut device: *const CodecDevice,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut index: uint32_t = 0;
    let mut initCfg: *mut AudioAddrConfig = 0 as *mut AudioAddrConfig;
    let mut regCfgGroup: *mut *mut AudioRegCfgGroupNode = 0
        as *mut *mut AudioRegCfgGroupNode;
    if device.is_null() || ((*device).devData).is_null()
        || ((*(*device).devData).Write).is_none()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecDeviceInitRegConfig\0"))
                .as_ptr(),
            741 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regCfgGroup = (*(*device).devData).regCfgGroup as *mut *mut AudioRegCfgGroupNode;
    if regCfgGroup.is_null()
        || (*regCfgGroup.offset(AUDIO_INIT_GROUP as core::ffi::c_int as isize)).is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: regCfgGroup init group is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecDeviceInitRegConfig\0"))
                .as_ptr(),
            747 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    initCfg = (**regCfgGroup.offset(AUDIO_INIT_GROUP as core::ffi::c_int as isize))
        .addrCfgItem;
    if initCfg.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: initCfg is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"CodecDeviceInitRegConfig\0"))
                .as_ptr(),
            753 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    index = 0 as uint32_t;
    while index
        < (**regCfgGroup.offset(AUDIO_INIT_GROUP as core::ffi::c_int as isize)).itemNum
            as core::ffi::c_uint
    {
        ret = ((*(*device).devData).Write)
            .expect(
                "non-null function pointer",
            )(
            device,
            (*initCfg.offset(index as isize)).addr,
            (*initCfg.offset(index as isize)).value,
        );
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Write err regAddr: 0x%x.\n\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 25],
                    [core::ffi::c_char; 25],
                >(*b"CodecDeviceInitRegConfig\0"))
                    .as_ptr(),
                760 as core::ffi::c_int,
                (*initCfg.offset(index as isize)).addr,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        OsalMSleep(COMM_WAIT_TIMES as uint32_t);
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn CodecDaiDeviceStartupRegConfig(
    mut device: *const DaiDevice,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut index: uint16_t = 0;
    let mut startupRegCfgItem: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut regCfgItemCount: uint16_t = 0;
    let mut regCfgGroup: *mut *mut AudioRegCfgGroupNode = 0
        as *mut *mut AudioRegCfgGroupNode;
    if device.is_null() || ((*device).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"CodecDaiDeviceStartupRegConfig\0"))
                .as_ptr(),
            778 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regCfgGroup = (*(*device).devData).regCfgGroup;
    if regCfgGroup.is_null()
        || (*regCfgGroup
            .offset(AUDIO_DAI_STARTUP_PATAM_GROUP as core::ffi::c_int as isize))
            .is_null()
        || ((**regCfgGroup
            .offset(AUDIO_DAI_STARTUP_PATAM_GROUP as core::ffi::c_int as isize))
            .regCfgItem)
            .is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: regCfgGroup is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"CodecDaiDeviceStartupRegConfig\0"))
                .as_ptr(),
            785 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    startupRegCfgItem = (**regCfgGroup
        .offset(AUDIO_DAI_STARTUP_PATAM_GROUP as core::ffi::c_int as isize))
        .regCfgItem;
    regCfgItemCount = (**regCfgGroup
        .offset(AUDIO_DAI_STARTUP_PATAM_GROUP as core::ffi::c_int as isize))
        .itemNum as uint16_t;
    index = 0 as uint16_t;
    while (index as core::ffi::c_int) < regCfgItemCount as core::ffi::c_int {
        ret = AudioDaiRegUpdate(device, &mut *startupRegCfgItem.offset(index as isize));
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: CodecDaiRegBitsUpdate fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 31],
                    [core::ffi::c_char; 31],
                >(*b"CodecDaiDeviceStartupRegConfig\0"))
                    .as_ptr(),
                794 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
