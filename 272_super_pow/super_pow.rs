/*
 * @Date: 2021-12-05 07:19:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-05 07:29:49
 */
pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
    let b = b.into_iter().fold(0, |s, bi| (s * 10 + bi) % 1140);
    (0..b).fold(1, |s, _| s * a as i64 % 1337) as i32
}

fn main() {
    assert_eq!(super_pow(2, vec![3]), 8);
    assert_eq!(super_pow(2, vec![1, 0]), 1024);
    assert_eq!(super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    assert_eq!(super_pow(2147483647, vec![2, 0, 0]), 1198);
}
