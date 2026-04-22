/*
 * @Date: 2024-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-14
 * @FilePath: /algorithm/rust/2244_minimum_rounds/minimum_rounds.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        for &t in &tasks {
            *cnt.entry(t).or_insert(0) += 1;
        }
        let mut res = 0;
        for (&_, &v) in &cnt {
            if v == 1 {
                return -1;
            } else if v % 3 == 0 {
                res += v / 3;
            } else {
                res += v / 3 + 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4], 4), (vec![2, 3, 3], -1)];

    for (tasks, ans) in tests {
        assert_eq!(Solution::minimum_rounds(tasks), ans);
    }
}
