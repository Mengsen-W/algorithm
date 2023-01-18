/*
 * @Date: 2021-08-06 10:54:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-06 11:53:28
 */

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_length_bfs(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut q = VecDeque::new();
        let mut seen = vec![vec![false; 1 << n]; n];
        for i in 0..n {
            q.push_back((i, 1 << i, 0));
            seen[i][1 << i] = true;
        }
        let mut ans = 0;
        while !q.is_empty() {
            if let Some((u, mask, dist)) = q.pop_front() {
                if mask == (1 << n) - 1 {
                    ans = dist;
                    break;
                }
                for &v in &graph[u] {
                    let mask_v = mask | (1 << v);
                    if !seen[v as usize][mask_v as usize] {
                        q.push_back((v as usize, mask_v, dist + 1));
                        seen[v as usize][mask_v as usize] = true;
                    }
                }
            }
        }
        ans
    }
    pub fn shortest_path_length_floyd(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut d = vec![vec![n + 1; n]; n];
        for i in 0..n {
            for &j in &graph[i] {
                d[i][j as usize] = 1;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                }
            }
        }

        let mut f = vec![vec![i32::MAX / 2; 1 << n]; n];
        for mask in 1usize..(1 << n) {
            if mask & (mask - 1) == 0 {
                let u = mask.trailing_zeros() as usize;
                f[u][mask] = 0;
            } else {
                for u in 0..n {
                    if mask & (1 << u) != 0  {
                        for v in 0..n {
                            if (mask & (1 << v) != 0) && u != v {
                                f[u][mask] =
                                    f[u][mask].min(f[v][mask ^ (1 << u)] + d[v][u as usize] as i32);
                            }
                        }
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for u in 0..n {
            ans = ans.min(f[u][(1 << n) - 1]);
        }

        ans
    }
}

fn main() {
    {
        let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        assert_eq!(Solution::shortest_path_length_bfs(graph.clone()), 4);
        assert_eq!(Solution::shortest_path_length_floyd(graph.clone()), 4);
    }

    {
        let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
        assert_eq!(Solution::shortest_path_length_bfs(graph.clone()), 4);
        assert_eq!(Solution::shortest_path_length_floyd(graph.clone()), 4);
    }
}
