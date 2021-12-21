#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

#[snippet("find_index")]
fn find_index<T: PartialEq + Copy>(vec: &Vec<T>, search_target: T) -> usize {
    vec.iter().position(|&x| x == search_target).unwrap()
}

#[snippet("a-z")]
fn a_z_vec_char() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
}

#[test]
fn test_find_index() {
    let vec = a_z_vec_char();
    let target = 'x';
    let result = find_index(&vec, target);
    assert_eq!(result, 23);

    let vec_num = vec![1,2,3,4,5];
    let result_num = find_index(&vec_num, 3);
    assert_eq!(result_num, 2);
}
