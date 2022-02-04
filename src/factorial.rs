#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

// 階乗
#[snippet("factorial - 階乗")]
#[snippet(include = "_factorial")]
fn factorial(n: usize) -> usize {
    let mut memo: Vec<usize> = vec![1; n + 1];
    _factorial(n, &mut memo)
}
#[snippet]
fn _factorial(n: usize, memo: &mut Vec<usize>) -> usize {
    if n < 2 || memo[n] > 1 {
        memo[n]
    } else {
        memo[n] = _factorial(n - 1, memo) * n;
        memo[n]
    }
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
}
