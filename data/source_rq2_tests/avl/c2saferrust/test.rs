// c2saferrust 测试文件 for avl (AVL3 API)
// 目标：真实功能测试（插入/查找/遍历/删除）。
//
// 说明：使用 AVL3 接口（VOS_AVL3_*），通过 usKeyOffset/usNodeOffset 指向包含 key 的用户结构体。

use crate::*;
use core::ffi::c_void;
use std::ptr::{self, NonNull};

#[repr(C)]
struct NodeWithKey {
    node: AVL3_NODE,
    key: i32,
}

fn node_key_offsets() -> (u16, u16) {
    let dummy = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 0,
    };

    let base = (&dummy as *const NodeWithKey as usize) as isize;
    let node_off = (&dummy.node as *const AVL3_NODE as usize) as isize - base;
    let key_off = (&dummy.key as *const i32 as usize) as isize - base;
    (key_off as u16, node_off as u16)
}

unsafe fn container_from_node<'a>(node: *mut AVL3_NODE, node_off: u16) -> *mut NodeWithKey {
    (node as *mut u8).offset(-(node_off as isize)) as *mut NodeWithKey
}

unsafe extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> core::ffi::c_int {
    let av = *(a as *const i32);
    let bv = *(b as *const i32);
    if av < bv {
        -1
    } else if av > bv {
        1
    } else {
        0
    }
}

#[test]
fn test_avl_insert_find_next_prev() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree = AVL3_TREE {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };
    let mut info = AVL3_TREE_INFO {
        pfCompare: Some(compare_i32),
        usKeyOffset: key_off,
        usNodeOffset: node_off,
    };

    let mut n1 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 1,
    };
    let mut n2 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 2,
    };
    let mut n3 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 3,
    };

    // Insert three nodes.
    let r = VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &mut info);
    assert!(r.is_null(), "insert(2) should return NULL");
    assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &mut info).is_null());
    assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &mut info).is_null());

    // Find existing key=2
    let key2 = 2i32;
    let key2_void: *const c_void = (&key2 as *const i32).cast();
    let found = VOS_AVL3_Find(Some(&mut tree), Some(unsafe { &*key2_void }), Some(&info));
    assert!(found.is_some());
    unsafe {
        let found_ptr = found.unwrap().as_ptr();
        let found_key = (*(found_ptr as *mut NodeWithKey)).key;
        assert_eq!(found_key, 2);
    }

    // Next traversal based on node pointers
    let n1_next = VOS_AVL3_Next(
        NonNull::new(&mut n1.node as *mut AVL3_NODE),
        NonNull::new(&mut info as *mut AVL3_TREE_INFO),
    );
    assert!(n1_next.is_some());
    unsafe {
        let next_ptr = n1_next.unwrap().as_ptr();
        let next_key = (*(next_ptr as *mut NodeWithKey)).key;
        assert_eq!(next_key, 2, "Next(1) 应该是 2");
    }

    let n2_next = VOS_AVL3_Next(
        NonNull::new(&mut n2.node as *mut AVL3_NODE),
        NonNull::new(&mut info as *mut AVL3_TREE_INFO),
    );
    assert!(n2_next.is_some());
    unsafe {
        let next_ptr = n2_next.unwrap().as_ptr();
        let next_key = (*(next_ptr as *mut NodeWithKey)).key;
        assert_eq!(next_key, 3, "Next(2) 应该是 3");
    }

    // First/Last pointers maintained on tree
    assert!(!tree.pstFirst.is_null());
    assert!(!tree.pstLast.is_null());
    unsafe {
        let first = container_from_node(tree.pstFirst, node_off);
        let last = container_from_node(tree.pstLast, node_off);
        assert_eq!((*first).key, 1);
        assert_eq!((*last).key, 3);
    }
}

#[test]
fn test_avl_delete_updates_links() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree = AVL3_TREE {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };
    let mut info = AVL3_TREE_INFO {
        pfCompare: Some(compare_i32),
        usKeyOffset: key_off,
        usNodeOffset: node_off,
    };

    let mut n1 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 1,
    };
    let mut n2 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 2,
    };
    let mut n3 = NodeWithKey {
        node: AVL3_NODE {
            pstParent: ptr::null_mut(),
            pstLeft: ptr::null_mut(),
            pstRight: ptr::null_mut(),
            sLHeight: 0,
            sRHeight: 0,
        },
        key: 3,
    };

    assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &mut info).is_null());
    assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &mut info).is_null());
    assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &mut info).is_null());

    // Delete middle node (key=2)
    VOS_AVL3_Delete(Some(&mut tree), Some(&mut n2.node));
    let key2 = 2i32;
    let key2_void: *const c_void = (&key2 as *const i32).cast();
    let found = VOS_AVL3_Find(Some(&mut tree), Some(unsafe { &*key2_void }), Some(&info));
    assert!(found.is_none(), "删除后不应再找到 key=2");

    // Now Next(1) should be 3
    let n1_next = VOS_AVL3_Next(
        NonNull::new(&mut n1.node as *mut AVL3_NODE),
        NonNull::new(&mut info as *mut AVL3_TREE_INFO),
    );
    assert!(n1_next.is_some());
    unsafe {
        let next_ptr = n1_next.unwrap().as_ptr();
        let next_key = (*(next_ptr as *mut NodeWithKey)).key;
        assert_eq!(next_key, 3);
    }

    // Delete remaining nodes -> tree becomes empty
    VOS_AVL3_Delete(Some(&mut tree), Some(&mut n1.node));
    VOS_AVL3_Delete(Some(&mut tree), Some(&mut n3.node));

    assert!(tree.pstRoot.is_null());
    assert!(tree.pstFirst.is_null());
    assert!(tree.pstLast.is_null());
}
