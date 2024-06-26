/*
 * @Date: 2022-07-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-06
 * @FilePath: /algorithm/rust/741_cherry_pickup/cherry_pickup.rs
 */

struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![i32::MIN; n]; n];
        dp[0][0] = grid[0][0];

        for k in 1..2 * n - 1 {
            let mut tmp = vec![vec![i32::MIN; n]; n];
            let (v, u) = if n - 1 < k {
                (k + 1 - n, n - 1)
            } else {
                (0, k)
            };
            for i in v..=u {
                for j in v..=u {
                    if grid[i][k - i] == -1 || grid[j][k - j] == -1 {
                        continue;
                    }
                    let mut val = grid[i][k - i];
                    if i != j {
                        val += grid[j][k - j];
                    }
                    for x in i as i32 - 1..=i as i32 {
                        for y in j as i32 - 1..=j as i32 {
                            if x >= 0 && y >= 0 {
                                tmp[i][j] = (tmp[i][j]).max(dp[x as usize][y as usize] + val);
                            }
                        }
                    }
                }
            }
            dp = tmp;
        }

        0.max(dp[n - 1][n - 1])
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]], 5),
        (vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]], 0),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::cherry_pickup(grid), ans);
    }
}
