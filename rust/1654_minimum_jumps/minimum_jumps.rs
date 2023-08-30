/*
 * @Date: 2023-08-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-30
 * @FilePath: /algorithm/rust/1654_minimum_jumps/minimum_jumps.rs
 */

struct Solution;
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        use std::cmp::Reverse as rvs;
        use std::collections::{BinaryHeap, HashSet};
        let limit = if a >= b {
            x + b
        } else {
            x.max(a + b + forbidden.iter().max().unwrap())
        };
        let forbidden: HashSet<_> = forbidden.into_iter().collect();
        let mut visited = HashSet::with_capacity(limit as usize * 2);
        let mut heap: _ = BinaryHeap::new();
        heap.push((rvs(x), rvs(0), 0, true));
        while let Some((_, rvs(step), cur_pos, can_back)) = heap.pop() {
            if cur_pos == x {
                return step;
            }
            if !visited.insert((cur_pos, can_back)) {
                continue;
            }
            let next_pos = cur_pos + a;
            if next_pos <= limit && forbidden.get(&next_pos).is_none() {
                let a_star = (x - next_pos).abs() + step + 1;
                heap.push((rvs(a_star), rvs(step + 1), next_pos, true));
            }
            let next_pos = cur_pos - b;
            if can_back && next_pos > 0 && forbidden.get(&next_pos).is_none() {
                let a_star = (x - next_pos).abs() + step + 1;
                heap.push((rvs(a_star), rvs(step + 1), next_pos, false));
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![14, 4, 18, 1, 15], 3, 15, 9, 3),
        (vec![8, 3, 16, 6, 12, 20], 15, 13, 11, -1),
        (vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7, 2),
    ];

    for (forbidden, a, b, x, ans) in tests {
        assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), ans)
    }
}
