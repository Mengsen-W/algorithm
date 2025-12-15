struct Solution;

impl Solution {
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
}

fn main() {
    let tests = vec![
        (vec![3, 6, 5, 1, 8], 18),
        (vec![4], 0),
        (vec![1, 2, 3, 4, 4], 12),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_sum_div_three(nums), ans);
    }
}
