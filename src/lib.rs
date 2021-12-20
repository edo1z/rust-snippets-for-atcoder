#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

#[snippet("hoge")]
fn hoge() -> u16 {
    16
}

#[test]
fn test_hoge() {
    assert_eq!(16, hoge());
}
