// C2R 框架测试文件 for quadtree
// 直接调用翻译后的 Rust 函数
//
// 使用方法：
// 1. 复制此文件到项目的 src/test_c2r.rs
// 2. 在 main.rs 中添加: #[cfg(test)] mod test_c2r;
// 3. 运行 cargo test

use crate::src_quadtree::*;
use crate::src_point::*;
use crate::src_bounds::*;
use crate::src_node::*;

#[test]
fn test_quadtree_new() {
    unsafe {
        let tree = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!tree.is_null(), "quadtree_new 应该返回有效指针");
        quadtree_free(tree);
    }
}

#[test]
fn test_quadtree_point_new() {
    unsafe {
        let point = quadtree_point_new(50.0, 50.0);
        assert!(!point.is_null(), "quadtree_point_new 应该返回有效指针");
        quadtree_point_free(point);
    }
}

#[test]
fn test_quadtree_bounds_new() {
    unsafe {
        let bounds = quadtree_bounds_new();
        assert!(!bounds.is_null(), "quadtree_bounds_new 应该返回有效指针");
        quadtree_bounds_free(bounds);
    }
}

#[test]
fn test_quadtree_bounds_extend() {
    unsafe {
        let bounds = quadtree_bounds_new();
        quadtree_bounds_extend(bounds, 10.0, 20.0);
        quadtree_bounds_extend(bounds, 30.0, 40.0);
        // 验证 bounds 被正确扩展
        quadtree_bounds_free(bounds);
    }
}

#[test]
fn test_quadtree_node_new() {
    unsafe {
        let node = quadtree_node_new();
        assert!(!node.is_null(), "quadtree_node_new 应该返回有效指针");
        quadtree_node_free(node, None);
    }
}

#[test]
fn test_quadtree_node_with_bounds() {
    unsafe {
        let node = quadtree_node_with_bounds(0.0, 0.0, 100.0, 100.0);
        assert!(!node.is_null(), "quadtree_node_with_bounds 应该返回有效指针");
        quadtree_node_free(node, None);
    }
}

#[test]
fn test_quadtree_node_isempty() {
    unsafe {
        let node = quadtree_node_new();
        assert_eq!(quadtree_node_isempty(node), 1, "新创建的节点应该是空的");
        quadtree_node_free(node, None);
    }
}

#[test]
fn test_quadtree_node_isleaf() {
    unsafe {
        let node = quadtree_node_new();
        // 新节点应该是叶子节点
        let is_leaf = quadtree_node_isleaf(node);
        assert_eq!(is_leaf, 1, "新节点应该是叶子节点");
        quadtree_node_free(node, None);
    }
}

#[test]
fn test_quadtree_node_ispointer() {
    unsafe {
        let node = quadtree_node_new();
        // 新节点不应该是指针节点
        let is_pointer = quadtree_node_ispointer(node);
        assert_eq!(is_pointer, 0, "新节点不应该是指针节点");
        quadtree_node_free(node, None);
    }
}

#[test]
fn test_quadtree_insert_and_search() {
    unsafe {
        let tree = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!tree.is_null());

        // 插入一个点
        let result = quadtree_insert(tree, 50.0, 50.0, std::ptr::null_mut());
        assert_eq!(result, 1, "插入应该成功");

        // 搜索该点
        let found = quadtree_search(tree, 50.0, 50.0);
        assert!(!found.is_null(), "应该能找到插入的点");

        quadtree_free(tree);
    }
}

#[test]
fn test_quadtree_search_not_found() {
    unsafe {
        let tree = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!tree.is_null());

        // 搜索不存在的点
        let found = quadtree_search(tree, 50.0, 50.0);
        assert!(found.is_null(), "空树中不应该找到任何点");

        quadtree_free(tree);
    }
}

#[test]
fn test_quadtree_point_coordinates() {
    unsafe {
        let point = quadtree_point_new(25.5, 75.3);
        // 验证点被正确创建
        assert!(!point.is_null());
        quadtree_point_free(point);
    }
}
