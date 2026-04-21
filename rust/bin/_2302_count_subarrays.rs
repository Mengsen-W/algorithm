struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut res = 0;
        let mut total = 0;
        let mut i = 0;
        for j in 0..n {
            total += nums[j] as i64;
            while i <= j && total * (j - i + 1) as i64 >= k {
                total -= nums[i] as i64;
                i += 1;
            }
            res += j - i + 1;
        }
        res as i64
    }
}

fn main() {
    let tests = vec![(vec![2, 1, 4, 3, 5], 10, 6), (vec![1, 1, 1], 5, 5)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::count_subarrays(nums, k), expected);
    }
}
