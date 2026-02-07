#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
    static mut BZ2_rNums: [Int32; 512];
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn strcat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    static mut BZ2_crc32Table: [UInt32; 256];
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn BZ2_compressBlock(_: *mut EState, _: Bool);
    fn fdopen(__fd: core::ffi::c_int, __modes: *const core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn BZ2_decompress(_: *mut DState) -> Int32;
    fn fgetc(__stream: *mut FILE) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
    fn ungetc(__c: core::ffi::c_int, __stream: *mut FILE) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type UInt32 = core::ffi::c_uint;
pub type Int32 = core::ffi::c_int;
pub type UChar = core::ffi::c_uchar;
pub type Bool = core::ffi::c_uchar;
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [Char; 5000],
    pub bufN: Int32,
    pub writing: Bool,
    pub strm: bz_stream,
    pub lastErr: Int32,
    pub initialisedOk: Bool,
}
pub type Char = core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn BZ2_bz__AssertH__fail(mut errcode: core::ffi::c_int) {
    fprintf(
        stderr,
        b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
        errcode,
        BZ2_bzlibVersion(),
    );
    if errcode == 1007 as core::ffi::c_int {
        fprintf(
            stderr,
            b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn't.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    exit(3 as core::ffi::c_int);
}
unsafe extern "C" fn bz_config_ok() -> core::ffi::c_int {
    if ::core::mem::size_of::<core::ffi::c_int>() as usize != 4 as usize {
        return 0 as core::ffi::c_int;
    }
    if ::core::mem::size_of::<core::ffi::c_short>() as usize != 2 as usize {
        return 0 as core::ffi::c_int;
    }
    if ::core::mem::size_of::<core::ffi::c_char>() as usize != 1 as usize {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn default_bzalloc(
    mut opaque: *mut core::ffi::c_void,
    mut items: Int32,
    mut size: Int32,
) -> *mut core::ffi::c_void {
    let mut v: *mut core::ffi::c_void = malloc((items * size) as size_t);
    return v;
}
unsafe extern "C" fn default_bzfree(
    mut opaque: *mut core::ffi::c_void,
    mut addr: *mut core::ffi::c_void,
) {
    if !addr.is_null() {
        free(addr);
    }
}
unsafe extern "C" fn prepare_new_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nblock = 0 as core::ffi::c_int as Int32;
    (*s).numZ = 0 as core::ffi::c_int as Int32;
    (*s).state_out_pos = 0 as core::ffi::c_int as Int32;
    (*s).blockCRC = 0xffffffff as UInt32;
    i = 0 as core::ffi::c_int as Int32;
    while i < 256 as core::ffi::c_int {
        (*s).inUse[i as usize] = 0 as core::ffi::c_int as Bool;
        i += 1;
    }
    (*s).blockNo += 1;
}
unsafe extern "C" fn init_RL(mut s: *mut EState) {
    (*s).state_in_ch = 256 as UInt32;
    (*s).state_in_len = 0 as core::ffi::c_int as Int32;
}
unsafe extern "C" fn isempty_RL(mut s: *mut EState) -> Bool {
    if (*s).state_in_ch < 256 as core::ffi::c_uint
        && (*s).state_in_len > 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_int as Bool
    } else {
        return 1 as core::ffi::c_int as Bool
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressInit(
    mut strm: *mut bz_stream,
    mut blockSize100k: core::ffi::c_int,
    mut verbosity: core::ffi::c_int,
    mut workFactor: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut n: Int32 = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if bz_config_ok() == 0 {
        return -(9 as core::ffi::c_int);
    }
    if strm.is_null() || blockSize100k < 1 as core::ffi::c_int
        || blockSize100k > 9 as core::ffi::c_int || workFactor < 0 as core::ffi::c_int
        || workFactor > 250 as core::ffi::c_int
    {
        return -(2 as core::ffi::c_int);
    }
    if workFactor == 0 as core::ffi::c_int {
        workFactor = 30 as core::ffi::c_int;
    }
    if ((*strm).bzalloc).is_none() {
        (*strm).bzalloc = Some(
            default_bzalloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    Int32,
                    Int32,
                ) -> *mut core::ffi::c_void,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> *mut core::ffi::c_void,
            >;
    }
    if ((*strm).bzfree).is_none() {
        (*strm).bzfree = Some(
            default_bzfree
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> (),
            >;
    }
    s = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ::core::mem::size_of::<EState>() as core::ffi::c_int,
        1 as core::ffi::c_int,
    ) as *mut EState;
    if s.is_null() {
        return -(3 as core::ffi::c_int);
    }
    (*s).strm = strm;
    (*s).arr1 = 0 as *mut UInt32;
    (*s).arr2 = 0 as *mut UInt32;
    (*s).ftab = 0 as *mut UInt32;
    n = (100000 as core::ffi::c_int * blockSize100k) as Int32;
    (*s).arr1 = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (n as usize).wrapping_mul(::core::mem::size_of::<UInt32>() as usize)
            as core::ffi::c_int,
        1 as core::ffi::c_int,
    ) as *mut UInt32;
    (*s).arr2 = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ((n as core::ffi::c_int
            + (2 as core::ffi::c_int + 12 as core::ffi::c_int + 18 as core::ffi::c_int
                + 2 as core::ffi::c_int)) as usize)
            .wrapping_mul(::core::mem::size_of::<UInt32>() as usize) as core::ffi::c_int,
        1 as core::ffi::c_int,
    ) as *mut UInt32;
    (*s).ftab = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (65537 as usize).wrapping_mul(::core::mem::size_of::<UInt32>() as usize)
            as core::ffi::c_int,
        1 as core::ffi::c_int,
    ) as *mut UInt32;
    if ((*s).arr1).is_null() || ((*s).arr2).is_null() || ((*s).ftab).is_null() {
        if !((*s).arr1).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).arr1 as *mut core::ffi::c_void);
        }
        if !((*s).arr2).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).arr2 as *mut core::ffi::c_void);
        }
        if !((*s).ftab).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).ftab as *mut core::ffi::c_void);
        }
        if !s.is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, s as *mut core::ffi::c_void);
        }
        return -(3 as core::ffi::c_int);
    }
    (*s).blockNo = 0 as core::ffi::c_int as Int32;
    (*s).state = 2 as core::ffi::c_int as Int32;
    (*s).mode = 2 as core::ffi::c_int as Int32;
    (*s).combinedCRC = 0 as UInt32;
    (*s).blockSize100k = blockSize100k as Int32;
    (*s).nblockMAX = (100000 as core::ffi::c_int * blockSize100k
        - 19 as core::ffi::c_int) as Int32;
    (*s).verbosity = verbosity as Int32;
    (*s).workFactor = workFactor as Int32;
    (*s).block = (*s).arr2 as *mut UChar;
    (*s).mtfv = (*s).arr1 as *mut UInt16;
    (*s).zbits = 0 as *mut UChar;
    (*s).ptr = (*s).arr1;
    (*strm).state = s as *mut core::ffi::c_void;
    (*strm).total_in_lo32 = 0 as core::ffi::c_uint;
    (*strm).total_in_hi32 = 0 as core::ffi::c_uint;
    (*strm).total_out_lo32 = 0 as core::ffi::c_uint;
    (*strm).total_out_hi32 = 0 as core::ffi::c_uint;
    init_RL(s);
    prepare_new_block(s);
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn add_pair_to_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    let mut ch: UChar = (*s).state_in_ch as UChar;
    i = 0 as core::ffi::c_int as Int32;
    while i < (*s).state_in_len {
        (*s).blockCRC = (*s).blockCRC << 8 as core::ffi::c_int
            ^ BZ2_crc32Table[((*s).blockCRC as core::ffi::c_uint
                >> 24 as core::ffi::c_int ^ ch as core::ffi::c_uint) as usize];
        i += 1;
    }
    (*s).inUse[(*s).state_in_ch as usize] = 1 as core::ffi::c_int as Bool;
    match (*s).state_in_len {
        1 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
        }
        2 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
        }
        3 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
        }
        _ => {
            (*s)
                .inUse[((*s).state_in_len as core::ffi::c_int - 4 as core::ffi::c_int)
                as usize] = 1 as core::ffi::c_int as Bool;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *((*s).block).offset((*s).nblock as isize) = ((*s).state_in_len
                as core::ffi::c_int - 4 as core::ffi::c_int) as UChar;
            (*s).nblock += 1;
        }
    };
}
unsafe extern "C" fn flush_RL(mut s: *mut EState) {
    if (*s).state_in_ch < 256 as core::ffi::c_uint {
        add_pair_to_block(s);
    }
    init_RL(s);
}
#[inline]
unsafe extern "C" fn ADD_CHAR_TO_BLOCK(mut zs: *mut EState, mut zchh0: UInt32) {
    let mut zchh: UInt32 = zchh0;
    if zchh != (*zs).state_in_ch && (*zs).state_in_len == 1 as core::ffi::c_int {
        let mut ch: UChar = (*zs).state_in_ch as UChar;
        (*zs).blockCRC = (*zs).blockCRC << 8 as core::ffi::c_int
            ^ BZ2_crc32Table[((*zs).blockCRC as core::ffi::c_uint
                >> 24 as core::ffi::c_int ^ ch as core::ffi::c_uint) as usize];
        (*zs).inUse[(*zs).state_in_ch as usize] = 1 as core::ffi::c_int as Bool;
        *((*zs).block).offset((*zs).nblock as isize) = ch;
        (*zs).nblock += 1;
        (*zs).state_in_ch = zchh;
    } else if zchh != (*zs).state_in_ch || (*zs).state_in_len == 255 as core::ffi::c_int
    {
        if (*zs).state_in_ch < 256 as core::ffi::c_uint {
            add_pair_to_block(zs);
        }
        (*zs).state_in_ch = zchh;
        (*zs).state_in_len = 1 as core::ffi::c_int as Int32;
    } else {
        (*zs).state_in_len += 1;
    };
}
unsafe extern "C" fn copy_input_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_in: Bool = 0 as core::ffi::c_int as Bool;
    if (*s).mode == 2 as core::ffi::c_int {
        while 1 as core::ffi::c_int as Bool != 0 {
            if (*s).nblock >= (*s).nblockMAX {
                break;
            }
            if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                break;
            }
            progress_in = 1 as core::ffi::c_int as Bool;
            ADD_CHAR_TO_BLOCK(s, *((*(*s).strm).next_in as *mut UChar) as UInt32);
            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
            (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                    .wrapping_add(1);
            }
        }
    } else {
        while 1 as core::ffi::c_int as Bool != 0 {
            if (*s).nblock >= (*s).nblockMAX {
                break;
            }
            if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
                break;
            }
            if (*s).avail_in_expect == 0 as core::ffi::c_uint {
                break;
            }
            progress_in = 1 as core::ffi::c_int as Bool;
            ADD_CHAR_TO_BLOCK(s, *((*(*s).strm).next_in as *mut UChar) as UInt32);
            (*(*s).strm).next_in = ((*(*s).strm).next_in).offset(1);
            (*(*s).strm).avail_in = ((*(*s).strm).avail_in).wrapping_sub(1);
            (*(*s).strm).total_in_lo32 = ((*(*s).strm).total_in_lo32).wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as core::ffi::c_uint {
                (*(*s).strm).total_in_hi32 = ((*(*s).strm).total_in_hi32)
                    .wrapping_add(1);
            }
            (*s).avail_in_expect = ((*s).avail_in_expect).wrapping_sub(1);
        }
    }
    return progress_in;
}
unsafe extern "C" fn copy_output_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_out: Bool = 0 as core::ffi::c_int as Bool;
    while 1 as core::ffi::c_int as Bool != 0 {
        if (*(*s).strm).avail_out == 0 as core::ffi::c_uint {
            break;
        }
        if (*s).state_out_pos >= (*s).numZ {
            break;
        }
        progress_out = 1 as core::ffi::c_int as Bool;
        *(*(*s).strm).next_out = *((*s).zbits).offset((*s).state_out_pos as isize)
            as core::ffi::c_char;
        (*s).state_out_pos += 1;
        (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
        (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
        (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32).wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 as core::ffi::c_uint {
            (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
        }
    }
    return progress_out;
}
unsafe extern "C" fn handle_compress(mut strm: *mut bz_stream) -> Bool {
    let mut progress_in: Bool = 0 as core::ffi::c_int as Bool;
    let mut progress_out: Bool = 0 as core::ffi::c_int as Bool;
    let mut s: *mut EState = (*strm).state as *mut EState;
    while 1 as core::ffi::c_int as Bool != 0 {
        if (*s).state == 1 as core::ffi::c_int {
            progress_out = (progress_out as core::ffi::c_int
                | copy_output_until_stop(s) as core::ffi::c_int) as Bool;
            if (*s).state_out_pos < (*s).numZ {
                break;
            }
            if (*s).mode == 4 as core::ffi::c_int
                && (*s).avail_in_expect == 0 as core::ffi::c_uint
                && isempty_RL(s) as core::ffi::c_int != 0
            {
                break;
            }
            prepare_new_block(s);
            (*s).state = 2 as core::ffi::c_int as Int32;
            if (*s).mode == 3 as core::ffi::c_int
                && (*s).avail_in_expect == 0 as core::ffi::c_uint
                && isempty_RL(s) as core::ffi::c_int != 0
            {
                break;
            }
        }
        if !((*s).state == 2 as core::ffi::c_int) {
            continue;
        }
        progress_in = (progress_in as core::ffi::c_int
            | copy_input_until_stop(s) as core::ffi::c_int) as Bool;
        if (*s).mode != 2 as core::ffi::c_int
            && (*s).avail_in_expect == 0 as core::ffi::c_uint
        {
            flush_RL(s);
            BZ2_compressBlock(
                s,
                ((*s).mode == 4 as core::ffi::c_int) as core::ffi::c_int as Bool,
            );
            (*s).state = 1 as core::ffi::c_int as Int32;
        } else if (*s).nblock >= (*s).nblockMAX {
            BZ2_compressBlock(s, 0 as core::ffi::c_int as Bool);
            (*s).state = 1 as core::ffi::c_int as Int32;
        } else if (*(*s).strm).avail_in == 0 as core::ffi::c_uint {
            break;
        }
    }
    return (progress_in as core::ffi::c_int != 0
        || progress_out as core::ffi::c_int != 0) as core::ffi::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompress(
    mut strm: *mut bz_stream,
    mut action: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut progress: Bool = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() {
        return -(2 as core::ffi::c_int);
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -(2 as core::ffi::c_int);
    }
    if (*s).strm != strm {
        return -(2 as core::ffi::c_int);
    }
    loop {
        match (*s).mode {
            1 => return -(1 as core::ffi::c_int),
            2 => {
                if action == 0 as core::ffi::c_int {
                    progress = handle_compress(strm);
                    return if progress as core::ffi::c_int != 0 {
                        1 as core::ffi::c_int
                    } else {
                        -(2 as core::ffi::c_int)
                    };
                } else if action == 1 as core::ffi::c_int {
                    (*s).avail_in_expect = (*strm).avail_in as UInt32;
                    (*s).mode = 3 as core::ffi::c_int as Int32;
                } else if action == 2 as core::ffi::c_int {
                    (*s).avail_in_expect = (*strm).avail_in as UInt32;
                    (*s).mode = 4 as core::ffi::c_int as Int32;
                } else {
                    return -(2 as core::ffi::c_int)
                }
            }
            3 => {
                if action != 1 as core::ffi::c_int {
                    return -(1 as core::ffi::c_int);
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as core::ffi::c_int);
                }
                progress = handle_compress(strm);
                if (*s).avail_in_expect > 0 as core::ffi::c_uint || isempty_RL(s) == 0
                    || (*s).state_out_pos < (*s).numZ
                {
                    return 2 as core::ffi::c_int;
                }
                (*s).mode = 2 as core::ffi::c_int as Int32;
                return 1 as core::ffi::c_int;
            }
            4 => {
                if action != 2 as core::ffi::c_int {
                    return -(1 as core::ffi::c_int);
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as core::ffi::c_int);
                }
                progress = handle_compress(strm);
                if progress == 0 {
                    return -(1 as core::ffi::c_int);
                }
                if (*s).avail_in_expect > 0 as core::ffi::c_uint || isempty_RL(s) == 0
                    || (*s).state_out_pos < (*s).numZ
                {
                    return 3 as core::ffi::c_int;
                }
                (*s).mode = 1 as core::ffi::c_int as Int32;
                return 4 as core::ffi::c_int;
            }
            _ => return 0 as core::ffi::c_int,
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressEnd(
    mut strm: *mut bz_stream,
) -> core::ffi::c_int {
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() {
        return -(2 as core::ffi::c_int);
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -(2 as core::ffi::c_int);
    }
    if (*s).strm != strm {
        return -(2 as core::ffi::c_int);
    }
    if !((*s).arr1).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).arr1 as *mut core::ffi::c_void);
    }
    if !((*s).arr2).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).arr2 as *mut core::ffi::c_void);
    }
    if !((*s).ftab).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ftab as *mut core::ffi::c_void);
    }
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    (*strm).state = 0 as *mut core::ffi::c_void;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressInit(
    mut strm: *mut bz_stream,
    mut verbosity: core::ffi::c_int,
    mut small: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if bz_config_ok() == 0 {
        return -(9 as core::ffi::c_int);
    }
    if strm.is_null() {
        return -(2 as core::ffi::c_int);
    }
    if small != 0 as core::ffi::c_int && small != 1 as core::ffi::c_int {
        return -(2 as core::ffi::c_int);
    }
    if verbosity < 0 as core::ffi::c_int || verbosity > 4 as core::ffi::c_int {
        return -(2 as core::ffi::c_int);
    }
    if ((*strm).bzalloc).is_none() {
        (*strm).bzalloc = Some(
            default_bzalloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    Int32,
                    Int32,
                ) -> *mut core::ffi::c_void,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> *mut core::ffi::c_void,
            >;
    }
    if ((*strm).bzfree).is_none() {
        (*strm).bzfree = Some(
            default_bzfree
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> (),
            >;
    }
    s = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ::core::mem::size_of::<DState>() as core::ffi::c_int,
        1 as core::ffi::c_int,
    ) as *mut DState;
    if s.is_null() {
        return -(3 as core::ffi::c_int);
    }
    (*s).strm = strm;
    (*strm).state = s as *mut core::ffi::c_void;
    (*s).state = 10 as core::ffi::c_int as Int32;
    (*s).bsLive = 0 as core::ffi::c_int as Int32;
    (*s).bsBuff = 0 as UInt32;
    (*s).calculatedCombinedCRC = 0 as UInt32;
    (*strm).total_in_lo32 = 0 as core::ffi::c_uint;
    (*strm).total_in_hi32 = 0 as core::ffi::c_uint;
    (*strm).total_out_lo32 = 0 as core::ffi::c_uint;
    (*strm).total_out_hi32 = 0 as core::ffi::c_uint;
    (*s).smallDecompress = small as Bool;
    (*s).ll4 = 0 as *mut UChar;
    (*s).ll16 = 0 as *mut UInt16;
    (*s).tt = 0 as *mut UInt32;
    (*s).currBlockNo = 0 as core::ffi::c_int as Int32;
    (*s).verbosity = verbosity as Int32;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn unRLE_obuf_to_output_FAST(mut s: *mut DState) -> Bool {
    let mut current_block: u64;
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as core::ffi::c_int as Bool != 0 {
            while 1 as core::ffi::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as core::ffi::c_uint {
                    return 0 as core::ffi::c_int as Bool;
                }
                if (*s).state_out_len == 0 as core::ffi::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC = (*s).calculatedBlockCRC
                    << 8 as core::ffi::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC as core::ffi::c_uint
                        >> 24 as core::ffi::c_int
                        ^ (*s).state_out_ch as core::ffi::c_uint) as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32)
                    .wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as core::ffi::c_uint {
                    (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32)
                        .wrapping_add(1);
                }
            }
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 0 as core::ffi::c_int as Bool;
            }
            if (*s).nblock_used
                > (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 1 as core::ffi::c_int as Bool;
            }
            (*s).state_out_len = 1 as core::ffi::c_int as Int32;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as core::ffi::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as core::ffi::c_int as Bool;
            }
            (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
            k1 = ((*s).tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint) as UChar;
            (*s).tPos >>= 8 as core::ffi::c_int;
            if (*s).rNToGo == 0 as core::ffi::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as core::ffi::c_int {
                    (*s).rTPos = 0 as core::ffi::c_int as Int32;
                }
            }
            (*s).rNToGo -= 1;
            k1 = (k1 as core::ffi::c_int
                ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                    1 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                continue;
            }
            if k1 as core::ffi::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as core::ffi::c_int as Int32;
                if (*s).tPos
                    >= (100000 as core::ffi::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as core::ffi::c_int as Bool;
                }
                (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                k1 = ((*s).tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                    as UChar;
                (*s).tPos >>= 8 as core::ffi::c_int;
                if (*s).rNToGo == 0 as core::ffi::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as core::ffi::c_int {
                        (*s).rTPos = 0 as core::ffi::c_int as Int32;
                    }
                }
                (*s).rNToGo -= 1;
                k1 = (k1 as core::ffi::c_int
                    ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used
                    == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                {
                    continue;
                }
                if k1 as core::ffi::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as core::ffi::c_int as Int32;
                    if (*s).tPos
                        >= (100000 as core::ffi::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as core::ffi::c_int as Bool;
                    }
                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                    k1 = ((*s).tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                        as UChar;
                    (*s).tPos >>= 8 as core::ffi::c_int;
                    if (*s).rNToGo == 0 as core::ffi::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as core::ffi::c_int {
                            (*s).rTPos = 0 as core::ffi::c_int as Int32;
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 = (k1 as core::ffi::c_int
                        ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                            1 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used
                        == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                    {
                        continue;
                    }
                    if k1 as core::ffi::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                        k1 = ((*s).tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                            as UChar;
                        (*s).tPos >>= 8 as core::ffi::c_int;
                        if (*s).rNToGo == 0 as core::ffi::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as core::ffi::c_int {
                                (*s).rTPos = 0 as core::ffi::c_int as Int32;
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 = (k1 as core::ffi::c_int
                            ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = (k1 as core::ffi::c_int
                            + 4 as core::ffi::c_int) as Int32;
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                        (*s).k0 = ((*s).tPos as core::ffi::c_uint
                            & 0xff as core::ffi::c_uint) as UChar as Int32;
                        (*s).tPos >>= 8 as core::ffi::c_int;
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
                        (*s).nblock_used += 1;
                    }
                }
            }
        }
    } else {
        let mut c_calculatedBlockCRC: UInt32 = (*s).calculatedBlockCRC;
        let mut c_state_out_ch: UChar = (*s).state_out_ch;
        let mut c_state_out_len: Int32 = (*s).state_out_len;
        let mut c_nblock_used: Int32 = (*s).nblock_used;
        let mut c_k0: Int32 = (*s).k0;
        let mut c_tt: *mut UInt32 = (*s).tt;
        let mut c_tPos: UInt32 = (*s).tPos;
        let mut cs_next_out: *mut core::ffi::c_char = (*(*s).strm).next_out;
        let mut cs_avail_out: core::ffi::c_uint = (*(*s).strm).avail_out;
        let mut ro_blockSize100k: Int32 = (*s).blockSize100k;
        let mut avail_out_INIT: UInt32 = cs_avail_out as UInt32;
        let mut s_save_nblockPP: Int32 = (*s).save_nblock + 1 as Int32;
        let mut total_out_lo32_old: core::ffi::c_uint = 0;
        's_453: while 1 as core::ffi::c_int as Bool != 0 {
            if c_state_out_len > 0 as core::ffi::c_int {
                while 1 as core::ffi::c_int as Bool != 0 {
                    if cs_avail_out == 0 as core::ffi::c_uint {
                        break 's_453;
                    }
                    if c_state_out_len == 1 as core::ffi::c_int {
                        break;
                    }
                    *(cs_next_out as *mut UChar) = c_state_out_ch;
                    c_calculatedBlockCRC = c_calculatedBlockCRC << 8 as core::ffi::c_int
                        ^ BZ2_crc32Table[(c_calculatedBlockCRC as core::ffi::c_uint
                            >> 24 as core::ffi::c_int
                            ^ c_state_out_ch as core::ffi::c_uint) as usize];
                    c_state_out_len -= 1;
                    cs_next_out = cs_next_out.offset(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
                current_block = 1417769144978639029;
            } else {
                current_block = 14483658890531361756;
            }
            loop {
                match current_block {
                    1417769144978639029 => {
                        if cs_avail_out == 0 as core::ffi::c_uint {
                            c_state_out_len = 1 as core::ffi::c_int as Int32;
                            break 's_453;
                        } else {
                            *(cs_next_out as *mut UChar) = c_state_out_ch;
                            c_calculatedBlockCRC = c_calculatedBlockCRC
                                << 8 as core::ffi::c_int
                                ^ BZ2_crc32Table[(c_calculatedBlockCRC as core::ffi::c_uint
                                    >> 24 as core::ffi::c_int
                                    ^ c_state_out_ch as core::ffi::c_uint) as usize];
                            cs_next_out = cs_next_out.offset(1);
                            cs_avail_out = cs_avail_out.wrapping_sub(1);
                            current_block = 14483658890531361756;
                        }
                    }
                    _ => {
                        if c_nblock_used > s_save_nblockPP {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        if c_nblock_used == s_save_nblockPP {
                            c_state_out_len = 0 as core::ffi::c_int as Int32;
                            break 's_453;
                        } else {
                            c_state_out_ch = c_k0 as UChar;
                            if c_tPos
                                >= (100000 as core::ffi::c_int as UInt32)
                                    .wrapping_mul(ro_blockSize100k as UInt32)
                            {
                                return 1 as core::ffi::c_int as Bool;
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            k1 = (c_tPos as core::ffi::c_uint
                                & 0xff as core::ffi::c_uint) as UChar;
                            c_tPos >>= 8 as core::ffi::c_int;
                            c_nblock_used += 1;
                            if k1 as core::ffi::c_int != c_k0 {
                                c_k0 = k1 as Int32;
                                current_block = 1417769144978639029;
                            } else {
                                if c_nblock_used == s_save_nblockPP {
                                    current_block = 1417769144978639029;
                                    continue;
                                }
                                c_state_out_len = 2 as core::ffi::c_int as Int32;
                                if c_tPos
                                    >= (100000 as core::ffi::c_int as UInt32)
                                        .wrapping_mul(ro_blockSize100k as UInt32)
                                {
                                    return 1 as core::ffi::c_int as Bool;
                                }
                                c_tPos = *c_tt.offset(c_tPos as isize);
                                k1 = (c_tPos as core::ffi::c_uint
                                    & 0xff as core::ffi::c_uint) as UChar;
                                c_tPos >>= 8 as core::ffi::c_int;
                                c_nblock_used += 1;
                                if c_nblock_used == s_save_nblockPP {
                                    continue 's_453;
                                }
                                if k1 as core::ffi::c_int != c_k0 {
                                    current_block = 6897179874198677617;
                                    break;
                                } else {
                                    current_block = 13256895345714485905;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                6897179874198677617 => {
                    c_k0 = k1 as Int32;
                }
                _ => {
                    c_state_out_len = 3 as core::ffi::c_int as Int32;
                    if c_tPos
                        >= (100000 as core::ffi::c_int as UInt32)
                            .wrapping_mul(ro_blockSize100k as UInt32)
                    {
                        return 1 as core::ffi::c_int as Bool;
                    }
                    c_tPos = *c_tt.offset(c_tPos as isize);
                    k1 = (c_tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                        as UChar;
                    c_tPos >>= 8 as core::ffi::c_int;
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockPP {
                        continue;
                    }
                    if k1 as core::ffi::c_int != c_k0 {
                        c_k0 = k1 as Int32;
                    } else {
                        if c_tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul(ro_blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        k1 = (c_tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                            as UChar;
                        c_tPos >>= 8 as core::ffi::c_int;
                        c_nblock_used += 1;
                        c_state_out_len = (k1 as core::ffi::c_int
                            + 4 as core::ffi::c_int) as Int32;
                        if c_tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul(ro_blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        c_k0 = (c_tPos as core::ffi::c_uint & 0xff as core::ffi::c_uint)
                            as UChar as Int32;
                        c_tPos >>= 8 as core::ffi::c_int;
                        c_nblock_used += 1;
                    }
                }
            }
        }
        total_out_lo32_old = (*(*s).strm).total_out_lo32;
        (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32)
            .wrapping_add(
                (avail_out_INIT as core::ffi::c_uint).wrapping_sub(cs_avail_out),
            );
        if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
            (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32).wrapping_add(1);
        }
        (*s).calculatedBlockCRC = c_calculatedBlockCRC;
        (*s).state_out_ch = c_state_out_ch;
        (*s).state_out_len = c_state_out_len;
        (*s).nblock_used = c_nblock_used;
        (*s).k0 = c_k0;
        (*s).tt = c_tt;
        (*s).tPos = c_tPos;
        (*(*s).strm).next_out = cs_next_out;
        (*(*s).strm).avail_out = cs_avail_out;
    }
    return 0 as core::ffi::c_int as Bool;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn BZ2_indexIntoF(
    mut indx: Int32,
    mut cftab: *mut Int32,
) -> Int32 {
    let mut nb: Int32 = 0;
    let mut na: Int32 = 0;
    let mut mid: Int32 = 0;
    nb = 0 as core::ffi::c_int as Int32;
    na = 256 as core::ffi::c_int as Int32;
    loop {
        mid = nb + na >> 1 as core::ffi::c_int;
        if indx >= *cftab.offset(mid as isize) {
            nb = mid;
        } else {
            na = mid;
        }
        if !(na - nb != 1 as core::ffi::c_int) {
            break;
        }
    }
    return nb;
}
unsafe extern "C" fn unRLE_obuf_to_output_SMALL(mut s: *mut DState) -> Bool {
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as core::ffi::c_int as Bool != 0 {
            while 1 as core::ffi::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as core::ffi::c_uint {
                    return 0 as core::ffi::c_int as Bool;
                }
                if (*s).state_out_len == 0 as core::ffi::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC = (*s).calculatedBlockCRC
                    << 8 as core::ffi::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC as core::ffi::c_uint
                        >> 24 as core::ffi::c_int
                        ^ (*s).state_out_ch as core::ffi::c_uint) as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32)
                    .wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as core::ffi::c_uint {
                    (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32)
                        .wrapping_add(1);
                }
            }
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 0 as core::ffi::c_int as Bool;
            }
            if (*s).nblock_used
                > (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 1 as core::ffi::c_int as Bool;
            }
            (*s).state_out_len = 1 as core::ffi::c_int as Int32;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as core::ffi::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as core::ffi::c_int as Bool;
            }
            k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr()) as UChar;
            (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize) as core::ffi::c_uint
                | (*((*s).ll4).offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_uint
                    >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                        & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                    << 16 as core::ffi::c_int) as UInt32;
            if (*s).rNToGo == 0 as core::ffi::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as core::ffi::c_int {
                    (*s).rTPos = 0 as core::ffi::c_int as Int32;
                }
            }
            (*s).rNToGo -= 1;
            k1 = (k1 as core::ffi::c_int
                ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                    1 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                continue;
            }
            if k1 as core::ffi::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as core::ffi::c_int as Int32;
                if (*s).tPos
                    >= (100000 as core::ffi::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as core::ffi::c_int as Bool;
                }
                k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                    as UChar;
                (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize) as core::ffi::c_uint
                    | (*((*s).ll4).offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_uint
                        >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                            & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                        << 16 as core::ffi::c_int) as UInt32;
                if (*s).rNToGo == 0 as core::ffi::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as core::ffi::c_int {
                        (*s).rTPos = 0 as core::ffi::c_int as Int32;
                    }
                }
                (*s).rNToGo -= 1;
                k1 = (k1 as core::ffi::c_int
                    ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used
                    == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                {
                    continue;
                }
                if k1 as core::ffi::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as core::ffi::c_int as Int32;
                    if (*s).tPos
                        >= (100000 as core::ffi::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as core::ffi::c_int as Bool;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                        as UChar;
                    (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize)
                        as core::ffi::c_uint
                        | (*((*s).ll4)
                            .offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_uint
                            >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                                & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                            << 16 as core::ffi::c_int) as UInt32;
                    if (*s).rNToGo == 0 as core::ffi::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as core::ffi::c_int {
                            (*s).rTPos = 0 as core::ffi::c_int as Int32;
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 = (k1 as core::ffi::c_int
                        ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                            1 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used
                        == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                    {
                        continue;
                    }
                    if k1 as core::ffi::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        k1 = BZ2_indexIntoF(
                            (*s).tPos as Int32,
                            ((*s).cftab).as_mut_ptr(),
                        ) as UChar;
                        (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize)
                            as core::ffi::c_uint
                            | (*((*s).ll4)
                                .offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                                as core::ffi::c_uint
                                >> (((*s).tPos as core::ffi::c_uint)
                                    << 2 as core::ffi::c_int & 0x4 as core::ffi::c_uint)
                                & 0xf as core::ffi::c_uint) << 16 as core::ffi::c_int)
                            as UInt32;
                        if (*s).rNToGo == 0 as core::ffi::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as core::ffi::c_int {
                                (*s).rTPos = 0 as core::ffi::c_int as Int32;
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 = (k1 as core::ffi::c_int
                            ^ if (*s).rNToGo == 1 as core::ffi::c_int {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = (k1 as core::ffi::c_int
                            + 4 as core::ffi::c_int) as Int32;
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
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
                        (*s).nblock_used += 1;
                    }
                }
            }
        }
    } else {
        while 1 as core::ffi::c_int as Bool != 0 {
            while 1 as core::ffi::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as core::ffi::c_uint {
                    return 0 as core::ffi::c_int as Bool;
                }
                if (*s).state_out_len == 0 as core::ffi::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC = (*s).calculatedBlockCRC
                    << 8 as core::ffi::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC as core::ffi::c_uint
                        >> 24 as core::ffi::c_int
                        ^ (*s).state_out_ch as core::ffi::c_uint) as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = ((*(*s).strm).next_out).offset(1);
                (*(*s).strm).avail_out = ((*(*s).strm).avail_out).wrapping_sub(1);
                (*(*s).strm).total_out_lo32 = ((*(*s).strm).total_out_lo32)
                    .wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as core::ffi::c_uint {
                    (*(*s).strm).total_out_hi32 = ((*(*s).strm).total_out_hi32)
                        .wrapping_add(1);
                }
            }
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 0 as core::ffi::c_int as Bool;
            }
            if (*s).nblock_used
                > (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                return 1 as core::ffi::c_int as Bool;
            }
            (*s).state_out_len = 1 as core::ffi::c_int as Int32;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as core::ffi::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as core::ffi::c_int as Bool;
            }
            k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr()) as UChar;
            (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize) as core::ffi::c_uint
                | (*((*s).ll4).offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_uint
                    >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                        & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                    << 16 as core::ffi::c_int) as UInt32;
            (*s).nblock_used += 1;
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
            {
                continue;
            }
            if k1 as core::ffi::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as core::ffi::c_int as Int32;
                if (*s).tPos
                    >= (100000 as core::ffi::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as core::ffi::c_int as Bool;
                }
                k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                    as UChar;
                (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize) as core::ffi::c_uint
                    | (*((*s).ll4).offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_uint
                        >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                            & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                        << 16 as core::ffi::c_int) as UInt32;
                (*s).nblock_used += 1;
                if (*s).nblock_used
                    == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                {
                    continue;
                }
                if k1 as core::ffi::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as core::ffi::c_int as Int32;
                    if (*s).tPos
                        >= (100000 as core::ffi::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as core::ffi::c_int as Bool;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                        as UChar;
                    (*s).tPos = (*((*s).ll16).offset((*s).tPos as isize)
                        as core::ffi::c_uint
                        | (*((*s).ll4)
                            .offset(((*s).tPos >> 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_uint
                            >> (((*s).tPos as core::ffi::c_uint) << 2 as core::ffi::c_int
                                & 0x4 as core::ffi::c_uint) & 0xf as core::ffi::c_uint)
                            << 16 as core::ffi::c_int) as UInt32;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used
                        == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                    {
                        continue;
                    }
                    if k1 as core::ffi::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
                        }
                        k1 = BZ2_indexIntoF(
                            (*s).tPos as Int32,
                            ((*s).cftab).as_mut_ptr(),
                        ) as UChar;
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
                        (*s).state_out_len = (k1 as core::ffi::c_int
                            + 4 as core::ffi::c_int) as Int32;
                        if (*s).tPos
                            >= (100000 as core::ffi::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as core::ffi::c_int as Bool;
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
                }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompress(mut strm: *mut bz_stream) -> core::ffi::c_int {
    let mut corrupt: Bool = 0;
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() {
        return -(2 as core::ffi::c_int);
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -(2 as core::ffi::c_int);
    }
    if (*s).strm != strm {
        return -(2 as core::ffi::c_int);
    }
    while 1 as core::ffi::c_int as Bool != 0 {
        if (*s).state == 1 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        if (*s).state == 2 as core::ffi::c_int {
            if (*s).smallDecompress != 0 {
                corrupt = unRLE_obuf_to_output_SMALL(s);
            } else {
                corrupt = unRLE_obuf_to_output_FAST(s);
            }
            if corrupt != 0 {
                return -(4 as core::ffi::c_int);
            }
            if (*s).nblock_used
                == (*s).save_nblock as core::ffi::c_int + 1 as core::ffi::c_int
                && (*s).state_out_len == 0 as core::ffi::c_int
            {
                (*s).calculatedBlockCRC = !(*s).calculatedBlockCRC;
                if (*s).verbosity >= 3 as core::ffi::c_int {
                    fprintf(
                        stderr,
                        b" {0x%08x, 0x%08x}\0" as *const u8 as *const core::ffi::c_char,
                        (*s).storedBlockCRC,
                        (*s).calculatedBlockCRC,
                    );
                }
                if (*s).verbosity >= 2 as core::ffi::c_int {
                    fprintf(stderr, b"]\0" as *const u8 as *const core::ffi::c_char);
                }
                if (*s).calculatedBlockCRC != (*s).storedBlockCRC {
                    return -(4 as core::ffi::c_int);
                }
                (*s).calculatedCombinedCRC = (*s).calculatedCombinedCRC
                    << 1 as core::ffi::c_int
                    | (*s).calculatedCombinedCRC >> 31 as core::ffi::c_int;
                (*s).calculatedCombinedCRC
                    ^= (*s).calculatedBlockCRC as core::ffi::c_uint;
                (*s).state = 14 as core::ffi::c_int as Int32;
            } else {
                return 0 as core::ffi::c_int
            }
        }
        if (*s).state >= 10 as core::ffi::c_int {
            let mut r: Int32 = BZ2_decompress(s);
            if r == 4 as core::ffi::c_int {
                if (*s).verbosity >= 3 as core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*s).storedCombinedCRC,
                        (*s).calculatedCombinedCRC,
                    );
                }
                if (*s).calculatedCombinedCRC != (*s).storedCombinedCRC {
                    return -(4 as core::ffi::c_int);
                }
                return r as core::ffi::c_int;
            }
            if (*s).state != 2 as core::ffi::c_int {
                return r as core::ffi::c_int;
            }
        }
    }
    if 0 as core::ffi::c_int == 0 {
        BZ2_bz__AssertH__fail(6001 as core::ffi::c_int);
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressEnd(
    mut strm: *mut bz_stream,
) -> core::ffi::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() {
        return -(2 as core::ffi::c_int);
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -(2 as core::ffi::c_int);
    }
    if (*s).strm != strm {
        return -(2 as core::ffi::c_int);
    }
    if !((*s).tt).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).tt as *mut core::ffi::c_void);
    }
    if !((*s).ll16).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ll16 as *mut core::ffi::c_void);
    }
    if !((*s).ll4).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ll4 as *mut core::ffi::c_void);
    }
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    (*strm).state = 0 as *mut core::ffi::c_void;
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn BZ_SETERR(
    mut bzerror: *mut core::ffi::c_int,
    mut bzf: *mut bzFile,
    mut eee: core::ffi::c_int,
) {
    if !bzerror.is_null() {
        *bzerror = eee;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = eee as Int32;
    }
}
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f) as Int32;
    if c == -(1 as core::ffi::c_int) {
        return 1 as core::ffi::c_int as Bool;
    }
    ungetc(c as core::ffi::c_int, f);
    return 0 as core::ffi::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteOpen(
    mut bzerror: *mut core::ffi::c_int,
    mut f: *mut FILE,
    mut blockSize100k: core::ffi::c_int,
    mut verbosity: core::ffi::c_int,
    mut workFactor: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    if f.is_null()
        || (blockSize100k < 1 as core::ffi::c_int
            || blockSize100k > 9 as core::ffi::c_int)
        || (workFactor < 0 as core::ffi::c_int || workFactor > 250 as core::ffi::c_int)
        || (verbosity < 0 as core::ffi::c_int || verbosity > 4 as core::ffi::c_int)
    {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    if ferror(f) != 0 {
        BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    bzf = malloc(::core::mem::size_of::<bzFile>() as size_t) as *mut bzFile;
    if bzf.is_null() {
        BZ_SETERR(bzerror, bzf, -(3 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    (*bzf).initialisedOk = 0 as core::ffi::c_int as Bool;
    (*bzf).bufN = 0 as core::ffi::c_int as Int32;
    (*bzf).handle = f;
    (*bzf).writing = 1 as core::ffi::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut core::ffi::c_void;
    if workFactor == 0 as core::ffi::c_int {
        workFactor = 30 as core::ffi::c_int;
    }
    ret = BZ2_bzCompressInit(&mut (*bzf).strm, blockSize100k, verbosity, workFactor)
        as Int32;
    if ret != 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, ret as core::ffi::c_int);
        free(bzf as *mut core::ffi::c_void);
        return 0 as *mut core::ffi::c_void;
    }
    (*bzf).strm.avail_in = 0 as core::ffi::c_uint;
    (*bzf).initialisedOk = 1 as core::ffi::c_int as Bool;
    return bzf as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWrite(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
    mut buf: *mut core::ffi::c_void,
    mut len: core::ffi::c_int,
) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    if bzf.is_null() || buf.is_null() || len < 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return;
    }
    if (*bzf).writing == 0 {
        BZ_SETERR(bzerror, bzf, -(1 as core::ffi::c_int));
        return;
    }
    if ferror((*bzf).handle) != 0 {
        BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
        return;
    }
    if len == 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
        return;
    }
    (*bzf).strm.avail_in = len as core::ffi::c_uint;
    (*bzf).strm.next_in = buf as *mut core::ffi::c_char;
    while 1 as core::ffi::c_int as Bool != 0 {
        (*bzf).strm.avail_out = 5000 as core::ffi::c_uint;
        (*bzf).strm.next_out = ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_char;
        ret = BZ2_bzCompress(&mut (*bzf).strm, 0 as core::ffi::c_int) as Int32;
        if ret != 1 as core::ffi::c_int {
            BZ_SETERR(bzerror, bzf, ret as core::ffi::c_int);
            return;
        }
        if (*bzf).strm.avail_out < 5000 as core::ffi::c_uint {
            n = (5000 as core::ffi::c_uint).wrapping_sub((*bzf).strm.avail_out) as Int32;
            n2 = fwrite(
                ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<UChar>() as size_t,
                n as size_t,
                (*bzf).handle,
            ) as Int32;
            if n != n2 || ferror((*bzf).handle) != 0 {
                BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
                return;
            }
        }
        if (*bzf).strm.avail_in == 0 as core::ffi::c_uint {
            BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
    mut abandon: core::ffi::c_int,
    mut nbytes_in: *mut core::ffi::c_uint,
    mut nbytes_out: *mut core::ffi::c_uint,
) {
    BZ2_bzWriteClose64(
        bzerror,
        b,
        abandon,
        nbytes_in,
        0 as *mut core::ffi::c_uint,
        nbytes_out,
        0 as *mut core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose64(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
    mut abandon: core::ffi::c_int,
    mut nbytes_in_lo32: *mut core::ffi::c_uint,
    mut nbytes_in_hi32: *mut core::ffi::c_uint,
    mut nbytes_out_lo32: *mut core::ffi::c_uint,
    mut nbytes_out_hi32: *mut core::ffi::c_uint,
) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
        return;
    }
    if (*bzf).writing == 0 {
        BZ_SETERR(bzerror, bzf, -(1 as core::ffi::c_int));
        return;
    }
    if ferror((*bzf).handle) != 0 {
        BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
        return;
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = 0 as core::ffi::c_uint;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = 0 as core::ffi::c_uint;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = 0 as core::ffi::c_uint;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = 0 as core::ffi::c_uint;
    }
    if abandon == 0 && (*bzf).lastErr == 0 as core::ffi::c_int {
        while 1 as core::ffi::c_int as Bool != 0 {
            (*bzf).strm.avail_out = 5000 as core::ffi::c_uint;
            (*bzf).strm.next_out = ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_char;
            ret = BZ2_bzCompress(&mut (*bzf).strm, 2 as core::ffi::c_int) as Int32;
            if ret != 3 as core::ffi::c_int && ret != 4 as core::ffi::c_int {
                BZ_SETERR(bzerror, bzf, ret as core::ffi::c_int);
                return;
            }
            if (*bzf).strm.avail_out < 5000 as core::ffi::c_uint {
                n = (5000 as core::ffi::c_uint).wrapping_sub((*bzf).strm.avail_out)
                    as Int32;
                n2 = fwrite(
                    ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_void,
                    ::core::mem::size_of::<UChar>() as size_t,
                    n as size_t,
                    (*bzf).handle,
                ) as Int32;
                if n != n2 || ferror((*bzf).handle) != 0 {
                    BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
                    return;
                }
            }
            if ret == 4 as core::ffi::c_int {
                break;
            }
        }
    }
    if abandon == 0 && ferror((*bzf).handle) == 0 {
        fflush((*bzf).handle);
        if ferror((*bzf).handle) != 0 {
            BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
            return;
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = (*bzf).strm.total_in_lo32;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = (*bzf).strm.total_in_hi32;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = (*bzf).strm.total_out_lo32;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = (*bzf).strm.total_out_hi32;
    }
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    BZ2_bzCompressEnd(&mut (*bzf).strm);
    free(bzf as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadOpen(
    mut bzerror: *mut core::ffi::c_int,
    mut f: *mut FILE,
    mut verbosity: core::ffi::c_int,
    mut small: core::ffi::c_int,
    mut unused: *mut core::ffi::c_void,
    mut nUnused: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    let mut ret: core::ffi::c_int = 0;
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    if f.is_null() || small != 0 as core::ffi::c_int && small != 1 as core::ffi::c_int
        || (verbosity < 0 as core::ffi::c_int || verbosity > 4 as core::ffi::c_int)
        || unused.is_null() && nUnused != 0 as core::ffi::c_int
        || !unused.is_null()
            && (nUnused < 0 as core::ffi::c_int || nUnused > 5000 as core::ffi::c_int)
    {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    if ferror(f) != 0 {
        BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    bzf = malloc(::core::mem::size_of::<bzFile>() as size_t) as *mut bzFile;
    if bzf.is_null() {
        BZ_SETERR(bzerror, bzf, -(3 as core::ffi::c_int));
        return 0 as *mut core::ffi::c_void;
    }
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    (*bzf).initialisedOk = 0 as core::ffi::c_int as Bool;
    (*bzf).handle = f;
    (*bzf).bufN = 0 as core::ffi::c_int as Int32;
    (*bzf).writing = 0 as core::ffi::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut core::ffi::c_void;
    while nUnused > 0 as core::ffi::c_int {
        (*bzf).buf[(*bzf).bufN as usize] = *(unused as *mut UChar) as Char;
        (*bzf).bufN += 1;
        unused = (unused as *mut UChar).offset(1 as core::ffi::c_int as isize)
            as *mut core::ffi::c_void;
        nUnused -= 1;
    }
    ret = BZ2_bzDecompressInit(&mut (*bzf).strm, verbosity, small);
    if ret != 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, ret);
        free(bzf as *mut core::ffi::c_void);
        return 0 as *mut core::ffi::c_void;
    }
    (*bzf).strm.avail_in = (*bzf).bufN as core::ffi::c_uint;
    (*bzf).strm.next_in = ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_char;
    (*bzf).initialisedOk = 1 as core::ffi::c_int as Bool;
    return bzf as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadClose(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    if bzf.is_null() {
        BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
        return;
    }
    if (*bzf).writing != 0 {
        BZ_SETERR(bzerror, bzf, -(1 as core::ffi::c_int));
        return;
    }
    if (*bzf).initialisedOk != 0 {
        BZ2_bzDecompressEnd(&mut (*bzf).strm);
    }
    free(bzf as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzRead(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
    mut buf: *mut core::ffi::c_void,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut n: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    if bzf.is_null() || buf.is_null() || len < 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return 0 as core::ffi::c_int;
    }
    if (*bzf).writing != 0 {
        BZ_SETERR(bzerror, bzf, -(1 as core::ffi::c_int));
        return 0 as core::ffi::c_int;
    }
    if len == 0 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
        return 0 as core::ffi::c_int;
    }
    (*bzf).strm.avail_out = len as core::ffi::c_uint;
    (*bzf).strm.next_out = buf as *mut core::ffi::c_char;
    while 1 as core::ffi::c_int as Bool != 0 {
        if ferror((*bzf).handle) != 0 {
            BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
            return 0 as core::ffi::c_int;
        }
        if (*bzf).strm.avail_in == 0 as core::ffi::c_uint && myfeof((*bzf).handle) == 0 {
            n = fread(
                ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<UChar>() as size_t,
                5000 as size_t,
                (*bzf).handle,
            ) as Int32;
            if ferror((*bzf).handle) != 0 {
                BZ_SETERR(bzerror, bzf, -(6 as core::ffi::c_int));
                return 0 as core::ffi::c_int;
            }
            (*bzf).bufN = n;
            (*bzf).strm.avail_in = (*bzf).bufN as core::ffi::c_uint;
            (*bzf).strm.next_in = ((*bzf).buf).as_mut_ptr() as *mut core::ffi::c_char;
        }
        ret = BZ2_bzDecompress(&mut (*bzf).strm) as Int32;
        if ret != 0 as core::ffi::c_int && ret != 4 as core::ffi::c_int {
            BZ_SETERR(bzerror, bzf, ret as core::ffi::c_int);
            return 0 as core::ffi::c_int;
        }
        if ret == 0 as core::ffi::c_int && myfeof((*bzf).handle) as core::ffi::c_int != 0
            && (*bzf).strm.avail_in == 0 as core::ffi::c_uint
            && (*bzf).strm.avail_out > 0 as core::ffi::c_uint
        {
            BZ_SETERR(bzerror, bzf, -(7 as core::ffi::c_int));
            return 0 as core::ffi::c_int;
        }
        if ret == 4 as core::ffi::c_int {
            BZ_SETERR(bzerror, bzf, 4 as core::ffi::c_int);
            return (len as core::ffi::c_uint).wrapping_sub((*bzf).strm.avail_out)
                as core::ffi::c_int;
        }
        if (*bzf).strm.avail_out == 0 as core::ffi::c_uint {
            BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
            return len;
        }
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    mut bzerror: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_void,
    mut unused: *mut *mut core::ffi::c_void,
    mut nUnused: *mut core::ffi::c_int,
) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return;
    }
    if (*bzf).lastErr != 4 as core::ffi::c_int {
        BZ_SETERR(bzerror, bzf, -(1 as core::ffi::c_int));
        return;
    }
    if unused.is_null() || nUnused.is_null() {
        BZ_SETERR(bzerror, bzf, -(2 as core::ffi::c_int));
        return;
    }
    BZ_SETERR(bzerror, bzf, 0 as core::ffi::c_int);
    *nUnused = (*bzf).strm.avail_in as core::ffi::c_int;
    *unused = (*bzf).strm.next_in as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffCompress(
    mut dest: *mut core::ffi::c_char,
    mut destLen: *mut core::ffi::c_uint,
    mut source: *mut core::ffi::c_char,
    mut sourceLen: core::ffi::c_uint,
    mut blockSize100k: core::ffi::c_int,
    mut verbosity: core::ffi::c_int,
    mut workFactor: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut strm: bz_stream = bz_stream {
        next_in: 0 as *mut core::ffi::c_char,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut core::ffi::c_char,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut core::ffi::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut core::ffi::c_void,
    };
    let mut ret: core::ffi::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null()
        || blockSize100k < 1 as core::ffi::c_int || blockSize100k > 9 as core::ffi::c_int
        || verbosity < 0 as core::ffi::c_int || verbosity > 4 as core::ffi::c_int
        || workFactor < 0 as core::ffi::c_int || workFactor > 250 as core::ffi::c_int
    {
        return -(2 as core::ffi::c_int);
    }
    if workFactor == 0 as core::ffi::c_int {
        workFactor = 30 as core::ffi::c_int;
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut core::ffi::c_void;
    ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != 0 as core::ffi::c_int {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzCompress(&mut strm, 2 as core::ffi::c_int);
    if ret == 3 as core::ffi::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return -(8 as core::ffi::c_int);
    } else if ret != 4 as core::ffi::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzCompressEnd(&mut strm);
        return 0 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffDecompress(
    mut dest: *mut core::ffi::c_char,
    mut destLen: *mut core::ffi::c_uint,
    mut source: *mut core::ffi::c_char,
    mut sourceLen: core::ffi::c_uint,
    mut small: core::ffi::c_int,
    mut verbosity: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut strm: bz_stream = bz_stream {
        next_in: 0 as *mut core::ffi::c_char,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut core::ffi::c_char,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut core::ffi::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut core::ffi::c_void,
    };
    let mut ret: core::ffi::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null()
        || small != 0 as core::ffi::c_int && small != 1 as core::ffi::c_int
        || verbosity < 0 as core::ffi::c_int || verbosity > 4 as core::ffi::c_int
    {
        return -(2 as core::ffi::c_int);
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut core::ffi::c_void;
    ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != 0 as core::ffi::c_int {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzDecompress(&mut strm);
    if ret == 0 as core::ffi::c_int {
        if strm.avail_out > 0 as core::ffi::c_uint {
            BZ2_bzDecompressEnd(&mut strm);
            return -(7 as core::ffi::c_int);
        } else {
            BZ2_bzDecompressEnd(&mut strm);
            return -(8 as core::ffi::c_int);
        }
    } else if ret != 4 as core::ffi::c_int {
        BZ2_bzDecompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzDecompressEnd(&mut strm);
        return 0 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzlibVersion() -> *const core::ffi::c_char {
    return b"1.0.8, 13-Jul-2019\0" as *const u8 as *const core::ffi::c_char;
}
unsafe extern "C" fn bzopen_or_bzdopen(
    mut path: *const core::ffi::c_char,
    mut fd: core::ffi::c_int,
    mut mode: *const core::ffi::c_char,
    mut open_mode: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut bzerr: core::ffi::c_int = 0;
    let mut unused: [core::ffi::c_char; 5000] = [0; 5000];
    let mut blockSize100k: core::ffi::c_int = 9 as core::ffi::c_int;
    let mut writing: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mode2: [core::ffi::c_char; 10] = ::core::mem::transmute::<
        [u8; 10],
        [core::ffi::c_char; 10],
    >(*b"\0\0\0\0\0\0\0\0\0\0");
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut bzfp: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut verbosity: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut workFactor: core::ffi::c_int = 30 as core::ffi::c_int;
    let mut smallMode: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut nUnused: core::ffi::c_int = 0 as core::ffi::c_int;
    if mode.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    while *mode != 0 {
        match *mode as core::ffi::c_int {
            114 => {
                writing = 0 as core::ffi::c_int;
            }
            119 => {
                writing = 1 as core::ffi::c_int;
            }
            115 => {
                smallMode = 1 as core::ffi::c_int;
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*mode as core::ffi::c_int as isize)
                    as core::ffi::c_int
                    & _ISdigit as core::ffi::c_int as core::ffi::c_ushort
                        as core::ffi::c_int != 0
                {
                    blockSize100k = *mode as core::ffi::c_int - 0x30 as core::ffi::c_int;
                }
            }
        }
        mode = mode.offset(1);
    }
    strcat(
        mode2.as_mut_ptr(),
        if writing != 0 {
            b"w\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"r\0" as *const u8 as *const core::ffi::c_char
        },
    );
    strcat(mode2.as_mut_ptr(), b"b\0" as *const u8 as *const core::ffi::c_char);
    if open_mode == 0 as core::ffi::c_int {
        if path.is_null()
            || strcmp(path, b"\0" as *const u8 as *const core::ffi::c_char)
                == 0 as core::ffi::c_int
        {
            fp = if writing != 0 { stdout } else { stdin };
        } else {
            fp = fopen(path, mode2.as_mut_ptr());
        }
    } else {
        fp = fdopen(fd, mode2.as_mut_ptr());
    }
    if fp.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    if writing != 0 {
        if blockSize100k < 1 as core::ffi::c_int {
            blockSize100k = 1 as core::ffi::c_int;
        }
        if blockSize100k > 9 as core::ffi::c_int {
            blockSize100k = 9 as core::ffi::c_int;
        }
        bzfp = BZ2_bzWriteOpen(&mut bzerr, fp, blockSize100k, verbosity, workFactor);
    } else {
        bzfp = BZ2_bzReadOpen(
            &mut bzerr,
            fp,
            verbosity,
            smallMode,
            unused.as_mut_ptr() as *mut core::ffi::c_void,
            nUnused,
        );
    }
    if bzfp.is_null() {
        if fp != stdin && fp != stdout {
            fclose(fp);
        }
        return 0 as *mut core::ffi::c_void;
    }
    return bzfp;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzopen(
    mut path: *const core::ffi::c_char,
    mut mode: *const core::ffi::c_char,
) -> *mut core::ffi::c_void {
    return bzopen_or_bzdopen(
        path,
        -(1 as core::ffi::c_int),
        mode,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzdopen(
    mut fd: core::ffi::c_int,
    mut mode: *const core::ffi::c_char,
) -> *mut core::ffi::c_void {
    return bzopen_or_bzdopen(
        0 as *const core::ffi::c_char,
        fd,
        mode,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzread(
    mut b: *mut core::ffi::c_void,
    mut buf: *mut core::ffi::c_void,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut bzerr: core::ffi::c_int = 0;
    let mut nread: core::ffi::c_int = 0;
    if (*(b as *mut bzFile)).lastErr == 4 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    nread = BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 as core::ffi::c_int || bzerr == 4 as core::ffi::c_int {
        return nread
    } else {
        return -(1 as core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzwrite(
    mut b: *mut core::ffi::c_void,
    mut buf: *mut core::ffi::c_void,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut bzerr: core::ffi::c_int = 0;
    BZ2_bzWrite(&mut bzerr, b, buf, len);
    if bzerr == 0 as core::ffi::c_int {
        return len
    } else {
        return -(1 as core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzflush(mut b: *mut core::ffi::c_void) -> core::ffi::c_int {
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzclose(mut b: *mut core::ffi::c_void) {
    let mut bzerr: core::ffi::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if b.is_null() {
        return;
    }
    fp = (*(b as *mut bzFile)).handle;
    if (*(b as *mut bzFile)).writing != 0 {
        BZ2_bzWriteClose(
            &mut bzerr,
            b,
            0 as core::ffi::c_int,
            0 as *mut core::ffi::c_uint,
            0 as *mut core::ffi::c_uint,
        );
        if bzerr != 0 as core::ffi::c_int {
            BZ2_bzWriteClose(
                0 as *mut core::ffi::c_int,
                b,
                1 as core::ffi::c_int,
                0 as *mut core::ffi::c_uint,
                0 as *mut core::ffi::c_uint,
            );
        }
    } else {
        BZ2_bzReadClose(&mut bzerr, b);
    }
    if fp != stdin && fp != stdout {
        fclose(fp);
    }
}
static mut bzerrorstrings: [*const core::ffi::c_char; 16] = [
    b"OK\0" as *const u8 as *const core::ffi::c_char,
    b"SEQUENCE_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"PARAM_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"MEM_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"DATA_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"DATA_ERROR_MAGIC\0" as *const u8 as *const core::ffi::c_char,
    b"IO_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"UNEXPECTED_EOF\0" as *const u8 as *const core::ffi::c_char,
    b"OUTBUFF_FULL\0" as *const u8 as *const core::ffi::c_char,
    b"CONFIG_ERROR\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
    b"???\0" as *const u8 as *const core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzerror(
    mut b: *mut core::ffi::c_void,
    mut errnum: *mut core::ffi::c_int,
) -> *const core::ffi::c_char {
    let mut err: core::ffi::c_int = (*(b as *mut bzFile)).lastErr as core::ffi::c_int;
    if err > 0 as core::ffi::c_int {
        err = 0 as core::ffi::c_int;
    }
    *errnum = err;
    return bzerrorstrings[(err * -(1 as core::ffi::c_int)) as usize];
}
