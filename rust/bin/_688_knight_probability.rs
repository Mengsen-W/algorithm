struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let dirs: [[i32; 2]; 8] = [
            [-2, -1],
            [-2, 1],
            [2, -1],
            [2, 1],
            [-1, -2],
            [-1, 2],
            [1, -2],
            [1, 2],
        ];

        let (k, n, row, column) = (k as usize, n as usize, row as usize, column as usize);
        let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; n]; n]; k + 1];
        (0..=k).for_each(|step| {
            (0..n).for_each(|i| {
                (0..n).for_each(|j| {
                    if step == 0 {
                        dp[step][i][j] = 1.0;
                    } else {
                        for dir in &dirs {
                            let (ni, nj) = (i as i32 + dir[0], j as i32 + dir[1]);
                            let n = n as i32;
                            if ni >= 0 && ni < n && nj >= 0 && nj < n {
                                let (ni, nj) = (ni as usize, nj as usize);
                                dp[step][i][j] += dp[step - 1][ni][nj] / 8.0;
                            }
                        }
                    }
                })
            })
        });

        dp[k][row][column]
    }
}

fn main() {
    let tests = vec![(3, 2, 0, 0, 0.0625), (1, 0, 0, 0, 1.0)];

    for (n, k, row, column, ans) in tests {
        assert_eq!(Solution::knight_probability(n, k, row, column), ans);
    }
}
