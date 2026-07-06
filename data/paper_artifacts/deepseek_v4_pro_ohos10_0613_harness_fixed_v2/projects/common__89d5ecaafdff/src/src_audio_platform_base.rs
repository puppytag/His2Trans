//! Module: src_audio_platform_base
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

pub extern "C" fn SysReadl(addr: ::core::ffi::c_ulong) -> u32 {
    unsafe { core::ptr::read_volatile(addr as usize as *const u32) }
}

pub extern "C" fn SysWritel(addr: ::core::ffi::c_ulong, value: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, value);
    }
}

pub extern "C" fn PlatformDataFromCard(card: *const crate::types::AudioCard) -> *mut crate::types::PlatformData {
    if card.is_null() {
        let fmt = b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8;
        let func = b"PlatformDataFromCard\0".as_ptr() as *const i8;
        let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const i8;
        unsafe {
            HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag, fmt, func, 45i32);
        }
        return std::ptr::null_mut();
    }
    let rtd = unsafe { (*card).rtd };
    if rtd.is_null() {
        let fmt = b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8;
        let func = b"PlatformDataFromCard\0".as_ptr() as *const i8;
        let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const i8;
        unsafe {
            HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag, fmt, func, 45i32);
        }
        return std::ptr::null_mut();
    }
    let platform = unsafe { (*rtd).platform };
    if platform.is_null() {
        let fmt = b"[%s][line:%d]: param is null.\0".as_ptr() as *const i8;
        let func = b"PlatformDataFromCard\0".as_ptr() as *const i8;
        let tag = b"HDF_AUDIO_KADM\0".as_ptr() as *const i8;
        unsafe {
            HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, tag, fmt, func, 45i32);
        }
        return std::ptr::null_mut();
    }
    return unsafe { (*platform).devData };
}

pub extern "C" fn AudioBytesToFrames(frameBits: u32, size: u32) -> u32 {
    if size == 0 {
        unsafe {
            let tag: *const i8 = b"HDF_AUDIO_KADM\0".as_ptr() as *const i8;
            let fmt: *const i8 = b"[%s][line:%d]: size is null.\0".as_ptr() as *const i8;
            let func: *const i8 = b"AudioBytesToFrames\0".as_ptr() as *const i8;
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                tag,
                fmt,
                func,
                54i32,
            );
        }
        return 0;
    } else {
        return frameBits / size;
    }
}

pub extern "C" fn AudioDataBigEndianChange(srcData: *mut ::core::ffi::c_char, audioLen: u32, bitWidth: crate::types::DataBitWidth) -> i32 {
    if srcData.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let mut change_data: *mut ::core::ffi::c_char = srcData;
    let mut p_data: *mut u32 = change_data as *mut u32;

    match bitWidth {
        crate::types::DATA_BIT_WIDTH8 => {
            return crate::types::HDF_SUCCESS;
        },
        crate::types::DATA_BIT_WIDTH24 => {
            let framesize: u32 = 3;
            let mut i: u32 = 0;
            while i < audioLen {
                unsafe {
                    *p_data = ((*p_data >> 0x10) & 0x000000FF) |
                               (*p_data & 0xFF00FF00) |
                               ((*p_data << 0x10) & 0x00FF0000);
                }
                unsafe {
                    change_data = change_data.add(framesize as usize);
                }
                p_data = change_data as *mut u32;
                i += framesize;
            }
        },
        _ => { // DATA_BIT_WIDTH16 or default
            let framesize: u32 = 4;
            let mut i: u32 = 0;
            while i < audioLen {
                unsafe {
                    *p_data = ((*p_data << 0x08) & 0xFF00FF00) |
                               ((*p_data >> 0x08) & 0x00FF00FF);
                    p_data = p_data.add(1);
                }
                i += framesize;
            }
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioFormatToBitWidth(format: crate::types::AudioFormat, bitWidth: *mut u32) -> i32 {
    if bitWidth.is_null() {
        unsafe { libc::printf(b"AudioFormatToBitWidth: bitWidth is null.\n\0".as_ptr() as *const i8); }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let bw = unsafe { &mut *bitWidth };
    match format {
        crate::types::AUDIO_FORMAT_TYPE_PCM_8_BIT => *bw = crate::types::DATA_BIT_WIDTH8,
        crate::types::AUDIO_FORMAT_TYPE_PCM_16_BIT => *bw = crate::types::DATA_BIT_WIDTH16,
        crate::types::AUDIO_FORMAT_TYPE_PCM_24_BIT => *bw = crate::types::DATA_BIT_WIDTH24,
        crate::types::AUDIO_FORMAT_TYPE_PCM_32_BIT => *bw = crate::types::DATA_BIT_WIDTH32,
        _ => {
            unsafe { libc::printf(b"AudioFormatToBitWidth: format is not define.\n\0".as_ptr() as *const i8); }
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSetPcmInfo(platformData: *mut crate::types::PlatformData, param: *const crate::types::AudioPcmHwParams) -> i32 {
    if platformData.is_null() || param.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let data = unsafe { &mut *platformData };
    data.renderBufInfo.chnId = 0;
    data.captureBufInfo.chnId = 0;
    let param = unsafe { &*param };
    if param.streamType == crate::types::AUDIO_RENDER_STREAM {
        let ret = crate::src_audio_platform_base::AudioFormatToBitWidth(
            param.format,
            core::ptr::addr_of_mut!(data.renderPcmInfo.bitWidth) as *mut u32,
        );
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
        data.renderPcmInfo.rate = param.rate;
        data.renderPcmInfo.frameSize =
            param.channels.wrapping_mul(data.renderPcmInfo.bitWidth) / 8;
        data.renderPcmInfo.channels = param.channels;
        data.renderPcmInfo.isBigEndian = param.isBigEndian;
        data.renderPcmInfo.isSignedData = param.isSignedData;
        data.renderPcmInfo.startThreshold = param.startThreshold;
        data.renderPcmInfo.stopThreshold = param.stopThreshold;
        data.renderPcmInfo.silenceThreshold = param.silenceThreshold;
        data.renderPcmInfo.interleaved = 1;
        data.renderPcmInfo.streamType = param.streamType;
    } else if param.streamType == crate::types::AUDIO_CAPTURE_STREAM {
        let ret = crate::src_audio_platform_base::AudioFormatToBitWidth(
            param.format,
            core::ptr::addr_of_mut!(data.capturePcmInfo.bitWidth) as *mut u32,
        );
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
        data.capturePcmInfo.rate = param.rate;
        data.capturePcmInfo.frameSize =
            param.channels.wrapping_mul(data.capturePcmInfo.bitWidth) / 8;
        data.capturePcmInfo.channels = param.channels;
        data.capturePcmInfo.isBigEndian = param.isBigEndian;
        data.capturePcmInfo.isSignedData = param.isSignedData;
        data.capturePcmInfo.startThreshold = param.startThreshold;
        data.capturePcmInfo.stopThreshold = param.stopThreshold;
        data.capturePcmInfo.silenceThreshold = param.silenceThreshold;
        data.capturePcmInfo.interleaved = 1;
        data.capturePcmInfo.streamType = param.streamType;
    } else {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSetRenderBufInfo(data: *mut crate::types::PlatformData, param: *const crate::types::AudioPcmHwParams) -> i32 {
    if data.is_null() || param.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let period = {
        let param_ref = unsafe { &*param };
        param_ref.period
    };
    if period < 2048 || period > 8192 {
        return crate::types::HDF_FAILURE;
    }
    let (bit_width, channels) = {
        let data_ref = unsafe { &*data };
        (data_ref.renderPcmInfo.bitWidth, data_ref.renderPcmInfo.channels)
    };
    let period_size = period * bit_width * channels / crate::types::BITSTOBYTE;
    let period_count = unsafe { crate::globals::PERIOD_COUNT } as u32;
    let size = period_count * period_size;
    if size < crate::types::MIN_BUFF_SIZE || size > crate::types::MAX_BUFF_SIZE {
        return crate::types::HDF_FAILURE;
    }
    let data_mut = unsafe { &mut *data };
    data_mut.renderBufInfo.periodSize = period_size;
    data_mut.renderBufInfo.periodCount = period_count;
    data_mut.renderBufInfo.trafBufSize = unsafe { crate::globals::RENDER_TRAF_BUF_SIZE } as u32;
    data_mut.renderBufInfo.cirBufSize = size;
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioSetCaptureBufInfo(data: *mut crate::types::PlatformData, param: *const crate::types::AudioPcmHwParams) -> i32 {
    if data.is_null() || param.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let period = {
        let param_ref = unsafe { &*param };
        param_ref.period
    };
    if period < (2 * 1024u32) || period > (8 * 1024u32) {
        return crate::types::HDF_FAILURE;
    }

    let (bit_width, channels, cir_buf_max) = {
        let data_ref = unsafe { &*data };
        (data_ref.capturePcmInfo.bitWidth, data_ref.capturePcmInfo.channels, data_ref.captureBufInfo.cirBufMax)
    };
    let period_size = period.wrapping_mul(bit_width).wrapping_mul(channels) / 8;
    let silence_threshold = {
        let param_ref = unsafe { &*param };
        param_ref.silenceThreshold
    };
    let min_threshold = unsafe { crate::globals::MIN_PERIOD_SILENCE_THRESHOLD } as u32;
    let max_threshold = unsafe { crate::globals::MAX_PERIOD_SILENCE_THRESHOLD } as u32;
    if silence_threshold < min_threshold || silence_threshold > max_threshold {
        return crate::types::HDF_FAILURE;
    }

    let period_count = unsafe { crate::globals::PERIOD_COUNT } as u32;
    let cir_buf_size = period_size.wrapping_mul(period_count);
    if cir_buf_size > cir_buf_max {
        return crate::types::HDF_FAILURE;
    }

    let data_mut = unsafe { &mut *data };
    data_mut.captureBufInfo.periodSize = period_size;
    data_mut.captureBufInfo.periodCount = period_count;
    data_mut.captureBufInfo.trafBufSize = silence_threshold;
    data_mut.captureBufInfo.cirBufSize = cir_buf_size;

    crate::types::HDF_SUCCESS
}

fn AudioDmaBuffStatus(card: *const crate::types::AudioCard, streamType: crate::types::AudioStreamType)-> i32 {
    let mut pointer: u32 = 0;
    let data: *mut crate::types::PlatformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        eprintln!("HDF_AUDIO_KADM: [AudioDmaBuffStatus][line:253]: PlatformDataFromCard failed.");
        return HDF_FAILURE;
    }
    let data_ref = unsafe { &*data };
    if data_ref.ops.is_null() {
        eprintln!("HDF_AUDIO_KADM: [AudioDmaBuffStatus][line:253]: ops is null.");
        return HDF_FAILURE;
    }

    if crate::src_audio_platform_base::AudioPcmPointer(card, &mut pointer as *mut u32, streamType) != HDF_SUCCESS {
        eprintln!("HDF_AUDIO_KADM: [AudioDmaBuffStatus][line:258]: get Pointer failed.");
        return ENUM_CIR_BUFF_FULL;
    }

    if streamType == AUDIO_RENDER_STREAM {
        let data = unsafe { &mut *data };
        data.renderBufInfo.pointer = pointer;
        let rptr = data.renderBufInfo.pointer.wrapping_mul(data.renderPcmInfo.frameSize);
        let dataAvailable = (data.renderBufInfo.wbufOffSet.wrapping_sub(rptr)) % data.renderBufInfo.cirBufSize;
        let residual = data.renderBufInfo.cirBufSize.wrapping_sub(dataAvailable);
        if residual > data.renderBufInfo.trafBufSize {
            return ENUM_CIR_BUFF_NORMAL;
        } else {
            return ((data.renderBufInfo.trafBufSize.wrapping_sub(residual) / data.renderBufInfo.oneMsBytes) as i32);
        }
    } else if streamType == AUDIO_CAPTURE_STREAM {
        let data = unsafe { &mut *data };
        let rptr = data.captureBufInfo.rptrOffSet;
        let wptr = pointer.wrapping_mul(data.capturePcmInfo.frameSize);
        data.captureBufInfo.pointer = pointer;
        if wptr >= rptr {
            let dataAvailable = wptr - rptr;
            if dataAvailable < data.captureBufInfo.trafBufSize {
                return ((data.captureBufInfo.trafBufSize.wrapping_sub(dataAvailable) / data.captureBufInfo.oneMsBytes) as i32);
            }
        }
        return ENUM_CIR_BUFF_NORMAL;
    } else {
        eprintln!("HDF_AUDIO_KADM: [AudioDmaBuffStatus][line:292]: streamType is invalid.");
        return HDF_FAILURE;
    }
}

pub extern "C" fn AudioPcmWrite(card: *const crate::types::AudioCard, txData: *mut crate::types::AudioTxData) -> i32 {
    if card.is_null() || txData.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let buf = unsafe { (*txData).buf };
    if buf.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let data_ptr = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data_ptr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let data = unsafe { &mut *data_ptr };
    let txData = unsafe { &mut *txData };
    let frames = txData.frames as u64;
    let frame_size = data.renderPcmInfo.frameSize as u64;
    let traf_buf_size = (frames * frame_size) as u32;
    data.renderBufInfo.trafBufSize = traf_buf_size;
    if data.renderPcmInfo.isBigEndian {
        let ret = crate::src_audio_platform_base::AudioDataBigEndianChange(
            buf,
            traf_buf_size,
            data.renderPcmInfo.bitWidth as crate::types::DataBitWidth,
        );
        if ret != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
    }
    let status = crate::src_audio_platform_base::AudioDmaBuffStatus(card, crate::types::AUDIO_RENDER_STREAM);
    if status != crate::types::ENUM_CIR_BUFF_NORMAL {
        txData.status = status;
        return crate::types::HDF_SUCCESS;
    }
    let cir_buf_size = data.renderBufInfo.cirBufSize;
    if traf_buf_size > cir_buf_size {
        return crate::types::HDF_FAILURE;
    }
    let wbuf_off_set = data.renderBufInfo.wbufOffSet;
    let w_ptr = wbuf_off_set % cir_buf_size;
    let virt_addr = data.renderBufInfo.virtAddr;
    if virt_addr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dest = (virt_addr as *mut u8).wrapping_add(w_ptr as usize);
    let src = buf as *const u8;
    let count = traf_buf_size as usize;
    if count > 0 {
        unsafe { core::ptr::copy_nonoverlapping(src, dest, count); }
    }
    txData.status = crate::types::ENUM_CIR_BUFF_NORMAL;
    data.renderBufInfo.wptrOffSet = w_ptr + traf_buf_size;
    data.renderBufInfo.wbufOffSet += traf_buf_size;
    crate::types::HDF_SUCCESS
}

fn PcmReadData(data: *mut crate::types::PlatformData, rxData: *mut crate::types::AudioRxData)-> i32 {
    if data.is_null() || rxData.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let data = unsafe { &mut *data };
    let rxData = unsafe { &mut *rxData };
    let rx_buf = unsafe {
        (data.captureBufInfo.virtAddr as *mut u8)
            .offset(data.captureBufInfo.rptrOffSet as isize) as *mut ::core::ffi::c_char
    };
    let wptr: u32 = data.captureBufInfo.pointer * data.capturePcmInfo.frameSize;
    let rptr: u32 = data.captureBufInfo.rptrOffSet;
    let traf_buf_size = data.captureBufInfo.trafBufSize;
    let cir_buf_size = data.captureBufInfo.cirBufSize;
    data.captureBufInfo.curTrafSize = traf_buf_size;
    if rptr > wptr {
        let validDataSize = cir_buf_size - rptr;
        if validDataSize < traf_buf_size {
            data.captureBufInfo.curTrafSize = validDataSize;
        }
    }
    if data.capturePcmInfo.isBigEndian {
        if rx_buf.is_null()
            || AudioDataBigEndianChange(
                rx_buf,
                data.captureBufInfo.curTrafSize,
                data.capturePcmInfo.bitWidth,
            ) != crate::types::HDF_SUCCESS
        {
            return crate::types::HDF_FAILURE;
        }
    }
    let cur_traf_size = data.captureBufInfo.curTrafSize;
    let frame_size = data.capturePcmInfo.frameSize;
    rxData.buf = rx_buf;
    rxData.frames = (cur_traf_size / frame_size) as u64;
    rxData.bufSize = cur_traf_size as u64;
    rxData.status = crate::types::ENUM_CIR_BUFF_NORMAL;
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioPcmRead(card: *const crate::types::AudioCard, rxData: *mut crate::types::AudioRxData) -> i32 {
    if card.is_null() || rxData.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let data: *mut crate::types::PlatformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let data_ref = unsafe { &mut *data };
    if data_ref.captureBufInfo.virtAddr.is_null() {
        return crate::types::HDF_FAILURE;
    }

    let status: i32 = crate::src_audio_platform_base::AudioDmaBuffStatus(card, crate::types::AUDIO_CAPTURE_STREAM);
    if status != crate::types::ENUM_CIR_BUFF_NORMAL {
        let rx = unsafe { &mut *rxData };
        rx.status = status;
        let buf_ptr: *mut ::core::ffi::c_char = unsafe {
            (data_ref.captureBufInfo.virtAddr as *mut u8)
                .add(data_ref.captureBufInfo.rptrOffSet as usize) as *mut ::core::ffi::c_char
        };
        rx.buf = buf_ptr;
        rx.frames = 0;
        rx.bufSize = 0;
        return crate::types::HDF_SUCCESS;
    }

    let ret: i32 = crate::src_audio_platform_base::PcmReadData(data, rxData);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    data_ref.captureBufInfo.rptrOffSet += data_ref.captureBufInfo.curTrafSize;
    if data_ref.captureBufInfo.rptrOffSet >= data_ref.captureBufInfo.cirBufSize {
        data_ref.captureBufInfo.rptrOffSet = 0;
    }
    data_ref.captureBufInfo.rbufOffSet += data_ref.captureBufInfo.curTrafSize;

    crate::types::HDF_SUCCESS
}

fn MmapWriteData(data: *mut crate::types::PlatformData, tmpBuf: *mut std::ffi::c_char)-> i32 {
    if data.is_null() {
        return HDF_FAILURE as i32;
    }
    let data = unsafe { &mut *data };
    let traf_buf_size = data.renderBufInfo.trafBufSize;
    let cir_buf_size = data.renderBufInfo.cirBufSize;
    if traf_buf_size > cir_buf_size {
        return HDF_FAILURE as i32;
    }
    let wbuf_off_set = data.renderBufInfo.wbufOffSet;
    let w_ptr = wbuf_off_set % cir_buf_size;
    // CopyFromUser: copy from user-space mmap buffer to kernel tmpBuf
    unsafe {
        let ret = CopyFromUser(
            tmpBuf as *mut ::core::ffi::c_void,
            (data.mmapData.memoryAddress as *mut u8).add(data.mmapData.offset as usize) as *const ::core::ffi::c_void,
            traf_buf_size,
        );
        if ret != 0 {
            return HDF_FAILURE as i32;
        }
    }
    // memcpy_s: copy from tmpBuf to virtAddr
    unsafe {
        let ret = memcpy_s(
            (data.renderBufInfo.virtAddr as *mut u8).add(w_ptr as usize) as *mut ::core::ffi::c_void,
            traf_buf_size as crate::types::size_t,
            tmpBuf as *const ::core::ffi::c_void,
            traf_buf_size as crate::types::size_t,
        );
        if ret != 0 {
            return HDF_FAILURE as i32;
        }
    }
    data.renderBufInfo.wptrOffSet = w_ptr.wrapping_add(traf_buf_size);
    data.renderBufInfo.wbufOffSet = wbuf_off_set.wrapping_add(traf_buf_size);
    let frame_size = data.renderPcmInfo.frameSize;
    if frame_size != 0 {
        data.renderBufInfo.framesPosition =
            data.renderBufInfo.framesPosition.wrapping_add(traf_buf_size.wrapping_div(frame_size));
    }
    data.mmapData.offset = data.mmapData.offset.wrapping_add(traf_buf_size);
    data.mmapLoopCount = data.mmapLoopCount.wrapping_add(1);
    HDF_SUCCESS as i32
}

fn AudioRenderPlatformDataInit(data: *mut crate::types::PlatformData, totalSize: *mut u32, lastBuffSize: *mut u32, loopTimes: *mut u32)-> i32 {
    if data.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: PlatformDataFromCard failed.\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioRenderPlatformDataInit\0").as_ptr(),
                478i32,
            );
        }
        return crate::types::HDF_FAILURE as i32;
    }
    let data_ref = unsafe { &mut *data };
    if data_ref.renderBufInfo.virtAddr.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: render buffer is null.\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioRenderPlatformDataInit\0").as_ptr(),
                482i32,
            );
        }
        return crate::types::HDF_FAILURE as i32;
    }
    if data_ref.renderBufInfo.runStatus != crate::types::PCM_START {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"HDF_AUDIO_KADM\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"[%s][line:%d]: render did not start.\0").as_ptr(),
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"AudioRenderPlatformDataInit\0").as_ptr(),
                486i32,
            );
        }
        return crate::types::HDF_FAILURE as i32;
    }
    let total_frames = data_ref.mmapData.totalBufferFrames as u32;
    let frame_size = data_ref.renderPcmInfo.frameSize;
    let total = total_frames.wrapping_mul(frame_size);
    unsafe { *totalSize = total; }
    const MIN_PERIOD_SIZE: u32 = 2048;
    let last = if total % MIN_PERIOD_SIZE == 0 {
        MIN_PERIOD_SIZE
    } else {
        total % MIN_PERIOD_SIZE
    };
    unsafe { *lastBuffSize = last; }
    let loops = if last == MIN_PERIOD_SIZE {
        total / MIN_PERIOD_SIZE
    } else {
        total / MIN_PERIOD_SIZE + 1
    };
    unsafe { *loopTimes = loops; }
    data_ref.mmapLoopCount = 0;
    crate::types::HDF_SUCCESS as i32
}

fn AudioMmapWriteTransfer(card: *const crate::types::AudioCard)-> i32 {
    let mut timeout: u32 = 0;
    let mut totalSize: u32 = 0;
    let mut lastBuffSize: u32 = 0;
    let mut loopTimes: u32 = 0;
    let mut tmpBuf: *mut std::ffi::c_char = std::ptr::null_mut();

    let data = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card) };
    if data.is_null() { return HDF_FAILURE; }
    if crate::src_audio_platform_base::AudioRenderPlatformDataInit(
        data,
        &mut totalSize as *mut u32,
        &mut lastBuffSize as *mut u32,
        &mut loopTimes as *mut u32,
    ) == HDF_FAILURE
    {
        return HDF_FAILURE;
    }

    tmpBuf = unsafe { OsalMemCalloc(2048u32) as *mut std::ffi::c_char };
    if tmpBuf.is_null() {
        let tag: &'static [u8] = b"HDF_AUDIO_KADM\0";
        let func_name: &'static [u8] = b"AudioMmapWriteTransfer\0";
        let fmt: &'static [u8] = b"[%s][line:%d]: tmpBuf is null.\0";
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                tag.as_ptr() as *const ::core::ffi::c_char,
                fmt.as_ptr() as *const ::core::ffi::c_char,
                func_name.as_ptr() as *const ::core::ffi::c_char,
                512i32,
            );
        }
        return HDF_FAILURE;
    }

    let data_mut = unsafe { &mut *data };
    while data_mut.mmapLoopCount < loopTimes && data_mut.renderBufInfo.runStatus != PCM_STOP {
        if data_mut.renderBufInfo.runStatus == PCM_PAUSE {
            unsafe { OsalMSleep(SLEEP_TIME as u32) };
            continue;
        }

        if crate::src_audio_platform_base::AudioDmaBuffStatus(card, AUDIO_RENDER_STREAM)
            != ENUM_CIR_BUFF_NORMAL
        {
            unsafe { OsalMSleep(SLEEP_TIME as u32) };
            timeout += 1;
            if timeout >= unsafe { TIME_OUT_CONST as u32 } {
                unsafe { OsalMemFree(tmpBuf as *mut ::core::ffi::c_void) };
                let tag: &'static [u8] = b"HDF_AUDIO_KADM\0";
                let func_name: &'static [u8] = b"AudioMmapWriteTransfer\0";
                let fmt: &'static [u8] = b"[%s][line:%d]: timeout failed.\0";
                unsafe {
                    HiLogPrint(
                        LOG_CORE,
                        LOG_ERROR,
                        0xD002510u32,
                        tag.as_ptr() as *const ::core::ffi::c_char,
                        fmt.as_ptr() as *const ::core::ffi::c_char,
                        func_name.as_ptr() as *const ::core::ffi::c_char,
                        527i32,
                    );
                }
                return HDF_FAILURE;
            }
            continue;
        }

        timeout = 0;
        data_mut.renderBufInfo.trafBufSize =
            if data_mut.mmapLoopCount < (loopTimes - 1) { 2048u32 } else { lastBuffSize };

        if crate::src_audio_platform_base::MmapWriteData(data_mut as *mut crate::types::PlatformData, tmpBuf) != HDF_SUCCESS {
            unsafe { OsalMemFree(tmpBuf as *mut ::core::ffi::c_void) };
            return HDF_FAILURE;
        }
    }

    let data_mut = unsafe { &mut *data };
    if data_mut.mmapLoopCount > loopTimes {
        data_mut.renderBufInfo.runStatus = PCM_STOP;
    }

    unsafe { OsalMemFree(tmpBuf as *mut ::core::ffi::c_void) };
    HDF_SUCCESS
}

pub extern "C" fn AudioPcmMmapWrite(card: *const crate::types::AudioCard, txMmapData: *const crate::types::AudioMmapData) -> i32 {
    let data: *mut crate::types::PlatformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    if txMmapData.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    let data_ref = unsafe { &mut *data };
    let tx = unsafe { &*txMmapData };
    if tx.memoryAddress.is_null() || tx.transferFrameSize <= 0 || tx.totalBufferFrames <= 0 {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    data_ref.mmapData.offset = tx.offset;
    data_ref.mmapData.memoryAddress = tx.memoryAddress;
    data_ref.mmapData.totalBufferFrames = tx.totalBufferFrames;
    data_ref.mmapData.transferFrameSize = tx.transferFrameSize;
    data_ref.mmapLoopCount = 0;
    let ret: i32 = crate::src_audio_platform_base::AudioMmapWriteTransfer(card);
    if ret != crate::types::HDF_SUCCESS as i32 {
        return crate::types::HDF_FAILURE as i32;
    }
    crate::types::HDF_SUCCESS as i32
}

fn MmapReadData(data: *mut crate::types::PlatformData, rxMmapData: *const crate::types::AudioMmapData, offset: u32)-> i32 {
    if data.is_null() || rxMmapData.is_null() {
        return -1; // HDF_FAILURE
    }

    let data_ref = unsafe { &mut *data };
    let r_ptr = data_ref.captureBufInfo.rptrOffSet;
    let w_ptr = data_ref.captureBufInfo.pointer * data_ref.capturePcmInfo.frameSize;

    if r_ptr > w_ptr {
        let valid_data_size = data_ref.captureBufInfo.cirBufSize - r_ptr;
        if valid_data_size < data_ref.captureBufInfo.trafBufSize {
            data_ref.captureBufInfo.curTrafSize = valid_data_size;
        }
    }

    if data_ref.capturePcmInfo.isBigEndian {
        let src_ptr = unsafe { (data_ref.captureBufInfo.virtAddr as *mut u8).add(r_ptr as usize) as *mut ::core::ffi::c_char };
        let ret = crate::src_audio_platform_base::AudioDataBigEndianChange(
            src_ptr,
            data_ref.captureBufInfo.curTrafSize,
            data_ref.capturePcmInfo.bitWidth,
        );
        if ret != 0 {
            return -1;
        }
    }

    unsafe {
        let ret = CopyToUser(
            ((*rxMmapData).memoryAddress as *mut u8).add(offset as usize) as *mut ::core::ffi::c_void,
            (data_ref.captureBufInfo.virtAddr as *mut u8).add(r_ptr as usize) as *const ::core::ffi::c_void,
            data_ref.captureBufInfo.curTrafSize,
        );
        if ret != 0 {
            return -1;
        }
    }

    data_ref.captureBufInfo.rptrOffSet += data_ref.captureBufInfo.curTrafSize;
    if data_ref.captureBufInfo.rptrOffSet >= data_ref.captureBufInfo.cirBufSize {
        data_ref.captureBufInfo.rptrOffSet = 0;
    }
    data_ref.captureBufInfo.framesPosition += data_ref.captureBufInfo.curTrafSize / data_ref.capturePcmInfo.frameSize;
    data_ref.captureBufInfo.rbufOffSet += data_ref.captureBufInfo.curTrafSize;

    0 // HDF_SUCCESS
}

fn AudioCapturePlatformDataInit(data: *mut crate::types::PlatformData, rxMmapData: *const crate::types::AudioMmapData, totalSize: *mut u32)-> i32 {
    if data.is_null() { return crate::types::HDF_FAILURE; }
    let data_ref = unsafe { &mut *data };
    data_ref.captureBufInfo.pointer = 0;
    let cur_traf = data_ref.captureBufInfo.trafBufSize;
    data_ref.captureBufInfo.curTrafSize = cur_traf;

    if data_ref.captureBufInfo.virtAddr.is_null() {
        let func_name = std::ffi::CStr::from_bytes_with_nul(b"AudioCapturePlatformDataInit\0").unwrap();
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: capture buffer is null.\0").unwrap();
        unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510u32, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 636i32); }
        return crate::types::HDF_FAILURE;
    }

    if data_ref.captureBufInfo.runStatus != crate::types::PCM_START {
        let func_name = std::ffi::CStr::from_bytes_with_nul(b"AudioCapturePlatformDataInit\0").unwrap();
        let tag = std::ffi::CStr::from_bytes_with_nul(b"HDF_AUDIO_KADM\0").unwrap();
        let fmt = std::ffi::CStr::from_bytes_with_nul(b"[%s][line:%d]: capture did not start.\0").unwrap();
        unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD002510u32, tag.as_ptr(), fmt.as_ptr(), func_name.as_ptr(), 641i32); }
        return crate::types::HDF_FAILURE;
    }

    unsafe { *totalSize = (*rxMmapData).totalBufferFrames as u32 * data_ref.capturePcmInfo.frameSize; }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioMmapReadTransfer(card: *const crate::types::AudioCard, rxMmapData: *const crate::types::AudioMmapData) -> i32 {
    let mut offset: u32 = 0;
    let mut timeout: u32 = 0;
    let mut total_size: u32 = 0;

    if card.is_null() || rxMmapData.is_null() || unsafe { (*rxMmapData).memoryAddress.is_null() } || unsafe { (*rxMmapData).totalBufferFrames <= 0 } {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let data: *mut crate::types::PlatformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }

    if crate::src_audio_platform_base::AudioCapturePlatformDataInit(data, rxMmapData, &mut total_size) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let mut first = true;
    while first || (offset < total_size && unsafe { (*data).captureBufInfo.runStatus } != 0) {
        first = false;

        if unsafe { (*data).captureBufInfo.runStatus == crate::types::PCM_PAUSE } {
            let sleep_ms = unsafe { crate::globals::SLEEP_TIME } as u64;
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
            continue;
        }

        let status: i32 = crate::src_audio_platform_base::AudioDmaBuffStatus(card, crate::types::AUDIO_CAPTURE_STREAM);
        if status != crate::types::ENUM_CIR_BUFF_NORMAL {
            let sleep_ms = unsafe { crate::globals::SLEEP_TIME } as u64;
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
            timeout += 1;
            if timeout >= unsafe { crate::globals::TIME_OUT_CONST } as u32 {
                return crate::types::HDF_FAILURE;
            }
            continue;
        }
        timeout = 0;

        if crate::src_audio_platform_base::MmapReadData(data, rxMmapData, offset) != crate::types::HDF_SUCCESS {
            return crate::types::HDF_FAILURE;
        }
        offset += unsafe { (*data).captureBufInfo.curTrafSize };
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioPcmMmapRead(card: *const crate::types::AudioCard, rxMmapData: *const crate::types::AudioMmapData) -> i32 {
    let mut data: *mut crate::types::PlatformData = std::ptr::null_mut();
    let mut ret: i32;

    ret = (card.is_null() || rxMmapData.is_null() || unsafe {
        (*rxMmapData).memoryAddress.is_null() || (*rxMmapData).totalBufferFrames <= 0
    }) as i32;
    if ret != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: param is invalid.\0".as_ptr() as *const i8,
                b"AudioPcmMmapRead\0".as_ptr() as *const i8,
                711i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    data = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: PlatformDataFromCard failed.\0".as_ptr() as *const i8,
                b"AudioPcmMmapRead\0".as_ptr() as *const i8,
                717i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    ret = crate::src_audio_platform_base::AudioMmapReadTransfer(card, rxMmapData);
    if ret != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: AudioMmapReadTransfer fail.\0".as_ptr() as *const i8,
                b"AudioPcmMmapRead\0".as_ptr() as *const i8,
                723i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    return crate::types::HDF_SUCCESS;
}

fn AudioRenderBuffInit(platformData: *mut crate::types::PlatformData)-> i32 {
    if platformData.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let data = unsafe { &mut *platformData };
    if !data.renderBufInfo.virtAddr.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    data.renderBufInfo.cirBufMax = crate::types::MAX_BUFF_SIZE;
    data.renderBufInfo.phyAddr = 0;
    let ret = crate::src_audio_dma_base::AudioDmaBufAlloc(
        platformData,
        crate::types::AUDIO_RENDER_STREAM,
    );
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    if data.renderBufInfo.virtAddr.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let virt_addr = data.renderBufInfo.virtAddr;
    let size = data.renderBufInfo.cirBufMax as usize;
    unsafe { std::ptr::write_bytes(virt_addr as *mut u8, 0u8, size); }
    crate::types::HDF_SUCCESS
}

fn AudioRenderBuffFree(platformData: *mut crate::types::PlatformData)-> i32 {
    if platformData.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: input param is NULL.\0".as_ptr() as *const i8,
                b"AudioRenderBuffFree\0".as_ptr() as *const i8,
                769i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let ret = crate::src_audio_dma_base::AudioDmaBufFree(platformData, crate::types::AUDIO_RENDER_STREAM);
    if ret != 0 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Dma Buf Alloc fail.\0".as_ptr() as *const i8,
                b"AudioRenderBuffFree\0".as_ptr() as *const i8,
                775i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        (*platformData).renderBufInfo.virtAddr = std::ptr::null_mut();
        (*platformData).renderBufInfo.phyAddr = 0;
    }

    crate::types::HDF_SUCCESS
}

fn AudioCaptureBuffInit(platformData: *mut crate::types::PlatformData)-> i32 {
    if platformData.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        if !(*platformData).captureBufInfo.virtAddr.is_null() {
            return crate::types::HDF_SUCCESS;
        }
    }

    unsafe {
        (*platformData).captureBufInfo.cirBufMax = crate::types::MAX_BUFF_SIZE;
        (*platformData).captureBufInfo.phyAddr = 0;
    }

    let ret = crate::src_audio_dma_base::AudioDmaBufAlloc(platformData, crate::types::AUDIO_CAPTURE_STREAM);
    if ret != 0 {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        if (*platformData).captureBufInfo.virtAddr.is_null() {
            return crate::types::HDF_FAILURE;
        }
    }

    unsafe {
        let virt_addr = (*platformData).captureBufInfo.virtAddr;
        let size = (*platformData).captureBufInfo.cirBufMax as usize;
        std::ptr::write_bytes(virt_addr, 0u8, size);
    }

    crate::types::HDF_SUCCESS
}

fn AudioCaptureBuffFree(platformData: *mut crate::types::PlatformData)-> i32 {
    unsafe {
        if platformData.is_null() {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: input param is NULL.\0".as_ptr() as *const i8,
                b"AudioCaptureBuffFree\0".as_ptr() as *const i8,
                821i32,
            );
            return crate::types::HDF_FAILURE;
        }

        let ret: i32 = crate::src_audio_dma_base::AudioDmaBufFree(
            platformData,
            crate::types::AUDIO_CAPTURE_STREAM,
        );
        if ret != crate::types::HDF_SUCCESS {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: Dma Buf Alloc fail.\0".as_ptr() as *const i8,
                b"AudioCaptureBuffFree\0".as_ptr() as *const i8,
                827i32,
            );
            return crate::types::HDF_FAILURE;
        }

        (*platformData).captureBufInfo.virtAddr = std::ptr::null_mut();
        (*platformData).captureBufInfo.phyAddr = 0;
        crate::types::HDF_SUCCESS
    }
}

pub extern "C" fn AudioRenderOpen(card: *const crate::types::AudioCard) -> i32 {
    let data = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        if (*data).renderBufInfo.virtAddr.is_null() {
            if crate::src_audio_platform_base::AudioRenderBuffInit(data) != crate::types::HDF_SUCCESS {
                return crate::types::HDF_FAILURE;
            }
        }
    }

    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn AudioRenderClose(card: *const crate::types::AudioCard) -> i32 {
    let data = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card) };
    if data.is_null() {
        return HDF_FAILURE;
    }
    crate::src_audio_platform_base::AudioRenderBuffFree(data)
}

pub extern "C" fn AudioCaptureOpen(card: *const crate::types::AudioCard) -> i32 {
    let mut platformData = core::ptr::null_mut::<crate::types::PlatformData>();
    if card.is_null() {
        // HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "HDF_AUDIO_KADM", "[%s][line:%d]: " "capture open param card is NULL.", __func__, 867);
        return crate::types::HDF_FAILURE;
    }

    platformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if platformData.is_null() {
        // HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "HDF_AUDIO_KADM", "[%s][line:%d]: " "PlatformDataFromCard failed.", __func__, 873);
        return crate::types::HDF_FAILURE;
    }

    if unsafe { (*platformData).captureBufInfo.virtAddr.is_null() } {
        if crate::src_audio_platform_base::AudioCaptureBuffInit(platformData) != crate::types::HDF_SUCCESS {
            // HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "HDF_AUDIO_KADM", "[%s][line:%d]: " "AudioCaptureBuffInit: fail.", __func__, 879);
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCaptureClose(card: *const crate::types::AudioCard) -> i32 {
    if card.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: capture close param card is NULL.\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"AudioCaptureClose\0" as *const u8 as *const ::core::ffi::c_char,
                890i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let platformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if platformData.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0" as *const u8 as *const ::core::ffi::c_char,
                b"[%s][line:%d]: PlatformDataFromCard failed.\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"AudioCaptureClose\0" as *const u8 as *const ::core::ffi::c_char,
                896i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    crate::src_audio_platform_base::AudioCaptureBuffFree(platformData)
}

fn AudioPcmPending(card: *mut crate::types::AudioCard, streamType: crate::types::AudioStreamType)-> i32 {
    unsafe {
        let data = crate::src_audio_platform_base::PlatformDataFromCard(card);
        if data.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: PlatformDataFromCard failed.\0".as_ptr() as *const i8,
                b"AudioPcmPending\0".as_ptr() as *const i8,
                906i32,
            );
            return HDF_FAILURE;
        }
        if crate::src_audio_dma_base::AudioDmaSubmit(data, streamType) != HDF_SUCCESS {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: DmaPending fail.\0".as_ptr() as *const i8,
                b"AudioPcmPending\0".as_ptr() as *const i8,
                911i32,
            );
            return HDF_FAILURE;
        }
        if crate::src_audio_dma_base::AudioDmaPending(data, streamType) != HDF_SUCCESS {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: DmaPending fail.\0".as_ptr() as *const i8,
                b"AudioPcmPending\0".as_ptr() as *const i8,
                916i32,
            );
            return HDF_FAILURE;
        }
        if AudioSampPowerUp(card) != HDF_SUCCESS {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const i8,
                b"[%s][line:%d]: PowerUp fail.\0".as_ptr() as *const i8,
                b"AudioPcmPending\0".as_ptr() as *const i8,
                921i32,
            );
            return HDF_FAILURE;
        }
        if AudioSampSetPowerMonitor(card, false) != HDF_SUCCESS {
            return HDF_FAILURE;
        }
        HDF_SUCCESS
    }
}

fn AudioPcmPause(card: *mut crate::types::AudioCard, streamType: crate::types::AudioStreamType)-> i32 {
    let data = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if crate::src_audio_dma_base::AudioDmaPause(data, streamType) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    extern "C" {
        fn AudioSampSetPowerMonitor(card: *mut crate::types::AudioCard, powerMonitorState: i32) -> i32;
    }
    if unsafe { AudioSampSetPowerMonitor(card, 1) } != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn AudioPcmResume(card: *mut crate::types::AudioCard, streamType: crate::types::AudioStreamType)-> i32 {
    let data = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card as *const crate::types::AudioCard) };
    if data.is_null() {
        return HDF_FAILURE;
    }

    if unsafe { crate::src_audio_dma_base::AudioDmaResume(data, streamType) } != HDF_SUCCESS {
        return HDF_FAILURE;
    }

    if unsafe { AudioSampPowerUp(card as *const crate::types::AudioCard) } != HDF_SUCCESS {
        return HDF_FAILURE;
    }
    if unsafe { AudioSampSetPowerMonitor(card, false) } != HDF_SUCCESS {
        return HDF_FAILURE;
    }

    HDF_SUCCESS
}

pub extern "C" fn AudioRenderTrigger(card: *mut crate::types::AudioCard, cmd: i32) -> i32 {
    let data = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if data.is_null() {
        return crate::types::HDF_FAILURE;
    }
    match cmd {
        _ if cmd == crate::types::AUDIO_DRV_PCM_IOCTL_RENDER_START as i32 => {
            unsafe {
                if crate::src_audio_platform_base::AudioPcmPending(card, crate::types::AUDIO_RENDER_STREAM) != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_FAILURE;
                }
                (*data).renderBufInfo.runStatus = crate::types::PCM_START;
            }
        }
        _ if cmd == crate::types::AUDIO_DRV_PCM_IOCTL_RENDER_STOP as i32 => {
            unsafe {
                if crate::src_audio_platform_base::AudioPcmPause(card, crate::types::AUDIO_RENDER_STREAM) != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_FAILURE;
                }
                (*data).renderBufInfo.runStatus = crate::types::PCM_STOP;
            }
        }
        _ if cmd == crate::types::AUDIO_DRV_PCM_IOCTL_RENDER_PAUSE as i32 => {
            unsafe {
                if crate::src_audio_platform_base::AudioPcmPause(card, crate::types::AUDIO_RENDER_STREAM) != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_FAILURE;
                }
                (*data).renderBufInfo.runStatus = crate::types::PCM_PAUSE;
            }
        }
        _ if cmd == crate::types::AUDIO_DRV_PCM_IOCTL_RENDER_RESUME as i32 => {
            unsafe {
                if crate::src_audio_platform_base::AudioPcmResume(card, crate::types::AUDIO_RENDER_STREAM) != crate::types::HDF_SUCCESS {
                    return crate::types::HDF_FAILURE;
                }
                (*data).renderBufInfo.runStatus = crate::types::PCM_START;
            }
        }
        _ => {
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCaptureTrigger(card: *mut crate::types::AudioCard, cmd: i32) -> i32 {
    use crate::types::*;
    let data = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card as *const AudioCard) };
    if data.is_null() {
        return HDF_FAILURE;
    }
    let cmd = cmd as StreamDispMethodCmd;
    match cmd {
        AUDIO_DRV_PCM_IOCTL_CAPTURE_START => {
            if crate::src_audio_platform_base::AudioPcmPending(card, AUDIO_CAPTURE_STREAM) != HDF_SUCCESS {
                return HDF_FAILURE;
            }
            unsafe { (*data).captureBufInfo.runStatus = PCM_START; }
        }
        AUDIO_DRV_PCM_IOCTL_CAPTURE_STOP => {
            if crate::src_audio_platform_base::AudioPcmPause(card, AUDIO_CAPTURE_STREAM) != HDF_SUCCESS {
                return HDF_FAILURE;
            }
            unsafe { (*data).captureBufInfo.runStatus = PCM_STOP; }
        }
        AUDIO_DRV_PCM_IOCTL_CAPTURE_PAUSE => {
            if crate::src_audio_platform_base::AudioPcmPause(card, AUDIO_CAPTURE_STREAM) != HDF_SUCCESS {
                return HDF_FAILURE;
            }
            unsafe { (*data).captureBufInfo.runStatus = PCM_PAUSE; }
        }
        AUDIO_DRV_PCM_IOCTL_CAPTURE_RESUME => {
            if crate::src_audio_platform_base::AudioPcmResume(card, AUDIO_CAPTURE_STREAM) != HDF_SUCCESS {
                return HDF_FAILURE;
            }
            unsafe { (*data).captureBufInfo.runStatus = PCM_START; }
        }
        _ => {
            return HDF_FAILURE;
        }
    }
    HDF_SUCCESS
}

fn AudioDmaConfig(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType)-> i32 {
    unsafe {
        if data.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        if streamType == crate::types::AUDIO_RENDER_STREAM {
            if crate::src_audio_dma_base::AudioDmaConfigChannel(data, crate::types::AUDIO_RENDER_STREAM) != 0 {
                return crate::types::HDF_FAILURE;
            }
        } else if streamType == crate::types::AUDIO_CAPTURE_STREAM {
            if crate::src_audio_dma_base::AudioDmaConfigChannel(data, crate::types::AUDIO_CAPTURE_STREAM) != 0 {
                return crate::types::HDF_FAILURE;
            }
        } else {
            return crate::types::HDF_FAILURE;
        }
        crate::types::HDF_SUCCESS
    }
}

fn AudioPcmTransferBytes(data: *mut crate::types::PlatformData, streamType: crate::types::AudioStreamType)-> i32 {
    if data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        if streamType == crate::types::AUDIO_RENDER_STREAM {
            let rate = (*data).renderPcmInfo.rate;
            let frame_size = (*data).renderPcmInfo.frameSize;
            (*data).renderBufInfo.oneMsBytes = rate.wrapping_mul(frame_size) / 1000u32;
            if (*data).renderBufInfo.oneMsBytes == 0 {
                return crate::types::HDF_FAILURE;
            }
        } else if streamType == crate::types::AUDIO_CAPTURE_STREAM {
            let rate = (*data).capturePcmInfo.rate;
            let frame_size = (*data).capturePcmInfo.frameSize;
            (*data).captureBufInfo.oneMsBytes = rate.wrapping_mul(frame_size) / 1000u32;
            if (*data).captureBufInfo.oneMsBytes == 0 {
                return crate::types::HDF_FAILURE;
            }
        } else {
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

fn AudioSetRenderHwParams(param: *const crate::types::AudioPcmHwParams, data: *mut crate::types::PlatformData)-> i32 {
    if param.is_null() || data.is_null() {
        eprintln!(
            "HDF_AUDIO_KADM [AudioSetRenderHwParams][{}]: input param is NULL.",
            line!()
        );
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    if crate::src_audio_dma_base::AudioDmaRequestChannel(
        data,
        crate::types::AUDIO_RENDER_STREAM,
    ) != crate::types::HDF_SUCCESS
    {
        eprintln!(
            "HDF_AUDIO_KADM [AudioSetRenderHwParams][{}]: Dma Request Channel fail.",
            line!()
        );
        return crate::types::HDF_FAILURE;
    }
    if crate::src_audio_platform_base::AudioSetRenderBufInfo(data, param)
        != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_FAILURE;
    }
    if crate::src_audio_platform_base::AudioDmaConfig(
        data,
        crate::types::AUDIO_RENDER_STREAM,
    ) != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_FAILURE;
    }
    if crate::src_audio_platform_base::AudioPcmTransferBytes(
        data,
        crate::types::AUDIO_RENDER_STREAM,
    ) != crate::types::HDF_SUCCESS
    {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

fn AudioSetCaptureHwParams(param: *const crate::types::AudioPcmHwParams, data: *mut crate::types::PlatformData)-> i32 {
    if param.is_null() || data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let ret = unsafe {
        crate::src_audio_dma_base::AudioDmaRequestChannel(data, crate::types::AUDIO_CAPTURE_STREAM)
    };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let ret = unsafe {
        crate::src_audio_platform_base::AudioSetCaptureBufInfo(data, param)
    };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let ret = crate::src_audio_platform_base::AudioDmaConfig(data, crate::types::AUDIO_CAPTURE_STREAM);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let ret = crate::src_audio_platform_base::AudioPcmTransferBytes(data, crate::types::AUDIO_CAPTURE_STREAM);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioHwParams(card: *const crate::types::AudioCard, param: *const crate::types::AudioPcmHwParams) -> i32 {
    if card.is_null() || param.is_null() || unsafe { (*param).cardServiceName.is_null() } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: input param is NULL.\0".as_ptr() as *const core::ffi::c_char,
                b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                1188i32,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    if unsafe { (*param).channels < 1u32 || (*param).channels > 2u32 } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: channels param is invalid.\0".as_ptr() as *const core::ffi::c_char,
                b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                1193i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let platformData = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if platformData.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: platformData is null.\0".as_ptr() as *const core::ffi::c_char,
                b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                1199i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    if crate::src_audio_platform_base::AudioSetPcmInfo(platformData, param) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let stream_type = unsafe { (*param).streamType };
    if stream_type == crate::types::AUDIO_RENDER_STREAM {
        if crate::src_audio_platform_base::AudioSetRenderHwParams(param, platformData) != crate::types::HDF_SUCCESS {
            unsafe {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                    b"[%s][line:%d]: set render hardware params is failed.\0".as_ptr() as *const core::ffi::c_char,
                    b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                    1209i32,
                );
            }
            return crate::types::HDF_FAILURE;
        }
    } else if stream_type == crate::types::AUDIO_CAPTURE_STREAM {
        if crate::src_audio_platform_base::AudioSetCaptureHwParams(param, platformData) != crate::types::HDF_SUCCESS {
            unsafe {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                    b"[%s][line:%d]: set capture hardware params is failed.\0".as_ptr() as *const core::ffi::c_char,
                    b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                    1214i32,
                );
            }
            return crate::types::HDF_FAILURE;
        }
    } else {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"HDF_AUDIO_KADM\0".as_ptr() as *const core::ffi::c_char,
                b"[%s][line:%d]: param streamType is invalid.\0".as_ptr() as *const core::ffi::c_char,
                b"AudioHwParams\0".as_ptr() as *const core::ffi::c_char,
                1218i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    return crate::types::HDF_SUCCESS;
}

pub extern "C" fn AudioRenderPrepare(card: *const crate::types::AudioCard) -> i32 {
    if card.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let platformData = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card) };
    if platformData.is_null() {
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        let virtAddr = (*platformData).renderBufInfo.virtAddr;
        if virtAddr.is_null() {
            return crate::types::HDF_FAILURE;
        }
        let cirBufSize = (*platformData).renderBufInfo.cirBufSize;
        std::ptr::write_bytes(virtAddr as *mut u8, 0, cirBufSize as usize);
        (*platformData).renderBufInfo.wbufOffSet = 0;
        (*platformData).renderBufInfo.wptrOffSet = 0;
        (*platformData).renderBufInfo.framesPosition = 0;
        (*platformData).renderBufInfo.pointer = 0;
        (*platformData).renderPcmInfo.totalStreamSize = 0;
        (*platformData).renderBufInfo.rbufOffSet = 0;
        (*platformData).renderBufInfo.trafCompCount = 0;
    }

    let ret = unsafe { crate::src_audio_dma_base::AudioDmaPrep(platformData, crate::types::AUDIO_RENDER_STREAM) };
    if ret != 0 {
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCapturePrepare(card: *const crate::types::AudioCard) -> i32 {
    if card.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let platform_data = crate::src_audio_platform_base::PlatformDataFromCard(card);
    if platform_data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        if (*platform_data).captureBufInfo.virtAddr.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        let buf_size = (*platform_data).captureBufInfo.cirBufSize as usize;
        core::ptr::write_bytes(
            (*platform_data).captureBufInfo.virtAddr as *mut u8,
            0,
            buf_size,
        );
        (*platform_data).captureBufInfo.rbufOffSet = 0;
        (*platform_data).captureBufInfo.rptrOffSet = 0;
        (*platform_data).captureBufInfo.chnId = 0;
        (*platform_data).captureBufInfo.framesPosition = 0;
        (*platform_data).captureBufInfo.pointer = 0;
        (*platform_data).capturePcmInfo.totalStreamSize = 0;
        (*platform_data).captureBufInfo.wbufOffSet = 0;
        (*platform_data).captureBufInfo.trafCompCount = 0;
    }

    let ret = crate::src_audio_dma_base::AudioDmaPrep(
        platform_data,
        crate::types::AUDIO_CAPTURE_STREAM,
    );
    if ret != 0 {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioPcmPointer(card: *const crate::types::AudioCard, pointer: *mut u32, streamType: crate::types::AudioStreamType) -> i32 {
    if card.is_null() || pointer.is_null() {
        // logging omitted: HiLogPrint not available
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let data = unsafe { crate::src_audio_platform_base::PlatformDataFromCard(card) };
    if data.is_null() {
        // logging omitted
        return crate::types::HDF_FAILURE;
    }

    let ret = unsafe { crate::src_audio_dma_base::AudioDmaPointer(data, streamType, pointer) };
    if ret != crate::types::HDF_SUCCESS {
        // logging omitted
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn AudioCapSilenceThresholdEvent(device: *mut crate::types::HdfDeviceObject, reportMsg: *const crate::types::AudioEvent) -> i32 {
    if device.is_null() || reportMsg.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let mut msg_buf: [u8; 256] = [0u8; 256];
    let event_type = unsafe { (*reportMsg).eventType };
    let device_type = unsafe { (*reportMsg).deviceType };

    let format_str = b"EVENT_TYPE=0x%x;DEVICE_TYPE=0x%x\0";
    let ret = unsafe {
        libc::snprintf(
            msg_buf.as_mut_ptr() as *mut i8,
            256usize,
            format_str.as_ptr() as *const i8,
            event_type,
            device_type,
        )
    };
    if ret < 0 {
        return crate::types::HDF_FAILURE;
    }

    let set_ret = unsafe {
        crate::compat::HdfDeviceObjectSetServInfo(device, msg_buf.as_ptr() as *const i8)
    };
    if set_ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    let update_ret = unsafe {
        crate::compat::HdfDeviceObjectUpdate(device)
    };
    if update_ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}
