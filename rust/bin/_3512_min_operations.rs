struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = nums.iter().map(|&x| x).sum();
        sum % k
    }
}

fn main() {
    let tests = vec![
        (vec![3, 9, 7], 5, 4),
        (vec![4, 1, 3], 4, 0),
        (vec![3, 2], 6, 5),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::min_operations(nums, k), ans);
    }
}
