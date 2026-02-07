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
    let iResult: i32;
    let iKeyOffset: i32;

    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        pstNode = (*pstTree).pstRoot;
        if pstNode.is_null() {
            return std::ptr::null_mut();
        }

        iKeyOffset = ((*pstTreeInfo).usKeyOffset as i32) - ((*pstTreeInfo).usNodeOffset as i32);

        loop {
            let pfCompare = (*pstTreeInfo).pfCompare;
            let key_ptr = (pstNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            let iResult = if let Some(cmp_fn) = pfCompare {
                cmp_fn(pKey, key_ptr)
            } else {
                0
            };

            if iResult > 0 {
                if (*pstNode).pstRight.is_null() {
                    pFoundNode = crate::src_avl::VOS_AVL3_Next(pstNode, pstTreeInfo);
                    break;
                }
                pstNode = (*pstNode).pstRight;
            } else if iResult < 0 {
                if (*pstNode).pstLeft.is_null() {
                    pFoundNode = (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void;
                    break;
                }
                pstNode = (*pstNode).pstLeft;
            } else {
                if bFlag != 0 {
                    pFoundNode = crate::src_avl::VOS_AVL3_Next(pstNode, pstTreeInfo);
                } else {
                    pFoundNode = (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void;
                }
                break;
            }
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

        let mut pstParentNode: *mut AVL3_NODE = (*pstTree).pstRoot;
        let iKeyOffset: i32 = ((*pstTreeInfo).usKeyOffset as i32) - ((*pstTreeInfo).usNodeOffset as i32);

        while !pstParentNode.is_null() {
            let pNodeKey = (pstNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            let pParentKey = (pstParentNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            
            let pfCompare = (*pstTreeInfo).pfCompare;
            let iResult: i32 = if let Some(cmp_fn) = pfCompare {
                cmp_fn(pNodeKey, pParentKey)
            } else {
                0
            };

            if iResult > 0 {
                if !(*pstParentNode).pstRight.is_null() {
                    pstParentNode = (*pstParentNode).pstRight;
                    continue;
                }
                crate::src_avl::VosAvlNodeRightInsert(
                    pstTree as *mut crate::types::AVLBASE_TREE_S,
                    pstParentNode as *mut crate::types::AVLBASE_NODE_S,
                    pstNode as *mut crate::types::AVLBASE_NODE_S,
                );
            } else if iResult < 0 {
                if !(*pstParentNode).pstLeft.is_null() {
                    pstParentNode = (*pstParentNode).pstLeft;
                    continue;
                }
                crate::src_avl::VosAvlNodeLeftInsert(
                    pstTree as *mut crate::types::AVLBASE_TREE_S,
                    pstParentNode as *mut crate::types::AVLBASE_NODE_S,
                    pstNode as *mut crate::types::AVLBASE_NODE_S,
                );
            } else {
                (*pstNode).sRHeight = -1;
                (*pstNode).sLHeight = -1;
                return (pstParentNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void;
            }
            break;
        }

        crate::src_avl::VosAvlBalanceTree(
            pstTree as *mut crate::types::AVLBASE_TREE_S,
            pstParentNode as *mut crate::types::AVLBASE_NODE_S,
        );
    }

    std::ptr::null_mut()
}

pub extern "C" fn VOS_AVL3_Delete(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE) {
    use crate::types::{AVLBASE_NODE_S, AVLBASE_TREE_S};
    
    if pstTree.is_null() || pstNode.is_null() {
        return;
    }
    
    let pstBaseNode: *mut AVLBASE_NODE_S = pstNode as *mut AVLBASE_NODE_S;
    let pstBaseTree: *mut AVLBASE_TREE_S = pstTree as *mut AVLBASE_TREE_S;
    
    crate::src_avl::VosAvlDelete(pstBaseNode, pstBaseTree);
}

pub extern "C" fn VOS_AVL3_Find(pstTree: *mut AVL3_TREE, pstKey: *const ::core::ffi::c_void, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let mut pstNode: *mut AVL3_NODE = (*pstTree).pstRoot;
        let iKeyOffset: i32 = ((*pstTreeInfo).usKeyOffset as i32) - ((*pstTreeInfo).usNodeOffset as i32);
        
        while !pstNode.is_null() {
            let key_ptr = (pstNode as *mut u8).offset(iKeyOffset as isize) as *mut ::core::ffi::c_void;
            let iResult: i32 = if let Some(compare_fn) = (*pstTreeInfo).pfCompare {
                compare_fn(pstKey, key_ptr)
            } else {
                break;
            };
            
            if iResult > 0 {
                pstNode = (*pstNode).pstRight;
            } else if iResult < 0 {
                pstNode = (*pstNode).pstLeft;
            } else {
                break;
            }
        }
        
        if !pstNode.is_null() {
            (pstNode as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL3_First(pstTree: *mut AVL3_TREE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let pstNode = (*pstTree).pstFirst;
        if !pstNode.is_null() {
            let offset = (*pstTreeInfo).usNodeOffset;
            (pstNode as *mut u8).sub(offset as usize) as *mut ::core::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL3_Last(pstTree: *mut AVL3_TREE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let pstNode = (*pstTree).pstLast;
        if !pstNode.is_null() {
            let offset = (*pstTreeInfo).usNodeOffset;
            (pstNode as *mut u8).sub(offset as usize) as *mut ::core::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL3_Next(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode;
    
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
                if (*pstNodeTmp).pstParent.is_null() || (*(*pstNodeTmp).pstParent).pstLeft == pstNodeTmp {
                    pstNodeTmp = (*pstNodeTmp).pstParent;
                    break;
                }
                pstNodeTmp = (*pstNodeTmp).pstParent;
            }
        }
        
        if !pstNodeTmp.is_null() {
            (pstNodeTmp as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL3_Prev(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode;
    
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if !(*pstNodeTmp).pstLeft.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstLeft;
            while !(*pstNodeTmp).pstRight.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstRight;
            }
        } else {
            while !pstNodeTmp.is_null() {
                if (*pstNodeTmp).pstParent.is_null() || (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp {
                    pstNodeTmp = (*pstNodeTmp).pstParent;
                    break;
                }
                pstNodeTmp = (*pstNodeTmp).pstParent;
            }
        }
        
        if !pstNodeTmp.is_null() {
            (pstNodeTmp as *mut u8).offset(-((*pstTreeInfo).usNodeOffset as isize)) as *mut ::core::ffi::c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

fn VosAvlNodeRightInsert(pstTree: *mut crate::types::AVLBASE_TREE_S, pstParentNode: *mut crate::types::AVLBASE_NODE_S, pstNode: *mut crate::types::AVLBASE_NODE_S) -> () {
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
        let pstLeftSon: *mut AVLBASE_NODE_S = (*(*ppstSubTree)).pstLeft;
        (*(*ppstSubTree)).pstLeft = (*pstLeftSon).pstRight;
        if !(*(*ppstSubTree)).pstLeft.is_null() {
            (*(*(*ppstSubTree)).pstLeft).pstParent = *ppstSubTree;
        }
        (*(*ppstSubTree)).sLHeight = (*pstLeftSon).sRHeight;
        (*pstLeftSon).pstParent = (*(*ppstSubTree)).pstParent;
        (*pstLeftSon).pstRight = *ppstSubTree;
        (*(*pstLeftSon).pstRight).pstParent = pstLeftSon;
        let r_height = (*(*ppstSubTree)).sRHeight;
        let l_height = (*(*ppstSubTree)).sLHeight;
        let max_height = if r_height > l_height { r_height } else { l_height };
        (*pstLeftSon).sRHeight = 1 + max_height;
        *ppstSubTree = pstLeftSon;
    }
}

pub extern "C" fn VosAvlRotateLeft(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pstRightSon: *mut AVLBASE_NODE_S = (*(*ppstSubTree)).pstRight;
        (*(*ppstSubTree)).pstRight = (*pstRightSon).pstLeft;
        if !(*(*ppstSubTree)).pstRight.is_null() {
            (*(*(*ppstSubTree)).pstRight).pstParent = *ppstSubTree;
        }
        (*(*ppstSubTree)).sRHeight = (*pstRightSon).sLHeight;
        (*pstRightSon).pstParent = (*(*ppstSubTree)).pstParent;
        (*pstRightSon).pstLeft = *ppstSubTree;
        (*(*pstRightSon).pstLeft).pstParent = pstRightSon;
        let max_height = if (*(*ppstSubTree)).sRHeight > (*(*ppstSubTree)).sLHeight {
            (*(*ppstSubTree)).sRHeight
        } else {
            (*(*ppstSubTree)).sLHeight
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
        (*(*pstSwapNode).pstRight).pstParent = pstSwapNode;
        (*(*pstSwapNode).pstLeft).pstParent = pstSwapNode;
        if (*pstBaseNode).pstParent.is_null() {
            (*pstTree).pstRoot = pstSwapNode;
        } else if (*(*pstBaseNode).pstParent).pstRight == pstBaseNode as *mut AVLBASE_NODE_S {
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
            (*(*pstNode).pstLeft).pstParent = pstNode;
            (*pstNode).sLHeight = 1;
        }
        if !pstNewRightSon.is_null() {
            (*(*pstNode).pstRight).pstParent = pstNode;
            (*pstNode).sRHeight = 1;
        }
    }
}

pub extern "C" fn VosAvlSwapRightMost(pstTree: *mut AVLBASE_TREE_S, pstSubTree: *mut AVLBASE_NODE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstSwapNode: *mut AVLBASE_NODE_S = pstSubTree;
    
    // FIND_RIGHTMOST_NODE macro expansion
    unsafe {
        while !(*pstSwapNode).pstRight.is_null() {
            pstSwapNode = (*pstSwapNode).pstRight;
        }
    }
    
    unsafe {
        if ((*pstSwapNode).sRHeight != 0) || ((*pstSwapNode).sLHeight > 1) {
            return;
        }
        
        let pstSwapParent: *mut AVLBASE_NODE_S = (*pstSwapNode).pstParent;
        let pstSwapLeft: *mut AVLBASE_NODE_S = (*pstSwapNode).pstLeft;
        
        crate::src_avl::VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode as *const AVLBASE_NODE_S);
        crate::src_avl::VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, std::ptr::null_mut());
        (*(*pstNode).pstParent).pstRight = pstNode;
    }
}

pub extern "C" fn VosAvlSwapLeftMost(pstTree: *mut AVLBASE_TREE_S, pstSubTree: *mut AVLBASE_NODE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstSwapNode: *mut AVLBASE_NODE_S = pstSubTree;
    
    // FIND_LEFTMOST_NODE macro expansion
    unsafe {
        while !(*pstSwapNode).pstLeft.is_null() {
            pstSwapNode = (*pstSwapNode).pstLeft;
        }
    }
    
    unsafe {
        if ((*pstSwapNode).sLHeight != 0) || ((*pstSwapNode).sRHeight > 1) {
            return;
        }
        
        let pstSwapParent: *mut AVLBASE_NODE_S = (*pstSwapNode).pstParent;
        let pstSwapRight: *mut AVLBASE_NODE_S = (*pstSwapNode).pstRight;
        
        crate::src_avl::VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode as *const AVLBASE_NODE_S);
        crate::src_avl::VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, std::ptr::null_mut(), pstSwapRight);
        (*(*pstNode).pstParent).pstLeft = pstNode;
    }
}

pub extern "C" fn VosAvlRebalance(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    unsafe {
        let iMoment: i32 = (*(*ppstSubTree)).sRHeight as i32 - (*(*ppstSubTree)).sLHeight as i32;
        if iMoment > 1 {
            if (*(*(*ppstSubTree)).pstRight).sLHeight > (*(*(*ppstSubTree)).pstRight).sRHeight {
                crate::src_avl::VosAvlRotateRight(&mut (*(*ppstSubTree)).pstRight);
            }
            crate::src_avl::VosAvlRotateLeft(ppstSubTree);
        } else if iMoment < -1 {
            if (*(*(*ppstSubTree)).pstLeft).sRHeight > (*(*(*ppstSubTree)).pstLeft).sLHeight {
                crate::src_avl::VosAvlRotateLeft(&mut (*(*ppstSubTree)).pstLeft);
            }
            crate::src_avl::VosAvlRotateRight(ppstSubTree);
        }
    }
}

pub extern "C" fn VosAvlBalanceTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) {
    let mut pstNodeTmp: *mut AVLBASE_NODE_S = pstNode;
    
    unsafe {
        while !(*pstNodeTmp).pstParent.is_null() {
            if (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                crate::src_avl::VosAvlRebalance(&mut (*pstNodeTmp).pstRight);
                let right_node = (*pstNodeTmp).pstRight;
                let r_height = (*right_node).sRHeight;
                let l_height = (*right_node).sLHeight;
                let max_height = if r_height > l_height { r_height } else { l_height };
                (*pstNodeTmp).sRHeight = 1 + max_height;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                crate::src_avl::VosAvlRebalance(&mut (*pstNodeTmp).pstLeft);
                let left_node = (*pstNodeTmp).pstLeft;
                let r_height = (*left_node).sRHeight;
                let l_height = (*left_node).sLHeight;
                let max_height = if r_height > l_height { r_height } else { l_height };
                (*pstNodeTmp).sLHeight = 1 + max_height;
            }
        }
        
        if (*pstNodeTmp).sLHeight != (*pstNodeTmp).sRHeight {
            crate::src_avl::VosAvlRebalance(&mut (*pstTree).pstRoot);
        }
    }
}

pub extern "C" fn VosAVLSearchReplaceNodeInRTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    unsafe {
        let pstReplaceNode: *mut AVLBASE_NODE_S;
        
        let pstRight = (*pstNode).pstRight;
        let pstRightLeft = (*pstRight).pstLeft;
        
        if pstRightLeft == std::ptr::null_mut() {
            pstReplaceNode = pstRight;
            (*pstReplaceNode).pstLeft = (*pstNode).pstLeft;
            (*(*pstReplaceNode).pstLeft).pstParent = pstReplaceNode;
            (*pstReplaceNode).sLHeight = (*pstNode).sLHeight;
        } else {
            crate::src_avl::VosAvlSwapLeftMost(pstTree, (*pstNode).pstRight, pstNode);
            pstReplaceNode = (*pstNode).pstRight;
        }
        
        pstReplaceNode
    }
}

pub extern "C" fn VosAvlSearchReplaceNodeInLTree(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let pstReplaceNode: *mut AVLBASE_NODE_S;
    
    unsafe {
        if (*(*pstNode).pstLeft).pstRight == std::ptr::null_mut() {
            pstReplaceNode = (*pstNode).pstLeft;
            (*pstReplaceNode).pstRight = (*pstNode).pstRight;
            (*(*pstReplaceNode).pstRight).pstParent = pstReplaceNode;
            (*pstReplaceNode).sRHeight = (*pstNode).sRHeight;
        } else {
            crate::src_avl::VosAvlSwapRightMost(pstTree, (*pstNode).pstLeft, pstNode);
            pstReplaceNode = (*pstNode).pstLeft;
        }
    }
    
    pstReplaceNode
}

pub extern "C" fn VosAvlSearchReplaceNode(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let pstReplaceNode: *mut AVLBASE_NODE_S;
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
    let pstReplaceNode: *mut AVLBASE_NODE_S;
    
    unsafe {
        let pstLeft = (*pstNode).pstLeft;
        let pstRight = (*pstNode).pstRight;
        
        if pstLeft.is_null() && pstRight.is_null() {
            pstReplaceNode = std::ptr::null_mut();
            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = (*pstNode).pstParent;
            }
            if (*pstTree).pstLast == pstNode {
                (*pstTree).pstLast = (*pstNode).pstParent;
            }
        } else if pstLeft.is_null() {
            pstReplaceNode = pstRight;
            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = pstReplaceNode;
            }
        } else if pstRight.is_null() {
            pstReplaceNode = pstLeft;
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
    unsafe {
        let mut sNewHeight: i16 = 0;
        
        let pstReplaceNode = crate::src_avl::VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
        let pstParentNode = (*pstBaseNode).pstParent;
        
        (*pstBaseNode).pstParent = std::ptr::null_mut();
        (*pstBaseNode).pstRight = std::ptr::null_mut();
        (*pstBaseNode).pstLeft = std::ptr::null_mut();
        (*pstBaseNode).sRHeight = -1;
        (*pstBaseNode).sLHeight = -1;
        
        if !pstReplaceNode.is_null() {
            (*pstReplaceNode).pstParent = pstParentNode;
            let lh = (*pstReplaceNode).sLHeight;
            let rh = (*pstReplaceNode).sRHeight;
            let max_h = if lh > rh { lh } else { rh };
            sNewHeight = 1 + max_h;
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

        let mut pstParentNode: *mut AVL_NODE = (*pstTree).pstRoot;
        while !pstParentNode.is_null() {
            let pfn = (*pstTree).pfnCompare;
            let iResult: i32 = if let Some(f) = pfn {
                f((*pstNode).pKey, (*pstParentNode).pKey)
            } else {
                0
            };

            if iResult > 0 {
                if !(*pstParentNode).pstRight.is_null() {
                    pstParentNode = (*pstParentNode).pstRight;
                    continue;
                }
                crate::src_avl::VosAvlNodeRightInsert(
                    &mut (*pstTree).pstRoot as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S,
                    pstParentNode as *mut crate::types::AVLBASE_NODE_S,
                    pstNode as *mut crate::types::AVLBASE_NODE_S,
                );
                break;
            } else if iResult < 0 {
                if !(*pstParentNode).pstLeft.is_null() {
                    pstParentNode = (*pstParentNode).pstLeft;
                    continue;
                }
                crate::src_avl::VosAvlNodeLeftInsert(
                    &mut (*pstTree).pstRoot as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S,
                    pstParentNode as *mut crate::types::AVLBASE_NODE_S,
                    pstNode as *mut crate::types::AVLBASE_NODE_S,
                );
                break;
            } else {
                (*pstNode).sRHeight = -1;
                (*pstNode).sLHeight = -1;
                return (*pstParentNode).pSelf;
            }
        }

        if !pstParentNode.is_null() {
            crate::src_avl::VosAvlBalanceTree(
                &mut (*pstTree).pstRoot as *mut *mut AVL_NODE as *mut crate::types::AVLBASE_TREE_S,
                pstParentNode as *mut crate::types::AVLBASE_NODE_S,
            );
        }

        std::ptr::null_mut()
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_avl_25
// c_function: VOS_AVL_Delete
// rust_file: src_avl.rs
// rust_signature: pub extern "C" fn VosAvlDelete(pstBaseNode: *mut AVLBASE_NODE_S, pstBaseTree: *mut AVLBASE_TREE_S)
// c_first_line: void VOS_AVL_Delete(AVL_TREE *pstTree, AVL_NODE *pstNode)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/avl/workspace/repair_history/avl/translate_by_qwen3_coder/_manual_fix/src_avl_25/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `pstBaseNode` in this scope
//      --> src/src_avl.rs:656:37
//       |
//       |                                     ^^^^^^^^^^^ not found in this scope
//   error[E0425]: cannot find value `pstBaseTree` in this scope
//      --> src/src_avl.rs:657:37
//       |
//       |                                     ^^^^^^^^^^^ not found in this scope
// =================================
pub extern "C" fn VOS_AVL_Delete(pstTree: *mut AVL_TREE, pstNode: *mut AVL_NODE) {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_avl_25
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/avl/workspace/repair_history/avl/translate_by_qwen3_coder/_manual_fix/src_avl_25/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn VosAvlDelete(pstBaseNode: *mut AVLBASE_NODE_S, pstBaseTree: *mut AVLBASE_TREE_S) {
    let node: *mut AVLBASE_NODE_S = pstBaseNode;
    let tree: *mut AVLBASE_TREE_S = pstBaseTree;
    let mut sNewHeight: i16 = 0;
    
    let pstReplaceNode = unsafe { VosAvlDeleteCheck(tree, node) };
    let pstParentNode = unsafe { (*node).pstParent };
    
    unsafe {
        (*node).pstParent = std::ptr::null_mut();
        (*node).pstRight = std::ptr::null_mut();
        (*node).pstLeft = std::ptr::null_mut();
        (*node).sRHeight = -1;
        (*node).sLHeight = -1;
    }
    
    if !pstReplaceNode.is_null() {
        unsafe {
            (*pstReplaceNode).pstParent = pstParentNode;
            let left_h = (*pstReplaceNode).sLHeight;
            let right_h = (*pstReplaceNode).sRHeight;
            let max_h = if left_h > right_h { left_h } else { right_h };
            sNewHeight = 1 + max_h;
        }
    }
    
    if !pstParentNode.is_null() {
        unsafe {
            if (*pstParentNode).pstRight == node {
                (*pstParentNode).pstRight = pstReplaceNode;
                (*pstParentNode).sRHeight = sNewHeight;
            } else {
                (*pstParentNode).pstLeft = pstReplaceNode;
                (*pstParentNode).sLHeight = sNewHeight;
            }
            VosAvlBalanceTree(tree, pstParentNode);
        }
    } else {
        unsafe {
            (*tree).pstRoot = pstReplaceNode;
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_avl_25
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn VOS_AVL_Find(pstTree: *mut AVL_TREE, pKey: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    if pstTree.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let mut pstNode: *mut AVL_NODE = (*pstTree).pstRoot;
        
        while !pstNode.is_null() {
            let pfnCompare = (*pstTree).pfnCompare;
            if let Some(compare_fn) = pfnCompare {
                let iResult: i32 = compare_fn(pKey, (*pstNode).pKey);
                
                if iResult > 0 {
                    pstNode = (*pstNode).pstRight;
                } else if iResult < 0 {
                    pstNode = (*pstNode).pstLeft;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if !pstNode.is_null() {
            (*pstNode).pSelf
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL_Next(pstNode: *mut AVL_NODE) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL_NODE = pstNode;
    
    unsafe {
        if pstNodeTmp.is_null() || !((*pstNodeTmp).sLHeight != -1 && (*pstNodeTmp).sRHeight != -1) {
            return std::ptr::null_mut();
        }
        
        if !(*pstNodeTmp).pstRight.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstRight;
            while !(*pstNodeTmp).pstLeft.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstLeft;
            }
        } else {
            while !pstNodeTmp.is_null() {
                if (*pstNodeTmp).pstParent.is_null() || (*(*pstNodeTmp).pstParent).pstLeft == pstNodeTmp {
                    pstNodeTmp = (*pstNodeTmp).pstParent;
                    break;
                }
                pstNodeTmp = (*pstNodeTmp).pstParent;
            }
        }
        
        if !pstNodeTmp.is_null() {
            (*pstNodeTmp).pSelf
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL_Prev(pstNode: *mut AVL_NODE) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL_NODE = pstNode;
    
    if pstNodeTmp.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        // Check if node is valid: (sLHeight != -1) && (sRHeight != -1)
        if !((*pstNodeTmp).sLHeight != -1 && (*pstNodeTmp).sRHeight != -1) {
            return std::ptr::null_mut();
        }
        
        if !(*pstNodeTmp).pstLeft.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstLeft;
            // Find rightmost node in left subtree
            while !(*pstNodeTmp).pstRight.is_null() {
                pstNodeTmp = (*pstNodeTmp).pstRight;
            }
        } else {
            // Go up until we find a node that is a right child
            while !pstNodeTmp.is_null() {
                if (*pstNodeTmp).pstParent.is_null() || (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp {
                    pstNodeTmp = (*pstNodeTmp).pstParent;
                    break;
                }
                pstNodeTmp = (*pstNodeTmp).pstParent;
            }
        }
        
        if !pstNodeTmp.is_null() {
            (*pstNodeTmp).pSelf
        } else {
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn VOS_AVL_Find_Or_Find_Next(pstTree: *mut AVL_TREE, pKey: *const ::core::ffi::c_void, bValue: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void {
    let mut pstNode: *mut AVL_NODE;
    let mut pFoundNode: *mut ::core::ffi::c_void = std::ptr::null_mut();
    let mut iResult: ::core::ffi::c_int;
    
    if pstTree.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        pstNode = (*pstTree).pstRoot;
    }
    
    if pstNode.is_null() {
        return pFoundNode;
    }
    
    loop {
        unsafe {
            if let Some(pfnCompare) = (*pstTree).pfnCompare {
                iResult = pfnCompare(pKey, (*pstNode).pKey);
            } else {
                return std::ptr::null_mut();
            }
            
            if iResult > 0 {
                if (*pstNode).pstRight.is_null() {
                    pFoundNode = crate::src_avl::VOS_AVL_Next(pstNode);
                    break;
                }
                pstNode = (*pstNode).pstRight;
            } else if iResult < 0 {
                if (*pstNode).pstLeft.is_null() {
                    pFoundNode = (*pstNode).pSelf;
                    break;
                }
                pstNode = (*pstNode).pstLeft;
            } else {
                if bValue != 0 {
                    pFoundNode = crate::src_avl::VOS_AVL_Next(pstNode);
                } else {
                    pFoundNode = (*pstNode).pSelf;
                }
                break;
            }
        }
    }
    
    pFoundNode
}
