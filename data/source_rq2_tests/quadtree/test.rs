// Rust translation of quadtree test.c
// Translated from C to Rust

use std::os::raw::{c_double, c_int, c_uint, c_void};
use std::ptr;
use std::time::Instant;

// Quadtree point structure
#[repr(C)]
pub struct quadtree_point_t {
    pub x: c_double,
    pub y: c_double,
    pub key: *mut c_void,
}

// Quadtree node structure
#[repr(C)]
pub struct quadtree_node_t {
    pub nw: *mut quadtree_node_t,
    pub ne: *mut quadtree_node_t,
    pub sw: *mut quadtree_node_t,
    pub se: *mut quadtree_node_t,
    pub bounds: *mut c_void,
    pub point: *mut quadtree_point_t,
    pub key: *mut c_void,
}

// Quadtree structure
#[repr(C)]
pub struct quadtree_t {
    pub root: *mut quadtree_node_t,
    pub key_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub length: c_uint,
}

// External declarations for quadtree library functions
extern "C" {
    fn quadtree_new(minx: c_double, miny: c_double, maxx: c_double, maxy: c_double) -> *mut quadtree_t;
    fn quadtree_free(qt: *mut quadtree_t);
    fn quadtree_insert(qt: *mut quadtree_t, x: c_double, y: c_double, key: *mut c_void) -> c_int;
    fn quadtree_search(qt: *mut quadtree_t, x: c_double, y: c_double) -> *mut quadtree_point_t;
    fn quadtree_walk(
        node: *mut quadtree_node_t,
        descent: Option<unsafe extern "C" fn(*mut quadtree_node_t)>,
        ascent: Option<unsafe extern "C" fn(*mut quadtree_node_t)>,
    );
    fn quadtree_node_isleaf(node: *mut quadtree_node_t) -> c_int;
    fn quadtree_node_isempty(node: *mut quadtree_node_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
}

// LCG random number generator (deterministic)
fn lcg32(state: &mut u32) -> u32 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    *state
}

// Calculate max depth of quadtree
fn max_depth(n: *mut quadtree_node_t) -> i32 {
    if n.is_null() {
        return 0;
    }
    unsafe {
        let d_nw = max_depth((*n).nw);
        let d_ne = max_depth((*n).ne);
        let d_sw = max_depth((*n).sw);
        let d_se = max_depth((*n).se);
        let mut max = d_nw;
        if d_ne > max { max = d_ne; }
        if d_sw > max { max = d_sw; }
        if d_se > max { max = d_se; }
        1 + max
    }
}

// Walk statistics structure
#[derive(Default)]
struct WalkStats {
    nodes: u32,
    leaves_with_points: u32,
    empty_leaves: u32,
    internal_nodes: u32,
}

// Global statistics (using thread-local storage for safety)
static mut G_STATS: WalkStats = WalkStats {
    nodes: 0,
    leaves_with_points: 0,
    empty_leaves: 0,
    internal_nodes: 0,
};

// Callback function for tree descent
unsafe extern "C" fn on_descent(node: *mut quadtree_node_t) {
    G_STATS.nodes += 1;
    if quadtree_node_isleaf(node) != 0 {
        G_STATS.leaves_with_points += 1;
    } else if quadtree_node_isempty(node) != 0 {
        G_STATS.empty_leaves += 1;
    } else {
        G_STATS.internal_nodes += 1;
    }
}

// Callback function for tree ascent (empty)
unsafe extern "C" fn on_ascent(_node: *mut quadtree_node_t) {
    // No-op
}

fn main() {
    const N: usize = 5000;
    const Q: usize = 20000;

    unsafe {
        // Use bounds that yield fractional quadrant boundaries
        let minx = -1.0;
        let miny = -1.0;
        let maxx = 1000.0;
        let maxy = 1000.0;

        let qt = quadtree_new(minx, miny, maxx, maxy);
        assert!(!qt.is_null(), "Failed to create quadtree");

        // Enable key freeing
        (*qt).key_free = Some(free);

        let mut xs: Vec<i32> = vec![0; N];
        let mut ys: Vec<i32> = vec![0; N];

        // Generate unique integer points within (0..999)
        let seen = calloc(1000 * 1000, 1) as *mut u8;
        assert!(!seen.is_null(), "Failed to allocate seen bitmap");

        let mut rng: u32 = 1;
        let mut inserted = 0;

        while inserted < N {
            let x = (lcg32(&mut rng) % 1000) as i32;
            let y = (lcg32(&mut rng) % 1000) as i32;
            let idx = (y * 1000 + x) as usize;

            if *seen.add(idx) != 0 {
                continue;
            }
            *seen.add(idx) = 1;

            let key = malloc(std::mem::size_of::<i32>()) as *mut i32;
            assert!(!key.is_null(), "Failed to allocate key");
            *key = inserted as i32;

            let ok = quadtree_insert(qt, x as c_double, y as c_double, key as *mut c_void);
            assert_eq!(ok, 1, "Insert failed");

            xs[inserted] = x;
            ys[inserted] = y;
            inserted += 1;
        }

        assert_eq!((*qt).length, N as c_uint, "Length mismatch");

        // Verify search works for inserted points
        for i in 0..N {
            let p = quadtree_search(qt, xs[i] as c_double, ys[i] as c_double);
            assert!(!p.is_null(), "Search returned null for point {}", i);
            assert_eq!((*p).x, xs[i] as c_double);
            assert_eq!((*p).y, ys[i] as c_double);
        }

        // Duplicate insertion test
        {
            let dup_key = malloc(std::mem::size_of::<i32>()) as *mut i32;
            assert!(!dup_key.is_null());
            *dup_key = 123456;
            let rc = quadtree_insert(qt, xs[0] as c_double, ys[0] as c_double, dup_key as *mut c_void);
            assert_eq!(rc, 0, "Duplicate insert should return 0");
            assert_eq!((*qt).length, N as c_uint, "Length should not change on duplicate");
            let p = quadtree_search(qt, xs[0] as c_double, ys[0] as c_double);
            assert!(!p.is_null());
        }

        // Walk stats: count nodes and point-bearing leaves
        G_STATS = WalkStats::default();
        quadtree_walk((*qt).root, Some(on_descent), Some(on_ascent));
        assert_eq!(G_STATS.leaves_with_points, N as u32);
        assert!(G_STATS.nodes > G_STATS.leaves_with_points);

        let depth = max_depth((*qt).root);
        assert!(depth > 0);

        // Stress: random searches
        let t0 = Instant::now();
        let mut hits: u32 = 0;
        for _ in 0..Q {
            let idx = (lcg32(&mut rng) % N as u32) as usize;
            let x = xs[idx];
            let y = ys[idx];
            let p = quadtree_search(qt, x as c_double, y as c_double);
            assert!(!p.is_null());
            hits += 1;
        }
        let elapsed = t0.elapsed();
        let sec = elapsed.as_secs_f64();

        println!("quadtree tests: ok");
        println!("  inserted={} length={}", inserted, (*qt).length);
        println!(
            "  walk.nodes={} leaves_with_points={} empty_leaves={} internal_nodes={}",
            G_STATS.nodes, G_STATS.leaves_with_points, G_STATS.empty_leaves, G_STATS.internal_nodes
        );
        println!("  max_depth={}", depth);
        println!("  random_searches={} hits={} time_sec={:.6}", Q, hits, sec);

        free(seen as *mut c_void);
        quadtree_free(qt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadtree_insert_and_search() {
        unsafe {
            let qt = quadtree_new(-1.0, -1.0, 100.0, 100.0);
            assert!(!qt.is_null());

            let key = malloc(std::mem::size_of::<i32>()) as *mut i32;
            *key = 42;
            let ok = quadtree_insert(qt, 50.0, 50.0, key as *mut c_void);
            assert_eq!(ok, 1);

            let p = quadtree_search(qt, 50.0, 50.0);
            assert!(!p.is_null());
            assert_eq!((*p).x, 50.0);
            assert_eq!((*p).y, 50.0);

            quadtree_free(qt);
        }
    }

    #[test]
    fn test_quadtree_duplicate() {
        unsafe {
            let qt = quadtree_new(-1.0, -1.0, 100.0, 100.0);
            (*qt).key_free = Some(free);

            let key1 = malloc(std::mem::size_of::<i32>()) as *mut i32;
            *key1 = 1;
            quadtree_insert(qt, 50.0, 50.0, key1 as *mut c_void);

            let key2 = malloc(std::mem::size_of::<i32>()) as *mut i32;
            *key2 = 2;
            let rc = quadtree_insert(qt, 50.0, 50.0, key2 as *mut c_void);
            assert_eq!(rc, 0);
            assert_eq!((*qt).length, 1);

            quadtree_free(qt);
        }
    }

    #[test]
    fn test_max_depth() {
        unsafe {
            let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
            (*qt).key_free = Some(free);

            // Insert points to create some depth
            for i in 0..10 {
                let key = malloc(std::mem::size_of::<i32>()) as *mut i32;
                *key = i;
                quadtree_insert(qt, (i * 50) as f64, (i * 50) as f64, key as *mut c_void);
            }

            let depth = max_depth((*qt).root);
            assert!(depth > 0);

            quadtree_free(qt);
        }
    }
}
