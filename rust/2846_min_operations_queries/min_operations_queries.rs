/*
 * @Date: 2024-01-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-26
 * @FilePath: /algorithm/rust/2846_min_operations_queries/min_operations_queries.rs
 */

struct Solution;

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        #[derive(Clone, Copy)]
        struct Edge {
            to: usize,
            wt: i32,
        }

        let mut g: Vec<Vec<Edge>> = vec![vec![]; n as usize];
        for e in edges {
            let (v, w, wt) = (e[0] as usize, e[1] as usize, e[2] - 1);
            g[v].push(Edge { to: w, wt });
            g[w].push(Edge { to: v, wt });
        }

        const MX: usize = 14;
        #[derive(Clone, Copy)]
        struct Pair {
            p: i32,
            cnt: [i32; 26],
        }

        let mut pa: Vec<[[Pair; MX]; 1]> = vec![
            [[Pair {
                p: -1,
                cnt: [0; 26]
            }; MX]; 1];
            n as usize
        ];
        let mut depth: Vec<i32> = vec![0; n as usize];

        fn build(
            v: usize,
            p: i32,
            d: i32,
            pa: &mut Vec<[[Pair; MX]; 1]>,
            depth: &mut Vec<i32>,
            g: &Vec<Vec<Edge>>,
        ) {
            pa[v][0][0].p = p;
            depth[v] = d;
            for &e in &g[v] {
                if e.to != p as usize {
                    pa[e.to][0][0].cnt[e.wt as usize] = 1;
                    build(e.to, v as i32, d + 1, pa, depth, g);
                }
            }
        }

        build(0, -1, 0, &mut pa, &mut depth, &g);

        for i in 0..MX - 1 {
            for v in 0..n as usize {
                if pa[v][0][i].p != -1 {
                    let pp = pa[pa[v][0][i].p as usize][0][i];
                    pa[v][0][i + 1].p = pp.p;
                    for j in 0..26 {
                        pa[v][0][i + 1].cnt[j] = pa[v][0][i].cnt[j] + pp.cnt[j];
                    }
                } else {
                    pa[v][0][i + 1].p = -1;
                }
            }
        }

        let mut ans: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            let (mut v, mut w) = (q[0] as usize, q[1] as usize);
            let mut path_len = depth[v] + depth[w];
            let mut cnt = [0; 26];
            if depth[v] > depth[w] {
                std::mem::swap(&mut v, &mut w);
            }
            for i in 0..MX {
                if ((depth[w] - depth[v]) >> i) & 1 > 0 {
                    let p = pa[w][0][i];
                    for j in 0..26 {
                        cnt[j] += p.cnt[j];
                    }
                    w = p.p as usize;
                }
            }
            if w != v {
                for i in (0..MX).rev() {
                    if pa[v][0][i].p != pa[w][0][i].p {
                        for j in 0..26 {
                            cnt[j] += pa[v][0][i].cnt[j] + pa[w][0][i].cnt[j];
                        }
                        v = pa[v][0][i].p as usize;
                        w = pa[w][0][i].p as usize;
                    }
                }
                for j in 0..26 {
                    cnt[j] += pa[v][0][0].cnt[j] + pa[w][0][0].cnt[j];
                }
                v = pa[v][0][0].p as usize;
            }
            path_len -= 2 * depth[v];
            ans.push(path_len - *cnt.iter().max().unwrap());
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            7,
            vec![
                vec![0, 1, 1],
                vec![1, 2, 1],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 2],
                vec![5, 6, 2],
            ],
            vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]],
            vec![0, 0, 1, 3],
        ),
        (
            8,
            vec![
                vec![1, 2, 6],
                vec![1, 3, 4],
                vec![2, 4, 6],
                vec![2, 5, 3],
                vec![3, 6, 6],
                vec![3, 0, 8],
                vec![7, 0, 2],
            ],
            vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]],
            vec![1, 2, 2, 3],
        ),
    ];

    for (n, edges, queries, ans) in tests {
        assert_eq!(Solution::min_operations_queries(n, edges, queries), ans);
    }
}
