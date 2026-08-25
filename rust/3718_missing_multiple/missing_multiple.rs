struct Solution;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let seen: HashSet<i32> = nums.into_iter().collect();
        let mut ans = k;
        while seen.contains(&ans) {
            ans += k;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![8, 2, 3, 4, 6], 2, 10), (vec![1, 4, 7, 10, 15], 5, 5)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::missing_multiple(nums, k), ans);
    }
}
