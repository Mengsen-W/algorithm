/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 09:49:08
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-19 16:55:57
 */

fn bottom_up(coins: &Vec<i32>, amount: &i32) {
    use std::collections::HashMap;
    let mut dict: HashMap<i32, i32> = HashMap::new();
    fn dp(n: i32) -> i32 {
        if dict.contains_key(n) {
            return dict[n];
        }

        if n == 0 {
            return 0;
        }
        if n < 0 {
            return -1;
        }

        let res: i32 = i32::MAX;

        for coin in coins {
            let subproblem = dp(n - coin);
            if subproblem == -1 {
                continue;
            }
            res = cmp::min(res, 1 + subproblem);
        }
        dict[n] = res;
        return dict[n];
    }

    dp(amount);
}

fn top_down(coins: &Vec<i32>, amount: &i32) -> i32 {
    let mut dp: Vec<i32> = vec![amount + 1; amount + 1];
    dp[0] = 0;

    for i in 0..coins.len() {
        for coin in coins {
            if (i - coin) < 0 {
                continue;
            }
            dp[i] = cmp::min(dp[i], dp[i - coin] + 1);
        }
    }
    if dp[amount] == amount + 1 {
        return -1;
    } else {
        return dp[amount];
    };
}

fn main() {}
