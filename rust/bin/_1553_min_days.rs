/*
 * @Date: 2024-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-12
 * @FilePath: /algorithm/rust/1553_min_days/min_days.rs
 */

struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        fn get_heuristic_value(rest: i32) -> i32 {
            return if rest == 0 {
                0
            } else {
                ((rest as f32).log2() / 3.0f32.log2()) as i32 + 1
            };
        }

        let mut q: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        q.push((Reverse(get_heuristic_value(n) + 0), 0, n));
        let mut visited = HashSet::new();
        let mut ans = 0;
        loop {
            let Some((_, days, rest)) = q.pop() else {
                break;
            };
            if visited.contains(&rest) {
                continue;
            }
            visited.insert(rest);
            if rest == 1 {
                ans = days + 1;
                break;
            }
            q.push((
                Reverse(days + rest % 2 + 1 + get_heuristic_value(rest / 2)),
                days + rest % 2 + 1,
                rest / 2,
            ));
            q.push((
                Reverse(days + rest % 3 + 1 + get_heuristic_value(rest / 3)),
                days + rest % 3 + 1,
                rest / 3,
            ));
        }
        return ans;
    }
}

fn main() {
    let tests = vec![(10, 4), (6, 3), (1, 1), (56, 6)];

    for (n, ans) in tests {
        assert_eq!(Solution::min_days(n), ans);
    }
}
