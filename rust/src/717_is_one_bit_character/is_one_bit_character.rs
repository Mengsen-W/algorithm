/*
 * @Date: 2022-02-20 00:38:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-20 01:15:44
 */

pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let n = bits.len() as i32;
    let mut i = n - 2;
    while i >= 0 && bits[i as usize] == 1 {
        i -= 1;
    }
    (n - i) % 2 == 0
}

fn main() {
    assert_eq!(is_one_bit_character(vec![1, 0, 0]), true);
    assert_eq!(is_one_bit_character(vec![1, 1, 1, 0]), false);
}
