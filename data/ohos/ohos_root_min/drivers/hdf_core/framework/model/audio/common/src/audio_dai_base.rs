use core::arch::asm;
extern "C" {
    pub type HdfSBuf;
    pub type AudioSapmRoute;
    pub type AudioSapmComponent;
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
    fn AudioInfoCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemInfo: *mut AudioCtrlElemInfo,
    ) -> int32_t;
    fn AudioCpuDaiSetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *const AudioCtrlElemValue,
    ) -> int32_t;
    fn AudioCpuDaiGetCtrlOps(
        kcontrol: *const AudioKcontrol,
        elemValue: *mut AudioCtrlElemValue,
    ) -> int32_t;
}
pub type size_t = core::ffi::c_uint;
pub type uintptr_t = core::ffi::c_uint;
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
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AUDIO_SAMPLE_RATE_BUTT: C2RustUnnamed_0 = 96001;
pub const AUDIO_SAMPLE_RATE_96000: C2RustUnnamed_0 = 96000;
pub const AUDIO_SAMPLE_RATE_64000: C2RustUnnamed_0 = 64000;
pub const AUDIO_SAMPLE_RATE_48000: C2RustUnnamed_0 = 48000;
pub const AUDIO_SAMPLE_RATE_44100: C2RustUnnamed_0 = 44100;
pub const AUDIO_SAMPLE_RATE_32000: C2RustUnnamed_0 = 32000;
pub const AUDIO_SAMPLE_RATE_24000: C2RustUnnamed_0 = 24000;
pub const AUDIO_SAMPLE_RATE_22050: C2RustUnnamed_0 = 22050;
pub const AUDIO_SAMPLE_RATE_16000: C2RustUnnamed_0 = 16000;
pub const AUDIO_SAMPLE_RATE_11025: C2RustUnnamed_0 = 11025;
pub const AUDIO_SAMPLE_RATE_12000: C2RustUnnamed_0 = 12000;
pub const AUDIO_SAMPLE_RATE_8000: C2RustUnnamed_0 = 8000;
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
pub const LOG_DOMAIN: core::ffi::c_int = 0xd002510 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AUDIO_CTRL_LIST_MAX: core::ffi::c_int = 100 as core::ffi::c_int;
static mut g_audioDaiControlsList: [*mut core::ffi::c_char; 100] = [
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
pub unsafe extern "C" fn DaiDataFromCard(mut card: *const AudioCard) -> *mut DaiData {
    if card.is_null() || ((*card).rtd).is_null() || ((*(*card).rtd).cpuDai).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null.\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"DaiDataFromCard\0"))
                .as_ptr(),
            25 as core::ffi::c_int,
        );
        return 0 as *mut DaiData;
    }
    return (*(*(*card).rtd).cpuDai).devData as *mut DaiData;
}
#[no_mangle]
pub unsafe extern "C" fn DaiGetConfigInfo(
    mut device: *const HdfDeviceObject,
    mut data: *mut DaiData,
) -> int32_t {
    if device.is_null() || data.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param is null!\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DaiGetConfigInfo\0"))
                .as_ptr(),
            35 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    if !((*data).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: g_codecData regConfig has been parsed!\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DaiGetConfigInfo\0"))
                .as_ptr(),
            40 as core::ffi::c_int,
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
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DaiGetConfigInfo\0"))
                .as_ptr(),
            46 as core::ffi::c_int,
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
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DaiGetConfigInfo\0"))
                .as_ptr(),
            51 as core::ffi::c_int,
        );
        OsalMemFree((*data).regConfig as *mut core::ffi::c_void);
        (*data).regConfig = 0 as *mut AudioRegCfgData;
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn DaiCheckSampleRate(mut sampleRates: uint32_t) -> int32_t {
    let mut check: bool = sampleRates
        == AUDIO_SAMPLE_RATE_8000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_12000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_11025 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_16000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_22050 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_24000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_32000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_44100 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_48000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_64000 as core::ffi::c_int as core::ffi::c_uint
        || sampleRates
            == AUDIO_SAMPLE_RATE_96000 as core::ffi::c_int as core::ffi::c_uint;
    if check {
        return HDF_SUCCESS as core::ffi::c_int as int32_t
    } else {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: FramatToSampleRate fail: Invalid sampling rate: %u.\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"DaiCheckSampleRate\0"))
                .as_ptr(),
            71 as core::ffi::c_int,
            sampleRates,
        );
        return HDF_ERR_NOT_SUPPORT as core::ffi::c_int as int32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DaiSetConfigInfoOfControls(mut data: *mut DaiData) -> int32_t {
    let mut index: uint16_t = 0;
    let mut regCfgGroup: *mut *mut AudioRegCfgGroupNode = 0
        as *mut *mut AudioRegCfgGroupNode;
    let mut patRegCfgItemTmp: *mut AudioMixerControl = 0 as *mut AudioMixerControl;
    let mut item: *mut AudioControlConfig = 0 as *mut AudioControlConfig;
    if data.is_null() || ((*data).regConfig).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: input para is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"DaiSetConfigInfoOfControls\0"))
                .as_ptr(),
            84 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    regCfgGroup = ((*(*data).regConfig).audioRegParams).as_mut_ptr();
    if regCfgGroup.is_null()
        || (*regCfgGroup.offset(AUDIO_CTRL_PATAM_GROUP as core::ffi::c_int as isize))
            .is_null()
        || (*regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
            .is_null()
    {
        return HDF_SUCCESS as core::ffi::c_int as int32_t;
    }
    patRegCfgItemTmp = (**regCfgGroup
        .offset(AUDIO_CTRL_PATAM_GROUP as core::ffi::c_int as isize))
        .regCfgItem;
    item = (**regCfgGroup.offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
        .ctrlCfgItem;
    if patRegCfgItemTmp.is_null() || item.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: patRegCfgItemTmp or item is NULL.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"DaiSetConfigInfoOfControls\0"))
                .as_ptr(),
            99 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    (*data).numControls = (**regCfgGroup
        .offset(AUDIO_CTRL_CFG_GROUP as core::ffi::c_int as isize))
        .itemNum as core::ffi::c_int;
    (*data).controls = OsalMemCalloc(
        ((*data).numControls as size_t)
            .wrapping_mul(::core::mem::size_of::<AudioKcontrol>() as size_t),
    ) as *mut AudioKcontrol;
    if ((*data).controls).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: OsalMemCalloc failed.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"DaiSetConfigInfoOfControls\0"))
                .as_ptr(),
            106 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    index = 0 as uint16_t;
    while (index as core::ffi::c_int) < (*data).numControls {
        (*((*data).controls).offset(index as isize)).iface = (*item
            .offset(index as isize))
            .iface as int32_t;
        if (*item.offset(index as isize)).arrayIndex as core::ffi::c_int
            >= AUDIO_CTRL_LIST_MAX
        {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                LOG_DOMAIN as core::ffi::c_uint,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
                b"[%s][line:%d]: Array super index.\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 27],
                    [core::ffi::c_char; 27],
                >(*b"DaiSetConfigInfoOfControls\0"))
                    .as_ptr(),
                113 as core::ffi::c_int,
            );
            OsalMemFree((*data).controls as *mut core::ffi::c_void);
            (*data).controls = 0 as *mut AudioKcontrol;
            return HDF_FAILURE as core::ffi::c_int as int32_t;
        }
        let ref mut fresh0 = (*((*data).controls).offset(index as isize)).name;
        *fresh0 = g_audioDaiControlsList[(*item.offset(index as isize)).arrayIndex
            as usize];
        let ref mut fresh1 = (*((*data).controls).offset(index as isize)).Info;
        *fresh1 = Some(
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
        (*((*data).controls).offset(index as isize)).privateValue = &mut *patRegCfgItemTmp
            .offset(index as isize) as *mut AudioMixerControl as *mut core::ffi::c_void
            as uintptr_t as core::ffi::c_ulong;
        let ref mut fresh2 = (*((*data).controls).offset(index as isize)).Get;
        *fresh2 = Some(
            AudioCpuDaiGetCtrlOps
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
        let ref mut fresh3 = (*((*data).controls).offset(index as isize)).Set;
        *fresh3 = Some(
            AudioCpuDaiSetCtrlOps
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
        index = index.wrapping_add(1);
    }
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn DaiDeviceReadReg(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut val: *mut uint32_t,
) -> int32_t {
    let mut virtualAddress: core::ffi::c_ulong = 0;
    if dai.is_null() || ((*dai).devData).is_null() || val.is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"DaiDeviceReadReg\0"))
                .as_ptr(),
            132 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    virtualAddress = (*(*dai).devData).regVirtualAddr;
    *val = ({
        let mut r: UINT32 = *(virtualAddress.wrapping_add(reg as core::ffi::c_ulong)
            as uintptr_t as *mut core::ffi::c_void as UINTPTR as *mut UINT32);
        asm!("dsb\n", options(preserves_flags));
        r
    }) as uint32_t;
    return HDF_SUCCESS as core::ffi::c_int as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn DaiDeviceWriteReg(
    mut dai: *const DaiDevice,
    mut reg: uint32_t,
    mut value: uint32_t,
) -> int32_t {
    let mut virtualAddress: core::ffi::c_ulong = 0;
    if dai.is_null() || ((*dai).devData).is_null() {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            LOG_DOMAIN as core::ffi::c_uint,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const core::ffi::c_char,
            b"[%s][line:%d]: param val is null.\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"DaiDeviceWriteReg\0"))
                .as_ptr(),
            144 as core::ffi::c_int,
        );
        return HDF_FAILURE as core::ffi::c_int as int32_t;
    }
    virtualAddress = (*(*dai).devData).regVirtualAddr;
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
