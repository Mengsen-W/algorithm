struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = nums.len();
        let num_set: HashSet<&i32> = nums.iter().collect();
        let mut prefix_len = 1;

        for i in 1..n {
            if nums[i] == nums[i - 1] + 1 {
                prefix_len += 1;
            } else {
                break;
            }
        }

        let mut total = (nums[prefix_len - 1] + nums[0]) * (prefix_len as i32) / 2;
        while num_set.contains(&total) {
            total += 1;
        }

        total
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 2, 5], 6), (vec![3, 4, 5, 1, 12, 14, 13], 15)];

    for (nums, expected) in tests {
        assert_eq!(Solution::missing_integer(nums), expected);
    }
}
