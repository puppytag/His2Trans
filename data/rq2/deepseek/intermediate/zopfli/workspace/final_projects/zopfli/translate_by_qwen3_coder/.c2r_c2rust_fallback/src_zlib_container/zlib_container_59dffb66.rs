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
    fn ZopfliDeflate(
        options: *const ZopfliOptions,
        btype: core::ffi::c_int,
        final_0: core::ffi::c_int,
        in_0: *const core::ffi::c_uchar,
        insize: size_t,
        bp: *mut core::ffi::c_uchar,
        out: *mut *mut core::ffi::c_uchar,
        outsize: *mut size_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: core::ffi::c_int,
    pub verbose_more: core::ffi::c_int,
    pub numiterations: core::ffi::c_int,
    pub blocksplitting: core::ffi::c_int,
    pub blocksplittinglast: core::ffi::c_int,
    pub blocksplittingmax: core::ffi::c_int,
}
pub type FILE = _IO_FILE;
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
unsafe extern "C" fn adler32(
    mut data: *const core::ffi::c_uchar,
    mut size: size_t,
) -> core::ffi::c_uint {
    static mut sums_overflow: core::ffi::c_uint = 5550 as core::ffi::c_uint;
    let mut s1: core::ffi::c_uint = 1 as core::ffi::c_uint;
    let mut s2: core::ffi::c_uint = (1 as core::ffi::c_int >> 16 as core::ffi::c_int)
        as core::ffi::c_uint;
    while size > 0 as core::ffi::c_ulong {
        let mut amount: size_t = if size > sums_overflow as core::ffi::c_ulong {
            sums_overflow as size_t
        } else {
            size
        };
        size = (size as core::ffi::c_ulong).wrapping_sub(amount as core::ffi::c_ulong)
            as size_t as size_t;
        while amount > 0 as core::ffi::c_ulong {
            let fresh0 = data;
            data = data.offset(1);
            s1 = s1.wrapping_add(*fresh0 as core::ffi::c_uint);
            s2 = s2.wrapping_add(s1);
            amount = amount.wrapping_sub(1);
        }
        s1 = s1.wrapping_rem(65521 as core::ffi::c_uint);
        s2 = s2.wrapping_rem(65521 as core::ffi::c_uint);
    }
    return s2 << 16 as core::ffi::c_int | s1;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliZlibCompress(
    mut options: *const ZopfliOptions,
    mut in_0: *const core::ffi::c_uchar,
    mut insize: size_t,
    mut out: *mut *mut core::ffi::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut bitpointer: core::ffi::c_uchar = 0 as core::ffi::c_uchar;
    let mut checksum: core::ffi::c_uint = adler32(
        in_0,
        insize as core::ffi::c_uint as size_t,
    );
    let mut cmf: core::ffi::c_uint = 120 as core::ffi::c_uint;
    let mut flevel: core::ffi::c_uint = 3 as core::ffi::c_uint;
    let mut fdict: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut cmfflg: core::ffi::c_uint = (256 as core::ffi::c_uint)
        .wrapping_mul(cmf)
        .wrapping_add(fdict.wrapping_mul(32 as core::ffi::c_uint))
        .wrapping_add(flevel.wrapping_mul(64 as core::ffi::c_uint));
    let mut fcheck: core::ffi::c_uint = (31 as core::ffi::c_uint)
        .wrapping_sub(cmfflg.wrapping_rem(31 as core::ffi::c_uint));
    cmfflg = cmfflg.wrapping_add(fcheck);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = cmfflg.wrapping_div(256 as core::ffi::c_uint)
        as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = cmfflg.wrapping_rem(256 as core::ffi::c_uint)
        as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    ZopfliDeflate(
        options,
        2 as core::ffi::c_int,
        1 as core::ffi::c_int,
        in_0,
        insize,
        &mut bitpointer,
        out,
        outsize,
    );
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 24 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_uint) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 16 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_uint) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = (checksum >> 8 as core::ffi::c_int)
        .wrapping_rem(256 as core::ffi::c_uint) as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        *out = (if *outsize == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut core::ffi::c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_uchar>() as size_t),
            )
        }) as *mut core::ffi::c_uchar;
    }
    *(*out).offset(*outsize as isize) = checksum.wrapping_rem(256 as core::ffi::c_uint)
        as core::ffi::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if (*options).verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %d, Zlib: %d, Compression: %f%% Removed\n\0" as *const u8
                as *const core::ffi::c_char,
            insize as core::ffi::c_int,
            *outsize as core::ffi::c_int,
            100.0f64 * insize.wrapping_sub(*outsize) as core::ffi::c_double
                / insize as core::ffi::c_double,
        );
    }
}
