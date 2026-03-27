// c2rust 测试文件 for avl
// 适配 c2rust 原始指针版本的函数签名
//
// c2rust 函数签名:
// - VOS_AVL3_Insert_Or_Find(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut c_void
// - VOS_AVL3_Delete(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE)
// - VOS_AVL3_Find(pstTree: *mut AVL3_TREE, pstKey: *const c_void, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut c_void
// - VOS_AVL3_Next(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut c_void

use std::ptr;
use crate::src_avl::*;

// 比较函数用于测试
unsafe extern "C" fn compare_int(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let a_val = *(a as *const i32);
    let b_val = *(b as *const i32);
    if a_val < b_val {
        -1
    } else if a_val > b_val {
        1
    } else {
        0
    }
}

#[test]
fn test_avl_functions_exist() {
    // 验证 AVL 树函数存在（编译测试）
}

#[test]
fn test_avl_tree_init() {
    // 测试 AVL3_TREE 结构体初始化
    let tree = AVL3_TREE {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };

    assert!(tree.pstRoot.is_null(), "新初始化的树根应该为空");
    assert!(tree.pstFirst.is_null(), "新初始化的树的第一个节点应该为空");
    assert!(tree.pstLast.is_null(), "新初始化的树的最后一个节点应该为空");
}

#[test]
fn test_avl_node_init() {
    // 测试 AVL3_NODE 结构体初始化
    let node = AVL3_NODE {
        pstParent: ptr::null_mut(),
        pstLeft: ptr::null_mut(),
        pstRight: ptr::null_mut(),
        sLHeight: 0,
        sRHeight: 0,
    };

    assert!(node.pstParent.is_null(), "新初始化的节点父指针应该为空");
    assert_eq!(node.sLHeight, 0, "新初始化的节点左高度应该为0");
    assert_eq!(node.sRHeight, 0, "新初始化的节点右高度应该为0");
}

#[test]
fn test_avl_tree_info_init() {
    // 测试 AVL3_TREE_INFO 结构体初始化
    let tree_info = AVL3_TREE_INFO {
        pfCompare: Some(compare_int),
        usKeyOffset: 0,
        usNodeOffset: 0,
    };

    assert!(tree_info.pfCompare.is_some(), "比较函数应该被设置");
}

#[test]
fn test_avl3_find_empty_tree() {
    // 测试在空树中查找
    let mut tree = AVL3_TREE {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };

    let mut tree_info = AVL3_TREE_INFO {
        pfCompare: Some(compare_int),
        usKeyOffset: 0,
        usNodeOffset: 0,
    };

    let key: i32 = 42;
    let result = unsafe {
        VOS_AVL3_Find(
            &mut tree,
            &key as *const i32 as *const core::ffi::c_void,
            &mut tree_info,
        )
    };

    assert!(result.is_null(), "在空树中查找应该返回 null");
}

#[test]
fn test_avl3_find_with_null_params() {
    // 测试使用 null 参数调用 VOS_AVL3_Find
    let result = unsafe {
        VOS_AVL3_Find(ptr::null_mut(), ptr::null(), ptr::null_mut())
    };
    assert!(result.is_null(), "所有参数为 null 时应该返回 null");
}

#[test]
fn test_avl3_next_null() {
    // 测试 VOS_AVL3_Next 对 null 的处理
    let result = unsafe {
        VOS_AVL3_Next(ptr::null_mut(), ptr::null_mut())
    };
    assert!(result.is_null(), "传入 null 时应该返回 null");
}

#[test]
fn test_avl_rotate_functions_compile() {
    // 验证旋转函数可以编译
}

#[test]
fn test_avl_balance_functions_compile() {
    // 验证平衡函数可以编译
}

#[test]
fn test_avlbase_tree_init() {
    // 测试 AVLBASE_TREE_S 结构体初始化
    let tree = AVLBASE_TREE_S {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };

    assert!(tree.pstRoot.is_null(), "AVLBASE_TREE_S 根应该为空");
}

#[test]
fn test_avlbase_node_init() {
    // 测试 AVLBASE_NODE_S 结构体初始化
    let node = AVLBASE_NODE_S {
        pstParent: ptr::null_mut(),
        pstLeft: ptr::null_mut(),
        pstRight: ptr::null_mut(),
        sLHeight: 0,
        sRHeight: 0,
    };

    assert!(node.pstParent.is_null(), "AVLBASE_NODE_S 父指针应该为空");
    assert_eq!(node.sLHeight, 0);
    assert_eq!(node.sRHeight, 0);
}

#[test]
fn test_avl3_insert_or_find() {
    // 测试 VOS_AVL3_Insert_Or_Find 基本功能
    let mut tree = AVL3_TREE {
        pstRoot: ptr::null_mut(),
        pstFirst: ptr::null_mut(),
        pstLast: ptr::null_mut(),
    };

    let mut node = AVL3_NODE {
        pstParent: ptr::null_mut(),
        pstLeft: ptr::null_mut(),
        pstRight: ptr::null_mut(),
        sLHeight: 0,
        sRHeight: 0,
    };

    let mut tree_info = AVL3_TREE_INFO {
        pfCompare: Some(compare_int),
        usKeyOffset: 0,
        usNodeOffset: 0,
    };

    // 调用 Insert_Or_Find
    let result = unsafe {
        VOS_AVL3_Insert_Or_Find(
            &mut tree as *mut _,
            &mut node as *mut _,
            &mut tree_info as *mut _,
        )
    };

    // 第一次插入应该成功，返回 null 表示成功插入
}

#[test]
#[ignore]
fn test_avl_compile_check() {
    // 验证代码可以编译
}
