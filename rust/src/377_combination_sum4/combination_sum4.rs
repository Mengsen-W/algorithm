/*
 * @Date: 2021-04-24 14:33:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-24 14:56:57
 */

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

fn main() {
    {
        let nums = vec![1, 2, 3];
        assert_eq!(combination_sum4(nums, 4), 7);
    }
    {
        let nums = vec![9];
        assert_eq!(combination_sum4(nums, 3), 0);
    }
}
