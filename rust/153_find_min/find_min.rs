/*
 * @Date: 2021-04-08 10:30:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-08 10:36:00
 */

fn find_min(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low < high {
        let pivot = low + (high - low) / 2;
        if nums[pivot] < nums[high] {
            high = pivot;
        } else {
            low = pivot + 1
        }
    }
    nums[low]
}

fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(find_min(nums), 1);
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(find_min(nums), 0);
    let nums = vec![11, 13, 15, 17];
    assert_eq!(find_min(nums), 11);
}
