/*
 * @Date: 2024-05-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-13
 * @FilePath: /algorithm/rust/994_oranges_rotting/oranges_rotting.rs
 */

struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let (R, C) = (grid.len(), grid[0].len());
        const dr: [i32; 4] = [-1, 0, 1, 0];
        const dc: [i32; 4] = [0, -1, 0, 1];
        let mut queue = VecDeque::new();
        let mut depth = HashMap::new();

        for r in 0..R {
            for c in 0..C {
                if grid[r][c] == 2 {
                    let code = r * C + c;
                    queue.push_back(code);
                    depth.insert(code, 0);
                }
            }
        }

        let mut ans = 0;
        while let Some(code) = queue.pop_front() {
            let r = code / C;
            let c = code % C;
            for k in 0..4 {
                let nr = r + dr[k] as usize;
                let nc = c + dc[k] as usize;
                if 0 <= nr && nr < R && 0 <= nc && nc < C && grid[nr][nc] == 1 {
                    grid[nr][nc] = 2;
                    let ncode = nr * C + nc;
                    queue.push_back(ncode);
                    depth.insert(ncode, *depth.get(&code).unwrap() + 1);
                    ans = *depth.get(&ncode).unwrap();
                }
            }
        }

        for row in grid {
            for v in row {
                if v == 1 {
                    return -1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
        (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
        (vec![vec![0, 2]], 0),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::oranges_rotting(grid), ans);
    }
}
