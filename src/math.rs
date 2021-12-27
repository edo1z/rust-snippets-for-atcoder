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

#[snippet("prime_number-素数判定")]
fn is_prime_number(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
#[test]
fn test_is_prime_number() {
    assert_eq!(is_prime_number(0), false);
    assert_eq!(is_prime_number(1), false);
    assert_eq!(is_prime_number(2), true);
    assert_eq!(is_prime_number(13), true);
    assert_eq!(is_prime_number(100), false);
    assert_eq!(is_prime_number(169), false);
}

#[snippet("約数列挙")]
fn list_factors(n: u64) -> Vec<u64> {
    if n <= 0 {
        return vec![];
    }
    let mut ans = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ans.push(i);
            if i != n && i != n / i {
                ans.push(n / i)
            }
        }
        i += 1;
    }
    ans.sort();
    ans
}
#[test]
fn test_list_factors() {
    assert_eq!(list_factors(1), vec![1]);
    assert_eq!(list_factors(2), vec![1, 2]);
    assert_eq!(list_factors(5), vec![1, 5]);
    assert_eq!(list_factors(10), vec![1, 2, 5, 10]);
    assert_eq!(list_factors(169), vec![1, 13, 169]);
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
