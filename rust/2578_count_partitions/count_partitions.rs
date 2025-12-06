struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let n = nums.len();
        let mod_val = 1_000_000_007i64;
        let mut dp = vec![0i64; n + 1];
        let mut prefix = vec![0i64; n + 1];
        let mut min_q = VecDeque::new();
        let mut max_q = VecDeque::new();

        dp[0] = 1;
        prefix[0] = 1;
        let mut j = 0;

        for i in 0..n {
            // 维护最大值队列
            while let Some(&back) = max_q.back() {
                if nums[back] <= nums[i] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(i);

            // 维护最小值队列
            while let Some(&back) = min_q.back() {
                if nums[back] >= nums[i] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(i);

            // 调整窗口
            while !max_q.is_empty()
                && !min_q.is_empty()
                && nums[*max_q.front().unwrap()] - nums[*min_q.front().unwrap()] > k
            {
                if *max_q.front().unwrap() == j {
                    max_q.pop_front();
                }
                if *min_q.front().unwrap() == j {
                    min_q.pop_front();
                }
                j += 1;
            }

            let val = if j > 0 { prefix[j - 1] } else { 0 };
            dp[i + 1] = (prefix[i] - val + mod_val) % mod_val;
            prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod_val;
        }

        dp[n] as i32
    }
}

fn main() {
    let tests = vec![(vec![9, 4, 1, 3, 7], 4, 6), (vec![3, 3, 4], 0, 2)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::count_partitions(nums, k), expected);
    }
}
