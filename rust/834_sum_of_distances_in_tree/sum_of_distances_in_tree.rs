/*
 * @Date: 2023-07-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-16
 * @FilePath: /algorithm/rust/834_sum_of_distances_in_tree/sum_of_distances_in_tree.rs
 */

struct Solution;

/// Stores the progress of calculations
pub struct Solver {
    sums: Vec<i32>,
    counters: Vec<usize>,
    adj_list: Vec<Vec<usize>>,
    n: i32,
}

impl Solver {
    /// Creates a new instance
    pub fn new(sums: Vec<i32>, counters: Vec<usize>, adj_list: Vec<Vec<usize>>, n: i32) -> Self {
        Self {
            sums,
            counters,
            adj_list,
            n,
        }
    }
    /// Counts nodes for this tree and all subtrees
    pub fn dfs_count(&mut self, parent: i32, node: usize) {
        let adj_list = self.adj_list[node].clone();
        for &child in &adj_list {
            if child as i32 != parent {
                self.dfs_count(node as i32, child);
                self.counters[node] += self.counters[child];
                self.sums[node] += self.sums[child] + self.counters[child] as i32;
            }
        }
    }
    /// Calculates the sum of all paths for this node and all child nodes
    pub fn dfs_sum(&mut self, parent: i32, node: usize) {
        let adj_list = self.adj_list[node].clone();
        for &child in &adj_list {
            if child as i32 != parent {
                self.sums[child] = (self.n - self.counters[child] as i32)
                    + (self.sums[node] - self.counters[child] as i32);
                self.dfs_sum(node as i32, child);
            }
        }
    }
    /// Solves the problem
    pub fn solve(mut self) -> Vec<i32> {
        self.dfs_count(-1, 0);
        self.dfs_sum(-1, 0);
        self.sums
    }
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges.iter() {
            adj_list[edge[0] as usize].push(edge[1] as usize);
            adj_list[edge[1] as usize].push(edge[0] as usize);
        }
        let count: Vec<usize> = vec![1; n];
        let sums: Vec<i32> = vec![0; n];
        let solver = Solver::new(sums, count, adj_list, n as i32);
        solver.solve()
    }
}

fn main() {
    let tests = vec![
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
            vec![8, 12, 6, 10, 10, 10],
        ),
        (1, vec![], vec![0]),
        (2, vec![vec![1, 0]], vec![1, 1]),
    ];

    for (n, edges, result) in tests {
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), result);
    }
}
