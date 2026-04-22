struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let n = coins[0].len();
        let mut dp = vec![[i32::MIN / 2; 3]; n + 1];

        dp[1] = [0, 0, 0];
        for row in coins {
            for j in 1..=n {
                let x = row[j - 1];
                dp[j][2] = *[dp[j - 1][2] + x, dp[j][2] + x, dp[j - 1][1], dp[j][1]]
                    .iter()
                    .max()
                    .unwrap();
                dp[j][1] = *[dp[j - 1][1] + x, dp[j][1] + x, dp[j - 1][0], dp[j][0]]
                    .iter()
                    .max()
                    .unwrap();
                dp[j][0] = dp[j - 1][0].max(dp[j][0]) + x;
            }
        }

        dp[n][2]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]], 8),
        (vec![vec![10, 10, 10], vec![10, 10, 10]], 40),
    ];

    for (coins, expected) in tests {
        assert_eq!(Solution::maximum_amount(coins), expected);
    }
}
