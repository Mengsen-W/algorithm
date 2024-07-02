struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let primes: HashSet<i32> = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
        .iter()
        .cloned()
        .collect();

        let mut first = -1;
        let mut ans = 0;

        for (i, &num) in nums.iter().enumerate() {
            if primes.contains(&num) {
                if first != -1 {
                    ans = ans.max(i as i32 - first);
                } else {
                    first = i as i32;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![4, 2, 9, 5, 3], 3), (vec![4, 8, 2, 8], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::maximum_prime_difference(nums), ans);
    }
}
