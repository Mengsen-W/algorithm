struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();

        // f[j][0]：通过向下移动进入当前列
        // f[j][1]：通过向右移动进入当前列
        let mut f = vec![[0i64, 0i64]; n];
        f[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    if grid[i - 1][j] == 0 {
                        f[j][0] = (f[j][0] + f[j][1]) % MOD;
                    } else {
                        f[j][0] = f[j][1];
                    }
                }

                if j > 0 {
                    if grid[i][j - 1] == 0 {
                        f[j][1] = (f[j - 1][0] + f[j - 1][1]) % MOD;
                    } else {
                        f[j][1] = f[j - 1][0];
                    }
                }
            }
        }

        ((f[n - 1][0] + f[n - 1][1]) % MOD) as i32
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]], 5),
        (vec![vec![0, 0], vec![0, 0]], 2),
        (vec![vec![0, 1, 1], vec![1, 1, 0]], 1),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::unique_paths(grid), ans);
    }
}
