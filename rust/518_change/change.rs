/*
 * @Date: 2021-06-10 09:11:51
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-25
 */

struct Solution;

impl Solution {
    fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for coin in coins {
            for i in coin..=amount {
                dp[i as usize] += dp[(i - coin) as usize];
            }
        }
        dp[amount as usize]
    }
}

fn main() {
    let tests = vec![(5, vec![1, 2, 5], 4), (3, vec![2], 0), (10, vec![10], 1)];

    for (amount, coins, ans) in tests {
        assert_eq!(Solution::change(amount, coins), ans);
    }
}
