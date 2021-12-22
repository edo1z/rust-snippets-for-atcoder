#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use std::io::{self, BufRead, Error, Read, Write};
use std::str::FromStr;

#[snippet("get_input-標準入力を1行読み込む")]
fn get_input<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}
#[test]
fn test_get_input() {
    // let s: Vec<String> = get_input();
    // assert_eq!(s, vec!["ho".to_string(), "ge".to_string()]);
}

#[snippet("get_input-標準入力を複数行読み込む")]
#[snippet(include = "get_input-標準入力を1行読み込む")]
fn get_input_lines<T: FromStr>(line_len: usize) -> Vec<Vec<T>> {
    let mut vec: Vec<Vec<T>> = vec![];
    for _ in 0..line_len {
        let v: Vec<T> = get_input();
        vec.push(v);
    }
    vec
}

#[test]
fn test_get_input_lines() {
    // let s: Vec<Vec<String>> = get_input_lines(2);
    // assert_eq!(s, vec![vec!["ho".to_string(), "ge".to_string()], vec!["ab".to_string(), "cd".to_string()]]);
    // let s: Vec<Vec<usize>> = get_input_lines(2);
    // assert_eq!(s, vec![vec![1, 2], vec![3, 4]]);
}
