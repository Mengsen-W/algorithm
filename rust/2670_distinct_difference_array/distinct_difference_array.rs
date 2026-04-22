/*
 * @Date: 2024-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-31
 * @FilePath: /algorithm/rust/2670_distinct_difference_array/distinct_difference_array.rs
 */

struct Solution;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let n = nums.len();
        let mut suf = vec![0; n + 1];
        let mut s = HashSet::new();

        for i in (0..n).rev() {
            s.insert(nums[i]);
            suf[i] = s.len();
        }

        let mut ans = Vec::new();
        s.clear();
        for i in 0..n {
            s.insert(nums[i]);
            ans.push((s.len() - suf[i + 1]) as i32);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], vec![-3, -1, 1, 3, 5]),
        (vec![3, 2, 3, 4, 2], vec![-2, -1, 0, 2, 3]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::distinct_difference_array(nums), ans);
    }
}
