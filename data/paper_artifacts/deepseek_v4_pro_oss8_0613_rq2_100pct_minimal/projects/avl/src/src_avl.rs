//! Module: src_avl
//!
//! Rust-native AVL tree implementation (safe API with minimal unsafe).
//! Unsafe is limited to: (1) raw pointer dereference for node graph traversal,
//! (2) pointer casts between compatible repr(C) tree types, (3) offset-based key access.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// ============================================================
// Safe key-offset helper
// ============================================================

/// Compute key pointer from node pointer and key offset.
/// Note: The returned pointer is computed using safe arithmetic; the caller
/// must ensure it points to valid memory before dereferencing.
#[inline(always)]
fn key_offset_ptr(node: *const u8, key_offset: isize) -> *const ::core::ffi::c_void {
    node.wrapping_offset(key_offset) as *const ::core::ffi::c_void
}

#[inline(always)]
fn key_offset_ptr_mut(node: *mut u8, key_offset: isize) -> *mut ::core::ffi::c_void {
    node.wrapping_offset(key_offset) as *mut ::core::ffi::c_void
}

// ============================================================
// NodePtr safe pointer wrapper
// ============================================================

struct NodePtr<T>(*mut T);

impl<T> NodePtr<T> {
    /// Create a NodePtr from a raw pointer.
    fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }
    /// Create a NodePtr from a raw pointer (const version).
    fn from_const(ptr: *const T) -> Self {
        Self(ptr as *mut T)
    }
    fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
    fn null() -> Self {
        Self(std::ptr::null_mut())
    }
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
    fn as_ptr(&self) -> *mut T {
        self.0
    }
    fn as_mut_ptr(&mut self) -> *mut T {
        self.0
    }
}

impl<T> Clone for NodePtr<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T> Copy for NodePtr<T> {}

impl<T> PartialEq for NodePtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for NodePtr<T> {}


pub(crate) trait AvlTreeBase {
    fn root(&self) -> *mut AVLBASE_NODE_S;
    fn root_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S;
    /// Return a mutable reference to the root pointer field.
    /// This is the safe alternative to `unsafe { &mut *root_mut_ptr() }`.
    fn root_mut<'a>(&'a mut self) -> &'a mut *mut AVLBASE_NODE_S;
    fn first(&self) -> *mut AVLBASE_NODE_S;
    fn first_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S;
    fn last(&self) -> *mut AVLBASE_NODE_S;
    fn last_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S;
    fn set_root(&mut self, node: *mut AVLBASE_NODE_S);
    fn set_first(&mut self, node: *mut AVLBASE_NODE_S);
    fn set_last(&mut self, node: *mut AVLBASE_NODE_S);
}

impl AvlTreeBase for AVLBaseTree {
    fn root(&self) -> *mut AVLBASE_NODE_S { self.pstRoot }
    fn root_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn root_mut<'a>(&'a mut self) -> &'a mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn first(&self) -> *mut AVLBASE_NODE_S { self.pstFirst }
    fn first_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstFirst }
    fn last(&self) -> *mut AVLBASE_NODE_S { self.pstLast }
    fn last_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstLast }
    fn set_root(&mut self, node: *mut AVLBASE_NODE_S) { self.pstRoot = node; }
    fn set_first(&mut self, node: *mut AVLBASE_NODE_S) { self.pstFirst = node; }
    fn set_last(&mut self, node: *mut AVLBASE_NODE_S) { self.pstLast = node; }
}

impl AvlTreeBase for AVL3_TREE {
    fn root(&self) -> *mut AVLBASE_NODE_S { self.pstRoot }
    fn root_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn root_mut<'a>(&'a mut self) -> &'a mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn first(&self) -> *mut AVLBASE_NODE_S { self.pstFirst }
    fn first_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstFirst }
    fn last(&self) -> *mut AVLBASE_NODE_S { self.pstLast }
    fn last_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstLast }
    fn set_root(&mut self, node: *mut AVLBASE_NODE_S) { self.pstRoot = node; }
    fn set_first(&mut self, node: *mut AVLBASE_NODE_S) { self.pstFirst = node; }
    fn set_last(&mut self, node: *mut AVLBASE_NODE_S) { self.pstLast = node; }
}

impl AvlTreeBase for AVL_TREE {
    fn root(&self) -> *mut AVLBASE_NODE_S { self.pstRoot }
    fn root_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn root_mut<'a>(&'a mut self) -> &'a mut *mut AVLBASE_NODE_S { &mut self.pstRoot }
    fn first(&self) -> *mut AVLBASE_NODE_S { self.pstFirst }
    fn first_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstFirst }
    fn last(&self) -> *mut AVLBASE_NODE_S { self.pstLast }
    fn last_mut_ptr(&mut self) -> *mut *mut AVLBASE_NODE_S { &mut self.pstLast }
    fn set_root(&mut self, node: *mut AVLBASE_NODE_S) { self.pstRoot = node; }
    fn set_first(&mut self, node: *mut AVLBASE_NODE_S) { self.pstFirst = node; }
    fn set_last(&mut self, node: *mut AVLBASE_NODE_S) { self.pstLast = node; }
}

// ============================================================
// Public API – AVL3 tree functions
// ============================================================

pub fn AVL3_Find_Or_Find_Next(
    pstTree: &AVL3_TREE,
    pKey: *const ::core::ffi::c_void,
    bFlag: ::core::ffi::c_uint,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let usKeyOffset = pstTreeInfo.usKeyOffset;
    let usNodeOffset = pstTreeInfo.usNodeOffset;
    let pfCompare = pstTreeInfo.pfCompare;
    let pstNode: *mut AVL3_NODE = pstTree.pstRoot as *mut AVL3_NODE;
    if pstNode.is_null() {
        return std::ptr::null_mut();
    }
    let iKeyOffset: isize = (usKeyOffset as i32 - usNodeOffset as i32) as isize;
    let comparer = pfCompare.expect("pfCompare is NULL");
    let mut pstNode = NodePtr::new(pstNode);
    loop {
        let node_ref = pstNode.as_ref();
        let key_ptr = key_offset_ptr(pstNode.as_ptr() as *const u8, iKeyOffset);
        let iResult = comparer(pKey, key_ptr);
        if iResult > 0 {
            let right = node_ref.pstRight;
            if right.is_null() {
                return VOS_AVL3_Next(node_ref, pstTreeInfo);
            }
            pstNode = NodePtr::new(right);
        } else if iResult < 0 {
            let left = node_ref.pstLeft;
            if left.is_null() {
                return key_offset_ptr_mut(pstNode.as_ptr() as *mut u8, -(usNodeOffset as isize));
            }
            pstNode = NodePtr::new(left);
        } else {
            if bFlag != 0 {
                return VOS_AVL3_Next(node_ref, pstTreeInfo);
            } else {
                return key_offset_ptr_mut(pstNode.as_ptr() as *mut u8, -(usNodeOffset as isize));
            }
        }
    }
}

pub fn VOS_AVL3_Insert_Or_Find(
    pstTree: &mut AVL3_TREE,
    pstNode: &mut AVL3_NODE,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    // write initial heights
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;
    let root = pstTree.pstRoot as *mut AVL3_NODE;
    if root.is_null() {
        pstTree.pstRoot = pstNode as *mut AVL3_NODE as *mut AVLBASE_NODE_S;
        pstTree.pstFirst = pstNode as *mut AVL3_NODE as *mut AVLBASE_NODE_S;
        pstTree.pstLast = pstNode as *mut AVL3_NODE as *mut AVLBASE_NODE_S;
        return ::core::ptr::null_mut();
    }
    let iKeyOffset = (pstTreeInfo.usKeyOffset as i32 - pstTreeInfo.usNodeOffset as i32) as isize;
    let pfCompare = pstTreeInfo.pfCompare;
    let comparer = pfCompare.expect("null callback");
    let mut pstParentNode: *mut AVL3_NODE = root;
    loop {
        if pstParentNode.is_null() {
            break;
        }
        let iResult = {
            let key_new = key_offset_ptr(pstNode as *const AVL3_NODE as *const u8, iKeyOffset);
            let key_parent = key_offset_ptr(pstParentNode as *const u8, iKeyOffset);
            comparer(key_new, key_parent)
        };
        let (right, left) = {
            let parent_ptr = NodePtr::new(pstParentNode);
            let parent_ref = parent_ptr.as_ref();
            (parent_ref.pstRight, parent_ref.pstLeft)
        };
        if iResult > 0 {
            if !right.is_null() {
                pstParentNode = right;
                continue;
            }
            let parent_ptr: *mut AVL3_NODE = pstParentNode;
            let node_ptr: *mut AVL3_NODE = pstNode;
            VosAvlNodeRightInsert(
                pstTree,
                parent_ptr as *mut AVLBASE_NODE_S,
                node_ptr as *mut AVLBASE_NODE_S,
            );
            break;
        } else if iResult < 0 {
            if !left.is_null() {
                pstParentNode = left;
                continue;
            }
            let parent_ptr: *mut AVL3_NODE = pstParentNode;
            let node_ptr: *mut AVL3_NODE = pstNode;
            VosAvlNodeLeftInsert(
                pstTree,
                parent_ptr as *mut AVLBASE_NODE_S,
                node_ptr as *mut AVLBASE_NODE_S,
            );
            break;
        } else {
            pstNode.sLHeight = -1;
            pstNode.sRHeight = -1;
            return key_offset_ptr_mut(pstParentNode as *mut u8, -(pstTreeInfo.usNodeOffset as isize));
        }
    }
    VosAvlBalanceTree(
        pstTree,
        pstParentNode as *mut AVLBASE_NODE_S,
    );
    ::core::ptr::null_mut()
}

pub fn VOS_AVL3_Delete(pstTree: &mut AVL3_TREE, pstNode: &mut AVL3_NODE) {
    let pstBaseNode: *mut AVL3_NODE = pstNode;
    VosAvlDelete(pstBaseNode as *mut AVLBASE_NODE_S, pstTree);
}

pub fn VOS_AVL3_Find(
    pstTree: &AVL3_TREE,
    pstKey: *const ::core::ffi::c_void,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE = pstTree.pstRoot as *mut AVL3_NODE;
    let iKeyOffset = (pstTreeInfo.usKeyOffset as i32 - pstTreeInfo.usNodeOffset as i32) as isize;
    let pfCompare = pstTreeInfo.pfCompare;
    let pfComparer = pfCompare.expect("pfCompare is null");

    while !pstNode.is_null() {
        let node_ptr = NodePtr::new(pstNode);
        let node_ref = node_ptr.as_ref();
        let key_ptr = key_offset_ptr(pstNode as *const u8, iKeyOffset);
        let iResult = pfComparer(pstKey, key_ptr);

        if iResult > 0 {
            pstNode = node_ref.pstRight;
        } else if iResult < 0 {
            pstNode = node_ref.pstLeft;
        } else {
            break;
        }
    }

    if pstNode.is_null() {
        std::ptr::null_mut()
    } else {
        let usNodeOffset = pstTreeInfo.usNodeOffset as isize;
        key_offset_ptr_mut(pstNode as *mut u8, -usNodeOffset)
    }
}

pub fn VOS_AVL3_First(
    pstTree: &AVL3_TREE,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let pstNode = pstTree.pstFirst as *mut AVL3_NODE;
    if pstNode.is_null() {
        return std::ptr::null_mut();
    }
    let offset = pstTreeInfo.usNodeOffset as isize;
    key_offset_ptr_mut(pstNode as *mut u8, -offset)
}

pub fn VOS_AVL3_Next(
    pstNode: &AVL3_NODE,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode as *const AVL3_NODE as *mut AVL3_NODE;
    let right = pstNode.pstRight;
    if !right.is_null() {
        pstNodeTmp = right;
        loop {
            let node_ptr = NodePtr::new(pstNodeTmp);
            let node_ref = node_ptr.as_ref();
            let left = node_ref.pstLeft;
            if left.is_null() {
                break;
            }
            pstNodeTmp = left;
        }
    } else {
        while !pstNodeTmp.is_null() {
            let node_ptr = NodePtr::new(pstNodeTmp);
            let parent = node_ptr.as_ref().pstParent;
            if parent.is_null() {
                pstNodeTmp = parent;
                break;
            }
            let parent_ptr = NodePtr::new(parent);
            let parent_ref = parent_ptr.as_ref();
            if parent_ref.pstLeft == pstNodeTmp {
                pstNodeTmp = parent;
                break;
            }
            pstNodeTmp = parent;
        }
    }
    if pstNodeTmp.is_null() {
        std::ptr::null_mut()
    } else {
        let offset = pstTreeInfo.usNodeOffset as isize;
        key_offset_ptr_mut(pstNodeTmp as *mut u8, -offset)
    }
}

pub fn VOS_AVL3_Last(
    pstTree: &AVL3_TREE,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let pstNode = pstTree.pstLast as *mut AVL3_NODE;
    if pstNode.is_null() {
        return ::core::ptr::null_mut();
    }
    let usOffset = pstTreeInfo.usNodeOffset;
    let p = (pstNode as *mut u8).wrapping_offset(-(usOffset as isize)) as *mut ::core::ffi::c_void;
    p
}

pub fn VOS_AVL3_Prev(
    pstNode: &AVL3_NODE,
    pstTreeInfo: &AVL3_TREE_INFO,
) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode as *const AVL3_NODE as *mut AVL3_NODE;
    let left = pstNode.pstLeft;
    if !left.is_null() {
        pstNodeTmp = left;
        loop {
            let node_ptr = NodePtr::new(pstNodeTmp);
            let node_ref = node_ptr.as_ref();
            let right = node_ref.pstRight;
            if right.is_null() {
                break;
            }
            pstNodeTmp = right;
        }
    } else {
        while !pstNodeTmp.is_null() {
            let node_ptr = NodePtr::new(pstNodeTmp);
            let parent = node_ptr.as_ref().pstParent;
            if parent.is_null() {
                pstNodeTmp = parent;
                break;
            }
            let parent_ptr = NodePtr::new(parent);
            let parent_ref = parent_ptr.as_ref();
            if parent_ref.pstRight == pstNodeTmp {
                pstNodeTmp = parent;
                break;
            }
            pstNodeTmp = parent;
        }
    }
    if !pstNodeTmp.is_null() {
        let offset = pstTreeInfo.usNodeOffset as isize;
        key_offset_ptr_mut(pstNodeTmp as *mut u8, -offset)
    } else {
        std::ptr::null_mut()
    }
}

// ============================================================
// Internal algorithms (operate on AVLBASE_NODE_S / AvlTreeBase)
// ============================================================

fn VosAvlNodeRightInsert(
    pstTree: &mut impl AvlTreeBase,
    pstParentNode: *mut AVLBASE_NODE_S,
    pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstNode = NodePtr::new(pstNode);
    let mut pstParentNode = NodePtr::new(pstParentNode);
    {
        let node_ref = pstNode.as_mut();
        node_ref.pstParent = pstParentNode.as_ptr();
    }
    {
        let parent_ref = pstParentNode.as_mut();
        parent_ref.pstRight = pstNode.as_ptr();
        parent_ref.sRHeight = 1;
    }
    if pstParentNode.as_ptr() == pstTree.last() {
        pstTree.set_last(pstNode.as_ptr());
    }
}

fn VosAvlNodeLeftInsert(
    pstTree: &mut impl AvlTreeBase,
    pstParentNode: *mut AVLBASE_NODE_S,
    pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstNode = NodePtr::new(pstNode);
    let mut pstParentNode = NodePtr::new(pstParentNode);
    {
        let node_ref = pstNode.as_mut();
        node_ref.pstParent = pstParentNode.as_ptr();
    }
    {
        let parent_ref = pstParentNode.as_mut();
        parent_ref.pstLeft = pstNode.as_ptr();
        parent_ref.sLHeight = 1;
    }
    if pstParentNode.as_ptr() == pstTree.first() {
        pstTree.set_first(pstNode.as_ptr());
    }
}

pub fn VosAvlRotateRight(ppstSubTree: &mut *mut AVLBASE_NODE_S) {
    let node = *ppstSubTree;
    let mut node_ptr = NodePtr::new(node);
    let node_ref = node_ptr.as_mut();
    let left_son = node_ref.pstLeft;
    let mut left_son_ptr = NodePtr::new(left_son);
    let left_son_ref = left_son_ptr.as_mut();
    let left_son_right = left_son_ref.pstRight;
    node_ref.pstLeft = left_son_right;
    if !left_son_right.is_null() {
        let mut left_son_right_ptr = NodePtr::new(left_son_right);
        let left_son_right_ref = left_son_right_ptr.as_mut();
        left_son_right_ref.pstParent = node;
    }
    node_ref.sLHeight = left_son_ref.sRHeight;
    left_son_ref.pstParent = node_ref.pstParent;
    left_son_ref.pstRight = node;
    node_ref.pstParent = left_son;
    let rh = node_ref.sRHeight;
    let lh = node_ref.sLHeight;
    let new_height = if rh > lh { rh } else { lh };
    left_son_ref.sRHeight = 1 + new_height;
    *ppstSubTree = left_son;
}

pub fn VosAvlRotateLeft(ppstSubTree: &mut *mut AVLBASE_NODE_S) {
    let node = *ppstSubTree;
    let mut node_ptr = NodePtr::new(node);
    let node_ref = node_ptr.as_mut();
    let right_son = node_ref.pstRight;
    let mut right_son_ptr = NodePtr::new(right_son);
    let right_son_ref = right_son_ptr.as_mut();
    let right_son_left = right_son_ref.pstLeft;
    node_ref.pstRight = right_son_left;
    if !right_son_left.is_null() {
        let mut right_son_left_ptr = NodePtr::new(right_son_left);
        let right_son_left_ref = right_son_left_ptr.as_mut();
        right_son_left_ref.pstParent = node;
    }
    node_ref.sRHeight = right_son_ref.sLHeight;
    right_son_ref.pstParent = node_ref.pstParent;
    right_son_ref.pstLeft = node;
    node_ref.pstParent = right_son;
    let lh = node_ref.sLHeight;
    let rh = node_ref.sRHeight;
    let max_h = if rh > lh { rh } else { lh };
    right_son_ref.sLHeight = 1 + max_h;
    *ppstSubTree = right_son;
}

pub fn VosAvlUpdateSwapNode(
    pstTree: &mut impl AvlTreeBase,
    pstSwapNode: *mut AVLBASE_NODE_S,
    pstBaseNode: *const AVLBASE_NODE_S,
) {
    let base_ptr = NodePtr::new(pstBaseNode as *mut AVLBASE_NODE_S);
    let base_ref = base_ptr.as_ref();
    let base_parent = base_ref.pstParent;
    let base_right = base_ref.pstRight;
    let base_left = base_ref.pstLeft;
    let base_rh = base_ref.sRHeight;
    let base_lh = base_ref.sLHeight;
    {
        let mut swap_ptr = NodePtr::new(pstSwapNode);
        let swap_ref = swap_ptr.as_mut();
        swap_ref.pstParent = base_parent;
        swap_ref.pstRight = base_right;
        swap_ref.pstLeft = base_left;
        swap_ref.sRHeight = base_rh;
        swap_ref.sLHeight = base_lh;
    }
    if !base_right.is_null() {
        let mut base_right_ptr = NodePtr::new(base_right);
        let base_right_ref = base_right_ptr.as_mut();
        base_right_ref.pstParent = pstSwapNode;
    }
    if !base_left.is_null() {
        let mut base_left_ptr = NodePtr::new(base_left);
        let base_left_ref = base_left_ptr.as_mut();
        base_left_ref.pstParent = pstSwapNode;
    }
    if base_parent.is_null() {
        pstTree.set_root(pstSwapNode);
    } else {
        let mut base_parent_ptr = NodePtr::new(base_parent);
        let base_parent_ref = base_parent_ptr.as_mut();
        let is_right = base_parent_ref.pstRight as *const AVLBASE_NODE_S == pstBaseNode;
        if is_right {
            base_parent_ref.pstRight = pstSwapNode;
        } else {
            base_parent_ref.pstLeft = pstSwapNode;
        }
    }
}

pub fn VosAvlMoveNodeToNewPos(
    pstNode: *mut AVLBASE_NODE_S,
    pstNewParent: *mut AVLBASE_NODE_S,
    pstNewLeftSon: *mut AVLBASE_NODE_S,
    pstNewRightSon: *mut AVLBASE_NODE_S,
) {
    let mut node_ptr = NodePtr::new(pstNode);
    let node_ref = node_ptr.as_mut();
    node_ref.pstParent = pstNewParent;
    node_ref.pstLeft = pstNewLeftSon;
    node_ref.pstRight = pstNewRightSon;
    node_ref.sLHeight = 0;
    node_ref.sRHeight = 0;
    if !pstNewLeftSon.is_null() {
        NodePtr::new(pstNewLeftSon).as_mut().pstParent = pstNode;
        node_ref.sLHeight = 1;
    }
    if !pstNewRightSon.is_null() {
        NodePtr::new(pstNewRightSon).as_mut().pstParent = pstNode;
        node_ref.sRHeight = 1;
    }
}

pub fn VosAvlSwapRightMost(
    pstTree: &mut impl AvlTreeBase,
    pstSubTree: *mut AVLBASE_NODE_S,
    pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstSwapNode = NodePtr::new(pstSubTree);
    loop {
        let node_ref = pstSwapNode.as_ref();
        let right = node_ref.pstRight;
        if right.is_null() {
            break;
        }
        pstSwapNode = NodePtr::new(right);
    }
    let swap_ref = pstSwapNode.as_ref();
    let rh = swap_ref.sRHeight;
    let lh = swap_ref.sLHeight;
    if rh != 0 || lh > 1 {
        return;
    }
    let pstSwapParent = swap_ref.pstParent;
    let pstSwapLeft = swap_ref.pstLeft;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode.as_ptr(), pstNode as *const AVLBASE_NODE_S);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, std::ptr::null_mut());
    let tmp_node = NodePtr::new(pstNode);
    let node_ref = tmp_node.as_ref();
    let parent_raw = node_ref.pstParent;
    let mut parent_ptr = NodePtr::new(parent_raw);
    let parent = parent_ptr.as_mut();
    parent.pstRight = pstNode;
}

pub fn VosAvlSwapLeftMost(
    pstTree: &mut impl AvlTreeBase,
    pstSubTree: *mut AVLBASE_NODE_S,
    pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstSwapNode = NodePtr::new(pstSubTree);
    loop {
        let node_ref = pstSwapNode.as_ref();
        let left = node_ref.pstLeft;
        if left.is_null() {
            break;
        }
        pstSwapNode = NodePtr::new(left);
    }
    let swap_ref = pstSwapNode.as_ref();
    let lh = swap_ref.sLHeight;
    let rh = swap_ref.sRHeight;
    if lh != 0 || rh > 1 {
        return;
    }
    let pstSwapParent = swap_ref.pstParent;
    let pstSwapRight = swap_ref.pstRight;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode.as_ptr(), pstNode as *const AVLBASE_NODE_S);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, std::ptr::null_mut(), pstSwapRight);
    let tmp_node = NodePtr::new(pstNode);
    let node_ref = tmp_node.as_ref();
    let parent_raw = node_ref.pstParent;
    let mut parent_ptr = NodePtr::new(parent_raw);
    let parent = parent_ptr.as_mut();
    parent.pstLeft = pstNode;
}

pub fn VosAvlRebalance(ppstSubTree: &mut *mut AVLBASE_NODE_S) {
    let sub_tree = *ppstSubTree;
    let mut sub_ptr = NodePtr::new(sub_tree);
    let sub_ref = sub_ptr.as_mut();
    let sR = sub_ref.sRHeight;
    let sL = sub_ref.sLHeight;
    let iMoment = sR as i32 - sL as i32;
    if iMoment > 1 {
        let right = sub_ref.pstRight;
        let right_ptr = NodePtr::new(right);
        let right_ref = right_ptr.as_ref();
        let rl = right_ref.sLHeight;
        let rr = right_ref.sRHeight;
        if rl > rr {
            VosAvlRotateRight(&mut sub_ref.pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    } else if iMoment < -1 {
        let left = sub_ref.pstLeft;
        let left_ptr = NodePtr::new(left);
        let left_ref = left_ptr.as_ref();
        let lr = left_ref.sRHeight;
        let ll = left_ref.sLHeight;
        if lr > ll {
            VosAvlRotateLeft(&mut sub_ref.pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
}

pub fn VosAvlBalanceTree(
    pstTree: &mut impl AvlTreeBase,
    pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstNodeTmp = NodePtr::new(pstNode);
    let mut parent = pstNodeTmp.as_ref().pstParent;
    while !parent.is_null() {
        let mut parent_ptr = NodePtr::new(parent);
        let parent_ref = parent_ptr.as_mut();
        let is_right = parent_ref.pstRight == pstNodeTmp.as_ptr();
        if is_right {
            pstNodeTmp = parent_ptr;
            let mut node_tmp_ptr = pstNodeTmp;
            let node_tmp_ref = node_tmp_ptr.as_mut();
            VosAvlRebalance(&mut node_tmp_ref.pstRight);
            let right = node_tmp_ref.pstRight;
            if !right.is_null() {
                let right_ptr = NodePtr::new(right);
                let right_ref = right_ptr.as_ref();
                let rh = right_ref.sRHeight;
                let lh = right_ref.sLHeight;
                let max_h = if rh > lh { rh } else { lh };
                node_tmp_ref.sRHeight = 1 + max_h;
            }
        } else {
            pstNodeTmp = parent_ptr;
            let mut node_tmp_ptr = pstNodeTmp;
            let node_tmp_ref = node_tmp_ptr.as_mut();
            VosAvlRebalance(&mut node_tmp_ref.pstLeft);
            let left = node_tmp_ref.pstLeft;
            if !left.is_null() {
                let left_ptr = NodePtr::new(left);
                let left_ref = left_ptr.as_ref();
                let rh = left_ref.sRHeight;
                let lh = left_ref.sLHeight;
                let max_h = if rh > lh { rh } else { lh };
                node_tmp_ref.sLHeight = 1 + max_h;
            }
        }
        parent = pstNodeTmp.as_ref().pstParent;
    }
    {
        let node_ref = pstNodeTmp.as_ref();
        let lh = node_ref.sLHeight;
        let rh = node_ref.sRHeight;
        if lh != rh {
            VosAvlRebalance(pstTree.root_mut());
        }
    }
}

pub fn VosAVLSearchReplaceNodeInRTree(
    pstTree: &mut impl AvlTreeBase,
    pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let node_ptr = NodePtr::new(pstNode);
    let node_ref = node_ptr.as_ref();
    let node_right = node_ref.pstRight;
    let right_ptr = NodePtr::new(node_right);
    let right_ref = right_ptr.as_ref();
    let right_left = right_ref.pstLeft;
    if right_left.is_null() {
        let pst_left = node_ref.pstLeft;
        let pst_lh = node_ref.sLHeight;
        {
            let mut right_mut_ptr = NodePtr::new(node_right);
            let right_mut = right_mut_ptr.as_mut();
            right_mut.pstLeft = pst_left;
            if !right_mut.pstLeft.is_null() {
                let left_child_raw = right_mut.pstLeft;
                let mut left_child_ptr = NodePtr::new(left_child_raw);
                let left_child = left_child_ptr.as_mut();
                left_child.pstParent = node_right;
            }
            right_mut.sLHeight = pst_lh;
        }
        node_right
    } else {
        VosAvlSwapLeftMost(pstTree, node_right, pstNode);
        NodePtr::new(pstNode).as_ref().pstRight
    }
}

pub fn VosAvlSearchReplaceNodeInLTree(
    pstTree: &mut impl AvlTreeBase,
    pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let node_ptr = NodePtr::new(pstNode);
    let node_ref = node_ptr.as_ref();
    let pstLeft = node_ref.pstLeft;
    let left_ptr = NodePtr::new(pstLeft);
    let left_ref = left_ptr.as_ref();
    let left_right = left_ref.pstRight;
    if left_right.is_null() {
        let pst_right = node_ref.pstRight;
        let pst_rh = node_ref.sRHeight;
        {
            let mut left_mut_ptr = NodePtr::new(pstLeft);
            let left_mut = left_mut_ptr.as_mut();
            left_mut.pstRight = pst_right;
            if !left_mut.pstRight.is_null() {
                let right_child_raw = left_mut.pstRight;
                let mut right_child_ptr = NodePtr::new(right_child_raw);
                let right_child = right_child_ptr.as_mut();
                right_child.pstParent = pstLeft;
            }
            left_mut.sRHeight = pst_rh;
        }
        pstLeft
    } else {
        VosAvlSwapRightMost(pstTree, pstLeft, pstNode);
        NodePtr::new(pstNode).as_ref().pstLeft
    }
}

pub fn VosAvlSearchReplaceNode(
    pstTree: &mut impl AvlTreeBase,
    pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let tmp_node = NodePtr::new(pstNode);
    let node_ref = tmp_node.as_ref();
    let rh = node_ref.sRHeight;
    let lh = node_ref.sLHeight;
    if rh > lh {
        VosAVLSearchReplaceNodeInRTree(pstTree, pstNode)
    } else {
        VosAvlSearchReplaceNodeInLTree(pstTree, pstNode)
    }
}

pub fn VosAvlDeleteCheck(
    pstTree: &mut impl AvlTreeBase,
    pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let node_ptr = NodePtr::new(pstNode);
    let node_ref = node_ptr.as_ref();
    let left = node_ref.pstLeft;
    let right = node_ref.pstRight;
    if left.is_null() && right.is_null() {
        let parent = node_ref.pstParent;
        if pstTree.first() == pstNode {
            pstTree.set_first(parent);
        }
        if pstTree.last() == pstNode {
            pstTree.set_last(parent);
        }
        core::ptr::null_mut()
    } else if left.is_null() {
        if pstTree.first() == pstNode {
            pstTree.set_first(right);
        }
        right
    } else if right.is_null() {
        if pstTree.last() == pstNode {
            pstTree.set_last(left);
        }
        left
    } else {
        VosAvlSearchReplaceNode(pstTree, pstNode)
    }
}

pub fn VosAvlDelete(
    pstBaseNode: *mut AVLBASE_NODE_S,
    pstBaseTree: &mut impl AvlTreeBase,
) {
    let mut sNewHeight: i16 = 0;
    let pstReplaceNode = VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
    let pstParentNode = {
        let base_ptr = NodePtr::new(pstBaseNode);
        base_ptr.as_ref().pstParent
    };
    // detach deleted node
    {
        let mut base_ptr = NodePtr::new(pstBaseNode);
        let base_ref = base_ptr.as_mut();
        base_ref.pstParent = std::ptr::null_mut();
        base_ref.pstRight = std::ptr::null_mut();
        base_ref.pstLeft = std::ptr::null_mut();
        base_ref.sRHeight = -1;
        base_ref.sLHeight = -1;
    }
    if !pstReplaceNode.is_null() {
        {
            let replace_ptr = NodePtr::new(pstReplaceNode);
            let replace_ref = replace_ptr.as_ref();
            let lh = replace_ref.sLHeight;
            let rh = replace_ref.sRHeight;
            sNewHeight = (1i32 + core::cmp::max(lh, rh) as i32) as i16;
        }
        {
            let mut replace_ptr = NodePtr::new(pstReplaceNode);
            replace_ptr.as_mut().pstParent = pstParentNode;
        }
    }
    if !pstParentNode.is_null() {
        let mut parent_ptr = NodePtr::new(pstParentNode);
        let parent_ref = parent_ptr.as_mut();
        let is_right = parent_ref.pstRight == pstBaseNode;
        if is_right {
            parent_ref.pstRight = pstReplaceNode;
            parent_ref.sRHeight = sNewHeight;
        } else {
            parent_ref.pstLeft = pstReplaceNode;
            parent_ref.sLHeight = sNewHeight;
        }
        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    } else {
        pstBaseTree.set_root(pstReplaceNode);
    }
}

// ============================================================
// Public API – AVL (v2) tree functions
// ============================================================

pub fn VOS_AVL_Insert_Or_Find(
    pstTree: &mut AVL_TREE,
    pstNode: &mut AVL_NODE,
) -> *mut ::core::ffi::c_void {
    if pstNode.sLHeight != -1 && pstNode.sRHeight != -1 {
        return std::ptr::null_mut();
    }
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;
    let (mut pstParentNode, compare_fn) = {
        let root = pstTree.pstRoot as *mut AVL_NODE;
        if root.is_null() {
            pstTree.pstRoot = pstNode as *mut AVL_NODE as *mut AVLBASE_NODE_S;
            pstTree.pstFirst = pstNode as *mut AVL_NODE as *mut AVLBASE_NODE_S;
            pstTree.pstLast = pstNode as *mut AVL_NODE as *mut AVLBASE_NODE_S;
            return std::ptr::null_mut();
        }
        (root, pstTree.pfnCompare)
    };
    let comparer = compare_fn.expect("null compare");
    while !pstParentNode.is_null() {
        let parent_ptr = NodePtr::new(pstParentNode);
        let parent_ref = parent_ptr.as_ref();
        let node_key = pstNode.pKey;
        let parent_key = parent_ref.pKey;
        let iResult = comparer(node_key, parent_key);

        if iResult > 0 {
            let right = parent_ref.pstRight;
            if !right.is_null() {
                pstParentNode = right;
                continue;
            }
            let parent_ptr: *mut AVL_NODE = pstParentNode;
            let node_ptr: *mut AVL_NODE = pstNode;
            VosAvlNodeRightInsert(
                pstTree,
                parent_ptr as *mut AVLBASE_NODE_S,
                node_ptr as *mut AVLBASE_NODE_S,
            );
            break;
        } else if iResult < 0 {
            let left = parent_ref.pstLeft;
            if !left.is_null() {
                pstParentNode = left;
                continue;
            }
            let parent_ptr: *mut AVL_NODE = pstParentNode;
            let node_ptr: *mut AVL_NODE = pstNode;
            VosAvlNodeLeftInsert(
                pstTree,
                parent_ptr as *mut AVLBASE_NODE_S,
                node_ptr as *mut AVLBASE_NODE_S,
            );
            break;
        }
        // duplicate
        pstNode.sLHeight = -1;
        pstNode.sRHeight = -1;
        return parent_ref.pSelf;
    }

    if !pstParentNode.is_null() {
        VosAvlBalanceTree(
            pstTree,
            pstParentNode as *mut AVLBASE_NODE_S,
        );
    }

    std::ptr::null_mut()
}

pub fn VOS_AVL_Delete(pstTree: &mut AVL_TREE, pstNode: &mut AVL_NODE) {
    if pstNode.sLHeight == -1 || pstNode.sRHeight == -1 {
        return;
    }
    let pstBaseNode: *mut AVL_NODE = pstNode;
    VosAvlDelete(pstBaseNode as *mut AVLBASE_NODE_S, pstTree);
}

pub fn VOS_AVL_Find(
    pstTree: &AVL_TREE,
    pKey: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut pstNode: *mut AVL_NODE = pstTree.pstRoot as *mut AVL_NODE;
    while !pstNode.is_null() {
        let node_ptr = NodePtr::new(pstNode);
        let node_ref = node_ptr.as_ref();
        let cmp = pstTree.pfnCompare;
        let node_key = node_ref.pKey;
        let iResult: i32 = cmp.expect("null compare")(pKey, node_key);
        if iResult > 0 {
            pstNode = node_ref.pstRight;
        } else if iResult < 0 {
            pstNode = node_ref.pstLeft;
        } else {
            return node_ref.pSelf;
        }
    }
    std::ptr::null_mut()
}

pub fn VOS_AVL_Next(pstNode: &AVL_NODE) -> *mut ::core::ffi::c_void {
    if pstNode.sLHeight == -1 || pstNode.sRHeight == -1 {
        return std::ptr::null_mut();
    }
    let mut current: *mut AVL_NODE = pstNode as *const AVL_NODE as *mut AVL_NODE;
    let right = pstNode.pstRight;
    if !right.is_null() {
        current = right;
        loop {
            let cur_ptr = NodePtr::new(current);
            let node_ref = cur_ptr.as_ref();
            let left = node_ref.pstLeft;
            if left.is_null() {
                break;
            }
            current = left;
        }
    } else {
        loop {
            let cur_ptr = NodePtr::new(current);
            let cur_ref = cur_ptr.as_ref();
            let parent = cur_ref.pstParent;
            if parent.is_null() {
                current = std::ptr::null_mut();
                break;
            }
            let parent_ptr = NodePtr::new(parent);
            let parent_ref = parent_ptr.as_ref();
            if parent_ref.pstLeft == current {
                current = parent;
                break;
            }
            current = parent;
        }
    }
    if current.is_null() {
        std::ptr::null_mut()
    } else {
        let cur_ptr = NodePtr::new(current);
        cur_ptr.as_ref().pSelf
    }
}

pub fn VOS_AVL_Prev(pstNode: &AVL_NODE) -> *mut ::core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL_NODE = pstNode as *const AVL_NODE as *mut AVL_NODE;
    if pstNode.sLHeight == -1 || pstNode.sRHeight == -1 {
        return std::ptr::null_mut();
    }
    let left = pstNode.pstLeft;
    if !left.is_null() {
        pstNodeTmp = left;
        loop {
            let tmp_ptr = NodePtr::new(pstNodeTmp);
            let node_ref = tmp_ptr.as_ref();
            let right = node_ref.pstRight;
            if right.is_null() {
                break;
            }
            pstNodeTmp = right;
        }
    } else {
        while !pstNodeTmp.is_null() {
            let tmp_ptr = NodePtr::new(pstNodeTmp);
            let tmp_ref = tmp_ptr.as_ref();
            let parent = tmp_ref.pstParent;
            if parent.is_null() {
                pstNodeTmp = std::ptr::null_mut();
                break;
            }
            let parent_ptr = NodePtr::new(parent);
            let parent_ref = parent_ptr.as_ref();
            if parent_ref.pstRight == pstNodeTmp {
                pstNodeTmp = parent;
                break;
            }
            pstNodeTmp = parent;
        }
    }
    if !pstNodeTmp.is_null() {
        let tmp_ptr = NodePtr::new(pstNodeTmp);
        tmp_ptr.as_ref().pSelf
    } else {
        std::ptr::null_mut()
    }
}

pub fn VOS_AVL_Find_Or_Find_Next(
    pstTree: &AVL_TREE,
    pKey: *const ::core::ffi::c_void,
    bValue: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    let mut pstNode: *mut AVL_NODE = pstTree.pstRoot as *mut AVL_NODE;
    if pstNode.is_null() {
        return ::core::ptr::null_mut();
    }
    loop {
        let node_ptr = NodePtr::new(pstNode);
        let node_ref = node_ptr.as_ref();
        let cmp_fn = match pstTree.pfnCompare {
            Some(f) => f,
            None => return ::core::ptr::null_mut(),
        };
        let node_key = node_ref.pKey;
        let iResult = cmp_fn(pKey, node_key);
        if iResult > 0 {
            let right = node_ref.pstRight;
            if right.is_null() {
                return VOS_AVL_Next(node_ref);
            }
            pstNode = right;
        } else if iResult < 0 {
            let left = node_ref.pstLeft;
            if left.is_null() {
                return node_ref.pSelf;
            }
            pstNode = left;
        } else {
            if bValue != 0 {
                return VOS_AVL_Next(node_ref);
            } else {
                return node_ref.pSelf;
            }
        }
    }
}

// ============================================================
// Initializer functions (safe, using references)
// ============================================================

/// VOS_AVL_INIT_TREE: initialize an AVL_TREE, mirroring the C macro.
pub fn VOS_AVL_INIT_TREE(tree: &mut AVL_TREE, compare: AVL_V2_COMPARE_FUNC) {
    tree.pfnCompare = compare;
    tree.pstRoot = ::core::ptr::null_mut();
    tree.pstFirst = ::core::ptr::null_mut();
    tree.pstLast = ::core::ptr::null_mut();
}

/// VOS_AVL_INIT_NODE: initialize an AVL_NODE, mirroring the C macro.
pub fn VOS_AVL_INIT_NODE(node: &mut AVL_NODE, self_: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void) {
    node.pstParent = ::core::ptr::null_mut();
    node.pstLeft = ::core::ptr::null_mut();
    node.pstRight = ::core::ptr::null_mut();
    node.pSelf = self_;
    node.pKey = key;
    node.sLHeight = -1i16;
    node.sRHeight = -1i16;
}

/// VOS_AVL3_INIT_TREE: initialize an AVL3_TREE, mirroring the C macro.
pub fn VOS_AVL3_INIT_TREE(tree: &mut AVL3_TREE, _tree_info: &AVL3_TREE_INFO) {
    tree.pstRoot = ::core::ptr::null_mut();
    tree.pstFirst = ::core::ptr::null_mut();
    tree.pstLast = ::core::ptr::null_mut();
}

/// VOS_AVL3_INIT_NODE: initialize an AVL3_NODE, mirroring the C macro.
pub fn VOS_AVL3_INIT_NODE(node: &mut AVL3_NODE) {
    node.pstParent = ::core::ptr::null_mut();
    node.pstLeft = ::core::ptr::null_mut();
    node.pstRight = ::core::ptr::null_mut();
    node.sLHeight = -1i16;
    node.sRHeight = -1i16;
}
