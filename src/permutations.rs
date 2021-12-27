#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::{chain, iproduct, iterate, izip, Itertools};

#[snippet("permutations-順列全列挙")]
fn permutaions(start: usize, end: usize, num: usize) -> Vec<Vec<usize>> {
    (start..end).permutations(num).collect()
}

#[test]
fn test_permutations() {
    let v = permutaions(5, 8, 2);
    assert_eq!(v[0], vec![5, 6]);
    assert_eq!(v[1], vec![5, 7]);
    assert_eq!(v[2], vec![6, 5]);
    assert_eq!(v.len(), 6);
}

#[snippet("combinations-組み合わせ")]
fn combinations(start: usize, end: usize, num: usize) -> Vec<Vec<usize>> {
    (start..end).combinations(num).collect()
}
#[test]
fn test_combinations() {
    let v = combinations(5, 8, 2);
    assert_eq!(v[0], vec![5, 6]);
    assert_eq!(v[1], vec![5, 7]);
    assert_eq!(v[2], vec![6, 7]);
    assert_eq!(v.len(), 3);
}
