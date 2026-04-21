/*
 * @Date: 2021-07-16 09:44:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-16 10:54:40
 */

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return 1;
            } else {
                return 0;
            }
        }
        return Solution::help(&nums, target) - Solution::help(&nums, target - 1);
    }

    fn help(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if mid > (nums.len() - 1) {
                break;
            }
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

fn main() {
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(Solution::search(nums, target), 2);
    }
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        assert_eq!(Solution::search(nums, target), 0);
    }
}
