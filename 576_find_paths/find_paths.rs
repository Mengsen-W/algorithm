/*
 * @Date: 2021-08-15 14:20:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-15 14:59:34
 */

struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = 1000000000 + 7;
        let directions: Vec<Vec<i32>> = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
        let mut out_counts = 0;
        let mut dp = vec![vec![0; n as usize]; m as usize];
        dp[start_row as usize][start_column as usize] = 1;
        for _ in 0..max_move {
            let mut dp_new: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
            for j in 0..m {
                for k in 0..n {
                    let count = dp[j as usize][k as usize];
                    if count > 0 {
                        for direction in &directions {
                            let j1 = j + direction[0];
                            let k1 = k + direction[1];
                            if j1 >= 0 && j1 < m && k1 >= 0 && k1 < n {
                                let (j1, k1) = (j1 as usize, k1 as usize);
                                dp_new[j1][k1] = (dp_new[j1][k1] + count) % MOD;
                            } else {
                                out_counts = (out_counts + count) % MOD;
                            }
                        }
                    }
                }
            }
            dp = dp_new;
        }
        out_counts
    }
}

fn main() {
    {
        let m = 2;
        let n = 2;
        let max_move = 2;
        let start_row = 0;
        let start_column = 0;
        let ans = 6;
        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            ans
        );
    }
    {
        let m = 1;
        let n = 3;
        let max_move = 3;
        let start_row = 0;
        let start_column = 1;
        let ans = 12;
        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            ans
        );
    }
}
