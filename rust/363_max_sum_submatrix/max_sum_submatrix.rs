/*
 * @Date: 2021-04-22 09:52:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-22 10:12:50
 */

fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp = matrix.clone();
    let mut res = i32::MIN;

    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            if j != 0 {
                dp[i][j] += dp[i][j - 1];
            }
            if i != 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if i != 0 && j != 0 {
                dp[i][j] -= dp[i - 1][j - 1];
            }
        }
    }

    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            let mut temp = dp[i][j];
            if temp <= k && temp > res {
                res = temp;
            }
            for m in 0..i {
                for n in 0..j {
                    temp = dp[i][j] - dp[m][j] - dp[i][n] + dp[m][n];
                    if temp <= k && temp > res {
                        res = temp;
                    }
                }
            }

            for m in 0..i {
                temp = dp[i][j] - dp[m][j];
                if temp <= k && temp > res {
                    res = temp;
                }
            }

            for n in 0..j {
                temp = dp[i][j] - dp[i][n];
                if temp <= k && temp > res {
                    res = temp;
                }
            }
        }
    }

    res
}

fn main() {
    {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        assert_eq!(max_sum_submatrix(matrix, 2), 2);
    }
    {
        let matrix = vec![vec![2, 2, -1]];
        assert_eq!(max_sum_submatrix(matrix, 3), 3);
    }
}
