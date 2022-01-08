/*
 * @Date: 2022-01-08 01:10:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-08 01:21:31
 */

pub fn gray_code(n: i32) -> Vec<i32> {
    (0..(1 << n)).map(|x| x ^ x >> 1).collect()
}

fn main() {
    assert_eq!(gray_code(2), vec![0, 1, 3, 2]);
    assert_eq!(gray_code(1), vec![0, 1]);
}
