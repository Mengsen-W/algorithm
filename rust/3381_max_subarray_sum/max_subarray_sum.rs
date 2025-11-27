struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut prefix_sum: i64 = 0;
        let mut max_sum: i64 = i64::MIN;
        let k = k as usize;
        let mut k_sum: Vec<i64> = vec![i64::MAX / 2; k];
        k_sum[k - 1] = 0;
        for i in 0..n {
            prefix_sum += nums[i] as i64;
            let idx = i % k;
            max_sum = max_sum.max(prefix_sum - k_sum[idx]);
            k_sum[idx] = k_sum[idx].min(prefix_sum);
        }
        max_sum
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2], 1, 3),
        (vec![-1, -2, -3, -4, -5], 4, -10),
        (vec![-5, 1, 2, -3, 4], 2, 4),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_subarray_sum(nums, k), expected);
    }
}
