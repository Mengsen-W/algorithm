struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1] % 2 == nums[i] % 2 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let tests = vec![
        (vec![1], true),
        (vec![2, 1, 4], true),
        (vec![4, 3, 1, 6], false),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::is_array_special(nums), ans);
    }
}
