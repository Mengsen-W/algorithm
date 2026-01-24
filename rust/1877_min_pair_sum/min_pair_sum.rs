struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        nums.sort();
        for i in 0..n {
            res = res.max(nums[i] + nums[n - i - 1]);
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![3, 5, 2, 3], 7), (vec![3, 5, 4, 2, 4, 6], 8)];

    for (nums, expected) in tests {
        assert_eq!(Solution::min_pair_sum(nums), expected);
    }
}
