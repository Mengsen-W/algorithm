struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        let mut dp = vec![vec![vec![0i64; k]; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if i == 1 && j == 1 {
                    dp[i][j][(grid[0][0] % k as i32) as usize] = 1;
                    continue;
                }

                let value = (grid[i - 1][j - 1] % k as i32) as usize;
                for r in 0..k {
                    let prev_mod = (r as i32 - value as i32 + k as i32) as usize % k;
                    dp[i][j][r] = (dp[i - 1][j][prev_mod] + dp[i][j - 1][prev_mod]) % MOD;
                }
            }
        }

        dp[m][n][0] as i32
    }
}

fn main() {
    let tests = vec![
        (vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3, 2),
        (vec![vec![0, 0]], 1, 5),
        (
            vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
            1,
            10,
        ),
    ];

    for (grid, k, expected) in tests {
        assert_eq!(Solution::number_of_paths(grid, k), expected);
    }
}
