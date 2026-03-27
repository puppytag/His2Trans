// simcrat 测试文件 for avl (AVL3 API)
//
// 目标：和 evolc2rust/c2saferrust 保持同一套语义（插入/查找/遍历/删除）。
// 注意：simcrat 生成的是单文件 Rust 代码，API 可能存在语义或内存布局问题；
// 这些测试会把问题暴露出来（失败/崩溃都是“真实失败”）。

use crate::*;

use std::ffi::c_void;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Default)]
struct NodeWithKey {
    node: Avl3Node,
    key: i32,
}

fn node_key_offsets() -> (u16, u16) {
    let dummy = NodeWithKey::default();
    let base = &dummy as *const NodeWithKey as usize;
    let node_off = (&dummy.node as *const Avl3Node as usize) - base;
    let key_off = (&dummy.key as *const i32 as usize) - base;
    (key_off as u16, node_off as u16)
}

fn compare_i32(a: &c_void, b: &c_void) -> c_int {
    let av = unsafe { *(a as *const c_void as *const i32) };
    let bv = unsafe { *(b as *const c_void as *const i32) };
    if av < bv {
        -1
    } else if av > bv {
        1
    } else {
        0
    }
}

unsafe fn key_of_container(p: *mut c_void) -> i32 {
    let c = p as *mut NodeWithKey;
    (*c).key
}

#[test]
fn test_avl_insert_find_next_prev() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: Avl3Tree = Default::default();
    let info = Avl3TreeInfo {
        pf_compare: compare_i32,
        us_key_offset: key_off,
        us_node_offset: node_off,
    };

    let mut n1: NodeWithKey = Default::default();
    let mut n2: NodeWithKey = Default::default();
    let mut n3: NodeWithKey = Default::default();
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    // Insert three nodes.
    assert!(
        vos_avl3_insert_or_find(&mut tree, &mut n2.node, &info).is_null(),
        "insert(2) should return NULL"
    );
    assert!(vos_avl3_insert_or_find(&mut tree, &mut n1.node, &info).is_null());
    assert!(vos_avl3_insert_or_find(&mut tree, &mut n3.node, &info).is_null());

    // Find existing key.
    let key2 = 2i32;
    let found = vos_avl3_find(&tree, &key2, &info);
    assert!(found.is_some());
    assert_eq!(*found.unwrap(), 2);

    // Next traversal based on node pointers (matches original AVL3 API usage).
    let n1_next = vos_avl3_next(Some(&n1.node), Some(&info));
    assert!(n1_next.is_some());
    unsafe {
        assert_eq!(key_of_container(n1_next.unwrap()), 2, "Next(1) 应该是 2");
    }
}

#[test]
fn test_avl_delete_updates_links() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: Avl3Tree = Default::default();
    let info = Avl3TreeInfo {
        pf_compare: compare_i32,
        us_key_offset: key_off,
        us_node_offset: node_off,
    };

    let mut n1: NodeWithKey = Default::default();
    let mut n2: NodeWithKey = Default::default();
    let mut n3: NodeWithKey = Default::default();
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    assert!(vos_avl3_insert_or_find(&mut tree, &mut n2.node, &info).is_null());
    assert!(vos_avl3_insert_or_find(&mut tree, &mut n1.node, &info).is_null());
    assert!(vos_avl3_insert_or_find(&mut tree, &mut n3.node, &info).is_null());

    // Delete middle node (key=2)
    let _ = vos_avl3_delete(&mut tree, &mut n2.node);
    let key2 = 2i32;
    let found = vos_avl3_find(&tree, &key2, &info);
    assert!(found.is_none(), "删除后不应再找到 key=2");

    // Now Next(1) should be 3.
    let n1_next = vos_avl3_next(Some(&n1.node), Some(&info));
    assert!(n1_next.is_some());
    unsafe {
        assert_eq!(key_of_container(n1_next.unwrap()), 3);
    }
}

