struct Solution;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        cnt.insert(0, 1);
        let mut mask = 0;
        let mut ans = 0;
        for x in nums {
            mask ^= x;
            ans += *cnt.get(&mask).unwrap_or(&0);
            *cnt.entry(mask).or_insert(0) += 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![4, 3, 1, 2, 4], 2), (vec![1, 10, 4], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::beautiful_subarrays(nums), ans);
    }
}
