/*
 * @Date: 2021-06-09 08:34:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-09 09:10:09
 */

fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = n as usize;
    let min_profit = min_profit as usize;
    let mut dp = vec![vec![0; min_profit as usize + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }
    let len = group.len();
    const MOD: i32 = 1e9 as i32 + 7;
    for i in 1..=len {
        let members = group[i - 1];
        let earn = profit[i - 1];
        for j in (members as usize..=n).rev() {
            for k in (0..=min_profit).rev() {
                dp[j][k] =
                    (dp[j][k] + dp[j - members as usize][(0.max(k as i32 - earn)) as usize]) % MOD;
            }
        }
    }
    dp[n][min_profit]
}

fn main() {
    {
        let n = 5;
        let min_profit = 3;
        let group = vec![2, 2];
        let profit = vec![2, 3];
        let ans = 2;
        assert_eq!(profitable_schemes(n, min_profit, group, profit), ans);
    }
    {
        let n = 10;
        let min_profit = 5;
        let group = vec![2, 3, 5];
        let profit = vec![6, 7, 8];
        let ans = 7;
        assert_eq!(profitable_schemes(n, min_profit, group, profit), ans);
    }
}
