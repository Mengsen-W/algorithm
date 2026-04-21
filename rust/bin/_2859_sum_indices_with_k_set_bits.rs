/*
 * @Date: 2024-01-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-25
 * @FilePath: /algorithm/rust/2859_sum_indices_with_k_set_bits/sum_indices_with_k_set_bits.rs
 */

struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().enumerate().fold(0, |acc, (i, &x)| {
            acc + if i.count_ones() == k as u32 { x } else { 0 }
        })
    }
}

fn main() {
    let tests = vec![(vec![5, 10, 1, 5, 2], 1, 13), (vec![4, 3, 2, 1], 2, 1)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::sum_indices_with_k_set_bits(nums, k), ans);
    }
}
