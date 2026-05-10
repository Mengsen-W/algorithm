struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![i32::MIN; n];
        dp[0] = 0;

        for i in 1..n {
            for j in 0..i {
                if (nums[j] - nums[i]).abs() <= target {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        if dp[n - 1] < 0 {
            -1
        } else {
            dp[n - 1]
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 6, 4, 1, 2], 2, 3),
        (vec![1, 3, 6, 4, 1, 2], 3, 5),
        (vec![1, 3, 6, 4, 1, 2], 0, -1),
    ];

    for (nums, target, expected) in tests {
        assert_eq!(Solution::maximum_jumps(nums, target), expected);
    }
}
