#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

// 二分探索
// s以上で一番小さい数の一番左のindexを返す
#[snippet("lower_bound - 二分探索")]
fn lower_bound(v: &Vec<usize>, s: usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left != right {
        let mid = (left + right) / 2;
        if s > v[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

#[test]
fn test_lower_bound() {
    let vec = vec![
        1, 12, 12, 12, 12, 35, 44, 44, 44, 62, 65, 72, 75, 81, 88, 90, 92, 95, 100,
    ];
    assert_eq!(lower_bound(&vec, 44), 6);
    assert_eq!(lower_bound(&vec, 75), 12);
    assert_eq!(lower_bound(&vec, 13), 5);
    assert_eq!(lower_bound(&vec, 12), 1);
    assert_eq!(lower_bound(&vec, 0), 0);
}

// sより大きくて一番小さい数の一番左のindexを返す
#[snippet("upper_bound - 二分探索")]
fn upper_bound(v: &Vec<usize>, s: usize) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left != right {
        let mid = (left + right) / 2;
        if s < v[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[test]
fn test_upper_bound() {
    let vec = vec![
        1, 12, 12, 12, 12, 35, 44, 44, 44, 62, 65, 72, 75, 81, 88, 90, 92, 95, 100,
    ];
    assert_eq!(upper_bound(&vec, 44), 9);
    assert_eq!(upper_bound(&vec, 75), 13);
    assert_eq!(upper_bound(&vec, 13), 5);
    assert_eq!(upper_bound(&vec, 12), 5);
    assert_eq!(upper_bound(&vec, 0), 0);
}
