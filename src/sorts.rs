#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::{chain, iproduct, iterate, izip, Itertools};

#[snippet("selection_sort-選択ソート")]
fn selection_sort(v: &mut Vec<usize>) {
    for i in 0..v.len() {
        let mut min_idx = i;
        let mut min_val = std::usize::MAX;
        for j in i..v.len() {
            if v[j] < min_val {
                min_val = v[j];
                min_idx = j;
            }
        }
        v.swap(i, min_idx);
    }
}
#[test]
fn test_selection_sort() {
    let mut v = vec![2, 3, 5, 4, 1, 6, 10, 8, 9, 7];
    selection_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
