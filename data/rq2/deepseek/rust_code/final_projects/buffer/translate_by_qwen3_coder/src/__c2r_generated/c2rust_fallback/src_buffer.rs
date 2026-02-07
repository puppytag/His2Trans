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
    pub fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    pub fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    pub fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    pub fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
    pub fn strncat(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    pub fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn strstr(
        __haystack: *const core::ffi::c_char,
        __needle: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    pub fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn vsnprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> core::ffi::c_int;
    pub fn strlen(__s: *const core::ffi::c_char) -> size_t;
    pub fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    pub fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    pub fn free(__ptr: *mut core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = core::ffi::c_ulong;
pub type va_list = __builtin_va_list;
pub type __ssize_t = core::ffi::c_long;
pub type ssize_t = __ssize_t;
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
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut core::ffi::c_char,
    pub data: *mut core::ffi::c_char,
}
pub unsafe extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(64 as size_t);
}
pub unsafe extern "C" fn buffer_new_with_size(mut n: size_t) -> *mut buffer_t {
    let mut self_0: *mut buffer_t = malloc(::core::mem::size_of::<buffer_t>() as size_t)
        as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = n;
    (*self_0).alloc = calloc(n.wrapping_add(1 as size_t), 1 as size_t)
        as *mut core::ffi::c_char;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
pub unsafe extern "C" fn buffer_new_with_string(
    mut str: *mut core::ffi::c_char,
) -> *mut buffer_t {
    return buffer_new_with_string_length(str, strlen(str));
}
pub unsafe extern "C" fn buffer_new_with_string_length(
    mut str: *mut core::ffi::c_char,
    mut len: size_t,
) -> *mut buffer_t {
    let mut self_0: *mut buffer_t = malloc(::core::mem::size_of::<buffer_t>() as size_t)
        as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = len;
    (*self_0).alloc = str;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
pub unsafe extern "C" fn buffer_new_with_copy(
    mut str: *mut core::ffi::c_char,
) -> *mut buffer_t {
    let mut len: size_t = strlen(str);
    let mut self_0: *mut buffer_t = buffer_new_with_size(len);
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    memcpy(
        (*self_0).alloc as *mut core::ffi::c_void,
        str as *const core::ffi::c_void,
        len,
    );
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
pub unsafe extern "C" fn buffer_compact(mut self_0: *mut buffer_t) -> ssize_t {
    let mut len: size_t = buffer_length(self_0);
    let mut rem: size_t = ((*self_0).len).wrapping_sub(len);
    let mut buf: *mut core::ffi::c_char = calloc(
        len.wrapping_add(1 as size_t),
        1 as size_t,
    ) as *mut core::ffi::c_char;
    if buf.is_null() {
        return -(1 as core::ffi::c_int) as ssize_t;
    }
    memcpy(
        buf as *mut core::ffi::c_void,
        (*self_0).data as *const core::ffi::c_void,
        len,
    );
    free((*self_0).alloc as *mut core::ffi::c_void);
    (*self_0).len = len;
    (*self_0).alloc = buf;
    (*self_0).data = (*self_0).alloc;
    return rem as ssize_t;
}
pub unsafe extern "C" fn buffer_free(mut self_0: *mut buffer_t) {
    free((*self_0).alloc as *mut core::ffi::c_void);
    free(self_0 as *mut core::ffi::c_void);
}
pub unsafe extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> size_t {
    return (*self_0).len;
}
pub unsafe extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> size_t {
    return strlen((*self_0).data);
}
pub unsafe extern "C" fn buffer_resize(
    mut self_0: *mut buffer_t,
    mut n: size_t,
) -> core::ffi::c_int {
    n = ((n as core::ffi::c_ulong)
        .wrapping_add(
            (1024 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_ulong,
        ) & !(1024 as core::ffi::c_int - 1 as core::ffi::c_int) as core::ffi::c_ulong)
        as size_t;
    (*self_0).len = n;
    (*self_0).data = realloc(
        (*self_0).alloc as *mut core::ffi::c_void,
        n.wrapping_add(1 as size_t),
    ) as *mut core::ffi::c_char;
    (*self_0).alloc = (*self_0).data;
    if ((*self_0).alloc).is_null() {
        return -(1 as core::ffi::c_int);
    }
    *((*self_0).alloc).offset(n as isize) = '\0' as i32 as core::ffi::c_char;
    return 0 as core::ffi::c_int;
}
pub unsafe extern "C" fn buffer_appendf(
    mut self_0: *mut buffer_t,
    mut format: *const core::ffi::c_char,
    mut args: ...
) -> core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut tmpa: ::core::ffi::VaListImpl;
    let mut dst: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut length: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut required: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bytes: core::ffi::c_int = 0 as core::ffi::c_int;
    ap = args.clone();
    length = buffer_length(self_0) as core::ffi::c_int;
    tmpa = ap.clone();
    required = vsnprintf(
        0 as *mut core::ffi::c_char,
        0 as size_t,
        format,
        tmpa.as_va_list(),
    );
    if -(1 as core::ffi::c_int) == buffer_resize(self_0, (length + required) as size_t) {
        return -(1 as core::ffi::c_int);
    }
    dst = ((*self_0).data).offset(length as isize);
    bytes = vsnprintf(
        dst,
        (1 as core::ffi::c_int + required) as size_t,
        format,
        ap.as_va_list(),
    );
    return if bytes < 0 as core::ffi::c_int {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
}
pub unsafe extern "C" fn buffer_append(
    mut self_0: *mut buffer_t,
    mut str: *const core::ffi::c_char,
) -> core::ffi::c_int {
    return buffer_append_n(self_0, str, strlen(str));
}
pub unsafe extern "C" fn buffer_append_n(
    mut self_0: *mut buffer_t,
    mut str: *const core::ffi::c_char,
    mut len: size_t,
) -> core::ffi::c_int {
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if (*self_0).len > needed {
        strncat((*self_0).data, str, len);
        return 0 as core::ffi::c_int;
    }
    let mut ret: core::ffi::c_int = buffer_resize(self_0, needed);
    if -(1 as core::ffi::c_int) == ret {
        return -(1 as core::ffi::c_int);
    }
    strncat((*self_0).data, str, len);
    return 0 as core::ffi::c_int;
}
pub unsafe extern "C" fn buffer_prepend(
    mut self_0: *mut buffer_t,
    mut str: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;
    let mut len: size_t = strlen(str);
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if !((*self_0).len > needed) {
        ret = buffer_resize(self_0, needed);
        if -(1 as core::ffi::c_int) == ret {
            return -(1 as core::ffi::c_int);
        }
    }
    memmove(
        ((*self_0).data).offset(len as isize) as *mut core::ffi::c_void,
        (*self_0).data as *const core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    memcpy(
        (*self_0).data as *mut core::ffi::c_void,
        str as *const core::ffi::c_void,
        len,
    );
    return 0 as core::ffi::c_int;
}
pub unsafe extern "C" fn buffer_slice(
    mut buf: *mut buffer_t,
    mut from: size_t,
    mut to: ssize_t,
) -> *mut buffer_t {
    let mut len: size_t = strlen((*buf).data);
    if (to as core::ffi::c_ulong) < from {
        return 0 as *mut buffer_t;
    }
    if to < 0 as core::ffi::c_long {
        to = (len as core::ffi::c_ulong).wrapping_sub(!to as core::ffi::c_ulong)
            as ssize_t;
    }
    if to as core::ffi::c_ulong > len {
        to = len as ssize_t;
    }
    let mut n: size_t = (to as size_t).wrapping_sub(from);
    let mut self_0: *mut buffer_t = buffer_new_with_size(n);
    memcpy(
        (*self_0).data as *mut core::ffi::c_void,
        ((*buf).data).offset(from as isize) as *const core::ffi::c_void,
        n,
    );
    return self_0;
}
pub unsafe extern "C" fn buffer_equals(
    mut self_0: *mut buffer_t,
    mut other: *mut buffer_t,
) -> core::ffi::c_int {
    return (0 as core::ffi::c_int == strcmp((*self_0).data, (*other).data))
        as core::ffi::c_int;
}
pub unsafe extern "C" fn buffer_indexof(
    mut self_0: *mut buffer_t,
    mut str: *mut core::ffi::c_char,
) -> ssize_t {
    let mut sub: *mut core::ffi::c_char = strstr((*self_0).data, str);
    if sub.is_null() {
        return -(1 as core::ffi::c_int) as ssize_t;
    }
    return sub.offset_from((*self_0).data) as ssize_t;
}
pub unsafe extern "C" fn buffer_trim_left(mut self_0: *mut buffer_t) {
    let mut c: core::ffi::c_int = 0;
    loop {
        c = *(*self_0).data as core::ffi::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as core::ffi::c_int
                & _ISspace as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
                != 0)
        {
            break;
        }
        (*self_0).data = ((*self_0).data).offset(1);
    };
}
pub unsafe extern "C" fn buffer_trim_right(mut self_0: *mut buffer_t) {
    let mut c: core::ffi::c_int = 0;
    let mut i: size_t = (buffer_length(self_0)).wrapping_sub(1 as size_t);
    loop {
        c = *((*self_0).data).offset(i as isize) as core::ffi::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as core::ffi::c_int
                & _ISspace as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
                != 0)
        {
            break;
        }
        let fresh0 = i;
        i = i.wrapping_sub(1);
        *((*self_0).data).offset(fresh0 as isize) = 0 as core::ffi::c_char;
    };
}
pub unsafe extern "C" fn buffer_trim(mut self_0: *mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
pub unsafe extern "C" fn buffer_fill(
    mut self_0: *mut buffer_t,
    mut c: core::ffi::c_int,
) {
    memset((*self_0).data as *mut core::ffi::c_void, c, (*self_0).len);
}
pub unsafe extern "C" fn buffer_clear(mut self_0: *mut buffer_t) {
    buffer_fill(self_0, 0 as core::ffi::c_int);
}
pub unsafe extern "C" fn buffer_print(mut self_0: *mut buffer_t) {
    let mut len: size_t = (*self_0).len;
    printf(b"\n \0" as *const u8 as *const core::ffi::c_char);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while (i as core::ffi::c_ulong) < len {
        printf(
            b" %02x\0" as *const u8 as *const core::ffi::c_char,
            *((*self_0).alloc).offset(i as isize) as core::ffi::c_int,
        );
        if (i + 1 as core::ffi::c_int) % 8 as core::ffi::c_int == 0 as core::ffi::c_int {
            printf(b"\n \0" as *const u8 as *const core::ffi::c_char);
        }
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
}
