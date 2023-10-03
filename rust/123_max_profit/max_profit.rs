/*
 * @Date: 2023-10-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-03
 * @FilePath: /algorithm/rust/123_max_profit/max_profit.rs
 */

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy0, mut sell0) = (-prices[0], 0);
        let (mut buy1, mut sell1) = (-prices[0], 0);
        prices.iter().skip(1).for_each(|&v| {
            buy0 = buy0.max(-v);
            sell0 = sell0.max(v + buy0);
            buy1 = buy1.max(sell0 - v);
            sell1 = sell1.max(v + buy1);
        });
        sell1
    }
}

fn main() {
    let tests = vec![
        (vec![3, 3, 5, 0, 0, 3, 1, 4], 6),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![1], 0),
    ];

    for (prices, ans) in tests {
        assert_eq!(Solution::max_profit(prices), ans);
    }
}
