// evo-c2rust-v2 测试文件 for avl (AVL3 API)
// 目标：真实功能测试（插入/查找/遍历/删除）。
//
// 说明：使用 AVL3 接口（VOS_AVL3_*），通过 usKeyOffset/usNodeOffset 指向包含 key 的用户结构体。

use crate::src::avl_c::*;
use crate::translation_utils::*;

#[repr(C)]
#[derive(Default)]
struct NodeWithKey {
    node: AVL3_NODE,
    key: i32,
}

fn node_key_offsets() -> (u16, u16) {
    let dummy = NodeWithKey::default();
    let base = (&dummy as *const NodeWithKey as usize) as isize;
    let node_off = (&dummy.node as *const AVL3_NODE as usize) as isize - base;
    let key_off = (&dummy.key as *const i32 as usize) as isize - base;
    (key_off as u16, node_off as u16)
}

fn compare_i32(mut a: VoidPtr, mut b: VoidPtr) -> i32 {
    let pa: Ptr<i32> = a.cast();
    let pb: Ptr<i32> = b.cast();
    let av = *pa;
    let bv = *pb;
    if av < bv {
        -1
    } else if av > bv {
        1
    } else {
        0
    }
}

fn key_of_container(p: Ptr<Void>) -> i32 {
    let c: Ptr<NodeWithKey> = p.cast();
    c.key
}

#[test]
fn test_avl_insert_find_next_prev() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = Default::default();
    let mut info: AVL3_TREE_INFO = Default::default();
    info.pfCompare = FuncPtr::new(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = Default::default();
    let mut n2: NodeWithKey = Default::default();
    let mut n3: NodeWithKey = Default::default();
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    let tree_ptr: Ptr<AVL3_TREE> = Ptr::new(&mut tree);
    let info_ptr: Ptr<AVL3_TREE_INFO> = Ptr::new(&mut info);
    let n1p: Ptr<AVL3_NODE> = Ptr::new(&mut n1.node);
    let n2p: Ptr<AVL3_NODE> = Ptr::new(&mut n2.node);
    let n3p: Ptr<AVL3_NODE> = Ptr::new(&mut n3.node);

    // Insert three nodes.
    assert!(
        !VOS_AVL3_Insert_Or_Find(tree_ptr, n2p, info_ptr).as_bool(),
        "insert(2) should return NULL"
    );
    assert!(!VOS_AVL3_Insert_Or_Find(tree_ptr, n1p, info_ptr).as_bool());
    assert!(!VOS_AVL3_Insert_Or_Find(tree_ptr, n3p, info_ptr).as_bool());

    // Find existing key
    let mut key2 = 2i32;
    let found = VOS_AVL3_Find(tree_ptr, Ptr::new(&mut key2).cast(), info_ptr);
    assert!(found.as_bool());
    assert_eq!(key_of_container(found), 2);

    // Next/Prev traversal based on node pointers
    let n1_next = VOS_AVL3_Next(n1p, info_ptr);
    assert!(n1_next.as_bool());
    assert_eq!(key_of_container(n1_next), 2, "Next(1) 应该是 2");

    let n2_next = VOS_AVL3_Next(n2p, info_ptr);
    assert!(n2_next.as_bool());
    assert_eq!(key_of_container(n2_next), 3, "Next(2) 应该是 3");

    // First/Last pointers maintained on tree
    assert!(tree.pstFirst.as_bool());
    assert!(tree.pstLast.as_bool());
    let first_key = key_of_container(tree.pstFirst.cast());
    let last_key = key_of_container(tree.pstLast.cast());
    assert_eq!(first_key, 1);
    assert_eq!(last_key, 3);
}

#[test]
fn test_avl_delete_updates_links() {
    let (key_off, node_off) = node_key_offsets();

    let mut tree: AVL3_TREE = Default::default();
    let mut info: AVL3_TREE_INFO = Default::default();
    info.pfCompare = FuncPtr::new(compare_i32);
    info.usKeyOffset = key_off;
    info.usNodeOffset = node_off;

    let mut n1: NodeWithKey = Default::default();
    let mut n2: NodeWithKey = Default::default();
    let mut n3: NodeWithKey = Default::default();
    n1.key = 1;
    n2.key = 2;
    n3.key = 3;

    let tree_ptr: Ptr<AVL3_TREE> = Ptr::new(&mut tree);
    let info_ptr: Ptr<AVL3_TREE_INFO> = Ptr::new(&mut info);
    let n1p: Ptr<AVL3_NODE> = Ptr::new(&mut n1.node);
    let n2p: Ptr<AVL3_NODE> = Ptr::new(&mut n2.node);
    let n3p: Ptr<AVL3_NODE> = Ptr::new(&mut n3.node);

    assert!(!VOS_AVL3_Insert_Or_Find(tree_ptr, n2p, info_ptr).as_bool());
    assert!(!VOS_AVL3_Insert_Or_Find(tree_ptr, n1p, info_ptr).as_bool());
    assert!(!VOS_AVL3_Insert_Or_Find(tree_ptr, n3p, info_ptr).as_bool());

    // Delete middle node (key=2)
    VOS_AVL3_Delete(tree_ptr, n2p);
    let mut key2 = 2i32;
    let found = VOS_AVL3_Find(tree_ptr, Ptr::new(&mut key2).cast(), info_ptr);
    assert!(!found.as_bool(), "删除后不应再找到 key=2");

    // Now Next(1) should be 3
    let n1_next = VOS_AVL3_Next(n1p, info_ptr);
    assert!(n1_next.as_bool());
    assert_eq!(key_of_container(n1_next), 3);

    // Delete remaining nodes -> tree becomes empty
    VOS_AVL3_Delete(tree_ptr, n1p);
    VOS_AVL3_Delete(tree_ptr, n3p);
    assert!(!tree.pstRoot.as_bool());
    assert!(!tree.pstFirst.as_bool());
    assert!(!tree.pstLast.as_bool());
}
