#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::{chain, iproduct, iterate, izip, Itertools};

// 検索して存在したらindexを返す
#[snippet("find_index")]
fn find_index<T: PartialEq + Copy>(vec: &Vec<T>, search_target: T) -> usize {
    vec.iter().position(|&x| x == search_target).unwrap()
}

// a-zのVec<char>を返す
#[snippet("a-z")]
fn a_z_vec_char() -> Vec<char> {
    (b'a'..=b'z').map(|b| b as char).collect()
}

#[test]
fn test_find_index() {
    let vec = a_z_vec_char();
    let target = 'x';
    let result = find_index(&vec, target);
    assert_eq!(result, 23);

    let vec_num = vec![1, 2, 3, 4, 5];
    let result_num = find_index(&vec_num, 3);
    assert_eq!(result_num, 2);
}

// 合計
#[snippet("sum")]
fn sum(vec: Vec<u32>) -> u32 {
    vec.iter().sum()
}

#[test]
fn test_sum() {
    assert_eq!(sum(vec![1, 2, 3]), 6);
}

// 文字列の各桁を数値化して合計する
#[snippet("digit sum")]
fn digit_sum(s: &String) -> u32 {
    s.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

#[test]
fn test_digit_sum() {
    let result = digit_sum(&String::from("123456789"));
    assert_eq!(result, 45);
}

// 文字列をVec<u32>に変換
#[snippet("string_to_vec_u32")]
fn string_to_vec_u32(s: &String) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[test]
fn test_string_to_vec_u32() {
    let result = string_to_vec_u32(&String::from("12345"));
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

// rangeからVec<usize>を作成
#[snippet("create_vec_by_range")]
fn create_vec_by_rane(start: usize, end: usize) -> Vec<usize> {
    (start..end).collect()
}

#[test]
fn test_create_vec_by_range() {
    let result = create_vec_by_rane(1, 5);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[snippet]
fn iproduct_sample() -> Vec<(usize, usize)> {
    iproduct!(0..2, 0..2).collect()
}
#[test]
fn test_iproduct_test() {
    let result = iproduct_sample();
    assert_eq!(result[0], (0, 0));
    assert_eq!(result[1], (0, 1));
    assert_eq!(result[2], (1, 0));
    assert_eq!(result[3], (1, 1));
}

#[snippet]
fn tuple_combinations_sample() -> Vec<(usize, usize)> {
    (0..3).tuple_combinations().collect()
}
#[test]
fn test_tuple_combinations() {
    let result = tuple_combinations_sample();
    assert_eq!(result[0], (0, 1));
    assert_eq!(result[1], (0, 2));
    assert_eq!(result[2], (1, 2));
}

#[snippet("vec_to_string-vecを区切り文字で区切った1つの文字列にする")]
fn vec_to_string<T: std::fmt::Display>(v: Vec<T>, s: &str) -> String {
    v.iter().join(s)
}
#[test]
fn test_vec_to_string() {
    assert_eq!(
        vec_to_string(vec![1, 2, 3, 4, 5], ","),
        String::from("1,2,3,4,5")
    );
    assert_eq!(
        vec_to_string(vec![1, 2, 3, 4, 5], " "),
        String::from("1 2 3 4 5")
    );
    assert_eq!(
        vec_to_string(vec![1, 2, 3, 4, 5], " -> "),
        String::from("1 -> 2 -> 3 -> 4 -> 5")
    );
}
