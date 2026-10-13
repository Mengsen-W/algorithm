struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len();

        for num in nums.iter_mut() {
            *num *= *num;
        }
        nums.sort_unstable();

        let mut ans = 0i64;
        for i in 0..n / 2 {
            ans -= nums[i] as i64;
        }
        for i in n / 2..n {
            ans += nums[i] as i64;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 12), (vec![1, -1, 2, -2, 3, -3], 16)];

    for (test, expected) in tests {
        assert_eq!(Solution::max_alternating_sum(test), expected);
    }
}
