struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums.len();
        let k_usize = k as usize;
        let mut result = vec![0i64; k_usize];
        let mut dp = vec![0i64; k_usize]; // 初始状态，表示尚未处理任何元素，因此不存在非空子数组

        for i in 0..n {
            let mut ndp = vec![0i64; k_usize]; // 当前层状态（滚动数组）
            ndp[(nums[i] as usize) % k_usize] += 1;
            for r in 0..k_usize {
                ndp[((r as i64 * nums[i] as i64) % k as i64) as usize] += dp[r];
            }

            dp = ndp; // 更新状态
                      // 累加答案
            for r in 0..k_usize {
                result[r] += dp[r];
            }
        }

        result
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], 3, vec![9, 2, 4]),
        (vec![1, 2, 4, 8, 16, 32], 4, vec![18, 1, 2, 0]),
        (vec![1, 1, 2, 1, 1], 2, vec![9, 6]),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::result_array(nums, k), expected);
    }
}
