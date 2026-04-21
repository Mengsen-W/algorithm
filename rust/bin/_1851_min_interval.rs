/*
 * @Date: 2023-07-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-18
 * @FilePath: /algorithm/rust/1851_min_interval/min_interval.rs
 */

struct Solution;
impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut queries_w_order: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();

        queries_w_order.sort_unstable_by_key(|q| q.1);
        intervals.sort_unstable_by_key(|v| v[0]);

        let mut cur = 0;
        let mut pq = BinaryHeap::new();
        let mut result = Vec::with_capacity(queries_w_order.len());

        for &(idx, query) in queries_w_order.iter() {
            while cur < intervals.len() && intervals[cur][0] <= query {
                pq.push(Reverse((
                    intervals[cur][1] - intervals[cur][0] + 1,
                    intervals[cur][1],
                )));
                cur += 1;
            }
            loop {
                match pq.peek() {
                    None => break,
                    Some(&Reverse(peek)) => {
                        if peek.1 < query {
                            pq.pop();
                        } else {
                            break;
                        }
                    }
                }
            }
            result.push((idx, pq.peek().map_or(-1, |&Reverse(x)| x.0)));
        }

        result.sort_unstable();
        result.into_iter().map(|x| x.1).collect()
    }
}

fn main() {
    let tests = vec![
        (
            [[1, 4], [2, 4], [3, 6], [4, 4]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            [2, 3, 4, 5].to_vec(),
            [3, 3, 1, 4].to_vec(),
        ),
        (
            [[2, 3], [2, 5], [1, 8], [20, 25]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            [2, 19, 5, 22].to_vec(),
            [2, -1, 4, 6].to_vec(),
        ),
    ];

    for (intervals, queries, result) in tests {
        assert_eq!(Solution::min_interval(intervals, queries), result);
    }
}
