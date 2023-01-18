/*
 * @Date: 2021-08-26 11:45:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-26 11:47:09
 * @FilePath: /algorithm/1_two_sum/two_sum.rs
 * @Description: file content
 */

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            if let Some(value) = map.get(&(target - nums[i])) {
                if *value != i as i32 {
                    return vec![*value as i32, i as i32];
                }
            }
            map.insert(nums[i], i as i32);
        }

        panic!("没有找到");
    }
}

fn main() {
    {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), ans);
    }
    {
        let nums = vec![3, 2, 4];
        let target = 6;
        let ans = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), ans);
    }
    {
        let nums = vec![3, 3];
        let target = 6;
        let ans = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), ans);
    }
}
