/*
 * @Date: 2021-11-04 00:52:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-04 01:00:33
 */

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut x0 = num as f64;
        loop {
            let x1 = (x0 + num as f64 / x0) / 2.0;
            if x1 == x0 {
                break;
            }
            x0 = x1;
        }
        let x = x0 as i32;
        return x * x == num;
    }
}

fn main() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
}
