struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
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
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], vec![1, 2]),
        (vec![1, 2, 4, 8], vec![1, 2, 4, 8]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::largest_divisible_subset(nums), ans);
    }
}
