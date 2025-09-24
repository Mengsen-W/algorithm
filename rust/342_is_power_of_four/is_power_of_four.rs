/*
 * @Date: 2021-05-31 09:05:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-31 09:12:40
 */
struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n % 3 == 1
    }
}

fn main() {
    let tests = vec![(16, true), (5, false), (1, true)];

    for (n, expected) in tests {
        assert_eq!(Solution::is_power_of_four(n), expected);
    }
}
