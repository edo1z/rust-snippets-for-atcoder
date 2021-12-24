#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::{chain, iproduct, iterate, izip, Itertools};

#[snippet("bit全探索")]
fn full_bit_search(n: u32) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    for i in 0..2_i32.pow(n) {
        let mut state = String::new();
        for j in 0..n {
            state += if i & (1 << j) > 0 { "o" } else { "x" };
        }
        state = state.chars().rev().collect::<String>();
        ans.push(format!("{:0>3b} => {}", i, state));
    }
    ans
}
#[test]
fn test_full_bit_search() {
    let result = full_bit_search(3);
    assert_eq!(result[0], String::from("000 => xxx"));
    assert_eq!(result[1], String::from("001 => xxo"));
    assert_eq!(result[2], String::from("010 => xox"));
}
