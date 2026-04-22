/*
 * @Date: 2023-10-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-01
 * @FilePath: /algorithm/rust/121_max_profit/max_profit.rs
 */

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min_price = prices[0];
        for &p in &prices {
            ans = ans.max(p - min_price);
            min_price = min_price.min(p);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];
    for (prices, ans) in tests {
        assert_eq!(Solution::max_profit(prices), ans);
    }
}
