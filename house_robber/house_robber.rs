/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-08 17:05:21
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-08 17:55:58
 */

use std::cmp;

fn house_robber_linear(nums: &Vec<i32>) -> i32 {
    let size = nums.len();
    if size <= 2 {
        return cmp::max(nums[0], nums[1]);
    }
    let mut dp = vec![i32::MIN; size];
    dp[0] = nums[0];
    dp[1] = nums[1];

    for i in 2..size {
        dp[i] = cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
    }
    return dp[size - 1];
}

fn house_robber_circle(nums: &Vec<i32>) -> i32 {
    let size = nums.len();
    if size <= 2 {
        return cmp::max(nums[0], nums[1]);
    }
    let mut dp1 = vec![i32::MIN; size - 1];
    dp1[0] = nums[0];
    dp1[1] = nums[1];
    for i in 2..size - 1 {
        dp1[i] = cmp::max(dp1[i - 2] + nums[i], dp1[i - 1]);
    }
    let res1 = dp1[size - 2];

    let mut dp2 = vec![i32::MIN; size - 1];
    dp2[0] = nums[1];
    dp2[1] = nums[2];
    for i in 2..size - 1 {
        dp2[i] = cmp::max(dp2[i - 2] + nums[i + 1], dp2[i - 1]);
    }
    let res2 = dp2[size - 2];
    return cmp::max(res1, res2);
}

fn main() {
    let mut nums: Vec<i32>;
    {
        nums = vec![1, 2, 3, 1];
        println!("{}", house_robber_linear(&nums));
        nums = vec![2, 7, 9, 3, 1];
        println!("{}", house_robber_linear(&nums));
    }
    {
        nums = vec![2, 3, 2];
        println!("{}", house_robber_circle(&nums));
    }
}
