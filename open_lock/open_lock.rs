/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-24 22:49:19
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-24 22:51:16
 */

// fn open_lock(deadends: Vec<String>, target: String) -> i32 {}

// fn open_lock_binary(deadends: Vec<String>, target: String) -> i32 {}

fn main() {
    let mut s: String = "Hello Rust".to_string();
    let mut b = s.into_bytes();
    b[1] = b'Q';
    s = String::from_utf8(b).unwrap();
    println!("{}", s);
}
