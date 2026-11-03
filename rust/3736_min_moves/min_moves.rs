struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        nums.iter().map(|x| mx - x).sum()
    }
}

fn main() {
    let tests = vec![(vec![2, 1, 3], 3), (vec![4, 4, 5], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::min_moves(nums), ans);
    }
}
