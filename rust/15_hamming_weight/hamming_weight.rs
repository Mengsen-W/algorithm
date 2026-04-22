/*
 * @Date: 2021-06-23 08:45:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-23 09:04:43
 */

fn hamming_weight(num: u32) -> u32 {
    num.count_ones()
}

fn main() {
    assert_eq!(hamming_weight(0b_00000000000000000000000000001011), 3);
    assert_eq!(hamming_weight(0b_00000000000000000000000010000000), 1);
    assert_eq!(hamming_weight(0b_11111111111111111111111111111101), 31);
}
