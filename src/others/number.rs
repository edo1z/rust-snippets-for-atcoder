#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

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

//２進数のStringを10進数の数値に変換
#[snippet("from_str_radix-２進数のStringを10進数の数値に変換")]
fn from_str_radix(s: &String, digit: u32) -> i64 {
    i64::from_str_radix(s, digit).unwrap()
}

#[snippet("数値の各桁のVecを作成")]
fn num_to_vec_of_digit(num: u64) -> Vec<u64> {
    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
}

#[snippet("各桁のVecを合体させて１つの数値にする")]
fn vec_of_digit_to_num(vec: Vec<u64>) -> u64 {
    vec.iter()
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[snippet("10進数以外から10進数への変換（10進数未満対応）")]
#[snippet(include = "数値の各桁のVecを作成")]
fn to_decimal(num: u64, radix: u64) -> u64 {
    let vec = num_to_vec_of_digit(num);
    let mut ans: u64 = 0;
    for (i, v) in vec.iter().enumerate() {
        ans += *v as u64 * radix.pow((vec.len() - i - 1) as u32);
    }
    ans
}

#[snippet("10進数から10進数以外への変換")]
fn from_decimal(mut num: u64, radix: u64) -> u64 {
    let mut v = vec![];
    loop {
        v.push(num % radix);
        num /= radix;
        if num == 0 {
            break;
        }
    }
    let ans = v
        .iter()
        .rev()
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_digits() {
        assert_eq!(convert_digits(32, 8), String::from("40"));
        assert_eq!(convert_digits(0b100000, 10), String::from("32"));
        assert_eq!(convert_digits(0o40, 16), String::from("20"));
        assert_eq!(convert_digits(0x20, 2), String::from("100000"));
    }
    #[test]
    fn test_from_str_radix() {
        assert_eq!(from_str_radix(&String::from("100000000"), 2), 256);
        assert_eq!(from_str_radix(&String::from("100"), 16), 256);
        assert_eq!(from_str_radix(&String::from("400"), 8), 256);
    }
    #[test]
    fn test_num_to_vec_of_digit() {
        assert_eq!(num_to_vec_of_digit(10), vec![1, 0]);
        assert_eq!(num_to_vec_of_digit(234), vec![2, 3, 4]);
    }
    #[test]
    fn test_vec_of_digit_to_num() {
        assert_eq!(vec_of_digit_to_num(vec![1, 2, 3]), 123);
        assert_eq!(vec_of_digit_to_num(vec![0, 1, 2]), 12);
    }
    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(1, 2), 1);
        assert_eq!(to_decimal(1000, 2), 8);
        assert_eq!(to_decimal(101, 2), 5);
        assert_eq!(to_decimal(2011, 8), 1033);
        assert_eq!(to_decimal(111, 3), 13);
        assert_eq!(to_decimal(223, 9), 183);
    }
    #[test]
    fn test_from_decimal() {
        assert_eq!(from_decimal(1, 2), 1);
        assert_eq!(from_decimal(8, 2), 1000);
        assert_eq!(from_decimal(5, 2), 101);
        assert_eq!(from_decimal(1033, 8), 2011);
        assert_eq!(from_decimal(13, 3), 111);
        assert_eq!(from_decimal(183, 9), 223);
    }
}
