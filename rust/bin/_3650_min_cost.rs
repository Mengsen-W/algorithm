struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (x, y, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[x].push((y as i32, w));
            g[y].push((x as i32, 2 * w));
        }

        let mut dist = vec![i32::MAX; n];
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new(); // 最大堆，但存负值

        dist[0] = 0;
        heap.push((0, 0)); // (-距离, 节点)

        while let Some((neg_d, node)) = heap.pop() {
            let d = -neg_d;
            let node = node as usize;

            if node == n - 1 {
                return d;
            }

            if visited[node] {
                continue;
            }
            visited[node] = true;

            for &(next, weight) in &g[node] {
                let next_idx = next as usize;
                let new_dist = d + weight;
                if new_dist < dist[next_idx] {
                    dist[next_idx] = new_dist;
                    heap.push((-new_dist, next));
                }
            }
        }

        -1
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]],
            5,
        ),
        (
            4,
            vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]],
            3,
        ),
    ];

    for (n, edges, expected) in tests {
        assert_eq!(Solution::min_cost(n, edges), expected);
    }
}
