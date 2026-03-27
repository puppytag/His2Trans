// c2rust 测试文件 for quadtree

use crate::src_bounds::*;
use crate::src_node::*;

#[test]
fn test_quadtree_bounds_new() {
    unsafe {
        let bounds = quadtree_bounds_new();
        assert!(!bounds.is_null());
        if !bounds.is_null() {
            quadtree_bounds_free(bounds);
        }
    }
}

#[test]
fn test_quadtree_node_new() {
    unsafe {
        let node = quadtree_node_new();
        assert!(!node.is_null());
        if !node.is_null() {
            quadtree_node_free(node);
        }
    }
}

#[test]
#[ignore]
fn test_compile_check() {
}
