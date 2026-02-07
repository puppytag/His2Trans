#![allow(deref_nullptr)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type AVL_V2_COMPARE_FUNC = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_node {
    pub pstParent: *mut avl_node,
    pub pstLeft: *mut avl_node,
    pub pstRight: *mut avl_node,
    pub sLHeight: core::ffi::c_short,
    pub sRHeight: core::ffi::c_short,
    pub pSelf: *mut core::ffi::c_void,
    pub pKey: *mut core::ffi::c_void,
}
pub type AVL_NODE = avl_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_tree {
    pub pfnCompare: AVL_V2_COMPARE_FUNC,
    pub pstRoot: *mut AVL_NODE,
    pub pstFirst: *mut AVL_NODE,
    pub pstLast: *mut AVL_NODE,
}
pub type AVL_TREE = avl_tree;
pub type AVL3_COMPARE = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl3_tree_info {
    pub pfCompare: AVL3_COMPARE,
    pub usKeyOffset: core::ffi::c_ushort,
    pub usNodeOffset: core::ffi::c_ushort,
}
pub type AVL3_TREE_INFO = avl3_tree_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl3_node {
    pub pstParent: *mut avl3_node,
    pub pstLeft: *mut avl3_node,
    pub pstRight: *mut avl3_node,
    pub sLHeight: core::ffi::c_short,
    pub sRHeight: core::ffi::c_short,
}
pub type AVL3_NODE = avl3_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl3_tree {
    pub pstRoot: *mut AVL3_NODE,
    pub pstFirst: *mut AVL3_NODE,
    pub pstLast: *mut AVL3_NODE,
}
pub type AVL3_TREE = avl3_tree;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVLBaseNode {
    pub pstParent: *mut AVLBaseNode,
    pub pstLeft: *mut AVLBaseNode,
    pub pstRight: *mut AVLBaseNode,
    pub sLHeight: core::ffi::c_short,
    pub sRHeight: core::ffi::c_short,
}
pub type AVLBASE_NODE_S = AVLBaseNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVLBaseTree {
    pub pstRoot: *mut AVLBASE_NODE_S,
    pub pstFirst: *mut AVLBASE_NODE_S,
    pub pstLast: *mut AVLBASE_NODE_S,
}
pub type AVLBASE_TREE_S = AVLBaseTree;
pub unsafe extern "C" fn AVL3_Find_Or_Find_Next(
    mut pstTree: *mut AVL3_TREE,
    mut pKey: *const core::ffi::c_void,
    mut bFlag: core::ffi::c_uint,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE = 0 as *mut AVL3_NODE;
    let mut pFoundNode: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut iResult: core::ffi::c_int = 0;
    let mut iKeyOffset: core::ffi::c_int = 0;
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstRoot;
    if pstNode.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    iKeyOffset = (*pstTreeInfo).usKeyOffset as core::ffi::c_int
        - (*pstTreeInfo).usNodeOffset as core::ffi::c_int;
    loop {
        iResult = ((*pstTreeInfo).pfCompare)
            .expect(
                "non-null function pointer",
            )(
            pKey,
            (pstNode as *mut core::ffi::c_uchar).offset(iKeyOffset as isize)
                as *mut core::ffi::c_void,
        );
        if iResult > 0 as core::ffi::c_int {
            if ((*pstNode).pstRight).is_null() {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                break;
            } else {
                pstNode = (*pstNode).pstRight as *mut AVL3_NODE;
            }
        } else if iResult < 0 as core::ffi::c_int {
            if ((*pstNode).pstLeft).is_null() {
                pFoundNode = (pstNode as *mut core::ffi::c_uchar)
                    .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
                    as *mut core::ffi::c_void;
                break;
            } else {
                pstNode = (*pstNode).pstLeft as *mut AVL3_NODE;
            }
        } else {
            if bFlag != 0 as core::ffi::c_uint {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
            } else {
                pFoundNode = (pstNode as *mut core::ffi::c_uchar)
                    .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
                    as *mut core::ffi::c_void;
            }
            break;
        }
    }
    return pFoundNode;
}
pub unsafe extern "C" fn VOS_AVL3_Insert_Or_Find(
    mut pstTree: *mut AVL3_TREE,
    mut pstNode: *mut AVL3_NODE,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstParentNode: *mut AVL3_NODE = 0 as *mut AVL3_NODE;
    let mut iResult: core::ffi::c_int = 0;
    let mut iKeyOffset: core::ffi::c_int = 0;
    if pstTree.is_null() || pstTreeInfo.is_null() || pstNode.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    (*pstNode).sRHeight = 0 as core::ffi::c_short;
    (*pstNode).sLHeight = 0 as core::ffi::c_short;
    if ((*pstTree).pstRoot).is_null() {
        (*pstTree).pstRoot = pstNode;
        (*pstTree).pstFirst = pstNode;
        (*pstTree).pstLast = pstNode;
        return 0 as *mut core::ffi::c_void;
    }
    pstParentNode = (*pstTree).pstRoot;
    iKeyOffset = (*pstTreeInfo).usKeyOffset as core::ffi::c_int
        - (*pstTreeInfo).usNodeOffset as core::ffi::c_int;
    while !pstParentNode.is_null() {
        iResult = ((*pstTreeInfo).pfCompare)
            .expect(
                "non-null function pointer",
            )(
            (pstNode as *mut core::ffi::c_uchar).offset(iKeyOffset as isize)
                as *mut core::ffi::c_void,
            (pstParentNode as *mut core::ffi::c_uchar).offset(iKeyOffset as isize)
                as *mut core::ffi::c_void,
        );
        if iResult > 0 as core::ffi::c_int {
            if !((*pstParentNode).pstRight).is_null() {
                pstParentNode = (*pstParentNode).pstRight as *mut AVL3_NODE;
            } else {
                VosAvlNodeRightInsert(
                    pstTree as *mut AVLBASE_TREE_S,
                    pstParentNode as *mut AVLBASE_NODE_S,
                    pstNode as *mut AVLBASE_NODE_S,
                );
                break;
            }
        } else if iResult < 0 as core::ffi::c_int {
            if !((*pstParentNode).pstLeft).is_null() {
                pstParentNode = (*pstParentNode).pstLeft as *mut AVL3_NODE;
            } else {
                VosAvlNodeLeftInsert(
                    pstTree as *mut AVLBASE_TREE_S,
                    pstParentNode as *mut AVLBASE_NODE_S,
                    pstNode as *mut AVLBASE_NODE_S,
                );
                break;
            }
        } else {
            (*pstNode).sRHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
            (*pstNode).sLHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
            return (pstParentNode as *mut core::ffi::c_uchar)
                .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
                as *mut core::ffi::c_void;
        }
    }
    VosAvlBalanceTree(
        pstTree as *mut AVLBASE_TREE_S,
        pstParentNode as *mut AVLBASE_NODE_S,
    );
    return 0 as *mut core::ffi::c_void;
}
pub unsafe extern "C" fn VOS_AVL3_Delete(
    mut pstTree: *mut AVL3_TREE,
    mut pstNode: *mut AVL3_NODE,
) {
    let mut pstBaseNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut pstBaseTree: *mut AVLBASE_TREE_S = 0 as *mut AVLBASE_TREE_S;
    if pstTree.is_null() || pstNode.is_null() {
        return;
    }
    pstBaseNode = pstNode as *mut AVLBASE_NODE_S;
    pstBaseTree = pstTree as *mut AVLBASE_TREE_S;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}
pub unsafe extern "C" fn VOS_AVL3_Find(
    mut pstTree: *mut AVL3_TREE,
    mut pstKey: *const core::ffi::c_void,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE = 0 as *mut AVL3_NODE;
    let mut iResult: core::ffi::c_int = 0;
    let mut iKeyOffset: core::ffi::c_int = 0;
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstRoot;
    iKeyOffset = (*pstTreeInfo).usKeyOffset as core::ffi::c_int
        - (*pstTreeInfo).usNodeOffset as core::ffi::c_int;
    while !pstNode.is_null() {
        iResult = ((*pstTreeInfo).pfCompare)
            .expect(
                "non-null function pointer",
            )(
            pstKey,
            (pstNode as *mut core::ffi::c_uchar).offset(iKeyOffset as isize)
                as *mut core::ffi::c_void,
        );
        if iResult > 0 as core::ffi::c_int {
            pstNode = (*pstNode).pstRight as *mut AVL3_NODE;
        } else {
            if !(iResult < 0 as core::ffi::c_int) {
                break;
            }
            pstNode = (*pstNode).pstLeft as *mut AVL3_NODE;
        }
    }
    return if !pstNode.is_null() {
        (pstNode as *mut core::ffi::c_uchar)
            .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
            as *mut core::ffi::c_void
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL3_First(
    mut pstTree: *mut AVL3_TREE,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE = 0 as *mut AVL3_NODE;
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstFirst;
    return if !pstNode.is_null() {
        (pstNode as *mut core::ffi::c_uchar)
            .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
            as *mut core::ffi::c_void
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL3_Last(
    mut pstTree: *mut AVL3_TREE,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL3_NODE = 0 as *mut AVL3_NODE;
    if pstTree.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstLast;
    return if !pstNode.is_null() {
        (pstNode as *mut core::ffi::c_uchar)
            .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
            as *mut core::ffi::c_void
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL3_Next(
    mut pstNode: *mut AVL3_NODE,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode;
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    if !((*pstNodeTmp).pstRight).is_null() {
        pstNodeTmp = (*pstNodeTmp).pstRight as *mut AVL3_NODE;
        while !((*pstNodeTmp).pstLeft).is_null() {
            pstNodeTmp = (*pstNodeTmp).pstLeft as *mut AVL3_NODE;
        }
    } else {
        while !pstNodeTmp.is_null() {
            if ((*pstNodeTmp).pstParent).is_null()
                || (*(*pstNodeTmp).pstParent).pstLeft == pstNodeTmp
            {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL3_NODE;
                break;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL3_NODE;
            }
        }
    }
    return if !pstNodeTmp.is_null() {
        (pstNodeTmp as *mut core::ffi::c_uchar)
            .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
            as *mut core::ffi::c_void
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL3_Prev(
    mut pstNode: *mut AVL3_NODE,
    mut pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL3_NODE = pstNode;
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    if !((*pstNodeTmp).pstLeft).is_null() {
        pstNodeTmp = (*pstNodeTmp).pstLeft as *mut AVL3_NODE;
        while !((*pstNodeTmp).pstRight).is_null() {
            pstNodeTmp = (*pstNodeTmp).pstRight as *mut AVL3_NODE;
        }
    } else {
        while !pstNodeTmp.is_null() {
            if ((*pstNodeTmp).pstParent).is_null()
                || (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp
            {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL3_NODE;
                break;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL3_NODE;
            }
        }
    }
    return if !pstNodeTmp.is_null() {
        (pstNodeTmp as *mut core::ffi::c_uchar)
            .offset(-((*pstTreeInfo).usNodeOffset as core::ffi::c_int as isize))
            as *mut core::ffi::c_void
    } else {
        0 as *mut core::ffi::c_void
    };
}
#[inline]
pub unsafe extern "C" fn VosAvlNodeRightInsert(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstParentNode: *mut AVLBASE_NODE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) {
    (*pstNode).pstParent = pstParentNode as *mut AVLBaseNode;
    (*pstParentNode).pstRight = pstNode as *mut AVLBaseNode;
    (*pstParentNode).sRHeight = 1 as core::ffi::c_short;
    if pstParentNode == (*pstTree).pstLast {
        (*pstTree).pstLast = pstNode;
    }
}
#[inline]
pub unsafe extern "C" fn VosAvlNodeLeftInsert(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstParentNode: *mut AVLBASE_NODE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) {
    (*pstNode).pstParent = pstParentNode as *mut AVLBaseNode;
    (*pstParentNode).pstLeft = pstNode as *mut AVLBaseNode;
    (*pstParentNode).sLHeight = 1 as core::ffi::c_short;
    if pstParentNode == (*pstTree).pstFirst {
        (*pstTree).pstFirst = pstNode;
    }
}
pub unsafe extern "C" fn VosAvlRotateRight(mut ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    let mut pstLeftSon: *mut AVLBASE_NODE_S = (**ppstSubTree).pstLeft
        as *mut AVLBASE_NODE_S;
    (**ppstSubTree).pstLeft = (*pstLeftSon).pstRight;
    if !((**ppstSubTree).pstLeft).is_null() {
        (*(**ppstSubTree).pstLeft).pstParent = *ppstSubTree as *mut AVLBaseNode;
    }
    (**ppstSubTree).sLHeight = (*pstLeftSon).sRHeight;
    (*pstLeftSon).pstParent = (**ppstSubTree).pstParent;
    (*pstLeftSon).pstRight = *ppstSubTree as *mut AVLBaseNode;
    (*(*pstLeftSon).pstRight).pstParent = pstLeftSon as *mut AVLBaseNode;
    (*pstLeftSon).sRHeight = (1 as core::ffi::c_int
        + (if (**ppstSubTree).sRHeight as core::ffi::c_int
            > (**ppstSubTree).sLHeight as core::ffi::c_int
        {
            (**ppstSubTree).sRHeight as core::ffi::c_int
        } else {
            (**ppstSubTree).sLHeight as core::ffi::c_int
        })) as core::ffi::c_short;
    *ppstSubTree = pstLeftSon;
}
pub unsafe extern "C" fn VosAvlRotateLeft(mut ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    let mut pstRightSon: *mut AVLBASE_NODE_S = (**ppstSubTree).pstRight
        as *mut AVLBASE_NODE_S;
    (**ppstSubTree).pstRight = (*pstRightSon).pstLeft;
    if !((**ppstSubTree).pstRight).is_null() {
        (*(**ppstSubTree).pstRight).pstParent = *ppstSubTree as *mut AVLBaseNode;
    }
    (**ppstSubTree).sRHeight = (*pstRightSon).sLHeight;
    (*pstRightSon).pstParent = (**ppstSubTree).pstParent;
    (*pstRightSon).pstLeft = *ppstSubTree as *mut AVLBaseNode;
    (*(*pstRightSon).pstLeft).pstParent = pstRightSon as *mut AVLBaseNode;
    (*pstRightSon).sLHeight = (1 as core::ffi::c_int
        + (if (**ppstSubTree).sRHeight as core::ffi::c_int
            > (**ppstSubTree).sLHeight as core::ffi::c_int
        {
            (**ppstSubTree).sRHeight as core::ffi::c_int
        } else {
            (**ppstSubTree).sLHeight as core::ffi::c_int
        })) as core::ffi::c_short;
    *ppstSubTree = pstRightSon;
}
pub unsafe extern "C" fn VosAvlUpdateSwapNode(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstSwapNode: *mut AVLBASE_NODE_S,
    mut pstBaseNode: *const AVLBASE_NODE_S,
) {
    (*pstSwapNode).pstParent = (*pstBaseNode).pstParent;
    (*pstSwapNode).pstRight = (*pstBaseNode).pstRight;
    (*pstSwapNode).pstLeft = (*pstBaseNode).pstLeft;
    (*pstSwapNode).sRHeight = (*pstBaseNode).sRHeight;
    (*pstSwapNode).sLHeight = (*pstBaseNode).sLHeight;
    (*(*pstSwapNode).pstRight).pstParent = pstSwapNode as *mut AVLBaseNode;
    (*(*pstSwapNode).pstLeft).pstParent = pstSwapNode as *mut AVLBaseNode;
    if ((*pstBaseNode).pstParent).is_null() {
        (*pstTree).pstRoot = pstSwapNode;
    } else if (*(*pstBaseNode).pstParent).pstRight == pstBaseNode as *mut AVLBaseNode {
        (*(*pstSwapNode).pstParent).pstRight = pstSwapNode as *mut AVLBaseNode;
    } else {
        (*(*pstSwapNode).pstParent).pstLeft = pstSwapNode as *mut AVLBaseNode;
    };
}
pub unsafe extern "C" fn VosAvlMoveNodeToNewPos(
    mut pstNode: *mut AVLBASE_NODE_S,
    mut pstNewParent: *mut AVLBASE_NODE_S,
    mut pstNewLeftSon: *mut AVLBASE_NODE_S,
    mut pstNewRightSon: *mut AVLBASE_NODE_S,
) {
    (*pstNode).pstParent = pstNewParent as *mut AVLBaseNode;
    (*pstNode).pstLeft = pstNewLeftSon as *mut AVLBaseNode;
    (*pstNode).pstRight = pstNewRightSon as *mut AVLBaseNode;
    (*pstNode).sLHeight = 0 as core::ffi::c_short;
    (*pstNode).sRHeight = 0 as core::ffi::c_short;
    if !pstNewLeftSon.is_null() {
        (*(*pstNode).pstLeft).pstParent = pstNode as *mut AVLBaseNode;
        (*pstNode).sLHeight = 1 as core::ffi::c_short;
    }
    if !pstNewRightSon.is_null() {
        (*(*pstNode).pstRight).pstParent = pstNode as *mut AVLBaseNode;
        (*pstNode).sRHeight = 1 as core::ffi::c_short;
    }
}
pub unsafe extern "C" fn VosAvlSwapRightMost(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstSubTree: *mut AVLBASE_NODE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstSwapNode: *mut AVLBASE_NODE_S = pstSubTree;
    let mut pstSwapParent: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut pstSwapLeft: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    while !((*pstSwapNode).pstRight).is_null() {
        pstSwapNode = (*pstSwapNode).pstRight as *mut AVLBASE_NODE_S;
    }
    if (*pstSwapNode).sRHeight as core::ffi::c_int != 0 as core::ffi::c_int
        || (*pstSwapNode).sLHeight as core::ffi::c_int > 1 as core::ffi::c_int
    {
        return;
    }
    pstSwapParent = (*pstSwapNode).pstParent as *mut AVLBASE_NODE_S;
    pstSwapLeft = (*pstSwapNode).pstLeft as *mut AVLBASE_NODE_S;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(
        pstNode,
        pstSwapParent,
        pstSwapLeft,
        0 as *mut AVLBASE_NODE_S,
    );
    (*(*pstNode).pstParent).pstRight = pstNode as *mut AVLBaseNode;
}
pub unsafe extern "C" fn VosAvlSwapLeftMost(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstSubTree: *mut AVLBASE_NODE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstSwapNode: *mut AVLBASE_NODE_S = pstSubTree;
    let mut pstSwapParent: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut pstSwapRight: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    while !((*pstSwapNode).pstLeft).is_null() {
        pstSwapNode = (*pstSwapNode).pstLeft as *mut AVLBASE_NODE_S;
    }
    if (*pstSwapNode).sLHeight as core::ffi::c_int != 0 as core::ffi::c_int
        || (*pstSwapNode).sRHeight as core::ffi::c_int > 1 as core::ffi::c_int
    {
        return;
    }
    pstSwapParent = (*pstSwapNode).pstParent as *mut AVLBASE_NODE_S;
    pstSwapRight = (*pstSwapNode).pstRight as *mut AVLBASE_NODE_S;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(
        pstNode,
        pstSwapParent,
        0 as *mut AVLBASE_NODE_S,
        pstSwapRight,
    );
    (*(*pstNode).pstParent).pstLeft = pstNode as *mut AVLBaseNode;
}
pub unsafe extern "C" fn VosAvlRebalance(mut ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    let mut iMoment: core::ffi::c_int = 0;
    iMoment = (**ppstSubTree).sRHeight as core::ffi::c_int
        - (**ppstSubTree).sLHeight as core::ffi::c_int;
    if iMoment > 1 as core::ffi::c_int {
        if (*(**ppstSubTree).pstRight).sLHeight as core::ffi::c_int
            > (*(**ppstSubTree).pstRight).sRHeight as core::ffi::c_int
        {
            VosAvlRotateRight(&mut (**ppstSubTree).pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    } else if iMoment < -(1 as core::ffi::c_int) {
        if (*(**ppstSubTree).pstLeft).sRHeight as core::ffi::c_int
            > (*(**ppstSubTree).pstLeft).sLHeight as core::ffi::c_int
        {
            VosAvlRotateLeft(&mut (**ppstSubTree).pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
}
pub unsafe extern "C" fn VosAvlBalanceTree(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) {
    let mut pstNodeTmp: *mut AVLBASE_NODE_S = pstNode;
    while !((*pstNodeTmp).pstParent).is_null() {
        if (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp {
            pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVLBASE_NODE_S;
            VosAvlRebalance(&mut (*pstNodeTmp).pstRight);
            (*pstNodeTmp).sRHeight = (1 as core::ffi::c_int
                + (if (*(*pstNodeTmp).pstRight).sRHeight as core::ffi::c_int
                    > (*(*pstNodeTmp).pstRight).sLHeight as core::ffi::c_int
                {
                    (*(*pstNodeTmp).pstRight).sRHeight as core::ffi::c_int
                } else {
                    (*(*pstNodeTmp).pstRight).sLHeight as core::ffi::c_int
                })) as core::ffi::c_short;
        } else {
            pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVLBASE_NODE_S;
            VosAvlRebalance(&mut (*pstNodeTmp).pstLeft);
            (*pstNodeTmp).sLHeight = (1 as core::ffi::c_int
                + (if (*(*pstNodeTmp).pstLeft).sRHeight as core::ffi::c_int
                    > (*(*pstNodeTmp).pstLeft).sLHeight as core::ffi::c_int
                {
                    (*(*pstNodeTmp).pstLeft).sRHeight as core::ffi::c_int
                } else {
                    (*(*pstNodeTmp).pstLeft).sLHeight as core::ffi::c_int
                })) as core::ffi::c_short;
        }
    }
    if (*pstNodeTmp).sLHeight as core::ffi::c_int
        != (*pstNodeTmp).sRHeight as core::ffi::c_int
    {
        VosAvlRebalance(&mut (*pstTree).pstRoot);
    }
}
pub unsafe extern "C" fn VosAVLSearchReplaceNodeInRTree(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    if ((*(*pstNode).pstRight).pstLeft).is_null() {
        pstReplaceNode = (*pstNode).pstRight as *mut AVLBASE_NODE_S;
        (*pstReplaceNode).pstLeft = (*pstNode).pstLeft;
        (*(*pstReplaceNode).pstLeft).pstParent = pstReplaceNode as *mut AVLBaseNode;
        (*pstReplaceNode).sLHeight = (*pstNode).sLHeight;
    } else {
        VosAvlSwapLeftMost(pstTree, (*pstNode).pstRight as *mut AVLBASE_NODE_S, pstNode);
        pstReplaceNode = (*pstNode).pstRight as *mut AVLBASE_NODE_S;
    }
    return pstReplaceNode;
}
pub unsafe extern "C" fn VosAvlSearchReplaceNodeInLTree(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    if ((*(*pstNode).pstLeft).pstRight).is_null() {
        pstReplaceNode = (*pstNode).pstLeft as *mut AVLBASE_NODE_S;
        (*pstReplaceNode).pstRight = (*pstNode).pstRight;
        (*(*pstReplaceNode).pstRight).pstParent = pstReplaceNode as *mut AVLBaseNode;
        (*pstReplaceNode).sRHeight = (*pstNode).sRHeight;
    } else {
        VosAvlSwapRightMost(pstTree, (*pstNode).pstLeft as *mut AVLBASE_NODE_S, pstNode);
        pstReplaceNode = (*pstNode).pstLeft as *mut AVLBASE_NODE_S;
    }
    return pstReplaceNode;
}
pub unsafe extern "C" fn VosAvlSearchReplaceNode(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    if (*pstNode).sRHeight as core::ffi::c_int > (*pstNode).sLHeight as core::ffi::c_int
    {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
    } else {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
    }
    return pstReplaceNode;
}
pub unsafe extern "C" fn VosAvlDeleteCheck(
    mut pstTree: *mut AVLBASE_TREE_S,
    mut pstNode: *mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    if ((*pstNode).pstLeft).is_null() && ((*pstNode).pstRight).is_null() {
        pstReplaceNode = 0 as *mut AVLBASE_NODE_S;
        if (*pstTree).pstFirst == pstNode {
            (*pstTree).pstFirst = (*pstNode).pstParent as *mut AVLBASE_NODE_S;
        }
        if (*pstTree).pstLast == pstNode {
            (*pstTree).pstLast = (*pstNode).pstParent as *mut AVLBASE_NODE_S;
        }
    } else if ((*pstNode).pstLeft).is_null() {
        pstReplaceNode = (*pstNode).pstRight as *mut AVLBASE_NODE_S;
        if (*pstTree).pstFirst == pstNode {
            (*pstTree).pstFirst = pstReplaceNode;
        }
    } else if ((*pstNode).pstRight).is_null() {
        pstReplaceNode = (*pstNode).pstLeft as *mut AVLBASE_NODE_S;
        if (*pstTree).pstLast == pstNode {
            (*pstTree).pstLast = pstReplaceNode;
        }
    } else {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree, pstNode);
    }
    return pstReplaceNode;
}
pub unsafe extern "C" fn VosAvlDelete(
    mut pstBaseNode: *mut AVLBASE_NODE_S,
    mut pstBaseTree: *mut AVLBASE_TREE_S,
) {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut pstParentNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut sNewHeight: core::ffi::c_short = 0 as core::ffi::c_short;
    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
    pstParentNode = (*pstBaseNode).pstParent as *mut AVLBASE_NODE_S;
    (*pstBaseNode).pstParent = 0 as *mut AVLBaseNode;
    (*pstBaseNode).pstRight = 0 as *mut AVLBaseNode;
    (*pstBaseNode).pstLeft = 0 as *mut AVLBaseNode;
    (*pstBaseNode).sRHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
    (*pstBaseNode).sLHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
    if !pstReplaceNode.is_null() {
        (*pstReplaceNode).pstParent = pstParentNode as *mut AVLBaseNode;
        sNewHeight = (1 as core::ffi::c_int
            + (if (*pstReplaceNode).sLHeight as core::ffi::c_int
                > (*pstReplaceNode).sRHeight as core::ffi::c_int
            {
                (*pstReplaceNode).sLHeight as core::ffi::c_int
            } else {
                (*pstReplaceNode).sRHeight as core::ffi::c_int
            })) as core::ffi::c_short;
    }
    if !pstParentNode.is_null() {
        if (*pstParentNode).pstRight == pstBaseNode {
            (*pstParentNode).pstRight = pstReplaceNode as *mut AVLBaseNode;
            (*pstParentNode).sRHeight = sNewHeight;
        } else {
            (*pstParentNode).pstLeft = pstReplaceNode as *mut AVLBaseNode;
            (*pstParentNode).sLHeight = sNewHeight;
        }
        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    } else {
        (*pstBaseTree).pstRoot = pstReplaceNode;
    };
}
pub unsafe extern "C" fn VOS_AVL_Insert_Or_Find(
    mut pstTree: *mut AVL_TREE,
    mut pstNode: *mut AVL_NODE,
) -> *mut core::ffi::c_void {
    let mut pstParentNode: *mut AVL_NODE = 0 as *mut AVL_NODE;
    let mut iResult: core::ffi::c_int = 0;
    if pstTree.is_null() || pstNode.is_null()
        || (*pstNode).sLHeight as core::ffi::c_int != -(1 as core::ffi::c_int)
            && (*pstNode).sRHeight as core::ffi::c_int != -(1 as core::ffi::c_int)
    {
        return 0 as *mut core::ffi::c_void;
    }
    (*pstNode).sRHeight = 0 as core::ffi::c_short;
    (*pstNode).sLHeight = 0 as core::ffi::c_short;
    if ((*pstTree).pstRoot).is_null() {
        (*pstTree).pstRoot = pstNode;
        (*pstTree).pstFirst = pstNode;
        (*pstTree).pstLast = pstNode;
        return 0 as *mut core::ffi::c_void;
    }
    pstParentNode = (*pstTree).pstRoot;
    while !pstParentNode.is_null() {
        iResult = ((*pstTree).pfnCompare)
            .expect("non-null function pointer")((*pstNode).pKey, (*pstParentNode).pKey);
        if iResult > 0 as core::ffi::c_int {
            if !((*pstParentNode).pstRight).is_null() {
                pstParentNode = (*pstParentNode).pstRight as *mut AVL_NODE;
            } else {
                VosAvlNodeRightInsert(
                    &mut (*pstTree).pstRoot as *mut *mut AVL_NODE
                        as *mut core::ffi::c_void as *mut AVLBASE_TREE_S,
                    pstParentNode as *mut AVLBASE_NODE_S,
                    pstNode as *mut AVLBASE_NODE_S,
                );
                break;
            }
        } else if iResult < 0 as core::ffi::c_int {
            if !((*pstParentNode).pstLeft).is_null() {
                pstParentNode = (*pstParentNode).pstLeft as *mut AVL_NODE;
            } else {
                VosAvlNodeLeftInsert(
                    &mut (*pstTree).pstRoot as *mut *mut AVL_NODE
                        as *mut core::ffi::c_void as *mut AVLBASE_TREE_S,
                    pstParentNode as *mut AVLBASE_NODE_S,
                    pstNode as *mut AVLBASE_NODE_S,
                );
                break;
            }
        } else {
            (*pstNode).sRHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
            (*pstNode).sLHeight = -(1 as core::ffi::c_int) as core::ffi::c_short;
            return (*pstParentNode).pSelf;
        }
    }
    if !pstParentNode.is_null() {
        VosAvlBalanceTree(
            &mut (*pstTree).pstRoot as *mut *mut AVL_NODE as *mut core::ffi::c_void
                as *mut AVLBASE_TREE_S,
            pstParentNode as *mut AVLBASE_NODE_S,
        );
    }
    return 0 as *mut core::ffi::c_void;
}
pub unsafe extern "C" fn VOS_AVL_Delete(
    mut pstTree: *mut AVL_TREE,
    mut pstNode: *mut AVL_NODE,
) {
    let mut pstBaseNode: *mut AVLBASE_NODE_S = 0 as *mut AVLBASE_NODE_S;
    let mut pstBaseTree: *mut AVLBASE_TREE_S = 0 as *mut AVLBASE_TREE_S;
    if pstTree.is_null() || pstNode.is_null()
        || !((*pstNode).sLHeight as core::ffi::c_int != -(1 as core::ffi::c_int)
            && (*pstNode).sRHeight as core::ffi::c_int != -(1 as core::ffi::c_int))
    {
        return;
    }
    pstBaseNode = pstNode as *mut AVLBASE_NODE_S;
    pstBaseTree = &mut (*pstTree).pstRoot as *mut *mut AVL_NODE as *mut core::ffi::c_void
        as *mut AVLBASE_TREE_S;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}
pub unsafe extern "C" fn VOS_AVL_Find(
    mut pstTree: *mut AVL_TREE,
    mut pKey: *const core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL_NODE = 0 as *mut AVL_NODE;
    let mut iResult: core::ffi::c_int = 0;
    if pstTree.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstRoot;
    while !pstNode.is_null() {
        iResult = ((*pstTree).pfnCompare)
            .expect("non-null function pointer")(pKey, (*pstNode).pKey);
        if iResult > 0 as core::ffi::c_int {
            pstNode = (*pstNode).pstRight as *mut AVL_NODE;
        } else {
            if !(iResult < 0 as core::ffi::c_int) {
                break;
            }
            pstNode = (*pstNode).pstLeft as *mut AVL_NODE;
        }
    }
    return if !pstNode.is_null() {
        (*pstNode).pSelf
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL_Next(
    mut pstNode: *mut AVL_NODE,
) -> *mut core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL_NODE = pstNode;
    if pstNodeTmp.is_null()
        || !((*pstNodeTmp).sLHeight as core::ffi::c_int != -(1 as core::ffi::c_int)
            && (*pstNodeTmp).sRHeight as core::ffi::c_int != -(1 as core::ffi::c_int))
    {
        return 0 as *mut core::ffi::c_void;
    }
    if !((*pstNodeTmp).pstRight).is_null() {
        pstNodeTmp = (*pstNodeTmp).pstRight as *mut AVL_NODE;
        while !((*pstNodeTmp).pstLeft).is_null() {
            pstNodeTmp = (*pstNodeTmp).pstLeft as *mut AVL_NODE;
        }
    } else {
        while !pstNodeTmp.is_null() {
            if ((*pstNodeTmp).pstParent).is_null()
                || (*(*pstNodeTmp).pstParent).pstLeft == pstNodeTmp
            {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL_NODE;
                break;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL_NODE;
            }
        }
    }
    return if !pstNodeTmp.is_null() {
        (*pstNodeTmp).pSelf
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL_Prev(
    mut pstNode: *mut AVL_NODE,
) -> *mut core::ffi::c_void {
    let mut pstNodeTmp: *mut AVL_NODE = pstNode;
    if pstNodeTmp.is_null()
        || !((*pstNodeTmp).sLHeight as core::ffi::c_int != -(1 as core::ffi::c_int)
            && (*pstNodeTmp).sRHeight as core::ffi::c_int != -(1 as core::ffi::c_int))
    {
        return 0 as *mut core::ffi::c_void;
    }
    if !((*pstNodeTmp).pstLeft).is_null() {
        pstNodeTmp = (*pstNodeTmp).pstLeft as *mut AVL_NODE;
        while !((*pstNodeTmp).pstRight).is_null() {
            pstNodeTmp = (*pstNodeTmp).pstRight as *mut AVL_NODE;
        }
    } else {
        while !pstNodeTmp.is_null() {
            if ((*pstNodeTmp).pstParent).is_null()
                || (*(*pstNodeTmp).pstParent).pstRight == pstNodeTmp
            {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL_NODE;
                break;
            } else {
                pstNodeTmp = (*pstNodeTmp).pstParent as *mut AVL_NODE;
            }
        }
    }
    return if !pstNodeTmp.is_null() {
        (*pstNodeTmp).pSelf
    } else {
        0 as *mut core::ffi::c_void
    };
}
pub unsafe extern "C" fn VOS_AVL_Find_Or_Find_Next(
    mut pstTree: *mut AVL_TREE,
    mut pKey: *const core::ffi::c_void,
    mut bValue: core::ffi::c_uint,
) -> *mut core::ffi::c_void {
    let mut pstNode: *mut AVL_NODE = 0 as *mut AVL_NODE;
    let mut pFoundNode: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut iResult: core::ffi::c_int = 0;
    if pstTree.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    pstNode = (*pstTree).pstRoot;
    if pstNode.is_null() {
        return pFoundNode;
    }
    loop {
        iResult = ((*pstTree).pfnCompare)
            .expect("non-null function pointer")(pKey, (*pstNode).pKey);
        if iResult > 0 as core::ffi::c_int {
            if ((*pstNode).pstRight).is_null() {
                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            } else {
                pstNode = (*pstNode).pstRight as *mut AVL_NODE;
            }
        } else if iResult < 0 as core::ffi::c_int {
            if ((*pstNode).pstLeft).is_null() {
                pFoundNode = (*pstNode).pSelf;
                break;
            } else {
                pstNode = (*pstNode).pstLeft as *mut AVL_NODE;
            }
        } else {
            if bValue != 0 as core::ffi::c_uint {
                pFoundNode = VOS_AVL_Next(pstNode);
            } else {
                pFoundNode = (*pstNode).pSelf;
            }
            break;
        }
    }
    return pFoundNode;
}
