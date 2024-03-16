/*
 * @Date: 2024-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-16
 * @FilePath: /algorithm/rust/2684_max_moves/max_moves.rs
 */

struct Solution;


impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
use std::collections::HashSet;
        let m = grid.len();
        let n = grid[0].len();
        let mut q: HashSet<usize> = HashSet::new();
        for i in 0..m {
            q.insert(i);
        }
        for j in 1..n {
            let mut q2: HashSet<usize> = HashSet::new();
            for &i in q.iter() {
                for i2 in i.saturating_sub(1)..=i + 1 {
                    if i2 < m && grid[i][j - 1] < grid[i2][j] {
                        q2.insert(i2);
                    }
                }
            }
            q = q2;
            if q.is_empty() {
                return (j - 1) as i32;
            }
        }
        (n - 1) as i32
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15],
            ],
            3,
        ),
        (vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]], 0),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::max_moves(grid), ans);
    }
}
