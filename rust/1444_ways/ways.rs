/*
 * @Date: 2023-08-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-17
 * @FilePath: /algorithm/rust/1444_ways/ways.rs
 */

struct Solution;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let pizza: Vec<_> = pizza.into_iter().map(|s| s.into_bytes()).collect();
        let k = k as usize;
        let (m, n) = (pizza.len(), pizza[0].len());
        let mut dp = vec![vec![vec![0; k + 1]; n + 1]; m + 1];
        let mut apples = vec![vec![0; n + 1]; m + 1];
        (0..m).rev().for_each(|i| {
            (0..n).rev().for_each(|j| {
                apples[i][j] = apples[i][j + 1] + apples[i + 1][j] - apples[i + 1][j + 1]
                    + if pizza[i][j] == b'A' { 1 } else { 0 };
                dp[i][j][1] = if apples[i][j] > 0 { 1 } else { 0 };
            })
        });
        (2..=k).for_each(|l| {
            (0..m).for_each(|i| {
                (0..n).for_each(|j| {
                    for ii in i + 1..m {
                        if apples[i][j] > apples[ii][j] {
                            dp[i][j][l] = (dp[i][j][l] + dp[ii][j][l - 1]) % MOD;
                        }
                    }
                    for jj in j + 1..n {
                        if apples[i][j] > apples[i][jj] {
                            dp[i][j][l] = (dp[i][j][l] + dp[i][jj][l - 1]) % MOD;
                        }
                    }
                })
            })
        });
        dp[0][0][k]
    }
}

fn main() {
    let tests = vec![
        (
            vec!["A..", "AAA", "..."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            3,
            3,
        ),
        (
            vec!["A..", "AA.", "..."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            3,
            1,
        ),
        (
            vec!["A..", "A..", "..."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            1,
            1,
        ),
    ];

    for (pizza, k, ans) in tests {
        assert_eq!(Solution::ways(pizza, k), ans);
    }
}
