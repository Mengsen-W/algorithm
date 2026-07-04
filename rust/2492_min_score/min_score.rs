struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    v: usize,
    dis: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dis.cmp(&self.dis).then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<Edge>> = vec![vec![]; n + 1];
        let mut vis = vec![false; n + 1];

        let mut ans = i32::MAX;
        let mut pq = BinaryHeap::new();

        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let dis = road[2];

            graph[u].push(Edge { v, dis });
            graph[v].push(Edge { v: u, dis });

            if pq.is_empty() && (u == 1 || v == 1) {
                pq.push(Edge { v, dis });
            }
        }

        while let Some(Edge { v: u, dis }) = pq.pop() {
            if vis[u] {
                continue;
            }

            ans = ans.min(dis);
            vis[u] = true;

            for edge in &graph[u] {
                if !vis[edge.v] {
                    pq.push(Edge {
                        v: edge.v,
                        dis: edge.dis,
                    });
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (4, vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5]], 5),
        (4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]], 2),
    ];

    for (n, roads, expected) in tests {
        assert_eq!(Solution::min_score(n, roads), expected);
    }
}
