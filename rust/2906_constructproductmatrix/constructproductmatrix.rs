struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;
        let n = grid.len();
        let m = grid[0].len();
        let mut p = vec![vec![0; m]; n];

        let mut suffix: i64 = 1;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                p[i][j] = (suffix % MOD) as i32;
                suffix = (suffix * grid[i][j] as i64) % MOD;
            }
        }

        let mut prefix: i64 = 1;
        for i in 0..n {
            for j in 0..m {
                p[i][j] = ((p[i][j] as i64 * prefix) % MOD) as i32;
                prefix = (prefix * grid[i][j] as i64) % MOD;
            }
        }

        p
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2], vec![3, 4]], vec![vec![24, 12], vec![8, 6]]),
        (
            vec![vec![12345], vec![2], vec![1]],
            vec![vec![2], vec![0], vec![0]],
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::construct_product_matrix(grid), ans);
    }
}
