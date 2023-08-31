/*
 * @Date: 2023-08-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-31
 * @FilePath: /algorithm/rust/1761_min_trio_degree/min_trio_degree.rs
 */

struct Solution;
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![false; n]; n];
        let mut dgr = vec![0; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize - 1, edge[1] as usize - 1);
            g[u][v] = true;
            g[v][u] = true;
            dgr[u] += 1;
            dgr[v] += 1;
        }
        let mut res = i32::MAX;
        (0..n).for_each(|i| {
            (i + 1..n).for_each(|j| {
                if !g[i][j] {
                    return;
                }
                (j + 1..n).for_each(|k| {
                    if g[i][k] && g[j][k] {
                        res = res.min(dgr[i] + dgr[j] + dgr[k] - 6);
                    }
                })
            })
        });
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

fn main() {
    let tests = vec![
        (
            6,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![3, 2],
                vec![4, 1],
                vec![5, 2],
                vec![3, 6],
            ],
            3,
        ),
        (
            7,
            vec![
                vec![1, 3],
                vec![4, 1],
                vec![4, 3],
                vec![2, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 5],
                vec![2, 6],
            ],
            0,
        ),
    ];

    for (n, edges, ans) in tests {
        assert_eq!(Solution::min_trio_degree(n, edges), ans);
    }
}
