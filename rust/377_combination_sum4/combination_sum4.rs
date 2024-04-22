/*
 * @Date: 2021-04-24 14:33:04
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-22
 */

struct Solution;

impl Solution {
    fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target as usize {
            for &j in nums.iter() {
                if i as i32 >= j {
                    dp[i] += dp[i - j as usize];
                }
            }
        }
        return dp[target as usize];
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 4, 7), (vec![9], 3, 0)];

    for (nums, target, ans) in tests {
        assert_eq!(combination_sum4(nums, target), ans);
    }
}
