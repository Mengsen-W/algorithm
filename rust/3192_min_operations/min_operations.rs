struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut operation = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] != nums[i + 1] {
                operation += 1;
            }
        }
        if nums[0] == 1 {
            operation
        } else {
            operation + 1
        }
    }
}

fn main() {
    let tests = vec![(vec![0, 1, 1, 0, 1], 4), (vec![1, 0, 0, 0], 1)];

    for (nums, ans) in tests {
        assert_eq!(Solution::min_operations(nums), ans);
    }
}
