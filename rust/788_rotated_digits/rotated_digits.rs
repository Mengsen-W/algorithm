/*
 * @Date: 2022-09-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-25
 * @FilePath: /algorithm/788_rotated_digits/rotated_digits.rs
 */

pub fn rotated_digits(n: i32) -> i32 {
    let mut d: Vec<i32> = vec![0, 0, 1, -1, -1, 1, 1, -1, 0, 1];
    d.extend(vec![0; (n - 9).max(0) as usize]);
    let mut ans = 0;
    for i in 0..=n as usize {
        d[i] = if d[i / 10] == -1 || d[i % 10] == -1 {
            -1
        } else {
            d[i / 10] | d[i % 10]
        };
        ans += if d[i] == 1 { 1 } else { 0 };
    }
    ans
}

fn main() {
    assert_eq!(rotated_digits(10), 4);
}
