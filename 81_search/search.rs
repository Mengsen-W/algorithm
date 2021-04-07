/*
 * @Date: 2021-04-07 10:33:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-07 10:53:05
 */

fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        while left < right && nums[left] == nums[left + 1] {
            left += 1;
        }
        while left < right && nums[right] == nums[right - 1] {
            right -= 1;
        }
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[mid] >= nums[left] {
            if target < nums[mid] && target >= nums[left] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if target > nums[mid] && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    false
}

fn main() {
    let nums = vec![5, 1, 3];
    assert!(search(nums.clone(), 5));
    assert!(search(nums.clone(), 3));
}
