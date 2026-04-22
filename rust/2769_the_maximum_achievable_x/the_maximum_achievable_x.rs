/*
 * @Date: 2024-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-21
 * @FilePath: /algorithm/rust/2769_the_maximum_achievable_x/the_maximum_achievable_x.rs
 */

struct Solution;

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + 2 * t
    }
}

fn main() {
    let tests = vec![(4, 1, 6), (3, 2, 7)];

    for (num, t, ans) in tests {
        assert_eq!(Solution::the_maximum_achievable_x(num, t), ans);
    }
}
