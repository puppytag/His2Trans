extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
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
pub const HASH_SHIFT: core::ffi::c_int = 5 as core::ffi::c_int;
pub const HASH_MASK: core::ffi::c_int = 32767 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ZopfliAllocHash(
    mut window_size: size_t,
    mut h: *mut ZopfliHash,
) {
    (*h).head = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(65536 as size_t),
    ) as *mut core::ffi::c_int;
    (*h).prev = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul(window_size),
    ) as *mut core::ffi::c_ushort;
    (*h).hashval = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t).wrapping_mul(window_size),
    ) as *mut core::ffi::c_int;
    (*h).same = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul(window_size),
    ) as *mut core::ffi::c_ushort;
    (*h).head2 = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(65536 as size_t),
    ) as *mut core::ffi::c_int;
    (*h).prev2 = malloc(
        (::core::mem::size_of::<core::ffi::c_ushort>() as size_t)
            .wrapping_mul(window_size),
    ) as *mut core::ffi::c_ushort;
    (*h).hashval2 = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t).wrapping_mul(window_size),
    ) as *mut core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliResetHash(
    mut window_size: size_t,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0;
    (*h).val = 0 as core::ffi::c_int;
    i = 0 as size_t;
    while i < 65536 as size_t {
        *((*h).head).offset(i as isize) = -(1 as core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *((*h).prev).offset(i as isize) = i as core::ffi::c_ushort;
        *((*h).hashval).offset(i as isize) = -(1 as core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *((*h).same).offset(i as isize) = 0 as core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    (*h).val2 = 0 as core::ffi::c_int;
    i = 0 as size_t;
    while i < 65536 as size_t {
        *((*h).head2).offset(i as isize) = -(1 as core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < window_size {
        *((*h).prev2).offset(i as isize) = i as core::ffi::c_ushort;
        *((*h).hashval2).offset(i as isize) = -(1 as core::ffi::c_int);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanHash(mut h: *mut ZopfliHash) {
    free((*h).head as *mut core::ffi::c_void);
    free((*h).prev as *mut core::ffi::c_void);
    free((*h).hashval as *mut core::ffi::c_void);
    free((*h).head2 as *mut core::ffi::c_void);
    free((*h).prev2 as *mut core::ffi::c_void);
    free((*h).hashval2 as *mut core::ffi::c_void);
    free((*h).same as *mut core::ffi::c_void);
}
unsafe extern "C" fn UpdateHashValue(mut h: *mut ZopfliHash, mut c: core::ffi::c_uchar) {
    (*h).val = ((*h).val << HASH_SHIFT ^ c as core::ffi::c_int) & HASH_MASK;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliUpdateHash(
    mut array: *const core::ffi::c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    let mut hpos: core::ffi::c_ushort = (pos & ZOPFLI_WINDOW_MASK as size_t)
        as core::ffi::c_ushort;
    let mut amount: size_t = 0 as size_t;
    UpdateHashValue(
        h,
        (if pos.wrapping_add(ZOPFLI_MIN_MATCH as size_t) <= end {
            *array
                .offset(
                    pos
                        .wrapping_add(ZOPFLI_MIN_MATCH as size_t)
                        .wrapping_sub(1 as size_t) as isize,
                ) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uchar,
    );
    *((*h).hashval).offset(hpos as isize) = (*h).val;
    if *((*h).head).offset((*h).val as isize) != -(1 as core::ffi::c_int)
        && *((*h).hashval).offset(*((*h).head).offset((*h).val as isize) as isize)
            == (*h).val
    {
        *((*h).prev).offset(hpos as isize) = *((*h).head).offset((*h).val as isize)
            as core::ffi::c_ushort;
    } else {
        *((*h).prev).offset(hpos as isize) = hpos;
    }
    *((*h).head).offset((*h).val as isize) = hpos as core::ffi::c_int;
    if *((*h).same)
        .offset((pos.wrapping_sub(1 as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize)
        as core::ffi::c_int > 1 as core::ffi::c_int
    {
        amount = (*((*h).same)
            .offset(
                (pos.wrapping_sub(1 as size_t) & ZOPFLI_WINDOW_MASK as size_t) as isize,
            ) as core::ffi::c_int - 1 as core::ffi::c_int) as size_t;
    }
    while pos.wrapping_add(amount).wrapping_add(1 as size_t) < end
        && *array.offset(pos as isize) as core::ffi::c_int
            == *array.offset(pos.wrapping_add(amount).wrapping_add(1 as size_t) as isize)
                as core::ffi::c_int
        && amount < -(1 as core::ffi::c_int) as core::ffi::c_ushort as size_t
    {
        amount = amount.wrapping_add(1);
    }
    *((*h).same).offset(hpos as isize) = amount as core::ffi::c_ushort;
    (*h).val2 = *((*h).same).offset(hpos as isize) as core::ffi::c_int - ZOPFLI_MIN_MATCH
        & 255 as core::ffi::c_int ^ (*h).val;
    *((*h).hashval2).offset(hpos as isize) = (*h).val2;
    if *((*h).head2).offset((*h).val2 as isize) != -(1 as core::ffi::c_int)
        && *((*h).hashval2).offset(*((*h).head2).offset((*h).val2 as isize) as isize)
            == (*h).val2
    {
        *((*h).prev2).offset(hpos as isize) = *((*h).head2).offset((*h).val2 as isize)
            as core::ffi::c_ushort;
    } else {
        *((*h).prev2).offset(hpos as isize) = hpos;
    }
    *((*h).head2).offset((*h).val2 as isize) = hpos as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliWarmupHash(
    mut array: *const core::ffi::c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    UpdateHashValue(h, *array.offset(pos.wrapping_add(0 as size_t) as isize));
    if pos.wrapping_add(1 as size_t) < end {
        UpdateHashValue(h, *array.offset(pos.wrapping_add(1 as size_t) as isize));
    }
}
pub const ZOPFLI_MIN_MATCH: core::ffi::c_int = 3 as core::ffi::c_int;
pub const ZOPFLI_WINDOW_SIZE: core::ffi::c_int = 32768 as core::ffi::c_int;
pub const ZOPFLI_WINDOW_MASK: core::ffi::c_int = ZOPFLI_WINDOW_SIZE
    - 1 as core::ffi::c_int;
