struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let positive_nums_set: HashSet<i32> = nums.iter().filter(|&&x| x > 0).cloned().collect();
        if positive_nums_set.is_empty() {
            *nums.iter().max().unwrap()
        } else {
            positive_nums_set.iter().sum()
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], 15),
        (vec![1, 1, 0, 1, 1], 1),
        (vec![1, 2, -1, -2, 1, 0, -1], 3),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_sum(nums), expected);
    }
}
