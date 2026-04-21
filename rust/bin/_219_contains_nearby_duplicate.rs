struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;
        let mut cache = HashSet::new();

        for i in 0..nums.len() {
            if i as i32 - k > 0 {
                cache.remove(&nums[i - k as usize - 1]);
            }
            if !cache.insert(nums[i]) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 1], 3, true),
        (vec![1, 0, 1, 1], 1, true),
        (vec![1, 2, 3, 1, 2, 3], 2, false),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), ans);
    }
}
