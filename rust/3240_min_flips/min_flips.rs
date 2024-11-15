struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![i32::MAX / 2; 4];
        f[0] = 0;
        for i in 0..(m + 1) / 2 {
            for j in 0..(n + 1) / 2 {
                let mut ones = grid[i][j];
                let mut cnt = 1;
                if j != n - 1 - j {
                    ones += grid[i][n - 1 - j];
                    cnt += 1;
                }
                if i != m - 1 - i {
                    ones += grid[m - 1 - i][j];
                    cnt += 1;
                }
                if i != m - 1 - i && j != n - 1 - j {
                    ones += grid[m - 1 - i][n - 1 - j];
                    cnt += 1;
                }
                // 计算将这一组全部变为 1 的代价
                let cnt1 = cnt - ones;
                // 计算将这一组全部变为 0 的代价
                let cnt0 = ones;
                let mut tmp = vec![0; 4];
                for k in 0..4 {
                    tmp[k] = f[k] + cnt0;
                }
                for k in 0..4 {
                    tmp[(k + cnt) as usize % 4] =
                        tmp[(k + cnt) as usize % 4].min(f[k as usize] + cnt1);
                }
                f = tmp;
            }
        }
        f[0]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
        (vec![vec![0, 1], vec![0, 1], vec![0, 0]], 2),
        (vec![vec![1], vec![1]], 2),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::min_flips(grid), ans);
    }
}
