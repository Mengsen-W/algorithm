struct Solution;

use std::cmp::{max, min};
use std::collections::HashSet;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        fn find_ors(nums: &Vec<i32>, k: i32) -> Vec<HashSet<i32>> {
            let mut dp = Vec::new();
            let mut prev = vec![HashSet::new(); k as usize + 1];
            prev[0].insert(0);

            for i in 0..nums.len() {
                for j in (0..=min(k as usize - 1, i + 1)).rev() {
                    let (before, after) = prev.split_at_mut(j + 1);
                    for &x in &before[j] {
                        after[0].insert(x | nums[i]);
                    }
                }
                dp.push(prev[k as usize].clone());
            }
            dp
        }

        let a = find_ors(&nums, k);
        let reversed_nums: Vec<i32> = nums.iter().rev().cloned().collect();
        let b = find_ors(&reversed_nums, k);
        let mut mx = 0;
        for i in k as usize - 1..nums.len() - k as usize {
            for &a_val in &a[i] {
                for &b_val in &b[nums.len() - i - 2] {
                    mx = mx.max(a_val ^ b_val);
                }
            }
        }
        mx
    }
}

fn main() {
    let tests = vec![(vec![2, 6, 7], 1, 5), (vec![4, 2, 5, 6, 7], 2, 2)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::max_value(nums, k), ans);
    }
}
