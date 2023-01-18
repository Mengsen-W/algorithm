/*
 * @Date: 2022-04-30 08:16:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-30 08:30:40
 * @FilePath: /algorithm/908_smallest_range_i/smallest_range_i.rs
 */

pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    (nums.iter().max().unwrap() - nums.iter().min().unwrap() - 2 * k).max(0)
}

fn main() {
    assert_eq!(smallest_range_i(vec![1], 0), 0);
    assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
}
