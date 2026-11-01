struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>() - nums.iter().min().unwrap() * nums.len() as i32
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 3), (vec![1, 1, 1], 0)];
    for (nums, ans) in tests {
        assert_eq!(Solution::min_moves(nums), ans);
    }
}
