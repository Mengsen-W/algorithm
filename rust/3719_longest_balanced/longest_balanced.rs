struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut max_len = 0;

        for i in 0..nums.len() {
            let mut odd = HashMap::new();
            let mut even = HashMap::new();

            for j in i..nums.len() {
                let map = if nums[j] & 1 == 1 {
                    &mut odd
                } else {
                    &mut even
                };

                *map.entry(nums[j]).or_insert(0) += 1;

                if odd.len() == even.len() {
                    max_len = max_len.max((j - i + 1) as i32);
                }
            }
        }

        max_len
    }
}

fn main() {
    let tests = vec![
        (vec![2, 5, 4, 3], 4),
        (vec![3, 2, 2, 5, 4], 5),
        (vec![1, 2, 3, 2], 3),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::longest_balanced(nums), expected);
    }
}
