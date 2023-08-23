/*
 * @Date: 2023-08-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-23
 * @FilePath: /algorithm/rust/1782_count_pairs/count_pairs.rs
 */

struct Solution;
use std::collections::HashMap;

impl Solution {
    fn get_edge_map(edges: &Vec<Vec<i32>>) -> HashMap<(i32, i32), i32> {
        let mut m = HashMap::new();
        for edge in edges.iter() {
            let (x, y) = (edge[0] - 1, edge[1] - 1);
            if x < y {
                m.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
            } else {
                m.entry((y, x)).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        m
    }

    fn get_into_vec(n: usize, edges: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut into = vec![0; n];
        for edge in edges.iter() {
            let (x, y) = (edge[0] - 1, edge[1] - 1);
            into[x as usize] += 1;
            into[y as usize] += 1;
        }
        into
    }

    fn get_sorted_into_vec(original: &Vec<i32>) -> Vec<i32> {
        let mut new: Vec<i32> = original.clone();
        new.sort_unstable();
        new
    }

    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let edge_map = Self::get_edge_map(&edges);
        let into = Self::get_into_vec(n, &edges);
        let sorted_into = Self::get_sorted_into_vec(&into);
        let mut ans = Vec::with_capacity(queries.len());

        for k in queries.into_iter() {
            let mut res = 0;
            let (mut i, mut j) = (0, n - 1);

            while j > i {
                while j > i && sorted_into[i] + sorted_into[j] > k {
                    j -= 1;
                }
                res += n - j - 1;
                i += 1;
            }

            res += (n - i) * (n - i - 1) / 2;

            for ((x, y), m) in edge_map.iter() {
                let (x, y) = (*x as usize, *y as usize);
                if into[x] + into[y] <= k + m && into[x] + into[y] > k {
                    res -= 1;
                }
            }

            ans.push(res as i32);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            4,
            [[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            [2, 3].to_vec(),
            [6, 5].to_vec(),
        ),
        (
            5,
            [
                [1, 5],
                [1, 5],
                [3, 4],
                [2, 5],
                [1, 3],
                [5, 1],
                [2, 3],
                [2, 5],
            ]
            .iter()
            .map(|v| v.to_vec())
            .collect(),
            [1, 2, 3, 4, 5].to_vec(),
            [10, 10, 9, 8, 6].to_vec(),
        ),
    ];

    for (n, edges, queries, ans) in tests {
        assert_eq!(Solution::count_pairs(n, edges, queries), ans)
    }
}
