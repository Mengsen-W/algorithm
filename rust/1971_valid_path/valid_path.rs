/*
 * @Date: 2022-12-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-19
 * @FilePath: /algorithm/1971_valid_path/valid_path.rs
 */

struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            return true;
        }
        let mut uf = UnionFind::new(n as usize);

        for edge in edges {
            uf.uni(edge[0], edge[1]);
        }
        return uf.connect(source, destination);
    }
}

struct UnionFind {
    parent: Vec<i32>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            rank: vec![0; n],
            parent: (0..n).map(|s| s as i32).collect(),
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn connect(&mut self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }

    fn uni(&mut self, x: i32, y: i32) {
        let rootx = self.find(x) as usize;
        let rooty = self.find(y) as usize;

        if rootx != rooty {
            if self.rank[rootx] > self.rank[rooty] {
                self.parent[rooty] = rootx as i32;
            } else if self.rank[rootx] < self.rank[rooty] {
                self.parent[rootx] = rooty as i32;
            } else {
                self.parent[rooty] = rootx as i32;
                self.rank[rootx] += 1;
            }
        }
    }
}

fn main() {
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        assert!(Solution::valid_path(n, edges, source, destination));
    }

    {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let source = 0;
        let destination = 5;
        assert!(!Solution::valid_path(n, edges, source, destination));
    }
}
