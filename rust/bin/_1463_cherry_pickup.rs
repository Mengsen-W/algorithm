/*
 * @Date: 2024-05-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-07
 * @FilePath: /algorithm/rust/1463_cherry_pickup/cherry_pickup.rs
 */

struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![-1; n]; n];
        let mut g = vec![vec![-1; n]; n];

        f[0][n - 1] = grid[0][0] + grid[0][n - 1];
        for i in 1..m {
            for j1 in 0..n {
                for j2 in 0..n {
                    let mut best = -1;
                    for dj1 in -1..=1 {
                        for dj2 in -1..=1 {
                            let dj1 = j1 as i32 + dj1;
                            let dj2 = j2 as i32 + dj2;
                            if dj1 >= 0
                                && dj1 < n as i32
                                && dj2 >= 0
                                && dj2 < n as i32
                                && f[dj1 as usize][dj2 as usize] != -1
                            {
                                best = best.max(
                                    f[dj1 as usize][dj2 as usize]
                                        + if j1 == j2 {
                                            grid[i][j1]
                                        } else {
                                            grid[i][j1] + grid[i][j2]
                                        },
                                );
                            }
                        }
                    }
                    g[j1][j2] = best;
                }
            }
            std::mem::swap(&mut f, &mut g);
        }

        let mut ans = 0;
        for j1 in 0..n {
            ans = ans.max(*f[j1].iter().max().unwrap_or(&0));
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]],
            24,
        ),
        (
            vec![
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 3, 0],
                vec![2, 0, 9, 0, 0, 0, 0],
                vec![0, 3, 0, 5, 4, 0, 0],
                vec![1, 0, 2, 3, 0, 0, 6],
            ],
            28,
        ),
        (
            vec![
                vec![1, 0, 0, 3],
                vec![0, 0, 0, 3],
                vec![0, 0, 3, 3],
                vec![9, 0, 3, 3],
            ],
            22,
        ),
        (vec![vec![1, 1], vec![1, 1]], 4),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::cherry_pickup(grid), ans);
    }
}
