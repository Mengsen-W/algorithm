/*
 * @Date: 2024-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-23
 * @FilePath: /algorithm/rust/2765_alternating_subarray/alternating_subarray.rs
 */

struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut first_index = 0;
        let n = nums.len();
        for i in 1..n {
            let length = i - first_index + 1;
            if nums[i] - nums[first_index] == (length - 1) as i32 % 2 {
                res = res.max(length as i32);
            } else {
                if nums[i] - nums[i - 1] == 1 {
                    first_index = i - 1;
                    res = res.max(2);
                } else {
                    first_index = i;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 4, 3, 4], 4), (vec![4, 5, 6], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::alternating_subarray(nums), ans);
    }
}
