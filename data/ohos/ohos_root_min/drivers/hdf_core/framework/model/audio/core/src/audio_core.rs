extern "C" {
    pub type HdfSBuf;
    fn strcmp(
        _: *const core::ffi::c_char,
        _: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
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
    fn OsalMutexLock(mutex: *mut OsalMutex) -> int32_t;
    fn OsalMutexUnlock(mutex: *mut OsalMutex) -> int32_t;
    fn iounmap(addr: *mut core::ffi::c_void);
}
pub type size_t = core::ffi::c_uint;
pub type uintptr_t = core::ffi::c_uint;
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
pub struct AudioCtrlElemValue {
    pub id: AudioCtrlElemId,
    pub value: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioEnumKcontrol {
    pub reg: uint32_t,
    pub reg2: uint32_t,
    pub shiftLeft: uint8_t,
    pub shiftRight: uint8_t,
    pub max: uint32_t,
    pub mask: uint32_t,
    pub texts: *const *const core::ffi::c_char,
    pub values: *const uint32_t,
    pub sapm: *mut core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OsalMutex {
    pub realMutex: *mut core::ffi::c_void,
}
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
pub const AUDIO_CTL_ELEM_TYPE_INTEGER: C2RustUnnamed_0 = 2;
pub const AUDIO_CTL_ELEM_TYPE_ENUMERATED: C2RustUnnamed_0 = 3;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AUDIO_CTL_ELEM_TYPE_LAST: C2RustUnnamed_0 = 4;
pub const AUDIO_CTL_ELEM_TYPE_BYTES: C2RustUnnamed_0 = 4;
pub const AUDIO_CTL_ELEM_TYPE_BOOLEAN: C2RustUnnamed_0 = 1;
pub const AUDIO_CTL_ELEM_TYPE_NONE: C2RustUnnamed_0 = 0;
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn DListHeadInit(mut head: *mut DListHead) {
    (*head).next = head;
    (*head).prev = head;
}
#[inline]
unsafe extern "C" fn DListIsEmpty(mut head: *const DListHead) -> bool {
    return if (*head).next == head as *mut DListHead { true_0 } else { false_0 } != 0;
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
pub const CHANNEL_MAX_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const CHANNEL_MIN_NUM: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AUDIO_DAI_LINK_COMPLETE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AUDIO_DAI_LINK_UNCOMPLETE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub static mut daiController: DListHead = unsafe {
    {
        let mut init = DListHead {
            next: &daiController as *const DListHead as *mut DListHead,
            prev: &daiController as *const DListHead as *mut DListHead,
        };
        init
    }
};
#[no_mangle]
pub static mut platformController: DListHead = unsafe {
    {
        let mut init = DListHead {
            next: &platformController as *const DListHead as *mut DListHead,
            prev: &platformController as *const DListHead as *mut DListHead,
        };
        init
    }
};
#[no_mangle]
pub static mut codecController: DListHead = unsafe {
    {
        let mut init = DListHead {
            next: &codecController as *const DListHead as *mut DListHead,
            prev: &codecController as *const DListHead as *mut DListHead,
        };
        init
    }
};
#[no_mangle]
pub static mut dspController: DListHead = unsafe {
    {
        let mut init = DListHead {
            next: &dspController as *const DListHead as *mut DListHead,
            prev: &dspController as *const DListHead as *mut DListHead,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn AudioSocRegisterPlatform(
    mut device: *mut HdfDeviceObject,
    mut platformData: *mut PlatformData,
) -> int32_t {
    let mut platformDevice: *mut PlatformDevice = 0 as *mut PlatformDevice;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: device is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSocRegisterPlatform\0"))
                .as_ptr(),
            41 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if platformData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: platformData is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSocRegisterPlatform\0"))
                .as_ptr(),
            45 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    platformDevice = OsalMemCalloc(::core::mem::size_of::<PlatformDevice>() as size_t)
        as *mut PlatformDevice;
    if platformDevice.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc platformDevice device fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioSocRegisterPlatform\0"))
                .as_ptr(),
            51 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*platformDevice).devPlatformName = (*platformData).drvPlatformName;
    (*platformDevice).devData = platformData;
    (*platformDevice).device = device;
    DListInsertHead(&mut (*platformDevice).list, &mut platformController);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: Register [%s] success.\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 25],
            [core::ffi::c_char; 25],
        >(*b"AudioSocRegisterPlatform\0"))
            .as_ptr(),
        60 as core::ffi::c_int,
        (*platformDevice).devPlatformName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSocRegisterDai(
    mut device: *mut HdfDeviceObject,
    mut daiData: *mut DaiData,
) -> int32_t {
    let mut dai: *mut DaiDevice = 0 as *mut DaiDevice;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: device is NULL\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSocRegisterDai\0"))
                .as_ptr(),
            69 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if daiData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: daiData is NULL\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSocRegisterDai\0"))
                .as_ptr(),
            73 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    dai = OsalMemCalloc(::core::mem::size_of::<DaiDevice>() as size_t) as *mut DaiDevice;
    if dai.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc dai device fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSocRegisterDai\0"))
                .as_ptr(),
            79 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*dai).devDaiName = (*daiData).drvDaiName;
    (*dai).devData = daiData as *mut DaiData;
    (*dai).device = device;
    DListInsertHead(&mut (*dai).list, &mut daiController);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: Register [%s] success.\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 20],
            [core::ffi::c_char; 20],
        >(*b"AudioSocRegisterDai\0"))
            .as_ptr(),
        87 as core::ffi::c_int,
        (*dai).devDaiName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRegisterCodec(
    mut device: *mut HdfDeviceObject,
    mut codecData: *mut CodecData,
    mut daiData: *mut DaiData,
) -> int32_t {
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut ret: int32_t = 0;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: device is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRegisterCodec\0"))
                .as_ptr(),
            98 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if codecData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: codecData is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRegisterCodec\0"))
                .as_ptr(),
            102 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if daiData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: daiData is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRegisterCodec\0"))
                .as_ptr(),
            106 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    codec = OsalMemCalloc(::core::mem::size_of::<CodecDevice>() as size_t)
        as *mut CodecDevice;
    if codec.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc codec device fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRegisterCodec\0"))
                .as_ptr(),
            112 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*codec).devCodecName = (*codecData).drvCodecName;
    (*codec).devData = codecData as *mut CodecData;
    (*codec).device = device;
    ret = AudioSocRegisterDai(device, daiData);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalIoUnmap(
            (*(*codec).devData).virtualAddress as uintptr_t as *mut core::ffi::c_void,
        );
        OsalMemFree(codec as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Register dai device fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioRegisterCodec\0"))
                .as_ptr(),
            124 as core::ffi::c_int,
            ret,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    DListInsertHead(&mut (*codec).list, &mut codecController);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: Register [%s] success.\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 19],
            [core::ffi::c_char; 19],
        >(*b"AudioRegisterCodec\0"))
            .as_ptr(),
        128 as core::ffi::c_int,
        (*codec).devCodecName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioRegisterDsp(
    mut device: *mut HdfDeviceObject,
    mut dspData: *mut DspData,
    mut DaiData: *mut DaiData,
) -> int32_t {
    let mut dspDev: *mut DspDevice = 0 as *mut DspDevice;
    let mut ret: int32_t = 0;
    if device.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: device is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRegisterDsp\0"))
                .as_ptr(),
            139 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if dspData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: dspData is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRegisterDsp\0"))
                .as_ptr(),
            143 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if DaiData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: daiData is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRegisterDsp\0"))
                .as_ptr(),
            147 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    dspDev = OsalMemCalloc(::core::mem::size_of::<DspDevice>() as size_t)
        as *mut DspDevice;
    if dspDev.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc codec device fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRegisterDsp\0"))
                .as_ptr(),
            153 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*dspDev).devDspName = (*dspData).drvDspName;
    (*dspDev).devData = dspData as *mut DspData;
    (*dspDev).device = device;
    ret = AudioSocRegisterDai(device, DaiData);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMemFree(dspDev as *mut core::ffi::c_void);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Register dai device fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioRegisterDsp\0"))
                .as_ptr(),
            164 as core::ffi::c_int,
            ret,
        );
        return HDF_ERR_IO as core::ffi::c_int as int32_t;
    }
    DListInsertHead(&mut (*dspDev).list, &mut dspController);
    HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        LOG_DOMAIN as core::ffi::c_uint,
        b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
        b"[%s][line:%d]: Register [%s] success.\0" as *const u8
            as *const core::ffi::c_char,
        (::core::mem::transmute::<
            [u8; 17],
            [core::ffi::c_char; 17],
        >(*b"AudioRegisterDsp\0"))
            .as_ptr(),
        168 as core::ffi::c_int,
        (*dspDev).devDspName,
    );
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSeekPlatformDevice(
    mut rtd: *mut AudioRuntimeDeivces,
    mut configData: *const AudioConfigData,
) -> int32_t {
    let mut platform: *mut PlatformDevice = 0 as *mut PlatformDevice;
    if rtd.is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSeekPlatformDevice\0"))
                .as_ptr(),
            177 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).platformName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input devicesName check error: configData->platformName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioSeekPlatformDevice\0"))
                .as_ptr(),
            181 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    platform = (platformController.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut PlatformDevice)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut PlatformDevice;
    while &mut (*platform).list as *mut DListHead
        != &mut platformController as *mut DListHead
    {
        if !((*platform).devPlatformName).is_null()
            && strcmp((*platform).devPlatformName, (*configData).platformName)
                == 0 as core::ffi::c_int
        {
            (*rtd).platform = platform as *mut PlatformDevice;
            break;
        } else {
            platform = ((*platform).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut PlatformDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut PlatformDevice;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSeekCpuDaiDevice(
    mut rtd: *mut AudioRuntimeDeivces,
    mut configData: *const AudioConfigData,
) -> int32_t {
    let mut cpuDai: *mut DaiDevice = 0 as *mut DaiDevice;
    if rtd.is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSeekCpuDaiDevice\0"))
                .as_ptr(),
            201 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).cpuDaiName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input cpuDaiName check error: configData->cpuDaiName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSeekCpuDaiDevice\0"))
                .as_ptr(),
            205 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if DListIsEmpty(&mut daiController) {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: daiController is empty.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioSeekCpuDaiDevice\0"))
                .as_ptr(),
            209 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    cpuDai = (daiController.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut DaiDevice;
    while &mut (*cpuDai).list as *mut DListHead != &mut daiController as *mut DListHead {
        if cpuDai.is_null() {
            return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
        }
        if !((*cpuDai).devDaiName).is_null()
            && strcmp((*cpuDai).devDaiName, (*configData).cpuDaiName)
                == 0 as core::ffi::c_int
        {
            (*rtd).cpuDai = cpuDai;
            break;
        } else {
            cpuDai = ((*cpuDai).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut DaiDevice;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSeekCodecDevice(
    mut rtd: *mut AudioRuntimeDeivces,
    mut configData: *const AudioConfigData,
) -> int32_t {
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut codecDai: *mut DaiDevice = 0 as *mut DaiDevice;
    if rtd.is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioSeekCodecDevice\0"))
                .as_ptr(),
            233 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).codecName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input devicesName check error: configData->codecName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioSeekCodecDevice\0"))
                .as_ptr(),
            237 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).codecDaiName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input devicesName check error: configData->codecDaiName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioSeekCodecDevice\0"))
                .as_ptr(),
            241 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    codec = (codecController.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut CodecDevice)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut CodecDevice;
    while &mut (*codec).list as *mut DListHead != &mut codecController as *mut DListHead
    {
        if !((*codec).devCodecName).is_null()
            && strcmp((*codec).devCodecName, (*configData).codecName)
                == 0 as core::ffi::c_int
        {
            (*rtd).codec = codec as *mut CodecDevice;
            codecDai = (daiController.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut DaiDevice;
            while &mut (*codecDai).list as *mut DListHead
                != &mut daiController as *mut DListHead
            {
                if codecDai.is_null() {
                    return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
                }
                if !((*codecDai).device).is_null()
                    && (*codec).device == (*codecDai).device
                    && !((*codecDai).devDaiName).is_null()
                    && strcmp((*codecDai).devDaiName, (*configData).codecDaiName)
                        == 0 as core::ffi::c_int
                {
                    (*rtd).codecDai = codecDai as *mut DaiDevice;
                    break;
                } else {
                    codecDai = ((*codecDai).list.next as *mut core::ffi::c_char)
                        .offset_from(
                            &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                                as *mut core::ffi::c_char,
                        ) as core::ffi::c_int as *mut DaiDevice;
                }
            }
            break;
        } else {
            codec = ((*codec).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut CodecDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut CodecDevice;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioSeekDspDevice(
    mut rtd: *mut AudioRuntimeDeivces,
    mut configData: *const AudioConfigData,
) -> int32_t {
    let mut dsp: *mut DspDevice = 0 as *mut DspDevice;
    let mut dspDai: *mut DaiDevice = 0 as *mut DaiDevice;
    if rtd.is_null() || configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSeekDspDevice\0"))
                .as_ptr(),
            270 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).dspName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input devicesName check error: configData->dspName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSeekDspDevice\0"))
                .as_ptr(),
            274 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if ((*configData).dspDaiName).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input devicesName check error: configData->dspDaiName is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSeekDspDevice\0"))
                .as_ptr(),
            278 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    dsp = (dspController.next as *mut core::ffi::c_char)
        .offset_from(
            &mut (*(0 as *mut DspDevice)).list as *mut DListHead
                as *mut core::ffi::c_char,
        ) as core::ffi::c_int as *mut DspDevice;
    while &mut (*dsp).list as *mut DListHead != &mut dspController as *mut DListHead {
        if dsp.is_null() {
            return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
        }
        if !((*dsp).devDspName).is_null()
            && strcmp((*dsp).devDspName, (*configData).dspName) == 0 as core::ffi::c_int
        {
            (*rtd).dsp = dsp as *mut DspDevice;
            dspDai = (daiController.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut DaiDevice;
            while &mut (*dspDai).list as *mut DListHead
                != &mut daiController as *mut DListHead
            {
                if dspDai.is_null() {
                    return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
                }
                if !((*dspDai).device).is_null() && (*dsp).device == (*dspDai).device
                    && !((*dspDai).devDaiName).is_null()
                    && strcmp((*dspDai).devDaiName, (*configData).dspDaiName)
                        == 0 as core::ffi::c_int
                {
                    (*rtd).dspDai = dspDai;
                    break;
                } else {
                    dspDai = ((*dspDai).list.next as *mut core::ffi::c_char)
                        .offset_from(
                            &mut (*(0 as *mut DaiDevice)).list as *mut DListHead
                                as *mut core::ffi::c_char,
                        ) as core::ffi::c_int as *mut DaiDevice;
                }
            }
            break;
        } else {
            dsp = ((*dsp).list.next as *mut core::ffi::c_char)
                .offset_from(
                    &mut (*(0 as *mut DspDevice)).list as *mut DListHead
                        as *mut core::ffi::c_char,
                ) as core::ffi::c_int as *mut DspDevice;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioBindDaiLink(
    mut audioCard: *mut AudioCard,
    mut configData: *const AudioConfigData,
) -> int32_t {
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            308 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if configData.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: configData is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            312 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    (*audioCard).rtd = OsalMemCalloc(
        ::core::mem::size_of::<AudioRuntimeDeivces>() as size_t,
    ) as *mut AudioRuntimeDeivces as *mut AudioRuntimeDeivces;
    if ((*audioCard).rtd).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc audioCard->rtd fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            318 as core::ffi::c_int,
        );
        return HDF_ERR_MALLOC_FAIL as core::ffi::c_int as int32_t;
    }
    (*(*audioCard).rtd).complete = AUDIO_DAI_LINK_UNCOMPLETE as uint8_t;
    if AudioSeekPlatformDevice((*audioCard).rtd as *mut AudioRuntimeDeivces, configData)
        == HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: PLATFORM [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            324 as core::ffi::c_int,
            (*configData).platformName,
        );
    }
    if AudioSeekCpuDaiDevice((*audioCard).rtd as *mut AudioRuntimeDeivces, configData)
        == HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CPUDAI [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            327 as core::ffi::c_int,
            (*configData).cpuDaiName,
        );
    }
    if AudioSeekCodecDevice((*audioCard).rtd as *mut AudioRuntimeDeivces, configData)
        == HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CODEC [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            330 as core::ffi::c_int,
            (*configData).codecName,
        );
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: CODECDAI [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            331 as core::ffi::c_int,
            (*configData).codecDaiName,
        );
    }
    if AudioSeekDspDevice((*audioCard).rtd as *mut AudioRuntimeDeivces, configData)
        == HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DSP [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            334 as core::ffi::c_int,
            (*configData).dspName,
        );
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: DSPDAI [%s] is registered!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioBindDaiLink\0"))
                .as_ptr(),
            335 as core::ffi::c_int,
            (*configData).dspDaiName,
        );
    }
    (*(*audioCard).rtd).complete = AUDIO_DAI_LINK_COMPLETE as uint8_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDaiReadReg(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut val: *mut uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if dai.is_null() || ((*dai).devData).is_null() || ((*(*dai).devData).Read).is_none()
        || val.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDaiReadReg\0"))
                .as_ptr(),
            348 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = ((*(*dai).devData).Read).expect("non-null function pointer")(dai, reg, val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: dai device read fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioDaiReadReg\0"))
                .as_ptr(),
            354 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDaiWriteReg(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut val: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if dai.is_null() || ((*dai).devData).is_null() || ((*(*dai).devData).Write).is_none()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioDaiWriteReg\0"))
                .as_ptr(),
            364 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = ((*(*dai).devData).Write).expect("non-null function pointer")(dai, reg, val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: dai device write fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioDaiWriteReg\0"))
                .as_ptr(),
            370 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecReadReg(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut val: *mut uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if codec.is_null() || ((*codec).devData).is_null()
        || ((*(*codec).devData).Read).is_none() || val.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCodecReadReg\0"))
                .as_ptr(),
            381 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = ((*(*codec).devData).Read)
        .expect("non-null function pointer")(codec, reg, val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Codec device read fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioCodecReadReg\0"))
                .as_ptr(),
            387 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecWriteReg(
    mut codec: *const CodecDevice,
    mut reg: uint32_t,
    mut val: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if codec.is_null() || ((*codec).devData).is_null()
        || ((*(*codec).devData).Write).is_none()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioCodecWriteReg\0"))
                .as_ptr(),
            398 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = ((*(*codec).devData).Write)
        .expect("non-null function pointer")(codec, reg, val);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Codec device write fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioCodecWriteReg\0"))
                .as_ptr(),
            404 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioUpdateCodecRegBits(
    mut codec: *mut CodecDevice,
    mut reg: uint32_t,
    mask: uint32_t,
    shift: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut curValue: uint32_t = 0 as uint32_t;
    let mut controlMask: uint32_t = 0;
    let mut tempVal: uint32_t = 0;
    if codec.is_null() || ((*codec).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Invalid input param.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioUpdateCodecRegBits\0"))
                .as_ptr(),
            418 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    value = value << shift;
    controlMask = mask << shift;
    OsalMutexLock(&mut (*(*codec).devData).mutex);
    ret = AudioCodecReadReg(codec, reg, &mut curValue);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Read reg fail ret=%d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioUpdateCodecRegBits\0"))
                .as_ptr(),
            429 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    tempVal = curValue & controlMask;
    if tempVal == value {
        OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    curValue = curValue & !controlMask | value & controlMask;
    ret = AudioCodecWriteReg(codec, reg, curValue);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMutexUnlock(&mut (*(*codec).devData).mutex);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Write reg fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioUpdateCodecRegBits\0"))
                .as_ptr(),
            443 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    OsalMutexUnlock(&mut (*(*codec).devData).mutex);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioUpdateDaiRegBits(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mask: uint32_t,
    shift: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    let mut curValue: uint32_t = 0 as uint32_t;
    let mut mixerControlMask: uint32_t = 0;
    let mut tempVal: uint32_t = 0;
    let mut data: *mut DaiData = 0 as *mut DaiData;
    if dai.is_null() || ((*dai).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Invalid input param.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioUpdateDaiRegBits\0"))
                .as_ptr(),
            462 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    data = (*dai).devData as *mut DaiData;
    value = value << shift;
    mixerControlMask = mask << shift;
    OsalMutexLock(&mut (*data).mutex);
    ret = AudioDaiReadReg(dai, reg, &mut curValue);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMutexUnlock(&mut (*data).mutex);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Read reg fail ret=%d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioUpdateDaiRegBits\0"))
                .as_ptr(),
            473 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    tempVal = curValue & mixerControlMask;
    if tempVal == value {
        OsalMutexUnlock(&mut (*data).mutex);
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    curValue = curValue & !mixerControlMask | value & mixerControlMask;
    ret = AudioDaiWriteReg(dai, reg, curValue);
    if ret != HDF_SUCCESS as core::ffi::c_int {
        OsalMutexUnlock(&mut (*data).mutex);
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Write reg fail ret=%d\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioUpdateDaiRegBits\0"))
                .as_ptr(),
            487 as core::ffi::c_int,
            ret,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    OsalMutexUnlock(&mut (*data).mutex);
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecRegUpdate(
    mut codec: *mut CodecDevice,
    mut mixerCtrl: *mut AudioMixerControl,
) -> int32_t {
    let mut mixerValue: uint32_t = 0;
    if codec.is_null() || mixerCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioKcontrolGetCodec is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCodecRegUpdate\0"))
                .as_ptr(),
            500 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerValue = (*mixerCtrl).value;
    if mixerValue < (*mixerCtrl).min || mixerValue > (*mixerCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec invalid value=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCodecRegUpdate\0"))
                .as_ptr(),
            506 as core::ffi::c_int,
            mixerValue,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).invert != 0 {
        mixerValue = ((*mixerCtrl).max).wrapping_sub((*mixerCtrl).value);
    }
    if AudioUpdateCodecRegBits(
        codec,
        (*mixerCtrl).reg,
        (*mixerCtrl).mask,
        (*mixerCtrl).shift,
        mixerValue,
    ) != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioCodecRegUpdate\0"))
                .as_ptr(),
            514 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).reg != (*mixerCtrl).rreg || (*mixerCtrl).shift != (*mixerCtrl).rshift
    {
        if AudioUpdateCodecRegBits(
            codec,
            (*mixerCtrl).rreg,
            (*mixerCtrl).mask,
            (*mixerCtrl).rshift,
            mixerValue,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"AudioCodecRegUpdate\0"))
                    .as_ptr(),
                521 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecMuxRegUpdate(
    mut codec: *mut CodecDevice,
    mut enumCtrl: *mut AudioEnumKcontrol,
    mut value: *const uint32_t,
) -> int32_t {
    let mut val: [uint32_t; 2] = [0; 2];
    let mut ret: int32_t = 0;
    if codec.is_null() || enumCtrl.is_null() || value.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioCodecMuxRegUpdate\0"))
                .as_ptr(),
            535 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if !((*enumCtrl).values).is_null() {
        val[0 as core::ffi::c_int as usize] = *((*enumCtrl).values)
            .offset(*value.offset(0 as core::ffi::c_int as isize) as isize);
        val[1 as core::ffi::c_int as usize] = *((*enumCtrl).values)
            .offset(*value.offset(1 as core::ffi::c_int as isize) as isize);
    } else {
        val[0 as core::ffi::c_int as usize] = *value
            .offset(0 as core::ffi::c_int as isize);
        val[1 as core::ffi::c_int as usize] = *value
            .offset(1 as core::ffi::c_int as isize);
    }
    if val[0 as core::ffi::c_int as usize] > (*enumCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio invalid value=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioCodecMuxRegUpdate\0"))
                .as_ptr(),
            548 as core::ffi::c_int,
            val[0 as core::ffi::c_int as usize],
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = AudioUpdateCodecRegBits(
        codec,
        (*enumCtrl).reg,
        (*enumCtrl).mask,
        (*enumCtrl).shiftLeft as uint32_t,
        val[0 as core::ffi::c_int as usize],
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: update left reg bits fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioCodecMuxRegUpdate\0"))
                .as_ptr(),
            553 as core::ffi::c_int,
        );
        return ret;
    }
    if (*enumCtrl).reg != (*enumCtrl).reg2
        || (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
    {
        if val[1 as core::ffi::c_int as usize] > (*enumCtrl).max {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid value=%u\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"AudioCodecMuxRegUpdate\0"))
                    .as_ptr(),
                559 as core::ffi::c_int,
                val[1 as core::ffi::c_int as usize],
            );
            return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
        }
        ret = AudioUpdateCodecRegBits(
            codec,
            (*enumCtrl).reg2,
            (*enumCtrl).mask,
            (*enumCtrl).shiftRight as uint32_t,
            val[1 as core::ffi::c_int as usize],
        );
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: update right reg bits fail!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"AudioCodecMuxRegUpdate\0"))
                    .as_ptr(),
                564 as core::ffi::c_int,
            );
            return ret;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDaiRegUpdate(
    mut dai: *const DaiDevice,
    mut mixerCtrl: *mut AudioMixerControl,
) -> int32_t {
    let mut value: uint32_t = 0;
    if dai.is_null() || mixerCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioKcontrolGetCodec is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioDaiRegUpdate\0"))
                .as_ptr(),
            576 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    value = (*mixerCtrl).value;
    if value < (*mixerCtrl).min || value > (*mixerCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio dai invalid value=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioDaiRegUpdate\0"))
                .as_ptr(),
            582 as core::ffi::c_int,
            value,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).invert != 0 {
        value = ((*mixerCtrl).max).wrapping_sub((*mixerCtrl).value);
    }
    if AudioUpdateDaiRegBits(
        dai,
        (*mixerCtrl).reg,
        (*mixerCtrl).mask,
        (*mixerCtrl).shift,
        value,
    ) != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"AudioDaiRegUpdate\0"))
                .as_ptr(),
            591 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).reg != (*mixerCtrl).rreg || (*mixerCtrl).shift != (*mixerCtrl).rshift
    {
        if AudioUpdateDaiRegBits(
            dai,
            (*mixerCtrl).rreg,
            (*mixerCtrl).mask,
            (*mixerCtrl).rshift,
            value,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 18],
                    [core::ffi::c_char; 18],
                >(*b"AudioDaiRegUpdate\0"))
                    .as_ptr(),
                598 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioKcontrolGetCodec(
    mut kcontrol: *const AudioKcontrol,
) -> *mut CodecDevice {
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    if kcontrol.is_null() || ((*kcontrol).pri).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Get Codec input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioKcontrolGetCodec\0"))
                .as_ptr(),
            610 as core::ffi::c_int,
        );
        return 0 as *mut CodecDevice;
    }
    audioCard = (*kcontrol).pri as uintptr_t as *mut AudioCard;
    if ((*audioCard).rtd).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Get audio card or rtd fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioKcontrolGetCodec\0"))
                .as_ptr(),
            616 as core::ffi::c_int,
        );
        return 0 as *mut CodecDevice;
    }
    return (*(*audioCard).rtd).codec as *mut CodecDevice;
}
#[no_mangle]
pub unsafe extern "C" fn AudioKcontrolGetCpuDai(
    mut kcontrol: *const AudioKcontrol,
) -> *mut DaiDevice {
    let mut audioCard: *mut AudioCard = 0 as *mut AudioCard;
    if kcontrol.is_null() || ((*kcontrol).pri).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Get CpuDai input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioKcontrolGetCpuDai\0"))
                .as_ptr(),
            627 as core::ffi::c_int,
        );
        return 0 as *mut DaiDevice;
    }
    audioCard = (*kcontrol).pri as uintptr_t as *mut AudioCard;
    if ((*audioCard).rtd).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Get audio card or rtd fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioKcontrolGetCpuDai\0"))
                .as_ptr(),
            633 as core::ffi::c_int,
        );
        return 0 as *mut DaiDevice;
    }
    return (*(*audioCard).rtd).cpuDai;
}
#[no_mangle]
pub unsafe extern "C" fn AudioAddControl(
    mut audioCard: *const AudioCard,
    mut ctrl: *const AudioKcontrol,
) -> *mut AudioKcontrol {
    let mut control: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    if audioCard.is_null() || ctrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioAddControl\0"))
                .as_ptr(),
            645 as core::ffi::c_int,
        );
        return 0 as *mut AudioKcontrol;
    }
    control = OsalMemCalloc(::core::mem::size_of::<AudioKcontrol>() as size_t)
        as *mut AudioKcontrol;
    if control.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Malloc control fail!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"AudioAddControl\0"))
                .as_ptr(),
            651 as core::ffi::c_int,
        );
        return 0 as *mut AudioKcontrol;
    }
    DListHeadInit(&mut (*control).list);
    (*control).name = (*ctrl).name;
    (*control).iface = (*ctrl).iface;
    (*control).Info = (*ctrl).Info;
    (*control).Get = (*ctrl).Get;
    (*control).Set = (*ctrl).Set;
    (*control).pri = audioCard as *mut core::ffi::c_void;
    (*control).privateValue = (*ctrl).privateValue;
    return control;
}
#[no_mangle]
pub unsafe extern "C" fn AudioAddControls(
    mut audioCard: *mut AudioCard,
    mut controls: *const AudioKcontrol,
    mut controlMaxNum: int32_t,
) -> int32_t {
    let mut control: *mut AudioKcontrol = 0 as *mut AudioKcontrol;
    let mut i: int32_t = 0;
    if audioCard.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: audioCard is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioAddControls\0"))
                .as_ptr(),
            673 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if controls.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: controls is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioAddControls\0"))
                .as_ptr(),
            677 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if controlMaxNum <= 0 as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input params check error: controlMaxNum=%d.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioAddControls\0"))
                .as_ptr(),
            681 as core::ffi::c_int,
            controlMaxNum,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    i = 0 as core::ffi::c_int as int32_t;
    while i < controlMaxNum {
        control = AudioAddControl(audioCard, &*controls.offset(i as isize));
        if control.is_null() {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Add control fail!\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"AudioAddControls\0"))
                    .as_ptr(),
                688 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        DListInsertHead(&mut (*control).list, &mut (*audioCard).controls);
        i += 1;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioInfoCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemInfo: *mut AudioCtrlElemInfo,
) -> int32_t {
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemInfo.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"AudioInfoCtrlOps\0"))
                .as_ptr(),
            702 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    if (*mixerCtrl).reg != (*mixerCtrl).rreg || (*mixerCtrl).shift != (*mixerCtrl).rshift
    {
        (*elemInfo).count = CHANNEL_MAX_NUM as uint32_t;
    } else {
        (*elemInfo).count = CHANNEL_MIN_NUM as uint32_t;
    }
    (*elemInfo).type_0 = AUDIO_CTL_ELEM_TYPE_INTEGER as core::ffi::c_int as int32_t;
    (*elemInfo).min = (*mixerCtrl).min as int32_t;
    (*elemInfo).max = (*mixerCtrl).max as int32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioInfoEnumCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemInfo: *mut AudioCtrlElemInfo,
) -> int32_t {
    let mut enumCtrl: *mut AudioEnumKcontrol = 0 as *mut AudioEnumKcontrol;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemInfo.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Input param kcontrol is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioInfoEnumCtrlOps\0"))
                .as_ptr(),
            725 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    enumCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioEnumKcontrol;
    if (*enumCtrl).reg != (*enumCtrl).reg2
        || (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
    {
        (*elemInfo).count = CHANNEL_MAX_NUM as uint32_t;
    } else {
        (*elemInfo).count = CHANNEL_MIN_NUM as uint32_t;
    }
    (*elemInfo).type_0 = AUDIO_CTL_ELEM_TYPE_ENUMERATED as core::ffi::c_int as int32_t;
    (*elemInfo).min = 0 as core::ffi::c_int as int32_t;
    (*elemInfo).max = (*enumCtrl).max as int32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioGetCtrlOpsRReg(
    mut elemValue: *mut AudioCtrlElemValue,
    mut mixerCtrl: *const AudioMixerControl,
    mut rcurValue: uint32_t,
) -> int32_t {
    if elemValue.is_null() || mixerCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioGetCtrlOpsRReg\0"))
                .as_ptr(),
            747 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).reg != (*mixerCtrl).rreg || (*mixerCtrl).shift != (*mixerCtrl).rshift
    {
        if (*mixerCtrl).reg == (*mixerCtrl).rreg {
            rcurValue = rcurValue >> (*mixerCtrl).rshift & (*mixerCtrl).mask;
        } else {
            rcurValue = rcurValue >> (*mixerCtrl).shift & (*mixerCtrl).mask;
        }
        if rcurValue > (*mixerCtrl).max || rcurValue < (*mixerCtrl).min {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid rcurValue=%u\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"AudioGetCtrlOpsRReg\0"))
                    .as_ptr(),
                758 as core::ffi::c_int,
                rcurValue,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        if (*mixerCtrl).invert != 0 {
            rcurValue = ((*mixerCtrl).max).wrapping_sub(rcurValue);
        }
        (*elemValue).value[1 as core::ffi::c_int as usize] = rcurValue;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioGetCtrlOpsReg(
    mut elemValue: *mut AudioCtrlElemValue,
    mut mixerCtrl: *const AudioMixerControl,
    mut curValue: uint32_t,
) -> int32_t {
    if elemValue.is_null() || mixerCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioGetCtrlOpsReg\0"))
                .as_ptr(),
            774 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    curValue = curValue >> (*mixerCtrl).shift & (*mixerCtrl).mask;
    if curValue > (*mixerCtrl).max || curValue < (*mixerCtrl).min {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio invalid curValue=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioGetCtrlOpsReg\0"))
                .as_ptr(),
            780 as core::ffi::c_int,
            curValue,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).invert != 0 {
        curValue = ((*mixerCtrl).max).wrapping_sub(curValue);
    }
    (*elemValue).value[0 as core::ffi::c_int as usize] = curValue;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioGetEnumCtrlOpsReg(
    mut elemValue: *mut AudioCtrlElemValue,
    mut enumCtrl: *const AudioEnumKcontrol,
    mut curValue: uint32_t,
) -> int32_t {
    if elemValue.is_null() || enumCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioGetEnumCtrlOpsReg\0"))
                .as_ptr(),
            795 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    curValue = curValue >> (*enumCtrl).shiftLeft as core::ffi::c_int & (*enumCtrl).mask;
    if curValue > (*enumCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio invalid curValue=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"AudioGetEnumCtrlOpsReg\0"))
                .as_ptr(),
            801 as core::ffi::c_int,
            curValue,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*elemValue).value[0 as core::ffi::c_int as usize] = curValue;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioGetEnumCtrlOpsRReg(
    mut elemValue: *mut AudioCtrlElemValue,
    mut enumCtrl: *const AudioEnumKcontrol,
    mut rcurValue: uint32_t,
) -> int32_t {
    if elemValue.is_null() || enumCtrl.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"AudioGetEnumCtrlOpsRReg\0"))
                .as_ptr(),
            814 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if (*enumCtrl).reg != (*enumCtrl).reg2
        || (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
    {
        rcurValue = rcurValue >> (*enumCtrl).shiftLeft as core::ffi::c_int
            & (*enumCtrl).mask;
        if rcurValue > (*enumCtrl).max {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid rcurValue=%u\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 24],
                    [core::ffi::c_char; 24],
                >(*b"AudioGetEnumCtrlOpsRReg\0"))
                    .as_ptr(),
                822 as core::ffi::c_int,
                rcurValue,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        (*elemValue).value[1 as core::ffi::c_int as usize] = rcurValue;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecGetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *mut AudioCtrlElemValue,
) -> int32_t {
    let mut curValue: uint32_t = 0 as uint32_t;
    let mut rcurValue: uint32_t = 0 as uint32_t;
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecGetCtrlOps\0"))
                .as_ptr(),
            839 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    codec = AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: mixerCtrl and codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecGetCtrlOps\0"))
                .as_ptr(),
            845 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCodecReadReg(codec, (*mixerCtrl).reg, &mut curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioCodecReadReg(codec, (*mixerCtrl).rreg, &mut rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Read Codec Reg fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecGetCtrlOps\0"))
                .as_ptr(),
            850 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioGetCtrlOpsReg(elemValue, mixerCtrl, curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioGetCtrlOpsRReg(elemValue, mixerCtrl, rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec get kcontrol reg and rreg fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecGetCtrlOps\0"))
                .as_ptr(),
            855 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecGetEnumCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *mut AudioCtrlElemValue,
) -> int32_t {
    let mut curValue: uint32_t = 0 as uint32_t;
    let mut rcurValue: uint32_t = 0 as uint32_t;
    let mut enumCtrl: *mut AudioEnumKcontrol = 0 as *mut AudioEnumKcontrol;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecGetEnumCtrlOps\0"))
                .as_ptr(),
            869 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    enumCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioEnumKcontrol;
    codec = AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: mixerCtrl and codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecGetEnumCtrlOps\0"))
                .as_ptr(),
            875 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioCodecReadReg(codec, (*enumCtrl).reg, &mut curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioCodecReadReg(codec, (*enumCtrl).reg2, &mut rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Read Reg fail.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecGetEnumCtrlOps\0"))
                .as_ptr(),
            880 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioGetEnumCtrlOpsReg(elemValue, enumCtrl, curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioGetEnumCtrlOpsRReg(elemValue, enumCtrl, rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec get kcontrol reg and rreg fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecGetEnumCtrlOps\0"))
                .as_ptr(),
            886 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSetCtrlOpsReg(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
    mut mixerCtrl: *const AudioMixerControl,
    mut value: *mut uint32_t,
) -> int32_t {
    let mut val: uint32_t = 0;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null() || mixerCtrl.is_null() || value.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSetCtrlOpsReg\0"))
                .as_ptr(),
            900 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    val = (*elemValue).value[0 as core::ffi::c_int as usize];
    if val < (*mixerCtrl).min || val > (*mixerCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio invalid value=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"AudioSetCtrlOpsReg\0"))
                .as_ptr(),
            906 as core::ffi::c_int,
            val,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    *value = (if (*mixerCtrl).invert == 0 as core::ffi::c_uint {
        val as core::ffi::c_uint
    } else {
        ((*mixerCtrl).max as core::ffi::c_uint).wrapping_sub(val as core::ffi::c_uint)
    }) as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSetCtrlOpsRReg(
    mut elemValue: *const AudioCtrlElemValue,
    mut mixerCtrl: *mut AudioMixerControl,
    mut rvalue: *mut uint32_t,
    mut updateRReg: *mut bool,
) -> int32_t {
    let mut val: uint32_t = 0;
    if elemValue.is_null() || mixerCtrl.is_null() || rvalue.is_null()
        || updateRReg.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"AudioSetCtrlOpsRReg\0"))
                .as_ptr(),
            920 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if (*mixerCtrl).reg != (*mixerCtrl).rreg || (*mixerCtrl).shift != (*mixerCtrl).rshift
    {
        val = (*elemValue).value[1 as core::ffi::c_int as usize];
        if val < (*mixerCtrl).min || val > (*mixerCtrl).max {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"AudioSetCtrlOpsRReg\0"))
                    .as_ptr(),
                927 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        if (*mixerCtrl).reg == (*mixerCtrl).rreg {
            (*mixerCtrl).shift = (*mixerCtrl).rshift;
        }
        *rvalue = (if (*mixerCtrl).invert == 0 as core::ffi::c_uint {
            val as core::ffi::c_uint
        } else {
            ((*mixerCtrl).max as core::ffi::c_uint)
                .wrapping_sub(val as core::ffi::c_uint)
        }) as uint32_t;
        *updateRReg = true_0 != 0;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
) -> int32_t {
    let mut value: uint32_t = 0 as uint32_t;
    let mut rvalue: uint32_t = 0 as uint32_t;
    let mut updateRReg: bool = false_0 != 0;
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec set control register input param is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecSetCtrlOps\0"))
                .as_ptr(),
            950 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    codec = AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioKcontrolGetCodec is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecSetCtrlOps\0"))
                .as_ptr(),
            956 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    if AudioSetCtrlOpsReg(kcontrol, elemValue, mixerCtrl, &mut value)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSetCtrlOpsReg is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecSetCtrlOps\0"))
                .as_ptr(),
            962 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if AudioUpdateCodecRegBits(
        codec,
        (*mixerCtrl).reg,
        (*mixerCtrl).mask,
        (*mixerCtrl).shift,
        value,
    ) != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecSetCtrlOps\0"))
                .as_ptr(),
            967 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSetCtrlOpsRReg(elemValue, mixerCtrl, &mut rvalue, &mut updateRReg)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSetCtrlOpsRReg is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"AudioCodecSetCtrlOps\0"))
                .as_ptr(),
            972 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if updateRReg {
        if AudioUpdateCodecRegBits(
            codec,
            (*mixerCtrl).rreg,
            (*mixerCtrl).mask,
            (*mixerCtrl).rshift,
            rvalue,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 21],
                    [core::ffi::c_char; 21],
                >(*b"AudioCodecSetCtrlOps\0"))
                    .as_ptr(),
                979 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
unsafe extern "C" fn AudioCodecSetEnumRegUpdate(
    mut codec: *mut CodecDevice,
    mut enumCtrl: *const AudioEnumKcontrol,
    mut value: *const uint32_t,
) -> int32_t {
    let mut setVal: [uint32_t; 2] = [0; 2];
    let mut ret: int32_t = 0;
    if codec.is_null() || enumCtrl.is_null() || value.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioCodecSetEnumRegUpdate\0"))
                .as_ptr(),
            994 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if !((*enumCtrl).values).is_null() {
        setVal[0 as core::ffi::c_int as usize] = *((*enumCtrl).values)
            .offset(*value.offset(0 as core::ffi::c_int as isize) as isize);
        setVal[1 as core::ffi::c_int as usize] = *((*enumCtrl).values)
            .offset(*value.offset(1 as core::ffi::c_int as isize) as isize);
    } else {
        setVal[0 as core::ffi::c_int as usize] = *value
            .offset(0 as core::ffi::c_int as isize);
        setVal[1 as core::ffi::c_int as usize] = *value
            .offset(1 as core::ffi::c_int as isize);
    }
    if setVal[0 as core::ffi::c_int as usize] > (*enumCtrl).max {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio invalid value=%u\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioCodecSetEnumRegUpdate\0"))
                .as_ptr(),
            1007 as core::ffi::c_int,
            setVal[0 as core::ffi::c_int as usize],
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    ret = AudioUpdateCodecRegBits(
        codec,
        (*enumCtrl).reg,
        (*enumCtrl).mask,
        (*enumCtrl).shiftLeft as uint32_t,
        setVal[0 as core::ffi::c_int as usize],
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"AudioCodecSetEnumRegUpdate\0"))
                .as_ptr(),
            1012 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if (*enumCtrl).reg != (*enumCtrl).reg2
        || (*enumCtrl).shiftLeft as core::ffi::c_int
            != (*enumCtrl).shiftRight as core::ffi::c_int
    {
        if setVal[1 as core::ffi::c_int as usize] > (*enumCtrl).max {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio invalid value=%u\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 27],
                    [core::ffi::c_char; 27],
                >(*b"AudioCodecSetEnumRegUpdate\0"))
                    .as_ptr(),
                1018 as core::ffi::c_int,
                setVal[1 as core::ffi::c_int as usize],
            );
            return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
        }
        ret = AudioUpdateCodecRegBits(
            codec,
            (*enumCtrl).reg2,
            (*enumCtrl).mask,
            (*enumCtrl).shiftRight as uint32_t,
            setVal[1 as core::ffi::c_int as usize],
        );
        if ret != HDF_SUCCESS as core::ffi::c_int {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 27],
                    [core::ffi::c_char; 27],
                >(*b"AudioCodecSetEnumRegUpdate\0"))
                    .as_ptr(),
                1023 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCodecSetEnumCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
) -> int32_t {
    let mut codec: *mut CodecDevice = 0 as *mut CodecDevice;
    let mut enumCtrl: *mut AudioEnumKcontrol = 0 as *mut AudioEnumKcontrol;
    let mut ret: int32_t = 0;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSetEnumCtrlOps\0"))
                .as_ptr(),
            1037 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    codec = AudioKcontrolGetCodec(kcontrol);
    if codec.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioKcontrolGetCodec is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSetEnumCtrlOps\0"))
                .as_ptr(),
            1042 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    enumCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioEnumKcontrol;
    ret = AudioCodecSetEnumRegUpdate(
        codec,
        enumCtrl,
        ((*elemValue).value).as_ptr() as *mut uint32_t,
    );
    if ret != HDF_SUCCESS as core::ffi::c_int {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioCodecSetEnumRegUpdate fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"AudioCodecSetEnumCtrlOps\0"))
                .as_ptr(),
            1048 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCpuDaiSetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *const AudioCtrlElemValue,
) -> int32_t {
    let mut value: uint32_t = 0;
    let mut rvalue: uint32_t = 0;
    let mut updateRReg: bool = false_0 != 0;
    let mut dai: *mut DaiDevice = 0 as *mut DaiDevice;
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio cpu dai set control register input param is NULL.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiSetCtrlOps\0"))
                .as_ptr(),
            1062 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    if AudioSetCtrlOpsReg(kcontrol, elemValue, mixerCtrl, &mut value)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSetCtrlOpsReg is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiSetCtrlOps\0"))
                .as_ptr(),
            1068 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    dai = AudioKcontrolGetCpuDai(kcontrol);
    if dai.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioKcontrolGetCodec is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiSetCtrlOps\0"))
                .as_ptr(),
            1073 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if AudioUpdateDaiRegBits(
        dai,
        (*mixerCtrl).reg,
        (*mixerCtrl).mask,
        (*mixerCtrl).shift,
        value,
    ) != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiSetCtrlOps\0"))
                .as_ptr(),
            1077 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioSetCtrlOpsRReg(elemValue, mixerCtrl, &mut rvalue, &mut updateRReg)
        != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: AudioSetCtrlOpsRReg is fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiSetCtrlOps\0"))
                .as_ptr(),
            1082 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    if updateRReg {
        if AudioUpdateDaiRegBits(
            dai,
            (*mixerCtrl).rreg,
            (*mixerCtrl).mask,
            (*mixerCtrl).rshift,
            rvalue,
        ) != HDF_SUCCESS as core::ffi::c_int
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Audio codec stereo update reg bits fail.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 22],
                    [core::ffi::c_char; 22],
                >(*b"AudioCpuDaiSetCtrlOps\0"))
                    .as_ptr(),
                1088 as core::ffi::c_int,
            );
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn AudioCpuDaiGetCtrlOps(
    mut kcontrol: *const AudioKcontrol,
    mut elemValue: *mut AudioCtrlElemValue,
) -> int32_t {
    let mut curValue: uint32_t = 0 as uint32_t;
    let mut rcurValue: uint32_t = 0 as uint32_t;
    let mut mixerCtrl: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut dai: *mut DaiDevice = 0 as *mut DaiDevice;
    if kcontrol.is_null() || (*kcontrol).privateValue <= 0 as core::ffi::c_ulong
        || elemValue.is_null()
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio input param is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiGetCtrlOps\0"))
                .as_ptr(),
            1102 as core::ffi::c_int,
        );
        return HDF_ERR_INVALID_OBJECT as core::ffi::c_int as int32_t;
    }
    mixerCtrl = (*kcontrol).privateValue as uintptr_t as *mut AudioMixerControl;
    dai = AudioKcontrolGetCpuDai(kcontrol);
    if dai.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: mixerCtrl and codec is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiGetCtrlOps\0"))
                .as_ptr(),
            1108 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioDaiReadReg(dai, (*mixerCtrl).reg, &mut curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioDaiReadReg(dai, (*mixerCtrl).rreg, &mut rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Read dai Reg fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiGetCtrlOps\0"))
                .as_ptr(),
            1113 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if AudioGetCtrlOpsReg(elemValue, mixerCtrl, curValue)
        != HDF_SUCCESS as core::ffi::c_int
        || AudioGetCtrlOpsRReg(elemValue, mixerCtrl, rcurValue)
            != HDF_SUCCESS as core::ffi::c_int
    {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: Audio dai get kcontrol reg and rreg fail.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"AudioCpuDaiGetCtrlOps\0"))
                .as_ptr(),
            1118 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn OsalIoUnmap(mut addr: *mut core::ffi::c_void) {
    iounmap(addr);
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
