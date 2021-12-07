/*
 * @Date: 2021-12-07 00:56:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-07 01:39:38
 */

const DX: [isize; 4] = [-1, 0, 1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

struct Solution;

impl Solution {
    pub fn color_border_bfs(
        mut grid: Vec<Vec<i32>>,
        row: i32,
        col: i32,
        color: i32,
    ) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut borders: Vec<(i32, i32)> = Vec::new();
        let original_color = grid[row as usize][col as usize];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((row, col));
        visited[row as usize][col as usize] = true;
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            let mut is_border = false;
            for i in 0..4 {
                let (nx, ny) = (x + DX[i] as i32, y + DY[i] as i32);
                if !(nx >= 0
                    && nx < m as i32
                    && ny >= 0
                    && ny < n as i32
                    && grid[nx as usize][ny as usize] == original_color)
                {
                    is_border = true;
                } else if !visited[nx as usize][ny as usize] {
                    visited[nx as usize][ny as usize] = true;
                    q.push_back((nx, ny));
                }
            }
            if is_border {
                borders.push((x, y));
            }
        }
        for (x, y) in borders {
            grid[x as usize][y as usize] = color;
        }
        grid
    }
    pub fn color_border_dfs(
        mut grid: Vec<Vec<i32>>,
        row: i32,
        col: i32,
        color: i32,
    ) -> Vec<Vec<i32>> {
        let row = row as usize;
        let col = col as usize;
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
        let raw = grid[row][col];
        Self::dfs(&mut vis, &mut grid, row, col, raw, color);
        grid.clone()
    }

    fn dfs(
        vis: &mut Vec<Vec<bool>>,
        grid: &mut Vec<Vec<i32>>,
        row: usize,
        col: usize,
        raw: i32,
        color: i32,
    ) {
        vis[row][col] = true;
        for d in 0..4 {
            let nx = row as isize + DX[d];
            let ny = col as isize + DY[d];
            let n = grid.len() as isize;
            let m = grid[0].len() as isize;
            if nx < 0 || nx >= n || ny < 0 || ny >= m || grid[nx as usize][ny as usize] != raw {
                if nx >= 0 && nx < n && ny >= 0 && ny < m && vis[nx as usize][ny as usize] {
                    continue;
                }
                grid[row][col] = color;
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] == raw && !vis[nx][ny] {
                Self::dfs(vis, grid, nx, ny, raw, color);
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::color_border_bfs(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![vec![3, 3], vec![3, 2]]
    );
    assert_eq!(
        Solution::color_border_dfs(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![vec![3, 3], vec![3, 2]]
    );
    assert_eq!(
        Solution::color_border_bfs(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![vec![1, 3, 3], vec![2, 3, 3]]
    );
    assert_eq!(
        Solution::color_border_dfs(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![vec![1, 3, 3], vec![2, 3, 3]]
    );
    assert_eq!(
        Solution::color_border_bfs(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
    );
    assert_eq!(
        Solution::color_border_dfs(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
    );
}
