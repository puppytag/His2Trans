// C2R 框架测试文件 for avl (AVL3 API)
//
// 目标：与 evolc2rust/c2saferrust/simcrat 的外部测试语义一致（插入/查找/遍历/删除）。

use crate::src_avl::*;
use crate::types::*;

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
struct NodeWithKey {
    node: AVL3_NODE,
    key: i32,
}

fn node_key_offsets() -> (u16, u16) {
    // Use a stack dummy to compute offsets (matches the C container_of usage).
    let dummy: NodeWithKey = unsafe { core::mem::zeroed() };
    let base = &dummy as *const NodeWithKey as usize;
    let node_off = (&dummy.node as *const AVL3_NODE as usize) - base;
    let key_off = (&dummy.key as *const i32 as usize) - base;
    (key_off as u16, node_off as u16)
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

unsafe fn key_of_container(p: *mut c_void) -> i32 {
    (*(p as *mut NodeWithKey)).key
}

unsafe fn container_from_node(node: *mut AVL3_NODE, node_off: u16) -> *mut NodeWithKey {
    (node as *mut u8).offset(-(node_off as isize)) as *mut NodeWithKey
}

#[test]
fn test_avl_insert_find_next_prev() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = unsafe { core::mem::zeroed() };
    let mut info: AVL3_TREE_INFO = unsafe { core::mem::zeroed() };
    info.pfCompare = Some(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n2: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n3: NodeWithKey = unsafe { core::mem::zeroed() };
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    unsafe {
        // Insert three nodes.
        assert!(
            VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &mut info).is_null(),
            "insert(2) should return NULL"
        );
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &mut info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &mut info).is_null());

        // Find existing key.
        let key2: i32 = 2;
        let found = VOS_AVL3_Find(&mut tree, &key2 as *const _ as *const c_void, &mut info);
        assert!(!found.is_null());
        assert_eq!(key_of_container(found), 2);

        // Next traversal.
        let n1_next = VOS_AVL3_Next(&mut n1.node, &mut info);
        assert!(!n1_next.is_null());
        assert_eq!(key_of_container(n1_next), 2, "Next(1) 应该是 2");

        let n2_next = VOS_AVL3_Next(&mut n2.node, &mut info);
        assert!(!n2_next.is_null());
        assert_eq!(key_of_container(n2_next), 3, "Next(2) 应该是 3");

        // First/Last pointers on tree.
        assert!(!tree.pstFirst.is_null());
        assert!(!tree.pstLast.is_null());
        let first = container_from_node(tree.pstFirst, node_off);
        let last = container_from_node(tree.pstLast, node_off);
        assert!(!first.is_null() && !last.is_null());
        assert_eq!((*first).key, 1);
        assert_eq!((*last).key, 3);
    }
}

#[test]
fn test_avl_delete_updates_links() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = unsafe { core::mem::zeroed() };
    let mut info: AVL3_TREE_INFO = unsafe { core::mem::zeroed() };
    info.pfCompare = Some(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n2: NodeWithKey = unsafe { core::mem::zeroed() };
    let mut n3: NodeWithKey = unsafe { core::mem::zeroed() };
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    unsafe {
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n2.node, &mut info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n1.node, &mut info).is_null());
        assert!(VOS_AVL3_Insert_Or_Find(&mut tree, &mut n3.node, &mut info).is_null());

        // Delete middle node (key=2).
        VOS_AVL3_Delete(&mut tree, &mut n2.node);
        let key2: i32 = 2;
        let found = VOS_AVL3_Find(&mut tree, &key2 as *const _ as *const c_void, &mut info);
        assert_eq!(found, ptr::null_mut(), "删除后不应再找到 key=2");

        // Now Next(1) should be 3.
        let n1_next = VOS_AVL3_Next(&mut n1.node, &mut info);
        assert!(!n1_next.is_null());
        assert_eq!(key_of_container(n1_next), 3);

        // Delete remaining nodes -> tree becomes empty.
        VOS_AVL3_Delete(&mut tree, &mut n1.node);
        VOS_AVL3_Delete(&mut tree, &mut n3.node);
        assert!(tree.pstRoot.is_null());
        assert!(tree.pstFirst.is_null());
        assert!(tree.pstLast.is_null());
    }
}

