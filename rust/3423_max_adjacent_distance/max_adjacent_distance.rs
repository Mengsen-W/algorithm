struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = (nums[0] - nums[n - 1]).abs();
        for i in 0..n - 1 {
            res = res.max((nums[i] - nums[i + 1]).abs());
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 4], 3), (vec![-5, -10, -5], 5)];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_adjacent_distance(nums), expected);
    }
}
