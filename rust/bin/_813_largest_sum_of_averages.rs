/*
 * @Date: 2022-11-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-28
 * @FilePath: /algorithm/813_largest_sum_of_averages/largest_sum_of_averages.rs
 */

pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();
    let mut prefix: Vec<f64> = vec![0.0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as f64;
    }

    let mut dp: Vec<f64> = vec![0.0; n + 1];
    for i in 1..=n {
        dp[i] = prefix[i] / i as f64;
    }

    let k = k as usize;
    for j in 2..=k {
        for i in (j..=n).rev() {
            for x in j - 1..i {
                dp[i] = dp[i].max(dp[x] + (prefix[i] - prefix[x]) / (i - x) as f64);
            }
        }
    }

    dp[n]
}

fn main() {
    {
        let nums = vec![9, 1, 2, 3, 9];
        let k = 3;
        let ans = 20.00000000000;
        assert_eq!(largest_sum_of_averages(nums, k), ans);
    }

    {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 4;
        let ans = 20.50000000000;
        assert_eq!(largest_sum_of_averages(nums, k), ans);
    }
}
