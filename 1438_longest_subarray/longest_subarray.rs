/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-21 09:12:53
 * @Last Modified by:   Mengsen.Wang
 * @Last Modified time: 2021-02-21 09:12:53
 */

use std::collections::VecDeque;

fn longest_sub_array(nums: Vec<i32>, limit: i32) -> i32 {
    let len = nums.len();
    let mut min_queue = VecDeque::new();
    let mut max_queue = VecDeque::new();
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;

    while right < len {
        while !max_queue.is_empty() && max_queue.back() < Some(&nums[right]) {
            max_queue.pop_back();
        }
        while !min_queue.is_empty() && min_queue.back() > Some(&nums[right]) {
            min_queue.pop_back();
        }
        max_queue.push_back(nums[right]);
        min_queue.push_back(nums[right]);

        while !min_queue.is_empty() && !min_queue.is_empty() && max_queue[0] - min_queue[0] > limit
        {
            if nums[left] == min_queue[0] {
                min_queue.pop_front();
            }
            if nums[left] == max_queue[0] {
                max_queue.pop_front();
            }
            left += 1;
        }

        res = res.max(right - left + 1);
        right += 1;
    }
    return res as i32;
}

fn main() {
    let mut nums: Vec<i32>;
    nums = vec![8, 2, 4, 7];
    assert_eq!(longest_sub_array(nums, 4), 2);
    nums = vec![10, 1, 2, 4, 7, 2];
    assert_eq!(longest_sub_array(nums, 5), 4);
    nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
    assert_eq!(longest_sub_array(nums, 0), 3);
}
