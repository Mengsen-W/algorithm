/*
 * @Date: 2023-09-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-17
 * @FilePath: /algorithm/rust/213_rob/rob.rs
 */

struct Solution;

impl Solution {
    pub fn sub_rob(nums: &[i32]) -> i32 {
        let mut dp = [0; 2];
        for num in nums {
            dp = [dp[1], (dp[0] + num).max(dp[1])];
        }
        dp[1]
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        Self::sub_rob(&nums[1..]).max(Self::sub_rob(&nums[..n - 1]))
    }
}

fn main() {
    let tests = vec![
        (vec![2, 3, 2], 3),
        (vec![1, 2, 3, 1], 4),
        (vec![1, 2, 3], 3),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::rob(nums), ans);
    }
}
