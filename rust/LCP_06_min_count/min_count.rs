/*
 * @Date: 2023-09-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-20
 * @FilePath: /algorithm/rust/LCP_06_min_count/min_count.rs
 */

struct Solution;
impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.iter().map(|&x| (x + 1) >> 1).sum::<i32>()
    }
}

fn main() {
    let tests = vec![(vec![4, 2, 1], 4), (vec![2, 3, 10], 8)];

    for (coins, ans) in tests {
        assert_eq!(Solution::min_count(coins), ans);
    }
}
