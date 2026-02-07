//! Module: src_avl
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn AVL3_Find_Or_Find_Next(pstTree: *mut AVL3_TREE, pKey: *const ::core::ffi::c_void, bFlag: ::core::ffi::c_uint, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE;
    let mut pFoundNode: *mut ::core::ffi::c_void = std::ptr::null_mut();
    let mut iResult: i32;
    let mut iKeyOffset: i32;
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        pstNode = (*pstTree).pstRoot;
    }
    if pstNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        iKeyOffset = ((*pstTreeInfo).usKeyOffset as i32 - (*pstTreeInfo).usNodeOffset as i32);
    }
    loop {
        unsafe {
            let key_ptr = (pstNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            if let Some(pfCompare) = (*pstTreeInfo).pfCompare {
                iResult = pfCompare(pKey, key_ptr);
            } else {
                return std::ptr::null_mut();
            }
        }
        if iResult > 0 {
            unsafe {
                if (*pstNode).pstRight.is_null() {
                    pFoundNode = crate::src_avl::VOS_AVL3_Next(pstNode, pstTreeInfo);
                    break;
                }
                pstNode = (*pstNode).pstRight;
            }
        } else if iResult < 0 {
            unsafe {
                if (*pstNode).pstLeft.is_null() {
                    pFoundNode = (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void;
                    break;
                }
                pstNode = (*pstNode).pstLeft;
            }
        } else {
            if bFlag != 0 {
                pFoundNode = crate::src_avl::VOS_AVL3_Next(pstNode, pstTreeInfo);
            } else {
                unsafe {
                    pFoundNode = (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void;
                }
            }
            break;
        }
    }
    pFoundNode
}

pub extern "C" fn VOS_AVL3_Insert_Or_Find(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() || pstNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*pstNode).sRHeight = 0;
        (*pstNode).sLHeight = 0;
        if (*pstTree).pstRoot.is_null() {
            (*pstTree).pstRoot = pstNode;
            (*pstTree).pstFirst = pstNode;
            (*pstTree).pstLast = pstNode;
            return std::ptr::null_mut();
        }
        let mut pstParentNode = (*pstTree).pstRoot;
        let iKeyOffset = ((*pstTreeInfo).usKeyOffset as i32 - (*pstTreeInfo).usNodeOffset as i32);
        while !pstParentNode.is_null() {
            let pfCompare = (*pstTreeInfo).pfCompare;
            let node_key = (pstNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            let parent_key = (pstParentNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            let iResult = if let Some(f) = pfCompare {
                f(node_key, parent_key)
            } else {
                0
            };
            if iResult > 0 {
                if !(*pstParentNode).pstRight.is_null() {
                    pstParentNode = (*pstParentNode).pstRight;
                    continue;
                }
                crate::src_avl::VosAvlNodeRightInsert(pstTree as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S, pstNode as *mut crate::types::AVLBASE_NODE_S);
            } else if iResult < 0 {
                if !(*pstParentNode).pstLeft.is_null() {
                    pstParentNode = (*pstParentNode).pstLeft;
                    continue;
                }
                crate::src_avl::VosAvlNodeLeftInsert(pstTree as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S, pstNode as *mut crate::types::AVLBASE_NODE_S);
            } else {
                (*pstNode).sRHeight = -1;
                (*pstNode).sLHeight = -1;
                let offset = (*pstTreeInfo).usNodeOffset as isize;
                return (pstParentNode as *mut u8).offset(-offset) as *mut ::core::ffi::c_void;
            }
            break;
        }
        crate::src_avl::VosAvlBalanceTree(pstTree as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S);
        std::ptr::null_mut()
    }
}

pub extern "C" fn VOS_AVL3_Delete(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE) {
    if pstTree.is_null() || pstNode.is_null() {
        return;
    }
    let pstBaseNode = pstNode as *mut crate::types::AVLBASE_NODE_S;
    let pstBaseTree = pstTree as *mut crate::types::AVLBASE_TREE_S;
    unsafe {
        crate::src_avl::VosAvlDelete(pstBaseNode, pstBaseTree);
    }
}

pub extern "C" fn VOS_AVL3_Find(pstTree: *mut AVL3_TREE, pstKey: *const ::core::ffi::c_void, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    let mut pstNode: *mut AVL3_NODE = unsafe { (*pstTree).pstRoot };
    let iKeyOffset: i32 = unsafe { ((*pstTreeInfo).usKeyOffset as i32) - ((*pstTreeInfo).usNodeOffset as i32) };
    while !pstNode.is_null() {
        let key_ptr: *const ::core::ffi::c_void = unsafe { (pstNode as *mut u8).offset(iKeyOffset as isize) as *const ::core::ffi::c_void };
        let pfCompare: Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32> = unsafe { (*pstTreeInfo).pfCompare };
        let iResult: i32 = if let Some(f) = pfCompare {
            unsafe { f(pstKey, key_ptr) }
        } else {
            0
        };
        if iResult > 0 {
            pstNode = unsafe { (*pstNode).pstRight };
        } else if iResult < 0 {
            pstNode = unsafe { (*pstNode).pstLeft };
        } else {
            break;
        }
    }
    if pstNode.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void }
    }
}

pub extern "C" fn VOS_AVL3_First(pstTree: *mut AVL3_TREE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    let pstNode = unsafe { (*pstTree).pstFirst };
    if pstNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let offset = (*pstTreeInfo).usNodeOffset as usize;
        (pstNode as *mut u8).offset(-(offset as isize)) as *mut ::core::ffi::c_void
    }
}

pub extern "C" fn VOS_AVL3_Last(pstTree: *mut AVL3_TREE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    let pstNode = unsafe { (*pstTree).pstLast };
    if pstNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let offset = (*pstTreeInfo).usNodeOffset as usize;
        (pstNode as *mut u8).offset(-(offset as isize)) as *mut ::core::ffi::c_void
    }
}

pub extern "C" fn VOS_AVL3_Next(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp = pstNode;
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        if !(*pstNodeTmp).pstRight.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstRight;
            while !(*pstNodeTmp).pstLeft.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstLeft;
            }
        } else {
            while !pstNodeTmp.is_null() {
                let parent = (*pstNodeTmp).pstParent;
                if parent.is_null() || (*parent).pstLeft == pstNodeTmp {
                    pstNodeTmp = parent;
                    break;
                }
                pstNodeTmp = parent;
            }
        }
        if pstNodeTmp.is_null() {
            std::ptr::null_mut()
        } else {
            let offset = (*pstTreeInfo).usNodeOffset as usize;
            (pstNodeTmp as *mut u8).offset(-(offset as isize)) as *mut ::core::ffi::c_void
        }
    }
}

pub extern "C" fn VOS_AVL3_Prev(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp = pstNode;
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        if (*pstNodeTmp).pstLeft.is_null() {
            while !pstNodeTmp.is_null() {
                let pstParent = (*pstNodeTmp).pstParent;
                if pstParent.is_null() || (*pstParent).pstRight == pstNodeTmp {
                    pstNodeTmp = pstParent;
                    break;
                }
                pstNodeTmp = pstParent;
            }
        } else {
            pstNodeTmp = (*pstNodeTmp).pstLeft;
            while !(*pstNodeTmp).pstRight.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstRight;
            }
        }
        if pstNodeTmp.is_null() {
            std::ptr::null_mut()
        } else {
            let offset = (*pstTreeInfo).usNodeOffset as usize;
            (pstNodeTmp as *mut u8).sub(offset) as *mut ::core::ffi::c_void
        }
    }
}

fn VosAvlNodeRightInsert(pstTree: *mut crate::types::AVLBASE_TREE_S, pstParentNode: *mut crate::types::AVLBASE_NODE_S, pstNode: *mut crate::types::AVLBASE_NODE_S) {
unsafe {
    (*pstNode).pstParent = pstParentNode;
    (*pstParentNode).pstRight = pstNode;
    (*pstParentNode).sRHeight = 1;
    if pstParentNode == (*pstTree).pstLast {
        (*pstTree).pstLast = pstNode;
    }
}
}

fn VosAvlNodeLeftInsert(pstTree: *mut crate::types::AVLBASE_TREE_S, pstParentNode: *mut crate::types::AVLBASE_NODE_S, pstNode: *mut crate::types::AVLBASE_NODE_S) {
    unsafe {
        (*pstNode).pstParent = pstParentNode;
        (*pstParentNode).pstLeft = pstNode;
        (*pstParentNode).sLHeight = 1;
        if pstParentNode == (*pstTree).pstFirst {
            (*pstTree).pstFirst = pstNode;
        }
    }
}

pub extern "C" fn VosAvlRotateRight(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pstSubTree = *ppstSubTree;
        let pstLeftSon = (*pstSubTree).pstLeft;
        (*pstSubTree).pstLeft = (*pstLeftSon).pstRight;
        if !(*pstSubTree).pstLeft.is_null() {
            (*(*pstSubTree).pstLeft).pstParent = pstSubTree;
        }
        (*pstSubTree).sLHeight = (*pstLeftSon).sRHeight;
        (*pstLeftSon).pstParent = (*pstSubTree).pstParent;
        (*pstLeftSon).pstRight = pstSubTree;
        (*(*pstLeftSon).pstRight).pstParent = pstLeftSon;
        let left_height = (*pstSubTree).sLHeight;
        let right_height = (*pstSubTree).sRHeight;
        let max_height = if left_height > right_height { left_height } else { right_height };
        (*pstLeftSon).sRHeight = 1 + max_height;
        *ppstSubTree = pstLeftSon;
    }
}

pub extern "C" fn VosAvlRotateLeft(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pstSubTree = *ppstSubTree;
        let pstRightSon = (*pstSubTree).pstRight;
        (*pstSubTree).pstRight = (*pstRightSon).pstLeft;
        if !(*pstSubTree).pstRight.is_null() {
            (*(*pstSubTree).pstRight).pstParent = pstSubTree;
        }
        (*pstSubTree).sRHeight = (*pstRightSon).sLHeight;
        (*pstRightSon).pstParent = (*pstSubTree).pstParent;
        (*pstRightSon).pstLeft = pstSubTree;
        (*(*pstRightSon).pstLeft).pstParent = pstRightSon;
        let left_height = (*pstSubTree).sLHeight;
        let right_height = (*pstSubTree).sRHeight;
        let max_height = if left_height > right_height {
            left_height
        } else {
            right_height
        };
        (*pstRightSon).sLHeight = 1 + max_height;
        *ppstSubTree = pstRightSon;
    }
}

pub extern "C" fn VosAvlUpdateSwapNode(pstTree: *mut AVLBASE_TREE_S, pstSwapNode: *mut AVLBASE_NODE_S, pstBaseNode: *const AVLBASE_NODE_S) {
    unsafe {
        (*pstSwapNode).pstParent = (*pstBaseNode).pstParent;
        (*pstSwapNode).pstRight = (*pstBaseNode).pstRight;
        (*pstSwapNode).pstLeft = (*pstBaseNode).pstLeft;
        (*pstSwapNode).sRHeight = (*pstBaseNode).sRHeight;
        (*pstSwapNode).sLHeight = (*pstBaseNode).sLHeight;
        if !(*pstSwapNode).pstRight.is_null() {
            (*(*pstSwapNode).pstRight).pstParent = pstSwapNode;
        }
        if !(*pstSwapNode).pstLeft.is_null() {
            (*(*pstSwapNode).pstLeft).pstParent = pstSwapNode;
        }
        if (*pstBaseNode).pstParent.is_null() {
            (*pstTree).pstRoot = pstSwapNode;
        } else if (*(*pstBaseNode).pstParent).pstRight == pstBaseNode as *mut crate::types::AVLBASE_NODE_S {
            (*(*pstSwapNode).pstParent).pstRight = pstSwapNode;
        } else {
            (*(*pstSwapNode).pstParent).pstLeft = pstSwapNode;
        }
    }
}

pub extern "C" fn VosAvlMoveNodeToNewPos(pstNode: *mut AVLBASE_NODE_S, pstNewParent: *mut AVLBASE_NODE_S, pstNewLeftSon: *mut AVLBASE_NODE_S, pstNewRightSon: *mut AVLBASE_NODE_S) {
    unsafe {
        (*pstNode).pstParent = pstNewParent;
        (*pstNode).pstLeft = pstNewLeftSon;
        (*pstNode).pstRight = pstNewRightSon;
        (*pstNode).sLHeight = 0;
        (*pstNode).sRHeight = 0;
        if !pstNewLeftSon.is_null() {
            (*pstNewLeftSon).pstParent = pstNode;
            (*pstNode).sLHeight = 1;
        }
        if !pstNewRightSon.is_null() {
            (*pstNewRightSon).pstParent = pstNode;
            (*pstNode).sRHeight = 1;
        }
    }
}

pub extern "C" fn VosAvlSwapRightMost(pstTree: *mut AVLBASE_TREE_S, pstSubTree: *mut AVLBASE_NODE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstSwapNode = pstSubTree;
    unsafe {
        while !(*pstSwapNode).pstRight.is_null() {
            pstSwapNode = (*pstSwapNode).pstRight;
        }
    }
    unsafe {
        if (*pstSwapNode).sRHeight != 0 || (*pstSwapNode).sLHeight > 1 {
            return;
        }
    }
    let pstSwapParent: *mut crate::types::AVLBASE_NODE_S;
    let pstSwapLeft: *mut crate::types::AVLBASE_NODE_S;
    unsafe {
        pstSwapParent = (*pstSwapNode).pstParent;
        pstSwapLeft = (*pstSwapNode).pstLeft;
    }
    crate::src_avl::VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode as *const _);
    crate::src_avl::VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, std::ptr::null_mut());
    unsafe {
        (*pstNode).pstParent.as_mut().unwrap().pstRight = pstNode;
    }
}

pub extern "C" fn VosAvlSwapLeftMost(pstTree: *mut AVLBASE_TREE_S, pstSubTree: *mut AVLBASE_NODE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstSwapNode = pstSubTree;
    unsafe {
        while !(*pstSwapNode).pstLeft.is_null() {
            pstSwapNode = (*pstSwapNode).pstLeft;
        }
    }
    unsafe {
        if (*pstSwapNode).sLHeight != 0 || (*pstSwapNode).sRHeight > 1 {
            return;
        }
    }
    let pstSwapParent: *mut crate::types::AVLBASE_NODE_S;
    let pstSwapRight: *mut crate::types::AVLBASE_NODE_S;
    unsafe {
        pstSwapParent = (*pstSwapNode).pstParent;
        pstSwapRight = (*pstSwapNode).pstRight;
    }
    crate::src_avl::VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode as *const _);
    crate::src_avl::VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, std::ptr::null_mut(), pstSwapRight);
    unsafe {
        (*(*pstNode).pstParent).pstLeft = pstNode;
    }
}

pub extern "C" fn VosAvlRebalance(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pst_sub_tree = *ppstSubTree;
        let i_moment = (*pst_sub_tree).sRHeight - (*pst_sub_tree).sLHeight;
        if i_moment > 1 {
            let pst_right = (*pst_sub_tree).pstRight;
            if (*pst_right).sLHeight > (*pst_right).sRHeight {
                crate::src_avl::VosAvlRotateRight(&mut (*pst_sub_tree).pstRight);
            }
            crate::src_avl::VosAvlRotateLeft(ppstSubTree);
        } else if i_moment < -1 {
            let pst_left = (*pst_sub_tree).pstLeft;
            if (*pst_left).sRHeight > (*pst_left).sLHeight {
                crate::src_avl::VosAvlRotateLeft(&mut (*pst_sub_tree).pstLeft);
            }
            crate::src_avl::VosAvlRotateRight(ppstSubTree);
        }
    }
}

pub extern "C" fn VosAvlBalanceTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstNodeTmp = pstNode;
    unsafe {
        while !(*pstNodeTmp).pstParent.is_null() {
            if (*pstNodeTmp).pstParent.as_ref().unwrap().pstRight == pstNodeTmp {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                crate::src_avl::VosAvlRebalance(&mut (*pstNodeTmp).pstRight);
                let right = (*pstNodeTmp).pstRight.as_ref().unwrap();
                let max = if right.sRHeight > right.sLHeight {
                    right.sRHeight
                } else {
                    right.sLHeight
                };
                (*pstNodeTmp).sRHeight = 1 + max;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                crate::src_avl::VosAvlRebalance(&mut (*pstNodeTmp).pstLeft);
                let left = (*pstNodeTmp).pstLeft.as_ref().unwrap();
                let max = if left.sRHeight > left.sLHeight {
                    left.sRHeight
                } else {
                    left.sLHeight
                };
                (*pstNodeTmp).sLHeight = 1 + max;
            }
        }
        if (*pstNodeTmp).sLHeight != (*pstNodeTmp).sRHeight {
            crate::src_avl::VosAvlRebalance(&mut (*pstTree).pstRoot);
        }
    }
}

pub extern "C" fn VosAVLSearchReplaceNodeInRTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut crate::types::AVLBASE_NODE_S;
    unsafe {
        if (*pstNode).pstRight.is_null() {
            return std::ptr::null_mut();
        }
        if (*(*pstNode).pstRight).pstLeft.is_null() {
            pstReplaceNode = (*pstNode).pstRight;
            (*pstReplaceNode).pstLeft = (*pstNode).pstLeft;
            if !(*pstReplaceNode).pstLeft.is_null() {
                (*(*pstReplaceNode).pstLeft).pstParent = pstReplaceNode;
            }
            (*pstReplaceNode).sLHeight = (*pstNode).sLHeight;
        } else {
            crate::src_avl::VosAvlSwapLeftMost(pstTree, (*pstNode).pstRight, pstNode);
            pstReplaceNode = (*pstNode).pstRight;
        }
    }
    pstReplaceNode
}

pub extern "C" fn VosAvlSearchReplaceNodeInLTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut crate::types::AVLBASE_NODE_S;
    unsafe {
        if (*pstNode).pstLeft.is_null() {
            return std::ptr::null_mut();
        }
        if (*(*pstNode).pstLeft).pstRight.is_null() {
            pstReplaceNode = (*pstNode).pstLeft;
            (*pstReplaceNode).pstRight = (*pstNode).pstRight;
            if !(*pstReplaceNode).pstRight.is_null() {
                (*(*pstReplaceNode).pstRight).pstParent = pstReplaceNode;
            }
            (*pstReplaceNode).sRHeight = (*pstNode).sRHeight;
        } else {
            crate::src_avl::VosAvlSwapRightMost(pstTree, (*pstNode).pstLeft, pstNode);
            pstReplaceNode = (*pstNode).pstLeft;
        }
    }
    pstReplaceNode
}

pub extern "C" fn VosAvlSearchReplaceNode(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = std::ptr::null_mut();
    unsafe {
        if (*pstNode).sRHeight > (*pstNode).sLHeight {
            pstReplaceNode = crate::src_avl::VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
        } else {
            pstReplaceNode = crate::src_avl::VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
        }
    }
    pstReplaceNode
}

pub extern "C" fn VosAvlDeleteCheck(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut crate::types::AVLBASE_NODE_S = std::ptr::null_mut();
    unsafe {
        if ((*pstNode).pstLeft).is_null() && ((*pstNode).pstRight).is_null() {
            pstReplaceNode = std::ptr::null_mut();
            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = (*pstNode).pstParent;
            }
            if (*pstTree).pstLast == pstNode {
                (*pstTree).pstLast = (*pstNode).pstParent;
            }
        } else if ((*pstNode).pstLeft).is_null() {
            pstReplaceNode = (*pstNode).pstRight;
            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = pstReplaceNode;
            }
        } else if ((*pstNode).pstRight).is_null() {
            pstReplaceNode = (*pstNode).pstLeft;
            if (*pstTree).pstLast == pstNode {
                (*pstTree).pstLast = pstReplaceNode;
            }
        } else {
            pstReplaceNode = crate::src_avl::VosAvlSearchReplaceNode(pstTree, pstNode);
        }
    }
    pstReplaceNode
}

pub extern "C" fn VosAvlDelete(pstBaseNode: *mut AVLBASE_NODE_S, pstBaseTree: *mut AVLBASE_TREE_S) {
    let mut pstReplaceNode: *mut crate::types::AVLBASE_NODE_S;
    let mut pstParentNode: *mut crate::types::AVLBASE_NODE_S;
    let mut sNewHeight: i16 = 0;
    unsafe {
        pstReplaceNode = crate::src_avl::VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
        pstParentNode = (*pstBaseNode).pstParent;
        (*pstBaseNode).pstParent = std::ptr::null_mut();
        (*pstBaseNode).pstRight = std::ptr::null_mut();
        (*pstBaseNode).pstLeft = std::ptr::null_mut();
        (*pstBaseNode).sRHeight = -1;
        (*pstBaseNode).sLHeight = -1;
        if !pstReplaceNode.is_null() {
            (*pstReplaceNode).pstParent = pstParentNode;
            let lh = (*pstReplaceNode).sLHeight;
            let rh = (*pstReplaceNode).sRHeight;
            sNewHeight = 1 + if lh > rh { lh } else { rh };
        }
        if !pstParentNode.is_null() {
            if (*pstParentNode).pstRight == pstBaseNode {
                (*pstParentNode).pstRight = pstReplaceNode;
                (*pstParentNode).sRHeight = sNewHeight;
            } else {
                (*pstParentNode).pstLeft = pstReplaceNode;
                (*pstParentNode).sLHeight = sNewHeight;
            }
            crate::src_avl::VosAvlBalanceTree(pstBaseTree, pstParentNode);
        } else {
            (*pstBaseTree).pstRoot = pstReplaceNode;
        }
    }
}

pub extern "C" fn VOS_AVL_Insert_Or_Find(pstTree: *mut AVL_TREE, pstNode: *mut AVL_NODE) -> *mut ::core::ffi::c_void {
    unsafe {
        if pstTree.is_null() || pstNode.is_null() || ((*pstNode).sLHeight != -1 && (*pstNode).sRHeight != -1) {
            return std::ptr::null_mut();
        }
        (*pstNode).sRHeight = 0;
        (*pstNode).sLHeight = 0;
        if (*pstTree).pstRoot.is_null() {
            (*pstTree).pstRoot = pstNode;
            (*pstTree).pstFirst = pstNode;
            (*pstTree).pstLast = pstNode;
            return std::ptr::null_mut();
        }
        let mut pstParentNode = (*pstTree).pstRoot;
        while !pstParentNode.is_null() {
            let iResult = if let Some(f) = (*pstTree).pfnCompare {
                f((*pstNode).pKey, (*pstParentNode).pKey)
            } else {
                0
            };
            if iResult > 0 {
                if !(*pstParentNode).pstRight.is_null() {
                    pstParentNode = (*pstParentNode).pstRight;
                    continue;
                }
                crate::src_avl::VosAvlNodeRightInsert((&mut (*pstTree).pstRoot) as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S, pstNode as *mut crate::types::AVLBASE_NODE_S);
                break;
            } else if iResult < 0 {
                if !(*pstParentNode).pstLeft.is_null() {
                    pstParentNode = (*pstParentNode).pstLeft;
                    continue;
                }
                crate::src_avl::VosAvlNodeLeftInsert((&mut (*pstTree).pstRoot) as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S, pstNode as *mut crate::types::AVLBASE_NODE_S);
                break;
            }
            (*pstNode).sRHeight = -1;
            (*pstNode).sLHeight = -1;
            return (*pstParentNode).pSelf;
        }
        if !pstParentNode.is_null() {
            crate::src_avl::VosAvlBalanceTree((&mut (*pstTree).pstRoot) as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S, pstParentNode as *mut crate::types::AVLBASE_NODE_S);
        }
        std::ptr::null_mut()
    }
}

pub extern "C" fn VOS_AVL_Delete(pstTree: *mut AVL_TREE, pstNode: *mut AVL_NODE) {
    if pstTree.is_null() || pstNode.is_null() {
        return;
    }
    unsafe {
        if (*pstNode).sLHeight == -1 || (*pstNode).sRHeight == -1 {
            return;
        }
        let pstBaseNode = pstNode as *mut AVLBASE_NODE_S;
        let pstBaseTree = (&mut (*pstTree).pstRoot) as *mut *mut AVL_NODE as *mut AVLBASE_TREE_S;
        VosAvlDelete(pstBaseNode, pstBaseTree);
    }
}

pub extern "C" fn VOS_AVL_Find(pstTree: *mut AVL_TREE, pKey: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() {
        return std::ptr::null_mut();
    }
    let mut pstNode: *mut crate::types::AVL_NODE = unsafe { (*pstTree).pstRoot };
    while !pstNode.is_null() {
        let pfnCompare = unsafe { (*pstTree).pfnCompare };
        if let Some(compare_fn) = pfnCompare {
            let iResult = unsafe { compare_fn(pKey, (*pstNode).pKey) };
            if iResult > 0 {
                pstNode = unsafe { (*pstNode).pstRight };
            } else if iResult < 0 {
                pstNode = unsafe { (*pstNode).pstLeft };
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if pstNode.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*pstNode).pSelf }
    }
}

pub extern "C" fn VOS_AVL_Next(pstNode: *mut AVL_NODE) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp = pstNode;
    if pstNodeTmp.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        if !(((*pstNodeTmp).sLHeight != -1) && ((*pstNodeTmp).sRHeight != -1)) {
            return std::ptr::null_mut();
        }
        if !(*pstNodeTmp).pstRight.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstRight;
            while !(*pstNodeTmp).pstLeft.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstLeft;
            }
        } else {
            while !pstNodeTmp.is_null() {
                if (*pstNodeTmp).pstParent.is_null() || (*((*pstNodeTmp).pstParent)).pstLeft == pstNodeTmp {
                    pstNodeTmp = (*pstNodeTmp).pstParent;
                    break;
                }
                pstNodeTmp = (*pstNodeTmp).pstParent;
            }
        }
        if pstNodeTmp.is_null() {
            std::ptr::null_mut()
        } else {
            (*pstNodeTmp).pSelf
        }
    }
}

pub extern "C" fn VOS_AVL_Prev(pstNode: *mut AVL_NODE) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp = pstNode;
    if pstNodeTmp.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        if !(((*pstNodeTmp).sLHeight != -1) && ((*pstNodeTmp).sRHeight != -1)) {
            return std::ptr::null_mut();
        }
        if (*pstNodeTmp).pstLeft.is_null() {
            while !pstNodeTmp.is_null() {
                let pstParent = (*pstNodeTmp).pstParent;
                if pstParent.is_null() || (*pstParent).pstRight == pstNodeTmp {
                    pstNodeTmp = pstParent;
                    break;
                }
                pstNodeTmp = pstParent;
            }
        } else {
            pstNodeTmp = (*pstNodeTmp).pstLeft;
            while !(*pstNodeTmp).pstRight.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstRight;
            }
        }
        if pstNodeTmp.is_null() {
            std::ptr::null_mut()
        } else {
            (*pstNodeTmp).pSelf
        }
    }
}

pub extern "C" fn VOS_AVL_Find_Or_Find_Next(pstTree: *mut AVL_TREE, pKey: *const ::core::ffi::c_void, bValue: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void {
    let mut pFoundNode: *mut ::core::ffi::c_void = std::ptr::null_mut();
    if pstTree.is_null() {
        return std::ptr::null_mut();
    }
    let mut pstNode: *mut AVL_NODE = unsafe { (*pstTree).pstRoot };
    if pstNode.is_null() {
        return pFoundNode;
    }
    loop {
        let iResult: i32 = unsafe {
            if let Some(f) = (*pstTree).pfnCompare {
                f(pKey, (*pstNode).pKey)
            } else {
                0
            }
        };
        if iResult > 0 {
            if unsafe { (*pstNode).pstRight }.is_null() {
                pFoundNode = crate::src_avl::VOS_AVL_Next(pstNode);
                break;
            }
            pstNode = unsafe { (*pstNode).pstRight };
        } else if iResult < 0 {
            if unsafe { (*pstNode).pstLeft }.is_null() {
                pFoundNode = unsafe { (*pstNode).pSelf };
                break;
            }
            pstNode = unsafe { (*pstNode).pstLeft };
        } else {
            if bValue != 0 {
                pFoundNode = crate::src_avl::VOS_AVL_Next(pstNode);
            } else {
                pFoundNode = unsafe { (*pstNode).pSelf };
            }
            break;
        }
    }
    pFoundNode
}
