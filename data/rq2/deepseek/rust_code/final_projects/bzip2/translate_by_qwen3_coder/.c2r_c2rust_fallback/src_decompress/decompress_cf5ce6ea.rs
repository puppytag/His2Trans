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
    static mut BZ2_rNums: [Int32; 512];
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn BZ2_indexIntoF(_: Int32, _: *mut Int32) -> Int32;
    fn BZ2_hbCreateDecodeTables(
        _: *mut Int32,
        _: *mut Int32,
        _: *mut Int32,
        _: *mut UChar,
        _: Int32,
        _: Int32,
        _: Int32,
    );
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
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: Int32,
    pub state_out_ch: UChar,
    pub state_out_len: Int32,
    pub blockRandomised: Bool,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockSize100k: Int32,
    pub smallDecompress: Bool,
    pub currBlockNo: Int32,
    pub verbosity: Int32,
    pub origPtr: Int32,
    pub tPos: UInt32,
    pub k0: Int32,
    pub unzftab: [Int32; 256],
    pub nblock_used: Int32,
    pub cftab: [Int32; 257],
    pub cftabCopy: [Int32; 257],
    pub tt: *mut UInt32,
    pub ll16: *mut UInt16,
    pub ll4: *mut UChar,
    pub storedBlockCRC: UInt32,
    pub storedCombinedCRC: UInt32,
    pub calculatedBlockCRC: UInt32,
    pub calculatedCombinedCRC: UInt32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub inUse16: [Bool; 16],
    pub seqToUnseq: [UChar; 256],
    pub mtfa: [UChar; 4096],
    pub mtfbase: [Int32; 16],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub limit: [[Int32; 258]; 6],
    pub base: [[Int32; 258]; 6],
    pub perm: [[Int32; 258]; 6],
    pub minLens: [Int32; 6],
    pub save_i: Int32,
    pub save_j: Int32,
    pub save_t: Int32,
    pub save_alphaSize: Int32,
    pub save_nGroups: Int32,
    pub save_nSelectors: Int32,
    pub save_EOB: Int32,
    pub save_groupNo: Int32,
    pub save_groupPos: Int32,
    pub save_nextSym: Int32,
    pub save_nblockMAX: Int32,
    pub save_nblock: Int32,
    pub save_es: Int32,
    pub save_N: Int32,
    pub save_curr: Int32,
    pub save_zt: Int32,
    pub save_zn: Int32,
    pub save_zvec: Int32,
    pub save_zj: Int32,
    pub save_gSel: Int32,
    pub save_gMinlen: Int32,
    pub save_gLimit: *mut Int32,
    pub save_gBase: *mut Int32,
    pub save_gPerm: *mut Int32,
}
unsafe extern "C" fn makeMaps_d(mut s: *mut DState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as core::ffi::c_int as Int32;
    i = 0 as core::ffi::c_int as Int32;
    while i < 256 as core::ffi::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).seqToUnseq[(*s).nInUse as usize] = i as UChar;
            (*s).nInUse += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_decompress(mut s: *mut DState) -> Int32 {
    let mut current_block: u64;
    let mut uc: UChar = 0;
    let mut retVal: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut strm: *mut bz_stream = (*s).strm;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut t: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut EOB: Int32 = 0;
    let mut groupNo: Int32 = 0;
    let mut groupPos: Int32 = 0;
    let mut nextSym: Int32 = 0;
    let mut nblockMAX: Int32 = 0;
    let mut nblock: Int32 = 0;
    let mut es: Int32 = 0;
    let mut N: Int32 = 0;
    let mut curr: Int32 = 0;
    let mut zt: Int32 = 0;
    let mut zn: Int32 = 0;
    let mut zvec: Int32 = 0;
    let mut zj: Int32 = 0;
    let mut gSel: Int32 = 0;
    let mut gMinlen: Int32 = 0;
    let mut gLimit: *mut Int32 = 0 as *mut Int32;
    let mut gBase: *mut Int32 = 0 as *mut Int32;
    let mut gPerm: *mut Int32 = 0 as *mut Int32;
    if (*s).state == 10 as core::ffi::c_int {
        (*s).save_i = 0 as core::ffi::c_int as Int32;
        (*s).save_j = 0 as core::ffi::c_int as Int32;
        (*s).save_t = 0 as core::ffi::c_int as Int32;
        (*s).save_alphaSize = 0 as core::ffi::c_int as Int32;
        (*s).save_nGroups = 0 as core::ffi::c_int as Int32;
        (*s).save_nSelectors = 0 as core::ffi::c_int as Int32;
        (*s).save_EOB = 0 as core::ffi::c_int as Int32;
        (*s).save_groupNo = 0 as core::ffi::c_int as Int32;
        (*s).save_groupPos = 0 as core::ffi::c_int as Int32;
        (*s).save_nextSym = 0 as core::ffi::c_int as Int32;
        (*s).save_nblockMAX = 0 as core::ffi::c_int as Int32;
        (*s).save_nblock = 0 as core::ffi::c_int as Int32;
        (*s).save_es = 0 as core::ffi::c_int as Int32;
        (*s).save_N = 0 as core::ffi::c_int as Int32;
        (*s).save_curr = 0 as core::ffi::c_int as Int32;
        (*s).save_zt = 0 as core::ffi::c_int as Int32;
        (*s).save_zn = 0 as core::ffi::c_int as Int32;
        (*s).save_zvec = 0 as core::ffi::c_int as Int32;
        (*s).save_zj = 0 as core::ffi::c_int as Int32;
        (*s).save_gSel = 0 as core::ffi::c_int as Int32;
        (*s).save_gMinlen = 0 as core::ffi::c_int as Int32;
        (*s).save_gLimit = 0 as *mut Int32;
        (*s).save_gBase = 0 as *mut Int32;
        (*s).save_gPerm = 0 as *mut Int32;
    }
    i = (*s).save_i;
    j = (*s).save_j;
    t = (*s).save_t;
    alphaSize = (*s).save_alphaSize;
    nGroups = (*s).save_nGroups;
    nSelectors = (*s).save_nSelectors;
    EOB = (*s).save_EOB;
    groupNo = (*s).save_groupNo;
    groupPos = (*s).save_groupPos;
    nextSym = (*s).save_nextSym;
    nblockMAX = (*s).save_nblockMAX;
    nblock = (*s).save_nblock;
    es = (*s).save_es;
    N = (*s).save_N;
    curr = (*s).save_curr;
    zt = (*s).save_zt;
    zn = (*s).save_zn;
    zvec = (*s).save_zvec;
    zj = (*s).save_zj;
    gSel = (*s).save_gSel;
    gMinlen = (*s).save_gMinlen;
    gLimit = (*s).save_gLimit;
    gBase = (*s).save_gBase;
    gPerm = (*s).save_gPerm;
    retVal = 0 as core::ffi::c_int as Int32;
    match (*s).state {
        10 => {
            (*s).state = 10 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 5235537862154438448;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v: UInt32 = 0;
                    v = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v as UChar;
                    current_block = 5235537862154438448;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x42 as core::ffi::c_int {
                        retVal = -(5 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 14932313468587699050;
                    }
                }
            }
        }
        11 => {
            current_block = 14932313468587699050;
        }
        12 => {
            current_block = 3846739063131686548;
        }
        13 => {
            current_block = 9872773783459987952;
        }
        14 => {
            current_block = 13582656083946270270;
        }
        15 => {
            current_block = 12237300376741949836;
        }
        16 => {
            current_block = 13986342074430558528;
        }
        17 => {
            current_block = 7655914781174535859;
        }
        18 => {
            current_block = 10500237484492514752;
        }
        19 => {
            current_block = 8308794451053641010;
        }
        20 => {
            current_block = 13651423580372165165;
        }
        21 => {
            current_block = 18233339199029667;
        }
        22 => {
            current_block = 13563171475922671077;
        }
        23 => {
            current_block = 5378243231166472823;
        }
        24 => {
            current_block = 14507743300660289476;
        }
        25 => {
            current_block = 14371489722196013220;
        }
        26 => {
            current_block = 1725138493261267529;
        }
        27 => {
            current_block = 5898665363076154828;
        }
        28 => {
            current_block = 3718145471434927944;
        }
        29 => {
            current_block = 16687080193289705592;
        }
        30 => {
            current_block = 15439061234641355781;
        }
        31 => {
            current_block = 10755683262200018870;
        }
        32 => {
            current_block = 3307780312744082722;
        }
        33 => {
            current_block = 3122919031762406036;
        }
        34 => {
            current_block = 12137889358298489534;
        }
        35 => {
            current_block = 7955586766290726337;
        }
        36 => {
            current_block = 10010940406495358186;
        }
        37 => {
            current_block = 14149047132912108267;
        }
        38 => {
            current_block = 8517808508854580601;
        }
        39 => {
            current_block = 258419738100199176;
        }
        40 => {
            current_block = 11311317566952550995;
        }
        41 => {
            current_block = 13566908542881619119;
        }
        42 => {
            current_block = 1525014864485157485;
        }
        43 => {
            current_block = 10861016926129603098;
        }
        44 => {
            current_block = 15431978180501365786;
        }
        45 => {
            current_block = 308338134236164298;
        }
        46 => {
            current_block = 10688835657438208386;
        }
        47 => {
            current_block = 16737651353374589449;
        }
        48 => {
            current_block = 6997290010865393001;
        }
        49 => {
            current_block = 16060096534408587340;
        }
        50 => {
            current_block = 7179728134253626143;
        }
        _ => {
            if 0 as core::ffi::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4001 as core::ffi::c_int);
            }
            if 0 as core::ffi::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4002 as core::ffi::c_int);
            }
            current_block = 10837989821191059030;
        }
    }
    match current_block {
        14932313468587699050 => {
            (*s).state = 11 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 2168227384378665163;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_0: UInt32 = 0;
                    v_0 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_0 as UChar;
                    current_block = 2168227384378665163;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x5a as core::ffi::c_int {
                        retVal = -(5 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 3846739063131686548;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        3846739063131686548 => {
            (*s).state = 12 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 178030534879405462;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_1: UInt32 = 0;
                    v_1 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_1 as UChar;
                    current_block = 178030534879405462;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x68 as core::ffi::c_int {
                        retVal = -(5 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 9872773783459987952;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        9872773783459987952 => {
            (*s).state = 13 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 7639320476250304355;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_2: UInt32 = 0;
                    v_2 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    (*s).blockSize100k = v_2 as Int32;
                    current_block = 7639320476250304355;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if (*s).blockSize100k
                        < 0x30 as core::ffi::c_int + 1 as core::ffi::c_int
                        || (*s).blockSize100k
                            > 0x30 as core::ffi::c_int + 9 as core::ffi::c_int
                    {
                        retVal = -(5 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        (*s).blockSize100k -= 0x30 as core::ffi::c_int;
                        if (*s).smallDecompress != 0 {
                            (*s).ll16 = ((*strm).bzalloc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                (((*s).blockSize100k as core::ffi::c_int
                                    * 100000 as core::ffi::c_int) as usize)
                                    .wrapping_mul(::core::mem::size_of::<UInt16>() as usize)
                                    as core::ffi::c_int,
                                1 as core::ffi::c_int,
                            ) as *mut UInt16;
                            (*s).ll4 = ((*strm).bzalloc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                ((1 as core::ffi::c_int
                                    + (*s).blockSize100k as core::ffi::c_int
                                        * 100000 as core::ffi::c_int >> 1 as core::ffi::c_int)
                                    as usize)
                                    .wrapping_mul(::core::mem::size_of::<UChar>() as usize)
                                    as core::ffi::c_int,
                                1 as core::ffi::c_int,
                            ) as *mut UChar;
                            if ((*s).ll16).is_null() || ((*s).ll4).is_null() {
                                retVal = -(3 as core::ffi::c_int) as Int32;
                                current_block = 10837989821191059030;
                            } else {
                                current_block = 13582656083946270270;
                            }
                        } else {
                            (*s).tt = ((*strm).bzalloc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                (((*s).blockSize100k as core::ffi::c_int
                                    * 100000 as core::ffi::c_int) as usize)
                                    .wrapping_mul(::core::mem::size_of::<Int32>() as usize)
                                    as core::ffi::c_int,
                                1 as core::ffi::c_int,
                            ) as *mut UInt32;
                            if ((*s).tt).is_null() {
                                retVal = -(3 as core::ffi::c_int) as Int32;
                                current_block = 10837989821191059030;
                            } else {
                                current_block = 13582656083946270270;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        13582656083946270270 => {
            (*s).state = 14 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 16937825661756021828;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_3: UInt32 = 0;
                    v_3 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_3 as UChar;
                    current_block = 16937825661756021828;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int == 0x17 as core::ffi::c_int {
                        current_block = 1525014864485157485;
                    } else if uc as core::ffi::c_int != 0x31 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 12237300376741949836;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1525014864485157485 => {
            (*s).state = 42 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 13733404100380861831;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_32: UInt32 = 0;
                    v_32 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_32 as UChar;
                    current_block = 13733404100380861831;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x72 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 10861016926129603098;
                    }
                }
            }
        }
        12237300376741949836 => {
            (*s).state = 15 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 1228639923084383292;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_4: UInt32 = 0;
                    v_4 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_4 as UChar;
                    current_block = 1228639923084383292;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x41 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 13986342074430558528;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        10861016926129603098 => {
            (*s).state = 43 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 12721425419429475574;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_33: UInt32 = 0;
                    v_33 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_33 as UChar;
                    current_block = 12721425419429475574;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x45 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 15431978180501365786;
                    }
                }
            }
        }
        13986342074430558528 => {
            (*s).state = 16 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 9235179519944561532;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_5: UInt32 = 0;
                    v_5 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_5 as UChar;
                    current_block = 9235179519944561532;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x59 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 7655914781174535859;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        15431978180501365786 => {
            (*s).state = 44 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 13813414375753095368;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_34: UInt32 = 0;
                    v_34 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_34 as UChar;
                    current_block = 13813414375753095368;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x38 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 308338134236164298;
                    }
                }
            }
        }
        7655914781174535859 => {
            (*s).state = 17 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 12467039471581323981;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_6: UInt32 = 0;
                    v_6 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_6 as UChar;
                    current_block = 12467039471581323981;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x26 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 10500237484492514752;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        308338134236164298 => {
            (*s).state = 45 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 1472103348880861285;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_35: UInt32 = 0;
                    v_35 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_35 as UChar;
                    current_block = 1472103348880861285;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x50 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 10688835657438208386;
                    }
                }
            }
        }
        10500237484492514752 => {
            (*s).state = 18 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 13164310931121142693;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_7: UInt32 = 0;
                    v_7 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_7 as UChar;
                    current_block = 13164310931121142693;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x53 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        current_block = 8308794451053641010;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        10688835657438208386 => {
            (*s).state = 46 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 8232347840743503282;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_36: UInt32 = 0;
                    v_36 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_36 as UChar;
                    current_block = 8232347840743503282;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x90 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        (*s).storedCombinedCRC = 0 as UInt32;
                        current_block = 16737651353374589449;
                    }
                }
            }
        }
        8308794451053641010 => {
            (*s).state = 19 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 14723615986260991866;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_8: UInt32 = 0;
                    v_8 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_8 as UChar;
                    current_block = 14723615986260991866;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    if uc as core::ffi::c_int != 0x59 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        (*s).currBlockNo += 1;
                        if (*s).verbosity >= 2 as core::ffi::c_int {
                            fprintf(
                                stderr,
                                b"\n    [%d: huff+mtf \0" as *const u8
                                    as *const core::ffi::c_char,
                                (*s).currBlockNo,
                            );
                        }
                        (*s).storedBlockCRC = 0 as UInt32;
                        current_block = 13651423580372165165;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        16737651353374589449 => {
            (*s).state = 47 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 5465979950226085365;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_37: UInt32 = 0;
                    v_37 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_37 as UChar;
                    current_block = 5465979950226085365;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedCombinedCRC = (*s).storedCombinedCRC
                        << 8 as core::ffi::c_int | uc as UInt32;
                    current_block = 6997290010865393001;
                }
            }
        }
        13651423580372165165 => {
            (*s).state = 20 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 15627786036016112248;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_9: UInt32 = 0;
                    v_9 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_9 as UChar;
                    current_block = 15627786036016112248;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedBlockCRC = (*s).storedBlockCRC << 8 as core::ffi::c_int
                        | uc as UInt32;
                    current_block = 18233339199029667;
                }
            }
        }
        _ => {}
    }
    match current_block {
        6997290010865393001 => {
            (*s).state = 48 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 3854366583354019639;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_38: UInt32 = 0;
                    v_38 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_38 as UChar;
                    current_block = 3854366583354019639;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedCombinedCRC = (*s).storedCombinedCRC
                        << 8 as core::ffi::c_int | uc as UInt32;
                    current_block = 16060096534408587340;
                }
            }
        }
        18233339199029667 => {
            (*s).state = 21 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 13493279574219925475;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_10: UInt32 = 0;
                    v_10 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_10 as UChar;
                    current_block = 13493279574219925475;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedBlockCRC = (*s).storedBlockCRC << 8 as core::ffi::c_int
                        | uc as UInt32;
                    current_block = 13563171475922671077;
                }
            }
        }
        _ => {}
    }
    match current_block {
        16060096534408587340 => {
            (*s).state = 49 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 12082794684616777938;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_39: UInt32 = 0;
                    v_39 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_39 as UChar;
                    current_block = 12082794684616777938;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedCombinedCRC = (*s).storedCombinedCRC
                        << 8 as core::ffi::c_int | uc as UInt32;
                    current_block = 7179728134253626143;
                }
            }
        }
        13563171475922671077 => {
            (*s).state = 22 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 4839309778395429725;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_11: UInt32 = 0;
                    v_11 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_11 as UChar;
                    current_block = 4839309778395429725;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedBlockCRC = (*s).storedBlockCRC << 8 as core::ffi::c_int
                        | uc as UInt32;
                    current_block = 5378243231166472823;
                }
            }
        }
        _ => {}
    }
    match current_block {
        5378243231166472823 => {
            (*s).state = 23 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 17937968408868551711;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_12: UInt32 = 0;
                    v_12 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_12 as UChar;
                    current_block = 17937968408868551711;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedBlockCRC = (*s).storedBlockCRC << 8 as core::ffi::c_int
                        | uc as UInt32;
                    current_block = 14507743300660289476;
                }
            }
        }
        7179728134253626143 => {
            (*s).state = 50 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 6276941480907995842;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_40: UInt32 = 0;
                    v_40 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_40 as UChar;
                    current_block = 6276941480907995842;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).storedCombinedCRC = (*s).storedCombinedCRC
                        << 8 as core::ffi::c_int | uc as UInt32;
                    (*s).state = 1 as core::ffi::c_int as Int32;
                    retVal = 4 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                }
            }
        }
        _ => {}
    }
    match current_block {
        14507743300660289476 => {
            (*s).state = 24 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 7926734633677835471;
                    break;
                }
                if (*s).bsLive >= 1 as core::ffi::c_int {
                    let mut v_13: UInt32 = 0;
                    v_13 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 1 as core::ffi::c_int;
                    (*s).blockRandomised = v_13 as Bool;
                    current_block = 7926734633677835471;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).origPtr = 0 as core::ffi::c_int as Int32;
                    current_block = 14371489722196013220;
                }
            }
        }
        _ => {}
    }
    match current_block {
        14371489722196013220 => {
            (*s).state = 25 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 5948065351908552372;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_14: UInt32 = 0;
                    v_14 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_14 as UChar;
                    current_block = 5948065351908552372;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as core::ffi::c_int | uc as Int32;
                    current_block = 1725138493261267529;
                }
            }
        }
        _ => {}
    }
    match current_block {
        1725138493261267529 => {
            (*s).state = 26 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 8940662058537996670;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_15: UInt32 = 0;
                    v_15 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_15 as UChar;
                    current_block = 8940662058537996670;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as core::ffi::c_int | uc as Int32;
                    current_block = 5898665363076154828;
                }
            }
        }
        _ => {}
    }
    match current_block {
        5898665363076154828 => {
            (*s).state = 27 as core::ffi::c_int as Int32;
            loop {
                if !(1 as core::ffi::c_int as Bool != 0) {
                    current_block = 13366002463409402866;
                    break;
                }
                if (*s).bsLive >= 8 as core::ffi::c_int {
                    let mut v_16: UInt32 = 0;
                    v_16 = ((*s).bsBuff as core::ffi::c_uint
                        >> (*s).bsLive as core::ffi::c_int - 8 as core::ffi::c_int
                        & (((1 as core::ffi::c_int) << 8 as core::ffi::c_int)
                            - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                    (*s).bsLive -= 8 as core::ffi::c_int;
                    uc = v_16 as UChar;
                    current_block = 13366002463409402866;
                    break;
                } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                    retVal = 0 as core::ffi::c_int as Int32;
                    current_block = 10837989821191059030;
                    break;
                } else {
                    (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as core::ffi::c_int;
                    (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                    (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                        .wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                        (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                            .wrapping_add(1);
                    }
                }
            }
            match current_block {
                10837989821191059030 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as core::ffi::c_int | uc as Int32;
                    if (*s).origPtr < 0 as core::ffi::c_int {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else if (*s).origPtr
                        > 10 as Int32 + 100000 as Int32 * (*s).blockSize100k
                    {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                    } else {
                        i = 0 as core::ffi::c_int as Int32;
                        current_block = 454873545234741267;
                    }
                }
            }
        }
        _ => {}
    }
    'c_9709: loop {
        match current_block {
            10837989821191059030 => {
                (*s).save_i = i;
                break;
            }
            11311317566952550995 => {
                (*s).state = 40 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_30: UInt32 = 0;
                        v_30 = ((*s).bsBuff as core::ffi::c_uint >> (*s).bsLive - zn
                            & (((1 as core::ffi::c_int) << zn) - 1 as core::ffi::c_int)
                                as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= zn as core::ffi::c_int;
                        zvec = v_30 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                current_block = 16348713635569416413;
            }
            258419738100199176 => {
                (*s).state = 39 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_29: UInt32 = 0;
                        v_29 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        zj = v_29 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as core::ffi::c_int | zj;
                current_block = 7923635230025172457;
            }
            8517808508854580601 => {
                (*s).state = 38 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_28: UInt32 = 0;
                        v_28 = ((*s).bsBuff as core::ffi::c_uint >> (*s).bsLive - zn
                            & (((1 as core::ffi::c_int) << zn) - 1 as core::ffi::c_int)
                                as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= zn as core::ffi::c_int;
                        zvec = v_28 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                current_block = 7923635230025172457;
            }
            14149047132912108267 => {
                (*s).state = 37 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_27: UInt32 = 0;
                        v_27 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        zj = v_27 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as core::ffi::c_int | zj;
                current_block = 9186389159759284570;
            }
            10010940406495358186 => {
                (*s).state = 36 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_26: UInt32 = 0;
                        v_26 = ((*s).bsBuff as core::ffi::c_uint >> (*s).bsLive - zn
                            & (((1 as core::ffi::c_int) << zn) - 1 as core::ffi::c_int)
                                as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= zn as core::ffi::c_int;
                        zvec = v_26 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                current_block = 9186389159759284570;
            }
            7955586766290726337 => {
                (*s).state = 35 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_25: UInt32 = 0;
                        v_25 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        uc = v_25 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if uc as core::ffi::c_int == 0 as core::ffi::c_int {
                    curr += 1;
                } else {
                    curr -= 1;
                }
                current_block = 5533056661327372531;
            }
            12137889358298489534 => {
                (*s).state = 34 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_24: UInt32 = 0;
                        v_24 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        uc = v_24 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if !(uc as core::ffi::c_int == 0 as core::ffi::c_int) {
                    current_block = 7955586766290726337;
                    continue;
                }
                current_block = 7746242308555130918;
            }
            3122919031762406036 => {
                (*s).state = 33 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 5 as core::ffi::c_int {
                        let mut v_23: UInt32 = 0;
                        v_23 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 5 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 5 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 5 as core::ffi::c_int;
                        curr = v_23 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                i = 0 as core::ffi::c_int as Int32;
                current_block = 16642413284942005565;
            }
            3307780312744082722 => {
                (*s).state = 32 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_21: UInt32 = 0;
                        v_21 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        uc = v_21 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if uc as core::ffi::c_int == 0 as core::ffi::c_int {
                    current_block = 10081471997089450706;
                } else {
                    j += 1;
                    if j >= nGroups {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                        continue;
                    } else {
                        current_block = 16531797892856733396;
                    }
                }
            }
            10755683262200018870 => {
                (*s).state = 31 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 15 as core::ffi::c_int {
                        let mut v_20: UInt32 = 0;
                        v_20 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 15 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 15 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 15 as core::ffi::c_int;
                        nSelectors = v_20 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if nSelectors < 1 as core::ffi::c_int {
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue;
                } else {
                    i = 0 as core::ffi::c_int as Int32;
                }
                current_block = 3503188808869013853;
            }
            15439061234641355781 => {
                (*s).state = 30 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 3 as core::ffi::c_int {
                        let mut v_19: UInt32 = 0;
                        v_19 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 3 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 3 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 3 as core::ffi::c_int;
                        nGroups = v_19 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if !(nGroups < 2 as core::ffi::c_int || nGroups > 6 as core::ffi::c_int)
                {
                    current_block = 10755683262200018870;
                    continue;
                }
                retVal = -(4 as core::ffi::c_int) as Int32;
                current_block = 10837989821191059030;
                continue;
            }
            16687080193289705592 => {
                (*s).state = 29 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_18: UInt32 = 0;
                        v_18 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        uc = v_18 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if uc as core::ffi::c_int == 1 as core::ffi::c_int {
                    (*s).inUse[(i * 16 as Int32 + j) as usize] = 1 as core::ffi::c_int
                        as Bool;
                }
                j += 1;
                current_block = 16953886395775657100;
            }
            454873545234741267 => {
                if i < 16 as core::ffi::c_int {
                    current_block = 3718145471434927944;
                    continue;
                }
                i = 0 as core::ffi::c_int as Int32;
                while i < 256 as core::ffi::c_int {
                    (*s).inUse[i as usize] = 0 as core::ffi::c_int as Bool;
                    i += 1;
                }
                i = 0 as core::ffi::c_int as Int32;
                current_block = 15415362524153386998;
            }
            3718145471434927944 => {
                (*s).state = 28 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_17: UInt32 = 0;
                        v_17 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        uc = v_17 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                if uc as core::ffi::c_int == 1 as core::ffi::c_int {
                    (*s).inUse16[i as usize] = 1 as core::ffi::c_int as Bool;
                } else {
                    (*s).inUse16[i as usize] = 0 as core::ffi::c_int as Bool;
                }
                i += 1;
                current_block = 454873545234741267;
                continue;
            }
            _ => {
                (*s).state = 41 as core::ffi::c_int as Int32;
                while 1 as core::ffi::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as core::ffi::c_int {
                        let mut v_31: UInt32 = 0;
                        v_31 = ((*s).bsBuff as core::ffi::c_uint
                            >> (*s).bsLive as core::ffi::c_int - 1 as core::ffi::c_int
                            & (((1 as core::ffi::c_int) << 1 as core::ffi::c_int)
                                - 1 as core::ffi::c_int) as core::ffi::c_uint) as UInt32;
                        (*s).bsLive -= 1 as core::ffi::c_int;
                        zj = v_31 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                        retVal = 0 as core::ffi::c_int as Int32;
                        current_block = 10837989821191059030;
                        continue 'c_9709;
                    } else {
                        (*s).bsBuff = (*s).bsBuff << 8 as core::ffi::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive += 8 as core::ffi::c_int;
                        (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
                        (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
                        (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32)
                            .wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                            (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                                .wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as core::ffi::c_int | zj;
                current_block = 16348713635569416413;
            }
        }
        match current_block {
            16348713635569416413 => {
                if zn > 20 as core::ffi::c_int {
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as core::ffi::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as core::ffi::c_int
                    {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn += 1;
                    current_block = 13566908542881619119;
                    continue;
                }
                current_block = 3575340618357869479;
            }
            7923635230025172457 => {
                if zn > 20 as core::ffi::c_int {
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as core::ffi::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as core::ffi::c_int
                    {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                        if nextSym == 0 as core::ffi::c_int
                            || nextSym == 1 as core::ffi::c_int
                        {
                            current_block = 5649595406143318745;
                        } else {
                            es += 1;
                            uc = (*s)
                                .seqToUnseq[(*s)
                                .mtfa[(*s).mtfbase[0 as core::ffi::c_int as usize] as usize]
                                as usize];
                            (*s).unzftab[uc as usize] += es as core::ffi::c_int;
                            if (*s).smallDecompress != 0 {
                                while es > 0 as core::ffi::c_int {
                                    if nblock >= nblockMAX {
                                        retVal = -(4 as core::ffi::c_int) as Int32;
                                        current_block = 10837989821191059030;
                                        continue 'c_9709;
                                    } else {
                                        *((*s).ll16).offset(nblock as isize) = uc as UInt16;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            } else {
                                while es > 0 as core::ffi::c_int {
                                    if nblock >= nblockMAX {
                                        retVal = -(4 as core::ffi::c_int) as Int32;
                                        current_block = 10837989821191059030;
                                        continue 'c_9709;
                                    } else {
                                        *((*s).tt).offset(nblock as isize) = uc as UInt32;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            }
                            current_block = 3575340618357869479;
                        }
                    }
                } else {
                    zn += 1;
                    current_block = 258419738100199176;
                    continue;
                }
            }
            9186389159759284570 => {
                if zn > 20 as core::ffi::c_int {
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as core::ffi::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as core::ffi::c_int
                    {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn += 1;
                    current_block = 14149047132912108267;
                    continue;
                }
                current_block = 3575340618357869479;
            }
            _ => {}
        }
        match current_block {
            3575340618357869479 => {
                if 1 as core::ffi::c_int as Bool != 0 {
                    if nextSym == EOB {
                        current_block = 4069074773319880902;
                    } else {
                        if nextSym == 0 as core::ffi::c_int
                            || nextSym == 1 as core::ffi::c_int
                        {
                            es = -(1 as core::ffi::c_int) as Int32;
                            N = 1 as core::ffi::c_int as Int32;
                        } else if nblock >= nblockMAX {
                            retVal = -(4 as core::ffi::c_int) as Int32;
                            current_block = 10837989821191059030;
                            continue;
                        } else {
                            let mut ii_0: Int32 = 0;
                            let mut jj_0: Int32 = 0;
                            let mut kk_0: Int32 = 0;
                            let mut pp: Int32 = 0;
                            let mut lno: Int32 = 0;
                            let mut off: Int32 = 0;
                            let mut nn: UInt32 = 0;
                            nn = (nextSym as core::ffi::c_int - 1 as core::ffi::c_int)
                                as UInt32;
                            if nn < 16 as core::ffi::c_uint {
                                pp = (*s).mtfbase[0 as core::ffi::c_int as usize];
                                uc = (*s).mtfa[(pp as UInt32).wrapping_add(nn) as usize];
                                while nn > 3 as core::ffi::c_uint {
                                    let mut z: Int32 = (pp as UInt32).wrapping_add(nn) as Int32;
                                    (*s).mtfa[z as usize] = (*s)
                                        .mtfa[(z as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize];
                                    (*s)
                                        .mtfa[(z as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize] = (*s)
                                        .mtfa[(z as core::ffi::c_int - 2 as core::ffi::c_int)
                                        as usize];
                                    (*s)
                                        .mtfa[(z as core::ffi::c_int - 2 as core::ffi::c_int)
                                        as usize] = (*s)
                                        .mtfa[(z as core::ffi::c_int - 3 as core::ffi::c_int)
                                        as usize];
                                    (*s)
                                        .mtfa[(z as core::ffi::c_int - 3 as core::ffi::c_int)
                                        as usize] = (*s)
                                        .mtfa[(z as core::ffi::c_int - 4 as core::ffi::c_int)
                                        as usize];
                                    nn = (nn as core::ffi::c_uint)
                                        .wrapping_sub(4 as core::ffi::c_uint) as UInt32 as UInt32;
                                }
                                while nn > 0 as core::ffi::c_uint {
                                    (*s).mtfa[(pp as UInt32).wrapping_add(nn) as usize] = (*s)
                                        .mtfa[(pp as core::ffi::c_uint)
                                        .wrapping_add(nn as core::ffi::c_uint)
                                        .wrapping_sub(1 as core::ffi::c_uint) as usize];
                                    nn = nn.wrapping_sub(1);
                                }
                                (*s).mtfa[pp as usize] = uc;
                            } else {
                                lno = (nn as core::ffi::c_uint)
                                    .wrapping_div(16 as core::ffi::c_uint) as Int32;
                                off = (nn as core::ffi::c_uint)
                                    .wrapping_rem(16 as core::ffi::c_uint) as Int32;
                                pp = (*s).mtfbase[lno as usize] + off;
                                uc = (*s).mtfa[pp as usize];
                                while pp > (*s).mtfbase[lno as usize] {
                                    (*s).mtfa[pp as usize] = (*s)
                                        .mtfa[(pp as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize];
                                    pp -= 1;
                                }
                                (*s).mtfbase[lno as usize] += 1;
                                while lno > 0 as core::ffi::c_int {
                                    (*s).mtfbase[lno as usize] -= 1;
                                    (*s).mtfa[(*s).mtfbase[lno as usize] as usize] = (*s)
                                        .mtfa[((*s)
                                        .mtfbase[(lno as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize] + 16 as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize];
                                    lno -= 1;
                                }
                                (*s).mtfbase[0 as core::ffi::c_int as usize] -= 1;
                                (*s)
                                    .mtfa[(*s).mtfbase[0 as core::ffi::c_int as usize]
                                    as usize] = uc;
                                if (*s).mtfbase[0 as core::ffi::c_int as usize]
                                    == 0 as core::ffi::c_int
                                {
                                    kk_0 = (4096 as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as Int32;
                                    ii_0 = (256 as core::ffi::c_int / 16 as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as Int32;
                                    while ii_0 >= 0 as core::ffi::c_int {
                                        jj_0 = (16 as core::ffi::c_int - 1 as core::ffi::c_int)
                                            as Int32;
                                        while jj_0 >= 0 as core::ffi::c_int {
                                            (*s).mtfa[kk_0 as usize] = (*s)
                                                .mtfa[((*s).mtfbase[ii_0 as usize] + jj_0) as usize];
                                            kk_0 -= 1;
                                            jj_0 -= 1;
                                        }
                                        (*s).mtfbase[ii_0 as usize] = (kk_0 as core::ffi::c_int
                                            + 1 as core::ffi::c_int) as Int32;
                                        ii_0 -= 1;
                                    }
                                }
                            }
                            (*s).unzftab[(*s).seqToUnseq[uc as usize] as usize] += 1;
                            if (*s).smallDecompress != 0 {
                                *((*s).ll16).offset(nblock as isize) = (*s)
                                    .seqToUnseq[uc as usize] as UInt16;
                            } else {
                                *((*s).tt).offset(nblock as isize) = (*s)
                                    .seqToUnseq[uc as usize] as UInt32;
                            }
                            nblock += 1;
                            if groupPos == 0 as core::ffi::c_int {
                                groupNo += 1;
                                if groupNo >= nSelectors {
                                    retVal = -(4 as core::ffi::c_int) as Int32;
                                    current_block = 10837989821191059030;
                                    continue;
                                } else {
                                    groupPos = 50 as core::ffi::c_int as Int32;
                                    gSel = (*s).selector[groupNo as usize] as Int32;
                                    gMinlen = (*s).minLens[gSel as usize];
                                    gLimit = &mut *(*((*s).limit)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                                    gPerm = &mut *(*((*s).perm)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                                    gBase = &mut *(*((*s).base)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                                }
                            }
                            groupPos -= 1;
                            zn = gMinlen;
                            current_block = 11311317566952550995;
                            continue;
                        }
                        current_block = 5649595406143318745;
                    }
                } else {
                    current_block = 4069074773319880902;
                }
                match current_block {
                    5649595406143318745 => {}
                    _ => {
                        if (*s).origPtr < 0 as core::ffi::c_int || (*s).origPtr >= nblock
                        {
                            retVal = -(4 as core::ffi::c_int) as Int32;
                            current_block = 10837989821191059030;
                            continue;
                        } else {
                            i = 0 as core::ffi::c_int as Int32;
                            while i <= 255 as core::ffi::c_int {
                                if (*s).unzftab[i as usize] < 0 as core::ffi::c_int
                                    || (*s).unzftab[i as usize] > nblock
                                {
                                    retVal = -(4 as core::ffi::c_int) as Int32;
                                    current_block = 10837989821191059030;
                                    continue 'c_9709;
                                } else {
                                    i += 1;
                                }
                            }
                            (*s).cftab[0 as core::ffi::c_int as usize] = 0
                                as core::ffi::c_int as Int32;
                            i = 1 as core::ffi::c_int as Int32;
                            while i <= 256 as core::ffi::c_int {
                                (*s).cftab[i as usize] = (*s)
                                    .unzftab[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                                    as usize];
                                i += 1;
                            }
                            i = 1 as core::ffi::c_int as Int32;
                            while i <= 256 as core::ffi::c_int {
                                (*s).cftab[i as usize]
                                    += (*s)
                                        .cftab[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as usize] as core::ffi::c_int;
                                i += 1;
                            }
                            i = 0 as core::ffi::c_int as Int32;
                            while i <= 256 as core::ffi::c_int {
                                if (*s).cftab[i as usize] < 0 as core::ffi::c_int
                                    || (*s).cftab[i as usize] > nblock
                                {
                                    retVal = -(4 as core::ffi::c_int) as Int32;
                                    current_block = 10837989821191059030;
                                    continue 'c_9709;
                                } else {
                                    i += 1;
                                }
                            }
                            i = 1 as core::ffi::c_int as Int32;
                            while i <= 256 as core::ffi::c_int {
                                if (*s)
                                    .cftab[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                                    as usize] > (*s).cftab[i as usize]
                                {
                                    retVal = -(4 as core::ffi::c_int) as Int32;
                                    current_block = 10837989821191059030;
                                    continue 'c_9709;
                                } else {
                                    i += 1;
                                }
                            }
                            (*s).state_out_len = 0 as core::ffi::c_int as Int32;
                            (*s).state_out_ch = 0 as UChar;
                            (*s).calculatedBlockCRC = 0xffffffff as UInt32;
                            (*s).state = 2 as core::ffi::c_int as Int32;
                            if (*s).verbosity >= 2 as core::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"rt+rld\0" as *const u8 as *const core::ffi::c_char,
                                );
                            }
                            if (*s).smallDecompress != 0 {
                                i = 0 as core::ffi::c_int as Int32;
                                while i <= 256 as core::ffi::c_int {
                                    (*s).cftabCopy[i as usize] = (*s).cftab[i as usize];
                                    i += 1;
                                }
                                i = 0 as core::ffi::c_int as Int32;
                                while i < nblock {
                                    uc = *((*s).ll16).offset(i as isize) as UChar;
                                    *((*s).ll16).offset(i as isize) = ((*s)
                                        .cftabCopy[uc as usize] & 0xffff as core::ffi::c_int)
                                        as UInt16;
                                    if i as core::ffi::c_int & 0x1 as core::ffi::c_int
                                        == 0 as core::ffi::c_int
                                    {
                                        *((*s).ll4).offset((i >> 1 as core::ffi::c_int) as isize) = (*((*s)
                                            .ll4)
                                            .offset((i >> 1 as core::ffi::c_int) as isize) as Int32
                                            & 0xf0 as Int32
                                            | (*s).cftabCopy[uc as usize] >> 16 as core::ffi::c_int)
                                            as UChar;
                                    } else {
                                        *((*s).ll4).offset((i >> 1 as core::ffi::c_int) as isize) = (*((*s)
                                            .ll4)
                                            .offset((i >> 1 as core::ffi::c_int) as isize) as Int32
                                            & 0xf as Int32
                                            | ((*s).cftabCopy[uc as usize] >> 16 as core::ffi::c_int)
                                                << 4 as core::ffi::c_int) as UChar;
                                    }
                                    (*s).cftabCopy[uc as usize] += 1;
                                    i += 1;
                                }
                                i = (*s).origPtr;
                                j = (*((*s).ll16).offset(i as isize) as core::ffi::c_uint
                                    | (*((*s).ll4).offset((i >> 1 as core::ffi::c_int) as isize)
                                        as core::ffi::c_uint
                                        >> ((i as core::ffi::c_int) << 2 as core::ffi::c_int
                                            & 0x4 as core::ffi::c_int) & 0xf as core::ffi::c_uint)
                                        << 16 as core::ffi::c_int) as Int32;
                                loop {
                                    let mut tmp_0: Int32 = (*((*s).ll16).offset(j as isize)
                                        as core::ffi::c_uint
                                        | (*((*s).ll4).offset((j >> 1 as core::ffi::c_int) as isize)
                                            as core::ffi::c_uint
                                            >> ((j as core::ffi::c_int) << 2 as core::ffi::c_int
                                                & 0x4 as core::ffi::c_int) & 0xf as core::ffi::c_uint)
                                            << 16 as core::ffi::c_int) as Int32;
                                    *((*s).ll16).offset(j as isize) = (i as core::ffi::c_int
                                        & 0xffff as core::ffi::c_int) as UInt16;
                                    if j as core::ffi::c_int & 0x1 as core::ffi::c_int
                                        == 0 as core::ffi::c_int
                                    {
                                        *((*s).ll4).offset((j >> 1 as core::ffi::c_int) as isize) = (*((*s)
                                            .ll4)
                                            .offset((j >> 1 as core::ffi::c_int) as isize) as Int32
                                            & 0xf0 as Int32 | i >> 16 as core::ffi::c_int) as UChar;
                                    } else {
                                        *((*s).ll4).offset((j >> 1 as core::ffi::c_int) as isize) = (*((*s)
                                            .ll4)
                                            .offset((j >> 1 as core::ffi::c_int) as isize) as Int32
                                            & 0xf as Int32
                                            | (i >> 16 as core::ffi::c_int) << 4 as core::ffi::c_int)
                                            as UChar;
                                    }
                                    i = j;
                                    j = tmp_0;
                                    if !(i != (*s).origPtr) {
                                        break;
                                    }
                                }
                                (*s).tPos = (*s).origPtr as UInt32;
                                (*s).nblock_used = 0 as core::ffi::c_int as Int32;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo = 0 as core::ffi::c_int as Int32;
                                    (*s).rTPos = 0 as core::ffi::c_int as Int32;
                                    if (*s).tPos
                                        >= (100000 as core::ffi::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as core::ffi::c_int as Bool as Int32;
                                    }
                                    (*s).k0 = BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        ((*s).cftab).as_mut_ptr(),
                                    );
                                    (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize)
                                        as core::ffi::c_uint
                                        | (*((*s).ll4)
                                            .offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                                            as core::ffi::c_uint
                                            >> (((*s).tPos as core::ffi::c_uint)
                                                << 2 as core::ffi::c_int & 0x4 as core::ffi::c_uint)
                                            & 0xf as core::ffi::c_uint) << 16 as core::ffi::c_int)
                                        as UInt32;
                                    (*s).nblock_used += 1;
                                    if (*s).rNToGo == 0 as core::ffi::c_int {
                                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                        (*s).rTPos += 1;
                                        if (*s).rTPos == 512 as core::ffi::c_int {
                                            (*s).rTPos = 0 as core::ffi::c_int as Int32;
                                        }
                                    }
                                    (*s).rNToGo -= 1;
                                    (*s).k0
                                        ^= if (*s).rNToGo == 1 as core::ffi::c_int {
                                            1 as core::ffi::c_int
                                        } else {
                                            0 as core::ffi::c_int
                                        };
                                } else {
                                    if (*s).tPos
                                        >= (100000 as core::ffi::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as core::ffi::c_int as Bool as Int32;
                                    }
                                    (*s).k0 = BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        ((*s).cftab).as_mut_ptr(),
                                    );
                                    (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize)
                                        as core::ffi::c_uint
                                        | (*((*s).ll4)
                                            .offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                                            as core::ffi::c_uint
                                            >> (((*s).tPos as core::ffi::c_uint)
                                                << 2 as core::ffi::c_int & 0x4 as core::ffi::c_uint)
                                            & 0xf as core::ffi::c_uint) << 16 as core::ffi::c_int)
                                        as UInt32;
                                    (*s).nblock_used += 1;
                                }
                            } else {
                                i = 0 as core::ffi::c_int as Int32;
                                while i < nblock {
                                    uc = (*((*s).tt).offset(i as isize)
                                        & 0xff as core::ffi::c_uint) as UChar;
                                    let ref mut fresh0 = *((*s).tt)
                                        .offset((*s).cftab[uc as usize] as isize);
                                    *fresh0
                                        |= (i << 8 as core::ffi::c_int) as core::ffi::c_uint;
                                    (*s).cftab[uc as usize] += 1;
                                    i += 1;
                                }
                                (*s).tPos = *((*s).tt).offset((*s).origPtr as isize)
                                    >> 8 as core::ffi::c_int;
                                (*s).nblock_used = 0 as core::ffi::c_int as Int32;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo = 0 as core::ffi::c_int as Int32;
                                    (*s).rTPos = 0 as core::ffi::c_int as Int32;
                                    if (*s).tPos
                                        >= (100000 as core::ffi::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as core::ffi::c_int as Bool as Int32;
                                    }
                                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                    (*s).k0 = ((*s).tPos as core::ffi::c_uint
                                        & 0xff as core::ffi::c_uint) as UChar as Int32;
                                    (*s).tPos >>= 8 as core::ffi::c_int;
                                    (*s).nblock_used += 1;
                                    if (*s).rNToGo == 0 as core::ffi::c_int {
                                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                        (*s).rTPos += 1;
                                        if (*s).rTPos == 512 as core::ffi::c_int {
                                            (*s).rTPos = 0 as core::ffi::c_int as Int32;
                                        }
                                    }
                                    (*s).rNToGo -= 1;
                                    (*s).k0
                                        ^= if (*s).rNToGo == 1 as core::ffi::c_int {
                                            1 as core::ffi::c_int
                                        } else {
                                            0 as core::ffi::c_int
                                        };
                                } else {
                                    if (*s).tPos
                                        >= (100000 as core::ffi::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as core::ffi::c_int as Bool as Int32;
                                    }
                                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                    (*s).k0 = ((*s).tPos as core::ffi::c_uint
                                        & 0xff as core::ffi::c_uint) as UChar as Int32;
                                    (*s).tPos >>= 8 as core::ffi::c_int;
                                    (*s).nblock_used += 1;
                                }
                            }
                            retVal = 0 as core::ffi::c_int as Int32;
                            current_block = 10837989821191059030;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            5649595406143318745 => {
                if N
                    >= 2 as core::ffi::c_int * 1024 as core::ffi::c_int
                        * 1024 as core::ffi::c_int
                {
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue;
                } else {
                    if nextSym == 0 as core::ffi::c_int {
                        es = es + (0 as Int32 + 1 as Int32) * N;
                    } else if nextSym == 1 as core::ffi::c_int {
                        es = es + (1 as Int32 + 1 as Int32) * N;
                    }
                    N = (N as core::ffi::c_int * 2 as core::ffi::c_int) as Int32;
                    if groupPos == 0 as core::ffi::c_int {
                        groupNo += 1;
                        if groupNo >= nSelectors {
                            retVal = -(4 as core::ffi::c_int) as Int32;
                            current_block = 10837989821191059030;
                            continue;
                        } else {
                            groupPos = 50 as core::ffi::c_int as Int32;
                            gSel = (*s).selector[groupNo as usize] as Int32;
                            gMinlen = (*s).minLens[gSel as usize];
                            gLimit = &mut *(*((*s).limit)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                            gPerm = &mut *(*((*s).perm)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                            gBase = &mut *(*((*s).base)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                        }
                    }
                    groupPos -= 1;
                    zn = gMinlen;
                    current_block = 8517808508854580601;
                    continue;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                16953886395775657100 => {
                    if j < 16 as core::ffi::c_int {
                        current_block = 16687080193289705592;
                        continue 'c_9709;
                    }
                }
                3503188808869013853 => {
                    if i < nSelectors {
                        j = 0 as core::ffi::c_int as Int32;
                        current_block = 16531797892856733396;
                        continue;
                    } else {
                        if nSelectors
                            > 2 as core::ffi::c_int
                                + 900000 as core::ffi::c_int / 50 as core::ffi::c_int
                        {
                            nSelectors = (2 as core::ffi::c_int
                                + 900000 as core::ffi::c_int / 50 as core::ffi::c_int)
                                as Int32;
                        }
                        let mut pos: [UChar; 6] = [0; 6];
                        let mut tmp: UChar = 0;
                        let mut v_22: UChar = 0;
                        v_22 = 0 as UChar;
                        while (v_22 as core::ffi::c_int) < nGroups {
                            pos[v_22 as usize] = v_22;
                            v_22 = v_22.wrapping_add(1);
                        }
                        i = 0 as core::ffi::c_int as Int32;
                        while i < nSelectors {
                            v_22 = (*s).selectorMtf[i as usize];
                            tmp = pos[v_22 as usize];
                            while v_22 as core::ffi::c_int > 0 as core::ffi::c_int {
                                pos[v_22 as usize] = pos[(v_22 as core::ffi::c_int
                                    - 1 as core::ffi::c_int) as usize];
                                v_22 = v_22.wrapping_sub(1);
                            }
                            pos[0 as core::ffi::c_int as usize] = tmp;
                            (*s).selector[i as usize] = tmp;
                            i += 1;
                        }
                        t = 0 as core::ffi::c_int as Int32;
                        current_block = 2488856075421756534;
                        break;
                    }
                }
                15415362524153386998 => {
                    if i < 16 as core::ffi::c_int {
                        if (*s).inUse16[i as usize] != 0 {
                            j = 0 as core::ffi::c_int as Int32;
                            current_block = 16953886395775657100;
                            continue;
                        }
                    } else {
                        makeMaps_d(s);
                        if (*s).nInUse == 0 as core::ffi::c_int {
                            current_block = 12571193857528100212;
                            break;
                        } else {
                            current_block = 9416928054198617439;
                            break;
                        }
                    }
                }
                7746242308555130918 => {
                    (*s).len[t as usize][i as usize] = curr as UChar;
                    i += 1;
                    current_block = 16642413284942005565;
                    continue;
                }
                16642413284942005565 => {
                    if i < alphaSize {
                        current_block = 5533056661327372531;
                        continue;
                    }
                    t += 1;
                    current_block = 2488856075421756534;
                    break;
                }
                10081471997089450706 => {
                    if i
                        < 2 as core::ffi::c_int
                            + 900000 as core::ffi::c_int / 50 as core::ffi::c_int
                    {
                        (*s).selectorMtf[i as usize] = j as UChar;
                    }
                    i += 1;
                    current_block = 3503188808869013853;
                    continue;
                }
                16531797892856733396 => {
                    if 1 as core::ffi::c_int as Bool != 0 {
                        current_block = 3307780312744082722;
                        continue 'c_9709;
                    } else {
                        current_block = 10081471997089450706;
                        continue;
                    }
                }
                _ => {
                    if !(1 as core::ffi::c_int as Bool != 0) {
                        current_block = 7746242308555130918;
                        continue;
                    }
                    if !(curr < 1 as core::ffi::c_int || curr > 20 as core::ffi::c_int) {
                        current_block = 12137889358298489534;
                        continue 'c_9709;
                    }
                    retVal = -(4 as core::ffi::c_int) as Int32;
                    current_block = 10837989821191059030;
                    continue 'c_9709;
                }
            }
            i += 1;
            current_block = 15415362524153386998;
        }
        match current_block {
            9416928054198617439 => {
                alphaSize = ((*s).nInUse as core::ffi::c_int + 2 as core::ffi::c_int)
                    as Int32;
                current_block = 15439061234641355781;
            }
            12571193857528100212 => {
                retVal = -(4 as core::ffi::c_int) as Int32;
                current_block = 10837989821191059030;
            }
            _ => {
                if t < nGroups {
                    current_block = 3122919031762406036;
                    continue;
                }
                t = 0 as core::ffi::c_int as Int32;
                while t < nGroups {
                    minLen = 32 as core::ffi::c_int as Int32;
                    maxLen = 0 as core::ffi::c_int as Int32;
                    i = 0 as core::ffi::c_int as Int32;
                    while i < alphaSize {
                        if (*s).len[t as usize][i as usize] as core::ffi::c_int > maxLen
                        {
                            maxLen = (*s).len[t as usize][i as usize] as Int32;
                        }
                        if ((*s).len[t as usize][i as usize] as core::ffi::c_int)
                            < minLen
                        {
                            minLen = (*s).len[t as usize][i as usize] as Int32;
                        }
                        i += 1;
                    }
                    BZ2_hbCreateDecodeTables(
                        &mut *(*((*s).limit).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *(*((*s).base).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *(*((*s).perm).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        minLen,
                        maxLen,
                        alphaSize,
                    );
                    (*s).minLens[t as usize] = minLen;
                    t += 1;
                }
                EOB = ((*s).nInUse as core::ffi::c_int + 1 as core::ffi::c_int) as Int32;
                nblockMAX = 100000 as Int32 * (*s).blockSize100k;
                groupNo = -(1 as core::ffi::c_int) as Int32;
                groupPos = 0 as core::ffi::c_int as Int32;
                i = 0 as core::ffi::c_int as Int32;
                while i <= 255 as core::ffi::c_int {
                    (*s).unzftab[i as usize] = 0 as core::ffi::c_int as Int32;
                    i += 1;
                }
                let mut ii: Int32 = 0;
                let mut jj: Int32 = 0;
                let mut kk: Int32 = 0;
                kk = (4096 as core::ffi::c_int - 1 as core::ffi::c_int) as Int32;
                ii = (256 as core::ffi::c_int / 16 as core::ffi::c_int
                    - 1 as core::ffi::c_int) as Int32;
                while ii >= 0 as core::ffi::c_int {
                    jj = (16 as core::ffi::c_int - 1 as core::ffi::c_int) as Int32;
                    while jj >= 0 as core::ffi::c_int {
                        (*s).mtfa[kk as usize] = (ii * 16 as Int32 + jj) as UChar;
                        kk -= 1;
                        jj -= 1;
                    }
                    (*s).mtfbase[ii as usize] = (kk as core::ffi::c_int
                        + 1 as core::ffi::c_int) as Int32;
                    ii -= 1;
                }
                nblock = 0 as core::ffi::c_int as Int32;
                if groupPos == 0 as core::ffi::c_int {
                    groupNo += 1;
                    if groupNo >= nSelectors {
                        retVal = -(4 as core::ffi::c_int) as Int32;
                        current_block = 10837989821191059030;
                        continue;
                    } else {
                        groupPos = 50 as core::ffi::c_int as Int32;
                        gSel = (*s).selector[groupNo as usize] as Int32;
                        gMinlen = (*s).minLens[gSel as usize];
                        gLimit = &mut *(*((*s).limit).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                        gPerm = &mut *(*((*s).perm).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                        gBase = &mut *(*((*s).base).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut Int32;
                    }
                }
                groupPos -= 1;
                zn = gMinlen;
                current_block = 10010940406495358186;
            }
        }
    }
    (*s).save_j = j;
    (*s).save_t = t;
    (*s).save_alphaSize = alphaSize;
    (*s).save_nGroups = nGroups;
    (*s).save_nSelectors = nSelectors;
    (*s).save_EOB = EOB;
    (*s).save_groupNo = groupNo;
    (*s).save_groupPos = groupPos;
    (*s).save_nextSym = nextSym;
    (*s).save_nblockMAX = nblockMAX;
    (*s).save_nblock = nblock;
    (*s).save_es = es;
    (*s).save_N = N;
    (*s).save_curr = curr;
    (*s).save_zt = zt;
    (*s).save_zn = zn;
    (*s).save_zvec = zvec;
    (*s).save_zj = zj;
    (*s).save_gSel = gSel;
    (*s).save_gMinlen = gMinlen;
    (*s).save_gLimit = gLimit;
    (*s).save_gBase = gBase;
    (*s).save_gPerm = gPerm;
    return retVal;
}
