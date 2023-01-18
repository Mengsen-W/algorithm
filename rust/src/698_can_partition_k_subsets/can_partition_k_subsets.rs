/*
 * @Date: 2022-09-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-20
 * @FilePath: /algorithm/698_can_partition_k_subsets/can_partition_k_subsets.rs
 */

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % k != 0 {
        return false;
    }

    let n = sum / k;

    let mut dp = vec![-1; 1 << nums.len()];

    dp[0] = 0;

    for i in 1..(1 << nums.len()) as usize {
        for j in 0..nums.len() {
            if i & (1 << j) == 0 {
                continue;
            }

            if dp[i - (1 << j)] + nums[j] > n {
                continue;
            }

            if dp[i - (1 << j)] != -1 {
                dp[i] = (dp[i - (1 << j)] + nums[j]) % n;
            }
        }
    }

    return dp[(1 << nums.len()) as usize - 1] == 0;
}

fn main() {
    {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        assert_eq!(can_partition_k_subsets(nums, k), true);
    }

    {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        assert_eq!(can_partition_k_subsets(nums, k), false);
    }
}
