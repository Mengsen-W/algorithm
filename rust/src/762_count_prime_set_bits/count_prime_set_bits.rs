/*
 * @Date: 2022-04-05 10:25:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-05 10:42:50
 * @FilePath: /algorithm/762_count_prime_set_bits/count_prime_set_bits.rs
 */

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    (left..=right).fold(0, |ret, i| ret + (665772 >> i.count_ones() & 1))
}

fn main() {
    assert_eq!(count_prime_set_bits(6, 10), 4);
    assert_eq!(count_prime_set_bits(10, 15), 5);
}
