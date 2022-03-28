/*
 * @Date: 2022-03-28 14:59:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-28 15:05:42
 * @FilePath: /algorithm/693_has_alternating_bits/has_alternating_bits.rs
 */

pub fn has_alternating_bits(mut n: i32) -> bool {
    n ^= n >> 1;
    (n & n + 1) == 0
}

fn main() {
    assert_eq!(has_alternating_bits(5), true);
    assert_eq!(has_alternating_bits(7), false);
    assert_eq!(has_alternating_bits(11), false);
}
