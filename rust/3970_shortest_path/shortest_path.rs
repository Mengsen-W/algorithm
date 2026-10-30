struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let labels = labels.as_bytes();
        let mut g = vec![Vec::new(); n];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2] as i64));
        }

        let mut dis = vec![vec![i32::MAX as i64; k + 1]; n];
        // {最短路长度, 节点编号, 最后连续相同字母个数}
        let mut pq = BinaryHeap::new();

        dis[0][1] = 0;
        pq.push(Reverse((0_i64, 0_usize, 1_usize)));

        while let Some(Reverse((d, x, cnt))) = pq.pop() {
            if d > dis[x][cnt] {
                continue;
            }
            if x == n - 1 {
                return d as i32;
            }

            for &(y, w) in &g[x] {
                let new_cnt = if labels[x] == labels[y] { cnt + 1 } else { 1 };
                if new_cnt > k {
                    continue;
                }

                let nd = d + w;
                if nd < dis[y][new_cnt] {
                    dis[y][new_cnt] = nd;
                    pq.push(Reverse((nd, y, new_cnt)));
                }
            }
        }

        -1
    }
}

fn main() {
    let tests = vec![
        (
            3,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
            "aab",
            1,
            3,
        ),
        (
            3,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
            "aab",
            2,
            2,
        ),
        (3, vec![vec![0, 1, 1], vec![1, 2, 1]], "aaa", 2, -1),
    ];

    for (n, edges, labels, k, expected) in tests {
        assert_eq!(
            Solution::shortest_path(n, edges, labels.to_string(), k),
            expected
        );
    }
}
