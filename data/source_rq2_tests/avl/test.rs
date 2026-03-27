// Rust translation of avl_test.c
// Copyright (c) 2019 xieqing. https://github.com/xieqing
// Translated from C to Rust

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// AVL tree structures (opaque/partial)
#[repr(C)]
pub struct avlnode {
    pub left: *mut avlnode,
    pub right: *mut avlnode,
    pub parent: *mut avlnode,
    pub bf: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct avltree {
    pub root: avlnode,
    pub nil: avlnode,
    pub compare: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

// mydata structure for testing
#[repr(C)]
pub struct mydata {
    pub key: c_int,
}

// External declarations for AVL library functions
extern "C" {
    fn avl_create(
        compare: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        destroy: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut avltree;
    fn avl_destroy(avlt: *mut avltree);
    fn avl_find(avlt: *mut avltree, data: *const c_void) -> *mut avlnode;
    fn avl_insert(avlt: *mut avltree, data: *mut c_void) -> *mut avlnode;
    fn avl_delete(avlt: *mut avltree, node: *mut avlnode, flag: c_int);
    fn avl_successor(avlt: *mut avltree, node: *mut avlnode) -> *mut avlnode;
    fn avl_check_order(avlt: *mut avltree, min: *const c_void, max: *const c_void) -> c_int;
    fn avl_check_height(avlt: *mut avltree) -> c_int;
    fn avl_print(avlt: *mut avltree, print_func: Option<unsafe extern "C" fn(*const c_void)>);

    fn compare_func(a: *const c_void, b: *const c_void) -> c_int;
    fn destroy_func(data: *mut c_void);
    fn print_func(data: *const c_void);
    fn makedata(key: c_int) -> *mut mydata;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn srand(seed: c_int);
    fn rand() -> c_int;
    fn time(t: *mut i64) -> i64;
}

// Constants
const MIN: c_int = i32::MIN;
const MAX: c_int = i32::MAX;
const CHARS: &str = "ABCDEFGHIJ";

// Minunit test framework translation
static mut MU_TESTS: i32 = 0;
static mut MU_FAILS: i32 = 0;

fn mu_test<F: FnOnce() -> bool>(name: &str, result: F) {
    unsafe {
        MU_TESTS += 1;
        println!("#{:03} {}", MU_TESTS, name);
        if result() {
            println!("PASSED");
        } else {
            println!("FAILED");
            MU_FAILS += 1;
        }
    }
}

// Global error counter for permutation tests
static mut PERMUTATION_ERROR: i32 = 0;

// Helper macro to get AVL_NIL
unsafe fn avl_nil(avlt: *mut avltree) -> *mut avlnode {
    &mut (*avlt).nil as *mut avlnode
}

// Helper functions
fn tree_create() -> *mut avltree {
    unsafe { avl_create(Some(compare_func), Some(destroy_func)) }
}

fn tree_find(avlt: *mut avltree, key: c_int) -> *mut avlnode {
    unsafe {
        let mut query = mydata { key };
        avl_find(avlt, &query as *const mydata as *const c_void)
    }
}

fn tree_check(avlt: *mut avltree) -> bool {
    unsafe {
        let min = mydata { key: MIN };
        let max = mydata { key: MAX };

        if avl_check_order(avlt, &min as *const mydata as *const c_void, &max as *const mydata as *const c_void) != 1 {
            println!("tree_check: invalid order");
            return false;
        }

        if avl_check_height(avlt) != 1 {
            println!("tree_check: invalid height");
            return false;
        }

        true
    }
}

fn tree_insert(avlt: *mut avltree, key: c_int) -> *mut avlnode {
    unsafe {
        if key < MIN || key > MAX {
            println!("tree_insert: invalid key {}", key);
            return ptr::null_mut();
        }

        let data = makedata(key);
        if data.is_null() {
            println!("tree_insert: insert {} failed (makedata)", key);
            return ptr::null_mut();
        }

        let node = avl_insert(avlt, data as *mut c_void);
        if node.is_null() {
            println!("tree_insert: insert {} failed", key);
            free(data as *mut c_void);
            return ptr::null_mut();
        }

        node
    }
}

fn tree_delete(avlt: *mut avltree, key: c_int) -> bool {
    unsafe {
        let node = tree_find(avlt, key);
        if node.is_null() {
            println!("tree_delete: {} not found", key);
            return false;
        }

        avl_delete(avlt, node, 0);

        if tree_find(avlt, key) == node {
            println!("tree_delete: delete {} failed", key);
            return false;
        }

        true
    }
}

// Permutation helpers
fn swap(a: &mut [u8], i: usize, j: usize) {
    a.swap(i, j);
}

fn permute<F: FnMut(&[u8])>(a: &mut [u8], start: usize, end: usize, func: &mut F) {
    if start == end {
        func(a);
        return;
    }

    for i in start..=end {
        swap(a, start, i);
        permute(a, start + 1, end, func);
        swap(a, start, i);
    }
}

fn permutation_insert(a: &[u8]) {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            PERMUTATION_ERROR += 1;
            return;
        }

        for i in 0..a.len() {
            let key = a[i] as c_int;
            let node = tree_insert(avlt, key);
            if node.is_null() || tree_find(avlt, key) != node || !tree_check(avlt) {
                println!("insert {} failed", a[i] as char);
                PERMUTATION_ERROR += 1;
                avl_destroy(avlt);
                return;
            }
        }

        avl_destroy(avlt);
    }
}

fn permutation_delete(a: &[u8]) {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            PERMUTATION_ERROR += 1;
            return;
        }

        let b = CHARS.as_bytes();
        for i in 0..b.len() {
            let key = b[i] as c_int;
            let node = tree_insert(avlt, key);
            if node.is_null() || tree_find(avlt, key) != node || !tree_check(avlt) {
                println!("insert {} failed", b[i] as char);
                PERMUTATION_ERROR += 1;
                avl_destroy(avlt);
                return;
            }
        }

        for i in 0..a.len() {
            let key = a[i] as c_int;
            if !tree_delete(avlt, key) || !tree_check(avlt) {
                println!("delete {} failed", a[i] as char);
                PERMUTATION_ERROR += 1;
                avl_destroy(avlt);
                return;
            }
        }

        avl_destroy(avlt);
    }
}

// Unit tests
fn unit_test_create() -> bool {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            return false;
        }

        let nil = avl_nil(avlt);
        if (*avlt).compare != Some(compare_func)
            || (*avlt).destroy != Some(destroy_func)
            || (*avlt).nil.left != nil
            || (*avlt).nil.right != nil
            || (*avlt).nil.parent != nil
            || (*avlt).nil.bf != 0
            || !(*avlt).nil.data.is_null()
            || (*avlt).root.left != nil
            || (*avlt).root.right != nil
            || (*avlt).root.parent != nil
            || (*avlt).root.bf != 0
            || !(*avlt).root.data.is_null()
        {
            println!("init 1");
            avl_destroy(avlt);
            return false;
        }

        avl_destroy(avlt);
        true
    }
}

fn unit_test_find() -> bool {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            return false;
        }

        let r = tree_insert(avlt, 'R' as c_int);
        let e = tree_insert(avlt, 'E' as c_int);
        let d = tree_insert(avlt, 'D' as c_int);
        let s = tree_insert(avlt, 'S' as c_int);
        let o = tree_insert(avlt, 'O' as c_int);
        let x = tree_insert(avlt, 'X' as c_int);
        let c = tree_insert(avlt, 'C' as c_int);
        let u = tree_insert(avlt, 'U' as c_int);
        let b = tree_insert(avlt, 'B' as c_int);
        let t = tree_insert(avlt, 'T' as c_int);

        if r.is_null() || e.is_null() || d.is_null() || s.is_null() || o.is_null()
            || x.is_null() || c.is_null() || u.is_null() || b.is_null() || t.is_null()
            || !tree_check(avlt)
        {
            println!("init failed");
            avl_destroy(avlt);
            return false;
        }

        if avl_find(avlt, (*r).data) != r
            || avl_find(avlt, (*e).data) != e
            || avl_find(avlt, (*d).data) != d
            || avl_find(avlt, (*s).data) != s
            || avl_find(avlt, (*o).data) != o
            || avl_find(avlt, (*x).data) != x
            || avl_find(avlt, (*c).data) != c
            || avl_find(avlt, (*u).data) != u
            || avl_find(avlt, (*b).data) != b
            || avl_find(avlt, (*t).data) != t
        {
            println!("find failed");
            avl_destroy(avlt);
            return false;
        }

        avl_destroy(avlt);
        true
    }
}

fn unit_test_successor() -> bool {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            return false;
        }

        let r = tree_insert(avlt, 'R' as c_int);
        let e = tree_insert(avlt, 'E' as c_int);
        let d = tree_insert(avlt, 'D' as c_int);
        let s = tree_insert(avlt, 'S' as c_int);
        let _o = tree_insert(avlt, 'O' as c_int);
        let x = tree_insert(avlt, 'X' as c_int);
        let c = tree_insert(avlt, 'C' as c_int);
        let u = tree_insert(avlt, 'U' as c_int);
        let b = tree_insert(avlt, 'B' as c_int);
        let t = tree_insert(avlt, 'T' as c_int);

        if !tree_delete(avlt, 'O' as c_int) || !tree_check(avlt) {
            println!("init failed");
            avl_destroy(avlt);
            return false;
        }

        if avl_successor(avlt, b) != c
            || avl_successor(avlt, c) != d
            || avl_successor(avlt, d) != e
            || avl_successor(avlt, e) != r
            || avl_successor(avlt, r) != s
            || avl_successor(avlt, s) != t
            || avl_successor(avlt, t) != u
            || avl_successor(avlt, u) != x
            || !avl_successor(avlt, x).is_null()
        {
            println!("successor failed");
            avl_destroy(avlt);
            return false;
        }

        avl_destroy(avlt);
        true
    }
}

fn unit_test_atomic_insertion() -> bool {
    let cases: Vec<&str> = vec![
        "P", "PH", "PX", "PHD", "PHXDLB", "PHXDLF", "PHL", "PHXDLJ", "PHXDLN",
        "PXb", "PHXTbZ", "PHXTbd", "PHXT", "PHXTbRV", "PHXTbR", "PHXTbV",
    ];

    unsafe {
        for case in cases {
            let avlt = tree_create();
            if avlt.is_null() {
                println!("{} - create AVL tree failed", case);
                return false;
            }

            for ch in case.chars() {
                if tree_insert(avlt, ch as c_int).is_null() || !tree_check(avlt) {
                    println!("{} - insert {} failed", case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            avl_destroy(avlt);
        }
    }

    true
}

fn unit_test_atomic_deletion() -> bool {
    let cases: Vec<(&str, &str)> = vec![
        ("P", "P"), ("PH", "H"), ("PH", "P"), ("PX", "P"), ("PX", "X"),
        ("PHX", "H"), ("PHX", "P"), ("PHX", "X"),
        ("PHXD", "X"), ("PHXDLTBF", "X"), ("PHXDLTB", "X"), ("PHXDLTF", "X"),
        ("PHXDL", "X"), ("PHXDLTBFJN", "X"), ("PHXDLTBFJ", "X"), ("PHXDLTBFN", "X"),
        ("PHXDLTBJN", "X"), ("PHXDLTBJ", "X"), ("PHXDLTBN", "X"), ("PHXDLTFJN", "X"),
        ("PHXDLTFJ", "X"), ("PHXDLTFN", "X"), ("PHXL", "X"), ("PHXDLTJN", "X"),
        ("PHXDLTJ", "X"), ("PHXDLTN", "X"), ("PHXb", "H"), ("PHXDTbZd", "H"),
        ("PHXDTbZ", "H"), ("PHXDTbd", "H"), ("PHXTb", "H"), ("PHXDTbRVZd", "H"),
        ("PHXDTbRVZ", "H"), ("PHXDTbRVd", "H"), ("PHXDTbRZd", "H"), ("PHXDTbRZ", "H"),
        ("PHXDTbRd", "H"), ("PHXDTbVZd", "H"), ("PHXDTbVZ", "H"), ("PHXDTbVd", "H"),
        ("PHXT", "H"), ("PHXDTbRV", "H"), ("PHXDTbR", "H"), ("PHXDTbV", "H"),
    ];

    unsafe {
        for (insert_case, delete_case) in cases {
            let avlt = tree_create();
            if avlt.is_null() {
                println!("{}-{} - create AVL tree failed", insert_case, delete_case);
                return false;
            }

            for ch in insert_case.chars() {
                if tree_insert(avlt, ch as c_int).is_null() || !tree_check(avlt) {
                    println!("{}-{} - insert {} failed", insert_case, delete_case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            for ch in delete_case.chars() {
                if !tree_delete(avlt, ch as c_int) || !tree_check(avlt) {
                    println!("{}-{} - delete {} failed", insert_case, delete_case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            avl_destroy(avlt);
        }
    }

    true
}

fn unit_test_chain_insertion() -> bool {
    let cases: Vec<&str> = vec![
        "PHXDB", "PHXDF", "PHXLJ", "PHXLN", "PHXTR", "PHXTV", "PHXbZ", "PHXbd",
        "PHXDLB", "PHXDLF", "PHXDLJ", "PHXDLN", "PHXTbR", "PHXTbV", "PHXTbV",
        "PHXTbZ", "PHXTbd",
    ];

    unsafe {
        for case in cases {
            let avlt = tree_create();
            if avlt.is_null() {
                println!("{} - create AVL tree failed", case);
                return false;
            }

            for ch in case.chars() {
                if tree_insert(avlt, ch as c_int).is_null() || !tree_check(avlt) {
                    println!("{} - insert {} failed", case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            avl_destroy(avlt);
        }
    }

    true
}

fn unit_test_chain_deletion() -> bool {
    let cases: Vec<(&str, &str)> = vec![
        ("PHXDLTbFJNZK", "T"),
        ("PHXDLTbFJNZdK", "T"),
        ("PHXDLTbFJNdK", "T"),
        ("PHXDLTbFRVZU", "L"),
        ("PHXDLTbBFRVZU", "L"),
        ("PHXDLTbBRVZU", "L"),
    ];

    unsafe {
        for (insert_case, delete_case) in cases {
            let avlt = tree_create();
            if avlt.is_null() {
                println!("{}-{} - create AVL tree failed", insert_case, delete_case);
                return false;
            }

            for ch in insert_case.chars() {
                if tree_insert(avlt, ch as c_int).is_null() || !tree_check(avlt) {
                    println!("{}-{} - insert {} failed", insert_case, delete_case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            for ch in delete_case.chars() {
                if !tree_delete(avlt, ch as c_int) || !tree_check(avlt) {
                    println!("{}-{} - delete {} failed", insert_case, delete_case, ch);
                    avl_destroy(avlt);
                    return false;
                }
            }

            avl_destroy(avlt);
        }
    }

    true
}

fn unit_test_permutation_insertion() -> bool {
    unsafe {
        let mut a: Vec<u8> = CHARS.bytes().collect();
        PERMUTATION_ERROR = 0;
        permute(&mut a, 0, a.len() - 1, &mut |perm| permutation_insert(perm));
        PERMUTATION_ERROR == 0
    }
}

fn unit_test_permutation_deletion() -> bool {
    unsafe {
        let mut a: Vec<u8> = CHARS.bytes().collect();
        PERMUTATION_ERROR = 0;
        permute(&mut a, 0, a.len() - 1, &mut |perm| permutation_delete(perm));
        PERMUTATION_ERROR == 0
    }
}

fn unit_test_random_insertion_deletion() -> bool {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            return false;
        }

        let mut ninsert = 0;
        let mut ndelete = 0;
        let max_val = 9999;

        srand(time(ptr::null_mut()) as c_int);

        for _ in 1..=1999 {
            let key = rand() % max_val;
            if !tree_find(avlt, key).is_null() {
                continue;
            }
            ninsert += 1;
            if tree_insert(avlt, key).is_null() || !tree_check(avlt) {
                println!("insert {} failed", key);
                avl_destroy(avlt);
                return false;
            }
        }

        for _ in 1..max_val {
            let key = rand() % max_val;
            if tree_find(avlt, key).is_null() {
                continue;
            }
            ndelete += 1;
            if !tree_delete(avlt, key) || !tree_check(avlt) {
                println!("delete {} failed", key);
                avl_destroy(avlt);
                return false;
            }
        }

        println!("\tstat: ninsert={}, ndelete={}", ninsert, ndelete);

        avl_destroy(avlt);
        true
    }
}

fn unit_test_dup() -> bool {
    unsafe {
        let avlt = tree_create();
        if avlt.is_null() {
            println!("create AVL tree failed");
            return false;
        }

        let n1 = tree_insert(avlt, 'N' as c_int);
        let n2 = tree_insert(avlt, 'N' as c_int);

        if n1.is_null() || n2.is_null() {
            println!("insert failed");
            avl_destroy(avlt);
            return false;
        }

        // Without AVL_DUP defined, n1 should equal n2
        if n1 != n2 {
            println!("invalid dup");
            avl_destroy(avlt);
            return false;
        }

        avl_destroy(avlt);
        true
    }
}

fn all_tests() {
    mu_test("unit_test_create", unit_test_create);
    mu_test("unit_test_find", unit_test_find);
    mu_test("unit_test_successor", unit_test_successor);
    mu_test("unit_test_atomic_insertion", unit_test_atomic_insertion);
    mu_test("unit_test_atomic_deletion", unit_test_atomic_deletion);
    mu_test("unit_test_chain_insertion", unit_test_chain_insertion);
    mu_test("unit_test_chain_deletion", unit_test_chain_deletion);
    mu_test("unit_test_permutation_insertion", unit_test_permutation_insertion);
    mu_test("unit_test_permutation_deletion", unit_test_permutation_deletion);
    mu_test("unit_test_random_insertion_deletion", unit_test_random_insertion_deletion);
    mu_test("unit_test_dup", unit_test_dup);
}

fn main() {
    all_tests();

    unsafe {
        if MU_FAILS > 0 {
            println!("*** {}/{} TESTS FAILED ***", MU_FAILS, MU_TESTS);
            std::process::exit(1);
        } else {
            println!("ALL TESTS PASSED");
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert!(unit_test_create());
    }

    #[test]
    fn test_find() {
        assert!(unit_test_find());
    }

    #[test]
    fn test_successor() {
        assert!(unit_test_successor());
    }

    #[test]
    fn test_atomic_insertion() {
        assert!(unit_test_atomic_insertion());
    }

    #[test]
    fn test_atomic_deletion() {
        assert!(unit_test_atomic_deletion());
    }

    #[test]
    fn test_chain_insertion() {
        assert!(unit_test_chain_insertion());
    }

    #[test]
    fn test_chain_deletion() {
        assert!(unit_test_chain_deletion());
    }

    #[test]
    fn test_dup() {
        assert!(unit_test_dup());
    }
}
