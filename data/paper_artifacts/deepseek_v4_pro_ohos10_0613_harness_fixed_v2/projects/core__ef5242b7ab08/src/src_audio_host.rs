//! Module: src_audio_host
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// -- Iterator helper for g_cardManager -------------------------------------------------

/// Walk the g_cardManager list, calling `f` for each AudioCard.
/// SAFETY: `head` must point to a valid circular DListHead.
#[inline]
unsafe fn for_each_audio_card<F: FnMut(*mut AudioCard) -> bool>(head: *mut DListHead, mut f: F) {
    let offset = unsafe {
        let null = std::ptr::null::<AudioCard>();
        core::ptr::addr_of!((*null).list) as usize
    };
    let mut node = unsafe { (*head).next };
    while node != head {
        let card = unsafe { (node as *mut u8).sub(offset) as *mut AudioCard };
        if f(card) {
            break;
        }
        node = unsafe { (*node).next };
    }
}

pub extern "C" fn GetAllCardInstance()-> *mut crate::types::DListHead {
    // Safely check if g_cardManager has been initialized (self-referencing empty head).
    // If not initialized, treat as empty.
    let head: *mut crate::types::DListHead = crate::globals::g_cardManager.get_ptr();
    if unsafe { (*head).next.is_null() || (*head).next == head } {
        // Not initialized or empty
        return core::ptr::null_mut();
    }
    head
}

pub extern "C" fn GetCardInstance(serviceName: *const std::ffi::c_char)-> *mut crate::types::AudioCard {
    if serviceName.is_null() {
        return std::ptr::null_mut();
    }
    let head = unsafe { crate::src_audio_host::GetAllCardInstance() as *mut crate::types::DListHead };
    if head.is_null() {
        return std::ptr::null_mut();
    }
    if unsafe { (*head).next == head } {
        return std::ptr::null_mut();
    }
    let service_cstr = unsafe { std::ffi::CStr::from_ptr(serviceName) };
    let mut found: *mut crate::types::AudioCard = std::ptr::null_mut();
    unsafe {
        for_each_audio_card(head, |card| {
            let card_service_name = (*card).configData.cardServiceName;
            if !card_service_name.is_null() {
                let card_cstr = std::ffi::CStr::from_ptr(card_service_name);
                if card_cstr == service_cstr {
                    found = card;
                    return true;
                }
            }
            false
        });
    }
    found
}

fn AudioCodecDevInit(audioCard: *mut crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        return crate::types::HDF_ERR_IO;
    }
    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        return crate::types::HDF_ERR_IO;
    }
    let rtd = unsafe { &*rtd };
    let codec = rtd.codec;
    if !codec.is_null() {
        let codec = unsafe { &*codec };
        let dev_data = codec.devData;
        if !dev_data.is_null() {
            let dev_data = unsafe { &*dev_data };
            if let Some(init_fn) = dev_data.Init {
                let ret = unsafe { init_fn(audioCard, codec as *const crate::types::CodecDevice) };
                if ret != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_ERR_IO;
                }
            }
        }
    }
    crate::types::HDF_SUCCESS
}

fn AudioPlatformDevInit(audioCard: *const crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        unsafe {
            HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                b"[%s][line:%d]: input param is NULL.\0" as *const u8 as *const i8,
                b"AudioPlatformDevInit\0" as *const u8 as *const i8,
                95i32);
        }
        return HDF_ERR_IO as i32;
    }

    let _ = (); // empty do {} while(0) from original

    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        unsafe {
            HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                b"[%s][line:%d]: Platform rtd is NULL.\0" as *const u8 as *const i8,
                b"AudioPlatformDevInit\0" as *const u8 as *const i8,
                102i32);
        }
        return HDF_ERR_IO as i32;
    }

    let rtd = unsafe { &*rtd };
    let platform = rtd.platform;
    if !platform.is_null() {
        let platform = unsafe { &*platform };
        let dev_data = platform.devData;
        if !dev_data.is_null() {
            let dev_data = unsafe { &*dev_data };
            if let Some(init_fn) = dev_data.PlatformInit {
                let ret = unsafe { init_fn(audioCard, platform as *const PlatformDevice) };
                if ret != HDF_SUCCESS as i32 {
                    unsafe {
                        HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510,
                            b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
                            b"[%s][line:%d]: platform initialization fail ret=%d\0" as *const u8 as *const i8,
                            b"AudioPlatformDevInit\0" as *const u8 as *const i8,
                            110i32,
                            ret);
                    }
                    return HDF_ERR_IO as i32;
                }
            }
        }
    }

    unsafe {
        HiLogPrint(LOG_CORE, LOG_INFO, 0xD002510,
            b"HDF_AUDIO_KADM\0" as *const u8 as *const i8,
            b"[%s][line:%d]: success.\0" as *const u8 as *const i8,
            b"AudioPlatformDevInit\0" as *const u8 as *const i8,
            115i32);
    }
    HDF_SUCCESS as i32
}

fn AudioDspDevInit(audioCard: *const crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        let func = b"AudioDspDevInit\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"[%s][line:%d]: audioCard is NULL.\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag, fmt, func, 125i32);
        }
        return crate::types::HDF_ERR_IO;
    }

    // empty do-while
    {}

    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        let func = b"AudioDspDevInit\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"[%s][line:%d]: audioCard rtd object is NULL.\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag, fmt, func, 132i32);
        }
        return crate::types::HDF_ERR_IO;
    }

    let rtd = unsafe { &*rtd };
    let dsp = rtd.dsp;
    if !dsp.is_null() {
        let dsp = unsafe { &*dsp };
        let dev_data = dsp.devData;
        if !dev_data.is_null() {
            let dev_data = unsafe { &*dev_data };
            if let Some(dsp_init) = dev_data.DspInit {
                let ret = unsafe { dsp_init(dsp as *const crate::types::DspDevice as *mut crate::types::DspDevice) };
                if ret != crate::types::HDF_SUCCESS {
                    let func = b"AudioDspDevInit\0".as_ptr() as *const ::core::ffi::c_char;
                    let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char;
                    let fmt = b"[%s][line:%d]: dsp initialization fail ret=%d\0".as_ptr() as *const ::core::ffi::c_char;
                    unsafe {
                        HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag, fmt, func, 141i32, ret);
                    }
                    return crate::types::HDF_ERR_IO;
                }
            }
        }
    }

    let func = b"AudioDspDevInit\0".as_ptr() as *const ::core::ffi::c_char;
    let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char;
    let fmt = b"[%s][line:%d]: success.\0".as_ptr() as *const ::core::ffi::c_char;
    unsafe {
        HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD002510u32, tag, fmt, func, 146i32);
    }

    crate::types::HDF_SUCCESS
}

fn AudioCodecDaiDevInit(audioCard: *mut crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        // stubbed: HiLogPrint(LOG_CORE, LOG_ERROR, ...)
        return crate::types::HDF_ERR_IO as i32;
    }

    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        // stubbed: HiLogPrint(LOG_CORE, LOG_ERROR, ...)
        return crate::types::HDF_ERR_IO as i32;
    }

    let rtd = unsafe { &*rtd };
    let codecDai = rtd.codecDai;
    if !codecDai.is_null() {
        let codecDai = unsafe { &*codecDai };
        let dev_data = codecDai.devData;
        if !dev_data.is_null() {
            let dev_data = unsafe { &*dev_data };
            if let Some(init_fn) = dev_data.DaiInit {
                let ret = unsafe { init_fn(audioCard, codecDai as *const crate::types::DaiDevice as *mut crate::types::DaiDevice) };
                if ret != crate::types::HDF_SUCCESS as i32 {
                    // stubbed: HiLogPrint(LOG_CORE, LOG_ERROR, ...)
                    return crate::types::HDF_ERR_IO as i32;
                }
            }
        }
    }

    // stubbed: HiLogPrint(LOG_CORE, LOG_INFO, ...)
    crate::types::HDF_SUCCESS as i32
}

fn AudioCpuDaiDevInit(audioCard: *mut crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: audioCard is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioCpuDaiDevInit\0".as_ptr() as *const ::core::ffi::c_char,
                186i32,
            );
        }
        return crate::types::HDF_ERR_IO;
    }

    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: cpuDai rtd is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioCpuDaiDevInit\0".as_ptr() as *const ::core::ffi::c_char,
                193i32,
            );
        }
        return crate::types::HDF_ERR_IO;
    }

    let rtd = unsafe { &*rtd };
    let cpuDai = rtd.cpuDai;
    if !cpuDai.is_null() {
        let cpuDai = unsafe { &*cpuDai };
        let devData = cpuDai.devData;
        if !devData.is_null() {
            let devData = unsafe { &*devData };
            if let Some(init_fn) = devData.DaiInit {
                let ret = unsafe { init_fn(audioCard, cpuDai as *const crate::types::DaiDevice as *mut crate::types::DaiDevice) };
                if ret != crate::types::HDF_SUCCESS {
                    unsafe {
                        let _ = HiLogPrint(
                            crate::types::LOG_CORE,
                            crate::types::LOG_ERROR,
                            0xD002510u32,
                            b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                            b"[%s][line:%d]: cpu dai initialization fail ret=%d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"AudioCpuDaiDevInit\0".as_ptr() as *const ::core::ffi::c_char,
                            201i32,
                            ret,
                        );
                    }
                    return crate::types::HDF_ERR_IO;
                }
            }
        }
    }

    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510u32,
            b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s][line:%d]: success.\0".as_ptr() as *const ::core::ffi::c_char,
            b"AudioCpuDaiDevInit\0".as_ptr() as *const ::core::ffi::c_char,
            206i32,
        );
    }
    crate::types::HDF_SUCCESS
}

fn AudioDspDaiDevInit(audioCard: *mut crate::types::AudioCard) -> i32 {
    if audioCard.is_null() {
        return crate::types::HDF_ERR_IO;
    }
    let card = unsafe { &*audioCard };
    let rtd = card.rtd;
    if rtd.is_null() {
        return crate::types::HDF_ERR_IO;
    }
    let rtd = unsafe { &*rtd };
    let dspDai = rtd.dspDai;
    if !dspDai.is_null() {
        let dspDai = unsafe { &*dspDai };
        let devData = dspDai.devData;
        if !devData.is_null() {
            let devData = unsafe { &*devData };
            if let Some(dai_init) = devData.DaiInit {
                let ret = unsafe { dai_init(audioCard, dspDai as *const crate::types::DaiDevice) };
                if ret != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_ERR_IO;
                }
            }
        }
    }
    crate::types::HDF_SUCCESS
}

fn AudioInitDaiLink(audioCard: *mut crate::types::AudioCard)-> i32 {
    if audioCard.is_null() {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: audioCard is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                244i32,
            )
        };
        return HDF_ERR_IO;
    }

    if AudioPlatformDevInit(audioCard as *const crate::types::AudioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Platform init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                250i32,
            )
        };
        return HDF_FAILURE;
    }

    if AudioCpuDaiDevInit(audioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: CpuDai init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                255i32,
            )
        };
        return HDF_FAILURE;
    }

    if AudioCodecDevInit(audioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: codec Device init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                259i32,
            )
        };
        return HDF_FAILURE;
    }

    if AudioCodecDaiDevInit(audioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: CodecDai Device init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                264i32,
            )
        };
        return HDF_FAILURE;
    }

    if AudioDspDevInit(audioCard as *const crate::types::AudioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: Dsp Device init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                269i32,
            )
        };
        return HDF_FAILURE;
    }

    if AudioDspDaiDevInit(audioCard) != HDF_SUCCESS {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s][line:%d]: DspDai Device init fail.\0".as_ptr() as *const ::core::ffi::c_char,
                b"AudioInitDaiLink\0".as_ptr() as *const ::core::ffi::c_char,
                274i32,
            )
        };
        return HDF_FAILURE;
    }

    HDF_SUCCESS
}

pub extern "C" fn AudioHostCreateAndBind(device: *mut crate::types::HdfDeviceObject)-> *mut crate::types::AudioHost {
    if device.is_null() {
        let tag = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0") };
        let msg = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: device is NULL!\0") };
        let func = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioHostCreateAndBind\0") };
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                msg.as_ptr(),
                func.as_ptr(),
                287i32,
            )
        };
        return std::ptr::null_mut();
    }

    let size = std::mem::size_of::<crate::types::AudioHost>();
    let audioHost = unsafe { OsalMemCalloc((size as usize).try_into().unwrap()) as *mut crate::types::AudioHost };
    if audioHost.is_null() {
        let tag = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0") };
        let msg =
            unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: Malloc audio host fail!\0") };
        let func = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioHostCreateAndBind\0") };
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                tag.as_ptr(),
                msg.as_ptr(),
                func.as_ptr(),
                293i32,
            )
        };
        return std::ptr::null_mut();
    }

    unsafe {
        (*audioHost).device = device;
        (*device).service =
            core::ptr::addr_of_mut!((*audioHost).service) as *mut crate::types::IDeviceIoService;
    }

    audioHost
}

pub(crate) unsafe extern "C" fn AudioDriverBind(device: *mut crate::types::HdfDeviceObject)-> i32 {
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let audio_host = crate::src_audio_host::AudioHostCreateAndBind(device);
    if audio_host.is_null() {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn AudioCardInit(device: *mut crate::types::HdfDeviceObject, audioHost: *mut crate::types::AudioHost)-> i32 {
    if device.is_null() || audioHost.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let audioCard: *mut crate::types::AudioCard = unsafe {
        crate::compat::OsalMemCalloc(core::mem::size_of::<crate::types::AudioCard>() as u32) as *mut crate::types::AudioCard
    };
    if audioCard.is_null() {
        return crate::types::HDF_FAILURE;
    }

    // Store into audioHost->priv (raw pointer write, no aliasing concern)
    unsafe { (*audioHost).priv_ = audioCard as *mut core::ffi::c_void; }

    // Safe initialisation of the card's fields and internal DListHeads
    {
        let card_ref = unsafe { &mut *audioCard };

        let ret = unsafe { crate::src_audio_parse::AudioFillConfigData(device, &mut card_ref.configData) };
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_ERR_IO;
        }

        card_ref.device = device;
        card_ref.standbyMode = crate::types::AUDIO_SAPM_TURN_STANDBY_LATER;

        dlist_self_init(&mut card_ref.list as *mut crate::types::DListHead);
        dlist_self_init(&mut card_ref.controls as *mut crate::types::DListHead);
        dlist_self_init(&mut card_ref.components as *mut crate::types::DListHead);
        dlist_self_init(&mut card_ref.paths as *mut crate::types::DListHead);
        dlist_self_init(&mut card_ref.sapmDirty as *mut crate::types::DListHead);
    }

    // From here, use raw pointer to avoid aliasing with extern calls
    let ret = unsafe { crate::src_audio_core::AudioBindDaiLink(audioCard, &(*audioCard).configData) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    let rtd = unsafe { (*audioCard).rtd };
    if rtd.is_null() {
        return crate::types::HDF_ERR_IO;
    }
    let complete = unsafe { (*rtd).complete };
    if complete == 0 {
        return crate::types::HDF_ERR_IO;
    }

    let ret = unsafe { crate::src_audio_host::AudioInitDaiLink(audioCard) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_ERR_IO;
    }

    // Add to global card manager list
    unsafe {
        let head: *mut crate::types::DListHead = crate::globals::g_cardManager.get_ptr();
        // If the global head hasn't been initialized yet, make it self-referencing.
        if (*head).next.is_null() {
            (*head).next = head;
            (*head).prev = head;
        }
        let entry: *mut crate::types::DListHead = &mut (*audioCard).list as *mut _;
        (*entry).next = (*head).next;
        (*entry).prev = head;
        (*(*head).next).prev = entry;
        (*head).next = entry;
    }

    crate::types::HDF_SUCCESS
}

fn dlist_self_init(list: *mut crate::types::DListHead) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

pub(crate) unsafe extern "C" fn AudioDriverInit(device: *mut crate::types::HdfDeviceObject)-> i32 {
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    if !unsafe { HdfDeviceSetClass(device, crate::types::DEVICE_CLASS_AUDIO) } {
        return crate::types::HDF_FAILURE;
    }
    let audioHost = unsafe { (*device).service as *mut crate::types::AudioHost };
    if crate::src_audio_host::AudioCardInit(device, audioHost) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn drain_intrusive_list<T>(
    head: *mut crate::types::DListHead,
    offset: isize,
    destroy: unsafe fn(*mut T),
) {
    if unsafe { (*head).next == head } {
        return;
    }
    let mut cur: *mut T = unsafe { ((*head).next as *mut u8).offset(-offset) as *mut T };
    loop {
        let list_ptr = unsafe { (cur as *mut u8).offset(offset) as *mut crate::types::DListHead };
        if list_ptr == head {
            break;
        }
        let next = unsafe { (*list_ptr).next };
        let tmp: *mut T = unsafe { (next as *mut u8).offset(-offset) as *mut T };
        unsafe { destroy(cur); }
        cur = tmp;
    }
}

unsafe fn free_audio_component(comp: *mut crate::types::AudioSapmComponent) {
    unsafe {
        crate::src_audio_core::dlist_remove(core::ptr::addr_of_mut!((*comp).list));
        OsalMemFree((*comp).componentName as *mut ::core::ffi::c_void);
        OsalMemFree(comp as *mut ::core::ffi::c_void);
    }
}

unsafe fn free_audio_control(ctrl: *mut crate::types::AudioKcontrol) {
    unsafe {
        crate::src_audio_core::dlist_remove(core::ptr::addr_of_mut!((*ctrl).list));
        OsalMemFree((*ctrl).privateData);
        OsalMemFree(ctrl as *mut ::core::ffi::c_void);
    }
}

pub(crate) extern "C" fn AudioDriverRelease(device: *mut crate::types::HdfDeviceObject) {
    if device.is_null() {
        return;
    }
    let audioHost = unsafe { (*device).service as *mut crate::types::AudioHost };
    if audioHost.is_null() {
        return;
    }

    let priv_ = {
        let host_ref = unsafe { &*audioHost };
        host_ref.priv_
    };
    if !priv_.is_null() {
        let audioCard = priv_ as *mut crate::types::AudioCard;
        let card_ref = unsafe { &mut *audioCard };

        // Free components
        let component_head: *mut crate::types::DListHead = core::ptr::addr_of_mut!(card_ref.components);
        let offset_component: isize =
            unsafe { core::ptr::addr_of!((*core::ptr::null::<crate::types::AudioSapmComponent>()).list) as isize };
        if unsafe { (*component_head).next != component_head } {
            drain_intrusive_list::<crate::types::AudioSapmComponent>(
                component_head,
                offset_component,
                free_audio_component,
            );
        }

        // Free controls
        let control_head: *mut crate::types::DListHead = core::ptr::addr_of_mut!(card_ref.controls);
        let offset_kcontrol: isize =
            unsafe { core::ptr::addr_of!((*core::ptr::null::<crate::types::AudioKcontrol>()).list) as isize };
        if unsafe { (*control_head).next != control_head } {
            drain_intrusive_list::<crate::types::AudioKcontrol>(
                control_head,
                offset_kcontrol,
                free_audio_control,
            );
        }

        // Remove card from list and free rtd / priv
        unsafe { crate::src_audio_core::dlist_remove(core::ptr::addr_of_mut!(card_ref.list)); }
        unsafe { OsalMemFree(card_ref.rtd as *mut ::core::ffi::c_void); }
        unsafe { OsalMemFree(priv_); }
    }
    unsafe { OsalMemFree(audioHost as *mut ::core::ffi::c_void); }
}
