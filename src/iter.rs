#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

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
