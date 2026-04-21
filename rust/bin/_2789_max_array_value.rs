/*
 * @Date: 2024-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-14
 * @FilePath: /algorithm/rust/2789_max_array_value/max_array_value.rs
 */

struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut sum = nums[n - 1] as i64;
        for i in (0..n - 1).rev() {
            let x = nums[i] as i64;
            sum = if x <= sum { sum + x } else { x };
        }
        sum
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 7, 9, 3], 21), (vec![5, 3, 3], 11)];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_array_value(nums), ans);
    }
}
