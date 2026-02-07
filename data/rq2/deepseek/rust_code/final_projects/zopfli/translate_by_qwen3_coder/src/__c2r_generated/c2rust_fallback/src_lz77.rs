#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub fn ZopfliInitCache(blocksize: size_t, lmc: *mut ZopfliLongestMatchCache);
    pub fn ZopfliCleanCache(lmc: *mut ZopfliLongestMatchCache);
    pub fn ZopfliSublenToCache(
        sublen: *const core::ffi::c_ushort,
        pos: size_t,
        length: size_t,
        lmc: *mut ZopfliLongestMatchCache,
    );
    pub fn ZopfliResetHash(window_size: size_t, h: *mut ZopfliHash);
    pub fn ZopfliCacheToSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
        sublen: *mut core::ffi::c_ushort,
    );
    pub fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    pub fn ZopfliMaxCachedSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
    ) -> core::ffi::c_uint;
    pub fn ZopfliUpdateHash(
        array: *const core::ffi::c_uchar,
        pos: size_t,
        end: size_t,
        h: *mut ZopfliHash,
    );
    pub fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    pub fn ZopfliWarmupHash(
        array: *const core::ffi::c_uchar,
        pos: size_t,
        end: size_t,
        h: *mut ZopfliHash,
    );
    pub fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    pub fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    pub fn free(__ptr: *mut core::ffi::c_void);
    pub fn exit(__status: core::ffi::c_int) -> !;
}
pub type size_t = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut core::ffi::c_ushort,
    pub dist: *mut core::ffi::c_ushort,
    pub sublen: *mut core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliHash {
    pub head: *mut core::ffi::c_int,
    pub prev: *mut core::ffi::c_ushort,
    pub hashval: *mut core::ffi::c_int,
    pub val: core::ffi::c_int,
    pub head2: *mut core::ffi::c_int,
    pub prev2: *mut core::ffi::c_ushort,
    pub hashval2: *mut core::ffi::c_int,
    pub val2: core::ffi::c_int,
    pub same: *mut core::ffi::c_ushort,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLZ77Store {
    pub litlens: *mut core::ffi::c_ushort,
    pub dists: *mut core::ffi::c_ushort,
    pub size: size_t,
    pub data: *const core::ffi::c_uchar,
    pub pos: *mut size_t,
    pub ll_symbol: *mut core::ffi::c_ushort,
    pub d_symbol: *mut core::ffi::c_ushort,
    pub ll_counts: *mut size_t,
    pub d_counts: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
}
pub unsafe extern "C" fn ZopfliInitLZ77Store(
    mut data: *const core::ffi::c_uchar,
    mut store: *mut ZopfliLZ77Store,
) {
    (*store).size = 0 as size_t;
    (*store).litlens = 0 as *mut core::ffi::c_ushort;
    (*store).dists = 0 as *mut core::ffi::c_ushort;
    (*store).pos = 0 as *mut size_t;
    (*store).data = data;
    (*store).ll_symbol = 0 as *mut core::ffi::c_ushort;
    (*store).d_symbol = 0 as *mut core::ffi::c_ushort;
    (*store).ll_counts = 0 as *mut size_t;
    (*store).d_counts = 0 as *mut size_t;
}
pub unsafe extern "C" fn ZopfliCleanLZ77Store(mut store: *mut ZopfliLZ77Store) {
    free((*store).litlens as *mut core::ffi::c_void);
    free((*store).dists as *mut core::ffi::c_void);
    free((*store).pos as *mut core::ffi::c_void);
    free((*store).ll_symbol as *mut core::ffi::c_void);
    free((*store).d_symbol as *mut core::ffi::c_void);
    free((*store).ll_counts as *mut core::ffi::c_void);
    free((*store).d_counts as *mut core::ffi::c_void);
}
pub unsafe extern "C" fn CeilDiv(mut a: size_t, mut b: size_t) -> size_t {
    return a.wrapping_add(b).wrapping_sub(1 as size_t).wrapping_div(b);
}
pub unsafe extern "C" fn ZopfliCopyLZ77Store(
    mut source: *const ZopfliLZ77Store,
    mut dest: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut llsize: size_t = (288 as size_t)
        .wrapping_mul(CeilDiv((*source).size, 288 as size_t));
    let mut dsize: size_t = (32 as size_t)
        .wrapping_mul(CeilDiv((*source).size, 32 as size_t));
    ZopfliCleanLZ77Store(dest);
    ZopfliInitLZ77Store((*source).data, dest);
    (*dest).litlens = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul((*source).size),
    ) as *mut core::ffi::c_ushort;
    (*dest).dists = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul((*source).size),
    ) as *mut core::ffi::c_ushort;
    (*dest).pos = malloc(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul((*source).size),
    ) as *mut size_t;
    (*dest).ll_symbol = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul((*source).size),
    ) as *mut core::ffi::c_ushort;
    (*dest).d_symbol = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul((*source).size),
    ) as *mut core::ffi::c_ushort;
    (*dest).ll_counts = malloc(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(llsize),
    ) as *mut size_t;
    (*dest).d_counts = malloc(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(dsize),
    ) as *mut size_t;
    if ((*dest).litlens).is_null() || ((*dest).dists).is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    if ((*dest).pos).is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    if ((*dest).ll_symbol).is_null() || ((*dest).d_symbol).is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    if ((*dest).ll_counts).is_null() || ((*dest).d_counts).is_null() {
        exit(-(1 as core::ffi::c_int));
    }
    (*dest).size = (*source).size;
    i = 0 as size_t;
    while i < (*source).size {
        *((*dest).litlens).offset(i as isize) = *((*source).litlens).offset(i as isize);
        *((*dest).dists).offset(i as isize) = *((*source).dists).offset(i as isize);
        *((*dest).pos).offset(i as isize) = *((*source).pos).offset(i as isize);
        *((*dest).ll_symbol).offset(i as isize) = *((*source).ll_symbol)
            .offset(i as isize);
        *((*dest).d_symbol).offset(i as isize) = *((*source).d_symbol)
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < llsize {
        *((*dest).ll_counts).offset(i as isize) = *((*source).ll_counts)
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < dsize {
        *((*dest).d_counts).offset(i as isize) = *((*source).d_counts)
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn ZopfliGetDistSymbol(
    mut dist: core::ffi::c_int,
) -> core::ffi::c_int {
    if dist < 5 as core::ffi::c_int {
        return dist - 1 as core::ffi::c_int
    } else {
        let mut l: core::ffi::c_int = 31 as core::ffi::c_int
            ^ ((dist - 1 as core::ffi::c_int) as core::ffi::c_uint).leading_zeros()
                as i32;
        let mut r: core::ffi::c_int = dist - 1 as core::ffi::c_int
            >> l - 1 as core::ffi::c_int & 1 as core::ffi::c_int;
        return l * 2 as core::ffi::c_int + r;
    };
}
pub unsafe extern "C" fn ZopfliStoreLitLenDist(
    mut length: core::ffi::c_ushort,
    mut dist: core::ffi::c_ushort,
    mut pos: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut origsize: size_t = (*store).size;
    let mut llstart: size_t = (288 as size_t)
        .wrapping_mul(origsize.wrapping_div(288 as size_t));
    let mut dstart: size_t = (32 as size_t)
        .wrapping_mul(origsize.wrapping_div(32 as size_t));
    if (origsize as core::ffi::c_ulong).wrapping_rem(288 as core::ffi::c_ulong)
        == 0 as core::ffi::c_ulong
    {
        let mut llsize: size_t = origsize;
        i = 0 as size_t;
        while i < 288 as core::ffi::c_ulong {
            if llsize & llsize.wrapping_sub(1 as core::ffi::c_ulong) == 0 {
                (*store).ll_counts = (if llsize == 0 as core::ffi::c_ulong {
                    malloc(::core::mem::size_of::<size_t>() as size_t)
                } else {
                    realloc(
                        (*store).ll_counts as *mut core::ffi::c_void,
                        llsize
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                    )
                }) as *mut size_t;
            }
            *((*store).ll_counts).offset(llsize as isize) = (if origsize
                == 0 as core::ffi::c_ulong
            {
                0 as core::ffi::c_ulong
            } else {
                *((*store).ll_counts)
                    .offset(
                        origsize.wrapping_sub(288 as size_t).wrapping_add(i) as isize,
                    )
            }) as size_t;
            llsize = llsize.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
    if (origsize as core::ffi::c_ulong).wrapping_rem(32 as core::ffi::c_ulong)
        == 0 as core::ffi::c_ulong
    {
        let mut dsize: size_t = origsize;
        i = 0 as size_t;
        while i < 32 as core::ffi::c_ulong {
            if dsize & dsize.wrapping_sub(1 as core::ffi::c_ulong) == 0 {
                (*store).d_counts = (if dsize == 0 as core::ffi::c_ulong {
                    malloc(::core::mem::size_of::<size_t>() as size_t)
                } else {
                    realloc(
                        (*store).d_counts as *mut core::ffi::c_void,
                        dsize
                            .wrapping_mul(2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                    )
                }) as *mut size_t;
            }
            *((*store).d_counts).offset(dsize as isize) = (if origsize
                == 0 as core::ffi::c_ulong
            {
                0 as core::ffi::c_ulong
            } else {
                *((*store).d_counts)
                    .offset(origsize.wrapping_sub(32 as size_t).wrapping_add(i) as isize)
            }) as size_t;
            dsize = dsize.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
    if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        (*store).litlens = (if (*store).size == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
        } else {
            realloc(
                (*store).litlens as *mut core::ffi::c_void,
                ((*store).size)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(
                        ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                    ),
            )
        }) as *mut core::ffi::c_ushort;
    }
    *((*store).litlens).offset((*store).size as isize) = length;
    (*store).size = ((*store).size).wrapping_add(1);
    (*store).size = origsize;
    if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        (*store).dists = (if (*store).size == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
        } else {
            realloc(
                (*store).dists as *mut core::ffi::c_void,
                ((*store).size)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(
                        ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                    ),
            )
        }) as *mut core::ffi::c_ushort;
    }
    *((*store).dists).offset((*store).size as isize) = dist;
    (*store).size = ((*store).size).wrapping_add(1);
    (*store).size = origsize;
    if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
        (*store).pos = (if (*store).size == 0 as core::ffi::c_ulong {
            malloc(::core::mem::size_of::<size_t>() as size_t)
        } else {
            realloc(
                (*store).pos as *mut core::ffi::c_void,
                ((*store).size)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
            )
        }) as *mut size_t;
    }
    *((*store).pos).offset((*store).size as isize) = pos;
    (*store).size = ((*store).size).wrapping_add(1);
    if (length as core::ffi::c_int) < 259 as core::ffi::c_int {} else {
        __assert_fail(
            b"length < 259\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            131 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 86],
                [core::ffi::c_char; 86],
            >(
                *b"void ZopfliStoreLitLenDist(unsigned short, unsigned short, size_t, ZopfliLZ77Store *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3094: {
        if (length as core::ffi::c_int) < 259 as core::ffi::c_int {} else {
            __assert_fail(
                b"length < 259\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                131 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 86],
                    [core::ffi::c_char; 86],
                >(
                    *b"void ZopfliStoreLitLenDist(unsigned short, unsigned short, size_t, ZopfliLZ77Store *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dist as core::ffi::c_int == 0 as core::ffi::c_int {
        (*store).size = origsize;
        if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
            (*store).ll_symbol = (if (*store).size == 0 as core::ffi::c_ulong {
                malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            } else {
                realloc(
                    (*store).ll_symbol as *mut core::ffi::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                        ),
                )
            }) as *mut core::ffi::c_ushort;
        }
        *((*store).ll_symbol).offset((*store).size as isize) = length;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size = origsize;
        if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
            (*store).d_symbol = (if (*store).size == 0 as core::ffi::c_ulong {
                malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            } else {
                realloc(
                    (*store).d_symbol as *mut core::ffi::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                        ),
                )
            }) as *mut core::ffi::c_ushort;
        }
        *((*store).d_symbol).offset((*store).size as isize) = 0 as core::ffi::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        let ref mut fresh0 = *((*store).ll_counts)
            .offset(
                (llstart as core::ffi::c_ulong)
                    .wrapping_add(length as core::ffi::c_ulong) as isize,
            );
        *fresh0 = (*fresh0).wrapping_add(1);
    } else {
        (*store).size = origsize;
        if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
            (*store).ll_symbol = (if (*store).size == 0 as core::ffi::c_ulong {
                malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            } else {
                realloc(
                    (*store).ll_symbol as *mut core::ffi::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                        ),
                )
            }) as *mut core::ffi::c_ushort;
        }
        *((*store).ll_symbol).offset((*store).size as isize) = ZopfliGetLengthSymbol(
            length as core::ffi::c_int,
        ) as core::ffi::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size = origsize;
        if (*store).size & ((*store).size).wrapping_sub(1 as core::ffi::c_ulong) == 0 {
            (*store).d_symbol = (if (*store).size == 0 as core::ffi::c_ulong {
                malloc(::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            } else {
                realloc(
                    (*store).d_symbol as *mut core::ffi::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_ushort>() as size_t,
                        ),
                )
            }) as *mut core::ffi::c_ushort;
        }
        *((*store).d_symbol).offset((*store).size as isize) = ZopfliGetDistSymbol(
            dist as core::ffi::c_int,
        ) as core::ffi::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        let ref mut fresh1 = *((*store).ll_counts)
            .offset(
                (llstart as core::ffi::c_ulong)
                    .wrapping_add(
                        ZopfliGetLengthSymbol(length as core::ffi::c_int)
                            as core::ffi::c_ulong,
                    ) as isize,
            );
        *fresh1 = (*fresh1).wrapping_add(1);
        let ref mut fresh2 = *((*store).d_counts)
            .offset(
                (dstart as core::ffi::c_ulong)
                    .wrapping_add(
                        ZopfliGetDistSymbol(dist as core::ffi::c_int)
                            as core::ffi::c_ulong,
                    ) as isize,
            );
        *fresh2 = (*fresh2).wrapping_add(1);
    };
}
pub unsafe extern "C" fn ZopfliAppendLZ77Store(
    mut store: *const ZopfliLZ77Store,
    mut target: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*store).size {
        ZopfliStoreLitLenDist(
            *((*store).litlens).offset(i as isize),
            *((*store).dists).offset(i as isize),
            *((*store).pos).offset(i as isize),
            target,
        );
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn ZopfliLZ77GetByteRange(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let mut l: size_t = lend.wrapping_sub(1 as size_t);
    if lstart == lend {
        return 0 as size_t;
    }
    return (*((*lz77).pos).offset(l as isize))
        .wrapping_add(
            (if *((*lz77).dists).offset(l as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                1 as core::ffi::c_int
            } else {
                *((*lz77).litlens).offset(l as isize) as core::ffi::c_int
            }) as size_t,
        )
        .wrapping_sub(*((*lz77).pos).offset(lstart as isize));
}
pub unsafe extern "C" fn ZopfliLZ77GetHistogramAt(
    mut lz77: *const ZopfliLZ77Store,
    mut lpos: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let mut llpos: size_t = (288 as size_t)
        .wrapping_mul(lpos.wrapping_div(288 as size_t));
    let mut dpos: size_t = (32 as size_t).wrapping_mul(lpos.wrapping_div(32 as size_t));
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 288 as core::ffi::c_ulong {
        *ll_counts.offset(i as isize) = *((*lz77).ll_counts)
            .offset(llpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
    }
    i = (lpos as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong) as size_t;
    while i < (llpos as core::ffi::c_ulong).wrapping_add(288 as core::ffi::c_ulong)
        && i < (*lz77).size
    {
        let ref mut fresh7 = *ll_counts
            .offset(*((*lz77).ll_symbol).offset(i as isize) as isize);
        *fresh7 = (*fresh7).wrapping_sub(1);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 32 as core::ffi::c_ulong {
        *d_counts.offset(i as isize) = *((*lz77).d_counts)
            .offset(dpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
    }
    i = (lpos as core::ffi::c_ulong).wrapping_add(1 as core::ffi::c_ulong) as size_t;
    while i < (dpos as core::ffi::c_ulong).wrapping_add(32 as core::ffi::c_ulong)
        && i < (*lz77).size
    {
        if *((*lz77).dists).offset(i as isize) as core::ffi::c_int
            != 0 as core::ffi::c_int
        {
            let ref mut fresh8 = *d_counts
                .offset(*((*lz77).d_symbol).offset(i as isize) as isize);
            *fresh8 = (*fresh8).wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn ZopfliGetLengthSymbol(mut l: core::ffi::c_int) -> core::ffi::c_int {
    static mut table: [core::ffi::c_int; 259] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        257 as core::ffi::c_int,
        258 as core::ffi::c_int,
        259 as core::ffi::c_int,
        260 as core::ffi::c_int,
        261 as core::ffi::c_int,
        262 as core::ffi::c_int,
        263 as core::ffi::c_int,
        264 as core::ffi::c_int,
        265 as core::ffi::c_int,
        265 as core::ffi::c_int,
        266 as core::ffi::c_int,
        266 as core::ffi::c_int,
        267 as core::ffi::c_int,
        267 as core::ffi::c_int,
        268 as core::ffi::c_int,
        268 as core::ffi::c_int,
        269 as core::ffi::c_int,
        269 as core::ffi::c_int,
        269 as core::ffi::c_int,
        269 as core::ffi::c_int,
        270 as core::ffi::c_int,
        270 as core::ffi::c_int,
        270 as core::ffi::c_int,
        270 as core::ffi::c_int,
        271 as core::ffi::c_int,
        271 as core::ffi::c_int,
        271 as core::ffi::c_int,
        271 as core::ffi::c_int,
        272 as core::ffi::c_int,
        272 as core::ffi::c_int,
        272 as core::ffi::c_int,
        272 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        273 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        274 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        275 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        276 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        277 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        278 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        279 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        280 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        281 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        282 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        283 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        284 as core::ffi::c_int,
        285 as core::ffi::c_int,
    ];
    return table[l as usize];
}
pub unsafe extern "C" fn ZopfliLZ77GetHistogram(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let mut i: size_t = 0;
    if (lstart as core::ffi::c_ulong)
        .wrapping_add(
            (288 as core::ffi::c_int * 3 as core::ffi::c_int) as core::ffi::c_ulong,
        ) > lend
    {
        memset(
            ll_counts as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(288 as size_t),
        );
        memset(
            d_counts as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(32 as size_t),
        );
        i = lstart;
        while i < lend {
            let ref mut fresh3 = *ll_counts
                .offset(*((*lz77).ll_symbol).offset(i as isize) as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            if *((*lz77).dists).offset(i as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int
            {
                let ref mut fresh4 = *d_counts
                    .offset(*((*lz77).d_symbol).offset(i as isize) as isize);
                *fresh4 = (*fresh4).wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        ZopfliLZ77GetHistogramAt(
            lz77,
            lend.wrapping_sub(1 as size_t),
            ll_counts,
            d_counts,
        );
        if lstart > 0 as core::ffi::c_ulong {
            let mut ll_counts2: [size_t; 288] = [0; 288];
            let mut d_counts2: [size_t; 32] = [0; 32];
            ZopfliLZ77GetHistogramAt(
                lz77,
                lstart.wrapping_sub(1 as size_t),
                ll_counts2.as_mut_ptr(),
                d_counts2.as_mut_ptr(),
            );
            i = 0 as size_t;
            while i < 288 as core::ffi::c_ulong {
                let ref mut fresh5 = *ll_counts.offset(i as isize);
                *fresh5 = (*fresh5 as core::ffi::c_ulong)
                    .wrapping_sub(ll_counts2[i as usize] as core::ffi::c_ulong) as size_t
                    as size_t;
                i = i.wrapping_add(1);
            }
            i = 0 as size_t;
            while i < 32 as core::ffi::c_ulong {
                let ref mut fresh6 = *d_counts.offset(i as isize);
                *fresh6 = (*fresh6 as core::ffi::c_ulong)
                    .wrapping_sub(d_counts2[i as usize] as core::ffi::c_ulong) as size_t
                    as size_t;
                i = i.wrapping_add(1);
            }
        }
    };
}
pub unsafe extern "C" fn ZopfliInitBlockState(
    mut options: *const ZopfliOptions,
    mut blockstart: size_t,
    mut blockend: size_t,
    mut add_lmc: core::ffi::c_int,
    mut s: *mut ZopfliBlockState,
) {
    (*s).options = options;
    (*s).blockstart = blockstart;
    (*s).blockend = blockend;
    if add_lmc != 0 {
        (*s).lmc = malloc(::core::mem::size_of::<ZopfliLongestMatchCache>() as size_t)
            as *mut ZopfliLongestMatchCache;
        ZopfliInitCache(blockend.wrapping_sub(blockstart), (*s).lmc);
    } else {
        (*s).lmc = 0 as *mut ZopfliLongestMatchCache;
    };
}
pub unsafe extern "C" fn ZopfliCleanBlockState(mut s: *mut ZopfliBlockState) {
    if !((*s).lmc).is_null() {
        ZopfliCleanCache((*s).lmc);
        free((*s).lmc as *mut core::ffi::c_void);
    }
}
pub unsafe extern "C" fn GetLengthScore(
    mut length: core::ffi::c_int,
    mut distance: core::ffi::c_int,
) -> core::ffi::c_int {
    return if distance > 1024 as core::ffi::c_int {
        length - 1 as core::ffi::c_int
    } else {
        length
    };
}
pub unsafe extern "C" fn ZopfliVerifyLenDist(
    mut data: *const core::ffi::c_uchar,
    mut datasize: size_t,
    mut pos: size_t,
    mut dist: core::ffi::c_ushort,
    mut length: core::ffi::c_ushort,
) {
    let mut i: size_t = 0;
    if (pos as core::ffi::c_ulong).wrapping_add(length as core::ffi::c_ulong) <= datasize
    {} else {
        __assert_fail(
            b"pos + length <= datasize\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            279 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 96],
                [core::ffi::c_char; 96],
            >(
                *b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6644: {
        if (pos as core::ffi::c_ulong).wrapping_add(length as core::ffi::c_ulong)
            <= datasize
        {} else {
            __assert_fail(
                b"pos + length <= datasize\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                279 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 96],
                    [core::ffi::c_char; 96],
                >(
                    *b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as size_t;
    while i < length as core::ffi::c_ulong {
        if *data.offset(pos.wrapping_sub(dist as size_t).wrapping_add(i) as isize)
            as core::ffi::c_int
            != *data.offset(pos.wrapping_add(i) as isize) as core::ffi::c_int
        {
            if *data.offset(pos.wrapping_sub(dist as size_t).wrapping_add(i) as isize)
                as core::ffi::c_int
                == *data.offset(pos.wrapping_add(i) as isize) as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"data[pos - dist + i] == data[pos + i]\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    282 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 96],
                        [core::ffi::c_char; 96],
                    >(
                        *b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6529: {
                if *data
                    .offset(pos.wrapping_sub(dist as size_t).wrapping_add(i) as isize)
                    as core::ffi::c_int
                    == *data.offset(pos.wrapping_add(i) as isize) as core::ffi::c_int
                {} else {
                    __assert_fail(
                        b"data[pos - dist + i] == data[pos + i]\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        282 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 96],
                            [core::ffi::c_char; 96],
                        >(
                            *b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
pub unsafe extern "C" fn GetMatch(
    mut scan: *const core::ffi::c_uchar,
    mut match_0: *const core::ffi::c_uchar,
    mut end: *const core::ffi::c_uchar,
    mut safe_end: *const core::ffi::c_uchar,
) -> *const core::ffi::c_uchar {
    if ::core::mem::size_of::<size_t>() as usize == 8 as usize {
        while scan < safe_end && *(scan as *mut size_t) == *(match_0 as *mut size_t) {
            scan = scan.offset(8 as core::ffi::c_int as isize);
            match_0 = match_0.offset(8 as core::ffi::c_int as isize);
        }
    } else if ::core::mem::size_of::<core::ffi::c_uint>() as usize == 4 as usize {
        while scan < safe_end
            && *(scan as *mut core::ffi::c_uint) == *(match_0 as *mut core::ffi::c_uint)
        {
            scan = scan.offset(4 as core::ffi::c_int as isize);
            match_0 = match_0.offset(4 as core::ffi::c_int as isize);
        }
    } else {
        while scan < safe_end
            && *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
            }
        {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
        }
    }
    while scan != end && *scan as core::ffi::c_int == *match_0 as core::ffi::c_int {
        scan = scan.offset(1);
        match_0 = match_0.offset(1);
    }
    return scan;
}
pub unsafe extern "C" fn TryGetFromLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: *mut size_t,
    mut sublen: *mut core::ffi::c_ushort,
    mut distance: *mut core::ffi::c_ushort,
    mut length: *mut core::ffi::c_ushort,
) -> core::ffi::c_int {
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    let mut cache_available: core::ffi::c_uchar = (!((*s).lmc).is_null()
        && (*((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
            == 0 as core::ffi::c_int
            || *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int)) as core::ffi::c_int as core::ffi::c_uchar;
    let mut limit_ok_for_cache: core::ffi::c_uchar = (cache_available as core::ffi::c_int
        != 0
        && (*limit == 258 as core::ffi::c_ulong
            || *((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_ulong
                <= *limit
            || !sublen.is_null()
                && ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *((*(*s).lmc).length).offset(lmcpos as isize) as size_t,
                ) as core::ffi::c_ulong >= *limit)) as core::ffi::c_int
        as core::ffi::c_uchar;
    if !((*s).lmc).is_null() && limit_ok_for_cache as core::ffi::c_int != 0
        && cache_available as core::ffi::c_int != 0
    {
        if sublen.is_null()
            || *((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_uint
                <= ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *((*(*s).lmc).length).offset(lmcpos as isize) as size_t,
                )
        {
            *length = *((*(*s).lmc).length).offset(lmcpos as isize);
            if *length as core::ffi::c_ulong > *limit {
                *length = *limit as core::ffi::c_ushort;
            }
            if !sublen.is_null() {
                ZopfliCacheToSublen((*s).lmc, lmcpos, *length as size_t, sublen);
                *distance = *sublen.offset(*length as isize);
                if *limit == 258 as core::ffi::c_ulong
                    && *length as core::ffi::c_int >= 3 as core::ffi::c_int
                {
                    if *sublen.offset(*length as isize) as core::ffi::c_int
                        == *((*(*s).lmc).dist).offset(lmcpos as isize)
                            as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"sublen[*length] == s->lmc->dist[lmcpos]\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            365 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 124],
                                [core::ffi::c_char; 124],
                            >(
                                *b"int TryGetFromLongestMatchCache(ZopfliBlockState *, size_t, size_t *, unsigned short *, unsigned short *, unsigned short *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6211: {
                        if *sublen.offset(*length as isize) as core::ffi::c_int
                            == *((*(*s).lmc).dist).offset(lmcpos as isize)
                                as core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"sublen[*length] == s->lmc->dist[lmcpos]\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                365 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 124],
                                    [core::ffi::c_char; 124],
                                >(
                                    *b"int TryGetFromLongestMatchCache(ZopfliBlockState *, size_t, size_t *, unsigned short *, unsigned short *, unsigned short *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
            } else {
                *distance = *((*(*s).lmc).dist).offset(lmcpos as isize);
            }
            return 1 as core::ffi::c_int;
        }
        *limit = *((*(*s).lmc).length).offset(lmcpos as isize) as size_t;
    }
    return 0 as core::ffi::c_int;
}
pub unsafe extern "C" fn StoreInLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: size_t,
    mut sublen: *const core::ffi::c_ushort,
    mut distance: core::ffi::c_ushort,
    mut length: core::ffi::c_ushort,
) {
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    let mut cache_available: core::ffi::c_uchar = (!((*s).lmc).is_null()
        && (*((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
            == 0 as core::ffi::c_int
            || *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                != 0 as core::ffi::c_int)) as core::ffi::c_int as core::ffi::c_uchar;
    if !((*s).lmc).is_null() && limit == 258 as core::ffi::c_ulong && !sublen.is_null()
        && cache_available == 0
    {
        if *((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
            == 1 as core::ffi::c_int
            && *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                398 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 122],
                    [core::ffi::c_char; 122],
                >(
                    *b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4770: {
            if *((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
                == 1 as core::ffi::c_int
                && *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                    == 0 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    398 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 122],
                        [core::ffi::c_char; 122],
                    >(
                        *b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *((*(*s).lmc).dist).offset(lmcpos as isize) = (if (length as core::ffi::c_int)
            < 3 as core::ffi::c_int
        {
            0 as core::ffi::c_int
        } else {
            distance as core::ffi::c_int
        }) as core::ffi::c_ushort;
        *((*(*s).lmc).length).offset(lmcpos as isize) = (if (length as core::ffi::c_int)
            < 3 as core::ffi::c_int
        {
            0 as core::ffi::c_int
        } else {
            length as core::ffi::c_int
        }) as core::ffi::c_ushort;
        if !(*((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
            == 1 as core::ffi::c_int
            && *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                == 0 as core::ffi::c_int)
        {} else {
            __assert_fail(
                b"!(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                401 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 122],
                    [core::ffi::c_char; 122],
                >(
                    *b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4640: {
            if !(*((*(*s).lmc).length).offset(lmcpos as isize) as core::ffi::c_int
                == 1 as core::ffi::c_int
                && *((*(*s).lmc).dist).offset(lmcpos as isize) as core::ffi::c_int
                    == 0 as core::ffi::c_int)
            {} else {
                __assert_fail(
                    b"!(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    401 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 122],
                        [core::ffi::c_char; 122],
                    >(
                        *b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        ZopfliSublenToCache(sublen, lmcpos, length as size_t, (*s).lmc);
    }
}
pub unsafe extern "C" fn ZopfliFindLongestMatch(
    mut s: *mut ZopfliBlockState,
    mut h: *const ZopfliHash,
    mut array: *const core::ffi::c_uchar,
    mut pos: size_t,
    mut size: size_t,
    mut limit: size_t,
    mut sublen: *mut core::ffi::c_ushort,
    mut distance: *mut core::ffi::c_ushort,
    mut length: *mut core::ffi::c_ushort,
) {
    let mut hpos: core::ffi::c_ushort = (pos as core::ffi::c_ulong
        & (32768 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_ulong)
        as core::ffi::c_ushort;
    let mut p: core::ffi::c_ushort = 0;
    let mut pp: core::ffi::c_ushort = 0;
    let mut bestdist: core::ffi::c_ushort = 0 as core::ffi::c_ushort;
    let mut bestlength: core::ffi::c_ushort = 1 as core::ffi::c_ushort;
    let mut scan: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut match_0: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut arrayend: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut arrayend_safe: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut chain_counter: core::ffi::c_int = 8192 as core::ffi::c_int;
    let mut dist: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut hhead: *mut core::ffi::c_int = (*h).head;
    let mut hprev: *mut core::ffi::c_ushort = (*h).prev;
    let mut hhashval: *mut core::ffi::c_int = (*h).hashval;
    let mut hval: core::ffi::c_int = (*h).val;
    if TryGetFromLongestMatchCache(s, pos, &mut limit, sublen, distance, length) != 0 {
        if (pos as core::ffi::c_ulong).wrapping_add(*length as core::ffi::c_ulong)
            <= size
        {} else {
            __assert_fail(
                b"pos + *length <= size\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                431 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6072: {
            if (pos as core::ffi::c_ulong).wrapping_add(*length as core::ffi::c_ulong)
                <= size
            {} else {
                __assert_fail(
                    b"pos + *length <= size\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    431 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return;
    }
    if limit <= 258 as core::ffi::c_ulong {} else {
        __assert_fail(
            b"limit <= ZOPFLI_MAX_MATCH\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            436 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6030: {
        if limit <= 258 as core::ffi::c_ulong {} else {
            __assert_fail(
                b"limit <= ZOPFLI_MAX_MATCH\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                436 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if limit >= 3 as core::ffi::c_ulong {} else {
        __assert_fail(
            b"limit >= ZOPFLI_MIN_MATCH\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            437 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5991: {
        if limit >= 3 as core::ffi::c_ulong {} else {
            __assert_fail(
                b"limit >= ZOPFLI_MIN_MATCH\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                437 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if pos < size {} else {
        __assert_fail(
            b"pos < size\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            438 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5953: {
        if pos < size {} else {
            __assert_fail(
                b"pos < size\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                438 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if size.wrapping_sub(pos) < 3 as core::ffi::c_ulong {
        *length = 0 as core::ffi::c_ushort;
        *distance = 0 as core::ffi::c_ushort;
        return;
    }
    if pos.wrapping_add(limit) > size {
        limit = size.wrapping_sub(pos);
    }
    arrayend = (&*array.offset(pos as isize) as *const core::ffi::c_uchar)
        .offset(limit as isize);
    arrayend_safe = arrayend.offset(-(8 as core::ffi::c_int as isize));
    if hval < 65536 as core::ffi::c_int {} else {
        __assert_fail(
            b"hval < 65536\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            454 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5859: {
        if hval < 65536 as core::ffi::c_int {} else {
            __assert_fail(
                b"hval < 65536\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                454 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    pp = *hhead.offset(hval as isize) as core::ffi::c_ushort;
    p = *hprev.offset(pp as isize);
    if pp as core::ffi::c_int == hpos as core::ffi::c_int {} else {
        __assert_fail(
            b"pp == hpos\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            459 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5800: {
        if pp as core::ffi::c_int == hpos as core::ffi::c_int {} else {
            __assert_fail(
                b"pp == hpos\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                459 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    dist = (if (p as core::ffi::c_int) < pp as core::ffi::c_int {
        pp as core::ffi::c_int - p as core::ffi::c_int
    } else {
        32768 as core::ffi::c_int - p as core::ffi::c_int + pp as core::ffi::c_int
    }) as core::ffi::c_uint;
    while dist < 32768 as core::ffi::c_uint {
        let mut currentlength: core::ffi::c_ushort = 0 as core::ffi::c_ushort;
        if (p as core::ffi::c_int) < 32768 as core::ffi::c_int {} else {
            __assert_fail(
                b"p < ZOPFLI_WINDOW_SIZE\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                467 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5728: {
            if (p as core::ffi::c_int) < 32768 as core::ffi::c_int {} else {
                __assert_fail(
                    b"p < ZOPFLI_WINDOW_SIZE\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    467 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if p as core::ffi::c_int == *hprev.offset(pp as isize) as core::ffi::c_int
        {} else {
            __assert_fail(
                b"p == hprev[pp]\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                468 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5677: {
            if p as core::ffi::c_int == *hprev.offset(pp as isize) as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"p == hprev[pp]\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    468 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if *hhashval.offset(p as isize) == hval {} else {
            __assert_fail(
                b"hhashval[p] == hval\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                469 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5631: {
            if *hhashval.offset(p as isize) == hval {} else {
                __assert_fail(
                    b"hhashval[p] == hval\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    469 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if dist > 0 as core::ffi::c_uint {
            if pos < size {} else {
                __assert_fail(
                    b"pos < size\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    472 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5588: {
                if pos < size {} else {
                    __assert_fail(
                        b"pos < size\0" as *const u8 as *const core::ffi::c_char,
                        b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        472 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 169],
                            [core::ffi::c_char; 169],
                        >(
                            *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if dist as core::ffi::c_ulong <= pos {} else {
                __assert_fail(
                    b"dist <= pos\0" as *const u8 as *const core::ffi::c_char,
                    b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    473 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 169],
                        [core::ffi::c_char; 169],
                    >(
                        *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5547: {
                if dist as core::ffi::c_ulong <= pos {} else {
                    __assert_fail(
                        b"dist <= pos\0" as *const u8 as *const core::ffi::c_char,
                        b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        473 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 169],
                            [core::ffi::c_char; 169],
                        >(
                            *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            scan = &*array.offset(pos as isize) as *const core::ffi::c_uchar;
            match_0 = &*array
                .offset(
                    (pos as core::ffi::c_ulong).wrapping_sub(dist as core::ffi::c_ulong)
                        as isize,
                ) as *const core::ffi::c_uchar;
            if (pos as core::ffi::c_ulong).wrapping_add(bestlength as core::ffi::c_ulong)
                >= size
                || *scan.offset(bestlength as core::ffi::c_int as isize)
                    as core::ffi::c_int
                    == *match_0.offset(bestlength as core::ffi::c_int as isize)
                        as core::ffi::c_int
            {
                let mut same0: core::ffi::c_ushort = *((*h).same)
                    .offset(
                        (pos as core::ffi::c_ulong
                            & (32768 as core::ffi::c_int - 1 as core::ffi::c_int)
                                as core::ffi::c_ulong) as isize,
                    );
                if same0 as core::ffi::c_int > 2 as core::ffi::c_int
                    && *scan as core::ffi::c_int == *match_0 as core::ffi::c_int
                {
                    let mut same1: core::ffi::c_ushort = *((*h).same)
                        .offset(
                            ((pos as core::ffi::c_ulong)
                                .wrapping_sub(dist as core::ffi::c_ulong)
                                & (32768 as core::ffi::c_int - 1 as core::ffi::c_int)
                                    as core::ffi::c_ulong) as isize,
                        );
                    let mut same: core::ffi::c_ushort = (if (same0 as core::ffi::c_int)
                        < same1 as core::ffi::c_int
                    {
                        same0 as core::ffi::c_int
                    } else {
                        same1 as core::ffi::c_int
                    }) as core::ffi::c_ushort;
                    if same as core::ffi::c_ulong > limit {
                        same = limit as core::ffi::c_ushort;
                    }
                    scan = scan.offset(same as core::ffi::c_int as isize);
                    match_0 = match_0.offset(same as core::ffi::c_int as isize);
                }
                scan = GetMatch(scan, match_0, arrayend, arrayend_safe);
                currentlength = scan
                    .offset_from(
                        &*array.offset(pos as isize) as *const core::ffi::c_uchar,
                    ) as core::ffi::c_long as core::ffi::c_ushort;
            }
            if currentlength as core::ffi::c_int > bestlength as core::ffi::c_int {
                if !sublen.is_null() {
                    let mut j: core::ffi::c_ushort = 0;
                    j = (bestlength as core::ffi::c_int + 1 as core::ffi::c_int)
                        as core::ffi::c_ushort;
                    while j as core::ffi::c_int <= currentlength as core::ffi::c_int {
                        *sublen.offset(j as isize) = dist as core::ffi::c_ushort;
                        j = j.wrapping_add(1);
                    }
                }
                bestdist = dist as core::ffi::c_ushort;
                bestlength = currentlength;
                if currentlength as core::ffi::c_ulong >= limit {
                    break;
                }
            }
        }
        if hhead != (*h).head2
            && bestlength as core::ffi::c_int
                >= *((*h).same).offset(hpos as isize) as core::ffi::c_int
            && (*h).val2 == *((*h).hashval2).offset(p as isize)
        {
            hhead = (*h).head2;
            hprev = (*h).prev2;
            hhashval = (*h).hashval2;
            hval = (*h).val2;
        }
        pp = p;
        p = *hprev.offset(p as isize);
        if p as core::ffi::c_int == pp as core::ffi::c_int {
            break;
        }
        dist = dist
            .wrapping_add(
                (if (p as core::ffi::c_int) < pp as core::ffi::c_int {
                    pp as core::ffi::c_int - p as core::ffi::c_int
                } else {
                    32768 as core::ffi::c_int - p as core::ffi::c_int
                        + pp as core::ffi::c_int
                }) as core::ffi::c_uint,
            );
        chain_counter -= 1;
        if chain_counter <= 0 as core::ffi::c_int {
            break;
        }
    }
    StoreInLongestMatchCache(s, pos, limit, sublen, bestdist, bestlength);
    if bestlength as core::ffi::c_ulong <= limit {} else {
        __assert_fail(
            b"bestlength <= limit\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            537 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4539: {
        if bestlength as core::ffi::c_ulong <= limit {} else {
            __assert_fail(
                b"bestlength <= limit\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                537 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *distance = bestdist;
    *length = bestlength;
    if (pos as core::ffi::c_ulong).wrapping_add(*length as core::ffi::c_ulong) <= size
    {} else {
        __assert_fail(
            b"pos + *length <= size\0" as *const u8 as *const core::ffi::c_char,
            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                as *const u8 as *const core::ffi::c_char,
            541 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 169],
                [core::ffi::c_char; 169],
            >(
                *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4469: {
        if (pos as core::ffi::c_ulong).wrapping_add(*length as core::ffi::c_ulong)
            <= size
        {} else {
            __assert_fail(
                b"pos + *length <= size\0" as *const u8 as *const core::ffi::c_char,
                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                    as *const u8 as *const core::ffi::c_char,
                541 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn ZopfliLZ77Greedy(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const core::ffi::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0 as size_t;
    let mut j: size_t = 0;
    let mut leng: core::ffi::c_ushort = 0;
    let mut dist: core::ffi::c_ushort = 0;
    let mut lengthscore: core::ffi::c_int = 0;
    let mut windowstart: size_t = if instart > 32768 as core::ffi::c_ulong {
        instart.wrapping_sub(32768 as size_t)
    } else {
        0 as size_t
    };
    let mut dummysublen: [core::ffi::c_ushort; 259] = [0; 259];
    let mut prev_length: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut prev_match: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut prevlengthscore: core::ffi::c_int = 0;
    let mut match_available: core::ffi::c_int = 0 as core::ffi::c_int;
    if instart == inend {
        return;
    }
    ZopfliResetHash(32768 as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1);
    }
    let mut current_block_44: u64;
    i = instart;
    while i < inend {
        ZopfliUpdateHash(in_0, i, inend, h);
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            258 as size_t,
            dummysublen.as_mut_ptr(),
            &mut dist,
            &mut leng,
        );
        lengthscore = GetLengthScore(leng as core::ffi::c_int, dist as core::ffi::c_int);
        prevlengthscore = GetLengthScore(
            prev_length as core::ffi::c_int,
            prev_match as core::ffi::c_int,
        );
        if match_available != 0 {
            match_available = 0 as core::ffi::c_int;
            if lengthscore > prevlengthscore + 1 as core::ffi::c_int {
                ZopfliStoreLitLenDist(
                    *in_0
                        .offset(
                            (i as core::ffi::c_ulong)
                                .wrapping_sub(1 as core::ffi::c_ulong) as isize,
                        ) as core::ffi::c_ushort,
                    0 as core::ffi::c_ushort,
                    i.wrapping_sub(1 as size_t),
                    store,
                );
                if lengthscore >= 3 as core::ffi::c_int
                    && (leng as core::ffi::c_int) < 258 as core::ffi::c_int
                {
                    match_available = 1 as core::ffi::c_int;
                    prev_length = leng as core::ffi::c_uint;
                    prev_match = dist as core::ffi::c_uint;
                    current_block_44 = 11650488183268122163;
                } else {
                    current_block_44 = 8704759739624374314;
                }
            } else {
                leng = prev_length as core::ffi::c_ushort;
                dist = prev_match as core::ffi::c_ushort;
                lengthscore = prevlengthscore;
                ZopfliVerifyLenDist(
                    in_0,
                    inend,
                    i.wrapping_sub(1 as size_t),
                    dist,
                    leng,
                );
                ZopfliStoreLitLenDist(leng, dist, i.wrapping_sub(1 as size_t), store);
                j = 2 as size_t;
                while j < leng as core::ffi::c_ulong {
                    if i < inend {} else {
                        __assert_fail(
                            b"i < inend\0" as *const u8 as *const core::ffi::c_char,
                            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            600 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 114],
                                [core::ffi::c_char; 114],
                            >(
                                *b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6931: {
                        if i < inend {} else {
                            __assert_fail(
                                b"i < inend\0" as *const u8 as *const core::ffi::c_char,
                                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                600 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 114],
                                    [core::ffi::c_char; 114],
                                >(
                                    *b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                }
                current_block_44 = 11650488183268122163;
            }
        } else if lengthscore >= 3 as core::ffi::c_int
            && (leng as core::ffi::c_int) < 258 as core::ffi::c_int
        {
            match_available = 1 as core::ffi::c_int;
            prev_length = leng as core::ffi::c_uint;
            prev_match = dist as core::ffi::c_uint;
            current_block_44 = 11650488183268122163;
        } else {
            current_block_44 = 8704759739624374314;
        }
        match current_block_44 {
            8704759739624374314 => {
                if lengthscore >= 3 as core::ffi::c_int {
                    ZopfliVerifyLenDist(in_0, inend, i, dist, leng);
                    ZopfliStoreLitLenDist(leng, dist, i, store);
                } else {
                    leng = 1 as core::ffi::c_ushort;
                    ZopfliStoreLitLenDist(
                        *in_0.offset(i as isize) as core::ffi::c_ushort,
                        0 as core::ffi::c_ushort,
                        i,
                        store,
                    );
                }
                j = 1 as size_t;
                while j < leng as core::ffi::c_ulong {
                    if i < inend {} else {
                        __assert_fail(
                            b"i < inend\0" as *const u8 as *const core::ffi::c_char,
                            b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            625 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 114],
                                [core::ffi::c_char; 114],
                            >(
                                *b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6751: {
                        if i < inend {} else {
                            __assert_fail(
                                b"i < inend\0" as *const u8 as *const core::ffi::c_char,
                                b"/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/zopfli_c/src/lz77.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                625 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 114],
                                    [core::ffi::c_char; 114],
                                >(
                                    *b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}
