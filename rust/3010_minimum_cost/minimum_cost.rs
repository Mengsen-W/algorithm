struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for i in 1..nums.len() {
            let x = nums[i];
            if x < first {
                second = first;
                first = x;
            } else if x < second {
                second = x;
            }
        }
        nums[0] + first + second
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 12], 6),
        (vec![5, 4, 3], 12),
        (vec![10, 3, 1, 1], 12),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::minimum_cost(nums), expected);
    }
}
