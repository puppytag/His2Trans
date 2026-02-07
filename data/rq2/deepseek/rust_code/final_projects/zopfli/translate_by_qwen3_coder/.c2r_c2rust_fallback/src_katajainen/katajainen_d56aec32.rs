#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub weight: size_t,
    pub tail: *mut Node,
    pub count: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodePool {
    pub next: *mut Node,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
unsafe extern "C" fn InitNode(
    mut weight: size_t,
    mut count: core::ffi::c_int,
    mut tail: *mut Node,
    mut node: *mut Node,
) {
    (*node).weight = weight;
    (*node).count = count;
    (*node).tail = tail;
}
unsafe extern "C" fn BoundaryPM(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: core::ffi::c_int,
    mut pool: *mut NodePool,
    mut index: core::ffi::c_int,
) {
    let mut newchain: *mut Node = 0 as *mut Node;
    let mut oldchain: *mut Node = 0 as *mut Node;
    let mut lastcount: core::ffi::c_int = (*(*lists
        .offset(index as isize))[1 as core::ffi::c_int as usize])
        .count;
    if index == 0 as core::ffi::c_int && lastcount >= numsymbols {
        return;
    }
    let fresh4 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    newchain = fresh4;
    oldchain = (*lists.offset(index as isize))[1 as core::ffi::c_int as usize];
    let ref mut fresh5 = (*lists.offset(index as isize))[0 as core::ffi::c_int as usize];
    *fresh5 = oldchain;
    let ref mut fresh6 = (*lists.offset(index as isize))[1 as core::ffi::c_int as usize];
    *fresh6 = newchain;
    if index == 0 as core::ffi::c_int {
        InitNode(
            (*leaves.offset(lastcount as isize)).weight,
            lastcount + 1 as core::ffi::c_int,
            0 as *mut Node,
            newchain,
        );
    } else {
        let mut sum: size_t = ((*(*lists
            .offset(
                (index - 1 as core::ffi::c_int) as isize,
            ))[0 as core::ffi::c_int as usize])
            .weight)
            .wrapping_add(
                (*(*lists
                    .offset(
                        (index - 1 as core::ffi::c_int) as isize,
                    ))[1 as core::ffi::c_int as usize])
                    .weight,
            );
        if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
            InitNode(
                (*leaves.offset(lastcount as isize)).weight,
                lastcount + 1 as core::ffi::c_int,
                (*oldchain).tail,
                newchain,
            );
        } else {
            InitNode(
                sum,
                lastcount,
                (*lists
                    .offset(
                        (index - 1 as core::ffi::c_int) as isize,
                    ))[1 as core::ffi::c_int as usize],
                newchain,
            );
            BoundaryPM(lists, leaves, numsymbols, pool, index - 1 as core::ffi::c_int);
            BoundaryPM(lists, leaves, numsymbols, pool, index - 1 as core::ffi::c_int);
        }
    };
}
unsafe extern "C" fn BoundaryPMFinal(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: core::ffi::c_int,
    mut pool: *mut NodePool,
    mut index: core::ffi::c_int,
) {
    let mut lastcount: core::ffi::c_int = (*(*lists
        .offset(index as isize))[1 as core::ffi::c_int as usize])
        .count;
    let mut sum: size_t = ((*(*lists
        .offset(
            (index - 1 as core::ffi::c_int) as isize,
        ))[0 as core::ffi::c_int as usize])
        .weight)
        .wrapping_add(
            (*(*lists
                .offset(
                    (index - 1 as core::ffi::c_int) as isize,
                ))[1 as core::ffi::c_int as usize])
                .weight,
        );
    if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
        let mut newchain: *mut Node = (*pool).next;
        let mut oldchain: *mut Node = (*(*lists
            .offset(index as isize))[1 as core::ffi::c_int as usize])
            .tail;
        let ref mut fresh2 = (*lists
            .offset(index as isize))[1 as core::ffi::c_int as usize];
        *fresh2 = newchain;
        (*newchain).count = lastcount + 1 as core::ffi::c_int;
        (*newchain).tail = oldchain;
    } else {
        let ref mut fresh3 = (*(*lists
            .offset(index as isize))[1 as core::ffi::c_int as usize])
            .tail;
        *fresh3 = (*lists
            .offset(
                (index - 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize];
    };
}
unsafe extern "C" fn InitLists(
    mut pool: *mut NodePool,
    mut leaves: *const Node,
    mut maxbits: core::ffi::c_int,
    mut lists: *mut [*mut Node; 2],
) {
    let mut i: core::ffi::c_int = 0;
    let fresh7 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    let mut node0: *mut Node = fresh7;
    let fresh8 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    let mut node1: *mut Node = fresh8;
    InitNode(
        (*leaves.offset(0 as core::ffi::c_int as isize)).weight,
        1 as core::ffi::c_int,
        0 as *mut Node,
        node0,
    );
    InitNode(
        (*leaves.offset(1 as core::ffi::c_int as isize)).weight,
        2 as core::ffi::c_int,
        0 as *mut Node,
        node1,
    );
    i = 0 as core::ffi::c_int;
    while i < maxbits {
        let ref mut fresh9 = (*lists.offset(i as isize))[0 as core::ffi::c_int as usize];
        *fresh9 = node0;
        let ref mut fresh10 = (*lists
            .offset(i as isize))[1 as core::ffi::c_int as usize];
        *fresh10 = node1;
        i += 1;
    }
}
unsafe extern "C" fn ExtractBitLengths(
    mut chain: *mut Node,
    mut leaves: *mut Node,
    mut bitlengths: *mut core::ffi::c_uint,
) {
    let mut counts: [core::ffi::c_int; 16] = [0 as core::ffi::c_int; 16];
    let mut end: core::ffi::c_uint = 16 as core::ffi::c_uint;
    let mut ptr: core::ffi::c_uint = 15 as core::ffi::c_uint;
    let mut value: core::ffi::c_uint = 1 as core::ffi::c_uint;
    let mut node: *mut Node = 0 as *mut Node;
    let mut val: core::ffi::c_int = 0;
    node = chain;
    while !node.is_null() {
        end = end.wrapping_sub(1);
        counts[end as usize] = (*node).count;
        node = (*node).tail;
    }
    val = counts[15 as core::ffi::c_int as usize];
    while ptr >= end {
        while val > counts[ptr.wrapping_sub(1 as core::ffi::c_uint) as usize] {
            *bitlengths
                .offset(
                    (*leaves.offset((val - 1 as core::ffi::c_int) as isize)).count
                        as isize,
                ) = value;
            val -= 1;
        }
        ptr = ptr.wrapping_sub(1);
        value = value.wrapping_add(1);
    }
}
unsafe extern "C" fn LeafComparator(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    return ((*(a as *const Node)).weight).wrapping_sub((*(b as *const Node)).weight)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliLengthLimitedCodeLengths(
    mut frequencies: *const size_t,
    mut n: core::ffi::c_int,
    mut maxbits: core::ffi::c_int,
    mut bitlengths: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut pool: NodePool = NodePool { next: 0 as *mut Node };
    let mut i: core::ffi::c_int = 0;
    let mut numsymbols: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut numBoundaryPMRuns: core::ffi::c_int = 0;
    let mut nodes: *mut Node = 0 as *mut Node;
    let mut lists: *mut [*mut Node; 2] = 0 as *mut [*mut Node; 2];
    let mut leaves: *mut Node = malloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<Node>() as size_t),
    ) as *mut Node;
    i = 0 as core::ffi::c_int;
    while i < n {
        *bitlengths.offset(i as isize) = 0 as core::ffi::c_uint;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < n {
        if *frequencies.offset(i as isize) != 0 {
            (*leaves.offset(numsymbols as isize)).weight = *frequencies
                .offset(i as isize);
            (*leaves.offset(numsymbols as isize)).count = i;
            numsymbols += 1;
        }
        i += 1;
    }
    if (1 as core::ffi::c_int) << maxbits < numsymbols {
        free(leaves as *mut core::ffi::c_void);
        return 1 as core::ffi::c_int;
    }
    if numsymbols == 0 as core::ffi::c_int {
        free(leaves as *mut core::ffi::c_void);
        return 0 as core::ffi::c_int;
    }
    if numsymbols == 1 as core::ffi::c_int {
        *bitlengths
            .offset((*leaves.offset(0 as core::ffi::c_int as isize)).count as isize) = 1
            as core::ffi::c_uint;
        free(leaves as *mut core::ffi::c_void);
        return 0 as core::ffi::c_int;
    }
    if numsymbols == 2 as core::ffi::c_int {
        let ref mut fresh0 = *bitlengths
            .offset((*leaves.offset(0 as core::ffi::c_int as isize)).count as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        let ref mut fresh1 = *bitlengths
            .offset((*leaves.offset(1 as core::ffi::c_int as isize)).count as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        free(leaves as *mut core::ffi::c_void);
        return 0 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < numsymbols {
        if (*leaves.offset(i as isize)).weight
            >= (1 as core::ffi::c_int as size_t)
                << (::core::mem::size_of::<size_t>() as usize)
                    .wrapping_mul(8 as usize)
                    .wrapping_sub(9 as usize)
        {
            free(leaves as *mut core::ffi::c_void);
            return 1 as core::ffi::c_int;
        }
        (*leaves.offset(i as isize)).weight = (((*leaves.offset(i as isize)).weight
            as core::ffi::c_ulong) << 9 as core::ffi::c_int
            | (*leaves.offset(i as isize)).count as core::ffi::c_ulong) as size_t;
        i += 1;
    }
    qsort(
        leaves as *mut core::ffi::c_void,
        numsymbols as size_t,
        ::core::mem::size_of::<Node>() as size_t,
        Some(
            LeafComparator
                as unsafe extern "C" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    i = 0 as core::ffi::c_int;
    while i < numsymbols {
        (*leaves.offset(i as isize)).weight >>= 9 as core::ffi::c_int;
        i += 1;
    }
    if (numsymbols - 1 as core::ffi::c_int) < maxbits {
        maxbits = numsymbols - 1 as core::ffi::c_int;
    }
    nodes = malloc(
        ((maxbits * 2 as core::ffi::c_int * numsymbols) as size_t)
            .wrapping_mul(::core::mem::size_of::<Node>() as size_t),
    ) as *mut Node;
    pool.next = nodes;
    lists = malloc(
        (maxbits as size_t)
            .wrapping_mul(::core::mem::size_of::<[*mut Node; 2]>() as size_t),
    ) as *mut [*mut Node; 2];
    InitLists(&mut pool, leaves, maxbits, lists);
    numBoundaryPMRuns = 2 as core::ffi::c_int * numsymbols - 4 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < numBoundaryPMRuns - 1 as core::ffi::c_int {
        BoundaryPM(
            lists,
            leaves,
            numsymbols,
            &mut pool,
            maxbits - 1 as core::ffi::c_int,
        );
        i += 1;
    }
    BoundaryPMFinal(
        lists,
        leaves,
        numsymbols,
        &mut pool,
        maxbits - 1 as core::ffi::c_int,
    );
    ExtractBitLengths(
        (*lists
            .offset(
                (maxbits - 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize],
        leaves,
        bitlengths,
    );
    free(lists as *mut core::ffi::c_void);
    free(leaves as *mut core::ffi::c_void);
    free(nodes as *mut core::ffi::c_void);
    return 0 as core::ffi::c_int;
}
