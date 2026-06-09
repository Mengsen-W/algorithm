struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let m1 = *nums.iter().min().unwrap() as i64;
        let m2 = *nums.iter().max().unwrap() as i64;
        (m2 - m1) * k as i64
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 2], 2, 4), (vec![4, 2, 5, 1], 3, 12)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_total_value(nums, k), expected);
    }
}
