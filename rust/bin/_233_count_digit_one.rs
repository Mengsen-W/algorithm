/*
 * @Date: 2021-08-13 11:39:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-13 12:48:23
 */

struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n = n as i64;
        let (mut num, mut s, mut i) = (n, 0, 1);
        while num != 0 {
            match num % 10 {
                0 => s = s + (num / 10) * i,
                1 => s = s + (num / 10) * i + (n % i) + 1,
                t if t > 1 => s = s + ((num as f64 / 10.0).ceil() * i as f64) as i64,
                _ => panic!("Match Error"),
            }
            num = num / 10;
            i = i * 10;
        }
        s as i32
    }
}

fn main() {
    assert_eq!(Solution::count_digit_one(13), 6);
    assert_eq!(Solution::count_digit_one(0), 0);
    assert_eq!(Solution::count_digit_one(1000000000), 900000001);
    assert_eq!(Solution::count_digit_one(824883294), 767944060);
}
