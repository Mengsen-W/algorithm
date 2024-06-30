/*
 * @Date: 2021-06-07 08:26:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-07 08:48:42
 */

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut sum: i32 = 0;
        for num in &nums {
            sum += num;
        }
        let diff = sum - s;
        if diff < 0 || diff % 2 != 0 {
            return 0;
        }

        let neg: usize = (diff / 2) as usize;
        let mut dp: Vec<i32> = vec![0; neg + 1];
        dp[0] = 1;

        for num in &nums {
            for j in (*num as usize..neg + 1).rev() {
                dp[j] += dp[j - *num as usize];
            }
        }
        return dp[neg];
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 1, 1, 1], 3, 5), (vec![1], 1, 1)];

    for (nums, s, ans) in tests {
        assert_eq!(Solution::find_target_sum_ways(nums, s), ans);
    }
}
