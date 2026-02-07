# 1 "/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/avl/src/avl.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 361 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/avl/src/avl.c" 2
typedef int (*AVL_V2_COMPARE_FUNC)(const void *, const void *);

typedef struct avl_node
{
    struct avl_node *pstParent;
    struct avl_node *pstLeft;
    struct avl_node *pstRight;
    short int sLHeight;
    short int sRHeight;
    void *pSelf;
    void *pKey;
} AVL_NODE;

typedef struct avl_tree
{
    AVL_V2_COMPARE_FUNC pfnCompare;
    AVL_NODE *pstRoot;
    AVL_NODE *pstFirst;
    AVL_NODE *pstLast;
} AVL_TREE;

typedef int (*AVL3_COMPARE)(const void *, const void *);

typedef struct avl3_tree_info
{
    AVL3_COMPARE pfCompare;
    unsigned short int usKeyOffset;
    unsigned short int usNodeOffset;
} AVL3_TREE_INFO;


typedef struct avl3_node
{
    struct avl3_node *pstParent;
    struct avl3_node *pstLeft;
    struct avl3_node *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVL3_NODE;

typedef struct avl3_tree
{
    AVL3_NODE *pstRoot;
    AVL3_NODE *pstFirst;
    AVL3_NODE *pstLast;
} AVL3_TREE;

typedef struct AVLBaseNode
{
    struct AVLBaseNode *pstParent;
    struct AVLBaseNode *pstLeft;
    struct AVLBaseNode *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVLBASE_NODE_S;

typedef struct AVLBaseTree
{
    AVLBASE_NODE_S *pstRoot;
    AVLBASE_NODE_S *pstFirst;
    AVLBASE_NODE_S *pstLast;
} AVLBASE_TREE_S;
# 185 "/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/avl/src/avl.c"
static inline void VosAvlNodeRightInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode, AVLBASE_NODE_S *pstNode);
static inline void VosAvlNodeLeftInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode, AVLBASE_NODE_S *pstNode);
void VosAvlBalanceTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode);
void VosAvlDelete(AVLBASE_NODE_S *pstBaseNode, AVLBASE_TREE_S *pstBaseTree);
void *VOS_AVL3_Next(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo);
void *VOS_AVL3_Prev(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo);

void *AVL3_Find_Or_Find_Next(AVL3_TREE *pstTree, const void *pKey, unsigned int bFlag, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    void *pFoundNode = 0L;
    int iResult;
    int iKeyOffset;
    if ((((pstTree) == 0L) || ((pstTreeInfo) == 0L)))
    {
        return 0L;
    }
    pstNode = pstTree->pstRoot;
    if (pstNode == 0L)
    {
        return 0L;
    }
    iKeyOffset = ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset));
    for (;;)
    {
        iResult = pstTreeInfo->pfCompare(pKey, (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0)
        {
            if (pstNode->pstRight == 0L)
            {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                break;
            }
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            if (pstNode->pstLeft == 0L)
            {
                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
                break;
            }
            pstNode = pstNode->pstLeft;
        }
        else
        {
            if (bFlag != 0)
            {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
            }
            else
            {
                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
            }
            break;
        }
    }
    return pFoundNode;
}

void *VOS_AVL3_Insert_Or_Find(AVL3_TREE *pstTree, AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstParentNode;
    int iResult;
    int iKeyOffset;
    if ((((pstTree) == 0L) || ((pstTreeInfo) == 0L)) || (pstNode == 0L))
    {
        return 0L;
    }
    pstNode->sRHeight = 0;
    pstNode->sLHeight = 0;
    if (pstTree->pstRoot == 0L)
    {
        pstTree->pstRoot = pstNode;
        pstTree->pstFirst = pstNode;
        pstTree->pstLast = pstNode;
        return 0L;
    }
    pstParentNode = pstTree->pstRoot;
    iKeyOffset = ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset));
    while (pstParentNode != 0L)
    {
        iResult = pstTreeInfo->pfCompare((void *)((unsigned char *)pstNode + iKeyOffset),
                                         (void *)((unsigned char *)pstParentNode + iKeyOffset));
        if (iResult > 0)
        {
            if (pstParentNode->pstRight != 0L)
            {
                pstParentNode = pstParentNode->pstRight;
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode,
                                  (AVLBASE_NODE_S *)pstNode);
        }
        else if (iResult < 0)
        {
            if (pstParentNode->pstLeft != 0L)
            {
                pstParentNode = pstParentNode->pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode, (AVLBASE_NODE_S *)pstNode);
        }
        else
        {
            pstNode->sRHeight = -1;
            pstNode->sLHeight = -1;
            return (void *)((unsigned char *)pstParentNode - pstTreeInfo->usNodeOffset);
        }
        break;
    }
    VosAvlBalanceTree((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode);
    return 0L;
}

void VOS_AVL3_Delete(AVL3_TREE *pstTree, AVL3_NODE *pstNode)
{
    AVLBASE_NODE_S *pstBaseNode;
    AVLBASE_TREE_S *pstBaseTree;
    if ((pstTree == 0L) || (pstNode == 0L))
    {
        return;
    }
    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)pstTree;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}

void *VOS_AVL3_Find(AVL3_TREE *pstTree, const void *pstKey, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    int iResult;
    int iKeyOffset;
    if ((((pstTree) == 0L) || ((pstTreeInfo) == 0L)))
    {
        return 0L;
    }
    pstNode = pstTree->pstRoot;
    iKeyOffset = ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset));
    while (pstNode != 0L)
    {
        iResult = pstTreeInfo->pfCompare(pstKey, (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return (((pstNode) != 0L) ? (void *)((unsigned char *)(pstNode) - (pstTreeInfo->usNodeOffset)) : 0L);
}

void *VOS_AVL3_First(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    if ((((pstTree) == 0L) || ((pstTreeInfo) == 0L)))
    {
        return 0L;
    }
    pstNode = pstTree->pstFirst;
    return (((pstNode) != 0L) ? (void *)((unsigned char *)(pstNode) - (pstTreeInfo->usNodeOffset)) : 0L);
}

void *VOS_AVL3_Last(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    if ((((pstTree) == 0L) || ((pstTreeInfo) == 0L)))
    {
        return 0L;
    }
    pstNode = pstTree->pstLast;
    return (((pstNode) != 0L) ? (void *)((unsigned char *)(pstNode) - (pstTreeInfo->usNodeOffset)) : 0L);
}

void *VOS_AVL3_Next(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == 0L) || (pstTreeInfo == 0L))
    {
        return 0L;
    }
    if (pstNodeTmp->pstRight != 0L)
    {
        pstNodeTmp = pstNodeTmp->pstRight;
        do { while ((pstNodeTmp)->pstLeft != 0L) { (pstNodeTmp) = (pstNodeTmp)->pstLeft; } } while (0);
    }
    else
    {
        while (pstNodeTmp != 0L)
        {
            if ((pstNodeTmp->pstParent == 0L) || (pstNodeTmp->pstParent->pstLeft == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return (((pstNodeTmp) != 0L) ? (void *)((unsigned char *)(pstNodeTmp) - (pstTreeInfo->usNodeOffset)) : 0L);
}

void *VOS_AVL3_Prev(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == 0L) || (pstTreeInfo == 0L))
    {
        return 0L;
    }
    if (pstNodeTmp->pstLeft != 0L)
    {
        pstNodeTmp = pstNodeTmp->pstLeft;
        do { while ((pstNodeTmp)->pstRight != 0L) { (pstNodeTmp) = (pstNodeTmp)->pstRight; } } while (0);
    }
    else
    {
        while (pstNodeTmp != 0L)
        {
            if ((pstNodeTmp->pstParent == 0L) || (pstNodeTmp->pstParent->pstRight == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return (((pstNodeTmp) != 0L) ? (void *)((unsigned char *)(pstNodeTmp) - (pstTreeInfo->usNodeOffset)) : 0L);
}

static inline void VosAvlNodeRightInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode,
                                         AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstRight = pstNode;
    pstParentNode->sRHeight = 1;
    if (pstParentNode == pstTree->pstLast)
    {
        pstTree->pstLast = pstNode;
    }
}

static inline void VosAvlNodeLeftInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode, AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstLeft = pstNode;
    pstParentNode->sLHeight = 1;
    if (pstParentNode == pstTree->pstFirst)
    {
        pstTree->pstFirst = pstNode;
    }
}

void VosAvlRotateRight(AVLBASE_NODE_S **ppstSubTree)
{
    AVLBASE_NODE_S *pstLeftSon = (*ppstSubTree)->pstLeft;
    (*ppstSubTree)->pstLeft = pstLeftSon->pstRight;
    if ((*ppstSubTree)->pstLeft != 0L)
    {
        (*ppstSubTree)->pstLeft->pstParent = (*ppstSubTree);
    }
    (*ppstSubTree)->sLHeight = pstLeftSon->sRHeight;
    pstLeftSon->pstParent = (*ppstSubTree)->pstParent;
    pstLeftSon->pstRight = *ppstSubTree;
    pstLeftSon->pstRight->pstParent = pstLeftSon;
    pstLeftSon->sRHeight = (1 + ((((*ppstSubTree)->sRHeight) > ((*ppstSubTree)->sLHeight)) ? ((*ppstSubTree)->sRHeight) : ((*ppstSubTree)->sLHeight)));
    *ppstSubTree = pstLeftSon;
    return;
}

void VosAvlRotateLeft(AVLBASE_NODE_S **ppstSubTree)
{
    AVLBASE_NODE_S *pstRightSon = (*ppstSubTree)->pstRight;
    (*ppstSubTree)->pstRight = pstRightSon->pstLeft;
    if ((*ppstSubTree)->pstRight != 0L)
    {
        (*ppstSubTree)->pstRight->pstParent = (*ppstSubTree);
    }
    (*ppstSubTree)->sRHeight = pstRightSon->sLHeight;
    pstRightSon->pstParent = (*ppstSubTree)->pstParent;
    pstRightSon->pstLeft = *ppstSubTree;
    pstRightSon->pstLeft->pstParent = pstRightSon;
    pstRightSon->sLHeight = (1 + ((((*ppstSubTree)->sRHeight) > ((*ppstSubTree)->sLHeight)) ? ((*ppstSubTree)->sRHeight) : ((*ppstSubTree)->sLHeight)));
    *ppstSubTree = pstRightSon;
    return;
}

void VosAvlUpdateSwapNode(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSwapNode, const AVLBASE_NODE_S *pstBaseNode)
{
    pstSwapNode->pstParent = pstBaseNode->pstParent;
    pstSwapNode->pstRight = pstBaseNode->pstRight;
    pstSwapNode->pstLeft = pstBaseNode->pstLeft;
    pstSwapNode->sRHeight = pstBaseNode->sRHeight;
    pstSwapNode->sLHeight = pstBaseNode->sLHeight;
    pstSwapNode->pstRight->pstParent = pstSwapNode;
    pstSwapNode->pstLeft->pstParent = pstSwapNode;
    if (pstBaseNode->pstParent == 0L)
    {
        pstTree->pstRoot = pstSwapNode;
    }
    else if (pstBaseNode->pstParent->pstRight == pstBaseNode)
    {
        pstSwapNode->pstParent->pstRight = pstSwapNode;
    }
    else
    {
        pstSwapNode->pstParent->pstLeft = pstSwapNode;
    }
}

void VosAvlMoveNodeToNewPos(AVLBASE_NODE_S *pstNode, AVLBASE_NODE_S *pstNewParent, AVLBASE_NODE_S *pstNewLeftSon,
                            AVLBASE_NODE_S *pstNewRightSon)
{
    pstNode->pstParent = pstNewParent;
    pstNode->pstLeft = pstNewLeftSon;
    pstNode->pstRight = pstNewRightSon;
    pstNode->sLHeight = 0;
    pstNode->sRHeight = 0;
    if (pstNewLeftSon != 0L)
    {
        pstNode->pstLeft->pstParent = pstNode;
        pstNode->sLHeight = 1;
    }
    if (pstNewRightSon != 0L)
    {
        pstNode->pstRight->pstParent = pstNode;
        pstNode->sRHeight = 1;
    }
}

void VosAvlSwapRightMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapLeft;
    do { while ((pstSwapNode)->pstRight != 0L) { (pstSwapNode) = (pstSwapNode)->pstRight; } } while (0);
    if ((pstSwapNode->sRHeight != 0) || (pstSwapNode->sLHeight > 1))
    {
        return;
    }
    pstSwapParent = pstSwapNode->pstParent;
    pstSwapLeft = pstSwapNode->pstLeft;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, 0L);
    pstNode->pstParent->pstRight = pstNode;
    return;
}

void VosAvlSwapLeftMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapRight;
    do { while ((pstSwapNode)->pstLeft != 0L) { (pstSwapNode) = (pstSwapNode)->pstLeft; } } while (0);
    if ((pstSwapNode->sLHeight != 0) || (pstSwapNode->sRHeight > 1))
    {
        return;
    }
    pstSwapParent = pstSwapNode->pstParent;
    pstSwapRight = pstSwapNode->pstRight;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, 0L, pstSwapRight);
    pstNode->pstParent->pstLeft = pstNode;
    return;
}

void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;
    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;
    if (iMoment > 1)
    {
        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight)
        {
            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    }
    else if (iMoment < -1)
    {
        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight)
        {
            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
    return;
}

void VosAvlBalanceTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstNodeTmp = pstNode;
    while (pstNodeTmp->pstParent != 0L)
    {
        if (pstNodeTmp->pstParent->pstRight == pstNodeTmp)
        {
            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstRight);
            pstNodeTmp->sRHeight = (1 + (((pstNodeTmp->pstRight->sRHeight) > (pstNodeTmp->pstRight->sLHeight)) ? (pstNodeTmp->pstRight->sRHeight) : (pstNodeTmp->pstRight->sLHeight)));
        }
        else
        {
            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstLeft);
            pstNodeTmp->sLHeight = (1 + (((pstNodeTmp->pstLeft->sRHeight) > (pstNodeTmp->pstLeft->sLHeight)) ? (pstNodeTmp->pstLeft->sRHeight) : (pstNodeTmp->pstLeft->sLHeight)));
        }
    }
    if (pstNodeTmp->sLHeight != pstNodeTmp->sRHeight)
    {
        VosAvlRebalance(&pstTree->pstRoot);
    }
    return;
}

AVLBASE_NODE_S *VosAVLSearchReplaceNodeInRTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->pstRight->pstLeft == 0L)
    {
        pstReplaceNode = pstNode->pstRight;
        pstReplaceNode->pstLeft = pstNode->pstLeft;
        pstReplaceNode->pstLeft->pstParent = pstReplaceNode;
        pstReplaceNode->sLHeight = pstNode->sLHeight;
    }
    else
    {
        VosAvlSwapLeftMost(pstTree, pstNode->pstRight, pstNode);
        pstReplaceNode = pstNode->pstRight;
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNodeInLTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->pstLeft->pstRight == 0L)
    {
        pstReplaceNode = pstNode->pstLeft;
        pstReplaceNode->pstRight = pstNode->pstRight;
        pstReplaceNode->pstRight->pstParent = pstReplaceNode;
        pstReplaceNode->sRHeight = pstNode->sRHeight;
    }
    else
    {
        VosAvlSwapRightMost(pstTree, pstNode->pstLeft, pstNode);
        pstReplaceNode = pstNode->pstLeft;
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNode(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->sRHeight > pstNode->sLHeight)
    {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
    }
    else
    {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlDeleteCheck(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if ((pstNode->pstLeft == 0L) && (pstNode->pstRight == 0L))
    {
        pstReplaceNode = 0L;
        if (pstTree->pstFirst == pstNode)
        {
            pstTree->pstFirst = pstNode->pstParent;
        }
        if (pstTree->pstLast == pstNode)
        {
            pstTree->pstLast = pstNode->pstParent;
        }
    }
    else if (pstNode->pstLeft == 0L)
    {
        pstReplaceNode = pstNode->pstRight;
        if (pstTree->pstFirst == pstNode)
        {
            pstTree->pstFirst = pstReplaceNode;
        }
    }
    else if (pstNode->pstRight == 0L)
    {
        pstReplaceNode = pstNode->pstLeft;
        if (pstTree->pstLast == pstNode)
        {
            pstTree->pstLast = pstReplaceNode;
        }
    }
    else
    {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree, pstNode);
    }
    return pstReplaceNode;
}

void VosAvlDelete(AVLBASE_NODE_S *pstBaseNode, AVLBASE_TREE_S *pstBaseTree)
{
    AVLBASE_NODE_S *pstReplaceNode;
    AVLBASE_NODE_S *pstParentNode;
    short int sNewHeight = 0;
    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
    pstParentNode = pstBaseNode->pstParent;
    pstBaseNode->pstParent = 0L;
    pstBaseNode->pstRight = 0L;
    pstBaseNode->pstLeft = 0L;
    pstBaseNode->sRHeight = -1;
    pstBaseNode->sLHeight = -1;
    if (pstReplaceNode != 0L)
    {
        pstReplaceNode->pstParent = pstParentNode;
        sNewHeight = (1 + (((pstReplaceNode->sLHeight) > (pstReplaceNode->sRHeight)) ? (pstReplaceNode->sLHeight) : (pstReplaceNode->sRHeight)));
    }
    if (pstParentNode != 0L)
    {
        if (pstParentNode->pstRight == pstBaseNode)
        {
            pstParentNode->pstRight = pstReplaceNode;
            pstParentNode->sRHeight = sNewHeight;
        }
        else
        {
            pstParentNode->pstLeft = pstReplaceNode;
            pstParentNode->sLHeight = sNewHeight;
        }
        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    }
    else
    {
        pstBaseTree->pstRoot = pstReplaceNode;
    }
    return;
}

void *VOS_AVL_Insert_Or_Find(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVL_NODE *pstParentNode;
    int iResult;
    if ((pstTree == 0L) || (pstNode == 0L) || ((((*pstNode).sLHeight != -1) && ((*pstNode).sRHeight != -1))))
    {
        return 0L;
    }
    pstNode->sRHeight = 0;
    pstNode->sLHeight = 0;
    if (pstTree->pstRoot == 0L)
    {
        pstTree->pstRoot = pstNode;
        pstTree->pstFirst = pstNode;
        pstTree->pstLast = pstNode;
        return 0L;
    }
    for (pstParentNode = pstTree->pstRoot; pstParentNode != 0L;)
    {
        iResult = pstTree->pfnCompare(pstNode->pKey, pstParentNode->pKey);
        if (iResult > 0)
        {
            if (pstParentNode->pstRight != 0L)
            {
                pstParentNode = pstParentNode->pstRight;
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode,
                                  (AVLBASE_NODE_S *)pstNode);
            break;
        }
        else if (iResult < 0)
        {
            if (pstParentNode->pstLeft != 0L)
            {
                pstParentNode = pstParentNode->pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode,
                                 (AVLBASE_NODE_S *)pstNode);
            break;
        }
        pstNode->sRHeight = -1;
        pstNode->sLHeight = -1;
        return pstParentNode->pSelf;
    }
    if (pstParentNode != 0L)
    {
        VosAvlBalanceTree((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode);
    }
    return 0L;
}

void VOS_AVL_Delete(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVLBASE_NODE_S *pstBaseNode;
    AVLBASE_TREE_S *pstBaseTree;
    if ((pstTree == 0L) || (pstNode == 0L) || (!(((*pstNode).sLHeight != -1) && ((*pstNode).sRHeight != -1))))
    {
        return;
    }
    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot));
    VosAvlDelete(pstBaseNode, pstBaseTree);
    return;
}

void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{
    AVL_NODE *pstNode;
    int iResult;
    if (pstTree == 0L)
    {
        return 0L;
    }
    pstNode = pstTree->pstRoot;
    while (pstNode != 0L)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return ((pstNode != 0L) ? pstNode->pSelf : 0L);
}

void *VOS_AVL_Next(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == 0L) || (!(((*pstNodeTmp).sLHeight != -1) && ((*pstNodeTmp).sRHeight != -1))))
    {
        return 0L;
    }
    if (pstNodeTmp->pstRight != 0L)
    {
        pstNodeTmp = pstNodeTmp->pstRight;
        do { while ((pstNodeTmp)->pstLeft != 0L) { (pstNodeTmp) = (pstNodeTmp)->pstLeft; } } while (0);
    }
    else
    {
        while (pstNodeTmp != 0L)
        {
            if ((pstNodeTmp->pstParent == 0L) || (pstNodeTmp->pstParent->pstLeft == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return ((pstNodeTmp != 0L) ? pstNodeTmp->pSelf : 0L);
}

void *VOS_AVL_Prev(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == 0L) || (!(((*pstNodeTmp).sLHeight != -1) && ((*pstNodeTmp).sRHeight != -1))))
    {
        return 0L;
    }
    if (pstNodeTmp->pstLeft != 0L)
    {
        pstNodeTmp = pstNodeTmp->pstLeft;
        do { while ((pstNodeTmp)->pstRight != 0L) { (pstNodeTmp) = (pstNodeTmp)->pstRight; } } while (0);
    }
    else
    {
        while (pstNodeTmp != 0L)
        {
            if ((pstNodeTmp->pstParent == 0L) || (pstNodeTmp->pstParent->pstRight == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return ((pstNodeTmp != 0L) ? pstNodeTmp->pSelf : 0L);
}

void *VOS_AVL_Find_Or_Find_Next(AVL_TREE *pstTree, const void *pKey, unsigned int bValue)
{
    AVL_NODE *pstNode;
    void *pFoundNode = 0L;
    int iResult;
    if (pstTree == 0L)
    {
        return 0L;
    }
    pstNode = pstTree->pstRoot;
    if (pstNode == 0L)
    {
        return (pFoundNode);
    }
    for (;;)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            if (pstNode->pstRight == 0L)
            {
                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            }
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            if (pstNode->pstLeft == 0L)
            {
                pFoundNode = pstNode->pSelf;
                break;
            }
            pstNode = pstNode->pstLeft;
        }
        else
        {
            if (bValue != 0)
            {
                pFoundNode = VOS_AVL_Next(pstNode);
            }
            else
            {
                pFoundNode = pstNode->pSelf;
            }
            break;
        }
    }
    return (pFoundNode);
}
