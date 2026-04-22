struct Solution;

impl Solution {
    pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res: i32 = 0x3f3f3f3f;
        for i in 1..nums.len() {
            res = std::cmp::min(res, nums[i] - nums[i - 1]);
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 2, 4], 1), (vec![100, 1, 10], 9)];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_value_of_partition(nums), ans);
    }
}
