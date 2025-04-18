struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut h = HashMap::new();
        let mut res = 0 as i64;
        for (i, x) in nums.iter().enumerate() {
            let key = x - i as i32;
            res += i as i64 - *h.get(&key).unwrap_or(&0);
            *h.entry(key).or_default() += 1;
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![4, 1, 3, 3], 5), (vec![1, 2, 3, 4, 5], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_bad_pairs(nums), ans);
    }
}
