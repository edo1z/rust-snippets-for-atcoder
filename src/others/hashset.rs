#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;
use std::collections::HashSet;

// 重複文字列を削除
#[snippet("unique_strings")]
fn vec_string_to_hashset(words: &Vec<String>) -> HashSet<String> {
    words.clone().into_iter().collect()
}

// 文字列の重複文字を削除
#[snippet("unique_chars")]
fn string_to_hashset_char(s: &String) -> HashSet<char> {
    let vec: Vec<char> = s.clone().chars().collect();
    vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_string_to_hashset() {
        let vec = vec![
            String::from("hoge"),
            String::from("abc"),
            String::from("hoge"),
        ];
        let result = vec_string_to_hashset(&vec);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_string_to_hashset_char() {
        let result = string_to_hashset_char(&String::from("hogehogehogehoga"));
        let ans: HashSet<char> = vec!['h', 'o', 'g', 'e', 'a'].into_iter().collect();
        assert_eq!(result, ans);
    }
}
