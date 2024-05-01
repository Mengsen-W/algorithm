/*
 * @Date: 2024-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-01
 * @FilePath: /algorithm/rust/2462_total_cost/total_cost.rs
 */

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let n = costs.len();
        let mut q: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut left = (candidates - 1) as usize;
        let mut right = (n as i32 - candidates) as usize;

        if left + 1 < right {
            for i in 0..=left {
                q.push(Reverse((costs[i], i as i32)));
            }
            for i in right..n {
                q.push(Reverse((costs[i], i as i32)));
            }
        } else {
            for i in 0..n {
                q.push(Reverse((costs[i], i as i32)));
            }
        }

        let mut ans = 0i64;
        for _ in 0..k {
            let (cost, id) = q.pop().unwrap().0;
            ans += cost as i64;
            if left + 1 < right {
                if id as usize <= left {
                    left += 1;
                    q.push(Reverse((costs[left], left as i32)));
                } else {
                    right -= 1;
                    q.push(Reverse((costs[right], right as i32)));
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4, 11),
        (vec![1, 2, 4, 1], 3, 3, 4),
    ];

    for (costs, k, candidates, ans) in tests {
        assert_eq!(Solution::total_cost(costs, k, candidates), ans);
    }
}
