struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        for &num in &nums {
            res |= num;
        }
        res << (n - 1)
    }
}

fn main() {
    let tests = vec![(vec![1, 3], 6), (vec![5, 1, 6], 28)];

    for (nums, ans) in tests {
        assert_eq!(Solution::subset_xor_sum(nums), ans);
    }
}
