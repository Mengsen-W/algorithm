struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 1..n - 1 {
            if nums[i] == (nums[i - 1] + nums[i + 1]) * 2 {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 1, 4, 1], 1), (vec![1, 1, 1], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_subarrays(nums), ans);
    }
}
