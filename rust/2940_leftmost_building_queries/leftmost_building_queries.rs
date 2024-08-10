struct Solution;

use std::vec::Vec;

fn build(l: usize, r: usize, rt: usize, heights: &Vec<i32>, zd: &mut Vec<i32>) {
    if l == r {
        zd[rt] = heights[l - 1];
        return;
    }

    let mid = (l + r) / 2;
    build(l, mid, rt << 1, heights, zd);
    build(mid + 1, r, rt << 1 | 1, heights, zd);
    zd[rt] = std::cmp::max(zd[rt << 1], zd[rt << 1 | 1]);
}

fn query(pos: usize, val: i32, l: usize, r: usize, rt: usize, zd: &Vec<i32>) -> i32 {
    if val >= zd[rt] {
        return 0;
    }

    if l == r {
        return l as i32;
    }

    let mid = (l + r) / 2;
    if pos <= mid {
        let res = query(pos, val, l, mid, rt << 1, zd);
        if res != 0 {
            return res;
        }
    }
    query(pos, val, mid + 1, r, rt << 1 | 1, zd)
}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let mut zd: Vec<i32> = vec![0; n * 4];
        build(1, n, 1, &heights, &mut zd);

        let m = queries.len();
        let mut ans = Vec::with_capacity(m);
        for q in queries {
            let (mut a, mut b) = (q[0] as usize, q[1] as usize);
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            if a == b || heights[a] < heights[b] {
                ans.push(b as i32);
                continue;
            }
            ans.push(query(b + 1, heights[a], 1, n, 1, &zd) - 1);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![6, 4, 8, 5, 2, 7],
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]],
            vec![2, 5, -1, 5, 2],
        ),
        (
            vec![5, 3, 8, 2, 6, 1, 4, 6],
            vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]],
            vec![7, 6, -1, 4, 6],
        ),
    ];

    for (height, queries, ans) in tests {
        assert_eq!(Solution::leftmost_building_queries(height, queries), ans);
    }
}
