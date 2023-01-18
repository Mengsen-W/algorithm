/*
 * @Date: 2022-02-14 08:26:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-14 09:05:14
 * @FilePath: /algorithm/540_single_non_duplicate/single_non_duplicate.rs
 */

pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let (mut low, mut high) = (0, nums.len() - 1);
    while low < high {
        let mut mid = (high - low) / 2 + low;
        mid -= mid & 1;
        if nums[mid] == nums[mid + 1] {
            low = mid + 2;
        } else {
            high = mid;
        }
    }
    nums[low]
}

fn main() {
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
