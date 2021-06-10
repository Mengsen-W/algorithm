/*
 * @Date: 2021-06-10 09:11:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-10 09:30:38
 */

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

fn main() {
    {
        let amount = 5;
        let coins = vec![1, 2, 5];
        let ans = 4;
        assert_eq!(change(amount, coins), ans);
    }
    {
        let amount = 3;
        let coins = vec![2];
        let ans = 0;
        assert_eq!(change(amount, coins), ans);
    }
    {
        let amount = 10;
        let coins = vec![10];
        let ans = 1;
        assert_eq!(change(amount, coins), ans);
    }
}
