/*
 * @Date: 2021-09-20 09:17:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-20 10:09:13
 */

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut max_len, mut ans) = (0, 0);
        let mut dp: Vec<_> = vec![0; n];
        let mut cnt: Vec<_> = vec![0; n];

        for i in 0..n {
            dp[i] = 1;
            cnt[i] = 1;
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        cnt[i] = cnt[j];
                    } else if dp[j] + 1 == dp[i] {
                        cnt[i] += cnt[j];
                    }
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                ans = cnt[i]; // 重置计数
            } else if dp[i] == max_len {
                ans += cnt[i];
            }
        }
        ans
    }
}

fn main() {
    {
        let nums = vec![1, 3, 5, 4, 7];
        assert_eq!(Solution::find_number_of_lis(nums), 2);
    }

    {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(Solution::find_number_of_lis(nums), 5);
    }
}
