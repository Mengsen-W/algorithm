/*
 * @Date: 2023-04-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-21
 * @FilePath: /algorithm/rust/2413_smallest_even_multiple/smallest_even_multiple.rs
 */

pub fn smallest_even_multiple(n: i32) -> i32 {
    n << (n & 1)
}

fn main() {
    assert_eq!(smallest_even_multiple(5), 10);
    assert_eq!(smallest_even_multiple(6), 6);
}
