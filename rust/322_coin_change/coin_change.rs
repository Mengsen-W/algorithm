/*
 * @Date: 2024-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-24
 * @FilePath: /algorithm/rust/322_coin_change/coin_change.rs
 */

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut dp = vec![max; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for j in 0..coins.len() {
                if coins[j] <= i {
                    dp[i as usize] = dp[i as usize].min(dp[(i - coins[j]) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 5], 11, 3), (vec![2], 3, -1), (vec![1], 0, 0)];

    for (coins, amount, ans) in tests {
        assert_eq!(Solution::coin_change(coins, amount), ans);
    }
}
