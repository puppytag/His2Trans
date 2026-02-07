#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn exit(__status: core::ffi::c_int) -> !;
}
pub type size_t = core::ffi::c_ulong;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut core::ffi::c_ushort,
    pub dist: *mut core::ffi::c_ushort,
    pub sublen: *mut core::ffi::c_uchar,
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
#[no_mangle]
pub unsafe extern "C" fn ZopfliInitCache(
    mut blocksize: size_t,
    mut lmc: *mut ZopfliLongestMatchCache,
) {
    let mut i: size_t = 0;
    (*lmc).length = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t).wrapping_mul(blocksize),
    ) as *mut core::ffi::c_ushort;
    (*lmc).dist = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t).wrapping_mul(blocksize),
    ) as *mut core::ffi::c_ushort;
    (*lmc).sublen = malloc(
        ((8 as core::ffi::c_int * 3 as core::ffi::c_int) as size_t)
            .wrapping_mul(blocksize),
    ) as *mut core::ffi::c_uchar;
    if ((*lmc).sublen).is_null() {
        fprintf(
            stderr,
            b"Error: Out of memory. Tried allocating %lu bytes of memory.\n\0"
                as *const u8 as *const core::ffi::c_char,
            (8 as core::ffi::c_int as size_t)
                .wrapping_mul(3 as size_t)
                .wrapping_mul(blocksize),
        );
        exit(1 as core::ffi::c_int);
    }
    i = 0 as size_t;
    while i < blocksize {
        *((*lmc).length).offset(i as isize) = 1 as core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < blocksize {
        *((*lmc).dist).offset(i as isize) = 0 as core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i
        < (8 as core::ffi::c_ulong)
            .wrapping_mul(blocksize as core::ffi::c_ulong)
            .wrapping_mul(3 as core::ffi::c_ulong)
    {
        *((*lmc).sublen).offset(i as isize) = 0 as core::ffi::c_uchar;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanCache(mut lmc: *mut ZopfliLongestMatchCache) {
    free((*lmc).length as *mut core::ffi::c_void);
    free((*lmc).dist as *mut core::ffi::c_void);
    free((*lmc).sublen as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliSublenToCache(
    mut sublen: *const core::ffi::c_ushort,
    mut pos: size_t,
    mut length: size_t,
    mut lmc: *mut ZopfliLongestMatchCache,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as size_t;
    let mut bestlength: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut cache: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    cache = &mut *((*lmc).sublen)
        .offset(
            (8 as core::ffi::c_ulong)
                .wrapping_mul(pos as core::ffi::c_ulong)
                .wrapping_mul(3 as core::ffi::c_ulong) as isize,
        ) as *mut core::ffi::c_uchar;
    if length < 3 as core::ffi::c_ulong {
        return;
    }
    i = 3 as size_t;
    while i <= length {
        if i == length
            || *sublen.offset(i as isize) as core::ffi::c_int
                != *sublen
                    .offset(
                        (i as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong)
                            as isize,
                    ) as core::ffi::c_int
        {
            *cache
                .offset(
                    (j as core::ffi::c_ulong).wrapping_mul(3 as core::ffi::c_ulong)
                        as isize,
                ) = (i as core::ffi::c_ulong).wrapping_sub(3 as core::ffi::c_ulong)
                as core::ffi::c_uchar;
            *cache
                .offset(
                    (j as core::ffi::c_ulong)
                        .wrapping_mul(3 as core::ffi::c_ulong)
                        .wrapping_add(1 as core::ffi::c_ulong) as isize,
                ) = (*sublen.offset(i as isize) as core::ffi::c_int
                % 256 as core::ffi::c_int) as core::ffi::c_uchar;
            *cache
                .offset(
                    (j as core::ffi::c_ulong)
                        .wrapping_mul(3 as core::ffi::c_ulong)
                        .wrapping_add(2 as core::ffi::c_ulong) as isize,
                ) = ((*sublen.offset(i as isize) as core::ffi::c_int
                >> 8 as core::ffi::c_int) % 256 as core::ffi::c_int)
                as core::ffi::c_uchar;
            bestlength = i as core::ffi::c_uint;
            j = j.wrapping_add(1);
            if j >= 8 as core::ffi::c_ulong {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if j < 8 as core::ffi::c_ulong {
        if bestlength as core::ffi::c_ulong == length {} else {
            __assert_fail(
                b"bestlength == length\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                    as *const u8 as *const core::ffi::c_char,
                79 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 92],
                    [core::ffi::c_char; 92],
                >(
                    *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_1903: {
            if bestlength as core::ffi::c_ulong == length {} else {
                __assert_fail(
                    b"bestlength == length\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    79 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 92],
                        [core::ffi::c_char; 92],
                    >(
                        *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *cache
            .offset(
                ((8 as core::ffi::c_int - 1 as core::ffi::c_int) * 3 as core::ffi::c_int)
                    as isize,
            ) = bestlength.wrapping_sub(3 as core::ffi::c_uint) as core::ffi::c_uchar;
    } else {
        if bestlength as core::ffi::c_ulong <= length {} else {
            __assert_fail(
                b"bestlength <= length\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                    as *const u8 as *const core::ffi::c_char,
                82 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 92],
                    [core::ffi::c_char; 92],
                >(
                    *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_1843: {
            if bestlength as core::ffi::c_ulong <= length {} else {
                __assert_fail(
                    b"bestlength <= length\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    82 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 92],
                        [core::ffi::c_char; 92],
                    >(
                        *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    if bestlength == ZopfliMaxCachedSublen(lmc, pos, length) {} else {
        __assert_fail(
            b"bestlength == ZopfliMaxCachedSublen(lmc, pos, length)\0" as *const u8
                as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                as *const u8 as *const core::ffi::c_char,
            84 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 92],
                [core::ffi::c_char; 92],
            >(
                *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1703: {
        if bestlength == ZopfliMaxCachedSublen(lmc, pos, length) {} else {
            __assert_fail(
                b"bestlength == ZopfliMaxCachedSublen(lmc, pos, length)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/cache.c\0"
                    as *const u8 as *const core::ffi::c_char,
                84 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 92],
                    [core::ffi::c_char; 92],
                >(
                    *b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCacheToSublen(
    mut lmc: *const ZopfliLongestMatchCache,
    mut pos: size_t,
    mut length: size_t,
    mut sublen: *mut core::ffi::c_ushort,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut maxlength: core::ffi::c_uint = ZopfliMaxCachedSublen(lmc, pos, length);
    let mut prevlength: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut cache: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    if length < 3 as core::ffi::c_ulong {
        return;
    }
    cache = &mut *((*lmc).sublen)
        .offset(
            (8 as core::ffi::c_ulong)
                .wrapping_mul(pos as core::ffi::c_ulong)
                .wrapping_mul(3 as core::ffi::c_ulong) as isize,
        ) as *mut core::ffi::c_uchar;
    j = 0 as size_t;
    while j < 8 as core::ffi::c_ulong {
        let mut length_0: core::ffi::c_uint = (*cache
            .offset(
                (j as core::ffi::c_ulong).wrapping_mul(3 as core::ffi::c_ulong) as isize,
            ) as core::ffi::c_int + 3 as core::ffi::c_int) as core::ffi::c_uint;
        let mut dist: core::ffi::c_uint = (*cache
            .offset(
                (j as core::ffi::c_ulong)
                    .wrapping_mul(3 as core::ffi::c_ulong)
                    .wrapping_add(1 as core::ffi::c_ulong) as isize,
            ) as core::ffi::c_int
            + 256 as core::ffi::c_int
                * *cache
                    .offset(
                        (j as core::ffi::c_ulong)
                            .wrapping_mul(3 as core::ffi::c_ulong)
                            .wrapping_add(2 as core::ffi::c_ulong) as isize,
                    ) as core::ffi::c_int) as core::ffi::c_uint;
        i = prevlength as size_t;
        while i <= length_0 as core::ffi::c_ulong {
            *sublen.offset(i as isize) = dist as core::ffi::c_ushort;
            i = i.wrapping_add(1);
        }
        if length_0 == maxlength {
            break;
        }
        prevlength = length_0.wrapping_add(1 as core::ffi::c_uint);
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliMaxCachedSublen(
    mut lmc: *const ZopfliLongestMatchCache,
    mut pos: size_t,
    mut length: size_t,
) -> core::ffi::c_uint {
    let mut cache: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    cache = &mut *((*lmc).sublen)
        .offset(
            (8 as core::ffi::c_ulong)
                .wrapping_mul(pos as core::ffi::c_ulong)
                .wrapping_mul(3 as core::ffi::c_ulong) as isize,
        ) as *mut core::ffi::c_uchar;
    if *cache.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        == 0 as core::ffi::c_int
        && *cache.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
            == 0 as core::ffi::c_int
    {
        return 0 as core::ffi::c_uint;
    }
    return (*cache
        .offset(
            ((8 as core::ffi::c_int - 1 as core::ffi::c_int) * 3 as core::ffi::c_int)
                as isize,
        ) as core::ffi::c_int + 3 as core::ffi::c_int) as core::ffi::c_uint;
}
