/*
 * @Date: 2023-06-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-19
 * @FilePath: /algorithm/rust/1262_max_sum_div_three/max_sum_div_three.rs
 */

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0, i32::MIN, i32::MIN];

    for num in nums {
        let mut temp_dp = dp.clone();
        for i in 0..3 {
            let cmod = ((dp[i] + num) % 3 + 3) as usize % 3;
            temp_dp[cmod] = temp_dp[cmod].max(dp[i] + num);
        }
        dp = temp_dp;
    }
    dp[0]
}

fn main() {
    {
        let nums = vec![3, 6, 5, 1, 8];
        let ans = 18;
        assert_eq!(max_sum_div_three(nums), ans);
    }

    {
        let nums = vec![4];
        let ans = 0;
        assert_eq!(max_sum_div_three(nums), ans);
    }

    {
        let nums = vec![1, 2, 3, 4, 4];
        let ans = 12;
        assert_eq!(max_sum_div_three(nums), ans);
    }
}
