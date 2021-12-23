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

//進数変換
#[snippet("進数変換")]
fn convert_digits(num: i64, digit: usize) -> String {
    match digit {
        2 => format!("{:b}", num),
        8 => format!("{:o}", num),
        10 => format!("{}", num),
        16 => format!("{:x}", num),
        _ => format!("{}", num),
    }
}
#[test]
fn test_convert_digits() {
    assert_eq!(convert_digits(32, 8), String::from("40"));
    assert_eq!(convert_digits(0b100000, 10), String::from("32"));
    assert_eq!(convert_digits(0o40, 16), String::from("20"));
    assert_eq!(convert_digits(0x20, 2), String::from("100000"));
}
