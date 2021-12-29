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

#[snippet("merge_sort-マージソート")]
#[snippet(include = "_merge")]
fn merge_sort(v: &mut Vec<usize>) {
    if v.len() == 1 {
        return;
    }
    let m = ((v.len() as f64 / 2.0).floor()) as usize;
    let mut a: Vec<usize> = v[0..m].to_vec();
    let mut b: Vec<usize> = v[m..v.len()].to_vec();
    merge_sort(&mut a);
    merge_sort(&mut b);
    *v = _merge(a, b);
}
#[snippet]
fn _merge(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut c: Vec<usize> = vec![];
    let mut a_idx = 0;
    let mut b_idx = 0;
    loop {
        if a_idx >= a.len() && b_idx >= b.len() {
            break;
        } else if a_idx >= a.len() {
            c.push(b[b_idx]);
            b_idx += 1;
        } else if b_idx >= b.len() {
            c.push(a[a_idx]);
            a_idx += 1;
        } else if a[a_idx] > b[b_idx] {
            c.push(b[b_idx]);
            b_idx += 1;
        } else if b[b_idx] >= a[a_idx] {
            c.push(a[a_idx]);
            a_idx += 1;
        }
    }
    c
}
#[test]
fn test_merge_sort() {
    let mut v = vec![2, 3, 5, 4, 1, 6, 10, 8, 9, 7];
    merge_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
