struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::collections::HashMap;
        let mut nums = nums;
        nums.sort();

        let mut ans = 0;
        let mut num_count = HashMap::new();

        let mut last_num_index = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[last_num_index] {
                num_count.insert(nums[last_num_index], (i - last_num_index) as i32);
                ans = ans.max((i - last_num_index) as i32);
                last_num_index = i;
            }
        }

        num_count.insert(nums[last_num_index], (nums.len() - last_num_index) as i32);
        ans = ans.max((nums.len() - last_num_index) as i32);

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

        for i in nums[0]..=nums[nums.len() - 1] {
            let l = left_bound(i - k);
            let r = right_bound(i + k);

            let temp_ans = if let Some(&count) = num_count.get(&i) {
                (r - l + 1).min(count as usize + num_operations as usize)
            } else {
                (r - l + 1).min(num_operations as usize)
            };
            ans = ans.max(temp_ans as i32);
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
