/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/rust/2656_maximize_sum/maximize_sum.rs
 */

struct Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mx = *nums.iter().max().unwrap();
        (2 * mx + k - 1) * k / 2
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 4, 5], 3, 18), (vec![5, 5, 5], 2, 11)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::maximize_sum(nums, k), ans);
    }
}
