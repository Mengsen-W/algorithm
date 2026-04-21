/*
 * @Date: 2023-10-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-02
 * @FilePath: /algorithm/rust/122_max_profit/max_profit.rs
 */

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        (1..prices.len()).fold(0, |acc, i| {
            acc + std::cmp::max(0, prices[i] - prices[i - 1])
        })
    }
}

fn main() {
    let tests = vec![
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ];

    for (prices, ans) in tests {
        assert_eq!(Solution::max_profit(prices), ans);
    }
}
