struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = online.len();
        let mut g = vec![vec![]; n];
        let mut l = i32::MAX;
        let mut r = 0;

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if !online[u] || !online[v] {
                continue;
            }
            g[u].push((v, w as i64));
            l = l.min(w);
            r = r.max(w);
        }

        let check = |mid: i32| -> bool {
            let mut dis = vec![i64::MAX; n];
            let mut pq = BinaryHeap::new();

            dis[0] = 0;
            pq.push(Reverse((0, 0)));

            while let Some(Reverse((d, u))) = pq.pop() {
                if d > k {
                    return false;
                }
                if u == n - 1 {
                    return true;
                }
                if d > dis[u] {
                    continue;
                }

                for &(v, w) in &g[u] {
                    if w < mid as i64 {
                        continue;
                    }
                    if dis[v] > dis[u] + w {
                        dis[v] = dis[u] + w;
                        pq.push(Reverse((dis[v], v)));
                    }
                }
            }
            false
        };

        if !check(l) {
            return -1;
        }

        while l <= r {
            let mid = (l + r) >> 1;
            if check(mid) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        r
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]],
            vec![true, true, true, true],
            10,
            3,
        ),
        (
            vec![
                vec![0, 1, 7],
                vec![1, 4, 5],
                vec![0, 2, 6],
                vec![2, 3, 6],
                vec![3, 4, 2],
                vec![2, 4, 6],
            ],
            vec![true, true, true, false, true],
            12,
            6,
        ),
    ];

    for (edges, expected, k, n) in tests {
        assert_eq!(Solution::find_max_path_score(edges, expected, k), n);
    }
}
