/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-27 16:40:23
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-27 17:31:42
 */

fn binary_search(nums: &Vec<i32>, target: &i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;
    let mut mid: i32;

    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid as usize] == *target {
            return mid;
        } else if nums[mid as usize] < *target {
            left = mid + 1;
        } else if nums[mid as usize] > *target {
            right = mid - 1;
        }
    }
    -1
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let target: i32 = 3;
    println!("{}", binary_search(&nums, &target));
}
