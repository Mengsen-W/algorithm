struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

fn main() {
    let tests = vec![
        (vec![10, 10, 3, 7, 6], 4),
        (vec![1, 2, 2], 0),
        (vec![2, 4, 6, 8], 3),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::count_partitions(nums), expected);
    }
}
