/*
 * @Date: 2023-10-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-06
 * @FilePath: /algorithm/rust/714_max_profit/max_profit.rs
 */

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = i32::MIN / 2;
        for p in &prices {
            let new_f0 = f0.max(f1 + p - fee);
            f1 = f1.max(f0 - p);
            f0 = new_f0;
        }
        f0
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2, 8, 4, 9], 2, 8),
        (vec![1, 3, 7, 5, 10, 3], 3, 6),
    ];

    for (prices, fee, ans) in tests {
        assert_eq!(Solution::max_profit(prices, fee), ans);
    }
}
