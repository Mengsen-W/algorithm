/*
 * @Date: 2024-03-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-23
 * @FilePath: /algorithm/rust/2549_distinct_integers/distinct_integers.rs
 */

struct Solution;
impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        std::cmp::max(1, n - 1)
    }
}

fn main() {
    let tests = vec![(5, 4), (3, 2)];

    for (n, ans) in tests {
        assert_eq!(Solution::distinct_integers(n), ans);
    }
}
