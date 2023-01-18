/*
 * @Date: 2021-04-09 08:48:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-09 09:00:20
 */

fn find_min(nums: &Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low < high {
        let pivot = low + (high - low) / 2;
        if nums[pivot] < nums[high] {
            high = pivot;
        } else if nums[pivot] > nums[high] {
            low = pivot + 1;
        } else {
            high -= 1;
        }
    }
    return nums[low];
}

fn main() {
    let nums = vec![1, 3, 5];
    assert_eq!(find_min(&nums), 1);
    let nums = vec![2, 2, 2, 0, 1];
    assert_eq!(find_min(&nums), 0);
}
