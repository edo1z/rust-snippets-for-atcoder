#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

// フィボナッチ数列
#[snippet("fibo-フィボナッチ数列")]
#[snippet(include = "_fibo")]
fn fibo(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    _fibo(n, &mut memo)
}
#[snippet("_fibo")]
fn _fibo(n: usize, memo: &mut Vec<usize>) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if memo[n] > 0 {
        memo[n]
    } else {
        memo[n] = _fibo(n - 1, memo) + _fibo(n - 2, memo);
        memo[n]
    }
}
#[test]
fn test_fibo() {
    assert_eq!(fibo(10), 55);
    assert_eq!(fibo(11), 89);
}
