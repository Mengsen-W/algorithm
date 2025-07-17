struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut res = 0;
        for num in nums {
            let mod_num = (num % k as i32) as usize;
            for prev in 0..k {
                dp[prev][mod_num] = dp[mod_num][prev] + 1;
                res = res.max(dp[prev][mod_num]);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 4, 5], 2, 5), (vec![1, 4, 2, 3, 1, 4], 3, 4)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::maximum_length(nums, k), expected);
    }
}
