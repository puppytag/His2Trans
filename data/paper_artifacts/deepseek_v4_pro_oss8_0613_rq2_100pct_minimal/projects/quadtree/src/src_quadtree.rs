use crate::types::{quadtree_t, quadtree_node_t, quadtree_point_t, c_void};
use crate::src_node::{
    quadtree_node_free, quadtree_node_isleaf, quadtree_node_isempty,
    quadtree_node_ispointer, quadtree_node_reset, quadtree_node_with_bounds_box,
    node_reset_impl,
};
use std::ptr;

enum Quadrant { NW, NE, SW, SE }

// --- Private helpers ---

unsafe extern "C" fn elision_(_key: *mut c_void) {}

fn node_contains_(outer: &quadtree_node_t, it: &quadtree_point_t) -> bool {
    if let Some(b) = outer.bounds_as_ref() {
        let nw = b.nw_as_ref();
        let se = b.se_as_ref();
        nw.x < it.x && nw.y > it.y && se.x > it.x && se.y < it.y
    } else {
        false
    }
}

fn get_quadrant_(root: &quadtree_node_t, point: &quadtree_point_t) -> Option<Quadrant> {
    if let Some(nw) = root.nw_as_ref() { if node_contains_(nw, point) { return Some(Quadrant::NW); } }
    if let Some(ne) = root.ne_as_ref() { if node_contains_(ne, point) { return Some(Quadrant::NE); } }
    if let Some(sw) = root.sw_as_ref() { if node_contains_(sw, point) { return Some(Quadrant::SW); } }
    if let Some(se) = root.se_as_ref() { if node_contains_(se, point) { return Some(Quadrant::SE); } }
    None
}

fn split_node_(node: &mut quadtree_node_t, key_free: Option<unsafe extern "C" fn(*mut c_void)>) -> i32 {
    let b = match node.bounds_as_ref() {
        Some(b) => b,
        None => return 0,
    };
    let x = b.nw_as_ref().x;
    let y = b.nw_as_ref().y;
    let hw = b.width / 2.0;
    let hh = b.height / 2.0;

    macro_rules! make_child {
        ($mx:expr, $my:expr, $Mx:expr, $My:expr) => {{
            match quadtree_node_with_bounds_box($mx, $my, $Mx, $My) {
                Some(node) => node,
                None => return 0,
            }
        }};
    }

    let nw_child = make_child!(x, y - hh, x + hw, y);
    let ne_child = make_child!(x + hw, y - hh, x + hw * 2.0, y);
    let sw_child = make_child!(x, y - hh * 2.0, x + hw, y - hh);
    let se_child = make_child!(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);

    node.nw = Some(nw_child);
    node.ne = Some(ne_child);
    node.sw = Some(sw_child);
    node.se = Some(se_child);

    let old_point = node.point.take();
    let old_key = node.key;
    node.key = ptr::null_mut();

    insert_(node, old_point, old_key, key_free)
}

fn insert_(
    root: &mut quadtree_node_t,
    point: Option<Box<quadtree_point_t>>,
    key: *mut c_void,
    key_free: Option<unsafe extern "C" fn(*mut c_void)>,
) -> i32 {
    if root.is_empty() {
        root.point = point;
        root.key = key;
        return 1;
    }
    if root.is_leaf() {
        let eq = match (root.point_as_ref(), point.as_ref()) {
            (Some(p), Some(pt)) => p.x == pt.x && p.y == pt.y,
            _ => false,
        };
        if eq {
            let kf = key_free.or(Some(elision_));
            node_reset_impl(root, kf);
            root.point = point;
            root.key = key;
            return 0;
        }
        if split_node_(root, key_free) == 0 {
            return 0;
        }
        return insert_(root, point, key, key_free);
    }
    if root.is_pointer() {
        let pt = match point.as_ref() {
            Some(p) => p,
            None => return 0,
        };
        let quadrant = get_quadrant_(root, pt);
        let child = match quadrant {
            Some(Quadrant::NW) => root.nw_as_mut(),
            Some(Quadrant::NE) => root.ne_as_mut(),
            Some(Quadrant::SW) => root.sw_as_mut(),
            Some(Quadrant::SE) => root.se_as_mut(),
            None => return 0,
        };
        if let Some(child) = child {
            return insert_(child, point, key, key_free);
        }
    }
    0
}

fn find_(node: &quadtree_node_t, x: f64, y: f64) -> *mut quadtree_point_t {
    if node.is_leaf() {
        if let Some(p) = node.point_as_ref() {
            if p.x == x && p.y == y {
                return p as *const quadtree_point_t as *mut quadtree_point_t;
            }
        }
    } else {
        let test = quadtree_point_t { x, y };
        if let Some(q) = get_quadrant_(node, &test) {
            let child = match q {
                Quadrant::NW => node.nw_as_ref(),
                Quadrant::NE => node.ne_as_ref(),
                Quadrant::SW => node.sw_as_ref(),
                Quadrant::SE => node.se_as_ref(),
            };
            if let Some(child) = child {
                return find_(child, x, y);
            }
        }
    }
    ptr::null_mut()
}

// --- Public API ---

#[no_mangle]
pub extern "C" fn quadtree_new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> *mut quadtree_t {
    let root = match quadtree_node_with_bounds_box(minx, miny, maxx, maxy) {
        Some(n) => n,
        None => return ptr::null_mut(),
    };
    Box::into_raw(Box::new(quadtree_t {
        root: Some(root),
        key_free: None,
        length: 0,
    }))
}

#[no_mangle]
pub extern "C" fn quadtree_free(tree: *mut quadtree_t) {
    if tree.is_null() { return; }
    let (root, key_free) = unsafe {
        let t = &mut *tree;
        let root = t.root.take();
        let kf = t.key_free;
        (root, kf)
    };
    if let Some(root_box) = root {
        let root_ptr = Box::into_raw(root_box);
        let kf = key_free.or(Some(elision_));
        quadtree_node_free(root_ptr, kf);
    }
    unsafe { drop(Box::from_raw(tree)); }
}

#[no_mangle]
pub extern "C" fn quadtree_search(tree: *mut quadtree_t, x: f64, y: f64) -> *mut quadtree_point_t {
    let t = match unsafe { tree.as_ref() } {
        Some(t) => t,
        None => return ptr::null_mut(),
    };
    match t.root.as_deref() {
        Some(r) => find_(r, x, y),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn quadtree_insert(tree: *mut quadtree_t, x: f64, y: f64, key: *mut c_void) -> i32 {
    let t = match unsafe { tree.as_mut() } {
        Some(t) => t,
        None => return 0,
    };
    let point = Box::new(quadtree_point_t { x, y });
    {
        let root_ref = match t.root.as_deref() {
            Some(r) => r,
            None => return 0,
        };
        if !node_contains_(root_ref, &point) {
            return 0;
        }
    }
    let root_mut = match t.root.as_deref_mut() {
        Some(r) => r,
        None => return 0,
    };
    let key_free = t.key_free;
    let result = insert_(root_mut, Some(point), key, key_free);
    if result != 0 {
        t.length += 1;
    }
    result
}

#[no_mangle]
pub extern "C" fn quadtree_walk(
    root: *mut quadtree_node_t,
    descent: Option<unsafe extern "C" fn(*mut quadtree_node_t)>,
    ascent: Option<unsafe extern "C" fn(*mut quadtree_node_t)>,
) {
    let r = match unsafe { root.as_mut() } {
        Some(r) => r,
        None => return,
    };
    if let Some(d) = descent {
        unsafe { d(root); }
    }
    if let Some(child) = r.nw.as_deref_mut() {
        quadtree_walk(child as *mut quadtree_node_t, descent, ascent);
    }
    if let Some(child) = r.ne.as_deref_mut() {
        quadtree_walk(child as *mut quadtree_node_t, descent, ascent);
    }
    if let Some(child) = r.sw.as_deref_mut() {
        quadtree_walk(child as *mut quadtree_node_t, descent, ascent);
    }
    if let Some(child) = r.se.as_deref_mut() {
        quadtree_walk(child as *mut quadtree_node_t, descent, ascent);
    }
    if let Some(a) = ascent {
        unsafe { a(root); }
    }
}
