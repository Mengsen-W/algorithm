/*
 * @Date: 2024-03-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-02
 * @FilePath: /algorithm/rust/2369_valid_partition/valid_partition.rs
 */

struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            if i >= 2 {
                dp[i] = dp[i - 2] && valid_two(nums[i - 2], nums[i - 1]);
            }
            if i >= 3 {
                dp[i] = dp[i] || (dp[i - 3] && valid_three(nums[i - 3], nums[i - 2], nums[i - 1]));
            }
        }
        dp[n]
    }
}

fn valid_two(num1: i32, num2: i32) -> bool {
    num1 == num2
}

fn valid_three(num1: i32, num2: i32, num3: i32) -> bool {
    (num1 == num2 && num1 == num3) || (num1 + 1 == num2 && num2 + 1 == num3)
}

fn main() {
    let tests = vec![(vec![4, 4, 4, 5, 6], true), (vec![1, 1, 1, 2], false)];

    for (nums, ans) in tests {
        assert_eq!(Solution::valid_partition(nums), ans);
    }
}
