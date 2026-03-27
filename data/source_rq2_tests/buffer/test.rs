// buffer test - translated from C
// Original: test_module/public/buffer/test.c

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

// Opaque type for buffer_t
#[repr(C)]
pub struct buffer_t {
    _private: [u8; 0],
}

// External functions from translated buffer library
extern "C" {
    fn buffer_new() -> *mut buffer_t;
    fn buffer_new_with_size(size: libc::size_t) -> *mut buffer_t;
    fn buffer_new_with_copy(str: *const c_char) -> *mut buffer_t;
    fn buffer_free(buf: *mut buffer_t);
    fn buffer_size(buf: *mut buffer_t) -> libc::size_t;
    fn buffer_length(buf: *mut buffer_t) -> libc::size_t;
    fn buffer_append(buf: *mut buffer_t, str: *const c_char) -> libc::c_int;
    fn buffer_append_n(buf: *mut buffer_t, str: *const c_char, len: libc::size_t) -> libc::c_int;
    fn buffer_prepend(buf: *mut buffer_t, str: *const c_char) -> libc::c_int;
    fn buffer_string(buf: *mut buffer_t) -> *mut c_char;
    fn buffer_slice(buf: *mut buffer_t, from: libc::ssize_t, to: libc::ssize_t) -> *mut buffer_t;
    fn buffer_equals(a: *mut buffer_t, b: *mut buffer_t) -> libc::c_int;
    fn buffer_indexof(buf: *mut buffer_t, str: *const c_char) -> libc::ssize_t;
    fn buffer_fill(buf: *mut buffer_t, c: libc::c_int);
    fn buffer_clear(buf: *mut buffer_t);
    fn buffer_trim(buf: *mut buffer_t);
    fn buffer_trim_left(buf: *mut buffer_t);
    fn buffer_trim_right(buf: *mut buffer_t);
    fn buffer_compact(buf: *mut buffer_t) -> libc::ssize_t;
    fn buffer_appendf(buf: *mut buffer_t, format: *const c_char, ...) -> libc::c_int;
}

const BUFFER_DEFAULT_SIZE: libc::size_t = 64;

fn equal(expected: &str, actual: *mut c_char) {
    unsafe {
        let actual_str = CStr::from_ptr(actual).to_str().unwrap();
        if expected != actual_str {
            println!();
            println!("  expected: '{}'", expected);
            println!("    actual: '{}'", actual_str);
            println!();
            std::process::exit(1);
        }
    }
}

fn test_buffer_new() {
    unsafe {
        let buf = buffer_new();
        assert_eq!(BUFFER_DEFAULT_SIZE, buffer_size(buf));
        assert_eq!(0, buffer_length(buf));
        buffer_free(buf);
    }
}

fn test_buffer_new_with_size() {
    unsafe {
        let buf = buffer_new_with_size(1024);
        assert_eq!(1024, buffer_size(buf));
        assert_eq!(0, buffer_length(buf));
        buffer_free(buf);
    }
}

fn test_buffer_append() {
    unsafe {
        let buf = buffer_new();
        let hello = CString::new("Hello").unwrap();
        let world = CString::new(" World").unwrap();
        assert_eq!(0, buffer_append(buf, hello.as_ptr()));
        assert_eq!(0, buffer_append(buf, world.as_ptr()));
        assert_eq!("Hello World".len(), buffer_length(buf));
        equal("Hello World", buffer_string(buf));
        buffer_free(buf);
    }
}

fn test_buffer_append_n() {
    unsafe {
        let buf = buffer_new();
        let subway = CString::new("subway").unwrap();
        let marines = CString::new("marines").unwrap();
        assert_eq!(0, buffer_append_n(buf, subway.as_ptr(), 3));
        assert_eq!(0, buffer_append_n(buf, marines.as_ptr(), 6));
        assert_eq!("submarine".len(), buffer_length(buf));
        equal("submarine", buffer_string(buf));
        buffer_free(buf);
    }
}

fn test_buffer_append_grow() {
    unsafe {
        let buf = buffer_new_with_size(10);
        let s1 = CString::new("Hello").unwrap();
        let s2 = CString::new(" tobi").unwrap();
        let s3 = CString::new(" was").unwrap();
        let s4 = CString::new(" here").unwrap();
        assert_eq!(0, buffer_append(buf, s1.as_ptr()));
        assert_eq!(0, buffer_append(buf, s2.as_ptr()));
        assert_eq!(0, buffer_append(buf, s3.as_ptr()));
        assert_eq!(0, buffer_append(buf, s4.as_ptr()));

        let expected = "Hello tobi was here";
        equal(expected, buffer_string(buf));
        assert_eq!(1024, buffer_size(buf));
        assert_eq!(expected.len(), buffer_length(buf));
        buffer_free(buf);
    }
}

fn test_buffer_prepend() {
    unsafe {
        let buf = buffer_new();
        let world = CString::new(" World").unwrap();
        let hello = CString::new("Hello").unwrap();
        assert_eq!(0, buffer_append(buf, world.as_ptr()));
        assert_eq!(0, buffer_prepend(buf, hello.as_ptr()));
        assert_eq!("Hello World".len(), buffer_length(buf));
        equal("Hello World", buffer_string(buf));
        buffer_free(buf);
    }
}

fn test_buffer_slice() {
    unsafe {
        let buf = buffer_new();
        let s = CString::new("Tobi Ferret").unwrap();
        buffer_append(buf, s.as_ptr());

        let a = buffer_slice(buf, 2, 8);
        equal("Tobi Ferret", buffer_string(buf));
        equal("bi Fer", buffer_string(a));

        buffer_free(buf);
        buffer_free(a);
    }
}

fn test_buffer_slice_range_error() {
    unsafe {
        let s = CString::new("Tobi Ferret").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());
        let a = buffer_slice(buf, 10, 2);
        assert!(a.is_null());
        buffer_free(buf);
    }
}

fn test_buffer_slice_end() {
    unsafe {
        let s = CString::new("Tobi Ferret").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());

        let a = buffer_slice(buf, 5, -1);
        equal("Tobi Ferret", buffer_string(buf));
        equal("Ferret", buffer_string(a));

        let b = buffer_slice(buf, 5, -3);
        equal("Ferr", buffer_string(b));

        let c = buffer_slice(buf, 8, -1);
        equal("ret", buffer_string(c));

        buffer_free(buf);
        buffer_free(a);
        buffer_free(b);
        buffer_free(c);
    }
}

fn test_buffer_slice_end_overflow() {
    unsafe {
        let s = CString::new("Tobi Ferret").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());
        let a = buffer_slice(buf, 5, 1000);
        equal("Tobi Ferret", buffer_string(buf));
        equal("Ferret", buffer_string(a));
        buffer_free(a);
        buffer_free(buf);
    }
}

fn test_buffer_equals() {
    unsafe {
        let s1 = CString::new("Hello").unwrap();
        let s2 = CString::new("Hello").unwrap();
        let world = CString::new(" World").unwrap();
        let a = buffer_new_with_copy(s1.as_ptr());
        let b = buffer_new_with_copy(s2.as_ptr());

        assert_eq!(1, buffer_equals(a, b));

        buffer_append(b, world.as_ptr());
        assert_eq!(0, buffer_equals(a, b));

        buffer_free(a);
        buffer_free(b);
    }
}

fn test_buffer_formatting() {
    unsafe {
        let buf = buffer_new();
        let fmt1 = CString::new("%d %s").unwrap();
        let cow = CString::new("cow").unwrap();
        let result = buffer_appendf(buf, fmt1.as_ptr(), 3 as libc::c_int, cow.as_ptr());
        assert_eq!(0, result);
        equal("3 cow", buffer_string(buf));

        let fmt2 = CString::new(" - 0x%08X").unwrap();
        let result = buffer_appendf(buf, fmt2.as_ptr(), 0xdeadbeefu32 as libc::c_uint);
        assert_eq!(0, result);
        equal("3 cow - 0xDEADBEEF", buffer_string(buf));
        buffer_free(buf);
    }
}

fn test_buffer_indexof() {
    unsafe {
        let s = CString::new("Tobi is a ferret").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());

        let is_str = CString::new("is").unwrap();
        let i = buffer_indexof(buf, is_str.as_ptr());
        assert_eq!(5, i);

        let a_str = CString::new("a").unwrap();
        let i = buffer_indexof(buf, a_str.as_ptr());
        assert_eq!(8, i);

        let something = CString::new("something").unwrap();
        let i = buffer_indexof(buf, something.as_ptr());
        assert_eq!(-1, i);

        buffer_free(buf);
    }
}

fn test_buffer_fill() {
    unsafe {
        let s = CString::new("Hello").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());
        assert_eq!(5, buffer_length(buf));

        buffer_fill(buf, 0);
        assert_eq!(0, buffer_length(buf));
        buffer_free(buf);
    }
}

fn test_buffer_clear() {
    unsafe {
        let s = CString::new("Hello").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());
        assert_eq!(5, buffer_length(buf));

        buffer_clear(buf);
        assert_eq!(0, buffer_length(buf));
        buffer_free(buf);
    }
}

fn test_buffer_trim() {
    unsafe {
        let s1 = CString::new("  Hello\n\n ").unwrap();
        let buf = buffer_new_with_copy(s1.as_ptr());
        buffer_trim(buf);
        equal("Hello", buffer_string(buf));
        buffer_free(buf);

        let buf = buffer_new_with_copy(s1.as_ptr());
        buffer_trim_left(buf);
        equal("Hello\n\n ", buffer_string(buf));
        buffer_free(buf);

        let buf = buffer_new_with_copy(s1.as_ptr());
        buffer_trim_right(buf);
        equal("  Hello", buffer_string(buf));
        buffer_free(buf);
    }
}

fn test_buffer_compact() {
    unsafe {
        let s = CString::new("  Hello\n\n ").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr());
        buffer_trim(buf);
        assert_eq!(5, buffer_length(buf));
        assert_eq!(10, buffer_size(buf));

        let removed = buffer_compact(buf);
        assert_eq!(5, removed);
        assert_eq!(5, buffer_length(buf));
        assert_eq!(5, buffer_size(buf));
        equal("Hello", buffer_string(buf));

        buffer_free(buf);
    }
}

fn test_buffer_prepend_issue_15() {
    unsafe {
        let file = buffer_new();
        let s1 = CString::new("layout.bk.html").unwrap();
        let s2 = CString::new("./example/").unwrap();
        assert_eq!(0, buffer_append(file, s1.as_ptr()));
        assert_eq!(0, buffer_prepend(file, s2.as_ptr()));
        assert_eq!("./example/layout.bk.html".len(), buffer_length(file));
        equal("./example/layout.bk.html", buffer_string(file));
        buffer_free(file);
    }
}

fn main() {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    test_buffer_append_grow();
    test_buffer_append_n();
    test_buffer_prepend();
    test_buffer_slice();
    test_buffer_slice_range_error();
    test_buffer_slice_end();
    test_buffer_slice_end_overflow();
    test_buffer_equals();
    test_buffer_formatting();
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    test_buffer_prepend_issue_15();
    println!("\n  \x1b[32m✓ \x1b[90mok\x1b[0m\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        test_buffer_new();
    }

    #[test]
    fn test_new_with_size() {
        test_buffer_new_with_size();
    }

    #[test]
    fn test_append() {
        test_buffer_append();
    }

    #[test]
    fn test_append_grow() {
        test_buffer_append_grow();
    }

    #[test]
    fn test_append_n() {
        test_buffer_append_n();
    }

    #[test]
    fn test_prepend() {
        test_buffer_prepend();
    }

    #[test]
    fn test_slice() {
        test_buffer_slice();
    }

    #[test]
    fn test_slice_range_error() {
        test_buffer_slice_range_error();
    }

    #[test]
    fn test_slice_end() {
        test_buffer_slice_end();
    }

    #[test]
    fn test_slice_end_overflow() {
        test_buffer_slice_end_overflow();
    }

    #[test]
    fn test_equals() {
        test_buffer_equals();
    }

    #[test]
    fn test_formatting() {
        test_buffer_formatting();
    }

    #[test]
    fn test_indexof() {
        test_buffer_indexof();
    }

    #[test]
    fn test_fill() {
        test_buffer_fill();
    }

    #[test]
    fn test_clear() {
        test_buffer_clear();
    }

    #[test]
    fn test_trim() {
        test_buffer_trim();
    }

    #[test]
    fn test_compact() {
        test_buffer_compact();
    }

    #[test]
    fn test_prepend_issue_15() {
        test_buffer_prepend_issue_15();
    }
}
