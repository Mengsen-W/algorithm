struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];

        let mut left_sum = 0;
        for i in 0..n {
            ans[i] = left_sum;
            left_sum += nums[i];
        }

        let mut right_sum = 0;
        for i in (0..n).rev() {
            ans[i] = (ans[i] - right_sum).abs();
            right_sum += nums[i];
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![10, 4, 8, 3], vec![15, 1, 11, 22]), (vec![1], vec![0])];

    for (nums, ans) in tests {
        assert_eq!(Solution::left_right_difference(nums), ans);
    }
}
