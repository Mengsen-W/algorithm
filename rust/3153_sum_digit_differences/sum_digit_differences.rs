struct Solution;

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut nums = nums.clone();
        let mut res: i64 = 0;
        let n = nums.len();
        while nums[0] > 0 {
            let mut cnt = vec![0; 10];
            for i in 0..n {
                cnt[nums[i] as usize % 10] += 1;
                nums[i] /= 10;
            }
            for i in 0..10 {
                res += (n - cnt[i]) as i64 * (cnt[i]) as i64;
            }
        }
        return res / 2;
    }
}

fn main() {
    let tests = vec![(vec![13, 23, 12], 4), (vec![10, 10, 10, 10], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::sum_digit_differences(nums), ans);
    }
}
