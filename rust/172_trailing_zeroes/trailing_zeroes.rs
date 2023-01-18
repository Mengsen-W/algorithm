/*
 * @Date: 2022-03-24 23:04:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-24 23:28:37
 * @FilePath: /algorithm/172_trailing_zeroes/trailing_zeroes.rs
 */

pub fn trailing_zeroes(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        n /= 5;
        ans += n;
    }
    ans
}

fn main() {
    assert_eq!(trailing_zeroes(3), 0);
    assert_eq!(trailing_zeroes(5), 1);
    assert_eq!(trailing_zeroes(0), 0);
}
