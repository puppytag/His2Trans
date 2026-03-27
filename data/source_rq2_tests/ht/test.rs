// Rust translation of hash.c test section (TEST_HASH)
// Copyright (c) 2012 TJ Holowaychuk <tj@vision-media.ca>
// Translated from C to Rust

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// hash_t opaque type (based on khash)
#[repr(C)]
pub struct hash_t {
    _private: [u8; 0],
}

// External declarations for hash library functions
extern "C" {
    fn hash_new() -> *mut hash_t;
    fn hash_free(hash: *mut hash_t);
    fn hash_set(hash: *mut hash_t, key: *const c_char, val: *mut c_void);
    fn hash_get(hash: *mut hash_t, key: *const c_char) -> *mut c_void;
    fn hash_has(hash: *mut hash_t, key: *const c_char) -> c_int;
    fn hash_del(hash: *mut hash_t, key: *const c_char);
    fn hash_size(hash: *mut hash_t) -> c_int;
    fn hash_clear(hash: *mut hash_t);

    // Iterator functions - these may need special handling due to macro expansion
    // In the original C code, hash_each, hash_each_key, hash_each_val are macros
    // that expand to kh_foreach loops. We'll need to use the underlying khash API
    // or provide wrapper functions.
}

// Note: hash_each, hash_each_key, hash_each_val are C macros that expand to loops.
// In Rust, we need to access the underlying khash iterator API or use FFI wrappers.
// For this translation, we'll create helper functions that iterate through the hash.

// External declarations for khash iteration (if available)
extern "C" {
    fn kh_begin(hash: *mut hash_t) -> u32;
    fn kh_end(hash: *mut hash_t) -> u32;
    fn kh_exist(hash: *mut hash_t, k: u32) -> c_int;
    fn kh_key(hash: *mut hash_t, k: u32) -> *const c_char;
    fn kh_value(hash: *mut hash_t, k: u32) -> *mut c_void;
}

// Helper function to iterate hash and collect keys/values
unsafe fn hash_collect_entries(hash: *mut hash_t) -> (Vec<*const c_char>, Vec<*mut c_void>) {
    let mut keys = Vec::new();
    let mut vals = Vec::new();

    let end = kh_end(hash);
    let mut k = kh_begin(hash);
    while k < end {
        if kh_exist(hash, k) != 0 {
            keys.push(kh_key(hash, k));
            vals.push(kh_value(hash, k));
        }
        k += 1;
    }

    (keys, vals)
}

fn test_hash_set() {
    unsafe {
        let hash = hash_new();
        assert_eq!(0, hash_size(hash), "Initial hash size should be 0");

        let name_key = CString::new("name").unwrap();
        let species_key = CString::new("species").unwrap();
        let tobi_val = CString::new("tobi").unwrap();
        let ferret_val = CString::new("ferret").unwrap();

        hash_set(hash, name_key.as_ptr(), tobi_val.as_ptr() as *mut c_void);
        hash_set(hash, species_key.as_ptr(), ferret_val.as_ptr() as *mut c_void);
        assert_eq!(2, hash_size(hash), "Hash size should be 2 after two inserts");

        let got_name = hash_get(hash, name_key.as_ptr()) as *const c_char;
        let got_species = hash_get(hash, species_key.as_ptr()) as *const c_char;

        assert_eq!(
            CStr::from_ptr(got_name).to_str().unwrap(),
            "tobi",
            "name should be 'tobi'"
        );
        assert_eq!(
            CStr::from_ptr(got_species).to_str().unwrap(),
            "ferret",
            "species should be 'ferret'"
        );

        hash_free(hash);
    }
}

fn test_hash_get() {
    unsafe {
        let hash = hash_new();

        let foo_key = CString::new("foo").unwrap();
        let bar_key = CString::new("bar").unwrap();
        let bar_val = CString::new("bar").unwrap();

        hash_set(hash, foo_key.as_ptr(), bar_val.as_ptr() as *mut c_void);

        let got = hash_get(hash, foo_key.as_ptr()) as *const c_char;
        assert_eq!(
            CStr::from_ptr(got).to_str().unwrap(),
            "bar",
            "foo should be 'bar'"
        );

        let got_null = hash_get(hash, bar_key.as_ptr());
        assert!(got_null.is_null(), "bar key should return NULL");

        hash_free(hash);
    }
}

fn test_hash_has() {
    unsafe {
        let hash = hash_new();

        let foo_key = CString::new("foo").unwrap();
        let bar_key = CString::new("bar").unwrap();
        let bar_val = CString::new("bar").unwrap();

        hash_set(hash, foo_key.as_ptr(), bar_val.as_ptr() as *mut c_void);

        assert_eq!(1, hash_has(hash, foo_key.as_ptr()), "should have 'foo'");
        assert_eq!(0, hash_has(hash, bar_key.as_ptr()), "should not have 'bar'");

        hash_free(hash);
    }
}

fn test_hash_size() {
    unsafe {
        let hash = hash_new();

        let foo_key = CString::new("foo").unwrap();
        let bar_key = CString::new("bar").unwrap();
        let bar_val = CString::new("bar").unwrap();
        let baz_val = CString::new("baz").unwrap();

        assert_eq!(0, hash_size(hash));

        hash_set(hash, foo_key.as_ptr(), bar_val.as_ptr() as *mut c_void);
        assert_eq!(1, hash_size(hash));

        hash_set(hash, bar_key.as_ptr(), baz_val.as_ptr() as *mut c_void);
        assert_eq!(2, hash_size(hash));

        hash_free(hash);
    }
}

fn test_hash_del() {
    unsafe {
        let hash = hash_new();

        let foo_key = CString::new("foo").unwrap();
        let bar_key = CString::new("bar").unwrap();
        let bar_val = CString::new("bar").unwrap();

        hash_set(hash, foo_key.as_ptr(), bar_val.as_ptr() as *mut c_void);

        assert_eq!(1, hash_has(hash, foo_key.as_ptr()));
        assert_eq!(0, hash_has(hash, bar_key.as_ptr()));

        hash_del(hash, foo_key.as_ptr());
        hash_del(hash, bar_key.as_ptr()); // Should be safe even if not present

        assert_eq!(0, hash_has(hash, foo_key.as_ptr()));

        hash_free(hash);
    }
}

fn test_hash_clear() {
    unsafe {
        let hash = hash_new();

        let foo_key = CString::new("foo").unwrap();
        let bar_key = CString::new("bar").unwrap();
        let raz_key = CString::new("raz").unwrap();
        let bar_val = CString::new("bar").unwrap();
        let baz_val = CString::new("baz").unwrap();
        let jaz_val = CString::new("jaz").unwrap();

        hash_set(hash, foo_key.as_ptr(), bar_val.as_ptr() as *mut c_void);
        hash_set(hash, bar_key.as_ptr(), baz_val.as_ptr() as *mut c_void);
        hash_set(hash, raz_key.as_ptr(), jaz_val.as_ptr() as *mut c_void);

        assert_eq!(3, hash_size(hash));

        hash_clear(hash);

        assert_eq!(0, hash_size(hash));

        hash_free(hash);
    }
}

fn test_hash_each() {
    unsafe {
        let hash = hash_new();

        let name_key = CString::new("name").unwrap();
        let age_key = CString::new("age").unwrap();
        let tj_val = CString::new("tj").unwrap();
        let age_val = CString::new("25").unwrap();

        hash_set(hash, name_key.as_ptr(), tj_val.as_ptr() as *mut c_void);
        hash_set(hash, age_key.as_ptr(), age_val.as_ptr() as *mut c_void);

        let (keys, vals) = hash_collect_entries(hash);

        assert_eq!(keys.len(), 2);
        assert_eq!(vals.len(), 2);

        // Keys should contain "age" and "name" (order not guaranteed)
        let key0 = CStr::from_ptr(keys[0]).to_str().unwrap();
        let key1 = CStr::from_ptr(keys[1]).to_str().unwrap();
        assert!(key0 == "age" || key0 == "name");
        assert!(key1 == "age" || key1 == "name");

        // Values should contain "25" and "tj"
        let val0 = CStr::from_ptr(vals[0] as *const c_char).to_str().unwrap();
        let val1 = CStr::from_ptr(vals[1] as *const c_char).to_str().unwrap();
        assert!(val0 == "25" || val0 == "tj");
        assert!(val1 == "25" || val1 == "tj");

        hash_free(hash);
    }
}

fn test_hash_each_key() {
    unsafe {
        let hash = hash_new();

        let name_key = CString::new("name").unwrap();
        let age_key = CString::new("age").unwrap();
        let tj_val = CString::new("tj").unwrap();
        let age_val = CString::new("25").unwrap();

        hash_set(hash, name_key.as_ptr(), tj_val.as_ptr() as *mut c_void);
        hash_set(hash, age_key.as_ptr(), age_val.as_ptr() as *mut c_void);

        let (keys, _) = hash_collect_entries(hash);

        assert_eq!(keys.len(), 2);

        let key0 = CStr::from_ptr(keys[0]).to_str().unwrap();
        let key1 = CStr::from_ptr(keys[1]).to_str().unwrap();
        assert!(key0 == "age" || key0 == "name");
        assert!(key1 == "age" || key1 == "name");

        hash_free(hash);
    }
}

fn test_hash_each_val() {
    unsafe {
        let hash = hash_new();

        let name_key = CString::new("name").unwrap();
        let age_key = CString::new("age").unwrap();
        let tj_val = CString::new("tj").unwrap();
        let age_val = CString::new("25").unwrap();

        hash_set(hash, name_key.as_ptr(), tj_val.as_ptr() as *mut c_void);
        hash_set(hash, age_key.as_ptr(), age_val.as_ptr() as *mut c_void);

        let (_, vals) = hash_collect_entries(hash);

        assert_eq!(vals.len(), 2);

        let val0 = CStr::from_ptr(vals[0] as *const c_char).to_str().unwrap();
        let val1 = CStr::from_ptr(vals[1] as *const c_char).to_str().unwrap();
        assert!(val0 == "25" || val0 == "tj");
        assert!(val1 == "25" || val1 == "tj");

        hash_free(hash);
    }
}

fn main() {
    test_hash_set();
    test_hash_get();
    test_hash_has();
    test_hash_del();
    test_hash_size();
    test_hash_clear();
    test_hash_each();
    test_hash_each_key();
    test_hash_each_val();
    println!("\n  \x1b[32m✓ \x1b[90mok\x1b[0m\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        test_hash_set();
    }

    #[test]
    fn test_get() {
        test_hash_get();
    }

    #[test]
    fn test_has() {
        test_hash_has();
    }

    #[test]
    fn test_del() {
        test_hash_del();
    }

    #[test]
    fn test_size() {
        test_hash_size();
    }

    #[test]
    fn test_clear() {
        test_hash_clear();
    }

    #[test]
    fn test_each() {
        test_hash_each();
    }

    #[test]
    fn test_each_key() {
        test_hash_each_key();
    }

    #[test]
    fn test_each_val() {
        test_hash_each_val();
    }
}
