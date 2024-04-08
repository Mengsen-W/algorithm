/*
 * @Date: 2024-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-08
 * @FilePath: /algorithm/rust/2009_min_operations/min_operations.rs
 */

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sorted_unique_nums: Vec<i32> = nums
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let mut sorted_unique_nums = sorted_unique_nums;
        sorted_unique_nums.sort_unstable();
        let mut res = n as i32;
        let mut j = 0;
        for (i, &left) in sorted_unique_nums.iter().enumerate() {
            let right = left + n as i32 - 1;
            while j < sorted_unique_nums.len() && sorted_unique_nums[j] <= right {
                res = res.min(n as i32 - (j - i + 1) as i32);
                j += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 5, 3], 0),
        (vec![1, 2, 3, 5, 6], 1),
        (vec![1, 10, 100, 1000], 3),
    ];

    for (input, ans) in tests {
        assert_eq!(Solution::min_operations(input), ans);
    }
}
