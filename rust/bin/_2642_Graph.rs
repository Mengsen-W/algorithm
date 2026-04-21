/*
 * @Date: 2024-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-26
 * @FilePath: /algorithm/rust/2642_Graph/Graph.rs
 */

use std::cmp::min;

struct Graph {
    dist: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for edge in edges {
            dist[edge[0] as usize][edge[1] as usize] = edge[2];
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
                    }
                }
            }
        }
        return Graph { dist };
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let x = edge[0] as usize;
        let y = edge[1] as usize;
        let cost = edge[2];
        if cost >= self.dist[x][y] {
            return;
        }
        let n = self.dist.len();
        for i in 0..n {
            for j in 0..n {
                if self.dist[i][x] != i32::MAX && self.dist[y][j] != i32::MAX {
                    self.dist[i][j] =
                        min(self.dist[i][j], self.dist[i][x] + cost + self.dist[y][j]);
                }
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let res = self.dist[node1 as usize][node2 as usize];
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

fn main() {
    let mut g = Graph::new(
        4,
        vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
    );

    assert_eq!(g.shortest_path(3, 2), 6);
    assert_eq!(g.shortest_path(0, 3), -1);
    g.add_edge(vec![1, 3, 4]);
    assert_eq!(g.shortest_path(0, 3), 6);
}
