/*
 * @Date: 2022-10-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-25
 * @FilePath: /algorithm/934_shortest_bridge/shortest_bridge.rs
 */

struct Solution;

use std::collections::VecDeque;
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut queue = VecDeque::new();
        let mut found = false;

        for i in 0..n {
            if found {
                break;
            }

            for j in 0..m {
                if grid[i][j] == 1 {
                    Self::dfs(&mut grid, &mut queue, i, j, n, m);
                    found = true;
                    break;
                }
            }
        }

        while let Some((i, j, d)) = queue.pop_front() {
            match grid[i][j] {
                1 => {
                    return d - 1;
                }
                0 | 2 => {
                    grid[i][j] = 3;

                    for &(dx, dy) in &DIRS {
                        let x = (i as i32 + dx) as usize;
                        let y = (j as i32 + dy) as usize;

                        if x < n && y < m {
                            queue.push_back((x, y, d + 1));
                        }
                    }
                }
                _ => {}
            }
        }

        0
    }

    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        queue: &mut VecDeque<(usize, usize, i32)>,
        i: usize,
        j: usize,
        n: usize,
        m: usize,
    ) {
        if grid[i][j] == 1 {
            grid[i][j] = 2;
            queue.push_back((i, j, 0));

            for &(dx, dy) in &DIRS {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;

                if x < n && y < m {
                    Self::dfs(grid, queue, x, y, n, m);
                }
            }
        }
    }
}

fn main() {
    {
        let grid = [[0, 1], [1, 0]].iter().map(|s| s.to_vec()).collect();
        let ans = 1;
        assert_eq!(Solution::shortest_bridge(grid), ans);
    }

    {
        let grid = [[0, 1, 0], [0, 0, 0], [0, 0, 1]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let ans = 2;
        assert_eq!(Solution::shortest_bridge(grid), ans);
    }

    {
        let grid = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ]
        .iter()
        .map(|s| s.to_vec())
        .collect();
        let ans = 1;
        assert_eq!(Solution::shortest_bridge(grid), ans);
    }
}
