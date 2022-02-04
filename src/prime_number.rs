#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

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

#[snippet("list_factors-約数列挙")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_number() {
        assert_eq!(is_prime_number(0), false);
        assert_eq!(is_prime_number(1), false);
        assert_eq!(is_prime_number(2), true);
        assert_eq!(is_prime_number(13), true);
        assert_eq!(is_prime_number(100), false);
        assert_eq!(is_prime_number(169), false);
    }
    #[test]
    fn test_list_factors() {
        assert_eq!(list_factors(1), vec![1]);
        assert_eq!(list_factors(2), vec![1, 2]);
        assert_eq!(list_factors(5), vec![1, 5]);
        assert_eq!(list_factors(10), vec![1, 2, 5, 10]);
        assert_eq!(list_factors(169), vec![1, 13, 169]);
    }
}
