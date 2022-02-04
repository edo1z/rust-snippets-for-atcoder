#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

pub fn repeated_pow(x: usize, mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut ans = x;
    let mut cnt = 0;
    while n > 1 {
        if n % 2 != 0 {
            n -= 1;
            cnt += 1;
        }
        n /= 2;
        ans *= ans;
    }
    ans * x.pow(cnt)
}

pub fn repeated_pow_2(x: usize, mut n: usize) -> usize {
    let mut d = x;
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans *= d;
        }
        n = n >> 1;
        d *= d;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_pow() {
        assert_eq!(repeated_pow(2, 1), 2);
        assert_eq!(repeated_pow(2, 2), 4);
        assert_eq!(repeated_pow(2, 8), 256);
        assert_eq!(repeated_pow(2, 3), 8);
        assert_eq!(repeated_pow(2, 16), 65536);
        assert_eq!(repeated_pow(2, 5), 32);
        assert_eq!(repeated_pow(3, 0), 1);
        assert_eq!(repeated_pow(3, 1), 3);
        assert_eq!(repeated_pow(3, 3), 27);
        assert_eq!(repeated_pow(3, 5), 243);
    }

    #[test]
    fn test_repeated_pow_2() {
        assert_eq!(repeated_pow_2(2, 1), 2);
        assert_eq!(repeated_pow_2(2, 2), 4);
        assert_eq!(repeated_pow_2(2, 8), 256);
        assert_eq!(repeated_pow_2(2, 3), 8);
        assert_eq!(repeated_pow_2(2, 16), 65536);
        assert_eq!(repeated_pow_2(2, 5), 32);
        assert_eq!(repeated_pow_2(3, 0), 1);
        assert_eq!(repeated_pow_2(3, 1), 3);
        assert_eq!(repeated_pow_2(3, 3), 27);
        assert_eq!(repeated_pow_2(3, 5), 243);
    }
}
