#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

// ユークリッドの互除法
#[snippet("gcd")]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// 拡張ユークリッドの互除法
#[snippet("ext_gcd")]
fn ext_gcd(a: i32, b: i32) -> (i32, i32) {
    if b == 0 {
        (1, 0)
    } else {
        let (s, t) = ext_gcd(b, a % b);
        (t, s - a / b * t)
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

    #[test]
    fn test_ext_gcd() {
        assert_eq!(ext_gcd(111, 30), (3, -11));
        assert_eq!(ext_gcd(12707, 12319), (32, -33));
        assert_eq!(ext_gcd(13, 5), (2, -5));
    }
}
