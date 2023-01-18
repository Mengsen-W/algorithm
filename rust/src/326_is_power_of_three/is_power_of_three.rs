/*
 * @Date: 2021-09-23 08:51:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-23 09:06:06
 */

struct Solution;

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n > 0 && n % 3 == 0 {
            n /= 3
        }
        n == 1
    }
}

fn main() {
    assert!(Solution::is_power_of_three(27));
    assert!(!Solution::is_power_of_three(0));
    assert!(Solution::is_power_of_three(9));
    assert!(!Solution::is_power_of_three(45));
}
