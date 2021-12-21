#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;
use itertools::Itertools;

// 型名の表示
#[snippet("typename")]
fn typename<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[test]
fn test_typename() {
    assert_eq!(typename(32 as u32), "u32");
    assert_eq!(typename(String::from("hoge")), "alloc::string::String");
}

// 実行時間の計測
#[snippet("exec_time")]
use std::time::Instant;
fn exec_time_start() -> Instant { Instant::now() }
fn exec_time_end(start:Instant) {
    let end = start.elapsed();
    let sec = end.as_secs();
    let subsec = end.subsec_nanos() / 1_000_000;
    println!("\ntime: {}.{:09}", sec, subsec);
}