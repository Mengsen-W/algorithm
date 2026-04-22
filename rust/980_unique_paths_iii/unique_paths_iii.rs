/*
 * @Date: 2023-08-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-04
 * @FilePath: /algorithm/rust/980_unique_paths_iii/unique_paths_iii.rs
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut one_d_grid = Vec::with_capacity(m * n);
        for i in 0..m {
            for j in 0..n {
                one_d_grid.push(grid[i][j] as i8);
            }
        }

        let mut s = Solver {
            memo: HashMap::new(),
            v: one_d_grid,
            width: n,
        };
        s.solve()
    }
}

struct Solver {
    memo: HashMap<(usize, Vec<i8>), i32>,
    v: Vec<i8>,
    width: usize,
}

impl Solver {
    fn solve(&mut self) -> i32 {
        for i in 0..self.v.len() {
            if self.v[i] == 1 {
                return self.backtracking(i / self.width, i % self.width);
            }
        }
        0
    }

    fn encode(&self, r: usize, c: usize) -> usize {
        r * self.width + c
    }

    fn backtracking(&mut self, r: usize, c: usize) -> i32 {
        let idx = self.encode(r, c);
        if let Some(x) = self.memo.get(&(idx, self.v.clone())) {
            return *x;
        }
        // println!("{:?}", self.v);
        if self.v[idx] == 2 {
            return self.v.iter().all(|x| *x != 0) as i32;
        }

        let old = self.v[idx];
        self.v[idx] = -1;

        let (m, n) = (self.v.len() / self.width, self.width);
        let mut ans = 0;
        for &(dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nc < 0 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let ni = self.encode(nr, nc);
            if nr < m && nc < n && self.v[ni] != -1 {
                ans += self.backtracking(nr, nc);
            }
        }

        self.v[idx] = old;
        self.memo.insert((idx, self.v.clone()), ans);
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]],
            2,
        ),
        (
            vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]],
            4,
        ),
        (vec![vec![0, 1], vec![2, 0]], 0),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::unique_paths_iii(grid), ans);
    }
}
