/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-29 18:50:13
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-29 18:52:13
 */

use std::cmp;

fn max_profit_k_1(prices: &Vec<i32>) {
    {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = i32::MIN.max(-prices[0]);
        for i in 1..prices.len() {
            dp[i][0] = cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
            dp[i][1] = cmp::max(dp[i - 1][1], -prices[i]);
        }
        println!("{}", dp[prices.len() - 1][0])
    }
    {
        let mut dp_i_0 = 0;
        let mut dp_i_1 = i32::MIN;
        for i in 0..prices.len() {
            dp_i_0 = dp_i_0.max(dp_i_1 + prices[i]);
            dp_i_1 = dp_i_1.max(-prices[i]);
        }
        println!("{}", dp_i_0);
    }
}

fn max_profit_k_inf(prices: &Vec<i32>) {
    {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = i32::MIN.max(dp[0][0] - prices[0]);
        for i in 1..prices.len() {
            dp[i][0] = cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
            dp[i][1] = cmp::max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
        }
        println!("{}", dp[prices.len() - 1][0])
    }
    {
        let mut dp_i_0 = 0;
        let mut dp_i_1 = i32::MIN;
        let mut temp;
        for i in 0..prices.len() {
            temp = dp_i_0;
            dp_i_0 = dp_i_0.max(dp_i_1 + prices[i]);
            dp_i_1 = dp_i_1.max(temp - prices[i]);
        }
        println!("{}", dp_i_0);
    }
}

fn max_profit_with_cool(prices: &Vec<i32>) {
    {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = i32::MIN.max(dp[0][0] - prices[0]);
        dp[1][0] = dp[0][0].max(dp[0][1] + prices[1]);
        dp[1][1] = dp[0][1].max(0 - prices[1]);
        for i in 2..prices.len() {
            dp[i][0] = cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
            dp[i][1] = cmp::max(dp[i - 1][1], dp[i - 2][0] - prices[i]);
        }
        println!("{}", dp[prices.len() - 1][0])
    }
    {
        let mut dp_i_0 = 0;
        let mut dp_i_1 = i32::MIN;
        let mut temp;
        let mut dp_pre_0 = 0;
        for i in 0..prices.len() {
            temp = dp_i_0;
            dp_i_0 = dp_i_0.max(dp_i_1 + prices[i]);
            dp_i_1 = dp_i_1.max(dp_pre_0 - prices[i]);
            dp_pre_0 = temp;
        }
        println!("{}", dp_i_0);
    }
}

fn max_profit_k_inf_with_fee(prices: &Vec<i32>, fee: &i32) {
    {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = i32::MIN.max(dp[0][0] - prices[0] - fee);
        for i in 1..prices.len() {
            dp[i][0] = cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
            dp[i][1] = cmp::max(dp[i - 1][1], dp[i - 1][0] - prices[i] - fee);
        }
        println!("{}", dp[prices.len() - 1][0])
    }
    {
        let mut dp_i_0 = 0;
        let mut dp_i_1 = i32::MIN;
        let mut temp;
        for i in 0..prices.len() {
            temp = dp_i_0;
            dp_i_0 = dp_i_0.max(dp_i_1 + prices[i]);
            dp_i_1 = dp_i_1.max(temp - prices[i] - fee);
        }
        println!("{}", dp_i_0);
    }
}

fn main() {
    let mut prices: Vec<i32>;
    {
        prices = vec![7, 1, 5, 3, 6, 4];
        max_profit_k_1(&prices);
        prices = vec![7, 6, 4, 3, 1];
        max_profit_k_1(&prices);
    }
    {
        prices = vec![7, 1, 5, 3, 6, 4];
        max_profit_k_inf(&prices);
        prices = vec![1, 2, 3, 4, 5];
        max_profit_k_inf(&prices);
    }
    {
        prices = vec![1, 2, 3, 0, 2];
        max_profit_with_cool(&prices);
    }
    {
        prices = vec![1, 3, 2, 8, 4, 9];
        let fee = 2;
        max_profit_k_inf_with_fee(&prices, &fee);
    }
}
