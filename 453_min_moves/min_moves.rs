/*
 * @Date: 2021-10-20 12:16:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-20 12:21:59
 */

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>() - nums.iter().min().unwrap() * nums.len() as i32
    }
}

fn main() {
    assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
}
