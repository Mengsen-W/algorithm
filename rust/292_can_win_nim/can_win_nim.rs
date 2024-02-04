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
    let tests = vec![(4, false), (1, true), (2, true)];

    for (n, ans) in tests {
        assert_eq!(Solution::can_win_nim(n), ans);
    }
}
