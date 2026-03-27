// simcrat 测试文件 for quadtree
//
// 目标：与 evolc2rust/c2saferrust 的外部测试语义保持一致（new/insert/search/duplicate/length）。
// 注意：simcrat 的 QuadtreePointT 在翻译中被退化成 usize，坐标信息可能丢失，
// 这类问题应通过这些测试真实暴露出来。

use crate::*;

#[test]
fn test_quadtree_new_and_free() {
    let qt = quadtree_new(0.0, 0.0, 100.0, 100.0).expect("quadtree_new failed");
    assert!(qt.root.is_some(), "root 不应为 None");
    assert_eq!(qt.length, 0, "新建树的 length 应为 0");
    quadtree_free(Box::new(qt));
}

#[test]
fn test_quadtree_insert_and_search() {
    let mut qt = quadtree_new(0.0, 0.0, 100.0, 100.0).expect("quadtree_new failed");

    let key: Box<dyn std::any::Any> = Box::new(0x1234usize);
    quadtree_insert(&mut qt, 10.0, 20.0, key).expect("insert should succeed");
    assert_eq!(qt.length, 1, "length 应增加");

    let p = quadtree_search(&qt, 10.0, 20.0);
    assert!(p.is_some(), "search 应该能找到已插入点");

    let miss = quadtree_search(&qt, 11.0, 20.0);
    assert!(miss.is_none(), "不存在的点应该返回 None");
}

#[test]
fn test_quadtree_duplicate_insert() {
    let mut qt = quadtree_new(0.0, 0.0, 100.0, 100.0).expect("quadtree_new failed");

    let key1: Box<dyn std::any::Any> = Box::new(0x1111usize);
    let key2: Box<dyn std::any::Any> = Box::new(0x2222usize);

    quadtree_insert(&mut qt, 1.0, 2.0, key1).expect("first insert should succeed");
    assert_eq!(qt.length, 1);

    // 同坐标重复插入应失败（或至少不改变 length）
    assert!(quadtree_insert(&mut qt, 1.0, 2.0, key2).is_err());
    assert_eq!(qt.length, 1);
}

#[test]
fn test_quadtree_multiple_points() {
    let mut qt = quadtree_new(0.0, 0.0, 100.0, 100.0).expect("quadtree_new failed");

    let a: Box<dyn std::any::Any> = Box::new(0xaaaausize);
    let b: Box<dyn std::any::Any> = Box::new(0xbbbbusize);
    let c: Box<dyn std::any::Any> = Box::new(0xccccusize);

    quadtree_insert(&mut qt, 10.0, 10.0, a).expect("insert a");
    quadtree_insert(&mut qt, 90.0, 10.0, b).expect("insert b");
    quadtree_insert(&mut qt, 50.0, 90.0, c).expect("insert c");
    assert_eq!(qt.length, 3);

    assert!(quadtree_search(&qt, 10.0, 10.0).is_some());
    assert!(quadtree_search(&qt, 90.0, 10.0).is_some());
    assert!(quadtree_search(&qt, 50.0, 90.0).is_some());
}

