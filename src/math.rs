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

//２進数のStringを10進数の数値に変換
#[snippet("from_str_radix-２進数のStringを10進数の数値に変換")]
fn from_str_radix(s: &String, digit: u32) -> i64 {
    i64::from_str_radix(s, digit).unwrap()
}
#[test]
fn test_from_str_radix() {
    assert_eq!(from_str_radix(&String::from("100000000"), 2), 256);
    assert_eq!(from_str_radix(&String::from("100"), 16), 256);
    assert_eq!(from_str_radix(&String::from("400"), 8), 256);
}

// ユークリッドの互除法
#[snippet("GCD-ユークリッドの互除法")]
fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}
#[test]
fn test_gcd() {
    assert_eq!(gcd(9, 6), 3);
    assert_eq!(gcd(6, 9), 3);
    assert_eq!(gcd(39, 26), 13);
    assert_eq!(gcd(144, 24), 24);
    assert_eq!(gcd(5, 7), 1);
}
