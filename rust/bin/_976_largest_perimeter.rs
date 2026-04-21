struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (2..nums.len()).rev() {
            if nums[i - 2] + nums[i - 1] > nums[i] {
                return nums[i - 2] + nums[i - 1] + nums[i];
            }
        }
        0
    }
}

fn main() {
    let tests = vec![(vec![2, 1, 2], 5), (vec![1, 2, 1, 10], 0)];

    for (nums, expected) in tests {
        assert_eq!(Solution::largest_perimeter(nums), expected);
    }
}
