#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

// charを数値とみなして計算する
#[snippet("char_to_u32")]
fn char_to_u32(c: Vec<char>) -> u32 {
    let a = c[0].to_digit(10).unwrap();
    let b = c[1].to_digit(10).unwrap();
    a * b
}

#[test]
fn test_char_to_u32() {
    let result = char_to_u32(vec!['3', '5']);
    assert_eq!(result, 15);
}

// 2つのVec<char>の文字を１文字ずつ交互に連結させる
#[snippet("interleave")]
fn interleave_vec_char(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    a.into_iter().interleave(b).collect()
}

#[test]
fn test_interleave_vec_char() {
    let a = vec!['a', 'b', 'c'];
    let b = vec!['d', 'e', 'f'];
    let result = interleave_vec_char(a, b);
    assert_eq!(result, vec!['a', 'd', 'b', 'e', 'c', 'f']);
}

// 2つの文字列のidx番目を交換する
#[snippet("swap")]
fn swap_char(a: &String, b: &String, idx: usize) -> (String, String) {
    let mut av: Vec<char> = a.chars().collect();
    let mut bv: Vec<char> = b.chars().collect();
    std::mem::swap(&mut av[idx], &mut bv[idx]);
    let new_a: String = av.into_iter().collect();
    let new_b: String = bv.into_iter().collect();
    (new_a, new_b)
}

#[test]
fn test_swap_char() {
    let a = String::from("hoge");
    let b = String::from("Hello");
    let result = swap_char(&a, &b, 2);
    assert_eq!(result, (String::from("hole"), String::from("Heglo")));
}

// Vec<char>からStringに変換
#[snippet("Vec<char> to string")]
fn vec_char_to_string(vec: &Vec<char>) -> String {
    vec.into_iter().collect()
}

#[test]
fn test_vec_char_to_string() {
    let vec = vec!['a', 'b', 'c', 'd'];
    let result = vec_char_to_string(&vec);
    assert_eq!(result, String::from("abcd"));
}

// 文字列の各文字を昇順でソート
#[snippet("sort chars of String")]
fn sort_chars_of_string(s: &String) -> Vec<char> {
    s.chars().sorted().collect_vec()
}

#[test]
fn test_sort_chars_of_string() {
    let s = String::from("dfcbae");
    let result = sort_chars_of_string(&s);
    assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
}
