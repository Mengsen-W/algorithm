/*
 * @Date: 2023-10-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-21
 * @FilePath: /algorithm/rust/2316_count_pairs/count_pairs.rs
 */

struct Solution;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct UnionFind(Vec<usize>);
impl UnionFind {
    fn new(n: usize) -> Self {
        Self((0..n).into_iter().collect())
    }
    fn find(&mut self, idx: usize) -> usize {
        if self.0[idx] == idx {
            return idx;
        }
        self.0[idx] = self.find(self.0[idx]);
        self.0[idx]
    }
    fn union(&mut self, idx0: usize, idx1: usize) {
        let p = self.find(idx0);
        self.0[p] = self.find(idx1);
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut uf = UnionFind::new(n as usize);
        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }
        let mut map = HashMap::new();
        for i in 0..n as usize {
            map.entry(uf.find(i)).and_modify(|x| *x += 1).or_insert(1);
        }
        map.into_values()
            .fold((0, 0), |(res, pre), x| (res + pre * x, pre + x))
            .0
    }
}

fn main() {
    let tests = vec![
        (3, vec![vec![0, 1], vec![0, 2], vec![1, 2]], 0),
        (
            7,
            vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]],
            14,
        ),
    ];

    for (n, edges, ans) in tests {
        assert_eq!(Solution::count_pairs(n, edges), ans);
    }
}
