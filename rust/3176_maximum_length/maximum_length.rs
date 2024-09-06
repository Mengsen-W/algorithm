struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![-1; 51]; n];
        let mut ans = 0;

        for i in 0..n {
            dp[i][0] = 1;
            for l in 0..=k as usize {
                for j in 0..i {
                    let add = if nums[i] != nums[j] { 1 } else { 0 };
                    if l >= add && dp[j][l - add] != -1 {
                        dp[i][l] = dp[i][l].max(dp[j][l - add] + 1);
                    }
                }
                ans = ans.max(dp[i][l]);
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 1, 1, 3], 2, 4), (vec![1, 2, 3, 4, 5, 1], 0, 2)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::maximum_length(nums, k), ans);
    }
}
