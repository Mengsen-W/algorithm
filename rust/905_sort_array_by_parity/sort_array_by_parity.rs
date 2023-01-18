/*
 * @Date: 2022-04-28 09:24:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-28 09:35:49
 * @FilePath: /algorithm/905_sort_array_by_parity/sort_array_by_parity.rs
 */

pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        while left < right && nums[left] % 2 == 0 {
            left += 1;
        }
        while left < right && nums[right] % 2 == 1 {
            right -= 1;
        }
        if left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    nums
}

fn main() {
    assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), vec![4, 2, 1, 3]);
    assert_eq!(sort_array_by_parity(vec![0]), vec![0]);
}
