struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<_> = nums.into_iter().collect();
        let mut original = original;
        while set.contains(&original) {
            original *= 2;
        }
        original
    }
}

fn main() {
    let tests = vec![(vec![5, 3, 6, 1, 12], 3, 24), (vec![2, 7, 9], 4, 4)];

    for (nums, original, expected) in tests {
        assert_eq!(Solution::find_final_value(nums, original), expected);
    }
}
