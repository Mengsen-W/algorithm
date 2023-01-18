/*
 * @Date: 2021-11-01 01:10:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-01 01:18:23
 */

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        candy_type
            .iter()
            .map(|&i| i)
            .collect::<HashSet<i32>>()
            .len()
            .min(candy_type.len() / 2) as i32
    }
}

fn main() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
}
