/*
 * @Date: 2023-08-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-24
 * @FilePath: /algorithm/rust/1267_count_servers/count_servers.rs
 */

struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let (m, n) = (grid.len(), grid[0].len());
        let (mut rows, mut cols) = (HashMap::new(), HashMap::new());

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    *rows.entry(i).or_insert(0) += 1;
                    *cols.entry(j).or_insert(0) += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && (*rows.get(&i).unwrap() > 1 || *cols.get(&j).unwrap() > 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 0], vec![0, 1]], 0),
        (vec![vec![1, 0], vec![1, 1]], 3),
        (
            vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
            ],
            4,
        ),
    ];
    for (grid, ans) in tests {
        assert_eq!(Solution::count_servers(grid), ans);
    }
}
