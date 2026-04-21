/*
 * @Date: 2022-02-12 00:02:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-12 03:55:20
 */

struct Solution;

impl Solution {
    pub fn num_enclaves_dfs(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        fn dfs(
            grid: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            m: &usize,
            n: &usize,
            row: &i32,
            col: &i32,
        ) {
            if row < &0
                || *row >= *m as i32
                || col < &0
                || *col >= (*n as i32)
                || grid[*row as usize][*col as usize] == 0
                || visited[*row as usize][*col as usize]
            {
                return;
            }
            visited[*row as usize][*col as usize] = true;
            const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for dir in DIRS {
                dfs(
                    grid,
                    visited,
                    m,
                    n,
                    &(*row as i32 + dir.0),
                    &(*col as i32 + dir.1),
                );
            }
        }

        for i in 0..m {
            dfs(&grid, &mut visited, &m, &n, &(i as i32), &0_i32);
            dfs(&grid, &mut visited, &m, &n, &(i as i32), &(n as i32 - 1));
        }
        for j in 1..n - 1 {
            dfs(&grid, &mut visited, &m, &n, &0, &(j as i32));
            dfs(&grid, &mut visited, &m, &n, &(m as i32 - 1), &(j as i32));
        }

        let mut enclaves = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 1 && !visited[i][j] {
                    enclaves += 1;
                }
            }
        }

        enclaves
    }

    pub fn num_enclaves_bfs(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut queue: std::collections::VecDeque<(usize, usize)> =
            std::collections::VecDeque::new();
        for i in 1..m {
            if grid[i][0] == 1 {
                visited[i][0] = true;
                queue.push_back((i, 0));
            }
            if grid[i][n - 1] == 1 {
                visited[i][n - 1] = true;
                queue.push_back((i, n - 1));
            }
        }

        for j in 1..n - 1 {
            if grid[0][j] == 1 {
                visited[0][j] = true;
                queue.push_back((0, j));
            }

            if grid[m - 1][j] == 1 {
                visited[m - 1][j] = true;
                queue.push_back((m - 1, j));
            }
        }

        while !queue.is_empty() {
            let (curr_row, curr_col) = queue.pop_front().unwrap();
            for dir in DIRS {
                let (next_row, next_col) = (curr_row as i32 + dir.0, curr_col as i32 + dir.1);
                if next_row >= 0
                    && next_row < m as i32
                    && next_col >= 0
                    && next_col < n as i32
                    && grid[next_row as usize][next_col as usize] == 1
                    && !visited[next_row as usize][next_col as usize]
                {
                    visited[next_row as usize][next_col as usize] = true;
                    queue.push_back((next_row as usize, next_col as usize));
                }
            }
        }
        let mut enclaves = 0;

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 1 && !visited[i][j] {
                    enclaves += 1;
                }
            }
        }

        enclaves
    }

    pub fn num_enclaves_diff_union(grid: Vec<Vec<i32>>) -> i32 {
        struct UnionFind {
            parent: Vec<usize>,
            on_edge: Vec<bool>,
            rank: Vec<usize>,
        }
        impl UnionFind {
            fn new(grid: &Vec<Vec<i32>>) -> UnionFind {
                let (m, n) = (grid.len(), grid[0].len());
                let mut union_find = UnionFind {
                    parent: vec![0; m * n],
                    on_edge: vec![false; m * n],
                    rank: vec![0; m * n],
                };

                for i in 0..m {
                    for j in 0..n {
                        if grid[i][j] == 1 {
                            let index = i * n + j;
                            union_find.parent[index] = index;
                            if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                                union_find.on_edge[index] = true;
                            }
                        }
                    }
                }
                union_find
            }

            fn find(&mut self, i: usize) -> usize {
                if self.parent[i] != i {
                    self.parent[i] = self.find(self.parent[i]);
                }
                return self.parent[i];
            }

            fn uni(&mut self, x: usize, y: usize) {
                let x_root = self.find(x);
                let y_root = self.find(y);
                if x_root != y_root {
                    if self.rank[x_root] > self.rank[y_root] {
                        self.parent[y_root] = x_root;
                        self.on_edge[x_root] |= self.on_edge[y_root];
                    } else if self.rank[x_root] < self.rank[y_root] {
                        self.parent[x_root] = y_root;
                        self.on_edge[y_root] |= self.on_edge[x_root];
                    } else {
                        self.parent[y_root] = x_root;
                        self.on_edge[x_root] |= self.on_edge[y_root];
                        self.rank[x_root] += 1;
                    }
                }
            }

            fn is_on_edge(&mut self, i: usize) -> bool {
                let f = self.find(i);
                self.on_edge[f]

                // let temp1 = &mut self.find(i);
                // let temp2 = &self.on_edge;     // index immutable get ERROR!
            }
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut union_find = UnionFind::new(&grid);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let index = i * n + j;
                    if j + 1 < n && grid[i][j + 1] == 1 {
                        union_find.uni(index, index + 1);
                    }
                    if i + 1 < m && grid[i + 1][j] == 1 {
                        union_find.uni(index, index + n);
                    }
                }
            }
        }

        let mut enclaves = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 1 && !union_find.is_on_edge(i * n + j) {
                    enclaves += 1;
                }
            }
        }

        enclaves
    }
}

fn main() {
    {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves_dfs(grid.clone()), 3);
        assert_eq!(Solution::num_enclaves_bfs(grid.clone()), 3);
        assert_eq!(Solution::num_enclaves_diff_union(grid.clone()), 3);
    }
    {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves_dfs(grid.clone()), 0);
        assert_eq!(Solution::num_enclaves_bfs(grid.clone()), 0);
        assert_eq!(Solution::num_enclaves_diff_union(grid.clone()), 0);
    }
}
