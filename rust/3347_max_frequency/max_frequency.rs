struct Solution;

use std::cmp;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut ans = 0;
        let mut num_count = HashMap::new();
        let mut modes = HashSet::new();

        let mut add_mode = |value: i32| {
            modes.insert(value);
            if value - k >= nums[0] {
                modes.insert(value - k);
            }
            if value + k <= nums[nums.len() - 1] {
                modes.insert(value + k);
            }
        };

        let mut last_num_index = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[last_num_index] {
                num_count.insert(nums[last_num_index], i - last_num_index);
                ans = cmp::max(ans, (i - last_num_index) as i32);
                add_mode(nums[last_num_index]);
                last_num_index = i;
            }
        }

        num_count.insert(nums[last_num_index], nums.len() - last_num_index);
        ans = cmp::max(ans, (nums.len() - last_num_index) as i32);
        add_mode(nums[last_num_index]);

        let left_bound = |value: i32| -> usize {
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left < right {
                let mid = (left + right) / 2;
                if nums[mid] < value {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        };

        let right_bound = |value: i32| -> usize {
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left < right {
                let mid = (left + right + 1) / 2;
                if nums[mid] > value {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            left
        };

        let mut sorted_modes: Vec<i32> = modes.into_iter().collect();
        sorted_modes.sort();

        for mode in sorted_modes {
            let l = left_bound(mode - k);
            let r = right_bound(mode + k);
            let temp_ans = if let Some(&count) = num_count.get(&mode) {
                cmp::min(r - l + 1, count + num_operations as usize)
            } else {
                cmp::min(r - l + 1, num_operations as usize)
            };
            ans = cmp::max(ans, temp_ans as i32);
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 4, 5], 1, 2, 2), (vec![5, 11, 20, 20], 5, 1, 2)];

    for (nums, k, num_operations, expected) in tests {
        assert_eq!(Solution::max_frequency(nums, k, num_operations), expected);
    }
}
