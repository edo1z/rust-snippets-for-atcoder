#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

// ユークリッドの互除法
#[snippet("GCD-ユークリッドの互除法")]
fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(9, 6), 3);
        assert_eq!(gcd(6, 9), 3);
        assert_eq!(gcd(39, 26), 13);
        assert_eq!(gcd(144, 24), 24);
        assert_eq!(gcd(5, 7), 1);
    }
}
