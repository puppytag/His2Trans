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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn BZ2_bz__AssertH__fail(errcode: core::ffi::c_int);
    static mut stderr: *mut FILE;
    fn BZ2_blockSort(_: *mut EState);
    fn BZ2_hbAssignCodes(_: *mut Int32, _: *mut UChar, _: Int32, _: Int32, _: Int32);
    fn BZ2_hbMakeCodeLengths(_: *mut UChar, _: *mut Int32, _: Int32, _: Int32);
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut core::ffi::c_char,
    pub avail_in: core::ffi::c_uint,
    pub total_in_lo32: core::ffi::c_uint,
    pub total_in_hi32: core::ffi::c_uint,
    pub next_out: *mut core::ffi::c_char,
    pub avail_out: core::ffi::c_uint,
    pub total_out_lo32: core::ffi::c_uint,
    pub total_out_hi32: core::ffi::c_uint,
    pub state: *mut core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> *mut core::ffi::c_void,
    >,
    pub bzfree: Option<
        unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> (),
    >,
    pub opaque: *mut core::ffi::c_void,
}
pub type Bool = core::ffi::c_uchar;
pub type UChar = core::ffi::c_uchar;
pub type Int32 = core::ffi::c_int;
pub type UInt32 = core::ffi::c_uint;
pub type UInt16 = core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; 258],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub code: [[Int32; 258]; 6],
    pub rfreq: [[Int32; 258]; 6],
    pub len_pack: [[UInt32; 4]; 258],
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
    (*s).bsLive = 0 as core::ffi::c_int as Int32;
    (*s).bsBuff = 0 as UInt32;
}
unsafe extern "C" fn bsFinishWrite(mut s: *mut EState) {
    while (*s).bsLive > 0 as core::ffi::c_int {
        *((*s).zbits).offset((*s).numZ as isize) = ((*s).bsBuff
            >> 24 as core::ffi::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as core::ffi::c_int;
        (*s).bsLive -= 8 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn bsNEEDW(mut s: *mut EState) {
    while (*s).bsLive >= 8 as core::ffi::c_int {
        *((*s).zbits).offset((*s).numZ as isize) = ((*s).bsBuff
            >> 24 as core::ffi::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as core::ffi::c_int;
        (*s).bsLive -= 8 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn bsW(mut s: *mut EState, mut n: Int32, mut v: UInt32) {
    bsNEEDW(s);
    (*s).bsBuff |= (v << 32 as Int32 - (*s).bsLive - n) as core::ffi::c_uint;
    (*s).bsLive += n as core::ffi::c_int;
}
unsafe extern "C" fn bsPutUInt32(mut s: *mut EState, mut u: UInt32) {
    bsW(
        s,
        8 as Int32,
        ((u >> 24 as core::ffi::c_int) as core::ffi::c_long & 0xff as core::ffi::c_long)
            as UInt32,
    );
    bsW(
        s,
        8 as Int32,
        ((u >> 16 as core::ffi::c_int) as core::ffi::c_long & 0xff as core::ffi::c_long)
            as UInt32,
    );
    bsW(
        s,
        8 as Int32,
        ((u >> 8 as core::ffi::c_int) as core::ffi::c_long & 0xff as core::ffi::c_long)
            as UInt32,
    );
    bsW(s, 8 as Int32, (u as core::ffi::c_long & 0xff as core::ffi::c_long) as UInt32);
}
unsafe extern "C" fn bsPutUChar(mut s: *mut EState, mut c: UChar) {
    bsW(s, 8 as Int32, c as UInt32);
}
unsafe extern "C" fn makeMaps_e(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as core::ffi::c_int as Int32;
    i = 0 as core::ffi::c_int as Int32;
    while i < 256 as core::ffi::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).unseqToSeq[i as usize] = (*s).nInUse as UChar;
            (*s).nInUse += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn generateMTFValues(mut s: *mut EState) {
    let mut yy: [UChar; 256] = [0; 256];
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut zPend: Int32 = 0;
    let mut wr: Int32 = 0;
    let mut EOB: Int32 = 0;
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    makeMaps_e(s);
    EOB = ((*s).nInUse as core::ffi::c_int + 1 as core::ffi::c_int) as Int32;
    i = 0 as core::ffi::c_int as Int32;
    while i <= EOB {
        (*s).mtfFreq[i as usize] = 0 as core::ffi::c_int as Int32;
        i += 1;
    }
    wr = 0 as core::ffi::c_int as Int32;
    zPend = 0 as core::ffi::c_int as Int32;
    i = 0 as core::ffi::c_int as Int32;
    while i < (*s).nInUse {
        yy[i as usize] = i as UChar;
        i += 1;
    }
    i = 0 as core::ffi::c_int as Int32;
    while i < (*s).nblock {
        let mut ll_i: UChar = 0;
        j = (*ptr.offset(i as isize)).wrapping_sub(1 as core::ffi::c_uint) as Int32;
        if j < 0 as core::ffi::c_int {
            j += (*s).nblock as core::ffi::c_int;
        }
        ll_i = (*s).unseqToSeq[*block.offset(j as isize) as usize];
        if yy[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ll_i as core::ffi::c_int
        {
            zPend += 1;
        } else {
            if zPend > 0 as core::ffi::c_int {
                zPend -= 1;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if zPend as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
                        *mtfv.offset(wr as isize) = 1 as UInt16;
                        wr += 1;
                        (*s).mtfFreq[1 as core::ffi::c_int as usize] += 1;
                    } else {
                        *mtfv.offset(wr as isize) = 0 as UInt16;
                        wr += 1;
                        (*s).mtfFreq[0 as core::ffi::c_int as usize] += 1;
                    }
                    if zPend < 2 as core::ffi::c_int {
                        break;
                    }
                    zPend = ((zPend as core::ffi::c_int - 2 as core::ffi::c_int)
                        / 2 as core::ffi::c_int) as Int32;
                }
                zPend = 0 as core::ffi::c_int as Int32;
            }
            let mut rtmp: UChar = 0;
            let mut ryy_j: *mut UChar = 0 as *mut UChar;
            let mut rll_i: UChar = 0;
            rtmp = yy[1 as core::ffi::c_int as usize];
            yy[1 as core::ffi::c_int as usize] = yy[0 as core::ffi::c_int as usize];
            ryy_j = &mut *yy.as_mut_ptr().offset(1 as core::ffi::c_int as isize)
                as *mut UChar;
            rll_i = ll_i;
            while rll_i as core::ffi::c_int != rtmp as core::ffi::c_int {
                let mut rtmp2: UChar = 0;
                ryy_j = ryy_j.offset(1);
                rtmp2 = rtmp;
                rtmp = *ryy_j;
                *ryy_j = rtmp2;
            }
            yy[0 as core::ffi::c_int as usize] = rtmp;
            j = ryy_j
                .offset_from(
                    &mut *yy.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
                        as *mut UChar,
                ) as core::ffi::c_long as Int32;
            *mtfv.offset(wr as isize) = (j as core::ffi::c_int + 1 as core::ffi::c_int)
                as UInt16;
            wr += 1;
            (*s).mtfFreq[(j as core::ffi::c_int + 1 as core::ffi::c_int) as usize] += 1;
        }
        i += 1;
    }
    if zPend > 0 as core::ffi::c_int {
        zPend -= 1;
        while 1 as core::ffi::c_int as Bool != 0 {
            if zPend as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
                *mtfv.offset(wr as isize) = 1 as UInt16;
                wr += 1;
                (*s).mtfFreq[1 as core::ffi::c_int as usize] += 1;
            } else {
                *mtfv.offset(wr as isize) = 0 as UInt16;
                wr += 1;
                (*s).mtfFreq[0 as core::ffi::c_int as usize] += 1;
            }
            if zPend < 2 as core::ffi::c_int {
                break;
            }
            zPend = ((zPend as core::ffi::c_int - 2 as core::ffi::c_int)
                / 2 as core::ffi::c_int) as Int32;
        }
        zPend = 0 as core::ffi::c_int as Int32;
    }
    *mtfv.offset(wr as isize) = EOB as UInt16;
    wr += 1;
    (*s).mtfFreq[EOB as usize] += 1;
    (*s).nMTF = wr;
}
unsafe extern "C" fn sendMTFValues(mut s: *mut EState) {
    let mut v: Int32 = 0;
    let mut t: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut gs: Int32 = 0;
    let mut ge: Int32 = 0;
    let mut totc: Int32 = 0;
    let mut bt: Int32 = 0;
    let mut bc: Int32 = 0;
    let mut iter: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut selCtr: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nBytes: Int32 = 0;
    let mut cost: [UInt16; 6] = [0; 6];
    let mut fave: [Int32; 6] = [0; 6];
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    if (*s).verbosity >= 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\0"
                as *const u8 as *const core::ffi::c_char,
            (*s).nblock,
            (*s).nMTF,
            (*s).nInUse,
        );
    }
    alphaSize = ((*s).nInUse as core::ffi::c_int + 2 as core::ffi::c_int) as Int32;
    t = 0 as core::ffi::c_int as Int32;
    while t < 6 as core::ffi::c_int {
        v = 0 as core::ffi::c_int as Int32;
        while v < alphaSize {
            (*s).len[t as usize][v as usize] = 15 as UChar;
            v += 1;
        }
        t += 1;
    }
    if !((*s).nMTF > 0 as core::ffi::c_int) {
        BZ2_bz__AssertH__fail(3001 as core::ffi::c_int);
    }
    if (*s).nMTF < 200 as core::ffi::c_int {
        nGroups = 2 as core::ffi::c_int as Int32;
    } else if (*s).nMTF < 600 as core::ffi::c_int {
        nGroups = 3 as core::ffi::c_int as Int32;
    } else if (*s).nMTF < 1200 as core::ffi::c_int {
        nGroups = 4 as core::ffi::c_int as Int32;
    } else if (*s).nMTF < 2400 as core::ffi::c_int {
        nGroups = 5 as core::ffi::c_int as Int32;
    } else {
        nGroups = 6 as core::ffi::c_int as Int32;
    }
    let mut nPart: Int32 = 0;
    let mut remF: Int32 = 0;
    let mut tFreq: Int32 = 0;
    let mut aFreq: Int32 = 0;
    nPart = nGroups;
    remF = (*s).nMTF;
    gs = 0 as core::ffi::c_int as Int32;
    while nPart > 0 as core::ffi::c_int {
        tFreq = remF / nPart;
        ge = (gs as core::ffi::c_int - 1 as core::ffi::c_int) as Int32;
        aFreq = 0 as core::ffi::c_int as Int32;
        while aFreq < tFreq && ge < alphaSize as core::ffi::c_int - 1 as core::ffi::c_int
        {
            ge += 1;
            aFreq += (*s).mtfFreq[ge as usize] as core::ffi::c_int;
        }
        if ge > gs && nPart != nGroups && nPart != 1 as core::ffi::c_int
            && (nGroups as core::ffi::c_int - nPart as core::ffi::c_int)
                % 2 as core::ffi::c_int == 1 as core::ffi::c_int
        {
            aFreq -= (*s).mtfFreq[ge as usize] as core::ffi::c_int;
            ge -= 1;
        }
        if (*s).verbosity >= 3 as core::ffi::c_int {
            fprintf(
                stderr,
                b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\0"
                    as *const u8 as *const core::ffi::c_char,
                nPart,
                gs,
                ge,
                aFreq,
                100.0f64 * aFreq as core::ffi::c_float as core::ffi::c_double
                    / (*s).nMTF as core::ffi::c_float as core::ffi::c_double,
            );
        }
        v = 0 as core::ffi::c_int as Int32;
        while v < alphaSize {
            if v >= gs && v <= ge {
                (*s)
                    .len[(nPart as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize][v as usize] = 0 as UChar;
            } else {
                (*s)
                    .len[(nPart as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize][v as usize] = 15 as UChar;
            }
            v += 1;
        }
        nPart -= 1;
        gs = (ge as core::ffi::c_int + 1 as core::ffi::c_int) as Int32;
        remF -= aFreq as core::ffi::c_int;
    }
    iter = 0 as core::ffi::c_int as Int32;
    while iter < 4 as core::ffi::c_int {
        t = 0 as core::ffi::c_int as Int32;
        while t < nGroups {
            fave[t as usize] = 0 as core::ffi::c_int as Int32;
            t += 1;
        }
        t = 0 as core::ffi::c_int as Int32;
        while t < nGroups {
            v = 0 as core::ffi::c_int as Int32;
            while v < alphaSize {
                (*s).rfreq[t as usize][v as usize] = 0 as core::ffi::c_int as Int32;
                v += 1;
            }
            t += 1;
        }
        if nGroups == 6 as core::ffi::c_int {
            v = 0 as core::ffi::c_int as Int32;
            while v < alphaSize {
                (*s).len_pack[v as usize][0 as core::ffi::c_int as usize] = (((*s)
                    .len[1 as core::ffi::c_int as usize][v as usize] as core::ffi::c_int)
                    << 16 as core::ffi::c_int
                    | (*s).len[0 as core::ffi::c_int as usize][v as usize]
                        as core::ffi::c_int) as UInt32;
                (*s).len_pack[v as usize][1 as core::ffi::c_int as usize] = (((*s)
                    .len[3 as core::ffi::c_int as usize][v as usize] as core::ffi::c_int)
                    << 16 as core::ffi::c_int
                    | (*s).len[2 as core::ffi::c_int as usize][v as usize]
                        as core::ffi::c_int) as UInt32;
                (*s).len_pack[v as usize][2 as core::ffi::c_int as usize] = (((*s)
                    .len[5 as core::ffi::c_int as usize][v as usize] as core::ffi::c_int)
                    << 16 as core::ffi::c_int
                    | (*s).len[4 as core::ffi::c_int as usize][v as usize]
                        as core::ffi::c_int) as UInt32;
                v += 1;
            }
        }
        nSelectors = 0 as core::ffi::c_int as Int32;
        totc = 0 as core::ffi::c_int as Int32;
        gs = 0 as core::ffi::c_int as Int32;
        while 1 as core::ffi::c_int as Bool != 0 {
            if gs >= (*s).nMTF {
                break;
            }
            ge = (gs as core::ffi::c_int + 50 as core::ffi::c_int
                - 1 as core::ffi::c_int) as Int32;
            if ge >= (*s).nMTF {
                ge = ((*s).nMTF as core::ffi::c_int - 1 as core::ffi::c_int) as Int32;
            }
            t = 0 as core::ffi::c_int as Int32;
            while t < nGroups {
                cost[t as usize] = 0 as UInt16;
                t += 1;
            }
            if nGroups == 6 as core::ffi::c_int
                && 50 as core::ffi::c_int
                    == ge as core::ffi::c_int - gs as core::ffi::c_int
                        + 1 as core::ffi::c_int
            {
                let mut cost01: UInt32 = 0;
                let mut cost23: UInt32 = 0;
                let mut cost45: UInt32 = 0;
                let mut icv: UInt16 = 0;
                cost45 = 0 as UInt32;
                cost23 = cost45;
                cost01 = cost23;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 0 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 2 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 3 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 4 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 5 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 6 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 7 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 8 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 9 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 10 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 11 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 12 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 13 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 14 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 15 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 16 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 17 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 18 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 19 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 20 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 21 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 22 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 23 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 24 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 25 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 26 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 27 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 28 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 29 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 30 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 31 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 32 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 33 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 34 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 35 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 36 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 37 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 38 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 39 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 40 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 41 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 42 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 43 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 44 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 45 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 46 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 47 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 48 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                icv = *mtfv
                    .offset((gs as core::ffi::c_int + 49 as core::ffi::c_int) as isize);
                cost01 = (cost01 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost23 = (cost23 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost45 = (cost45 as core::ffi::c_uint)
                    .wrapping_add(
                        (*s).len_pack[icv as usize][2 as core::ffi::c_int as usize]
                            as core::ffi::c_uint,
                    ) as UInt32 as UInt32;
                cost[0 as core::ffi::c_int as usize] = (cost01 as core::ffi::c_uint
                    & 0xffff as core::ffi::c_uint) as UInt16;
                cost[1 as core::ffi::c_int as usize] = (cost01 >> 16 as core::ffi::c_int)
                    as UInt16;
                cost[2 as core::ffi::c_int as usize] = (cost23 as core::ffi::c_uint
                    & 0xffff as core::ffi::c_uint) as UInt16;
                cost[3 as core::ffi::c_int as usize] = (cost23 >> 16 as core::ffi::c_int)
                    as UInt16;
                cost[4 as core::ffi::c_int as usize] = (cost45 as core::ffi::c_uint
                    & 0xffff as core::ffi::c_uint) as UInt16;
                cost[5 as core::ffi::c_int as usize] = (cost45 >> 16 as core::ffi::c_int)
                    as UInt16;
            } else {
                i = gs;
                while i <= ge {
                    let mut icv_0: UInt16 = *mtfv.offset(i as isize);
                    t = 0 as core::ffi::c_int as Int32;
                    while t < nGroups {
                        cost[t as usize] = (cost[t as usize] as core::ffi::c_int
                            + (*s).len[t as usize][icv_0 as usize] as core::ffi::c_int)
                            as UInt16;
                        t += 1;
                    }
                    i += 1;
                }
            }
            bc = 999999999 as core::ffi::c_int as Int32;
            bt = -(1 as core::ffi::c_int) as Int32;
            t = 0 as core::ffi::c_int as Int32;
            while t < nGroups {
                if (cost[t as usize] as core::ffi::c_int) < bc {
                    bc = cost[t as usize] as Int32;
                    bt = t;
                }
                t += 1;
            }
            totc += bc as core::ffi::c_int;
            fave[bt as usize] += 1;
            (*s).selector[nSelectors as usize] = bt as UChar;
            nSelectors += 1;
            if nGroups == 6 as core::ffi::c_int
                && 50 as core::ffi::c_int
                    == ge as core::ffi::c_int - gs as core::ffi::c_int
                        + 1 as core::ffi::c_int
            {
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 2 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 3 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 4 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 5 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 6 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 7 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 8 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 9 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 10 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 11 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 12 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 13 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 14 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 15 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 16 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 17 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 18 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 19 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 20 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 21 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 22 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 23 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 24 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 25 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 26 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 27 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 28 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 29 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 30 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 31 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 32 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 33 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 34 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 35 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 36 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 37 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 38 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 39 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 40 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 41 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 42 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 43 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 44 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 45 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 46 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 47 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 48 as core::ffi::c_int) as isize)
                    as usize] += 1;
                (*s)
                    .rfreq[bt
                    as usize][*mtfv
                    .offset((gs as core::ffi::c_int + 49 as core::ffi::c_int) as isize)
                    as usize] += 1;
            } else {
                i = gs;
                while i <= ge {
                    (*s).rfreq[bt as usize][*mtfv.offset(i as isize) as usize] += 1;
                    i += 1;
                }
            }
            gs = (ge as core::ffi::c_int + 1 as core::ffi::c_int) as Int32;
        }
        if (*s).verbosity >= 3 as core::ffi::c_int {
            fprintf(
                stderr,
                b"      pass %d: size is %d, grp uses are \0" as *const u8
                    as *const core::ffi::c_char,
                iter as core::ffi::c_int + 1 as core::ffi::c_int,
                totc as core::ffi::c_int / 8 as core::ffi::c_int,
            );
            t = 0 as core::ffi::c_int as Int32;
            while t < nGroups {
                fprintf(
                    stderr,
                    b"%d \0" as *const u8 as *const core::ffi::c_char,
                    fave[t as usize],
                );
                t += 1;
            }
            fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        }
        t = 0 as core::ffi::c_int as Int32;
        while t < nGroups {
            BZ2_hbMakeCodeLengths(
                &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                &mut *(*((*s).rfreq).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                alphaSize,
                17 as Int32,
            );
            t += 1;
        }
        iter += 1;
    }
    if !(nGroups < 8 as core::ffi::c_int) {
        BZ2_bz__AssertH__fail(3002 as core::ffi::c_int);
    }
    if !(nSelectors < 32768 as core::ffi::c_int
        && nSelectors
            <= 2 as core::ffi::c_int
                + 900000 as core::ffi::c_int / 50 as core::ffi::c_int)
    {
        BZ2_bz__AssertH__fail(3003 as core::ffi::c_int);
    }
    let mut pos: [UChar; 6] = [0; 6];
    let mut ll_i: UChar = 0;
    let mut tmp2: UChar = 0;
    let mut tmp: UChar = 0;
    i = 0 as core::ffi::c_int as Int32;
    while i < nGroups {
        pos[i as usize] = i as UChar;
        i += 1;
    }
    i = 0 as core::ffi::c_int as Int32;
    while i < nSelectors {
        ll_i = (*s).selector[i as usize];
        j = 0 as core::ffi::c_int as Int32;
        tmp = pos[j as usize];
        while ll_i as core::ffi::c_int != tmp as core::ffi::c_int {
            j += 1;
            tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2;
        }
        pos[0 as core::ffi::c_int as usize] = tmp;
        (*s).selectorMtf[i as usize] = j as UChar;
        i += 1;
    }
    t = 0 as core::ffi::c_int as Int32;
    while t < nGroups {
        minLen = 32 as core::ffi::c_int as Int32;
        maxLen = 0 as core::ffi::c_int as Int32;
        i = 0 as core::ffi::c_int as Int32;
        while i < alphaSize {
            if (*s).len[t as usize][i as usize] as core::ffi::c_int > maxLen {
                maxLen = (*s).len[t as usize][i as usize] as Int32;
            }
            if ((*s).len[t as usize][i as usize] as core::ffi::c_int) < minLen {
                minLen = (*s).len[t as usize][i as usize] as Int32;
            }
            i += 1;
        }
        if maxLen > 17 as core::ffi::c_int {
            BZ2_bz__AssertH__fail(3004 as core::ffi::c_int);
        }
        if minLen < 1 as core::ffi::c_int {
            BZ2_bz__AssertH__fail(3005 as core::ffi::c_int);
        }
        BZ2_hbAssignCodes(
            &mut *(*((*s).code).as_mut_ptr().offset(t as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
            &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
            minLen,
            maxLen,
            alphaSize,
        );
        t += 1;
    }
    let mut inUse16: [Bool; 16] = [0; 16];
    i = 0 as core::ffi::c_int as Int32;
    while i < 16 as core::ffi::c_int {
        inUse16[i as usize] = 0 as core::ffi::c_int as Bool;
        j = 0 as core::ffi::c_int as Int32;
        while j < 16 as core::ffi::c_int {
            if (*s).inUse[(i * 16 as Int32 + j) as usize] != 0 {
                inUse16[i as usize] = 1 as core::ffi::c_int as Bool;
            }
            j += 1;
        }
        i += 1;
    }
    nBytes = (*s).numZ;
    i = 0 as core::ffi::c_int as Int32;
    while i < 16 as core::ffi::c_int {
        if inUse16[i as usize] != 0 {
            bsW(s, 1 as Int32, 1 as UInt32);
        } else {
            bsW(s, 1 as Int32, 0 as UInt32);
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as Int32;
    while i < 16 as core::ffi::c_int {
        if inUse16[i as usize] != 0 {
            j = 0 as core::ffi::c_int as Int32;
            while j < 16 as core::ffi::c_int {
                if (*s).inUse[(i * 16 as Int32 + j) as usize] != 0 {
                    bsW(s, 1 as Int32, 1 as UInt32);
                } else {
                    bsW(s, 1 as Int32, 0 as UInt32);
                }
                j += 1;
            }
        }
        i += 1;
    }
    if (*s).verbosity >= 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"      bytes: mapping %d, \0" as *const u8 as *const core::ffi::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    bsW(s, 3 as Int32, nGroups as UInt32);
    bsW(s, 15 as Int32, nSelectors as UInt32);
    i = 0 as core::ffi::c_int as Int32;
    while i < nSelectors {
        j = 0 as core::ffi::c_int as Int32;
        while j < (*s).selectorMtf[i as usize] as core::ffi::c_int {
            bsW(s, 1 as Int32, 1 as UInt32);
            j += 1;
        }
        bsW(s, 1 as Int32, 0 as UInt32);
        i += 1;
    }
    if (*s).verbosity >= 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"selectors %d, \0" as *const u8 as *const core::ffi::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    t = 0 as core::ffi::c_int as Int32;
    while t < nGroups {
        let mut curr: Int32 = (*s).len[t as usize][0 as core::ffi::c_int as usize]
            as Int32;
        bsW(s, 5 as Int32, curr as UInt32);
        i = 0 as core::ffi::c_int as Int32;
        while i < alphaSize {
            while curr < (*s).len[t as usize][i as usize] as core::ffi::c_int {
                bsW(s, 2 as Int32, 2 as UInt32);
                curr += 1;
            }
            while curr > (*s).len[t as usize][i as usize] as core::ffi::c_int {
                bsW(s, 2 as Int32, 3 as UInt32);
                curr -= 1;
            }
            bsW(s, 1 as Int32, 0 as UInt32);
            i += 1;
        }
        t += 1;
    }
    if (*s).verbosity >= 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"code lengths %d, \0" as *const u8 as *const core::ffi::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    selCtr = 0 as core::ffi::c_int as Int32;
    gs = 0 as core::ffi::c_int as Int32;
    while 1 as core::ffi::c_int as Bool != 0 {
        if gs >= (*s).nMTF {
            break;
        }
        ge = (gs as core::ffi::c_int + 50 as core::ffi::c_int - 1 as core::ffi::c_int)
            as Int32;
        if ge >= (*s).nMTF {
            ge = ((*s).nMTF as core::ffi::c_int - 1 as core::ffi::c_int) as Int32;
        }
        if !(((*s).selector[selCtr as usize] as core::ffi::c_int) < nGroups) {
            BZ2_bz__AssertH__fail(3006 as core::ffi::c_int);
        }
        if nGroups == 6 as core::ffi::c_int
            && 50 as core::ffi::c_int
                == ge as core::ffi::c_int - gs as core::ffi::c_int
                    + 1 as core::ffi::c_int
        {
            let mut mtfv_i: UInt16 = 0;
            let mut s_len_sel_selCtr: *mut UChar = &mut *(*((*s).len)
                .as_mut_ptr()
                .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut UChar;
            let mut s_code_sel_selCtr: *mut Int32 = &mut *(*((*s).code)
                .as_mut_ptr()
                .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut Int32;
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 0 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 2 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 3 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 4 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 5 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 6 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 7 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 8 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 9 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 10 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 11 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 12 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 13 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 14 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 15 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 16 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 17 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 18 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 19 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 20 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 21 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 22 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 23 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 24 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 25 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 26 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 27 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 28 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 29 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 30 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 31 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 32 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 33 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 34 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 35 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 36 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 37 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 38 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 39 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 40 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 41 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 42 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 43 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 44 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 45 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 46 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 47 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 48 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv
                .offset((gs as core::ffi::c_int + 49 as core::ffi::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
        } else {
            i = gs;
            while i <= ge {
                bsW(
                    s,
                    (*s)
                        .len[(*s).selector[selCtr as usize]
                        as usize][*mtfv.offset(i as isize) as usize] as Int32,
                    (*s)
                        .code[(*s).selector[selCtr as usize]
                        as usize][*mtfv.offset(i as isize) as usize] as UInt32,
                );
                i += 1;
            }
        }
        gs = (ge as core::ffi::c_int + 1 as core::ffi::c_int) as Int32;
        selCtr += 1;
    }
    if !(selCtr == nSelectors) {
        BZ2_bz__AssertH__fail(3007 as core::ffi::c_int);
    }
    if (*s).verbosity >= 3 as core::ffi::c_int {
        fprintf(
            stderr,
            b"codes %d\n\0" as *const u8 as *const core::ffi::c_char,
            (*s).numZ - nBytes,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_compressBlock(mut s: *mut EState, mut is_last_block: Bool) {
    if (*s).nblock > 0 as core::ffi::c_int {
        (*s).blockCRC = !(*s).blockCRC;
        (*s).combinedCRC = (*s).combinedCRC << 1 as core::ffi::c_int
            | (*s).combinedCRC >> 31 as core::ffi::c_int;
        (*s).combinedCRC ^= (*s).blockCRC as core::ffi::c_uint;
        if (*s).blockNo > 1 as core::ffi::c_int {
            (*s).numZ = 0 as core::ffi::c_int as Int32;
        }
        if (*s).verbosity >= 2 as core::ffi::c_int {
            fprintf(
                stderr,
                b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\0"
                    as *const u8 as *const core::ffi::c_char,
                (*s).blockNo,
                (*s).blockCRC,
                (*s).combinedCRC,
                (*s).nblock,
            );
        }
        BZ2_blockSort(s);
    }
    (*s).zbits = &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize)
        as *mut UChar;
    if (*s).blockNo == 1 as core::ffi::c_int {
        BZ2_bsInitWrite(s);
        bsPutUChar(s, 0x42 as UChar);
        bsPutUChar(s, 0x5a as UChar);
        bsPutUChar(s, 0x68 as UChar);
        bsPutUChar(s, (0x30 as Int32 + (*s).blockSize100k) as UChar);
    }
    if (*s).nblock > 0 as core::ffi::c_int {
        bsPutUChar(s, 0x31 as UChar);
        bsPutUChar(s, 0x41 as UChar);
        bsPutUChar(s, 0x59 as UChar);
        bsPutUChar(s, 0x26 as UChar);
        bsPutUChar(s, 0x53 as UChar);
        bsPutUChar(s, 0x59 as UChar);
        bsPutUInt32(s, (*s).blockCRC);
        bsW(s, 1 as Int32, 0 as UInt32);
        bsW(s, 24 as Int32, (*s).origPtr as UInt32);
        generateMTFValues(s);
        sendMTFValues(s);
    }
    if is_last_block != 0 {
        bsPutUChar(s, 0x17 as UChar);
        bsPutUChar(s, 0x72 as UChar);
        bsPutUChar(s, 0x45 as UChar);
        bsPutUChar(s, 0x38 as UChar);
        bsPutUChar(s, 0x50 as UChar);
        bsPutUChar(s, 0x90 as UChar);
        bsPutUInt32(s, (*s).combinedCRC);
        if (*s).verbosity >= 2 as core::ffi::c_int {
            fprintf(
                stderr,
                b"    final combined CRC = 0x%08x\n   \0" as *const u8
                    as *const core::ffi::c_char,
                (*s).combinedCRC,
            );
        }
        bsFinishWrite(s);
    }
}
