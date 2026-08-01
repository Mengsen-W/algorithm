struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![0; n];
        for i in (0..n).rev() {
            dp[i] = nums[i];
            for j in i + 1..n {
                dp[j] = std::cmp::max(nums[i] - dp[j], nums[j] - dp[j - 1]);
            }
        }
        dp[n - 1] >= 0
    }
}

fn main() {
    let tests = vec![(vec![1, 5, 2], false), (vec![1, 5, 233, 7], true)];

    for (nums, ans) in tests {
        assert_eq!(Solution::predict_the_winner(nums), ans);
    }
}
