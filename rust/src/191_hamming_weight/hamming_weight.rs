/*
 * @Date: 2021-03-22 08:14:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-22 08:29:24
 */

fn hamming_weight(n: u32) -> i32 {
    match n {
        n if n > 0 => 1 + hamming_weight(n & (n - 1)),
        _ => 0,
    }
}

fn main() {
    assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
    assert_eq!(hamming_weight(0b00000000000000000000000010000000), 1);
    assert_eq!(hamming_weight(0b11111111111111111111111111111101), 31);
}
