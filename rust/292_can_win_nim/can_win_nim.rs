/*
 * @Date: 2021-09-18 08:42:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-18 08:49:16
 */

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

fn main() {
    assert!(!Solution::can_win_nim(4));
    assert!(Solution::can_win_nim(1));
    assert!(Solution::can_win_nim(2));
}
