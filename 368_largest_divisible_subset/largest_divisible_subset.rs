/*
 * @Date: 2021-04-23 08:57:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-23 09:37:24
 * @FilePath: \algorithm\368_largest_divisible_subset\largest_divisible_subset.rs
 * @Description: file content
 */

fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort(); // 排序
    let l = nums.len();

    if l == 0 {
        return Vec::new();
    }

    let mut dp = vec![vec![nums[0]]]; // nums[i]的整除子集
    let mut ans = dp[0].clone();

    for i in 1..l {
        dp.push(vec![nums[i]]);

        for j in 0..i {
            if nums[i] % dp[j][dp[j].len() - 1] == 0 {
                // 最后一个能整除
                if dp[j].len() >= dp[i].len() - 1 {
                    dp[i] = dp[j].clone();
                    dp[i].push(nums[i]);
                }
            }
        }

        if ans.len() < dp[i].len() {
            ans = dp[i].clone();
        }
    }

    ans
}

fn main() {
    {
        let nums = vec![1, 2, 3];
        assert_eq!(largest_divisible_subset(nums), vec![1, 2]);
    }
    {
        let nums = vec![1, 2, 4, 8];
        assert_eq!(largest_divisible_subset(nums), vec![1, 2, 4, 8]);
    }
}
