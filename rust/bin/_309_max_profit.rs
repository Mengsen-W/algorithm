/*
 * @Date: 2023-10-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-05
 * @FilePath: /algorithm/rust/309_max_profit/max_profit.rs
 */

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut f = vec![[0, 0]; prices.len() + 2];
        f[0][1] = i32::MIN;
        f[1][1] = i32::MIN;
        for (index, price) in prices.into_iter().enumerate() {
            f[index + 2][1] = std::cmp::max(f[index + 1][1], f[index][0] - price);
            f[index + 2][0] = std::cmp::max(f[index + 1][0], f[index + 1][1] + price);
        }
        f.last().unwrap()[0]
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 0, 2], 3), (vec![1], 0)];
    for (prices, ans) in tests {
        assert_eq!(Solution::max_profit(prices), ans);
    }
}
